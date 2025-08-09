use syn::{
    Expr, ExprCall, ExprPath, ExprStruct, Member, Path, Type,
    visit_mut::{
        VisitMut, visit_expr_call_mut, visit_expr_mut, visit_expr_struct_mut, visit_type_mut,
    },
};

use crate::utilities::squote::parse_squote;

pub struct ReplaceTypeInfer(pub Type);

impl VisitMut for ReplaceTypeInfer {
    fn visit_type_mut(&mut self, ty: &mut Type) {
        let Type::Infer(_) = ty else {
            visit_type_mut(self, ty);
            return;
        };

        *ty = self.0.clone();
    }
}

pub struct ReplaceExprInfer(pub Expr);

impl VisitMut for ReplaceExprInfer {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        let Expr::Infer(_) = expr else {
            visit_expr_mut(self, expr);
            return;
        };

        *expr = self.0.clone();
    }
}

pub struct AddFieldInStructConstruction<'a> {
    pub path: &'a Path,
    pub field_member: Member,
    pub field_expr: Expr,
}

impl AddFieldInStructConstruction<'_> {
    fn should_modify(&self, other: &Path) -> bool {
        self.path
            .segments
            .iter()
            .map(|seg| &seg.ident)
            .eq(other.segments.iter().map(|seg| &seg.ident))
    }
}

