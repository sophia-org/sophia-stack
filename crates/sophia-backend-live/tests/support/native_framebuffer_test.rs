#![cfg(test)]

use super::*;

use crate::drm::framebuffer_test_device::*;

fn selection() -> LibdrmNativePrimaryPlaneSelectionResult {
    LibdrmNativePrimaryPlaneSelectionResult {
        status: LibdrmNativePrimaryPlaneSelectionStatus::Selected,
        selection: Some(LibdrmNativePrimaryPlaneSelection::new(
            drm::control::from_u32(11).unwrap(),
            drm::control::from_u32(12).unwrap(),
            drm::control::from_u32(13).unwrap(),
            SIZE,
            None,
        )),
    }
}

fn cursor(x: i32) -> LibdrmNativeAtomicCursor {
    LibdrmNativeAtomicCursor {
        plane: drm::control::from_u32(14).unwrap(),
        properties: LibdrmNativeCursorPlanePropertyHandles::new(
            property(201),
            property(202),
            property(203),
            property(204),
            property(205),
            property(206),
            property(207),
            property(208),
            property(209),
            property(210),
        ),
        placement: Some(LibdrmNativeCursorPlacement {
            framebuffer: drm::control::from_u32(99).unwrap(),
            x,
            y: 3,
            width: 64,
            height: 64,
        }),
    }
}

fn policy() -> LibdrmNativePrimaryPlaneScanoutSubmitPolicy {
    LibdrmNativePrimaryPlaneScanoutSubmitPolicy::page_flip()
        .with_vrr_enabled(false)
        .with_cursor(cursor(2))
}

impl Device {
    fn prepare(
        &self,
        descriptor: LiveRendererScanoutBufferDescriptor,
        policy: LibdrmNativePrimaryPlaneScanoutSubmitPolicy,
    ) -> LibdrmNativePrimaryPlaneScanoutPrepareResult {
        let fds = std::array::from_fn(|index| {
            (index < descriptor.plane_count as usize)
                .then(|| std::fs::File::open("/dev/null").unwrap().into())
        });
        prepare_native_primary_plane_scanout_from_selection_and_renderer_dma_bufs_with_policy(
            self,
            selection(),
            descriptor,
            fds,
            policy,
        )
    }

    fn rejected(
        &self,
        policy: LibdrmNativePrimaryPlaneScanoutSubmitPolicy,
    ) -> LibdrmNativeFramebufferRejection {
        let result = self.prepare(descriptor(DCC), policy);
        assert_eq!(
            result.resources,
            Some(LibdrmNativePrimaryPlaneResourceCreateStatus::FramebufferCreateFailed)
        );
        assert!(result.prepared.is_none());
        assert!(result.cleanup.is_none());
        result
            .framebuffer_rejection
            .expect("the actual explicit AddFB2 failure is retained")
    }

    fn alternative(
        &self,
        policy: LibdrmNativePrimaryPlaneScanoutSubmitPolicy,
    ) -> LibdrmNativePrimaryPlanePreparedScanout {
        let result = self.prepare(descriptor(0), policy);
        assert!(result.framebuffer_rejection.is_none());
        result.prepared.expect("linear alternative is prepared")
    }

    fn cancel(&self, alternative: LibdrmNativePrimaryPlanePreparedScanout) {
        assert!(
            self.destroyed.borrow().is_empty(),
            "testing must not release the alternative"
        );
        let result = cancel_prepared_native_primary_plane_scanout(self, alternative);
        assert!(result.cleanup.is_none());
        assert_eq!(*self.destroyed.borrow(), [100]);
    }
}

