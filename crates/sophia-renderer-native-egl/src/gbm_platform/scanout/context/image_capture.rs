impl<T: std::os::fd::AsFd> NativeGbmRenderedScanoutContext<T> {
    fn render_renderer_image_snapshot(
        &mut self,
        image_id: NativeRendererImageId,
        source: NativeMultiPlaneDmaBufFrame<'_>,
        linear_bridge: bool,
    ) -> Result<NativeGbmOwnedScanoutBuffer, NativeGbmScanoutBufferExportDetail> {
        let format = match source.format {
            0x3432_5258 => gbm::Format::Xrgb8888,
            0x3432_5241 => gbm::Format::Argb8888,
            _ => return Err(NativeGbmScanoutBufferExportDetail::InvalidTarget),
        };
        self.egl
            .bind_api(khronos_egl::OPENGL_API)
            .map_err(|_| NativeGbmScanoutBufferExportDetail::EglBindApiFailed)?;
        let layer = NativeCompositionLayer::DmaBuf(NativeDmaBufCompositionLayer {
            image_id,
            frame: source,
            target: NativeCompositionRect {
                x: 0,
                y: 0,
                width: i32::try_from(source.width).unwrap_or(i32::MAX),
                height: i32::try_from(source.height).unwrap_or(i32::MAX),
            },
            clip: None,
            alpha: 1.0,
            sampling: crate::NativeCompositionSampling::ExactNearest,
        });
        let layers = [layer];
        let frame = NativeCompositionFrame {
            width: source.width,
            height: source.height,
            layers: &layers,
            trace: None,
            repaint: None,
        };
        let mut last_detail = NativeGbmScanoutBufferExportDetail::EglConfigUnavailable;
        let candidates = if linear_bridge {
            vec![RenderedScanoutCandidate {
                format,
                modifiers: vec![gbm::Modifier::Linear],
                // Explicit modifiers and the GBM LINEAR usage flag are mutually exclusive.
                usage: gbm::BufferObjectFlags::RENDERING,
                config_attributes: if format == gbm::Format::Argb8888 {
                    window_config_attributes()
                } else {
                    xrgb_window_config_attributes()
                },
            }]
        } else {
            rendered_scanout_candidates(&[])
                .into_iter()
                .filter(|candidate| candidate.format == format)
                .collect()
        };
        let mut import_failure = None;
        for candidate in candidates {
            let Some(config) = choose_scanout_config_for_format(
                &self.egl,
                self.display,
                candidate.config_attributes,
                candidate.format,
            ) else {
                continue;
            };
            let (mut target, surface, _) = match self.create_render_target(RenderTargetSpec {
                width: source.width,
                height: source.height,
                config,
                candidate,
            }) {
                Ok(created) => created,
                Err(detail) => {
                    last_detail = preferred_scanout_failure_detail(last_detail, detail);
                    continue;
                }
            };
            self.stats.dmabuf_target_creations =
                self.stats.dmabuf_target_creations.saturating_add(1);
            let mut import_cache = NativeDmaBufImportCache::with_capacity_and_stats(
                1,
                NativeDmaBufImportCacheStats::default(),
            );
            let empty_images = std::collections::BTreeMap::new();
            let rendered = render_native_target_composition(
                &self.egl,
                self.display,
                &mut target,
                surface.clone(),
                &mut import_cache,
                &empty_images,
                frame,
                false,
                true,
                self.buffer_age_supported,
            );
            let generation = self.allocate_target_generation();
            let persistent = PersistentCompositionTarget {
                target,
                surface,
                import_cache,
                preferred_modifiers: Vec::new(),
                generation,
            };
            match rendered {
                Ok((buffer, _))
                    if is_supported_rendered_scanout_candidate_buffer(&buffer)
                        && buffer.format() == source.format
                        && buffer.width() == source.width
                        && buffer.height() == source.height
                        && (!linear_bridge
                            || (buffer.modifier() == Some(0) && buffer.plane_count() == 1)) =>
                {
                    self.destroy_renderer_image_capture_target(persistent);
                    return Ok(buffer);
                }
                Ok(_) => {
                    last_detail = NativeGbmScanoutBufferExportDetail::InvalidBufferDescriptor;
                }
                Err(detail) => {
                    if image_import_failure(detail) {
                        import_failure = Some(detail);
                    }
                    last_detail = preferred_scanout_failure_detail(last_detail, detail);
                }
            }
            self.destroy_renderer_image_capture_target(persistent);
        }
        Err(import_failure.unwrap_or(last_detail))
    }

    fn destroy_renderer_image_capture_target(&mut self, target: PersistentCompositionTarget) {
        // Capture uses a one-entry temporary import cache for the client
        // source. Do not merge it into the persistent output-import ledger.
        let output_import_stats = self.stats.import_cache;
        self.destroy_persistent_composition_target(target);
        self.stats.import_cache = output_import_stats;
    }
}
