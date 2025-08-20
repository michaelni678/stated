use trybuild::TestCases;

#[test]
fn ui() {
    let tests = TestCases::new();
    tests.compile_fail("tests/ui/**/*.rs");
}

#[test]
fn states() {
    let tests = TestCases::new();
    tests.pass("tests/states/pass/**/*.rs");
}
