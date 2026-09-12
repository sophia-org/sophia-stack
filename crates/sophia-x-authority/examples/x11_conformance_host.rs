//! Private, software-only production frontend for external protocol clients.
//! No session, renderer, DRM, physical input, or operator display is opened.
use sophia_protocol::{NamespaceCapabilities, NamespaceContext, NamespaceId, NamespaceProfile};
use sophia_x_authority::{XServerFrontend, XServerFrontendConfig, XServerFrontendRouteBroker};
use std::num::NonZeroUsize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args_os().skip(1);
    let path = args
        .next()
        .ok_or("usage: x11_conformance_host PRIVATE_SOCKET")?;
    if args.next().is_some() {
        return Err("expected exactly one private socket path".into());
    }
    let namespace = NamespaceContext::new(
        NamespaceId::from_raw(1),
        NamespaceProfile::ClassicShared,
        NamespaceCapabilities::NONE,
    )
    .ok_or("invalid namespace")?;
    let config = XServerFrontendConfig::new_with_namespace_context(path, namespace)?;
    let mut frontend = XServerFrontend::bind(config)?;
    let broker = XServerFrontendRouteBroker::new(NonZeroUsize::new(64).ok_or("zero queue")?);
    // Production bounded worker admission and shared protocol state. The runner
    // owns the process lifetime and enforces an absolute deadline externally.
    loop {
        frontend.serve_next_concurrently_routed(&broker)?;
    }
}
