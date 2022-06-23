// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Composite` X11 extension.

#![allow(clippy::too_many_arguments)]

#[allow(unused_imports)]
use super::xfixes;
#[allow(unused_imports)]
use super::xproto;
#[allow(unused_imports)]
use crate::cookie::{Cookie, VoidCookie};
use crate::errors::ConnectionError;
#[allow(unused_imports)]
use crate::errors::ReplyOrIdError;
#[allow(unused_imports)]
use crate::x11_utils::{RequestHeader, Serialize, TryParse, TryParseFd};
use crate::SocketConnection;
#[allow(unused_imports)]
use std::borrow::Cow;
#[allow(unused_imports)]
use std::convert::TryInto;

pub use crate::protocol::composite::*;

/// Get the major opcode of this extension
fn major_opcode(conn: &SocketConnection) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME);
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub fn query_version(
    conn: &mut SocketConnection,
    client_major_version: u32,
    client_minor_version: u32,
    forget: bool,
) -> Result<Cookie<QueryVersionReply>, ConnectionError> {
    let request0 = QueryVersionRequest {
        client_major_version,
        client_minor_version,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn redirect_window(
    conn: &mut SocketConnection,
    window: xproto::Window,
    update: Redirect,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = RedirectWindowRequest { window, update };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn redirect_subwindows(
    conn: &mut SocketConnection,
    window: xproto::Window,
    update: Redirect,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = RedirectSubwindowsRequest { window, update };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn unredirect_window(
    conn: &mut SocketConnection,
    window: xproto::Window,
    update: Redirect,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = UnredirectWindowRequest { window, update };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn unredirect_subwindows(
    conn: &mut SocketConnection,
    window: xproto::Window,
    update: Redirect,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = UnredirectSubwindowsRequest { window, update };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_region_from_border_clip(
    conn: &mut SocketConnection,
    region: xfixes::Region,
    window: xproto::Window,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateRegionFromBorderClipRequest { region, window };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn name_window_pixmap(
    conn: &mut SocketConnection,
    window: xproto::Window,
    pixmap: xproto::Pixmap,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = NameWindowPixmapRequest { window, pixmap };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_overlay_window(
    conn: &mut SocketConnection,
    window: xproto::Window,
    forget: bool,
) -> Result<Cookie<GetOverlayWindowReply>, ConnectionError> {
    let request0 = GetOverlayWindowRequest { window };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn release_overlay_window(
    conn: &mut SocketConnection,
    window: xproto::Window,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = ReleaseOverlayWindowRequest { window };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}
