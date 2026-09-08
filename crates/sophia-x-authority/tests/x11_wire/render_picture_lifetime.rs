fn retained_picture_pixel(fixture: &mut RenderFixture, picture: u32) -> [u8; 4] {
    let cursor = 0x0020_0170;
    let result = fixture.send(&render_create_cursor_request(
        RenderFixture::ORDER,
        cursor,
        picture,
        0,
        0,
    ));
    assert_eq!(RenderFixture::error_of(&result), None);
    let image = fixture
        .runtime
        .render_cursor_image(XResourceId::new(u64::from(cursor), 1))
        .unwrap();
    let pixel = image.premultiplied_bgra[..4].try_into().unwrap();
    assert_eq!(
        RenderFixture::error_of(&fixture.send(&free_cursor_request(RenderFixture::ORDER, cursor))),
        None
    );
    pixel
}

#[test]
fn render_picture_keeps_pixels_after_free_pixmap_and_xid_reuse() {
    let mut fixture = RenderFixture::with_argb_pixmap(2, 2);
    let whole = Rect {
        x: 0,
        y: 0,
        width: 2,
        height: 2,
    };
    fixture.fill_destination([0x8080, 0, 0, 0x8080], whole);
    let alias = 0x0020_0120;
    assert_eq!(
        RenderFixture::error_of(&fixture.send(&render_create_picture_request(
            RenderFixture::ORDER,
            alias,
            RenderFixture::PIXMAP,
            X_RENDER_FORMAT_ARGB32,
            &[],
        ))),
        None
    );
    assert_eq!(
        RenderFixture::error_of(&fixture.send(&free_pixmap_request(
            RenderFixture::ORDER,
            RenderFixture::PIXMAP,
        ))),
        None
    );
    assert!(
        fixture
            .runtime
            .pixmap_size(
                RenderFixture::NS,
                XResourceId::new(u64::from(RenderFixture::PIXMAP), 1)
            )
            .is_err()
    );
    assert_eq!(
        retained_picture_pixel(&mut fixture, RenderFixture::PICTURE),
        [0, 0, 0x80, 0x80]
    );

    // Reusing the public XID must not redirect an existing picture to the new
    // allocation. Two retained pictures must still see each other's writes.
    assert_eq!(
        RenderFixture::error_of(&fixture.send(&create_pixmap_request(
            RenderFixture::ORDER,
            32,
            RenderFixture::PIXMAP,
            X_SETUP_DEFAULT_ROOT,
            1,
            1,
        ))),
        None
    );
    let replacement = 0x0020_0130;
    assert_eq!(
        RenderFixture::error_of(&fixture.send(&render_create_picture_request(
            RenderFixture::ORDER,
            replacement,
            RenderFixture::PIXMAP,
            X_RENDER_FORMAT_ARGB32,
            &[],
        ))),
        None
    );
    assert_eq!(
        RenderFixture::error_of(&fixture.send(&render_fill_rectangles_request(
            RenderFixture::ORDER,
            1,
            replacement,
            [0, 0xffff, 0, 0xffff],
            &[whole],
        ))),
        None
    );
    assert_eq!(
        RenderFixture::error_of(&fixture.send(&render_fill_rectangles_request(
            RenderFixture::ORDER,
            1,
            alias,
            [0, 0, 0xffff, 0xffff],
            &[whole],
        ))),
        None
    );
    assert_eq!(
        retained_picture_pixel(&mut fixture, RenderFixture::PICTURE),
        [0xff, 0, 0, 0xff]
    );
    assert_eq!(
        retained_picture_pixel(&mut fixture, replacement),
        [0, 0xff, 0, 0xff]
    );
    assert_eq!(
        RenderFixture::error_of(
            &fixture.send(&render_free_picture_request(RenderFixture::ORDER, alias))
        ),
        None
    );
    assert_eq!(
        retained_picture_pixel(&mut fixture, RenderFixture::PICTURE),
        [0xff, 0, 0, 0xff]
    );

    // A foreign namespace cannot borrow the retained backing through its picture.
    let bytes = render_create_cursor_request(
        RenderFixture::ORDER,
        0x0040_0001,
        RenderFixture::PICTURE,
        0,
        0,
    );
    let other = NamespaceId::from_raw(89);
    let request =
        decode_x11_core_request(context(other, 9000, RenderFixture::ORDER), &bytes).unwrap();
    let result = dispatch_x11_wire_request(
        dispatch_context(other, 99, RenderFixture::ORDER, bytes[0]),
        request,
        &mut fixture.runtime,
        &mut fixture.atoms,
        &mut fixture.properties,
    );
    assert_eq!(
        RenderFixture::error_of(&result),
        Some(XErrorCode::RenderPicture)
    );
    assert_eq!(
        retained_picture_pixel(&mut fixture, RenderFixture::PICTURE),
        [0xff, 0, 0, 0xff]
    );
    assert_eq!(
        RenderFixture::error_of(&fixture.send(&render_free_picture_request(
            RenderFixture::ORDER,
            RenderFixture::PICTURE
        ))),
        None
    );
    assert!(
        !fixture
            .runtime
            .resource_id_in_use(XResourceId::new(u64::from(RenderFixture::PICTURE), 1))
    );
}

