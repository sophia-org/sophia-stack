#![cfg(all(test, unix))]

use super::*;
use crate::{
    XAuthorityRequestKind, XAuthorityRequestPacket, XAuthorityResponseOutcome,
    XPresentLayoutComparison, XPresentLayoutComparisonResult, XRenderDeviceIdentity,
    XServerFrontendDeviceBundle, XServerFrontendDmaBufImportFormat, XWindowAllocationContext,
    XWindowAllocationPreference, XWindowAllocationPreferences, XWindowAllocationUpdate,
};
use sophia_protocol::{BufferHandle, DRM_FORMAT_ARGB8888, OutputId, SurfaceConstraints};
use std::fs::File;

const NS: NamespaceId = NamespaceId::from_raw(91);
const OWNER: XServerFrontendClientId = XServerFrontendClientId::from_raw(1);
const PRESENTER: XServerFrontendClientId = XServerFrontendClientId::from_raw(2);
const WINDOW: XResourceId = XResourceId::new(0x200001, 1);
const PIXMAP: XResourceId = XResourceId::new(0x400001, 1);
const SURFACE: SurfaceId = SurfaceId::new(73, 1);
const TRANSACTION: TransactionId = TransactionId::from_raw(70);
const TILED: u64 = 0x0200_0000_0040_1b03;
const GEOMETRY: Rect = Rect {
    x: 10,
    y: 20,
    width: 32,
    height: 32,
};

struct Provider(XRenderDeviceIdentity);
impl XServerFrontendRenderDeviceProvider for Provider {
    fn open_render_device_fd(&self) -> Result<OwnedFd, XServerFrontendRenderDeviceError> {
        Ok(File::open("/dev/null").unwrap().into())
    }
    fn render_device_identity(&self) -> Option<XRenderDeviceIdentity> {
        Some(self.0)
    }
    fn dma_buf_import_formats(&self) -> Vec<XServerFrontendDmaBufImportFormat> {
        vec![XServerFrontendDmaBufImportFormat {
            format: DRM_FORMAT_ARGB8888,
            modifiers: vec![0, TILED],
        }]
    }
}

fn identity() -> XRenderDeviceIdentity {
    let stat = rustix::fs::fstat(File::open("/dev/null").unwrap()).unwrap();
    XRenderDeviceIdentity {
        device: stat.st_dev,
        inode: stat.st_ino,
        device_number: stat.st_rdev,
    }
}

fn create_window(runtime: &mut XAuthorityRuntime, window: XResourceId, surface: SurfaceId) {
    for kind in [
        XAuthorityRequestKind::CreateWindow {
            window,
            surface,
            geometry: GEOMETRY,
            constraints: SurfaceConstraints {
                min_size: None,
                max_size: None,
            },
            generation: 1,
        },
        XAuthorityRequestKind::MapWindow {
            window,
            generation: 1,
        },
    ] {
        assert_eq!(
            runtime
                .apply(XAuthorityRequestPacket {
                    namespace: NS,
                    transaction: TransactionId::from_raw(1),
                    kind,
                })
                .outcome,
            XAuthorityResponseOutcome::Accepted
        );
    }
}

fn create_pixmap(runtime: &mut XAuthorityRuntime) -> sophia_protocol::DmaBufDescriptor {
    runtime
        .create_dri3_pixmap_from_buffers(
            NS,
            PIXMAP,
            1,
            1,
            32,
            32,
            [128, 0, 0, 0],
            [0; 4],
            32,
            32,
            TILED,
        )
        .unwrap()
}

struct Fixture {
    state: X11CoreSocketServerState,
    broker: XServerFrontendRouteBroker,
    _owner: XServerFrontendClientRouteRegistration,
    presenter: XServerFrontendClientRouteRegistration,
    channels: XServerFrontendClientRouteChannels,
    comparison: XPresentLayoutComparison,
}

