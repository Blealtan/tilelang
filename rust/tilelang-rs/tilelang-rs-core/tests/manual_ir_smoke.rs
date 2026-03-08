use tilelang_rs_core::{debug_print, ffi, language as T, BuilderContext, FromLoopVars, Result};
use tvm_ffi::{Array, DLDataTypeExt, String as FfiString};

fn make_span() -> Result<ffi::ir::Span> {
    let source = ffi::ir::SourceName(FfiString::from("tilelang_rs_manual_ir_smoke"))?;
    ffi::ir::Span(source, 0, 0, 0, 0)
}

fn prim_expr_i64(value: i64, span: &ffi::ir::Span) -> Result<ffi::ir::PrimExpr> {
    let dtype = tvm_ffi::DLDataType::try_from_str("int64")?;
    let imm = ffi::ir::IntImm(dtype, value, span.clone())?;
    Ok(imm.into())
}

#[test]
fn manual_ir_smoke_prints_valid_module() -> Result<()> {
    let ctx = BuilderContext::new("manual_ir_smoke")?;
    ctx.with_tir_prim_func("add", false, |_prim_func| {
        let span = make_span().expect("make_span should not fail");
        let extent = prim_expr_i64(1024, &span).expect("prim_expr_i64 should not fail");
        let shape = Array::new(vec![extent.clone(), extent.clone()]);
        let float32 = tvm_ffi::DLDataType::try_from_str("float32").expect("float32 dtype");

        let buffer_a = ffi::script::ir_builder::tir::Buffer(
            shape.clone(),
            float32,
            FfiString::from("A"),
            None,
            None,
            None,
            FfiString::from("global"),
            0,
            0,
            FfiString::from(""),
            None,
        )
        .expect("Buffer A construction");
        let buffer_b = ffi::script::ir_builder::tir::Buffer(
            shape.clone(),
            float32,
            FfiString::from("B"),
            None,
            None,
            None,
            FfiString::from("global"),
            0,
            0,
            FfiString::from(""),
            None,
        )
        .expect("Buffer B construction");

        ffi::script::ir_builder::tir::Arg(FfiString::from("a"), buffer_a.clone().into())
            .expect("Arg a");
        ffi::script::ir_builder::tir::Arg(FfiString::from("b"), buffer_b.clone().into())
            .expect("Arg b");

        // Use serial outer loop + parallel inner loop to test manual BufferLoad/Add/BufferStore.
        ctx.for_each(T::serial(0i64, 1024i64), |ctx, outer_vars| {
            let x_var = FromLoopVars::bind1(outer_vars);

            ctx.for_each(T::parallel([1024i64]), |_ctx, inner_vars| {
                let y_var = FromLoopVars::bind1(inner_vars);

                let x_expr: ffi::ir::PrimExpr = x_var.clone().into();
                let y_expr: ffi::ir::PrimExpr = y_var.clone().into();
                let indices = Array::new(vec![x_expr.clone(), y_expr.clone()]);

                let load_a =
                    ffi::tir::BufferLoad(buffer_a.clone(), indices.clone(), None, span.clone())
                        .expect("BufferLoad a");
                let load_b =
                    ffi::tir::BufferLoad(buffer_b.clone(), indices.clone(), None, span.clone())
                        .expect("BufferLoad b");
                let add_expr = ffi::tir::Add(
                    ffi::ir::PrimExpr::from(load_a),
                    ffi::ir::PrimExpr::from(load_b),
                    span.clone(),
                )
                .expect("Add");

                ffi::script::ir_builder::tir::BufferStore(
                    buffer_a.clone(),
                    ffi::ir::PrimExpr::from(add_expr),
                    indices,
                    None,
                )
                .expect("BufferStore");
            });
        });
    });

    let module = ctx.finish_ir_module();
    let printed = debug_print(module.clone());
    println!("{}", printed);

    assert!(printed.contains("from tvm.script import ir as I"));
    assert!(printed.contains("@T.prim_func"));
    assert!(printed.contains("def add("));
    assert!(printed.contains("a: T.Buffer"));

    Ok(())
}