fn retained_shm_picture_fixture() -> (
    RenderFixture,
    std::sync::Weak<sophia_sysv_shm::ClientMapping>,
) {
    let mut fixture = RenderFixture::new();
    let segment = XResourceId::new(0x0020_0180, 1);
    let (mapping, _fd) = sophia_sysv_shm::DescriptorMapping::create_sealed(4096).unwrap();
    let mapping = std::sync::Arc::new(sophia_sysv_shm::ClientMapping::Descriptor(mapping));
    let weak = std::sync::Arc::downgrade(&mapping);
    fixture
        .runtime
        .attach_shm_descriptor_segment(RenderFixture::NS, segment, mapping, false, 1)
        .unwrap();
    fixture
        .runtime
        .create_shm_pixmap(
            RenderFixture::NS,
            XResourceId::new(u64::from(RenderFixture::PIXMAP), 1),
            Size {
                width: 2,
                height: 2,
            },
            32,
            1,
            segment,
            0,
        )
        .unwrap();
    assert_eq!(
        RenderFixture::error_of(&fixture.send(&render_create_picture_request(
            RenderFixture::ORDER,
            RenderFixture::PICTURE,
            RenderFixture::PIXMAP,
            X_RENDER_FORMAT_ARGB32,
            &[],
        ))),
        None
    );
    fixture
        .runtime
        .detach_shm_segment(RenderFixture::NS, segment)
        .unwrap();
    (fixture, weak)
}

#[test]
fn render_retained_backing_releases_with_last_picture() {
    let (mut fixture, mapping) = retained_shm_picture_fixture();
    assert_eq!(
        RenderFixture::error_of(&fixture.send(&free_pixmap_request(
            RenderFixture::ORDER,
            RenderFixture::PIXMAP
        ))),
        None
    );
    assert!(mapping.upgrade().is_some());
    assert_eq!(
        RenderFixture::error_of(&fixture.send(&render_free_picture_request(
            RenderFixture::ORDER,
            RenderFixture::PICTURE
        ))),
        None
    );
    assert!(
        mapping.upgrade().is_none(),
        "last picture must release retained allocation"
    );
    assert_eq!(fixture.runtime.resource_count(), 0);
}

#[test]
fn render_picture_survives_pixmap_owner_disconnect_and_releases_with_its_owner() {
    for free_first in [false, true] {
        let (mut fixture, mapping) = retained_shm_picture_fixture();
        let other_picture = 0x0040_0001;
        assert_eq!(
            RenderFixture::error_of(&fixture.send(&render_create_picture_request(
                RenderFixture::ORDER,
                other_picture,
                RenderFixture::PIXMAP,
                X_RENDER_FORMAT_ARGB32,
                &[],
            ))),
            None
        );
        if free_first {
            assert_eq!(
                RenderFixture::error_of(&fixture.send(&free_pixmap_request(
                    RenderFixture::ORDER,
                    RenderFixture::PIXMAP
                ))),
                None
            );
        }
        fixture
            .runtime
            .release_client_resource_range(
                RenderFixture::NS,
                XWireClientResourceRange {
                    base: 0x0020_0000,
                    mask: 0x001f_ffff,
                },
            )
            .unwrap();
        assert!(
            mapping.upgrade().is_some(),
            "another client in the same namespace still owns a picture"
        );
        assert_eq!(fixture.runtime.resource_count(), 1);
        let result = fixture.send(&render_fill_rectangles_request(
            RenderFixture::ORDER,
            1,
            other_picture,
            [0xffff, 0, 0, 0xffff],
            &[Rect {
                x: 0,
                y: 0,
                width: 2,
                height: 2,
            }],
        ));
        assert_eq!(RenderFixture::error_of(&result), None);
        assert_eq!(
            retained_picture_pixel(&mut fixture, other_picture),
            [0, 0, 0xff, 0xff]
        );
        fixture
            .runtime
            .release_client_resource_range(
                RenderFixture::NS,
                XWireClientResourceRange {
                    base: 0x0040_0000,
                    mask: 0x001f_ffff,
                },
            )
            .unwrap();
        assert!(mapping.upgrade().is_none());
        assert_eq!(fixture.runtime.resource_count(), 0);
    }
}

