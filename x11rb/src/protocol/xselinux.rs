// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `SELinux` X11 extension.

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
pub const X11_EXTENSION_NAME: &str = "SELinux";

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
    pub client_major: u8,
    pub client_minor: u8,
}
impl QueryVersionRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let client_major_bytes = self.client_major.serialize();
        let client_minor_bytes = self.client_minor.serialize();
        let mut request0 = [
            major_opcode,
            QUERY_VERSION_REQUEST,
            0,
            0,
            client_major_bytes[0],
            client_minor_bytes[0],
            0,
            0,
        ];
        request0[2..4].copy_from_slice(&(2u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryVersionReply {
    pub sequence: u16,
    pub length: u32,
    pub server_major: u16,
    pub server_minor: u16,
}
impl TryParse for QueryVersionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (server_major, remaining) = u16::try_parse(remaining)?;
        let (server_minor, remaining) = u16::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryVersionReply {
            sequence,
            length,
            server_major,
            server_minor,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the SetDeviceCreateContext request
pub const SET_DEVICE_CREATE_CONTEXT_REQUEST: u8 = 1;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SetDeviceCreateContextRequest<'input> {
    pub context: Cow<'input, [u8]>,
}
impl<'input> SetDeviceCreateContextRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let length_so_far = 0;
        let context_len =
            u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_DEVICE_CREATE_CONTEXT_REQUEST,
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let referenced: &[u8] = self.context.as_ref();
        let length_so_far = length_so_far + referenced.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        debug_assert_eq!(0, length_so_far % 4);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        request0.extend_from_slice(self.context.as_ref());
        request0.extend_from_slice(padding0.as_ref());
        request0
    }
}

/// Opcode for the GetDeviceCreateContext request
pub const GET_DEVICE_CREATE_CONTEXT_REQUEST: u8 = 2;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetDeviceCreateContextRequest;
impl GetDeviceCreateContextRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let mut request0 = [major_opcode, GET_DEVICE_CREATE_CONTEXT_REQUEST, 0, 0];
        request0[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetDeviceCreateContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetDeviceCreateContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) =
            crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetDeviceCreateContextReply {
            sequence,
            length,
            context,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl GetDeviceCreateContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    #[must_use]
    pub fn context_len(&self) -> u32 {
        self.context.len().try_into().unwrap()
    }
}

/// Opcode for the SetDeviceContext request
pub const SET_DEVICE_CONTEXT_REQUEST: u8 = 3;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SetDeviceContextRequest<'input> {
    pub device: u32,
    pub context: Cow<'input, [u8]>,
}
impl<'input> SetDeviceContextRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let length_so_far = 0;
        let device_bytes = self.device.serialize();
        let context_len =
            u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_DEVICE_CONTEXT_REQUEST,
            0,
            0,
            device_bytes[0],
            device_bytes[1],
            device_bytes[2],
            device_bytes[3],
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let referenced: &[u8] = self.context.as_ref();
        let length_so_far = length_so_far + referenced.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        debug_assert_eq!(0, length_so_far % 4);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        request0.extend_from_slice(self.context.as_ref());
        request0.extend_from_slice(padding0.as_ref());
        request0
    }
}

/// Opcode for the GetDeviceContext request
pub const GET_DEVICE_CONTEXT_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetDeviceContextRequest {
    pub device: u32,
}
impl GetDeviceContextRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let device_bytes = self.device.serialize();
        let mut request0 = [
            major_opcode,
            GET_DEVICE_CONTEXT_REQUEST,
            0,
            0,
            device_bytes[0],
            device_bytes[1],
            device_bytes[2],
            device_bytes[3],
        ];
        request0[2..4].copy_from_slice(&(2u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetDeviceContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetDeviceContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) =
            crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetDeviceContextReply {
            sequence,
            length,
            context,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl GetDeviceContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    #[must_use]
    pub fn context_len(&self) -> u32 {
        self.context.len().try_into().unwrap()
    }
}

/// Opcode for the SetWindowCreateContext request
pub const SET_WINDOW_CREATE_CONTEXT_REQUEST: u8 = 5;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SetWindowCreateContextRequest<'input> {
    pub context: Cow<'input, [u8]>,
}
impl<'input> SetWindowCreateContextRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let length_so_far = 0;
        let context_len =
            u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_WINDOW_CREATE_CONTEXT_REQUEST,
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let referenced: &[u8] = self.context.as_ref();
        let length_so_far = length_so_far + referenced.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        debug_assert_eq!(0, length_so_far % 4);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        request0.extend_from_slice(self.context.as_ref());
        request0.extend_from_slice(padding0.as_ref());
        request0
    }
}

