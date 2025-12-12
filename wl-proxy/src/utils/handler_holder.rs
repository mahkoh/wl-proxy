use std::{
    cell::{Cell, RefCell, RefMut},
    mem,
    ops::{Deref, DerefMut},
};

pub(crate) struct HandlerHolder<T: ?Sized> {
    // TODO: make private
    pub(crate) handler: RefCell<Option<Box<T>>>,
    new: Cell<Option<Option<Box<T>>>>,
}

pub(crate) struct HandlerHolderBorrow<'a, T: ?Sized> {
    handler: RefMut<'a, Option<Box<T>>>,
    new: &'a Cell<Option<Option<Box<T>>>>,
}

impl<T: ?Sized> Default for HandlerHolder<T> {
    fn default() -> Self {
        Self {
            handler: Default::default(),
            new: Default::default(),
        }
    }
}

impl<T: ?Sized> HandlerHolder<T> {
    pub(crate) fn borrow(&self) -> HandlerHolderBorrow<'_, T> {
        HandlerHolderBorrow {
            handler: self.handler.borrow_mut(),
            new: &self.new,
        }
    }

    pub(crate) fn try_borrow(&self) -> Option<HandlerHolderBorrow<'_, T>> {
        Some(HandlerHolderBorrow {
            handler: self.handler.try_borrow_mut().ok()?,
            new: &self.new,
        })
    }

    pub(crate) fn set(&self, handler: Option<Box<T>>) {
        let _prev;
        if let Ok(mut cell) = self.handler.try_borrow_mut() {
            _prev = mem::replace(&mut *cell, handler);
        } else {
            self.new.set(Some(handler));
        }
    }
}

impl<T: ?Sized> Deref for HandlerHolderBorrow<'_, T> {
    type Target = Option<Box<T>>;

    fn deref(&self) -> &Self::Target {
        &self.handler
    }
}

impl<T: ?Sized> DerefMut for HandlerHolderBorrow<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.handler
    }
}

impl<T: ?Sized> Drop for HandlerHolderBorrow<'_, T> {
    fn drop(&mut self) {
        if let Some(new) = self.new.take() {
            *self.handler = new;
        }
    }
}
