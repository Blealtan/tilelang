use tilelang_rs_core::language as T;
use tilelang_rs_macros::tl_ir;

// Target expansion shape:
// let __ctx = BuilderContext::new("kernel")?;
// __ctx.for_each(T::serial(0, 4), |__ctx, __vars| {
//     let i = FromLoopVars::bind1(__vars);
//     __ctx.if_stmt(pred::lt(i, 2), || { ... }, Some(|| { ... }));
// });
// __ctx.for_each(T::parallel([2, 3]), |__ctx, __vars| { let (i, j) = FromLoopVars::bind2(__vars); ...; });
// __ctx.for_each(T::pipelined(0, 2), |__ctx, __vars| { let i = FromLoopVars::bind1(__vars); ...; });
// Ok(__ctx.finish_ir_module())

#[tl_ir]
fn kernel() {
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

fn main() {}
