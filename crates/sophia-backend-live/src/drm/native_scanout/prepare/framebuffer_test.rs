use super::*;

/// An explicit AddFB2 refusal after every source plane was imported successfully.
/// This is an observation; the caller retains source identity and native context.
#[derive(Clone, Copy, Debug)]
pub struct LibdrmNativeFramebufferRejection {
    pub(super) descriptor: LiveRendererScanoutBufferDescriptor,
    pub(super) selected: LibdrmNativePrimaryPlaneSelection,
    pub(super) property_handles: LibdrmNativePrimaryPlanePropertyHandles,
    pub(super) policy: LibdrmNativePrimaryPlaneScanoutSubmitPolicy,
    pub(super) error_kind: io::ErrorKind,
    pub(super) raw_os_error: Option<i32>,
}

impl LibdrmNativeFramebufferRejection {
    pub const fn descriptor(&self) -> LiveRendererScanoutBufferDescriptor {
        self.descriptor
    }

    pub const fn error_kind(&self) -> io::ErrorKind {
        self.error_kind
    }

    pub const fn raw_os_error(&self) -> Option<i32> {
        self.raw_os_error
    }
}

/// The intended request uses the alternative's existing framebuffer. Only the
/// alternative is tested; no original atomic result is implied by AddFB2 failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LibdrmNativeFramebufferTestReport {
    pub status: LibdrmNativeAtomicTestPairStatus,
    pub intended_request: Option<LibdrmNativeAtomicRequestEvidence>,
    pub error_kind: io::ErrorKind,
    pub raw_os_error: Option<i32>,
    pub alternative: Option<LibdrmNativeAtomicTestReport>,
}

/// Compare fresh preparation evidence on one device without changing ownership.
/// The caller supplies the device and context owning both preparations.
pub fn validate_alternative_after_framebuffer_rejection<D>(
    device: &D,
    rejection: LibdrmNativeFramebufferRejection,
    alternative: LibdrmNativePrimaryPlanePreparedScanout,
) -> (
    LibdrmNativeFramebufferTestReport,
    LibdrmNativePrimaryPlanePreparedScanout,
)
where
    D: LibdrmNativeAtomicCommitDevice,
{
    let mut report = LibdrmNativeFramebufferTestReport {
        status: LibdrmNativeAtomicTestPairStatus::FramebufferRejectionIneligible,
        intended_request: None,
        error_kind: rejection.error_kind,
        raw_os_error: rejection.raw_os_error,
        alternative: None,
    };
    if rejection.error_kind != io::ErrorKind::InvalidInput
        || rejection.raw_os_error != Some(22)
        || rejection.policy.allow_modeset
        || rejection.policy.test_only
        || !rejection.policy.page_flip_event
        || alternative.request_scope != LibdrmNativeAtomicCommitRequestScope::PageFlip
        || alternative.commit_flags.allow_modeset
        || alternative.commit_flags.test_only
        || !alternative.commit_flags.page_flip_event
    {
        return (report, alternative);
    }
    if rejection.descriptor.size != alternative.descriptor.size {
        report.status = LibdrmNativeAtomicTestPairStatus::GeometryMismatch;
        return (report, alternative);
    }
    if rejection.selected != alternative.selected {
        report.status = LibdrmNativeAtomicTestPairStatus::SelectionMismatch;
        return (report, alternative);
    }
    let explicit = |modifier: Option<u64>| {
        modifier.filter(|modifier| {
            *modifier != sophia_protocol::DRM_FORMAT_MOD_INVALID && *modifier != u64::MAX
        })
    };
    let (Some(original_modifier), Some(alternative_modifier)) = (
        explicit(rejection.descriptor.modifier),
        explicit(alternative.descriptor.modifier),
    ) else {
        report.status = LibdrmNativeAtomicTestPairStatus::LayoutMismatch;
        return (report, alternative);
    };
    if rejection.descriptor.format != alternative.descriptor.format
        || original_modifier == alternative_modifier
    {
        report.status = LibdrmNativeAtomicTestPairStatus::LayoutMismatch;
        return (report, alternative);
    }

    // Reuse the real alternative resource, but the failed source's selection,
    // discovered properties and policy. The common builder owns every property.
    let intended = build_native_primary_plane_atomic_request_for_policy(
        rejection
            .selected
            .into_objects(alternative.resources.framebuffer, None),
        rejection.property_handles,
        rejection.policy,
    );
    let Some(intended) = intended.request else {
        report.status = LibdrmNativeAtomicTestPairStatus::MissingRequestEvidence;
        return (report, alternative);
    };
    report.intended_request = apply_scanout_submit_policy(intended, rejection.policy)
        .test_only()
        .evidence();
    let (Some(intended), Some(current)) =
        (report.intended_request, alternative.test_request_evidence())
    else {
        report.status = LibdrmNativeAtomicTestPairStatus::MissingRequestEvidence;
        return (report, alternative);
    };
    if intended != current {
        report.status = LibdrmNativeAtomicTestPairStatus::RequestMismatch;
        return (report, alternative);
    }
    let (tested, alternative) =
        validate_prepared_native_primary_plane_scanout_detailed(device, alternative);
    report.status = LibdrmNativeAtomicTestPairStatus::Tested;
    report.alternative = Some(tested);
    (report, alternative)
}

#[path = "../../../../tests/support/native_framebuffer_test.rs"]
mod tests;
