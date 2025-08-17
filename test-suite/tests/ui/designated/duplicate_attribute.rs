use stated::stated;

#[stated(states(A))]
#[rustfmt::skip] // Stops the attributes from going on different lines.
pub struct Test<#[stated] #[stated] S>;

#[stated]
#[rustfmt::skip] // Stops the attributes from going on different lines.
impl<#[stated] #[stated] S> Test<S> {}

fn main() {}
