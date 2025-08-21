use stated::stated;

#[stated(states(A), preset(A, A))]
pub struct Test<#[stated] S>;

#[stated]
impl<#[stated] S> Test<S> {}

fn main() {}
