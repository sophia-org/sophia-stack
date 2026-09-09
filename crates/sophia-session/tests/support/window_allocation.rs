use crate::live_session::window_allocation::window_allocation_rows;
use sophia_backend_live::LiveOutputAllocationPreference;
use sophia_protocol::{
    BufferSource, CommittedSurfaceState, OutputId, Rect, Region, Size, SurfaceId,
};
use std::collections::BTreeSet;

#[test]
fn window_preferences_follow_exact_placement_and_clear_on_ambiguity_or_withdrawal() {
    let surface = SurfaceId::new(7, 1);
    let area = Rect {
        x: 0,
        y: 0,
        width: 100,
        height: 100,
    };
    let left = OutputId::from_raw(1);
    let right = OutputId::from_raw(2);
    let bounds = [(left, area), (right, Rect { x: 100, ..area })];
    let preferences = [
        LiveOutputAllocationPreference {
            output: left,
            device_number: rustix::fs::makedev(226, 128),
            modifiers: vec![2, 3],
        },
        LiveOutputAllocationPreference {
            output: right,
            device_number: rustix::fs::makedev(226, 129),
            modifiers: vec![4],
        },
    ];
    let mut committed = CommittedSurfaceState::with_source(
        surface,
        1,
        area,
        BufferSource::CpuBuffer { handle: 1 },
        Size {
            width: 100,
            height: 100,
        },
        Region { rects: vec![] },
    );
    let mapped = BTreeSet::from([surface]);
    for (x, expected_device, expected_modifiers) in [(0, 128, vec![2, 3]), (100, 129, vec![4])] {
        committed.geometry.x = x;
        let rows = window_allocation_rows(&[committed.clone()], &mapped, &bounds, &preferences);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].surface, surface);
        assert_eq!(rows[0].device.minor, expected_device);
        assert_eq!(
            rows[0].formats[0].format,
            sophia_protocol::DRM_FORMAT_XRGB8888
        );
        assert_eq!(rows[0].formats[0].modifiers, expected_modifiers);
    }
    committed.geometry.x = 50;
    assert!(
        window_allocation_rows(&[committed.clone()], &mapped, &bounds, &preferences).is_empty()
    );
    committed.geometry.x = 0;
    assert!(
        window_allocation_rows(
            &[committed.clone()],
            &BTreeSet::new(),
            &bounds,
            &preferences
        )
        .is_empty()
    );
    assert!(
        window_allocation_rows(
            &[committed.clone()],
            &mapped,
            &[(left, area), (right, area)],
            &preferences
        )
        .is_empty()
    );
    assert!(window_allocation_rows(&[committed], &mapped, &bounds, &[]).is_empty());
}
