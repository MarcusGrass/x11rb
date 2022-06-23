// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `ScreenSaver` X11 extension.

#![allow(clippy::too_many_arguments)]

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

pub use crate::protocol::screensaver::*;

/// Get the major opcode of this extension
fn major_opcode(conn: &SocketConnection) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME);
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub fn query_version(
    conn: &mut SocketConnection,
    client_major_version: u8,
    client_minor_version: u8,
    forget: bool,
) -> Result<Cookie<QueryVersionReply>, ConnectionError> {
    let request0 = QueryVersionRequest {
        client_major_version,
        client_minor_version,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_info(
    conn: &mut SocketConnection,
    drawable: xproto::Drawable,
    forget: bool,
) -> Result<Cookie<QueryInfoReply>, ConnectionError> {
    let request0 = QueryInfoRequest { drawable };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn select_input<A>(
    conn: &mut SocketConnection,
    drawable: xproto::Drawable,
    event_mask: A,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<u32>,
{
    let event_mask: u32 = event_mask.into();
    let request0 = SelectInputRequest {
        drawable,
        event_mask,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_attributes<'c, 'input>(
    conn: &'c mut SocketConnection,
    drawable: xproto::Drawable,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    border_width: u16,
    class: xproto::WindowClass,
    depth: u8,
    visual: xproto::Visualid,
    value_list: &'input SetAttributesAux,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetAttributesRequest {
        drawable,
        x,
        y,
        width,
        height,
        border_width,
        class,
        depth,
        visual,
        value_list: Cow::Borrowed(value_list),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn unset_attributes(
    conn: &mut SocketConnection,
    drawable: xproto::Drawable,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = UnsetAttributesRequest { drawable };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn suspend(
    conn: &mut SocketConnection,
    suspend: u32,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SuspendRequest { suspend };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}