impl Fixture {
    fn new() -> Self {
        let device = identity();
        let state = X11CoreSocketServerState::new();
        let bundle = Arc::new(
            XServerFrontendDeviceBundle::new(1, Arc::new(Provider(device)), None).unwrap(),
        );
        state.install_device_bundle(bundle.clone()).unwrap();
        let broker = XServerFrontendRouteBroker::new(NonZeroUsize::new(8).unwrap());
        broker.registry.bind_runtime(&state.runtime).unwrap();
        let (owner, _) = broker.registry.register_client(OWNER).unwrap();
        let (presenter, channels) = broker.registry.register_client(PRESENTER).unwrap();
        broker
            .registry
            .register_surface(OWNER, NS, SURFACE, WINDOW)
            .unwrap();
        broker
            .registry
            .select_present_input(PRESENTER, XResourceId::new(0x400003, 1), WINDOW, 6)
            .unwrap();
        let comparison = {
            let mut runtime = state.runtime.lock().unwrap();
            runtime.pin_client_device_bundle(PRESENTER.raw(), Some(bundle));
            create_window(&mut runtime, WINDOW, SURFACE);
            let descriptor = create_pixmap(&mut runtime);
            let comparison = XPresentLayoutComparison {
                surface: SURFACE,
                buffer: descriptor.handle,
                format: descriptor.format,
                original_modifier: descriptor.modifier,
                alternative_modifier: 0,
                preference_generation: 1,
                topology_generation: runtime.output_topology().generation,
                native_context: XWindowAllocationContext {
                    generation: 3,
                    output: OutputId::from_raw(1),
                },
                device_identity: device,
                geometry: GEOMETRY,
            };
            let snapshot = Self::snapshot(comparison);
            assert_eq!(
                runtime.update_window_allocation_preferences(snapshot),
                XWindowAllocationUpdate::Applied
            );
            comparison
        };
        let fixture = Self {
            state,
            broker,
            _owner: owner,
            presenter,
            channels,
            comparison,
        };
        fixture.admit();
        fixture
    }

    fn snapshot(comparison: XPresentLayoutComparison) -> XWindowAllocationPreferences {
        XWindowAllocationPreferences {
            generation: comparison.preference_generation,
            topology_generation: comparison.topology_generation,
            windows: vec![XWindowAllocationPreference {
                surface: comparison.surface,
                device: crate::XDrmDeviceHint {
                    major: rustix::fs::major(comparison.device_identity.device_number),
                    minor: rustix::fs::minor(comparison.device_identity.device_number),
                },
                identity: Some(comparison.device_identity),
                context: Some(comparison.native_context),
                formats: vec![XServerFrontendDmaBufImportFormat {
                    format: comparison.format,
                    modifiers: vec![0],
                }],
            }],
        }
    }

    fn admit(&self) {
        self.broker
            .registry
            .queue_present(TRANSACTION, PRESENTER, WINDOW, PIXMAP, 99, None)
            .unwrap();
        let mut runtime = self.state.runtime.lock().unwrap();
        let response =
            runtime.present_standard_pixmap(TRANSACTION, NS, WINDOW, PIXMAP, 0, 0, None, None);
        assert_eq!(response.outcome, XAuthorityResponseOutcome::Accepted);
        assert_eq!(response.transactions.len(), 1);
        let transaction = &response.transactions[0];
        let sophia_protocol::BufferSource::DmaBuf { handle } = transaction.target_buffer() else {
            panic!("expected DMA Present")
        };
        let present = XAuthorityPresentSubmission {
            transaction: response.transaction,
            surface: transaction.surface,
            buffer: BufferHandle::from_raw(handle),
            x_offset: 0,
            y_offset: 0,
            acquire_fence: None,
            idle_fence: None,
        };
        let subject = runtime
            .present_allocation_subject(NS, PRESENTER.raw(), WINDOW, PIXMAP, &present)
            .unwrap();
        self.broker
            .registry
            .record_present_allocation_subject(subject);
    }

    fn complete(
        &self,
        comparison: XPresentLayoutComparison,
        mode: XPresentCompletionMode,
    ) -> crate::XPresentCompleteRouteOutcome {
        self.broker
            .protocol_router()
            .route_present_complete_with_layout(TRANSACTION, 11, 22, mode, Some(comparison))
            .unwrap()
    }

