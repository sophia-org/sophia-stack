#[test]
fn a_cached_direct_episode_supplies_no_test_evidence_for_its_next_buffer() {
    let root = ready_drm_sysfs_fixture("direct-test-evidence-cache");
    let mut assembly = discover_live_backend(&LiveBackendConfig::new(&root))
        .into_live_runtime_assembly(QueuedInputPoller::default())
        .unwrap();
    let device = full_primary_plane_scanout_device();
    let mut exporter = direct_scanout_runtime_exporter();

    let first =
        assembly.submit_and_track_rendered_primary_plane_scanout_with(&device, &mut exporter);
    let test = first
        .atomic_test
        .expect("the episode starts with a real test");
    assert_eq!(test.status, LibdrmNativeAtomicCommitSubmitStatus::Submitted);
    assert_eq!(test.raw_os_error, None);
    let request = test
        .request
        .expect("owned lowering records its exact request");
    assert_eq!(request.flags, test.commit_flags);
    assert!(test.commit_flags.test_only);
    assert!(!test.commit_flags.page_flip_event);
    assert!(!first.commit_flags.unwrap().test_only);
    assert_eq!(device.test_only_commits(), 1);

    let pending =
        assembly.submit_and_track_rendered_primary_plane_scanout_with(&device, &mut exporter);
    assert_eq!(
        pending.status,
        LiveTrackedRenderedPrimaryPlaneScanoutSubmitStatus::AlreadyInFlight
    );
    assert!(pending.atomic_test.is_none());
    let retired = assembly.retire_tracked_rendered_primary_plane_scanout_after_page_flip(
        &device,
        &LivePageFlipCallbackReport {
            decision: LivePageFlipCallbackDecision::Accepted,
            event: LivePageFlipEvent {
                status: LivePageFlipEventStatus::Presented,
                frame_serial: Some(1),
            },
        },
    );
    assert_eq!(
        retired.status,
        LiveTrackedRenderedPrimaryPlaneScanoutRetireStatus::RetiredAfterPageFlip
    );

    let mut next = direct_scanout_runtime_frame(Size {
        width: 1280,
        height: 720,
    });
    next.trace.as_mut().unwrap().scene_generation = 92;
    let sophia_renderer_live::LiveOwnedMixedCompositionLayer::DmaBuf { image_id, .. } =
        &mut next.layers[0]
    else {
        panic!("the fixture must supply a DMA-BUF");
    };
    *image_id = sophia_renderer_live::LiveRendererImageId::from_raw(12);
    exporter.set_pending_mixed_frame(next);
    let second =
        assembly.submit_and_track_rendered_primary_plane_scanout_with(&device, &mut exporter);
    assert_eq!(
        second.status,
        LiveTrackedRenderedPrimaryPlaneScanoutSubmitStatus::SubmittedWaitingForPageFlip
    );
    assert!(
        second.atomic_test.is_none(),
        "episode caching is not candidate evidence"
    );
    assert_eq!(device.test_only_commits(), 1);
    assert_eq!(device.commits(), 3, "no extra steady-state test ioctl");
    let retired = assembly.retire_tracked_rendered_primary_plane_scanout_after_page_flip(
        &device,
        &LivePageFlipCallbackReport {
            decision: LivePageFlipCallbackDecision::Accepted,
            event: LivePageFlipEvent {
                status: LivePageFlipEventStatus::Presented,
                frame_serial: Some(2),
            },
        },
    );
    assert_eq!(
        retired.status,
        LiveTrackedRenderedPrimaryPlaneScanoutRetireStatus::RetiredAfterPageFlip
    );
    assert_eq!(device.destroyed_framebuffers(), 2);
    assert_eq!(device.closed_buffers(), 2);
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn a_direct_refusal_keeps_its_actual_test_error_through_tracking_and_cleanup() {
    for errno in [5, 11, 16, 22] {
        let root = ready_drm_sysfs_fixture(&format!("direct-test-evidence-error-{errno}"));
        let mut assembly = discover_live_backend(&LiveBackendConfig::new(&root))
            .into_live_runtime_assembly(QueuedInputPoller::default())
            .unwrap();
        let mut device = full_primary_plane_scanout_device();
        device.submit = Err(io::Error::from_raw_os_error(errno));
        let mut exporter = direct_scanout_runtime_exporter();
        let result =
            assembly.submit_and_track_rendered_primary_plane_scanout_with(&device, &mut exporter);
        assert_eq!(
            result.status,
            LiveTrackedRenderedPrimaryPlaneScanoutSubmitStatus::ScanoutExportPending
        );
        let test = result.atomic_test.expect("refusal retains the actual test");
        assert_eq!(test.raw_os_error, Some(errno));
        assert_eq!(
            test.error_kind,
            Some(io::Error::from_raw_os_error(errno).kind())
        );
        assert_eq!(
            test.status,
            if errno == 11 {
                LibdrmNativeAtomicCommitSubmitStatus::WouldBlock
            } else {
                LibdrmNativeAtomicCommitSubmitStatus::Rejected
            }
        );
        assert!(test.request.is_some());
        assert_eq!(result.commit_submit, Some(test.status));
        assert_eq!(device.commits(), 1);
        assert_eq!(device.test_only_commits(), 1);
        assert_eq!(device.destroyed_framebuffers(), 1);
        assert_eq!(device.closed_buffers(), 1);
        assert!(exporter.pending_mixed_frame());
        std::fs::remove_dir_all(root).unwrap();
    }
}

#[test]
fn a_failed_real_commit_does_not_rewrite_its_passing_test() {
    let root = ready_drm_sysfs_fixture("direct-test-evidence-later-refusal");
    let mut assembly = discover_live_backend(&LiveBackendConfig::new(&root))
        .into_live_runtime_assembly(QueuedInputPoller::default())
        .unwrap();
    let device = full_primary_plane_scanout_device().accepting_commits(1);
    let mut exporter = direct_scanout_runtime_exporter();
    let result =
        assembly.submit_and_track_rendered_primary_plane_scanout_with(&device, &mut exporter);
    assert_eq!(
        result.status,
        LiveTrackedRenderedPrimaryPlaneScanoutSubmitStatus::ScanoutExportPending
    );
    assert_eq!(
        result.commit_submit,
        Some(LibdrmNativeAtomicCommitSubmitStatus::Rejected)
    );
    let test = result
        .atomic_test
        .expect("the passing test is retained as history");
    assert_eq!(test.status, LibdrmNativeAtomicCommitSubmitStatus::Submitted);
    assert_eq!(test.raw_os_error, None);
    assert_eq!(test.error_kind, None);
    assert!(test.request.is_some());
    assert!(test.commit_flags.test_only);
    assert!(!result.commit_flags.unwrap().test_only);
    assert_eq!(device.commits(), 2);
    assert_eq!(device.test_only_commits(), 1);
    assert_eq!(device.destroyed_framebuffers(), 1);
    assert_eq!(device.closed_buffers(), 1);
    assert!(exporter.pending_mixed_frame());
    std::fs::remove_dir_all(root).unwrap();
}
