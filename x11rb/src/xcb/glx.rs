// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Glx` X11 extension.

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

pub use crate::protocol::glx::*;

/// Get the major opcode of this extension
fn major_opcode(conn: &SocketConnection) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME);
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub fn render<'c, 'input>(
    conn: &'c mut SocketConnection,
    context_tag: ContextTag,
    data: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = RenderRequest {
        context_tag,
        data: Cow::Borrowed(data),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn render_large<'c, 'input>(
    conn: &'c mut SocketConnection,
    context_tag: ContextTag,
    request_num: u16,
    request_total: u16,
    data: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = RenderLargeRequest {
        context_tag,
        request_num,
        request_total,
        data: Cow::Borrowed(data),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_context(
    conn: &mut SocketConnection,
    context: Context,
    visual: xproto::Visualid,
    screen: u32,
    share_list: Context,
    is_direct: bool,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateContextRequest {
        context,
        visual,
        screen,
        share_list,
        is_direct,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn destroy_context(
    conn: &mut SocketConnection,
    context: Context,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DestroyContextRequest { context };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn make_current(
    conn: &mut SocketConnection,
    drawable: Drawable,
    context: Context,
    old_context_tag: ContextTag,
    forget: bool,
) -> Result<Cookie<MakeCurrentReply>, ConnectionError> {
    let request0 = MakeCurrentRequest {
        drawable,
        context,
        old_context_tag,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn is_direct(
    conn: &mut SocketConnection,
    context: Context,
    forget: bool,
) -> Result<Cookie<IsDirectReply>, ConnectionError> {
    let request0 = IsDirectRequest { context };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
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

pub fn wait_gl(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = WaitGLRequest { context_tag };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn wait_x(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = WaitXRequest { context_tag };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn copy_context(
    conn: &mut SocketConnection,
    src: Context,
    dest: Context,
    mask: u32,
    src_context_tag: ContextTag,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CopyContextRequest {
        src,
        dest,
        mask,
        src_context_tag,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn swap_buffers(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    drawable: Drawable,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SwapBuffersRequest {
        context_tag,
        drawable,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn use_x_font(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    font: xproto::Font,
    first: u32,
    count: u32,
    list_base: u32,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = UseXFontRequest {
        context_tag,
        font,
        first,
        count,
        list_base,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_glx_pixmap(
    conn: &mut SocketConnection,
    screen: u32,
    visual: xproto::Visualid,
    pixmap: xproto::Pixmap,
    glx_pixmap: Pixmap,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateGLXPixmapRequest {
        screen,
        visual,
        pixmap,
        glx_pixmap,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_visual_configs(
    conn: &mut SocketConnection,
    screen: u32,
    forget: bool,
) -> Result<Cookie<GetVisualConfigsReply>, ConnectionError> {
    let request0 = GetVisualConfigsRequest { screen };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn destroy_glx_pixmap(
    conn: &mut SocketConnection,
    glx_pixmap: Pixmap,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DestroyGLXPixmapRequest { glx_pixmap };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn vendor_private<'c, 'input>(
    conn: &'c mut SocketConnection,
    vendor_code: u32,
    context_tag: ContextTag,
    data: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = VendorPrivateRequest {
        vendor_code,
        context_tag,
        data: Cow::Borrowed(data),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn vendor_private_with_reply<'c, 'input>(
    conn: &'c mut SocketConnection,
    vendor_code: u32,
    context_tag: ContextTag,
    data: &'input [u8],
    forget: bool,
) -> Result<Cookie<VendorPrivateWithReplyReply>, ConnectionError> {
    let request0 = VendorPrivateWithReplyRequest {
        vendor_code,
        context_tag,
        data: Cow::Borrowed(data),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_extensions_string(
    conn: &mut SocketConnection,
    screen: u32,
    forget: bool,
) -> Result<Cookie<QueryExtensionsStringReply>, ConnectionError> {
    let request0 = QueryExtensionsStringRequest { screen };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_server_string(
    conn: &mut SocketConnection,
    screen: u32,
    name: u32,
    forget: bool,
) -> Result<Cookie<QueryServerStringReply>, ConnectionError> {
    let request0 = QueryServerStringRequest { screen, name };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn client_info<'c, 'input>(
    conn: &'c mut SocketConnection,
    major_version: u32,
    minor_version: u32,
    string: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = ClientInfoRequest {
        major_version,
        minor_version,
        string: Cow::Borrowed(string),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_fb_configs(
    conn: &mut SocketConnection,
    screen: u32,
    forget: bool,
) -> Result<Cookie<GetFBConfigsReply>, ConnectionError> {
    let request0 = GetFBConfigsRequest { screen };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_pixmap<'c, 'input>(
    conn: &'c mut SocketConnection,
    screen: u32,
    fbconfig: Fbconfig,
    pixmap: xproto::Pixmap,
    glx_pixmap: Pixmap,
    attribs: &'input [u32],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreatePixmapRequest {
        screen,
        fbconfig,
        pixmap,
        glx_pixmap,
        attribs: Cow::Borrowed(attribs),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn destroy_pixmap(
    conn: &mut SocketConnection,
    glx_pixmap: Pixmap,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DestroyPixmapRequest { glx_pixmap };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_new_context(
    conn: &mut SocketConnection,
    context: Context,
    fbconfig: Fbconfig,
    screen: u32,
    render_type: u32,
    share_list: Context,
    is_direct: bool,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateNewContextRequest {
        context,
        fbconfig,
        screen,
        render_type,
        share_list,
        is_direct,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_context(
    conn: &mut SocketConnection,
    context: Context,
    forget: bool,
) -> Result<Cookie<QueryContextReply>, ConnectionError> {
    let request0 = QueryContextRequest { context };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn make_context_current(
    conn: &mut SocketConnection,
    old_context_tag: ContextTag,
    drawable: Drawable,
    read_drawable: Drawable,
    context: Context,
    forget: bool,
) -> Result<Cookie<MakeContextCurrentReply>, ConnectionError> {
    let request0 = MakeContextCurrentRequest {
        old_context_tag,
        drawable,
        read_drawable,
        context,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_pbuffer<'c, 'input>(
    conn: &'c mut SocketConnection,
    screen: u32,
    fbconfig: Fbconfig,
    pbuffer: Pbuffer,
    attribs: &'input [u32],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreatePbufferRequest {
        screen,
        fbconfig,
        pbuffer,
        attribs: Cow::Borrowed(attribs),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn destroy_pbuffer(
    conn: &mut SocketConnection,
    pbuffer: Pbuffer,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DestroyPbufferRequest { pbuffer };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_drawable_attributes(
    conn: &mut SocketConnection,
    drawable: Drawable,
    forget: bool,
) -> Result<Cookie<GetDrawableAttributesReply>, ConnectionError> {
    let request0 = GetDrawableAttributesRequest { drawable };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn change_drawable_attributes<'c, 'input>(
    conn: &'c mut SocketConnection,
    drawable: Drawable,
    attribs: &'input [u32],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = ChangeDrawableAttributesRequest {
        drawable,
        attribs: Cow::Borrowed(attribs),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_window<'c, 'input>(
    conn: &'c mut SocketConnection,
    screen: u32,
    fbconfig: Fbconfig,
    window: xproto::Window,
    glx_window: Window,
    attribs: &'input [u32],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateWindowRequest {
        screen,
        fbconfig,
        window,
        glx_window,
        attribs: Cow::Borrowed(attribs),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn delete_window(
    conn: &mut SocketConnection,
    glxwindow: Window,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DeleteWindowRequest { glxwindow };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_client_info_arb<'c, 'input>(
    conn: &'c mut SocketConnection,
    major_version: u32,
    minor_version: u32,
    gl_versions: &'input [u32],
    gl_extension_string: &'input [u8],
    glx_extension_string: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetClientInfoARBRequest {
        major_version,
        minor_version,
        gl_versions: Cow::Borrowed(gl_versions),
        gl_extension_string: Cow::Borrowed(gl_extension_string),
        glx_extension_string: Cow::Borrowed(glx_extension_string),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_context_attribs_arb<'c, 'input>(
    conn: &'c mut SocketConnection,
    context: Context,
    fbconfig: Fbconfig,
    screen: u32,
    share_list: Context,
    is_direct: bool,
    attribs: &'input [u32],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateContextAttribsARBRequest {
        context,
        fbconfig,
        screen,
        share_list,
        is_direct,
        attribs: Cow::Borrowed(attribs),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_client_info2_arb<'c, 'input>(
    conn: &'c mut SocketConnection,
    major_version: u32,
    minor_version: u32,
    gl_versions: &'input [u32],
    gl_extension_string: &'input [u8],
    glx_extension_string: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetClientInfo2ARBRequest {
        major_version,
        minor_version,
        gl_versions: Cow::Borrowed(gl_versions),
        gl_extension_string: Cow::Borrowed(gl_extension_string),
        glx_extension_string: Cow::Borrowed(glx_extension_string),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn new_list(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    list: u32,
    mode: u32,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = NewListRequest {
        context_tag,
        list,
        mode,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn end_list(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = EndListRequest { context_tag };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn delete_lists(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    list: u32,
    range: i32,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DeleteListsRequest {
        context_tag,
        list,
        range,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn gen_lists(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    range: i32,
    forget: bool,
) -> Result<Cookie<GenListsReply>, ConnectionError> {
    let request0 = GenListsRequest { context_tag, range };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn feedback_buffer(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    size: i32,
    type_: i32,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = FeedbackBufferRequest {
        context_tag,
        size,
        type_,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn select_buffer(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    size: i32,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SelectBufferRequest { context_tag, size };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn render_mode(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    mode: u32,
    forget: bool,
) -> Result<Cookie<RenderModeReply>, ConnectionError> {
    let request0 = RenderModeRequest { context_tag, mode };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn finish(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    forget: bool,
) -> Result<Cookie<FinishReply>, ConnectionError> {
    let request0 = FinishRequest { context_tag };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn pixel_storef(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    pname: u32,
    datum: Float32,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = PixelStorefRequest {
        context_tag,
        pname,
        datum,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn pixel_storei(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    pname: u32,
    datum: i32,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = PixelStoreiRequest {
        context_tag,
        pname,
        datum,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn read_pixels(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    format: u32,
    type_: u32,
    swap_bytes: bool,
    lsb_first: bool,
    forget: bool,
) -> Result<Cookie<ReadPixelsReply>, ConnectionError> {
    let request0 = ReadPixelsRequest {
        context_tag,
        x,
        y,
        width,
        height,
        format,
        type_,
        swap_bytes,
        lsb_first,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_booleanv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    pname: i32,
    forget: bool,
) -> Result<Cookie<GetBooleanvReply>, ConnectionError> {
    let request0 = GetBooleanvRequest { context_tag, pname };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_clip_plane(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    plane: i32,
    forget: bool,
) -> Result<Cookie<GetClipPlaneReply>, ConnectionError> {
    let request0 = GetClipPlaneRequest { context_tag, plane };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_doublev(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetDoublevReply>, ConnectionError> {
    let request0 = GetDoublevRequest { context_tag, pname };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_error(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    forget: bool,
) -> Result<Cookie<GetErrorReply>, ConnectionError> {
    let request0 = GetErrorRequest { context_tag };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_floatv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetFloatvReply>, ConnectionError> {
    let request0 = GetFloatvRequest { context_tag, pname };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_integerv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetIntegervReply>, ConnectionError> {
    let request0 = GetIntegervRequest { context_tag, pname };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_lightfv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    light: u32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetLightfvReply>, ConnectionError> {
    let request0 = GetLightfvRequest {
        context_tag,
        light,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_lightiv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    light: u32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetLightivReply>, ConnectionError> {
    let request0 = GetLightivRequest {
        context_tag,
        light,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_mapdv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    query: u32,
    forget: bool,
) -> Result<Cookie<GetMapdvReply>, ConnectionError> {
    let request0 = GetMapdvRequest {
        context_tag,
        target,
        query,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_mapfv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    query: u32,
    forget: bool,
) -> Result<Cookie<GetMapfvReply>, ConnectionError> {
    let request0 = GetMapfvRequest {
        context_tag,
        target,
        query,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_mapiv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    query: u32,
    forget: bool,
) -> Result<Cookie<GetMapivReply>, ConnectionError> {
    let request0 = GetMapivRequest {
        context_tag,
        target,
        query,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_materialfv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    face: u32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetMaterialfvReply>, ConnectionError> {
    let request0 = GetMaterialfvRequest {
        context_tag,
        face,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_materialiv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    face: u32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetMaterialivReply>, ConnectionError> {
    let request0 = GetMaterialivRequest {
        context_tag,
        face,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_pixel_mapfv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    map: u32,
    forget: bool,
) -> Result<Cookie<GetPixelMapfvReply>, ConnectionError> {
    let request0 = GetPixelMapfvRequest { context_tag, map };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_pixel_mapuiv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    map: u32,
    forget: bool,
) -> Result<Cookie<GetPixelMapuivReply>, ConnectionError> {
    let request0 = GetPixelMapuivRequest { context_tag, map };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_pixel_mapusv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    map: u32,
    forget: bool,
) -> Result<Cookie<GetPixelMapusvReply>, ConnectionError> {
    let request0 = GetPixelMapusvRequest { context_tag, map };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_polygon_stipple(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    lsb_first: bool,
    forget: bool,
) -> Result<Cookie<GetPolygonStippleReply>, ConnectionError> {
    let request0 = GetPolygonStippleRequest {
        context_tag,
        lsb_first,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_string(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    name: u32,
    forget: bool,
) -> Result<Cookie<GetStringReply>, ConnectionError> {
    let request0 = GetStringRequest { context_tag, name };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_tex_envfv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetTexEnvfvReply>, ConnectionError> {
    let request0 = GetTexEnvfvRequest {
        context_tag,
        target,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_tex_enviv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetTexEnvivReply>, ConnectionError> {
    let request0 = GetTexEnvivRequest {
        context_tag,
        target,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_tex_gendv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    coord: u32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetTexGendvReply>, ConnectionError> {
    let request0 = GetTexGendvRequest {
        context_tag,
        coord,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_tex_genfv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    coord: u32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetTexGenfvReply>, ConnectionError> {
    let request0 = GetTexGenfvRequest {
        context_tag,
        coord,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_tex_geniv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    coord: u32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetTexGenivReply>, ConnectionError> {
    let request0 = GetTexGenivRequest {
        context_tag,
        coord,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_tex_image(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    level: i32,
    format: u32,
    type_: u32,
    swap_bytes: bool,
    forget: bool,
) -> Result<Cookie<GetTexImageReply>, ConnectionError> {
    let request0 = GetTexImageRequest {
        context_tag,
        target,
        level,
        format,
        type_,
        swap_bytes,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_tex_parameterfv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetTexParameterfvReply>, ConnectionError> {
    let request0 = GetTexParameterfvRequest {
        context_tag,
        target,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_tex_parameteriv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetTexParameterivReply>, ConnectionError> {
    let request0 = GetTexParameterivRequest {
        context_tag,
        target,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_tex_level_parameterfv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    level: i32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetTexLevelParameterfvReply>, ConnectionError> {
    let request0 = GetTexLevelParameterfvRequest {
        context_tag,
        target,
        level,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_tex_level_parameteriv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    level: i32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetTexLevelParameterivReply>, ConnectionError> {
    let request0 = GetTexLevelParameterivRequest {
        context_tag,
        target,
        level,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn is_enabled(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    capability: u32,
    forget: bool,
) -> Result<Cookie<IsEnabledReply>, ConnectionError> {
    let request0 = IsEnabledRequest {
        context_tag,
        capability,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn is_list(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    list: u32,
    forget: bool,
) -> Result<Cookie<IsListReply>, ConnectionError> {
    let request0 = IsListRequest { context_tag, list };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn flush(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = FlushRequest { context_tag };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn are_textures_resident<'c, 'input>(
    conn: &'c mut SocketConnection,
    context_tag: ContextTag,
    textures: &'input [u32],
    forget: bool,
) -> Result<Cookie<AreTexturesResidentReply>, ConnectionError> {
    let request0 = AreTexturesResidentRequest {
        context_tag,
        textures: Cow::Borrowed(textures),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn delete_textures<'c, 'input>(
    conn: &'c mut SocketConnection,
    context_tag: ContextTag,
    textures: &'input [u32],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DeleteTexturesRequest {
        context_tag,
        textures: Cow::Borrowed(textures),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn gen_textures(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    n: i32,
    forget: bool,
) -> Result<Cookie<GenTexturesReply>, ConnectionError> {
    let request0 = GenTexturesRequest { context_tag, n };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn is_texture(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    texture: u32,
    forget: bool,
) -> Result<Cookie<IsTextureReply>, ConnectionError> {
    let request0 = IsTextureRequest {
        context_tag,
        texture,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_color_table(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    format: u32,
    type_: u32,
    swap_bytes: bool,
    forget: bool,
) -> Result<Cookie<GetColorTableReply>, ConnectionError> {
    let request0 = GetColorTableRequest {
        context_tag,
        target,
        format,
        type_,
        swap_bytes,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_color_table_parameterfv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetColorTableParameterfvReply>, ConnectionError> {
    let request0 = GetColorTableParameterfvRequest {
        context_tag,
        target,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_color_table_parameteriv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetColorTableParameterivReply>, ConnectionError> {
    let request0 = GetColorTableParameterivRequest {
        context_tag,
        target,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_convolution_filter(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    format: u32,
    type_: u32,
    swap_bytes: bool,
    forget: bool,
) -> Result<Cookie<GetConvolutionFilterReply>, ConnectionError> {
    let request0 = GetConvolutionFilterRequest {
        context_tag,
        target,
        format,
        type_,
        swap_bytes,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_convolution_parameterfv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetConvolutionParameterfvReply>, ConnectionError> {
    let request0 = GetConvolutionParameterfvRequest {
        context_tag,
        target,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_convolution_parameteriv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetConvolutionParameterivReply>, ConnectionError> {
    let request0 = GetConvolutionParameterivRequest {
        context_tag,
        target,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_separable_filter(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    format: u32,
    type_: u32,
    swap_bytes: bool,
    forget: bool,
) -> Result<Cookie<GetSeparableFilterReply>, ConnectionError> {
    let request0 = GetSeparableFilterRequest {
        context_tag,
        target,
        format,
        type_,
        swap_bytes,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_histogram(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    format: u32,
    type_: u32,
    swap_bytes: bool,
    reset: bool,
    forget: bool,
) -> Result<Cookie<GetHistogramReply>, ConnectionError> {
    let request0 = GetHistogramRequest {
        context_tag,
        target,
        format,
        type_,
        swap_bytes,
        reset,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_histogram_parameterfv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetHistogramParameterfvReply>, ConnectionError> {
    let request0 = GetHistogramParameterfvRequest {
        context_tag,
        target,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_histogram_parameteriv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetHistogramParameterivReply>, ConnectionError> {
    let request0 = GetHistogramParameterivRequest {
        context_tag,
        target,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_minmax(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    format: u32,
    type_: u32,
    swap_bytes: bool,
    reset: bool,
    forget: bool,
) -> Result<Cookie<GetMinmaxReply>, ConnectionError> {
    let request0 = GetMinmaxRequest {
        context_tag,
        target,
        format,
        type_,
        swap_bytes,
        reset,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_minmax_parameterfv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetMinmaxParameterfvReply>, ConnectionError> {
    let request0 = GetMinmaxParameterfvRequest {
        context_tag,
        target,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_minmax_parameteriv(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetMinmaxParameterivReply>, ConnectionError> {
    let request0 = GetMinmaxParameterivRequest {
        context_tag,
        target,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_compressed_tex_image_arb(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    level: i32,
    forget: bool,
) -> Result<Cookie<GetCompressedTexImageARBReply>, ConnectionError> {
    let request0 = GetCompressedTexImageARBRequest {
        context_tag,
        target,
        level,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn delete_queries_arb<'c, 'input>(
    conn: &'c mut SocketConnection,
    context_tag: ContextTag,
    ids: &'input [u32],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DeleteQueriesARBRequest {
        context_tag,
        ids: Cow::Borrowed(ids),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn gen_queries_arb(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    n: i32,
    forget: bool,
) -> Result<Cookie<GenQueriesARBReply>, ConnectionError> {
    let request0 = GenQueriesARBRequest { context_tag, n };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn is_query_arb(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    id: u32,
    forget: bool,
) -> Result<Cookie<IsQueryARBReply>, ConnectionError> {
    let request0 = IsQueryARBRequest { context_tag, id };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_queryiv_arb(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    target: u32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetQueryivARBReply>, ConnectionError> {
    let request0 = GetQueryivARBRequest {
        context_tag,
        target,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_query_objectiv_arb(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    id: u32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetQueryObjectivARBReply>, ConnectionError> {
    let request0 = GetQueryObjectivARBRequest {
        context_tag,
        id,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_query_objectuiv_arb(
    conn: &mut SocketConnection,
    context_tag: ContextTag,
    id: u32,
    pname: u32,
    forget: bool,
) -> Result<Cookie<GetQueryObjectuivARBReply>, ConnectionError> {
    let request0 = GetQueryObjectuivARBRequest {
        context_tag,
        id,
        pname,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}
