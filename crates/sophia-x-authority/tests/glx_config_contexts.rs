use sophia_protocol::{NamespaceId, TransactionId};
use sophia_x_authority::*;

struct Client {
    runtime: XAuthorityRuntime,
    atoms: XAtomTable,
    properties: XPropertyTable,
    sequence: u16,
}

impl Client {
    fn new(pixmap_textures: bool) -> Self {
        let mut runtime = XAuthorityRuntime::new();
        runtime.set_pixmap_textures_supported(pixmap_textures);
        Self {
            runtime,
            atoms: XAtomTable::new(),
            properties: XPropertyTable::new(),
            sequence: 0,
        }
    }

    fn send(&mut self, request: XWireRequest) -> Vec<XClientOutput> {
        self.send_with_opcode(X_GLX_MAJOR_OPCODE, request)
    }

    fn send_with_opcode(&mut self, major_opcode: u8, request: XWireRequest) -> Vec<XClientOutput> {
        self.sequence += 1;
        dispatch_x11_wire_request(
            XDispatchContext {
                byte_order: XByteOrder::LittleEndian,
                namespace: NamespaceId::from_raw(71),
                transaction: TransactionId::from_raw(u64::from(self.sequence)),
                sequence: self.sequence,
                major_opcode,
                client_id: 1,
            },
            request,
            &mut self.runtime,
            &mut self.atoms,
            &mut self.properties,
        )
        .outputs
    }

    fn create_pixmap(&mut self, pixmap: XResourceId, depth: u8) {
        let outputs = self.send_with_opcode(
            53,
            XWireRequest::CreatePixmap {
                depth,
                pixmap,
                drawable: XResourceId::new(u64::from(X_SETUP_DEFAULT_ROOT), 1),
                width: 3,
                height: 2,
            },
        );
        assert!(
            outputs.is_empty(),
            "core pixmap creation failed: {outputs:?}"
        );
    }
}

#[test]
fn every_advertised_configuration_can_create_and_bind_a_direct_context() {
    for supported in [false, true] {
        let mut client = Client::new(supported);
        let configurations = match client
            .send(XWireRequest::GlxGetFbConfigs { screen: 0 })
            .remove(0)
        {
            XClientOutput::Reply(XClientReply::GlxFbConfigs { configs, .. }) => configs,
            other => panic!("unexpected configuration reply: {other:?}"),
        };
        for attributes in configurations {
            let config = attributes
                .iter()
                .find(|(attribute, _)| *attribute == X_GLX_FBCONFIG_ID_ATTRIBUTE)
                .unwrap()
                .1;
            let context = XResourceId::new(u64::from(config) * 10, 1);
            let pbuffer = XResourceId::new(u64::from(config) * 10 + 1, 1);
            assert!(
                client
                    .send(XWireRequest::GlxCreateContext {
                        context,
                        config: XGlxContextConfig::FbConfig(config),
                        screen: 0,
                        share: None,
                        direct: true,
                    })
                    .is_empty(),
                "advertised config {config} rejected its context"
            );
            assert!(
                client
                    .send(XWireRequest::GlxCreatePbuffer {
                        screen: 0,
                        fbconfig: config,
                        pbuffer,
                        width: 3,
                        height: 2,
                        largest: false,
                    })
                    .is_empty()
            );
            assert!(matches!(
                client
                    .send(XWireRequest::GlxMakeCurrent {
                        drawable: Some(pbuffer),
                        context: Some(context),
                        old_context_tag: 0,
                    })
                    .as_slice(),
                [XClientOutput::Reply(XClientReply::GlxMakeCurrent {
                    context_tag: 1,
                    ..
                })]
            ));
        }
    }
}

#[test]
fn hidden_stencil_configurations_cannot_be_used_without_the_provider() {
    let mut client = Client::new(false);
    for config in 4..=6 {
        let context = XResourceId::new(u64::from(config) * 10, 1);
        assert!(matches!(
            client
                .send(XWireRequest::GlxCreateContext {
                    context,
                    config: XGlxContextConfig::FbConfig(config),
                    screen: 0,
                    share: None,
                    direct: true,
                })
                .as_slice(),
            [XClientOutput::Error(_)]
        ));
        assert!(
            client
                .runtime
                .glx_context(NamespaceId::from_raw(71), context)
                .is_err()
        );
    }
}

