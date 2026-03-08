use std::path::PathBuf;

use tilelang_rs_core::{debug_print, ffi, language as T, pred, BuilderContext, Result};

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

#[test]
fn alloc_var_smoke_prints_local_var_updates() -> Result<()> {
    load_tilelang_runtime()?;

    let ctx = BuilderContext::new("alloc_var_smoke")?;
    ctx.with_tir_prim_func("alloc_var_if", false, |_prim_func| {
        let acc = T::alloc_var(T::int32(), 0i64)?;
        let cond = pred::lt(1i64, 2i64)?;

        ctx.if_stmt(
            cond,
            || acc.store(1i64),
            Some(|| {
                let merged = T::select(false, 2i64, 3i64)?;
                acc.store(merged)
            }),
        )?;

        ffi::script::ir_builder::tir::Evaluate(acc.load()?)
    })?;

    let module = ctx.finish_ir_module()?;
    let printed = debug_print(module)?;
    println!("{}", printed);

    assert!(printed.contains("def alloc_var_if("));
    assert!(printed.contains("local.var"));
    assert!(printed.contains("if "));

    Ok(())
}
