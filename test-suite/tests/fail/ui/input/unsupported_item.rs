use stated::stated;

#[stated(states(A))]
pub enum Test<#[stated] S> {}

#[stated]
impl<#[stated] S> Test<S> {}

fn main() {}
