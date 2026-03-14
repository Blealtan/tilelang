use tilelang_rs_core::{
    BuilderContext, Expr, FromLoopVars, IntoPrimExpr, Result, debug_print, ffi, language as T,
};

fn evaluate(expr: impl IntoPrimExpr) {
    ffi::script::ir_builder::tir::Evaluate(expr.into_prim_expr())
        .expect("Evaluate should not fail");
}

#[test]
fn expr_arithmetic_ops_roundtrip() -> Result<()> {
    let ctx = BuilderContext::new("expr_arithmetic_ops")?;

    ctx.with_tir_prim_func("arith", false, |_| {
        ctx.for_each(T::serial(0i64, 16i64), |_ctx, vars| {
            let i = FromLoopVars::bind1(vars);

            evaluate(Expr::from(i.clone()) + 1i64);
            evaluate(Expr::from(i.clone()) - 1i64);
            evaluate(Expr::from(i.clone()) * 2i64);
            evaluate(Expr::from(i.clone()) / 2i64);
            evaluate(Expr::from(i.clone()) + &i);
            evaluate(Expr::from(i.clone()) * 2i64 + 1i64);
        });
    });

    let module = ctx.finish_ir_module();
    let printed = debug_print(module);
    println!("{}", printed);

    assert!(printed.contains("def arith("));
    assert!(printed.contains("T.evaluate"));
    Ok(())
}
