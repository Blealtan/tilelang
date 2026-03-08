use tilelang_rs_macros::tl_ir;

#[tl_ir]
fn ok(x: i32) {
    let y = x;
    let y = y + 1;
    let _z = y;
}

fn main() {}
