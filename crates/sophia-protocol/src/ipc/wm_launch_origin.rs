//! Opaque placement bookmarks use the existing uncounted extension envelope.
use std::collections::BTreeSet;

use super::{IpcCodecError, WmV1ProjectionChunk, WmV1SnapshotChunk, WmV1SnapshotTransfer};
use crate::{POLICY_MAX_SURFACES, PolicyLaunchContext, SurfaceId};

pub const PROJECTION_LAUNCH_CONTEXT_RECORD_KIND: u16 = 0xff05;
pub const SNAPSHOT_LAUNCH_ORIGIN_RECORD_KIND: u16 = 0xff06;
pub const LAUNCH_CONTEXT_RECORD_LEN: usize = 24;

fn invalid() -> IpcCodecError {
    IpcCodecError::InvalidEnum {
        field: "launch_origin",
        value: 0,
    }
}

pub fn encode_wm_launch_context_records(
    records: &[PolicyLaunchContext],
) -> Result<Vec<u8>, IpcCodecError> {
    if records.len() > POLICY_MAX_SURFACES {
        return Err(invalid());
    }
    let mut seen = BTreeSet::new();
    let mut bytes = Vec::with_capacity(records.len() * LAUNCH_CONTEXT_RECORD_LEN);
    for record in records {
        if !record.surface.is_valid()
            || record.epoch == 0
            || record.token == 0
            || !seen.insert(record.surface)
        {
            return Err(invalid());
        }
        bytes.extend(record.surface.index().to_le_bytes());
        bytes.extend(record.surface.generation().to_le_bytes());
        bytes.extend(record.epoch.to_le_bytes());
        bytes.extend(record.token.to_le_bytes());
    }
    Ok(bytes)
}

pub fn decode_wm_launch_context_records(
    bytes: &[u8],
    count: u32,
) -> Result<Vec<PolicyLaunchContext>, IpcCodecError> {
    if count as usize > POLICY_MAX_SURFACES
        || bytes.len() != count as usize * LAUNCH_CONTEXT_RECORD_LEN
    {
        return Err(invalid());
    }
    let mut records = Vec::with_capacity(count as usize);
    for b in bytes.chunks_exact(LAUNCH_CONTEXT_RECORD_LEN) {
        records.push(PolicyLaunchContext {
            surface: SurfaceId::new(
                u32::from_le_bytes(b[0..4].try_into().unwrap()),
                u32::from_le_bytes(b[4..8].try_into().unwrap()),
            ),
            epoch: u64::from_le_bytes(b[8..16].try_into().unwrap()),
            token: u64::from_le_bytes(b[16..24].try_into().unwrap()),
        });
    }
    // Apply the same identity, duplicate and zero checks in either direction.
    encode_wm_launch_context_records(&records)?;
    Ok(records)
}

pub fn encode_wm_launch_contexts(
    records: &[PolicyLaunchContext],
    epoch: u64,
    ordinal: u16,
) -> Result<Vec<WmV1ProjectionChunk>, IpcCodecError> {
    if records.is_empty() {
        return Ok(Vec::new());
    }
    if records.iter().any(|r| r.epoch != epoch) {
        return Err(invalid());
    }
    Ok(vec![WmV1ProjectionChunk {
        connection_epoch: epoch,
        ordinal,
        record_kind: PROJECTION_LAUNCH_CONTEXT_RECORD_KIND,
        item_count: records.len() as u32,
        data: encode_wm_launch_context_records(records)?,
    }])
}

pub fn decode_wm_launch_contexts(
    chunks: &[WmV1ProjectionChunk],
) -> Result<Vec<PolicyLaunchContext>, IpcCodecError> {
    let mut records = Vec::new();
    for chunk in chunks
        .iter()
        .filter(|c| c.record_kind == PROJECTION_LAUNCH_CONTEXT_RECORD_KIND)
    {
        if chunk.item_count == 0 {
            return Err(invalid());
        }
        let batch = decode_wm_launch_context_records(&chunk.data, chunk.item_count)?;
        if batch.iter().any(|r| r.epoch != chunk.connection_epoch) {
            return Err(invalid());
        }
        records.extend(batch);
    }
    encode_wm_launch_context_records(&records)?;
    Ok(records)
}

/// Preserve the frozen counted prefix; origin records are only sent to a peer
/// which selected the capability. A previous-epoch bookmark is not replayed.
pub fn append_wm_launch_origins(
    transfer: &mut WmV1SnapshotTransfer,
    records: &[PolicyLaunchContext],
    capabilities: u64,
) -> Result<(), IpcCodecError> {
    if capabilities & super::SOPHIA_WM_CAPABILITY_LAUNCH_ORIGIN == 0 || records.is_empty() {
        return Ok(());
    }
    if records
        .iter()
        .any(|r| r.epoch != transfer.begin.connection_epoch)
    {
        return Err(invalid());
    }
    transfer.chunks.push(WmV1SnapshotChunk {
        connection_epoch: transfer.begin.connection_epoch,
        ordinal: u16::try_from(transfer.chunks.len()).map_err(|_| invalid())?,
        record_kind: SNAPSHOT_LAUNCH_ORIGIN_RECORD_KIND,
        item_count: records.len() as u32,
        data: encode_wm_launch_context_records(records)?,
    });
    Ok(())
}
