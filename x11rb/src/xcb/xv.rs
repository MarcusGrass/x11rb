// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Xv` X11 extension.

#![allow(clippy::too_many_arguments)]

#[allow(unused_imports)]
use super::shm;
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

pub use crate::protocol::xv::*;

/// Get the major opcode of this extension
fn major_opcode(conn: &SocketConnection) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME);
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub fn query_extension(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<QueryExtensionReply>, ConnectionError> {
    let request0 = QueryExtensionRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_adaptors(
    conn: &mut SocketConnection,
    window: xproto::Window,
    forget: bool,
) -> Result<Cookie<QueryAdaptorsReply>, ConnectionError> {
    let request0 = QueryAdaptorsRequest { window };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_encodings(
    conn: &mut SocketConnection,
    port: Port,
    forget: bool,
) -> Result<Cookie<QueryEncodingsReply>, ConnectionError> {
    let request0 = QueryEncodingsRequest { port };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn grab_port<A>(
    conn: &mut SocketConnection,
    port: Port,
    time: A,
    forget: bool,
) -> Result<Cookie<GrabPortReply>, ConnectionError>
where
    A: Into<xproto::Timestamp>,
{
    let time: xproto::Timestamp = time.into();
    let request0 = GrabPortRequest { port, time };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn ungrab_port<A>(
    conn: &mut SocketConnection,
    port: Port,
    time: A,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<xproto::Timestamp>,
{
    let time: xproto::Timestamp = time.into();
    let request0 = UngrabPortRequest { port, time };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn put_video(
    conn: &mut SocketConnection,
    port: Port,
    drawable: xproto::Drawable,
    gc: xproto::Gcontext,
    vid_x: i16,
    vid_y: i16,
    vid_w: u16,
    vid_h: u16,
    drw_x: i16,
    drw_y: i16,
    drw_w: u16,
    drw_h: u16,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = PutVideoRequest {
        port,
        drawable,
        gc,
        vid_x,
        vid_y,
        vid_w,
        vid_h,
        drw_x,
        drw_y,
        drw_w,
        drw_h,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn put_still(
    conn: &mut SocketConnection,
    port: Port,
    drawable: xproto::Drawable,
    gc: xproto::Gcontext,
    vid_x: i16,
    vid_y: i16,
    vid_w: u16,
    vid_h: u16,
    drw_x: i16,
    drw_y: i16,
    drw_w: u16,
    drw_h: u16,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = PutStillRequest {
        port,
        drawable,
        gc,
        vid_x,
        vid_y,
        vid_w,
        vid_h,
        drw_x,
        drw_y,
        drw_w,
        drw_h,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_video(
    conn: &mut SocketConnection,
    port: Port,
    drawable: xproto::Drawable,
    gc: xproto::Gcontext,
    vid_x: i16,
    vid_y: i16,
    vid_w: u16,
    vid_h: u16,
    drw_x: i16,
    drw_y: i16,
    drw_w: u16,
    drw_h: u16,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = GetVideoRequest {
        port,
        drawable,
        gc,
        vid_x,
        vid_y,
        vid_w,
        vid_h,
        drw_x,
        drw_y,
        drw_w,
        drw_h,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_still(
    conn: &mut SocketConnection,
    port: Port,
    drawable: xproto::Drawable,
    gc: xproto::Gcontext,
    vid_x: i16,
    vid_y: i16,
    vid_w: u16,
    vid_h: u16,
    drw_x: i16,
    drw_y: i16,
    drw_w: u16,
    drw_h: u16,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = GetStillRequest {
        port,
        drawable,
        gc,
        vid_x,
        vid_y,
        vid_w,
        vid_h,
        drw_x,
        drw_y,
        drw_w,
        drw_h,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn stop_video(
    conn: &mut SocketConnection,
    port: Port,
    drawable: xproto::Drawable,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = StopVideoRequest { port, drawable };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn select_video_notify(
    conn: &mut SocketConnection,
    drawable: xproto::Drawable,
    onoff: bool,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SelectVideoNotifyRequest { drawable, onoff };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn select_port_notify(
    conn: &mut SocketConnection,
    port: Port,
    onoff: bool,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SelectPortNotifyRequest { port, onoff };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_best_size(
    conn: &mut SocketConnection,
    port: Port,
    vid_w: u16,
    vid_h: u16,
    drw_w: u16,
    drw_h: u16,
    motion: bool,
    forget: bool,
) -> Result<Cookie<QueryBestSizeReply>, ConnectionError> {
    let request0 = QueryBestSizeRequest {
        port,
        vid_w,
        vid_h,
        drw_w,
        drw_h,
        motion,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_port_attribute(
    conn: &mut SocketConnection,
    port: Port,
    attribute: xproto::Atom,
    value: i32,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetPortAttributeRequest {
        port,
        attribute,
        value,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_port_attribute(
    conn: &mut SocketConnection,
    port: Port,
    attribute: xproto::Atom,
    forget: bool,
) -> Result<Cookie<GetPortAttributeReply>, ConnectionError> {
    let request0 = GetPortAttributeRequest { port, attribute };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_port_attributes(
    conn: &mut SocketConnection,
    port: Port,
    forget: bool,
) -> Result<Cookie<QueryPortAttributesReply>, ConnectionError> {
    let request0 = QueryPortAttributesRequest { port };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn list_image_formats(
    conn: &mut SocketConnection,
    port: Port,
    forget: bool,
) -> Result<Cookie<ListImageFormatsReply>, ConnectionError> {
    let request0 = ListImageFormatsRequest { port };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_image_attributes(
    conn: &mut SocketConnection,
    port: Port,
    id: u32,
    width: u16,
    height: u16,
    forget: bool,
) -> Result<Cookie<QueryImageAttributesReply>, ConnectionError> {
    let request0 = QueryImageAttributesRequest {
        port,
        id,
        width,
        height,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn put_image<'c, 'input>(
    conn: &'c mut SocketConnection,
    port: Port,
    drawable: xproto::Drawable,
    gc: xproto::Gcontext,
    id: u32,
    src_x: i16,
    src_y: i16,
    src_w: u16,
    src_h: u16,
    drw_x: i16,
    drw_y: i16,
    drw_w: u16,
    drw_h: u16,
    width: u16,
    height: u16,
    data: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = PutImageRequest {
        port,
        drawable,
        gc,
        id,
        src_x,
        src_y,
        src_w,
        src_h,
        drw_x,
        drw_y,
        drw_w,
        drw_h,
        width,
        height,
        data: Cow::Borrowed(data),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn shm_put_image(
    conn: &mut SocketConnection,
    port: Port,
    drawable: xproto::Drawable,
    gc: xproto::Gcontext,
    shmseg: shm::Seg,
    id: u32,
    offset: u32,
    src_x: i16,
    src_y: i16,
    src_w: u16,
    src_h: u16,
    drw_x: i16,
    drw_y: i16,
    drw_w: u16,
    drw_h: u16,
    width: u16,
    height: u16,
    send_event: u8,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = ShmPutImageRequest {
        port,
        drawable,
        gc,
        shmseg,
        id,
        offset,
        src_x,
        src_y,
        src_w,
        src_h,
        drw_x,
        drw_y,
        drw_w,
        drw_h,
        width,
        height,
        send_event,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}
