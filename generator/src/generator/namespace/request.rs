use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::TryFrom;

use super::{
    emit_doc, expr_to_str, ext_params_to_call_args, field_is_visible, gather_deducible_fields,
    get_ns_name_prefix, parse, rust_value_type_is_u8, serialize, special_cases, struct_type,
    switch, to_rust_type_name, to_rust_variable_name, CaseInfo, DeducibleField, Derives,
    NamespaceGenerator, Output, PerModuleEnumCases, StructSizeConstraint,
};

use xcbgen::defs as xcbdefs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IovecConversion {
    // No conversion is required.
    None,
    // A simple .into() is enough.
    Into,
    // Call cow_strip_length.
    CowStripLength,
}

/// A helper to represent either simple or borrowed types.
#[derive(Debug, Clone)]
enum Type {
    /// This type is simple can we can just use the string provided.
    Simple(String),
    /// This type has a lifetime and needs to be a borrow or a Cow, depending
    /// on the context. But during parsing it has to be a Cow::Owned because
    /// it is generated.
    VariableOwnership(String),
    /// This type has a lifetime and needs to be a borrow or a Cow, but
    /// when parsing it can be borrowed from the byte stream.
    VariableOwnershipRawBytes(String),
}

impl Type {
    /// Does this type need a Cow?
    fn needs_any_cow(&self) -> bool {
        match self {
            Type::Simple(_) => false,
            Type::VariableOwnership(_) | Type::VariableOwnershipRawBytes(_) => true,
        }
    }

    /// Render this type in a form suitable for function arguments.
    fn as_argument(&self) -> Cow<'_, str> {
        match self {
            Type::Simple(ref type_) => type_.into(),
            Type::VariableOwnership(ref type_) | Type::VariableOwnershipRawBytes(ref type_) => {
                format!("&'input {}", type_).into()
            }
        }
    }

    /// Render this type in a form suitable for struct fields.
    fn as_field(&self) -> Cow<'_, str> {
        match self {
            Type::Simple(ref type_) => type_.into(),
            Type::VariableOwnership(ref type_) | Type::VariableOwnershipRawBytes(ref type_) => {
                format!("Cow<'input, {}>", type_).into()
            }
        }
    }
}

/// Some information about the fields of a request.
struct GatheredRequestFields {
    /// Whether lifetimes in the request need to be explicitly
    /// specified. If a request function takes any other
    /// references other than the connection object, we'll need
    /// to disambiguate the lifetimes for rustc.
    needs_lifetime: bool,
    /// Function arguments
    /// `(name, type)`
    args: Vec<(String, Type)>,
    /// Request arguments
    /// The type here has been converted as necessary.
    /// `(name, type)`
    request_args: Vec<(String, Type)>,
    /// Generic type parameters
    ///
    /// `(name, where clause)`
    generics: Vec<(String, String)>,
    /// Code at the beginning of the function.
    preamble: Vec<String>,
}

pub(super) fn generate_request(
    generator: &NamespaceGenerator<'_, '_>,
    request_def: &xcbdefs::RequestDef,
    proto_out: &mut Output,
    x11rb_out: &mut Output,
    trait_out: &mut Output,
    enum_cases: &mut PerModuleEnumCases,
) {
    let name = to_rust_type_name(&request_def.name);

    let mut function_name = super::super::camel_case_to_lower_snake(&name);
    if function_name == "await" {
        function_name.push('_');
    }

    // If this have a <switch>, we generate an *Aux structure
    let request_fields = request_def.fields.borrow();
    let switch_fields = request_fields
        .iter()
        .filter_map(|field| match field {
            xcbdefs::FieldDef::Switch(switch_field) => Some(switch_field),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(
        switch_fields.len() <= 1,
        "request {}::{} has more than one <switch> field",
        generator.ns.header,
        request_def.name,
    );

    let deducible_fields = gather_deducible_fields(&*request_fields);

    if switch_fields.len() == 1 {
        if let Some(aux_start_align) = switch_fields[0].required_start_align {
            assert_eq!(aux_start_align.offset(), 0);
        }
        generate_aux(
            generator,
            request_def,
            switch_fields[0],
            &function_name,
            proto_out,
        );
    }

    outln!(proto_out, "/// Opcode for the {} request", name);
    outln!(
        proto_out,
        "pub const {}_REQUEST: u8 = {};",
        super::super::camel_case_to_upper_snake(&name),
        request_def.opcode,
    );

    let gathered = if let Some(v) = gather_request_fields(generator, request_def, &deducible_fields)
    {
        v
    } else {
        return;
    };

    emit_request_struct(
        generator,
        request_def,
        &name,
        &deducible_fields,
        &gathered,
        proto_out,
    );
    let ns_prefix = get_ns_name_prefix(generator.ns);
    let lifetime_block = if gathered.needs_lifetime {
        "<'input>"
    } else {
        ""
    };
    enum_cases.request_variants.push(format!(
        "{ns_prefix}{name}({header}::{name}Request{lifetime}),",
        ns_prefix = ns_prefix,
        name = name,
        header = generator.ns.header,
        lifetime = lifetime_block
    ));
    emit_request_function(request_def, &name, &function_name, &gathered, x11rb_out);
    emit_request_trait_function(request_def, &name, &function_name, &gathered, trait_out);

    special_cases::handle_request(request_def, proto_out);

    outln!(proto_out, "");
    outln!(x11rb_out, "");

    if let Some(ref reply) = request_def.reply {
        let reply_struct_name = format!("{}Reply", name);
        enum_cases.reply_variants.push(format!(
            "{ns_prefix}{name}({header}::{name}Reply),",
            ns_prefix = ns_prefix,
            name = name,
            header = generator.ns.header,
        ));
        enum_cases.reply_parse_cases.push(format!(
            "Request::{ns_prefix}{name}(_) => Some({func}::<{header}::{name}Request{lifetime}>),",
            ns_prefix = ns_prefix,
            name = name,
            header = generator.ns.header,
            func = "parse_reply",
            lifetime = if gathered.needs_lifetime { "<'_>" } else { "" }
        ));
        enum_cases.reply_from_cases.push(format!(
            r#"impl From<{header}::{name}Reply> for Reply {{
  fn from(reply: {header}::{name}Reply) -> Reply {{
    Reply::{ns_prefix}{name}(reply)
  }}
}}"#,
            ns_prefix = ns_prefix,
            name = name,
            header = generator.ns.header,
        ));
        let reply_fields = reply.fields.borrow();
        let mut reply_derives = Derives::all();
        generator.filter_derives_for_fields(&mut reply_derives, &*reply_fields, false);
        struct_type::emit_struct_type(
            generator,
            &reply_struct_name,
            &name,
            reply_derives,
            &*reply_fields,
            &[],
            false,
            true,
            StructSizeConstraint::EmbeddedLength { minimum: 32 },
            false,
            false,
            reply.doc.as_ref(),
            proto_out,
        );

        outln!(proto_out, "");
    } else {
        enum_cases.reply_parse_cases.push(format!(
            "Request::{ns_prefix}{name}(_) => None,",
            ns_prefix = ns_prefix,
            name = name,
        ));
    }

    if gathered.needs_lifetime {
        enum_cases.request_into_owned_cases.push(format!(
            "Request::{ns_prefix}{name}(req) => Request::{ns_prefix}{name}(req.into_owned()),",
            ns_prefix = ns_prefix,
            name = name,
        ));
    } else {
        enum_cases.request_into_owned_cases.push(format!(
            "Request::{ns_prefix}{name}(req) => Request::{ns_prefix}{name}(req),",
            ns_prefix = ns_prefix,
            name = name,
        ));
    }
}

