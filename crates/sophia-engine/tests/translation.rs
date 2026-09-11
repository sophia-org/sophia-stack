mod support;

use sophia_engine::TranslationTimeline;
use sophia_protocol::{LayerSnapshot, LayerTranslation, OutputId, Region};

fn layer(index: u32, local: i32, camera: i32) -> LayerSnapshot {
    let mut layer = support::test_layer(index, index, local + camera, Region::empty());
    layer.output = Some(OutputId::from_raw(1));
    layer.translation = Some(LayerTranslation {
        connection_epoch: 1,
        group: 1,
        x: camera,
        y: 0,
    });
    layer
}

fn x(timeline: &TranslationTimeline, layer: &LayerSnapshot, time: f64) -> i32 {
    timeline
        .geometry(layer.surface, layer.output.unwrap(), layer.geometry, time)
        .x
}

#[test]
fn camera_is_shared_and_retarget_does_not_jump_or_restart_on_identical_requests() {
    let mut timeline = TranslationTimeline::default();
    timeline.replace_targets(&[layer(1, 0, 0), layer(2, 1000, 0)], 0.0);
    let targets = [layer(1, 0, -1000), layer(2, 1000, -1000)];
    timeline.replace_targets(&targets, 1.0);
    assert_eq!(x(&timeline, &targets[0], 1.0), 0);
    let midway = x(&timeline, &targets[0], 1.05);
    assert!(midway > -1000 && midway < 0);
    assert_eq!(x(&timeline, &targets[1], 1.05) - midway, 1000);
    let expected = x(&timeline, &targets[0], 1.1);
    timeline.replace_targets(&targets, 1.05);
    assert_eq!(x(&timeline, &targets[0], 1.1), expected);
    let reversed = [layer(1, 0, 0), layer(2, 1000, 0)];
    timeline.replace_targets(&reversed, 1.05);
    assert_eq!(x(&timeline, &reversed[0], 1.05), midway);
    assert_eq!(x(&timeline, &reversed[0], 3.1), 0);
    assert!(!timeline.active(3.1));
}

#[test]
fn insertion_and_member_motion_share_camera_and_separate_toward_targets() {
    let mut timeline = TranslationTimeline::default();
    timeline.replace_targets(&[layer(1, 0, 0), layer(2, 1000, 0)], 0.0);
    let targets = [
        layer(1, 0, -1000),
        layer(2, 2000, -1000),
        layer(3, 1000, -1000),
    ];
    timeline.replace_targets(&targets, 1.0);
    assert_eq!(x(&timeline, &targets[0], 1.0), 0);
    assert_eq!(x(&timeline, &targets[1], 1.0), 1000);
    assert_eq!(x(&timeline, &targets[2], 1.0), 1000);
    assert!(x(&timeline, &targets[1], 1.1) > x(&timeline, &targets[2], 1.1));
    assert_eq!(x(&timeline, &targets[1], 3.1), 1000);
}

#[test]
fn epoch_output_size_and_motion_off_bound_presentation_state() {
    let mut timeline = TranslationTimeline::default();
    timeline.replace_targets(&[layer(1, 0, 0)], 0.0);
    let mut target = layer(1, 0, -1000);
    timeline.replace_targets(std::slice::from_ref(&target), 1.0);
    assert!(timeline.active_on(OutputId::from_raw(1), 1.05));
    assert!(!timeline.active_on(OutputId::from_raw(2), 1.05));
    assert_eq!(
        timeline.geometry(target.surface, OutputId::from_raw(2), target.geometry, 1.05),
        target.geometry
    );
    let resized = sophia_protocol::Rect {
        width: 200,
        ..target.geometry
    };
    assert_eq!(
        timeline.geometry(target.surface, OutputId::from_raw(1), resized, 1.05),
        resized
    );
    target.translation.as_mut().unwrap().connection_epoch = 2;
    timeline.replace_targets(std::slice::from_ref(&target), 1.05);
    assert_eq!(x(&timeline, &target, 1.05), -1000);
    target.geometry.x = 0;
    target.translation.as_mut().unwrap().x = 0;
    timeline.replace_targets(std::slice::from_ref(&target), 2.0);
    timeline.set_enabled(false);
    assert_eq!(x(&timeline, &target, 2.0), 0);
    assert!(!timeline.active(2.0));
    timeline.replace_targets(&[], 3.0);
    assert_eq!(x(&timeline, &target, 3.0), 0);
}

