use tilelang_rs::{debug_print, language as T, tl_ir, Result};

#[tl_ir]
fn add_kernel() {
    let acc = T::alloc_var(T::int32(), 0i64)?;

    for i in T::serial(0i64, 4i64) {
        if i < 2i64 {
            acc.store(1i64)?;
        } else {
            acc.store(T::select(false, 2i64, 3i64)?)?;
        }
    }
}

fn main() -> Result<()> {
    let module = add_kernel()?;
    println!("{}", debug_print(module)?);
    Ok(())
}
