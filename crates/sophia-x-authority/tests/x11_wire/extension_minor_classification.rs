/// Refusing an extension minor is two-tier, and the tiers mean different
/// things to a client. A minor defined within the advertised version but not
/// implemented answers BadImplementation: the server of that version has an
/// entry for it and declined. A minor beyond the advertised version answers
/// BadRequest, because a genuine server of that version had no entry at all.
///
/// The doctrine is stated in `wire/constants.rs` beside the RENDER minors and
/// was already honoured by RENDER and XFIXES. This checks every extension that
/// classifies, so one of them cannot quietly drift back to a single answer.
#[test]
fn every_extension_separates_unimplemented_minors_from_unknown_ones() {
    let namespace = NamespaceId::from_raw(46);

    // (major opcode, a minor inside the advertised version that is not
    // implemented, a minor beyond the advertised version)
    let cases: Vec<(u8, u8, u8, fn(u8) -> XWireRequest)> = vec![
        // Present 1.2 defines 0..=4 and implements all of them, so the
        // in-version tier is exercised by its own boundary rather than a gap.
        (X_PRESENT_MAJOR_OPCODE, 4, 255, |minor| {
            XWireRequest::PresentUnimplemented { minor_opcode: minor }
        }),
        // DRI3 minor 5 is FDFromFence: defined, deliberately not implemented.
        (X_DRI3_MAJOR_OPCODE, 5, 200, |minor| {
            XWireRequest::Dri3Unimplemented { minor_opcode: minor }
        }),
        // SHAPE minor 6 is SelectInput.
        (X_SHAPE_MAJOR_OPCODE, 6, 200, |minor| {
            XWireRequest::ShapeUnimplemented { minor_opcode: minor }
        }),
        // XF86VidMode minor 10 is SwitchToMode.
        (X_XF86_VIDMODE_MAJOR_OPCODE, 10, 200, |minor| {
            XWireRequest::XF86VidModeUnimplemented { minor_opcode: minor }
        }),
        // GLX minor 30 sits inside the range its last minor, 35, closes.
        (X_GLX_MAJOR_OPCODE, 30, 200, |minor| {
            XWireRequest::GlxUnimplemented { minor_opcode: minor }
        }),
        // XFIXES and RENDER already classified; they are here so a regression
        // in the shared doctrine is caught wherever it happens.
        (X_XFIXES_MAJOR_OPCODE, 30, 200, |minor| {
            XWireRequest::XfixesUnimplemented { minor_opcode: minor }
        }),
        (X_RENDER_MAJOR_OPCODE, 10, 200, |minor| {
            XWireRequest::RenderUnimplemented { minor_opcode: minor }
        }),
    ];

    for (major, defined_minor, unknown_minor, build) in cases {
        for (minor, expected) in [
            (defined_minor, XErrorCode::BadImplementation),
            (unknown_minor, XErrorCode::BadRequest),
        ] {
            let mut runtime = XAuthorityRuntime::new();
            let mut atoms = XAtomTable::new();
            let mut properties = XPropertyTable::new();
            let result = dispatch_x11_wire_request(
                dispatch_context(namespace, 7, XByteOrder::LittleEndian, major),
                build(minor),
                &mut runtime,
                &mut atoms,
                &mut properties,
            );
            let [XClientOutput::Error(error)] = result.outputs.as_slice() else {
                panic!("major {major} minor {minor}: expected one error, got {:?}", result.outputs);
            };
            assert_eq!(
                error.code, expected,
                "major {major} minor {minor} must answer {expected:?}"
            );
            assert_eq!(error.major_code, major, "the error names its extension");
            assert_eq!(
                error.minor_code,
                u16::from(minor),
                "the error names the refused minor"
            );
            assert_eq!(error.sequence, 7, "the client can attribute the failure");
        }
    }
}
