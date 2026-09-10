//! Writing a desktop profile out as one file per authority.
//!
//! A profile is read as a whole and handed over in pieces: each authority
//! gets its own fragment, owner-only, so a client is given exactly the slice
//! it is allowed to see and nothing else.
//!
//! Each staged generation owns an immutable private directory. A replacement
//! gets distinct paths and cannot change fragments still held by its predecessor.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::{
    ConfigDigest, ConfigGeneration, DesktopAuthority, DesktopAuthorityCandidate,
    DesktopProfileError, DesktopProfileGeneration, prepare_desktop_profile_candidates,
};

#[derive(Debug)]
pub struct DesktopProfileFragments {
    pub generation: ConfigGeneration,
    pub digest: ConfigDigest,
    paths: BTreeMap<DesktopAuthority, PathBuf>,
    private_root: PathBuf,
    directory: PathBuf,
}

impl DesktopProfileFragments {
    pub fn path(&self, authority: DesktopAuthority) -> &Path {
        self.paths
            .get(&authority)
            .expect("every desktop authority has a staged fragment")
    }
}

impl Drop for DesktopProfileFragments {
    fn drop(&mut self) {
        for path in self.paths.values() {
            let _ = fs::remove_file(path);
        }
        let _ = fs::remove_dir(&self.directory);
    }
}

pub fn restage_desktop_profile(
    profile: &DesktopProfileGeneration,
    fragments: &DesktopProfileFragments,
) -> Result<DesktopProfileFragments, DesktopProfileError> {
    stage_desktop_profile(profile, &fragments.private_root)
}

/// The body of one staged fragment. Shared so a reload writes byte-for-byte
/// what startup wrote, which is what lets the digest be compared at all.
fn write_desktop_profile_fragment(
    file: &mut fs::File,
    profile: &DesktopProfileGeneration,
    authority: DesktopAuthority,
    candidate: &DesktopAuthorityCandidate,
) -> Result<(), DesktopProfileError> {
    use std::io::Write as _;

    writeln!(file, "schema 1")
        .and_then(|_| writeln!(file, "profile-generation {}", profile.generation.raw()))
        .and_then(|_| writeln!(file, "profile-digest \"{}\"", profile.digest))
        .and_then(|_| writeln!(file, "{} {{", authority.name()))
        .map_err(|error| DesktopProfileError::Stage(error.to_string()))?;
    for value in &candidate.values {
        writeln!(file, "  {}", value.encoded)
            .map_err(|error| DesktopProfileError::Stage(error.to_string()))?;
    }
    writeln!(file, "}}").map_err(|error| DesktopProfileError::Stage(error.to_string()))?;
    Ok(())
}

pub fn stage_desktop_profile(
    profile: &DesktopProfileGeneration,
    directory: &Path,
) -> Result<DesktopProfileFragments, DesktopProfileError> {
    use std::os::unix::fs::{
        DirBuilderExt as _, MetadataExt as _, OpenOptionsExt as _, PermissionsExt as _,
    };

    if !directory.is_absolute() {
        return Err(DesktopProfileError::Stage(
            "staging directory must be absolute".to_owned(),
        ));
    }
    let profile = super::lower_application_commands(profile)?;
    prepare_desktop_profile_candidates(&profile)?;
    let metadata = fs::symlink_metadata(directory)
        .map_err(|error| DesktopProfileError::Stage(error.to_string()))?;
    if metadata.file_type().is_symlink()
        || !metadata.is_dir()
        || metadata.uid() != rustix::process::geteuid().as_raw()
        || metadata.permissions().mode() & 0o077 != 0
    {
        return Err(DesktopProfileError::Stage(
            "staging directory must be a private directory owned by the effective user".to_owned(),
        ));
    }

    let private_root = directory;
    let directory = private_root.join(format!(
        "profile-{}-{}",
        profile.generation.raw(),
        profile.digest,
    ));
    fs::DirBuilder::new()
        .mode(0o700)
        .create(&directory)
        .map_err(|error| DesktopProfileError::Stage(error.to_string()))?;
    let mut fragments = DesktopProfileFragments {
        generation: profile.generation,
        digest: profile.digest,
        paths: BTreeMap::new(),
        private_root: private_root.to_path_buf(),
        directory,
    };
    for authority in DesktopAuthority::ALL {
        let candidate = profile
            .candidates
            .get(&authority)
            .expect("all desktop authority candidates validated before staging");
        let path = fragments
            .directory
            .join(format!("{}.profile.kdl", authority.name()));
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&path)
            .map_err(|error| DesktopProfileError::Stage(error.to_string()))?;
        fragments.paths.insert(authority, path);
        write_desktop_profile_fragment(&mut file, &profile, authority, candidate)?;
        file.sync_all()
            .map_err(|error| DesktopProfileError::Stage(error.to_string()))?;
        file.set_permissions(fs::Permissions::from_mode(0o600))
            .map_err(|error| DesktopProfileError::Stage(error.to_string()))?;
    }
    for directory in [&fragments.directory, &fragments.private_root] {
        fs::File::open(directory)
            .and_then(|directory| directory.sync_all())
            .map_err(|error| DesktopProfileError::Stage(error.to_string()))?;
    }
    Ok(fragments)
}
