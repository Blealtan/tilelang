use tilelang_rs_core::{debug_print, ffi, BuilderContext, Result};
use tvm_ffi::{AnyValue, Array, DLDataTypeExt, Map, String as FfiString};

fn make_span() -> Result<ffi::ir::Span> {
    let source = ffi::ir::SourceName(FfiString::from("tilelang_rs_manual_ir_smoke"))?;
    ffi::ir::Span(source, 0, 0, 0, 0)
}

fn int_imm(value: i64, span: &ffi::ir::Span) -> Result<ffi::ir::IntImm> {
    let dtype = tvm_ffi::DLDataType::try_from_str("int64")?;
    ffi::ir::IntImm(dtype, value, span.clone())
}

fn prim_expr_from_int(value: i64, span: &ffi::ir::Span) -> Result<ffi::ir::PrimExpr> {
    let imm = int_imm(value, span)?;
    Ok(ffi::ir::PrimExpr::from_object(imm.as_object_ref().clone()))
}

#[test]
fn manual_ir_smoke_prints_valid_module() -> Result<()> {
    let ctx = BuilderContext::new("manual_ir_smoke")?;
    ctx.with_tir_prim_func("add", false, |_prim_func| {
        let span = make_span()?;
        let extent = prim_expr_from_int(1024, &span)?;
        let shape = Array::new(vec![extent.clone(), extent.clone()]);
        let float32 = tvm_ffi::DLDataType::try_from_str("float32")?;

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
        )?;
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
        )?;

        ffi::script::ir_builder::tir::Arg(FfiString::from("a"), buffer_a.clone().into())?;
        ffi::script::ir_builder::tir::Arg(FfiString::from("b"), buffer_b.clone().into())?;

        let grid = Array::new(vec![extent.clone()]);
        let threads = Array::new(vec![
            prim_expr_from_int(128, &span)?,
            prim_expr_from_int(1, &span)?,
            prim_expr_from_int(1, &span)?,
        ]);
        let empty_attrs: Map<FfiString, AnyValue> = Map::new(Vec::new())?;
        let kernel_frame = ffi::tl::KernelLaunch(grid, Some(threads), empty_attrs)?;
        let kernel_base = ffi::script::ir_builder::IRBuilderFrame::from_object(
            kernel_frame.as_object_ref().clone(),
        );

        ctx.with_frame(kernel_base, || {
            let frames_any = kernel_frame.frames()?;
            let frames: Array<ffi::script::ir_builder::tir::TIRFrame> = frames_any.try_into()?;
            let block_frame = frames.get(0)?;
            let launch_frame = ffi::script::ir_builder::tir::LaunchThreadFrame::from_object(
                block_frame.as_object_ref().clone(),
            );
            let iter_any = launch_frame.iter_var()?;
            let iter_var: ffi::tir::IterVar = iter_any.try_into()?;
            let x_var = iter_var.var()?;

            let parallel_extents = Array::new(vec![extent.clone()]);
            let empty_loop_attrs: Map<FfiString, AnyValue> = Map::new(Vec::new())?;
            let parallel_frame = ffi::tl::Parallel(parallel_extents, empty_loop_attrs)?;
            let parallel_base = ffi::script::ir_builder::IRBuilderFrame::from_object(
                parallel_frame.as_object_ref().clone(),
            );

            ctx.with_frame(parallel_base, || {
                let y_vars = parallel_frame.vars()?;
                let y_var = y_vars.get(0)?;

                let x_expr = ffi::ir::PrimExpr::from_object(x_var.as_object_ref().clone());
                let y_expr = ffi::ir::PrimExpr::from_object(y_var.as_object_ref().clone());
                let indices = Array::new(vec![x_expr.clone(), y_expr.clone()]);

                let load_a =
                    ffi::tir::BufferLoad(buffer_a.clone(), indices.clone(), None, span.clone())?;
                let load_b =
                    ffi::tir::BufferLoad(buffer_b.clone(), indices.clone(), None, span.clone())?;
                let add_expr = ffi::tir::Add(
                    ffi::ir::PrimExpr::from_object(load_a.as_object_ref().clone()),
                    ffi::ir::PrimExpr::from_object(load_b.as_object_ref().clone()),
                    span.clone(),
                )?;
                let value_expr = ffi::ir::PrimExpr::from_object(add_expr.as_object_ref().clone());

                ffi::script::ir_builder::tir::BufferStore(
                    buffer_a.clone(),
                    value_expr,
                    indices,
                    None,
                )?;
                Ok(())
            })
        })
    })?;

    let module = ctx.finish_ir_module()?;
    let printed = debug_print(module.clone())?;
    println!("{}", printed);

    assert!(printed.contains("from tvm.script import ir as I"));
    assert!(printed.contains("@T.prim_func"));
    assert!(printed.contains("def add("));
    assert!(printed.contains("a: T.Buffer"));

    Ok(())
}
