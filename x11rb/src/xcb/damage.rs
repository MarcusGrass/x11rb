// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Damage` X11 extension.

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

pub use crate::protocol::damage::*;

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

pub fn create(
    conn: &mut SocketConnection,
    damage: Damage,
    drawable: xproto::Drawable,
    level: ReportLevel,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateRequest {
        damage,
        drawable,
        level,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn destroy(
    conn: &mut SocketConnection,
    damage: Damage,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DestroyRequest { damage };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn subtract<A, B>(
    conn: &mut SocketConnection,
    damage: Damage,
    repair: A,
    parts: B,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<xfixes::Region>,
    B: Into<xfixes::Region>,
{
    let repair: xfixes::Region = repair.into();
    let parts: xfixes::Region = parts.into();
    let request0 = SubtractRequest {
        damage,
        repair,
        parts,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn add(
    conn: &mut SocketConnection,
    drawable: xproto::Drawable,
    region: xfixes::Region,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = AddRequest { drawable, region };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}
