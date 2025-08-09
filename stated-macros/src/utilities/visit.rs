use syn::{
    Expr, ExprCall, ExprPath, ExprStruct, Member, Path, Type,
    visit_mut::{
        VisitMut, visit_expr_call_mut, visit_expr_mut, visit_expr_struct_mut, visit_type_mut,
    },
};

use crate::utilities::squote::parse_squote;

pub struct ReplaceInferInReturnType(pub Type);

impl VisitMut for ReplaceInferInReturnType {
    fn visit_type_mut(&mut self, ty: &mut Type) {
        let Type::Infer(_) = ty else {
            visit_type_mut(self, ty);
            return;
        };

        *ty = self.0.clone();
    }
}

pub struct ReplaceInferInBlock(pub Expr);

impl VisitMut for ReplaceInferInBlock {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        let Expr::Infer(_) = expr else {
            visit_expr_mut(self, expr);
            return;
        };

        *expr = self.0.clone();
    }
}

pub struct AddFieldInStructConstructionInBlock<'a> {
    pub path: &'a Path,
    pub field_member: Member,
    pub field_type: Type,
}

impl AddFieldInStructConstructionInBlock<'_> {
    fn should_modify(&self, other: &Path) -> bool {
        self.path
            .segments
            .iter()
            .map(|seg| &seg.ident)
            .eq(other.segments.iter().map(|seg| &seg.ident))
    }
}

impl VisitMut for AddFieldInStructConstructionInBlock<'_> {
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
    fn replace_infer_in_return_type_unnested() {
        let mut return_type = parse_squote!(-> _);

        ReplaceInferInReturnType(parse_squote!(Dummy)).visit_return_type_mut(&mut return_type);

        assert_eq!(return_type, parse_squote!(-> Dummy));
    }

    #[test]
    fn replace_infer_in_return_type_nested() {
        let mut return_type = parse_squote!(-> Returned<_>);

        ReplaceInferInReturnType(parse_squote!(Dummy)).visit_return_type_mut(&mut return_type);

        assert_eq!(return_type, parse_squote!(-> Returned<Dummy>));
    }

    #[test]
    fn replace_infer_in_return_type_nested_multiple() {
        let mut return_type = parse_squote!(-> Returned<_, (_, i64), Nest<_, String>>);

        ReplaceInferInReturnType(parse_squote!(Dummy)).visit_return_type_mut(&mut return_type);

        assert_eq!(
            return_type,
            parse_squote!(-> Returned<Dummy, (Dummy, i64), Nest<Dummy, String>>)
        );
    }

    #[test]
    fn replace_infer_in_block_end_unnested() {
        let mut block = parse_squote! {{
            _
        }};

        ReplaceInferInBlock(parse_squote!(Dummy)).visit_block_mut(&mut block);

        assert_eq!(block, parse_squote! {{ Dummy }});
    }

    #[test]
    fn replace_infer_in_block_end_nested() {
        let mut block = parse_squote! {{
            Ok(_)
        }};

        ReplaceInferInBlock(parse_squote!(Dummy)).visit_block_mut(&mut block);

        assert_eq!(
            block,
            parse_squote! {{
                Ok(Dummy)
            }}
        );
    }

    #[test]
    fn replace_infer_in_block_not_end_unnested() {
        let mut block = parse_squote! {{
            let dummy = _;
            dummy
        }};

        ReplaceInferInBlock(parse_squote!(Dummy)).visit_block_mut(&mut block);

        assert_eq!(
            block,
            parse_squote! {{
               let dummy = Dummy;
               dummy
            }}
        );
    }

    #[test]
    fn replace_infer_in_block_not_end_nested() {
        let mut block = parse_squote! {{
            let dummy = Ok(_);
            dummy
        }};

        ReplaceInferInBlock(parse_squote!(Dummy)).visit_block_mut(&mut block);

        assert_eq!(
            block,
            parse_squote! {{
               let dummy = Ok(Dummy);
               dummy
            }}
        );
    }

    #[test]
    fn replace_infer_in_block_unrelated_infers() {
        let mut block = parse_squote! {{
            let _ = 5;
            let v: Vec<_> = vec![];
            match v {
                _ => {},
            }

            return Ok(_);
        }};

        ReplaceInferInBlock(parse_squote!(Dummy)).visit_block_mut(&mut block);

        assert_eq!(
            block,
            parse_squote! {{
                let _ = 5;
                let v: Vec<_> = vec![];
                match v {
                    _ => {},
                }

                return Ok(Dummy);
            }}
        );
    }
}
