impl PersistentLiveLayout {
    /// Whether a policy-managed surface belongs in the scene.
    ///
    /// Mapping is the authority's fact; policy's projection is a cached view of
    /// it that is not refreshed in the step carrying an unmap. Asking policy
    /// alone therefore keeps a torn-down window compositing and taking pointer
    /// hits until policy catches up, and keeps the popups it owns on screen
    /// with it. Requiring the authority's mapping first means an unmapped
    /// surface leaves the scene on the cycle that unmaps it.
    ///
    /// This is the same requirement the client-positioned path has always
    /// placed on a popup's own mapping, applied to the managed surface too.
    fn managed_scene_visible<E>(
        &self,
        surface: SurfaceId,
        projected: impl FnOnce(SurfaceId) -> Result<bool, E>,
    ) -> Result<bool, E> {
        if !self.mapped_surfaces.contains(&surface) {
            return Ok(false);
        }
        projected(surface)
    }

    fn client_positioned_visible<E>(
        &self,
        surface: SurfaceId,
        managed_visible: impl FnOnce(SurfaceId) -> Result<bool, E>,
    ) -> Result<bool, E> {
        let mut current = surface;
        // Panels and nested popups bypass WM placement. Follow their mapped
        // ownership chain before consulting policy for a managed ancestor.
        // A cycle or a stale owner cannot establish visible ancestry.
        for _ in 0..=self.presentation_owners.len() {
            if !self.knows_surface(current) {
                return Ok(false);
            }
            if !self.is_client_positioned(current) {
                return self.managed_scene_visible(current, managed_visible);
            }
            if !self.client_positioned_mapped(current) {
                return Ok(false);
            }
            match self.presentation_owner(current) {
                Some(owner) => current = owner,
                None => return Ok(true),
            }
        }
        Ok(false)
    }
}

fn reconcile_live_layout_progress(
    layout: &mut PersistentLiveLayout,
    update_slot_available: bool,
) -> LiveLayoutProgress {
    if !layout.pending_is_ready() {
        return LiveLayoutProgress::Blocked;
    }
    if !update_slot_available {
        return LiveLayoutProgress::DeferredReady;
    }
    LiveLayoutProgress::Committed(
        layout
            .resolve_pending()
            .expect("ready pending layout resolves when its output slot is available"),
    )
}

fn wm_update_coordinator_batch(
    transaction: TransactionId,
) -> XAuthorityObservedTransactionBatch {
    XAuthorityObservedTransactionBatch {
        client: None,
        admission: None,
        surface_routes: Vec::new(),
        transaction,
        transactions: Vec::new(),
        surface_presentations: Vec::new(),
        presentation_intents: Vec::new(),
        removed_surfaces: Vec::new(),
        surface_output_reservations: Vec::new(),
        cpu_buffer_updates: Vec::new(),
        raster_responses: Vec::new(),
        dma_buf_registrations: Vec::new(),
        fence_registrations: Vec::new(),
        present_submissions: Vec::new(),
        software_present_submissions: Vec::new(),
        released_dma_bufs: Vec::new(),
        released_fences: Vec::new(),
        protocol_errors: Vec::new(),
        expected_protocol_errors: Vec::new(),
        metadata: Vec::new(),
        selection_owner_change: false,
        selection_conversion: false,
    }
}

fn center_geometry_without_scaling(mut geometry: Rect, output: Size) -> Rect {
    geometry.x = output.width.saturating_sub(geometry.width).max(0) / 2;
    geometry.y = output.height.saturating_sub(geometry.height).max(0) / 2;
    geometry
}

fn successful_primary_exit_ends_session(input_proof_requested: bool) -> bool {
    !input_proof_requested
}

fn global_runtime_deadline_ends_session(input_proof_requested: bool) -> bool {
    !input_proof_requested
}
