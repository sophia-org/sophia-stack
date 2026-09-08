use std::fs;
use std::path::{Path, PathBuf};

use rustix::fs::Stat;

pub(super) fn same_node(left: &Stat, right: &Stat) -> bool {
    left.st_rdev == right.st_rdev && left.st_dev == right.st_dev && left.st_ino == right.st_ino
}

// Enumeration supplies candidates; only physical identity selects a device.
pub(super) fn unique_render_node(
    physical: &Path,
    candidates: impl IntoIterator<Item = (PathBuf, PathBuf)>,
) -> Result<PathBuf, String> {
    let mut matches = candidates
        .into_iter()
        .filter(|(_, parent)| parent == physical)
        .map(|(node, _)| node);
    let selected = matches.next().ok_or("render_node_missing")?;
    if matches.next().is_some() {
        return Err("render_node_ambiguous".into());
    }
    Ok(selected)
}

/// Unrelated disappearing nodes do not invalidate the selected device's own checks.
pub(super) fn render_candidates(directory: &Path) -> Result<Vec<(PathBuf, PathBuf)>, String> {
    let mut candidates = Vec::new();
    for entry in fs::read_dir(directory).map_err(|_| "device_identity_unavailable")? {
        let entry = entry.map_err(|_| "device_identity_unavailable")?;
        if entry.file_name().to_string_lossy().starts_with("renderD")
            && let Ok(node) = fs::canonicalize(entry.path())
            && let Ok(parent) = fs::canonicalize(node.join("device"))
        {
            candidates.push((node, parent));
        }
    }
    Ok(candidates)
}