/// A GLX pixmap keeps the pixels it wrapped when the XID is freed and reused,
/// and the registration it owes is released only when the drawable goes.
///
/// The XID is the client's name for the pixmap, not the pixels' identity. A
/// backing released with the XID would be released while something still reads
/// it, and one released never would leak.
#[test]
fn a_glx_pixmap_keeps_its_backing_across_free_pixmap_and_defers_the_release() {
    const NAMESPACE: NamespaceId = NamespaceId::from_raw(77);
    const PIXMAP: u32 = 0x0060_0100;
    const GLX_PIXMAP: u32 = 0x0060_0101;
    // A power-of-two extent, so the default texture target is available.
    const WIDTH: u16 = 64;
    const HEIGHT: u16 = 64;
    const STRIDE: u16 = WIDTH * 4;

    let pixmap = XResourceId::new(u64::from(PIXMAP), 1);
    let glx_pixmap = XResourceId::new(u64::from(GLX_PIXMAP), 1);
    let mut runtime = XAuthorityRuntime::new();
    runtime.set_pixmap_textures_supported(true);

    let descriptor = runtime
        .create_dri3_pixmap(
            NAMESPACE,
            pixmap,
            1,
            u32::from(STRIDE) * u32::from(HEIGHT),
            WIDTH,
            HEIGHT,
            STRIDE,
            32,
            32,
        )
        .unwrap();
    let config = x_glx_fb_config(2, true).unwrap();
    runtime
        .create_glx_pixmap(NAMESPACE, glx_pixmap, config, pixmap, None, None, None)
        .unwrap();

    // Freeing the XID must not release a registration something still reads.
    assert_eq!(runtime.free_pixmap(NAMESPACE, pixmap).unwrap(), None);
    assert!(runtime.take_retired_pixmap_registrations(NAMESPACE).is_empty());

    // The drawable still resolves, and to a backing that is no longer the XID.
    let (backing, fbconfig) = runtime.glx_pixmap(NAMESPACE, glx_pixmap).unwrap();
    assert_eq!(fbconfig, 2);
    assert_ne!(
        backing, pixmap,
        "the backing must have left the client's XID"
    );

    // The XID is free for reuse, and reusing it must not disturb the drawable.
    runtime
        .create_dri3_pixmap(
            NAMESPACE,
            pixmap,
            2,
            u32::from(STRIDE) * u32::from(HEIGHT),
            WIDTH,
            HEIGHT,
            STRIDE,
            32,
            32,
        )
        .unwrap();
    assert_eq!(
        runtime.glx_pixmap(NAMESPACE, glx_pixmap).unwrap().0,
        backing,
        "a reused XID must not capture a drawable that outlived it",
    );

    // Imported storage retires its Engine registration, never a provider allocation.
    assert!(runtime.take_pending_backing_releases().is_empty());
    // Only when the last referent goes is the release owed.
    runtime.destroy_glx_pixmap(NAMESPACE, glx_pixmap).unwrap();
    assert_eq!(
        runtime.take_retired_pixmap_registrations(NAMESPACE),
        vec![descriptor.handle],
    );
    assert!(runtime.take_pending_backing_releases().is_empty());
    // And taken once: a release handed back is the caller's obligation, not a
    // second copy the runtime still holds.
    assert!(runtime.take_retired_pixmap_registrations(NAMESPACE).is_empty());
}

/// A release the caller could not complete is returned, not discarded.
#[test]
fn an_uncompleted_backing_release_is_returned_ahead_of_later_ones() {
    let mut runtime = XAuthorityRuntime::new();
    let first = sophia_protocol::BufferHandle::from_raw(11);
    let second = sophia_protocol::BufferHandle::from_raw(12);
    runtime.restore_pending_backing_releases([first]);
    runtime.restore_pending_backing_releases([second]);
    assert_eq!(
        runtime.take_pending_backing_releases(),
        vec![second, first],
        "a returned release goes ahead of what was queued after it",
    );
}

