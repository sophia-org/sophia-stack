#![cfg(test)]

use crate::prelude::*;
use std::{
    cell::{Cell, RefCell},
    os::fd::BorrowedFd,
};

pub(crate) const DCC: u64 = 0x0200_0000_28a6_bf04;
pub(crate) const SIZE: Size = Size {
    width: 1920,
    height: 1080,
};

pub(crate) fn property(raw: u32) -> drm::control::property::Handle {
    drm::control::from_u32(raw).unwrap()
}

pub(crate) fn descriptor(modifier: u64) -> LiveRendererScanoutBufferDescriptor {
    let (count, pitches, offsets) = if modifier == 0 {
        (1, [7680, 0, 0, 0], [0; 4])
    } else {
        (2, [8192, 2048, 0, 0], [0, 10485760, 0, 0])
    };
    LiveRendererScanoutBufferDescriptor::for_imported_dma_buf_planes(
        SIZE,
        LIVE_RENDERER_SCANOUT_FORMAT_XRGB8888,
        count,
        pitches,
        offsets,
        Some(modifier),
    )
}

pub(crate) struct Device {
    pub(crate) rejected_descriptor: Cell<LiveRendererScanoutBufferDescriptor>,
    pub(crate) errno: Cell<Option<i32>>,
    pub(crate) error_kind: Cell<Option<io::ErrorKind>>,
    pub(crate) import_failure: Cell<Option<usize>>,
    pub(crate) close_failure: Cell<bool>,
    pub(crate) imports: Cell<usize>,
    pub(crate) addfb_calls: Cell<usize>,
    pub(crate) test_errno: Cell<Option<i32>>,
    pub(crate) tests: RefCell<Vec<(drm::control::AtomicCommitFlags, String)>>,
    pub(crate) closed: RefCell<Vec<u32>>,
    pub(crate) destroyed: RefCell<Vec<u32>>,
}

impl Device {
    pub(crate) fn new() -> Self {
        Self {
            rejected_descriptor: Cell::new(descriptor(DCC)),
            errno: Cell::new(Some(22)),
            error_kind: Cell::new(None),
            import_failure: Cell::new(None),
            close_failure: Cell::new(false),
            imports: Cell::new(0),
            addfb_calls: Cell::new(0),
            test_errno: Cell::new(None),
            tests: RefCell::new(Vec::new()),
            closed: RefCell::new(Vec::new()),
            destroyed: RefCell::new(Vec::new()),
        }
    }
}

impl LibdrmNativePropertyLookupDevice for Device {
    fn connector_property_handles(
        &self,
        _: drm::control::connector::Handle,
    ) -> io::Result<LibdrmNativePropertyHandleSet> {
        Ok(LibdrmNativePropertyHandleSet::new([(
            "CRTC_ID",
            property(101),
        )]))
    }
    fn crtc_property_handles(
        &self,
        _: drm::control::crtc::Handle,
    ) -> io::Result<LibdrmNativePropertyHandleSet> {
        Ok(LibdrmNativePropertyHandleSet::new([
            ("MODE_ID", property(102)),
            ("ACTIVE", property(103)),
            ("VRR_ENABLED", property(115)),
        ]))
    }
    fn plane_property_handles(
        &self,
        _: drm::control::plane::Handle,
    ) -> io::Result<LibdrmNativePropertyHandleSet> {
        Ok(LibdrmNativePropertyHandleSet::new([
            ("FB_ID", property(104)),
            ("CRTC_ID", property(105)),
            ("SRC_X", property(106)),
            ("SRC_Y", property(107)),
            ("SRC_W", property(108)),
            ("SRC_H", property(109)),
            ("CRTC_X", property(110)),
            ("CRTC_Y", property(111)),
            ("CRTC_W", property(112)),
            ("CRTC_H", property(113)),
            ("IN_FORMATS", property(114)),
        ]))
    }
}

impl LibdrmNativePrimaryPlaneResourceDevice for Device {
    fn create_mode_blob_for_selection(
        &self,
        _: LibdrmNativePrimaryPlaneSelection,
    ) -> io::Result<u64> {
        Ok(300)
    }
    fn create_mode_blob(&self, _: drm::control::Mode) -> io::Result<u64> {
        Ok(300)
    }
    fn destroy_mode_blob(&self, _: u64) -> io::Result<()> {
        Ok(())
    }
    fn add_scanout_framebuffer_with_modifiers<B: drm::buffer::PlanarBuffer + ?Sized>(
        &self,
        buffer: &B,
    ) -> io::Result<drm::control::framebuffer::Handle> {
        self.addfb_calls.set(self.addfb_calls.get() + 1);
        let rejected = self.rejected_descriptor.get();
        if buffer.modifier().map(u64::from) == rejected.modifier {
            assert_eq!(buffer.pitches(), rejected.plane_pitches);
            assert_eq!(buffer.offsets(), rejected.plane_offsets);
            assert!(
                buffer.handles()[..rejected.plane_count as usize]
                    .iter()
                    .all(Option::is_some)
            );
            if let Some(kind) = self.error_kind.get() {
                return Err(io::Error::from(kind));
            }
            if let Some(errno) = self.errno.get() {
                return Err(io::Error::from_raw_os_error(errno));
            }
        }
        Ok(drm::control::from_u32(100).unwrap())
    }
    fn add_scanout_framebuffer_without_modifiers<B: drm::buffer::PlanarBuffer + ?Sized>(
        &self,
        _: &B,
    ) -> io::Result<drm::control::framebuffer::Handle> {
        panic!("explicit layout must not become implicit")
    }
    fn add_legacy_scanout_framebuffer<B: drm::buffer::Buffer + ?Sized>(
        &self,
        _: &B,
        _: u32,
        _: u32,
    ) -> io::Result<drm::control::framebuffer::Handle> {
        panic!("explicit layout must not become legacy")
    }
    fn import_scanout_dma_buf(&self, _: BorrowedFd<'_>) -> io::Result<drm::buffer::Handle> {
        let call = self.imports.get() + 1;
        self.imports.set(call);
        if self.import_failure.get() == Some(call) {
            return Err(io::Error::from_raw_os_error(22));
        }
        // The two DCC planes share one imported GEM handle, as in the physical probe.
        Ok(drm::control::from_u32(if call <= 2 { 200 } else { 201 }).unwrap())
    }
    fn close_scanout_buffer(&self, handle: drm::buffer::Handle) -> io::Result<()> {
        if self.close_failure.get() {
            return Err(io::Error::from_raw_os_error(16));
        }
        self.closed.borrow_mut().push(handle.into());
        Ok(())
    }
    fn destroy_scanout_framebuffer(
        &self,
        framebuffer: drm::control::framebuffer::Handle,
    ) -> io::Result<()> {
        self.destroyed.borrow_mut().push(framebuffer.into());
        Ok(())
    }
}

impl LibdrmNativeAtomicCommitDevice for Device {
    fn submit_atomic_commit(
        &self,
        flags: drm::control::AtomicCommitFlags,
        request: drm::control::atomic::AtomicModeReq,
    ) -> io::Result<()> {
        self.tests
            .borrow_mut()
            .push((flags, format!("{request:?}")));
        self.test_errno
            .get()
            .map_or(Ok(()), |errno| Err(io::Error::from_raw_os_error(errno)))
    }
}