fn generate_aux(
    generator: &NamespaceGenerator<'_, '_>,
    request_def: &xcbdefs::RequestDef,
    switch_field: &xcbdefs::SwitchField,
    function_name: &str,
    out: &mut Output,
) {
    let aux_name = format!("{}Aux", request_def.name);

    if switch_field.kind == xcbdefs::SwitchKind::Case {
        switch::emit_switch_type(generator, switch_field, &aux_name, true, true, None, out);
    } else {
        let doc = format!(
            "Auxiliary and optional information for the `{}` function",
            function_name,
        );
        let cases_infos = switch::emit_switch_type(
            generator,
            switch_field,
            &aux_name,
            false,
            true,
            Some(&doc),
            out,
        );

        outln!(out, "impl {} {{", aux_name);
        out.indented(|out| {
            outln!(
                out,
                "/// Create a new instance with all fields unset / not present."
            );
            outln!(out, "#[must_use]");
            outln!(out, "pub fn new() -> Self {{");
            outln!(out.indent(), "Default::default()");
            outln!(out, "}}");

            for (case, case_info) in switch_field.cases.iter().zip(cases_infos.iter()) {
                let (rust_case_name, rust_case_type) = match case_info {
                    CaseInfo::SingleField(index) => {
                        let fields = case.fields.borrow();
                        let single_field = &fields[*index];
                        let field_name = single_field.name().unwrap();
                        (
                            to_rust_variable_name(field_name),
                            generator.field_to_rust_type(single_field, &aux_name),
                        )
                    }
                    CaseInfo::MultiField(field_name, struct_name) => {
                        (to_rust_variable_name(field_name), struct_name.clone())
                    }
                };
                outln!(
                    out,
                    "/// Set the `{}` field of this structure.",
                    rust_case_name,
                );
                outln!(out, "#[must_use]");
                outln!(
                    out,
                    "pub fn {}<I>(mut self, value: I) -> Self where I: Into<Option<{}>> {{",
                    rust_case_name,
                    rust_case_type,
                );
                outln!(out.indent(), "self.{} = value.into();", rust_case_name);
                outln!(out.indent(), "self");
                outln!(out, "}}");
            }
        });
        outln!(out, "}}");
    }

    special_cases::handle_request_switch(request_def, switch_field, &aux_name, out);

    outln!(out, "");
}

