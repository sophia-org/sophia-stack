fn decode_xfixes(
    context: XWireClientContext,
    bytes: &[u8],
) -> Result<XWireRequest, XWireParseError> {
    match bytes[1] {
        X_XFIXES_QUERY_VERSION_MINOR_OPCODE => decode_extension_query_version(
            context,
            bytes,
            X_XFIXES_MAJOR_OPCODE,
            X_XFIXES_QUERY_VERSION_MINOR_OPCODE,
            |major_version, minor_version| XWireRequest::XfixesQueryVersion {
                major_version,
                minor_version,
            },
        ),
        X_XFIXES_SELECT_SELECTION_INPUT_MINOR_OPCODE => {
            require_exact_len(X_XFIXES_MAJOR_OPCODE, 16, bytes.len())?;
            Ok(XWireRequest::XfixesSelectSelectionInput {
                window: XResourceId::new(u64::from(context.byte_order.u32(&bytes[4..8])), 1),
                selection: context.byte_order.u32(&bytes[8..12]),
                event_mask: context.byte_order.u32(&bytes[12..16]),
            })
        }
        X_XFIXES_CREATE_REGION_MINOR_OPCODE => {
            require_len(X_XFIXES_MAJOR_OPCODE, 8, bytes.len())?;
            if !(bytes.len() - 8).is_multiple_of(8) {
                return Err(XWireParseError::InvalidLength {
                    opcode: X_XFIXES_MAJOR_OPCODE,
                    expected_at_least: 8,
                    actual: bytes.len(),
                });
            }
            let region = context.byte_order.u32(&bytes[4..8]);
            context.validate_new_resource_id(region)?;
            let rectangles = bytes[8..]
                .chunks_exact(8)
                .map(|rectangle| Rect {
                    x: i32::from(context.byte_order.i16(&rectangle[0..2])),
                    y: i32::from(context.byte_order.i16(&rectangle[2..4])),
                    width: i32::from(context.byte_order.u16(&rectangle[4..6])),
                    height: i32::from(context.byte_order.u16(&rectangle[6..8])),
                })
                .collect();
            Ok(XWireRequest::XfixesCreateRegion {
                region: XResourceId::new(u64::from(region), 1),
                rectangles,
            })
        }
        X_XFIXES_SET_REGION_MINOR_OPCODE => {
            require_len(X_XFIXES_MAJOR_OPCODE, 8, bytes.len())?;
            if !(bytes.len() - 8).is_multiple_of(8) {
                return Err(XWireParseError::InvalidLength {
                    opcode: X_XFIXES_MAJOR_OPCODE,
                    expected_at_least: 8,
                    actual: bytes.len(),
                });
            }
            let rectangles = bytes[8..]
                .chunks_exact(8)
                .map(|rectangle| Rect {
                    x: i32::from(context.byte_order.i16(&rectangle[0..2])),
                    y: i32::from(context.byte_order.i16(&rectangle[2..4])),
                    width: i32::from(context.byte_order.u16(&rectangle[4..6])),
                    height: i32::from(context.byte_order.u16(&rectangle[6..8])),
                })
                .collect();
            Ok(XWireRequest::XfixesSetRegion {
                region: XResourceId::new(u64::from(context.byte_order.u32(&bytes[4..8])), 1),
                rectangles,
            })
        }
        X_XFIXES_DESTROY_REGION_MINOR_OPCODE => {
            require_exact_len(X_XFIXES_MAJOR_OPCODE, 8, bytes.len())?;
            Ok(XWireRequest::XfixesDestroyRegion {
                region: XResourceId::new(u64::from(context.byte_order.u32(&bytes[4..8])), 1),
            })
        }
        minor @ (X_XFIXES_COPY_REGION_MINOR_OPCODE
        | X_XFIXES_UNION_REGION_MINOR_OPCODE
        | X_XFIXES_INTERSECT_REGION_MINOR_OPCODE
        | X_XFIXES_SUBTRACT_REGION_MINOR_OPCODE) => {
            let expected = if minor == X_XFIXES_COPY_REGION_MINOR_OPCODE {
                X_XFIXES_COPY_REGION_REQ_LEN
            } else {
                X_XFIXES_COMBINE_REGION_REQ_LEN
            };
            require_exact_len(X_XFIXES_MAJOR_OPCODE, expected, bytes.len())?;
            // Copy has one source; the rest have two. Reading the second as
            // the first keeps one variant serving all four.
            let (source, other) = if minor == X_XFIXES_COPY_REGION_MINOR_OPCODE {
                let source = context.byte_order.u32(&bytes[4..8]);
                (source, source)
            } else {
                (
                    context.byte_order.u32(&bytes[4..8]),
                    context.byte_order.u32(&bytes[8..12]),
                )
            };
            let destination = if minor == X_XFIXES_COPY_REGION_MINOR_OPCODE {
                context.byte_order.u32(&bytes[8..12])
            } else {
                context.byte_order.u32(&bytes[12..16])
            };
            Ok(XWireRequest::XfixesCombineRegion {
                minor_opcode: minor,
                source: XResourceId::new(u64::from(source), 1),
                other: XResourceId::new(u64::from(other), 1),
                destination: XResourceId::new(u64::from(destination), 1),
            })
        }
        X_XFIXES_INVERT_REGION_MINOR_OPCODE => {
            require_exact_len(
                X_XFIXES_MAJOR_OPCODE,
                X_XFIXES_INVERT_REGION_REQ_LEN,
                bytes.len(),
            )?;
            Ok(XWireRequest::XfixesInvertRegion {
                source: XResourceId::new(u64::from(context.byte_order.u32(&bytes[4..8])), 1),
                bounds: Rect {
                    x: i32::from(context.byte_order.i16(&bytes[8..10])),
                    y: i32::from(context.byte_order.i16(&bytes[10..12])),
                    width: i32::from(context.byte_order.u16(&bytes[12..14])),
                    height: i32::from(context.byte_order.u16(&bytes[14..16])),
                },
                destination: XResourceId::new(u64::from(context.byte_order.u32(&bytes[16..20])), 1),
            })
        }
        X_XFIXES_TRANSLATE_REGION_MINOR_OPCODE => {
            require_exact_len(
                X_XFIXES_MAJOR_OPCODE,
                X_XFIXES_TRANSLATE_REGION_REQ_LEN,
                bytes.len(),
            )?;
            Ok(XWireRequest::XfixesTranslateRegion {
                region: XResourceId::new(u64::from(context.byte_order.u32(&bytes[4..8])), 1),
                dx: i32::from(context.byte_order.i16(&bytes[8..10])),
                dy: i32::from(context.byte_order.i16(&bytes[10..12])),
            })
        }
        minor @ (X_XFIXES_REGION_EXTENTS_MINOR_OPCODE | X_XFIXES_FETCH_REGION_MINOR_OPCODE) => {
            let expected = if minor == X_XFIXES_REGION_EXTENTS_MINOR_OPCODE {
                X_XFIXES_COPY_REGION_REQ_LEN
            } else {
                X_XFIXES_REGION_QUERY_REQ_LEN
            };
            require_exact_len(X_XFIXES_MAJOR_OPCODE, expected, bytes.len())?;
            if minor == X_XFIXES_REGION_EXTENTS_MINOR_OPCODE {
                Ok(XWireRequest::XfixesRegionExtents {
                    source: XResourceId::new(u64::from(context.byte_order.u32(&bytes[4..8])), 1),
                    destination: XResourceId::new(
                        u64::from(context.byte_order.u32(&bytes[8..12])),
                        1,
                    ),
                })
            } else {
                Ok(XWireRequest::XfixesFetchRegion {
                    region: XResourceId::new(u64::from(context.byte_order.u32(&bytes[4..8])), 1),
                })
            }
        }
        // Decoded so the refusal names the request. XFIXES answers version
        // 6.0 and does not implement every minor behind it; a parse rejection
        // would tell a client only that the extension exists, which is the
        // refusal style this server replaced everywhere else.
        minor_opcode => Ok(XWireRequest::XfixesUnimplemented { minor_opcode }),
    }
}

