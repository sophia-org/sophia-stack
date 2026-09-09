use super::*;
use std::cell::Cell;

struct Device {
    base: FakeNativePrimaryPlaneScanoutDevice,
    fail_prime: bool,
    hold_cleanup: Cell<bool>,
    import_attempts: Cell<usize>,
    close_attempts: Cell<usize>,
}

impl LibdrmNativeKmsSelectionDevice for Device {
    fn connector_handles(&self) -> io::Result<Vec<drm::control::connector::Handle>> {
        self.base.selection.connector_handles()
    }

    fn crtc_handles(&self) -> io::Result<Vec<drm::control::crtc::Handle>> {
        self.base.selection.crtc_handles()
    }

    fn connector_snapshot(
        &self,
        connector: drm::control::connector::Handle,
    ) -> io::Result<LibdrmNativeConnectorSnapshot> {
        self.base.selection.connector_snapshot(connector)
    }

    fn encoder_snapshot(
        &self,
        encoder: drm::control::encoder::Handle,
    ) -> io::Result<LibdrmNativeEncoderSnapshot> {
        self.base.selection.encoder_snapshot(encoder)
    }

    fn plane_handles(&self) -> io::Result<Vec<drm::control::plane::Handle>> {
        self.base.selection.plane_handles()
    }

    fn plane_snapshot(
        &self,
        plane: drm::control::plane::Handle,
    ) -> io::Result<LibdrmNativePlaneSnapshot> {
        self.base.selection.plane_snapshot(plane)
    }

    fn plane_type(
        &self,
        plane: drm::control::plane::Handle,
    ) -> io::Result<Option<drm::control::PlaneType>> {
        self.base.selection.plane_type(plane)
    }
}

impl LibdrmNativePropertyLookupDevice for Device {
    fn connector_property_handles(
        &self,
        connector: drm::control::connector::Handle,
    ) -> io::Result<LibdrmNativePropertyHandleSet> {
        self.base.properties.connector_property_handles(connector)
    }

    fn crtc_property_handles(
        &self,
        crtc: drm::control::crtc::Handle,
    ) -> io::Result<LibdrmNativePropertyHandleSet> {
        self.base.properties.crtc_property_handles(crtc)
    }

    fn plane_property_handles(
        &self,
        plane: drm::control::plane::Handle,
    ) -> io::Result<LibdrmNativePropertyHandleSet> {
        self.base.properties.plane_property_handles(plane)
    }
}

impl LibdrmNativePrimaryPlaneResourceDevice for Device {
    fn create_mode_blob_for_selection(
        &self,
        selection: sophia_backend_live::LibdrmNativePrimaryPlaneSelection,
    ) -> io::Result<u64> {
        self.base
            .resources
            .create_mode_blob_for_selection(selection)
    }

    fn create_mode_blob(&self, mode: drm::control::Mode) -> io::Result<u64> {
        self.base.resources.create_mode_blob(mode)
    }

    fn add_scanout_framebuffer_with_modifiers<B>(
        &self,
        buffer: &B,
    ) -> io::Result<drm::control::framebuffer::Handle>
    where
        B: drm::buffer::PlanarBuffer + ?Sized,
    {
        self.base
            .resources
            .add_scanout_framebuffer_with_modifiers(buffer)
    }

    fn add_scanout_framebuffer_without_modifiers<B>(
        &self,
        buffer: &B,
    ) -> io::Result<drm::control::framebuffer::Handle>
    where
        B: drm::buffer::PlanarBuffer + ?Sized,
    {
        self.base
            .resources
            .add_scanout_framebuffer_without_modifiers(buffer)
    }

    fn add_legacy_scanout_framebuffer<B>(
        &self,
        buffer: &B,
        depth: u32,
        bpp: u32,
    ) -> io::Result<drm::control::framebuffer::Handle>
    where
        B: drm::buffer::Buffer + ?Sized,
    {
        self.base
            .resources
            .add_legacy_scanout_framebuffer(buffer, depth, bpp)
    }

    fn destroy_scanout_framebuffer(
        &self,
        framebuffer: drm::control::framebuffer::Handle,
    ) -> io::Result<()> {
        self.base.resources.destroy_scanout_framebuffer(framebuffer)
    }

