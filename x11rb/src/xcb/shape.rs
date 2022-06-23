// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Shape` X11 extension.

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

pub use crate::protocol::shape::*;

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

pub fn rectangles<'c, 'input>(
    conn: &'c mut SocketConnection,
    operation: SO,
    destination_kind: SK,
    ordering: xproto::ClipOrdering,
    destination_window: xproto::Window,
    x_offset: i16,
    y_offset: i16,
    rectangles: &'input [xproto::Rectangle],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = RectanglesRequest {
        operation,
        destination_kind,
        ordering,
        destination_window,
        x_offset,
        y_offset,
        rectangles: Cow::Borrowed(rectangles),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn mask<A>(
    conn: &mut SocketConnection,
    operation: SO,
    destination_kind: SK,
    destination_window: xproto::Window,
    x_offset: i16,
    y_offset: i16,
    source_bitmap: A,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<xproto::Pixmap>,
{
    let source_bitmap: xproto::Pixmap = source_bitmap.into();
    let request0 = MaskRequest {
        operation,
        destination_kind,
        destination_window,
        x_offset,
        y_offset,
        source_bitmap,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn combine(
    conn: &mut SocketConnection,
    operation: SO,
    destination_kind: SK,
    source_kind: SK,
    destination_window: xproto::Window,
    x_offset: i16,
    y_offset: i16,
    source_window: xproto::Window,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CombineRequest {
        operation,
        destination_kind,
        source_kind,
        destination_window,
        x_offset,
        y_offset,
        source_window,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn offset(
    conn: &mut SocketConnection,
    destination_kind: SK,
    destination_window: xproto::Window,
    x_offset: i16,
    y_offset: i16,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = OffsetRequest {
        destination_kind,
        destination_window,
        x_offset,
        y_offset,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_extents(
    conn: &mut SocketConnection,
    destination_window: xproto::Window,
    forget: bool,
) -> Result<Cookie<QueryExtentsReply>, ConnectionError> {
    let request0 = QueryExtentsRequest { destination_window };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn select_input(
    conn: &mut SocketConnection,
    destination_window: xproto::Window,
    enable: bool,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SelectInputRequest {
        destination_window,
        enable,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn input_selected(
    conn: &mut SocketConnection,
    destination_window: xproto::Window,
    forget: bool,
) -> Result<Cookie<InputSelectedReply>, ConnectionError> {
    let request0 = InputSelectedRequest { destination_window };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_rectangles(
    conn: &mut SocketConnection,
    window: xproto::Window,
    source_kind: SK,
    forget: bool,
) -> Result<Cookie<GetRectanglesReply>, ConnectionError> {
    let request0 = GetRectanglesRequest {
        window,
        source_kind,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}
