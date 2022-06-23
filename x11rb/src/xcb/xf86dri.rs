// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `XF86Dri` X11 extension.

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

pub use crate::protocol::xf86dri::*;

/// Get the major opcode of this extension
fn major_opcode(conn: &SocketConnection) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME);
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub fn query_version(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<QueryVersionReply>, ConnectionError> {
    let request0 = QueryVersionRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_direct_rendering_capable(
    conn: &mut SocketConnection,
    screen: u32,
    forget: bool,
) -> Result<Cookie<QueryDirectRenderingCapableReply>, ConnectionError> {
    let request0 = QueryDirectRenderingCapableRequest { screen };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn open_connection(
    conn: &mut SocketConnection,
    screen: u32,
    forget: bool,
) -> Result<Cookie<OpenConnectionReply>, ConnectionError> {
    let request0 = OpenConnectionRequest { screen };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn close_connection(
    conn: &mut SocketConnection,
    screen: u32,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CloseConnectionRequest { screen };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_client_driver_name(
    conn: &mut SocketConnection,
    screen: u32,
    forget: bool,
) -> Result<Cookie<GetClientDriverNameReply>, ConnectionError> {
    let request0 = GetClientDriverNameRequest { screen };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_context(
    conn: &mut SocketConnection,
    screen: u32,
    visual: u32,
    context: u32,
    forget: bool,
) -> Result<Cookie<CreateContextReply>, ConnectionError> {
    let request0 = CreateContextRequest {
        screen,
        visual,
        context,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn destroy_context(
    conn: &mut SocketConnection,
    screen: u32,
    context: u32,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DestroyContextRequest { screen, context };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_drawable(
    conn: &mut SocketConnection,
    screen: u32,
    drawable: u32,
    forget: bool,
) -> Result<Cookie<CreateDrawableReply>, ConnectionError> {
    let request0 = CreateDrawableRequest { screen, drawable };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn destroy_drawable(
    conn: &mut SocketConnection,
    screen: u32,
    drawable: u32,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DestroyDrawableRequest { screen, drawable };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_drawable_info(
    conn: &mut SocketConnection,
    screen: u32,
    drawable: u32,
    forget: bool,
) -> Result<Cookie<GetDrawableInfoReply>, ConnectionError> {
    let request0 = GetDrawableInfoRequest { screen, drawable };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_device_info(
    conn: &mut SocketConnection,
    screen: u32,
    forget: bool,
) -> Result<Cookie<GetDeviceInfoReply>, ConnectionError> {
    let request0 = GetDeviceInfoRequest { screen };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn auth_connection(
    conn: &mut SocketConnection,
    screen: u32,
    magic: u32,
    forget: bool,
) -> Result<Cookie<AuthConnectionReply>, ConnectionError> {
    let request0 = AuthConnectionRequest { screen, magic };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}
