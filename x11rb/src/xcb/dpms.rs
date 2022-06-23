// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `DPMS` X11 extension.

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

pub use crate::protocol::dpms::*;

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

pub fn capable(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<CapableReply>, ConnectionError> {
    let request0 = CapableRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_timeouts(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<GetTimeoutsReply>, ConnectionError> {
    let request0 = GetTimeoutsRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_timeouts(
    conn: &mut SocketConnection,
    standby_timeout: u16,
    suspend_timeout: u16,
    off_timeout: u16,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetTimeoutsRequest {
        standby_timeout,
        suspend_timeout,
        off_timeout,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn enable(conn: &mut SocketConnection, forget: bool) -> Result<VoidCookie, ConnectionError> {
    let request0 = EnableRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn disable(conn: &mut SocketConnection, forget: bool) -> Result<VoidCookie, ConnectionError> {
    let request0 = DisableRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn force_level(
    conn: &mut SocketConnection,
    power_level: DPMSMode,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = ForceLevelRequest { power_level };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn info(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<InfoReply>, ConnectionError> {
    let request0 = InfoRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}
