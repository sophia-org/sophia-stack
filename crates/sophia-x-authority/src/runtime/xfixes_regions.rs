// The XFIXES region resource: a rectangle list a client can name, combine
// with another, and read back.
//
// Split out of `render_resources.rs` because the region surface grows with
// every XFIXES minor that builds one, and that file had reached the size
// where it stopped having one reason to change.

impl XAuthorityRuntime {
    pub fn create_xfixes_region(
        &mut self,
        namespace: NamespaceId,
        region: crate::XResourceId,
        rectangles: Vec<Rect>,
        generation: u64,
    ) -> Result<(), XAuthorityRuntimeError> {
        self.resources
            .insert(region, XResourceKind::Region, namespace, generation)?;
        self.xfixes_regions
            .insert(region, Region { rects: rectangles });
        Ok(())
    }

    pub fn set_xfixes_region(
        &mut self,
        namespace: NamespaceId,
        region: crate::XResourceId,
        rectangles: Vec<Rect>,
    ) -> Result<(), XAuthorityRuntimeError> {
        self.validate_xfixes_region_access(namespace, region)?;
        self.xfixes_regions
            .insert(region, Region { rects: rectangles });
        Ok(())
    }

    pub fn destroy_xfixes_region(
        &mut self,
        namespace: NamespaceId,
        region: crate::XResourceId,
    ) -> Result<(), XAuthorityRuntimeError> {
        self.validate_xfixes_region_access(namespace, region)?;
        self.resources.remove(region);
        self.xfixes_regions.remove(&region);
        Ok(())
    }

    pub fn validate_xfixes_region_access(
        &self,
        namespace: NamespaceId,
        region: crate::XResourceId,
    ) -> Result<(), XAuthorityRuntimeError> {
        self.resources
            .lookup(namespace, region, XResourceKind::Region)
            .map(|_| ())
            .map_err(Into::into)
    }

    /// Replace a region's contents with the result of combining two others.
    ///
    /// The destination may name either source: the operands are read out
    /// before anything is written, so `UnionRegion(a, b, a)` means what a
    /// client expects rather than reading half-updated state.
    pub fn combine_xfixes_regions(
        &mut self,
        namespace: NamespaceId,
        source: crate::XResourceId,
        other: crate::XResourceId,
        destination: crate::XResourceId,
        combine: fn(&[Rect], &[Rect]) -> Vec<Rect>,
    ) -> Result<(), XAuthorityRuntimeError> {
        self.validate_xfixes_region_access(namespace, source)?;
        self.validate_xfixes_region_access(namespace, other)?;
        self.validate_xfixes_region_access(namespace, destination)?;
        let left = self.xfixes_region_snapshot(namespace, source)?.rects;
        let right = self.xfixes_region_snapshot(namespace, other)?.rects;
        let rects = combine(&left, &right);
        self.xfixes_regions.insert(destination, Region { rects });
        Ok(())
    }

    /// Replace a region with the source subtracted from a bounding rectangle.
    pub fn invert_xfixes_region(
        &mut self,
        namespace: NamespaceId,
        source: crate::XResourceId,
        bounds: Rect,
        destination: crate::XResourceId,
    ) -> Result<(), XAuthorityRuntimeError> {
        self.validate_xfixes_region_access(namespace, source)?;
        self.validate_xfixes_region_access(namespace, destination)?;
        let rects = sophia_protocol::geometry::region_algebra::subtract(
            &[bounds],
            &self.xfixes_region_snapshot(namespace, source)?.rects,
        );
        self.xfixes_regions.insert(destination, Region { rects });
        Ok(())
    }

    pub fn translate_xfixes_region(
        &mut self,
        namespace: NamespaceId,
        region: crate::XResourceId,
        dx: i32,
        dy: i32,
    ) -> Result<(), XAuthorityRuntimeError> {
        self.validate_xfixes_region_access(namespace, region)?;
        let rects = sophia_protocol::geometry::region_algebra::translate(
            &self.xfixes_region_snapshot(namespace, region)?.rects,
            dx,
            dy,
        );
        self.xfixes_regions.insert(region, Region { rects });
        Ok(())
    }

