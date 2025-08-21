use trybuild::TestCases;

#[test]
fn tests() {
    let tests = TestCases::new();

    tests.compile_fail("tests/fail/**/*.rs");
    tests.pass("tests/pass/**/*.rs");
}
