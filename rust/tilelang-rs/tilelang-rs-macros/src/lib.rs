use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{
    Arm, BinOp, Block, Expr, ExprAssign, ExprBinary, ExprBlock, ExprClosure, ExprForLoop,
    ExprGroup, ExprIf, ExprLet, ExprParen, ExprReference, ExprUnary, FnArg, ItemFn, Local, Pat,
    PatIdent, PatReference, PatType, ReturnType, Stmt, TypeReference, UnOp, parse_macro_input,
    parse_quote,
};

/// Emit a `set_current_span` call that records the Rust source location of `s`
/// in the thread-local span used by subsequent IR node construction.
fn span_setter(s: &Span) -> TokenStream2 {
    let lc = s.start();
    let line = lc.line as i64;
    let col = lc.column as i64;
    quote! {
        ::tilelang_rs_core::set_current_span(file!(), #line, #col);
    }
}

#[proc_macro_attribute]
pub fn tl_ir(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let mut checker = Precheck::default();
    checker.visit_item_fn(&input);

    if let Some(error) = checker.into_error() {
        return TokenStream::from(error.to_compile_error());
    }

    match expand_tl_ir(&input) {
        Ok(tokens) => TokenStream::from(tokens),
        Err(error) => TokenStream::from(error.to_compile_error()),
    }
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

    fn push_error(&mut self, span: Span, code: &str, message: &str, help: &str) {
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

fn expand_tl_ir(input: &ItemFn) -> syn::Result<TokenStream2> {
    let attrs = &input.attrs;
    let vis = &input.vis;
    let mut sig = input.sig.clone();
    let fn_name = sig.ident.clone();

    // Collect and strip Tensor parameters before setting output type.
    let tensor_params = extract_tensor_params(&mut sig);

    sig.output = ReturnType::Type(
        Default::default(),
        Box::new(parse_quote!(
            ::tilelang_rs_core::Result<::tilelang_rs_core::ffi::ir::IRModule>
        )),
    );

    // Generate `let a = ::tilelang_rs_core::Tensor::new();` for each stripped param.
    let tensor_inits: TokenStream2 = tensor_params
        .iter()
        .map(|ident| {
            quote! {
                let #ident = ::tilelang_rs_core::Tensor::new();
            }
        })
        .collect();

    let ctx_ident = format_ident!("__ctx");
    let body = transform_block(&input.block, &ctx_ident)?;

    Ok(quote! {
        #(#attrs)*
        #vis #sig {
            let #ctx_ident = ::tilelang_rs_core::BuilderContext::new(stringify!(#fn_name))?;
            #ctx_ident.with_tir_prim_func(stringify!(#fn_name), false, |_prim_func| {
                #tensor_inits
                #body
            });
            Ok(#ctx_ident.finish_ir_module())
        }
    })
}

/// Returns `true` if the type is a path whose last segment is `Tensor`.
///
/// This matches `Tensor`, `T::Tensor`, `tilelang_rs::Tensor`, etc.
fn is_tensor_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty
        && let Some(seg) = type_path.path.segments.last()
    {
        return seg.ident == "Tensor";
    }
    false
}

/// Remove all `fn(param: *::Tensor)` arguments from `sig.inputs` and return their names.
///
/// Any `FnArg::Typed` whose type path ends in `Tensor` is stripped from the signature.
/// A `let param = ::tilelang_rs_core::Tensor::new();` initializer is injected into the
/// function body by `expand_tl_ir` for each stripped parameter.
fn extract_tensor_params(sig: &mut syn::Signature) -> Vec<syn::Ident> {
    let mut tensor_params = Vec::new();
    sig.inputs = sig
        .inputs
        .iter()
        .filter(|arg| {
            if let FnArg::Typed(pat_type) = arg
                && is_tensor_type(&pat_type.ty)
                && let Pat::Ident(PatIdent {
                    ident,
                    mutability: None,
                    subpat: None,
                    ..
                }) = pat_type.pat.as_ref()
            {
                tensor_params.push(ident.clone());
                return false;
            }
            true
        })
        .cloned()
        .collect();
    tensor_params
}

