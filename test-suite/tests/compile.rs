use trybuild::TestCases;

#[test]
fn ui() {
    let tests = TestCases::new();
    tests.compile_fail("tests/ui/**/*.rs");
}
