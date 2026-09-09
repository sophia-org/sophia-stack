#![cfg(all(feature = "gbm-platform", target_os = "linux"))]

use std::{fs::OpenOptions, os::fd::AsFd};

use sophia_renderer_native_egl::{
    NativeCompositionFormatRequest, NativeCompositionFrame, NativeCompositionLayer,
    NativeCompositionOutputRequest, NativeCompositionRect, NativeDmaBufPlane,
    NativeFrameTargetSetId, NativeGbmRenderedScanoutContext, NativeGbmScanoutBufferExportDetail,
    NativeMultiPlaneDmaBufFrame, NativePixmapImportProbe, NativeSolidCompositionLayer,
};

#[path = "../src/gbm_platform/config.rs"]
mod config;
use config::{window_config_attributes, xrgb_window_config_attributes};
#[path = "../src/gbm_platform/scanout/output_candidates.rs"]
mod output_candidates;
use output_candidates::rendered_scanout_candidates;
#[path = "../src/gbm_platform/scanout/output_format.rs"]
mod output_format;
use output_format::CompositionFormatAdmission;

const XR24: u32 = 0x3432_5258;
const AR24: u32 = 0x3432_5241;

#[test]
fn an_absent_format_preserves_candidate_order_and_allocation_attributes() {
    let usage = gbm::BufferObjectFlags::SCANOUT | gbm::BufferObjectFlags::RENDERING;
    let linear_usage = usage | gbm::BufferObjectFlags::LINEAR;
    let preferred = [gbm::Modifier::from(0x0200_0000_0040_1b03)];
    for modifiers in [&[][..], &preferred[..]] {
        let actual = rendered_scanout_candidates(modifiers, None)
            .into_iter()
            .map(|candidate| {
                (
                    candidate.format as u32,
                    candidate.modifiers,
                    candidate.usage,
                    candidate.config_attributes,
                )
            })
            .collect::<Vec<_>>();
        let xrgb = xrgb_window_config_attributes();
        let argb = window_config_attributes();
        let mut expected = vec![(XR24, vec![gbm::Modifier::Linear], linear_usage, xrgb)];
        if !modifiers.is_empty() {
            expected.push((XR24, modifiers.to_vec(), usage, xrgb));
        }
        expected.extend([
            (XR24, vec![], linear_usage, xrgb),
            (XR24, vec![], usage, xrgb),
            (AR24, vec![gbm::Modifier::Linear], linear_usage, argb),
            (AR24, vec![], usage, argb),
        ]);
        assert_eq!(actual, expected);
    }
}

#[test]
fn a_requested_format_never_admits_the_other_formats_candidates() {
    let preferred = [gbm::Modifier::from(0x0200_0000_0040_1b03)];
    for format in [AR24, XR24] {
        let actual = rendered_scanout_candidates(&preferred, Some(format));
        assert_eq!(actual.len(), if format == AR24 { 2 } else { 4 });
        for candidate in actual {
            assert_eq!(candidate.format as u32, format);
            assert_eq!(
                candidate.config_attributes[11],
                if format == AR24 { 8 } else { 0 }
            );
        }
    }
    assert!(rendered_scanout_candidates(&preferred, Some(0x3231_564e)).is_empty());
}

#[test]
fn unknown_output_formats_are_invalid_requests() {
    assert!(NativeCompositionOutputRequest::default().is_valid());
    for constructor in [
        NativeCompositionFormatRequest::Required,
        NativeCompositionFormatRequest::Preferred,
    ] {
        for format in [XR24, AR24] {
            assert!(
                NativeCompositionOutputRequest {
                    preferred_modifiers: &[],
                    format: Some(constructor(format))
                }
                .is_valid()
            );
        }
        for format in [0, 0x3231_564e, u32::MAX] {
            assert!(
                !NativeCompositionOutputRequest {
                    preferred_modifiers: &[],
                    format: Some(constructor(format))
                }
                .is_valid()
            );
        }
    }
}

