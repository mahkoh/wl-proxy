use std::{
    cell::{Cell, UnsafeCell},
    mem,
};

#[cfg(test)]
mod tests;

pub(crate) struct Stack<T> {
    vec: UnsafeCell<Vec<T>>,
    borrowed: Cell<bool>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self {
            vec: Default::default(),
            borrowed: Default::default(),
        }
    }
}

impl<T> Stack<T> {
    #[inline(always)]
    fn with<U>(&self, f: impl FnOnce(&mut Vec<T>) -> U) -> U {
        if self.borrowed.replace(true) {
            std::process::abort();
        }
        // SAFETY: The borrowed flag ensures that there is only ever one reference.
        let res = f(unsafe { &mut *self.vec.get() });
        self.borrowed.set(false);
        res
    }

    #[inline(always)]
    pub(crate) fn pop(&self) -> Option<T> {
        self.with(|vec| vec.pop())
    }

    #[inline(always)]
    pub(crate) fn push(&self, v: T) {
        self.with(|vec| vec.push(v))
    }

    pub(crate) fn take(&self) -> Vec<T> {
        self.with(mem::take)
    }
}
