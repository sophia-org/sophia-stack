/// A pixmap whose XID was freed while RENDER pictures still reference it.
/// The private storage key is never registered as a wire resource. All those
/// pictures keep one mutable backing, independent of subsequent XID reuse.
#[derive(Debug)]
struct XRetainedRenderPixmap {
    namespace: NamespaceId,
    pixmap: XPixmapRecord,
    pictures: usize,
    // Keep the underlying allocations alive even after the public pixmap and
    // its renderer registration disappear. RENDER currently uses CPU pixels;
    // retaining FDs does not add GPU sampling to that software path.
    _shm: Option<XShmPixmapBinding>,
    _dri3: Option<XDri3PixmapRecord>,
}

impl XAuthorityRuntime {
    fn render_retain_freed_pixmap(
        &mut self,
        namespace: NamespaceId,
        pixmap: crate::XResourceId,
    ) -> Result<(), XAuthorityRuntimeError> {
        let pictures = self
            .render_pictures
            .values()
            .filter(|record| !record.drawable_is_window && record.drawable == pixmap)
            .count();
        if pictures == 0 {
            return Ok(());
        }
        let metadata = *self
            .pixmaps
            .get(&pixmap)
            .ok_or(XAuthorityRuntimeError::UnknownResource)?;
        // X11 can name only 32-bit resources. Allocate above that range, and
        // never recycle a private key while an old picture can still hold it.
        let backing = loop {
            let key = crate::XResourceId::new(self.next_render_backing, 1);
            self.next_render_backing = self
                .next_render_backing
                .checked_add(1)
                .ok_or(XAuthorityRuntimeError::InvalidResource)?;
            if !self.resource_id_in_use(key) && !self.retained_render_pixmaps.contains_key(&key) {
                break key;
            }
        };
        self.software_buffers.rekey_pixmap(pixmap, backing);
        self.retained_render_pixmaps.insert(
            backing,
            XRetainedRenderPixmap {
                namespace,
                pixmap: metadata,
                pictures,
                _shm: self.shm_pixmaps.remove(&pixmap),
                _dri3: self.dri3_pixmaps.remove(&pixmap),
            },
        );
        for record in self.render_pictures.values_mut() {
            if !record.drawable_is_window && record.drawable == pixmap {
                record.drawable = backing;
            }
        }
        Ok(())
    }

    fn render_release_picture(&mut self, picture: crate::XResourceId) {
        self.resources.remove(picture);
        let Some(record) = self.render_pictures.remove(&picture) else {
            return;
        };
        let Some(retained) = self.retained_render_pixmaps.get_mut(&record.drawable) else {
            return;
        };
        retained.pictures -= 1;
        if retained.pictures == 0 {
            self.retained_render_pixmaps.remove(&record.drawable);
            self.software_buffers.remove(record.drawable);
            self.shm_mappings
                .retain(|_, mapping| mapping.strong_count() != 0);
        }
    }
}
