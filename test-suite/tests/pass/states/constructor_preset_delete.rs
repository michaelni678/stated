use stated::{N, Y, stated};

#[stated(states(A, B, C), preset(A, B))]
pub struct Test<#[stated] S>;

#[stated]
impl<#[stated] S> Test<S> {
    #[stated(delete(B))]
    pub fn new() -> Test<_> {
        Test
    }
}

fn main() {
    assert!(matches!(Test::new(), Test::<(Y, N, N)> { .. }));
}
