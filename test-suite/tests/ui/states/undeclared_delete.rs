use stated::stated;

#[stated(states(A))]
pub struct Test<#[stated] S>;

#[stated]
impl<#[stated] S> Test<S> {
    #[stated(delete(B))]
    pub fn foo(self) {}
}

fn main() {}