fn transform_block(block: &Block, ctx_ident: &syn::Ident) -> syn::Result<TokenStream2> {
    let mut tokens = TokenStream2::new();
    for stmt in &block.stmts {
        tokens.extend(transform_stmt(stmt, ctx_ident)?);
    }
    Ok(tokens)
}

fn transform_stmt(stmt: &Stmt, ctx_ident: &syn::Ident) -> syn::Result<TokenStream2> {
    match stmt {
        Stmt::Local(local) => {
            let setter = span_setter(&local.span());
            let body = if let Some(tokens) = try_named_let_transform(local) {
                tokens
            } else {
                quote!(#local)
            };
            Ok(quote! { #setter #body })
        }
        Stmt::Item(item) => Ok(quote!(#item)),
        Stmt::Macro(mac) => Ok(quote!(#mac)),
        Stmt::Expr(expr, semi) => {
            let setter = span_setter(&expr.span());
            let body = transform_stmt_expr(expr, semi.is_some(), ctx_ident)?;
            Ok(quote! { #setter #body })
        }
    }
}

/// Attempt to transform `let x = expr;` into the autoref named_let pattern.
///
/// Matches `Pat::Ident` and `Pat::Type(Pat::Ident, ...)` bindings that have an init
/// expression and no diverge clause (`let x = expr else { ... }`).  Ignored bindings
/// (`let _ = …`) and `mut` bindings are left unchanged (the precheck already rejects
/// the latter, but we guard here for safety).
///
/// The generated expansion is:
/// ```ignore
/// let x = {
///     #[allow(unused_imports)]
///     use ::tilelang_rs_core::PassthroughTag as _;
///     let __val = expr;
///     (&__val).__tl_named_tag().apply(__val, "x")
/// };
/// ```
///
/// For nameable types (e.g. `PendingBuffer`) the inherent `__tl_named_tag` takes
/// priority and returns `NameableKind`, whose `apply` registers `Arg("x", buffer)` and
/// returns a `Buffer`.  For all other types the blanket `PassthroughTag` impl returns
/// `PassthroughKind`, whose generic `apply` is a no-op passthrough.
fn try_named_let_transform(local: &Local) -> Option<TokenStream2> {
    // Only transform if there is an init expression with no `else` diverge clause.
    let init = local.init.as_ref()?;
    if init.diverge.is_some() {
        return None;
    }
    let expr = &init.expr;

    // Extract the binding identifier and (optional) type annotation from the pattern.
    let (var_ident, opt_ty) = match &local.pat {
        Pat::Ident(PatIdent {
            ident,
            mutability: None,
            subpat: None,
            ..
        }) => {
            if ident == "_" {
                return None;
            }
            (ident.clone(), None)
        }
        Pat::Type(PatType { pat, ty, .. }) => {
            if let Pat::Ident(PatIdent {
                ident,
                mutability: None,
                subpat: None,
                ..
            }) = pat.as_ref()
            {
                if ident == "_" {
                    return None;
                }
                (ident.clone(), Some(ty.as_ref()))
            } else {
                return None;
            }
        }
        _ => return None,
    };

    let attrs = &local.attrs;
    let var_name = var_ident.to_string();
    let val_ident = format_ident!("__val");

    // Reconstruct the left-hand side pattern (with optional type annotation).
    let lhs = if let Some(ty) = opt_ty {
        quote!(#var_ident: #ty)
    } else {
        quote!(#var_ident)
    };

    Some(quote! {
        #(#attrs)*
        let #lhs = {
            #[allow(unused_imports)]
            use ::tilelang_rs_core::PassthroughTag as _;
            let #val_ident = #expr;
            (&#val_ident).__tl_named_tag().apply(#val_ident, #var_name)
        };
    })
}

fn transform_stmt_expr(
    expr: &Expr,
    has_semi: bool,
    ctx_ident: &syn::Ident,
) -> syn::Result<TokenStream2> {
    match expr {
        Expr::ForLoop(for_loop) => transform_for_loop(for_loop, ctx_ident),
        Expr::If(expr_if) => transform_if_stmt(expr_if, ctx_ident),
        Expr::Block(expr_block) => transform_block_expr(expr_block, has_semi, ctx_ident),
        Expr::Group(ExprGroup { expr, .. }) | Expr::Paren(ExprParen { expr, .. }) => {
            transform_stmt_expr(expr, has_semi, ctx_ident)
        }
        _ => Ok(quote!(#expr;)),
    }
}

fn transform_block_expr(
    expr_block: &ExprBlock,
    has_semi: bool,
    ctx_ident: &syn::Ident,
) -> syn::Result<TokenStream2> {
    let attrs = &expr_block.attrs;
    let label = &expr_block.label;
    let inner = transform_block(&expr_block.block, ctx_ident)?;
    let block = quote! {
        #(#attrs)*
        #label
        {
            #inner
        }
    };
    if has_semi {
        Ok(quote!(#block;))
    } else {
        Ok(block)
    }
}

fn transform_for_loop(for_loop: &ExprForLoop, ctx_ident: &syn::Ident) -> syn::Result<TokenStream2> {
    let dsl = &for_loop.expr;
    let vars_ident = format_ident!("__vars");
    let inner_ctx_ident = format_ident!("__ctx");
    let binding = loop_binding(&for_loop.pat, &vars_ident)?;
    let body = transform_block(&for_loop.body, &inner_ctx_ident)?;

    Ok(quote! {
        #ctx_ident.for_each(#dsl, |#inner_ctx_ident, #vars_ident| {
            #binding
            #body
        });
    })
}

/// Emit the named-let binding for one loop variable.
///
/// `raw_var_expr` must evaluate to a `PendingLoopVar` (the direct return value
/// of [`FromLoopVars::bind*`]).  Generates:
///
/// ```ignore
/// let <ident> = {
///     #[allow(unused_imports)]
///     use ::tilelang_rs_core::PassthroughTag as _;
///     let __val = raw_var_expr;   // PendingLoopVar from bind*
///     (&__val).__tl_named_tag().apply(__val, "ident")
/// };
/// ```
fn named_loop_var_binding(pat: &Pat, var_name: &str, raw_var_expr: TokenStream2) -> TokenStream2 {
    quote! {
        let #pat = {
            #[allow(unused_imports)]
            use ::tilelang_rs_core::PassthroughTag as _;
            let __val = #raw_var_expr;
            (&__val).__tl_named_tag().apply(__val, #var_name)
        };
    }
}

fn loop_binding(pat: &Pat, vars_ident: &syn::Ident) -> syn::Result<TokenStream2> {
    match pat {
        Pat::Ident(pat_ident) => {
            let var_name = pat_ident.ident.to_string();
            let raw = quote! { ::tilelang_rs_core::FromLoopVars::bind1(#vars_ident) };
            Ok(named_loop_var_binding(pat, &var_name, raw))
        }
        Pat::Tuple(tuple) => {
            let arity = tuple.elems.len();
            if !(2..=4).contains(&arity) {
                return Err(syn::Error::new(
                    tuple.span(),
                    "#[tl_ir] 目前仅支持单变量循环绑定或 2-4 元组绑定。",
                ));
            }

            let bind_fn = match arity {
                2 => format_ident!("bind2"),
                3 => format_ident!("bind3"),
                _ => format_ident!("bind4"),
            };

            // Generate temporary names for each raw Var from bind*.
            let tmp_idents: Vec<syn::Ident> =
                (0..arity).map(|i| format_ident!("__tup_{}", i)).collect();

            // Destructure bind* into temporaries.
            let bind_stmt = quote! {
                let (#(#tmp_idents),*) =
                    ::tilelang_rs_core::FromLoopVars::#bind_fn(#vars_ident);
            };

            // Generate a named-let binding for each element.
            let mut bindings = bind_stmt;
            for (i, elem_pat) in tuple.elems.iter().enumerate() {
                let var_name = match elem_pat {
                    Pat::Ident(pi) => pi.ident.to_string(),
                    _ => "_".to_string(),
                };
                let tmp = &tmp_idents[i];
                let raw = quote! { #tmp };
                bindings.extend(named_loop_var_binding(elem_pat, &var_name, raw));
            }
            Ok(bindings)
        }
        _ => Err(syn::Error::new(
            pat.span(),
            "#[tl_ir] 目前仅支持标识符或元组循环绑定模式。",
        )),
    }
}

fn transform_if_stmt(expr_if: &ExprIf, ctx_ident: &syn::Ident) -> syn::Result<TokenStream2> {
    let cond = lower_predicate(&expr_if.cond)?;
    let then_body = transform_block(&expr_if.then_branch, ctx_ident)?;
    let else_body = if let Some((_, else_expr)) = &expr_if.else_branch {
        let else_tokens = transform_else_expr(else_expr, ctx_ident)?;
        quote!(Some(|| {
            #else_tokens
        }))
    } else {
        quote!(None::<fn()>)
    };

    Ok(quote! {
        #ctx_ident.if_stmt(
            #cond,
            || {
                #then_body
            },
            #else_body,
        );
    })
}

fn transform_else_expr(expr: &Expr, ctx_ident: &syn::Ident) -> syn::Result<TokenStream2> {
    match expr {
        Expr::Block(expr_block) => transform_block(&expr_block.block, ctx_ident),
        Expr::If(expr_if) => transform_if_stmt(expr_if, ctx_ident),
        Expr::Group(ExprGroup { expr, .. }) | Expr::Paren(ExprParen { expr, .. }) => {
            transform_else_expr(expr, ctx_ident)
        }
        _ => Ok(quote!(#expr;)),
    }
}

fn lower_predicate(expr: &Expr) -> syn::Result<TokenStream2> {
    match expr {
        Expr::Binary(ExprBinary {
            left, op, right, ..
        }) => match op {
            BinOp::Eq(_) => Ok(quote!(::tilelang_rs_core::pred::eq(#left, #right))),
            BinOp::Ne(_) => Ok(quote!(::tilelang_rs_core::pred::ne(#left, #right))),
            BinOp::Lt(_) => Ok(quote!(::tilelang_rs_core::pred::lt(#left, #right))),
            BinOp::Le(_) => Ok(quote!(::tilelang_rs_core::pred::le(#left, #right))),
            BinOp::Gt(_) => Ok(quote!(::tilelang_rs_core::pred::gt(#left, #right))),
            BinOp::Ge(_) => Ok(quote!(::tilelang_rs_core::pred::ge(#left, #right))),
            BinOp::And(_) => {
                let lhs = lower_predicate(left)?;
                let rhs = lower_predicate(right)?;
                Ok(quote!(::tilelang_rs_core::pred::and(#lhs, || { #rhs })))
            }
            BinOp::Or(_) => {
                let lhs = lower_predicate(left)?;
                let rhs = lower_predicate(right)?;
                Ok(quote!(::tilelang_rs_core::pred::or(#lhs, || { #rhs })))
            }
            _ => Ok(quote!(::tilelang_rs_core::pred::to_ir_bool(#expr))),
        },
        Expr::Unary(ExprUnary {
            op: UnOp::Not(_),
            expr,
            ..
        }) => {
            let inner = lower_predicate(expr)?;
            Ok(quote!(::tilelang_rs_core::pred::not(#inner)))
        }
        Expr::Paren(ExprParen { expr, .. }) | Expr::Group(ExprGroup { expr, .. }) => {
            lower_predicate(expr)
        }
        _ => Ok(quote!(::tilelang_rs_core::pred::to_ir_bool(#expr))),
    }
}