#[test]
fn frame_snapshot_carries_presented_geometry_and_chrome_without_changing_pixels() {
    use sophia_engine::*;
    use sophia_protocol::*;
    let mut timeline = TranslationTimeline::default();
    timeline.replace_targets(&[layer(1, 100, 0)], 0.0);
    let target = layer(1, 100, -50);
    timeline.replace_targets(std::slice::from_ref(&target), 1.0);
    let committed = CommittedSurfaceState::from_layer_snapshot(&target);
    let output = target.output.unwrap();
    let list = surface_chrome_display_list(
        output,
        &[target.surface],
        std::slice::from_ref(&committed),
        Some(target.surface),
        SurfaceChromeStyle::default(),
    )
    .unwrap();
    let before_borders = list.borders().collect::<Vec<_>>();
    let (presented, list) = timeline.project(output, std::slice::from_ref(&committed), list, 1.04);
    let delta = presented[0].geometry.x - committed.geometry.x;
    assert!(delta > 0 && delta < 50);
    assert_eq!(presented[0].content, committed.content);
    assert_eq!(
        presented[0].committed_generation,
        committed.committed_generation
    );
    for (before, after) in before_borders.iter().zip(list.borders()) {
        assert_eq!(after.outer.x - before.outer.x, delta);
        assert_eq!(after.inner.x - before.inner.x, delta);
    }
    let snapshot = output_frame_damage_snapshot(
        HeadlessOutput {
            id: output,
            size: Size {
                width: 800,
                height: 600,
            },
            scale: 1,
        },
        list,
        &presented,
        None,
    )
    .unwrap();
    assert_eq!(snapshot.surfaces[0].geometry, presented[0].geometry);
    assert_eq!(snapshot.surfaces[0].buffer, committed.buffer());
    assert_eq!(snapshot.surfaces[0].source_size, target.source_size);
}

#[test]
fn hidden_strip_returns_without_motion_leaking_into_a_new_edge_gap() {
    use sophia_engine::*;
    use sophia_protocol::*;

    let output = OutputId::from_raw(1);
    let mut edge = layer(1, 0, 0);
    edge.translation = None;
    edge.geometry = Rect {
        x: 0,
        y: 32,
        width: 1600,
        height: 968,
    };
    let mut targets = vec![layer(2, 1584, 8), layer(3, 2372, 8), layer(1, 8, 8)];
    for target in &mut targets {
        target.geometry.y = 40;
        target.geometry.height = 952;
        target.geometry.width = if target.surface == edge.surface {
            1568
        } else {
            780
        };
        target.source_size = Size {
            width: target.geometry.width,
            height: target.geometry.height,
        };
    }
    let mut timeline = TranslationTimeline::default();
    timeline.replace_targets(&[layer(2, 796, 8), layer(3, 1584, 8)], 0.0);
    // A presented edge pane replaces the strip. Returning panes must not
    // inherit positions from when they were concealed behind its pixels.
    timeline.replace_targets(std::slice::from_ref(&edge), 0.5);
    timeline.replace_targets(&targets, 1.0);
    let committed = targets
        .iter()
        .map(CommittedSurfaceState::from_layer_snapshot)
        .collect::<Vec<_>>();
    let order = targets
        .iter()
        .map(|target| target.surface)
        .collect::<Vec<_>>();
    for time in [1.0, 1.016, 1.05, 1.1, 1.25, 1.5, 3.1] {
        let list = surface_chrome_display_list(
            output,
            &order,
            &committed,
            Some(edge.surface),
            SurfaceChromeStyle::default(),
        )
        .unwrap();
        let (presented, list) = timeline.project(output, &committed, list, time);
        let scene = output_scene_snapshot_from_committed_in_view(
            output,
            1,
            Rect {
                x: 0,
                y: 0,
                width: 1600,
                height: 1000,
            },
            &presented,
            list,
            None,
        )
        .unwrap();
        assert!(!timeline.active(time));
        // The gap is x=1584..1592. A deliberate resting peek beyond it is
        // allowed, but no returning neighbor may sweep across this gap.
        assert!(
            scene
                .surfaces
                .iter()
                .all(|surface| !(surface.geometry.x <= 1590
                    && 1590 < surface.geometry.x + surface.geometry.width)),
            "gap exposed at {time}"
        );
        let top = scene
            .display_list
            .commands
            .iter()
            .rev()
            .find_map(|command| match command {
                CompositorDisplayCommand::Surface { surface } => scene
                    .surfaces
                    .iter()
                    .find(|state| {
                        state.surface == *surface
                            && state.geometry.x <= 800
                            && 800 < state.geometry.x + state.geometry.width
                    })
                    .map(|state| state.surface),
                _ => None,
            });
        assert_eq!(top, Some(edge.surface));
    }
}
