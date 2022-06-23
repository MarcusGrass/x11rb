// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Present` X11 extension.

#![allow(clippy::too_many_arguments)]

#[allow(unused_imports)]
use super::randr;
#[allow(unused_imports)]
use super::sync;
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

pub use crate::protocol::present::*;

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

pub fn pixmap<'c, 'input>(
    conn: &'c mut SocketConnection,
    window: xproto::Window,
    pixmap: xproto::Pixmap,
    serial: u32,
    valid: xfixes::Region,
    update: xfixes::Region,
    x_off: i16,
    y_off: i16,
    target_crtc: randr::Crtc,
    wait_fence: sync::Fence,
    idle_fence: sync::Fence,
    options: u32,
    target_msc: u64,
    divisor: u64,
    remainder: u64,
    notifies: &'input [Notify],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = PixmapRequest {
        window,
        pixmap,
        serial,
        valid,
        update,
        x_off,
        y_off,
        target_crtc,
        wait_fence,
        idle_fence,
        options,
        target_msc,
        divisor,
        remainder,
        notifies: Cow::Borrowed(notifies),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn notify_msc(
    conn: &mut SocketConnection,
    window: xproto::Window,
    serial: u32,
    target_msc: u64,
    divisor: u64,
    remainder: u64,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = NotifyMSCRequest {
        window,
        serial,
        target_msc,
        divisor,
        remainder,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn select_input<A>(
    conn: &mut SocketConnection,
    eid: Event,
    window: xproto::Window,
    event_mask: A,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<u32>,
{
    let event_mask: u32 = event_mask.into();
    let request0 = SelectInputRequest {
        eid,
        window,
        event_mask,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_capabilities(
    conn: &mut SocketConnection,
    target: u32,
    forget: bool,
) -> Result<Cookie<QueryCapabilitiesReply>, ConnectionError> {
    let request0 = QueryCapabilitiesRequest { target };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}
