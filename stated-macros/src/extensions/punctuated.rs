use std::mem;

use extend::ext;
use itertools::Itertools;
use syn::punctuated::Punctuated;

#[ext(name = PunctuatedExt)]
pub impl<T, P> Punctuated<T, P> {
    /// Calls `op`, passing in `self` as a vector.
    fn call<F, U>(&mut self, op: F) -> U
    where
        P: Default,
        F: FnOnce(&mut Vec<T>) -> U,
    {
        let mut taken = mem::take(self).into_iter().collect_vec();
        let returned = op(&mut taken);
        self.extend(taken);
        returned
    }
}
