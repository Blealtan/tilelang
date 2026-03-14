use tilelang_rs_core::{
    debug_print, ffi, language as T, BuilderContext, FromLoopVars, IntoPrimExpr, Result,
};

fn emit_eval(expr: impl IntoPrimExpr) {
    ffi::script::ir_builder::tir::Evaluate(expr.into_prim_expr())
        .expect("Evaluate should not fail");
}

#[test]
fn loop_dsl_smoke_prints_all_loop_kinds() -> Result<()> {
    let ctx = BuilderContext::new("loop_dsl_smoke")?;

    ctx.with_tir_prim_func("serial_loop", false, |_prim_func| {
        ctx.for_each(T::serial(0i64, 4i64), |_ctx, vars| {
            let i = FromLoopVars::bind1(vars);
            emit_eval(i);
        });
    });

    ctx.with_tir_prim_func("parallel_loop", false, |_prim_func| {
        ctx.for_each(T::parallel([2i64, 3i64]), |_ctx, vars| {
            let (i, j) = FromLoopVars::bind2(vars);
            emit_eval(i);
            emit_eval(j);
        });
    });

    ctx.with_tir_prim_func("vectorized_loop", false, |_prim_func| {
        ctx.for_each(T::vectorized(0i64, 8i64), |_ctx, vars| {
            let i = FromLoopVars::bind1(vars);
            emit_eval(i);
        });
    });

    ctx.with_tir_prim_func("unroll_loop", false, |_prim_func| {
        ctx.for_each(T::unroll(0i64, 8i64), |_ctx, vars| {
            let i = FromLoopVars::bind1(vars);
            emit_eval(i);
        });
    });

    ctx.with_tir_prim_func("pipelined_loop", false, |_prim_func| {
        ctx.for_each(T::pipelined(0i64, 4i64).num_stages(2), |_ctx, vars| {
            let i = FromLoopVars::bind1(vars);
            emit_eval(i);
        });
    });

    let module = ctx.finish_ir_module();
    let printed = debug_print(module);
    println!("{}", printed);

    assert!(printed.contains("def serial_loop("));
    assert!(printed.contains("def parallel_loop("));
    assert!(printed.contains("def vectorized_loop("));
    assert!(printed.contains("def unroll_loop("));
    assert!(printed.contains("def pipelined_loop("));
    assert!(printed.contains("T.parallel"));

    Ok(())
}
