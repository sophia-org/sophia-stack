use super::*;

impl RealAtomicScanoutPageFlipSession {
    #[cfg(all(feature = "gbm-probe", feature = "libdrm-events"))]
    pub fn preferred_xrgb8888_scanout_modifiers(&self) -> Vec<u64> {
        self.preferred_xrgb8888_scanout_modifiers_for_selection(self.selection())
    }

    #[cfg(all(feature = "gbm-probe", feature = "libdrm-events"))]
    pub fn preferred_xrgb8888_scanout_modifiers_for_selection(
        &self,
        selection: LibdrmNativePrimaryPlaneSelection,
    ) -> Vec<u64> {
        self.scanout_format_capabilities_for_selection(selection)
            .preferred_xrgb8888_modifiers
    }

    #[cfg(all(feature = "gbm-probe", feature = "libdrm-events"))]
    pub fn scanout_format_capabilities_for_selection(
        &self,
        selection: LibdrmNativePrimaryPlaneSelection,
    ) -> LibdrmNativePlaneFormatCapabilities {
        let plane = selection.plane.into();
        match self.read_scanout_format_blob_for_selection(selection) {
            Ok((blob_id, blob)) => {
                LibdrmNativePlaneFormatCapabilities::parse(plane, blob_id, &blob)
            }
            Err(reason) => LibdrmNativePlaneFormatCapabilities::unavailable(plane, reason),
        }
    }

    #[cfg(all(feature = "gbm-probe", feature = "libdrm-events"))]
    fn read_scanout_format_blob_for_selection(
        &self,
        selection: LibdrmNativePrimaryPlaneSelection,
    ) -> Result<(u64, Vec<u8>), LibdrmNativePlaneFormatSnapshotUnknown> {
        use LibdrmNativePlaneFormatSnapshotUnknown as Unknown;
        let discovery = discover_native_primary_plane_property_handles(
            &self.card,
            selection.connector,
            selection.crtc,
            selection.plane,
        );
        let properties = discovery.properties.ok_or(
            if discovery.status == LibdrmNativePrimaryPlanePropertyDiscoveryStatus::ReadFailed {
                Unknown::ReadFailed
            } else {
                Unknown::Unavailable
            },
        )?;
        let in_formats = properties.plane_in_formats().ok_or(Unknown::Unavailable)?;
        let plane_properties = drm::control::Device::get_properties(&self.card, selection.plane)
            .map_err(|_| Unknown::ReadFailed)?;
        let blob_id = plane_properties
            .iter()
            .find_map(|(property, value)| (*property == in_formats).then_some(*value))
            .filter(|id| *id != 0)
            .ok_or(Unknown::Unavailable)?;
        let blob = drm::control::Device::get_property_blob(&self.card, blob_id)
            .map_err(|_| Unknown::ReadFailed)?;
        Ok((blob_id, blob))
    }
}
