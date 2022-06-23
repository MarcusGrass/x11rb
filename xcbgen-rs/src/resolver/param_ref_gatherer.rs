use crate::{defs, ResolveError};

/// Gather elements to param references in the module.
pub(super) fn gather(module: &defs::Module) -> Result<(), ResolveError> {
    for ns in module.namespaces.borrow().values() {
        for type_def in ns.type_defs.borrow().values() {
            if let defs::TypeDef::Struct(struct_def) = type_def {
                StructParamRefGatherer::gather_param_refs_in_struct(struct_def)?;
            }
        }
    }
    Ok(())
}

struct StructParamRefGatherer {
    external_params: Vec<defs::ExternalParam>,
}

impl StructParamRefGatherer {
    fn add_external_param(&mut self, name: &str, type_: defs::TypeRef) -> Result<(), ResolveError> {
        for ext_param in &self.external_params {
            if ext_param.name == name {
                return if ext_param.type_.same_as(&type_) {
                    Ok(())
                } else {
                    Err(ResolveError::DiscrepantParamRefTypes(name.into()))
                };
            }
        }
        self.external_params.push(defs::ExternalParam {
            name: name.into(),
            type_,
        });
        Ok(())
    }

    fn gather_param_refs_in_struct(struct_def: &defs::StructDef) -> Result<(), ResolveError> {
        let mut gatherer = Self {
            external_params: Vec::new(),
        };

        for field in struct_def.fields.borrow().iter() {
            gatherer.gather_param_refs_in_field(field)?;
        }

        struct_def.external_params.replace(gatherer.external_params);

        Ok(())
    }

    fn gather_param_refs_in_field(&mut self, field: &defs::FieldDef) -> Result<(), ResolveError> {
        match field {
            defs::FieldDef::List(list_field) => {
                if let Some(ref length_expr) = list_field.length_expr {
                    self.gather_param_refs_in_expr(length_expr)?;
                }
                Ok(())
            }
            defs::FieldDef::Switch(switch_field) => {
                self.gather_param_refs_in_expr(&switch_field.expr)?;
                for case in &switch_field.cases {
                    for case_expr in &case.exprs {
                        self.gather_param_refs_in_expr(case_expr)?;
                    }
                    for case_field in case.fields.borrow().iter() {
                        self.gather_param_refs_in_field(case_field)?;
                    }
                }
                Ok(())
            }
            defs::FieldDef::Fd(_)
            | defs::FieldDef::Normal(_)
            | defs::FieldDef::Pad(_)
            | defs::FieldDef::VirtualLen(_) => Ok(()),
            defs::FieldDef::FdList(fd_list_field) => {
                self.gather_param_refs_in_expr(&fd_list_field.length_expr)?;
                Ok(())
            }
            defs::FieldDef::Expr(expr_field) => {
                self.gather_param_refs_in_expr(&expr_field.expr)?;
                Ok(())
            }
        }
    }

    fn gather_param_refs_in_expr(&mut self, expr: &defs::Expression) -> Result<(), ResolveError> {
        match expr {
            defs::Expression::BinaryOp(bin_op_expr) => {
                self.gather_param_refs_in_expr(&bin_op_expr.lhs)?;
                self.gather_param_refs_in_expr(&bin_op_expr.rhs)?;
                Ok(())
            }
            defs::Expression::UnaryOp(unary_op_expr) => {
                self.gather_param_refs_in_expr(&unary_op_expr.rhs)?;
                Ok(())
            }
            defs::Expression::FieldRef(_)
            | defs::Expression::Value(_)
            | defs::Expression::EnumRef(_)
            | defs::Expression::ListElementRef
            | defs::Expression::Bit(_) => Ok(()),
            defs::Expression::ParamRef(param_ref_expr) => {
                self.add_external_param(
                    &param_ref_expr.field_name,
                    param_ref_expr.type_.get_resolved().clone(),
                )?;
                Ok(())
            }
            defs::Expression::PopCount(expr) => {
                self.gather_param_refs_in_expr(expr)?;
                Ok(())
            }
            defs::Expression::SumOf(sum_of_expr) => {
                self.gather_param_refs_in_expr(&sum_of_expr.operand)?;
                Ok(())
            }
        }
    }
}