/// Opcode for the GetWindowCreateContext request
pub const GET_WINDOW_CREATE_CONTEXT_REQUEST: u8 = 6;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetWindowCreateContextRequest;
impl GetWindowCreateContextRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let mut request0 = [major_opcode, GET_WINDOW_CREATE_CONTEXT_REQUEST, 0, 0];
        request0[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetWindowCreateContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetWindowCreateContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) =
            crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetWindowCreateContextReply {
            sequence,
            length,
            context,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl GetWindowCreateContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    #[must_use]
    pub fn context_len(&self) -> u32 {
        self.context.len().try_into().unwrap()
    }
}

/// Opcode for the GetWindowContext request
pub const GET_WINDOW_CONTEXT_REQUEST: u8 = 7;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetWindowContextRequest {
    pub window: xproto::Window,
}
impl GetWindowContextRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let window_bytes = self.window.serialize();
        let mut request0 = [
            major_opcode,
            GET_WINDOW_CONTEXT_REQUEST,
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

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetWindowContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetWindowContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) =
            crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetWindowContextReply {
            sequence,
            length,
            context,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl GetWindowContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    #[must_use]
    pub fn context_len(&self) -> u32 {
        self.context.len().try_into().unwrap()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ListItem {
    pub name: xproto::Atom,
    pub object_context: Vec<u8>,
    pub data_context: Vec<u8>,
}
impl TryParse for ListItem {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (name, remaining) = xproto::Atom::try_parse(remaining)?;
        let (object_context_len, remaining) = u32::try_parse(remaining)?;
        let (data_context_len, remaining) = u32::try_parse(remaining)?;
        let (object_context, remaining) =
            crate::x11_utils::parse_u8_list(remaining, object_context_len.try_to_usize()?)?;
        let object_context = object_context.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining
            .get(misalignment..)
            .ok_or(ParseError::InsufficientData)?;
        let (data_context, remaining) =
            crate::x11_utils::parse_u8_list(remaining, data_context_len.try_to_usize()?)?;
        let data_context = data_context.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining
            .get(misalignment..)
            .ok_or(ParseError::InsufficientData)?;
        let result = ListItem {
            name,
            object_context,
            data_context,
        };
        Ok((result, remaining))
    }
}
impl Serialize for ListItem {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        self.name.serialize_into(bytes);
        let object_context_len = u32::try_from(self.object_context.len())
            .expect("`object_context` has too many elements");
        object_context_len.serialize_into(bytes);
        let data_context_len =
            u32::try_from(self.data_context.len()).expect("`data_context` has too many elements");
        data_context_len.serialize_into(bytes);
        bytes.extend_from_slice(&self.object_context);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
        bytes.extend_from_slice(&self.data_context);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
    }
}
impl ListItem {
    /// Get the value of the `object_context_len` field.
    ///
    /// The `object_context_len` field is used as the length field of the `object_context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    #[must_use]
    pub fn object_context_len(&self) -> u32 {
        self.object_context.len().try_into().unwrap()
    }
    /// Get the value of the `data_context_len` field.
    ///
    /// The `data_context_len` field is used as the length field of the `data_context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    #[must_use]
    pub fn data_context_len(&self) -> u32 {
        self.data_context.len().try_into().unwrap()
    }
}

