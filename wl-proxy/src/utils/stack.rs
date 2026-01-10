use std::{cell::UnsafeCell, mem};

pub(crate) struct Stack<T> {
    vec: UnsafeCell<Vec<T>>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self {
            vec: Default::default(),
        }
    }
}

impl<T> Stack<T> {
    pub(crate) fn pop(&self) -> Option<T> {
        // SAFETY: This is a leaf function.
        let vec = unsafe { &mut *self.vec.get() };
        vec.pop()
    }

    pub(crate) fn push(&self, v: T) {
        // SAFETY: The allocator invoked by Vec::push could theoretically call back into
        //         this object but it's not something we worry about here.
        //         We could do a `mem::replace` dance if capacity == len but then we'd
        //         still have to rely on the allocator only being invoked in those
        //         situations, which is undocumented.
        let vec = unsafe { &mut *self.vec.get() };
        vec.push(v);
    }

    pub(crate) fn take(&self) -> Vec<T> {
        // SAFETY: This is a a leaf function.
        let vec = unsafe { &mut *self.vec.get() };
        mem::take(vec)
    }
}
