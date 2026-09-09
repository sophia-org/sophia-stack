#![cfg(feature = "libdrm-events")]

use drm::buffer::{DrmFourcc as Format, DrmModifier as Modifier};
use sophia_backend_live::{
    LibdrmNativePlaneFormatCapabilities as Capabilities, LibdrmNativePlaneFormatModifierTable,
    LibdrmNativePlaneFormatSnapshot as Snapshot, LibdrmNativePlaneFormatSnapshotUnknown as Unknown,
    LibdrmNativePlaneFormatSupport as Support,
};

fn blob(formats: &[Format], modifiers: &[(u64, u32, Modifier)]) -> Vec<u8> {
    let modifiers_offset = (24 + formats.len() * 4).next_multiple_of(8);
    let mut bytes = vec![0; modifiers_offset + modifiers.len() * 24];
    put(&mut bytes, 0, 1);
    put(&mut bytes, 8, formats.len() as u32);
    put(&mut bytes, 12, 24);
    put(&mut bytes, 16, modifiers.len() as u32);
    put(&mut bytes, 20, modifiers_offset as u32);
    for (index, format) in formats.iter().enumerate() {
        put(&mut bytes, 24 + index * 4, *format as u32);
    }
    for (index, (mask, offset, modifier)) in modifiers.iter().enumerate() {
        let base = modifiers_offset + index * 24;
        bytes[base..base + 8].copy_from_slice(&mask.to_ne_bytes());
        put(&mut bytes, base + 8, *offset);
        bytes[base + 16..base + 24].copy_from_slice(&u64::from(*modifier).to_ne_bytes());
    }
    bytes
}

fn put(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_ne_bytes());
}

fn assert_unknown(bytes: &[u8], reason: Unknown) {
    let snapshot = Snapshot::parse(13, 77, bytes);
    assert_eq!(snapshot.plane, 13);
    assert_eq!(snapshot.blob_id, Some(77));
    assert_eq!(snapshot.unknown_reason(), Some(reason));
    for format in [Format::Xrgb8888, Format::Argb8888] {
        assert_eq!(snapshot.modifiers(format), None);
        assert_eq!(snapshot.support(format, Modifier::Linear), Support::Unknown);
    }
}

#[test]
fn exact_formats_keep_distinct_ordered_modifier_sets() {
    let tiled = Modifier::from(0x0200_0000_0040_1b03);
    let bytes = blob(
        &[Format::Xrgb8888, Format::Argb8888],
        &[(1, 0, tiled), (3, 0, Modifier::Linear), (1, 0, tiled)],
    );
    let snapshot = Snapshot::parse(13, 77, &bytes);
    assert_eq!(snapshot.unknown_reason(), None);
    assert_eq!(
        snapshot.modifiers(Format::Xrgb8888),
        Some([tiled, Modifier::Linear].as_slice())
    );
    assert_eq!(
        snapshot.modifiers(Format::Argb8888),
        Some([Modifier::Linear].as_slice())
    );
    assert_eq!(
        snapshot.support(Format::Xrgb8888, tiled),
        Support::Supported
    );
    assert_eq!(
        snapshot.support(Format::Argb8888, tiled),
        Support::Unsupported
    );
    assert_eq!(snapshot.support(Format::Rgb565, tiled), Support::Unknown);
    assert_eq!(
        snapshot.support(Format::Xrgb8888, Modifier::Invalid),
        Support::Unknown
    );
    let legacy = LibdrmNativePlaneFormatModifierTable::parse_for_format(&bytes, Format::Xrgb8888)
        .table
        .unwrap();
    assert_eq!(
        snapshot.modifiers(Format::Xrgb8888),
        Some(legacy.modifiers())
    );
}

#[test]
fn known_absent_format_and_known_empty_format_are_not_unknown() {
    for bytes in [blob(&[], &[]), blob(&[Format::Xrgb8888], &[])] {
        let snapshot = Snapshot::parse(13, 77, &bytes);
        assert_eq!(snapshot.unknown_reason(), None);
        for format in [Format::Xrgb8888, Format::Argb8888] {
            assert_eq!(snapshot.modifiers(format), Some([].as_slice()));
            assert_eq!(
                snapshot.support(format, Modifier::Linear),
                Support::Unsupported
            );
        }
    }
    for reason in [Unknown::Unavailable, Unknown::ReadFailed] {
        let snapshot = Snapshot::unavailable(13, reason);
        assert_eq!(snapshot.blob_id, None);
        assert_eq!(snapshot.unknown_reason(), Some(reason));
        assert_eq!(
            snapshot.support(Format::Xrgb8888, Modifier::Linear),
            Support::Unknown
        );
    }
}

#[test]
fn implicit_records_never_create_negative_explicit_evidence() {
    let bytes = blob(
        &[Format::Xrgb8888, Format::Argb8888],
        &[(1, 0, Modifier::Linear), (2, 0, Modifier::Invalid)],
    );
    assert_unknown(&bytes, Unknown::ImplicitModifier);
    // Allocation preferences retain their existing permissive reduction.
    let legacy = LibdrmNativePlaneFormatModifierTable::parse_for_format(&bytes, Format::Xrgb8888)
        .table
        .unwrap();
    assert_eq!(legacy.modifiers(), &[Modifier::Linear]);
}

