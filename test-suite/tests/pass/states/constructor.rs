use stated::{stated, N};

#[stated(states(A, B, C))]
pub struct Test<#[stated] S>;

#[stated]
impl<#[stated] S> Test<S> {
    #[stated]
    pub fn new() -> Test<_> {
        Test
    }
}

fn main() {
    assert!(matches!(Test::new(), Test::<(N, N, N)> { .. }));
}
