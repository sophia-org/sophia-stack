#[derive(Clone, Copy, Debug)]
pub(crate) struct XPointerQuery {
    pub child: crate::XResourceId,
    pub root_x: i16,
    pub root_y: i16,
    pub win_x: i16,
    pub win_y: i16,
    pub mask: u16,
}

impl XAuthorityRuntime {
    // Serialize anchor adjustment with input publication, not with socket I/O.
    // A stationary pointer must not move along with a reconfigured X window.
    fn change_pointer_anchor_geometry<T>(
        &mut self,
        namespace: NamespaceId,
        change: impl FnOnce(&mut Self) -> Result<T, XAuthorityRuntimeError>,
    ) -> Result<T, XAuthorityRuntimeError> {
        let shared = self.input_authority.clone();
        let mut authority = shared.lock().expect("X11 input authority lock poisoned");
        let anchor = authority
            .pointer_query_state(namespace)
            .position
            .map(|pointer| pointer.surface_window);
        let old = anchor.and_then(|window| self.window_root_position(window));
        let result = change(self)?;
        if let Some((old, new)) =
            old.zip(anchor.and_then(|window| self.window_root_position(window)))
        {
            authority.shift_query_anchor(namespace, old, new);
        }
        Ok(result)
    }
    pub(crate) fn query_pointer(
        &self,
        namespace: NamespaceId,
        window: crate::XResourceId,
    ) -> Result<XPointerQuery, XAuthorityRuntimeError> {
        let root = crate::XResourceId::new(u64::from(crate::X_SETUP_DEFAULT_ROOT), 1);
        if window != root {
            self.validate_window_access(namespace, window)?;
        }
        let state = self.input_authority_mut().pointer_query_state(namespace);
        let mut query = XPointerQuery {
            child: crate::XResourceId::NONE,
            root_x: 0,
            root_y: 0,
            win_x: 0,
            win_y: 0,
            mask: state.mask,
        };
        let Some(pointer) = state.position else {
            return Ok(query);
        };
        query.root_x = pointer.root_x;
        query.root_y = pointer.root_y;
        // Root coordinates describe Engine's output space. Within an X tree,
        // retain the local position Engine computed through visual transforms.
        let logical = self
            .windows
            .get(pointer.surface_window)
            .filter(|record| record.namespace == namespace && record.surface == pointer.surface)
            .and_then(|_| self.window_root_position(pointer.surface_window))
            .map_or(
                (i32::from(pointer.root_x), i32::from(pointer.root_y)),
                |(x, y)| {
                    (
                        x.saturating_add(pointer.local_x),
                        y.saturating_add(pointer.local_y),
                    )
                },
            );
        if window == root {
            query.win_x = pointer.root_x;
            query.win_y = pointer.root_y;
        } else if let Some((x, y)) = self.window_root_position(window) {
            let clamp = |value: i32| value.clamp(i32::from(i16::MIN), i32::from(i16::MAX)) as i16;
            query.win_x = clamp(logical.0.saturating_sub(x));
            query.win_y = clamp(logical.1.saturating_sub(y));
        }
        // Engine chose the surface. Only refine its X descendants; scanning
        // other top levels here would invent compositor hit-testing authority.
        if self
            .windows
            .get(pointer.surface_window)
            .is_none_or(|record| record.surface != pointer.surface)
            || !self.pointer_window_contains(namespace, pointer.surface_window, logical)
        {
            return Ok(query);
        }
        let mut deepest = pointer.surface_window;
        for _ in 0..64 {
            let child = self
                .windows
                .direct_children(namespace, deepest)
                .into_iter()
                .filter(|child| self.pointer_window_contains(namespace, *child, logical))
                .max_by_key(|child| {
                    self.windows
                        .get(*child)
                        .map(|record| (record.stack_rank, record.id))
                });
            let Some(child) = child else {
                break;
            };
            deepest = child;
        }
        let mut candidate = deepest;
        for _ in 0..64 {
            let Some(record) = self.windows.get(candidate) else {
                break;
            };
            if record.namespace != namespace || record.map_state != crate::XMapState::Viewable {
                break;
            }
            if record.parent == window {
                query.child = candidate;
                break;
            }
            if record.parent == root {
                break;
            }
            candidate = record.parent;
        }
        Ok(query)
    }

    fn pointer_window_contains(
        &self,
        namespace: NamespaceId,
        window: crate::XResourceId,
        point: (i32, i32),
    ) -> bool {
        let Some(record) = self.windows.get(window) else {
            return false;
        };
        if record.namespace != namespace || record.map_state != crate::XMapState::Viewable {
            return false;
        }
        let Some(origin) = self.window_root_position(window) else {
            return false;
        };
        let x = point.0.saturating_sub(origin.0);
        let y = point.1.saturating_sub(origin.1);
        if x < 0 || y < 0 || x >= record.geometry.width || y >= record.geometry.height {
            return false;
        }
        self.window_shapes.get(&window).is_none_or(|shape| {
            [&shape.bounding, &shape.input].into_iter().all(|region| {
                region.as_ref().is_none_or(|rects| {
                    sophia_protocol::geometry::region_algebra::contains_point(rects, x, y)
                })
            })
        })
    }
}
