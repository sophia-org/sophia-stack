#[cfg(unix)]
const X11_PIXMAP_PUBLICATION_TIMEOUT: Duration = Duration::from_secs(4);

#[cfg(unix)]
impl X11CoreSocketServerState {
    fn notify_pixmap_progress(&self) -> Result<(), X11SetupSocketError> {
        let _guard = self.pixmap_progress.0.lock().map_err(|_| {
            X11SetupSocketError::new("X11 pixmap progress lock poisoned")
        })?;
        self.pixmap_progress.1.notify_all();
        Ok(())
    }

    /// The progress lock closes the gap between checking state and sleeping.
    /// Neither this wait nor a provider call holds the authority runtime lock.
    fn wait_for_pixmap_progress(
        &self,
        deadline: Instant,
        pending: impl Fn(&XAuthorityRuntime) -> bool,
    ) -> Result<bool, X11SetupSocketError> {
        let mut guard = self.pixmap_progress.0.lock().map_err(|_| {
            X11SetupSocketError::new("X11 pixmap progress lock poisoned")
        })?;
        loop {
            let pending = {
                let runtime = self.runtime.lock().map_err(|_| {
                    X11SetupSocketError::new("X11 authority runtime lock poisoned")
                })?;
                pending(&runtime)
            };
            if !pending { return Ok(true); }
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() { return Ok(false); }
            guard = self.pixmap_progress.1.wait_timeout(guard, remaining)
                .map_err(|_| X11SetupSocketError::new("X11 pixmap progress lock poisoned"))?.0;
        }
    }

    fn prepare_exported_pixmap(
        &self,
        namespace: NamespaceId,
        drawable: XResourceId,
    ) -> Result<Option<crate::XPixmapExportToken>, X11SetupSocketError> {
        let Some(provider) = self.pixmap_allocator() else { return Ok(None); };
        let deadline = Instant::now() + X11_PIXMAP_PUBLICATION_TIMEOUT;
        let preparation = self.runtime.lock()
            .map_err(|_| X11SetupSocketError::new("X11 authority runtime lock poisoned"))?
            .prepare_pixmap_export(namespace, drawable);
        let Ok(preparation) = preparation else { return Ok(None); };
        let token = match preparation {
            crate::XPixmapExportPreparation::Ready => None,
            crate::XPixmapExportPreparation::Pending(token) => {
                if !self.wait_for_pixmap_progress(deadline, |runtime| {
                    runtime.pixmap_export_allocation_pending(token)
                })? {
                    tracing::warn!("sophia_pixmap_export schema=1 status=refused reason=allocation_deadline");
                }
                Some(token)
            }
            crate::XPixmapExportPreparation::Allocate { token, request } => {
                let allocation = provider.allocate_pixmap_buffer(request);
                if let Err(error) = &allocation {
                    tracing::warn!("sophia_pixmap_export schema=1 status=refused reason=allocation error={error}");
                }
                let adopted = self.runtime.lock()
                    .map_err(|_| X11SetupSocketError::new("X11 authority runtime lock poisoned"))?
                    .finish_pixmap_export_allocation(token, allocation.ok());
                self.notify_pixmap_progress()?;
                if let Err(error) = adopted {
                    tracing::warn!("sophia_pixmap_export schema=1 status=refused reason=adoption error={error:?}");
                }
                Some(token)
            }
        };
        self.release_exported_pixmaps()?;
        Ok(token)
    }

    fn publish_pixmap_prefix(
        &self,
        targets: &[crate::XPixmapPublicationTarget],
    ) -> Result<bool, X11SetupSocketError> {
        if targets.is_empty() { return Ok(true); }
        let Some(provider) = self.pixmap_allocator() else { return Ok(false); };
        let deadline = Instant::now() + X11_PIXMAP_PUBLICATION_TIMEOUT;
        for &target in targets {
            loop {
                let update = {
                    let mut runtime = self.runtime.lock().map_err(|_| {
                        X11SetupSocketError::new("X11 authority runtime lock poisoned")
                    })?;
                    if runtime.pixmap_publication_target_settled(target) { break; }
                    runtime.take_pixmap_publication_update(target)
                };
                match update {
                    Ok(Some(update)) => {
                        let (handle, revision) = (update.handle, update.revision);
                        let result = provider.update_pixmap_buffer(update);
                        self.runtime.lock()
                            .map_err(|_| X11SetupSocketError::new("X11 authority runtime lock poisoned"))?
                            .finish_pixmap_publication_update(handle, revision, result.is_ok());
                        self.notify_pixmap_progress()?;
                        if let Err(error) = result {
                            tracing::warn!("sophia_pixmap_export schema=1 status=refused reason=upload error={error}");
                            return Ok(false);
                        }
                    }
                    Ok(None) => {
                        // A concurrent publisher owns the current update. Its
                        // completion wakes this request without extending its prefix.
                        if !self.wait_for_pixmap_progress(deadline, |runtime| {
                            runtime.pixmap_publication_update_pending(target)
                        })? { return Ok(false); }
                    }
                    Err(error) => {
                        tracing::warn!("sophia_pixmap_export schema=1 status=refused reason=publication error={error:?}");
                        return Ok(false);
                    }
                }
                if Instant::now() >= deadline { return Ok(false); }
            }
        }
        Ok(true)
    }

    /// Failed cleanup remains debt, including after the originating client exits.
    fn release_exported_pixmaps(&self) -> Result<(), X11SetupSocketError> {
        let Some(provider) = self.pixmap_allocator() else { return Ok(()); };
        let mut releases = self.runtime.lock()
            .map_err(|_| X11SetupSocketError::new("X11 authority runtime lock poisoned"))?
            .take_pending_backing_releases().into_iter();
        let deadline = Instant::now() + X11_PIXMAP_PUBLICATION_TIMEOUT;
        while let Some(handle) = releases.next() {
            let result = provider.release_pixmap_buffer(handle);
            if result.is_ok() {
                self.runtime.lock()
                    .map_err(|_| X11SetupSocketError::new("X11 authority runtime lock poisoned"))?
                    .finish_pixmap_backing_release(handle);
            }
            if result.is_err() || Instant::now() >= deadline {
                let restore = result.is_err().then_some(handle).into_iter().chain(releases);
                self.runtime.lock()
                    .map_err(|_| X11SetupSocketError::new("X11 authority runtime lock poisoned"))?
                    .restore_pending_backing_releases(restore);
                if let Err(error) = result {
                    tracing::warn!("sophia_pixmap_export schema=1 status=deferred reason=release error={error}");
                }
                break;
            }
        }
        Ok(())
    }
}
