use stated::{stated, N, Y};

#[stated(states(A, B, C), preset(A, B))]
pub struct Test<#[stated] S>;

#[stated]
impl<#[stated] S> Test<S> {
    #[stated]
    pub fn new() -> Test<_> {
        Test
    }

    #[stated(delete(A))]
    pub fn foo(self) -> Test<_> {
        _
    }
}

fn main() {
    assert!(matches!(Test::new().foo(), Test::<(N, Y, N)> { .. }));
}
