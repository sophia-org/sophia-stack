mod atomic_test_pair {
    use super::*;
    use sophia_backend_live::{
        LibdrmNativeAtomicTestPairStatus as PairStatus, LibdrmNativeAtomicTestReport,
        LibdrmNativePrimaryPlanePreparedScanout, LibdrmNativePrimaryPlaneSelectionResult,
        prepare_native_primary_plane_scanout_from_selection_and_renderer_dma_bufs_with_policy,
        validate_prepared_native_primary_plane_scanout_pair,
    };
    use std::{
        cell::{Cell, RefCell},
        collections::VecDeque,
    };

    struct Device {
        base: FakeNativePrimaryPlaneScanoutDevice,
        next_framebuffer: Cell<u32>,
        next_buffer: Cell<u32>,
        next_mode_blob: Cell<u64>,
        destroyed_mode_blobs: RefCell<Vec<u64>>,
        captured: RefCell<
            Vec<(
                drm::control::AtomicCommitFlags,
                drm::control::atomic::AtomicModeReq,
            )>,
        >,
        outcomes: RefCell<VecDeque<Option<i32>>>,
        destroyed: RefCell<Vec<u32>>,
        closed: RefCell<Vec<u32>>,
    }

    impl Device {
        fn new(outcomes: &[Option<i32>]) -> Self {
            let mut base = full_primary_plane_scanout_device();
            base.properties.crtc = Ok(LibdrmNativePropertyHandleSet::new([
                ("MODE_ID", property_handle(102)),
                ("ACTIVE", property_handle(103)),
                ("VRR_ENABLED", property_handle(115)),
            ]));
            Self {
                base,
                next_framebuffer: Cell::new(100),
                next_buffer: Cell::new(200),
                next_mode_blob: Cell::new(300),
                destroyed_mode_blobs: RefCell::new(Vec::new()),
                captured: RefCell::new(Vec::new()),
                outcomes: RefCell::new(outcomes.iter().copied().collect()),
                destroyed: RefCell::new(Vec::new()),
                closed: RefCell::new(Vec::new()),
            }
        }
        fn framebuffer(&self) -> io::Result<drm::control::framebuffer::Handle> {
            let value = self.next_framebuffer.get();
            self.next_framebuffer.set(value + 1);
            Ok(drm::control::from_u32(value).unwrap())
        }
        fn selection(&self) -> LibdrmNativePrimaryPlaneSelectionResult {
            select_native_primary_plane_target(&self.base)
        }
        fn prepare(
            &self,
            selection: LibdrmNativePrimaryPlaneSelectionResult,
            size: Size,
            policy: LibdrmNativePrimaryPlaneScanoutSubmitPolicy,
        ) -> LibdrmNativePrimaryPlanePreparedScanout {
            let result = prepare_native_primary_plane_scanout_from_selection_and_renderer_dma_bufs_with_policy(
                self,
                selection,
                scanout_descriptor(size),
                [
                    Some(std::fs::File::open("/dev/null").unwrap().into()),
                    None,
                    None,
                    None,
                ],
                policy,
            );
            result.prepared.unwrap_or_else(|| {
                panic!(
                    "fake preparation failed: {:?}, size={size:?}, policy={policy:?}",
                    result.status
                )
            })
        }
    }

    impl LibdrmNativePropertyLookupDevice for Device {
        fn connector_property_handles(
            &self,
            connector: drm::control::connector::Handle,
        ) -> io::Result<LibdrmNativePropertyHandleSet> {
            self.base.connector_property_handles(connector)
        }
        fn crtc_property_handles(
            &self,
            crtc: drm::control::crtc::Handle,
        ) -> io::Result<LibdrmNativePropertyHandleSet> {
            self.base.crtc_property_handles(crtc)
        }
        fn plane_property_handles(
            &self,
            plane: drm::control::plane::Handle,
        ) -> io::Result<LibdrmNativePropertyHandleSet> {
            self.base.plane_property_handles(plane)
        }
    }

    impl LibdrmNativePrimaryPlaneResourceDevice for Device {
        fn create_mode_blob_for_selection(
            &self,
            selection: sophia_backend_live::LibdrmNativePrimaryPlaneSelection,
        ) -> io::Result<u64> {
            let _ = selection;
            let value = self.next_mode_blob.get();
            self.next_mode_blob.set(value + 1);
            Ok(value)
        }
        fn create_mode_blob(&self, mode: drm::control::Mode) -> io::Result<u64> {
            let _ = mode;
            let value = self.next_mode_blob.get();
            self.next_mode_blob.set(value + 1);
            Ok(value)
        }
        fn add_scanout_framebuffer_with_modifiers<B: drm::buffer::PlanarBuffer + ?Sized>(
            &self,
            _: &B,
        ) -> io::Result<drm::control::framebuffer::Handle> {
            self.framebuffer()
        }
        fn add_scanout_framebuffer_without_modifiers<B: drm::buffer::PlanarBuffer + ?Sized>(
            &self,
            _: &B,
        ) -> io::Result<drm::control::framebuffer::Handle> {
            self.framebuffer()
        }
        fn add_legacy_scanout_framebuffer<B: drm::buffer::Buffer + ?Sized>(
            &self,
            _: &B,
            _: u32,
            _: u32,
        ) -> io::Result<drm::control::framebuffer::Handle> {
            self.framebuffer()
        }
        fn destroy_scanout_framebuffer(
            &self,
            framebuffer: drm::control::framebuffer::Handle,
        ) -> io::Result<()> {
            self.destroyed.borrow_mut().push(framebuffer.into());
            Ok(())
        }
        fn import_scanout_dma_buf(&self, _: BorrowedFd<'_>) -> io::Result<drm::buffer::Handle> {
            let value = self.next_buffer.get();
            self.next_buffer.set(value + 1);
            Ok(buffer_handle(value))
        }
        fn close_scanout_buffer(&self, handle: drm::buffer::Handle) -> io::Result<()> {
            self.closed.borrow_mut().push(handle.into());
            Ok(())
        }
        fn destroy_mode_blob(&self, blob: u64) -> io::Result<()> {
            self.destroyed_mode_blobs.borrow_mut().push(blob);
            Ok(())
        }
    }

    impl LibdrmNativeAtomicCommitDevice for Device {
        fn submit_atomic_commit(
            &self,
            flags: drm::control::AtomicCommitFlags,
            request: drm::control::atomic::AtomicModeReq,
        ) -> io::Result<()> {
            self.captured.borrow_mut().push((flags, request));
            self.outcomes
                .borrow_mut()
                .pop_front()
                .expect("no extra ioctl is permitted")
                .map_or(Ok(()), |errno| Err(io::Error::from_raw_os_error(errno)))
        }
    }

    fn assert_observation(
        device: &Device,
        index: usize,
        expected_framebuffer: u64,
        expected_errno: Option<i32>,
        report: LibdrmNativeAtomicTestReport,
    ) {
        let captured = device.captured.borrow();
        let (flags, raw) = &captured[index];
        assert!(flags.contains(drm::control::AtomicCommitFlags::TEST_ONLY));
        assert!(!flags.contains(drm::control::AtomicCommitFlags::PAGE_FLIP_EVENT));
        assert_eq!(report.raw_os_error, expected_errno);
        let expected_kind = expected_errno.map(|errno| io::Error::from_raw_os_error(errno).kind());
        assert_eq!(report.error_kind, expected_kind);
        assert_eq!(
            report.status,
            match expected_kind {
                None => LibdrmNativeAtomicCommitSubmitStatus::Submitted,
                Some(io::ErrorKind::WouldBlock) => LibdrmNativeAtomicCommitSubmitStatus::WouldBlock,
                Some(_) => LibdrmNativeAtomicCommitSubmitStatus::Rejected,
            }
        );
        let evidence = report
            .request
            .expect("fresh test records canonical request");
        assert!(
            evidence.properties().iter().all(|row| row.property != 102),
            "page flips must not carry MODE_ID"
        );
        assert_eq!(device.next_mode_blob.get(), 300);
        assert!(device.destroyed_mode_blobs.borrow().is_empty());
        let framebuffer = evidence.primary_framebuffer();
        assert_eq!(framebuffer, (13, 104));
        assert_eq!(
            evidence
                .properties()
                .iter()
                .find(|row| (row.object, row.property) == framebuffer)
                .unwrap()
                .value,
            expected_framebuffer
        );
        let mut expected = drm::control::atomic::AtomicModeReq::new();
        for row in evidence.properties() {
            expected.add_raw_property(
                drm::control::from_u32(row.object).unwrap(),
                drm::control::from_u32(row.property).unwrap(),
                row.value,
            );
        }
        // AtomicModeReq exposes no equality or iterator. Compare complete captured requests,
        // using Debug only in this test, never as production evidence or a parser.
        assert_eq!(format!("{raw:?}"), format!("{expected:?}"));
    }

    fn cancel_pair(
        device: &Device,
        original: LibdrmNativePrimaryPlanePreparedScanout,
        alternative: LibdrmNativePrimaryPlanePreparedScanout,
        expected_framebuffers: [u32; 2],
    ) {
        assert!(device.destroyed.borrow().is_empty());
        assert!(device.closed.borrow().is_empty());
        let cancelled = cancel_prepared_native_primary_plane_scanout(device, original);
        assert_eq!(
            cancelled.status,
            LibdrmNativePrimaryPlaneResourceDestroyStatus::Destroyed
        );
        assert!(cancelled.cleanup.is_none());
        assert_eq!(*device.destroyed.borrow(), [expected_framebuffers[0]]);
        assert_eq!(*device.closed.borrow(), [200]);
        let cancelled = cancel_prepared_native_primary_plane_scanout(device, alternative);
        assert_eq!(
            cancelled.status,
            LibdrmNativePrimaryPlaneResourceDestroyStatus::Destroyed
        );
        assert!(cancelled.cleanup.is_none());
        assert_eq!(*device.destroyed.borrow(), expected_framebuffers);
        assert_eq!(*device.closed.borrow(), [200, 201]);
        assert_eq!(device.next_mode_blob.get(), 300);
        assert!(device.destroyed_mode_blobs.borrow().is_empty());
    }

    #[test]
    fn pair_issues_two_fresh_ordered_tests_for_every_driver_outcome_and_returns_owners() {
        let size = Size {
            width: 1280,
            height: 720,
        };
        for outcomes in [
            [None, None],
            [Some(22), None],
            [Some(11), None],
            [Some(16), None],
            [Some(5), None],
            [Some(19), None],
            [None, Some(22)],
            [Some(22), Some(11)],
        ] {
            let device = Device::new(&outcomes);
            let original = device.prepare(
                device.selection(),
                size,
                LibdrmNativePrimaryPlaneScanoutSubmitPolicy::page_flip(),
            );
            let alternative = device.prepare(
                device.selection(),
                size,
                LibdrmNativePrimaryPlaneScanoutSubmitPolicy::page_flip(),
            );
            let (report, original, alternative) =
                validate_prepared_native_primary_plane_scanout_pair(&device, original, alternative);
            assert_eq!(report.status, PairStatus::Tested);
            assert_eq!(device.captured.borrow().len(), 2);
            assert!(device.outcomes.borrow().is_empty());
            assert_observation(&device, 0, 100, outcomes[0], report.original.unwrap());
            assert_observation(&device, 1, 101, outcomes[1], report.alternative.unwrap());
            cancel_pair(&device, original, alternative, [100, 101]);
            assert_eq!(device.captured.borrow().len(), 2);
        }
    }

    #[test]
    fn mismatched_pair_does_not_test_or_release_either_owner() {
        let size = Size {
            width: 1280,
            height: 720,
        };
        for case in 0..4 {
            let device = Device::new(&[]);
            let original = device.prepare(
                device.selection(),
                size,
                LibdrmNativePrimaryPlaneScanoutSubmitPolicy::page_flip().with_vrr_enabled(false),
            );
            let mut selection = device.selection();
            let mut policy =
                LibdrmNativePrimaryPlaneScanoutSubmitPolicy::page_flip().with_vrr_enabled(false);
            let expected = match case {
                0 => {
                    device.next_framebuffer.set(100);
                    PairStatus::RequestMismatch
                }
                1 => {
                    let selected = selection.selection.unwrap();
                    selection.selection =
                        Some(sophia_backend_live::LibdrmNativePrimaryPlaneSelection::new(
                            drm::control::from_u32(46).unwrap(),
                            selected.crtc_handle(),
                            selected.plane_handle(),
                            selected.size(),
                            None,
                        ));
                    PairStatus::SelectionMismatch
                }
                2 => {
                    policy.nonblocking = false;
                    PairStatus::RequestMismatch
                }
                3 => {
                    policy.vrr_enabled = Some(true);
                    PairStatus::RequestMismatch
                }
                _ => unreachable!(),
            };
            let alternative = device.prepare(selection, size, policy);
            let (report, original, alternative) =
                validate_prepared_native_primary_plane_scanout_pair(&device, original, alternative);
            assert_eq!(report.status, expected, "case {case}");
            assert!(report.original.is_none());
            assert!(report.alternative.is_none());
            assert!(device.captured.borrow().is_empty());
            cancel_pair(
                &device,
                original,
                alternative,
                [100, if case == 0 { 100 } else { 101 }],
            );
            assert!(device.captured.borrow().is_empty());
        }
    }
    #[test]
    fn paired_tests_leave_both_real_commit_requests_unchanged() {
        let size = Size {
            width: 1280,
            height: 720,
        };
        let device = Device::new(&[None, None, None, None]);
        let original = device.prepare(
            device.selection(),
            size,
            LibdrmNativePrimaryPlaneScanoutSubmitPolicy::page_flip(),
        );
        let alternative = device.prepare(
            device.selection(),
            size,
            LibdrmNativePrimaryPlaneScanoutSubmitPolicy::page_flip(),
        );
        let (report, original, alternative) =
            validate_prepared_native_primary_plane_scanout_pair(&device, original, alternative);
        assert_eq!(report.status, PairStatus::Tested);
        for (index, prepared) in [original, alternative].into_iter().enumerate() {
            let submitted = submit_prepared_native_primary_plane_scanout(&device, prepared);
            assert_eq!(
                submitted.status,
                LibdrmNativePrimaryPlaneScanoutSubmitStatus::SubmittedWaitingForPageFlip
            );
            {
                let captures = device.captured.borrow();
                let (flags, raw) = &captures[2 + index];
                assert!(!flags.contains(drm::control::AtomicCommitFlags::TEST_ONLY));
                assert!(flags.contains(drm::control::AtomicCommitFlags::PAGE_FLIP_EVENT));
                assert_eq!(format!("{raw:?}"), format!("{:?}", captures[index].1));
            }
            let retired = submitted
                .submission
                .expect("real commit retains affine resources")
                .retire(&device);
            assert_eq!(
                retired.status,
                LibdrmNativePrimaryPlaneResourceDestroyStatus::Destroyed
            );
            assert!(retired.cleanup.is_none());
        }
        assert_eq!(*device.destroyed.borrow(), [100, 101]);
        assert_eq!(*device.closed.borrow(), [200, 201]);
        assert_eq!(device.captured.borrow().len(), 4);
        assert!(device.outcomes.borrow().is_empty());
    }
    #[test]
    fn mismatched_geometry_is_refused_before_a_prepared_pair_can_exist() {
        let device = Device::new(&[]);
        let result =
            prepare_native_primary_plane_scanout_from_selection_and_renderer_dma_bufs_with_policy(
                &device,
                device.selection(),
                scanout_descriptor(Size {
                    width: 640,
                    height: 720,
                }),
                [
                    Some(std::fs::File::open("/dev/null").unwrap().into()),
                    None,
                    None,
                    None,
                ],
                LibdrmNativePrimaryPlaneScanoutSubmitPolicy::page_flip(),
            );
        assert_eq!(result.status, sophia_backend_live::LibdrmNativePrimaryPlaneScanoutPrepareStatus::ResourceCreationUnavailable);
        assert!(result.prepared.is_none());
        assert!(result.cleanup.is_none());
        assert_eq!(device.next_framebuffer.get(), 100);
        assert_eq!(device.next_buffer.get(), 200);
        assert!(device.captured.borrow().is_empty());
    }
}
