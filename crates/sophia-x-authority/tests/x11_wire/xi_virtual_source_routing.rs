#[test]
fn xi_virtual_source_values_share_the_master_baseline_without_crossing_namespaces() {
    for order in [XByteOrder::LittleEndian, XByteOrder::BigEndian] {
        let mut fixture = Fixture::new(true);
        let mut first = fixture.connect(order);
        let top = first.window(X_SETUP_DEFAULT_ROOT, (100, 200, 320, 240));
        let surface = fixture.surface(&mut first, top);
        fixture.route(surface, InputEventKind::PointerMotion);
        fixture.route(
            surface,
            InputEventKind::PointerAxis {
                horizontal_v120: 120,
                vertical_v120: -240,
            },
        );
        assert_eq!(
            first.device_valuators(X_INPUT_POINTER_SOURCE_ID),
            [143, 259, 120, -240]
        );
        assert_eq!(
            first.device_valuators(X_INPUT_POINTER_SOURCE_ID),
            first.valuators()
        );
        // Querying again must expose the accumulated baseline, not consume it.
        fixture.route(
            surface,
            InputEventKind::PointerAxis {
                horizontal_v120: -60,
                vertical_v120: 120,
            },
        );
        assert_eq!(
            first.device_valuators(X_INPUT_POINTER_SOURCE_ID),
            [143, 259, 60, -120]
        );
        let mut other = fixture.connect(order);
        assert_eq!(other.device_valuators(X_INPUT_POINTER_SOURCE_ID), [0; 4]);
        assert_eq!(other.valuators(), [0; 4]);
    }
}

#[test]
fn xi_virtual_source_grab_is_denied_before_any_engine_reservation() {
    for order in [XByteOrder::LittleEndian, XByteOrder::BigEndian] {
        let (grabs, engine) =
            x_authority_explicit_pointer_grab_bridge(NonZeroUsize::new(8).unwrap());
        let mut fixture = Fixture::with_grabs(true, Some(grabs));
        let mut client = fixture.connect(order);
        let mut request = vec![X_INPUT_MAJOR_OPCODE, X_INPUT_GRAB_DEVICE_MINOR_OPCODE];
        push_u16(&mut request, order, 6);
        push_u32(&mut request, order, X_SETUP_DEFAULT_ROOT);
        push_u32(&mut request, order, 0); // CurrentTime
        push_u32(&mut request, order, 0); // no cursor
        push_u16(&mut request, order, X_INPUT_POINTER_SOURCE_ID);
        request.extend_from_slice(&[1, 1, 0, 0]);
        push_u16(&mut request, order, 0); // empty mask
        client.stream.write_all(&request).unwrap();
        assert_eq!(
            &client.reply()[..2],
            &[0, XErrorCode::BadAccess.wire_code()]
        );
        client.barrier();
        assert!(
            engine.try_recv().is_err(),
            "source grab must not reserve the master route"
        );
    }
}

#[test]
fn xi_virtual_source_ungrab_cannot_release_the_master_grab() {
    let order = XByteOrder::LittleEndian;
    let mut fixture = Fixture::new(false);
    let mut owner = fixture.connect(order);
    let top = owner.window(X_SETUP_DEFAULT_ROOT, (100, 200, 320, 240));
    owner.grab(top, false);
    let mut request = vec![X_INPUT_MAJOR_OPCODE, X_INPUT_UNGRAB_DEVICE_MINOR_OPCODE];
    push_u16(&mut request, order, 3);
    push_u32(&mut request, order, 0);
    push_u16(&mut request, order, X_INPUT_POINTER_SOURCE_ID);
    push_u16(&mut request, order, 0);
    owner.stream.write_all(&request).unwrap();
    owner.barrier();
    let mut other = fixture.connect(order);
    other.start_grab(top, false);
    assert_eq!(
        &other.reply()[..2],
        &[1, 1],
        "master remains AlreadyGrabbed"
    );
}
