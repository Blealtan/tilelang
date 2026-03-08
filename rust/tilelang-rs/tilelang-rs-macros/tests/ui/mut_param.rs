use tilelang_rs_macros::tl_ir;

#[tl_ir]
fn bad(mut x: i32) {
    let _ = x;
}

fn main() {}
