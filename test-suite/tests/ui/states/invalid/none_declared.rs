use stated::stated;

#[stated]
pub struct Test<#[stated] S>;

#[stated]
impl<#[stated] S> Test<S> {}

fn main() {}