impl VisitMut for AddFieldInStructConstruction<'_> {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        // Constructing a unit struct is considered a path expression. Since the
        // expression variant must be changed, capture it here.
        let Expr::Path(expr_path) = expr else {
            visit_expr_mut(self, expr);
            return;
        };

        // Check that the path of the struct being constructed is the impl type path.
        if !self.should_modify(&expr_path.path) {
            visit_expr_mut(self, expr);
            return;
        }

        *expr = parse_squote!(#expr_path(#{self.field_expr}));
    }

    // Constructing a tuple struct is considered a call expression.
    fn visit_expr_call_mut(&mut self, expr_call: &mut ExprCall) {
        let ExprCall { func, args, .. } = expr_call;

        let Expr::Path(ExprPath { path, .. }) = func.as_ref() else {
            visit_expr_call_mut(self, expr_call);
            return;
        };

        // Check that the path of the struct being constructed is the impl type path.
        if !self.should_modify(path) {
            visit_expr_call_mut(self, expr_call);
            return;
        }

        // Add an argument to the tuple struct construction.
        args.push(self.field_expr.clone());
    }

    fn visit_expr_struct_mut(&mut self, expr_struct: &mut ExprStruct) {
        let ExprStruct { path, fields, .. } = expr_struct;

        // Check that the path of the struct being constructed is the impl type path.
        if !self.should_modify(path) {
            visit_expr_struct_mut(self, expr_struct);
            return;
        }

        fields.push(parse_squote!(#{self.field_member}: #{self.field_expr}));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_type_infer_single_unnested() {
        let mut ty = parse_squote!(_);

        ReplaceTypeInfer(parse_squote!(ReplacementType)).visit_type_mut(&mut ty);

        assert_eq!(ty, parse_squote!(ReplacementType));
    }

    #[test]
    fn replace_type_infer_single_nested() {
        let mut ty = parse_squote!(Wrapper<_>);

        ReplaceTypeInfer(parse_squote!(ReplacementType)).visit_type_mut(&mut ty);

        assert_eq!(ty, parse_squote!(Wrapper<ReplacementType>));
    }

    #[test]
    fn replace_type_infer_multiple_nested() {
        let mut ty = parse_squote!(Wrapper<_, (_, i64), InnerWrapper<_, String>>);

        ReplaceTypeInfer(parse_squote!(ReplacementType)).visit_type_mut(&mut ty);

        assert_eq!(
            ty,
            parse_squote!(Wrapper<ReplacementType, (ReplacementType, i64), InnerWrapper<ReplacementType, String>>)
        );
    }

    #[test]
    fn replace_expr_infer_single_unnested() {
        let mut block = parse_squote! {{
            _
        }};

        ReplaceExprInfer(parse_squote!(replacement_expr())).visit_block_mut(&mut block);

        assert_eq!(block, parse_squote! {{ replacement_expr() }});
    }

    #[test]
    fn replace_expr_infer_single_nested() {
        let mut block = parse_squote! {{
            Ok(_)
        }};

        ReplaceExprInfer(parse_squote!(replacement_expr())).visit_block_mut(&mut block);

        assert_eq!(
            block,
            parse_squote! {{
                Ok(replacement_expr())
            }}
        );
    }

    #[test]
    fn replace_expr_infer_multiple_unnested() {
        let mut block = parse_squote! {{
            if true {
                return _;
            }

            _
        }};

        ReplaceExprInfer(parse_squote!(replacement_expr())).visit_block_mut(&mut block);

        assert_eq!(
            block,
            parse_squote! {{
                if true {
                    return replacement_expr();
                }

                replacement_expr()
            }}
        );
    }

    #[test]
    fn replace_expr_infer_multiple_nested() {
        let mut block = parse_squote! {{
            if true {
                return Ok(_);
            }

            Ok(_)
        }};

        ReplaceExprInfer(parse_squote!(replacement_expr())).visit_block_mut(&mut block);

        assert_eq!(
            block,
            parse_squote! {{
                if true {
                    return Ok(replacement_expr());
                }

                Ok(replacement_expr())
            }}
        );
    }

    #[test]
    fn replace_expr_infer_unrelated_infers() {
        let mut block = parse_squote! {{
            let _ = 5;
            let v: Vec<_> = vec![];
            match v {
                _ => {},
            }

            return Ok(_);
        }};

        ReplaceExprInfer(parse_squote!(replacement_expr())).visit_block_mut(&mut block);

        assert_eq!(
            block,
            parse_squote! {{
                let _ = 5;
                let v: Vec<_> = vec![];
                match v {
                    _ => {},
                }

                return Ok(replacement_expr());
            }}
        );
    }

    #[test]
    fn add_field_in_struct_construction_unit_single_segment() {
        let mut block = parse_squote! {{
            Struct
        }};

        AddFieldInStructConstruction {
            path: &parse_squote!(Struct),
            field_member: parse_squote!(added_field_member),
            field_expr: parse_squote!(added_field_expr()),
        }
        .visit_block_mut(&mut block);

        assert_eq!(
            block,
            parse_squote! {{
                Struct(added_field_expr())
            }}
        );
    }

    #[test]
    fn add_field_in_struct_construction_unit_multiple_segments() {
        let mut block = parse_squote! {{
            a::b::Struct
        }};

        AddFieldInStructConstruction {
            path: &parse_squote!(a::b::Struct),
            field_member: parse_squote!(added_field_member),
            field_expr: parse_squote!(added_field_expr()),
        }
        .visit_block_mut(&mut block);

        assert_eq!(
            block,
            parse_squote! {{
                a::b::Struct(added_field_expr())
            }}
        );
    }

    #[test]
    fn add_field_in_struct_construction_unnamed_empty_single_segment() {
        let mut block = parse_squote! {{
            Struct()
        }};

        AddFieldInStructConstruction {
            path: &parse_squote!(Struct),
            field_member: parse_squote!(added_field_member),
            field_expr: parse_squote!(added_field_expr()),
        }
        .visit_block_mut(&mut block);

        assert_eq!(
            block,
            parse_squote! {{
                Struct(added_field_expr())
            }}
        );
    }

    #[test]
    fn add_field_in_struct_construction_unnamed_empty_multiple_segments() {
        let mut block = parse_squote! {{
            a::b::Struct()
        }};

        AddFieldInStructConstruction {
            path: &parse_squote!(a::b::Struct),
            field_member: parse_squote!(added_field_member),
            field_expr: parse_squote!(added_field_expr()),
        }
        .visit_block_mut(&mut block);

        assert_eq!(
            block,
            parse_squote! {{
                a::b::Struct(added_field_expr())
            }}
        );
    }

    #[test]
    fn add_field_in_struct_construction_unnamed_single_segment() {
        let mut block = parse_squote! {{
            Struct(some_variable, SomeExpr::new())
        }};

        AddFieldInStructConstruction {
            path: &parse_squote!(Struct),
            field_member: parse_squote!(added_field_member),
            field_expr: parse_squote!(added_field_expr()),
        }
        .visit_block_mut(&mut block);

        assert_eq!(
            block,
            parse_squote! {{
                Struct(some_variable, SomeExpr::new(), added_field_expr())
            }}
        );
    }

    #[test]
    fn add_field_in_struct_construction_unnamed_multiple_segments() {
        let mut block = parse_squote! {{
            a::b::Struct(some_variable, SomeExpr::new())
        }};

        AddFieldInStructConstruction {
            path: &parse_squote!(a::b::Struct),
            field_member: parse_squote!(added_field_member),
            field_expr: parse_squote!(added_field_expr()),
        }
        .visit_block_mut(&mut block);

        assert_eq!(
            block,
            parse_squote! {{
                a::b::Struct(some_variable, SomeExpr::new(), added_field_expr())
            }}
        );
    }

    #[test]
    fn add_field_in_struct_construction_named_single_segment() {
        let mut block = parse_squote! {{
            Struct {
                some_variable,
                some_variable2: SomeExpr::new(),
            }
        }};

        AddFieldInStructConstruction {
            path: &parse_squote!(Struct),
            field_member: parse_squote!(added_field_member),
            field_expr: parse_squote!(added_field_expr()),
        }
        .visit_block_mut(&mut block);

        assert_eq!(
            block,
            parse_squote! {{
                Struct {
                    some_variable,
                    some_variable2: SomeExpr::new(),
                    added_field_member: added_field_expr()
                }
            }}
        );
    }

    #[test]
    fn add_field_in_struct_construction_named_multiple_segments() {
        let mut block = parse_squote! {{
            a::b::Struct {
                some_variable,
                some_variable2: SomeExpr::new(),
            }
        }};

        AddFieldInStructConstruction {
            path: &parse_squote!(a::b::Struct),
            field_member: parse_squote!(added_field_member),
            field_expr: parse_squote!(added_field_expr()),
        }
        .visit_block_mut(&mut block);

        assert_eq!(
            block,
            parse_squote! {{
                a::b::Struct {
                    some_variable,
                    some_variable2: SomeExpr::new(),
                    added_field_member: added_field_expr()
                }
            }}
        );
    }

    #[test]
    fn add_field_in_struct_construction_generics() {
        let mut block = parse_squote! {{
            a::b::Struct {
                some_variable,
                some_variable2: T::default(),
            }
        }};

        AddFieldInStructConstruction {
            path: &parse_squote!(a::b::Struct<T>),
            field_member: parse_squote!(added_field_member),
            field_expr: parse_squote!(added_field_expr()),
        }
        .visit_block_mut(&mut block);

        assert_eq!(
            block,
            parse_squote! {{
                a::b::Struct {
                    some_variable,
                    some_variable2: T::default(),
                    added_field_member: added_field_expr()
                }
            }}
        );
    }
}
