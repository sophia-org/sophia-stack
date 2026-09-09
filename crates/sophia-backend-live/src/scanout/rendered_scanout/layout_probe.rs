use super::*;
use crate::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LiveScanoutLayoutProbeReport {
    pub source_image: sophia_renderer_live::LiveRendererImageId,
    pub original: crate::LiveRendererFrameCorrelation,
    pub alternative: crate::LiveRendererFrameCorrelation,
    pub format: u32,
    pub original_modifier: u64,
    pub alternative_modifier: u64,
    pub tests: LibdrmNativeAtomicTestPairReport,
}

/// A comparison carried by its accepted alternative; presentation is established separately.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LiveScanoutLayoutWitness {
    pub source_image: sophia_renderer_live::LiveRendererImageId,
    pub alternative: crate::LiveRendererFrameCorrelation,
    pub format: u32,
    pub original_modifier: u64,
    pub alternative_modifier: u64,
}

impl LiveScanoutLayoutProbeReport {
    pub(super) fn witness_for<Owner>(
        &self,
        prepared: &LivePreparedRenderedPrimaryPlaneScanout<Owner>,
    ) -> Option<LiveScanoutLayoutWitness> {
        self.witness_for_current(
            prepared.correlation(),
            prepared.primary_plane.descriptor(),
            prepared.primary_plane.test_request_evidence()?,
        )
    }

    fn witness_for_current(
        &self,
        correlation: Option<crate::LiveRendererFrameCorrelation>,
        descriptor: LiveRendererScanoutBufferDescriptor,
        current: LibdrmNativeAtomicRequestEvidence,
    ) -> Option<LiveScanoutLayoutWitness> {
        let original = self.tests.original?;
        let alternative = self.tests.alternative?;
        let original_request = original.request?;
        let alternative_request = alternative.request?;
        if self.tests.status != LibdrmNativeAtomicTestPairStatus::Tested
            || original.status != LibdrmNativeAtomicCommitSubmitStatus::Rejected
            || original.raw_os_error != Some(22)
            || original.error_kind != Some(std::io::ErrorKind::InvalidInput)
            || alternative.status != LibdrmNativeAtomicCommitSubmitStatus::Submitted
            || alternative.raw_os_error.is_some()
            || alternative.error_kind.is_some()
            || !original_request.equivalent_except_primary_framebuffer(&alternative_request)
            || current != alternative_request
            || correlation != Some(self.alternative)
            || self.original.trace.is_none()
            || self.original.trace != self.alternative.trace
            || self.original.direct_scanout != Some(sophia_engine::DirectScanoutVerdict::Eligible)
            || self.alternative.direct_scanout
                != Some(sophia_engine::DirectScanoutVerdict::CompositionRequired(
                    "refused",
                ))
            || descriptor.format != self.format
            || descriptor.modifier != Some(self.alternative_modifier)
            || self.original_modifier == self.alternative_modifier
            || [self.original_modifier, self.alternative_modifier]
                .iter()
                .any(|modifier| {
                    *modifier == sophia_protocol::DRM_FORMAT_MOD_INVALID || *modifier == u64::MAX
                })
        {
            return None;
        }
        Some(LiveScanoutLayoutWitness {
            source_image: self.source_image,
            alternative: self.alternative,
            format: self.format,
            original_modifier: self.original_modifier,
            alternative_modifier: self.alternative_modifier,
        })
    }
}

/// Retries one optional resource obligation without delaying ordinary frames.
pub fn retry_scanout_layout_probe_cleanup<D, E>(device: &D, exporter: &mut E) -> bool
where
    D: LibdrmNativePrimaryPlaneResourceDevice,
    E: LiveRenderedScanoutBufferExporter,
{
    let Some(slot) = exporter.layout_probe_cleanup() else {
        return false;
    };
    if let Some(cleanup) = slot.take() {
        *slot = retry_rendered_primary_plane_scanout_cleanup(device, cleanup).cleanup;
    }
    slot.is_some()
}

