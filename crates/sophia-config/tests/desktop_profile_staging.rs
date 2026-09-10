use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use sophia_config::{
    ConfigGeneration, DesktopAuthority, DesktopProfileActivationKey, DesktopProfileError,
    DesktopProfileFragments, load_desktop_profile, restage_desktop_profile, stage_desktop_profile,
    validate_desktop_profile_fragments,
};

struct StagingRoot(PathBuf);

impl StagingRoot {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "sophia-profile-staging-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed),
        ));
        fs::create_dir(&path).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o700)).unwrap();
        Self(path)
    }

    fn child_count(&self) -> usize {
        fs::read_dir(&self.0).unwrap().count()
    }
}

impl Drop for StagingRoot {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn contents(fragments: &DesktopProfileFragments) -> Vec<Vec<u8>> {
    DesktopAuthority::ALL
        .into_iter()
        .map(|authority| fs::read(fragments.path(authority)).unwrap())
        .collect()
}

#[test]
fn dropping_a_declined_replacement_preserves_active_fragments_and_allows_retry() {
    let root = StagingRoot::new();
    let first = load_desktop_profile(None, ConfigGeneration::INITIAL).unwrap();
    let active = stage_desktop_profile(&first, &root.0).unwrap();
    let before = contents(&active);
    let second = load_desktop_profile(None, ConfigGeneration::from_raw(2)).unwrap();
    let replacement = restage_desktop_profile(&second, &active).unwrap();
    let replacement_path = replacement.path(DesktopAuthority::Policy).to_path_buf();
    assert_eq!(root.child_count(), 2);
    assert_eq!(contents(&active), before);
    validate_desktop_profile_fragments(&active, DesktopProfileActivationKey::from(&first)).unwrap();
    drop(replacement);
    assert!(!replacement_path.exists());
    assert_eq!(root.child_count(), 1);
    assert_eq!(contents(&active), before);

    let retried = restage_desktop_profile(&second, &active).unwrap();
    validate_desktop_profile_fragments(&retried, DesktopProfileActivationKey::from(&second))
        .unwrap();
    assert_eq!(contents(&active), before);
    drop(retried);
    drop(active);
    assert_eq!(root.child_count(), 0);
}

#[test]
fn repeated_reloads_are_siblings_and_owners_can_drop_in_any_order() {
    let root = StagingRoot::new();
    let first = load_desktop_profile(None, ConfigGeneration::INITIAL).unwrap();
    let first = stage_desktop_profile(&first, &root.0).unwrap();
    let second = load_desktop_profile(None, ConfigGeneration::from_raw(2)).unwrap();
    let second = restage_desktop_profile(&second, &first).unwrap();
    let third = load_desktop_profile(None, ConfigGeneration::from_raw(3)).unwrap();
    let third = restage_desktop_profile(&third, &second).unwrap();
    for fragments in [&first, &second, &third] {
        let directory = fragments.path(DesktopAuthority::Policy).parent().unwrap();
        assert_eq!(directory.parent(), Some(root.0.as_path()));
        assert_eq!(
            fs::metadata(directory).unwrap().permissions().mode() & 0o777,
            0o700
        );
        assert_eq!(
            fs::read_dir(directory).unwrap().count(),
            DesktopAuthority::ALL.len()
        );
        for authority in DesktopAuthority::ALL {
            let path = fragments.path(authority);
            assert_eq!(path.parent(), Some(directory));
            assert_eq!(
                fs::metadata(path).unwrap().permissions().mode() & 0o777,
                0o600
            );
        }
    }
    let first_bytes = contents(&first);
    let third_bytes = contents(&third);
    drop(second);
    assert_eq!(root.child_count(), 2);
    assert_eq!(contents(&first), first_bytes);
    assert_eq!(contents(&third), third_bytes);
    drop(first);
    assert_eq!(root.child_count(), 1);
    assert_eq!(contents(&third), third_bytes);
    drop(third);
    assert_eq!(root.child_count(), 0);
}

#[test]
fn a_failed_restage_never_changes_previous_fragments_or_claims_a_colliding_directory() {
    let root = StagingRoot::new();
    let first = load_desktop_profile(None, ConfigGeneration::INITIAL).unwrap();
    let active = stage_desktop_profile(&first, &root.0).unwrap();
    let before = contents(&active);
    assert!(matches!(
        stage_desktop_profile(&first, &root.0),
        Err(DesktopProfileError::Stage(_)),
    ));
    assert!(matches!(
        restage_desktop_profile(&first, &active),
        Err(DesktopProfileError::Stage(_)),
    ));
    assert_eq!(contents(&active), before);
    assert_eq!(root.child_count(), 1);

    let mut invalid = load_desktop_profile(None, ConfigGeneration::from_raw(2)).unwrap();
    invalid
        .candidates
        .get_mut(&DesktopAuthority::Shortcut)
        .unwrap()
        .values[0]
        .encoded = "bind \"Super+q\" \"close-window\"".to_owned();
    assert!(restage_desktop_profile(&invalid, &active).is_err());
    assert_eq!(contents(&active), before);
    assert_eq!(root.child_count(), 1);

    fs::set_permissions(&root.0, fs::Permissions::from_mode(0o755)).unwrap();
    let valid = load_desktop_profile(None, ConfigGeneration::from_raw(2)).unwrap();
    assert!(matches!(
        restage_desktop_profile(&valid, &active),
        Err(DesktopProfileError::Stage(_)),
    ));
    fs::set_permissions(&root.0, fs::Permissions::from_mode(0o700)).unwrap();
    assert_eq!(contents(&active), before);
    assert_eq!(root.child_count(), 1);
    drop(active);
    assert_eq!(root.child_count(), 0);
}

#[test]
fn fragment_cleanup_removes_only_the_files_it_owns() {
    let root = StagingRoot::new();
    let profile = load_desktop_profile(None, ConfigGeneration::INITIAL).unwrap();
    let fragments = stage_desktop_profile(&profile, &root.0).unwrap();
    let directory = fragments
        .path(DesktopAuthority::Policy)
        .parent()
        .unwrap()
        .to_path_buf();
    let unowned = directory.join("unowned-marker");
    fs::write(&unowned, b"must survive bounded cleanup").unwrap();
    let paths = DesktopAuthority::ALL
        .into_iter()
        .map(|authority| fragments.path(authority).to_path_buf())
        .collect::<Vec<_>>();
    drop(fragments);
    assert!(paths.iter().all(|path| !path.exists()));
    assert_eq!(fs::read(&unowned).unwrap(), b"must survive bounded cleanup");
    assert_eq!(fs::read_dir(&directory).unwrap().count(), 1);
    fs::remove_file(unowned).unwrap();
    fs::remove_dir(directory).unwrap();
    assert_eq!(root.child_count(), 0);
}
