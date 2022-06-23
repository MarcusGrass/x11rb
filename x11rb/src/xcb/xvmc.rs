// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `XvMC` X11 extension.

#![allow(clippy::too_many_arguments)]

#[allow(unused_imports)]
use super::xv;
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

pub use crate::protocol::xvmc::*;

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

pub fn list_surface_types(
    conn: &mut SocketConnection,
    port_id: xv::Port,
    forget: bool,
) -> Result<Cookie<ListSurfaceTypesReply>, ConnectionError> {
    let request0 = ListSurfaceTypesRequest { port_id };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_context(
    conn: &mut SocketConnection,
    context_id: Context,
    port_id: xv::Port,
    surface_id: Surface,
    width: u16,
    height: u16,
    flags: u32,
    forget: bool,
) -> Result<Cookie<CreateContextReply>, ConnectionError> {
    let request0 = CreateContextRequest {
        context_id,
        port_id,
        surface_id,
        width,
        height,
        flags,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn destroy_context(
    conn: &mut SocketConnection,
    context_id: Context,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DestroyContextRequest { context_id };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_surface(
    conn: &mut SocketConnection,
    surface_id: Surface,
    context_id: Context,
    forget: bool,
) -> Result<Cookie<CreateSurfaceReply>, ConnectionError> {
    let request0 = CreateSurfaceRequest {
        surface_id,
        context_id,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn destroy_surface(
    conn: &mut SocketConnection,
    surface_id: Surface,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DestroySurfaceRequest { surface_id };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_subpicture(
    conn: &mut SocketConnection,
    subpicture_id: Subpicture,
    context: Context,
    xvimage_id: u32,
    width: u16,
    height: u16,
    forget: bool,
) -> Result<Cookie<CreateSubpictureReply>, ConnectionError> {
    let request0 = CreateSubpictureRequest {
        subpicture_id,
        context,
        xvimage_id,
        width,
        height,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn destroy_subpicture(
    conn: &mut SocketConnection,
    subpicture_id: Subpicture,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DestroySubpictureRequest { subpicture_id };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn list_subpicture_types(
    conn: &mut SocketConnection,
    port_id: xv::Port,
    surface_id: Surface,
    forget: bool,
) -> Result<Cookie<ListSubpictureTypesReply>, ConnectionError> {
    let request0 = ListSubpictureTypesRequest {
        port_id,
        surface_id,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}
