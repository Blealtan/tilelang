use tilelang_rs_macros::tl_ir;

#[tl_ir]
fn bad(x: i32) {
    let (mut a, b) = (x, 1);
    let _ = (a, b);
}

fn main() {}
