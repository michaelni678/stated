use stated::stated;

#[stated(states(A))]
pub struct Test<#[stated] #[stated] S>;

#[stated]
impl<#[stated] #[stated] S> Test<S> {}

fn main() {}
