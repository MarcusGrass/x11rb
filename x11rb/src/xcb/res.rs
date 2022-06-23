// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Res` X11 extension.

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

pub use crate::protocol::res::*;

/// Get the major opcode of this extension
fn major_opcode(conn: &SocketConnection) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME);
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub fn query_version(
    conn: &mut SocketConnection,
    client_major: u8,
    client_minor: u8,
    forget: bool,
) -> Result<Cookie<QueryVersionReply>, ConnectionError> {
    let request0 = QueryVersionRequest {
        client_major,
        client_minor,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_clients(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<QueryClientsReply>, ConnectionError> {
    let request0 = QueryClientsRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_client_resources(
    conn: &mut SocketConnection,
    xid: u32,
    forget: bool,
) -> Result<Cookie<QueryClientResourcesReply>, ConnectionError> {
    let request0 = QueryClientResourcesRequest { xid };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_client_pixmap_bytes(
    conn: &mut SocketConnection,
    xid: u32,
    forget: bool,
) -> Result<Cookie<QueryClientPixmapBytesReply>, ConnectionError> {
    let request0 = QueryClientPixmapBytesRequest { xid };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_client_ids<'c, 'input>(
    conn: &'c mut SocketConnection,
    specs: &'input [ClientIdSpec],
    forget: bool,
) -> Result<Cookie<QueryClientIdsReply>, ConnectionError> {
    let request0 = QueryClientIdsRequest {
        specs: Cow::Borrowed(specs),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_resource_bytes<'c, 'input>(
    conn: &'c mut SocketConnection,
    client: u32,
    specs: &'input [ResourceIdSpec],
    forget: bool,
) -> Result<Cookie<QueryResourceBytesReply>, ConnectionError> {
    let request0 = QueryResourceBytesRequest {
        client,
        specs: Cow::Borrowed(specs),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}
