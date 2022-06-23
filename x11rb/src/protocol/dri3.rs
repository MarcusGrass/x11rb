// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `DRI3` X11 extension.

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
pub const X11_EXTENSION_NAME: &str = "DRI3";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 2);

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryVersionRequest {
    pub major_version: u32,
    pub minor_version: u32,
}
impl QueryVersionRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let major_version_bytes = self.major_version.serialize();
        let minor_version_bytes = self.minor_version.serialize();
        let mut request0 = [
            major_opcode,
            QUERY_VERSION_REQUEST,
            0,
            0,
            major_version_bytes[0],
            major_version_bytes[1],
            major_version_bytes[2],
            major_version_bytes[3],
            minor_version_bytes[0],
            minor_version_bytes[1],
            minor_version_bytes[2],
            minor_version_bytes[3],
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

/// Opcode for the Open request
pub const OPEN_REQUEST: u8 = 1;
/// Opcode for the PixmapFromBuffer request
pub const PIXMAP_FROM_BUFFER_REQUEST: u8 = 2;
/// Opcode for the BufferFromPixmap request
pub const BUFFER_FROM_PIXMAP_REQUEST: u8 = 3;
/// Opcode for the FenceFromFD request
pub const FENCE_FROM_FD_REQUEST: u8 = 4;
/// Opcode for the FDFromFence request
pub const FD_FROM_FENCE_REQUEST: u8 = 5;
/// Opcode for the GetSupportedModifiers request
pub const GET_SUPPORTED_MODIFIERS_REQUEST: u8 = 6;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetSupportedModifiersRequest {
    pub window: u32,
    pub depth: u8,
    pub bpp: u8,
}
impl GetSupportedModifiersRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let window_bytes = self.window.serialize();
        let depth_bytes = self.depth.serialize();
        let bpp_bytes = self.bpp.serialize();
        let mut request0 = [
            major_opcode,
            GET_SUPPORTED_MODIFIERS_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            depth_bytes[0],
            bpp_bytes[0],
            0,
            0,
        ];
        request0[2..4].copy_from_slice(&(3u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetSupportedModifiersReply {
    pub sequence: u16,
    pub length: u32,
    pub window_modifiers: Vec<u64>,
    pub screen_modifiers: Vec<u64>,
}
impl TryParse for GetSupportedModifiersReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_window_modifiers, remaining) = u32::try_parse(remaining)?;
        let (num_screen_modifiers, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::InsufficientData)?;
        let (window_modifiers, remaining) =
            crate::x11_utils::parse_list::<u64>(remaining, num_window_modifiers.try_to_usize()?)?;
        let (screen_modifiers, remaining) =
            crate::x11_utils::parse_list::<u64>(remaining, num_screen_modifiers.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetSupportedModifiersReply {
            sequence,
            length,
            window_modifiers,
            screen_modifiers,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl GetSupportedModifiersReply {
    /// Get the value of the `num_window_modifiers` field.
    ///
    /// The `num_window_modifiers` field is used as the length field of the `window_modifiers` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    #[must_use]
    pub fn num_window_modifiers(&self) -> u32 {
        self.window_modifiers.len().try_into().unwrap()
    }
    /// Get the value of the `num_screen_modifiers` field.
    ///
    /// The `num_screen_modifiers` field is used as the length field of the `screen_modifiers` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    #[must_use]
    pub fn num_screen_modifiers(&self) -> u32 {
        self.screen_modifiers.len().try_into().unwrap()
    }
}

/// Opcode for the PixmapFromBuffers request
pub const PIXMAP_FROM_BUFFERS_REQUEST: u8 = 7;
/// Opcode for the BuffersFromPixmap request
pub const BUFFERS_FROM_PIXMAP_REQUEST: u8 = 8;
