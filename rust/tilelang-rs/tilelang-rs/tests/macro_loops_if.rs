use tilelang_rs::{debug_print, language as T, tl_ir, Result};

#[tl_ir]
fn macro_kernel() {
    let acc = T::alloc_var(T::int32(), 0i64)?;

    for i in T::serial(0i64, 4i64) {
        if i < 2i64 {
            acc.store(1i64)?;
        } else {
            acc.store(T::select(false, 2i64, 3i64)?)?;
        }
    }

    for (i, j) in T::parallel([2i64, 3i64]) {
        let _ = (i, j);
    }
}

#[test]
fn public_macro_kernel_prints_ir() -> Result<()> {
    let printed = debug_print(macro_kernel()?)?;
    println!("{}", printed);

    assert!(printed.contains("@T.prim_func"));
    assert!(printed.contains("for v in range(T.int64(4))"));
    assert!(printed.contains("for v in T.parallel(T.int64(2))"));
    assert!(printed.contains("scope=\"local.var\""));
    assert!(printed.contains("if v < T.int64(2):"));

    Ok(())
}
