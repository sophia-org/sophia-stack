use crate::LiveRendererImageId;

#[derive(Debug)]
pub struct LiveRendererImageSnapshot {
    pub(super) image_id: LiveRendererImageId,
    pub(super) inner: sophia_renderer_native_egl::NativeRendererImageSnapshot,
}

impl LiveRendererImageSnapshot {
    pub const fn image_id(&self) -> LiveRendererImageId {
        self.image_id
    }
}
