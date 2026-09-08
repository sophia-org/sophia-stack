use super::*;

pub(super) struct LiveXAdmissionPolicy {
    pub(super) registry: Arc<Mutex<NamespaceRegistry>>,
    pub(super) namespace: NamespaceId,
    pub(super) session_user_id: u32,
}

impl XServerFrontendAdmissionPolicy for LiveXAdmissionPolicy {
    fn admit(
        &self,
        request: XServerFrontendAdmissionRequest,
    ) -> Result<ClientAdmissionContext, XServerFrontendAdmissionError> {
        let peer = request
            .peer_credentials
            .ok_or(XServerFrontendAdmissionError::Denied)?;
        if peer.user_id != self.session_user_id {
            return Err(XServerFrontendAdmissionError::Denied);
        }
        self.registry
            .lock()
            .map_err(|_| XServerFrontendAdmissionError::Unavailable)?
            .admit(self.namespace, request.setup_authentication)
            .map_err(|_| XServerFrontendAdmissionError::Unavailable)
    }

    fn revoke(&self, context: ClientAdmissionContext) -> Result<(), XServerFrontendAdmissionError> {
        if context.namespace.id != self.namespace {
            return Err(XServerFrontendAdmissionError::Unavailable);
        }
        self.registry
            .lock()
            .map_err(|_| XServerFrontendAdmissionError::Unavailable)?
            .revoke_admission(context.client_id)
            .map(|_| ())
            .map_err(|_| XServerFrontendAdmissionError::Unavailable)
    }
}

/// Originates buffers for pixmaps a client did not allocate.
///
/// Holds the same device the render node is handed from, so a buffer the
/// authority originates and one the client imports come from one device and can
/// be composited by the same path.
#[cfg(feature = "native-session")]
pub(super) struct LiveXPixmapAllocator {
    pub(super) device: std::fs::File,
    shared: Option<sophia_backend_live::LiveSharedPixmapService>,
}

#[cfg(feature = "native-session")]
impl LiveXPixmapAllocator {
    pub(super) fn new(device: std::fs::File) -> Self {
        let shared = device
            .try_clone()
            .map_err(|_| sophia_backend_live::LiveSharedPixmapError::Unavailable)
            .and_then(sophia_backend_live::LiveSharedPixmapService::new);
        let shared = match shared {
            Ok(service) => Some(service),
            Err(reason) => {
                tracing::warn!(?reason, "pixmap texture export capability unavailable");
                None
            }
        };
        Self { device, shared }
    }
}

#[cfg(feature = "native-session")]
impl XServerFrontendPixmapAllocator for LiveXPixmapAllocator {
    fn allocate_pixmap_buffer(
        &self,
        request: XServerFrontendPixmapAllocation,
    ) -> Result<XServerFrontendAllocatedPixmap, XServerFrontendPixmapAllocationError> {
        if let Some(shared) = self.shared.as_ref() {
            let allocation = shared
                .allocate(
                    sophia_protocol::BufferHandle::from_raw(request.handle),
                    request.size,
                    request.depth,
                )
                .map_err(shared_pixmap_error)?;
            return Ok(XServerFrontendAllocatedPixmap {
                descriptor: allocation.descriptor,
                plane_fds: allocation.plane_fds,
            });
        }
        let allocation = sophia_backend_live::allocate_shared_buffer(
            &self.device,
            request.handle,
            request.size,
            request.depth,
        )
        .map_err(|error| match error {
            sophia_backend_live::LiveSharedBufferError::UnsupportedTarget => {
                XServerFrontendPixmapAllocationError::UnsupportedTarget
            }
            sophia_backend_live::LiveSharedBufferError::DeviceRejected
            | sophia_backend_live::LiveSharedBufferError::ExportFailed => {
                XServerFrontendPixmapAllocationError::AllocationFailed
            }
        })?;
        Ok(XServerFrontendAllocatedPixmap {
            descriptor: allocation.descriptor,
            plane_fds: allocation.plane_fds,
        })
    }

