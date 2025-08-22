use stated::stated;

#[stated(states(A))]
pub struct Test<#[stated] S>;

#[stated]
impl<#[stated] S> Test<S> {
    #[stated(assert(A), assign(A))]
    pub fn foo(self) {}
}

fn main() {}
