const DMA_BUF_IMPORT_MAX_FORMATS: usize = 512;
const DMA_BUF_IMPORT_MAX_MODIFIERS: usize = 16_384;

impl XAuthorityRuntime {
    /// Latches the fixed provider's inventory; oversized input advertises no explicit layouts.
    pub fn set_dma_buf_import_formats(
        &mut self,
        formats: Vec<crate::XServerFrontendDmaBufImportFormat>,
    ) {
        if self.dma_buf_import_formats.is_some() {
            return;
        }
        let mut canonical = BTreeMap::<u32, Vec<u64>>::new();
        let modifier_count = formats
            .iter()
            .try_fold(0usize, |count, row| count.checked_add(row.modifiers.len()));
        if formats.len() <= DMA_BUF_IMPORT_MAX_FORMATS
            && modifier_count.is_some_and(|count| count <= DMA_BUF_IMPORT_MAX_MODIFIERS)
        {
            for row in formats {
                canonical
                    .entry(row.format)
                    .or_default()
                    .extend(row.modifiers.into_iter().filter(|modifier| {
                        *modifier != sophia_protocol::DRM_FORMAT_MOD_INVALID
                            && *modifier != u64::MAX
                    }));
            }
            for modifiers in canonical.values_mut() {
                modifiers.sort_unstable();
                modifiers.dedup();
            }
        }
        self.dma_buf_import_formats = Some(canonical);
    }

    pub fn dma_buf_import_modifiers(&self, format: u32) -> &[u64] {
        self.dma_buf_import_formats
            .as_ref()
            .and_then(|formats| formats.get(&format))
            .map_or(&[], Vec::as_slice)
    }
}
