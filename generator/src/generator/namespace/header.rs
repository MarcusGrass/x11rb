use crate::generator::output::Output;

use xcbgen::defs as xcbdefs;

#[derive(Debug, PartialEq, Copy, Clone)]
pub(super) enum Mode {
    Protocol,
    X11rb,
}

pub(super) fn write_header(out: &mut Output, ns: &xcbdefs::Namespace, mode: Mode) {
    if let Some(info) = &ns.ext_info {
        outln!(out, "//! Bindings to the `{}` X11 extension.", info.name);
    } else {
        outln!(out, "//! Bindings to the core X11 protocol.");
        outln!(out, "//!");
        outln!(
            out,
            "//! For more documentation on the X11 protocol, see the"
        );
        outln!(
            out,
            "//! [protocol reference manual](https://www.x.org/releases/X11R7.6/doc/xproto/x11protocol.html).",
        );
        outln!(
            out,
            "//! This is especially recommended for looking up the exact semantics of"
        );
        outln!(out, "//! specific errors, events, or requests.");
    }

    outln!(out, "");
    outln!(out, "#![allow(clippy::too_many_arguments)]");
    outln!(out, "");
    if mode == Mode::Protocol {
        outln!(out, "#[allow(unused_imports)]");
        outln!(out, "use core::convert::TryFrom;");
        outln!(out, "use crate::errors::ParseError;");
        outln!(out, "#[allow(unused_imports)]");
        outln!(out, "use crate::x11_utils::TryIntoUSize;");
    }
    outln!(out, "#[allow(unused_imports)]");
    outln!(out, "use std::borrow::Cow;");
    if mode == Mode::Protocol {
        outln!(out, "#[allow(unused_imports)]");
        outln!(
            out,
            "use crate::utils::{{pretty_print_bitmask, pretty_print_enum}};"
        );
    }
    outln!(out, "#[allow(unused_imports)]");
    outln!(
        out,
        "use crate::x11_utils::{{RequestHeader, Serialize, TryParse, TryParseFd}};"
    );
    outln!(out, "#[allow(unused_imports)]");
    outln!(out, "use std::convert::TryInto;");
    if mode == Mode::X11rb {
        outln!(out, "use crate::SocketConnection;");
        outln!(out, "#[allow(unused_imports)]");
        outln!(out, "use crate::cookie::{{VoidCookie, Cookie}};");
        outln!(out, "use crate::errors::ConnectionError;");
        outln!(out, "#[allow(unused_imports)]");
        outln!(out, "use crate::errors::ReplyOrIdError;");
    }

    let mut imports = ns
        .imports
        .borrow()
        .values()
        .map(|import| import.name.clone())
        .collect::<Vec<_>>();
    imports.sort();
    for import in &imports {
        outln!(out, "#[allow(unused_imports)]");
        outln!(out, "use super::{};", import);
    }

    if mode == Mode::X11rb {
        outln!(out, "");
        outln!(out, "pub use crate::protocol::{}::*;", ns.header);
    }

    if let Some(ref ext_info) = ns.ext_info {
        if mode == Mode::Protocol {
            outln!(out, "");
            outln!(out, "/// The X11 name of the extension for QueryExtension");
            outln!(
                out,
                "pub const X11_EXTENSION_NAME: &str = \"{}\";",
                ext_info.xname,
            );
            outln!(out, "");
            outln!(
                out,
                "/// The version number of this extension that this client library supports.",
            );
            outln!(out, "///");
            outln!(
                out,
                "/// This constant contains the version number of this extension that is supported",
            );
            outln!(
                out,
                "/// by this build of x11rb. For most things, it does not make sense to use this",
            );
            outln!(
                out,
                "/// information. If you need to send a `QueryVersion`, it is recommended to \
             instead"
            );
            outln!(
                out,
                "/// send the maximum version of the extension that you need.",
            );
            outln!(
                out,
                "pub const X11_XML_VERSION: (u32, u32) = ({}, {});",
                ext_info.major_version,
                ext_info.minor_version,
            );
        }

        if mode == Mode::X11rb {
            outln!(out, "");
            outln!(out, "/// Get the major opcode of this extension");
            outln!(
                out,
                "fn major_opcode(conn: &SocketConnection) -> Result<u8, ConnectionError> {{"
            );
            out.indented(|out| {
                outln!(
                    out,
                    "let info = conn.extension_information(X11_EXTENSION_NAME);",
                );
                outln!(
                    out,
                    "let info = info.ok_or(ConnectionError::UnsupportedExtension)?;",
                );
                outln!(out, "Ok(info.major_opcode)");
            });
            outln!(out, "}}");
        }
    }
    outln!(out, "");
}
