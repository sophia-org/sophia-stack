const NATIVE_IMAGE_BRIDGE_CAPACITY: usize = 3;

struct NativeImageBridgeSlot {
    source: usize,
    bridge: NativeLinearImageBridge,
    completion: Option<khronos_egl::Sync>,
}

// These BOs are internal scratch, never returned as retained renderer images.
// Their displays belong to the source inventory and outlive the entire pool.
struct NativeLinearImageBridge {
    egl: khronos_egl::DynamicInstance<khronos_egl::EGL1_5>,
    display: khronos_egl::Display,
    context: khronos_egl::Context,
    gl: glow::Context,
    pipeline: Option<PersistentXrgb8888GlPipeline>,
    framebuffer: Option<glow::Framebuffer>,
    texture: Option<glow::Texture>,
    image: Option<khronos_egl::Image>,
    buffer: NativeGbmOwnedScanoutBuffer,
    bytes: u64,
}

impl NativeLinearImageBridge {
    fn new<T: AsFd>(
        owner: &NativeGbmRenderedScanoutContext<T>,
        source: NativeMultiPlaneDmaBufFrame<'_>,
    ) -> Result<Self, NativeGbmScanoutBufferExportDetail> {
        use NativeGbmScanoutBufferExportDetail as E;
        use glow::HasContext;
        let format = match source.format {
            0x3432_5258 => gbm::Format::Xrgb8888,
            0x3432_5241 => gbm::Format::Argb8888,
            _ => return Err(E::InvalidTarget),
        };
        let bo = owner
            .gbm_device
            .create_buffer_object_with_modifiers2::<()>(
                source.width,
                source.height,
                format,
                std::iter::once(gbm::Modifier::Linear),
                gbm::BufferObjectFlags::RENDERING,
            )
            .map_err(|_| E::GbmSurfaceUnavailable)?;
        if bo.modifier() != gbm::Modifier::Linear || bo.plane_count() != 1 {
            return Err(E::InvalidBufferDescriptor);
        }
        let buffer = native_owned_scanout_buffer_from_bo(source.width, source.height, bo, None)?;
        let fd = buffer.export_plane_fds()?.into_plane_fds()[0]
            .take()
            .ok_or(E::InvalidBufferDescriptor)?;
        let bytes = u64::try_from(
            rustix::fs::fstat(&fd)
                .map_err(|_| E::InvalidBufferDescriptor)?
                .st_size,
        )
        .map_err(|_| E::InvalidBufferDescriptor)?;
        if bytes < u64::from(buffer.pitch()) * u64::from(buffer.height()) {
            return Err(E::InvalidBufferDescriptor);
        }
        // Keep entry points loaded while the source display and scratch context live.
        let egl = unsafe { khronos_egl::DynamicInstance::<khronos_egl::EGL1_5>::load_required() }
            .map_err(|_| E::EglUnavailable)?;
        let display = owner.display;
        egl.bind_api(khronos_egl::OPENGL_API)
            .map_err(|_| E::EglBindApiFailed)?;
        let config = choose_scanout_config_for_format(
            &egl,
            display,
            if format == gbm::Format::Argb8888 {
                window_config_attributes()
            } else {
                xrgb_window_config_attributes()
            },
            format,
        )
        .ok_or(E::EglConfigUnavailable)?;
        let context = egl
            .create_context(display, config, None, &context_attributes())
            .map_err(|_| E::EglContextUnavailable)?;
        if egl
            .make_current(display, None, None, Some(context))
            .is_err()
        {
            let _ = egl.destroy_context(display, context);
            return Err(E::EglMakeCurrentFailed);
        }
        let loader = |name: &str| {
            egl.get_proc_address(name)
                .map_or(ptr::null(), |proc| proc as *const c_void)
        };
        let gl = unsafe { glow::Context::from_loader_function(loader) };
        let pipeline_gl = unsafe { glow::Context::from_loader_function(loader) };
        let mut bridge = Self {
            egl,
            display,
            context,
            gl,
            pipeline: None,
            framebuffer: None,
            texture: None,
            image: None,
            buffer,
            bytes,
        };
        bridge.pipeline = Some(
            unsafe {
                PersistentXrgb8888GlPipeline::new_image_target(
                    pipeline_gl,
                    source.width,
                    source.height,
                )
            }
            .map_err(|_| E::GlSmokeFailed)?,
        );
        let frame = NativeMultiPlaneDmaBufFrame {
            width: source.width,
            height: source.height,
            format: source.format,
            modifier: 0,
            plane_count: 1,
            planes: [
                Some(NativeDmaBufPlane {
                    fd: fd.as_fd(),
                    offset: 0,
                    stride: bridge.buffer.pitch(),
                }),
                None,
                None,
                None,
            ],
        };
        let image = create_dma_buf_image(&bridge.egl, display, frame)?;
        bridge.image = Some(image);
        let texture = unsafe {
            bridge
                .pipeline
                .as_ref()
                .unwrap()
                .create_egl_image_texture(&bridge.egl, image.as_ptr())
        }?;
        bridge.texture = Some(texture);
        unsafe {
            let framebuffer = bridge
                .gl
                .create_framebuffer()
                .map_err(|_| E::CompositionDrawFailed)?;
            bridge.framebuffer = Some(framebuffer);
            bridge
                .gl
                .bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer));
            bridge.gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                Some(texture),
                0,
            );
            if bridge.gl.check_framebuffer_status(glow::FRAMEBUFFER) != glow::FRAMEBUFFER_COMPLETE {
                return Err(E::CompositionDrawFailed);
            }
        }
        let _ = bridge.egl.make_current(display, None, None, None);
        Ok(bridge)
    }

    fn draw(
        &self,
        source: NativeMultiPlaneDmaBufFrame<'_>,
    ) -> Result<(), NativeGbmScanoutBufferExportDetail> {
        use NativeGbmScanoutBufferExportDetail as E;
        use glow::HasContext;
        self.egl
            .make_current(self.display, None, None, Some(self.context))
            .map_err(|_| E::EglMakeCurrentFailed)?;
        let result = (|| {
            let image = create_dma_buf_image(&self.egl, self.display, source)?;
            let pipeline = self.pipeline.as_ref().ok_or(E::GlSmokeFailed)?;
            let texture = unsafe { pipeline.create_egl_image_texture(&self.egl, image.as_ptr()) };
            let result = match texture {
                Ok(texture) => {
                    unsafe {
                        self.gl
                            .bind_framebuffer(glow::FRAMEBUFFER, self.framebuffer);
                    }
                    pipeline.begin_composition_with_clear_alpha(0.0);
                    let drawn = pipeline
                        .draw_texture_layer(
                            texture,
                            (source.width, source.height),
                            GlCompositionRect {
                                x: 0,
                                y: 0,
                                width: source.width as i32,
                                height: source.height as i32,
                            },
                            None,
                            1.0,
                            if source.format == 0x3432_5241 {
                                crate::NativeCompositionAlphaMode::Premultiplied
                            } else {
                                crate::NativeCompositionAlphaMode::Opaque
                            },
                            crate::NativeCompositionSampling::ExactNearest,
                            None,
                        )
                        .and_then(|()| pipeline.validate_composition())
                        .map_err(|_| E::CompositionDrawFailed);
                    unsafe {
                        self.gl.flush();
                        pipeline.delete_texture(texture);
                    }
                    drawn
                }
                Err(error) => Err(error),
            };
            let destroyed = self
                .egl
                .destroy_image(self.display, image)
                .map_err(|_| E::EglImageDestroyFailed);
            result.and(destroyed)
        })();
        let _ = self.egl.make_current(self.display, None, None, None);
        result
    }
}

