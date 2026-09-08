#![cfg(feature = "gbm-probe")]

use sophia_backend_live::{LiveDmaBufImportFormat, common_dma_buf_import_formats};
use sophia_protocol::{DRM_FORMAT_ARGB8888 as AR, DRM_FORMAT_XRGB8888 as XR};

fn row(format: u32, modifiers: &[u64]) -> LiveDmaBufImportFormat {
    LiveDmaBufImportFormat {
        format,
        modifiers: modifiers.to_vec(),
    }
}

#[test]
fn one_device_preserves_measured_tiling_without_implicit_sentinels() {
    assert_eq!(
        common_dma_buf_import_formats(&[vec![
            row(XR, &[9, 0, 9, u64::MAX, 0x00ff_ffff_ffff_ffff]),
            row(XR, &[3]),
            row(AR, &[u64::MAX]),
        ]]),
        vec![row(XR, &[0, 3, 9])]
    );
}

#[test]
fn multiple_devices_require_measured_linear_support_on_each_device() {
    assert_eq!(
        common_dma_buf_import_formats(&[
            vec![row(XR, &[0, 9]), row(AR, &[0, 7])],
            vec![row(XR, &[0, 9]), row(AR, &[7])],
        ]),
        vec![row(XR, &[0])]
    );
    assert!(
        common_dma_buf_import_formats(&[vec![row(XR, &[9])], vec![row(XR, &[9])]]).is_empty(),
        "matching tiled modifier values do not prove a portable transfer"
    );
}

#[test]
fn absent_or_empty_device_evidence_never_invents_a_layout() {
    assert!(common_dma_buf_import_formats(&[]).is_empty());
    assert!(common_dma_buf_import_formats(&[Vec::new()]).is_empty());
    assert!(common_dma_buf_import_formats(&[vec![row(XR, &[0])], Vec::new()]).is_empty());
    assert!(common_dma_buf_import_formats(&[vec![row(XR, &[0])], vec![row(AR, &[0])]]).is_empty());
}