/// The texture attributes a GLX pixmap was created with are queryable, and the
/// ones a configuration cannot honour are refused rather than reinterpreted.
#[test]
fn glx_pixmap_texture_attributes_round_trip_and_refuse_what_is_unbacked() {
    const NAMESPACE: NamespaceId = NamespaceId::from_raw(78);
    const WIDTH: u16 = 30;
    const HEIGHT: u16 = 17;
    const STRIDE: u16 = WIDTH * 4;

    fn pixmap_of(runtime: &mut XAuthorityRuntime, raw: u32, depth: u8) -> XResourceId {
        let pixmap = XResourceId::new(u64::from(raw), 1);
        runtime
            .create_dri3_pixmap(
                NAMESPACE,
                pixmap,
                1,
                u32::from(STRIDE) * u32::from(HEIGHT),
                WIDTH,
                HEIGHT,
                STRIDE,
                depth,
                32,
            )
            .unwrap();
        pixmap
    }

    let mut runtime = XAuthorityRuntime::new();
    runtime.set_pixmap_textures_supported(true);
    let argb = x_glx_fb_config(2, true).unwrap();

    // A non-power-of-two extent binds to an ordinary 2D target: refusing it
    // would refuse what GL_ARB_texture_non_power_of_two allows.
    let pixmap = pixmap_of(&mut runtime, 0x0060_0200, 32);
    let drawable = XResourceId::new(0x0060_0201, 1);
    runtime
        .create_glx_pixmap(NAMESPACE, drawable, argb, pixmap, None, None, None)
        .unwrap();
    let (size, config, texture) = runtime.glx_pixmap_attributes(NAMESPACE, drawable).unwrap();
    assert_eq!(size.width, i32::from(WIDTH));
    assert_eq!(size.height, i32::from(HEIGHT));
    assert_eq!(config, argb.id);
    assert_eq!(texture.target, X_GLX_TEXTURE_2D_BIT_VALUE);
    assert_eq!(texture.format, X_GLX_TEXTURE_FORMAT_RGBA_VALUE);
    assert!(!texture.mipmap);

    // A named rectangle target round trips as itself.
    let rectangle = XResourceId::new(0x0060_0202, 1);
    runtime
        .create_glx_pixmap(
            NAMESPACE,
            rectangle,
            argb,
            pixmap,
            Some(X_GLX_TEXTURE_RECTANGLE_VALUE),
            None,
            None,
        )
        .unwrap();
    assert_eq!(
        runtime
            .glx_pixmap_attributes(NAMESPACE, rectangle)
            .unwrap()
            .2
            .target,
        X_GLX_TEXTURE_RECTANGLE_BIT_VALUE,
    );

    // A one-dimensional target needs a single row.
    assert!(
        runtime
            .create_glx_pixmap(
                NAMESPACE,
                XResourceId::new(0x0060_0203, 1),
                argb,
                pixmap,
                Some(X_GLX_TEXTURE_1D_VALUE),
                None,
                None,
            )
            .is_err(),
    );

    // A mipmapped binding has no capability behind it.
    assert!(
        runtime
            .create_glx_pixmap(
                NAMESPACE,
                XResourceId::new(0x0060_0204, 1),
                argb,
                pixmap,
                None,
                None,
                Some(true),
            )
            .is_err(),
    );

    // Format follows the ADVERTISED binding capability, not the alpha depth.
    // The opaque configuration advertises RGBA binding, so it must accept it;
    // refusing on alpha would refuse what the catalog promises.
    let opaque_pixmap = pixmap_of(&mut runtime, 0x0060_0210, 24);
    let opaque = x_glx_fb_config(1, true).unwrap();
    assert!(opaque.bind_to_texture_rgba());
    runtime
        .create_glx_pixmap(
            NAMESPACE,
            XResourceId::new(0x0060_0211, 1),
            opaque,
            opaque_pixmap,
            None,
            Some(X_GLX_TEXTURE_FORMAT_RGBA_VALUE),
            None,
        )
        .unwrap();

    // A format outside the extension's set is refused.
    assert!(
        runtime
            .create_glx_pixmap(
                NAMESPACE,
                XResourceId::new(0x0060_0212, 1),
                opaque,
                opaque_pixmap,
                None,
                Some(0x1234),
                None,
            )
            .is_err(),
    );

    // And the mipmap refusal is the capability the row advertises, not a
    // constant the constructor keeps to itself.
    assert!(!opaque.bind_to_mipmap_texture());
}


fn glx_create_pixmap_request(fbconfig: u32, pixmap: u32, glx_pixmap: u32) -> Vec<u8> {
    let mut out = vec![0u8; 24];
    out[0] = X_GLX_MAJOR_OPCODE;
    out[1] = X_GLX_CREATE_PIXMAP_MINOR_OPCODE;
    out[2..4].copy_from_slice(&6u16.to_le_bytes());
    out[8..12].copy_from_slice(&fbconfig.to_le_bytes());
    out[12..16].copy_from_slice(&pixmap.to_le_bytes());
    out[16..20].copy_from_slice(&glx_pixmap.to_le_bytes());
    out
}

