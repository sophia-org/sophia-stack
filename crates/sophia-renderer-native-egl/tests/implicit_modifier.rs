#![cfg(all(feature = "gbm-platform", target_os = "linux"))]

use std::{
    fs::{File, OpenOptions},
    os::fd::AsFd,
};

use sophia_renderer_native_egl::{
    NativeDmaBufFrame, NativeDmaBufPlane, NativeGbmRenderedScanoutContext,
    NativeMultiPlaneDmaBufFrame, NativePixmapImportProbe, native_dmabuf_cpu_write_access,
};

const IMPLICIT_MODIFIER: u64 = 0x00ff_ffff_ffff_ffff;

#[test]
fn implicit_modifier_matches_the_drm_abi_and_is_not_all_bits_set() {
    assert_eq!(u64::from(gbm::Modifier::Invalid), IMPLICIT_MODIFIER);
    assert_ne!(IMPLICIT_MODIFIER, u64::MAX);
}

#[test]
fn single_plane_descriptor_accepts_the_abi_implicit_modifier() {
    let file = File::open("/dev/null").unwrap();
    let frame = NativeDmaBufFrame {
        width: 2,
        height: 1,
        format: gbm::Format::Argb8888 as u32,
        modifier: IMPLICIT_MODIFIER,
        fd: file.as_fd(),
        offset: 0,
        stride: 8,
    };
    assert!(frame.is_valid());
    assert!(
        NativeDmaBufFrame {
            modifier: 0,
            ..frame
        }
        .is_valid()
    );
    assert!(
        !NativeDmaBufFrame {
            modifier: u64::MAX,
            ..frame
        }
        .is_valid()
    );
    assert!(!NativeDmaBufFrame { stride: 7, ..frame }.is_valid());
}

#[test]
#[ignore = "requires SOPHIA_TEST_RENDER_NODE; real DMA-BUF import and readback without KMS or windows"]
fn implicit_dma_buf_import_preserves_pixels_through_native_rendering() {
    let path = std::env::var_os("SOPHIA_TEST_RENDER_NODE").expect("select a render node");
    let open = || {
        OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .unwrap()
    };
    let allocator = gbm::Device::new(open()).unwrap();
    let mut source = allocator
        .create_buffer_object_with_modifiers2::<()>(
            2,
            1,
            gbm::Format::Argb8888,
            std::iter::once(gbm::Modifier::Linear),
            gbm::BufferObjectFlags::RENDERING,
        )
        .unwrap();
    assert_eq!(source.plane_count(), 1);
    let fd = source.fd_for_plane(0).unwrap();
    native_dmabuf_cpu_write_access(&fd, false).unwrap();
    let written = source.map_mut(0, 0, 2, 1, |mapped| {
        mapped.buffer_mut()[..8].copy_from_slice(&[0x21, 0x43, 0x65, 0xff, 0xab, 0xcd, 0xef, 0xff]);
    });
    let ended = native_dmabuf_cpu_write_access(&fd, true);
    written.unwrap();
    ended.unwrap();
    let expected = [0x65, 0x43, 0x21, 0xff, 0xef, 0xcd, 0xab, 0xff];
    let input = NativeMultiPlaneDmaBufFrame {
        width: 2,
        height: 1,
        format: source.format() as u32,
        modifier: IMPLICIT_MODIFIER,
        plane_count: 1,
        planes: [
            Some(NativeDmaBufPlane {
                fd: fd.as_fd(),
                offset: source.offset(0),
                stride: source.stride_for_plane(0),
            }),
            None,
            None,
            None,
        ],
    };
    let probe = NativePixmapImportProbe::new(open(), input).unwrap();
    assert_eq!(probe.read_rgba().unwrap(), expected);
    drop(probe);

    let initialized = NativeGbmRenderedScanoutContext::from_backend_device_result(Ok(open()));
    let mut renderer = initialized
        .context
        .unwrap_or_else(|| panic!("{:?}", initialized.status));
    let rendered = renderer.export_dmabuf_owned_scanout_buffer_with_modifiers(
        NativeDmaBufFrame {
            width: 2,
            height: 1,
            format: source.format() as u32,
            modifier: IMPLICIT_MODIFIER,
            fd: fd.as_fd(),
            offset: source.offset(0),
            stride: source.stride_for_plane(0),
        },
        &[u64::MAX, IMPLICIT_MODIFIER, 0],
    );
    let output = rendered
        .buffer
        .unwrap_or_else(|| panic!("{:?}", rendered.detail));
    let fds = output.export_plane_fds().unwrap().into_plane_fds();
    let offsets = output.plane_offsets();
    let strides = output.plane_pitches();
    let probe = NativePixmapImportProbe::new(
        open(),
        NativeMultiPlaneDmaBufFrame {
            width: 2,
            height: 1,
            format: output.format(),
            modifier: output.modifier().unwrap_or(IMPLICIT_MODIFIER),
            plane_count: output.plane_count(),
            planes: std::array::from_fn(|index| {
                fds[index].as_ref().map(|fd| NativeDmaBufPlane {
                    fd: fd.as_fd(),
                    offset: offsets[index],
                    stride: strides[index],
                })
            }),
        },
    )
    .unwrap();
    assert_eq!(probe.read_rgba().unwrap(), expected);
}
