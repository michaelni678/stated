use stated::stated;

#[stated(states(A))]
pub struct Test<#[stated] S>;

#[stated]
impl<#[stated] S> Test<S> {
    #[stated(assert(A), unsupported)]
    pub fn foo(self) {}
}

fn main() {}