#[test]
fn framebuffer_rejection_tests_only_the_exact_owned_alternative() {
    let device = Device::new();
    let rejection = device.rejected(policy());
    assert_eq!(rejection.descriptor(), descriptor(DCC));
    assert_eq!(rejection.error_kind(), io::ErrorKind::InvalidInput);
    assert_eq!(rejection.raw_os_error(), Some(22));
    assert_eq!(device.imports.get(), 2);
    assert_eq!(*device.closed.borrow(), [200]);
    let alternative = device.alternative(policy());
    let original_commit = alternative.request.evidence();
    let expected_test = alternative.test_request_evidence();
    let (report, alternative) =
        validate_alternative_after_framebuffer_rejection(&device, rejection, alternative);
    assert_eq!(report.status, LibdrmNativeAtomicTestPairStatus::Tested);
    assert_eq!(report.intended_request, expected_test);
    assert_eq!(report.alternative.unwrap().request, expected_test);
    assert_eq!(
        report.alternative.unwrap().status,
        LibdrmNativeAtomicCommitSubmitStatus::Submitted
    );
    assert_eq!(alternative.request.evidence(), original_commit);
    assert_eq!(device.tests.borrow().len(), 1);
    let tests = device.tests.borrow();
    assert!(
        tests[0]
            .0
            .contains(drm::control::AtomicCommitFlags::TEST_ONLY)
    );
    assert!(
        !tests[0]
            .0
            .contains(drm::control::AtomicCommitFlags::PAGE_FLIP_EVENT)
    );
    let evidence = expected_test.unwrap();
    let framebuffer = evidence.primary_framebuffer();
    assert_eq!(
        evidence
            .properties()
            .iter()
            .find(|row| (row.object, row.property) == framebuffer)
            .unwrap()
            .value,
        100
    );
    let mut expected = drm::control::atomic::AtomicModeReq::new();
    for row in evidence.properties() {
        expected.add_raw_property(
            drm::control::from_u32(row.object).unwrap(),
            drm::control::from_u32(row.property).unwrap(),
            row.value,
        );
    }
    assert_eq!(tests[0].1, format!("{expected:?}"));
    drop(tests);
    assert_eq!(*device.closed.borrow(), [200]);
    device.cancel(alternative);
    assert_eq!(*device.closed.borrow(), [200, 201]);
}

#[test]
fn framebuffer_errors_other_than_exact_einval_do_not_test() {
    for errno in [
        None,
        Some(1),
        Some(4),
        Some(5),
        Some(11),
        Some(12),
        Some(13),
        Some(16),
        Some(19),
        Some(28),
    ] {
        let device = Device::new();
        device.errno.set(errno);
        if errno.is_none() {
            device.error_kind.set(Some(io::ErrorKind::InvalidInput));
        }
        let rejection = device.rejected(policy());
        assert_eq!(rejection.raw_os_error(), errno);
        let alternative = device.alternative(policy());
        let (report, alternative) =
            validate_alternative_after_framebuffer_rejection(&device, rejection, alternative);
        assert_eq!(
            report.status,
            LibdrmNativeAtomicTestPairStatus::FramebufferRejectionIneligible,
            "errno={errno:?}"
        );
        assert_eq!(report.raw_os_error, errno);
        assert!(report.alternative.is_none());
        assert!(device.tests.borrow().is_empty());
        device.cancel(alternative);
    }
}

#[test]
fn framebuffer_rejection_requires_completed_prime_and_preserves_cleanup() {
    let device = Device::new();
    device.import_failure.set(Some(2));
    let result = device.prepare(descriptor(DCC), policy());
    assert_eq!(
        result.resources,
        Some(LibdrmNativePrimaryPlaneResourceCreateStatus::BufferImportFailed)
    );
    assert!(result.framebuffer_rejection.is_none());
    assert_eq!(device.addfb_calls.get(), 0);
    assert_eq!(*device.closed.borrow(), [200]);

    let device = Device::new();
    let result =
        prepare_native_primary_plane_scanout_from_selection_and_renderer_descriptor_with_policy(
            &device,
            selection(),
            descriptor(DCC),
            policy(),
        );
    assert!(matches!(
        result.framebuffer,
        Some(LibdrmNativePrimaryPlaneFramebufferCreateDetail::AddFb2ModifiersFailed { .. })
    ));
    assert!(
        result.framebuffer_rejection.is_none(),
        "GEM-only preparation does not prove PRIME admission"
    );

    let device = Device::new();
    let mut invalid = descriptor(DCC);
    invalid.plane_pitches[1] = 0;
    let result = device.prepare(invalid, policy());
    assert!(result.framebuffer_rejection.is_none());
    assert_eq!(device.imports.get(), 0);

    let device = Device::new();
    device.close_failure.set(true);
    let result = device.prepare(descriptor(DCC), policy());
    assert!(result.framebuffer_rejection.is_some());
    assert!(result.prepared.is_none());
    let cleanup = result
        .cleanup
        .expect("the failed source still owns its cleanup obligation");
    device.close_failure.set(false);
    let result = destroy_native_primary_plane_resource_cleanup(&device, cleanup);
    assert!(result.cleanup.is_none());
    assert_eq!(*device.closed.borrow(), [200]);
}

