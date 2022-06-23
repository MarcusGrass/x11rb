// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `XF86VidMode` X11 extension.

#![allow(clippy::too_many_arguments)]

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

pub use crate::protocol::xf86vidmode::*;

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

pub fn get_mode_line(
    conn: &mut SocketConnection,
    screen: u16,
    forget: bool,
) -> Result<Cookie<GetModeLineReply>, ConnectionError> {
    let request0 = GetModeLineRequest { screen };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn mod_mode_line<'c, 'input, A>(
    conn: &'c mut SocketConnection,
    screen: u32,
    hdisplay: u16,
    hsyncstart: u16,
    hsyncend: u16,
    htotal: u16,
    hskew: u16,
    vdisplay: u16,
    vsyncstart: u16,
    vsyncend: u16,
    vtotal: u16,
    flags: A,
    private: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<u32>,
{
    let flags: u32 = flags.into();
    let request0 = ModModeLineRequest {
        screen,
        hdisplay,
        hsyncstart,
        hsyncend,
        htotal,
        hskew,
        vdisplay,
        vsyncstart,
        vsyncend,
        vtotal,
        flags,
        private: Cow::Borrowed(private),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn switch_mode(
    conn: &mut SocketConnection,
    screen: u16,
    zoom: u16,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SwitchModeRequest { screen, zoom };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_monitor(
    conn: &mut SocketConnection,
    screen: u16,
    forget: bool,
) -> Result<Cookie<GetMonitorReply>, ConnectionError> {
    let request0 = GetMonitorRequest { screen };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn lock_mode_switch(
    conn: &mut SocketConnection,
    screen: u16,
    lock: u16,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = LockModeSwitchRequest { screen, lock };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_all_mode_lines(
    conn: &mut SocketConnection,
    screen: u16,
    forget: bool,
) -> Result<Cookie<GetAllModeLinesReply>, ConnectionError> {
    let request0 = GetAllModeLinesRequest { screen };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn add_mode_line<'c, 'input, A, B>(
    conn: &'c mut SocketConnection,
    screen: u32,
    dotclock: Dotclock,
    hdisplay: u16,
    hsyncstart: u16,
    hsyncend: u16,
    htotal: u16,
    hskew: u16,
    vdisplay: u16,
    vsyncstart: u16,
    vsyncend: u16,
    vtotal: u16,
    flags: A,
    after_dotclock: Dotclock,
    after_hdisplay: u16,
    after_hsyncstart: u16,
    after_hsyncend: u16,
    after_htotal: u16,
    after_hskew: u16,
    after_vdisplay: u16,
    after_vsyncstart: u16,
    after_vsyncend: u16,
    after_vtotal: u16,
    after_flags: B,
    private: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<u32>,
    B: Into<u32>,
{
    let flags: u32 = flags.into();
    let after_flags: u32 = after_flags.into();
    let request0 = AddModeLineRequest {
        screen,
        dotclock,
        hdisplay,
        hsyncstart,
        hsyncend,
        htotal,
        hskew,
        vdisplay,
        vsyncstart,
        vsyncend,
        vtotal,
        flags,
        after_dotclock,
        after_hdisplay,
        after_hsyncstart,
        after_hsyncend,
        after_htotal,
        after_hskew,
        after_vdisplay,
        after_vsyncstart,
        after_vsyncend,
        after_vtotal,
        after_flags,
        private: Cow::Borrowed(private),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn delete_mode_line<'c, 'input, A>(
    conn: &'c mut SocketConnection,
    screen: u32,
    dotclock: Dotclock,
    hdisplay: u16,
    hsyncstart: u16,
    hsyncend: u16,
    htotal: u16,
    hskew: u16,
    vdisplay: u16,
    vsyncstart: u16,
    vsyncend: u16,
    vtotal: u16,
    flags: A,
    private: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<u32>,
{
    let flags: u32 = flags.into();
    let request0 = DeleteModeLineRequest {
        screen,
        dotclock,
        hdisplay,
        hsyncstart,
        hsyncend,
        htotal,
        hskew,
        vdisplay,
        vsyncstart,
        vsyncend,
        vtotal,
        flags,
        private: Cow::Borrowed(private),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn validate_mode_line<'c, 'input, A>(
    conn: &'c mut SocketConnection,
    screen: u32,
    dotclock: Dotclock,
    hdisplay: u16,
    hsyncstart: u16,
    hsyncend: u16,
    htotal: u16,
    hskew: u16,
    vdisplay: u16,
    vsyncstart: u16,
    vsyncend: u16,
    vtotal: u16,
    flags: A,
    private: &'input [u8],
    forget: bool,
) -> Result<Cookie<ValidateModeLineReply>, ConnectionError>
where
    A: Into<u32>,
{
    let flags: u32 = flags.into();
    let request0 = ValidateModeLineRequest {
        screen,
        dotclock,
        hdisplay,
        hsyncstart,
        hsyncend,
        htotal,
        hskew,
        vdisplay,
        vsyncstart,
        vsyncend,
        vtotal,
        flags,
        private: Cow::Borrowed(private),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn switch_to_mode<'c, 'input, A>(
    conn: &'c mut SocketConnection,
    screen: u32,
    dotclock: Dotclock,
    hdisplay: u16,
    hsyncstart: u16,
    hsyncend: u16,
    htotal: u16,
    hskew: u16,
    vdisplay: u16,
    vsyncstart: u16,
    vsyncend: u16,
    vtotal: u16,
    flags: A,
    private: &'input [u8],
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<u32>,
{
    let flags: u32 = flags.into();
    let request0 = SwitchToModeRequest {
        screen,
        dotclock,
        hdisplay,
        hsyncstart,
        hsyncend,
        htotal,
        hskew,
        vdisplay,
        vsyncstart,
        vsyncend,
        vtotal,
        flags,
        private: Cow::Borrowed(private),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_view_port(
    conn: &mut SocketConnection,
    screen: u16,
    forget: bool,
) -> Result<Cookie<GetViewPortReply>, ConnectionError> {
    let request0 = GetViewPortRequest { screen };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_view_port(
    conn: &mut SocketConnection,
    screen: u16,
    x: u32,
    y: u32,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetViewPortRequest { screen, x, y };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_dot_clocks(
    conn: &mut SocketConnection,
    screen: u16,
    forget: bool,
) -> Result<Cookie<GetDotClocksReply>, ConnectionError> {
    let request0 = GetDotClocksRequest { screen };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_client_version(
    conn: &mut SocketConnection,
    major: u16,
    minor: u16,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetClientVersionRequest { major, minor };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_gamma(
    conn: &mut SocketConnection,
    screen: u16,
    red: u32,
    green: u32,
    blue: u32,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetGammaRequest {
        screen,
        red,
        green,
        blue,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_gamma(
    conn: &mut SocketConnection,
    screen: u16,
    forget: bool,
) -> Result<Cookie<GetGammaReply>, ConnectionError> {
    let request0 = GetGammaRequest { screen };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_gamma_ramp(
    conn: &mut SocketConnection,
    screen: u16,
    size: u16,
    forget: bool,
) -> Result<Cookie<GetGammaRampReply>, ConnectionError> {
    let request0 = GetGammaRampRequest { screen, size };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_gamma_ramp<'c, 'input>(
    conn: &'c mut SocketConnection,
    screen: u16,
    size: u16,
    red: &'input [u16],
    green: &'input [u16],
    blue: &'input [u16],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetGammaRampRequest {
        screen,
        size,
        red: Cow::Borrowed(red),
        green: Cow::Borrowed(green),
        blue: Cow::Borrowed(blue),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_gamma_ramp_size(
    conn: &mut SocketConnection,
    screen: u16,
    forget: bool,
) -> Result<Cookie<GetGammaRampSizeReply>, ConnectionError> {
    let request0 = GetGammaRampSizeRequest { screen };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_permissions(
    conn: &mut SocketConnection,
    screen: u16,
    forget: bool,
) -> Result<Cookie<GetPermissionsReply>, ConnectionError> {
    let request0 = GetPermissionsRequest { screen };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}
