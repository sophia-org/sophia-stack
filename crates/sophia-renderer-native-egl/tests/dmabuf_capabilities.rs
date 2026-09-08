#![cfg(feature = "gbm-platform")]

use std::fs::OpenOptions;

use sophia_renderer_native_egl::query_native_dmabuf_import_formats;

#[test]
#[ignore = "requires SOPHIA_TEST_RENDER_NODE naming an explicitly selected DRM render node"]
fn selected_device_reports_a_canonical_bounded_import_capability_table() {
    let path =
        std::env::var_os("SOPHIA_TEST_RENDER_NODE").expect("select a render node explicitly");
    let device = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .unwrap();
    let formats = query_native_dmabuf_import_formats(device).unwrap();
    assert!(
        !formats.is_empty(),
        "selected test device must support non-external imports"
    );
    assert!(formats.len() <= 512);
    assert!(
        formats
            .windows(2)
            .all(|pair| pair[0].format < pair[1].format)
    );
    assert!(
        formats
            .iter()
            .map(|entry| entry.modifiers.len())
            .sum::<usize>()
            <= 16_384
    );
    for entry in formats {
        assert!(!entry.modifiers.is_empty());
        assert!(entry.modifiers.len() <= 4096);
        assert!(entry.modifiers.windows(2).all(|pair| pair[0] < pair[1]));
        assert!(!entry.modifiers.contains(&u64::from(gbm::Modifier::Invalid)));
        assert!(!entry.modifiers.contains(&u64::MAX));
    }
}
