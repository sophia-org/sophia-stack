fn grab_control(
    owner: &XAuthorityExplicitPointerGrabOwner,
) -> XAuthorityExplicitPointerGrabRequest {
    let deadline = std::time::Instant::now() + Duration::from_secs(2);
    loop {
        if let Ok(request) = owner.try_recv() {
            return request;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "missing explicit-grab request"
        );
        std::thread::yield_now();
    }
}

#[test]
fn explicit_grab_replaces_the_wire_clients_click_and_survives_later_buttons() {
    use sophia_protocol::{ApplicationRouteLeaseId, ApplicationRouteLeaseIdentity};
    for order in [XByteOrder::LittleEndian, XByteOrder::BigEndian] {
        let (grabs, engine) =
            x_authority_explicit_pointer_grab_bridge(NonZeroUsize::new(8).unwrap());
        let mut f = Fixture::with_grabs(true, Some(grabs));
        let mut owner = f.connect(order);
        let top = owner.window(X_SETUP_DEFAULT_ROOT, (100, 200, 320, 240));
        let surface = f.surface(&mut owner, top);
        let click = ApplicationRouteLeaseIdentity {
            id: ApplicationRouteLeaseId::from_raw(8),
            seat: SeatId::from_raw(1),
            frontend_sequence: 1,
            control_epoch: 1,
        };
        f.lease = Some(click);
        f.route(
            surface,
            InputEventKind::PointerButton {
                button: 0x110,
                pressed: true,
            },
        );
        let confirmation = f
            .lease_updates
            .recv_timeout(Duration::from_secs(2))
            .unwrap();
        assert_eq!(confirmation.identity, click);
        assert_eq!(confirmation.kind, XAuthorityRouteLeaseUpdateKind::Confirmed);

        owner.start_grab(top, false);
        let request = grab_control(&engine);
        assert_eq!(request.admission, confirmation.admission);
        assert_eq!(
            request.kind,
            XAuthorityExplicitPointerGrabRequestKind::Prepare {
                anchor: XAuthorityExplicitPointerGrabAnchor::Surface(surface),
                replaces: Some(click),
                after_observation: match request.kind {
                    XAuthorityExplicitPointerGrabRequestKind::Prepare {
                        after_observation, ..
                    } => {
                        assert!(after_observation.is_some());
                        after_observation
                    }
                    _ => unreachable!(),
                },
                control_epoch: 1,
            }
        );
        let explicit = ApplicationRouteLeaseIdentity {
            id: ApplicationRouteLeaseId::from_raw(9),
            frontend_sequence: 2,
            ..click
        };
        engine
            .respond(
                request.id,
                XAuthorityExplicitPointerGrabResponse::Prepared(explicit),
            )
            .unwrap();
        let activation = grab_control(&engine);
        assert_eq!(
            activation.kind,
            XAuthorityExplicitPointerGrabRequestKind::Activate { identity: explicit }
        );
        engine
            .respond(
                activation.id,
                XAuthorityExplicitPointerGrabResponse::Activated,
            )
            .unwrap();
        assert_eq!(&owner.reply()[..2], &[1, 0]);
        f.lease = Some(explicit);

        // Releasing the initiating button, then clicking again, must not
        // produce automatic lease releases or lose the explicit identity.
        for pressed in [false, true, false, true, false] {
            f.route(
                surface,
                InputEventKind::PointerButton {
                    button: 0x110,
                    pressed,
                },
            );
        }
        f.route_barrier(surface);
        assert!(f.lease_updates.try_recv().is_err());
        owner.start_grab(top, false);
        let regrab = grab_control(&engine);
        assert_eq!(
            regrab.kind,
            XAuthorityExplicitPointerGrabRequestKind::Prepare {
                anchor: XAuthorityExplicitPointerGrabAnchor::Surface(surface),
                replaces: Some(explicit),
                after_observation: match regrab.kind {
                    XAuthorityExplicitPointerGrabRequestKind::Prepare {
                        after_observation, ..
                    } => {
                        assert!(after_observation.is_some());
                        after_observation
                    }
                    _ => unreachable!(),
                },
                control_epoch: 1,
            }
        );
        engine
            .respond(
                regrab.id,
                XAuthorityExplicitPointerGrabResponse::Rejected(
                    XAuthorityExplicitPointerGrabRejection::AlreadyOwned,
                ),
            )
            .unwrap();
        assert_eq!(&owner.reply()[..2], &[1, 1]);

        let mut ungrab = vec![27, 0];
        push_u16(&mut ungrab, order, 2);
        push_u32(&mut ungrab, order, 0);
        owner.stream.write_all(&ungrab).unwrap();
        let release = grab_control(&engine);
        assert_eq!(
            release.kind,
            XAuthorityExplicitPointerGrabRequestKind::BeginRelease { identity: explicit }
        );
        engine
            .respond(
                release.id,
                XAuthorityExplicitPointerGrabResponse::ReleaseReady,
            )
            .unwrap();
        let finish = grab_control(&engine);
        assert_eq!(
            finish.kind,
            XAuthorityExplicitPointerGrabRequestKind::FinishRelease { identity: explicit }
        );
        engine
            .respond(finish.id, XAuthorityExplicitPointerGrabResponse::Released)
            .unwrap();
        owner.barrier();
    }
}

