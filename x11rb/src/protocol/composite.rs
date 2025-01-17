// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Composite` X11 extension.

#![allow(clippy::too_many_arguments)]

#[allow(unused_imports)]
use super::xfixes;
#[allow(unused_imports)]
use super::xproto;
use crate::errors::ParseError;
#[allow(unused_imports)]
use crate::utils::{pretty_print_bitmask, pretty_print_enum};
#[allow(unused_imports)]
use crate::x11_utils::TryIntoUSize;
#[allow(unused_imports)]
use crate::x11_utils::{RequestHeader, Serialize, TryParse, TryParseFd};
#[allow(unused_imports)]
use core::convert::TryFrom;
#[allow(unused_imports)]
use std::borrow::Cow;
#[allow(unused_imports)]
use std::convert::TryInto;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "Composite";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (0, 4);

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Redirect(u8);
impl Redirect {
    pub const AUTOMATIC: Self = Self(0);
    pub const MANUAL: Self = Self(1);
}
impl From<Redirect> for u8 {
    #[inline]
    fn from(input: Redirect) -> Self {
        input.0
    }
}
impl From<Redirect> for Option<u8> {
    #[inline]
    fn from(input: Redirect) -> Self {
        Some(input.0)
    }
}
impl From<Redirect> for u16 {
    #[inline]
    fn from(input: Redirect) -> Self {
        u16::from(input.0)
    }
}
impl From<Redirect> for Option<u16> {
    #[inline]
    fn from(input: Redirect) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<Redirect> for u32 {
    #[inline]
    fn from(input: Redirect) -> Self {
        u32::from(input.0)
    }
}
impl From<Redirect> for Option<u32> {
    #[inline]
    fn from(input: Redirect) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for Redirect {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for Redirect {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::AUTOMATIC.0.into(), "AUTOMATIC", "Automatic"),
            (Self::MANUAL.0.into(), "MANUAL", "Manual"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryVersionRequest {
    pub client_major_version: u32,
    pub client_minor_version: u32,
}
impl QueryVersionRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let client_major_version_bytes = self.client_major_version.serialize();
        let client_minor_version_bytes = self.client_minor_version.serialize();
        let mut request0 = [
            major_opcode,
            QUERY_VERSION_REQUEST,
            0,
            0,
            client_major_version_bytes[0],
            client_major_version_bytes[1],
            client_major_version_bytes[2],
            client_major_version_bytes[3],
            client_minor_version_bytes[0],
            client_minor_version_bytes[1],
            client_minor_version_bytes[2],
            client_minor_version_bytes[3],
        ];
        request0[2..4].copy_from_slice(&(3u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryVersionReply {
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
}
impl TryParse for QueryVersionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major_version, remaining) = u32::try_parse(remaining)?;
        let (minor_version, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryVersionReply {
            sequence,
            length,
            major_version,
            minor_version,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the RedirectWindow request
pub const REDIRECT_WINDOW_REQUEST: u8 = 1;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RedirectWindowRequest {
    pub window: xproto::Window,
    pub update: Redirect,
}
impl RedirectWindowRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let window_bytes = self.window.serialize();
        let update_bytes = u8::from(self.update).serialize();
        let mut request0 = [
            major_opcode,
            REDIRECT_WINDOW_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            update_bytes[0],
            0,
            0,
            0,
        ];
        request0[2..4].copy_from_slice(&(3u16).to_ne_bytes());
        request0
    }
}

/// Opcode for the RedirectSubwindows request
pub const REDIRECT_SUBWINDOWS_REQUEST: u8 = 2;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RedirectSubwindowsRequest {
    pub window: xproto::Window,
    pub update: Redirect,
}
impl RedirectSubwindowsRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let window_bytes = self.window.serialize();
        let update_bytes = u8::from(self.update).serialize();
        let mut request0 = [
            major_opcode,
            REDIRECT_SUBWINDOWS_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            update_bytes[0],
            0,
            0,
            0,
        ];
        request0[2..4].copy_from_slice(&(3u16).to_ne_bytes());
        request0
    }
}

/// Opcode for the UnredirectWindow request
pub const UNREDIRECT_WINDOW_REQUEST: u8 = 3;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnredirectWindowRequest {
    pub window: xproto::Window,
    pub update: Redirect,
}
impl UnredirectWindowRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let window_bytes = self.window.serialize();
        let update_bytes = u8::from(self.update).serialize();
        let mut request0 = [
            major_opcode,
            UNREDIRECT_WINDOW_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            update_bytes[0],
            0,
            0,
            0,
        ];
        request0[2..4].copy_from_slice(&(3u16).to_ne_bytes());
        request0
    }
}

/// Opcode for the UnredirectSubwindows request
pub const UNREDIRECT_SUBWINDOWS_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnredirectSubwindowsRequest {
    pub window: xproto::Window,
    pub update: Redirect,
}
impl UnredirectSubwindowsRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let window_bytes = self.window.serialize();
        let update_bytes = u8::from(self.update).serialize();
        let mut request0 = [
            major_opcode,
            UNREDIRECT_SUBWINDOWS_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            update_bytes[0],
            0,
            0,
            0,
        ];
        request0[2..4].copy_from_slice(&(3u16).to_ne_bytes());
        request0
    }
}

