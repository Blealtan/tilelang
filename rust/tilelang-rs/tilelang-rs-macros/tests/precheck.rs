#[test]
fn tl_ir_precheck_reports_mutability_violations() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/pass_immutable.rs");
    t.compile_fail("tests/ui/let_mut.rs");
    t.compile_fail("tests/ui/mut_param.rs");
    t.compile_fail("tests/ui/mut_pattern.rs");
    t.compile_fail("tests/ui/mut_borrow.rs");
    t.compile_fail("tests/ui/assign.rs");
}
