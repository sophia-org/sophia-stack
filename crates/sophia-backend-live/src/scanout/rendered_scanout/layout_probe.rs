use super::*;
use crate::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LiveScanoutLayoutProbeReport {
    pub original: crate::LiveRendererFrameCorrelation,
    pub alternative: crate::LiveRendererFrameCorrelation,
    pub format: u32,
    pub original_modifier: u64,
    pub alternative_modifier: u64,
    pub tests: LibdrmNativeAtomicTestPairReport,
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
    let source = source.normalized();
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
    exporter.record_layout_probe(LiveScanoutLayoutProbeReport {
        original,
        alternative: completed,
        format: descriptor.format,
        original_modifier,
        alternative_modifier,
        tests,
    });
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
