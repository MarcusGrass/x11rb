#![allow(
    clippy::missing_errors_doc,
    clippy::doc_markdown,
    clippy::default_trait_access,
    clippy::unused_self,
    clippy::missing_panics_doc,
    clippy::similar_names,
    clippy::useless_asref
)]
extern crate core;

mod connect;
pub mod cookie;
#[cfg(feature = "cursor")]
pub mod cursor;
pub mod errors;
mod id_allocator;
mod parse_display;
pub mod properties;
pub mod protocol;
pub mod resource_manager;
mod socket_connection;
pub mod utils;
mod wrapper;
pub mod x11_utils;
pub mod xauth;
pub mod xcb;

use crate::protocol::xproto::{Keysym, Timestamp};
pub use socket_connection::SocketConnection;

/// The universal null resource or null atom parameter value for many core X requests
pub const NONE: u32 = 0;

/// This constant can be used for many parameters in `create_window`
pub const COPY_FROM_PARENT: u32 = 0;

/// This constant can be used for the depth parameter in `create_window`. It indicates to use the
/// parent window's depth.
pub const COPY_DEPTH_FROM_PARENT: u8 = 0;

/// This constant can be used for the class parameter in `create_window`. It indicates to use the
/// parent window's class.
pub const COPY_CLASS_FROM_PARENT: u16 = 0;

/// This constant can be used in most request that take a timestamp argument
pub const CURRENT_TIME: Timestamp = 0;

/// This constant can be used to fill unused entries in `Keysym` tables
pub const NO_SYMBOL: Keysym = 0;
