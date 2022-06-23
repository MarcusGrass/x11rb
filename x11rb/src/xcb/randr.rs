// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `RandR` X11 extension.

#![allow(clippy::too_many_arguments)]

#[allow(unused_imports)]
use super::render;
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

pub use crate::protocol::randr::*;

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

pub fn set_screen_config<A>(
    conn: &mut SocketConnection,
    window: xproto::Window,
    timestamp: xproto::Timestamp,
    config_timestamp: xproto::Timestamp,
    size_id: u16,
    rotation: A,
    rate: u16,
    forget: bool,
) -> Result<Cookie<SetScreenConfigReply>, ConnectionError>
where
    A: Into<u16>,
{
    let rotation: u16 = rotation.into();
    let request0 = SetScreenConfigRequest {
        window,
        timestamp,
        config_timestamp,
        size_id,
        rotation,
        rate,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn select_input<A>(
    conn: &mut SocketConnection,
    window: xproto::Window,
    enable: A,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<u16>,
{
    let enable: u16 = enable.into();
    let request0 = SelectInputRequest { window, enable };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_screen_info(
    conn: &mut SocketConnection,
    window: xproto::Window,
    forget: bool,
) -> Result<Cookie<GetScreenInfoReply>, ConnectionError> {
    let request0 = GetScreenInfoRequest { window };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_screen_size_range(
    conn: &mut SocketConnection,
    window: xproto::Window,
    forget: bool,
) -> Result<Cookie<GetScreenSizeRangeReply>, ConnectionError> {
    let request0 = GetScreenSizeRangeRequest { window };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_screen_size(
    conn: &mut SocketConnection,
    window: xproto::Window,
    width: u16,
    height: u16,
    mm_width: u32,
    mm_height: u32,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetScreenSizeRequest {
        window,
        width,
        height,
        mm_width,
        mm_height,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_screen_resources(
    conn: &mut SocketConnection,
    window: xproto::Window,
    forget: bool,
) -> Result<Cookie<GetScreenResourcesReply>, ConnectionError> {
    let request0 = GetScreenResourcesRequest { window };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_output_info(
    conn: &mut SocketConnection,
    output: Output,
    config_timestamp: xproto::Timestamp,
    forget: bool,
) -> Result<Cookie<GetOutputInfoReply>, ConnectionError> {
    let request0 = GetOutputInfoRequest {
        output,
        config_timestamp,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn list_output_properties(
    conn: &mut SocketConnection,
    output: Output,
    forget: bool,
) -> Result<Cookie<ListOutputPropertiesReply>, ConnectionError> {
    let request0 = ListOutputPropertiesRequest { output };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_output_property(
    conn: &mut SocketConnection,
    output: Output,
    property: xproto::Atom,
    forget: bool,
) -> Result<Cookie<QueryOutputPropertyReply>, ConnectionError> {
    let request0 = QueryOutputPropertyRequest { output, property };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn configure_output_property<'c, 'input>(
    conn: &'c mut SocketConnection,
    output: Output,
    property: xproto::Atom,
    pending: bool,
    range: bool,
    values: &'input [i32],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = ConfigureOutputPropertyRequest {
        output,
        property,
        pending,
        range,
        values: Cow::Borrowed(values),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn change_output_property<'c, 'input>(
    conn: &'c mut SocketConnection,
    output: Output,
    property: xproto::Atom,
    type_: xproto::Atom,
    format: u8,
    mode: xproto::PropMode,
    num_units: u32,
    data: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = ChangeOutputPropertyRequest {
        output,
        property,
        type_,
        format,
        mode,
        num_units,
        data: Cow::Borrowed(data),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn delete_output_property(
    conn: &mut SocketConnection,
    output: Output,
    property: xproto::Atom,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DeleteOutputPropertyRequest { output, property };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_output_property<A>(
    conn: &mut SocketConnection,
    output: Output,
    property: xproto::Atom,
    type_: A,
    long_offset: u32,
    long_length: u32,
    delete: bool,
    pending: bool,
    forget: bool,
) -> Result<Cookie<GetOutputPropertyReply>, ConnectionError>
where
    A: Into<xproto::Atom>,
{
    let type_: xproto::Atom = type_.into();
    let request0 = GetOutputPropertyRequest {
        output,
        property,
        type_,
        long_offset,
        long_length,
        delete,
        pending,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_mode<'c, 'input>(
    conn: &'c mut SocketConnection,
    window: xproto::Window,
    mode_info: ModeInfo,
    name: &'input [u8],
    forget: bool,
) -> Result<Cookie<CreateModeReply>, ConnectionError> {
    let request0 = CreateModeRequest {
        window,
        mode_info,
        name: Cow::Borrowed(name),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn destroy_mode(
    conn: &mut SocketConnection,
    mode: Mode,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DestroyModeRequest { mode };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn add_output_mode(
    conn: &mut SocketConnection,
    output: Output,
    mode: Mode,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = AddOutputModeRequest { output, mode };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn delete_output_mode(
    conn: &mut SocketConnection,
    output: Output,
    mode: Mode,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DeleteOutputModeRequest { output, mode };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_crtc_info(
    conn: &mut SocketConnection,
    crtc: Crtc,
    config_timestamp: xproto::Timestamp,
    forget: bool,
) -> Result<Cookie<GetCrtcInfoReply>, ConnectionError> {
    let request0 = GetCrtcInfoRequest {
        crtc,
        config_timestamp,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_crtc_config<'c, 'input, A>(
    conn: &'c mut SocketConnection,
    crtc: Crtc,
    timestamp: xproto::Timestamp,
    config_timestamp: xproto::Timestamp,
    x: i16,
    y: i16,
    mode: Mode,
    rotation: A,
    outputs: &'input [Output],
    forget: bool,
) -> Result<Cookie<SetCrtcConfigReply>, ConnectionError>
where
    A: Into<u16>,
{
    let rotation: u16 = rotation.into();
    let request0 = SetCrtcConfigRequest {
        crtc,
        timestamp,
        config_timestamp,
        x,
        y,
        mode,
        rotation,
        outputs: Cow::Borrowed(outputs),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_crtc_gamma_size(
    conn: &mut SocketConnection,
    crtc: Crtc,
    forget: bool,
) -> Result<Cookie<GetCrtcGammaSizeReply>, ConnectionError> {
    let request0 = GetCrtcGammaSizeRequest { crtc };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_crtc_gamma(
    conn: &mut SocketConnection,
    crtc: Crtc,
    forget: bool,
) -> Result<Cookie<GetCrtcGammaReply>, ConnectionError> {
    let request0 = GetCrtcGammaRequest { crtc };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_crtc_gamma<'c, 'input>(
    conn: &'c mut SocketConnection,
    crtc: Crtc,
    red: &'input [u16],
    green: &'input [u16],
    blue: &'input [u16],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetCrtcGammaRequest {
        crtc,
        red: Cow::Borrowed(red),
        green: Cow::Borrowed(green),
        blue: Cow::Borrowed(blue),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_screen_resources_current(
    conn: &mut SocketConnection,
    window: xproto::Window,
    forget: bool,
) -> Result<Cookie<GetScreenResourcesCurrentReply>, ConnectionError> {
    let request0 = GetScreenResourcesCurrentRequest { window };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_crtc_transform<'c, 'input>(
    conn: &'c mut SocketConnection,
    crtc: Crtc,
    transform: render::Transform,
    filter_name: &'input [u8],
    filter_params: &'input [render::Fixed],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetCrtcTransformRequest {
        crtc,
        transform,
        filter_name: Cow::Borrowed(filter_name),
        filter_params: Cow::Borrowed(filter_params),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_crtc_transform(
    conn: &mut SocketConnection,
    crtc: Crtc,
    forget: bool,
) -> Result<Cookie<GetCrtcTransformReply>, ConnectionError> {
    let request0 = GetCrtcTransformRequest { crtc };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_panning(
    conn: &mut SocketConnection,
    crtc: Crtc,
    forget: bool,
) -> Result<Cookie<GetPanningReply>, ConnectionError> {
    let request0 = GetPanningRequest { crtc };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_panning(
    conn: &mut SocketConnection,
    crtc: Crtc,
    timestamp: xproto::Timestamp,
    left: u16,
    top: u16,
    width: u16,
    height: u16,
    track_left: u16,
    track_top: u16,
    track_width: u16,
    track_height: u16,
    border_left: i16,
    border_top: i16,
    border_right: i16,
    border_bottom: i16,
    forget: bool,
) -> Result<Cookie<SetPanningReply>, ConnectionError> {
    let request0 = SetPanningRequest {
        crtc,
        timestamp,
        left,
        top,
        width,
        height,
        track_left,
        track_top,
        track_width,
        track_height,
        border_left,
        border_top,
        border_right,
        border_bottom,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_output_primary(
    conn: &mut SocketConnection,
    window: xproto::Window,
    output: Output,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetOutputPrimaryRequest { window, output };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_output_primary(
    conn: &mut SocketConnection,
    window: xproto::Window,
    forget: bool,
) -> Result<Cookie<GetOutputPrimaryReply>, ConnectionError> {
    let request0 = GetOutputPrimaryRequest { window };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_providers(
    conn: &mut SocketConnection,
    window: xproto::Window,
    forget: bool,
) -> Result<Cookie<GetProvidersReply>, ConnectionError> {
    let request0 = GetProvidersRequest { window };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_provider_info(
    conn: &mut SocketConnection,
    provider: Provider,
    config_timestamp: xproto::Timestamp,
    forget: bool,
) -> Result<Cookie<GetProviderInfoReply>, ConnectionError> {
    let request0 = GetProviderInfoRequest {
        provider,
        config_timestamp,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_provider_offload_sink(
    conn: &mut SocketConnection,
    provider: Provider,
    sink_provider: Provider,
    config_timestamp: xproto::Timestamp,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetProviderOffloadSinkRequest {
        provider,
        sink_provider,
        config_timestamp,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_provider_output_source(
    conn: &mut SocketConnection,
    provider: Provider,
    source_provider: Provider,
    config_timestamp: xproto::Timestamp,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetProviderOutputSourceRequest {
        provider,
        source_provider,
        config_timestamp,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn list_provider_properties(
    conn: &mut SocketConnection,
    provider: Provider,
    forget: bool,
) -> Result<Cookie<ListProviderPropertiesReply>, ConnectionError> {
    let request0 = ListProviderPropertiesRequest { provider };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_provider_property(
    conn: &mut SocketConnection,
    provider: Provider,
    property: xproto::Atom,
    forget: bool,
) -> Result<Cookie<QueryProviderPropertyReply>, ConnectionError> {
    let request0 = QueryProviderPropertyRequest { provider, property };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn configure_provider_property<'c, 'input>(
    conn: &'c mut SocketConnection,
    provider: Provider,
    property: xproto::Atom,
    pending: bool,
    range: bool,
    values: &'input [i32],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = ConfigureProviderPropertyRequest {
        provider,
        property,
        pending,
        range,
        values: Cow::Borrowed(values),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn change_provider_property<'c, 'input>(
    conn: &'c mut SocketConnection,
    provider: Provider,
    property: xproto::Atom,
    type_: xproto::Atom,
    format: u8,
    mode: u8,
    num_items: u32,
    data: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = ChangeProviderPropertyRequest {
        provider,
        property,
        type_,
        format,
        mode,
        num_items,
        data: Cow::Borrowed(data),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn delete_provider_property(
    conn: &mut SocketConnection,
    provider: Provider,
    property: xproto::Atom,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DeleteProviderPropertyRequest { provider, property };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_provider_property(
    conn: &mut SocketConnection,
    provider: Provider,
    property: xproto::Atom,
    type_: xproto::Atom,
    long_offset: u32,
    long_length: u32,
    delete: bool,
    pending: bool,
    forget: bool,
) -> Result<Cookie<GetProviderPropertyReply>, ConnectionError> {
    let request0 = GetProviderPropertyRequest {
        provider,
        property,
        type_,
        long_offset,
        long_length,
        delete,
        pending,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_monitors(
    conn: &mut SocketConnection,
    window: xproto::Window,
    get_active: bool,
    forget: bool,
) -> Result<Cookie<GetMonitorsReply>, ConnectionError> {
    let request0 = GetMonitorsRequest { window, get_active };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_monitor(
    conn: &mut SocketConnection,
    window: xproto::Window,
    monitorinfo: MonitorInfo,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetMonitorRequest {
        window,
        monitorinfo,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn delete_monitor(
    conn: &mut SocketConnection,
    window: xproto::Window,
    name: xproto::Atom,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DeleteMonitorRequest { window, name };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn free_lease(
    conn: &mut SocketConnection,
    lid: Lease,
    terminate: u8,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = FreeLeaseRequest { lid, terminate };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}
