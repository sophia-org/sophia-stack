fn dispatch_xfixes_request(
    context: XDispatchContext,
    request: XWireRequest,
    runtime: &mut XAuthorityRuntime,
    atoms: &mut XAtomTable,
) -> XDispatchFamilyResult {
    if !matches!(
        &request,
            XWireRequest::XfixesCreateRegion { .. }
            | XWireRequest::XfixesSetRegion { .. }
            | XWireRequest::XfixesDestroyRegion { .. }
            | XWireRequest::XfixesSelectSelectionInput { .. }
            | XWireRequest::XfixesCombineRegion { .. }
            | XWireRequest::XfixesInvertRegion { .. }
            | XWireRequest::XfixesTranslateRegion { .. }
            | XWireRequest::XfixesRegionExtents { .. }
            | XWireRequest::XfixesFetchRegion { .. }
            | XWireRequest::XfixesUnimplemented { .. }
    ) {
        return Unhandled(request);
    }
    Handled(match request {
                XWireRequest::XfixesCreateRegion { region, rectangles } => {
                    let output = runtime
                        .create_xfixes_region(
                            context.namespace,
                            region,
                            rectangles,
                            u64::from(context.sequence),
                        )
                        .err()
                        .map(|error| {
                            XClientOutput::Error(x_error_from_runtime(
                                error,
                                context.sequence,
                                context.major_opcode,
                                u16::from(crate::X_XFIXES_CREATE_REGION_MINOR_OPCODE),
                                u32::try_from(region.local.raw()).unwrap_or(0)))
                        });
                    XDispatchResult {
                        response: None,
                        outputs: output.into_iter().collect(),
                        metadata_candidates: Vec::new(),
                    }
                }
                XWireRequest::XfixesSetRegion { region, rectangles } => {
                    let output = runtime
                        .set_xfixes_region(context.namespace, region, rectangles)
                        .err()
                        .map(|error| {
                            XClientOutput::Error(x_error_from_runtime(
                                error,
                                context.sequence,
                                context.major_opcode,
                                u16::from(crate::X_XFIXES_SET_REGION_MINOR_OPCODE),
                                u32::try_from(region.local.raw()).unwrap_or(0)))
                        });
                    XDispatchResult {
                        response: None,
                        outputs: output.into_iter().collect(),
                        metadata_candidates: Vec::new(),
                    }
                }
                XWireRequest::XfixesDestroyRegion { region } => {
                    let output = runtime
                        .destroy_xfixes_region(context.namespace, region)
                        .err()
                        .map(|error| {
                            XClientOutput::Error(x_error_from_runtime(
                                error,
                                context.sequence,
                                context.major_opcode,
                                u16::from(crate::X_XFIXES_DESTROY_REGION_MINOR_OPCODE),
                                u32::try_from(region.local.raw()).unwrap_or(0)))
                        });
                    XDispatchResult {
                        response: None,
                        outputs: output.into_iter().collect(),
                        metadata_candidates: Vec::new(),
                    }
                }
                XWireRequest::XfixesSelectSelectionInput {
                    window,
                    selection,
                    event_mask,
                } => {
                    let output = if event_mask & !0b111 != 0 {
                        Some(XClientOutput::Error(crate::XClientError {
                            code: XErrorCode::BadValue,
                            sequence: context.sequence,
                            resource_id: event_mask,
                            minor_code: crate::X_XFIXES_SELECT_SELECTION_INPUT_MINOR_OPCODE.into(),
                            major_code: context.major_opcode,
                        }))
                    } else if atoms.name(selection).is_none() {
                        Some(XClientOutput::Error(crate::XClientError {
                            code: XErrorCode::BadAtom,
                            sequence: context.sequence,
                            resource_id: selection,
                            minor_code: crate::X_XFIXES_SELECT_SELECTION_INPUT_MINOR_OPCODE.into(),
                            major_code: context.major_opcode,
                        }))
                    } else if let Err(error) =
                        validate_window_or_root_access(runtime, context.namespace, window)
                    {
                        let error = x_error_from_runtime(
                            error,
                            context.sequence,
                            context.major_opcode,
                            u16::from(crate::X_XFIXES_SELECT_SELECTION_INPUT_MINOR_OPCODE),
                            u32::try_from(window.local.raw()).unwrap_or(0));
                        Some(XClientOutput::Error(error))
                    } else {
                        None
                    };
                    XDispatchResult {
                        response: None,
                        outputs: output.into_iter().collect(),
                        metadata_candidates: Vec::new(),
                    }
                }
        XWireRequest::XfixesCombineRegion {
            minor_opcode,
            source,
            other,
            destination,
        } => {
            let combine: fn(&[Rect], &[Rect]) -> Vec<Rect> = match minor_opcode {
                crate::X_XFIXES_UNION_REGION_MINOR_OPCODE => {
                    sophia_protocol::geometry::region_algebra::union
                }
                crate::X_XFIXES_INTERSECT_REGION_MINOR_OPCODE => {
                    sophia_protocol::geometry::region_algebra::intersect
                }
                crate::X_XFIXES_SUBTRACT_REGION_MINOR_OPCODE => {
                    sophia_protocol::geometry::region_algebra::subtract
                }
                // Copy names one source twice, so unioning it with itself is
                // the copy -- and canonicalises on the way, which is what
                // every other operation here leaves behind too.
                _ => sophia_protocol::geometry::region_algebra::union,
            };
            xfixes_region_result(
                context,
                runtime.combine_xfixes_regions(
                    context.namespace,
                    source,
                    other,
                    destination,
                    combine,
                ),
                minor_opcode,
                destination,
            )
        }
        XWireRequest::XfixesInvertRegion {
            source,
            bounds,
            destination,
        } => xfixes_region_result(
            context,
            runtime.invert_xfixes_region(context.namespace, source, bounds, destination),
            crate::X_XFIXES_INVERT_REGION_MINOR_OPCODE,
            destination,
        ),
        XWireRequest::XfixesTranslateRegion { region, dx, dy } => xfixes_region_result(
            context,
            runtime.translate_xfixes_region(context.namespace, region, dx, dy),
            crate::X_XFIXES_TRANSLATE_REGION_MINOR_OPCODE,
            region,
        ),
        XWireRequest::XfixesRegionExtents {
            source,
            destination,
        } => xfixes_region_result(
            context,
            runtime.set_xfixes_region_to_extents(context.namespace, source, destination),
            crate::X_XFIXES_REGION_EXTENTS_MINOR_OPCODE,
            destination,
        ),
        XWireRequest::XfixesFetchRegion { region } => {
            let outputs = match runtime.fetch_xfixes_region(context.namespace, region) {
                Ok(rects) => {
                    let extents = sophia_protocol::geometry::region_algebra::extents(&rects)
                        .unwrap_or_default();
                    vec![XClientOutput::Reply(XClientReply::XfixesFetchRegion {
                        sequence: context.sequence,
                        extents,
                        rects,
                    })]
                }
                Err(error) => vec![XClientOutput::Error(x_error_from_runtime(
                    error,
                    context.sequence,
                    context.major_opcode,
                    u16::from(crate::X_XFIXES_FETCH_REGION_MINOR_OPCODE),
                    u32::try_from(region.local.raw()).unwrap_or(0),
                ))],
            };
            XDispatchResult {
                response: None,
                outputs,
                metadata_candidates: Vec::new(),
            }
        }
        // Refused by name. This server answers XFIXES 6.0 without every
        // minor behind it, so a minor defined by that version and not
        // implemented says so, and one above it says the version does not
        // reach that far.
        XWireRequest::XfixesUnimplemented { minor_opcode } => XDispatchResult {
            response: None,
            outputs: vec![XClientOutput::Error(crate::XClientError {
                code: if minor_opcode <= crate::X_XFIXES_LAST_MINOR_OPCODE {
                    XErrorCode::BadImplementation
                } else {
                    XErrorCode::BadRequest
                },
                sequence: context.sequence,
                resource_id: 0,
                minor_code: u16::from(minor_opcode),
                major_code: context.major_opcode,
            })],
            metadata_candidates: Vec::new(),
        },
        _ => unreachable!("request family checked before dispatch"),
    })
}

/// One region operation's outcome as a dispatch result: nothing on success,
/// a named error on failure.
fn xfixes_region_result(
    context: XDispatchContext,
    outcome: Result<(), XAuthorityRuntimeError>,
    minor_opcode: u8,
    resource: crate::XResourceId,
) -> XDispatchResult {
    XDispatchResult {
        response: None,
        outputs: outcome
            .err()
            .map(|error| {
                XClientOutput::Error(x_error_from_runtime(
                    error,
                    context.sequence,
                    context.major_opcode,
                    u16::from(minor_opcode),
                    u32::try_from(resource.local.raw()).unwrap_or(0),
                ))
            })
            .into_iter()
            .collect(),
        metadata_candidates: Vec::new(),
    }
}
