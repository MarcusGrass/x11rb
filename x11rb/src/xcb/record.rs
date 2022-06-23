// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Record` X11 extension.

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

pub use crate::protocol::record::*;

/// Get the major opcode of this extension
fn major_opcode(conn: &SocketConnection) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME);
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub fn query_version(
    conn: &mut SocketConnection,
    major_version: u16,
    minor_version: u16,
    forget: bool,
) -> Result<Cookie<QueryVersionReply>, ConnectionError> {
    let request0 = QueryVersionRequest {
        major_version,
        minor_version,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_context<'c, 'input>(
    conn: &'c mut SocketConnection,
    context: Context,
    element_header: ElementHeader,
    client_specs: &'input [ClientSpec],
    ranges: &'input [Range],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateContextRequest {
        context,
        element_header,
        client_specs: Cow::Borrowed(client_specs),
        ranges: Cow::Borrowed(ranges),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn register_clients<'c, 'input>(
    conn: &'c mut SocketConnection,
    context: Context,
    element_header: ElementHeader,
    client_specs: &'input [ClientSpec],
    ranges: &'input [Range],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = RegisterClientsRequest {
        context,
        element_header,
        client_specs: Cow::Borrowed(client_specs),
        ranges: Cow::Borrowed(ranges),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn unregister_clients<'c, 'input>(
    conn: &'c mut SocketConnection,
    context: Context,
    client_specs: &'input [ClientSpec],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = UnregisterClientsRequest {
        context,
        client_specs: Cow::Borrowed(client_specs),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_context(
    conn: &mut SocketConnection,
    context: Context,
    forget: bool,
) -> Result<Cookie<GetContextReply>, ConnectionError> {
    let request0 = GetContextRequest { context };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn disable_context(
    conn: &mut SocketConnection,
    context: Context,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DisableContextRequest { context };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn free_context(
    conn: &mut SocketConnection,
    context: Context,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = FreeContextRequest { context };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}
