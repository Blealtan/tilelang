use tilelang_rs_macros::tl_ir;

#[tl_ir]
fn bad() {
    let x = 1;
    x = 2;
}

fn main() {}
