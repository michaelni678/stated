use stated::stated;

#[stated(states(A))]
pub struct Test<#[stated] S, T> {
    pub t: T,
}

#[stated]
impl<#[stated] S, T> Test<T> {}

fn main() {}
