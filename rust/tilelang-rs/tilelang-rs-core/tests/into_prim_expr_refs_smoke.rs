use tilelang_rs_core::{
    ffi, language as T, BuilderContext, Expr, FromLoopVars, IntoPrimExpr, Result,
};

#[test]
fn into_prim_expr_ref_impls() -> Result<()> {
    let ctx = BuilderContext::new("ref_impls")?;

    ctx.with_tir_prim_func("refs", false, |_| {
        ctx.for_each(T::serial(0i64, 4i64), |_ctx, vars| {
            let i = FromLoopVars::bind1(vars);

            // &PendingLoopVar -> PrimExpr
            let _e1: ffi::ir::PrimExpr = (&i).into_prim_expr();
            // PendingLoopVar -> PrimExpr (via IntoPrimExpr)
            let raw: ffi::ir::PrimExpr = i.clone().into_prim_expr();
            // &PrimExpr -> PrimExpr
            let _e2: ffi::ir::PrimExpr = (&raw).into_prim_expr();
            // PendingLoopVar -> Expr -> PrimExpr
            let expr = Expr::from(i.clone());
            let _e3: ffi::ir::PrimExpr = (&expr).into_prim_expr();

            ffi::script::ir_builder::tir::Evaluate(i.into_prim_expr())
                .expect("Evaluate should not fail");
        });
    });

    let module = ctx.finish_ir_module();
    let _ = tilelang_rs_core::debug_print(module);
    Ok(())
}
