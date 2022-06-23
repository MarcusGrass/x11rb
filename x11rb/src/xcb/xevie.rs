// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Xevie` X11 extension.

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

pub use crate::protocol::xevie::*;

/// Get the major opcode of this extension
fn major_opcode(conn: &SocketConnection) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME);
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub fn query_version(
    conn: &mut SocketConnection,
    client_major_version: u16,
    client_minor_version: u16,
    forget: bool,
) -> Result<Cookie<QueryVersionReply>, ConnectionError> {
    let request0 = QueryVersionRequest {
        client_major_version,
        client_minor_version,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn start(
    conn: &mut SocketConnection,
    screen: u32,
    forget: bool,
) -> Result<Cookie<StartReply>, ConnectionError> {
    let request0 = StartRequest { screen };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn end(
    conn: &mut SocketConnection,
    cmap: u32,
    forget: bool,
) -> Result<Cookie<EndReply>, ConnectionError> {
    let request0 = EndRequest { cmap };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn send(
    conn: &mut SocketConnection,
    event: Event,
    data_type: u32,
    forget: bool,
) -> Result<Cookie<SendReply>, ConnectionError> {
    let request0 = SendRequest { event, data_type };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn select_input(
    conn: &mut SocketConnection,
    event_mask: u32,
    forget: bool,
) -> Result<Cookie<SelectInputReply>, ConnectionError> {
    let request0 = SelectInputRequest { event_mask };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}
