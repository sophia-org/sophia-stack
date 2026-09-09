#![cfg(feature = "drm-hotplug")]

use rustix::fs::Stat;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

#[path = "../src/production_session/native_scanout/persistent_native_scanout/render_devices/identity.rs"]
mod identity;

fn held(minor: u32) -> Stat {
    let mut metadata = rustix::fs::fstat(File::open("/dev/null").unwrap()).unwrap();
    metadata.st_rdev = rustix::fs::makedev(226, minor);
    metadata
}

fn observations(name: &str) -> [Option<PathBuf>; 6] {
    let node = PathBuf::from(format!("/sys/devices/gpu/drm/{name}"));
    let physical = PathBuf::from("/sys/devices/gpu");
    [
        Some(node.clone()),
        Some(node.clone()),
        Some(physical.clone()),
        Some(node.clone()),
        Some(node),
        Some(physical),
    ]
}

fn resolve(
    name: &str,
    held: &Stat,
    metadata: [Option<Stat>; 2],
    resolved: [Option<PathBuf>; 6],
) -> (Option<PathBuf>, usize, usize) {
    let path = PathBuf::from(format!("/dev/dri/{name}"));
    let named = PathBuf::from(format!("/sys/class/drm/{name}"));
    let numbered = PathBuf::from(format!(
        "/sys/dev/char/226:{}",
        rustix::fs::minor(held.st_rdev)
    ));
    let expected_paths = [
        &named,
        &numbered,
        &named.join("device"),
        &named,
        &numbered,
        &named.join("device"),
    ];
    let mut stats = 0;
    let mut paths = 0;
    let result = identity::resolve_physical_device(
        held,
        &path,
        |requested| {
            assert_eq!(requested, path);
            let observed = metadata[stats];
            stats += 1;
            observed
        },
        |requested| {
            assert_eq!(requested, expected_paths[paths]);
            let observed = resolved[paths].clone();
            paths += 1;
            observed
        },
    );
    (result, stats, paths)
}

#[test]
fn stable_card_and_render_nodes_resolve_the_same_physical_device() {
    for (name, minor) in [("card0", 0), ("renderD128", 128)] {
        let held = held(minor);
        let (result, stats, paths) = resolve(name, &held, [Some(held); 2], observations(name));
        assert_eq!(result, Some(PathBuf::from("/sys/devices/gpu")));
        assert_eq!(
            (stats, paths),
            (2, 6),
            "both sides of resolution are observed"
        );
    }
}

#[test]
fn reused_device_numbers_cannot_reidentify_a_held_card_or_render_descriptor() {
    for (name, minor) in [("card0", 0), ("renderD128", 128)] {
        let held = held(minor);
        for phase in 0..2 {
            for field in 0..4 {
                let mut changed = held;
                match field {
                    0 => changed.st_ino ^= 1,
                    1 => changed.st_dev ^= 1,
                    2 => changed.st_rdev ^= 1,
                    3 => changed.st_mode = 0,
                    _ => unreachable!(),
                }
                let mut metadata = [Some(held); 2];
                metadata[phase] = Some(changed);
                let (result, _, paths) = resolve(name, &held, metadata, observations(name));
                assert_eq!(result, None, "{name}: phase {phase}, changed field {field}");
                if phase == 0 {
                    assert_eq!(
                        paths, 0,
                        "a replaced node must not consult its new sysfs occupant"
                    );
                }
            }
            let mut metadata = [Some(held); 2];
            metadata[phase] = None;
            assert_eq!(resolve(name, &held, metadata, observations(name)).0, None);
        }
    }
}

#[test]
fn missing_or_changed_sysfs_observations_refuse_the_association() {
    for (name, minor) in [("card0", 0), ("renderD128", 128)] {
        let held = held(minor);
        for phase in 0..6 {
            for replacement in [None, Some(PathBuf::from("/sys/devices/another-gpu"))] {
                let mut resolved = observations(name);
                resolved[phase] = replacement;
                assert_eq!(
                    resolve(name, &held, [Some(held); 2], resolved).0,
                    None,
                    "{name}: changed sysfs observation {phase}"
                );
            }
        }
    }
}

#[test]
fn non_drm_or_deleted_descriptors_never_supply_identity_evidence() {
    let mut invalid = held(128);
    invalid.st_rdev = rustix::fs::makedev(1, 3);
    for (metadata, path) in [
        (invalid, Path::new("/dev/dri/renderD128")),
        (held(128), Path::new("/dev/dri/renderD128 (deleted)")),
        (held(128), Path::new("/dev/dri/renderD")),
    ] {
        assert_eq!(
            identity::resolve_physical_device(
                &metadata,
                path,
                |_| panic!("invalid held descriptor must not consult the current node"),
                |_| panic!("invalid held descriptor must not consult sysfs"),
            ),
            None
        );
    }
    assert!(identity::physical_device(&File::open("/dev/null").unwrap()).is_none());
}
