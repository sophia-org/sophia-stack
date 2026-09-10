#[derive(Clone, Copy, Debug)]
pub(crate) struct XPresentAllocationSubject {
    pub(crate) client_id: u64,
    pub(crate) transaction: TransactionId,
    pub(crate) window: crate::XResourceId,
    pub(crate) pixmap: crate::XResourceId,
    namespace: NamespaceId,
    window_surface: sophia_protocol::SurfaceId,
    published_surface: sophia_protocol::SurfaceId,
    root: crate::XResourceId,
    child_offset: (i32, i32),
    buffer: sophia_protocol::BufferHandle,
    format: u32,
    modifier: u64,
}

impl XAuthorityRuntime {
    /// Claims one reallocation notice per surface in the accepted preference generation.
    pub(crate) fn claim_present_reallocation(
        &mut self,
        subject: XPresentAllocationSubject,
        comparison: crate::XPresentLayoutComparison,
    ) -> bool {
        if !self.compare_present_layout(subject, comparison) {
            return false;
        }
        let Some(entry) = self
            .window_allocation
            .preferences
            .get_mut(&subject.window_surface)
        else {
            return false;
        };
        if entry.reallocation_claimed {
            return false;
        }
        entry.reallocation_claimed = true;
        true
    }

    pub(crate) fn present_allocation_subject(
        &self,
        namespace: NamespaceId,
        client_id: u64,
        window: crate::XResourceId,
        pixmap: crate::XResourceId,
        present: &crate::XAuthorityPresentSubmission,
    ) -> Option<XPresentAllocationSubject> {
        let descriptor = self.dri3_pixmap_descriptor(namespace, pixmap).ok()?;
        let (root, surface, x, y) = self
            .window_presentation_root_and_offset(namespace, window)
            .ok()?;
        if descriptor.handle != present.buffer || surface != present.surface {
            return None;
        }
        Some(XPresentAllocationSubject {
            client_id,
            transaction: present.transaction,
            window,
            pixmap,
            namespace,
            window_surface: self.windows.get(window)?.surface,
            published_surface: surface,
            root,
            child_offset: (x, y),
            buffer: descriptor.handle,
            format: descriptor.format,
            modifier: descriptor.modifier,
        })
    }

    pub(crate) fn compare_present_layout(
        &self,
        subject: XPresentAllocationSubject,
        comparison: crate::XPresentLayoutComparison,
    ) -> bool {
        if comparison.surface != subject.published_surface
            || comparison.buffer != subject.buffer
            || comparison.format != subject.format
            || comparison.original_modifier != subject.modifier
            || comparison.preference_generation != self.window_allocation.generation
            || comparison.topology_generation != self.output_topology.generation
            || comparison.native_context.generation == 0
            || comparison.native_context.output == sophia_protocol::OutputId::INVALID
            || comparison.original_modifier == comparison.alternative_modifier
            || matches!(
                comparison.original_modifier,
                sophia_protocol::DRM_FORMAT_MOD_INVALID | u64::MAX
            )
            || matches!(
                comparison.alternative_modifier,
                sophia_protocol::DRM_FORMAT_MOD_INVALID | u64::MAX
            )
        {
            return false;
        }
        let Some(window) = self.windows.get(subject.window) else {
            return false;
        };
        if window.surface != subject.window_surface
            || window.surface != subject.published_surface
            || window.map_state != crate::XMapState::Viewable
            || window.policy_map_pending
        {
            return false;
        }
        let Ok((root, surface, x, y)) =
            self.window_presentation_root_and_offset(subject.namespace, subject.window)
        else {
            return false;
        };
        if root != subject.root
            || surface != subject.published_surface
            || (x, y) != subject.child_offset
        {
            return false;
        }
        // The presentation root is a child of the X root; its rectangle is global.
        let Some(root) = self.windows.get(root) else {
            return false;
        };
        if root.geometry != comparison.geometry
            || root.map_state != crate::XMapState::Viewable
            || root.policy_map_pending
        {
            return false;
        }
        let Some(effective) = self.effective_window_allocation_preference(
            subject.namespace,
            subject.client_id,
            subject.window,
            subject.format,
        ) else {
            return false;
        };
        effective.same_device
            && effective.preference.identity == Some(comparison.device_identity)
            && effective.preference.context == Some(comparison.native_context)
            && effective
                .screen_modifiers
                .binary_search(&comparison.alternative_modifier)
                .is_ok()
            && effective
                .window_modifiers
                .binary_search(&comparison.alternative_modifier)
                .is_ok()
            && !(effective
                .screen_modifiers
                .binary_search(&comparison.original_modifier)
                .is_ok()
                && effective
                    .window_modifiers
                    .binary_search(&comparison.original_modifier)
                    .is_ok())
    }
}
