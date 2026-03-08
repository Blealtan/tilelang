use tilelang_rs::{debug_print, ffi, runtime, Result};
use tvm_ffi::DLDataTypeExt;

#[test]
fn public_ffi_surface_can_construct_ir_nodes() -> Result<()> {
    runtime::ensure_runtime_loaded()?;

    let source = ffi::ir::SourceName(tvm_ffi::String::from("tilelang_rs_public_test"))?;
    let span = ffi::ir::Span(source, 0, 0, 0, 0)?;
    let dtype = tvm_ffi::DLDataType::try_from_str("int64")?;
    let imm = ffi::ir::IntImm(dtype, 42, span)?;
    let expr = ffi::ir::PrimExpr::from(imm);
    let printed = debug_print(expr);

    assert!(printed.contains("T.int64(42)"));
    Ok(())
}
