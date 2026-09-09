use super::*;
use std::fs::File;
use std::io;

mod identity;

/// Identity of the retained render node after physical card-to-render mapping.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LiveRenderDeviceNodeIdentity {
    pub device: u64,
    pub inode: u64,
    pub device_number: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveOutputAllocationPreference {
    pub output: OutputId,
    pub device_number: u64,
    pub identity: Option<LiveRenderDeviceNodeIdentity>,
    pub modifiers: Vec<u64>,
}

pub(super) struct LiveRenderDeviceState {
    head_modifiers: Vec<Vec<u64>>,
    group_devices: Vec<Option<LiveRenderDeviceNodeIdentity>>,
    pub(super) generation: u64,
    pub(super) pending: BTreeMap<usize, u64>,
    pub(super) applied: BTreeMap<usize, u64>,
}

impl LiveRenderDeviceState {
    pub(super) fn new(head_modifiers: Vec<Vec<u64>>) -> Self {
        Self {
            head_modifiers,
            group_devices: Vec::new(),
            generation: 0,
            pending: BTreeMap::new(),
            applied: BTreeMap::new(),
        }
    }
}

fn physical_device(file: &File) -> Option<(LiveRenderDeviceNodeIdentity, std::path::PathBuf)> {
    let (metadata, physical) = identity::physical_device(file)?;
    Some((
        LiveRenderDeviceNodeIdentity {
            device: metadata.st_dev,
            inode: metadata.st_ino,
            device_number: metadata.st_rdev,
        },
        physical,
    ))
}

impl LiveProductionNativeScanout {
    pub(super) fn refresh_allocation_devices(&mut self) {
        let render_devices = self
            .image_import_devices
            .iter()
            .filter_map(physical_device)
            .collect::<Vec<_>>();
        self.render_devices.group_devices = self
            .groups
            .iter()
            .map(|group| {
                let card = group.session.card().try_clone_file().ok()?;
                let (_, physical) = physical_device(&card)?;
                let mut matching = render_devices.iter().filter(|(_, path)| *path == physical);
                let device = matching.next()?.0;
                matching.next().is_none().then_some(device)
            })
            .collect();
    }

    /// Cached plane preferences. Mirrors offer no single-device flip preference.
    pub fn output_allocation_preferences(&self) -> Vec<LiveOutputAllocationPreference> {
        self.logical_outputs
            .iter()
            .filter_map(|output| {
                let indices = self.head_indices(output.id);
                if indices.len() != 1 {
                    return None;
                }
                let index = indices[0];
                let head = &self.heads[index];
                if !head.enabled {
                    return None;
                }
                let identity = self
                    .render_devices
                    .group_devices
                    .get(head.group)
                    .copied()
                    .flatten()?;
                Some(LiveOutputAllocationPreference {
                    output: output.id,
                    device_number: identity.device_number,
                    identity: Some(identity),
                    modifiers: self.render_devices.head_modifiers.get(index)?.clone(),
                })
            })
            .collect()
    }

    /// Replaces source-device candidates without replacing output or image ownership.
    pub fn request_image_import_inventory(
        &mut self,
        generation: u64,
        devices: Vec<File>,
    ) -> io::Result<()> {
        if generation == 0 || generation <= self.render_devices.generation {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "stale render inventory",
            ));
        }
        if devices.len() > 16 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "render inventory capacity",
            ));
        }
        self.image_import_devices = devices;
        self.render_devices.generation = generation;
        self.refresh_allocation_devices();
        Ok(())
    }

    pub fn image_import_inventory_generation(&self) -> u64 {
        self.render_devices.generation
    }

    /// Polls bounded worker commands; queued renders and retained images stay owned.
    pub fn poll_image_import_inventory(&mut self) -> io::Result<bool> {
        let generation = self.render_devices.generation;
        if generation == 0 {
            return Ok(true);
        }
        let shared = Self::shared_renderer_worker_enabled();
        let mut visited = BTreeSet::new();
        let mut complete = true;
        for index in 0..self.heads.len() {
            if !self.heads[index].enabled || !self.exporters[index].worker_enabled() {
                continue;
            }
            let key = if shared {
                self.heads[index].group
            } else {
                index
            };
            if !visited.insert(key) {
                continue;
            }
            if self.render_devices.pending.contains_key(&key) {
                let result = if shared {
                    self.groups[key]
                        .renderer_core
                        .as_ref()
                        .ok_or_else(|| io::Error::other("renderer core unavailable"))?
                        .poll_image_import_device_replacement()
                } else {
                    self.exporters[index].poll_image_import_device_replacement()
                };
                match result {
                    Ok(Some(applied)) => {
                        self.render_devices.pending.remove(&key);
                        self.render_devices.applied.insert(key, applied);
                    }
                    Ok(None) => {
                        complete = false;
                        continue;
                    }
                    Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                        self.render_devices.pending.remove(&key);
                        complete = false;
                        continue;
                    }
                    Err(error) => {
                        self.render_devices.pending.remove(&key);
                        return Err(error);
                    }
                }
            }
            if self.render_devices.applied.get(&key) == Some(&generation) {
                continue;
            }
            let devices = self.image_import_device_fds()?;
            let requested = if shared {
                self.groups[key]
                    .renderer_core
                    .as_ref()
                    .ok_or_else(|| io::Error::other("renderer core unavailable"))?
                    .request_image_import_device_replacement(generation, devices)
            } else {
                self.exporters[index].request_image_import_device_replacement(generation, devices)
            };
            match requested {
                Ok(()) => {
                    self.render_devices.pending.insert(key, generation);
                }
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
                Err(error) => return Err(error),
            }
            complete = false;
        }
        Ok(complete)
    }
}