/// Opcode for the SetPropertyCreateContext request
pub const SET_PROPERTY_CREATE_CONTEXT_REQUEST: u8 = 8;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SetPropertyCreateContextRequest<'input> {
    pub context: Cow<'input, [u8]>,
}
impl<'input> SetPropertyCreateContextRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let length_so_far = 0;
        let context_len =
            u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_PROPERTY_CREATE_CONTEXT_REQUEST,
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let referenced: &[u8] = self.context.as_ref();
        let length_so_far = length_so_far + referenced.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        debug_assert_eq!(0, length_so_far % 4);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        request0.extend_from_slice(self.context.as_ref());
        request0.extend_from_slice(padding0.as_ref());
        request0
    }
}

/// Opcode for the GetPropertyCreateContext request
pub const GET_PROPERTY_CREATE_CONTEXT_REQUEST: u8 = 9;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetPropertyCreateContextRequest;
impl GetPropertyCreateContextRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let mut request0 = [major_opcode, GET_PROPERTY_CREATE_CONTEXT_REQUEST, 0, 0];
        request0[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetPropertyCreateContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetPropertyCreateContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) =
            crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetPropertyCreateContextReply {
            sequence,
            length,
            context,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl GetPropertyCreateContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    #[must_use]
    pub fn context_len(&self) -> u32 {
        self.context.len().try_into().unwrap()
    }
}

/// Opcode for the SetPropertyUseContext request
pub const SET_PROPERTY_USE_CONTEXT_REQUEST: u8 = 10;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SetPropertyUseContextRequest<'input> {
    pub context: Cow<'input, [u8]>,
}
impl<'input> SetPropertyUseContextRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let length_so_far = 0;
        let context_len =
            u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_PROPERTY_USE_CONTEXT_REQUEST,
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let referenced: &[u8] = self.context.as_ref();
        let length_so_far = length_so_far + referenced.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        debug_assert_eq!(0, length_so_far % 4);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        request0.extend_from_slice(self.context.as_ref());
        request0.extend_from_slice(padding0.as_ref());
        request0
    }
}

/// Opcode for the GetPropertyUseContext request
pub const GET_PROPERTY_USE_CONTEXT_REQUEST: u8 = 11;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetPropertyUseContextRequest;
impl GetPropertyUseContextRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let mut request0 = [major_opcode, GET_PROPERTY_USE_CONTEXT_REQUEST, 0, 0];
        request0[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetPropertyUseContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetPropertyUseContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) =
            crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetPropertyUseContextReply {
            sequence,
            length,
            context,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl GetPropertyUseContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    #[must_use]
    pub fn context_len(&self) -> u32 {
        self.context.len().try_into().unwrap()
    }
}

