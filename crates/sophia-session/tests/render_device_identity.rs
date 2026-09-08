use std::fs::{self, File};
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

#[path = "../src/live_session/x_frontend/render_device.rs"]
mod render_device;

use render_device::{
    open_render_device, unique_render_node, validate_node_identity, validate_physical_identity,
};

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "sophia-render-provider-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed),
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }

    fn physical(&self, name: &str) -> PathBuf {
        let path = self.0.join(name);
        fs::create_dir(&path).unwrap();
        path.canonicalize().unwrap()
    }

    fn render(&self, name: &str, physical: &Path) -> PathBuf {
        let node = self.0.join(name);
        fs::create_dir(&node).unwrap();
        symlink(physical, node.join("device")).unwrap();
        node
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn drm_facts(mut stat: rustix::fs::Stat) -> rustix::fs::Stat {
    // Passive facts only: tests neither create a DRM node nor open a GPU.
    stat.st_mode = rustix::fs::FileType::CharacterDevice.as_raw_mode();
    stat.st_rdev = rustix::fs::makedev(226, 128);
    stat
}

#[test]
fn identical_node_facts_pass_but_each_identity_component_is_required() {
    let file = File::open("/dev/null").unwrap();
    let original = drm_facts(rustix::fs::fstat(&file).unwrap());
    validate_node_identity(&original, &[original, original]).unwrap();
    for phase in 0..2 {
        for field in 0..3 {
            let mut observed = [original, original];
            match field {
                0 => observed[phase].st_dev ^= 1,
                1 => observed[phase].st_ino ^= 1,
                2 => observed[phase].st_rdev = rustix::fs::makedev(226, 129),
                _ => unreachable!(),
            }
            assert_eq!(
                validate_node_identity(&original, &observed)
                    .unwrap_err()
                    .to_string(),
                "device_node_identity_changed",
                "phase {phase}, field {field}",
            );
        }
    }
    let mut regular = original;
    regular.st_mode = rustix::fs::FileType::RegularFile.as_raw_mode();
    assert_eq!(
        validate_node_identity(&original, &[regular])
            .unwrap_err()
            .to_string(),
        "not_drm_device"
    );
}

#[test]
fn pathname_replacement_is_refused_even_with_the_same_device_number() {
    let fixture = Fixture::new();
    let path = fixture.0.join("renderD128");
    let held = File::create(&path).unwrap();
    let original = drm_facts(rustix::fs::fstat(&held).unwrap());
    fs::remove_file(&path).unwrap();
    let replacement = File::create(&path).unwrap();
    let current = drm_facts(rustix::fs::fstat(&replacement).unwrap());
    assert_eq!(original.st_rdev, current.st_rdev);
    assert_ne!(original.st_ino, current.st_ino);
    assert_eq!(
        validate_node_identity(&original, &[current])
            .unwrap_err()
            .to_string(),
        "device_node_identity_changed"
    );
}

#[test]
fn physical_identity_must_match_independently_of_node_metadata() {
    let retained = Path::new("/sys/devices/physical-a");
    validate_physical_identity(retained, retained).unwrap();
    assert_eq!(
        validate_physical_identity(retained, Path::new("/sys/devices/physical-b"))
            .unwrap_err()
            .to_string(),
        "physical_device_identity_changed",
    );
}

#[test]
fn sibling_selection_ignores_unrelated_disappearing_nodes_and_has_no_ordinal_fallback() {
    let fixture = Fixture::new();
    let physical = fixture.physical("physical-a");
    let other = fixture.physical("physical-b");
    for index in 0..80 {
        fixture.render(&format!("renderD{}", 128 + index), &other);
    }
    symlink(fixture.0.join("gone"), fixture.0.join("renderD250")).unwrap();
    fs::create_dir(fixture.0.join("renderD251")).unwrap();
    fixture.render("renderD252-invalid", &physical);
    let selected = fixture.render("renderD300", &physical);
    assert_eq!(unique_render_node(&fixture.0, &physical).unwrap(), selected);
    fs::remove_dir_all(&selected).unwrap();
    assert_eq!(
        unique_render_node(&fixture.0, &physical)
            .unwrap_err()
            .to_string(),
        "render_node_missing"
    );
}

#[test]
fn ambiguous_siblings_fail_instead_of_choosing_one() {
    let fixture = Fixture::new();
    let physical = fixture.physical("physical");
    fixture.render("renderD128", &physical);
    fixture.render("renderD129", &physical);
    assert_eq!(
        unique_render_node(&fixture.0, &physical)
            .unwrap_err()
            .to_string(),
        "render_node_ambiguous"
    );
}

#[test]
fn a_non_drm_held_descriptor_is_refused_without_closing_it() {
    let held = File::open("/dev/null").unwrap();
    let before = rustix::fs::fstat(&held).unwrap();
    assert_eq!(
        open_render_device(&held).unwrap_err().to_string(),
        "not_drm_device"
    );
    let after = rustix::fs::fstat(&held).unwrap();
    assert_eq!(
        (before.st_dev, before.st_ino, before.st_rdev),
        (after.st_dev, after.st_ino, after.st_rdev)
    );
}
