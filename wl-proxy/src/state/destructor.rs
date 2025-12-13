use {
    crate::state::State,
    std::{
        cell::Cell,
        os::fd::OwnedFd,
        rc::Rc,
        sync::{
            Arc,
            atomic::{
                AtomicBool,
                Ordering::{Relaxed, Release},
            },
        },
    },
};

/// A destructor for a [`State`].
///
/// Dropping this object might destroy the state if the destructor is enabled.
///
/// This object can be constructed with [`State::create_destructor`].
pub struct Destructor {
    pub(super) state: Rc<State>,
    pub(super) enabled: Cell<bool>,
}

/// A remote destructor for a [`State`].
///
/// This type serves the same purpose as [`Destructor`] but also implements `Send+Sync`.
///
/// This object can be constructed with [`State::create_remote_destructor`].
pub struct RemoteDestructor {
    pub(super) destroy: Arc<AtomicBool>,
    pub(super) _fd: OwnedFd,
    pub(super) enabled: AtomicBool,
}

impl Destructor {
    /// Returns the underlying state.
    pub fn state(&self) -> &Rc<State> {
        &self.state
    }

    /// Returns whether this destructor is currently enabled.
    ///
    /// If the destructor is enabled when it is dropped, the underlying state is
    /// destroyed.
    pub fn enabled(&self) -> bool {
        self.enabled.get()
    }

    /// Enables this destructor.
    ///
    /// This is the default.
    pub fn enable(&self) {
        self.enabled.set(true);
    }

    /// Disables this destructor.
    pub fn disable(&self) {
        self.enabled.set(false);
    }
}

impl Drop for Destructor {
    fn drop(&mut self) {
        if self.enabled.get() {
            self.state.destroy();
        }
    }
}

impl RemoteDestructor {
    /// Returns whether this destructor is currently enabled.
    ///
    /// If the destructor is enabled when it is dropped, the underlying state is
    /// destroyed.
    pub fn enabled(&self) -> bool {
        self.enabled.load(Relaxed)
    }

    /// Enables this destructor.
    ///
    /// This is the default.
    pub fn enable(&self) {
        self.enabled.store(true, Relaxed);
    }

    /// Disables this destructor.
    pub fn disable(&self) {
        self.enabled.store(false, Relaxed);
    }
}

impl Drop for RemoteDestructor {
    fn drop(&mut self) {
        if self.enabled.load(Relaxed) {
            self.destroy.store(true, Release);
        }
    }
}