#[test]
fn a_glx_pixmap_cannot_take_an_existing_graphics_context_id() {
    let mut client = Client::new(true);
    let pixmap = XResourceId::new(0x1001, 1);
    let gc = XResourceId::new(0x1002, 1);
    client.create_pixmap(pixmap, 32);
    let values = XGraphicsContextValues {
        foreground: 0x12345678,
        ..Default::default()
    };
    assert!(
        client
            .send_with_opcode(
                55,
                XWireRequest::CreateGraphicsContext {
                    gc,
                    drawable: pixmap,
                    values: values.clone(),
                }
            )
            .is_empty()
    );

    let outputs = client.send(XWireRequest::GlxCreatePixmap {
        screen: 0,
        fbconfig: 2,
        pixmap,
        glx_pixmap: gc,
        target: Some(X_GLX_TEXTURE_2D_VALUE),
        format: Some(X_GLX_TEXTURE_FORMAT_RGBA_VALUE),
        mipmap: Some(false),
    });
    assert!(
        matches!(outputs.as_slice(), [XClientOutput::Error(_)]),
        "a GLX pixmap reused the live GC id: {outputs:?}"
    );
    assert_eq!(
        client
            .runtime
            .graphics_context_values(NamespaceId::from_raw(71), gc)
            .unwrap(),
        values
    );
    assert!(
        client
            .runtime
            .glx_pixmap(NamespaceId::from_raw(71), gc)
            .is_err(),
        "refusing the collision must not create a GLX alias"
    );
}

fn check_make_context_current(pixmap: bool, direct: bool) {
    let mut client = Client::new(true);
    let backing = XResourceId::new(0x2001, 1);
    let drawable = XResourceId::new(0x2002, 1);
    let context = XResourceId::new(0x2003, 1);
    if pixmap {
        client.create_pixmap(backing, 32);
        let outputs = client.send(XWireRequest::GlxCreatePixmap {
            screen: 0,
            fbconfig: 2,
            pixmap: backing,
            glx_pixmap: drawable,
            target: Some(X_GLX_TEXTURE_2D_VALUE),
            format: Some(X_GLX_TEXTURE_FORMAT_RGBA_VALUE),
            mipmap: Some(false),
        });
        assert!(
            outputs.is_empty(),
            "GLX pixmap creation failed: {outputs:?}"
        );
    } else {
        assert!(
            client
                .send(XWireRequest::GlxCreatePbuffer {
                    screen: 0,
                    fbconfig: 2,
                    pbuffer: drawable,
                    width: 3,
                    height: 2,
                    largest: false,
                })
                .is_empty()
        );
    }
    let created = client.send(XWireRequest::GlxCreateContext {
        context,
        config: XGlxContextConfig::FbConfig(2),
        screen: 0,
        share: None,
        direct,
    });
    assert!(created.is_empty(), "context creation failed: {created:?}");
    assert!(
        matches!(client.send(XWireRequest::GlxIsDirect { context }).as_slice(),
        [XClientOutput::Reply(XClientReply::GlxIsDirect { direct: observed, .. })] if *observed == direct)
    );

    let outputs = client.send(XWireRequest::GlxMakeContextCurrent {
        drawable,
        read_drawable: drawable,
        context: Some(context),
    });
    if direct {
        assert!(
            matches!(
                outputs.as_slice(),
                [XClientOutput::Reply(XClientReply::GlxMakeCurrent {
                    context_tag: 1,
                    ..
                })]
            ),
            "direct context was not bound to pixmap={pixmap}: {outputs:?}"
        );
    } else {
        assert!(
            matches!(outputs.as_slice(), [XClientOutput::Error(_)]),
            "indirect rendering was accepted for pixmap={pixmap}: {outputs:?}"
        );
    }
}