    fn assert_completion(&self, expected: XPresentLayoutComparisonResult) {
        let outcome = self.complete(self.comparison, XPresentCompletionMode::Copy);
        assert_eq!(
            outcome,
            crate::XPresentCompleteRouteOutcome {
                routed: true,
                layout_comparison: Some(expected)
            }
        );
        assert!(
            matches!(self.channels.protocol.recv_timeout(Duration::from_secs(1)).unwrap(), XClientEvent::PresentCompleteNotify { mode, serial: 99, .. } if mode == XPresentCompletionMode::Copy as u8)
        );
        assert_eq!(
            self.complete(self.comparison, XPresentCompletionMode::Copy),
            crate::XPresentCompleteRouteOutcome {
                routed: false,
                layout_comparison: None
            }
        );
        assert_eq!(
            *self.broker.registry.present_clock.lock().unwrap(),
            Some((11, 22))
        );
        assert!(self.broker.route_present_idle(TRANSACTION).unwrap());
        assert!(matches!(
            self.channels
                .protocol
                .recv_timeout(Duration::from_secs(1))
                .unwrap(),
            XClientEvent::PresentIdleNotify { pixmap: PIXMAP, .. }
        ));
        assert!(
            self.broker
                .registry
                .pending_presentations
                .entries
                .lock()
                .unwrap()
                .is_empty()
        );
    }
}

#[test]
fn present_layout_comparison_matches_only_the_exact_current_retired_subject() {
    Fixture::new().assert_completion(XPresentLayoutComparisonResult::Matched);
    let changes: &[fn(&mut XPresentLayoutComparison)] = &[
        |c| c.surface = SurfaceId::new(73, 2),
        |c| c.buffer = BufferHandle::from_raw(c.buffer.raw() + 1),
        |c| c.format = sophia_protocol::DRM_FORMAT_XRGB8888,
        |c| c.original_modifier += 1,
        |c| c.alternative_modifier = TILED,
        |c| c.preference_generation += 1,
        |c| c.topology_generation += 1,
        |c| c.native_context.generation += 1,
        |c| c.native_context.output = OutputId::from_raw(2),
        |c| c.device_identity.inode += 1,
        |c| c.geometry.x += 1,
        |c| c.geometry.width += 1,
    ];
    for (index, change) in changes.iter().enumerate() {
        let mut fixture = Fixture::new();
        change(&mut fixture.comparison);
        let outcome = fixture.complete(fixture.comparison, XPresentCompletionMode::Copy);
        assert_eq!(
            outcome.layout_comparison,
            Some(XPresentLayoutComparisonResult::Rejected),
            "case {index}"
        );
        assert!(outcome.routed, "case {index} must still route Copy");
        assert!(matches!(
            fixture
                .channels
                .protocol
                .recv_timeout(Duration::from_secs(1))
                .unwrap(),
            XClientEvent::PresentCompleteNotify { mode: 0, .. }
        ));
    }
}

#[test]
fn present_layout_comparison_preserves_idle_first_and_never_changes_wire_modes() {
    let fixture = Fixture::new();
    assert!(fixture.broker.route_present_idle(TRANSACTION).unwrap());
    assert!(matches!(
        fixture.channels.protocol.recv().unwrap(),
        XClientEvent::PresentIdleNotify { .. }
    ));
    assert_eq!(
        fixture
            .complete(fixture.comparison, XPresentCompletionMode::Copy)
            .layout_comparison,
        Some(XPresentLayoutComparisonResult::Matched)
    );
    assert!(
        fixture
            .broker
            .registry
            .pending_presentations
            .entries
            .lock()
            .unwrap()
            .is_empty()
    );
    for mode in [XPresentCompletionMode::Flip, XPresentCompletionMode::Skip] {
        let fixture = Fixture::new();
        assert_eq!(
            fixture.complete(fixture.comparison, mode).layout_comparison,
            Some(XPresentLayoutComparisonResult::Rejected)
        );
        assert!(
            matches!(fixture.channels.protocol.recv().unwrap(), XClientEvent::PresentCompleteNotify { mode: actual, .. } if actual == mode as u8)
        );
    }
}

#[test]
fn present_layout_comparison_uses_captured_buffer_after_pixmap_xid_reuse() {
    let fixture = Fixture::new();
    {
        let mut runtime = fixture.state.runtime.lock().unwrap();
        runtime.free_pixmap(NS, PIXMAP).unwrap();
        let replacement = create_pixmap(&mut runtime);
        assert_ne!(replacement.handle, fixture.comparison.buffer);
    }
    fixture.assert_completion(XPresentLayoutComparisonResult::Matched);
}

