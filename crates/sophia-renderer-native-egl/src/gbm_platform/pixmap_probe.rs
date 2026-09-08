use std::{ffi::c_void, fs::File, ptr};

use gbm::AsRaw;
use glow::HasContext;

use super::{
    NativeMultiPlaneDmaBufFrame, config::window_config_attributes, scanout::create_dma_buf_image,
};

/// A private GL consumer used to verify that a retained export remains coherent.
/// It never creates a window or acquires a scanout resource.
pub struct NativePixmapImportProbe {
    egl: khronos_egl::DynamicInstance<khronos_egl::EGL1_5>,
    display: khronos_egl::Display,
    context: khronos_egl::Context,
    image: Option<khronos_egl::Image>,
    gl: glow::Context,
    texture: Option<glow::Texture>,
    framebuffer: Option<glow::Framebuffer>,
    width: i32,
    height: i32,
    _device: gbm::Device<File>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativePixmapImportProbeError {
    InvalidDescriptor,
    Platform,
    Context,
    Import,
    Readback,
}

impl NativePixmapImportProbe {
    pub fn new(
        device: File,
        frame: NativeMultiPlaneDmaBufFrame<'_>,
    ) -> Result<Self, NativePixmapImportProbeError> {
        use NativePixmapImportProbeError as E;
        if !frame.is_valid() || frame.width > 64 || frame.height > 64 {
            return Err(E::InvalidDescriptor);
        }
        let device = gbm::Device::new(device).map_err(|_| E::Platform)?;
        // EGL owns no native display; the GBM device is retained until termination.
        let egl = unsafe { khronos_egl::DynamicInstance::<khronos_egl::EGL1_5>::load_required() }
            .map_err(|_| E::Platform)?;
        let display = unsafe {
            egl.get_platform_display(
                super::EGL_PLATFORM_GBM_KHR,
                device.as_raw() as khronos_egl::NativeDisplayType,
                &[khronos_egl::ATTRIB_NONE],
            )
        }
        .map_err(|_| E::Platform)?;
        egl.initialize(display).map_err(|_| E::Platform)?;
        let prepared = (|| {
            egl.bind_api(khronos_egl::OPENGL_API)
                .map_err(|_| E::Context)?;
            let config = egl
                .choose_first_config(display, &window_config_attributes())
                .map_err(|_| E::Context)?
                .ok_or(E::Context)?;
            egl.create_context(display, config, None, &crate::gl::context_attributes())
                .map_err(|_| E::Context)
        })();
        let context = match prepared {
            Ok(context) => context,
            Err(error) => {
                let _ = egl.terminate(display);
                return Err(error);
            }
        };
        if egl
            .make_current(display, None, None, Some(context))
            .is_err()
        {
            let _ = egl.destroy_context(display, context);
            let _ = egl.terminate(display);
            return Err(E::Context);
        }
        // Entry points come from the current EGL context and stay loaded with it.
        let gl = unsafe {
            glow::Context::from_loader_function(|name| {
                egl.get_proc_address(name)
                    .map_or(ptr::null(), |function| function as *const c_void)
            })
        };
        let mut probe = Self {
            egl,
            display,
            context,
            gl,
            image: None,
            texture: None,
            framebuffer: None,
            width: frame.width as i32,
            height: frame.height as i32,
            _device: device,
        };
        probe.attach(frame)?;
        Ok(probe)
    }

    fn attach(
        &mut self,
        frame: NativeMultiPlaneDmaBufFrame<'_>,
    ) -> Result<(), NativePixmapImportProbeError> {
        use NativePixmapImportProbeError as E;
        let image = create_dma_buf_image(&self.egl, self.display, frame).map_err(|_| E::Import)?;
        self.image = Some(image);
        let entry = self
            .egl
            .get_proc_address("glEGLImageTargetTexture2DOES")
            .ok_or(E::Import)?;
        // The extension entry point binds the EGL image retained by this probe.
        unsafe {
            let image_target: unsafe extern "system" fn(u32, *const c_void) =
                std::mem::transmute(entry);
            let texture = self.gl.create_texture().map_err(|_| E::Import)?;
            self.texture = Some(texture);
            self.gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            image_target(glow::TEXTURE_2D, image.as_ptr());
            let framebuffer = self.gl.create_framebuffer().map_err(|_| E::Import)?;
            self.framebuffer = Some(framebuffer);
            self.gl
                .bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer));
            self.gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                Some(texture),
                0,
            );
            if self.gl.check_framebuffer_status(glow::FRAMEBUFFER) != glow::FRAMEBUFFER_COMPLETE
                || self.gl.get_error() != glow::NO_ERROR
            {
                return Err(E::Import);
            }
        }
        Ok(())
    }

    /// Rebind the retained image before reading, as texture-from-pixmap requires
    /// after the producer changes the drawable.
    pub fn read_rgba(&self) -> Result<Vec<u8>, NativePixmapImportProbeError> {
        use NativePixmapImportProbeError as E;
        self.egl
            .make_current(self.display, None, None, Some(self.context))
            .map_err(|_| E::Context)?;
        let mut bytes = vec![0; self.width as usize * self.height as usize * 4];
        // Dimensions are bounded at construction; the slice holds every RGBA pixel.
        unsafe {
            let entry = self
                .egl
                .get_proc_address("glEGLImageTargetTexture2DOES")
                .ok_or(E::Import)?;
            let image_target: unsafe extern "system" fn(u32, *const c_void) =
                std::mem::transmute(entry);
            self.gl.bind_texture(glow::TEXTURE_2D, self.texture);
            image_target(glow::TEXTURE_2D, self.image.ok_or(E::Import)?.as_ptr());
            self.gl
                .bind_framebuffer(glow::FRAMEBUFFER, self.framebuffer);
            self.gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                self.texture,
                0,
            );
            self.gl.read_pixels(
                0,
                0,
                self.width,
                self.height,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelPackData::Slice(Some(&mut bytes)),
            );
            self.gl.finish();
            if self.gl.get_error() != glow::NO_ERROR {
                return Err(E::Readback);
            }
        }
        Ok(bytes)
    }
}

impl Drop for NativePixmapImportProbe {
    fn drop(&mut self) {
        if self
            .egl
            .make_current(self.display, None, None, Some(self.context))
            .is_ok()
        {
            // These names were allocated in this context and are deleted once.
            unsafe {
                if let Some(framebuffer) = self.framebuffer.take() {
                    self.gl.delete_framebuffer(framebuffer);
                }
                if let Some(texture) = self.texture.take() {
                    self.gl.delete_texture(texture);
                }
            }
        }
        if let Some(image) = self.image.take() {
            let _ = self.egl.destroy_image(self.display, image);
        }
        let _ = self.egl.make_current(self.display, None, None, None);
        let _ = self.egl.destroy_context(self.display, self.context);
        let _ = self.egl.terminate(self.display);
    }
}