/// Opcode for the CreateRegionFromBorderClip request
pub const CREATE_REGION_FROM_BORDER_CLIP_REQUEST: u8 = 5;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateRegionFromBorderClipRequest {
    pub region: xfixes::Region,
    pub window: xproto::Window,
}
impl CreateRegionFromBorderClipRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let region_bytes = self.region.serialize();
        let window_bytes = self.window.serialize();
        let mut request0 = [
            major_opcode,
            CREATE_REGION_FROM_BORDER_CLIP_REQUEST,
            0,
            0,
            region_bytes[0],
            region_bytes[1],
            region_bytes[2],
            region_bytes[3],
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
        ];
        request0[2..4].copy_from_slice(&(3u16).to_ne_bytes());
        request0
    }
}

/// Opcode for the NameWindowPixmap request
pub const NAME_WINDOW_PIXMAP_REQUEST: u8 = 6;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NameWindowPixmapRequest {
    pub window: xproto::Window,
    pub pixmap: xproto::Pixmap,
}
impl NameWindowPixmapRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let window_bytes = self.window.serialize();
        let pixmap_bytes = self.pixmap.serialize();
        let mut request0 = [
            major_opcode,
            NAME_WINDOW_PIXMAP_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            pixmap_bytes[0],
            pixmap_bytes[1],
            pixmap_bytes[2],
            pixmap_bytes[3],
        ];
        request0[2..4].copy_from_slice(&(3u16).to_ne_bytes());
        request0
    }
}

/// Opcode for the GetOverlayWindow request
pub const GET_OVERLAY_WINDOW_REQUEST: u8 = 7;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetOverlayWindowRequest {
    pub window: xproto::Window,
}
impl GetOverlayWindowRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let window_bytes = self.window.serialize();
        let mut request0 = [
            major_opcode,
            GET_OVERLAY_WINDOW_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
        ];
        request0[2..4].copy_from_slice(&(2u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetOverlayWindowReply {
    pub sequence: u16,
    pub length: u32,
    pub overlay_win: xproto::Window,
}
impl TryParse for GetOverlayWindowReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (overlay_win, remaining) = xproto::Window::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetOverlayWindowReply {
            sequence,
            length,
            overlay_win,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the ReleaseOverlayWindow request
pub const RELEASE_OVERLAY_WINDOW_REQUEST: u8 = 8;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReleaseOverlayWindowRequest {
    pub window: xproto::Window,
}
impl ReleaseOverlayWindowRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let window_bytes = self.window.serialize();
        let mut request0 = [
            major_opcode,
            RELEASE_OVERLAY_WINDOW_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
        ];
        request0[2..4].copy_from_slice(&(2u16).to_ne_bytes());
        request0
    }
}
