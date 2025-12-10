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
    fn get(&self) -> &mut Vec<T> {
        unsafe { &mut *self.vec.get() }
    }

    pub(crate) fn pop(&self) -> Option<T> {
        self.get().pop()
    }

    pub(crate) fn push(&self, v: T) {
        self.get().push(v);
    }

    pub(crate) fn take(&self) -> Vec<T> {
        mem::take(self.get())
    }
}
