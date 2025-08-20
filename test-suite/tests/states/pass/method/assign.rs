use stated::{N, Y, stated};

#[stated(states(A, B, C))]
pub struct Test<#[stated] S>;

#[stated]
impl<#[stated] S> Test<S> {
    #[stated]
    pub fn new() -> Test<_> {
        Test
    }

    #[stated(assign(A))]
    pub fn foo(self) -> Test<_> {
        _
    }
}

fn main() {
    assert!(matches!(Test::new().foo(), Test::<(Y, N, N)> { .. }));
}