#[test]
fn malformed_extra_records_and_table_extents_never_prove_absence() {
    let valid = blob(&[Format::Xrgb8888], &[(1, 0, Modifier::Linear)]);
    let modifier_offset = u32::from_ne_bytes(valid[20..24].try_into().unwrap()) as usize;
    let mutations = [
        (4, 1),   // Unknown header flags.
        (12, 0),  // Formats overlap the header.
        (12, 25), // Misaligned formats.
        (20, 24), // Modifier records overlap formats.
        (20, u32::MAX),
        (modifier_offset + 8, u32::MAX),
        (modifier_offset + 12, 1), // Reserved record padding.
    ];
    for (offset, value) in mutations {
        let mut bytes = valid.clone();
        put(&mut bytes, offset, value);
        assert_unknown(&bytes, Unknown::Malformed);
    }
    assert_unknown(&valid[..valid.len() - 1], Unknown::Malformed);
    assert_unknown(
        &blob(&[Format::Xrgb8888], &[(2, 0, Modifier::Linear)]),
        Unknown::Malformed,
    );
    assert_unknown(
        &blob(&[Format::Xrgb8888, Format::Xrgb8888], &[]),
        Unknown::Malformed,
    );
    // The requested format being absent does not excuse a malformed other record.
    assert_unknown(
        &blob(&[Format::Rgb565], &[(2, 0, Modifier::Linear)]),
        Unknown::Malformed,
    );
}

#[test]
fn unknown_versions_and_capacity_exhaustion_never_publish_partial_sets() {
    let mut version = blob(&[Format::Xrgb8888], &[]);
    put(&mut version, 0, 2);
    assert_unknown(&version, Unknown::UnsupportedVersion);
    assert_unknown(&vec![0; 64 * 1024 + 1], Unknown::CapacityExceeded);
    for offset in [8, 16] {
        let mut count = blob(&[], &[]);
        put(&mut count, offset, 257);
        assert_unknown(&count, Unknown::CapacityExceeded);
    }
    let modifiers: Vec<_> = (0..65).map(|value| (1, 0, Modifier::from(value))).collect();
    let bytes = blob(&[Format::Xrgb8888], &modifiers);
    assert_unknown(&bytes, Unknown::CapacityExceeded);
    let boundary = Snapshot::parse(13, 77, &blob(&[Format::Xrgb8888], &modifiers[..64]));
    assert_eq!(boundary.unknown_reason(), None);
    assert_eq!(boundary.modifiers(Format::Xrgb8888).unwrap().len(), 64);
}

#[test]
fn one_blob_keeps_legacy_preferences_when_strict_evidence_is_unknown() {
    let explicit = Modifier::from(0x0200_0000_0040_1b03);
    let bytes = blob(
        &[Format::Xrgb8888],
        &[
            (1, 0, explicit),
            (1, 0, Modifier::Invalid),
            (1, 0, Modifier::Linear),
        ],
    );
    let capabilities = Capabilities::parse(13, 77, &bytes);
    assert_eq!(
        capabilities.snapshot.unknown_reason(),
        Some(Unknown::ImplicitModifier)
    );
    assert_eq!(
        capabilities.preferred_xrgb8888_modifiers,
        vec![u64::from(explicit), 0]
    );
    assert_eq!(
        capabilities.snapshot.support(Format::Xrgb8888, explicit),
        Support::Unknown
    );
    for reason in [Unknown::Unavailable, Unknown::ReadFailed] {
        let missing = Capabilities::unavailable(13, reason);
        assert!(missing.preferred_xrgb8888_modifiers.is_empty());
        assert_eq!(missing.snapshot.unknown_reason(), Some(reason));
    }
}

#[test]
fn modifier_bit_windows_are_relative_to_each_record_offset() {
    let tiled = Modifier::from(0x0200_0000_0040_1b03);
    let mut bytes = blob(
        &[Format::Rgb565; 65],
        &[(1, 0, Modifier::Linear), (1, 64, tiled)],
    );
    for index in 0..65 {
        put(&mut bytes, 24 + index * 4, 0x1000_0000 + index as u32);
    }
    put(&mut bytes, 24, Format::Xrgb8888 as u32);
    put(&mut bytes, 24 + 64 * 4, Format::Argb8888 as u32);
    let snapshot = Snapshot::parse(13, 77, &bytes);
    assert_eq!(snapshot.unknown_reason(), None);
    assert_eq!(
        snapshot.modifiers(Format::Xrgb8888),
        Some([Modifier::Linear].as_slice())
    );
    assert_eq!(
        snapshot.modifiers(Format::Argb8888),
        Some([tiled].as_slice())
    );
    assert_eq!(
        snapshot.support(Format::Xrgb8888, tiled),
        Support::Unsupported
    );
    assert_eq!(
        snapshot.support(Format::Argb8888, Modifier::Linear),
        Support::Unsupported
    );
}
