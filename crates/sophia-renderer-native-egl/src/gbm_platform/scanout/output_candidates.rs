use super::{window_config_attributes, xrgb_window_config_attributes};

#[derive(Clone)]
pub(super) struct RenderedScanoutCandidate {
    pub(super) format: gbm::Format,
    pub(super) modifiers: Vec<gbm::Modifier>,
    pub(super) usage: gbm::BufferObjectFlags,
    pub(super) config_attributes: [khronos_egl::Int; 13],
}

pub(super) fn rendered_scanout_candidates(
    preferred_modifiers: &[gbm::Modifier],
    requested_format: Option<u32>,
) -> Vec<RenderedScanoutCandidate> {
    let mut candidates = Vec::with_capacity(6);
    candidates.push(RenderedScanoutCandidate {
        format: gbm::Format::Xrgb8888,
        modifiers: Vec::from(LINEAR_SCANOUT_MODIFIERS),
        usage: rendered_scanout_usage().union(gbm::BufferObjectFlags::LINEAR),
        config_attributes: xrgb_window_config_attributes(),
    });
    if !preferred_modifiers.is_empty() {
        candidates.push(RenderedScanoutCandidate {
            format: gbm::Format::Xrgb8888,
            modifiers: preferred_modifiers.to_vec(),
            usage: rendered_scanout_usage(),
            config_attributes: xrgb_window_config_attributes(),
        });
    }
    candidates.extend([
        RenderedScanoutCandidate {
            format: gbm::Format::Xrgb8888,
            modifiers: Vec::new(),
            usage: rendered_scanout_usage().union(gbm::BufferObjectFlags::LINEAR),
            config_attributes: xrgb_window_config_attributes(),
        },
        RenderedScanoutCandidate {
            format: gbm::Format::Xrgb8888,
            modifiers: Vec::new(),
            usage: rendered_scanout_usage(),
            config_attributes: xrgb_window_config_attributes(),
        },
        RenderedScanoutCandidate {
            format: gbm::Format::Argb8888,
            modifiers: Vec::from(LINEAR_SCANOUT_MODIFIERS),
            usage: rendered_scanout_usage().union(gbm::BufferObjectFlags::LINEAR),
            config_attributes: window_config_attributes(),
        },
        RenderedScanoutCandidate {
            format: gbm::Format::Argb8888,
            modifiers: Vec::new(),
            usage: rendered_scanout_usage(),
            config_attributes: window_config_attributes(),
        },
    ]);
    if let Some(format) = requested_format {
        candidates.retain(|candidate| candidate.format as u32 == format);
    }
    candidates
}

const LINEAR_SCANOUT_MODIFIERS: [gbm::Modifier; 1] = [gbm::Modifier::Linear];

fn rendered_scanout_usage() -> gbm::BufferObjectFlags {
    gbm::BufferObjectFlags::SCANOUT | gbm::BufferObjectFlags::RENDERING
}
