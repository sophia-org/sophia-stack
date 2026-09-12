use super::{ContentResourceStore, ContentStoreError};
use sophia_protocol::{ContentGrant, ContentLimits};

/// Session-wide owner: reserve a live grant's maximum footprint before admission
/// and retain disconnected epochs until all renderer references have drained.
/// This pool neither exposes a GPU nor negotiates a transport capability.
pub struct ContentEpochPool {
    active: Option<ContentResourceStore>,
    retired: Vec<ContentResourceStore>,
    last_grant: ContentGrant,
    reserved_live: u64,
    max_retiring_bytes: u64,
}

impl ContentEpochPool {
    /// Metadata is independently bounded: even tiny pinned resources cannot
    /// retain arbitrarily many replay tables through reconnect churn.
    pub const MAX_RETIRED_EPOCHS: usize = 16;

    pub fn new(max_retiring_bytes: u64) -> Result<Self, ContentStoreError> {
        if max_retiring_bytes == 0 || max_retiring_bytes > 64 * 1024 * 1024 {
            return Err(ContentStoreError::Budget);
        }
        Ok(Self {
            active: None,
            retired: Vec::new(),
            last_grant: ContentGrant::default(),
            reserved_live: 0,
            max_retiring_bytes,
        })
    }

    pub fn retired_bytes(&self) -> u64 {
        self.retired
            .iter()
            .map(|store| {
                let usage = store.usage();
                usage.staging + usage.resident + usage.retiring
            })
            .sum()
    }

    pub fn reserved_bytes(&self) -> u64 {
        self.reserved_live + self.retired_bytes()
    }
    pub fn active_mut(&mut self) -> Option<&mut ContentResourceStore> {
        self.active.as_mut()
    }
    pub fn active(&self) -> Option<&ContentResourceStore> {
        self.active.as_ref()
    }

    /// Permission must already have been established by the admission owner.
    /// This reserves storage capacity; it does not establish that permission.
    pub fn admit(&mut self, limits: ContentLimits) -> Result<(), ContentStoreError> {
        self.collect();
        if self.active.is_some() {
            return Err(ContentStoreError::Budget);
        }
        if limits.grant.connection_epoch <= self.last_grant.connection_epoch
            || limits.grant.content_grant_epoch <= self.last_grant.content_grant_epoch
        {
            return Err(ContentStoreError::Stale);
        }
        limits
            .validate()
            .map_err(|_| ContentStoreError::Malformed)?;
        let reserve =
            limits.max_staging_bytes + limits.max_resident_bytes + limits.max_retiring_bytes;
        if limits.max_session_retiring_bytes != self.max_retiring_bytes
            || self.retired.len() >= Self::MAX_RETIRED_EPOCHS
            || self.retired_bytes() + reserve > self.max_retiring_bytes
        {
            return Err(ContentStoreError::Budget);
        }
        self.last_grant = limits.grant;
        self.reserved_live = reserve;
        self.active = Some(ContentResourceStore::new(limits)?);
        Ok(())
    }

    pub fn disconnect(&mut self) {
        if let Some(mut store) = self.active.take() {
            store.revoke();
            // Delivery obligations are accounted under peer loss; they are not
            // misreported as delivered, nor transferred to the next connection.
            while store.take_event().is_some() {}
            if !store.quiescent() {
                self.retired.push(store);
            }
        }
        self.reserved_live = 0;
        self.collect();
    }

    pub fn collect(&mut self) {
        if let Some(store) = &mut self.active {
            store.collect();
        }
        for store in &mut self.retired {
            store.collect();
            while store.take_event().is_some() {}
        }
        self.retired.retain(|store| !store.quiescent());
    }
}
