use stated::stated;

#[stated(states(A))]
pub struct Test<#[stated] S, #[stated] T>;

#[stated]
impl<#[stated] S, #[stated] T> Test<S, T> {}

fn main() {}
