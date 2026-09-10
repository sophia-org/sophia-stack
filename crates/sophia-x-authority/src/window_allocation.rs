use sophia_protocol::SurfaceId;

/// An advisory DRM device number. It grants no device access or import rights.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XDrmDeviceHint {
    pub major: u32,
    pub minor: u32,
}

/// Opaque native allocation context observed for one output.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XWindowAllocationContext {
    pub generation: u64,
    pub output: sophia_protocol::OutputId,
}

/// Renderer-qualified allocation preferences for one exact surface incarnation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XWindowAllocationPreference {
    pub surface: SurfaceId,
    pub device: XDrmDeviceHint,
    pub identity: Option<crate::XRenderDeviceIdentity>,
    pub context: Option<XWindowAllocationContext>,
    pub formats: Vec<crate::XServerFrontendDmaBufImportFormat>,
}

/// Complete replacement, ordered independently from the immutable screen contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XWindowAllocationPreferences {
    pub generation: u64,
    pub topology_generation: u64,
    pub windows: Vec<XWindowAllocationPreference>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum XWindowAllocationUpdate {
    Applied,
    Stale,
    Invalid,
}

/// Retired layout facts compared with the current frontend allocation state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XPresentLayoutComparison {
    pub surface: SurfaceId,
    pub buffer: sophia_protocol::BufferHandle,
    pub format: u32,
    pub original_modifier: u64,
    pub alternative_modifier: u64,
    pub preference_generation: u64,
    pub topology_generation: u64,
    pub native_context: XWindowAllocationContext,
    pub device_identity: crate::XRenderDeviceIdentity,
    pub geometry: sophia_protocol::Rect,
}

/// Informational comparison result; it grants no permission or protocol mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum XPresentLayoutComparisonResult {
    Matched,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XPresentCompleteRouteOutcome {
    pub routed: bool,
    /// Effective completion mode after current frontend validation.
    pub mode: crate::XPresentCompletionMode,
    pub layout_comparison: Option<XPresentLayoutComparisonResult>,
}