#[test]
fn make_context_current_binds_a_direct_pixmap() {
    check_make_context_current(true, true);
}

#[test]
fn make_context_current_binds_a_direct_pbuffer() {
    check_make_context_current(false, true);
}

#[test]
fn make_context_current_refuses_an_indirect_pixmap() {
    check_make_context_current(true, false);
}

#[test]
fn make_context_current_refuses_an_indirect_pbuffer() {
    check_make_context_current(false, false);
}

fn config_attribute(attributes: &[(u32, u32)], name: u32) -> u32 {
    attributes
        .iter()
        .find(|(attribute, _)| *attribute == name)
        .unwrap_or_else(|| panic!("configuration omitted {name:#x}: {attributes:?}"))
        .1
}

#[test]
fn advertised_rgb_and_rgba_bindings_agree_with_pixmap_creation() {
    let mut client = Client::new(true);
    let configurations = match client
        .send(XWireRequest::GlxGetFbConfigs { screen: 0 })
        .remove(0)
    {
        XClientOutput::Reply(XClientReply::GlxFbConfigs { configs, .. }) => configs,
        other => panic!("unexpected configuration reply: {other:?}"),
    };
    assert!(!configurations.is_empty());
    let mut opaque_rgba_cases = 0;
    for attributes in configurations {
        let config = config_attribute(&attributes, X_GLX_FBCONFIG_ID_ATTRIBUTE);
        let alpha = config_attribute(&attributes, X_GLX_ALPHA_SIZE_ATTRIBUTE);
        let depth =
            u8::try_from(config_attribute(&attributes, X_GLX_BUFFER_SIZE_ATTRIBUTE)).unwrap();
        for (index, capability, format) in [
            (
                0,
                X_GLX_BIND_TO_TEXTURE_RGB_ATTRIBUTE,
                X_GLX_TEXTURE_FORMAT_RGB_VALUE,
            ),
            (
                1,
                X_GLX_BIND_TO_TEXTURE_RGBA_ATTRIBUTE,
                X_GLX_TEXTURE_FORMAT_RGBA_VALUE,
            ),
        ] {
            let advertised = config_attribute(&attributes, capability);
            assert!(advertised <= 1, "binding capability must be boolean");
            if alpha == 0 && format == X_GLX_TEXTURE_FORMAT_RGBA_VALUE && advertised == 1 {
                opaque_rgba_cases += 1;
            }
            let pixmap = XResourceId::new(0x3000 + u64::from(config) * 10 + index * 2, 1);
            let glx_pixmap = XResourceId::new(pixmap.local.raw() + 1, 1);
            client.create_pixmap(pixmap, depth);
            let outputs = client.send(XWireRequest::GlxCreatePixmap {
                screen: 0,
                fbconfig: config,
                pixmap,
                glx_pixmap,
                target: Some(X_GLX_TEXTURE_2D_VALUE),
                format: Some(format),
                mipmap: Some(false),
            });
            if advertised == 1 {
                assert!(
                    outputs.is_empty(),
                    "config {config} advertises format {format:#x} but rejects it: {outputs:?}"
                );
                assert_eq!(
                    client
                        .runtime
                        .glx_pixmap(NamespaceId::from_raw(71), glx_pixmap)
                        .unwrap(),
                    (pixmap, config)
                );
                let (_, _, texture) = client
                    .runtime
                    .glx_pixmap_attributes(NamespaceId::from_raw(71), glx_pixmap)
                    .unwrap();
                assert_eq!(
                    texture.format, format,
                    "the requested format must survive creation"
                );
            } else {
                assert!(
                    matches!(outputs.as_slice(), [XClientOutput::Error(_)]),
                    "config {config} accepts unadvertised format {format:#x}: {outputs:?}"
                );
                assert!(
                    client
                        .runtime
                        .glx_pixmap(NamespaceId::from_raw(71), glx_pixmap)
                        .is_err()
                );
            }
        }
    }
    assert!(
        opaque_rgba_cases > 0,
        "opaque RGBA must exercise the advertised compatibility row"
    );
}
