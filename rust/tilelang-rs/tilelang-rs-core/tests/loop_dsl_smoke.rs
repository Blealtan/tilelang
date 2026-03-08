use std::path::PathBuf;

use tilelang_rs_core::{
    debug_print, ffi, language as T, BuilderContext, FromLoopVars, IntoPrimExpr, Result,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("tilelang-rs-core should live under rust/tilelang-rs")
        .parent()
        .expect("tilelang-rs workspace should live under rust/")
        .parent()
        .expect("rust/ should live under repository root")
        .to_path_buf()
}

fn load_tilelang_runtime() -> Result<()> {
    let lib_dir = repo_root().join("build/lib");
    let tvm_path = lib_dir.join("libtvm.so");
    let tilelang_path = lib_dir.join("libtilelang_module.so");

    ffi::load_library(tvm_path.to_str().expect("library path must be valid UTF-8"))?;
    ffi::load_library(
        tilelang_path
            .to_str()
            .expect("library path must be valid UTF-8"),
    )?;
    Ok(())
}

fn emit_eval(var: ffi::tir::Var) -> Result<()> {
    ffi::script::ir_builder::tir::Evaluate(var.into_prim_expr()?)
}

#[test]
fn loop_dsl_smoke_prints_all_loop_kinds() -> Result<()> {
    load_tilelang_runtime()?;

    let ctx = BuilderContext::new("loop_dsl_smoke")?;

    ctx.with_tir_prim_func("serial_loop", false, |_prim_func| {
        ctx.for_each(T::serial(0i64, 4i64), |_ctx, vars| {
            let i = FromLoopVars::bind1(vars)?;
            emit_eval(i)
        })
    })?;

    ctx.with_tir_prim_func("parallel_loop", false, |_prim_func| {
        ctx.for_each(T::parallel([2i64, 3i64]), |_ctx, vars| {
            let (i, j) = FromLoopVars::bind2(vars)?;
            emit_eval(i)?;
            emit_eval(j)
        })
    })?;

    ctx.with_tir_prim_func("vectorized_loop", false, |_prim_func| {
        ctx.for_each(T::vectorized(0i64, 8i64), |_ctx, vars| {
            let i = FromLoopVars::bind1(vars)?;
            emit_eval(i)
        })
    })?;

    ctx.with_tir_prim_func("unroll_loop", false, |_prim_func| {
        ctx.for_each(T::unroll(0i64, 8i64), |_ctx, vars| {
            let i = FromLoopVars::bind1(vars)?;
            emit_eval(i)
        })
    })?;

    ctx.with_tir_prim_func("pipelined_loop", false, |_prim_func| {
        ctx.for_each(T::pipelined(0i64, 4i64).num_stages(2), |_ctx, vars| {
            let i = FromLoopVars::bind1(vars)?;
            emit_eval(i)
        })
    })?;

    let module = ctx.finish_ir_module()?;
    let printed = debug_print(module)?;
    println!("{}", printed);

    assert!(printed.contains("def serial_loop("));
    assert!(printed.contains("def parallel_loop("));
    assert!(printed.contains("def vectorized_loop("));
    assert!(printed.contains("def unroll_loop("));
    assert!(printed.contains("def pipelined_loop("));
    assert!(printed.contains("T.parallel"));

    Ok(())
}
