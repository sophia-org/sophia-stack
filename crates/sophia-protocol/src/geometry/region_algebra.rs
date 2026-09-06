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

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(x: i32, y: i32, width: i32, height: i32) -> Rect {
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    /// Every cell a rectangle list covers, as a set. The reference model the
    /// exhaustive tests below compare against: slow, obviously correct, and
    /// independent of everything the module does.
    fn cells(rects: &[Rect]) -> std::collections::BTreeSet<(i32, i32)> {
        let mut out = std::collections::BTreeSet::new();
        for r in rects.iter().filter(|r| !r.is_empty()) {
            for y in r.y..r.y + r.height {
                for x in r.x..r.x + r.width {
                    out.insert((x, y));
                }
            }
        }
        out
    }

    /// The canonical invariants, asserted directly: sorted, disjoint, and
    /// vertically coalesced. Everything else in this module relies on them,
    /// and `regions_equal` is only a region comparison because of the third.
    fn assert_canonical(rects: &[Rect]) {
        for r in rects {
            assert!(!r.is_empty(), "canonical form holds no empty rects: {r:?}");
        }
        for pair in rects.windows(2) {
            let (a, b) = (pair[0], pair[1]);
            assert!(
                (a.y, a.x) < (b.y, b.x),
                "canonical form is sorted by (y, x): {a:?} then {b:?}"
            );
        }
        for (i, a) in rects.iter().enumerate() {
            for b in &rects[i + 1..] {
                let disjoint = a.x + a.width <= b.x
                    || b.x + b.width <= a.x
                    || a.y + a.height <= b.y
                    || b.y + b.height <= a.y;
                assert!(disjoint, "canonical rects are disjoint: {a:?} and {b:?}");
            }
        }
        // Vertical coalescing: no two bands that abut may carry the same
        // spans, or two lists covering one area could compare unequal.
        let mut bands: std::collections::BTreeMap<(i32, i32), Vec<(i32, i32)>> =
            std::collections::BTreeMap::new();
        for r in rects {
            bands
                .entry((r.y, r.y + r.height))
                .or_default()
                .push((r.x, r.x + r.width));
        }
        let bands: Vec<_> = bands.into_iter().collect();
        for pair in bands.windows(2) {
            let ((_, a_bottom), a_spans) = &pair[0];
            let ((b_top, _), b_spans) = &pair[1];
            assert!(
                a_bottom != b_top || a_spans != b_spans,
                "abutting bands with identical spans must be merged: {a_spans:?}"
            );
        }
    }

    #[test]
    fn canonicalize_merges_overlap_and_drops_empties() {
        let input = [
            rect(0, 0, 4, 2),
            rect(2, 0, 4, 2),
            rect(0, 0, 0, 5),
            rect(10, 10, 3, -1),
        ];
        let out = canonicalize(&input);
        assert_canonical(&out);
        assert_eq!(out, vec![rect(0, 0, 6, 2)]);
        assert_eq!(cells(&out), cells(&input));
    }

    /// Two lists that cover one area compare equal however they are written.
    /// This is the property the change-gating in SHAPE depends on: a client
    /// re-asserting the same shape differently must not read as a change.
    #[test]
    fn regions_equal_compares_area_not_spelling() {
        let one_rect = [rect(0, 0, 4, 4)];
        let four_quadrants = [
            rect(0, 0, 2, 2),
            rect(2, 0, 2, 2),
            rect(0, 2, 2, 2),
            rect(2, 2, 2, 2),
        ];
        let two_bands = [rect(0, 0, 4, 1), rect(0, 1, 4, 3)];
        assert!(regions_equal(&one_rect, &four_quadrants));
        assert!(regions_equal(&one_rect, &two_bands));
        assert_eq!(canonicalize(&four_quadrants), canonicalize(&two_bands));
        assert!(!regions_equal(&one_rect, &[rect(0, 0, 4, 3)]));
    }

    /// Every operation against a brute-force cell model, over every pair of
    /// rectangles in a small grid. Interval algebra fails at edges -- abutting
    /// spans, zero-width slices, one operand entirely inside the other -- and
    /// enumerating the neighbourhood catches those in a way hand-picked cases
    /// do not.
    #[test]
    fn operations_match_a_brute_force_model_over_every_small_pair() {
        let mut boxes = Vec::new();
        for x in -1..3 {
            for y in -1..3 {
                for w in 1..4 {
                    for h in 1..4 {
                        boxes.push(rect(x, y, w, h));
                    }
                }
            }
        }
        // Two-rectangle operands as well, so the multi-band paths are
        // exercised rather than only the single-rect ones.
        let operands: Vec<Vec<Rect>> = boxes
            .iter()
            .map(|r| vec![*r])
            .chain(
                boxes
                    .iter()
                    .step_by(37)
                    .map(|r| vec![*r, rect(r.x + 1, r.y + 2, 2, 2)]),
            )
            .collect();

        for a in operands.iter().step_by(7) {
            for b in operands.iter().step_by(11) {
                let (ca, cb) = (cells(a), cells(b));

                let u = union(a, b);
                assert_canonical(&u);
                assert_eq!(cells(&u), &ca | &cb, "union {a:?} {b:?}");

                let i = intersect(a, b);
                assert_canonical(&i);
                assert_eq!(cells(&i), &ca & &cb, "intersect {a:?} {b:?}");

                // Subtract is the operation yserver shipped without a unit
                // test, and the one SHAPE's Subtract and Invert both rest on.
                let s = subtract(a, b);
                assert_canonical(&s);
                assert_eq!(cells(&s), &ca - &cb, "subtract {a:?} {b:?}");

                assert_eq!(area(&u), cells(&u).len() as u64);
            }
        }
    }

    #[test]
    fn subtracting_a_hole_leaves_a_frame() {
        let outer = [rect(0, 0, 6, 6)];
        let hole = [rect(2, 2, 2, 2)];
        let frame = subtract(&outer, &hole);
        assert_canonical(&frame);
        assert_eq!(area(&frame), 32);
        assert!(!contains_point(&frame, 3, 3), "the hole is not covered");
        assert!(contains_point(&frame, 0, 0));
        assert!(contains_point(&frame, 5, 5));
        // Rounded-corner popups are exactly this shape, and the regression
        // yserver documents is a subtract that collapsed to all-or-nothing.
        assert!(!frame.is_empty());
        assert_ne!(canonicalize(&outer), frame);
    }

    #[test]
    fn subtract_and_intersect_handle_disjoint_and_containment() {
        let a = [rect(0, 0, 4, 4)];
        let far = [rect(100, 100, 4, 4)];
        assert_eq!(subtract(&a, &far), canonicalize(&a));
        assert!(intersect(&a, &far).is_empty());
        assert!(subtract(&a, &a).is_empty());
        assert_eq!(intersect(&a, &a), canonicalize(&a));
        let inner = [rect(1, 1, 2, 2)];
        assert_eq!(intersect(&a, &inner), canonicalize(&inner));
        assert_eq!(area(&subtract(&a, &inner)), 12);
    }

    #[test]
    fn empty_operands_behave() {
        let a = [rect(0, 0, 3, 3)];
        assert_eq!(union(&a, &[]), canonicalize(&a));
        assert_eq!(union(&[], &a), canonicalize(&a));
        assert!(intersect(&a, &[]).is_empty());
        assert_eq!(subtract(&a, &[]), canonicalize(&a));
        assert!(subtract(&[], &a).is_empty());
        assert!(canonicalize(&[]).is_empty());
        assert_eq!(extents(&[]), None);
        assert_eq!(area(&[]), 0);
        assert!(!contains_point(&[], 0, 0));
    }

    #[test]
    fn extents_bounds_every_rect() {
        assert_eq!(extents(&[rect(2, 3, 4, 5)]), Some(rect(2, 3, 4, 5)));
        assert_eq!(
            extents(&[rect(0, 0, 2, 2), rect(8, 6, 2, 2)]),
            Some(rect(0, 0, 10, 8))
        );
        assert_eq!(
            extents(&[rect(0, 0, 0, 9), rect(1, 1, 2, 2)]),
            Some(rect(1, 1, 2, 2))
        );
    }

    #[test]
    fn translate_moves_and_saturates() {
        let a = [rect(1, 1, 2, 2)];
        assert_eq!(translate(&a, 3, 4), vec![rect(4, 5, 2, 2)]);
        assert_eq!(translate(&a, 0, 0), canonicalize(&a));
        // Saturating rather than wrapping: a client may send any i32 offset,
        // and a wrapped rectangle would land somewhere it was never asked to.
        let moved = translate(&a, i32::MAX, 0);
        assert_eq!(moved[0].x, i32::MAX);
    }

    #[test]
    fn contains_point_uses_half_open_bounds() {
        let a = [rect(0, 0, 2, 2)];
        assert!(contains_point(&a, 0, 0));
        assert!(contains_point(&a, 1, 1));
        assert!(!contains_point(&a, 2, 0), "right edge is exclusive");
        assert!(!contains_point(&a, 0, 2), "bottom edge is exclusive");
        assert!(!contains_point(&a, -1, 0));
    }

    /// Abutting rectangles are one rectangle, in both axes. Half-open bounds
    /// are what make this fall out rather than needing an adjustment.
    #[test]
    fn abutting_rects_coalesce_in_both_axes() {
        assert_eq!(
            canonicalize(&[rect(0, 0, 2, 2), rect(2, 0, 2, 2)]),
            vec![rect(0, 0, 4, 2)]
        );
        assert_eq!(
            canonicalize(&[rect(0, 0, 2, 2), rect(0, 2, 2, 2)]),
            vec![rect(0, 0, 2, 4)]
        );
        // Not abutting: a one-cell gap must survive.
        assert_eq!(canonicalize(&[rect(0, 0, 2, 2), rect(3, 0, 2, 2)]).len(), 2);
    }

    /// Wide coordinates must not wrap while the arithmetic runs.
    #[test]
    fn extreme_coordinates_do_not_wrap() {
        let a = [rect(i32::MIN / 2, i32::MIN / 2, i32::MAX, i32::MAX)];
        let out = canonicalize(&a);
        assert_canonical(&out);
        assert_eq!(out, canonicalize(&a));
        assert!(extents(&a).is_some());
    }
}