    /// Replace a region with its own bounding rectangle.
    pub fn set_xfixes_region_to_extents(
        &mut self,
        namespace: NamespaceId,
        source: crate::XResourceId,
        destination: crate::XResourceId,
    ) -> Result<(), XAuthorityRuntimeError> {
        self.validate_xfixes_region_access(namespace, source)?;
        self.validate_xfixes_region_access(namespace, destination)?;
        let extents = sophia_protocol::geometry::region_algebra::extents(
            &self.xfixes_region_snapshot(namespace, source)?.rects,
        );
        self.xfixes_regions.insert(
            destination,
            Region {
                rects: extents.into_iter().collect(),
            },
        );
        Ok(())
    }

    /// A region's canonical rectangles, for `FetchRegion`.
    pub fn fetch_xfixes_region(
        &self,
        namespace: NamespaceId,
        region: crate::XResourceId,
    ) -> Result<Vec<Rect>, XAuthorityRuntimeError> {
        self.validate_xfixes_region_access(namespace, region)?;
        Ok(sophia_protocol::geometry::region_algebra::canonicalize(
            &self.xfixes_region_snapshot(namespace, region)?.rects,
        ))
    }

    pub fn xfixes_region_snapshot(
        &self,
        namespace: NamespaceId,
        region: crate::XResourceId,
    ) -> Result<Region, XAuthorityRuntimeError> {
        self.validate_xfixes_region_access(namespace, region)?;
        self.xfixes_regions
            .get(&region)
            .cloned()
            .ok_or(XAuthorityRuntimeError::UnknownResource)
    }

    /// Why a region could not be built from something the server holds.
    ///
    /// Each source has its own way of being wrong, and the protocol gives
    /// each a different error, so they stay apart rather than collapsing
    /// into one "bad request".
    ///
    /// A region built this way is a copy: the source's clip origins are
    /// deliberately not folded in. Those origins describe how the graphics
    /// context or picture *uses* its clip, not where the region's rectangles
    /// are, and translating here would apply them twice once the region is
    /// installed as a clip again.
    pub(crate) fn create_xfixes_region_from_bitmap(
        &mut self,
        namespace: NamespaceId,
        region: crate::XResourceId,
        bitmap: crate::XResourceId,
        generation: u64,
    ) -> Result<(), XFixesSourceError> {
        if self.resource_id_in_use(region) {
            return Err(XFixesSourceError::IdInUse);
        }
        let rects = self
            .shape_mask_rects(namespace, bitmap)
            .map_err(|error| match error {
                crate::XShapeError::NotABitmap => XFixesSourceError::NotABitmap,
                _ => XFixesSourceError::UnknownPixmap,
            })?;
        self.create_xfixes_region(namespace, region, rects, generation)
            .map_err(|_| XFixesSourceError::IdInUse)
    }

    pub(crate) fn create_xfixes_region_from_window(
        &mut self,
        namespace: NamespaceId,
        region: crate::XResourceId,
        window: crate::XResourceId,
        kind: u8,
        generation: u64,
    ) -> Result<(), XFixesSourceError> {
        if self.resource_id_in_use(region) {
            return Err(XFixesSourceError::IdInUse);
        }
        // The window is checked before the kind, which is the order Xorg
        // reports them in: a client that named a window it does not have
        // hears about that first.
        self.validate_window_access(namespace, window)
            .map_err(|_| XFixesSourceError::UnknownWindow)?;
        // XFIXES will build a region from a window's bounding or clip shape
        // and no other. SHAPE has a third kind, Input, and asking for it here
        // is a value this request does not define.
        if kind != crate::X_XFIXES_WINDOW_REGION_BOUNDING
            && kind != crate::X_XFIXES_WINDOW_REGION_CLIP
        {
            return Err(XFixesSourceError::InvalidKind);
        }
        let (_, rects) = self.effective_shape(window, kind);
        self.create_xfixes_region(namespace, region, rects, generation)
            .map_err(|_| XFixesSourceError::IdInUse)
    }