    // Forwarded rather than left to the trait default, which refuses. A
    // client's buffer reaches this device only through PRIME, so a scanout
    // device that cannot import cannot reach the direct path at all.
    fn import_scanout_dma_buf(&self, fd: BorrowedFd<'_>) -> io::Result<drm::buffer::Handle> {
        if self.fail_prime {
            self.import_attempts.set(self.import_attempts.get() + 1);
            Err(io::Error::from_raw_os_error(22))
        } else {
            self.base.resources.import_scanout_dma_buf(fd)
        }
    }

    fn close_scanout_buffer(&self, handle: drm::buffer::Handle) -> io::Result<()> {
        self.close_attempts.set(self.close_attempts.get() + 1);
        if self.hold_cleanup.get() {
            Err(io::Error::from_raw_os_error(16))
        } else {
            self.base.resources.close_scanout_buffer(handle)
        }
    }

    fn destroy_mode_blob(&self, mode_blob: u64) -> io::Result<()> {
        self.base.resources.destroy_mode_blob(mode_blob)
    }
}

impl LibdrmNativeAtomicCommitDevice for Device {
    fn submit_atomic_commit(
        &self,
        flags: drm::control::AtomicCommitFlags,
        request: drm::control::atomic::AtomicModeReq,
    ) -> io::Result<()> {
        self.base.submit_atomic_commit(flags, request)
    }
}

#[test]
fn direct_resource_refusal_retains_composition_without_atomic_test_evidence() {
    for fail_prime in [true, false] {
        let root = ready_drm_sysfs_fixture(if fail_prime {
            "direct-prime-refusal-composes"
        } else {
            "direct-addfb-refusal-composes"
        });
        let mut assembly = discover_live_backend(&LiveBackendConfig::new(&root))
            .into_live_runtime_assembly(QueuedInputPoller::default())
            .expect("ready fixture");
        let mut base = full_primary_plane_scanout_device();
        base.resources.framebuffer = Err(io::Error::from_raw_os_error(22));
        let device = Device {
            base,
            fail_prime,
            hold_cleanup: Cell::new(true),
            import_attempts: Cell::new(0),
            close_attempts: Cell::new(0),
        };
        let mut exporter = direct_scanout_runtime_exporter();
        let mut result =
            assembly.submit_rendered_primary_plane_scanout_with(&device, &mut exporter);

        assert_eq!(
            result.status,
            LiveRenderedPrimaryPlaneScanoutSubmitStatus::ScanoutExportPending
        );
        assert!(result.submission.is_none());
        assert!(result.atomic_test.is_none());
        assert_eq!(device.base.commits(), 0);
        assert_eq!(exporter.direct_scanout_tests(), 0);
        assert_eq!(exporter.direct_scanout_test_rejections(), 0);
        assert_eq!(exporter.direct_scanout_fallbacks(), 1);
        assert!(exporter.pending_mixed_frame());
        assert!(!exporter.direct_scanout_outstanding());
        assert!(LiveRenderedScanoutBufferExporter::direct_scanout_test_required(&exporter));

        if fail_prime {
            assert_eq!(device.import_attempts.get(), 1);
            assert_eq!(device.base.imported_buffers(), 0);
            assert!(result.cleanup.is_none());
            assert_eq!(device.close_attempts.get(), 0);
        } else {
            assert_eq!(device.base.imported_buffers(), 1);
            assert_eq!(device.close_attempts.get(), 1);
            assert_eq!(device.base.closed_buffers(), 0);
            let cleanup = result
                .cleanup
                .take()
                .expect("failed GEM close remains owned");
            let cleanup = cleanup.map_scanout_buffer(|owner| {
                assert!(owner.is_direct_client_buffer());
                owner
            });
            device.hold_cleanup.set(false);
            let retried =
                sophia_backend_live::retry_rendered_primary_plane_scanout_cleanup(&device, cleanup);
            assert!(retried.cleanup.is_none());
            assert_eq!(device.close_attempts.get(), 2);
            assert_eq!(device.base.closed_buffers(), 1);
        }
        assert_eq!(device.base.destroyed_framebuffers(), 0);
        assert!(
            exporter.pending_mixed_frame(),
            "resource cleanup cannot consume fallback pixels"
        );
        std::fs::remove_dir_all(root).unwrap();
    }
}
