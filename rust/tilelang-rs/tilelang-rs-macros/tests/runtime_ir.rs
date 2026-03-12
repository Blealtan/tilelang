use tilelang_rs_core::{debug_print, language as T, Expr, Result, Tensor};
use tilelang_rs_macros::tl_ir;

// ── kernels ───────────────────────────────────────────────────────────────────

#[tl_ir]
fn loops_if_kernel() {
    for i in T::serial(0i64, 4i64) {
        if i < 2i64 {
            let _v = T::select(true, 1i64, 0i64);
        } else {
            let acc = T::alloc_var(T::INT32, 0i64);
            acc.store(1i64);
            let _out = acc.load();
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
    let out = T::alloc_var(T::INT32, 0i64);
    if (1i64 < 2i64) && (!false || 3i64 == 3i64) {
        out.store(T::select(true, 7i64, 9i64));
    } else {
        out.store(T::select(false, 1i64, 2i64));
    }
}

/// 1-D vector_add using all Phase-2 APIs through the `#[tl_ir]` macro frontend.
///
/// `Tensor::new().declare(...)` is used directly in the body until Phase-3 adds
/// automatic macro support for `T::Tensor` function parameters.
#[tl_ir]
fn vector_add_kernel() {
    let n = T::dynamic("n", T::INT64);
    let a = Tensor::new().declare(&n, T::FLOAT32, "A");
    let b = Tensor::new().declare(&n, T::FLOAT32, "B");
    let c = Tensor::new().declare(&n, T::FLOAT32, "C");

    for bx in T::kernel(T::ceildiv(&n, 2048i64)).threads(128i64) {
        let start_x = Expr::from(&bx) * 2048i64;
        for ix in T::parallel(2048i64) {
            let x = start_x.clone() + &ix;
            c.store_at(&x, a.load_at(&x) + b.load_at(&x));
        }
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────
// Single #[test] to run all kernels sequentially.
// KernelLaunchFrame has Python-callback global state that is not safe to use
// from concurrent test threads; all runtime IR tests live in one function.

#[test]
fn tl_ir_runtime_ir() -> Result<()> {
    // loops + if
    {
        let printed = debug_print(loops_if_kernel()?);
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
    }

    // logic predicates
    {
        let printed = debug_print(logic_if_kernel()?);
        println!("{}", printed);
        assert!(printed.contains("@T.prim_func"));
        assert!(printed.contains("T.LT(T.int64(1), T.int64(2))"));
        assert!(printed.contains("T.Select"));
    }

    // Phase-2 end-to-end: T::dynamic + T::ceildiv + Tensor::declare +
    //                     T::kernel + T::parallel + Buffer::load_at/store_at
    {
        let printed = debug_print(vector_add_kernel()?);
        println!("{}", printed);
        assert!(printed.contains("def vector_add_kernel("));
        assert!(
            printed.contains("T.match_buffer") || printed.contains("T.Buffer"),
            "missing buffer declarations"
        );
        assert!(printed.contains("blockIdx.x"), "missing kernel block var");
        assert!(printed.contains("T.parallel"), "missing parallel loop");
        assert!(
            printed.contains("= A[") || printed.contains("+ B["),
            "missing load expressions"
        );
    }

    Ok(())
}
