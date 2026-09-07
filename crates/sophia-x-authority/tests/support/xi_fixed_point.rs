// XI2 FP3232 is an INT32 integral followed by a CARD32 fraction. Decode
// those fields independently, as libxcb does, rather than as a native i64.
fn decode_xi_fp3232(order: XByteOrder, bytes: &[u8]) -> (i32, u32) {
    let integral = bytes[..4].try_into().unwrap();
    let fraction = bytes[4..8].try_into().unwrap();
    match order {
        XByteOrder::LittleEndian => (i32::from_le_bytes(integral), u32::from_le_bytes(fraction)),
        XByteOrder::BigEndian => (i32::from_be_bytes(integral), u32::from_be_bytes(fraction)),
    }
}

#[test]
fn xi_scroll_events_preserve_signed_positions_in_both_byte_orders() {
    for order in [XByteOrder::LittleEndian, XByteOrder::BigEndian] {
        for position in [120, 240, 0, -120, -240, i32::MIN, i32::MAX] {
            let event = XAuthorityInputEvent::Pointer(XAuthorityPointerEvent {
                kind: XAuthorityPointerEventKind::Axis {
                    button: 5,
                    pressed: true,
                    horizontal_position_v120: Some(position),
                    vertical_position_v120: Some(position),
                },
                surface: SurfaceId::new(1, 1),
                root_x: 0,
                root_y: 0,
                event_x: 0,
                event_y: 0,
                state: 0,
                time_msec: 1,
            });
            let bytes = encode_xi_device_event(
                order,
                1,
                6,
                event,
                XResourceId::new(1, 1),
                XResourceId::NONE,
                0,
                0,
                0,
            );
            assert_eq!(bytes.len(), 100);
            assert_eq!(decode_xi_fp3232(order, &bytes[84..92]), (position, 0));
            assert_eq!(decode_xi_fp3232(order, &bytes[92..100]), (position, 0));
        }
    }
}

#[test]
fn xi_device_classes_encode_integral_then_fraction_in_both_byte_orders() {
    for order in [XByteOrder::LittleEndian, XByteOrder::BigEndian] {
        let bytes = crate::encode_x_client_reply(
            order,
            crate::XClientReply::XiQueryDevice {
                sequence: 1,
                devices: vec![crate::XXiDeviceInfo {
                    device_id: 2,
                    device_type: 1,
                    attachment: 3,
                    name: String::new(),
                    classes: vec![
                        crate::XXiDeviceClass::Valuator {
                            label: 0,
                            source_id: 2,
                            number: 2,
                            min: -(3_i64 << 31),              // -1.5
                            max: (7_i64 << 32) | 0x4000_0000, // 7.25
                            value: -(1_i64 << 32),
                        },
                        crate::XXiDeviceClass::Scroll {
                            source_id: 2,
                            number: 2,
                            scroll_type: 2,
                            flags: 0,
                            increment: 120_i64 << 32,
                        },
                    ],
                }],
            },
        );
        assert_eq!(decode_xi_fp3232(order, &bytes[56..64]), (-2, 0x8000_0000));
        assert_eq!(decode_xi_fp3232(order, &bytes[64..72]), (7, 0x4000_0000));
        assert_eq!(decode_xi_fp3232(order, &bytes[72..80]), (-1, 0));
        assert_eq!(decode_xi_fp3232(order, &bytes[104..112]), (120, 0));
    }
}
