#![cfg(feature = "gbm-probe")]

use std::{fs::OpenOptions, os::fd::AsFd};

use sophia_protocol::{BufferHandle, DRM_FORMAT_ARGB8888, Rect, Size};
use sophia_renderer_live::{
    LiveSharedBufferAllocation, LiveSharedPixmapError as Error, LiveSharedPixmapPatch as Patch,
    LiveSharedPixmapService, LiveSharedPixmapStore, LiveSharedPixmapUpdate as Update,
};
use sophia_renderer_native_egl::{
    NativeDmaBufPlane, NativeMultiPlaneDmaBufFrame, NativePixmapImportProbe,
};

const SIZE: Size = Size {
    width: 2,
    height: 1,
};

fn patch(x: i32, bytes: Vec<u8>) -> Patch {
    Patch {
        rect: Rect {
            x,
            y: 0,
            width: 1,
            height: 1,
        },
        bytes,
    }
}

fn update(handle: BufferHandle, revision: u64, format: u32, patches: Vec<Patch>) -> Update {
    Update {
        handle,
        revision,
        size: SIZE,
        format,
        patches,
    }
}

#[test]
fn malformed_patches_are_rejected_before_any_upload() {
    let handle = BufferHandle::from_raw(7);
    for (x, bytes) in [
        (-1, vec![0; 4]),
        (2, vec![0; 4]),
        (0, vec![0; 3]),
        (i32::MAX, vec![0; 4]),
    ] {
        assert_eq!(
            update(handle, 1, DRM_FORMAT_ARGB8888, vec![patch(x, bytes)]).validate(),
            Err(Error::InvalidTarget)
        );
    }
    let too_many = (0..33).map(|_| patch(0, vec![0; 4])).collect();
    assert_eq!(
        update(handle, 1, DRM_FORMAT_ARGB8888, too_many).validate(),
        Err(Error::Capacity)
    );
    assert_eq!(
        update(handle, 0, DRM_FORMAT_ARGB8888, vec![]).validate(),
        Err(Error::InvalidTarget)
    );
    assert_eq!(
        update(handle, 1, 0, vec![]).validate(),
        Err(Error::InvalidTarget)
    );
    assert_eq!(
        update(BufferHandle::from_raw(0), 1, DRM_FORMAT_ARGB8888, vec![]).validate(),
        Err(Error::InvalidTarget)
    );
}

fn device() -> std::fs::File {
    let path = std::env::var_os("SOPHIA_PIXMAP_TEST_DEVICE")
        .expect("set SOPHIA_PIXMAP_TEST_DEVICE to a DRM render node");
    OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .expect("open test render node")
}

fn consumer(allocation: &LiveSharedBufferAllocation) -> NativePixmapImportProbe {
    let descriptor = allocation.descriptor;
    NativePixmapImportProbe::new(
        device(),
        NativeMultiPlaneDmaBufFrame {
            width: descriptor.size.width as u32,
            height: descriptor.size.height as u32,
            format: descriptor.format,
            modifier: descriptor.modifier,
            plane_count: descriptor.plane_count,
            planes: std::array::from_fn(|index| {
                descriptor.planes[index].map(|plane| NativeDmaBufPlane {
                    fd: allocation.plane_fds[index].as_fd(),
                    offset: plane.offset,
                    stride: plane.stride,
                })
            }),
        },
    )
    .expect("import export into independent GL texture")
}

#[test]
#[ignore = "requires an explicitly selected DRM render node; creates no windows"]
fn retained_gl_texture_observes_dirty_updates_and_outlives_the_provider() {
    for depth in [24, 32] {
        let mut store = LiveSharedPixmapStore::new(device()).unwrap();
        let handle = BufferHandle::from_raw(7);
        let allocation = store.allocate(handle, SIZE, depth).unwrap();
        let format = allocation.descriptor.format;
        let imported = consumer(&allocation);
        let alpha = if depth == 32 { 0 } else { 255 };
        assert_eq!(imported.read_rgba().unwrap(), [0, 0, 0, alpha].repeat(2));
        assert_eq!(store.resident_count(), 1);
        let resident_bytes = store.resident_bytes();
        assert!(resident_bytes >= 8);
        assert_eq!(
            store.allocate(handle, SIZE, depth).unwrap_err(),
            Error::IdentityInUse
        );
        assert_eq!(store.resident_bytes(), resident_bytes);

        store
            .update(update(
                handle,
                1,
                format,
                vec![patch(0, vec![0x21, 0x43, 0x65, 0x87])],
            ))
            .unwrap();
        let alpha = if depth == 32 { 0x87 } else { 255 };
        assert_eq!(
            imported.read_rgba().unwrap(),
            [
                0x65,
                0x43,
                0x21,
                alpha,
                0,
                0,
                0,
                if depth == 32 { 0 } else { 255 }
            ]
        );
        store
            .update(update(
                handle,
                2,
                format,
                vec![patch(1, vec![0xab, 0xcd, 0xef, 0xff])],
            ))
            .unwrap();
        let expected = vec![0x65, 0x43, 0x21, alpha, 0xef, 0xcd, 0xab, 0xff];
        assert_eq!(imported.read_rgba().unwrap(), expected);
        // A duplicate must not replay bytes over the later published revision.
        store
            .update(update(handle, 1, format, vec![patch(0, vec![0; 4])]))
            .unwrap();
        assert_eq!(imported.read_rgba().unwrap(), expected);
        let mut wrong = update(handle, 3, format, vec![patch(0, vec![0; 4])]);
        wrong.size.width = 1;
        assert_eq!(store.update(wrong), Err(Error::InvalidTarget));
        assert_eq!(imported.read_rgba().unwrap(), expected);
        assert!(store.release(handle));
        assert!(!store.release(handle));
        assert_eq!((store.resident_count(), store.resident_bytes()), (0, 0));
        assert_eq!(
            store.update(update(handle, 3, format, vec![])),
            Err(Error::UnknownBacking)
        );
        drop(allocation);
        drop(store);
        assert_eq!(imported.read_rgba().unwrap(), expected);
    }
}

#[test]
#[ignore = "requires an explicitly selected DRM render node; creates no windows"]
fn renderer_worker_probes_coherence_and_releases_allocations() {
    let service = LiveSharedPixmapService::new(device()).expect("coherence capability probe");
    let handle = BufferHandle::from_raw(73);
    let allocation = service.allocate(handle, SIZE, 32).unwrap();
    let imported = consumer(&allocation);
    service
        .update(update(
            handle,
            1,
            allocation.descriptor.format,
            vec![patch(0, vec![0x21, 0x43, 0x65, 0xff])],
        ))
        .unwrap();
    assert_eq!(
        imported.read_rgba().unwrap(),
        [0x65, 0x43, 0x21, 0xff, 0, 0, 0, 0]
    );
    service.release(handle).unwrap();
    service.release(handle).unwrap();
    assert_eq!(
        service.update(update(handle, 2, allocation.descriptor.format, vec![])),
        Err(Error::UnknownBacking)
    );
}
