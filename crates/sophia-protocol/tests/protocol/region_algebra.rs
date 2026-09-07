// Region set algebra, exercised through the public API.
//
// The operations are compared against a brute-force cell model over every
// pair in a small grid: interval algebra fails at edges -- abutting spans,
// empty slices, one operand inside the other -- and enumerating the
// neighbourhood catches what hand-picked cases miss.

use sophia_protocol::geometry::region_algebra::*;


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
