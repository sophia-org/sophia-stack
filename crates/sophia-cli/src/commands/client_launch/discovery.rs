use x11rb::connection::Connection;
use x11rb::protocol::dri3::ConnectionExt as _;
use x11rb::reexports::x11rb_protocol::{
    parse_display::{ConnectAddress, parse_display},
    xauth::get_auth,
};
use x11rb::rust_connection::{DefaultStream, RustConnection};

use super::device::{self, RenderDevice};

pub(super) fn discover() -> Result<RenderDevice, String> {
    let parsed = parse_display(None).map_err(|_| "invalid_display")?;
    if !matches!(parsed.protocol.as_deref(), None | Some("unix")) {
        return Err("nonlocal_display".into());
    }
    let address = if parsed.protocol.as_deref() == Some("unix") && parsed.host.starts_with('/') {
        ConnectAddress::Socket(parsed.host.clone())
    } else if parsed.host.is_empty() || parsed.host == "unix" {
        ConnectAddress::Socket(format!("/tmp/.X11-unix/X{}", parsed.display))
    } else {
        return Err("nonlocal_display".into());
    };
    let (stream, (family, address)) =
        DefaultStream::connect(&address).map_err(|_| "display_connect_failed")?;
    // Do not silently retry unauthenticated when the authority file is absent or unreadable.
    let (name, data) = get_auth(family, &address, parsed.display)
        .map_err(|_| "display_authentication_unavailable")?
        .filter(|(name, data)| name == b"MIT-MAGIC-COOKIE-1" && data.len() == 16)
        .ok_or("display_authentication_unavailable")?;
    let screen = usize::from(parsed.screen);
    let connection = RustConnection::connect_to_stream_with_auth_info(stream, screen, name, data)
        .map_err(|_| "display_authentication_failed")?;
    let root = connection
        .setup()
        .roots
        .get(screen)
        .ok_or("display_screen_missing")?
        .root;
    let reply = connection
        .dri3_open(root, 0)
        .map_err(|_| "dri3_open_unavailable")?
        .reply()
        .map_err(|_| "dri3_open_failed")?;
    if reply.nfd != 1 || reply.length != 0 {
        return Err("dri3_invalid_descriptor_reply".into());
    }
    device::resolve(&reply.device_fd)
}