/// Opcode for the GetPropertyContext request
pub const GET_PROPERTY_CONTEXT_REQUEST: u8 = 12;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetPropertyContextRequest {
    pub window: xproto::Window,
    pub property: xproto::Atom,
}
impl GetPropertyContextRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let window_bytes = self.window.serialize();
        let property_bytes = self.property.serialize();
        let mut request0 = [
            major_opcode,
            GET_PROPERTY_CONTEXT_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            property_bytes[0],
            property_bytes[1],
            property_bytes[2],
            property_bytes[3],
        ];
        request0[2..4].copy_from_slice(&(3u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetPropertyContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetPropertyContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) =
            crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetPropertyContextReply {
            sequence,
            length,
            context,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl GetPropertyContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    #[must_use]
    pub fn context_len(&self) -> u32 {
        self.context.len().try_into().unwrap()
    }
}

/// Opcode for the GetPropertyDataContext request
pub const GET_PROPERTY_DATA_CONTEXT_REQUEST: u8 = 13;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetPropertyDataContextRequest {
    pub window: xproto::Window,
    pub property: xproto::Atom,
}
impl GetPropertyDataContextRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let window_bytes = self.window.serialize();
        let property_bytes = self.property.serialize();
        let mut request0 = [
            major_opcode,
            GET_PROPERTY_DATA_CONTEXT_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            property_bytes[0],
            property_bytes[1],
            property_bytes[2],
            property_bytes[3],
        ];
        request0[2..4].copy_from_slice(&(3u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetPropertyDataContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetPropertyDataContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) =
            crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetPropertyDataContextReply {
            sequence,
            length,
            context,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl GetPropertyDataContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    #[must_use]
    pub fn context_len(&self) -> u32 {
        self.context.len().try_into().unwrap()
    }
}

/// Opcode for the ListProperties request
pub const LIST_PROPERTIES_REQUEST: u8 = 14;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ListPropertiesRequest {
    pub window: xproto::Window,
}
impl ListPropertiesRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let window_bytes = self.window.serialize();
        let mut request0 = [
            major_opcode,
            LIST_PROPERTIES_REQUEST,
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

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ListPropertiesReply {
    pub sequence: u16,
    pub length: u32,
    pub properties: Vec<ListItem>,
}
impl TryParse for ListPropertiesReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (properties_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (properties, remaining) =
            crate::x11_utils::parse_list::<ListItem>(remaining, properties_len.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = ListPropertiesReply {
            sequence,
            length,
            properties,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl ListPropertiesReply {
    /// Get the value of the `properties_len` field.
    ///
    /// The `properties_len` field is used as the length field of the `properties` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    #[must_use]
    pub fn properties_len(&self) -> u32 {
        self.properties.len().try_into().unwrap()
    }
}

/// Opcode for the SetSelectionCreateContext request
pub const SET_SELECTION_CREATE_CONTEXT_REQUEST: u8 = 15;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SetSelectionCreateContextRequest<'input> {
    pub context: Cow<'input, [u8]>,
}
impl<'input> SetSelectionCreateContextRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let length_so_far = 0;
        let context_len =
            u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_SELECTION_CREATE_CONTEXT_REQUEST,
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let referenced: &[u8] = self.context.as_ref();
        let length_so_far = length_so_far + referenced.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        debug_assert_eq!(0, length_so_far % 4);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        request0.extend_from_slice(self.context.as_ref());
        request0.extend_from_slice(padding0.as_ref());
        request0
    }
}

/// Opcode for the GetSelectionCreateContext request
pub const GET_SELECTION_CREATE_CONTEXT_REQUEST: u8 = 16;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetSelectionCreateContextRequest;
impl GetSelectionCreateContextRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let mut request0 = [major_opcode, GET_SELECTION_CREATE_CONTEXT_REQUEST, 0, 0];
        request0[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetSelectionCreateContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetSelectionCreateContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) =
            crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetSelectionCreateContextReply {
            sequence,
            length,
            context,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl GetSelectionCreateContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    #[must_use]
    pub fn context_len(&self) -> u32 {
        self.context.len().try_into().unwrap()
    }
}

/// Opcode for the SetSelectionUseContext request
pub const SET_SELECTION_USE_CONTEXT_REQUEST: u8 = 17;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SetSelectionUseContextRequest<'input> {
    pub context: Cow<'input, [u8]>,
}
impl<'input> SetSelectionUseContextRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let length_so_far = 0;
        let context_len =
            u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_SELECTION_USE_CONTEXT_REQUEST,
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let referenced: &[u8] = self.context.as_ref();
        let length_so_far = length_so_far + referenced.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        debug_assert_eq!(0, length_so_far % 4);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        request0.extend_from_slice(self.context.as_ref());
        request0.extend_from_slice(padding0.as_ref());
        request0
    }
}

/// Opcode for the GetSelectionUseContext request
pub const GET_SELECTION_USE_CONTEXT_REQUEST: u8 = 18;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetSelectionUseContextRequest;
impl GetSelectionUseContextRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let mut request0 = [major_opcode, GET_SELECTION_USE_CONTEXT_REQUEST, 0, 0];
        request0[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetSelectionUseContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetSelectionUseContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) =
            crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetSelectionUseContextReply {
            sequence,
            length,
            context,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl GetSelectionUseContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    #[must_use]
    pub fn context_len(&self) -> u32 {
        self.context.len().try_into().unwrap()
    }
}

