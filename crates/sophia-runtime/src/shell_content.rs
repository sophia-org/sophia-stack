//! CPU content ownership for one admitted grant.
//!
//! This reducer does not grant capabilities, route bytes or report native
//! presentation. Its leases must outlive every consumer, including a retired
//! connection. The session-global admission owner must retain this store until
//! `quiescent`; dropping a peer is not a renderer completion.

mod candidates;
mod epochs;
mod resources;

pub use candidates::*;
pub use epochs::*;
pub use resources::*;
