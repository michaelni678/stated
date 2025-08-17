use stated::stated;

pub trait MyTrait {}

#[stated(states(A))]
pub struct Test<#[stated] S>;

#[stated]
impl<#[stated] S> MyTrait for Test<S> {}

fn main() {}
