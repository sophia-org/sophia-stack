use sophia_protocol::SurfaceId;

/// An advisory DRM device number. It grants no device access or import rights.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XDrmDeviceHint {
    pub major: u32,
    pub minor: u32,
}

/// Renderer-qualified allocation preferences for one exact surface incarnation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XWindowAllocationPreference {
    pub surface: SurfaceId,
    pub device: XDrmDeviceHint,
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