#[test]
fn refused_preferred_targets_admit_the_normal_order_once_before_drawing() {
    let mut admission =
        CompositionFormatAdmission::new(Some(NativeCompositionFormatRequest::Preferred(AR24)));
    let preferred = rendered_scanout_candidates(&[], admission.format());
    assert_eq!(preferred.len(), 2);
    assert!(
        preferred
            .iter()
            .all(|candidate| candidate.format as u32 == AR24)
    );
    for _ in preferred {
        admission.target_refused(NativeGbmScanoutBufferExportDetail::GbmSurfaceUnavailable);
    }
    assert!(admission.relax());
    assert_eq!(admission.format(), None);
    let normal = rendered_scanout_candidates(&[], admission.format());
    assert_eq!(normal.len(), 5);
    assert_eq!(normal[0].format as u32, XR24);
    admission.drawing_started();
    assert!(!admission.relax());
    assert_eq!(admission.format(), None);
}

#[test]
fn required_formats_and_started_draws_cannot_relax_even_during_recovery() {
    let mut required =
        CompositionFormatAdmission::new(Some(NativeCompositionFormatRequest::Required(AR24)));
    required.target_refused(NativeGbmScanoutBufferExportDetail::EglConfigUnavailable);
    assert!(!required.relax());
    assert_eq!(required.format(), Some(AR24));

    let mut preferred =
        CompositionFormatAdmission::new(Some(NativeCompositionFormatRequest::Preferred(AR24)));
    preferred.drawing_started();
    for detail in [
        NativeGbmScanoutBufferExportDetail::FrontBufferLockFailed,
        NativeGbmScanoutBufferExportDetail::EglSwapBuffersFailed,
        NativeGbmScanoutBufferExportDetail::GbmSurfaceUnavailable,
    ] {
        preferred.target_refused(detail);
        assert!(!preferred.relax());
        assert_eq!(preferred.format(), Some(AR24));
    }
}

#[test]
fn context_and_device_failures_do_not_become_format_preferences() {
    for detail in [
        NativeGbmScanoutBufferExportDetail::EglContextUnavailable,
        NativeGbmScanoutBufferExportDetail::EglMakeCurrentFailed,
        NativeGbmScanoutBufferExportDetail::GlSmokeFailed,
        NativeGbmScanoutBufferExportDetail::BackendDeviceUnavailable,
    ] {
        let mut admission =
            CompositionFormatAdmission::new(Some(NativeCompositionFormatRequest::Preferred(AR24)));
        admission.target_refused(detail);
        admission.target_refused(NativeGbmScanoutBufferExportDetail::EglConfigUnavailable);
        assert!(!admission.relax());
        assert_eq!(admission.format(), Some(AR24));
    }
}

