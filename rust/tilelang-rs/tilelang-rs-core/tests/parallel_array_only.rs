#[test]
fn parallel_rejects_variadic_args() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/parallel_variadic.rs");
}
