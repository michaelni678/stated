use stated::{stated, N, Y};

#[stated(states(A, B, C))]
pub struct Test<#[stated] S>;

#[stated]
impl<#[stated] S> Test<S> {
    #[stated(assign(A, C))]
    pub fn new() -> Test<_> {
        Test
    }
}

fn main() {
    assert!(matches!(Test::new(), Test::<(Y, N, Y)> { .. }));
}
