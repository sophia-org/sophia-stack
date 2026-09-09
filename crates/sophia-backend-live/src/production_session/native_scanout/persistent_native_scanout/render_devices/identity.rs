use std::{
    fs::{self, File},
    os::fd::AsRawFd,
    path::{Path, PathBuf},
};

use rustix::fs::{FileType, Stat, fstat, major, minor, stat};

/// A reused device number must not associate a retained descriptor with a new GPU.
/// Metadata brackets sysfs resolution; missing or changed observations refuse advice.
pub(super) fn physical_device(file: &File) -> Option<(Stat, PathBuf)> {
    let held = fstat(file).ok()?;
    let path = fs::read_link(format!("/proc/self/fd/{}", file.as_raw_fd())).ok()?;
    let physical = resolve_physical_device(
        &held,
        &path,
        |path| stat(path).ok(),
        |path| fs::canonicalize(path).ok(),
    )?;
    Some((held, physical))
}

pub(super) fn resolve_physical_device(
    held: &Stat,
    path: &Path,
    mut metadata: impl FnMut(&Path) -> Option<Stat>,
    mut canonicalize: impl FnMut(&Path) -> Option<PathBuf>,
) -> Option<PathBuf> {
    let name = path.file_name()?.to_str()?;
    let suffix = name
        .strip_prefix("card")
        .or_else(|| name.strip_prefix("renderD"))?;
    if suffix.is_empty() || !suffix.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let same_node = |observed: &Stat| {
        FileType::from_raw_mode(observed.st_mode) == FileType::CharacterDevice
            && major(observed.st_rdev) == 226
            && observed.st_dev == held.st_dev
            && observed.st_ino == held.st_ino
            && observed.st_rdev == held.st_rdev
    };
    if !same_node(held) || !same_node(&metadata(path)?) {
        return None;
    }
    let named = Path::new("/sys/class/drm").join(name);
    let numbered = PathBuf::from(format!(
        "/sys/dev/char/{}:{}",
        major(held.st_rdev),
        minor(held.st_rdev),
    ));
    let node = canonicalize(&named)?;
    if canonicalize(&numbered)? != node {
        return None;
    }
    let physical = canonicalize(&named.join("device"))?;
    if canonicalize(&named)? != node
        || canonicalize(&numbered)? != node
        || canonicalize(&named.join("device"))? != physical
        || !same_node(&metadata(path)?)
    {
        return None;
    }
    Some(physical)
}
