#[test]
fn duplicate_case_names() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/duplicate_generated_case_name.rs");
}
