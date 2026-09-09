use crate::live_session::window_allocation::window_allocation_rows;
use sophia_backend_live::{
    LiveOutputAllocationFormatPreference, LiveOutputAllocationPreference,
    LiveRenderDeviceNodeIdentity,
};
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
            identity: Some(LiveRenderDeviceNodeIdentity {
                device: 4,
                inode: 77,
                device_number: rustix::fs::makedev(226, 128),
            }),
            formats: vec![LiveOutputAllocationFormatPreference {
                format: sophia_protocol::DRM_FORMAT_XRGB8888,
                modifiers: vec![2, 3],
            }],
        },
        LiveOutputAllocationPreference {
            output: right,
            device_number: rustix::fs::makedev(226, 129),
            identity: None,
            formats: vec![LiveOutputAllocationFormatPreference {
                format: sophia_protocol::DRM_FORMAT_XRGB8888,
                modifiers: vec![4],
            }],
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
            rows[0].identity,
            if expected_device == 128 {
                Some(sophia_x_authority::XRenderDeviceIdentity {
                    device: 4,
                    inode: 77,
                    device_number: rustix::fs::makedev(226, 128),
                })
            } else {
                None
            }
        );
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

#[test]
fn window_preferences_preserve_independent_formats_and_never_invent_an_opaque_row() {
    use sophia_protocol::{DRM_FORMAT_ARGB8888 as AR24, DRM_FORMAT_XRGB8888 as XR24};
    use sophia_x_authority::XServerFrontendDmaBufImportFormat as Row;

    let surface = SurfaceId::new(7, 1);
    let output = OutputId::from_raw(1);
    let area = Rect {
        x: 0,
        y: 0,
        width: 100,
        height: 100,
    };
    let committed = CommittedSurfaceState::with_source(
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
    let mut preference = LiveOutputAllocationPreference {
        output,
        device_number: rustix::fs::makedev(226, 128),
        identity: Some(LiveRenderDeviceNodeIdentity {
            device: 4,
            inode: 77,
            device_number: rustix::fs::makedev(226, 128),
        }),
        formats: vec![
            LiveOutputAllocationFormatPreference {
                format: XR24,
                modifiers: vec![9, 2],
            },
            LiveOutputAllocationFormatPreference {
                format: AR24,
                modifiers: vec![7, 3, 0],
            },
        ],
    };
    let mapped = BTreeSet::from([surface]);
    let bounds = [(output, area)];
    let rows = |preference: &LiveOutputAllocationPreference| {
        window_allocation_rows(
            std::slice::from_ref(&committed),
            &mapped,
            &bounds,
            std::slice::from_ref(preference),
        )
    };
    let published = rows(&preference);
    assert_eq!(published.len(), 1);
    assert_eq!(
        published[0].formats,
        vec![
            Row {
                format: XR24,
                modifiers: vec![9, 2],
            },
            Row {
                format: AR24,
                modifiers: vec![7, 3, 0],
            },
        ]
    );

    // A plane can offer AR24 without offering XR24. The client's storage
    // format, including alpha, remains part of the allocation preference.
    preference.formats.remove(0);
    let published = rows(&preference);
    assert_eq!(published.len(), 1);
    assert_eq!(
        published[0].formats,
        vec![Row {
            format: AR24,
            modifiers: vec![7, 3, 0],
        }]
    );
    preference.formats.clear();
    assert!(rows(&preference).is_empty());
}
