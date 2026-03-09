//! Rust frontend for building TileLang IR modules.
//!
//! # Example
//!
//! ```ignore
//! use tilelang_rs::{debug_print, language as T, tl_ir, Result};
//!
//! #[tl_ir]
//! fn kernel() {
//!     let acc = T::alloc_var(T::int32(), 0i64)?;
//!     for i in T::serial(0i64, 4i64) {
//!         if i < 2i64 {
//!             acc.store(acc.load()? + 1i64)?;
//!         }
//!     }
//! }
//!
//! fn main() -> Result<()> {
//!     let module = kernel()?;
//!     println!("{}", debug_print(module)?);
//!     Ok(())
//! }
//! ```

pub use tilelang_rs_core as core;
pub use tilelang_rs_core::{
    debug_print, language, pred, runtime, BuilderContext, DLDataTypeExt, Expr, ForDsl,
    FromLoopVars, IntoPredExpr, IntoPrimExpr, IntoPrimExprs, LocalVar, PredExpr, Result,
};
pub use tilelang_rs_ffi as ffi;
pub use tilelang_rs_macros::tl_ir;
