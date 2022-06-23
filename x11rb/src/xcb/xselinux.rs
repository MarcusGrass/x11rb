// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `SELinux` X11 extension.

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

pub use crate::protocol::xselinux::*;

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

pub fn set_device_create_context<'c, 'input>(
    conn: &'c mut SocketConnection,
    context: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetDeviceCreateContextRequest {
        context: Cow::Borrowed(context),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_device_create_context(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<GetDeviceCreateContextReply>, ConnectionError> {
    let request0 = GetDeviceCreateContextRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_device_context<'c, 'input>(
    conn: &'c mut SocketConnection,
    device: u32,
    context: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetDeviceContextRequest {
        device,
        context: Cow::Borrowed(context),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_device_context(
    conn: &mut SocketConnection,
    device: u32,
    forget: bool,
) -> Result<Cookie<GetDeviceContextReply>, ConnectionError> {
    let request0 = GetDeviceContextRequest { device };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_window_create_context<'c, 'input>(
    conn: &'c mut SocketConnection,
    context: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetWindowCreateContextRequest {
        context: Cow::Borrowed(context),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_window_create_context(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<GetWindowCreateContextReply>, ConnectionError> {
    let request0 = GetWindowCreateContextRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_window_context(
    conn: &mut SocketConnection,
    window: xproto::Window,
    forget: bool,
) -> Result<Cookie<GetWindowContextReply>, ConnectionError> {
    let request0 = GetWindowContextRequest { window };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_property_create_context<'c, 'input>(
    conn: &'c mut SocketConnection,
    context: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetPropertyCreateContextRequest {
        context: Cow::Borrowed(context),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_property_create_context(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<GetPropertyCreateContextReply>, ConnectionError> {
    let request0 = GetPropertyCreateContextRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_property_use_context<'c, 'input>(
    conn: &'c mut SocketConnection,
    context: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetPropertyUseContextRequest {
        context: Cow::Borrowed(context),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_property_use_context(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<GetPropertyUseContextReply>, ConnectionError> {
    let request0 = GetPropertyUseContextRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_property_context(
    conn: &mut SocketConnection,
    window: xproto::Window,
    property: xproto::Atom,
    forget: bool,
) -> Result<Cookie<GetPropertyContextReply>, ConnectionError> {
    let request0 = GetPropertyContextRequest { window, property };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_property_data_context(
    conn: &mut SocketConnection,
    window: xproto::Window,
    property: xproto::Atom,
    forget: bool,
) -> Result<Cookie<GetPropertyDataContextReply>, ConnectionError> {
    let request0 = GetPropertyDataContextRequest { window, property };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn list_properties(
    conn: &mut SocketConnection,
    window: xproto::Window,
    forget: bool,
) -> Result<Cookie<ListPropertiesReply>, ConnectionError> {
    let request0 = ListPropertiesRequest { window };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_selection_create_context<'c, 'input>(
    conn: &'c mut SocketConnection,
    context: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetSelectionCreateContextRequest {
        context: Cow::Borrowed(context),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_selection_create_context(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<GetSelectionCreateContextReply>, ConnectionError> {
    let request0 = GetSelectionCreateContextRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_selection_use_context<'c, 'input>(
    conn: &'c mut SocketConnection,
    context: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetSelectionUseContextRequest {
        context: Cow::Borrowed(context),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_selection_use_context(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<GetSelectionUseContextReply>, ConnectionError> {
    let request0 = GetSelectionUseContextRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_selection_context(
    conn: &mut SocketConnection,
    selection: xproto::Atom,
    forget: bool,
) -> Result<Cookie<GetSelectionContextReply>, ConnectionError> {
    let request0 = GetSelectionContextRequest { selection };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_selection_data_context(
    conn: &mut SocketConnection,
    selection: xproto::Atom,
    forget: bool,
) -> Result<Cookie<GetSelectionDataContextReply>, ConnectionError> {
    let request0 = GetSelectionDataContextRequest { selection };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn list_selections(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<ListSelectionsReply>, ConnectionError> {
    let request0 = ListSelectionsRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_client_context(
    conn: &mut SocketConnection,
    resource: u32,
    forget: bool,
) -> Result<Cookie<GetClientContextReply>, ConnectionError> {
    let request0 = GetClientContextRequest { resource };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}
