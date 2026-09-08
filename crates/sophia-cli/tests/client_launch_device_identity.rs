#[path = "../src/commands/client_launch/device/identity.rs"]
mod identity;

use std::fs::File;
use std::path::{Path, PathBuf};

use identity::{render_candidates, same_node, unique_render_node};
use rustix::fs::{Stat, fstat};

const DISCRETE: &str = "/sys/devices/pci0000:00/0000:03:00.0";
const INTEGRATED: &str = "/sys/devices/pci0000:00/0000:16:00.0";

fn candidate(physical: &str, node: &str) -> (PathBuf, PathBuf) {
    (Path::new(physical).join("drm").join(node), physical.into())
}

#[test]
fn same_vendor_devices_are_selected_by_exact_physical_parent() {
    let candidates = vec![
        candidate(INTEGRATED, "renderD128"),
        candidate(DISCRETE, "renderD129"),
    ];
    assert_eq!(
        unique_render_node(Path::new(DISCRETE), candidates.clone()).unwrap(),
        candidates[1].0
    );
    assert_eq!(
        unique_render_node(Path::new(INTEGRATED), candidates.clone()).unwrap(),
        candidates[0].0
    );
    assert_eq!(
        unique_render_node(Path::new(DISCRETE), candidates.into_iter().rev()).unwrap(),
        candidate(DISCRETE, "renderD129").0
    );
}

#[test]
fn render_node_renumbering_does_not_change_physical_selection() {
    for (integrated_node, discrete_node) in [
        ("renderD128", "renderD129"),
        ("renderD129", "renderD128"),
        ("renderD150", "renderD140"),
    ] {
        let candidates = [
            candidate(INTEGRATED, integrated_node),
            candidate(DISCRETE, discrete_node),
        ];
        assert_eq!(
            unique_render_node(Path::new(DISCRETE), candidates).unwrap(),
            candidate(DISCRETE, discrete_node).0
        );
    }
}

#[test]
fn multiple_render_nodes_for_one_physical_device_are_ambiguous() {
    let candidates = [
        candidate(DISCRETE, "renderD128"),
        candidate(INTEGRATED, "renderD129"),
        candidate(DISCRETE, "renderD130"),
    ];
    assert_eq!(
        unique_render_node(Path::new(DISCRETE), candidates),
        Err("render_node_ambiguous".into())
    );
    let duplicate = candidate(DISCRETE, "renderD128");
    assert_eq!(
        unique_render_node(Path::new(DISCRETE), [duplicate.clone(), duplicate]),
        Err("render_node_ambiguous".into())
    );
}

#[test]
fn missing_physical_device_never_falls_back_to_the_first_render_node() {
    assert_eq!(
        unique_render_node(Path::new(DISCRETE), []),
        Err("render_node_missing".into())
    );
    assert_eq!(
        unique_render_node(Path::new(DISCRETE), [candidate(INTEGRATED, "renderD128")]),
        Err("render_node_missing".into())
    );
    let descendant = format!("{DISCRETE}/child");
    assert_eq!(
        unique_render_node(Path::new(DISCRETE), [candidate(&descendant, "renderD128")]),
        Err("render_node_missing".into())
    );
}

fn metadata() -> Stat {
    fstat(File::open("/dev/null").unwrap()).unwrap()
}

#[test]
fn the_same_open_node_has_identical_device_filesystem_and_inode_identity() {
    let fd = File::open("/dev/null").unwrap();
    assert!(same_node(&fstat(&fd).unwrap(), &fstat(&fd).unwrap()));
}

#[test]
fn device_number_filesystem_and_inode_are_independently_required() {
    let original = metadata();
    let mut other_device = metadata();
    other_device.st_rdev ^= 1;
    assert!(!same_node(&original, &other_device));
    let mut other_filesystem = metadata();
    other_filesystem.st_dev ^= 1;
    assert!(!same_node(&original, &other_filesystem));
    let mut other_inode = metadata();
    other_inode.st_ino ^= 1;
    assert!(!same_node(&original, &other_inode));
}

#[test]
fn a_recreated_path_with_the_same_device_number_cannot_replace_the_held_node() {
    let original = metadata();
    let mut recreated = metadata();
    recreated.st_ino ^= 1;
    assert_eq!(original.st_rdev, recreated.st_rdev);
    assert_eq!(original.st_dev, recreated.st_dev);
    assert!(
        !same_node(&original, &recreated),
        "same rdev does not prove the path still names the opened inode"
    );
    recreated.st_ino = original.st_ino;
    recreated.st_dev ^= 1;
    assert_eq!(original.st_rdev, recreated.st_rdev);
    assert!(
        !same_node(&original, &recreated),
        "an inode number on another filesystem is not the held node"
    );
}

struct TempSysfs(PathBuf);

impl TempSysfs {
    fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(0);
        loop {
            let path = std::env::temp_dir().join(format!(
                "sophia-device-identity-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            match std::fs::create_dir(&path) {
                Ok(()) => return Self(path),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => panic!("create sysfs fixture: {error}"),
            }
        }
    }
}

impl Drop for TempSysfs {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn unrelated_broken_sysfs_entries_do_not_block_an_exact_target_or_supply_a_fallback() {
    use std::fs::{create_dir_all, remove_file};
    use std::os::unix::fs::symlink;
    let fixture = TempSysfs::new();
    let class = fixture.0.join("class/drm");
    let target_physical = fixture.0.join("devices/0000:03:00.0");
    let other_physical = fixture.0.join("devices/0000:16:00.0");
    let target_node = target_physical.join("drm/renderD137");
    let other_node = other_physical.join("drm/renderD130");
    let unbound_node = fixture.0.join("devices/unbound/drm/renderD129");
    for directory in [&class, &target_node, &other_node, &unbound_node] {
        create_dir_all(directory).unwrap();
    }
    symlink(&target_physical, target_node.join("device")).unwrap();
    symlink(&other_physical, other_node.join("device")).unwrap();
    symlink(&target_node, class.join("renderD137")).unwrap();
    symlink(&other_node, class.join("renderD130")).unwrap();
    symlink(&unbound_node, class.join("renderD129")).unwrap();
    symlink(fixture.0.join("missing"), class.join("renderD128")).unwrap();
    symlink(&other_node, class.join("card0")).unwrap();

    let target_physical = std::fs::canonicalize(target_physical).unwrap();
    let target_node = std::fs::canonicalize(target_node).unwrap();
    let candidates = render_candidates(&class).unwrap();
    assert_eq!(
        candidates.len(),
        2,
        "only complete render entries are candidates"
    );
    assert_eq!(
        unique_render_node(&target_physical, candidates).unwrap(),
        target_node
    );

    remove_file(target_node.join("device")).unwrap();
    let candidates = render_candidates(&class).unwrap();
    assert_eq!(
        candidates.len(),
        1,
        "an unrelated valid device remains discoverable"
    );
    assert_eq!(
        unique_render_node(&target_physical, candidates),
        Err("render_node_missing".into())
    );
    assert_eq!(
        render_candidates(&fixture.0.join("missing-directory")),
        Err("device_identity_unavailable".into())
    );
}
