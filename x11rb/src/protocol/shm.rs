// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Shm` X11 extension.

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
pub const X11_EXTENSION_NAME: &str = "MIT-SHM";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 2);

pub type Seg = u32;

/// Opcode for the Completion event
pub const COMPLETION_EVENT: u8 = 0;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompletionEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub drawable: xproto::Drawable,
    pub minor_event: u16,
    pub major_event: u8,
    pub shmseg: Seg,
    pub offset: u32,
}
impl TryParse for CompletionEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (drawable, remaining) = xproto::Drawable::try_parse(remaining)?;
        let (minor_event, remaining) = u16::try_parse(remaining)?;
        let (major_event, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (shmseg, remaining) = Seg::try_parse(remaining)?;
        let (offset, remaining) = u32::try_parse(remaining)?;
        let result = CompletionEvent {
            response_type,
            sequence,
            drawable,
            minor_event,
            major_event,
            shmseg,
            offset,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl From<&CompletionEvent> for [u8; 32] {
    fn from(input: &CompletionEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let sequence_bytes = input.sequence.serialize();
        let drawable_bytes = input.drawable.serialize();
        let minor_event_bytes = input.minor_event.serialize();
        let major_event_bytes = input.major_event.serialize();
        let shmseg_bytes = input.shmseg.serialize();
        let offset_bytes = input.offset.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            minor_event_bytes[0],
            minor_event_bytes[1],
            major_event_bytes[0],
            0,
            shmseg_bytes[0],
            shmseg_bytes[1],
            shmseg_bytes[2],
            shmseg_bytes[3],
            offset_bytes[0],
            offset_bytes[1],
            offset_bytes[2],
            offset_bytes[3],
            // trailing padding
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
        ]
    }
}
impl From<CompletionEvent> for [u8; 32] {
    fn from(input: CompletionEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadSeg error
pub const BAD_SEG_ERROR: u8 = 0;

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryVersionRequest;
impl QueryVersionRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let mut request0 = [major_opcode, QUERY_VERSION_REQUEST, 0, 0];
        request0[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryVersionReply {
    pub shared_pixmaps: bool,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub uid: u16,
    pub gid: u16,
    pub pixmap_format: u8,
}
impl TryParse for QueryVersionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (shared_pixmaps, remaining) = bool::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major_version, remaining) = u16::try_parse(remaining)?;
        let (minor_version, remaining) = u16::try_parse(remaining)?;
        let (uid, remaining) = u16::try_parse(remaining)?;
        let (gid, remaining) = u16::try_parse(remaining)?;
        let (pixmap_format, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(15..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryVersionReply {
            shared_pixmaps,
            sequence,
            length,
            major_version,
            minor_version,
            uid,
            gid,
            pixmap_format,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the Attach request
pub const ATTACH_REQUEST: u8 = 1;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttachRequest {
    pub shmseg: Seg,
    pub shmid: u32,
    pub read_only: bool,
}
impl AttachRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let shmseg_bytes = self.shmseg.serialize();
        let shmid_bytes = self.shmid.serialize();
        let read_only_bytes = self.read_only.serialize();
        let mut request0 = [
            major_opcode,
            ATTACH_REQUEST,
            0,
            0,
            shmseg_bytes[0],
            shmseg_bytes[1],
            shmseg_bytes[2],
            shmseg_bytes[3],
            shmid_bytes[0],
            shmid_bytes[1],
            shmid_bytes[2],
            shmid_bytes[3],
            read_only_bytes[0],
            0,
            0,
            0,
        ];
        request0[2..4].copy_from_slice(&(4u16).to_ne_bytes());
        request0
    }
}

/// Opcode for the Detach request
pub const DETACH_REQUEST: u8 = 2;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DetachRequest {
    pub shmseg: Seg,
}
impl DetachRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let shmseg_bytes = self.shmseg.serialize();
        let mut request0 = [
            major_opcode,
            DETACH_REQUEST,
            0,
            0,
            shmseg_bytes[0],
            shmseg_bytes[1],
            shmseg_bytes[2],
            shmseg_bytes[3],
        ];
        request0[2..4].copy_from_slice(&(2u16).to_ne_bytes());
        request0
    }
}

/// Opcode for the PutImage request
pub const PUT_IMAGE_REQUEST: u8 = 3;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PutImageRequest {
    pub drawable: xproto::Drawable,
    pub gc: xproto::Gcontext,
    pub total_width: u16,
    pub total_height: u16,
    pub src_x: u16,
    pub src_y: u16,
    pub src_width: u16,
    pub src_height: u16,
    pub dst_x: i16,
    pub dst_y: i16,
    pub depth: u8,
    pub format: u8,
    pub send_event: bool,
    pub shmseg: Seg,
    pub offset: u32,
}
impl PutImageRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let drawable_bytes = self.drawable.serialize();
        let gc_bytes = self.gc.serialize();
        let total_width_bytes = self.total_width.serialize();
        let total_height_bytes = self.total_height.serialize();
        let src_x_bytes = self.src_x.serialize();
        let src_y_bytes = self.src_y.serialize();
        let src_width_bytes = self.src_width.serialize();
        let src_height_bytes = self.src_height.serialize();
        let dst_x_bytes = self.dst_x.serialize();
        let dst_y_bytes = self.dst_y.serialize();
        let depth_bytes = self.depth.serialize();
        let format_bytes = self.format.serialize();
        let send_event_bytes = self.send_event.serialize();
        let shmseg_bytes = self.shmseg.serialize();
        let offset_bytes = self.offset.serialize();
        let mut request0 = [
            major_opcode,
            PUT_IMAGE_REQUEST,
            0,
            0,
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            gc_bytes[0],
            gc_bytes[1],
            gc_bytes[2],
            gc_bytes[3],
            total_width_bytes[0],
            total_width_bytes[1],
            total_height_bytes[0],
            total_height_bytes[1],
            src_x_bytes[0],
            src_x_bytes[1],
            src_y_bytes[0],
            src_y_bytes[1],
            src_width_bytes[0],
            src_width_bytes[1],
            src_height_bytes[0],
            src_height_bytes[1],
            dst_x_bytes[0],
            dst_x_bytes[1],
            dst_y_bytes[0],
            dst_y_bytes[1],
            depth_bytes[0],
            format_bytes[0],
            send_event_bytes[0],
            0,
            shmseg_bytes[0],
            shmseg_bytes[1],
            shmseg_bytes[2],
            shmseg_bytes[3],
            offset_bytes[0],
            offset_bytes[1],
            offset_bytes[2],
            offset_bytes[3],
        ];
        request0[2..4].copy_from_slice(&(10u16).to_ne_bytes());
        request0
    }
}

