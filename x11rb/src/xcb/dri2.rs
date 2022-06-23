// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `DRI2` X11 extension.

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

pub use crate::protocol::dri2::*;

/// Get the major opcode of this extension
fn major_opcode(conn: &SocketConnection) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME);
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub fn query_version(
    conn: &mut SocketConnection,
    major_version: u32,
    minor_version: u32,
    forget: bool,
) -> Result<Cookie<QueryVersionReply>, ConnectionError> {
    let request0 = QueryVersionRequest {
        major_version,
        minor_version,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn connect(
    conn: &mut SocketConnection,
    window: xproto::Window,
    driver_type: DriverType,
    forget: bool,
) -> Result<Cookie<ConnectReply>, ConnectionError> {
    let request0 = ConnectRequest {
        window,
        driver_type,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn authenticate(
    conn: &mut SocketConnection,
    window: xproto::Window,
    magic: u32,
    forget: bool,
) -> Result<Cookie<AuthenticateReply>, ConnectionError> {
    let request0 = AuthenticateRequest { window, magic };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_drawable(
    conn: &mut SocketConnection,
    drawable: xproto::Drawable,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateDrawableRequest { drawable };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn destroy_drawable(
    conn: &mut SocketConnection,
    drawable: xproto::Drawable,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DestroyDrawableRequest { drawable };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_buffers<'c, 'input>(
    conn: &'c mut SocketConnection,
    drawable: xproto::Drawable,
    count: u32,
    attachments: &'input [u32],
    forget: bool,
) -> Result<Cookie<GetBuffersReply>, ConnectionError> {
    let request0 = GetBuffersRequest {
        drawable,
        count,
        attachments: Cow::Borrowed(attachments),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn copy_region(
    conn: &mut SocketConnection,
    drawable: xproto::Drawable,
    region: u32,
    dest: u32,
    src: u32,
    forget: bool,
) -> Result<Cookie<CopyRegionReply>, ConnectionError> {
    let request0 = CopyRegionRequest {
        drawable,
        region,
        dest,
        src,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_buffers_with_format<'c, 'input>(
    conn: &'c mut SocketConnection,
    drawable: xproto::Drawable,
    count: u32,
    attachments: &'input [AttachFormat],
    forget: bool,
) -> Result<Cookie<GetBuffersWithFormatReply>, ConnectionError> {
    let request0 = GetBuffersWithFormatRequest {
        drawable,
        count,
        attachments: Cow::Borrowed(attachments),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn swap_buffers(
    conn: &mut SocketConnection,
    drawable: xproto::Drawable,
    target_msc_hi: u32,
    target_msc_lo: u32,
    divisor_hi: u32,
    divisor_lo: u32,
    remainder_hi: u32,
    remainder_lo: u32,
    forget: bool,
) -> Result<Cookie<SwapBuffersReply>, ConnectionError> {
    let request0 = SwapBuffersRequest {
        drawable,
        target_msc_hi,
        target_msc_lo,
        divisor_hi,
        divisor_lo,
        remainder_hi,
        remainder_lo,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_msc(
    conn: &mut SocketConnection,
    drawable: xproto::Drawable,
    forget: bool,
) -> Result<Cookie<GetMSCReply>, ConnectionError> {
    let request0 = GetMSCRequest { drawable };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn wait_msc(
    conn: &mut SocketConnection,
    drawable: xproto::Drawable,
    target_msc_hi: u32,
    target_msc_lo: u32,
    divisor_hi: u32,
    divisor_lo: u32,
    remainder_hi: u32,
    remainder_lo: u32,
    forget: bool,
) -> Result<Cookie<WaitMSCReply>, ConnectionError> {
    let request0 = WaitMSCRequest {
        drawable,
        target_msc_hi,
        target_msc_lo,
        divisor_hi,
        divisor_lo,
        remainder_hi,
        remainder_lo,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn wait_sbc(
    conn: &mut SocketConnection,
    drawable: xproto::Drawable,
    target_sbc_hi: u32,
    target_sbc_lo: u32,
    forget: bool,
) -> Result<Cookie<WaitSBCReply>, ConnectionError> {
    let request0 = WaitSBCRequest {
        drawable,
        target_sbc_hi,
        target_sbc_lo,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn swap_interval(
    conn: &mut SocketConnection,
    drawable: xproto::Drawable,
    interval: u32,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SwapIntervalRequest { drawable, interval };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_param(
    conn: &mut SocketConnection,
    drawable: xproto::Drawable,
    param: u32,
    forget: bool,
) -> Result<Cookie<GetParamReply>, ConnectionError> {
    let request0 = GetParamRequest { drawable, param };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}
