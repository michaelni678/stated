use syn::{
    Expr, ExprCall, ExprPath, ExprStruct, Path, Type,
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

pub struct ModifyStructConstructionInBlock<'a>(pub &'a Path);

impl ModifyStructConstructionInBlock<'_> {
    pub fn should_modify(&self, other: &Path) -> bool {
        let other_idents = other.segments.iter().map(|seg| &seg.ident);

        self.0
            .segments
            .iter()
            .map(|seg| &seg.ident)
            .eq(other_idents)
    }
}

impl VisitMut for ModifyStructConstructionInBlock<'_> {
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

        *expr = parse_squote!(#expr_path(::std::marker::PhantomData));
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
        args.push(parse_squote!(::std::marker::PhantomData));
    }

    fn visit_expr_struct_mut(&mut self, expr_struct: &mut ExprStruct) {
        let ExprStruct { path, fields, .. } = expr_struct;

        // Check that the path of the struct being constructed is the impl type path.
        if !self.should_modify(path) {
            visit_expr_struct_mut(self, expr_struct);
            return;
        }

        fields.push(parse_squote!(__states: ::std::marker::PhantomData));
    }
}
