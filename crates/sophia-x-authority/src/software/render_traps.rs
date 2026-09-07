//! Coverage rasterisation for RENDER trapezoids and triangles.
//!
//! Adapted from yserver's `crates/yserver/src/kms/vk/ops/traps.rs` as it
//! stood at commit `5bf046b^` (MIT License, Copyright (c) 2026 Jos Dehaes).
//! The 4x4 supersampling scheme, the per-row x-range derivation clamped to
//! the trapezoid's own top and bottom, and the saturating-add union across
//! primitives are theirs. The fixed-point types, the rectangle bounds, and
//! the composite integration are Sophia's.
//!
//! GTK draws its client-side window decorations with these: the shadow under
//! a tooltip or a rounded corner is a trapezoid list composited through a
//! coverage mask. Refusing them ends the client, which is how this arrived --
//! Thunar reached `BadImplementation` on minor 10 in a live session, having
//! got past the startup refusal that came before it.

use sophia_protocol::Rect;

/// Subsamples per axis. Sixteen samples per pixel is what pixman's default
/// trapezoid quality works out to at ordinary sizes; the result is not
/// pixel-identical to pixman and is not meant to be.
const X_RENDER_SUBSAMPLES_PER_AXIS: i32 = 4;
const X_RENDER_SUBSAMPLES_TOTAL: i32 = X_RENDER_SUBSAMPLES_PER_AXIS * X_RENDER_SUBSAMPLES_PER_AXIS;

/// One trapezoid, in the 16.16 fixed point the wire carries.
///
/// The two sides are line segments rather than vertical edges, which is what
/// makes a trapezoid able to describe a rounded corner: `top` and `bottom`
/// bound it, and each side is sampled for its x at a given y.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XRenderTrapezoid {
    pub top: i32,
    pub bottom: i32,
    pub left_p1: (i32, i32),
    pub left_p2: (i32, i32),
    pub right_p1: (i32, i32),
    pub right_p2: (i32, i32),
}

/// One triangle, in the same fixed point.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XRenderTriangle {
    pub p1: (i32, i32),
    pub p2: (i32, i32),
    pub p3: (i32, i32),
}

fn fixed_to_f32(value: i32) -> f32 {
    value as f32 / 65536.0
}

/// One side of a trapezoid, and where it sits at a given row.
#[derive(Clone, Copy)]
struct XRenderEdge {
    p1: (f32, f32),
    p2: (f32, f32),
}

impl XRenderEdge {
    /// The x this edge occupies at a height. A horizontal edge has no single
    /// answer, so it reports its midpoint rather than dividing by zero.
    fn x_at(self, y: f32) -> f32 {
        let dy = self.p2.1 - self.p1.1;
        if dy.abs() < f32::EPSILON {
            (self.p1.0 + self.p2.0) * 0.5
        } else {
            let t = (y - self.p1.1) / dy;
            self.p1.0 + t * (self.p2.0 - self.p1.0)
        }
    }
}

/// The integer rectangle a set of points spans, or `None` when it is empty.
fn bounds_of(points: impl Iterator<Item = (f32, f32)>) -> Option<Rect> {
    let mut extremes: Option<(f32, f32, f32, f32)> = None;
    for (x, y) in points {
        if !x.is_finite() || !y.is_finite() {
            return None;
        }
        extremes = Some(match extremes {
            Some((left, top, right, bottom)) => {
                (left.min(x), top.min(y), right.max(x), bottom.max(y))
            }
            None => (x, y, x, y),
        });
    }
    let (left, top, right, bottom) = extremes?;
    let x = left.floor() as i32;
    let y = top.floor() as i32;
    let width = (right.ceil() as i32).saturating_sub(x);
    let height = (bottom.ceil() as i32).saturating_sub(y);
    (width > 0 && height > 0).then_some(Rect {
        x,
        y,
        width,
        height,
    })
}

pub(crate) fn trapezoid_bounds(traps: &[XRenderTrapezoid]) -> Option<Rect> {
    bounds_of(traps.iter().flat_map(|trap| {
        let top = fixed_to_f32(trap.top);
        let bottom = fixed_to_f32(trap.bottom);
        [
            (fixed_to_f32(trap.left_p1.0), top),
            (fixed_to_f32(trap.left_p2.0), bottom),
            (fixed_to_f32(trap.right_p1.0), top),
            (fixed_to_f32(trap.right_p2.0), bottom),
        ]
        .into_iter()
    }))
}

pub(crate) fn triangle_bounds(triangles: &[XRenderTriangle]) -> Option<Rect> {
    bounds_of(triangles.iter().flat_map(|triangle| {
        [triangle.p1, triangle.p2, triangle.p3]
            .into_iter()
            .map(|(x, y)| (fixed_to_f32(x), fixed_to_f32(y)))
    }))
}