    pub(crate) fn create_xfixes_region_from_gc(
        &mut self,
        namespace: NamespaceId,
        region: crate::XResourceId,
        gc: crate::XResourceId,
        generation: u64,
    ) -> Result<(), XFixesSourceError> {
        if self.resource_id_in_use(region) {
            return Err(XFixesSourceError::IdInUse);
        }
        let record = self
            .graphics_contexts
            .get(namespace, gc)
            .map_err(|_| XFixesSourceError::UnknownGraphicsContext)?;
        if record.values.clip_rectangles.is_empty() {
            return Err(XFixesSourceError::NoClip);
        }
        let rects = record.values.clip_rectangles.clone();
        self.create_xfixes_region(namespace, region, rects, generation)
            .map_err(|_| XFixesSourceError::IdInUse)
    }

    pub(crate) fn create_xfixes_region_from_picture(
        &mut self,
        namespace: NamespaceId,
        region: crate::XResourceId,
        picture: crate::XResourceId,
        generation: u64,
    ) -> Result<(), XFixesSourceError> {
        if self.resource_id_in_use(region) {
            return Err(XFixesSourceError::IdInUse);
        }
        let record = self
            .render_picture_record(namespace, picture)
            .map_err(|_| XFixesSourceError::UnknownPicture)?;
        // A solid fill or a gradient has no drawable, and a region describes
        // an area of one.
        if record.generated.is_some() {
            return Err(XFixesSourceError::UnknownPicture);
        }
        if record.clip_rects.is_empty() {
            return Err(XFixesSourceError::NoClip);
        }
        self.create_xfixes_region(namespace, region, record.clip_rects.clone(), generation)
            .map_err(|_| XFixesSourceError::IdInUse)
    }

    /// Grow a region outward on each side.
    ///
    /// A client uses this to turn a window's shape into the area its shadow
    /// needs. An empty source leaves the destination alone rather than
    /// emptying it, which is what Xorg does and what a reimplementation
    /// reaches for the other way round.
    pub(crate) fn expand_xfixes_region(
        &mut self,
        namespace: NamespaceId,
        source: crate::XResourceId,
        destination: crate::XResourceId,
        left: u16,
        right: u16,
        top: u16,
        bottom: u16,
    ) -> Result<(), XFixesSourceError> {
        self.validate_xfixes_region_access(namespace, source)
            .map_err(|_| XFixesSourceError::UnknownRegion)?;
        self.validate_xfixes_region_access(namespace, destination)
            .map_err(|_| XFixesSourceError::UnknownRegion)?;
        let rects = self
            .xfixes_region_snapshot(namespace, source)
            .map_err(|_| XFixesSourceError::UnknownRegion)?
            .rects;
        if rects.is_empty() {
            return Ok(());
        }
        // Each rectangle grows and the results are unioned, so an L-shaped
        // region keeps its notch instead of becoming a grown bounding box.
        let grown: Vec<Rect> = rects
            .iter()
            .map(|rect| Rect {
                x: rect.x.saturating_sub(i32::from(left)),
                y: rect.y.saturating_sub(i32::from(top)),
                width: rect
                    .width
                    .saturating_add(i32::from(left))
                    .saturating_add(i32::from(right)),
                height: rect
                    .height
                    .saturating_add(i32::from(top))
                    .saturating_add(i32::from(bottom)),
            })
            .collect();
        let expanded = sophia_protocol::geometry::region_algebra::canonicalize(&grown);
        self.xfixes_regions
            .insert(destination, Region { rects: expanded });
        Ok(())
    }
}

/// Why a region could not be built from, or expanded into, what a client
/// named.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum XFixesSourceError {
    IdInUse,
    UnknownPixmap,
    NotABitmap,
    UnknownWindow,
    InvalidKind,
    UnknownGraphicsContext,
    UnknownPicture,
    /// The source has no clip to copy, which the protocol distinguishes from
    /// the source not existing.
    NoClip,
    UnknownRegion,
}
