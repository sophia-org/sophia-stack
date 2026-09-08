use crate::LiveDmaBufImportFormat;
use std::collections::{BTreeMap, BTreeSet};

/// A movable surface must remain sampleable on every admitted rendering device.
/// Import support does not establish allocation or direct-scanout support.
pub fn common_dma_buf_import_formats(
    devices: &[Vec<LiveDmaBufImportFormat>],
) -> Vec<LiveDmaBufImportFormat> {
    // Matching tiled modifier numbers do not prove cross-device compatibility.
    // Until a transfer path is validated, multi-device clients get measured linear only.
    let cross_device = devices.len() > 1;
    let mut devices = devices.iter().map(|formats| {
        let mut inventory: BTreeMap<u32, BTreeSet<u64>> = BTreeMap::new();
        for row in formats {
            inventory
                .entry(row.format)
                .or_default()
                .extend(row.modifiers.iter().copied().filter(|modifier| {
                    *modifier != sophia_protocol::DRM_FORMAT_MOD_INVALID
                        && *modifier != 0x00ff_ffff_ffff_ffff
                        && (!cross_device || *modifier == 0)
                }));
        }
        inventory
    });
    let Some(mut common) = devices.next() else {
        return Vec::new();
    };
    for device in devices {
        common.retain(|format, modifiers| {
            let Some(other) = device.get(format) else {
                return false;
            };
            modifiers.retain(|modifier| other.contains(modifier));
            !modifiers.is_empty()
        });
    }
    common
        .into_iter()
        .filter(|(_, modifiers)| !modifiers.is_empty())
        .map(|(format, modifiers)| LiveDmaBufImportFormat {
            format,
            modifiers: modifiers.into_iter().collect(),
        })
        .collect()
}
