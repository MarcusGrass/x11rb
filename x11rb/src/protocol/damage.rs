// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Damage` X11 extension.

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
pub const X11_EXTENSION_NAME: &str = "DAMAGE";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 1);

pub type Damage = u32;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReportLevel(u8);
impl ReportLevel {
    pub const RAW_RECTANGLES: Self = Self(0);
    pub const DELTA_RECTANGLES: Self = Self(1);
    pub const BOUNDING_BOX: Self = Self(2);
    pub const NON_EMPTY: Self = Self(3);
}
impl From<ReportLevel> for u8 {
    #[inline]
    fn from(input: ReportLevel) -> Self {
        input.0
    }
}
impl From<ReportLevel> for Option<u8> {
    #[inline]
    fn from(input: ReportLevel) -> Self {
        Some(input.0)
    }
}
impl From<ReportLevel> for u16 {
    #[inline]
    fn from(input: ReportLevel) -> Self {
        u16::from(input.0)
    }
}
impl From<ReportLevel> for Option<u16> {
    #[inline]
    fn from(input: ReportLevel) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<ReportLevel> for u32 {
    #[inline]
    fn from(input: ReportLevel) -> Self {
        u32::from(input.0)
    }
}
impl From<ReportLevel> for Option<u32> {
    #[inline]
    fn from(input: ReportLevel) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for ReportLevel {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for ReportLevel {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (
                Self::RAW_RECTANGLES.0.into(),
                "RAW_RECTANGLES",
                "RawRectangles",
            ),
            (
                Self::DELTA_RECTANGLES.0.into(),
                "DELTA_RECTANGLES",
                "DeltaRectangles",
            ),
            (Self::BOUNDING_BOX.0.into(), "BOUNDING_BOX", "BoundingBox"),
            (Self::NON_EMPTY.0.into(), "NON_EMPTY", "NonEmpty"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

/// Opcode for the BadDamage error
pub const BAD_DAMAGE_ERROR: u8 = 0;

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

/// Opcode for the Create request
pub const CREATE_REQUEST: u8 = 1;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateRequest {
    pub damage: Damage,
    pub drawable: xproto::Drawable,
    pub level: ReportLevel,
}
impl CreateRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let damage_bytes = self.damage.serialize();
        let drawable_bytes = self.drawable.serialize();
        let level_bytes = u8::from(self.level).serialize();
        let mut request0 = [
            major_opcode,
            CREATE_REQUEST,
            0,
            0,
            damage_bytes[0],
            damage_bytes[1],
            damage_bytes[2],
            damage_bytes[3],
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            level_bytes[0],
            0,
            0,
            0,
        ];
        request0[2..4].copy_from_slice(&(4u16).to_ne_bytes());
        request0
    }
}

/// Opcode for the Destroy request
pub const DESTROY_REQUEST: u8 = 2;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DestroyRequest {
    pub damage: Damage,
}
impl DestroyRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let damage_bytes = self.damage.serialize();
        let mut request0 = [
            major_opcode,
            DESTROY_REQUEST,
            0,
            0,
            damage_bytes[0],
            damage_bytes[1],
            damage_bytes[2],
            damage_bytes[3],
        ];
        request0[2..4].copy_from_slice(&(2u16).to_ne_bytes());
        request0
    }
}

/// Opcode for the Subtract request
pub const SUBTRACT_REQUEST: u8 = 3;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubtractRequest {
    pub damage: Damage,
    pub repair: xfixes::Region,
    pub parts: xfixes::Region,
}
impl SubtractRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let damage_bytes = self.damage.serialize();
        let repair_bytes = self.repair.serialize();
        let parts_bytes = self.parts.serialize();
        let mut request0 = [
            major_opcode,
            SUBTRACT_REQUEST,
            0,
            0,
            damage_bytes[0],
            damage_bytes[1],
            damage_bytes[2],
            damage_bytes[3],
            repair_bytes[0],
            repair_bytes[1],
            repair_bytes[2],
            repair_bytes[3],
            parts_bytes[0],
            parts_bytes[1],
            parts_bytes[2],
            parts_bytes[3],
        ];
        request0[2..4].copy_from_slice(&(4u16).to_ne_bytes());
        request0
    }
}

/// Opcode for the Add request
pub const ADD_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddRequest {
    pub drawable: xproto::Drawable,
    pub region: xfixes::Region,
}
impl AddRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let drawable_bytes = self.drawable.serialize();
        let region_bytes = self.region.serialize();
        let mut request0 = [
            major_opcode,
            ADD_REQUEST,
            0,
            0,
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            region_bytes[0],
            region_bytes[1],
            region_bytes[2],
            region_bytes[3],
        ];
        request0[2..4].copy_from_slice(&(3u16).to_ne_bytes());
        request0
    }
}

/// Opcode for the Notify event
pub const NOTIFY_EVENT: u8 = 0;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NotifyEvent {
    pub response_type: u8,
    pub level: ReportLevel,
    pub sequence: u16,
    pub drawable: xproto::Drawable,
    pub damage: Damage,
    pub timestamp: xproto::Timestamp,
    pub area: xproto::Rectangle,
    pub geometry: xproto::Rectangle,
}
impl TryParse for NotifyEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (level, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (drawable, remaining) = xproto::Drawable::try_parse(remaining)?;
        let (damage, remaining) = Damage::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (area, remaining) = xproto::Rectangle::try_parse(remaining)?;
        let (geometry, remaining) = xproto::Rectangle::try_parse(remaining)?;
        let level = level.into();
        let result = NotifyEvent {
            response_type,
            level,
            sequence,
            drawable,
            damage,
            timestamp,
            area,
            geometry,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl From<&NotifyEvent> for [u8; 32] {
    fn from(input: &NotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let level_bytes = u8::from(input.level).serialize();
        let sequence_bytes = input.sequence.serialize();
        let drawable_bytes = input.drawable.serialize();
        let damage_bytes = input.damage.serialize();
        let timestamp_bytes = input.timestamp.serialize();
        let area_bytes = input.area.serialize();
        let geometry_bytes = input.geometry.serialize();
        [
            response_type_bytes[0],
            level_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            damage_bytes[0],
            damage_bytes[1],
            damage_bytes[2],
            damage_bytes[3],
            timestamp_bytes[0],
            timestamp_bytes[1],
            timestamp_bytes[2],
            timestamp_bytes[3],
            area_bytes[0],
            area_bytes[1],
            area_bytes[2],
            area_bytes[3],
            area_bytes[4],
            area_bytes[5],
            area_bytes[6],
            area_bytes[7],
            geometry_bytes[0],
            geometry_bytes[1],
            geometry_bytes[2],
            geometry_bytes[3],
            geometry_bytes[4],
            geometry_bytes[5],
            geometry_bytes[6],
            geometry_bytes[7],
        ]
    }
}
impl From<NotifyEvent> for [u8; 32] {
    fn from(input: NotifyEvent) -> Self {
        Self::from(&input)
    }
}
