// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `XPrint` X11 extension.

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

pub use crate::protocol::xprint::*;

/// Get the major opcode of this extension
fn major_opcode(conn: &SocketConnection) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME);
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub fn print_query_version(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<PrintQueryVersionReply>, ConnectionError> {
    let request0 = PrintQueryVersionRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn print_get_printer_list<'c, 'input>(
    conn: &'c mut SocketConnection,
    printer_name: &'input [String8],
    locale: &'input [String8],
    forget: bool,
) -> Result<Cookie<PrintGetPrinterListReply>, ConnectionError> {
    let request0 = PrintGetPrinterListRequest {
        printer_name: Cow::Borrowed(printer_name),
        locale: Cow::Borrowed(locale),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn print_rehash_printer_list(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = PrintRehashPrinterListRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_context<'c, 'input>(
    conn: &'c mut SocketConnection,
    context_id: u32,
    printer_name: &'input [String8],
    locale: &'input [String8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateContextRequest {
        context_id,
        printer_name: Cow::Borrowed(printer_name),
        locale: Cow::Borrowed(locale),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn print_set_context(
    conn: &mut SocketConnection,
    context: u32,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = PrintSetContextRequest { context };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn print_get_context(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<PrintGetContextReply>, ConnectionError> {
    let request0 = PrintGetContextRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn print_destroy_context(
    conn: &mut SocketConnection,
    context: u32,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = PrintDestroyContextRequest { context };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn print_get_screen_of_context(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<PrintGetScreenOfContextReply>, ConnectionError> {
    let request0 = PrintGetScreenOfContextRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn print_start_job(
    conn: &mut SocketConnection,
    output_mode: u8,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = PrintStartJobRequest { output_mode };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn print_end_job(
    conn: &mut SocketConnection,
    cancel: bool,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = PrintEndJobRequest { cancel };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn print_start_doc(
    conn: &mut SocketConnection,
    driver_mode: u8,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = PrintStartDocRequest { driver_mode };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn print_end_doc(
    conn: &mut SocketConnection,
    cancel: bool,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = PrintEndDocRequest { cancel };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn print_put_document_data<'c, 'input>(
    conn: &'c mut SocketConnection,
    drawable: xproto::Drawable,
    data: &'input [u8],
    doc_format: &'input [String8],
    options: &'input [String8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = PrintPutDocumentDataRequest {
        drawable,
        data: Cow::Borrowed(data),
        doc_format: Cow::Borrowed(doc_format),
        options: Cow::Borrowed(options),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn print_get_document_data(
    conn: &mut SocketConnection,
    context: Pcontext,
    max_bytes: u32,
    forget: bool,
) -> Result<Cookie<PrintGetDocumentDataReply>, ConnectionError> {
    let request0 = PrintGetDocumentDataRequest { context, max_bytes };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn print_start_page(
    conn: &mut SocketConnection,
    window: xproto::Window,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = PrintStartPageRequest { window };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn print_end_page(
    conn: &mut SocketConnection,
    cancel: bool,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = PrintEndPageRequest { cancel };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn print_select_input(
    conn: &mut SocketConnection,
    context: Pcontext,
    event_mask: u32,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = PrintSelectInputRequest {
        context,
        event_mask,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn print_input_selected(
    conn: &mut SocketConnection,
    context: Pcontext,
    forget: bool,
) -> Result<Cookie<PrintInputSelectedReply>, ConnectionError> {
    let request0 = PrintInputSelectedRequest { context };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn print_get_attributes(
    conn: &mut SocketConnection,
    context: Pcontext,
    pool: u8,
    forget: bool,
) -> Result<Cookie<PrintGetAttributesReply>, ConnectionError> {
    let request0 = PrintGetAttributesRequest { context, pool };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn print_get_one_attributes<'c, 'input>(
    conn: &'c mut SocketConnection,
    context: Pcontext,
    pool: u8,
    name: &'input [String8],
    forget: bool,
) -> Result<Cookie<PrintGetOneAttributesReply>, ConnectionError> {
    let request0 = PrintGetOneAttributesRequest {
        context,
        pool,
        name: Cow::Borrowed(name),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn print_set_attributes<'c, 'input>(
    conn: &'c mut SocketConnection,
    context: Pcontext,
    string_len: u32,
    pool: u8,
    rule: u8,
    attributes: &'input [String8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = PrintSetAttributesRequest {
        context,
        string_len,
        pool,
        rule,
        attributes: Cow::Borrowed(attributes),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn print_get_page_dimensions(
    conn: &mut SocketConnection,
    context: Pcontext,
    forget: bool,
) -> Result<Cookie<PrintGetPageDimensionsReply>, ConnectionError> {
    let request0 = PrintGetPageDimensionsRequest { context };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn print_query_screens(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<PrintQueryScreensReply>, ConnectionError> {
    let request0 = PrintQueryScreensRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn print_set_image_resolution(
    conn: &mut SocketConnection,
    context: Pcontext,
    image_resolution: u16,
    forget: bool,
) -> Result<Cookie<PrintSetImageResolutionReply>, ConnectionError> {
    let request0 = PrintSetImageResolutionRequest {
        context,
        image_resolution,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn print_get_image_resolution(
    conn: &mut SocketConnection,
    context: Pcontext,
    forget: bool,
) -> Result<Cookie<PrintGetImageResolutionReply>, ConnectionError> {
    let request0 = PrintGetImageResolutionRequest { context };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}