fn emit_request_struct(
    generator: &NamespaceGenerator<'_, '_>,
    request_def: &xcbdefs::RequestDef,
    name: &str,
    deducible_fields: &HashMap<String, DeducibleField>,
    gathered: &GatheredRequestFields,
    out: &mut Output,
) {
    let ns = request_def.namespace.upgrade().unwrap();
    let is_xproto = ns.header == "xproto";

    if let Some(ref doc) = request_def.doc {
        emit_doc(doc, out, Some(deducible_fields));
    }

    let mut derives = Derives::all();
    generator.filter_derives_for_fields(&mut derives, &request_def.fields.borrow(), true);
    let derives = derives.to_list();
    if !derives.is_empty() {
        outln!(out, "#[derive({})]", derives.join(", "));
    }

    let (struct_lifetime_block, _serialize_lifetime_return) = if gathered.needs_lifetime {
        ("<'input>", "'input")
    } else {
        ("", "'static")
    };

    let has_members = !gathered.request_args.is_empty();
    let members_start = if has_members { " {" } else { ";" };
    outln!(
        out,
        "pub struct {name}Request{lifetime}{members_start}",
        name = name,
        lifetime = struct_lifetime_block,
        members_start = members_start
    );
    out.indented(|out| {
        for (member_name, member_type) in &gathered.request_args {
            outln!(out, "pub {name}: {type},",
                   name=member_name,
                   type=member_type.as_field());
        }
    });
    if has_members {
        outln!(out, "}}",);
    }

    // Methods implemented on every request
    outln!(
        out,
        "impl{lifetime} {name}Request{lifetime} {{",
        lifetime = struct_lifetime_block,
        name = name
    );
    out.indented(|out| {
        outln!(
            out,
            "/// Serialize this request into bytes for the provided connection",
        );
        outln!(out, "#[must_use]",);
        outln!(
            out,
            "pub fn serialize(self{opcode}) -> impl AsRef<[u8]> {{",
            opcode = if is_xproto { "" } else { ", major_opcode: u8" },
        );
        out.indented(|out| {
            let fields = request_def.fields.borrow();

            let has_expr_fields = fields
                .iter()
                .any(|field| matches!(field, xcbdefs::FieldDef::Expr(_)));
            // Calculate `VirtualLen` field values because they
            // may be used by <exprfield>s.
            if has_expr_fields {
                for field in fields.iter() {
                    if let xcbdefs::FieldDef::VirtualLen(virtual_len_field) = field {
                        outln!(
                            out,
                            "let {} = u32::try_from(self.{}.len()).unwrap();",
                            to_rust_variable_name(&virtual_len_field.name),
                            to_rust_variable_name(&virtual_len_field.list_name),
                        );
                    }
                }
            }
            let mut request_size = fields
                .iter()
                .try_fold(0, |sum, field| Some(sum + field.size()?));
            // If the request is fixed size, keep it on the stack
            if request_size.is_some() {
                let mut length = 0;
                let mut request_slices = Vec::new();
                let mut fixed_fields_bytes = Vec::new();
                let mut num_fixed_len_slices = 0;
                let mut pad_count = 0;

                for (field_i, field) in fields.iter().enumerate() {
                    let mut next_slice = None;

                    let mut tmp_out = Output::new();
                    serialize::emit_assert_for_field_serialize(
                        generator,
                        field,
                        deducible_fields,
                        |field_name| {
                            let rust_field_name = to_rust_variable_name(field_name);
                            if deducible_fields.contains_key(field_name) {
                                rust_field_name
                            } else {
                                format!("self.{}", rust_field_name)
                            }
                        },
                        &mut tmp_out,
                    );
                    match field {
                        xcbdefs::FieldDef::Pad(pad_field) => match pad_field.kind {
                            xcbdefs::PadKind::Bytes(bytes) => {
                                for _ in 0..bytes {
                                    fixed_fields_bytes.push(String::from("0"));
                                }
                            }
                            xcbdefs::PadKind::Align(align) => {
                                outln!(
                                    tmp_out,
                                    "let padding{} = &[0; {}][..({} - (length_so_far % {})) % {}];",
                                    pad_count,
                                    align - 1,
                                    align,
                                    align,
                                    align,
                                );
                                next_slice =
                                    Some((format!("padding{}", pad_count), IovecConversion::Into));
                                pad_count += 1;
                            }
                        },
                        xcbdefs::FieldDef::Normal(normal_field) => {
                            if normal_field.name == "major_opcode" {
                                if ns.ext_info.is_some() {
                                    fixed_fields_bytes.push(String::from("major_opcode"));
                                } else {
                                    fixed_fields_bytes.push(format!(
                                        "{}_REQUEST",
                                        super::super::camel_case_to_upper_snake(name),
                                    ));
                                }
                            } else if normal_field.name == "minor_opcode" {
                                assert!(ns.ext_info.is_some());
                                fixed_fields_bytes.push(format!(
                                    "{}_REQUEST",
                                    super::super::camel_case_to_upper_snake(name),
                                ));
                            } else if normal_field.name == "length" {
                                // the actual length will be calculated later
                                fixed_fields_bytes.push(String::from("0"));
                                fixed_fields_bytes.push(String::from("0"));
                            } else {
                                let rust_field_name = to_rust_variable_name(&normal_field.name);

                                let was_deduced = if let Some(deducible_field) =
                                    deducible_fields.get(&normal_field.name)
                                {
                                    generator.emit_calc_deducible_field(
                                        field,
                                        deducible_field,
                                        |field_name| {
                                            let rust_field_name = to_rust_variable_name(field_name);
                                            if deducible_fields.contains_key(field_name) {
                                                rust_field_name
                                            } else {
                                                format!("self.{}", rust_field_name)
                                            }
                                        },
                                        &rust_field_name,
                                        out,
                                    );
                                    true
                                } else {
                                    false
                                };
                                let bytes_name = super::postfix_var_name(&rust_field_name, "bytes");
                                if let Some(field_size) = normal_field.type_.size() {
                                    let field_name = if was_deduced {
                                        // If we deduced this value it comes from a local.
                                        rust_field_name
                                    } else {
                                        // Otherwise a member.
                                        format!("self.{}", rust_field_name)
                                    };

                                    outln!(
                                        out,
                                        "let {} = {};",
                                        bytes_name,
                                        serialize::emit_value_serialize(
                                            generator,
                                            &normal_field.type_,
                                            &field_name,
                                            was_deduced,
                                        ),
                                    );
                                    for i in 0..field_size {
                                        fixed_fields_bytes.push(format!("{}[{}]", bytes_name, i));
                                    }
                                } else {
                                    outln!(
                                        tmp_out,
                                        "let {} = self.{}.serialize();",
                                        bytes_name,
                                        rust_field_name,
                                    );
                                    next_slice = Some((bytes_name, IovecConversion::Into));
                                }
                            }
                        }
                        xcbdefs::FieldDef::List(list_field) => {
                            let rust_field_name = to_rust_variable_name(&list_field.name);
                            let bytes_name = super::postfix_var_name(&rust_field_name, "bytes");
                            outln!(
                                out,
                                "let {} = self.{}.as_ref();",
                                bytes_name,
                                rust_field_name
                            );
                            for i in 0..list_field.length().unwrap() {
                                fixed_fields_bytes.push(format!("{}[{}]", bytes_name, i));
                            }
                        }
                        _ => {}
                    }

                    // The XML does not describe trailing padding in requests. Requests
                    // are implicitly padded to a four byte boundary.
                    if next_slice.is_none() && field_i == (fields.len() - 1) {
                        if let Some(ref mut request_size) = request_size {
                            let req_size_rem = *request_size % 4;
                            if req_size_rem != 0 {
                                let pad_size = 4 - req_size_rem;
                                for _ in 0..pad_size {
                                    fixed_fields_bytes.push(String::from("0"));
                                }
                                *request_size += pad_size;
                            }
                        }
                    }

                    if next_slice.is_some() || field_i == (fields.len() - 1) {
                        if !fixed_fields_bytes.is_empty() {
                            let maybe_mut = if num_fixed_len_slices == 0 {
                                // contains the length field, which will be modified
                                "mut "
                            } else {
                                ""
                            };
                            outln!(out, "let {}request{} = [", maybe_mut, num_fixed_len_slices,);
                            for byte in &fixed_fields_bytes {
                                length += 1;
                                outln!(out.indent(), "{},", byte);
                            }
                            outln!(out, "];");
                            request_slices.push(format!("request{}.into()", num_fixed_len_slices));
                            fixed_fields_bytes.clear();
                            num_fixed_len_slices += 1;
                        }
                        if let Some((next_slice, conversion)) = next_slice {
                            outln!(tmp_out, "let referenced: &[u8] = {}.as_ref();", next_slice,);
                            outln!(
                                tmp_out,
                                "let length_so_far = length_so_far + referenced.len();",
                            );
                            match conversion {
                                IovecConversion::None => request_slices.push(next_slice),
                                IovecConversion::Into | IovecConversion::CowStripLength => {
                                    request_slices.push(next_slice.to_string());
                                }
                            }
                        }
                    }

                    out!(out, "{}", tmp_out.into_data());
                }

                // The XML does not describe trailing padding in requests. Requests
                // are implicitly padded to a four byte boundary.
                if let Some(request_size) = request_size {
                    let req_size_rem = request_size % 4;
                    if req_size_rem != 0 {
                        assert_eq!(pad_count, 0);
                        outln!(out, "let padding = vec![0; {}];", 4 - req_size_rem);
                        outln!(out, "let length_so_far = length_so_far + padding.len();");
                        request_slices.push(String::from("padding"));
                    }
                } else {
                    outln!(
                        out,
                        "let padding{} = &[0; 3][..(4 - (length_so_far % 4)) % 4];",
                        pad_count,
                    );
                    outln!(
                        out,
                        "let length_so_far = length_so_far + padding{}.len();",
                        pad_count,
                    );
                    request_slices.push(format!("padding{}", pad_count));
                }
                assert_eq!(
                    length % 4,
                    0,
                    "Misaligned struct {}, length={}",
                    name,
                    length
                );
                let wire_length = length / 4;

                // Set the length in the request.
                // If it does not fit into u16, there's been a horrible mistake.
                assert!(
                    u16::try_from(wire_length).is_ok(),
                    "Fixed length too long for struct {}, wire_length {}",
                    name,
                    wire_length
                );
                outln!(
                    out,
                    "request0[2..4].copy_from_slice(&({wire_length}u16).to_ne_bytes());",
                );

                for slice in request_slices.iter().skip(1) {
                    outln!(out, "request0.extend_from_slice({slice}.as_ref());");
                }
                outln!(out, "request0");
            } else {
                outln!(out, "let length_so_far = 0;");

                let mut request_slices = Vec::new();
                let mut fixed_fields_bytes = Vec::new();
                let mut num_fixed_len_slices = 0;
                let mut pad_count = 0;

                for (field_i, field) in fields.iter().enumerate() {
                    let mut next_slice = None;

                    let mut tmp_out = Output::new();
                    serialize::emit_assert_for_field_serialize(
                        generator,
                        field,
                        deducible_fields,
                        |field_name| {
                            let rust_field_name = to_rust_variable_name(field_name);
                            if deducible_fields.contains_key(field_name) {
                                rust_field_name
                            } else {
                                format!("self.{}", rust_field_name)
                            }
                        },
                        &mut tmp_out,
                    );
                    match field {
                        xcbdefs::FieldDef::Pad(pad_field) => match pad_field.kind {
                            xcbdefs::PadKind::Bytes(bytes) => {
                                for _ in 0..bytes {
                                    fixed_fields_bytes.push(String::from("0"));
                                }
                            }
                            xcbdefs::PadKind::Align(align) => {
                                outln!(
                                    tmp_out,
                                    "let padding{} = &[0; {}][..({} - (length_so_far % {})) % {}];",
                                    pad_count,
                                    align - 1,
                                    align,
                                    align,
                                    align,
                                );
                                next_slice =
                                    Some((format!("padding{}", pad_count), IovecConversion::Into));
                                pad_count += 1;
                            }
                        },
                        xcbdefs::FieldDef::Normal(normal_field) => {
                            if normal_field.name == "major_opcode" {
                                if ns.ext_info.is_some() {
                                    fixed_fields_bytes.push(String::from("major_opcode"));
                                } else {
                                    fixed_fields_bytes.push(format!(
                                        "{}_REQUEST",
                                        super::super::camel_case_to_upper_snake(name),
                                    ));
                                }
                            } else if normal_field.name == "minor_opcode" {
                                assert!(ns.ext_info.is_some());
                                fixed_fields_bytes.push(format!(
                                    "{}_REQUEST",
                                    super::super::camel_case_to_upper_snake(name),
                                ));
                            } else if normal_field.name == "length" {
                                // the actual length will be calculated later
                                fixed_fields_bytes.push(String::from("0"));
                                fixed_fields_bytes.push(String::from("0"));
                            } else {
                                let rust_field_name = to_rust_variable_name(&normal_field.name);

                                let was_deduced = if let Some(deducible_field) =
                                    deducible_fields.get(&normal_field.name)
                                {
                                    generator.emit_calc_deducible_field(
                                        field,
                                        deducible_field,
                                        |field_name| {
                                            let rust_field_name = to_rust_variable_name(field_name);
                                            if deducible_fields.contains_key(field_name) {
                                                rust_field_name
                                            } else {
                                                format!("self.{}", rust_field_name)
                                            }
                                        },
                                        &rust_field_name,
                                        out,
                                    );
                                    true
                                } else {
                                    false
                                };

                                let bytes_name = super::postfix_var_name(&rust_field_name, "bytes");
                                if let Some(field_size) = normal_field.type_.size() {
                                    let field_name = if was_deduced {
                                        // If we deduced this value it comes from a local.
                                        rust_field_name
                                    } else {
                                        // Otherwise a member.
                                        format!("self.{}", rust_field_name)
                                    };

                                    outln!(
                                        out,
                                        "let {} = {};",
                                        bytes_name,
                                        serialize::emit_value_serialize(
                                            generator,
                                            &normal_field.type_,
                                            &field_name,
                                            was_deduced,
                                        ),
                                    );
                                    for i in 0..field_size {
                                        fixed_fields_bytes.push(format!("{}[{}]", bytes_name, i));
                                    }
                                } else {
                                    outln!(
                                        tmp_out,
                                        "let {} = self.{}.serialize();",
                                        bytes_name,
                                        rust_field_name,
                                    );
                                    next_slice = Some((bytes_name, IovecConversion::Into));
                                }
                            }
                        }
                        xcbdefs::FieldDef::List(list_field) => {
                            let rust_field_name = to_rust_variable_name(&list_field.name);
                            let list_length = list_field.length();
                            if rust_value_type_is_u8(&list_field.element_type) {
                                let conversion = if list_length.is_some() {
                                    // If this is a fixed length array we need to erase its length.
                                    IovecConversion::CowStripLength
                                } else {
                                    IovecConversion::None
                                };
                                next_slice =
                                    Some((format!("self.{}", rust_field_name), conversion));
                            } else {
                                assert_eq!(
                                    list_length, None,
                                    "fixed length arrays in requests must be [u8; N]"
                                );
                                let bytes_name = super::postfix_var_name(&rust_field_name, "bytes");
                                if parse::can_use_simple_list_parsing(&list_field.element_type) {
                                    outln!(
                                        tmp_out,
                                        "let {} = self.{}.serialize();",
                                        bytes_name,
                                        rust_field_name,
                                    );
                                } else {
                                    outln!(tmp_out, "let mut {} = Vec::new();", bytes_name);
                                    outln!(tmp_out, "for element in {}.iter() {{", rust_field_name);
                                    tmp_out.indented(|tmp_out| {
                                        serialize::emit_value_serialize_into(
                                            generator,
                                            &list_field.element_type,
                                            "element",
                                            false,
                                            &bytes_name,
                                            tmp_out,
                                        );
                                    });
                                    outln!(tmp_out, "}}");
                                }
                                next_slice = Some((bytes_name, IovecConversion::Into));
                            }
                        }
                        xcbdefs::FieldDef::Switch(switch_field) => {
                            let rust_field_name = to_rust_variable_name(&switch_field.name);
                            let bytes_name = super::postfix_var_name(&rust_field_name, "bytes");
                            outln!(
                                tmp_out,
                                "let {} = self.{}.serialize({});",
                                bytes_name,
                                rust_field_name,
                                ext_params_to_call_args(
                                    false,
                                    |name| {
                                        if deducible_fields.get(name).is_some() {
                                            to_rust_variable_name(name)
                                        } else {
                                            format!("self.{}", to_rust_variable_name(name))
                                        }
                                    },
                                    &*switch_field.external_params.borrow(),
                                )
                            );
                            if let Some(field_size) = switch_field.size() {
                                for i in 0..field_size {
                                    fixed_fields_bytes.push(format!("{}[{}]", bytes_name, i));
                                }
                            } else {
                                next_slice = Some((bytes_name, IovecConversion::Into));
                            }
                        }
                        xcbdefs::FieldDef::Fd(_)
                        | xcbdefs::FieldDef::FdList(_)
                        | xcbdefs::FieldDef::VirtualLen(_) => {}
                        xcbdefs::FieldDef::Expr(expr_field) => {
                            let rust_field_name = to_rust_variable_name(&expr_field.name);
                            let bytes_name = super::postfix_var_name(&rust_field_name, "bytes");
                            let type_ = generator.field_value_type_to_rust_type(&expr_field.type_);
                            if type_ == "bool" {
                                outln!(
                                    out,
                                    "let {} = {} != 0;",
                                    rust_field_name,
                                    expr_to_str(
                                        generator,
                                        &expr_field.expr,
                                        to_rust_variable_name,
                                        true,
                                        Some("u32"),
                                        true,
                                    ),
                                );
                            } else {
                                // the only case found in the XML definitions is with a bool
                                unreachable!();
                            }
                            let field_size = expr_field.type_.size().unwrap();
                            outln!(
                                out,
                                "let {} = {};",
                                bytes_name,
                                serialize::emit_value_serialize(
                                    generator,
                                    &expr_field.type_,
                                    &rust_field_name,
                                    false,
                                ),
                            );
                            for i in 0..field_size {
                                fixed_fields_bytes.push(format!("{}[{}]", bytes_name, i));
                            }
                        }
                    }

                    // The XML does not describe trailing padding in requests. Requests
                    // are implicitly padded to a four byte boundary.
                    if next_slice.is_none() && field_i == (fields.len() - 1) {
                        if let Some(ref mut request_size) = request_size {
                            let req_size_rem = *request_size % 4;
                            if req_size_rem != 0 {
                                let pad_size = 4 - req_size_rem;
                                for _ in 0..pad_size {
                                    fixed_fields_bytes.push(String::from("0"));
                                }
                                *request_size += pad_size;
                            }
                        }
                    }

                    if next_slice.is_some() || field_i == (fields.len() - 1) {
                        if !fixed_fields_bytes.is_empty() {
                            let maybe_mut = if num_fixed_len_slices == 0 {
                                // contains the length field, which will be modified
                                "mut "
                            } else {
                                ""
                            };
                            outln!(
                                out,
                                "let {}request{} = vec![",
                                maybe_mut,
                                num_fixed_len_slices,
                            );
                            for byte in &fixed_fields_bytes {
                                outln!(out.indent(), "{},", byte);
                            }
                            outln!(out, "];");
                            outln!(
                                out,
                                "let length_so_far = length_so_far + request{}.len();",
                                num_fixed_len_slices,
                            );
                            request_slices.push(format!("request{}.into()", num_fixed_len_slices));
                            fixed_fields_bytes.clear();
                            num_fixed_len_slices += 1;
                        }
                        if let Some((next_slice, conversion)) = next_slice {
                            outln!(tmp_out, "let referenced: &[u8] = {}.as_ref();", next_slice,);
                            outln!(
                                tmp_out,
                                "let length_so_far = length_so_far + referenced.len();",
                            );
                            match conversion {
                                IovecConversion::None => request_slices.push(next_slice),
                                IovecConversion::Into | IovecConversion::CowStripLength => {
                                    request_slices.push(next_slice.to_string());
                                }
                            }
                        }
                    }

                    out!(out, "{}", tmp_out.into_data());
                }
                // The XML does not describe trailing padding in requests. Requests
                // are implicitly padded to a four byte boundary.
                if let Some(request_size) = request_size {
                    let req_size_rem = request_size % 4;
                    if req_size_rem != 0 {
                        assert_eq!(pad_count, 0);
                        outln!(out, "let padding = vec![0; {}];", 4 - req_size_rem);
                        outln!(out, "let length_so_far = length_so_far + padding.len();");
                        request_slices.push(String::from("padding"));
                    }
                } else {
                    outln!(
                        out,
                        "let padding{} = &[0; 3][..(4 - (length_so_far % 4)) % 4];",
                        pad_count,
                    );
                    outln!(
                        out,
                        "let length_so_far = length_so_far + padding{}.len();",
                        pad_count,
                    );
                    request_slices.push(format!("padding{}", pad_count));
                }

                outln!(out, "debug_assert_eq!(0, length_so_far % 4);");
                // Set the length in the request.
                // If it does not fit into u16, compute_length_field will use BigRequests.
                outln!(
                    out,
                    "let length = u16::try_from(length_so_far / 4).unwrap_or(0);",
                );
                outln!(
                    out,
                    "request0[2..4].copy_from_slice(&length.to_ne_bytes());",
                );

                for slice in request_slices.iter().skip(1) {
                    outln!(out, "request0.extend_from_slice({slice}.as_ref());");
                }
                outln!(out, "request0");
            }
        });
        outln!(out, "}}");
    });
    outln!(out, "}}");
}