#[test]
fn present_layout_comparison_rejects_window_reuse_loss_and_changed_hints() {
    for case in 0..5 {
        let fixture = Fixture::new();
        match case {
            0 => {
                fixture.state.mark_device_generation_unavailable(1).unwrap();
            }
            1 => {
                let mut runtime = fixture.state.runtime.lock().unwrap();
                runtime
                    .set_window_render_device_hint(
                        NS,
                        WINDOW,
                        crate::XDrmDeviceHint {
                            major: 999,
                            minor: 999,
                        },
                    )
                    .unwrap();
            }
            2 => {
                fixture
                    .state
                    .runtime
                    .lock()
                    .unwrap()
                    .unmap_window(NS, WINDOW)
                    .unwrap();
            }
            3 => {
                let mut runtime = fixture.state.runtime.lock().unwrap();
                runtime.destroy_window(NS, WINDOW).unwrap();
                create_window(&mut runtime, WINDOW, SurfaceId::new(73, 2));
            }
            4 => {
                fixture
                    .state
                    .runtime
                    .lock()
                    .unwrap()
                    .configure_window_geometry(
                        NS,
                        WINDOW,
                        crate::XWindowGeometryUpdate {
                            x: Some(11),
                            y: None,
                            width: None,
                            height: None,
                            generation: 1,
                        },
                    )
                    .unwrap();
            }
            _ => unreachable!(),
        }
        fixture.assert_completion(XPresentLayoutComparisonResult::Rejected);
    }
}

#[test]
fn present_layout_comparison_rejects_missing_format_or_still_preferred_original() {
    for case in 0..4 {
        let mut fixture = Fixture::new();
        fixture.comparison.preference_generation += 1;
        let mut snapshot = Fixture::snapshot(fixture.comparison);
        match case {
            0 => snapshot.windows[0].formats[0].format = sophia_protocol::DRM_FORMAT_XRGB8888,
            1 => snapshot.windows[0].formats[0].modifiers.push(TILED),
            2 => snapshot.windows[0].context = None,
            3 => {
                let replacement = Arc::new(
                    XServerFrontendDeviceBundle::new(
                        2,
                        Arc::new(Provider(XRenderDeviceIdentity {
                            inode: identity().inode + 1,
                            ..identity()
                        })),
                        None,
                    )
                    .unwrap(),
                );
                let mut runtime = fixture.state.runtime.lock().unwrap();
                runtime.release_client_device_bundle(PRESENTER.raw());
                runtime.pin_client_device_bundle(PRESENTER.raw(), Some(replacement));
            }
            _ => unreachable!(),
        }
        assert_eq!(
            fixture
                .state
                .runtime
                .lock()
                .unwrap()
                .update_window_allocation_preferences(snapshot),
            XWindowAllocationUpdate::Applied
        );
        fixture.assert_completion(XPresentLayoutComparisonResult::Rejected);
    }
}

#[test]
fn present_layout_comparison_runtime_binding_is_permanent_and_optional_failure_is_nonfatal() {
    let fixture = Fixture::new();
    fixture
        .broker
        .registry
        .bind_runtime(&fixture.state.runtime.clone())
        .unwrap();
    assert!(
        fixture
            .broker
            .registry
            .bind_runtime(&Arc::new(Mutex::new(XAuthorityRuntime::new())))
            .is_err()
    );
    // Poisoning optional comparison state must not consume or suppress ordinary feedback.
    let runtime = fixture.state.runtime.clone();
    assert!(
        std::thread::spawn(move || {
            let _guard = runtime.lock().unwrap();
            panic!("test poison");
        })
        .join()
        .is_err()
    );
    fixture.assert_completion(XPresentLayoutComparisonResult::Rejected);

    let fixture = Fixture::new();
    let Fixture {
        state,
        broker,
        _owner,
        presenter,
        channels,
        comparison,
    } = fixture;
    drop(state);
    assert!(
        broker
            .registry
            .bind_runtime(&Arc::new(Mutex::new(XAuthorityRuntime::new())))
            .is_err()
    );
    let outcome = broker
        .protocol_router()
        .route_present_complete_with_layout(
            TRANSACTION,
            1,
            2,
            XPresentCompletionMode::Copy,
            Some(comparison),
        )
        .unwrap();
    assert!(outcome.routed);
    assert_eq!(
        outcome.layout_comparison,
        Some(XPresentLayoutComparisonResult::Rejected)
    );
    assert!(matches!(
        channels.protocol.recv().unwrap(),
        XClientEvent::PresentCompleteNotify { mode: 0, .. }
    ));
    drop((_owner, presenter));
}