#[test]
fn explicit_grab_waits_without_authority_locks_and_names_only_published_observations() {
    let (grabs, engine) = x_authority_explicit_pointer_grab_bridge(NonZeroUsize::new(8).unwrap());
    let mut f = Fixture::with_grabs(true, Some(grabs));
    let mut client = f.connect(XByteOrder::LittleEndian);
    let top = client.window(X_SETUP_DEFAULT_ROOT, (100, 200, 320, 240));
    let surface = f.surface(&mut client, top);
    let _child = client.window(top, (10, 10, 40, 40));
    let last_published = f
        .transactions
        .try_iter()
        .last()
        .expect("child lifecycle publishes observations")
        .transaction;
    // GetInputFocus consumes transaction tickets without publishing a batch.
    for _ in 0..3 {
        client.barrier();
    }
    assert!(f.transactions.try_recv().is_err());
    client.start_grab(top, false);
    let prepare = grab_control(&engine);
    assert!(
        matches!(prepare.kind, XAuthorityExplicitPointerGrabRequestKind::Prepare {
        after_observation: Some(receipt), ..
    } if receipt == last_published)
    );
    f.route_barrier(surface);
    assert!(
        !engine.is_cancelled(prepare.id).unwrap(),
        "control writer must acquire runtime while Prepare waits"
    );
    let identity = sophia_protocol::ApplicationRouteLeaseIdentity {
        id: sophia_protocol::ApplicationRouteLeaseId::from_raw(81),
        seat: SeatId::from_raw(1),
        frontend_sequence: 1,
        control_epoch: 1,
    };
    assert_eq!(
        engine
            .respond(
                prepare.id,
                XAuthorityExplicitPointerGrabResponse::Prepared(identity)
            )
            .unwrap(),
        XAuthorityExplicitPointerGrabResponseDisposition::Delivered
    );
    let activation = grab_control(&engine);
    f.route_barrier(surface);
    assert!(
        !engine.is_cancelled(activation.id).unwrap(),
        "activation also releases runtime"
    );
    engine
        .respond(
            activation.id,
            XAuthorityExplicitPointerGrabResponse::Activated,
        )
        .unwrap();
    assert_eq!(&client.reply()[..2], &[1, 0]);
    client.window_request(27, 0);
    let release = grab_control(&engine);
    f.route_barrier(surface);
    assert!(!engine.is_cancelled(release.id).unwrap());
    engine
        .respond(
            release.id,
            XAuthorityExplicitPointerGrabResponse::ReleaseReady,
        )
        .unwrap();
    let finish = grab_control(&engine);
    f.route_barrier(surface);
    assert!(!engine.is_cancelled(finish.id).unwrap());
    engine
        .respond(finish.id, XAuthorityExplicitPointerGrabResponse::Released)
        .unwrap();
    client.barrier();
}

#[test]
fn explicit_grab_timeout_refuses_the_grab_and_keeps_the_client_alive() {
    let (grabs, engine) = x_authority_explicit_pointer_grab_bridge(NonZeroUsize::new(8).unwrap());
    let mut f = Fixture::with_grabs(true, Some(grabs));
    let mut client = f.connect(XByteOrder::LittleEndian);
    let top = client.window(X_SETUP_DEFAULT_ROOT, (100, 200, 320, 240));
    f.surface(&mut client, top);
    client.start_grab(top, false);
    let prepare = grab_control(&engine);
    assert_eq!(&client.reply()[..2], &[1, 1]);
    assert!(engine.is_cancelled(prepare.id).unwrap());
    assert_eq!(
        engine
            .respond(
                prepare.id,
                XAuthorityExplicitPointerGrabResponse::Rejected(
                    XAuthorityExplicitPointerGrabRejection::Stale
                )
            )
            .unwrap(),
        XAuthorityExplicitPointerGrabResponseDisposition::Cancelled
    );
    client.barrier();
}

