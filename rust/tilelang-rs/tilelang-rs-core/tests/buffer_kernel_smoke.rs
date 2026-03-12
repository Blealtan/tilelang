/// Core-layer unit tests for Phase-2 DSL building blocks.
///
/// These tests call the builder APIs directly (no `#[tl_ir]` macro) to verify
/// that each primitive works at the core level.  End-to-end macro integration
/// lives in `tilelang-rs-macros/tests/runtime_ir.rs`.
use tilelang_rs_core::{
    debug_print, language as T, BuilderContext, Expr, FromLoopVars, IntoPrimExpr, Result, Tensor,
};

fn ffi_eval(expr: tilelang_rs_core::ffi::ir::PrimExpr) {
    tilelang_rs_core::ffi::script::ir_builder::tir::Evaluate(expr)
        .expect("Evaluate should not fail");
}

#[test]
fn buffer_kernel_smoke_all() -> Result<()> {
    // ── T::dynamic + T::ceildiv + Tensor::declare + T::kernel + T::parallel ─
    {
        let block_n = 2048i64;
        let ctx = BuilderContext::new("vector_add")?;
        ctx.with_tir_prim_func("vector_add", false, |_pf| {
            let n = T::dynamic("n", T::INT64);
            let a = Tensor::new().declare(&n, T::FLOAT32, "A");
            let b = Tensor::new().declare(&n, T::FLOAT32, "B");
            let c = Tensor::new().declare(&n, T::FLOAT32, "C");

            let grid = T::ceildiv(&n, block_n);
            ctx.for_each(T::kernel(grid).threads(128i64), |ctx, bvars| {
                let bx = FromLoopVars::bind1(bvars);
                let start_x = Expr::from(&bx) * block_n;
                ctx.for_each(T::parallel(block_n), |_ctx, ivars| {
                    let ix = FromLoopVars::bind1(ivars);
                    let x = start_x.clone() + &ix;
                    c.store_at(&x, a.load_at(&x) + b.load_at(&x));
                });
            });
        });

        let printed = debug_print(ctx.finish_ir_module());
        assert!(printed.contains("def vector_add("));
        assert!(printed.contains("T.match_buffer") || printed.contains("T.Buffer"));
        assert!(printed.contains("blockIdx.x"));
        assert!(printed.contains("T.parallel"));
    }

    // ── T::parallel: scalar extent (new IntoPrimExprs scalar impl) ───────────
    {
        let ctx = BuilderContext::new("parallel_1d")?;
        ctx.with_tir_prim_func("parallel_1d", false, |_pf| {
            ctx.for_each(T::parallel(128i64), |_ctx, vars| {
                let _i = FromLoopVars::bind1(vars);
            });
        });
        assert!(debug_print(ctx.finish_ir_module()).contains("def parallel_1d("));
    }

    // ── T::parallel: 2-D array extent (backward compat) ─────────────────────
    {
        let ctx = BuilderContext::new("parallel_2d")?;
        ctx.with_tir_prim_func("parallel_2d", false, |_pf| {
            ctx.for_each(T::parallel([4i64, 8i64]), |_ctx, vars| {
                let (_i, _j) = FromLoopVars::bind2(vars);
            });
        });
        assert!(debug_print(ctx.finish_ir_module()).contains("def parallel_2d("));
    }

    // ── T::dynamic creates a symbolic Var visible in IR ──────────────────────
    {
        let ctx = BuilderContext::new("dyn_var")?;
        ctx.with_tir_prim_func("dyn_var", false, |_pf| {
            let n = T::dynamic("n", T::INT64);
            ctx.for_each(T::serial(0i64, n), |_ctx, vars| {
                ffi_eval(FromLoopVars::bind1(vars).into_prim_expr());
            });
        });
        assert!(debug_print(ctx.finish_ir_module()).contains("def dyn_var("));
    }

    // ── T::ceildiv produces a ceiling-division expression ────────────────────
    {
        let ctx = BuilderContext::new("ceildiv_expr")?;
        ctx.with_tir_prim_func("ceildiv_expr", false, |_pf| {
            let n = T::dynamic("n", T::INT64);
            ffi_eval(T::ceildiv(&n, 128i64).into_inner());
        });
        assert!(debug_print(ctx.finish_ir_module()).contains("def ceildiv_expr("));
    }

    Ok(())
}
