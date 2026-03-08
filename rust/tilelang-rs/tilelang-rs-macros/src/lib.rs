use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{
    parse_macro_input, Arm, BinOp, ExprAssign, ExprBinary, ExprClosure, ExprForLoop, ExprLet,
    ExprReference, FnArg, ItemFn, Local, Pat, PatIdent, PatReference, TypeReference,
};

#[proc_macro_attribute]
pub fn tl_ir(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let mut checker = Precheck::default();
    checker.visit_item_fn(&input);

    if let Some(error) = checker.into_error() {
        return TokenStream::from(error.to_compile_error());
    }

    TokenStream::from(quote!(#input))
}

#[derive(Default)]
struct Precheck {
    errors: Vec<syn::Error>,
}

impl Precheck {
    fn into_error(self) -> Option<syn::Error> {
        let mut errors = self.errors.into_iter();
        let mut first = errors.next()?;
        for error in errors {
            first.combine(error);
        }
        Some(first)
    }

    fn push_error(&mut self, span: proc_macro2::Span, code: &str, message: &str, help: &str) {
        self.errors.push(syn::Error::new(
            span,
            format!("{code}: {message}\nhelp: {help}"),
        ));
    }
}

impl<'ast> Visit<'ast> for Precheck {
    fn visit_fn_arg(&mut self, node: &'ast FnArg) {
        if let FnArg::Typed(pat_type) = node {
            if pattern_has_mut(&pat_type.pat) {
                self.push_error(
                    pat_type.pat.span(),
                    "TLRS002",
                    "#[tl_ir] 中禁止 mut 参数与 mut 模式绑定。",
                    "使用不可变绑定 + 遮蔽，或改用 alloc_var。",
                );
            }
            self.visit_type(&pat_type.ty);
        }
    }

    fn visit_local(&mut self, node: &'ast Local) {
        if is_simple_let_mut(&node.pat) {
            self.push_error(
                node.pat.span(),
                "TLRS001",
                "#[tl_ir] 中禁止 let mut。",
                "改用 alloc_var/load/store 或 select，避免 host 可变状态。",
            );
        } else if pattern_has_mut(&node.pat) {
            self.push_error(
                node.pat.span(),
                "TLRS002",
                "#[tl_ir] 中禁止 mut 参数与 mut 模式绑定。",
                "使用不可变绑定 + 遮蔽，或改用 alloc_var。",
            );
        }

        if let Some(init) = &node.init {
            self.visit_expr(&init.expr);
            if let Some((_, diverge)) = &init.diverge {
                self.visit_expr(diverge);
            }
        }
    }

    fn visit_expr_let(&mut self, node: &'ast ExprLet) {
        if pattern_has_mut(&node.pat) {
            self.push_error(
                node.pat.span(),
                "TLRS002",
                "#[tl_ir] 中禁止 mut 参数与 mut 模式绑定。",
                "使用不可变绑定 + 遮蔽，或改用 alloc_var。",
            );
        }
        self.visit_expr(&node.expr);
    }

    fn visit_arm(&mut self, node: &'ast Arm) {
        if pattern_has_mut(&node.pat) {
            self.push_error(
                node.pat.span(),
                "TLRS002",
                "#[tl_ir] 中禁止 mut 参数与 mut 模式绑定。",
                "使用不可变绑定 + 遮蔽，或改用 alloc_var。",
            );
        }

        if let Some((_, guard)) = &node.guard {
            self.visit_expr(guard);
        }
        self.visit_expr(&node.body);
    }

    fn visit_expr_for_loop(&mut self, node: &'ast ExprForLoop) {
        if pattern_has_mut(&node.pat) {
            self.push_error(
                node.pat.span(),
                "TLRS002",
                "#[tl_ir] 中禁止 mut 参数与 mut 模式绑定。",
                "使用不可变绑定 + 遮蔽，或改用 alloc_var。",
            );
        }
        self.visit_expr(&node.expr);
        self.visit_block(&node.body);
    }

    fn visit_expr_closure(&mut self, node: &'ast ExprClosure) {
        for input in &node.inputs {
            if pattern_has_mut(input) {
                self.push_error(
                    input.span(),
                    "TLRS002",
                    "#[tl_ir] 中禁止 mut 参数与 mut 模式绑定。",
                    "使用不可变绑定 + 遮蔽，或改用 alloc_var。",
                );
            }
        }
        self.visit_expr(&node.body);
    }

    fn visit_expr_reference(&mut self, node: &'ast ExprReference) {
        if node.mutability.is_some() {
            self.push_error(
                node.span(),
                "TLRS003",
                "#[tl_ir] 中禁止显式 &mut 可变借用。",
                "使用 DSL 状态容器，不要依赖 host 可变引用。",
            );
        }
        visit::visit_expr_reference(self, node);
    }

    fn visit_type_reference(&mut self, node: &'ast TypeReference) {
        if node.mutability.is_some() {
            self.push_error(
                node.span(),
                "TLRS003",
                "#[tl_ir] 中禁止显式 &mut 可变借用。",
                "使用 DSL 状态容器，不要依赖 host 可变引用。",
            );
        }
        visit::visit_type_reference(self, node);
    }

    fn visit_expr_assign(&mut self, node: &'ast ExprAssign) {
        self.push_error(
            node.span(),
            "TLRS004",
            "#[tl_ir] 中禁止 host 赋值/复合赋值。",
            "改用 alloc_var/load/store 或 select 表达分支结果。",
        );
        visit::visit_expr_assign(self, node);
    }

    fn visit_expr_binary(&mut self, node: &'ast ExprBinary) {
        if matches!(
            node.op,
            BinOp::AddAssign(_)
                | BinOp::SubAssign(_)
                | BinOp::MulAssign(_)
                | BinOp::DivAssign(_)
                | BinOp::RemAssign(_)
                | BinOp::BitXorAssign(_)
                | BinOp::BitAndAssign(_)
                | BinOp::BitOrAssign(_)
                | BinOp::ShlAssign(_)
                | BinOp::ShrAssign(_)
        ) {
            self.push_error(
                node.span(),
                "TLRS004",
                "#[tl_ir] 中禁止 host 赋值/复合赋值。",
                "改用 alloc_var/load/store 或 select 表达分支结果。",
            );
        }
        visit::visit_expr_binary(self, node);
    }
}

fn is_simple_let_mut(pat: &Pat) -> bool {
    matches!(
        pat,
        Pat::Ident(PatIdent {
            mutability: Some(_),
            ..
        })
    )
}

fn pattern_has_mut(pat: &Pat) -> bool {
    match pat {
        Pat::Ident(ident) => ident.mutability.is_some(),
        Pat::Reference(PatReference {
            pat, mutability, ..
        }) => mutability.is_some() || pattern_has_mut(pat),
        Pat::Or(pat_or) => pat_or.cases.iter().any(pattern_has_mut),
        Pat::Paren(paren) => pattern_has_mut(&paren.pat),
        Pat::Slice(slice) => slice.elems.iter().any(pattern_has_mut),
        Pat::Struct(strukt) => strukt
            .fields
            .iter()
            .any(|field| pattern_has_mut(&field.pat)),
        Pat::Tuple(tuple) => tuple.elems.iter().any(pattern_has_mut),
        Pat::TupleStruct(tuple) => tuple.elems.iter().any(pattern_has_mut),
        Pat::Type(typed) => pattern_has_mut(&typed.pat),
        _ => false,
    }
}
