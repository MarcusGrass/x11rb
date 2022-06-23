// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Sync` X11 extension.

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

pub use crate::protocol::sync::*;

/// Get the major opcode of this extension
fn major_opcode(conn: &SocketConnection) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME);
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub fn initialize(
    conn: &mut SocketConnection,
    desired_major_version: u8,
    desired_minor_version: u8,
    forget: bool,
) -> Result<Cookie<InitializeReply>, ConnectionError> {
    let request0 = InitializeRequest {
        desired_major_version,
        desired_minor_version,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn list_system_counters(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<ListSystemCountersReply>, ConnectionError> {
    let request0 = ListSystemCountersRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_counter(
    conn: &mut SocketConnection,
    id: Counter,
    initial_value: Int64,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateCounterRequest { id, initial_value };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn destroy_counter(
    conn: &mut SocketConnection,
    counter: Counter,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DestroyCounterRequest { counter };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_counter(
    conn: &mut SocketConnection,
    counter: Counter,
    forget: bool,
) -> Result<Cookie<QueryCounterReply>, ConnectionError> {
    let request0 = QueryCounterRequest { counter };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn await_<'c, 'input>(
    conn: &'c mut SocketConnection,
    wait_list: &'input [Waitcondition],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = AwaitRequest {
        wait_list: Cow::Borrowed(wait_list),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn change_counter(
    conn: &mut SocketConnection,
    counter: Counter,
    amount: Int64,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = ChangeCounterRequest { counter, amount };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_counter(
    conn: &mut SocketConnection,
    counter: Counter,
    value: Int64,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetCounterRequest { counter, value };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_alarm<'c, 'input>(
    conn: &'c mut SocketConnection,
    id: Alarm,
    value_list: &'input CreateAlarmAux,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateAlarmRequest {
        id,
        value_list: Cow::Borrowed(value_list),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn change_alarm<'c, 'input>(
    conn: &'c mut SocketConnection,
    id: Alarm,
    value_list: &'input ChangeAlarmAux,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = ChangeAlarmRequest {
        id,
        value_list: Cow::Borrowed(value_list),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn destroy_alarm(
    conn: &mut SocketConnection,
    alarm: Alarm,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DestroyAlarmRequest { alarm };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_alarm(
    conn: &mut SocketConnection,
    alarm: Alarm,
    forget: bool,
) -> Result<Cookie<QueryAlarmReply>, ConnectionError> {
    let request0 = QueryAlarmRequest { alarm };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_priority(
    conn: &mut SocketConnection,
    id: u32,
    priority: i32,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SetPriorityRequest { id, priority };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_priority(
    conn: &mut SocketConnection,
    id: u32,
    forget: bool,
) -> Result<Cookie<GetPriorityReply>, ConnectionError> {
    let request0 = GetPriorityRequest { id };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn create_fence(
    conn: &mut SocketConnection,
    drawable: xproto::Drawable,
    fence: Fence,
    initially_triggered: bool,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CreateFenceRequest {
        drawable,
        fence,
        initially_triggered,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn trigger_fence(
    conn: &mut SocketConnection,
    fence: Fence,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = TriggerFenceRequest { fence };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn reset_fence(
    conn: &mut SocketConnection,
    fence: Fence,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = ResetFenceRequest { fence };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn destroy_fence(
    conn: &mut SocketConnection,
    fence: Fence,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DestroyFenceRequest { fence };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_fence(
    conn: &mut SocketConnection,
    fence: Fence,
    forget: bool,
) -> Result<Cookie<QueryFenceReply>, ConnectionError> {
    let request0 = QueryFenceRequest { fence };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn await_fence<'c, 'input>(
    conn: &'c mut SocketConnection,
    fence_list: &'input [Fence],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = AwaitFenceRequest {
        fence_list: Cow::Borrowed(fence_list),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}
