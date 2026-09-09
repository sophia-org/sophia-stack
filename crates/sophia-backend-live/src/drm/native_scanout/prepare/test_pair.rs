use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LibdrmNativeAtomicTestPairStatus {
    Tested,
    MissingRequestEvidence,
    SelectionMismatch,
    GeometryMismatch,
    RequestMismatch,
}

/// Observations from consecutive tests, without authorizing a later commit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LibdrmNativeAtomicTestPairReport {
    pub status: LibdrmNativeAtomicTestPairStatus,
    pub original: Option<LibdrmNativeAtomicTestReport>,
    pub alternative: Option<LibdrmNativeAtomicTestReport>,
}

/// Tests comparable requests on one device and returns both affine owners untouched.
/// Neither a mismatch nor a driver refusal releases their resources.
/// The caller supplies the device that owns both preparations.
pub fn validate_prepared_native_primary_plane_scanout_pair<D>(
    device: &D,
    original: LibdrmNativePrimaryPlanePreparedScanout,
    alternative: LibdrmNativePrimaryPlanePreparedScanout,
) -> (
    LibdrmNativeAtomicTestPairReport,
    LibdrmNativePrimaryPlanePreparedScanout,
    LibdrmNativePrimaryPlanePreparedScanout,
)
where
    D: LibdrmNativeAtomicCommitDevice,
{
    let status = if original.selected != alternative.selected {
        LibdrmNativeAtomicTestPairStatus::SelectionMismatch
    } else if original.descriptor.size != alternative.descriptor.size {
        LibdrmNativeAtomicTestPairStatus::GeometryMismatch
    } else {
        compare_test_requests(&original.request, &alternative.request)
    };
    if status != LibdrmNativeAtomicTestPairStatus::Tested {
        return (
            LibdrmNativeAtomicTestPairReport {
                status,
                original: None,
                alternative: None,
            },
            original,
            alternative,
        );
    }
    let (original_report, original) =
        validate_prepared_native_primary_plane_scanout_detailed(device, original);
    let (alternative_report, alternative) =
        validate_prepared_native_primary_plane_scanout_detailed(device, alternative);
    (
        LibdrmNativeAtomicTestPairReport {
            status,
            original: Some(original_report),
            alternative: Some(alternative_report),
        },
        original,
        alternative,
    )
}

fn compare_test_requests(
    original: &LibdrmNativeAtomicCommitRequest,
    alternative: &LibdrmNativeAtomicCommitRequest,
) -> LibdrmNativeAtomicTestPairStatus {
    let (Some(original), Some(alternative)) = (
        original.clone().test_only().evidence(),
        alternative.clone().test_only().evidence(),
    ) else {
        return LibdrmNativeAtomicTestPairStatus::MissingRequestEvidence;
    };
    if original.equivalent_except_primary_framebuffer(&alternative) {
        LibdrmNativeAtomicTestPairStatus::Tested
    } else {
        LibdrmNativeAtomicTestPairStatus::RequestMismatch
    }
}

#[path = "../../../../tests/support/native_atomic_test_pair.rs"]
mod tests;
