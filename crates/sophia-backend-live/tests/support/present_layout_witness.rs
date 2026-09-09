#![cfg(test)]

use super::*;

const OUTPUT: OutputId = OutputId::from_raw(1);
const TRANSACTION: TransactionId = TransactionId::from_raw(71);
const SURFACE: SurfaceId = SurfaceId::new(7, 1);
const FRAME: LiveProductionNativeFrameId = LiveProductionNativeFrameId::from_raw(8);
const SIZE: Size = Size {
    width: 16,
    height: 16,
};

fn runtime() -> LiveProductionVisualRuntime {
    LiveProductionVisualRuntime::new(
        &[HeadlessOutput {
            id: OUTPUT,
            size: SIZE,
            scale: 1,
        }],
        None,
    )
    .unwrap()
}

fn submitted(
    runtime: &LiveProductionVisualRuntime,
    outputs: &[OutputId],
) -> LiveProductionSubmittedPresent {
    let geometry = Rect {
        x: 0,
        y: 0,
        width: SIZE.width,
        height: SIZE.height,
    };
    let candidate = SurfaceTransaction {
        input_region: None,
        transaction: TRANSACTION,
        surface: SURFACE,
        authority: AuthorityKind::SophiaX,
        namespace: None,
        target_geometry: geometry,
        presentation_extent: SIZE,
        content: SurfaceContentSet::singleton(BufferSource::DmaBuf { handle: 91 }, SIZE),
        damage: Region::empty(),
        readiness: SurfaceTransactionReadiness::Ready,
        timeout_msec: 250,
        previous_committed_generation: 0,
    };
    let prepared = runtime.production.prepare_present_transaction(&candidate);
    assert!(prepared.is_ready());
    LiveProductionSubmittedPresent::new(
        outputs.iter().map(|output| (*output, FRAME)).collect(),
        OUTPUT,
        candidate.key(),
        TRANSACTION,
        SURFACE,
        prepared,
        LiveRetainedRendererImageLayer {
            image_id: renderer_image_for_present(TRANSACTION),
            size: SIZE,
            format: DRM_FORMAT_XRGB8888,
            placement: LiveCompositionPlacement {
                target: geometry,
                clip: None,
                transform: Transform::IDENTITY,
                alpha: 1.0,
                sampling: HeadSamplingClass::Exact,
            },
        },
    )
    .unwrap()
}

fn retirement() -> LiveProductionNativeFrameRetirement {
    let head = RenderHeadId::from_raw(2);
    LiveProductionNativeFrameRetirement {
        output: OUTPUT,
        frame: FRAME,
        submission: 3,
        content: LiveProductionScanoutContent::MixedPresent {
            frame: FRAME,
            transaction: TRANSACTION,
            nonzero_rgb_pixels: 256,
        },
        direct: false,
        ust: 1_000,
        msc: 4,
        layout_witness: Some(LiveProductionRetiredLayoutWitness {
            witness: LiveScanoutLayoutWitness {
                source_image: renderer_image_for_present(TRANSACTION),
                alternative: LiveRendererFrameCorrelation {
                    request: None,
                    trace: Some(LiveCompositionTrace {
                        output: OUTPUT,
                        head,
                        scene_generation: 41,
                    }),
                    direct_scanout: Some(DirectScanoutVerdict::CompositionRequired("refused")),
                },
                format: DRM_FORMAT_XRGB8888,
                original_modifier: 0x0200_0000_0040_1b03,
                alternative_modifier: 0,
            },
            device: LiveRenderDeviceNodeIdentity {
                device: 1,
                inode: 2,
                device_number: 226,
            },
            head,
            target_generation: 5,
        }),
    }
}

#[test]
fn a_copied_layout_comparison_belongs_to_its_exact_committed_present() {
    let mut runtime = runtime();
    let submitted = submitted(&runtime, &[OUTPUT]);
    let identity = SubmittedLayoutIdentity::from_submitted(&submitted).unwrap();
    let commit = runtime
        .production
        .apply_prepared_surface_commit(submitted.prepared);
    assert_eq!(commit.outcome, TransactionOutcome::Committed);
    let retired = retirement();
    assert_eq!(identity.settle(retired, &commit), retired.layout_witness);
}