#[test]
fn explicit_grab_activation_cannot_cross_a_security_epoch() {
    let (grabs, engine) = x_authority_explicit_pointer_grab_bridge(NonZeroUsize::new(8).unwrap());
    let mut f = Fixture::with_grabs(true, Some(grabs));
    let mut client = f.connect(XByteOrder::LittleEndian);
    let top = client.window(X_SETUP_DEFAULT_ROOT, (100, 200, 320, 240));
    let surface = f.surface(&mut client, top);
    client.start_grab(top, false);
    let prepare = grab_control(&engine);
    let identity = sophia_protocol::ApplicationRouteLeaseIdentity {
        id: sophia_protocol::ApplicationRouteLeaseId::from_raw(82),
        seat: SeatId::from_raw(1),
        frontend_sequence: 1,
        control_epoch: 1,
    };
    engine
        .respond(
            prepare.id,
            XAuthorityExplicitPointerGrabResponse::Prepared(identity),
        )
        .unwrap();
    let activation = grab_control(&engine);
    assert!(f.input.advance_control_epoch(2));
    f.route_barrier(surface);
    engine
        .respond(
            activation.id,
            XAuthorityExplicitPointerGrabResponse::Activated,
        )
        .unwrap();
    let abort = grab_control(&engine);
    assert_eq!(
        abort.kind,
        XAuthorityExplicitPointerGrabRequestKind::Abort { identity }
    );
    engine
        .respond(abort.id, XAuthorityExplicitPointerGrabResponse::Aborted)
        .unwrap();
    assert_eq!(&client.reply()[..2], &[1, 1]);
    client.barrier();
}

#[test]
fn failed_grab_release_resolves_its_ticket_before_cleanup_and_peer_requests() {
    for fail_after_dispatch in [false, true] {
        let (grabs, engine) =
            x_authority_explicit_pointer_grab_bridge(NonZeroUsize::new(8).unwrap());
        let mut f = Fixture::with_grabs(true, Some(grabs));
        let mut client = f.connect(XByteOrder::LittleEndian);
        let top = client.window(X_SETUP_DEFAULT_ROOT, (100, 200, 320, 240));
        let surface = f.surface(&mut client, top);
        let mut peer = f.connect(XByteOrder::LittleEndian);
        peer.barrier();
        client.start_grab(top, false);
        let prepare = grab_control(&engine);
        let identity = sophia_protocol::ApplicationRouteLeaseIdentity {
            id: sophia_protocol::ApplicationRouteLeaseId::from_raw(91),
            seat: SeatId::from_raw(1),
            frontend_sequence: 1,
            control_epoch: 1,
        };
        engine
            .respond(
                prepare.id,
                XAuthorityExplicitPointerGrabResponse::Prepared(identity),
            )
            .unwrap();
        let activation = grab_control(&engine);
        engine
            .respond(
                activation.id,
                XAuthorityExplicitPointerGrabResponse::Activated,
            )
            .unwrap();
        assert_eq!(&client.reply()[..2], &[1, 0]);
        client.window_request(27, 0);
        let mut release = grab_control(&engine);
        if fail_after_dispatch {
            engine
                .respond(
                    release.id,
                    XAuthorityExplicitPointerGrabResponse::ReleaseReady,
                )
                .unwrap();
            release = grab_control(&engine);
            assert_eq!(
                release.kind,
                XAuthorityExplicitPointerGrabRequestKind::FinishRelease { identity }
            );
        }
        engine
            .respond(
                release.id,
                XAuthorityExplicitPointerGrabResponse::Rejected(
                    XAuthorityExplicitPointerGrabRejection::Invalid,
                ),
            )
            .unwrap();
        // The failed request must account its ticket before the same worker's
        // cleanup allocates another. Otherwise cleanup and every later producer
        // wait forever behind the absent ticket.
        loop {
            let batch = f
                .transactions
                .recv_timeout(Duration::from_secs(2))
                .expect("failed client cleanup must publish without an egress hole");
            if batch.removed_surfaces.contains(&surface) {
                break;
            }
        }
        peer.barrier();
    }
}
