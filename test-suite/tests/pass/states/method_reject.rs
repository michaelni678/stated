use stated::stated;

#[stated(states(A, B, C))]
pub struct Test<#[stated] S>;

#[stated]
impl<#[stated] S> Test<S> {
    #[stated]
    pub fn new() -> Test<_> {
        Test
    }

    #[stated(reject(A, C))]
    pub fn foo(self) {}
}

fn main() {
    Test::new().foo();
}
