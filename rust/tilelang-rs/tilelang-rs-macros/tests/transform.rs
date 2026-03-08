#[test]
fn tl_ir_transforms_loops_and_ifs() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/transform_loops_if.rs");
    t.pass("tests/ui/transform_logic_if.rs");
}
