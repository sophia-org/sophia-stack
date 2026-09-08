use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::os::fd::{AsRawFd, OwnedFd};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use rustix::fs::{FileType, OFlags, Stat, fstat, major, minor, stat};

/// Reopen only the render node; the primary may require libseat authority.
pub(super) fn open_render_device(device: &File) -> io::Result<OwnedFd> {
    let held = fstat(device).map_err(|_| refused("held_device_metadata_unavailable"))?;
    validate_node_identity(&held, &[])?;
    let primary = fs::read_link(format!("/proc/self/fd/{}", device.as_raw_fd()))
        .map_err(|_| refused("held_device_path_unavailable"))?;
    let primary_name = primary
        .file_name()
        .filter(|name| node_name(name, "card") || node_name(name, "renderD"))
        .ok_or_else(|| refused("held_device_path_invalid"))?;
    let primary_before = stat(&primary).map_err(|_| refused("held_device_path_missing"))?;
    validate_node_identity(&held, &[primary_before])?;
    let physical = physical_device(primary_name, &held)?;

    let render = if node_name(primary_name, "renderD") {
        primary.clone()
    } else {
        let sibling = unique_render_node(Path::new("/sys/class/drm"), &physical)?;
        Path::new("/dev/dri").join(
            sibling
                .file_name()
                .ok_or_else(|| refused("render_node_missing"))?,
        )
    };
    let render_name = render
        .file_name()
        .ok_or_else(|| refused("render_node_missing"))?;
    let before = stat(&render).map_err(|_| refused("render_node_metadata_unavailable"))?;
    validate_node_identity(&before, &[])?;
    validate_physical_identity(&physical, &physical_device(render_name, &before)?)?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags((OFlags::CLOEXEC | OFlags::NOFOLLOW).bits() as i32)
        .open(&render)
        .map_err(|_| io::Error::other("render_node_open_failed"))?;
    let opened = fstat(&file).map_err(|_| refused("render_fd_metadata_unavailable"))?;
    let after = stat(&render).map_err(|_| refused("render_node_disappeared"))?;
    validate_node_identity(&before, &[opened, after])?;
    let primary_after = stat(&primary).map_err(|_| refused("held_device_path_disappeared"))?;
    validate_node_identity(&held, &[primary_after])?;
    validate_physical_identity(&physical, &physical_device(primary_name, &held)?)?;
    validate_physical_identity(&physical, &physical_device(render_name, &opened)?)?;
    Ok(file.into())
}

fn refused(reason: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, reason)
}

fn node_name(name: &OsStr, prefix: &str) -> bool {
    name.to_str()
        .and_then(|name| name.strip_prefix(prefix))
        .is_some_and(|suffix| {
            !suffix.is_empty() && suffix.bytes().all(|byte| byte.is_ascii_digit())
        })
}

pub(super) fn validate_node_identity(expected: &Stat, observed: &[Stat]) -> io::Result<()> {
    if std::iter::once(expected).chain(observed).any(|stat| {
        FileType::from_raw_mode(stat.st_mode) != FileType::CharacterDevice
            || major(stat.st_rdev) != 226
    }) {
        return Err(refused("not_drm_device"));
    }
    if observed.iter().any(|stat| {
        stat.st_dev != expected.st_dev
            || stat.st_ino != expected.st_ino
            || stat.st_rdev != expected.st_rdev
    }) {
        return Err(refused("device_node_identity_changed"));
    }
    Ok(())
}

pub(super) fn validate_physical_identity(expected: &Path, observed: &Path) -> io::Result<()> {
    if observed != expected {
        return Err(refused("physical_device_identity_changed"));
    }
    Ok(())
}

fn physical_device(name: &OsStr, stat: &Stat) -> io::Result<PathBuf> {
    let named = fs::canonicalize(Path::new("/sys/class/drm").join(name))
        .map_err(|_| refused("named_sysfs_node_missing"))?;
    let numbered = fs::canonicalize(format!(
        "/sys/dev/char/{}:{}",
        major(stat.st_rdev),
        minor(stat.st_rdev),
    ))
    .map_err(|_| refused("opened_sysfs_node_missing"))?;
    if named != numbered {
        return Err(refused("sysfs_node_identity_changed"));
    }
    fs::canonicalize(numbered.join("device"))
        .map_err(|_| refused("physical_device_identity_unavailable"))
}

/// Disappearing unrelated entries cannot select or replace the retained device.
pub(super) fn unique_render_node(root: &Path, physical: &Path) -> io::Result<PathBuf> {
    let mut selected = None;
    for entry in fs::read_dir(root).map_err(|_| refused("render_inventory_unavailable"))? {
        let Ok(entry) = entry else { continue };
        if !node_name(&entry.file_name(), "renderD") {
            continue;
        }
        let Ok(parent) = fs::canonicalize(entry.path().join("device")) else {
            continue;
        };
        if parent != physical {
            continue;
        }
        if selected.is_some() {
            return Err(refused("render_node_ambiguous"));
        }
        selected = Some(entry.path());
    }
    selected.ok_or_else(|| refused("render_node_missing"))
}