/// Opcode for the GetImage request
pub const GET_IMAGE_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetImageRequest {
    pub drawable: xproto::Drawable,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub plane_mask: u32,
    pub format: u8,
    pub shmseg: Seg,
    pub offset: u32,
}
impl GetImageRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let drawable_bytes = self.drawable.serialize();
        let x_bytes = self.x.serialize();
        let y_bytes = self.y.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let plane_mask_bytes = self.plane_mask.serialize();
        let format_bytes = self.format.serialize();
        let shmseg_bytes = self.shmseg.serialize();
        let offset_bytes = self.offset.serialize();
        let mut request0 = [
            major_opcode,
            GET_IMAGE_REQUEST,
            0,
            0,
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            x_bytes[0],
            x_bytes[1],
            y_bytes[0],
            y_bytes[1],
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
            plane_mask_bytes[0],
            plane_mask_bytes[1],
            plane_mask_bytes[2],
            plane_mask_bytes[3],
            format_bytes[0],
            0,
            0,
            0,
            shmseg_bytes[0],
            shmseg_bytes[1],
            shmseg_bytes[2],
            shmseg_bytes[3],
            offset_bytes[0],
            offset_bytes[1],
            offset_bytes[2],
            offset_bytes[3],
        ];
        request0[2..4].copy_from_slice(&(8u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetImageReply {
    pub depth: u8,
    pub sequence: u16,
    pub length: u32,
    pub visual: xproto::Visualid,
    pub size: u32,
}
impl TryParse for GetImageReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (depth, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (visual, remaining) = xproto::Visualid::try_parse(remaining)?;
        let (size, remaining) = u32::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetImageReply {
            depth,
            sequence,
            length,
            visual,
            size,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the CreatePixmap request
pub const CREATE_PIXMAP_REQUEST: u8 = 5;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreatePixmapRequest {
    pub pid: xproto::Pixmap,
    pub drawable: xproto::Drawable,
    pub width: u16,
    pub height: u16,
    pub depth: u8,
    pub shmseg: Seg,
    pub offset: u32,
}
impl CreatePixmapRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let pid_bytes = self.pid.serialize();
        let drawable_bytes = self.drawable.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let depth_bytes = self.depth.serialize();
        let shmseg_bytes = self.shmseg.serialize();
        let offset_bytes = self.offset.serialize();
        let mut request0 = [
            major_opcode,
            CREATE_PIXMAP_REQUEST,
            0,
            0,
            pid_bytes[0],
            pid_bytes[1],
            pid_bytes[2],
            pid_bytes[3],
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
            depth_bytes[0],
            0,
            0,
            0,
            shmseg_bytes[0],
            shmseg_bytes[1],
            shmseg_bytes[2],
            shmseg_bytes[3],
            offset_bytes[0],
            offset_bytes[1],
            offset_bytes[2],
            offset_bytes[3],
        ];
        request0[2..4].copy_from_slice(&(7u16).to_ne_bytes());
        request0
    }
}

/// Opcode for the AttachFd request
pub const ATTACH_FD_REQUEST: u8 = 6;
/// Opcode for the CreateSegment request
pub const CREATE_SEGMENT_REQUEST: u8 = 7;
