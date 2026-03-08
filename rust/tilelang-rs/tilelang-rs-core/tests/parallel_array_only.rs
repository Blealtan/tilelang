#[test]
fn parallel_requires_array_input() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/parallel_tuple.rs");
    t.compile_fail("tests/ui/parallel_variadic.rs");
}
