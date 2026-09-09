use std::{
    collections::BTreeMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use crate::{
    XServerFrontendDmaBufImportFormat, XServerFrontendPixmapAllocator,
    XServerFrontendRenderDeviceProvider,
};

pub const X_SERVER_FRONTEND_DEVICE_BUNDLE_CAPACITY: usize = 16;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum XServerFrontendDeviceBundleError {
    InvalidGeneration,
    InvalidInventory,
    StaleGeneration,
    Capacity,
    CapabilityMismatch,
    UnknownGeneration,
    Unavailable,
}

impl core::fmt::Display for XServerFrontendDeviceBundleError {
    fn fmt(&self, out: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(out, "X11 device bundle: {self:?}")
    }
}
impl std::error::Error for XServerFrontendDeviceBundleError {}

/// A connection's fixed device and capability contract. Loss changes availability,
/// never the device or the screen modifiers already returned to that connection.
pub struct XServerFrontendDeviceBundle {
    generation: u64,
    pub(crate) provider: Arc<dyn XServerFrontendRenderDeviceProvider>,
    pub(crate) allocator: Option<Arc<dyn XServerFrontendPixmapAllocator>>,
    formats: BTreeMap<u32, Vec<u64>>,
    pixmap_textures: bool,
    available: AtomicBool,
}

impl core::fmt::Debug for XServerFrontendDeviceBundle {
    fn fmt(&self, out: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        out.debug_struct("XServerFrontendDeviceBundle")
            .field("generation", &self.generation)
            .field("formats", &self.formats)
            .field("pixmap_textures", &self.pixmap_textures)
            .field("available", &self.available())
            .finish()
    }
}

impl XServerFrontendDeviceBundle {
    /// Snapshot callbacks run here, before installation or authority locking.
    pub fn new(
        generation: u64,
        provider: Arc<dyn XServerFrontendRenderDeviceProvider>,
        allocator: Option<Arc<dyn XServerFrontendPixmapAllocator>>,
    ) -> Result<Self, XServerFrontendDeviceBundleError> {
        let formats = provider.dma_buf_import_formats();
        Self::from_snapshot(generation, provider, allocator, formats)
    }

    pub(crate) fn from_snapshot(
        generation: u64,
        provider: Arc<dyn XServerFrontendRenderDeviceProvider>,
        allocator: Option<Arc<dyn XServerFrontendPixmapAllocator>>,
        formats: Vec<XServerFrontendDmaBufImportFormat>,
    ) -> Result<Self, XServerFrontendDeviceBundleError> {
        if generation == 0 {
            return Err(XServerFrontendDeviceBundleError::InvalidGeneration);
        }
        let count = formats
            .iter()
            .try_fold(0usize, |sum, row| sum.checked_add(row.modifiers.len()));
        if formats.len() > 512 || count.is_none_or(|count| count > 16_384) {
            return Err(XServerFrontendDeviceBundleError::InvalidInventory);
        }
        let mut canonical = BTreeMap::<u32, Vec<u64>>::new();
        for row in formats {
            canonical
                .entry(row.format)
                .or_default()
                .extend(row.modifiers.into_iter().filter(|value| {
                    *value != sophia_protocol::DRM_FORMAT_MOD_INVALID && *value != u64::MAX
                }));
        }
        for modifiers in canonical.values_mut() {
            modifiers.sort_unstable();
            modifiers.dedup();
        }
        let pixmap_textures = allocator
            .as_ref()
            .is_some_and(|value| value.supports_pixmap_textures());
        Ok(Self {
            generation,
            provider,
            allocator,
            formats: canonical,
            pixmap_textures,
            available: AtomicBool::new(true),
        })
    }

    pub const fn generation(&self) -> u64 {
        self.generation
    }
    pub fn dma_buf_import_modifiers(&self, format: u32) -> &[u64] {
        self.formats.get(&format).map_or(&[], Vec::as_slice)
    }
    pub const fn supports_pixmap_textures(&self) -> bool {
        self.pixmap_textures
    }
    pub(crate) fn available(&self) -> bool {
        self.available.load(Ordering::Acquire)
    }
    pub(crate) fn mark_unavailable(&self) {
        self.available.store(false, Ordering::Release);
    }
}