#[test]
#[ignore = "requires SOPHIA_TEST_RENDER_NODE; offscreen format allocation and exact exported pixel proof"]
fn requested_formats_rebuild_only_the_matching_target_and_export_real_pixels() {
    let path = std::env::var_os("SOPHIA_TEST_RENDER_NODE").expect("select a DRM render node");
    let open = || {
        OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .expect("open render node")
    };
    let created = NativeGbmRenderedScanoutContext::from_backend_device_result(Ok(open()));
    let mut context = created
        .context
        .unwrap_or_else(|| panic!("native context: {:?}", created.status));
    let layers = [NativeCompositionLayer::Solid(NativeSolidCompositionLayer {
        target: NativeCompositionRect {
            x: 0,
            y: 0,
            width: 32,
            height: 16,
        },
        color: [37, 91, 163],
    })];
    let frame = NativeCompositionFrame {
        width: 32,
        height: 16,
        layers: &layers,
        trace: None,
        repaint: None,
    };
    for slot in [None, Some((NativeFrameTargetSetId::from_raw(41), 1))] {
        let mut previous_generation = None;
        let mut previous_format = None;
        for (index, format) in [
            None,
            Some(XR24),
            None,
            Some(AR24),
            Some(AR24),
            None,
            Some(XR24),
            None,
            None,
        ]
        .into_iter()
        .enumerate()
        {
            let request = NativeCompositionOutputRequest {
                preferred_modifiers: &[0],
                format: format.map(if index == 4 {
                    NativeCompositionFormatRequest::Preferred
                } else {
                    NativeCompositionFormatRequest::Required
                }),
            };
            let before = context.persistent_render_stats();
            let report = match slot {
                None if index == 8 => {
                    context.export_composed_owned_scanout_buffer_with_modifiers(frame, &[0])
                }
                Some((set, slot)) if index == 8 => context
                    .export_composed_owned_scanout_buffer_with_modifiers_in_frame_slot(
                        set,
                        slot,
                        frame,
                        &[0],
                    ),
                None => context.export_composed_owned_scanout_buffer(frame, request),
                Some((set, slot)) => context
                    .export_composed_owned_scanout_buffer_in_frame_slot(set, slot, frame, request),
            };
            let generation = report.target_generation.expect("render target generation");
            let buffer = report
                .buffer
                .unwrap_or_else(|| panic!("format {format:?}: {:?}", report.detail));
            let expected_format = format.or(previous_format).unwrap_or(buffer.format());
            assert!(matches!(expected_format, XR24 | AR24));
            assert_eq!(buffer.format(), expected_format);
            eprintln!(
                "output_format slot={slot:?} request={format:?} actual={:#x} modifier={:?} generation={generation}",
                buffer.format(),
                buffer.modifier()
            );
            let after = context.persistent_render_stats();
            if index > 0 && format.is_none_or(|format| Some(format) == previous_format) {
                assert_eq!(
                    Some(generation),
                    previous_generation,
                    "a satisfied format request must reuse the actual target"
                );
                assert_eq!(
                    after.composition_target_creations,
                    before.composition_target_creations
                );
                assert_eq!(
                    after.composition_target_reuses,
                    before.composition_target_reuses + 1
                );
            } else {
                assert_ne!(
                    Some(generation),
                    previous_generation,
                    "a different actual format must rebuild the target"
                );
                assert_eq!(
                    after.composition_target_creations,
                    before.composition_target_creations + 1
                );
            }
            let fds = buffer
                .export_plane_fds()
                .expect("export target")
                .into_plane_fds();
            let strides = buffer.plane_pitches();
            let offsets = buffer.plane_offsets();
            let probe = NativePixmapImportProbe::new(
                open(),
                NativeMultiPlaneDmaBufFrame {
                    width: buffer.width(),
                    height: buffer.height(),
                    format: buffer.format(),
                    modifier: buffer
                        .modifier()
                        .unwrap_or(u64::from(gbm::Modifier::Invalid)),
                    plane_count: buffer.plane_count(),
                    planes: std::array::from_fn(|index| {
                        fds[index].as_ref().map(|fd| NativeDmaBufPlane {
                            fd: fd.as_fd(),
                            offset: offsets[index],
                            stride: strides[index],
                        })
                    }),
                },
            )
            .expect("independently import actual output");
            assert_eq!(
                probe.read_rgba().expect("read actual output"),
                [37, 91, 163, 255].repeat(32 * 16)
            );
            drop(probe);
            drop(fds);
            previous_format = Some(buffer.format());
            drop(buffer);
            previous_generation = Some(generation);

            let before = context.persistent_render_stats();
            let refused = context.export_composed_owned_scanout_buffer(
                frame,
                NativeCompositionOutputRequest {
                    preferred_modifiers: &[],
                    format: Some(NativeCompositionFormatRequest::Required(0x3231_564e)),
                },
            );
            assert_eq!(
                refused.detail,
                NativeGbmScanoutBufferExportDetail::InvalidTarget
            );
            assert!(refused.buffer.is_none());
            let after = context.persistent_render_stats();
            assert_eq!(
                after.composition_target_creations,
                before.composition_target_creations
            );
            assert_eq!(
                after.composition_target_reuses,
                before.composition_target_reuses
            );
        }
    }
}
