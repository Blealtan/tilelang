use tilelang_rs_macros::tl_ir;

#[tl_ir]
fn bad() {
    let x = 1;
    let _r = &mut { x };
}

fn main() {}
