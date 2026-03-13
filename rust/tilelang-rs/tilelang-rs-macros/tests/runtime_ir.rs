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

/// 1-D vector_add using Phase-3 named_let API through the `#[tl_ir]` macro frontend.
///
/// `Tensor::new().declare(shape, dtype)` (no explicit name) relies on the macro's
/// `named_let` transform to inject the binding variable's name into the IR.
#[tl_ir]
fn vector_add_named_let() {
    let n = T::dynamic("n", T::INT64);
    let a = Tensor::new().declare(&n, T::FLOAT32);
    let b = Tensor::new().declare(&n, T::FLOAT32);
    let c = Tensor::new().declare(&n, T::FLOAT32);

    for bx in T::kernel(T::ceildiv(&n, 2048i64)).threads(128i64) {
        let start_x = Expr::from(&bx) * 2048i64;
        for ix in T::parallel(2048i64) {
            let x = start_x.clone() + &ix;
            c.store_at(&x, a.load_at(&x) + b.load_at(&x));
        }
    }
}

/// Element-wise add using `T::Tensor` parameter syntax (p3-macro-tensor-params).
///
/// Parameters typed `T::Tensor` are stripped from the function signature by
/// `#[tl_ir]` and replaced with `let a = Tensor::new();` at the start of the
/// function body. The `named_let` transform then injects the binding name "a",
/// "b", "c" via `PendingBuffer::apply_name` when `declare` is called.
#[tl_ir]
fn buffer_kernel(a: T::Tensor, b: T::Tensor, c: T::Tensor) {
    let n = T::dynamic("n", T::INT64);
    let a = a.declare(&n, T::FLOAT32);
    let b = b.declare(&n, T::FLOAT32);
    let c = c.declare(&n, T::FLOAT32);
    for i in T::serial(0i64, &n) {
        c.store_at(&i, a.load_at(&i) + b.load_at(&i));
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

    // Phase-3 named_let: Tensor::declare without name — variable binding name
    //                    is injected automatically by the macro transform.
    {
        let printed = debug_print(vector_add_named_let()?);
        println!("{}", printed);
        assert!(printed.contains("def vector_add_named_let("));
        assert!(
            printed.contains("T.match_buffer") || printed.contains("T.Buffer"),
            "missing buffer declarations"
        );
        assert!(printed.contains("blockIdx.x"), "missing kernel block var");
        assert!(printed.contains("T.parallel"), "missing parallel loop");
        // The buffers should be named by the binding variable (a, b, c).
        assert!(
            printed.contains(": a") || printed.contains("\"a\"") || printed.contains("a["),
            "buffer 'a' name not injected into IR"
        );
    }

    Ok(())
}

/// Verify that `#[tl_ir]` correctly strips `T::Tensor` parameters and that the
/// named_let transform injects buffer names into the resulting IR.
///
/// Uses `T::serial` (no KernelLaunch) so this test is safe to run concurrently.
#[test]
fn tl_ir_runtime_prints_buffer_ir() -> Result<()> {
    let printed = debug_print(buffer_kernel()?);
    println!("{}", printed);
    assert!(
        printed.contains("def buffer_kernel("),
        "missing function name"
    );
    assert!(
        printed.contains("T.match_buffer") || printed.contains("T.Buffer"),
        "missing buffer declarations"
    );
    // The buffers should be named by the binding variable (a, b, c).
    assert!(
        printed.contains(": a") || printed.contains("\"a\"") || printed.contains("a["),
        "buffer 'a' name not injected into IR"
    );
    assert!(
        printed.contains(": b") || printed.contains("\"b\"") || printed.contains("b["),
        "buffer 'b' name not injected into IR"
    );
    assert!(
        printed.contains(": c") || printed.contains("\"c\"") || printed.contains("c["),
        "buffer 'c' name not injected into IR"
    );
    Ok(())
}