#[test]
fn framebuffer_comparison_rejects_request_and_layout_changes_without_testing() {
    for case in 0..11 {
        let device = Device::new();
        let mut rejection = device.rejected(policy());
        let mut alternate_policy = policy();
        match case {
            0 => alternate_policy.vrr_enabled = Some(true),
            1 => alternate_policy.cursor = Some(cursor(3)),
            2 => alternate_policy.cursor = None,
            3 => alternate_policy.nonblocking = false,
            4 => alternate_policy = alternate_policy.validating(),
            _ => {}
        }
        let mut alternative = device.alternative(alternate_policy);
        let expected = match case {
            0..=3 => LibdrmNativeAtomicTestPairStatus::RequestMismatch,
            4 => LibdrmNativeAtomicTestPairStatus::FramebufferRejectionIneligible,
            5 => {
                alternative.selected.connector = drm::control::from_u32(77).unwrap();
                LibdrmNativeAtomicTestPairStatus::SelectionMismatch
            }
            6 => {
                alternative.descriptor.size.width -= 1;
                LibdrmNativeAtomicTestPairStatus::GeometryMismatch
            }
            7 => {
                alternative.descriptor.format = LIVE_RENDERER_SCANOUT_FORMAT_ARGB8888;
                LibdrmNativeAtomicTestPairStatus::LayoutMismatch
            }
            8 => {
                alternative.descriptor.modifier = Some(DCC);
                LibdrmNativeAtomicTestPairStatus::LayoutMismatch
            }
            9 => {
                alternative.descriptor.modifier = Some(sophia_protocol::DRM_FORMAT_MOD_INVALID);
                LibdrmNativeAtomicTestPairStatus::LayoutMismatch
            }
            10 => {
                rejection.policy.allow_modeset = true;
                LibdrmNativeAtomicTestPairStatus::FramebufferRejectionIneligible
            }
            _ => unreachable!(),
        };
        let (report, alternative) =
            validate_alternative_after_framebuffer_rejection(&device, rejection, alternative);
        assert_eq!(report.status, expected, "case={case}");
        assert!(report.alternative.is_none());
        assert!(device.tests.borrow().is_empty());
        device.cancel(alternative);
    }
}

#[test]
fn alternative_test_refusal_is_preserved_without_releasing_its_owner() {
    for errno in [11, 16, 22] {
        let device = Device::new();
        let rejection = device.rejected(policy());
        let alternative = device.alternative(policy());
        device.test_errno.set(Some(errno));
        let (report, alternative) =
            validate_alternative_after_framebuffer_rejection(&device, rejection, alternative);
        assert_eq!(report.status, LibdrmNativeAtomicTestPairStatus::Tested);
        let observed = report.alternative.unwrap();
        assert_eq!(observed.raw_os_error, Some(errno));
        assert_ne!(
            observed.status,
            LibdrmNativeAtomicCommitSubmitStatus::Submitted
        );
        assert_eq!(device.tests.borrow().len(), 1);
        device.cancel(alternative);
    }
}
