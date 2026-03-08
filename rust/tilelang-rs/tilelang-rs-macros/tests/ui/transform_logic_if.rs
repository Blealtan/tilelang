use tilelang_rs_core::language as T;
use tilelang_rs_macros::tl_ir;

// Target expansion shape:
// let __cond = pred::and(pred::lt(1, 2), || { pred::or(pred::not(false), || { pred::eq(3, 3) }) });
// __ctx.if_stmt(__cond, || { ...; }, Some(|| { ...; }));

#[tl_ir]
fn kernel() {
    if (1i64 < 2i64) && (!false || 3i64 == 3i64) {
        let _a = T::select(true, 7i64, 9i64);
    } else {
        let _b = T::select(false, 1i64, 2i64);
    }
}

fn main() {}
