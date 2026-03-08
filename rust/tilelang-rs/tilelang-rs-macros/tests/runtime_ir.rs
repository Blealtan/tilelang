use std::path::PathBuf;

use tilelang_rs_core::{debug_print, ffi, language as T, Result};
use tilelang_rs_macros::tl_ir;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("tilelang-rs-macros should live under rust/tilelang-rs")
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

#[tl_ir]
fn loops_if_kernel() {
    for i in T::serial(0i64, 4i64) {
        if i < 2i64 {
            let _v = T::select(true, 1i64, 0i64)?;
        } else {
            let acc = T::alloc_var(T::int32(), 0i64)?;
            acc.store(1i64)?;
            let _out = acc.load()?;
        }
    }

    for (i, j) in T::parallel([2i64, 3i64]) {
        let _ = (i, j);
    }

    for i in T::pipelined(0i64, 2i64) {
        let _ = i;
    }
}

#[tl_ir]
fn logic_if_kernel() {
    let out = T::alloc_var(T::int32(), 0i64)?;
    if (1i64 < 2i64) && (!false || 3i64 == 3i64) {
        out.store(T::select(true, 7i64, 9i64)?)?;
    } else {
        out.store(T::select(false, 1i64, 2i64)?)?;
    }
}

#[test]
fn tl_ir_runtime_prints_loop_and_if_ir() -> Result<()> {
    load_tilelang_runtime()?;

    let printed = debug_print(loops_if_kernel()?)?;
    println!("{}", printed);

    assert!(printed.contains("@T.prim_func"));
    assert!(printed.contains("for v in range(T.int64(4))"));
    assert!(printed.contains("for v in T.parallel(T.int64(2))"));
    assert!(
        printed.contains("annotations={\"num_stages\": 0}")
            || printed.contains("for v in range(T.int64(2))")
            || printed.contains("for v in T.serial(T.int64(2)")
    );
    assert!(printed.contains("if v < T.int64(2):"));
    assert!(printed.contains("scope=\"local.var\""));

    Ok(())
}

#[test]
fn tl_ir_runtime_prints_logic_predicates() -> Result<()> {
    load_tilelang_runtime()?;

    let printed = debug_print(logic_if_kernel()?)?;
    println!("{}", printed);

    assert!(printed.contains("@T.prim_func"));
    assert!(printed.contains("if "));
    assert!(printed.contains("T.LT(T.int64(1), T.int64(2))"));
    assert!(printed.contains("T.Select"));

    Ok(())
}
