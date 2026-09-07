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
                replaces: Some(click)
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
                replaces: Some(explicit)
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
