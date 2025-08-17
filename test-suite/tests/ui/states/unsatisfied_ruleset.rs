use stated::stated;

#[stated(states(A))]
pub struct Test<#[stated] S>;

#[stated]
impl<#[stated] S> Test<S> {
    #[stated]
    pub fn new() -> Test<_> {
        Test
    }

    #[stated(assert(A))]
    pub fn foo(self) {}
}

fn main() {
    Test::new().foo();
}
