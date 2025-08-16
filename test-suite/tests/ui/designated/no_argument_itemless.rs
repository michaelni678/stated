use stated::stated;

#[stated(states(A))]
pub struct Test<#[stated] S>;

#[stated]
#[rustfmt::skip] // Stops `<>` from getting removed.
impl<#[stated] S> Test<> {}

fn main() {}