fn emit_request_function(
    request_def: &xcbdefs::RequestDef,
    name: &str,
    function_name: &str,
    gathered: &GatheredRequestFields,
    out: &mut Output,
) {
    let ns = request_def.namespace.upgrade().unwrap();
    let is_xproto = ns.header == "xproto";
    let is_list_fonts_with_info = request_def.name == "ListFontsWithInfo" && is_xproto;
    if is_list_fonts_with_info {
        return;
    }
    let is_send_event = request_def.name == "SendEvent" && is_xproto;
    let is_record_enable_context = request_def.name == "EnableContext" && ns.header == "record";

    let needs_lifetime = gathered.needs_lifetime && !is_send_event;

    let mut generic_params = String::new();
    if needs_lifetime {
        generic_params.push_str("'c, 'input");
    }
    for (ind, (param_name, _)) in gathered.generics.iter().enumerate() {
        if ind == 0 && generic_params.is_empty() {
        } else {
            generic_params.push_str(", ");
        }
        generic_params.push_str(param_name);
    }

    let ret_type = if is_list_fonts_with_info || is_record_enable_context {
        return;
    } else {
        let ret_type = if request_def.reply.is_some() {
            format!("Cookie<{}Reply>", name)
        } else {
            "VoidCookie".to_owned()
        };
        ret_type
    };

    let mut args = String::new();
    if needs_lifetime {
        args.push_str("conn: &'c mut SocketConnection");
    } else {
        args.push_str("conn: &mut SocketConnection");
    }
    for (arg_name, arg_type) in &gathered.args {
        args.push_str(", ");
        args.push_str(arg_name);
        args.push_str(": ");
        args.push_str(&arg_type.as_argument());
    }
    args.push_str(", forget: bool");

    if let Some(ref doc) = request_def.doc {
        emit_doc(doc, out, None);
    }
    if generic_params.is_empty() {
        outln!(
            out,
            "pub fn {}({}) -> Result<{}, ConnectionError>",
            function_name,
            args,
            ret_type,
        );
    } else {
        outln!(
            out,
            "pub fn {}<{}>({}) -> Result<{}, ConnectionError>",
            function_name,
            generic_params,
            args,
            ret_type,
        );
    }

    if !gathered.generics.is_empty() {
        outln!(out, "where");
        for (param_name, where_) in &gathered.generics {
            outln!(out.indent(), "{}: {},", param_name, where_);
        }
    }

    outln!(out, "{{");
    #[allow(clippy::cognitive_complexity)]
    out.indented(|out| {
        for preamble in &gathered.preamble {
            outln!(out, "{}", preamble);
        }

        let has_members = !gathered.request_args.is_empty();
        let members_start = if has_members { " {" } else { ";" };
        outln!(out, "let request0 = {}Request{}", name, members_start);
        out.indented(|out| {
            for (arg_name, arg_type) in &gathered.args {
                if arg_type.needs_any_cow() {
                    // Because the argument is passed in from outside this function,
                    // it is always ok to use a Cow::Borrowed here.
                    outln!(out, "{name}: Cow::Borrowed({name}),", name = arg_name);
                } else {
                    outln!(out, "{name},", name = arg_name);
                }
            }
        });
        if has_members {
            outln!(out, "}};");
        }

        outln!(
            out,
            "let bytes = request0.serialize({});",
            if is_xproto { "" } else { "major_opcode(conn)?" }
        );
        if request_def.reply.is_some() {
            outln!(out, "Ok(Cookie::new(conn.write(bytes.as_ref(), forget)?))",);
        } else {
            outln!(
                out,
                "Ok(VoidCookie::new(conn.write(bytes.as_ref(), forget)?))",
            );
        }
    });
    outln!(out, "}}");
}

