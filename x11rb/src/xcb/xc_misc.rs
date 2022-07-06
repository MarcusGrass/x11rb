// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `XCMisc` X11 extension.

#![allow(clippy::too_many_arguments)]

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

pub use crate::protocol::xc_misc::*;

/// Get the major opcode of this extension
fn major_opcode(conn: &SocketConnection) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME);
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub fn get_version(
    conn: &mut SocketConnection,
    client_major_version: u16,
    client_minor_version: u16,
    forget: bool,
) -> Result<Cookie<GetVersionReply>, ConnectionError> {
    let request0 = GetVersionRequest {
        client_major_version,
        client_minor_version,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_xid_range(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<GetXIDRangeReply>, ConnectionError> {
    let request0 = GetXIDRangeRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_xid_list(
    conn: &mut SocketConnection,
    count: u32,
    forget: bool,
) -> Result<Cookie<GetXIDListReply>, ConnectionError> {
    let request0 = GetXIDListRequest { count };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}