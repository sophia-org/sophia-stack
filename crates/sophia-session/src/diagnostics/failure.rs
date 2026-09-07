use sophia_renderer_live::LiveRendererScanoutBufferExportDetail as Detail;

// Only compiler-owned error variants and exact internal invariant messages
// cross into ordinary records. Never copy arbitrary Display or Debug text.
const RENDERER_CODES: &[(Detail, &str)] = &[
    (Detail::Exported, "renderer_exported"),
    (Detail::WorkerPending, "renderer_worker_pending"),
    (Detail::WorkerQueueFull, "renderer_worker_queue_full"),
    (Detail::WorkerDisconnected, "renderer_worker_disconnected"),
    (Detail::WorkerStalled, "renderer_worker_stalled"),
    (Detail::InvalidTarget, "renderer_invalid_target"),
    (Detail::ComposeRefused, "renderer_compose_refused"),
    (
        Detail::BackendDeviceUnavailable,
        "renderer_backend_device_unavailable",
    ),
    (
        Detail::GbmDeviceUnavailable,
        "renderer_gbm_device_unavailable",
    ),
    (Detail::EglUnavailable, "renderer_egl_unavailable"),
    (
        Detail::EglDisplayUnavailable,
        "renderer_egl_display_unavailable",
    ),
    (
        Detail::EglInitializeFailed,
        "renderer_egl_initialize_failed",
    ),
    (Detail::EglBindApiFailed, "renderer_egl_bind_api_failed"),
    (
        Detail::EglConfigUnavailable,
        "renderer_egl_config_unavailable",
    ),
    (
        Detail::GbmSurfaceUnavailable,
        "renderer_gbm_surface_unavailable",
    ),
    (
        Detail::EglSurfaceUnavailable,
        "renderer_egl_surface_unavailable",
    ),
    (
        Detail::EglContextUnavailable,
        "renderer_egl_context_unavailable",
    ),
    (
        Detail::EglMakeCurrentFailed,
        "renderer_egl_make_current_failed",
    ),
    (Detail::GlSmokeFailed, "renderer_gl_smoke_failed"),
    (
        Detail::CpuLayerUploadFailed,
        "renderer_cpu_layer_upload_failed",
    ),
    (
        Detail::DmaBufImageCreateFailed,
        "renderer_dma_buf_image_create_failed",
    ),
    (
        Detail::DmaBufImageBindFailed,
        "renderer_dma_buf_image_bind_failed",
    ),
    (
        Detail::CompositionDrawFailed,
        "renderer_composition_draw_failed",
    ),
    (
        Detail::CompositionFinishFailed,
        "renderer_composition_finish_failed",
    ),
    (
        Detail::EglImageDestroyFailed,
        "renderer_egl_image_destroy_failed",
    ),
    (Detail::DmaBufImportFailed, "renderer_dma_buf_import_failed"),
    (
        Detail::EglSwapBuffersFailed,
        "renderer_egl_swap_buffers_failed",
    ),
    (
        Detail::FrontBufferLockFailed,
        "renderer_front_buffer_lock_failed",
    ),
    (
        Detail::InvalidBufferDescriptor,
        "renderer_invalid_buffer_descriptor",
    ),
    (
        Detail::InvalidRendererImageId,
        "renderer_invalid_renderer_image_id",
    ),
    (
        Detail::DmaBufDescriptorMismatch,
        "renderer_dma_buf_descriptor_mismatch",
    ),
    (
        Detail::DmaBufImportCacheFull,
        "renderer_dma_buf_import_cache_full",
    ),
    (
        Detail::RendererImageStoreFull,
        "renderer_renderer_image_store_full",
    ),
    (
        Detail::RetainedBufferMissing,
        "renderer_retained_buffer_missing",
    ),
];
const INVARIANT_CODES: &[(&str, &str)] = &[
    (
        "renderer-image handoff targets an unknown output",
        "handoff_unknown_output",
    ),
    (
        "retained scene refers to an unavailable promoted renderer image",
        "handoff_missing_image",
    ),
    (
        "renderer-image handoff does not cover the retained scene",
        "handoff_coverage_mismatch",
    ),
    (
        "renderer-image handoff is unexpectedly missing",
        "handoff_missing",
    ),
];

pub fn failure_code(error: &(dyn std::error::Error + 'static)) -> &'static str {
    if let Some(detail) = error.downcast_ref::<Detail>() {
        return RENDERER_CODES
            .iter()
            .find(|(candidate, _)| candidate == detail)
            .map_or("unclassified", |(_, code)| *code);
    }
    let message = error.to_string();
    INVARIANT_CODES
        .iter()
        .find(|(candidate, _)| *candidate == message)
        .map_or("unclassified", |(_, code)| *code)
}

pub(super) fn approved_failure_code(value: &str) -> bool {
    value == "unclassified"
        || RENDERER_CODES.iter().any(|(_, code)| *code == value)
        || INVARIANT_CODES.iter().any(|(_, code)| *code == value)
}
