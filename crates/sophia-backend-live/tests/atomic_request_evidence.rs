#![cfg(feature = "libdrm-events")]

use sophia_backend_live::*;
use sophia_protocol::Size;

#[path = "../src/drm/native_atomic/request/properties.rs"]
mod properties;

fn property(raw: u32) -> drm::control::property::Handle {
    drm::control::from_u32(raw).unwrap()
}

fn handles() -> LibdrmNativePrimaryPlanePropertyHandles {
    LibdrmNativePrimaryPlanePropertyHandles::new(
        property(101),
        property(102),
        property(103),
        property(104),
        property(105),
        property(106),
        property(107),
        property(108),
        property(109),
        property(110),
        property(111),
        property(112),
        property(113),
    )
    .with_crtc_vrr_enabled(Some(property(114)))
}

fn request(
    fb: u32,
    width: i32,
    mode: u64,
    policy: LibdrmNativePrimaryPlaneScanoutSubmitPolicy,
) -> LibdrmNativeAtomicCommitRequest {
    build_native_primary_plane_atomic_request_for_policy(
        LibdrmNativePrimaryPlaneObjects::new(
            drm::control::from_u32(5).unwrap(),
            drm::control::from_u32(4).unwrap(),
            drm::control::from_u32(3).unwrap(),
            drm::control::from_u32(fb).unwrap(),
            mode,
            Size { width, height: 720 },
        ),
        handles(),
        policy,
    )
    .request
    .unwrap()
}

fn cursor(x: i32, fb: u32) -> LibdrmNativeAtomicCursor {
    LibdrmNativeAtomicCursor {
        plane: drm::control::from_u32(2).unwrap(),
        properties: LibdrmNativeCursorPlanePropertyHandles::new(
            property(104),
            property(105),
            property(106),
            property(107),
            property(108),
            property(109),
            property(110),
            property(111),
            property(112),
            property(113),
        ),
        placement: Some(LibdrmNativeCursorPlacement {
            framebuffer: drm::control::from_u32(fb).unwrap(),
            x,
            y: -7,
            width: 64,
            height: 64,
        }),
    }
}

#[test]
fn primary_evidence_contains_the_actual_sorted_cursor_geometry_and_vrr_values() {
    let policy = LibdrmNativePrimaryPlaneScanoutSubmitPolicy::page_flip()
        .with_cursor(cursor(10, 12))
        .with_vrr_enabled(true);
    let evidence = request(11, 1280, 90, policy).evidence().unwrap();
    assert_eq!(evidence.primary_framebuffer(), (3, 104));
    let rows = evidence.properties();
    assert_eq!(rows.len(), 21);
    assert!(
        rows.windows(2)
            .all(|pair| (pair[0].object, pair[0].property) < (pair[1].object, pair[1].property))
    );
    let value = |object, property| {
        rows.iter()
            .find(|row| row.object == object && row.property == property)
            .unwrap()
            .value
    };
    assert_eq!(value(3, 104), 11);
    assert_eq!(value(3, 108), 1280 << 16);
    assert_eq!(value(3, 112), 1280);
    assert_eq!(value(2, 104), 12);
    assert_eq!(value(2, 110), 10);
    assert_eq!(value(2, 111), (-7i64) as u64);
    assert_eq!(value(4, 114), 1);
}

#[test]
fn only_a_different_primary_framebuffer_may_differ_in_equivalent_evidence() {
    let policy = LibdrmNativePrimaryPlaneScanoutSubmitPolicy::page_flip()
        .with_cursor(cursor(10, 12))
        .with_vrr_enabled(true);
    let a = request(11, 1280, 90, policy).evidence().unwrap();
    let b = request(21, 1280, 90, policy).evidence().unwrap();
    assert!(a.equivalent_except_primary_framebuffer(&b));
    assert!(!a.equivalent_except_primary_framebuffer(&a));
    for changed in [
        request(21, 1279, 90, policy),
        request(21, 1280, 90, policy.with_cursor(cursor(11, 12))),
        request(21, 1280, 90, policy.with_cursor(cursor(10, 13))),
        request(21, 1280, 90, policy.with_vrr_enabled(false)),
        request(
            21,
            1280,
            90,
            LibdrmNativePrimaryPlaneScanoutSubmitPolicy::page_flip(),
        ),
        request(21, 1280, 90, policy).blocking(),
        request(21, 1280, 90, policy).without_page_flip_event(),
        request(21, 1280, 90, policy).allow_modeset(),
        request(21, 1280, 90, policy).test_only(),
    ] {
        assert!(!a.equivalent_except_primary_framebuffer(&changed.evidence().unwrap()));
    }
    let modeset = LibdrmNativePrimaryPlaneScanoutSubmitPolicy::modeset();
    let a = request(11, 1280, 90, modeset).evidence().unwrap();
    assert!(!a.equivalent_except_primary_framebuffer(
        &request(21, 1280, 91, modeset).evidence().unwrap()
    ));
    assert!(a.equivalent_except_primary_framebuffer(
        &request(21, 1280, 90, modeset).evidence().unwrap()
    ));
}

#[test]
fn raw_requests_have_no_evidence_and_flag_transforms_are_observed() {
    assert!(
        LibdrmNativeAtomicCommitRequest::new(Default::default())
            .evidence()
            .is_none()
    );
    assert!(
        LibdrmNativeAtomicCommitRequest::modeset(Default::default())
            .evidence()
            .is_none()
    );
    let request = request(
        11,
        1280,
        90,
        LibdrmNativePrimaryPlaneScanoutSubmitPolicy::page_flip(),
    )
    .test_only();
    let evidence = request.evidence().unwrap();
    assert_eq!(evidence.flags, request.reduced_flags());
    assert!(evidence.flags.test_only);
    assert!(!evidence.flags.page_flip_event);
}

#[test]
fn collector_orders_keys_replaces_duplicates_and_refuses_truncation() {
    use properties::{CanonicalAtomicProperties, LibdrmNativeAtomicProperty as Row};
    let mut properties = CanonicalAtomicProperties::new();
    for property in (1..=32).rev() {
        assert!(properties.insert(Row {
            object: 7,
            property,
            value: u64::from(property)
        }));
    }
    assert!(properties.insert(Row {
        object: 7,
        property: 16,
        value: 999
    }));
    assert_eq!(properties.as_slice().len(), 32);
    assert_eq!(properties.as_slice()[15].value, 999);
    assert!(!properties.insert(Row {
        object: 8,
        property: 1,
        value: 1
    }));
    assert_eq!(properties.as_slice()[31].property, 32);
}
