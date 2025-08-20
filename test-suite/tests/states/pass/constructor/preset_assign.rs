use stated::{N, Y, stated};

#[stated(states(A, B, C), preset(B))]
pub struct Test<#[stated] S>;

#[stated]
impl<#[stated] S> Test<S> {
    #[stated(assign(A))]
    pub fn new() -> Test<_> {
        Test
    }
}

fn main() {
    assert!(matches!(Test::new(), Test::<(Y, Y, N)> { .. }));
}