/// The coverage a trapezoid list produces over `bounds`.
///
/// Overlapping primitives saturate rather than accumulating past full
/// coverage: the mask a trapezoid list describes is their union.
pub(crate) fn rasterize_trapezoids(traps: &[XRenderTrapezoid], bounds: Rect) -> Vec<u8> {
    let width = bounds.width.max(0) as usize;
    let height = bounds.height.max(0) as usize;
    let mut coverage = vec![0u8; width.saturating_mul(height)];
    if width == 0 || height == 0 {
        return coverage;
    }
    for trap in traps {
        let top = fixed_to_f32(trap.top);
        let bottom = fixed_to_f32(trap.bottom);
        if bottom <= top {
            continue;
        }
        let left = XRenderEdge {
            p1: (fixed_to_f32(trap.left_p1.0), fixed_to_f32(trap.left_p1.1)),
            p2: (fixed_to_f32(trap.left_p2.0), fixed_to_f32(trap.left_p2.1)),
        };
        let right = XRenderEdge {
            p1: (fixed_to_f32(trap.right_p1.0), fixed_to_f32(trap.right_p1.1)),
            p2: (fixed_to_f32(trap.right_p2.0), fixed_to_f32(trap.right_p2.1)),
        };
        let first_row = (top.floor() as i32).max(bounds.y);
        let last_row = (bottom.ceil() as i32).min(bounds.y.saturating_add(bounds.height));
        for y in first_row..last_row {
            let row = (y - bounds.y) as usize;
            // The sides are line segments, so x moves monotonically with y
            // along each. Sampling both sides at the part of the row the
            // trapezoid actually covers -- rather than at the whole row --
            // is what stops a partial first or last row extrapolating the
            // side lines and widening the span.
            let y_float = y as f32;
            let row_top = y_float.max(top);
            let row_bottom = (y_float + 1.0).min(bottom);
            let candidates = [
                left.x_at(row_top),
                left.x_at(row_bottom),
                right.x_at(row_top),
                right.x_at(row_bottom),
            ];
            let row_min = candidates.iter().copied().fold(f32::INFINITY, f32::min);
            let row_max = candidates.iter().copied().fold(f32::NEG_INFINITY, f32::max);
            if !row_min.is_finite() || !row_max.is_finite() {
                continue;
            }
            let first_column = (row_min.floor() as i32).max(bounds.x);
            let last_column = (row_max.ceil() as i32).min(bounds.x.saturating_add(bounds.width));
            for x in first_column..last_column {
                let mut hits = 0i32;
                for subsample_y in 0..X_RENDER_SUBSAMPLES_PER_AXIS {
                    let sample_y =
                        y_float + (subsample_y as f32 + 0.5) / X_RENDER_SUBSAMPLES_PER_AXIS as f32;
                    if sample_y < top || sample_y >= bottom {
                        continue;
                    }
                    let left_x = left.x_at(sample_y);
                    let right_x = right.x_at(sample_y);
                    if right_x <= left_x {
                        continue;
                    }
                    for subsample_x in 0..X_RENDER_SUBSAMPLES_PER_AXIS {
                        let sample_x = x as f32
                            + (subsample_x as f32 + 0.5) / X_RENDER_SUBSAMPLES_PER_AXIS as f32;
                        if sample_x >= left_x && sample_x < right_x {
                            hits += 1;
                        }
                    }
                }
                if hits == 0 {
                    continue;
                }
                let column = (x - bounds.x) as usize;
                let value = (hits * 255 / X_RENDER_SUBSAMPLES_TOTAL) as u8;
                if let Some(slot) = coverage.get_mut(row * width + column) {
                    *slot = slot.saturating_add(value);
                }
            }
        }
    }
    coverage
}

/// The coverage a triangle list produces over `bounds`, by the same
/// supersampling.
pub(crate) fn rasterize_triangles(triangles: &[XRenderTriangle], bounds: Rect) -> Vec<u8> {
    let width = bounds.width.max(0) as usize;
    let height = bounds.height.max(0) as usize;
    let mut coverage = vec![0u8; width.saturating_mul(height)];
    if width == 0 || height == 0 {
        return coverage;
    }
    for triangle in triangles {
        let vertices = [
            (fixed_to_f32(triangle.p1.0), fixed_to_f32(triangle.p1.1)),
            (fixed_to_f32(triangle.p2.0), fixed_to_f32(triangle.p2.1)),
            (fixed_to_f32(triangle.p3.0), fixed_to_f32(triangle.p3.1)),
        ];
        let Some(extent) = bounds_of(vertices.into_iter()) else {
            continue;
        };
        let first_row = extent.y.max(bounds.y);
        let last_row = extent
            .y
            .saturating_add(extent.height)
            .min(bounds.y.saturating_add(bounds.height));
        let first_column = extent.x.max(bounds.x);
        let last_column = extent
            .x
            .saturating_add(extent.width)
            .min(bounds.x.saturating_add(bounds.width));
        for y in first_row..last_row {
            for x in first_column..last_column {
                let mut hits = 0i32;
                for subsample_y in 0..X_RENDER_SUBSAMPLES_PER_AXIS {
                    let sample_y =
                        y as f32 + (subsample_y as f32 + 0.5) / X_RENDER_SUBSAMPLES_PER_AXIS as f32;
                    for subsample_x in 0..X_RENDER_SUBSAMPLES_PER_AXIS {
                        let sample_x = x as f32
                            + (subsample_x as f32 + 0.5) / X_RENDER_SUBSAMPLES_PER_AXIS as f32;
                        if point_in_triangle((sample_x, sample_y), vertices) {
                            hits += 1;
                        }
                    }
                }
                if hits == 0 {
                    continue;
                }
                let row = (y - bounds.y) as usize;
                let column = (x - bounds.x) as usize;
                let value = (hits * 255 / X_RENDER_SUBSAMPLES_TOTAL) as u8;
                if let Some(slot) = coverage.get_mut(row * width + column) {
                    *slot = slot.saturating_add(value);
                }
            }
        }
    }
    coverage
}

/// Whether a point falls inside a triangle, by consistent edge sign.
///
/// A degenerate triangle has no interior and covers nothing, which falls out
/// of every cross product being zero.
fn point_in_triangle(point: (f32, f32), vertices: [(f32, f32); 3]) -> bool {
    let cross = |a: (f32, f32), b: (f32, f32), p: (f32, f32)| {
        (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0)
    };
    let d1 = cross(vertices[0], vertices[1], point);
    let d2 = cross(vertices[1], vertices[2], point);
    let d3 = cross(vertices[2], vertices[0], point);
    let has_negative = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let has_positive = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
    !(has_negative && has_positive)
}
