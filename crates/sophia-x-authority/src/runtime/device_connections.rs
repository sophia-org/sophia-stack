impl XAuthorityRuntime {
    pub(crate) fn pin_client_device_bundle(
        &mut self,
        client_id: u64,
        bundle: Option<std::sync::Arc<crate::XServerFrontendDeviceBundle>>,
    ) {
        self.device_connections.entry(client_id).or_insert(bundle);
    }

    pub(crate) fn release_client_device_bundle(&mut self, client_id: u64) {
        self.device_connections.remove(&client_id);
    }

    /// Socket clients have an immutable pin, including an explicitly absent device.
    /// Only pure dispatch fixtures without a connection use the legacy inventory.
    pub fn dma_buf_import_modifiers_for_client(&self, client_id: u64, format: u32) -> &[u64] {
        match self.device_connections.get(&client_id) {
            Some(Some(bundle)) => bundle.dma_buf_import_modifiers(format),
            Some(None) => &[],
            None => self.dma_buf_import_modifiers(format),
        }
    }
}