/// Opcode for the GetSelectionContext request
pub const GET_SELECTION_CONTEXT_REQUEST: u8 = 19;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetSelectionContextRequest {
    pub selection: xproto::Atom,
}
impl GetSelectionContextRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let selection_bytes = self.selection.serialize();
        let mut request0 = [
            major_opcode,
            GET_SELECTION_CONTEXT_REQUEST,
            0,
            0,
            selection_bytes[0],
            selection_bytes[1],
            selection_bytes[2],
            selection_bytes[3],
        ];
        request0[2..4].copy_from_slice(&(2u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetSelectionContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetSelectionContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) =
            crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetSelectionContextReply {
            sequence,
            length,
            context,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl GetSelectionContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    #[must_use]
    pub fn context_len(&self) -> u32 {
        self.context.len().try_into().unwrap()
    }
}

/// Opcode for the GetSelectionDataContext request
pub const GET_SELECTION_DATA_CONTEXT_REQUEST: u8 = 20;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetSelectionDataContextRequest {
    pub selection: xproto::Atom,
}
impl GetSelectionDataContextRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let selection_bytes = self.selection.serialize();
        let mut request0 = [
            major_opcode,
            GET_SELECTION_DATA_CONTEXT_REQUEST,
            0,
            0,
            selection_bytes[0],
            selection_bytes[1],
            selection_bytes[2],
            selection_bytes[3],
        ];
        request0[2..4].copy_from_slice(&(2u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetSelectionDataContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetSelectionDataContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) =
            crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetSelectionDataContextReply {
            sequence,
            length,
            context,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl GetSelectionDataContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    #[must_use]
    pub fn context_len(&self) -> u32 {
        self.context.len().try_into().unwrap()
    }
}

/// Opcode for the ListSelections request
pub const LIST_SELECTIONS_REQUEST: u8 = 21;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ListSelectionsRequest;
impl ListSelectionsRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let mut request0 = [major_opcode, LIST_SELECTIONS_REQUEST, 0, 0];
        request0[2..4].copy_from_slice(&(1u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ListSelectionsReply {
    pub sequence: u16,
    pub length: u32,
    pub selections: Vec<ListItem>,
}
impl TryParse for ListSelectionsReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (selections_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (selections, remaining) =
            crate::x11_utils::parse_list::<ListItem>(remaining, selections_len.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = ListSelectionsReply {
            sequence,
            length,
            selections,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl ListSelectionsReply {
    /// Get the value of the `selections_len` field.
    ///
    /// The `selections_len` field is used as the length field of the `selections` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    #[must_use]
    pub fn selections_len(&self) -> u32 {
        self.selections.len().try_into().unwrap()
    }
}

/// Opcode for the GetClientContext request
pub const GET_CLIENT_CONTEXT_REQUEST: u8 = 22;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetClientContextRequest {
    pub resource: u32,
}
impl GetClientContextRequest {
    /// Serialize this request into bytes for the provided connection
    #[must_use]
    pub fn serialize(self, major_opcode: u8) -> impl AsRef<[u8]> {
        let resource_bytes = self.resource.serialize();
        let mut request0 = [
            major_opcode,
            GET_CLIENT_CONTEXT_REQUEST,
            0,
            0,
            resource_bytes[0],
            resource_bytes[1],
            resource_bytes[2],
            resource_bytes[3],
        ];
        request0[2..4].copy_from_slice(&(2u16).to_ne_bytes());
        request0
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetClientContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetClientContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) =
            crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetClientContextReply {
            sequence,
            length,
            context,
        };
        let _ = remaining;
        let remaining = initial_value
            .get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl GetClientContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    #[must_use]
    pub fn context_len(&self) -> u32 {
        self.context.len().try_into().unwrap()
    }
}
