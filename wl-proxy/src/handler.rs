//! Helpers for event handlers.
//!
//! These types are similar to [`Ref`] and [`RefMut`] but allow the handler to be replaced
//! while it is borrowed. When this happens, the handler will be replaced before the next
//! event is emitted.

use {
    crate::utils::cold_path::cold_path,
    std::{
        cell::{Cell, Ref, RefCell, RefMut},
        fmt::{Debug, Display, Formatter},
        mem::{self},
        ops::{Deref, DerefMut},
    },
    thiserror::Error,
};

#[cfg(test)]
mod tests;

pub(crate) struct HandlerHolder<T: ?Sized> {
    handler: RefCell<Option<Box<T>>>,
    needs_update: Cell<bool>,
    new: Cell<Option<Option<Box<T>>>>,
}

trait HandlerHolderDyn {
    fn update(&self);
}

#[derive(Clone)]
struct HandlerHolderUpdate<'a> {
    needs_update: &'a Cell<bool>,
    holder: &'a dyn HandlerHolderDyn,
}

/// A mutable reference to a handler.
pub struct HandlerMut<'a, U: ?Sized> {
    handler: RefMut<'a, U>,
    update: HandlerHolderUpdate<'a>,
}

/// A shared reference to a handler.
pub struct HandlerRef<'a, U: ?Sized> {
    handler: Ref<'a, U>,
    update: HandlerHolderUpdate<'a>,
}

/// An error returned when trying to access a handler.
#[derive(Debug, Error)]
pub enum HandlerAccessError {
    /// The handler is already borrowed.
    #[error("the handler is already borrowed")]
    AlreadyBorrowed,
    /// The object has no handler.
    #[error("the object has no handler")]
    NoHandler,
    /// The handler has a different type.
    #[error("the handler has a different type")]
    InvalidType,
}

impl<T: ?Sized> HandlerHolderDyn for HandlerHolder<T> {
    fn update(&self) {
        let _old;
        if let Ok(mut handler) = self.handler.try_borrow_mut() {
            if let Some(new) = self.new.take() {
                _old = mem::replace(&mut *handler, new);
            }
            self.needs_update.set(false);
        }
    }
}

impl<T: ?Sized> Default for HandlerHolder<T> {
    fn default() -> Self {
        Self {
            handler: Default::default(),
            needs_update: Default::default(),
            new: Default::default(),
        }
    }
}

impl<T: ?Sized> HandlerHolder<T> {
    #[inline]
    fn update(&self) -> HandlerHolderUpdate<'_> {
        HandlerHolderUpdate {
            needs_update: &self.needs_update,
            holder: self,
        }
    }

    #[inline]
    pub(crate) fn borrow_mut(&self) -> HandlerMut<'_, Option<Box<T>>> {
        HandlerMut {
            handler: self.handler.borrow_mut(),
            update: self.update(),
        }
    }

    #[inline]
    pub(crate) fn try_borrow(&self) -> Option<HandlerRef<'_, Option<Box<T>>>> {
        Some(HandlerRef {
            handler: self.handler.try_borrow().ok()?,
            update: self.update(),
        })
    }

    #[inline]
    pub(crate) fn try_borrow_mut(&self) -> Option<HandlerMut<'_, Option<Box<T>>>> {
        Some(HandlerMut {
            handler: self.handler.try_borrow_mut().ok()?,
            update: self.update(),
        })
    }

    pub(crate) fn set(&self, handler: Option<Box<T>>) {
        let _prev;
        if let Ok(mut cell) = self.handler.try_borrow_mut() {
            _prev = mem::replace(&mut *cell, handler);
        } else {
            cold_path();
            self.new.set(Some(handler));
            self.needs_update.set(true);
        }
    }
}

impl<T: ?Sized> Deref for HandlerRef<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.handler
    }
}

impl<T: ?Sized> Deref for HandlerMut<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.handler
    }
}

impl<T: ?Sized> DerefMut for HandlerMut<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.handler
    }
}

impl<T: ?Sized> HandlerRef<'_, T> {
    /// Copies a `HandlerRef`.
    ///
    /// This is an associated function that needs to be used as `HandlerRef::clone(...)`.
    #[expect(clippy::should_implement_trait)]
    pub fn clone(orig: &Self) -> Self {
        Self {
            handler: Ref::clone(&orig.handler),
            update: orig.update.clone(),
        }
    }
}

impl<'a, T: ?Sized> HandlerRef<'a, T> {
    /// Makes a new `HandlerRef` for a component of the borrowed data.
    ///
    /// This is an associated function that needs to be used as `HandlerRef::map(...)`.
    /// A method would interfere with methods of the same name on the contents
    /// of a handler used through `Deref`.
    #[inline]
    pub fn map<U: ?Sized, F>(orig: Self, f: F) -> HandlerRef<'a, U>
    where
        F: FnOnce(&T) -> &U,
    {
        HandlerRef {
            handler: Ref::map(orig.handler, f),
            update: orig.update,
        }
    }
}

impl<'a, T: ?Sized> HandlerMut<'a, T> {
    /// Makes a new `HandlerMut` for a component of the borrowed data.
    ///
    /// This is an associated function that needs to be used as `HandlerMut::map(...)`.
    /// A method would interfere with methods of the same name on the contents
    /// of a handler used through `Deref`.
    #[inline]
    pub fn map<U: ?Sized, F>(orig: Self, f: F) -> HandlerMut<'a, U>
    where
        F: FnOnce(&mut T) -> &mut U,
    {
        HandlerMut {
            handler: RefMut::map(orig.handler, f),
            update: orig.update,
        }
    }
}

impl Drop for HandlerHolderUpdate<'_> {
    #[inline]
    fn drop(&mut self) {
        if self.needs_update.get() {
            cold_path();
            self.holder.update();
        }
    }
}

impl<T: ?Sized> Debug for HandlerRef<'_, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.handler.fmt(f)
    }
}

impl<T: ?Sized> Debug for HandlerMut<'_, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.handler.fmt(f)
    }
}

impl<T: ?Sized> Display for HandlerRef<'_, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.handler.fmt(f)
    }
}

impl<T: ?Sized> Display for HandlerMut<'_, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.handler.fmt(f)
    }
}
