struct X11XiSourceEvent<'a> {
    byte_order: XByteOrder,
    sequence: u16,
    namespace: NamespaceId,
    client: XServerFrontendClientId,
    authority: &'a crate::XInputAuthorityState,
    selections: &'a XCoreEventSelectionState,
    surface_window: XResourceId,
    ancestry: &'a [XResourceId],
    pointer: XAuthorityPointerEvent,
    crossing: Option<(Option<XResourceId>, XResourceId)>,
}

// The source stream describes the same Engine-admitted event. It never samples
// a device or chooses another application route. XI2 orders it before the master.
fn encode_xi_source_pointer_events(event: X11XiSourceEvent<'_>) -> [Option<Vec<u8>>; 4] {
    let source = crate::X_INPUT_POINTER_SOURCE_ID;
    let selected = |ancestry: &[XResourceId], event_type| {
        x11_selected_xi_event_window(
            event.authority,
            event.namespace,
            event.client.raw(),
            ancestry,
            source,
            event_type,
        )
    };
    let mut records = [None, None, None, None];
    let mut count = 0;
    let mut push = |bytes| {
        records[count] = Some(bytes);
        count += 1;
    };
    if let Some((previous, current)) = event.crossing {
        for (window, event_type) in [(previous, 8), (Some(current), 7)] {
            let Some(window) = window else { continue };
            let ancestry = event.selections.ancestry_including(window);
            let Some(target) = selected(&ancestry, event_type) else {
                continue;
            };
            let mut pointer = event.pointer;
            (pointer.event_x, pointer.event_y) = event.selections.pointer_event_coordinates(
                event.surface_window,
                target,
                pointer.event_x,
                pointer.event_y,
            );
            let mut bytes = encode_xi_crossing_event(
                event.byte_order,
                event.sequence,
                event_type,
                XAuthorityInputEvent::Pointer(pointer),
                target,
            );
            write_xi_u16(event.byte_order, &mut bytes[10..12], source);
            push(bytes);
        }
    }
    let types = match event.pointer.kind {
        XAuthorityPointerEventKind::Motion => [(Some(6), 0), (None, 0)],
        XAuthorityPointerEventKind::Button { pressed, .. } => {
            [(Some(if pressed { 4 } else { 5 }), 0), (None, 0)]
        }
        XAuthorityPointerEventKind::Axis {
            pressed,
            horizontal_position_v120,
            vertical_position_v120,
            ..
        } => [
            (
                (horizontal_position_v120.is_some() || vertical_position_v120.is_some())
                    .then_some(6),
                0,
            ),
            (Some(if pressed { 4 } else { 5 }), XI_POINTER_EMULATED),
        ],
    };
    for (event_type, flags) in types {
        let Some(event_type) = event_type else {
            continue;
        };
        let target = selected(event.ancestry, event_type);
        let Some(delivery) = x11_xi_pointer_delivery(
            event.selections,
            event.surface_window,
            event.ancestry,
            target,
            event.pointer.event_x,
            event.pointer.event_y,
        ) else {
            continue;
        };
        let mut bytes = encode_xi_device_event(
            event.byte_order,
            event.sequence,
            event_type,
            XAuthorityInputEvent::Pointer(event.pointer),
            delivery.window,
            delivery.child,
            delivery.event_x,
            delivery.event_y,
            flags,
        );
        write_xi_u16(event.byte_order, &mut bytes[10..12], source);
        push(bytes);
    }
    records
}
