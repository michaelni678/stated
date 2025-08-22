use stated::{stated, N};

#[stated(states(A, B, C))]
pub struct Test<#[stated] S>;

#[stated]
impl<#[stated] S> Test<S> {
    #[stated]
    pub fn new() -> Test<_> {
        Test
    }

    #[stated]
    pub fn foo(self) -> Test<_> {
        _
    }
}

fn main() {
    assert!(matches!(Test::new().foo(), Test::<(N, N, N)> { .. }));
}
