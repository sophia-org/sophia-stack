// A pointer over a head that does not begin at the root origin has to reach
// the window drawn there.
//
// Included into `production_visual_runtime::projection`'s test module, which
// is where `presented_input_layer_snapshots` lives.

/// The retired frame a head would publish for one window.
///
/// `native_size` is the head's framebuffer. When it differs from the viewport
/// the plan scales into it, which is exactly the case where a reconstructed
/// root rectangle would drift and a carried one cannot.
fn presented_head_frame(
    output: OutputId,
    viewport: Rect,
    root_geometry: Rect,
    native_size: sophia_protocol::Size,
    target: SurfaceId,
) -> sophia_engine::OutputFrameDamageSnapshot {
    let committed = vec![CommittedSurfaceState::with_source(
        target,
        1,
        root_geometry,
        BufferSource::CpuBuffer { handle: 31 },
        sophia_protocol::Size {
            width: root_geometry.width,
            height: root_geometry.height,
        },
        Region::default(),
    )];
    let display_list = sophia_engine::surface_chrome_display_list_for_surfaces(
        output,
        &[target],
        &[],
        &committed,
        None,
        SurfaceChromeStyle::default(),
    )
    .unwrap();
    let scene = sophia_engine::output_scene_snapshot_from_committed_in_view(
        output,
        1,
        viewport,
        &committed,
        display_list,
        None,
    )
    .unwrap();
    let plan = sophia_engine::build_output_head_plans(
        &scene,
        &[sophia_engine::HeadRenderTarget {
            head: sophia_engine::RenderHeadId::from_raw(output.raw()),
            output,
            target_generation: 1,
            native_size,
            scale: 1,
            refresh_millihz: 60_000,
            transform: sophia_protocol::OutputTransform::Normal,
            mapping: sophia_protocol::OutputHeadMapping::Fit,
        }],
    )
    .unwrap()
    .remove(0);
    sophia_engine::head_output_damage_snapshot(&plan)
}

/// A click at a root position, the way the pointer path delivers one.
fn root_click(x: f64, y: f64) -> sophia_protocol::InputEventPacket {
    sophia_protocol::InputEventPacket {
        serial: 1,
        seat: sophia_protocol::SeatId::from_raw(1),
        device: sophia_protocol::DeviceId::from_raw(2),
        time_msec: 1,
        kind: sophia_protocol::InputEventKind::PointerButton {
            button: 272,
            pressed: true,
        },
        global_position: Some(sophia_protocol::Point { x, y }),
        target_surface: None,
        local_position: None,
    }
}

fn centre_of(rect: Rect) -> (f64, f64) {
    (
        f64::from(rect.x) + f64::from(rect.width) / 2.0,
        f64::from(rect.y) + f64::from(rect.height) / 2.0,
    )
}

/// A pointer over a head that does not begin at the root origin has to reach
/// the window drawn there.
///
/// A head plan places pixels in its own framebuffer's coordinates, so a window
/// at root x=3050 on an output whose viewport starts at x=2560 is drawn at
/// x=490. Input arrives in root coordinates and is hit-tested against this
/// projection, so copying the head's geometry straight across moved every
/// window on a non-origin output left by that output's origin, and every click
/// on the second monitor landed on nothing.
#[test]
fn presented_projection_keeps_root_geometry_for_a_non_origin_head() {
    let target = surface(31, 1);
    let viewport = Rect {
        x: 2560,
        y: 32,
        width: 1920,
        height: 1048,
    };
    let root_geometry = Rect {
        x: 3050,
        y: 40,
        width: 940,
        height: 1032,
    };
    let presented = presented_head_frame(
        OutputId::from_raw(2),
        viewport,
        root_geometry,
        sophia_protocol::Size {
            width: 1920,
            height: 1080,
        },
        target,
    );

    // The head really does draw it somewhere else; that is not the bug.
    assert_ne!(
        presented.surfaces[0].geometry.x, root_geometry.x,
        "the head's own placement is output-local"
    );

    let layers = presented_input_layer_snapshots(&presented, &BTreeMap::new(), &[target]);
    assert_eq!(layers.len(), 1, "the retired surface stays eligible");

    let (x, y) = centre_of(root_geometry);
    let route = sophia_engine::hit_test_scene_surface_for_input(&root_click(x, y), &layers);
    assert_eq!(
        route.target_surface,
        Some(target),
        "a click inside the window on the second head must reach it"
    );
    // The client is handed the position within its own surface, which is the
    // click offset from where the layout put the window -- not from where this
    // head happens to draw it.
    assert_eq!(
        route.local_position,
        Some(sophia_protocol::Point { x: 470.0, y: 516.0 })
    );
}

/// The same, on a head whose framebuffer is not the size of its viewport.
///
/// A Fit head scales and letterboxes, so the head-local rectangle is neither
/// the root one nor a fixed offset from it. Carrying the root placement is
/// what makes this independent of the head's size; reconstructing it by
/// inverting the projection would land a few pixels off and drift with the
/// topology.
#[test]
fn presented_projection_keeps_root_geometry_on_a_scaled_head() {
    let target = surface(32, 1);
    let viewport = Rect {
        x: 2560,
        y: 32,
        width: 1920,
        height: 1048,
    };
    let root_geometry = Rect {
        x: 3050,
        y: 40,
        width: 940,
        height: 1032,
    };
    let presented = presented_head_frame(
        OutputId::from_raw(2),
        viewport,
        root_geometry,
        sophia_protocol::Size {
            width: 3840,
            height: 2160,
        },
        target,
    );

    assert_ne!(
        presented.surfaces[0].geometry.width, root_geometry.width,
        "a scaled head draws this window at a different size"
    );
    assert_eq!(
        presented.surfaces[0].logical_geometry, root_geometry,
        "the root placement travels unscaled"
    );

    let layers = presented_input_layer_snapshots(&presented, &BTreeMap::new(), &[target]);
    let (x, y) = centre_of(root_geometry);
    let route = sophia_engine::hit_test_scene_surface_for_input(&root_click(x, y), &layers);
    assert_eq!(route.target_surface, Some(target));
    // Identical to the unscaled head: a surface-local coordinate is a property
    // of the window, and the head's scale is not allowed to reach it.
    assert_eq!(
        route.local_position,
        Some(sophia_protocol::Point { x: 470.0, y: 516.0 })
    );

    // Just outside the window, still on this head: nothing to click.
    let route = sophia_engine::hit_test_scene_surface_for_input(
        &root_click(f64::from(viewport.x) + 8.0, y),
        &layers,
    );
    assert_eq!(route.target_surface, None);
}

/// Carrying the root placement does not widen what a head publishes.
///
/// A surface whose pixels retired but which has left the presentation order is
/// still not interactive. The rule this projection exists to enforce -- no
/// presented pixels, no input -- is unchanged by which rectangle it carries.
#[test]
fn presented_projection_still_requires_retired_membership() {
    let target = surface(33, 1);
    let presented = presented_head_frame(
        OutputId::from_raw(2),
        Rect {
            x: 2560,
            y: 32,
            width: 1920,
            height: 1048,
        },
        Rect {
            x: 3050,
            y: 40,
            width: 940,
            height: 1032,
        },
        sophia_protocol::Size {
            width: 1920,
            height: 1080,
        },
        target,
    );

    let layers = presented_input_layer_snapshots(&presented, &BTreeMap::new(), &[]);
    assert!(
        layers.is_empty(),
        "a surface outside the presentation order stays uninteractive"
    );
}
