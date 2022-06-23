// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Render` X11 extension.

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

pub use crate::protocol::render::*;

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

pub fn query_pict_formats(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<QueryPictFormatsReply>, ConnectionError> {
    let request0 = QueryPictFormatsRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_pict_index_values(
    conn: &mut SocketConnection,
    format: Pictformat,
    forget: bool,
) -> Result<Cookie<QueryPictIndexValuesReply>, ConnectionError> {
    let request0 = QueryPictIndexValuesRequest { format };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_picture<'c, 'input>(
    conn: &'c mut SocketConnection,
    pid: Picture,
    drawable: xproto::Drawable,
    format: Pictformat,
    value_list: &'input CreatePictureAux,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreatePictureRequest {
        pid,
        drawable,
        format,
        value_list: Cow::Borrowed(value_list),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn change_picture<'c, 'input>(
    conn: &'c mut SocketConnection,
    picture: Picture,
    value_list: &'input ChangePictureAux,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = ChangePictureRequest {
        picture,
        value_list: Cow::Borrowed(value_list),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_picture_clip_rectangles<'c, 'input>(
    conn: &'c mut SocketConnection,
    picture: Picture,
    clip_x_origin: i16,
    clip_y_origin: i16,
    rectangles: &'input [xproto::Rectangle],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetPictureClipRectanglesRequest {
        picture,
        clip_x_origin,
        clip_y_origin,
        rectangles: Cow::Borrowed(rectangles),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn free_picture(
    conn: &mut SocketConnection,
    picture: Picture,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = FreePictureRequest { picture };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn composite<A>(
    conn: &mut SocketConnection,
    op: PictOp,
    src: Picture,
    mask: A,
    dst: Picture,
    src_x: i16,
    src_y: i16,
    mask_x: i16,
    mask_y: i16,
    dst_x: i16,
    dst_y: i16,
    width: u16,
    height: u16,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<Picture>,
{
    let mask: Picture = mask.into();
    let request0 = CompositeRequest {
        op,
        src,
        mask,
        dst,
        src_x,
        src_y,
        mask_x,
        mask_y,
        dst_x,
        dst_y,
        width,
        height,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn trapezoids<'c, 'input>(
    conn: &'c mut SocketConnection,
    op: PictOp,
    src: Picture,
    dst: Picture,
    mask_format: Pictformat,
    src_x: i16,
    src_y: i16,
    traps: &'input [Trapezoid],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = TrapezoidsRequest {
        op,
        src,
        dst,
        mask_format,
        src_x,
        src_y,
        traps: Cow::Borrowed(traps),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn triangles<'c, 'input>(
    conn: &'c mut SocketConnection,
    op: PictOp,
    src: Picture,
    dst: Picture,
    mask_format: Pictformat,
    src_x: i16,
    src_y: i16,
    triangles: &'input [Triangle],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = TrianglesRequest {
        op,
        src,
        dst,
        mask_format,
        src_x,
        src_y,
        triangles: Cow::Borrowed(triangles),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn tri_strip<'c, 'input>(
    conn: &'c mut SocketConnection,
    op: PictOp,
    src: Picture,
    dst: Picture,
    mask_format: Pictformat,
    src_x: i16,
    src_y: i16,
    points: &'input [Pointfix],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = TriStripRequest {
        op,
        src,
        dst,
        mask_format,
        src_x,
        src_y,
        points: Cow::Borrowed(points),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn tri_fan<'c, 'input>(
    conn: &'c mut SocketConnection,
    op: PictOp,
    src: Picture,
    dst: Picture,
    mask_format: Pictformat,
    src_x: i16,
    src_y: i16,
    points: &'input [Pointfix],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = TriFanRequest {
        op,
        src,
        dst,
        mask_format,
        src_x,
        src_y,
        points: Cow::Borrowed(points),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_glyph_set(
    conn: &mut SocketConnection,
    gsid: Glyphset,
    format: Pictformat,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateGlyphSetRequest { gsid, format };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn reference_glyph_set(
    conn: &mut SocketConnection,
    gsid: Glyphset,
    existing: Glyphset,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = ReferenceGlyphSetRequest { gsid, existing };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn free_glyph_set(
    conn: &mut SocketConnection,
    glyphset: Glyphset,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = FreeGlyphSetRequest { glyphset };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn add_glyphs<'c, 'input>(
    conn: &'c mut SocketConnection,
    glyphset: Glyphset,
    glyphids: &'input [u32],
    glyphs: &'input [Glyphinfo],
    data: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = AddGlyphsRequest {
        glyphset,
        glyphids: Cow::Borrowed(glyphids),
        glyphs: Cow::Borrowed(glyphs),
        data: Cow::Borrowed(data),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn free_glyphs<'c, 'input>(
    conn: &'c mut SocketConnection,
    glyphset: Glyphset,
    glyphs: &'input [Glyph],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = FreeGlyphsRequest {
        glyphset,
        glyphs: Cow::Borrowed(glyphs),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn composite_glyphs8<'c, 'input>(
    conn: &'c mut SocketConnection,
    op: PictOp,
    src: Picture,
    dst: Picture,
    mask_format: Pictformat,
    glyphset: Glyphset,
    src_x: i16,
    src_y: i16,
    glyphcmds: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CompositeGlyphs8Request {
        op,
        src,
        dst,
        mask_format,
        glyphset,
        src_x,
        src_y,
        glyphcmds: Cow::Borrowed(glyphcmds),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn composite_glyphs16<'c, 'input>(
    conn: &'c mut SocketConnection,
    op: PictOp,
    src: Picture,
    dst: Picture,
    mask_format: Pictformat,
    glyphset: Glyphset,
    src_x: i16,
    src_y: i16,
    glyphcmds: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CompositeGlyphs16Request {
        op,
        src,
        dst,
        mask_format,
        glyphset,
        src_x,
        src_y,
        glyphcmds: Cow::Borrowed(glyphcmds),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn composite_glyphs32<'c, 'input>(
    conn: &'c mut SocketConnection,
    op: PictOp,
    src: Picture,
    dst: Picture,
    mask_format: Pictformat,
    glyphset: Glyphset,
    src_x: i16,
    src_y: i16,
    glyphcmds: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CompositeGlyphs32Request {
        op,
        src,
        dst,
        mask_format,
        glyphset,
        src_x,
        src_y,
        glyphcmds: Cow::Borrowed(glyphcmds),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn fill_rectangles<'c, 'input>(
    conn: &'c mut SocketConnection,
    op: PictOp,
    dst: Picture,
    color: Color,
    rects: &'input [xproto::Rectangle],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = FillRectanglesRequest {
        op,
        dst,
        color,
        rects: Cow::Borrowed(rects),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_cursor(
    conn: &mut SocketConnection,
    cid: xproto::Cursor,
    source: Picture,
    x: u16,
    y: u16,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateCursorRequest { cid, source, x, y };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_picture_transform(
    conn: &mut SocketConnection,
    picture: Picture,
    transform: Transform,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetPictureTransformRequest { picture, transform };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_filters(
    conn: &mut SocketConnection,
    drawable: xproto::Drawable,
    forget: bool,
) -> Result<Cookie<QueryFiltersReply>, ConnectionError> {
    let request0 = QueryFiltersRequest { drawable };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_picture_filter<'c, 'input>(
    conn: &'c mut SocketConnection,
    picture: Picture,
    filter: &'input [u8],
    values: &'input [Fixed],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetPictureFilterRequest {
        picture,
        filter: Cow::Borrowed(filter),
        values: Cow::Borrowed(values),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_anim_cursor<'c, 'input>(
    conn: &'c mut SocketConnection,
    cid: xproto::Cursor,
    cursors: &'input [Animcursorelt],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateAnimCursorRequest {
        cid,
        cursors: Cow::Borrowed(cursors),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn add_traps<'c, 'input>(
    conn: &'c mut SocketConnection,
    picture: Picture,
    x_off: i16,
    y_off: i16,
    traps: &'input [Trap],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = AddTrapsRequest {
        picture,
        x_off,
        y_off,
        traps: Cow::Borrowed(traps),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_solid_fill(
    conn: &mut SocketConnection,
    picture: Picture,
    color: Color,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateSolidFillRequest { picture, color };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_linear_gradient<'c, 'input>(
    conn: &'c mut SocketConnection,
    picture: Picture,
    p1: Pointfix,
    p2: Pointfix,
    stops: &'input [Fixed],
    colors: &'input [Color],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateLinearGradientRequest {
        picture,
        p1,
        p2,
        stops: Cow::Borrowed(stops),
        colors: Cow::Borrowed(colors),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_radial_gradient<'c, 'input>(
    conn: &'c mut SocketConnection,
    picture: Picture,
    inner: Pointfix,
    outer: Pointfix,
    inner_radius: Fixed,
    outer_radius: Fixed,
    stops: &'input [Fixed],
    colors: &'input [Color],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateRadialGradientRequest {
        picture,
        inner,
        outer,
        inner_radius,
        outer_radius,
        stops: Cow::Borrowed(stops),
        colors: Cow::Borrowed(colors),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_conical_gradient<'c, 'input>(
    conn: &'c mut SocketConnection,
    picture: Picture,
    center: Pointfix,
    angle: Fixed,
    stops: &'input [Fixed],
    colors: &'input [Color],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateConicalGradientRequest {
        picture,
        center,
        angle,
        stops: Cow::Borrowed(stops),
        colors: Cow::Borrowed(colors),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}
