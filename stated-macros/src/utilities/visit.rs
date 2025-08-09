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
    pub field_type: Type,
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

        *expr = parse_squote!(#expr_path(#{self.field_type}));
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
        args.push(parse_squote!(#{self.field_type}));
    }

    fn visit_expr_struct_mut(&mut self, expr_struct: &mut ExprStruct) {
        let ExprStruct { path, fields, .. } = expr_struct;

        // Check that the path of the struct being constructed is the impl type path.
        if !self.should_modify(path) {
            visit_expr_struct_mut(self, expr_struct);
            return;
        }

        fields.push(parse_squote!(#{self.field_member}: #{self.field_type}));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_type_infer_single_unnested() {
        let mut ty = parse_squote!(_);

        ReplaceTypeInfer(parse_squote!(Replacement)).visit_type_mut(&mut ty);

        assert_eq!(ty, parse_squote!(Replacement));
    }

    #[test]
    fn replace_type_infer_single_nested() {
        let mut ty = parse_squote!(Wrapper<_>);

        ReplaceTypeInfer(parse_squote!(Replacement)).visit_type_mut(&mut ty);

        assert_eq!(ty, parse_squote!(Wrapper<Replacement>));
    }

    #[test]
    fn replace_type_infer_multiple_nested() {
        let mut ty = parse_squote!(Wrapper<_, (_, i64), InnerWrapper<_, String>>);

        ReplaceTypeInfer(parse_squote!(Replacement)).visit_type_mut(&mut ty);

        assert_eq!(
            ty,
            parse_squote!(Wrapper<Replacement, (Replacement, i64), InnerWrapper<Replacement, String>>)
        );
    }

    #[test]
    fn replace_expr_infer_last_single_unnested() {
        let mut block = parse_squote! {{
            _
        }};

        ReplaceExprInfer(parse_squote!(Replacement)).visit_block_mut(&mut block);

        assert_eq!(block, parse_squote! {{ Replacement }});
    }

    #[test]
    fn replace_expr_infer_last_single_nested() {
        let mut block = parse_squote! {{
            Ok(_)
        }};

        ReplaceExprInfer(parse_squote!(Replacement)).visit_block_mut(&mut block);

        assert_eq!(
            block,
            parse_squote! {{
                Ok(Replacement)
            }}
        );
    }

    #[test]
    fn replace_expr_infer_single_unnested() {
        let mut block = parse_squote! {{
            let replacement = _;
            replacement
        }};

        ReplaceExprInfer(parse_squote!(Replacement)).visit_block_mut(&mut block);

        assert_eq!(
            block,
            parse_squote! {{
               let replacement = Replacement;
               replacement
            }}
        );
    }

    #[test]
    fn replace_expr_infer_single_nested() {
        let mut block = parse_squote! {{
            let replacement = Ok(_);
            replacement
        }};

        ReplaceExprInfer(parse_squote!(Replacement)).visit_block_mut(&mut block);

        assert_eq!(
            block,
            parse_squote! {{
               let replacement = Ok(Replacement);
               replacement
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

        ReplaceExprInfer(parse_squote!(Replacement)).visit_block_mut(&mut block);

        assert_eq!(
            block,
            parse_squote! {{
                let _ = 5;
                let v: Vec<_> = vec![];
                match v {
                    _ => {},
                }

                return Ok(Replacement);
            }}
        );
    }
}
