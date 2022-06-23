// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `XFixes` X11 extension.

#![allow(clippy::too_many_arguments)]

#[allow(unused_imports)]
use super::render;
#[allow(unused_imports)]
use super::shape;
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

pub use crate::protocol::xfixes::*;

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

pub fn change_save_set(
    conn: &mut SocketConnection,
    mode: SaveSetMode,
    target: SaveSetTarget,
    map: SaveSetMapping,
    window: xproto::Window,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = ChangeSaveSetRequest {
        mode,
        target,
        map,
        window,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn select_selection_input<A>(
    conn: &mut SocketConnection,
    window: xproto::Window,
    selection: xproto::Atom,
    event_mask: A,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<u32>,
{
    let event_mask: u32 = event_mask.into();
    let request0 = SelectSelectionInputRequest {
        window,
        selection,
        event_mask,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn select_cursor_input<A>(
    conn: &mut SocketConnection,
    window: xproto::Window,
    event_mask: A,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<u32>,
{
    let event_mask: u32 = event_mask.into();
    let request0 = SelectCursorInputRequest { window, event_mask };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_cursor_image(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<GetCursorImageReply>, ConnectionError> {
    let request0 = GetCursorImageRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_region<'c, 'input>(
    conn: &'c mut SocketConnection,
    region: Region,
    rectangles: &'input [xproto::Rectangle],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateRegionRequest {
        region,
        rectangles: Cow::Borrowed(rectangles),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_region_from_bitmap(
    conn: &mut SocketConnection,
    region: Region,
    bitmap: xproto::Pixmap,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateRegionFromBitmapRequest { region, bitmap };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_region_from_window(
    conn: &mut SocketConnection,
    region: Region,
    window: xproto::Window,
    kind: shape::SK,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateRegionFromWindowRequest {
        region,
        window,
        kind,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_region_from_gc(
    conn: &mut SocketConnection,
    region: Region,
    gc: xproto::Gcontext,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateRegionFromGCRequest { region, gc };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_region_from_picture(
    conn: &mut SocketConnection,
    region: Region,
    picture: render::Picture,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateRegionFromPictureRequest { region, picture };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn destroy_region(
    conn: &mut SocketConnection,
    region: Region,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DestroyRegionRequest { region };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_region<'c, 'input>(
    conn: &'c mut SocketConnection,
    region: Region,
    rectangles: &'input [xproto::Rectangle],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetRegionRequest {
        region,
        rectangles: Cow::Borrowed(rectangles),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn copy_region(
    conn: &mut SocketConnection,
    source: Region,
    destination: Region,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CopyRegionRequest {
        source,
        destination,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn union_region(
    conn: &mut SocketConnection,
    source1: Region,
    source2: Region,
    destination: Region,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = UnionRegionRequest {
        source1,
        source2,
        destination,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn intersect_region(
    conn: &mut SocketConnection,
    source1: Region,
    source2: Region,
    destination: Region,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = IntersectRegionRequest {
        source1,
        source2,
        destination,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn subtract_region(
    conn: &mut SocketConnection,
    source1: Region,
    source2: Region,
    destination: Region,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SubtractRegionRequest {
        source1,
        source2,
        destination,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn invert_region(
    conn: &mut SocketConnection,
    source: Region,
    bounds: xproto::Rectangle,
    destination: Region,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = InvertRegionRequest {
        source,
        bounds,
        destination,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn translate_region(
    conn: &mut SocketConnection,
    region: Region,
    dx: i16,
    dy: i16,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = TranslateRegionRequest { region, dx, dy };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn region_extents(
    conn: &mut SocketConnection,
    source: Region,
    destination: Region,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = RegionExtentsRequest {
        source,
        destination,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn fetch_region(
    conn: &mut SocketConnection,
    region: Region,
    forget: bool,
) -> Result<Cookie<FetchRegionReply>, ConnectionError> {
    let request0 = FetchRegionRequest { region };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_gc_clip_region<A>(
    conn: &mut SocketConnection,
    gc: xproto::Gcontext,
    region: A,
    x_origin: i16,
    y_origin: i16,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<Region>,
{
    let region: Region = region.into();
    let request0 = SetGCClipRegionRequest {
        gc,
        region,
        x_origin,
        y_origin,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_window_shape_region<A>(
    conn: &mut SocketConnection,
    dest: xproto::Window,
    dest_kind: shape::SK,
    x_offset: i16,
    y_offset: i16,
    region: A,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<Region>,
{
    let region: Region = region.into();
    let request0 = SetWindowShapeRegionRequest {
        dest,
        dest_kind,
        x_offset,
        y_offset,
        region,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_picture_clip_region<A>(
    conn: &mut SocketConnection,
    picture: render::Picture,
    region: A,
    x_origin: i16,
    y_origin: i16,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<Region>,
{
    let region: Region = region.into();
    let request0 = SetPictureClipRegionRequest {
        picture,
        region,
        x_origin,
        y_origin,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_cursor_name<'c, 'input>(
    conn: &'c mut SocketConnection,
    cursor: xproto::Cursor,
    name: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetCursorNameRequest {
        cursor,
        name: Cow::Borrowed(name),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_cursor_name(
    conn: &mut SocketConnection,
    cursor: xproto::Cursor,
    forget: bool,
) -> Result<Cookie<GetCursorNameReply>, ConnectionError> {
    let request0 = GetCursorNameRequest { cursor };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_cursor_image_and_name(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<GetCursorImageAndNameReply>, ConnectionError> {
    let request0 = GetCursorImageAndNameRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn change_cursor(
    conn: &mut SocketConnection,
    source: xproto::Cursor,
    destination: xproto::Cursor,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = ChangeCursorRequest {
        source,
        destination,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn change_cursor_by_name<'c, 'input>(
    conn: &'c mut SocketConnection,
    src: xproto::Cursor,
    name: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = ChangeCursorByNameRequest {
        src,
        name: Cow::Borrowed(name),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn expand_region(
    conn: &mut SocketConnection,
    source: Region,
    destination: Region,
    left: u16,
    right: u16,
    top: u16,
    bottom: u16,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = ExpandRegionRequest {
        source,
        destination,
        left,
        right,
        top,
        bottom,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn hide_cursor(
    conn: &mut SocketConnection,
    window: xproto::Window,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = HideCursorRequest { window };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn show_cursor(
    conn: &mut SocketConnection,
    window: xproto::Window,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = ShowCursorRequest { window };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_pointer_barrier<'c, 'input, A>(
    conn: &'c mut SocketConnection,
    barrier: Barrier,
    window: xproto::Window,
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16,
    directions: A,
    devices: &'input [u16],
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<u32>,
{
    let directions: u32 = directions.into();
    let request0 = CreatePointerBarrierRequest {
        barrier,
        window,
        x1,
        y1,
        x2,
        y2,
        directions,
        devices: Cow::Borrowed(devices),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn delete_pointer_barrier(
    conn: &mut SocketConnection,
    barrier: Barrier,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DeletePointerBarrierRequest { barrier };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}