fn emit_request_trait_function(
    request_def: &xcbdefs::RequestDef,
    name: &str,
    function_name: &str,
    gathered: &GatheredRequestFields,
    out: &mut Output,
) {
    let ns = request_def.namespace.upgrade().unwrap();
    let is_send_event = request_def.name == "SendEvent" && ns.header == "xproto";
    let needs_lifetime = gathered.needs_lifetime && !is_send_event;

    let mut generic_params = String::new();
    if needs_lifetime || !gathered.generics.is_empty() {
        generic_params.push('<');
        if needs_lifetime {
            generic_params.push_str("'c, 'input");
        }
        for (i, (param_name, _)) in gathered.generics.iter().enumerate() {
            if i != 0 || needs_lifetime {
                generic_params.push_str(", ");
            }
            generic_params.push_str(param_name);
        }
        generic_params.push('>');
    }

    let ret_lifetime = if needs_lifetime { "'c" } else { "'_" };
    let ret_type = if request_def.reply.is_some() {
        format!("VoidCookie<{}, Self>", ret_lifetime)
    } else {
        format!("Cookie<{}, Self, {}Reply>", ret_lifetime, name)
    };

    let mut args = String::new();
    if needs_lifetime {
        args.push_str("&'c self");
    } else {
        args.push_str("&self");
    }
    for (arg_name, arg_type) in &gathered.args {
        args.push_str(", ");
        args.push_str(arg_name);
        args.push_str(": ");
        args.push_str(&arg_type.as_argument());
    }

    let func_name_prefix = if ns.ext_info.is_some() {
        format!("{}_", ns.header)
    } else {
        String::new()
    };

    if let Some(ref doc) = request_def.doc {
        emit_doc(doc, out, Option::default());
    }
    outln!(
        out,
        "fn {}{}{}({}) -> Result<{}, ConnectionError>",
        func_name_prefix,
        function_name,
        generic_params,
        args,
        ret_type,
    );
    if !gathered.generics.is_empty() {
        outln!(out, "where");
        for (param_name, where_) in &gathered.generics {
            outln!(out.indent(), "{}: {},", param_name, where_);
        }
    }
    outln!(out, "{{");

    let mut call_args = String::from("self");
    for (arg_name, _) in &gathered.args {
        call_args.push_str(", ");
        call_args.push_str(arg_name);
    }

    let func_name_same_as_field_name = request_def
        .fields
        .borrow()
        .iter()
        .any(|field| field.name() == Some(function_name));
    if func_name_same_as_field_name {
        outln!(out.indent(), "self::{}({})", function_name, call_args);
    } else {
        outln!(out.indent(), "{}({})", function_name, call_args);
    }

    outln!(out, "}}");
}

