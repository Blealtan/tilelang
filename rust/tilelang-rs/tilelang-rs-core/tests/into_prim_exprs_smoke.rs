use tilelang_rs_core::{
    BuilderContext, Expr, FromLoopVars, IntoPrimExpr, IntoPrimExprs, Result, debug_print, ffi,
    language as T,
};

#[test]
fn into_prim_exprs_variants() -> Result<()> {
    let ctx = BuilderContext::new("into_prim_exprs")?;

    ctx.with_tir_prim_func("variants", false, |_| {
        ctx.for_each(T::serial(0i64, 4i64), |_ctx, vars| {
            let i = FromLoopVars::bind1(vars);

            // Scalar -> 1-element array
            let s = (1i64).into_prim_exprs();
            assert_eq!(s.len(), 1);

            // Array -> N-element array
            let a = [1i64, 2i64, 3i64].into_prim_exprs();
            assert_eq!(a.len(), 3);

            // 0-element array
            let z: tvm_ffi::Array<ffi::ir::PrimExpr> = ([] as [i64; 0]).into_prim_exprs();
            assert_eq!(z.len(), 0);

            // 2-tuple: same types
            let t2 = (1i64, 2i64).into_prim_exprs();
            assert_eq!(t2.len(), 2);

            // 2-tuple: mixed types (&Var, i64) — the primary motivating case for T::kernel((&n1, n2))
            let t2_mixed = (&i, 128i64).into_prim_exprs();
            assert_eq!(t2_mixed.len(), 2);

            // 3-tuple with mixed types (&Var + i64)
            let t3 = (&i, 2i64, 3i64).into_prim_exprs();
            assert_eq!(t3.len(), 3);

            // 4-tuple
            let t4 = (1i64, 2i64, 3i64, 4i64).into_prim_exprs();
            assert_eq!(t4.len(), 4);

            // Expr scalar
            let expr = Expr::from(i.clone());
            let se = expr.into_prim_exprs();
            assert_eq!(se.len(), 1);

            ffi::script::ir_builder::tir::Evaluate(i.into_prim_expr())
                .expect("Evaluate should not fail");
        });
    });

    let module = ctx.finish_ir_module();
    let printed = debug_print(module);
    println!("{}", printed);
    assert!(printed.contains("def variants("));
    Ok(())
}
