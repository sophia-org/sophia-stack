pub const NATIVE_IMAGE_IMPORT_DEVICE_CAPACITY: usize = 16;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NativeImageTransferStats {
    pub device_initializations: u64,
    pub attempts: u64,
    pub captures: u64,
    pub failures: u64,
    pub bridge_bytes: u64,
    pub last_source_failure: Option<NativeGbmScanoutBufferExportDetail>,
    pub last_destination_failure: Option<NativeGbmScanoutBufferExportDetail>,
    pub max_capture_duration: std::time::Duration,
}

enum NativeImageImportDevice {
    Unopened(OwnedFd),
    Ready(Box<NativeGbmRenderedScanoutContext<OwnedFd>>),
    Unavailable,
}

fn image_import_failure(detail: NativeGbmScanoutBufferExportDetail) -> bool {
    matches!(
        detail,
        NativeGbmScanoutBufferExportDetail::DmaBufImageCreateFailed
            | NativeGbmScanoutBufferExportDetail::DmaBufImageBindFailed
            | NativeGbmScanoutBufferExportDetail::DmaBufImportFailed
    )
}

impl<T: AsFd> NativeGbmRenderedScanoutContext<T> {
    /// Latch backend-admitted devices for this worker incarnation. Contexts are
    /// opened lazily; no path discovery or device replacement occurs here.
    pub fn set_image_import_devices(
        &mut self,
        devices: Vec<OwnedFd>,
    ) -> Result<(), NativeGbmScanoutBufferExportDetail> {
        if self.import_devices.is_some()
            || !self.renderer_images.is_empty()
            || devices.len() > NATIVE_IMAGE_IMPORT_DEVICE_CAPACITY
        {
            return Err(NativeGbmScanoutBufferExportDetail::InvalidTarget);
        }
        self.import_devices = Some(
            devices
                .into_iter()
                .map(NativeImageImportDevice::Unopened)
                .collect(),
        );
        Ok(())
    }

    pub fn image_transfer_stats(&self) -> NativeImageTransferStats {
        self.transfer_stats
    }

    fn transfer_renderer_image(
        &mut self,
        image_id: NativeRendererImageId,
        source: NativeMultiPlaneDmaBufFrame<'_>,
        direct_failure: NativeGbmScanoutBufferExportDetail,
    ) -> Result<NativeGbmOwnedScanoutBuffer, NativeGbmScanoutBufferExportDetail> {
        let Some(mut devices) = self.import_devices.take() else {
            return Err(direct_failure);
        };
        let started = Instant::now();
        let result =
            self.transfer_renderer_image_on_devices(image_id, source, &mut devices, direct_failure);
        self.import_devices = Some(devices);
        if result.is_ok() {
            if self.transferred_layouts.len() < 64 {
                self.transferred_layouts
                    .insert((source.format, source.modifier));
            }
            self.transfer_stats.captures = self.transfer_stats.captures.saturating_add(1);
        } else {
            self.transfer_stats.failures = self.transfer_stats.failures.saturating_add(1);
        }
        self.transfer_stats.max_capture_duration = self
            .transfer_stats
            .max_capture_duration
            .max(started.elapsed());
        result
    }

    fn transfer_renderer_image_on_devices(
        &mut self,
        image_id: NativeRendererImageId,
        source: NativeMultiPlaneDmaBufFrame<'_>,
        devices: &mut [NativeImageImportDevice],
        direct_failure: NativeGbmScanoutBufferExportDetail,
    ) -> Result<NativeGbmOwnedScanoutBuffer, NativeGbmScanoutBufferExportDetail> {
        let minimum_bytes = u64::from(source.width) * u64::from(source.height) * 4;
        if minimum_bytes
            .saturating_mul(2)
            .saturating_add(self.renderer_image_bytes)
            > DEFAULT_NATIVE_RENDERER_IMAGE_BYTE_BUDGET
        {
            return Err(NativeGbmScanoutBufferExportDetail::RendererImageStoreFull);
        }
        for device in devices {
            if matches!(device, NativeImageImportDevice::Unopened(_)) {
                let NativeImageImportDevice::Unopened(fd) =
                    std::mem::replace(device, NativeImageImportDevice::Unavailable)
                else {
                    unreachable!();
                };
                self.transfer_stats.device_initializations =
                    self.transfer_stats.device_initializations.saturating_add(1);
                if let Ok(context) = NativeGbmRenderedScanoutContext::new(fd, 1) {
                    *device = NativeImageImportDevice::Ready(Box::new(context));
                }
            }
            let NativeImageImportDevice::Ready(context) = device else {
                continue;
            };
            self.transfer_stats.attempts = self.transfer_stats.attempts.saturating_add(1);
            let bridge = match context.render_renderer_image_snapshot(image_id, source, true) {
                Ok(bridge) => bridge,
                Err(detail) => {
                    self.transfer_stats.last_source_failure = Some(detail);
                    continue;
                }
            };
            let bridge_bytes = u64::from(bridge.pitch()) * u64::from(bridge.height());
            if bridge_bytes
                .saturating_add(minimum_bytes)
                .saturating_add(self.renderer_image_bytes)
                > DEFAULT_NATIVE_RENDERER_IMAGE_BYTE_BUDGET
            {
                return Err(NativeGbmScanoutBufferExportDetail::RendererImageStoreFull);
            }
            let fds = bridge.export_plane_fds()?.into_plane_fds();
            let pitches = bridge.plane_pitches();
            let offsets = bridge.plane_offsets();
            let frame = NativeMultiPlaneDmaBufFrame {
                width: bridge.width(),
                height: bridge.height(),
                format: bridge.format(),
                modifier: 0,
                plane_count: bridge.plane_count(),
                planes: std::array::from_fn(|index| {
                    fds[index].as_ref().map(|fd| NativeDmaBufPlane {
                        fd: fd.as_fd(),
                        offset: offsets[index],
                        stride: pitches[index],
                    })
                }),
            };
            // The destination owns the retained image. The temporary bridge and
            // its source display stay alive through submission of this copy;
            // DMA-BUF implicit fences order the two GPU accesses.
            match self.render_renderer_image_snapshot(image_id, frame, false) {
                Ok(buffer) => {
                    let bytes = u64::from(buffer.pitch()) * u64::from(buffer.height());
                    if bytes
                        .saturating_add(bridge_bytes)
                        .saturating_add(self.renderer_image_bytes)
                        > DEFAULT_NATIVE_RENDERER_IMAGE_BYTE_BUDGET
                    {
                        return Err(NativeGbmScanoutBufferExportDetail::RendererImageStoreFull);
                    }
                    self.transfer_stats.bridge_bytes = self
                        .transfer_stats
                        .bridge_bytes
                        .saturating_add(bridge_bytes);
                    return Ok(buffer);
                }
                Err(detail) if image_import_failure(detail) => {
                    self.transfer_stats.last_destination_failure = Some(detail);
                    continue;
                }
                Err(detail) => return Err(detail),
            }
        }
        Err(direct_failure)
    }
}
