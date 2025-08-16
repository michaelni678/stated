use stated::stated;

#[stated(states(A))]
pub struct Test<#[stated] S>;

#[stated]
impl<#[stated] S> Test<> {
    #[stated]
    fn foo(self) {}
}

fn main() {}