fn glx_destroy_pixmap_request(glx_pixmap: u32) -> Vec<u8> {
    let mut out = vec![0u8; 8];
    out[0] = X_GLX_MAJOR_OPCODE;
    out[1] = X_GLX_DESTROY_PIXMAP_MINOR_OPCODE;
    out[2..4].copy_from_slice(&2u16.to_le_bytes());
    out[4..8].copy_from_slice(&glx_pixmap.to_le_bytes());
    out
}

/// A pixmap referenced by both a RENDER picture and a GLX pixmap has ONE
/// backing, and it is released exactly once, after whichever reference happens
/// to go last.
///
/// Two independent lifetimes would release it when the first went, while the
/// other was still reading, or never at all.
fn joint_render_and_glx_backing(order: &str) {
    const GLX_PIXMAP: u32 = 0x0020_0102;

    let mut fixture = RenderFixture::new();
    fixture.runtime.set_pixmap_textures_supported(true);

    // A DRI3 pixmap, so the backing owes a renderer registration.
    assert_eq!(
        RenderFixture::error_of(&fixture.send(&dri3_pixmap_from_buffer_request(
            RenderFixture::ORDER,
            RenderFixture::PIXMAP,
            X_SETUP_DEFAULT_ROOT,
            8 * 4 * 8,
            8,
            8,
            8 * 4,
            32,
            32,
        ))),
        None,
        "{order}: DRI3 pixmap",
    );
    assert_eq!(
        RenderFixture::error_of(&fixture.send(&render_create_picture_request(
            RenderFixture::ORDER,
            RenderFixture::PICTURE,
            RenderFixture::PIXMAP,
            X_RENDER_FORMAT_ARGB32,
            &[],
        ))),
        None,
        "{order}: picture",
    );
    assert_eq!(
        RenderFixture::error_of(&fixture.send(&glx_create_pixmap_request(
            2,
            RenderFixture::PIXMAP,
            GLX_PIXMAP,
        ))),
        None,
        "{order}: GLX pixmap",
    );

    // Freeing the XID retains the backing: two references still read it.
    assert_eq!(
        RenderFixture::error_of(&fixture.send(&free_pixmap_request(
            RenderFixture::ORDER,
            RenderFixture::PIXMAP,
        ))),
        None,
        "{order}: free pixmap",
    );
    assert!(
        fixture.runtime.take_retired_pixmap_registrations(RenderFixture::NS).is_empty(),
        "{order}: released while two references still held it",
    );

    let free_picture = |fixture: &mut RenderFixture| {
        assert_eq!(
            RenderFixture::error_of(&fixture.send(&render_free_picture_request(
                RenderFixture::ORDER,
                RenderFixture::PICTURE,
            ))),
            None,
        );
    };
    let destroy_glx = |fixture: &mut RenderFixture| {
        assert_eq!(
            RenderFixture::error_of(&fixture.send(&glx_destroy_pixmap_request(GLX_PIXMAP))),
            None,
        );
    };

    match order {
        "picture-first" => {
            free_picture(&mut fixture);
            assert!(
                fixture.runtime.take_retired_pixmap_registrations(RenderFixture::NS).is_empty(),
                "{order}: released while the GLX pixmap still held it",
            );
            destroy_glx(&mut fixture);
        }
        "glx-first" => {
            destroy_glx(&mut fixture);
            assert!(
                fixture.runtime.take_retired_pixmap_registrations(RenderFixture::NS).is_empty(),
                "{order}: released while the picture still held it",
            );
            free_picture(&mut fixture);
        }
        "disconnect" => {
            fixture
                .runtime
                .release_client_resource_range(
                    RenderFixture::NS,
                    XWireClientResourceRange {
                        base: 0x0020_0000,
                        mask: 0x001f_ffff,
                    },
                )
                .unwrap();
        }
        other => panic!("unknown order {other}"),
    }

    assert!(fixture.runtime.take_pending_backing_releases().is_empty(), "imported storage is not provider-owned");
    assert_eq!(
        fixture.runtime.take_retired_pixmap_registrations(RenderFixture::NS).len(),
        1,
        "{order}: the backing must be released exactly once",
    );
    assert!(
        fixture.runtime.take_retired_pixmap_registrations(RenderFixture::NS).is_empty(),
        "{order}: released twice",
    );
}

#[test]
fn joint_render_and_glx_backing_releases_once_whichever_reference_goes_last() {
    for order in ["picture-first", "glx-first", "disconnect"] {
        joint_render_and_glx_backing(order);
    }
}
