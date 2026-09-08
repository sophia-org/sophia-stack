use std::fs::{self, File};
use std::os::fd::AsFd;
use std::path::{Path, PathBuf};

use rustix::fs::{FileType, Mode, OFlags, Stat, fstat, major, minor, open};

mod identity;
use identity::{render_candidates, same_node, unique_render_node};

#[derive(Debug)]
pub(super) struct RenderDevice {
    pub path: PathBuf,
    pub major: u32,
    pub minor: u32,
    physical: PathBuf,
    stat: Stat,
}

fn device_stat(fd: impl AsFd) -> Result<Stat, String> {
    let stat = fstat(fd).map_err(|_| "device_metadata_unavailable")?;
    if FileType::from_raw_mode(stat.st_mode) != FileType::CharacterDevice
        || major(stat.st_rdev) != 226
    {
        return Err("not_drm_device".into());
    }
    Ok(stat)
}

fn sysfs_node(stat: &Stat) -> Result<PathBuf, String> {
    fs::canonicalize(format!(
        "/sys/dev/char/{}:{}",
        major(stat.st_rdev),
        minor(stat.st_rdev)
    ))
    .map_err(|_| "device_identity_unavailable".into())
}

fn physical_device(node: &Path) -> Result<PathBuf, String> {
    fs::canonicalize(node.join("device")).map_err(|_| "device_identity_unavailable".into())
}

fn is_render_node(node: &Path) -> bool {
    node.file_name()
        .and_then(|name| name.to_str())
        .and_then(|name| name.strip_prefix("renderD"))
        .is_some_and(|number| {
            !number.is_empty() && number.bytes().all(|byte| byte.is_ascii_digit())
        })
}

fn open_device(path: &Path) -> Result<File, String> {
    open(
        path,
        OFlags::RDWR | OFlags::CLOEXEC | OFlags::NONBLOCK | OFlags::NOFOLLOW,
        Mode::empty(),
    )
    .map(File::from)
    .map_err(|_| "device_open_failed".into())
}

pub(super) fn resolve(fd: impl AsFd) -> Result<RenderDevice, String> {
    let supplied = device_stat(&fd)?;
    let node = sysfs_node(&supplied)?;
    let physical = physical_device(&node)?;
    let name = node.file_name().ok_or("device_identity_unavailable")?;
    let original_path = Path::new("/dev/dri").join(name);
    let original = open_device(&original_path)?;
    if !same_node(&supplied, &device_stat(&original)?) {
        return Err("device_identity_changed".into());
    }

    let candidates = render_candidates(Path::new("/sys/class/drm"))?;
    let selected = unique_render_node(&physical, candidates)?;
    let path = Path::new("/dev/dri").join(selected.file_name().ok_or("render_node_missing")?);
    let reopened = open_device(&path)?;
    let stat = device_stat(&reopened)?;
    if !is_render_node(&selected)
        || sysfs_node(&stat)? != selected
        || physical_device(&selected)? != physical
        || sysfs_node(&supplied)? != node
        || !same_node(&supplied, &device_stat(open_device(&original_path)?)?)
    {
        return Err("device_identity_changed".into());
    }
    Ok(RenderDevice {
        path,
        major: major(stat.st_rdev),
        minor: minor(stat.st_rdev),
        physical,
        stat,
    })
}

pub(super) fn validate_explicit(path: &Path, selected: &RenderDevice) -> Result<(), String> {
    let path = fs::canonicalize(path).map_err(|_| "explicit_device_unavailable")?;
    let fd = open_device(&path)?;
    let stat = device_stat(&fd)?;
    let node = sysfs_node(&stat)?;
    if !is_render_node(&node)
        || physical_device(&node)? != selected.physical
        || major(stat.st_rdev) != selected.major
        || minor(stat.st_rdev) != selected.minor
        || !same_node(&stat, &selected.stat)
    {
        return Err("explicit_device_conflict".into());
    }
    Ok(())
}
