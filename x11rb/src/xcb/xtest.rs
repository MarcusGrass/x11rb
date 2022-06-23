// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Test` X11 extension.

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

pub use crate::protocol::xtest::*;

/// Get the major opcode of this extension
fn major_opcode(conn: &SocketConnection) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME);
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub fn get_version(
    conn: &mut SocketConnection,
    major_version: u8,
    minor_version: u16,
    forget: bool,
) -> Result<Cookie<GetVersionReply>, ConnectionError> {
    let request0 = GetVersionRequest {
        major_version,
        minor_version,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn compare_cursor(
    conn: &mut SocketConnection,
    window: xproto::Window,
    cursor: xproto::Cursor,
    forget: bool,
) -> Result<Cookie<CompareCursorReply>, ConnectionError> {
    let request0 = CompareCursorRequest { window, cursor };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn fake_input(
    conn: &mut SocketConnection,
    type_: u8,
    detail: u8,
    time: u32,
    root: xproto::Window,
    root_x: i16,
    root_y: i16,
    deviceid: u8,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = FakeInputRequest {
        type_,
        detail,
        time,
        root,
        root_x,
        root_y,
        deviceid,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn grab_control(
    conn: &mut SocketConnection,
    impervious: bool,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = GrabControlRequest { impervious };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}
