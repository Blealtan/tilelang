use tilelang_rs_core::{
    debug_print, ffi, language as T, pred, BuilderContext, FromLoopVars, IntoPrimExpr, Result,
};

fn emit_eval_expr(expr: ffi::ir::PrimExpr) {
    ffi::script::ir_builder::tir::Evaluate(expr).expect("Evaluate should not fail");
}

fn emit_eval_int(value: i64) {
    emit_eval_expr(value.into_prim_expr());
}

#[test]
fn pred_if_smoke_prints_if_and_select() -> Result<()> {
    let ctx = BuilderContext::new("pred_if_smoke")?;

    ctx.with_tir_prim_func("host_bool_if", false, |_prim_func| {
        let cond = pred::to_ir_bool(true);
        ctx.if_stmt(cond, || emit_eval_int(1), Some(|| emit_eval_int(0)));
    });

    ctx.with_tir_prim_func("compare_if", false, |_prim_func| {
        ctx.for_each(T::serial(0i64, 4i64), |ctx, vars| {
            let i = FromLoopVars::bind1(vars);
            let cond = pred::lt(i.clone(), 2i64);
            ctx.if_stmt(
                cond,
                || emit_eval_expr(i.clone().into_prim_expr()),
                Some(|| emit_eval_int(0)),
            );
        });
    });

    ctx.with_tir_prim_func("logic_if", false, |_prim_func| {
        let cond = pred::and(pred::lt(1i64, 2i64), || {
            pred::or(pred::not(false), || pred::eq(3i64, 3i64))
        });
        ctx.if_stmt(cond, || emit_eval_int(7), Some(|| emit_eval_int(9)));
    });

    ctx.with_tir_prim_func("select_expr", false, |_prim_func| {
        let cond = pred::lt(1i64, 2i64);
        let selected = T::select(cond, 11i64, 22i64);
        emit_eval_expr(selected);
    });

    let module = ctx.finish_ir_module();
    let printed = debug_print(module);
    println!("{}", printed);

    assert!(printed.contains("def host_bool_if("));
    assert!(printed.contains("def compare_if("));
    assert!(printed.contains("def logic_if("));
    assert!(printed.contains("def select_expr("));
    assert!(printed.contains("if "));
    assert!(printed.contains("T.Select") || printed.contains("T.if_then_else"));

    Ok(())
}
