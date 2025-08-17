use stated::stated;

#[stated(states(A))]
pub struct Test<#[stated] S>;

#[stated]
impl<#[stated] S> Test<S> {
    #[stated(reject(A, A))]
    pub fn foo(self) {}
}

fn main() {}
