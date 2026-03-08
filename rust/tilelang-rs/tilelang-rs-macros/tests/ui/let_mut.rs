use tilelang_rs_macros::tl_ir;

#[tl_ir]
fn bad() {
    let mut acc = 0;
    let _ = acc;
}

fn main() {}
