#[test]
fn tl_ir_trybuild() {
    let t = trybuild::TestCases::new();
    // precheck: mutability violations
    t.pass("tests/ui/pass_immutable.rs");
    t.compile_fail("tests/ui/let_mut.rs");
    t.compile_fail("tests/ui/mut_param.rs");
    t.compile_fail("tests/ui/mut_pattern.rs");
    t.compile_fail("tests/ui/mut_borrow.rs");
    t.compile_fail("tests/ui/assign.rs");
    // transform: macro expansion compiles
    t.pass("tests/ui/transform_loops_if.rs");
    t.pass("tests/ui/transform_logic_if.rs");
    t.pass("tests/ui/transform_tensor_params.rs");
}