    fn supports_pixmap_textures(&self) -> bool {
        self.shared.is_some()
    }

    fn update_pixmap_buffer(
        &self,
        request: sophia_x_authority::XServerFrontendPixmapUpdate,
    ) -> Result<(), XServerFrontendPixmapAllocationError> {
        let shared = self
            .shared
            .as_ref()
            .ok_or(XServerFrontendPixmapAllocationError::Unavailable)?;
        shared
            .update(sophia_backend_live::LiveSharedPixmapUpdate {
                handle: request.handle,
                revision: request.revision,
                size: request.size,
                format: request.format,
                patches: request
                    .patches
                    .into_iter()
                    .map(|patch| sophia_backend_live::LiveSharedPixmapPatch {
                        rect: patch.rect,
                        bytes: patch.bytes,
                    })
                    .collect(),
            })
            .map_err(shared_pixmap_error)
    }

    fn release_pixmap_buffer(
        &self,
        handle: sophia_protocol::BufferHandle,
    ) -> Result<(), XServerFrontendPixmapAllocationError> {
        self.shared
            .as_ref()
            .ok_or(XServerFrontendPixmapAllocationError::Unavailable)?
            .release(handle)
            .map_err(shared_pixmap_error)
    }
}

#[cfg(feature = "native-session")]
fn shared_pixmap_error(
    error: sophia_backend_live::LiveSharedPixmapError,
) -> XServerFrontendPixmapAllocationError {
    use sophia_backend_live::LiveSharedPixmapError as E;
    match error {
        E::InvalidTarget => XServerFrontendPixmapAllocationError::UnsupportedTarget,
        E::UnknownBacking => XServerFrontendPixmapAllocationError::UnknownBacking,
        E::Unavailable => XServerFrontendPixmapAllocationError::Unavailable,
        E::Capacity | E::IdentityInUse | E::DeviceRejected | E::ExportFailed | E::UploadFailed => {
            XServerFrontendPixmapAllocationError::AllocationFailed
        }
    }
}

pub(super) struct LiveXRenderDeviceProvider {
    pub(super) device: std::fs::File,
}

impl XServerFrontendRenderDeviceProvider for LiveXRenderDeviceProvider {
    fn open_render_device_fd(
        &self,
    ) -> Result<std::os::fd::OwnedFd, XServerFrontendRenderDeviceError> {
        use std::os::fd::AsRawFd as _;

        let proc_path = format!("/proc/self/fd/{}", self.device.as_raw_fd());
        let selected_node = std::fs::read_link(&proc_path)
            .map_err(|_| XServerFrontendRenderDeviceError::Unavailable)?;
        let selected_name = selected_node
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or(XServerFrontendRenderDeviceError::Unavailable)?;

        let render_node = if selected_name.starts_with("renderD") {
            selected_node
        } else {
            let selected_device =
                std::fs::canonicalize(format!("/sys/class/drm/{selected_name}/device"))
                    .map_err(|_| XServerFrontendRenderDeviceError::Unavailable)?;
            std::fs::read_dir("/sys/class/drm")
                .map_err(|_| XServerFrontendRenderDeviceError::Unavailable)?
                .filter_map(Result::ok)
                .take(64)
                .find_map(|entry| {
                    let name = entry.file_name();
                    let name = name.to_str()?;
                    if !name.starts_with("renderD") {
                        return None;
                    }
                    let device = std::fs::canonicalize(entry.path().join("device")).ok()?;
                    (device == selected_device).then(|| std::path::Path::new("/dev/dri").join(name))
                })
                .ok_or(XServerFrontendRenderDeviceError::Unavailable)?
        };

        // A fresh render-node open gives each DRI3 client its own DRM file
        // description and withholds the compositor's primary/KMS node.
        std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(render_node)
            .map(std::os::fd::OwnedFd::from)
            .map_err(|_| XServerFrontendRenderDeviceError::OpenFailed)
    }
}
