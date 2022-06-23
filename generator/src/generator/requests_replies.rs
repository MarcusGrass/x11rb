use std::collections::HashMap;
use std::rc::Rc;

use xcbgen::defs as xcbdefs;

use super::output::Output;

#[derive(Debug, Default)]
pub(super) struct PerModuleEnumCases {
    /// Lines that belong in the Request enum definition.
    pub(super) request_variants: Vec<String>,
    /// Lines that belong in the definition of Request::reply_parser.
    pub(super) reply_parse_cases: Vec<String>,
    /// Lines that belong in the definition of Request::into_owned.
    pub(super) request_into_owned_cases: Vec<String>,
    /// Lines that belong in the Reply enum definition.
    pub(super) reply_variants: Vec<String>,
    /// Impls for From<ReplyType> for Reply enum.
    pub(super) reply_from_cases: Vec<String>,
}

pub(super) type EnumCases = HashMap<String, PerModuleEnumCases>;

/// Generate the Request and Reply enums containing all possible requests and replies, respectively.
pub(super) fn generate(out: &mut Output, module: &xcbdefs::Module, _enum_cases: EnumCases) {
    let namespaces = module.sorted_namespaces();
    outln!(
        out,
        "/// Get the name of a request from its extension name and opcodes.",
    );
    outln!(out, "pub(crate) fn request_name(extension: Option<&str>, major_opcode: u8, minor_opcode: u16) -> Option<&'static str> {{");
    out.indented(|out| {
        outln!(out, "// Check if this is a core protocol request.");
        outln!(out, "match major_opcode {{");
        out.indented(|out| {
            let xproto_ns = module.namespace("xproto").unwrap();
            for def in sorted_requests(&*xproto_ns) {
                outln!(out, "{} => return Some(\"{}\"),", def.opcode, def.name);
            }
            outln!(out, "_ => (),");
        });
        outln!(out, "}}");

        outln!(out, "// Check the extension");
        outln!(out, "match (extension, minor_opcode) {{");
        out.indented(|out| {
            for ns in &namespaces {
                if ns.header == "xproto" {
                    continue;
                }
                let has_feature = super::ext_has_feature(&ns.header);
                for def in sorted_requests(ns) {
                    if has_feature {
                        outln!(out, "#[cfg(feature = \"{}\")]", ns.header);
                    }
                    outln!(
                        out,
                        "(Some({}::X11_EXTENSION_NAME), {}) => Some(\"{}\"),",
                        ns.header,
                        def.opcode,
                        def.name,
                    );
                }
            }
            outln!(out, "_ => None,");
        });
        outln!(out, "}}");
    });
    outln!(out, "}}");
    outln!(out, "");
}

/// Get all requests in the namespace in a sorted order
fn sorted_requests(ns: &xcbgen::defs::Namespace) -> Vec<Rc<xcbgen::defs::RequestDef>> {
    let mut events: Vec<_> = ns.request_defs.borrow().values().cloned().collect();
    events.sort_by(|a, b| a.opcode.cmp(&b.opcode));
    events
}
