use tilelang_rs::{debug_print, language as T, tl_ir, Expr, Result};

/// 1-D element-wise add: C[i] = A[i] + B[i]
///
/// Demonstrates the Phase-3/4 `#[tl_ir]` API:
/// - `T::Tensor` parameters are stripped from the signature by the macro; each
///   placeholder is available inside the body as `Tensor::new()`.
/// - `named_let`: `let a = a.declare(...)` automatically injects the variable
///   name "a" into the IR buffer declaration.
/// - `T::kernel(grid).threads(n)` compiles to a `tl.KernelLaunch` node.
/// - `T::parallel(extent)` compiles to a `tl.Parallel` loop.
/// - Arithmetic on `Expr` values uses standard Rust operator syntax.
#[tl_ir]
fn vector_add(a: T::Tensor, b: T::Tensor, c: T::Tensor) {
    let n = T::dynamic("n", T::INT64);
    let block_n = 2048i64;

    let a = a.declare(&n, T::FLOAT32);
    let b = b.declare(&n, T::FLOAT32);
    let c = c.declare(&n, T::FLOAT32);

    for bx in T::kernel(T::ceildiv(&n, block_n)).threads(128i64) {
        let start_x = Expr::from(&bx) * block_n;
        for ix in T::parallel(block_n) {
            let x = start_x + &ix;
            c.store_at(&x, a.load_at(&x) + b.load_at(&x));
        }
    }
}

fn main() -> Result<()> {
    let module = vector_add()?;
    println!("{}", debug_print(module));
    Ok(())
}
