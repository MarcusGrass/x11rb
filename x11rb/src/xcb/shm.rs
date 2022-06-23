// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Shm` X11 extension.

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

pub use crate::protocol::shm::*;

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

pub fn attach(
    conn: &mut SocketConnection,
    shmseg: Seg,
    shmid: u32,
    read_only: bool,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = AttachRequest {
        shmseg,
        shmid,
        read_only,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn detach(
    conn: &mut SocketConnection,
    shmseg: Seg,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DetachRequest { shmseg };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn put_image(
    conn: &mut SocketConnection,
    drawable: xproto::Drawable,
    gc: xproto::Gcontext,
    total_width: u16,
    total_height: u16,
    src_x: u16,
    src_y: u16,
    src_width: u16,
    src_height: u16,
    dst_x: i16,
    dst_y: i16,
    depth: u8,
    format: u8,
    send_event: bool,
    shmseg: Seg,
    offset: u32,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = PutImageRequest {
        drawable,
        gc,
        total_width,
        total_height,
        src_x,
        src_y,
        src_width,
        src_height,
        dst_x,
        dst_y,
        depth,
        format,
        send_event,
        shmseg,
        offset,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_image(
    conn: &mut SocketConnection,
    drawable: xproto::Drawable,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    plane_mask: u32,
    format: u8,
    shmseg: Seg,
    offset: u32,
    forget: bool,
) -> Result<Cookie<GetImageReply>, ConnectionError> {
    let request0 = GetImageRequest {
        drawable,
        x,
        y,
        width,
        height,
        plane_mask,
        format,
        shmseg,
        offset,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_pixmap(
    conn: &mut SocketConnection,
    pid: xproto::Pixmap,
    drawable: xproto::Drawable,
    width: u16,
    height: u16,
    depth: u8,
    shmseg: Seg,
    offset: u32,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreatePixmapRequest {
        pid,
        drawable,
        width,
        height,
        depth,
        shmseg,
        offset,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}
