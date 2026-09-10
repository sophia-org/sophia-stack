use crate::prelude::*;
use std::os::fd::OwnedFd;

mod test_pair;
pub use test_pair::*;
mod framebuffer_test;
pub use framebuffer_test::*;

/// Complete native resources and atomic request for one head, before the
/// kernel has accepted a page flip.
///
/// This is an affine owner. Callers must either submit it or cancel it against
/// the same DRM device; dropping it would leak the framebuffer/import/blob
/// resources it owns.
#[derive(Debug)]
pub struct LibdrmNativePrimaryPlanePreparedScanout {
    descriptor: LiveRendererScanoutBufferDescriptor,
    selection: LibdrmNativePrimaryPlaneSelectionStatus,
    properties: LibdrmNativePrimaryPlanePropertyDiscoveryStatus,
    format_table: LibdrmNativePrimaryPlaneFormatTableStatus,
    resources_status: LibdrmNativePrimaryPlaneResourceCreateStatus,
    framebuffer: Option<LibdrmNativePrimaryPlaneFramebufferCreateDetail>,
    request_scope: LibdrmNativeAtomicCommitRequestScope,
    commit_flags: LibdrmNativeAtomicCommitFlagsReport,
    selected: LibdrmNativePrimaryPlaneSelection,
    property_handles: LibdrmNativePrimaryPlanePropertyHandles,
    resources: LibdrmNativePrimaryPlaneResourceBundle,
    request: LibdrmNativeAtomicCommitRequest,
    /// The same commit without its cursor, prepared beside the one that
    /// carries it.
    ///
    /// A combined commit shares one fate: a cursor-side refusal takes the
    /// frame with it, a failure class that cannot exist while the cursor
    /// rides a separate ioctl. The retry is built here, from the same
    /// objects, rather than rebuilt at rejection time -- the frame the
    /// driver then accepts is one whose construction did not depend on
    /// anything having gone wrong first. A cursor must never cost a frame.
    retry_without_cursor: Option<LibdrmNativeAtomicCommitRequest>,
}

impl LibdrmNativePrimaryPlanePreparedScanout {
    pub(crate) fn test_request_evidence(&self) -> Option<LibdrmNativeAtomicRequestEvidence> {
        self.request.clone().test_only().evidence()
    }

    pub const fn descriptor(&self) -> LiveRendererScanoutBufferDescriptor {
        self.descriptor
    }
}

/// One enabled head's complete resources for a card-scoped topology commit.
///
/// The framebuffer/imports/mode blob remain affine. After the combined request
/// succeeds, adopt this owner into a scanout submission; otherwise cancel it.
#[derive(Debug)]
pub struct LibdrmNativePrimaryPlanePreparedTopologyHead {
    atomic_head: LibdrmNativeAtomicHead,
    resources: LibdrmNativePrimaryPlaneResourceBundle,
}

#[derive(Clone, Copy, Debug)]
pub struct LibdrmNativePreparedDisabledTopologyHead {
    atomic_head: LibdrmNativeAtomicDisabledHead,
}