#[test]
fn retirement_of_a_stale_engine_candidate_cannot_publish_a_layout_comparison() {
    let mut runtime = runtime();
    let old = submitted(&runtime, &[OUTPUT]);
    let identity = SubmittedLayoutIdentity::from_submitted(&old).unwrap();
    let newer = submitted(&runtime, &[OUTPUT]);
    let commit = runtime
        .production
        .apply_prepared_surface_commit(newer.prepared);
    assert_eq!(commit.outcome, TransactionOutcome::Committed);
    let stale = runtime
        .production
        .apply_prepared_surface_commit(old.prepared);
    assert_eq!(stale.outcome, TransactionOutcome::RejectedStaleSurface);
    assert!(identity.settle(retirement(), &stale).is_none());
}

#[test]
fn mismatched_retirement_image_transaction_output_or_disposition_is_inconclusive() {
    for case in 0..13 {
        let mut runtime = runtime();
        let submitted = submitted(&runtime, &[OUTPUT]);
        let identity = SubmittedLayoutIdentity::from_submitted(&submitted).unwrap();
        let mut commit = runtime
            .production
            .apply_prepared_surface_commit(submitted.prepared);
        let mut retired = retirement();
        match case {
            0 => retired.direct = true,
            1 => retired.output = OutputId::from_raw(2),
            2 => retired.frame = LiveProductionNativeFrameId::from_raw(9),
            3 => {
                retired.content = LiveProductionScanoutContent::MixedPresent {
                    frame: FRAME,
                    transaction: TransactionId::from_raw(72),
                    nonzero_rgb_pixels: 256,
                }
            }
            4 => {
                retired.content = LiveProductionScanoutContent::Cpu {
                    frame: FRAME,
                    checksum: 1,
                }
            }
            5 => retired.layout_witness = None,
            6 => {
                retired
                    .layout_witness
                    .as_mut()
                    .unwrap()
                    .witness
                    .source_image = LiveRendererImageId::from_raw(72)
            }
            7 => retired.layout_witness.as_mut().unwrap().witness.format = DRM_FORMAT_ARGB8888,
            8 => {
                retired
                    .layout_witness
                    .as_mut()
                    .unwrap()
                    .witness
                    .alternative
                    .trace
                    .as_mut()
                    .unwrap()
                    .output = OutputId::from_raw(2)
            }
            9 => retired.layout_witness.as_mut().unwrap().head = RenderHeadId::from_raw(3),
            10 => commit.transaction = TransactionId::from_raw(72),
            11 => commit.applied_surfaces = vec![SurfaceId::new(7, 2)],
            12 => commit.outcome = TransactionOutcome::TimedOut,
            _ => unreachable!(),
        }
        assert!(identity.settle(retired, &commit).is_none(), "case {case}");
    }
}

#[test]
fn a_retained_image_cannot_impersonate_another_presents_source() {
    let mut runtime = runtime();
    let mut submitted = submitted(&runtime, &[OUTPUT]);
    submitted.displayed_layer.image_id = LiveRendererImageId::from_raw(72);
    let identity = SubmittedLayoutIdentity::from_submitted(&submitted).unwrap();
    let commit = runtime
        .production
        .apply_prepared_surface_commit(submitted.prepared);
    let mut retired = retirement();
    retired
        .layout_witness
        .as_mut()
        .unwrap()
        .witness
        .source_image = LiveRendererImageId::from_raw(72);
    assert!(identity.settle(retired, &commit).is_none());
}

#[test]
fn a_single_head_comparison_cannot_certify_an_entire_multi_output_present() {
    let runtime = runtime();
    let submitted = submitted(&runtime, &[OUTPUT, OutputId::from_raw(2)]);
    assert!(SubmittedLayoutIdentity::from_submitted(&submitted).is_none());
}

#[test]
fn layout_identity_keeps_the_prepared_surface_and_backing_generation() {
    for case in 0..4 {
        let runtime = runtime();
        let mut submitted = submitted(&runtime, &[OUTPUT]);
        match case {
            0 => submitted.candidate.surface = SurfaceId::new(7, 2),
            1 => submitted.candidate.transaction = TransactionId::from_raw(72),
            2 => submitted.candidate.target_buffer = BufferSource::DmaBuf { handle: 92 },
            3 => {
                submitted.transaction = TransactionId::from_raw(72);
                submitted.candidate.transaction = submitted.transaction;
            }
            _ => unreachable!(),
        }
        assert!(
            SubmittedLayoutIdentity::from_submitted(&submitted).is_none(),
            "case {case}"
        );
    }
}
