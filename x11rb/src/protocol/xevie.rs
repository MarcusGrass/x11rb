// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Xevie` X11 extension.

#![allow(clippy::too_many_arguments)]

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
pub const X11_EXTENSION_NAME: &str = "XEVIE";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 0);

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryVersionRequest {
    pub client_major_version: u16,
    pub client_minor_version: u16,
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
            client_minor_version_bytes[0],
            client_minor_version_bytes[1],
        ];
        request0[2..4].copy_from_slice(&(2u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryVersionReply {
    pub sequence: u16,
    pub length: u32,
    pub server_major_version: u16,
    pub server_minor_version: u16,
}
impl TryParse for QueryVersionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (server_major_version, remaining) = u16::try_parse(remaining)?;
        let (server_minor_version, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryVersionReply {
            sequence,
            length,
            server_major_version,
            server_minor_version,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the Start request
pub const START_REQUEST: u8 = 1;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StartRequest {
    pub screen: u32,
}
impl StartRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let screen_bytes = self.screen.serialize();
        let mut request0 = [
            major_opcode,
            START_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
        ];
        request0[2..4].copy_from_slice(&(2u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StartReply {
    pub sequence: u16,
    pub length: u32,
}
impl TryParse for StartReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = StartReply { sequence, length };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the End request
pub const END_REQUEST: u8 = 2;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EndRequest {
    pub cmap: u32,
}
impl EndRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let cmap_bytes = self.cmap.serialize();
        let mut request0 = [
            major_opcode,
            END_REQUEST,
            0,
            0,
            cmap_bytes[0],
            cmap_bytes[1],
            cmap_bytes[2],
            cmap_bytes[3],
        ];
        request0[2..4].copy_from_slice(&(2u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EndReply {
    pub sequence: u16,
    pub length: u32,
}
impl TryParse for EndReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = EndReply { sequence, length };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Datatype(bool);
impl Datatype {
    pub const UNMODIFIED: Self = Self(false);
    pub const MODIFIED: Self = Self(true);
}
impl From<Datatype> for bool {
    #[inline]
    fn from(input: Datatype) -> Self {
        input.0
    }
}
impl From<Datatype> for Option<bool> {
    #[inline]
    fn from(input: Datatype) -> Self {
        Some(input.0)
    }
}
impl From<Datatype> for u8 {
    #[inline]
    fn from(input: Datatype) -> Self {
        u8::from(input.0)
    }
}
impl From<Datatype> for Option<u8> {
    #[inline]
    fn from(input: Datatype) -> Self {
        Some(u8::from(input.0))
    }
}
impl From<Datatype> for u16 {
    #[inline]
    fn from(input: Datatype) -> Self {
        u16::from(input.0)
    }
}
impl From<Datatype> for Option<u16> {
    #[inline]
    fn from(input: Datatype) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<Datatype> for u32 {
    #[inline]
    fn from(input: Datatype) -> Self {
        u32::from(input.0)
    }
}
impl From<Datatype> for Option<u32> {
    #[inline]
    fn from(input: Datatype) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<bool> for Datatype {
    #[inline]
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for Datatype {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::UNMODIFIED.0.into(), "UNMODIFIED", "Unmodified"),
            (Self::MODIFIED.0.into(), "MODIFIED", "Modified"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Event {}
impl TryParse for Event {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = remaining.get(32..).ok_or(ParseError::InsufficientData)?;
        let result = Event {};
        Ok((result, remaining))
    }
}
impl Serialize for Event {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        bytes.extend_from_slice(&[0; 32]);
    }
}

/// Opcode for the Send request
pub const SEND_REQUEST: u8 = 3;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendRequest {
    pub event: Event,
    pub data_type: u32,
}
impl SendRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let event_bytes = self.event.serialize();
        let data_type_bytes = self.data_type.serialize();
        let mut request0 = [
            major_opcode,
            SEND_REQUEST,
            0,
            0,
            event_bytes[0],
            event_bytes[1],
            event_bytes[2],
            event_bytes[3],
            event_bytes[4],
            event_bytes[5],
            event_bytes[6],
            event_bytes[7],
            event_bytes[8],
            event_bytes[9],
            event_bytes[10],
            event_bytes[11],
            event_bytes[12],
            event_bytes[13],
            event_bytes[14],
            event_bytes[15],
            event_bytes[16],
            event_bytes[17],
            event_bytes[18],
            event_bytes[19],
            event_bytes[20],
            event_bytes[21],
            event_bytes[22],
            event_bytes[23],
            event_bytes[24],
            event_bytes[25],
            event_bytes[26],
            event_bytes[27],
            event_bytes[28],
            event_bytes[29],
            event_bytes[30],
            event_bytes[31],
            data_type_bytes[0],
            data_type_bytes[1],
            data_type_bytes[2],
            data_type_bytes[3],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        request0[2..4].copy_from_slice(&(26u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendReply {
    pub sequence: u16,
    pub length: u32,
}
impl TryParse for SendReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = SendReply { sequence, length };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the SelectInput request
pub const SELECT_INPUT_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SelectInputRequest {
    pub event_mask: u32,
}
impl SelectInputRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let event_mask_bytes = self.event_mask.serialize();
        let mut request0 = [
            major_opcode,
            SELECT_INPUT_REQUEST,
            0,
            0,
            event_mask_bytes[0],
            event_mask_bytes[1],
            event_mask_bytes[2],
            event_mask_bytes[3],
        ];
        request0[2..4].copy_from_slice(&(2u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SelectInputReply {
    pub sequence: u16,
    pub length: u32,
}
impl TryParse for SelectInputReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = SelectInputReply { sequence, length };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