impl LibdrmNativePreparedDisabledTopologyHead {
    pub const fn atomic_head(self) -> LibdrmNativeAtomicDisabledHead {
        self.atomic_head
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LibdrmNativeDisabledTopologyHeadPrepareStatus {
    Prepared,
    PropertyDiscoveryUnavailable,
}

#[derive(Clone, Copy, Debug)]
pub struct LibdrmNativeDisabledTopologyHeadPrepareResult {
    pub status: LibdrmNativeDisabledTopologyHeadPrepareStatus,
    pub properties: LibdrmNativePrimaryPlanePropertyDiscoveryStatus,
    pub prepared: Option<LibdrmNativePreparedDisabledTopologyHead>,
}

impl LibdrmNativePrimaryPlanePreparedTopologyHead {
    pub const fn atomic_head(&self) -> LibdrmNativeAtomicHead {
        self.atomic_head
    }
}

/// Resolves the property handles needed to detach one disabled head.
///
/// Unlike an enabled head this owns no framebuffer or mode blob, but it is still
/// a required prepared member: property discovery must succeed before any card
/// applies, otherwise rollback could be required after a failure that was known
/// in advance.
pub fn prepare_native_disabled_topology_head<D>(
    device: &D,
    selection: LibdrmNativePrimaryPlaneSelection,
) -> LibdrmNativeDisabledTopologyHeadPrepareResult
where
    D: LibdrmNativePropertyLookupDevice,
{
    let properties = discover_native_primary_plane_property_handles(
        device,
        selection.connector_handle(),
        selection.crtc_handle(),
        selection.plane_handle(),
    );
    let Some(handles) = properties.properties else {
        return LibdrmNativeDisabledTopologyHeadPrepareResult {
            status: LibdrmNativeDisabledTopologyHeadPrepareStatus::PropertyDiscoveryUnavailable,
            properties: properties.status,
            prepared: None,
        };
    };
    LibdrmNativeDisabledTopologyHeadPrepareResult {
        status: LibdrmNativeDisabledTopologyHeadPrepareStatus::Prepared,
        properties: properties.status,
        prepared: Some(LibdrmNativePreparedDisabledTopologyHead {
            atomic_head: LibdrmNativeAtomicDisabledHead::new(selection, handles),
        }),
    }
}

#[derive(Debug)]
pub struct LibdrmNativePrimaryPlaneScanoutPrepareResult {
    pub status: LibdrmNativePrimaryPlaneScanoutPrepareStatus,
    pub selection: LibdrmNativePrimaryPlaneSelectionStatus,
    pub scanout_buffer: LiveRendererScanoutBufferStatus,
    pub buffer_format: Option<LibdrmNativeScanoutBufferFormatDetail>,
    pub buffer_modifier: Option<LibdrmNativeScanoutBufferModifierDetail>,
    pub buffer_planes: Option<LibdrmNativeScanoutBufferPlaneDetail>,
    pub properties: Option<LibdrmNativePrimaryPlanePropertyDiscoveryStatus>,
    pub format_table: Option<LibdrmNativePrimaryPlaneFormatTableStatus>,
    pub resources: Option<LibdrmNativePrimaryPlaneResourceCreateStatus>,
    pub framebuffer: Option<LibdrmNativePrimaryPlaneFramebufferCreateDetail>,
    pub framebuffer_rejection: Option<LibdrmNativeFramebufferRejection>,
    pub request: Option<LibdrmNativeAtomicRequestBuildStatus>,
    pub request_scope: Option<LibdrmNativeAtomicCommitRequestScope>,
    pub commit_flags: Option<LibdrmNativeAtomicCommitFlagsReport>,
    pub prepared: Option<LibdrmNativePrimaryPlanePreparedScanout>,
    pub cleanup: Option<LibdrmNativePrimaryPlaneResourceCleanup>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LibdrmNativePrimaryPlaneScanoutPrepareStatus {
    Prepared,
    KmsTargetUnavailable,
    ScanoutBufferUnavailable,
    PropertyDiscoveryUnavailable,
    ResourceCreationUnavailable,
    AtomicRequestBuildFailed,
}

impl LibdrmNativePrimaryPlaneScanoutPrepareResult {
    fn from_descriptor(
        status: LibdrmNativePrimaryPlaneScanoutPrepareStatus,
        selection: LibdrmNativePrimaryPlaneSelectionStatus,
        scanout_buffer: LiveRendererScanoutBufferStatus,
        descriptor: LiveRendererScanoutBufferDescriptor,
    ) -> Self {
        Self {
            status,
            selection,
            scanout_buffer,
            buffer_format: Some(LibdrmNativeScanoutBufferFormatDetail::from_descriptor(
                descriptor,
            )),
            buffer_modifier: Some(LibdrmNativeScanoutBufferModifierDetail::from_descriptor(
                descriptor,
            )),
            buffer_planes: Some(LibdrmNativeScanoutBufferPlaneDetail::from_descriptor(
                descriptor,
            )),
            properties: None,
            format_table: None,
            resources: None,
            framebuffer: None,
            framebuffer_rejection: None,
            request: None,
            request_scope: None,
            commit_flags: None,
            prepared: None,
            cleanup: None,
        }
    }
}

pub fn prepare_native_primary_plane_scanout_from_selection_and_renderer_descriptor_with_policy<D>(
    device: &D,
    selection: LibdrmNativePrimaryPlaneSelectionResult,
    descriptor: LiveRendererScanoutBufferDescriptor,
    policy: LibdrmNativePrimaryPlaneScanoutSubmitPolicy,
) -> LibdrmNativePrimaryPlaneScanoutPrepareResult
where
    D: LibdrmNativePropertyLookupDevice + LibdrmNativePrimaryPlaneResourceDevice,
{
    prepare_native_primary_plane_scanout_from_selection_and_renderer_descriptor_with_optional_dma_bufs(
        device, selection, descriptor, None, policy,
    )
}

pub fn prepare_native_primary_plane_scanout_from_selection_and_renderer_dma_bufs_with_policy<D>(
    device: &D,
    selection: LibdrmNativePrimaryPlaneSelectionResult,
    descriptor: LiveRendererScanoutBufferDescriptor,
    plane_fds: [Option<OwnedFd>; 4],
    policy: LibdrmNativePrimaryPlaneScanoutSubmitPolicy,
) -> LibdrmNativePrimaryPlaneScanoutPrepareResult
where
    D: LibdrmNativePropertyLookupDevice + LibdrmNativePrimaryPlaneResourceDevice,
{
    prepare_native_primary_plane_scanout_from_selection_and_renderer_descriptor_with_optional_dma_bufs(
        device,
        selection,
        descriptor,
        Some(plane_fds),
        policy,
    )
}

fn prepare_native_primary_plane_scanout_from_selection_and_renderer_descriptor_with_optional_dma_bufs<
    D,
>(
    device: &D,
    selection: LibdrmNativePrimaryPlaneSelectionResult,
    descriptor: LiveRendererScanoutBufferDescriptor,
    plane_fds: Option<[Option<OwnedFd>; 4]>,
    policy: LibdrmNativePrimaryPlaneScanoutSubmitPolicy,
) -> LibdrmNativePrimaryPlaneScanoutPrepareResult
where
    D: LibdrmNativePropertyLookupDevice + LibdrmNativePrimaryPlaneResourceDevice,
{
    let scanout_buffer = if descriptor.is_valid_scanout_buffer() {
        LiveRendererScanoutBufferStatus::Ready
    } else {
        LiveRendererScanoutBufferStatus::Invalid
    };
    if selection.status != LibdrmNativePrimaryPlaneSelectionStatus::Selected {
        return LibdrmNativePrimaryPlaneScanoutPrepareResult::from_descriptor(
            LibdrmNativePrimaryPlaneScanoutPrepareStatus::KmsTargetUnavailable,
            selection.status,
            scanout_buffer,
            descriptor,
        );
    }
    let Some(selected) = selection.selection else {
        return LibdrmNativePrimaryPlaneScanoutPrepareResult::from_descriptor(
            LibdrmNativePrimaryPlaneScanoutPrepareStatus::KmsTargetUnavailable,
            selection.status,
            scanout_buffer,
            descriptor,
        );
    };
    let buffer = LibdrmRendererScanoutBuffer::from_descriptor(descriptor);
    if buffer.is_none() {
        return LibdrmNativePrimaryPlaneScanoutPrepareResult::from_descriptor(
            LibdrmNativePrimaryPlaneScanoutPrepareStatus::ScanoutBufferUnavailable,
            selection.status,
            scanout_buffer,
            descriptor,
        );
    }

    let properties = discover_native_primary_plane_property_handles(
        device,
        selected.connector,
        selected.crtc,
        selected.plane,
    );
    let Some(property_handles) = properties.properties else {
        let mut result = LibdrmNativePrimaryPlaneScanoutPrepareResult::from_descriptor(
            LibdrmNativePrimaryPlaneScanoutPrepareStatus::PropertyDiscoveryUnavailable,
            selection.status,
            scanout_buffer,
            descriptor,
        );
        result.properties = Some(properties.status);
        return result;
    };
    let format_table =
        LibdrmNativePrimaryPlaneFormatTableStatus::from_property_handles(property_handles);
    let imported_dma_bufs = plane_fds.is_some();
    let resources = match (policy.allow_modeset, plane_fds) {
        (true, Some(plane_fds)) => create_native_primary_plane_resources_from_dma_bufs(
            device, selected, descriptor, plane_fds,
        ),
        (false, Some(plane_fds)) => create_native_primary_plane_page_flip_resources_from_dma_bufs(
            device, selected, descriptor, plane_fds,
        ),
        (true, None) => create_native_primary_plane_resources(
            device,
            selected,
            buffer
                .as_ref()
                .expect("validated descriptor should produce a buffer"),
        ),
        (false, None) => create_native_primary_plane_page_flip_resources(
            device,
            selected,
            buffer
                .as_ref()
                .expect("validated descriptor should produce a buffer"),
        ),
    };
    let Some(resource_bundle) = resources.resources else {
        let mut result = LibdrmNativePrimaryPlaneScanoutPrepareResult::from_descriptor(
            LibdrmNativePrimaryPlaneScanoutPrepareStatus::ResourceCreationUnavailable,
            selection.status,
            scanout_buffer,
            descriptor,
        );
        result.properties = Some(properties.status);
        result.format_table = Some(format_table);
        result.resources = Some(resources.status);
        result.framebuffer = resources.framebuffer;
        if imported_dma_bufs
            && resources.status
                == LibdrmNativePrimaryPlaneResourceCreateStatus::FramebufferCreateFailed
            && let Some(LibdrmNativePrimaryPlaneFramebufferCreateDetail::AddFb2ModifiersFailed {
                error_kind,
                raw_os_error,
            }) = resources.framebuffer
        {
            result.framebuffer_rejection = Some(LibdrmNativeFramebufferRejection {
                descriptor,
                selected,
                property_handles,
                policy,
                error_kind,
                raw_os_error,
            });
        }
        result.cleanup = resources.cleanup;
        return result;
    };

    let objects = resource_bundle.into_objects(selected);
    let request =
        build_native_primary_plane_atomic_request_for_policy(objects, property_handles, policy);
    let retry_without_cursor = policy.cursor.is_some().then(|| {
        build_native_primary_plane_atomic_request_for_policy(
            objects,
            property_handles,
            policy.without_cursor(),
        )
        .request
    });
    let Some(request_owner) = request.request else {
        let destroy = destroy_native_primary_plane_resources(device, resource_bundle);
        let mut result = LibdrmNativePrimaryPlaneScanoutPrepareResult::from_descriptor(
            LibdrmNativePrimaryPlaneScanoutPrepareStatus::AtomicRequestBuildFailed,
            selection.status,
            scanout_buffer,
            descriptor,
        );
        result.properties = Some(properties.status);
        result.format_table = Some(format_table);
        result.resources = Some(resources.status);
        result.framebuffer = resources.framebuffer;
        result.request = Some(request.status);
        result.cleanup = destroy.cleanup;
        return result;
    };
    let request_owner = apply_scanout_submit_policy(request_owner, policy);
    let request_scope = request_owner.reduced_scope();
    if request_scope != policy.expected_request_scope() {
        let commit_flags = request_owner.reduced_flags();
        let destroy = destroy_native_primary_plane_resources(device, resource_bundle);
        let mut result = LibdrmNativePrimaryPlaneScanoutPrepareResult::from_descriptor(
            LibdrmNativePrimaryPlaneScanoutPrepareStatus::AtomicRequestBuildFailed,
            selection.status,
            scanout_buffer,
            descriptor,
        );
        result.properties = Some(properties.status);
        result.format_table = Some(format_table);
        result.resources = Some(resources.status);
        result.framebuffer = resources.framebuffer;
        result.request = Some(LibdrmNativeAtomicRequestBuildStatus::Built);
        result.request_scope = Some(request_scope);
        result.commit_flags = Some(commit_flags);
        result.cleanup = destroy.cleanup;
        return result;
    }
    let commit_flags = request_owner.reduced_flags();
    LibdrmNativePrimaryPlaneScanoutPrepareResult {
        status: LibdrmNativePrimaryPlaneScanoutPrepareStatus::Prepared,
        selection: selection.status,
        scanout_buffer,
        buffer_format: Some(LibdrmNativeScanoutBufferFormatDetail::from_descriptor(
            descriptor,
        )),
        buffer_modifier: Some(LibdrmNativeScanoutBufferModifierDetail::from_descriptor(
            descriptor,
        )),
        buffer_planes: Some(LibdrmNativeScanoutBufferPlaneDetail::from_descriptor(
            descriptor,
        )),
        properties: Some(properties.status),
        format_table: Some(format_table),
        resources: Some(resources.status),
        framebuffer: resources.framebuffer,
        framebuffer_rejection: None,
        request: Some(LibdrmNativeAtomicRequestBuildStatus::Built),
        request_scope: Some(request_scope),
        commit_flags: Some(commit_flags),
        prepared: Some(LibdrmNativePrimaryPlanePreparedScanout {
            descriptor,
            selection: selection.status,
            properties: properties.status,
            format_table,
            resources_status: resources.status,
            framebuffer: resources.framebuffer,
            request_scope,
            commit_flags,
            selected,
            property_handles,
            resources: resource_bundle,
            request: request_owner,
            retry_without_cursor: retry_without_cursor.flatten(),
        }),
        cleanup: None,
    }
}

fn apply_scanout_submit_policy(
    mut request: LibdrmNativeAtomicCommitRequest,
    policy: LibdrmNativePrimaryPlaneScanoutSubmitPolicy,
) -> LibdrmNativeAtomicCommitRequest {
    if policy.allow_modeset {
        request = request.allow_modeset();
    }
    if !policy.page_flip_event {
        request = request.without_page_flip_event();
    }
    if !policy.nonblocking {
        request = request.blocking();
    }
    if policy.test_only {
        request = request.test_only();
    }
    request
}

#[expect(
    clippy::result_large_err,
    reason = "the error is the prepared scanout handed back for reuse. Boxing \
it would allocate on the submit path to move a value the caller already owns \
and is about to use again"
)]
pub fn prepare_native_topology_head_from_prepared_scanout(
    prepared: LibdrmNativePrimaryPlanePreparedScanout,
    vrr_enabled: Option<bool>,
) -> Result<LibdrmNativePrimaryPlanePreparedTopologyHead, LibdrmNativePrimaryPlanePreparedScanout> {
    if prepared.request_scope != LibdrmNativeAtomicCommitRequestScope::Modeset
        || prepared.resources.mode_blob.is_none_or(|blob| blob == 0)
    {
        return Err(prepared);
    }
    let mut atomic_head = LibdrmNativeAtomicHead::new(
        prepared.resources.into_objects(prepared.selected),
        prepared.property_handles,
    );
    if let Some(enabled) = vrr_enabled {
        atomic_head = atomic_head.with_vrr(enabled);
    }
    Ok(LibdrmNativePrimaryPlanePreparedTopologyHead {
        atomic_head,
        resources: prepared.resources,
    })
}

/// Transfers prepared resources into the ordinary page-flip retirement owner
/// after the containing card-scoped topology request was accepted.
pub fn adopt_prepared_native_topology_head_after_commit(
    prepared: LibdrmNativePrimaryPlanePreparedTopologyHead,
) -> LibdrmNativePrimaryPlaneScanoutSubmission {
    LibdrmNativePrimaryPlaneScanoutSubmission {
        resources: prepared.resources,
        completion_fence: None,
    }
}

pub fn cancel_prepared_native_topology_head<D>(
    device: &D,
    prepared: LibdrmNativePrimaryPlanePreparedTopologyHead,
) -> LibdrmNativePrimaryPlaneResourceDestroyReport
where
    D: LibdrmNativePrimaryPlaneResourceDevice,
{
    destroy_native_primary_plane_resources(device, prepared.resources)
}

/// Validates once and returns the affine owner for submission or cancellation.
#[cfg(feature = "libdrm-events")]
pub fn validate_prepared_native_primary_plane_scanout<D>(
    device: &D,
    prepared: LibdrmNativePrimaryPlanePreparedScanout,
) -> (
    LibdrmNativeAtomicCommitSubmitStatus,
    LibdrmNativePrimaryPlanePreparedScanout,
)
where
    D: LibdrmNativeAtomicCommitDevice,
{
    let (report, prepared) =
        validate_prepared_native_primary_plane_scanout_detailed(device, prepared);
    (report.status, prepared)
}

/// Preserves the actual test error and flags without changing the committing request.
/// Both outcomes return the prepared owner without transferring or disposing its resources.
#[cfg(feature = "libdrm-events")]
pub fn validate_prepared_native_primary_plane_scanout_detailed<D>(
    device: &D,
    prepared: LibdrmNativePrimaryPlanePreparedScanout,
) -> (
    LibdrmNativeAtomicTestReport,
    LibdrmNativePrimaryPlanePreparedScanout,
)
where
    D: LibdrmNativeAtomicCommitDevice,
{
    let request = prepared.request.clone().test_only();
    let request_evidence = request.evidence();
    let request_scope = request.reduced_scope();
    let commit_flags = request.reduced_flags();
    let (flags, native) = request.into_native();
    let (status, error_kind, raw_os_error) = match device.submit_atomic_commit(flags, native) {
        Ok(()) => (LibdrmNativeAtomicCommitSubmitStatus::Submitted, None, None),
        Err(error) => {
            let status = if error.kind() == std::io::ErrorKind::WouldBlock {
                LibdrmNativeAtomicCommitSubmitStatus::WouldBlock
            } else {
                LibdrmNativeAtomicCommitSubmitStatus::Rejected
            };
            (status, Some(error.kind()), error.raw_os_error())
        }
    };
    (
        LibdrmNativeAtomicTestReport {
            status,
            request: request_evidence,
            error_kind,
            raw_os_error,
            request_scope,
            commit_flags,
        },
        prepared,
    )
}

fn submit_atomic_commit_with_optional_out_fence<D>(
    device: &D,
    flags: drm::control::AtomicCommitFlags,
    request: drm::control::atomic::AtomicModeReq,
    selected: LibdrmNativePrimaryPlaneSelection,
    properties: LibdrmNativePrimaryPlanePropertyHandles,
    commit_flags: LibdrmNativeAtomicCommitFlagsReport,
) -> std::io::Result<Option<OwnedFd>>
where
    D: LibdrmNativeAtomicCommitDevice,
{
    if let Some(out_fence_property) = properties.crtc_out_fence_ptr().filter(|_| {
        commit_flags.page_flip_event && commit_flags.nonblocking && !commit_flags.test_only
    }) {
        device.submit_atomic_commit_with_out_fence(
            flags,
            request,
            selected.crtc_handle(),
            out_fence_property,
        )
    } else {
        device.submit_atomic_commit(flags, request)?;
        Ok(None)
    }
}

pub fn submit_prepared_native_primary_plane_scanout<D>(
    device: &D,
    mut prepared: LibdrmNativePrimaryPlanePreparedScanout,
) -> LibdrmNativePrimaryPlaneScanoutSubmitResult
where
    D: LibdrmNativeAtomicCommitDevice + LibdrmNativePrimaryPlaneResourceDevice,
{
    let (flags, request) = prepared.request.into_native();
    let mut cursor_dropped = false;
    let mut completion_fence = None;
    let mut submit = match submit_atomic_commit_with_optional_out_fence(
        device,
        flags,
        request,
        prepared.selected,
        prepared.property_handles,
        prepared.commit_flags,
    ) {
        Ok(fence) => {
            completion_fence = fence;
            LibdrmNativeAtomicCommitSubmitStatus::Submitted
        }
        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
            LibdrmNativeAtomicCommitSubmitStatus::WouldBlock
        }
        Err(_) => LibdrmNativeAtomicCommitSubmitStatus::Rejected,
    };
    // A rejected combined commit retries with the primary alone. The frame
    // survives and the cursor stays pending for a later commit -- the
    // model's NoFrameLostToCursor, as a second submit rather than a hope.
    if submit == LibdrmNativeAtomicCommitSubmitStatus::Rejected
        && let Some(retry) = prepared.retry_without_cursor.take()
    {
        let (flags, request) = retry.into_native();
        if let Ok(fence) = submit_atomic_commit_with_optional_out_fence(
            device,
            flags,
            request,
            prepared.selected,
            prepared.property_handles,
            prepared.commit_flags,
        ) {
            submit = LibdrmNativeAtomicCommitSubmitStatus::Submitted;
            cursor_dropped = true;
            completion_fence = fence;
        }
    }
    let mut result = LibdrmNativePrimaryPlaneScanoutSubmitResult::from_descriptor(
        if submit == LibdrmNativeAtomicCommitSubmitStatus::Submitted {
            LibdrmNativePrimaryPlaneScanoutSubmitStatus::SubmittedWaitingForPageFlip
        } else {
            LibdrmNativePrimaryPlaneScanoutSubmitStatus::AtomicSubmitFailed
        },
        prepared.selection,
        LiveRendererScanoutBufferStatus::Ready,
        prepared.descriptor,
    );
    result.properties = Some(prepared.properties);
    result.format_table = Some(prepared.format_table);
    result.resources = Some(prepared.resources_status);
    result.framebuffer = prepared.framebuffer;
    result.request = Some(LibdrmNativeAtomicRequestBuildStatus::Built);
    result.request_scope = Some(prepared.request_scope);
    result.commit_flags = Some(prepared.commit_flags);
    result.submit = Some(submit);
    result.cursor_dropped = cursor_dropped;
    if submit == LibdrmNativeAtomicCommitSubmitStatus::Submitted {
        result.submission = Some(LibdrmNativePrimaryPlaneScanoutSubmission {
            resources: prepared.resources,
            completion_fence,
        });
    } else {
        result.cleanup = destroy_native_primary_plane_resources(device, prepared.resources).cleanup;
    }
    result
}

