// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Xinerama` X11 extension.

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

pub use crate::protocol::xinerama::*;

/// Get the major opcode of this extension
fn major_opcode(conn: &SocketConnection) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME);
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub fn query_version(
    conn: &mut SocketConnection,
    major: u8,
    minor: u8,
    forget: bool,
) -> Result<Cookie<QueryVersionReply>, ConnectionError> {
    let request0 = QueryVersionRequest { major, minor };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_state(
    conn: &mut SocketConnection,
    window: xproto::Window,
    forget: bool,
) -> Result<Cookie<GetStateReply>, ConnectionError> {
    let request0 = GetStateRequest { window };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_screen_count(
    conn: &mut SocketConnection,
    window: xproto::Window,
    forget: bool,
) -> Result<Cookie<GetScreenCountReply>, ConnectionError> {
    let request0 = GetScreenCountRequest { window };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_screen_size(
    conn: &mut SocketConnection,
    window: xproto::Window,
    screen: u32,
    forget: bool,
) -> Result<Cookie<GetScreenSizeReply>, ConnectionError> {
    let request0 = GetScreenSizeRequest { window, screen };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn is_active(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<IsActiveReply>, ConnectionError> {
    let request0 = IsActiveRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_screens(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<QueryScreensReply>, ConnectionError> {
    let request0 = QueryScreensRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}
