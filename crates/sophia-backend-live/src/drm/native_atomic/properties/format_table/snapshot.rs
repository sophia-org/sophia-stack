use super::{
    FORMAT_BLOB_CURRENT, FORMAT_MODIFIER_RECORD_SIZE, FormatModifierBlobHeader, checked_table_end,
    read_u32, read_u64,
};
use drm::buffer::{DrmFourcc, DrmModifier};

/// One blob read supplies strict evidence and the existing allocation preferences.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LibdrmNativePlaneFormatCapabilities {
    pub snapshot: LibdrmNativePlaneFormatSnapshot,
    pub preferred_xrgb8888_modifiers: Vec<u64>,
}

impl LibdrmNativePlaneFormatCapabilities {
    pub fn parse(plane: u32, blob_id: u64, blob: &[u8]) -> Self {
        let preferred_xrgb8888_modifiers =
            super::LibdrmNativePlaneFormatModifierTable::parse_for_format(
                blob,
                DrmFourcc::Xrgb8888,
            )
            .table
            .map(|table| table.modifiers().iter().copied().map(u64::from).collect())
            .unwrap_or_default();
        Self {
            snapshot: LibdrmNativePlaneFormatSnapshot::parse(plane, blob_id, blob),
            preferred_xrgb8888_modifiers,
        }
    }

    pub fn unavailable(plane: u32, reason: LibdrmNativePlaneFormatSnapshotUnknown) -> Self {
        Self {
            snapshot: LibdrmNativePlaneFormatSnapshot::unavailable(plane, reason),
            preferred_xrgb8888_modifiers: Vec::new(),
        }
    }
}

