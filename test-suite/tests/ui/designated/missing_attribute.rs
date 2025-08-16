use stated::stated;

#[stated(states(A))]
pub struct Test<S>;

#[stated]
impl<S> Test<S> {}

fn main() {}
