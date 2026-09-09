#![cfg(test)]

use super::*;
use crate::{
    LibdrmNativePlaneFormatCapabilities as Capabilities,
    LibdrmNativePlaneFormatSnapshot as Snapshot, LibdrmNativePlaneFormatSnapshotUnknown as Unknown,
};
use sophia_protocol::{DRM_FORMAT_ARGB8888 as AR24, DRM_FORMAT_XRGB8888 as XR24};

fn blob(modifiers: &[(u64, u64)]) -> Vec<u8> {
    let mut bytes = vec![0; 32 + modifiers.len() * 24];
    for (offset, value) in [
        (0, 1u32),
        (8, 2),
        (12, 24),
        (16, modifiers.len() as u32),
        (20, 32),
        (24, XR24),
        (28, AR24),
    ] {
        bytes[offset..offset + 4].copy_from_slice(&value.to_ne_bytes());
    }
    for (index, (mask, modifier)) in modifiers.iter().enumerate() {
        let offset = 32 + index * 24;
        bytes[offset..offset + 8].copy_from_slice(&mask.to_ne_bytes());
        bytes[offset + 16..offset + 24].copy_from_slice(&modifier.to_ne_bytes());
    }
    bytes
}

fn preference(format: u32, modifiers: Vec<u64>) -> LiveOutputAllocationFormatPreference {
    LiveOutputAllocationFormatPreference { format, modifiers }
}

#[test]
fn exact_format_preferences_preserve_distinct_modifier_order() {
    let xrgb_tiled = 0x0200_0000_28a0_1f04;
    let argb_tiled = 0x0200_0000_0040_1b03;
    let bytes = blob(&[(1, xrgb_tiled), (2, argb_tiled), (3, 0), (1, xrgb_tiled)]);
    let capabilities = Capabilities::parse(13, 77, &bytes);
    let legacy = capabilities.preferred_xrgb8888_modifiers.clone();
    let rows = allocation_format_preferences(&capabilities.snapshot);
    assert_eq!(
        rows,
        vec![
            preference(XR24, vec![xrgb_tiled, 0]),
            preference(AR24, vec![argb_tiled, 0]),
        ]
    );
    assert_eq!(legacy, rows[0].modifiers);
    assert_eq!(capabilities.preferred_xrgb8888_modifiers, legacy);
}

#[test]
fn a_format_without_explicit_layouts_has_no_preference_row() {
    for (mask, format) in [(1, XR24), (2, AR24)] {
        let snapshot = Snapshot::parse(13, 77, &blob(&[(mask, 0)]));
        assert_eq!(
            allocation_format_preferences(&snapshot),
            vec![preference(format, vec![0])]
        );
    }
    let snapshot = Snapshot::parse(13, 77, &blob(&[]));
    assert!(allocation_format_preferences(&snapshot).is_empty());
}

#[test]
fn unknown_preferences_do_not_change_legacy_renderer_admission() {
    for reason in [
        Unknown::Unavailable,
        Unknown::ReadFailed,
        Unknown::Malformed,
        Unknown::UnsupportedVersion,
        Unknown::CapacityExceeded,
        Unknown::ImplicitModifier,
    ] {
        assert!(allocation_format_preferences(&Snapshot::unavailable(13, reason)).is_empty());
    }

    // Reserved flags invalidate strict evidence without changing the legacy parser.
    let mut bytes = blob(&[(3, 0)]);
    bytes[4..8].copy_from_slice(&1u32.to_ne_bytes());
    let capabilities = Capabilities::parse(13, 77, &bytes);
    assert_eq!(
        capabilities.snapshot.unknown_reason(),
        Some(Unknown::Malformed)
    );
    assert!(allocation_format_preferences(&capabilities.snapshot).is_empty());
    assert_eq!(capabilities.preferred_xrgb8888_modifiers, vec![0]);
}

#[test]
fn invalid_sentinels_never_become_allocation_preferences() {
    let snapshot = Snapshot::parse(13, 77, &blob(&[(3, u64::MAX), (3, 0)]));
    assert_eq!(
        allocation_format_preferences(&snapshot),
        vec![preference(XR24, vec![0]), preference(AR24, vec![0])]
    );
    let snapshot = Snapshot::parse(13, 77, &blob(&[(3, u64::MAX)]));
    assert!(allocation_format_preferences(&snapshot).is_empty());

    let snapshot = Snapshot::parse(
        13,
        77,
        &blob(&[(3, sophia_protocol::DRM_FORMAT_MOD_INVALID), (3, 0)]),
    );
    assert_eq!(snapshot.unknown_reason(), Some(Unknown::ImplicitModifier));
    assert!(allocation_format_preferences(&snapshot).is_empty());
}
