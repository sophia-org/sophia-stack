impl XAuthorityRuntime {
    /// Composite a coverage mask built from trapezoids or triangles.
    ///
    /// Both requests are the same operation over different primitives: the
    /// shapes become an A8 coverage mask, and one composite draws the source
    /// through it. GTK's window decorations arrive this way.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn render_apply_primitive_coverage(
        &mut self,
        transaction: TransactionId,
        namespace: NamespaceId,
        op: u8,
        source: crate::XResourceId,
        destination: crate::XResourceId,
        mask_format: u32,
        source_origin: (i16, i16),
        coverage: XRenderPrimitiveCoverage<'_>,
    ) -> Result<XAuthorityResponsePacket, XRenderPictureError> {
        let source_record = self.render_picture_record(namespace, source)?;
        let destination_record = self.render_picture_record(namespace, destination)?;
        // A mask format of None means the primitives are drawn without one.
        // Any other value has to name a format this server has, even though
        // the coverage is always eight-bit: answering for a format that does
        // not exist would tell a client its identifier was good.
        if mask_format != 0
            && crate::XRenderPictFormatKind::from_format_id(mask_format).is_none()
        {
            return Err(XRenderPictureError::UnknownFormat);
        }
        let Some(bounds) = coverage.bounds() else {
            return Ok(XAuthorityResponsePacket::accepted(transaction));
        };
        let Some((size, window_generation)) =
            self.render_target_geometry(namespace, &destination_record)
        else {
            return Err(XRenderPictureError::Drawable);
        };
        // The primitives are placed by the client and can extend past the
        // destination; what misses is not drawn, and is not an error either.
        let Some(visible) = intersect_with_extent(bounds, size) else {
            return Ok(XAuthorityResponsePacket::accepted(transaction));
        };
        let mask = coverage.rasterize(bounds);
        let mask_plane = crate::XRenderSamplePlane::from_coverage(
            &mask,
            bounds.width.max(0) as usize,
            bounds.height.max(0) as usize,
        );
        // The coverage was rasterised over the whole primitive extent, so a
        // clipped composite reads into it at the offset it was clipped by.
        let mask_origin = (
            visible.x.saturating_sub(bounds.x),
            visible.y.saturating_sub(bounds.y),
        );
        let source_plane = self.render_source_plane(&source_record);
        // Xorg anchors the source at the first primitive's leading corner
        // before compositing (fb/fbtrap.c). A client that places a shape far
        // from the origin sends a source offset measured from that corner
        // rather than from the destination, so without the subtraction the
        // read lands outside the source and the primitive comes out with a
        // transparent band -- which is what a window shadow looks like when
        // this is wrong.
        let anchor = coverage.anchor();
        let origin = (
            visible
                .x
                .saturating_add(i32::from(source_origin.0))
                .saturating_sub(anchor.0),
            visible
                .y
                .saturating_add(i32::from(source_origin.1))
                .saturating_sub(anchor.1),
        );
        let clip = Self::render_translated_clip(&destination_record);
        let Some(result) = self.software_buffers.render_composite(
            destination_record.drawable,
            size,
            op,
            &source_plane,
            Some(&mask_plane),
            false,
            origin,
            mask_origin,
            visible,
            &clip,
            destination_record.format,
        ) else {
            return Ok(XAuthorityResponsePacket::rejected(
                transaction,
                XAuthorityRuntimeError::InvalidResource,
            ));
        };
        let Some(generation) = window_generation else {
            return Ok(XAuthorityResponsePacket::accepted(transaction));
        };
        self.pending_raster_command = Some(XAuthorityRasterCommand::Unsupported(
            XRasterUnsupportedKind::RenderOperation,
        ));
        Ok(self.finish_drawing_update(XDrawingUpdate::core_draw(
            transaction,
            namespace,
            destination_record.drawable,
            result.handle(),
            Region::single(visible),
            generation,
            250,
        )))
    }
}

/// The primitives a coverage composite draws, whichever request carried them.
pub(crate) enum XRenderPrimitiveCoverage<'a> {
    Trapezoids(&'a [crate::XRenderTrapezoid]),
    Triangles(&'a [crate::XRenderTriangle]),
}

impl XRenderPrimitiveCoverage<'_> {
    fn bounds(&self) -> Option<Rect> {
        match self {
            Self::Trapezoids(traps) => crate::software::trapezoid_bounds(traps),
            Self::Triangles(triangles) => crate::software::triangle_bounds(triangles),
        }
    }

    fn rasterize(&self, bounds: Rect) -> Vec<u8> {
        match self {
            Self::Trapezoids(traps) => crate::software::rasterize_trapezoids(traps, bounds),
            Self::Triangles(triangles) => {
                crate::software::rasterize_triangles(triangles, bounds)
            }
        }
    }

    /// The leading corner the source offset is measured from.
    fn anchor(&self) -> (i32, i32) {
        match self {
            Self::Trapezoids(traps) => traps
                .first()
                .map(|trap| (trap.left_p1.0 >> 16, trap.left_p1.1 >> 16))
                .unwrap_or((0, 0)),
            Self::Triangles(triangles) => triangles
                .first()
                .map(|triangle| (triangle.p1.0 >> 16, triangle.p1.1 >> 16))
                .unwrap_or((0, 0)),
        }
    }
}
