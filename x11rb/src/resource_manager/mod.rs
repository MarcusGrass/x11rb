//! X11 resource manager library.
//! The code in this module is only available when the `resource_manager` feature of the library is
//! enabled.

use crate::errors::ReplyError;
use crate::protocol::xproto::GetPropertyReply;
use crate::SocketConnection;
use std::ffi::OsString;

pub mod protocol;
use crate::utils::get_hostname;
use crate::x11_utils::{TryParse, X11Error};
use protocol::Database;

fn send_request(conn: &mut SocketConnection) -> Result<GetPropertyReply, ReplyError> {
    let mut request = Database::GET_RESOURCE_DATABASE;
    request.window = conn.setup().roots[0].root;
    let seq = conn.write(request.serialize().as_ref(), false)?;
    let response = conn.block_for_reply(seq)?;
    if let Ok((reply, _)) = GetPropertyReply::try_parse(&response) {
        Ok(reply)
    } else {
        Err(ReplyError::X11Error(X11Error::try_parse(
            &response,
            &conn.extensions,
        )?))
    }
}

/// Create a new X11 resource database from the `RESOURCE_MANAGER` property of the first
/// screen's root window.
///
/// This function returns an error if the `GetProperty` request to get the `RESOURCE_MANAGER`
/// property fails. It returns `Ok(None)` if the property does not exist, has the wrong format,
/// or is empty.
pub fn new_from_resource_manager(
    conn: &mut SocketConnection,
) -> Result<Option<Database>, ReplyError> {
    Ok(Database::new_from_get_property_reply(&send_request(conn)?))
}

/// Create a new X11 resource database from the default locations.
///
/// The default location is a combination of two places. First, the following places are
/// searched for data:
/// - The `RESOURCE_MANAGER` property of the first screen's root window (See
///   [`Self::new_from_resource_manager`]).
/// - If not found, the file `$HOME/.Xresources` is loaded.
/// - If not found, the file `$HOME/.Xdefaults` is loaded.
///
/// The result of the above search of the above search is combined with:
/// - The contents of the file `$XENVIRONMENT`, if this environment variable is set.
/// - Otherwise, the contents of `$HOME/.Xdefaults-[hostname]`.
///
/// This function only returns an error if communication with the X11 server fails. All other
/// errors are ignored. It might be that an empty database is returned.
///
/// The behaviour of this function is mostly equivalent to Xlib's `XGetDefault()`. The
/// exception is that `XGetDefault()` does not load `$HOME/.Xresources`.
///
/// The behaviour of this function is equivalent to xcb-util-xrm's
/// `xcb_xrm_database_from_default()`.
pub fn new_from_default(conn: &mut SocketConnection) -> Result<Database, ReplyError> {
    Ok(Database::new_from_default(
        &send_request(conn)?,
        OsString::from(get_hostname().unwrap_or_else(|| "localhost".to_string())), // TODO: Maybe fix?
    ))
}
