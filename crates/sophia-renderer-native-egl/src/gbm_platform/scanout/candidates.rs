#[derive(Clone, Copy)]
struct NativeXrgb8888Frame<'a> {
    stride: u32,
    pixels: &'a [u8],
}

fn create_rendered_scanout_surface<T: std::os::fd::AsFd>(
    gbm_device: &gbm::Device<T>,
    width: u32,
    height: u32,
    format: gbm::Format,
    modifiers: &[gbm::Modifier],
    usage: gbm::BufferObjectFlags,
) -> Result<gbm::Surface<()>, NativeGbmScanoutBufferExportDetail> {
    if modifiers.is_empty() {
        gbm_device
            .create_surface::<()>(width, height, format, usage)
            .map_err(|_error| NativeGbmScanoutBufferExportDetail::GbmSurfaceUnavailable)
    } else {
        gbm_device
            .create_surface_with_modifiers2::<()>(
                width,
                height,
                format,
                modifiers.iter().copied(),
                usage,
            )
            .map_err(|_error| NativeGbmScanoutBufferExportDetail::GbmSurfaceUnavailable)
    }
}

fn choose_scanout_config_for_format(
    egl: &khronos_egl::DynamicInstance<khronos_egl::EGL1_5>,
    display: khronos_egl::Display,
    config_attributes: [khronos_egl::Int; 13],
    format: gbm::Format,
) -> Option<khronos_egl::Config> {
    let count = egl
        .matching_config_count(display, &config_attributes)
        .ok()?;
    let mut configs = Vec::with_capacity(count);
    egl.choose_config(display, &config_attributes, &mut configs)
        .ok()?;
    configs.into_iter().find(|config| {
        egl.get_config_attrib(display, *config, khronos_egl::NATIVE_VISUAL_ID)
            .ok()
            == Some(format as khronos_egl::Int)
    })
}

fn reduced_gbm_scanout_modifiers(modifiers: &[u64]) -> Vec<gbm::Modifier> {
    let mut reduced = Vec::new();
    for modifier in modifiers.iter().copied().map(gbm::Modifier::from) {
        if matches!(modifier, gbm::Modifier::Invalid)
            || u64::from(modifier) == u64::MAX
            || reduced.contains(&modifier)
        {
            continue;
        }
        reduced.push(modifier);
        if reduced.len() >= MAX_PREFERRED_SCANOUT_MODIFIERS {
            break;
        }
    }
    reduced
}

const MAX_PREFERRED_SCANOUT_MODIFIERS: usize = 16;

fn is_supported_rendered_scanout_candidate_buffer(buffer: &NativeGbmOwnedScanoutBuffer) -> bool {
    is_supported_rendered_scanout_candidate_shape(buffer.plane_count())
}

const fn is_supported_rendered_scanout_candidate_shape(plane_count: u8) -> bool {
    plane_count > 0 && plane_count <= MAX_RENDERED_SCANOUT_PLANES
}

const MAX_RENDERED_SCANOUT_PLANES: u8 = 4;

fn preferred_scanout_failure_detail(
    current: NativeGbmScanoutBufferExportDetail,
    next: NativeGbmScanoutBufferExportDetail,
) -> NativeGbmScanoutBufferExportDetail {
    if current == NativeGbmScanoutBufferExportDetail::EglConfigUnavailable {
        next
    } else {
        current
    }
}

fn is_supported_scanout_format(format: u32) -> bool {
    format == gbm::Format::Xrgb8888 as u32 || format == gbm::Format::Argb8888 as u32
}

fn exported_scanout_buffer_report(
    buffer: NativeGbmOwnedScanoutBuffer,
) -> NativeGbmOwnedScanoutBufferExportReport {
    NativeGbmOwnedScanoutBufferExportReport {
        status: NativeGbmScanoutBufferExportStatus::Exported,
        detail: NativeGbmScanoutBufferExportDetail::Exported,
        buffer: Some(buffer),
        buffer_age: None,
        target_generation: None,
        repaint: NativeCompositionRepaintOutcome::Full,
    }
}

fn failed_scanout_buffer_report(
    detail: NativeGbmScanoutBufferExportDetail,
) -> NativeGbmOwnedScanoutBufferExportReport {
    NativeGbmOwnedScanoutBufferExportReport {
        status: detail.status(),
        detail,
        buffer: None,
        buffer_age: None,
        target_generation: None,
        repaint: NativeCompositionRepaintOutcome::Full,
    }
}

#[path = "candidates/tests.rs"]
mod candidate_tests;