impl Drop for NativeLinearImageBridge {
    fn drop(&mut self) {
        use glow::HasContext;
        if self
            .egl
            .make_current(self.display, None, None, Some(self.context))
            .is_ok()
        {
            unsafe {
                if let Some(framebuffer) = self.framebuffer.take() {
                    self.gl.delete_framebuffer(framebuffer);
                }
                if let Some(texture) = self.texture.take() {
                    self.gl.delete_texture(texture);
                }
            }
        }
        self.pipeline.take();
        if let Some(image) = self.image.take() {
            let _ = self.egl.destroy_image(self.display, image);
        }
        let _ = self.egl.make_current(self.display, None, None, None);
        let _ = self.egl.destroy_context(self.display, self.context);
    }
}

impl<T: AsFd> NativeGbmRenderedScanoutContext<T> {
    fn poll_image_bridges(&mut self) -> Result<(), NativeGbmScanoutBufferExportDetail> {
        for slot in &mut self.image_bridges {
            if let Some(sync) = slot.completion {
                match unsafe { self.egl.client_wait_sync(self.display, sync, 0, 0) } {
                    Ok(khronos_egl::CONDITION_SATISFIED) => {
                        unsafe { self.egl.destroy_sync(self.display, sync) }.map_err(|_| {
                            NativeGbmScanoutBufferExportDetail::CompositionFinishFailed
                        })?;
                        slot.completion = None;
                    }
                    Ok(khronos_egl::TIMEOUT_EXPIRED) => {}
                    _ => return Err(NativeGbmScanoutBufferExportDetail::CompositionFinishFailed),
                }
            }
        }
        Ok(())
    }

    fn destroy_image_bridges(&mut self) {
        for mut slot in self.image_bridges.drain(..) {
            if let Some(sync) = slot.completion.take() {
                let _ = unsafe { self.egl.destroy_sync(self.display, sync) };
            }
            drop(slot);
        }
    }

    fn image_bridge_bytes(&self) -> u64 {
        self.image_bridges
            .iter()
            .map(|slot| slot.bridge.bytes)
            .sum()
    }
}
