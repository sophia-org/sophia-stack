#![cfg(test)]

use super::*;

fn canonical(framebuffer: u64) -> LibdrmNativeAtomicCommitRequest {
    let mut request = crate::drm::native_atomic::LibdrmNativeRecordedAtomicRequest::new();
    request.add_property(
        drm::control::from_u32::<drm::control::plane::Handle>(13).unwrap(),
        drm::control::from_u32::<drm::control::property::Handle>(104).unwrap(),
        drm::control::property::Value::UnsignedRange(framebuffer),
    );
    request.finish(LibdrmNativeAtomicCommitRequestScope::PageFlip, (13, 104))
}

#[test]
fn paired_test_requests_reject_raw_provenance_on_either_side() {
    let raw = LibdrmNativeAtomicCommitRequest::new(drm::control::atomic::AtomicModeReq::new());
    let recorded = canonical(14);
    for (original, alternative) in [(&raw, &raw), (&raw, &recorded), (&recorded, &raw)] {
        assert_eq!(
            compare_test_requests(original, alternative),
            LibdrmNativeAtomicTestPairStatus::MissingRequestEvidence
        );
    }
    assert_eq!(
        compare_test_requests(&recorded, &canonical(15)),
        LibdrmNativeAtomicTestPairStatus::Tested
    );
    assert_eq!(
        compare_test_requests(&recorded, &recorded),
        LibdrmNativeAtomicTestPairStatus::RequestMismatch
    );
    assert_eq!(
        compare_test_requests(&recorded, &canonical(15).blocking()),
        LibdrmNativeAtomicTestPairStatus::RequestMismatch
    );
}
