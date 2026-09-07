//! Solid fills and gradients: pictures that generate their pixels rather
//! than reading them from a drawable.
//!
//! The stop interpolation follows yserver's `crates/yserver/src/kms/vk/
//! gradient.rs` (MIT License, Copyright (c) 2026 Jos Dehaes), in particular
//! the asymmetry that is easiest to get wrong: `CreateSolidFill` carries a
//! colour that is already premultiplied on the wire, while gradient stops
//! carry straight alpha and must be interpolated straight and premultiplied
//! afterwards. Interpolating premultiplied stops darkens every gradient that
//! fades to transparent.
//!
//! Cairo paints widget backgrounds with these, and sends them without asking
//! what version the server offers -- which is how they arrived here.

use super::render_ops::XRenderRepeat;

/// One stop: where it sits on the gradient's parameter, and its colour in
/// the straight-alpha 16-bit channels the wire carries.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XRenderGradientStop {
    /// 16.16 fixed point, nominally within [0, 1].
    pub position: i32,
    /// Red, green, blue, alpha, straight rather than premultiplied.
    pub color: [u16; 4],
}

/// The shape a gradient's parameter is derived from.
///
/// Held in the 16.16 fixed point the wire carries, so a decoded request
/// compares equal to itself and the geometry a client sent is the geometry
/// stored. The conversion to floating point happens where the arithmetic
/// does.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum XRenderGradientGeometry {
    /// Distance along the line from `p1` to `p2`.
    Linear { p1: (i32, i32), p2: (i32, i32) },
    /// The larger root of the two-circle interpolation.
    Radial {
        inner: (i32, i32),
        inner_radius: i32,
        outer: (i32, i32),
        outer_radius: i32,
    },
    /// Angle about a centre, in degrees, starting from a given rotation.
    Conical { center: (i32, i32), angle: i32 },
}

/// 16.16 fixed point as a real number.
fn fixed(value: i32) -> f64 {
    f64::from(value) / 65536.0
}

/// A picture that computes its own pixels.
#[derive(Clone, Debug, PartialEq)]
pub enum XRenderGeneratedSource {
    /// Already premultiplied on the wire, and stored exactly as received.
    Solid([u8; 4]),
    Gradient {
        geometry: XRenderGradientGeometry,
        stops: Vec<XRenderGradientStop>,
    },
}

impl XRenderGeneratedSource {
    /// The premultiplied sample at a point in the picture's own space.
    pub(super) fn sample(&self, x: f64, y: f64, repeat: XRenderRepeat) -> [u8; 4] {
        match self {
            Self::Solid(color) => *color,
            Self::Gradient { geometry, stops } => {
                let Some(t) = geometry.parameter(x, y) else {
                    return [0; 4];
                };
                let Some(t) = repeat.wrap_parameter(t) else {
                    return [0; 4];
                };
                sample_stops(stops, t)
            }
        }
    }
}

impl XRenderGradientGeometry {
    /// The gradient's parameter at a point, before repeat is applied, or
    /// `None` where the gradient does not reach.
    fn parameter(&self, x: f64, y: f64) -> Option<f64> {
        match *self {
            Self::Linear { p1, p2 } => {
                let p1 = (fixed(p1.0), fixed(p1.1));
                let p2 = (fixed(p2.0), fixed(p2.1));
                let (vx, vy) = (p2.0 - p1.0, p2.1 - p1.1);
                let length_squared = vx * vx + vy * vy;
                // A gradient between two coincident points has no direction
                // to run along; the protocol leaves it undefined and drawing
                // nothing is the answer that cannot be mistaken for content.
                if length_squared <= f64::EPSILON {
                    return None;
                }
                Some(((x - p1.0) * vx + (y - p1.1) * vy) / length_squared)
            }
            Self::Radial {
                inner,
                inner_radius,
                outer,
                outer_radius,
            } => {
                // The circle at parameter t is centred between the two and
                // has the interpolated radius; the parameter wanted is the
                // largest t whose circle passes through the point.
                let inner = (fixed(inner.0), fixed(inner.1));
                let outer = (fixed(outer.0), fixed(outer.1));
                let inner_radius = fixed(inner_radius);
                let outer_radius = fixed(outer_radius);
                let cdx = outer.0 - inner.0;
                let cdy = outer.1 - inner.1;
                let dr = outer_radius - inner_radius;
                let pdx = x - inner.0;
                let pdy = y - inner.1;
                let a = cdx * cdx + cdy * cdy - dr * dr;
                let b = pdx * cdx + pdy * cdy + inner_radius * dr;
                let c = pdx * pdx + pdy * pdy - inner_radius * inner_radius;
                let t = if a.abs() < 1e-9 {
                    // Concentric circles of equal radius growth degenerate to
                    // a linear relation rather than a quadratic.
                    if b.abs() < 1e-9 {
                        return None;
                    }
                    c / (2.0 * b)
                } else {
                    let discriminant = b * b - a * c;
                    if discriminant < 0.0 {
                        return None;
                    }
                    (b + discriminant.sqrt()) / a
                };
                // A circle of negative radius does not exist, so neither does
                // the point on it.
                (inner_radius + t * dr >= 0.0).then_some(t)
            }
            Self::Conical { center, angle } => {
                let center = (fixed(center.0), fixed(center.1));
                // The wire carries degrees; the arithmetic wants radians.
                let angle = fixed(angle).to_radians();
                let dx = x - center.0;
                let dy = y - center.1;
                if dx == 0.0 && dy == 0.0 {
                    return Some(0.0);
                }
                let theta = dy.atan2(dx) - angle;
                Some(theta / std::f64::consts::TAU)
            }
        }
    }
}

/// The premultiplied colour a gradient shows at a parameter.
///
/// Interpolation happens in straight alpha and premultiplies afterwards,
/// which is what keeps a fade to transparent from darkening as it goes.
fn sample_stops(stops: &[XRenderGradientStop], t: f64) -> [u8; 4] {
    let Some(first) = stops.first() else {
        return [0; 4];
    };
    let position = |stop: &XRenderGradientStop| f64::from(stop.position) / 65536.0;
    if t <= position(first) {
        return premultiply(first.color);
    }
    let Some(last) = stops.last() else {
        return [0; 4];
    };
    if t >= position(last) {
        return premultiply(last.color);
    }
    for pair in stops.windows(2) {
        let (low, high) = (pair[0], pair[1]);
        let (low_position, high_position) = (position(&low), position(&high));
        if t < low_position || t > high_position {
            continue;
        }
        let span = high_position - low_position;
        // Two stops at one position are a hard edge, and the later one wins.
        let ratio = if span <= f64::EPSILON {
            1.0
        } else {
            (t - low_position) / span
        };
        let mut blended = [0u16; 4];
        for (channel, slot) in blended.iter_mut().enumerate() {
            let a = f64::from(low.color[channel]);
            let b = f64::from(high.color[channel]);
            *slot = (a + (b - a) * ratio).round().clamp(0.0, 65535.0) as u16;
        }
        return premultiply(blended);
    }
    premultiply(last.color)
}

/// Straight 16-bit channels to premultiplied bytes, in the store's order.
fn premultiply(color: [u16; 4]) -> [u8; 4] {
    let alpha = u32::from(color[3]);
    let scale = |channel: u16| -> u8 {
        let value = u32::from(channel) * alpha / 65535;
        (value >> 8) as u8
    };
    [
        scale(color[2]),
        scale(color[1]),
        scale(color[0]),
        (alpha >> 8) as u8,
    ]
}
