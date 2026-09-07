//! Set algebra over rectangle lists, in a canonical YX-banded form.
//!
//! Adapted from yserver's `crates/yserver/src/kms/render/region.rs` (MIT
//! License, Copyright (c) 2026 Jos Dehaes). The half-open band
//! decomposition, the shared one-dimensional interval merge, and the
//! canonical-form invariants are theirs. The rectangle type, the overflow
//! discipline, and the removal of their bounded-collapse cap are Sophia's.
//!
//! Binary operations decompose on `y` rather than merging bands
//! incrementally the way pixman does: the distinct `y` edges of both
//! operands are collected, each resulting slice combines the operands'
//! `x` spans with a one-dimensional interval operation, and adjacent slices
//! carrying identical spans are merged. That is `O(n*m)` in rectangle counts
//! against pixman's `O(n+m)`, which is the right trade here -- the operands
//! are window shapes and clip lists of a handful of rectangles, and interval
//! algebra is far easier to get right, and to test exhaustively, than
//! incremental band merging.
//!
//! yserver's implementation collapses a region to its bounding box past
//! thirty-two rectangles. That cap is deliberately absent here: it is only
//! safe in one direction. A region that is collapsed and then *added*
//! over-covers, which costs work; one that is collapsed and then
//! *subtracted* over-subtracts, which is a wrong answer. These regions come
//! from client requests whose length already bounds them, so nothing needs
//! the cap and nothing pays for its asymmetry.
//!
//! # Canonical form
//!
//! Every function here returns rectangles that are sorted by `(y, x)`,
//! pairwise disjoint, and vertically coalesced -- adjacent bands never carry
//! identical `x` spans. Two canonical lists are equal exactly when they
//! cover the same area, which is what makes [`regions_equal`] a real region
//! comparison rather than a list comparison, and what lets a caller treat a
//! change in the list as a change in the shape.

use super::Rect;

/// A half-open span `[start, end)` on one axis.
///
/// Half-open bounds are what keep this free of the off-by-one that inclusive
/// rectangle algebra invites: two spans abut exactly when one's `end` equals
/// the other's `start`, with no adjustment.
type Span = (i64, i64);

/// Which way two span lists combine.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Op {
    Union,
    Intersect,
    Subtract,
}

/// One `y` band of a decomposed region: the slice `[top, bottom)` and the
/// `x` spans covered within it.
struct Band {
    top: i64,
    bottom: i64,
    spans: Vec<Span>,
}

/// The `x` spans a rectangle list covers within the slice `[top, bottom)`.
///
/// A rectangle takes part only when it spans the slice completely, which it
/// must: the slice boundaries were built from every rectangle's own edges,
/// so no rectangle can begin or end inside one.
fn spans_in_slice(rects: &[Rect], top: i64, bottom: i64) -> Vec<Span> {
    let mut spans: Vec<Span> = rects
        .iter()
        .filter(|rect| !rect.is_empty())
        .filter_map(|rect| {
            let rect_top = i64::from(rect.y);
            let rect_bottom = rect_top + i64::from(rect.height);
            (rect_top <= top && rect_bottom >= bottom).then(|| {
                let left = i64::from(rect.x);
                (left, left + i64::from(rect.width))
            })
        })
        .collect();
    coalesce_1d(&mut spans);
    spans
}

/// Sort and merge overlapping or abutting spans in place.
fn coalesce_1d(spans: &mut Vec<Span>) {
    spans.retain(|(start, end)| start < end);
    spans.sort_unstable();
    let mut merged: Vec<Span> = Vec::with_capacity(spans.len());
    for (start, end) in spans.iter().copied() {
        match merged.last_mut() {
            // Abutting counts as overlapping: [0,5) and [5,9) are one span,
            // which is what keeps the canonical form minimal.
            Some(last) if start <= last.1 => last.1 = last.1.max(end),
            _ => merged.push((start, end)),
        }
    }
    *spans = merged;
}

/// Combine two span lists on one axis.
fn combine_1d(a: &[Span], b: &[Span], op: Op) -> Vec<Span> {
    let mut edges: Vec<i64> = Vec::with_capacity((a.len() + b.len()) * 2);
    for (start, end) in a.iter().chain(b).copied() {
        edges.push(start);
        edges.push(end);
    }
    edges.sort_unstable();
    edges.dedup();

    let covers = |spans: &[Span], point: i64| spans.iter().any(|(s, e)| *s <= point && point < *e);
    let mut out: Vec<Span> = Vec::new();
    for window in edges.windows(2) {
        let (start, end) = (window[0], window[1]);
        if start >= end {
            continue;
        }
        let in_a = covers(a, start);
        let in_b = covers(b, start);
        let keep = match op {
            Op::Union => in_a || in_b,
            Op::Intersect => in_a && in_b,
            Op::Subtract => in_a && !in_b,
        };
        if keep {
            out.push((start, end));
        }
    }
    coalesce_1d(&mut out);
    out
}

