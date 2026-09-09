#![cfg(test)]

use super::*;

fn request(framebuffer: u64, other_property: u64) -> LibdrmNativeAtomicCommitRequest {
    let mut request = crate::LibdrmNativeRecordedAtomicRequest::new();
    let plane = drm::control::from_u32::<drm::control::plane::Handle>(13).unwrap();
    for (property, value) in [(104, framebuffer), (105, other_property)] {
        request.add_property(
            plane,
            drm::control::from_u32::<drm::control::property::Handle>(property).unwrap(),
            drm::control::property::Value::UnsignedRange(value),
        );
    }
    request
        .finish(LibdrmNativeAtomicCommitRequestScope::PageFlip, (13, 104))
        .test_only()
}

fn observation(framebuffer: u64, errno: Option<i32>) -> LibdrmNativeAtomicTestReport {
    let request = request(framebuffer, 9);
    LibdrmNativeAtomicTestReport {
        status: if errno.is_some() {
            LibdrmNativeAtomicCommitSubmitStatus::Rejected
        } else {
            LibdrmNativeAtomicCommitSubmitStatus::Submitted
        },
        request: request.evidence(),
        error_kind: errno.map(|errno| std::io::Error::from_raw_os_error(errno).kind()),
        raw_os_error: errno,
        request_scope: request.reduced_scope(),
        commit_flags: request.reduced_flags(),
    }
}

fn report() -> LiveScanoutLayoutProbeReport {
    let alternative = crate::LiveRendererFrameCorrelation {
        request: None,
        trace: Some(sophia_renderer_live::LiveCompositionTrace {
            output: sophia_protocol::OutputId::from_raw(1),
            head: sophia_engine::RenderHeadId::from_raw(2),
            scene_generation: 3,
        }),
        direct_scanout: Some(sophia_engine::DirectScanoutVerdict::CompositionRequired(
            "refused",
        )),
    };
    let mut original = alternative;
    original.direct_scanout = Some(sophia_engine::DirectScanoutVerdict::Eligible);
    LiveScanoutLayoutProbeReport {
        source_image: sophia_renderer_live::LiveRendererImageId::from_raw(8),
        original,
        alternative,
        format: sophia_renderer_live::LIVE_RENDERER_SCANOUT_FORMAT_XRGB8888,
        original_modifier: 7,
        alternative_modifier: 0,
        tests: LibdrmNativeAtomicTestPairReport {
            status: LibdrmNativeAtomicTestPairStatus::Tested,
            original: Some(observation(14, Some(22))),
            alternative: Some(observation(15, None)),
        },
    }
}

#[test]
fn layout_witness_requires_the_exact_current_request_and_correlation() {
    let report = report();
    let mut descriptor = LiveRendererScanoutBufferDescriptor::new(
        sophia_protocol::Size {
            width: 4,
            height: 4,
        },
        16,
        report.format,
        15,
    );
    descriptor.modifier = Some(0);
    let current = request(15, 9).evidence().unwrap();
    assert!(
        report
            .witness_for_current(Some(report.alternative), descriptor, current)
            .is_some()
    );
    for changed in [request(16, 9), request(15, 10), request(15, 9).blocking()] {
        assert!(
            report
                .witness_for_current(
                    Some(report.alternative),
                    descriptor,
                    changed.evidence().unwrap()
                )
                .is_none()
        );
    }
    let mut newer = report.alternative;
    newer.trace.as_mut().unwrap().scene_generation += 1;
    for correlation in [None, Some(newer), Some(report.original)] {
        assert!(
            report
                .witness_for_current(correlation, descriptor, current)
                .is_none()
        );
    }
    let mut changed = report;
    changed.tests.alternative.as_mut().unwrap().request = None;
    assert!(
        changed
            .witness_for_current(Some(report.alternative), descriptor, current)
            .is_none()
    );
    let mut changed = report;
    changed.tests.original.as_mut().unwrap().request = Some(current);
    assert!(
        changed
            .witness_for_current(Some(report.alternative), descriptor, current)
            .is_none(),
        "same framebuffer is not an alternate layout comparison"
    );
}
