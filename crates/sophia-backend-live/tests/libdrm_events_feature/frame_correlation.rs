mod frame_correlation {
    use super::*;
    use sophia_backend_live::LiveRendererFrameCorrelation;
    use sophia_renderer_live::LiveCompositionTrace;
    use std::{cell::Cell, rc::Rc};

    fn metadata(generation: u64) -> LiveRendererFrameCorrelation {
        LiveRendererFrameCorrelation {
            request: None,
            trace: Some(LiveCompositionTrace {
                output: OutputId::from_raw(7),
                head: RenderHeadId::from_raw(9),
                scene_generation: generation,
            }),
            direct_scanout: Some(sophia_engine::DirectScanoutVerdict::CompositionRequired(
                "refused",
            )),
        }
    }

    #[derive(Debug)]
    struct Owner(Rc<Cell<usize>>);

    impl Drop for Owner {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    impl LiveRenderedScanoutBufferPrimeSource for Owner {
        fn shares_kms_drm_file(&self) -> bool {
            true
        }
        fn export_scanout_dma_buf_fds(&self) -> io::Result<Option<LiveRenderedScanoutDmaBufFds>> {
            Ok(None)
        }
    }

    struct Exporter {
        pending: Option<LiveRenderedScanoutBufferExport<Owner>>,
    }

    impl LiveRenderedScanoutBufferExporter for Exporter {
        type Owner = Owner;
        fn export_rendered_scanout_buffer(
            &mut self,
            _: LiveGbmEglFrameTargetRecord,
        ) -> LiveRenderedScanoutBufferExport<Owner> {
            self.pending
                .take()
                .expect("one offered export per preparation")
        }
    }

    fn owned_export(
        size: Size,
        generation: u64,
        drops: &Rc<Cell<usize>>,
    ) -> LiveRenderedScanoutBufferExport<Owner> {
        LiveRenderedScanoutBufferExport::new(
            LiveRendererScanoutBufferExportStatus::Exported,
            LiveRendererScanoutBufferExportDetail::from_status(
                LiveRendererScanoutBufferExportStatus::Exported,
            ),
            Some(scanout_descriptor(size)),
            Some(Owner(drops.clone())),
        )
        .with_correlation(Some(metadata(generation)))
    }

    #[test]
    fn normalization_clears_frame_metadata_without_an_owned_successful_export() {
        let size = Size {
            width: 1280,
            height: 720,
        };
        for (status, has_descriptor, has_owner) in [
            (
                LiveRendererScanoutBufferExportStatus::Unavailable,
                true,
                true,
            ),
            (LiveRendererScanoutBufferExportStatus::Degraded, true, true),
            (LiveRendererScanoutBufferExportStatus::Pending, true, true),
            (
                LiveRendererScanoutBufferExportStatus::InvalidTarget,
                true,
                true,
            ),
            (LiveRendererScanoutBufferExportStatus::Exported, true, false),
            (LiveRendererScanoutBufferExportStatus::Exported, false, true),
            (
                LiveRendererScanoutBufferExportStatus::Exported,
                false,
                false,
            ),
        ] {
            for normalize in [false, true] {
                let drops = Rc::new(Cell::new(0));
                // Public fields can carry stale metadata; normalization must revalidate ownership.
                let export = LiveRenderedScanoutBufferExport {
                    status,
                    detail: LiveRendererScanoutBufferExportDetail::from_status(status),
                    descriptor: has_descriptor.then(|| scanout_descriptor(size)),
                    owner: has_owner.then(|| Owner(drops.clone())),
                    correlation: Some(metadata(41)),
                };
                let checked = if normalize {
                    export.normalized()
                } else {
                    export.with_correlation(Some(metadata(41)))
                };
                assert_eq!(
                    checked.correlation, None,
                    "{status:?}, descriptor={has_descriptor}, owner={has_owner}, normalized={normalize}"
                );
                if normalize {
                    assert!(checked.owner.is_none());
                    assert!(checked.descriptor.is_none());
                    assert_eq!(drops.get(), usize::from(has_owner));
                }
                drop(checked);
                assert_eq!(drops.get(), usize::from(has_owner));
            }
        }
        let drops = Rc::new(Cell::new(0));
        let valid = owned_export(size, 41, &drops).normalized();
        assert_eq!(valid.correlation, Some(metadata(41)));
        assert!(valid.owner.is_some());
        assert!(valid.descriptor.is_some());
        assert_eq!(drops.get(), 0);
        drop(valid);
        assert_eq!(drops.get(), 1);
    }

    #[test]
    fn prepared_frame_keeps_export_metadata_when_a_newer_frame_is_offered() {
        let device = full_primary_plane_scanout_device();
        let size = Size {
            width: 1280,
            height: 720,
        };
        let first_drops = Rc::new(Cell::new(0));
        let second_drops = Rc::new(Cell::new(0));
        let mut exporter = Exporter {
            pending: Some(owned_export(size, 41, &first_drops)),
        };
        let mut prepare = prepare_rendered_primary_plane_scanout_from_target_and_selection_with(
            LiveKmsScanoutTargetStatus::Ready,
            Some(LiveGbmEglFrameTargetRecord::new(size)),
            select_native_primary_plane_target(&device),
            None,
            &device,
            &mut exporter,
        );
        assert_eq!(
            prepare.status,
            LiveRenderedPrimaryPlaneScanoutPrepareStatus::Prepared
        );
        let first = prepare.prepared.take().expect("first framebuffer prepared");
        assert_eq!(first.correlation(), Some(metadata(41)));
        assert_eq!(first_drops.get(), 0);

        exporter.pending = Some(owned_export(size, 42, &second_drops));
        assert_eq!(first.correlation(), Some(metadata(41)));
        let second = prepare_rendered_primary_plane_scanout_from_target_and_selection_with(
            LiveKmsScanoutTargetStatus::Ready,
            Some(LiveGbmEglFrameTargetRecord::new(size)),
            select_native_primary_plane_target(&device),
            None,
            &device,
            &mut exporter,
        )
        .prepared
        .expect("newly offered framebuffer prepared separately");
        assert_eq!(second.correlation(), Some(metadata(42)));
        assert_eq!(first.correlation(), Some(metadata(41)));
        assert_eq!(device.commits.get(), 0);

        let cancelled = cancel_prepared_rendered_primary_plane_scanout(&device, first);
        assert_eq!(
            cancelled.destroy,
            LibdrmNativePrimaryPlaneResourceDestroyStatus::Destroyed
        );
        assert!(cancelled.cleanup.is_none());
        assert_eq!(device.resources.destroyed_framebuffers.get(), 1);
        assert_eq!(first_drops.get(), 1);
        assert_eq!(second_drops.get(), 0);
        assert_eq!(second.correlation(), Some(metadata(42)));
        let cancelled = cancel_prepared_rendered_primary_plane_scanout(&device, second);
        assert_eq!(
            cancelled.destroy,
            LibdrmNativePrimaryPlaneResourceDestroyStatus::Destroyed
        );
        assert!(cancelled.cleanup.is_none());
        assert_eq!(device.resources.destroyed_framebuffers.get(), 2);
        assert_eq!(first_drops.get(), 1);
        assert_eq!(second_drops.get(), 1);
        assert_eq!(device.resources.imported_buffers.get(), 0);
        assert_eq!(device.resources.closed_buffers.get(), 0);
        assert_eq!(device.commits.get(), 0);
    }
}
