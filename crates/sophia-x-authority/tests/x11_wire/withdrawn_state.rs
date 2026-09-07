#[test]
fn withdrawn_state_hints_do_not_override_mapped_pending_or_foreign_authority() {
    for order in [XByteOrder::LittleEndian, XByteOrder::BigEndian] {
        let mut fixture = RenderFixture::new();
        let xid = RenderFixture::PIXMAP;
        let window = XResourceId::new(u64::from(xid), 1);
        let create = create_window_request(order, xid, 0, 0, 4, 4);
        assert_eq!(
            RenderFixture::error_of(&clip_send(&mut fixture, order, &create)),
            None
        );
        apply_engine_presentation_state(
            &mut fixture.properties,
            &mut fixture.atoms,
            RenderFixture::NS,
            window,
            order,
            PolicyPresentationState::default(),
        )
        .unwrap();
        let state = fixture.atoms.atom(X_ATOM_NAME_NET_WM_STATE).unwrap();
        let wm_state = fixture.atoms.atom(X_ATOM_NAME_WM_STATE).unwrap();
        let fullscreen = fixture
            .atoms
            .intern(X_ATOM_NAME_NET_WM_STATE_FULLSCREEN, false)
            .unwrap()
            .unwrap();
        let mut hint = Vec::new();
        push_u32(&mut hint, order, fullscreen);
        let change = change_property_request(
            order,
            XPropertyMode::Replace,
            xid,
            state,
            X_ATOM_ATOM,
            32,
            &hint,
        );
        assert_eq!(
            RenderFixture::error_of(&clip_send(&mut fixture, order, &change)),
            None
        );
        assert_eq!(
            fixture
                .properties
                .get(RenderFixture::NS, window, state)
                .unwrap()
                .bytes,
            hint
        );
        let protected = change_property_request(
            order,
            XPropertyMode::Replace,
            xid,
            wm_state,
            X_ATOM_ATOM,
            32,
            &[],
        );
        assert_eq!(
            RenderFixture::error_of(&clip_send(&mut fixture, order, &protected)),
            Some(XErrorCode::BadAccess)
        );

        let foreign = NamespaceId::from_raw(89);
        let result = dispatch_x11_wire_request(
            dispatch_context(foreign, 20, order, 18),
            decode_x11_core_request(context(foreign, 20, order), &change).unwrap(),
            &mut fixture.runtime,
            &mut fixture.atoms,
            &mut fixture.properties,
        );
        assert!(RenderFixture::error_of(&result).is_some());
        assert!(fixture.properties.get(foreign, window, state).is_none());

        // A pending policy admission still reports Unmapped on the X wire.
        // It must not reopen the initial-hint exception after MapWindow.
        fixture.runtime.set_policy_map_deferred(true);
        let map = resource_request(order, 8, xid);
        assert_eq!(
            RenderFixture::error_of(&clip_send(&mut fixture, order, &map)),
            None
        );
        assert_eq!(
            fixture
                .runtime
                .window_policy_map_pending(RenderFixture::NS, window),
            Ok(true)
        );
        let clear = change_property_request(
            order,
            XPropertyMode::Replace,
            xid,
            state,
            X_ATOM_ATOM,
            32,
            &[],
        );
        assert_eq!(
            RenderFixture::error_of(&clip_send(&mut fixture, order, &clear)),
            Some(XErrorCode::BadAccess)
        );
        let unmap = resource_request(order, 10, xid);
        assert_eq!(
            RenderFixture::error_of(&clip_send(&mut fixture, order, &unmap)),
            None
        );
        assert_eq!(
            RenderFixture::error_of(&clip_send(&mut fixture, order, &clear)),
            None
        );

        fixture.runtime.set_policy_map_deferred(false);
        assert_eq!(
            RenderFixture::error_of(&clip_send(&mut fixture, order, &map)),
            None
        );
        assert_eq!(
            RenderFixture::error_of(&clip_send(&mut fixture, order, &change)),
            Some(XErrorCode::BadAccess)
        );
        assert!(
            fixture
                .properties
                .get(RenderFixture::NS, window, state)
                .unwrap()
                .bytes
                .is_empty()
        );
        let delete = delete_property_request(order, xid, state);
        assert_eq!(
            RenderFixture::error_of(&clip_send(&mut fixture, order, &delete)),
            Some(XErrorCode::BadAccess)
        );
    }
}
