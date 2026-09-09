#[cfg(unix)]
impl XServerFrontendRouteRegistry {
    fn bind_runtime(
        &self,
        runtime: &Arc<Mutex<XAuthorityRuntime>>,
    ) -> Result<(), X11SetupSocketError> {
        let offered = Arc::downgrade(runtime);
        let bound = self.runtime.get_or_init(|| offered.clone());
        if !std::sync::Weak::ptr_eq(bound, &offered) {
            return Err(X11SetupSocketError::new(
                "X11 route registry is already bound to a different authority",
            ));
        }
        Ok(())
    }

    fn record_present_allocation_subject(
        &self,
        subject: crate::runtime::XPresentAllocationSubject,
    ) {
        let Ok(mut pending) = self.pending_presentations.entries.lock() else {
            return;
        };
        let Some(presentation) = pending.get_mut(&subject.transaction) else {
            return;
        };
        if presentation.client.raw() == subject.client_id
            && presentation.window == subject.window
            && presentation.pixmap == subject.pixmap
            && presentation.allocation_subject.is_none()
        {
            presentation.allocation_subject = Some(subject);
        }
    }
}
