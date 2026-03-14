/// Verifies that `#[tl_ir]`-injected spans appear in expression IR nodes.
///
/// The `set_current_span(file, line, col)` call injected by the macro before
/// each statement is read by `empty_span()` when our code creates expression
/// nodes (`Var` via `T::dynamic`, `Add`/`Mul`, `BufferLoad`).
///
/// Loop variable `Var` objects are created internally by TVM's `Serial()` /
/// `Parallel()` / `KernelLaunch()` with a null span.  `IRBuilderName` renames
/// them in-place but does not set a span.  Calling `get_span()` on a
/// TVM-internal node with a null span causes a null-pointer crash, so we only
/// verify span tracking on nodes we explicitly construct with `empty_span()`.
use tilelang_rs_core::{ffi, language as T, IntoPrimExpr, Result};
use tilelang_rs_macros::tl_ir;

// ── kernel showing correct loop variable names ────────────────────────────────

#[tl_ir]
fn span_kernel(a: T::Tensor, b: T::Tensor, c: T::Tensor) {
    let n = T::dynamic("n", T::INT64);
    let a = a.declare(&n, T::FLOAT32);
    let b = b.declare(&n, T::FLOAT32);
    let c = c.declare(&n, T::FLOAT32);
    for i in T::serial(0i64, &n) {
        c.store_at(&i, a.load_at(&i) + b.load_at(&i));
    }
}

// ── test ─────────────────────────────────────────────────────────────────────

#[test]
fn tl_ir_span_round_trip_via_dynamic() -> Result<()> {
    use tilelang_rs_core::set_current_span;

    // Verify the set_current_span → empty_span() → node.span round-trip using
    // T::dynamic (which calls ffi::tir::Var(name, dtype, empty_span())).
    let ctx = tilelang_rs_core::BuilderContext::new("span_rt")?;
    ctx.with_tir_prim_func("f", false, |_| {
        set_current_span("span_demo.rs", 77, 8);
        let v = T::dynamic("x", T::INT64);
        // ffi::tir::Var inherits get_span() from ir::BaseExpr via Deref.
        let span = ffi::ir::BaseExpr::from(v).get_span();
        assert_eq!(span.get_source_name().get_name().as_str(), "span_demo.rs");
        assert_eq!(span.get_line(), 77);
        assert_eq!(span.get_column(), 8);
        println!(
            "T::dynamic span → {}:{}:{}  ✓",
            span.get_source_name().get_name().as_str(),
            span.get_line(),
            span.get_column(),
        );
        ffi::script::ir_builder::tir::Evaluate(1i64.into_prim_expr()).expect("Evaluate");
    });
    let _ = ctx.finish_ir_module();

    // Verify that loop variable names appear correctly in the generated TIR.
    // IRBuilderName renames the ForFrame's Var in-place (same mechanism as
    // TVM's Python TVMScript parser) so the For node itself gets the name.
    let module = span_kernel()?;
    let printed = tilelang_rs_core::debug_print(module);
    println!("\nspan_kernel TVMScript:\n{}", printed);

    assert!(
        printed.contains("for i in range(n)") || printed.contains("for i in range(T.int64"),
        "loop var 'i' should appear directly in for-loop, got:\n{printed}"
    );
    assert!(
        !printed.contains("i: T.int64 = v"),
        "LetStmt indirection should be gone with IRBuilderName approach"
    );
    println!("✓  for i in range(n) — no LetStmt indirection");

    Ok(())
}
