// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Input` X11 extension.

#![allow(clippy::too_many_arguments)]

#[allow(unused_imports)]
use super::xfixes;
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

pub use crate::protocol::xinput::*;

/// Get the major opcode of this extension
fn major_opcode(conn: &SocketConnection) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME);
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub fn get_extension_version<'c, 'input>(
    conn: &'c mut SocketConnection,
    name: &'input [u8],
    forget: bool,
) -> Result<Cookie<GetExtensionVersionReply>, ConnectionError> {
    let request0 = GetExtensionVersionRequest {
        name: Cow::Borrowed(name),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn list_input_devices(
    conn: &mut SocketConnection,
    forget: bool,
) -> Result<Cookie<ListInputDevicesReply>, ConnectionError> {
    let request0 = ListInputDevicesRequest;
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn open_device(
    conn: &mut SocketConnection,
    device_id: u8,
    forget: bool,
) -> Result<Cookie<OpenDeviceReply>, ConnectionError> {
    let request0 = OpenDeviceRequest { device_id };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn close_device(
    conn: &mut SocketConnection,
    device_id: u8,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = CloseDeviceRequest { device_id };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_device_mode(
    conn: &mut SocketConnection,
    device_id: u8,
    mode: ValuatorMode,
    forget: bool,
) -> Result<Cookie<SetDeviceModeReply>, ConnectionError> {
    let request0 = SetDeviceModeRequest { device_id, mode };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn select_extension_event<'c, 'input>(
    conn: &'c mut SocketConnection,
    window: xproto::Window,
    classes: &'input [EventClass],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SelectExtensionEventRequest {
        window,
        classes: Cow::Borrowed(classes),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_selected_extension_events(
    conn: &mut SocketConnection,
    window: xproto::Window,
    forget: bool,
) -> Result<Cookie<GetSelectedExtensionEventsReply>, ConnectionError> {
    let request0 = GetSelectedExtensionEventsRequest { window };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn change_device_dont_propagate_list<'c, 'input>(
    conn: &'c mut SocketConnection,
    window: xproto::Window,
    mode: PropagateMode,
    classes: &'input [EventClass],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = ChangeDeviceDontPropagateListRequest {
        window,
        mode,
        classes: Cow::Borrowed(classes),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_device_dont_propagate_list(
    conn: &mut SocketConnection,
    window: xproto::Window,
    forget: bool,
) -> Result<Cookie<GetDeviceDontPropagateListReply>, ConnectionError> {
    let request0 = GetDeviceDontPropagateListRequest { window };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_device_motion_events<A>(
    conn: &mut SocketConnection,
    start: xproto::Timestamp,
    stop: A,
    device_id: u8,
    forget: bool,
) -> Result<Cookie<GetDeviceMotionEventsReply>, ConnectionError>
where
    A: Into<xproto::Timestamp>,
{
    let stop: xproto::Timestamp = stop.into();
    let request0 = GetDeviceMotionEventsRequest {
        start,
        stop,
        device_id,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn change_keyboard_device(
    conn: &mut SocketConnection,
    device_id: u8,
    forget: bool,
) -> Result<Cookie<ChangeKeyboardDeviceReply>, ConnectionError> {
    let request0 = ChangeKeyboardDeviceRequest { device_id };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn change_pointer_device(
    conn: &mut SocketConnection,
    x_axis: u8,
    y_axis: u8,
    device_id: u8,
    forget: bool,
) -> Result<Cookie<ChangePointerDeviceReply>, ConnectionError> {
    let request0 = ChangePointerDeviceRequest {
        x_axis,
        y_axis,
        device_id,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn grab_device<'c, 'input, A>(
    conn: &'c mut SocketConnection,
    grab_window: xproto::Window,
    time: A,
    this_device_mode: xproto::GrabMode,
    other_device_mode: xproto::GrabMode,
    owner_events: bool,
    device_id: u8,
    classes: &'input [EventClass],
    forget: bool,
) -> Result<Cookie<GrabDeviceReply>, ConnectionError>
where
    A: Into<xproto::Timestamp>,
{
    let time: xproto::Timestamp = time.into();
    let request0 = GrabDeviceRequest {
        grab_window,
        time,
        this_device_mode,
        other_device_mode,
        owner_events,
        device_id,
        classes: Cow::Borrowed(classes),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn ungrab_device<A>(
    conn: &mut SocketConnection,
    time: A,
    device_id: u8,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<xproto::Timestamp>,
{
    let time: xproto::Timestamp = time.into();
    let request0 = UngrabDeviceRequest { time, device_id };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn grab_device_key<'c, 'input, A, B, C>(
    conn: &'c mut SocketConnection,
    grab_window: xproto::Window,
    modifiers: A,
    modifier_device: B,
    grabbed_device: u8,
    key: C,
    this_device_mode: xproto::GrabMode,
    other_device_mode: xproto::GrabMode,
    owner_events: bool,
    classes: &'input [EventClass],
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<u16>,
    B: Into<u8>,
    C: Into<u8>,
{
    let modifiers: u16 = modifiers.into();
    let modifier_device: u8 = modifier_device.into();
    let key: u8 = key.into();
    let request0 = GrabDeviceKeyRequest {
        grab_window,
        modifiers,
        modifier_device,
        grabbed_device,
        key,
        this_device_mode,
        other_device_mode,
        owner_events,
        classes: Cow::Borrowed(classes),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn ungrab_device_key<A, B, C>(
    conn: &mut SocketConnection,
    grab_window: xproto::Window,
    modifiers: A,
    modifier_device: B,
    key: C,
    grabbed_device: u8,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<u16>,
    B: Into<u8>,
    C: Into<u8>,
{
    let modifiers: u16 = modifiers.into();
    let modifier_device: u8 = modifier_device.into();
    let key: u8 = key.into();
    let request0 = UngrabDeviceKeyRequest {
        grab_window,
        modifiers,
        modifier_device,
        key,
        grabbed_device,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn grab_device_button<'c, 'input, A, B, C>(
    conn: &'c mut SocketConnection,
    grab_window: xproto::Window,
    grabbed_device: u8,
    modifier_device: A,
    modifiers: B,
    this_device_mode: xproto::GrabMode,
    other_device_mode: xproto::GrabMode,
    button: C,
    owner_events: bool,
    classes: &'input [EventClass],
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<u8>,
    B: Into<u16>,
    C: Into<u8>,
{
    let modifier_device: u8 = modifier_device.into();
    let modifiers: u16 = modifiers.into();
    let button: u8 = button.into();
    let request0 = GrabDeviceButtonRequest {
        grab_window,
        grabbed_device,
        modifier_device,
        modifiers,
        this_device_mode,
        other_device_mode,
        button,
        owner_events,
        classes: Cow::Borrowed(classes),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn ungrab_device_button<A, B, C>(
    conn: &mut SocketConnection,
    grab_window: xproto::Window,
    modifiers: A,
    modifier_device: B,
    button: C,
    grabbed_device: u8,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<u16>,
    B: Into<u8>,
    C: Into<u8>,
{
    let modifiers: u16 = modifiers.into();
    let modifier_device: u8 = modifier_device.into();
    let button: u8 = button.into();
    let request0 = UngrabDeviceButtonRequest {
        grab_window,
        modifiers,
        modifier_device,
        button,
        grabbed_device,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn allow_device_events<A>(
    conn: &mut SocketConnection,
    time: A,
    mode: DeviceInputMode,
    device_id: u8,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<xproto::Timestamp>,
{
    let time: xproto::Timestamp = time.into();
    let request0 = AllowDeviceEventsRequest {
        time,
        mode,
        device_id,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_device_focus(
    conn: &mut SocketConnection,
    device_id: u8,
    forget: bool,
) -> Result<Cookie<GetDeviceFocusReply>, ConnectionError> {
    let request0 = GetDeviceFocusRequest { device_id };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_device_focus<A, B>(
    conn: &mut SocketConnection,
    focus: A,
    time: B,
    revert_to: xproto::InputFocus,
    device_id: u8,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<xproto::Window>,
    B: Into<xproto::Timestamp>,
{
    let focus: xproto::Window = focus.into();
    let time: xproto::Timestamp = time.into();
    let request0 = SetDeviceFocusRequest {
        focus,
        time,
        revert_to,
        device_id,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_feedback_control(
    conn: &mut SocketConnection,
    device_id: u8,
    forget: bool,
) -> Result<Cookie<GetFeedbackControlReply>, ConnectionError> {
    let request0 = GetFeedbackControlRequest { device_id };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn change_feedback_control<A>(
    conn: &mut SocketConnection,
    mask: A,
    device_id: u8,
    feedback_id: u8,
    feedback: FeedbackCtl,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<u32>,
{
    let mask: u32 = mask.into();
    let request0 = ChangeFeedbackControlRequest {
        mask,
        device_id,
        feedback_id,
        feedback,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_device_key_mapping(
    conn: &mut SocketConnection,
    device_id: u8,
    first_keycode: KeyCode,
    count: u8,
    forget: bool,
) -> Result<Cookie<GetDeviceKeyMappingReply>, ConnectionError> {
    let request0 = GetDeviceKeyMappingRequest {
        device_id,
        first_keycode,
        count,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn change_device_key_mapping<'c, 'input>(
    conn: &'c mut SocketConnection,
    device_id: u8,
    first_keycode: KeyCode,
    keysyms_per_keycode: u8,
    keycode_count: u8,
    keysyms: &'input [xproto::Keysym],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = ChangeDeviceKeyMappingRequest {
        device_id,
        first_keycode,
        keysyms_per_keycode,
        keycode_count,
        keysyms: Cow::Borrowed(keysyms),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_device_modifier_mapping(
    conn: &mut SocketConnection,
    device_id: u8,
    forget: bool,
) -> Result<Cookie<GetDeviceModifierMappingReply>, ConnectionError> {
    let request0 = GetDeviceModifierMappingRequest { device_id };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_device_modifier_mapping<'c, 'input>(
    conn: &'c mut SocketConnection,
    device_id: u8,
    keymaps: &'input [u8],
    forget: bool,
) -> Result<Cookie<SetDeviceModifierMappingReply>, ConnectionError> {
    let request0 = SetDeviceModifierMappingRequest {
        device_id,
        keymaps: Cow::Borrowed(keymaps),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_device_button_mapping(
    conn: &mut SocketConnection,
    device_id: u8,
    forget: bool,
) -> Result<Cookie<GetDeviceButtonMappingReply>, ConnectionError> {
    let request0 = GetDeviceButtonMappingRequest { device_id };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_device_button_mapping<'c, 'input>(
    conn: &'c mut SocketConnection,
    device_id: u8,
    map: &'input [u8],
    forget: bool,
) -> Result<Cookie<SetDeviceButtonMappingReply>, ConnectionError> {
    let request0 = SetDeviceButtonMappingRequest {
        device_id,
        map: Cow::Borrowed(map),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn query_device_state(
    conn: &mut SocketConnection,
    device_id: u8,
    forget: bool,
) -> Result<Cookie<QueryDeviceStateReply>, ConnectionError> {
    let request0 = QueryDeviceStateRequest { device_id };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn device_bell(
    conn: &mut SocketConnection,
    device_id: u8,
    feedback_id: u8,
    feedback_class: u8,
    percent: i8,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DeviceBellRequest {
        device_id,
        feedback_id,
        feedback_class,
        percent,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn set_device_valuators<'c, 'input>(
    conn: &'c mut SocketConnection,
    device_id: u8,
    first_valuator: u8,
    valuators: &'input [i32],
    forget: bool,
) -> Result<Cookie<SetDeviceValuatorsReply>, ConnectionError> {
    let request0 = SetDeviceValuatorsRequest {
        device_id,
        first_valuator,
        valuators: Cow::Borrowed(valuators),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_device_control(
    conn: &mut SocketConnection,
    control_id: DeviceControl,
    device_id: u8,
    forget: bool,
) -> Result<Cookie<GetDeviceControlReply>, ConnectionError> {
    let request0 = GetDeviceControlRequest {
        control_id,
        device_id,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn change_device_control(
    conn: &mut SocketConnection,
    control_id: DeviceControl,
    device_id: u8,
    control: DeviceCtl,
    forget: bool,
) -> Result<Cookie<ChangeDeviceControlReply>, ConnectionError> {
    let request0 = ChangeDeviceControlRequest {
        control_id,
        device_id,
        control,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn list_device_properties(
    conn: &mut SocketConnection,
    device_id: u8,
    forget: bool,
) -> Result<Cookie<ListDevicePropertiesReply>, ConnectionError> {
    let request0 = ListDevicePropertiesRequest { device_id };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn change_device_property<'c, 'input>(
    conn: &'c mut SocketConnection,
    property: xproto::Atom,
    type_: xproto::Atom,
    device_id: u8,
    mode: xproto::PropMode,
    num_items: u32,
    items: &'input ChangeDevicePropertyAux,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = ChangeDevicePropertyRequest {
        property,
        type_,
        device_id,
        mode,
        num_items,
        items: Cow::Borrowed(items),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn delete_device_property(
    conn: &mut SocketConnection,
    property: xproto::Atom,
    device_id: u8,
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = DeleteDevicePropertyRequest {
        property,
        device_id,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn get_device_property(
    conn: &mut SocketConnection,
    property: xproto::Atom,
    type_: xproto::Atom,
    offset: u32,
    len: u32,
    device_id: u8,
    delete: bool,
    forget: bool,
) -> Result<Cookie<GetDevicePropertyReply>, ConnectionError> {
    let request0 = GetDevicePropertyRequest {
        property,
        type_,
        offset,
        len,
        device_id,
        delete,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn xi_query_pointer<A>(
    conn: &mut SocketConnection,
    window: xproto::Window,
    deviceid: A,
    forget: bool,
) -> Result<Cookie<XIQueryPointerReply>, ConnectionError>
where
    A: Into<DeviceId>,
{
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIQueryPointerRequest { window, deviceid };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn xi_warp_pointer<A>(
    conn: &mut SocketConnection,
    src_win: xproto::Window,
    dst_win: xproto::Window,
    src_x: Fp1616,
    src_y: Fp1616,
    src_width: u16,
    src_height: u16,
    dst_x: Fp1616,
    dst_y: Fp1616,
    deviceid: A,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<DeviceId>,
{
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIWarpPointerRequest {
        src_win,
        dst_win,
        src_x,
        src_y,
        src_width,
        src_height,
        dst_x,
        dst_y,
        deviceid,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn xi_change_cursor<A>(
    conn: &mut SocketConnection,
    window: xproto::Window,
    cursor: xproto::Cursor,
    deviceid: A,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<DeviceId>,
{
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIChangeCursorRequest {
        window,
        cursor,
        deviceid,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn xi_change_hierarchy<'c, 'input>(
    conn: &'c mut SocketConnection,
    changes: &'input [HierarchyChange],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = XIChangeHierarchyRequest {
        changes: Cow::Borrowed(changes),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn xi_set_client_pointer<A>(
    conn: &mut SocketConnection,
    window: xproto::Window,
    deviceid: A,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<DeviceId>,
{
    let deviceid: DeviceId = deviceid.into();
    let request0 = XISetClientPointerRequest { window, deviceid };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn xi_get_client_pointer(
    conn: &mut SocketConnection,
    window: xproto::Window,
    forget: bool,
) -> Result<Cookie<XIGetClientPointerReply>, ConnectionError> {
    let request0 = XIGetClientPointerRequest { window };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn xi_select_events<'c, 'input>(
    conn: &'c mut SocketConnection,
    window: xproto::Window,
    masks: &'input [EventMask],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = XISelectEventsRequest {
        window,
        masks: Cow::Borrowed(masks),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn xi_query_version(
    conn: &mut SocketConnection,
    major_version: u16,
    minor_version: u16,
    forget: bool,
) -> Result<Cookie<XIQueryVersionReply>, ConnectionError> {
    let request0 = XIQueryVersionRequest {
        major_version,
        minor_version,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn xi_query_device<A>(
    conn: &mut SocketConnection,
    deviceid: A,
    forget: bool,
) -> Result<Cookie<XIQueryDeviceReply>, ConnectionError>
where
    A: Into<DeviceId>,
{
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIQueryDeviceRequest { deviceid };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn xi_set_focus<A, B>(
    conn: &mut SocketConnection,
    window: xproto::Window,
    time: A,
    deviceid: B,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<xproto::Timestamp>,
    B: Into<DeviceId>,
{
    let time: xproto::Timestamp = time.into();
    let deviceid: DeviceId = deviceid.into();
    let request0 = XISetFocusRequest {
        window,
        time,
        deviceid,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn xi_get_focus<A>(
    conn: &mut SocketConnection,
    deviceid: A,
    forget: bool,
) -> Result<Cookie<XIGetFocusReply>, ConnectionError>
where
    A: Into<DeviceId>,
{
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIGetFocusRequest { deviceid };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn xi_grab_device<'c, 'input, A, B>(
    conn: &'c mut SocketConnection,
    window: xproto::Window,
    time: A,
    cursor: xproto::Cursor,
    deviceid: B,
    mode: xproto::GrabMode,
    paired_device_mode: xproto::GrabMode,
    owner_events: GrabOwner,
    mask: &'input [u32],
    forget: bool,
) -> Result<Cookie<XIGrabDeviceReply>, ConnectionError>
where
    A: Into<xproto::Timestamp>,
    B: Into<DeviceId>,
{
    let time: xproto::Timestamp = time.into();
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIGrabDeviceRequest {
        window,
        time,
        cursor,
        deviceid,
        mode,
        paired_device_mode,
        owner_events,
        mask: Cow::Borrowed(mask),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn xi_ungrab_device<A, B>(
    conn: &mut SocketConnection,
    time: A,
    deviceid: B,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<xproto::Timestamp>,
    B: Into<DeviceId>,
{
    let time: xproto::Timestamp = time.into();
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIUngrabDeviceRequest { time, deviceid };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn xi_allow_events<A, B>(
    conn: &mut SocketConnection,
    time: A,
    deviceid: B,
    event_mode: EventMode,
    touchid: u32,
    grab_window: xproto::Window,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<xproto::Timestamp>,
    B: Into<DeviceId>,
{
    let time: xproto::Timestamp = time.into();
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIAllowEventsRequest {
        time,
        deviceid,
        event_mode,
        touchid,
        grab_window,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn xi_passive_grab_device<'c, 'input, A, B>(
    conn: &'c mut SocketConnection,
    time: A,
    grab_window: xproto::Window,
    cursor: xproto::Cursor,
    detail: u32,
    deviceid: B,
    grab_type: GrabType,
    grab_mode: GrabMode22,
    paired_device_mode: xproto::GrabMode,
    owner_events: GrabOwner,
    mask: &'input [u32],
    modifiers: &'input [u32],
    forget: bool,
) -> Result<Cookie<XIPassiveGrabDeviceReply>, ConnectionError>
where
    A: Into<xproto::Timestamp>,
    B: Into<DeviceId>,
{
    let time: xproto::Timestamp = time.into();
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIPassiveGrabDeviceRequest {
        time,
        grab_window,
        cursor,
        detail,
        deviceid,
        grab_type,
        grab_mode,
        paired_device_mode,
        owner_events,
        mask: Cow::Borrowed(mask),
        modifiers: Cow::Borrowed(modifiers),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn xi_passive_ungrab_device<'c, 'input, A>(
    conn: &'c mut SocketConnection,
    grab_window: xproto::Window,
    detail: u32,
    deviceid: A,
    grab_type: GrabType,
    modifiers: &'input [u32],
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<DeviceId>,
{
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIPassiveUngrabDeviceRequest {
        grab_window,
        detail,
        deviceid,
        grab_type,
        modifiers: Cow::Borrowed(modifiers),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn xi_list_properties<A>(
    conn: &mut SocketConnection,
    deviceid: A,
    forget: bool,
) -> Result<Cookie<XIListPropertiesReply>, ConnectionError>
where
    A: Into<DeviceId>,
{
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIListPropertiesRequest { deviceid };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn xi_change_property<'c, 'input, A>(
    conn: &'c mut SocketConnection,
    deviceid: A,
    mode: xproto::PropMode,
    property: xproto::Atom,
    type_: xproto::Atom,
    num_items: u32,
    items: &'input XIChangePropertyAux,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<DeviceId>,
{
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIChangePropertyRequest {
        deviceid,
        mode,
        property,
        type_,
        num_items,
        items: Cow::Borrowed(items),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn xi_delete_property<A>(
    conn: &mut SocketConnection,
    deviceid: A,
    property: xproto::Atom,
    forget: bool,
) -> Result<VoidCookie, ConnectionError>
where
    A: Into<DeviceId>,
{
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIDeletePropertyRequest { deviceid, property };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn xi_get_property<A>(
    conn: &mut SocketConnection,
    deviceid: A,
    delete: bool,
    property: xproto::Atom,
    type_: xproto::Atom,
    offset: u32,
    len: u32,
    forget: bool,
) -> Result<Cookie<XIGetPropertyReply>, ConnectionError>
where
    A: Into<DeviceId>,
{
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIGetPropertyRequest {
        deviceid,
        delete,
        property,
        type_,
        offset,
        len,
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn xi_get_selected_events(
    conn: &mut SocketConnection,
    window: xproto::Window,
    forget: bool,
) -> Result<Cookie<XIGetSelectedEventsReply>, ConnectionError> {
    let request0 = XIGetSelectedEventsRequest { window };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn xi_barrier_release_pointer<'c, 'input>(
    conn: &'c mut SocketConnection,
    barriers: &'input [BarrierReleasePointerInfo],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = XIBarrierReleasePointerRequest {
        barriers: Cow::Borrowed(barriers),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}

pub fn send_extension_event<'c, 'input>(
    conn: &'c mut SocketConnection,
    destination: xproto::Window,
    device_id: u8,
    propagate: bool,
    events: &'input [EventForSend],
    classes: &'input [EventClass],
    forget: bool,
) -> Result<VoidCookie, ConnectionError> {
    let request0 = SendExtensionEventRequest {
        destination,
        device_id,
        propagate,
        events: Cow::Borrowed(events),
        classes: Cow::Borrowed(classes),
    };
    let bytes = request0.serialize(major_opcode(conn)?);
    Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))
}