/// Gathers information about the fields of a request,
/// returning a `GatheredRequestFields`.
fn gather_request_fields(
    generator: &NamespaceGenerator<'_, '_>,
    request_def: &xcbdefs::RequestDef,
    deducible_fields: &HashMap<String, DeducibleField>,
) -> Option<GatheredRequestFields> {
    let mut letter_iter = b'A'..=b'Z';

    let mut needs_lifetime = false;
    let mut args = Vec::new();
    let mut request_args = Vec::new();
    let mut generics = Vec::new();
    let mut preamble = Vec::new();
    let mut single_fds = Vec::new();
    let mut fd_lists = Vec::new();

    let ns = request_def.namespace.upgrade().unwrap();
    let is_change_property = request_def.name == "ChangeProperty" && ns.header == "xproto";
    let is_get_property = request_def.name == "GetProperty" && ns.header == "xproto";
    let is_send_event = request_def.name == "SendEvent" && ns.header == "xproto";

    for field in request_def.fields.borrow().iter() {
        if !field_is_visible(field, deducible_fields) {
            continue;
        }
        if let Some("major_opcode" | "minor_opcode" | "length") = field.name() {
            continue;
        }

        match field {
            xcbdefs::FieldDef::Normal(normal_field) => {
                let rust_field_name = to_rust_variable_name(&normal_field.name);
                let rust_field_type = generator.field_value_type_to_rust_type(&normal_field.type_);
                let use_into = if ((is_change_property || is_get_property)
                    && normal_field.name == "property")
                    || (is_change_property && normal_field.name == "type")
                {
                    true
                } else if crate::generator::namespace::use_enum_type_in_field(&normal_field.type_)
                    .is_none()
                {
                    !matches!(normal_field.type_.value_set, xcbdefs::FieldValueSet::None)
                } else {
                    false
                };

                if use_into {
                    let preamble_part = format!(
                        "let {}: {} = {}.into();",
                        rust_field_name, rust_field_type, rust_field_name,
                    );
                    let generic_param = format!("{}", char::from(letter_iter.next().unwrap()));
                    let where_ = format!("Into<{}>", rust_field_type);
                    args.push((rust_field_name.clone(), Type::Simple(generic_param.clone())));
                    request_args.push((rust_field_name, Type::Simple(rust_field_type)));
                    generics.push((generic_param, where_));
                    preamble.push(preamble_part);
                } else {
                    args.push((
                        rust_field_name.clone(),
                        Type::Simple(rust_field_type.clone()),
                    ));
                    request_args.push((rust_field_name, Type::Simple(rust_field_type)));
                }
            }
            xcbdefs::FieldDef::List(list_field) => {
                if is_send_event && list_field.name == "event" {
                    let generic_param = format!("{}", char::from(letter_iter.next().unwrap()));
                    args.push((list_field.name.clone(), Type::Simple(generic_param.clone())));
                    request_args.push((
                        list_field.name.clone(),
                        Type::VariableOwnershipRawBytes("[u8; 32]".into()),
                    ));
                    generics.push((generic_param, String::from("Into<[u8; 32]>")));
                    preamble.push(String::from("let event = Cow::Owned(event.into());"));
                } else {
                    let element_type =
                        generator.field_value_type_to_rust_type(&list_field.element_type);
                    let rust_field_name = to_rust_variable_name(&list_field.name);
                    let rust_field_type = if rust_value_type_is_u8(&list_field.element_type) {
                        if let Some(list_len) = list_field.length() {
                            Type::VariableOwnershipRawBytes(format!(
                                "[{}; {}]",
                                element_type, list_len
                            ))
                        } else {
                            Type::VariableOwnershipRawBytes(format!("[{}]", element_type))
                        }
                    } else {
                        assert_eq!(
                            list_field.length(),
                            None,
                            "Fixed length arrays of types other than u8 are not implemented"
                        );

                        Type::VariableOwnership(format!("[{}]", element_type))
                    };
                    args.push((rust_field_name.clone(), rust_field_type.clone()));
                    request_args.push((rust_field_name, rust_field_type));
                }
                needs_lifetime = true;
            }
            xcbdefs::FieldDef::Switch(switch_field) => {
                let rust_field_name = to_rust_variable_name(&switch_field.name);
                let rust_field_type =
                    Type::VariableOwnership(format!("{}Aux", to_rust_type_name(&request_def.name)));
                args.push((rust_field_name.clone(), rust_field_type.clone()));
                request_args.push((rust_field_name, rust_field_type));
                needs_lifetime = true;
            }
            xcbdefs::FieldDef::Fd(fd_field) => {
                let rust_field_name = to_rust_variable_name(&fd_field.name);
                let generic_param = format!("{}", char::from(letter_iter.next().unwrap()));
                let preamble_part = format!(
                    "let {}: RawFdContainer = {}.into();",
                    rust_field_name, rust_field_name,
                );
                args.push((rust_field_name.clone(), Type::Simple(generic_param.clone())));
                request_args.push((rust_field_name, Type::Simple("RawFdContainer".into())));
                generics.push((generic_param, "Into<RawFdContainer>".into()));
                preamble.push(preamble_part);
                single_fds.push(fd_field.name.clone());
            }
            xcbdefs::FieldDef::FdList(fd_list_field) => {
                let rust_field_name = to_rust_variable_name(&fd_list_field.name);
                args.push((
                    rust_field_name.clone(),
                    Type::Simple("Vec<RawFdContainer>".into()),
                ));
                request_args.push((rust_field_name, Type::Simple("Vec<RawFdContainer>".into())));
                fd_lists.push(fd_list_field.name.clone());
            }
            xcbdefs::FieldDef::Expr(_)
            | xcbdefs::FieldDef::Pad(_)
            | xcbdefs::FieldDef::VirtualLen(_) => unreachable!(),
        }
    }

    let reply_has_fds = request_def.reply.as_ref().map_or(false, |reply_def| {
        reply_def.fields.borrow().iter().any(|field| {
            matches!(
                field,
                xcbdefs::FieldDef::Fd(_) | xcbdefs::FieldDef::FdList(_)
            )
        })
    });

    assert_eq!(args.len(), request_args.len());
    if reply_has_fds || !single_fds.is_empty() || !fd_lists.is_empty() {
        None
    } else {
        Some(GatheredRequestFields {
            needs_lifetime,
            args,
            request_args,
            generics,
            preamble,
        })
    }
}
