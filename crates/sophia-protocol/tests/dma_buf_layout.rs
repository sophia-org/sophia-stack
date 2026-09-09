use sophia_protocol::{
    BufferHandle, DMA_BUF_MAX_BYTES, DMA_BUF_MAX_DIMENSION, DRM_FORMAT_MOD_INVALID,
    DRM_FORMAT_XRGB8888, DmaBufDescriptor, DmaBufDescriptorError, DmaBufPlaneDescriptor, Size,
};

#[test]
fn implicit_modifier_uses_the_drm_reserved_value_without_a_vendor_byte() {
    // drm_fourcc.h: fourcc_mod_code(NONE, (1ULL << 56) - 1).
    assert_eq!(DRM_FORMAT_MOD_INVALID, 0x00ff_ffff_ffff_ffff);
}

fn compressed_image() -> DmaBufDescriptor {
    // Mesa's first 300x300 GLX buffer: color and compression metadata share
    // one allocation. The metadata pitch is smaller than an RGB image row.
    DmaBufDescriptor {
        handle: BufferHandle::from_raw(1),
        size: Size {
            width: 300,
            height: 300,
        },
        format: DRM_FORMAT_XRGB8888,
        modifier: 144_115_188_757_872_388,
        plane_count: 2,
        planes: [
            Some(DmaBufPlaneDescriptor {
                offset: 0,
                stride: 2048,
            }),
            Some(DmaBufPlaneDescriptor {
                offset: 1_048_576,
                stride: 1024,
            }),
            None,
            None,
        ],
    }
}

#[test]
fn explicit_layouts_do_not_give_auxiliary_planes_image_geometry() {
    let image = compressed_image();
    assert!(image.planes[1].unwrap().stride < image.size.width as u32 * 4);
    assert_eq!(image.validate(), Ok(()));

    // Metadata geometry is opaque, even near the descriptor's byte budget.
    // Only the importer can determine its height from the modifier.
    let mut sparse = image;
    sparse.planes[1] = Some(DmaBufPlaneDescriptor {
        offset: DMA_BUF_MAX_BYTES as u32 - 64,
        stride: 64,
    });
    assert_eq!(sparse.validate(), Ok(()));
}

#[test]
fn linear_and_legacy_implicit_row_bounds_remain_enforced() {
    for modifier in [0, 0x00ff_ffff_ffff_ffff] {
        let image = DmaBufDescriptor {
            modifier,
            plane_count: 1,
            planes: [
                Some(DmaBufPlaneDescriptor {
                    offset: 0,
                    stride: 1200,
                }),
                None,
                None,
                None,
            ],
            ..compressed_image()
        };
        assert_eq!(image.validate(), Ok(()));

        let mut short_row = image;
        short_row.planes[0].as_mut().unwrap().stride = 1199;
        assert_eq!(
            short_row.validate(),
            Err(DmaBufDescriptorError::InvalidStride)
        );

        let mut over_budget = image;
        over_budget.planes[0].as_mut().unwrap().offset = DMA_BUF_MAX_BYTES as u32 - 1200;
        assert_eq!(
            over_budget.validate(),
            Err(DmaBufDescriptorError::BufferTooLarge)
        );
    }
}

#[test]
fn opaque_layouts_keep_structural_and_resource_bounds() {
    let valid = compressed_image();
    let mut cases = Vec::new();
    let mut add = |expected, change: fn(&mut DmaBufDescriptor)| {
        let mut descriptor = valid;
        change(&mut descriptor);
        cases.push((descriptor, expected));
    };
    add(DmaBufDescriptorError::InvalidHandle, |d| {
        d.handle = BufferHandle::INVALID
    });
    add(DmaBufDescriptorError::InvalidSize, |d| d.size.width = 0);
    add(DmaBufDescriptorError::InvalidSize, |d| {
        d.size.height = DMA_BUF_MAX_DIMENSION + 1
    });
    add(DmaBufDescriptorError::UnsupportedFormat, |d| d.format = 0);
    add(DmaBufDescriptorError::InvalidPlaneCount, |d| {
        d.plane_count = 0
    });
    add(DmaBufDescriptorError::InvalidPlaneCount, |d| {
        d.plane_count = 5
    });
    add(DmaBufDescriptorError::MissingPlane, |d| d.planes[1] = None);
    add(DmaBufDescriptorError::UnexpectedPlane, |d| {
        d.plane_count = 1
    });
    add(DmaBufDescriptorError::InvalidStride, |d| {
        d.planes[1].as_mut().unwrap().stride = 0
    });
    add(DmaBufDescriptorError::BufferTooLarge, |d| {
        d.planes[1].as_mut().unwrap().stride = DMA_BUF_MAX_BYTES as u32 + 1;
    });
    add(DmaBufDescriptorError::BufferTooLarge, |d| {
        d.planes[1].as_mut().unwrap().offset = DMA_BUF_MAX_BYTES as u32;
    });
    add(DmaBufDescriptorError::BufferTooLarge, |d| {
        d.size = Size {
            width: DMA_BUF_MAX_DIMENSION,
            height: DMA_BUF_MAX_DIMENSION,
        };
    });
    for (descriptor, expected) in cases {
        assert_eq!(descriptor.validate(), Err(expected), "{descriptor:?}");
    }
}
