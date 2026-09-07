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
}