// The ordinary fallback already owns its rendered pixels. Only the original
// client's framebuffer is prepared temporarily; no render or copy is added.
#[allow(clippy::too_many_arguments)]
pub(super) fn probe_rendered_scanout_layout<D, E>(
    scanout_target: LiveKmsScanoutTargetStatus,
    target: Option<LiveGbmEglFrameTargetRecord>,
    selection: LibdrmNativePrimaryPlaneSelectionResult,
    vrr_enabled: Option<bool>,
    cursor_ride: Option<LibdrmNativeAtomicCursor>,
    device: &D,
    exporter: &mut E,
    mut alternative: LivePreparedRenderedPrimaryPlaneScanout<E::Owner>,
) -> LivePreparedRenderedPrimaryPlaneScanout<E::Owner>
where
    D: LibdrmNativePropertyLookupDevice
        + LibdrmNativePrimaryPlaneResourceDevice
        + LibdrmNativeAtomicCommitDevice,
    E: LiveRenderedScanoutBufferExporter,
    E::Owner: LiveRenderedScanoutBufferPrimeSource,
{
    if alternative.scanout_buffer.is_direct_client_buffer()
        || exporter.layout_probe_cleanup().is_none()
        || retry_scanout_layout_probe_cleanup(device, exporter)
    {
        return alternative;
    }
    let descriptor = alternative.primary_plane.descriptor();
    let Some(source) = exporter.take_layout_probe_source(alternative.correlation(), descriptor)
    else {
        return alternative;
    };
    let source_image = source.image;
    let source = source.export.normalized();
    let (Some(original_descriptor), Some(original), Some(completed)) = (
        source.descriptor,
        source.correlation,
        alternative.correlation(),
    ) else {
        return alternative;
    };
    let (Some(original_modifier), Some(alternative_modifier)) =
        (original_descriptor.modifier, descriptor.modifier)
    else {
        return alternative;
    };
    if original_descriptor.format != descriptor.format
        || original_descriptor.size != descriptor.size
        || original_modifier == alternative_modifier
        || [original_modifier, alternative_modifier]
            .iter()
            .any(|modifier| {
                *modifier == sophia_protocol::DRM_FORMAT_MOD_INVALID || *modifier == u64::MAX
            })
        || original.trace.is_none()
        || original.trace != completed.trace
        || completed.direct_scanout
            != Some(sophia_engine::DirectScanoutVerdict::CompositionRequired(
                "refused",
            ))
        || !original
            .direct_scanout
            .is_some_and(|verdict| verdict.is_eligible())
    {
        return alternative;
    }
    let mut source_exporter = ProbeSourceExporter(Some(source));
    let mut prepared = prepare_rendered_primary_plane_scanout_from_target_and_selection_with_cursor(
        scanout_target,
        target,
        selection,
        vrr_enabled,
        cursor_ride,
        device,
        &mut source_exporter,
    );
    let Some(source) = prepared.prepared.take() else {
        *exporter
            .layout_probe_cleanup()
            .expect("probe owner was admitted") = prepared.cleanup;
        return alternative;
    };
    let (tests, original_plane, alternative_plane) =
        validate_prepared_native_primary_plane_scanout_pair(
            device,
            source.primary_plane,
            alternative.primary_plane,
        );
    alternative.primary_plane = alternative_plane;
    let cancelled = cancel_prepared_native_primary_plane_scanout(device, original_plane);
    *exporter
        .layout_probe_cleanup()
        .expect("probe owner was admitted") =
        cancelled
            .cleanup
            .map(|primary_plane| LiveRenderedPrimaryPlaneScanoutCleanup {
                scanout_buffer: source.scanout_buffer,
                primary_plane,
            });
    let report = LiveScanoutLayoutProbeReport {
        source_image,
        original,
        alternative: completed,
        format: descriptor.format,
        original_modifier,
        alternative_modifier,
        tests,
    };
    alternative.layout_probe = Some(Box::new(report));
    exporter.record_layout_probe(report);
    alternative
}

struct ProbeSourceExporter<Owner>(Option<LiveRenderedScanoutBufferExport<Owner>>);

impl<Owner> LiveRenderedScanoutBufferExporter for ProbeSourceExporter<Owner> {
    type Owner = Owner;

    fn export_rendered_scanout_buffer(
        &mut self,
        _target: LiveGbmEglFrameTargetRecord,
    ) -> LiveRenderedScanoutBufferExport<Owner> {
        self.0.take().expect("one preparation consumes one source")
    }
}

#[path = "../../../tests/support/layout_witness_reduction.rs"]
mod tests;