/// An IN_FORMATS observation; the caller owns its card and topology lifetime.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LibdrmNativePlaneFormatSnapshot {
    pub plane: u32,
    pub blob_id: Option<u64>,
    unknown: Option<LibdrmNativePlaneFormatSnapshotUnknown>,
    formats: [FormatModifiers; 2],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LibdrmNativePlaneFormatSnapshotUnknown {
    Unavailable,
    ReadFailed,
    Malformed,
    UnsupportedVersion,
    CapacityExceeded,
    ImplicitModifier,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LibdrmNativePlaneFormatSupport {
    Unknown,
    Supported,
    Unsupported,
}

impl LibdrmNativePlaneFormatSnapshot {
    pub fn unavailable(plane: u32, reason: LibdrmNativePlaneFormatSnapshotUnknown) -> Self {
        Self {
            plane,
            blob_id: None,
            unknown: Some(reason),
            formats: [
                FormatModifiers::new(DrmFourcc::Xrgb8888),
                FormatModifiers::new(DrmFourcc::Argb8888),
            ],
        }
    }

    pub fn parse(plane: u32, blob_id: u64, blob: &[u8]) -> Self {
        let mut snapshot =
            Self::unavailable(plane, LibdrmNativePlaneFormatSnapshotUnknown::Malformed);
        snapshot.blob_id = Some(blob_id);
        if blob_id == 0 {
            return snapshot;
        }
        match snapshot.parse_tables(blob) {
            Ok(()) => snapshot.unknown = None,
            Err(reason) => snapshot.unknown = Some(reason),
        }
        snapshot
    }

    pub const fn unknown_reason(&self) -> Option<LibdrmNativePlaneFormatSnapshotUnknown> {
        self.unknown
    }

    /// Some(empty) means the explicit table contains no layout for this format.
    pub fn modifiers(&self, format: DrmFourcc) -> Option<&[DrmModifier]> {
        if self.unknown.is_some() {
            return None;
        }
        self.formats
            .iter()
            .find(|entry| entry.format == format)
            .map(|entry| &entry.modifiers[..entry.len])
    }

    pub fn support(
        &self,
        format: DrmFourcc,
        modifier: DrmModifier,
    ) -> LibdrmNativePlaneFormatSupport {
        if modifier == DrmModifier::Invalid {
            return LibdrmNativePlaneFormatSupport::Unknown;
        }
        match self.modifiers(format) {
            None => LibdrmNativePlaneFormatSupport::Unknown,
            Some(modifiers) if modifiers.contains(&modifier) => {
                LibdrmNativePlaneFormatSupport::Supported
            }
            Some(_) => LibdrmNativePlaneFormatSupport::Unsupported,
        }
    }

    fn parse_tables(&mut self, blob: &[u8]) -> Result<(), LibdrmNativePlaneFormatSnapshotUnknown> {
        let header = validated_header(blob)?;
        for entry in &mut self.formats {
            let Some(format_index) = (0..header.count_formats as usize).find(|index| {
                read_u32(blob, header.formats_offset as usize + index * 4)
                    == Some(entry.format as u32)
            }) else {
                continue;
            };
            for index in 0..header.count_modifiers as usize {
                let base = header.modifiers_offset as usize + index * FORMAT_MODIFIER_RECORD_SIZE;
                let formats = read_u64(blob, base).expect("validated modifier record");
                let offset = read_u32(blob, base + 8).expect("validated modifier record") as usize;
                if format_index < offset
                    || format_index - offset >= 64
                    || formats & (1 << (format_index - offset)) == 0
                {
                    continue;
                }
                let modifier = DrmModifier::from(
                    read_u64(blob, base + 16).expect("validated modifier record"),
                );
                if entry.modifiers[..entry.len].contains(&modifier) {
                    continue;
                }
                if entry.len == entry.modifiers.len() {
                    return Err(LibdrmNativePlaneFormatSnapshotUnknown::CapacityExceeded);
                }
                entry.modifiers[entry.len] = modifier;
                entry.len += 1;
            }
        }
        Ok(())
    }
}

// Fixed per-head evidence storage. Oversized tables remain unknown, never truncated.
const SNAPSHOT_MODIFIERS_PER_FORMAT: usize = 64;
#[derive(Clone, Debug, Eq, PartialEq)]
struct FormatModifiers {
    format: DrmFourcc,
    modifiers: [DrmModifier; SNAPSHOT_MODIFIERS_PER_FORMAT],
    len: usize,
}
impl FormatModifiers {
    fn new(format: DrmFourcc) -> Self {
        Self {
            format,
            modifiers: [DrmModifier::Linear; SNAPSHOT_MODIFIERS_PER_FORMAT],
            len: 0,
        }
    }
}

const FORMAT_BLOB_MAX_BYTES: usize = 64 * 1024;
const FORMAT_BLOB_MAX_ROWS: u32 = 256;

fn validated_header(
    blob: &[u8],
) -> Result<FormatModifierBlobHeader, LibdrmNativePlaneFormatSnapshotUnknown> {
    use LibdrmNativePlaneFormatSnapshotUnknown as Status;
    if blob.len() > FORMAT_BLOB_MAX_BYTES {
        return Err(Status::CapacityExceeded);
    }
    let header = FormatModifierBlobHeader::parse(blob).ok_or(Status::Malformed)?;
    if header.version != FORMAT_BLOB_CURRENT {
        return Err(Status::UnsupportedVersion);
    }
    if read_u32(blob, 4) != Some(0) {
        return Err(Status::Malformed);
    }
    if header.count_formats > FORMAT_BLOB_MAX_ROWS || header.count_modifiers > FORMAT_BLOB_MAX_ROWS
    {
        return Err(Status::CapacityExceeded);
    }
    let formats_start = header.formats_offset as usize;
    let modifiers_start = header.modifiers_offset as usize;
    let formats_end =
        checked_table_end(formats_start, header.count_formats as usize, 4, blob.len())
            .ok_or(Status::Malformed)?;
    let modifiers_end = checked_table_end(
        modifiers_start,
        header.count_modifiers as usize,
        FORMAT_MODIFIER_RECORD_SIZE,
        blob.len(),
    )
    .ok_or(Status::Malformed)?;
    if (header.count_formats != 0 && (formats_start < 24 || !formats_start.is_multiple_of(4)))
        || (header.count_modifiers != 0
            && (modifiers_start < 24 || !modifiers_start.is_multiple_of(8)))
        || (header.count_formats != 0
            && header.count_modifiers != 0
            && formats_start < modifiers_end
            && modifiers_start < formats_end)
    {
        return Err(Status::Malformed);
    }
    for index in 0..header.count_formats as usize {
        let format = read_u32(blob, formats_start + index * 4).ok_or(Status::Malformed)?;
        if (0..index).any(|previous| read_u32(blob, formats_start + previous * 4) == Some(format)) {
            return Err(Status::Malformed);
        }
    }
    for index in 0..header.count_modifiers as usize {
        let base = modifiers_start + index * FORMAT_MODIFIER_RECORD_SIZE;
        let formats = read_u64(blob, base).ok_or(Status::Malformed)?;
        let offset = read_u32(blob, base + 8).ok_or(Status::Malformed)?;
        if read_u32(blob, base + 12) != Some(0)
            || offset > header.count_formats
            || (formats != 0
                && offset
                    .checked_add(64 - formats.leading_zeros())
                    .is_none_or(|end| end > header.count_formats))
        {
            return Err(Status::Malformed);
        }
        if read_u64(blob, base + 16) == Some(u64::from(drm::buffer::DrmModifier::Invalid)) {
            return Err(Status::ImplicitModifier);
        }
    }
    Ok(header)
}
