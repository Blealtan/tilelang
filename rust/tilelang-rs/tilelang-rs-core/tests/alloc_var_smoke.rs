use tilelang_rs_core::{BuilderContext, Result, debug_print, ffi, language as T, pred};

#[test]
fn alloc_var_smoke_prints_local_var_updates() -> Result<()> {
    let ctx = BuilderContext::new("alloc_var_smoke")?;
    ctx.with_tir_prim_func("alloc_var_if", false, |_prim_func| {
        let acc = T::alloc_var(T::INT32, 0i64);
        let cond = pred::lt(1i64, 2i64);

        ctx.if_stmt(
            cond,
            || acc.store(1i64),
            Some(|| {
                let merged = T::select(false, 2i64, 3i64);
                acc.store(merged);
            }),
        );

        ffi::script::ir_builder::tir::Evaluate(acc.load()).expect("Evaluate should not fail");
    });

    let module = ctx.finish_ir_module();
    let printed = debug_print(module);
    println!("{}", printed);

    assert!(printed.contains("def alloc_var_if("));
    assert!(printed.contains("local.var"));
    assert!(printed.contains("if "));

    Ok(())
}