/// Decompose two rectangle lists into shared `y` slices and combine each.
fn combine(a: &[Rect], b: &[Rect], op: Op) -> Vec<Rect> {
    let mut edges: Vec<i64> = Vec::new();
    for rect in a.iter().chain(b).filter(|rect| !rect.is_empty()) {
        let top = i64::from(rect.y);
        edges.push(top);
        edges.push(top + i64::from(rect.height));
    }
    edges.sort_unstable();
    edges.dedup();

    let mut bands: Vec<Band> = Vec::new();
    for window in edges.windows(2) {
        let (top, bottom) = (window[0], window[1]);
        if top >= bottom {
            continue;
        }
        let spans = combine_1d(
            &spans_in_slice(a, top, bottom),
            &spans_in_slice(b, top, bottom),
            op,
        );
        if spans.is_empty() {
            continue;
        }
        // Vertical coalescing: a band with the same spans as the one it
        // abuts is the same band. Without this the form is not canonical and
        // equality stops meaning what it should.
        match bands.last_mut() {
            Some(last) if last.bottom == top && last.spans == spans => last.bottom = bottom,
            _ => bands.push(Band { top, bottom, spans }),
        }
    }

    let mut out = Vec::new();
    for band in bands {
        for (start, end) in band.spans {
            // The inputs are i32 rectangles and every edge is one of theirs,
            // so this cannot lose information; the arithmetic ran wider only
            // so intermediate sums could not wrap.
            out.push(Rect {
                x: start as i32,
                y: band.top as i32,
                width: (end - start) as i32,
                height: (band.bottom - band.top) as i32,
            });
        }
    }
    out
}

/// The canonical form of a rectangle list: sorted, disjoint, coalesced.
///
/// Empty rectangles drop out, overlaps merge, and the result covers exactly
/// the area the input did.
pub fn canonicalize(rects: &[Rect]) -> Vec<Rect> {
    combine(rects, &[], Op::Union)
}

/// Everything either list covers.
pub fn union(a: &[Rect], b: &[Rect]) -> Vec<Rect> {
    combine(a, b, Op::Union)
}

/// Everything both lists cover.
pub fn intersect(a: &[Rect], b: &[Rect]) -> Vec<Rect> {
    combine(a, b, Op::Intersect)
}

/// Everything `a` covers and `b` does not.
pub fn subtract(a: &[Rect], b: &[Rect]) -> Vec<Rect> {
    combine(a, b, Op::Subtract)
}

/// Move every rectangle, saturating rather than wrapping at the edges of the
/// coordinate space.
pub fn translate(rects: &[Rect], dx: i32, dy: i32) -> Vec<Rect> {
    let moved: Vec<Rect> = rects
        .iter()
        .filter(|rect| !rect.is_empty())
        .map(|rect| Rect {
            x: rect.x.saturating_add(dx),
            y: rect.y.saturating_add(dy),
            width: rect.width,
            height: rect.height,
        })
        .collect();
    canonicalize(&moved)
}

/// The smallest rectangle containing every rectangle, or `None` when the
/// region is empty.
pub fn extents(rects: &[Rect]) -> Option<Rect> {
    let mut bounds: Option<(i64, i64, i64, i64)> = None;
    for rect in rects.iter().filter(|rect| !rect.is_empty()) {
        let left = i64::from(rect.x);
        let top = i64::from(rect.y);
        let right = left + i64::from(rect.width);
        let bottom = top + i64::from(rect.height);
        bounds = Some(match bounds {
            Some((l, t, r, b)) => (l.min(left), t.min(top), r.max(right), b.max(bottom)),
            None => (left, top, right, bottom),
        });
    }
    bounds.map(|(left, top, right, bottom)| Rect {
        x: left as i32,
        y: top as i32,
        width: (right - left) as i32,
        height: (bottom - top) as i32,
    })
}

/// Whether a point falls inside the region.
pub fn contains_point(rects: &[Rect], x: i32, y: i32) -> bool {
    rects.iter().any(|rect| {
        !rect.is_empty()
            && x >= rect.x
            && y >= rect.y
            && x < rect.x.saturating_add(rect.width)
            && y < rect.y.saturating_add(rect.height)
    })
}

/// Whether two rectangle lists cover the same area, however each is written.
pub fn regions_equal(a: &[Rect], b: &[Rect]) -> bool {
    canonicalize(a) == canonicalize(b)
}

/// The total area covered, counting overlap once.
pub fn area(rects: &[Rect]) -> u64 {
    canonicalize(rects)
        .iter()
        .map(|rect| u64::from(rect.width.max(0) as u32) * u64::from(rect.height.max(0) as u32))
        .sum()
}