pub fn cancel_prepared_native_primary_plane_scanout<D>(
    device: &D,
    prepared: LibdrmNativePrimaryPlanePreparedScanout,
) -> LibdrmNativePrimaryPlaneResourceDestroyReport
where
    D: LibdrmNativePrimaryPlaneResourceDevice,
{
    destroy_native_primary_plane_resources(device, prepared.resources)
}

/// Put a cursor on its plane, and nothing else.
///
/// Returns when the commit has been applied, because it blocks -- so the CRTC
/// is free by the time the caller reads the answer, and the owner never has
/// to guess at a completion it did not observe.
///
/// A refusal is reported rather than raised. The cursor's position stays
/// pending and the next commit carries it; a pointer that stutters is not a
/// reason to fail a session, and a cursor must never cost a frame.
#[cfg(all(feature = "libdrm-events", feature = "gbm-probe"))]
pub fn submit_native_cursor_only_commit<D>(
    device: &D,
    request: LibdrmNativeAtomicCommitRequest,
) -> LibdrmNativeAtomicCommitSubmitStatus
where
    D: LibdrmNativeAtomicCommitDevice,
{
    let (flags, request) = request.into_native();
    match device.submit_atomic_commit(flags, request) {
        Ok(()) => LibdrmNativeAtomicCommitSubmitStatus::Submitted,
        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
            LibdrmNativeAtomicCommitSubmitStatus::WouldBlock
        }
        Err(_) => LibdrmNativeAtomicCommitSubmitStatus::Rejected,
    }
}
