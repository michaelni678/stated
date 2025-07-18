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

    /// Finds and removes the first element that matches the given predicate.
    fn find_remove<F>(&mut self, f: F) -> Option<T>
    where
        P: Default,
        F: FnMut(&T) -> bool,
    {
        self.iter()
            .position(f)
            .map(|index| self.call(|this| this.remove(index)))
    }
}
