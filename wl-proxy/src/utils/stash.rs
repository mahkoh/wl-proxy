use std::{
    cell::Cell,
    mem,
    ops::{Deref, DerefMut},
};

#[cfg(test)]
mod tests;

pub(crate) struct Stash<T> {
    elements: Cell<Vec<T>>,
}

impl<T> Stash<T> {
    pub(crate) fn borrow(&self) -> BorrowedStash<'_, T> {
        BorrowedStash {
            cell: &self.elements,
            elements: self.elements.take(),
        }
    }
}

impl<T> Default for Stash<T> {
    fn default() -> Self {
        Self {
            elements: Default::default(),
        }
    }
}

pub(crate) struct BorrowedStash<'a, T> {
    cell: &'a Cell<Vec<T>>,
    elements: Vec<T>,
}

impl<T> Drop for BorrowedStash<'_, T> {
    fn drop(&mut self) {
        self.elements.clear();
        self.cell.set(mem::take(&mut self.elements));
    }
}

impl<T> Deref for BorrowedStash<'_, T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

impl<T> DerefMut for BorrowedStash<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements
    }
}
