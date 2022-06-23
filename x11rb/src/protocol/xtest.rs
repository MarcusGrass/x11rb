// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Test` X11 extension.

#![allow(clippy::too_many_arguments)]

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
pub const X11_EXTENSION_NAME: &str = "XTEST";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (2, 2);

/// Opcode for the GetVersion request
pub const GET_VERSION_REQUEST: u8 = 0;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetVersionRequest {
    pub major_version: u8,
    pub minor_version: u16,
}
impl GetVersionRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let major_version_bytes = self.major_version.serialize();
        let minor_version_bytes = self.minor_version.serialize();
        let mut request0 = [
            major_opcode,
            GET_VERSION_REQUEST,
            0,
            0,
            major_version_bytes[0],
            0,
            minor_version_bytes[0],
            minor_version_bytes[1],
        ];
        request0[2..4].copy_from_slice(&(2u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetVersionReply {
    pub major_version: u8,
    pub sequence: u16,
    pub length: u32,
    pub minor_version: u16,
}
impl TryParse for GetVersionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (major_version, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (minor_version, remaining) = u16::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetVersionReply {
            major_version,
            sequence,
            length,
            minor_version,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cursor(bool);
impl Cursor {
    pub const NONE: Self = Self(false);
    pub const CURRENT: Self = Self(true);
}
impl From<Cursor> for bool {
    #[inline]
    fn from(input: Cursor) -> Self {
        input.0
    }
}
impl From<Cursor> for Option<bool> {
    #[inline]
    fn from(input: Cursor) -> Self {
        Some(input.0)
    }
}
impl From<Cursor> for u8 {
    #[inline]
    fn from(input: Cursor) -> Self {
        u8::from(input.0)
    }
}
impl From<Cursor> for Option<u8> {
    #[inline]
    fn from(input: Cursor) -> Self {
        Some(u8::from(input.0))
    }
}
impl From<Cursor> for u16 {
    #[inline]
    fn from(input: Cursor) -> Self {
        u16::from(input.0)
    }
}
impl From<Cursor> for Option<u16> {
    #[inline]
    fn from(input: Cursor) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<Cursor> for u32 {
    #[inline]
    fn from(input: Cursor) -> Self {
        u32::from(input.0)
    }
}
impl From<Cursor> for Option<u32> {
    #[inline]
    fn from(input: Cursor) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<bool> for Cursor {
    #[inline]
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for Cursor {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::NONE.0.into(), "NONE", "None"),
            (Self::CURRENT.0.into(), "CURRENT", "Current"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

/// Opcode for the CompareCursor request
pub const COMPARE_CURSOR_REQUEST: u8 = 1;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompareCursorRequest {
    pub window: xproto::Window,
    pub cursor: xproto::Cursor,
}
impl CompareCursorRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let window_bytes = self.window.serialize();
        let cursor_bytes = self.cursor.serialize();
        let mut request0 = [
            major_opcode,
            COMPARE_CURSOR_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            cursor_bytes[0],
            cursor_bytes[1],
            cursor_bytes[2],
            cursor_bytes[3],
        ];
        request0[2..4].copy_from_slice(&(3u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompareCursorReply {
    pub same: bool,
    pub sequence: u16,
    pub length: u32,
}
impl TryParse for CompareCursorReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (same, remaining) = bool::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = CompareCursorReply {
            same,
            sequence,
            length,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the FakeInput request
pub const FAKE_INPUT_REQUEST: u8 = 2;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FakeInputRequest {
    pub type_: u8,
    pub detail: u8,
    pub time: u32,
    pub root: xproto::Window,
    pub root_x: i16,
    pub root_y: i16,
    pub deviceid: u8,
}
impl FakeInputRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let type_bytes = self.type_.serialize();
        let detail_bytes = self.detail.serialize();
        let time_bytes = self.time.serialize();
        let root_bytes = self.root.serialize();
        let root_x_bytes = self.root_x.serialize();
        let root_y_bytes = self.root_y.serialize();
        let deviceid_bytes = self.deviceid.serialize();
        let mut request0 = [
            major_opcode,
            FAKE_INPUT_REQUEST,
            0,
            0,
            type_bytes[0],
            detail_bytes[0],
            0,
            0,
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            root_bytes[0],
            root_bytes[1],
            root_bytes[2],
            root_bytes[3],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            root_x_bytes[0],
            root_x_bytes[1],
            root_y_bytes[0],
            root_y_bytes[1],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            deviceid_bytes[0],
        ];
        request0[2..4].copy_from_slice(&(9u16).to_ne_bytes());
        request0
    }
}

/// Opcode for the GrabControl request
pub const GRAB_CONTROL_REQUEST: u8 = 3;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GrabControlRequest {
    pub impervious: bool,
}
impl GrabControlRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let impervious_bytes = self.impervious.serialize();
        let mut request0 = [
            major_opcode,
            GRAB_CONTROL_REQUEST,
            0,
            0,
            impervious_bytes[0],
            0,
            0,
            0,
        ];
        request0[2..4].copy_from_slice(&(2u16).to_ne_bytes());
        request0
    }
}
