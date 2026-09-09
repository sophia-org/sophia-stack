const WINDOW_ALLOCATION_MAX_ROWS: usize = 4096;

#[derive(Debug, Default)]
struct XWindowAllocationState {
    generation: u64,
    topology_generation: u64,
    hints: BTreeMap<sophia_protocol::SurfaceId, crate::XDrmDeviceHint>,
    preferences: BTreeMap<sophia_protocol::SurfaceId, crate::XWindowAllocationPreference>,
}

impl XAuthorityRuntime {
    pub fn set_window_render_device_hint(
        &mut self,
        namespace: NamespaceId,
        window: crate::XResourceId,
        hint: crate::XDrmDeviceHint,
    ) -> Result<(), XAuthorityRuntimeError> {
        if window.local.raw() == u64::from(crate::X_SETUP_DEFAULT_ROOT) {
            return Ok(());
        }
        self.resources
            .lookup(namespace, window, XResourceKind::Window)?;
        let surface = self
            .windows
            .get(window)
            .ok_or(XAuthorityRuntimeError::UnknownResource)?
            .surface;
        self.window_allocation.hints.insert(surface, hint);
        Ok(())
    }

    /// Changes only window preferences; a client's screen formats remain pinned.
    pub fn update_window_allocation_preferences(
        &mut self,
        snapshot: crate::XWindowAllocationPreferences,
    ) -> crate::XWindowAllocationUpdate {
        use crate::XWindowAllocationUpdate as U;
        if snapshot.generation <= self.window_allocation.generation
            || snapshot.topology_generation != self.output_topology.generation
        {
            return U::Stale;
        }
        let modifiers = snapshot
            .windows
            .iter()
            .flat_map(|window| &window.formats)
            .try_fold(0usize, |count, row| count.checked_add(row.modifiers.len()));
        let formats = snapshot.windows.iter().try_fold(0usize, |count, window| {
            count.checked_add(window.formats.len())
        });
        if snapshot.windows.len() > WINDOW_ALLOCATION_MAX_ROWS
            || modifiers.is_none_or(|count| count > DMA_BUF_IMPORT_MAX_MODIFIERS)
            || formats.is_none_or(|count| count > DMA_BUF_IMPORT_MAX_FORMATS)
        {
            return U::Invalid;
        }
        let mut replacement = BTreeMap::new();
        for mut window in snapshot.windows {
            if !window.surface.is_valid()
                || replacement.contains_key(&window.surface)
                || window.identity.is_some_and(|identity| {
                    rustix::fs::major(identity.device_number) != window.device.major
                        || rustix::fs::minor(identity.device_number) != window.device.minor
                })
            {
                return U::Invalid;
            }
            let mut seen = BTreeSet::new();
            for row in &mut window.formats {
                if !matches!(
                    row.format,
                    sophia_protocol::DRM_FORMAT_XRGB8888 | sophia_protocol::DRM_FORMAT_ARGB8888
                ) || !seen.insert(row.format)
                    || row.modifiers.iter().any(|modifier| {
                        matches!(
                            *modifier,
                            sophia_protocol::DRM_FORMAT_MOD_INVALID | u64::MAX
                        )
                    })
                {
                    return U::Invalid;
                }
                row.modifiers.sort_unstable();
                row.modifiers.dedup();
            }
            replacement.insert(window.surface, window);
        }
        self.window_allocation = XWindowAllocationState {
            generation: snapshot.generation,
            topology_generation: snapshot.topology_generation,
            hints: std::mem::take(&mut self.window_allocation.hints),
            preferences: replacement,
        };
        U::Applied
    }

    pub fn window_allocation_modifiers(
        &self,
        namespace: NamespaceId,
        client_id: u64,
        window: crate::XResourceId,
        format: u32,
        screen_modifiers: &[u64],
    ) -> Vec<u64> {
        if self
            .resources
            .lookup(namespace, window, XResourceKind::Window)
            .is_err()
            || self.window_allocation.topology_generation != self.output_topology.generation
        {
            return Vec::new();
        }
        let Some(record) = self.windows.get(window) else {
            return Vec::new();
        };
        let Some(preference) = self.window_allocation.preferences.get(&record.surface) else {
            return Vec::new();
        };
        if self
            .window_allocation
            .hints
            .get(&record.surface)
            .is_some_and(|hint| *hint != preference.device)
        {
            return Vec::new();
        }
        let identity = self
            .device_connections
            .get(&client_id)
            .and_then(|bundle| bundle.as_ref())
            .filter(|bundle| bundle.available())
            .and_then(|bundle| bundle.identity);
        let same_device = identity.is_some() && identity == preference.identity;
        preference
            .formats
            .iter()
            .find(|row| row.format == format)
            .map(|row| {
                row.modifiers
                    .iter()
                    .copied()
                    .filter(|modifier| screen_modifiers.contains(modifier))
                    .filter(|modifier| same_device || *modifier == 0)
                    .collect()
            })
            .unwrap_or_default()
    }
}
