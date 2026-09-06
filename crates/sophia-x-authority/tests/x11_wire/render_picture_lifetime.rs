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
