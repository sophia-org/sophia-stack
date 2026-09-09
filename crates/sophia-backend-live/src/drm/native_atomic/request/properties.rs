/// One raw value passed to the DRM atomic request.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LibdrmNativeAtomicProperty {
    pub object: u32,
    pub property: u32,
    pub value: u64,
}

const CAPACITY: usize = 32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CanonicalAtomicProperties {
    rows: [LibdrmNativeAtomicProperty; CAPACITY],
    len: usize,
}

impl CanonicalAtomicProperties {
    pub(crate) const fn new() -> Self {
        Self {
            rows: [LibdrmNativeAtomicProperty {
                object: 0,
                property: 0,
                value: 0,
            }; CAPACITY],
            len: 0,
        }
    }

    pub(crate) fn as_slice(&self) -> &[LibdrmNativeAtomicProperty] {
        &self.rows[..self.len]
    }

    /// Replaces duplicate keys exactly as AtomicModeReq::add_raw_property does.
    pub(crate) fn insert(&mut self, row: LibdrmNativeAtomicProperty) -> bool {
        match self
            .as_slice()
            .binary_search_by_key(&(row.object, row.property), |row| {
                (row.object, row.property)
            }) {
            Ok(index) => self.rows[index] = row,
            Err(index) => {
                if self.len == CAPACITY {
                    return false;
                }
                self.rows.copy_within(index..self.len, index + 1);
                self.rows[index] = row;
                self.len += 1;
            }
        }
        true
    }
}