#[test]
fn present_layout_comparison_disconnect_cancels_the_exact_pending_subject() {
    let fixture = Fixture::new();
    let Fixture {
        state: _state,
        broker,
        _owner,
        presenter,
        channels: _,
        comparison,
    } = fixture;
    drop(presenter);
    assert!(
        broker
            .registry
            .pending_presentations
            .entries
            .lock()
            .unwrap()
            .is_empty()
    );
    let outcome = broker
        .protocol_router()
        .route_present_complete_with_layout(
            TRANSACTION,
            1,
            2,
            XPresentCompletionMode::Copy,
            Some(comparison),
        )
        .unwrap();
    assert_eq!(
        outcome,
        crate::XPresentCompleteRouteOutcome {
            routed: false,
            layout_comparison: None
        }
    );
}

#[test]
fn a_real_socket_dma_present_records_its_subject_after_dispatch() {
    let path =
        std::env::temp_dir().join(format!("sophia-present-layout-{}.sock", std::process::id()));
    let device = identity();
    let config = XServerFrontendConfig::new(&path, NS)
        .unwrap()
        .with_device_bundle(Arc::new(
            XServerFrontendDeviceBundle::new(1, Arc::new(Provider(device)), None).unwrap(),
        ));
    let mut frontend = XServerFrontend::bind(config).unwrap();
    let broker = XServerFrontendRouteBroker::new(NonZeroUsize::new(8).unwrap());
    let (sender, observed) = std::sync::mpsc::channel();
    let mut socket = UnixStream::connect(&path).unwrap();
    socket
        .set_read_timeout(Some(Duration::from_secs(3)))
        .unwrap();
    frontend
        .serve_next_concurrently_routed_traced(
            &broker,
            Arc::new(move |trace| {
                sender.send(trace).unwrap();
                Ok(None)
            }),
        )
        .unwrap();
    socket
        .write_all(&[b'l', 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let mut header = [0; 8];
    socket.read_exact(&mut header).unwrap();
    assert_eq!(header[0], 1);
    let mut setup = vec![0; usize::from(u16::from_le_bytes(header[6..8].try_into().unwrap())) * 4];
    socket.read_exact(&mut setup).unwrap();
    let base = u32::from_le_bytes(setup[4..8].try_into().unwrap());
    let window = base | 1;
    let pixmap = base | 2;
    let mut create = vec![0; 32];
    create[0] = 1;
    create[2..4].copy_from_slice(&8u16.to_le_bytes());
    create[4..8].copy_from_slice(&window.to_le_bytes());
    create[8..12].copy_from_slice(&X_SETUP_DEFAULT_ROOT.to_le_bytes());
    create[12..14].copy_from_slice(&10i16.to_le_bytes());
    create[14..16].copy_from_slice(&20i16.to_le_bytes());
    create[16..18].copy_from_slice(&32u16.to_le_bytes());
    create[18..20].copy_from_slice(&32u16.to_le_bytes());
    socket.write_all(&create).unwrap();
    let created = observed.recv_timeout(Duration::from_secs(3)).unwrap();
    assert_eq!(created.major_opcode, 1);
    assert!(created.failure.is_none());
    let surface = created.result.response.as_ref().unwrap().surfaces[0].surface;
    let mut map = vec![8, 0, 2, 0];
    map.extend(window.to_le_bytes());
    socket.write_all(&map).unwrap();
    let mapped = observed.recv_timeout(Duration::from_secs(3)).unwrap();
    assert_eq!(mapped.major_opcode, 8);
    assert!(mapped.failure.is_none());
    // The source is descriptor-backed; the Present itself must traverse real dispatch.
    let (descriptor, topology_generation) = {
        let mut runtime = frontend.state.runtime.lock().unwrap();
        let descriptor = runtime
            .create_dri3_pixmap_from_buffers(
                NS,
                XResourceId::new(u64::from(pixmap), 1),
                1,
                1,
                32,
                32,
                [128, 0, 0, 0],
                [0; 4],
                32,
                32,
                TILED,
            )
            .unwrap();
        runtime
            .attach_dri3_plane_fds(
                NS,
                XResourceId::new(u64::from(pixmap), 1),
                vec![Arc::new(File::open("/dev/null").unwrap().into())],
            )
            .unwrap();
        (descriptor, runtime.output_topology().generation)
    };
    let comparison = XPresentLayoutComparison {
        surface,
        buffer: descriptor.handle,
        format: descriptor.format,
        original_modifier: descriptor.modifier,
        alternative_modifier: 0,
        preference_generation: 1,
        topology_generation,
        native_context: XWindowAllocationContext {
            generation: 1,
            output: OutputId::from_raw(1),
        },
        device_identity: device,
        geometry: GEOMETRY,
    };
    assert_eq!(
        frontend
            .update_window_allocation_preferences(Fixture::snapshot(comparison))
            .unwrap(),
        XWindowAllocationUpdate::Applied
    );
    let mut select = vec![
        crate::X_PRESENT_MAJOR_OPCODE,
        crate::X_PRESENT_SELECT_INPUT_MINOR_OPCODE,
        4,
        0,
    ];
    select.extend((base | 3).to_le_bytes());
    select.extend(window.to_le_bytes());
    select.extend(6u32.to_le_bytes());
    socket.write_all(&select).unwrap();
    let selected = observed.recv_timeout(Duration::from_secs(3)).unwrap();
    assert!(selected.failure.is_none());
    let mut present = vec![0; 72];
    present[0] = crate::X_PRESENT_MAJOR_OPCODE;
    present[1] = crate::X_PRESENT_PIXMAP_MINOR_OPCODE;
    present[2..4].copy_from_slice(&18u16.to_le_bytes());
    present[4..8].copy_from_slice(&window.to_le_bytes());
    present[8..12].copy_from_slice(&pixmap.to_le_bytes());
    present[12..16].copy_from_slice(&99u32.to_le_bytes());
    socket.write_all(&present).unwrap();
    let presented = observed.recv_timeout(Duration::from_secs(3)).unwrap();
    assert!(presented.failure.is_none(), "{presented:?}");
    let submission = presented.present_submission.expect("accepted DMA Present");
    assert_eq!(submission.buffer, descriptor.handle);
    assert_eq!(submission.surface, surface);
    let outcome = broker
        .protocol_router()
        .route_present_complete_with_layout(
            submission.transaction,
            1,
            2,
            XPresentCompletionMode::Copy,
            Some(comparison),
        )
        .unwrap();
    assert_eq!(
        outcome,
        crate::XPresentCompleteRouteOutcome {
            routed: true,
            layout_comparison: Some(XPresentLayoutComparisonResult::Matched)
        }
    );
    let mut event = [0; 40];
    socket.read_exact(&mut event).unwrap();
    assert_eq!(event[0], 35);
    assert_eq!(event[1], crate::X_PRESENT_MAJOR_OPCODE);
    assert_eq!(event[11], XPresentCompletionMode::Copy as u8);
    assert_eq!(u16::from_le_bytes(event[8..10].try_into().unwrap()), 1);
    drop(socket);
    frontend.wait_for_clients().unwrap();
    drop(frontend);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn present_layout_comparison_rejects_unbound_subjects_and_policy_pending_windows() {
    let fixture = Fixture::new();
    fixture.broker.registry.cancel_present(TRANSACTION).unwrap();
    fixture
        .broker
        .registry
        .queue_present(TRANSACTION, PRESENTER, WINDOW, PIXMAP, 99, None)
        .unwrap();
    fixture.assert_completion(XPresentLayoutComparisonResult::Rejected);

    let fixture = Fixture::new();
    {
        let mut runtime = fixture.state.runtime.lock().unwrap();
        runtime.unmap_window(NS, WINDOW).unwrap();
        runtime.set_policy_map_deferred(true);
        let response = runtime.apply(XAuthorityRequestPacket {
            transaction: TransactionId::from_raw(100),
            namespace: NS,
            kind: XAuthorityRequestKind::MapWindow {
                window: WINDOW,
                generation: 1,
            },
        });
        assert_eq!(response.outcome, XAuthorityResponseOutcome::Accepted);
        assert!(runtime.window_policy_map_pending(NS, WINDOW).unwrap());
    }
    fixture.assert_completion(XPresentLayoutComparisonResult::Rejected);
}

#[test]
fn present_layout_comparison_rejects_invalid_context_without_replacing_preferences() {
    let fixture = Fixture::new();
    for context in [
        XWindowAllocationContext {
            generation: 0,
            output: OutputId::from_raw(1),
        },
        XWindowAllocationContext {
            generation: 1,
            output: OutputId::INVALID,
        },
    ] {
        let mut snapshot = Fixture::snapshot(fixture.comparison);
        snapshot.generation += 1;
        snapshot.windows[0].context = Some(context);
        assert_eq!(
            fixture
                .state
                .runtime
                .lock()
                .unwrap()
                .update_window_allocation_preferences(snapshot),
            XWindowAllocationUpdate::Invalid
        );
    }
    fixture.assert_completion(XPresentLayoutComparisonResult::Matched);
}

#[test]
fn present_layout_comparison_never_borrows_the_parents_preference_for_a_child() {
    let fixture = Fixture::new();
    let parent = XResourceId::new(0x200099, 1);
    let parent_surface = SurfaceId::new(74, 1);
    let subject = {
        let mut runtime = fixture.state.runtime.lock().unwrap();
        create_window(&mut runtime, parent, parent_surface);
        runtime.set_window_parent(NS, WINDOW, parent).unwrap();
        let present = XAuthorityPresentSubmission {
            transaction: TRANSACTION,
            surface: parent_surface,
            buffer: fixture.comparison.buffer,
            x_offset: GEOMETRY.x,
            y_offset: GEOMETRY.y,
            acquire_fence: None,
            idle_fence: None,
        };
        runtime
            .present_allocation_subject(NS, PRESENTER.raw(), WINDOW, PIXMAP, &present)
            .unwrap()
    };
    fixture.broker.registry.cancel_present(TRANSACTION).unwrap();
    fixture
        .broker
        .registry
        .queue_present(TRANSACTION, PRESENTER, WINDOW, PIXMAP, 99, None)
        .unwrap();
    fixture
        .broker
        .registry
        .record_present_allocation_subject(subject);
    let mut comparison = fixture.comparison;
    comparison.surface = parent_surface;
    comparison.preference_generation += 1;
    assert_eq!(
        fixture
            .state
            .runtime
            .lock()
            .unwrap()
            .update_window_allocation_preferences(Fixture::snapshot(comparison)),
        XWindowAllocationUpdate::Applied
    );
    let outcome = fixture.complete(comparison, XPresentCompletionMode::Copy);
    assert!(outcome.routed);
    assert_eq!(
        outcome.layout_comparison,
        Some(XPresentLayoutComparisonResult::Rejected)
    );
}

#[test]
fn a_busy_runtime_rejects_optional_comparison_without_delaying_complete() {
    let fixture = Fixture::new();
    let held = fixture.state.runtime.lock().unwrap();
    let router = fixture.broker.protocol_router();
    let comparison = fixture.comparison;
    let (sender, receiver) = std::sync::mpsc::sync_channel(1);
    let worker = std::thread::spawn(move || {
        sender
            .send(router.route_present_complete_with_layout(
                TRANSACTION,
                11,
                22,
                XPresentCompletionMode::Copy,
                Some(comparison),
            ))
            .unwrap();
    });
    let before_unlock = receiver.recv_timeout(Duration::from_secs(1));
    // Release even when the deadline fails, so a blocking implementation can exit.
    drop(held);
    worker.join().unwrap();
    let outcome = before_unlock
        .expect("optional comparison blocked ordinary Complete")
        .unwrap();
    assert_eq!(
        outcome,
        crate::XPresentCompleteRouteOutcome {
            routed: true,
            layout_comparison: Some(XPresentLayoutComparisonResult::Rejected),
        }
    );
    assert!(matches!(
        fixture
            .channels
            .protocol
            .recv_timeout(Duration::from_secs(1))
            .unwrap(),
        XClientEvent::PresentCompleteNotify { mode: 0, .. }
    ));
    assert_eq!(
        *fixture.broker.registry.present_clock.lock().unwrap(),
        Some((11, 22))
    );
    assert!(fixture.broker.route_present_idle(TRANSACTION).unwrap());
    assert!(matches!(
        fixture
            .channels
            .protocol
            .recv_timeout(Duration::from_secs(1))
            .unwrap(),
        XClientEvent::PresentIdleNotify { .. }
    ));
}
