#![allow(unused_imports)]
use tilelang_rs_core::{language as T, Tensor};
use tilelang_rs_macros::tl_ir;

// Target expansion shape for Tensor params:
//
// fn kernel(/* a: T::Tensor, b: T::Tensor, c: T::Tensor stripped */) -> Result<IRModule> {
//     let __ctx = BuilderContext::new("kernel")?;
//     __ctx.with_tir_prim_func("kernel", false, |_prim_func| {
//         let a = ::tilelang_rs_core::Tensor::new();
//         let b = ::tilelang_rs_core::Tensor::new();
//         let c = ::tilelang_rs_core::Tensor::new();
//         // user body (let-declares via named_let transform)...
//     });
//     Ok(__ctx.finish_ir_module())
// }

/// Using `T::Tensor` (path with `T` alias) for parameter type annotation.
#[tl_ir]
fn kernel_t_tensor(a: T::Tensor, b: T::Tensor, c: T::Tensor) {
    let n = T::dynamic("n", T::INT64);
    let a = a.declare(&n, T::FLOAT32);
    let b = b.declare(&n, T::FLOAT32);
    let c = c.declare(&n, T::FLOAT32);
    for i in T::serial(0i64, &n) {
        c.store_at(&i, a.load_at(&i) + b.load_at(&i));
    }
}

/// Using bare `Tensor` for parameter type annotation.
///
/// Loop variables are now `Expr` directly — no `Expr::from` needed.
#[tl_ir]
fn kernel_bare_tensor(a: Tensor, b: Tensor, c: Tensor) {
    let n = T::dynamic("n", T::INT64);
    let a = a.declare(&n, T::FLOAT32);
    let b = b.declare(&n, T::FLOAT32);
    let c = c.declare(&n, T::FLOAT32);
    for i in T::serial(0i64, &n) {
        c.store_at(&i, a.load_at(&i) + b.load_at(&i));
    }
}

/// Mixed: some Tensor params, some regular params (regular params remain in signature).
#[tl_ir]
fn kernel_mixed(a: T::Tensor, scale: i64) {
    let n = T::dynamic("n", T::INT64);
    let a = a.declare(&n, T::FLOAT32);
    let _ = scale;
    for i in T::serial(0i64, &n) {
        let _v = a.load_at(&i);
    }
}

fn main() {}
