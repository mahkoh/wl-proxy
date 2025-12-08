//! sub-surface interface to a wl_surface
//!
//! An additional interface to a wl_surface object, which has been
//! made a sub-surface. A sub-surface has one parent surface. A
//! sub-surface's size and position are not limited to that of the parent.
//! Particularly, a sub-surface is not automatically clipped to its
//! parent's area.
//!
//! A sub-surface becomes mapped, when a non-NULL wl_buffer is applied
//! and the parent surface is mapped. The order of which one happens
//! first is irrelevant. A sub-surface is hidden if the parent becomes
//! hidden, or if a NULL wl_buffer is applied. These rules apply
//! recursively through the tree of surfaces.
//!
//! The behaviour of a wl_surface.commit request on a sub-surface
//! depends on the sub-surface's mode. The possible modes are
//! synchronized and desynchronized, see methods
//! wl_subsurface.set_sync and wl_subsurface.set_desync. Synchronized
//! mode caches the wl_surface state to be applied when the parent's
//! state gets applied, and desynchronized mode applies the pending
//! wl_surface state directly. A sub-surface is initially in the
//! synchronized mode.
//!
//! Sub-surfaces also have another kind of state, which is managed by
//! wl_subsurface requests, as opposed to wl_surface requests. This
//! state includes the sub-surface position relative to the parent
//! surface (wl_subsurface.set_position), and the stacking order of
//! the parent and its sub-surfaces (wl_subsurface.place_above and
//! .place_below). This state is applied when the parent surface's
//! wl_surface state is applied, regardless of the sub-surface's mode.
//! As the exception, set_sync and set_desync are effective immediately.
//!
//! The main surface can be thought to be always in desynchronized mode,
//! since it does not have a parent in the sub-surfaces sense.
//!
//! Even if a sub-surface is in desynchronized mode, it will behave as
//! in synchronized mode, if its parent surface behaves as in
//! synchronized mode. This rule is applied recursively throughout the
//! tree of surfaces. This means, that one can set a sub-surface into
//! synchronized mode, and then assume that all its child and grand-child
//! sub-surfaces are synchronized, too, without explicitly setting them.
//!
//! Destroying a sub-surface takes effect immediately. If you need to
//! synchronize the removal of a sub-surface to the parent surface update,
//! unmap the sub-surface first by attaching a NULL wl_buffer, update parent,
//! and then destroy the sub-surface.
//!
//! If the parent wl_surface object is destroyed, the sub-surface is
//! unmapped.
//!
//! A sub-surface never has the keyboard focus of any seat.
//!
//! The wl_surface.offset request is ignored: clients must use set_position
//! instead to move the sub-surface.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wl_subsurface proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWlSubsurface {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWlSubsurfaceMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWlSubsurfaceMessageHandler for DefaultMessageHandler { }

impl MetaWlSubsurface {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWlSubsurface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWlSubsurface")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWlSubsurface {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// remove sub-surface interface
    ///
    /// The sub-surface interface is removed from the wl_surface object
    /// that was turned into a sub-surface with a
    /// wl_subcompositor.get_subsurface request. The wl_surface's association
    /// to the parent is deleted. The wl_surface is unmapped immediately.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
        ]);
        Ok(())
    }

    /// Since when the set_position message is available.
    #[allow(dead_code)]
    pub const MSG__SET_POSITION__SINCE: u32 = 1;

    /// reposition the sub-surface
    ///
    /// This schedules a sub-surface position change.
    /// The sub-surface will be moved so that its origin (top left
    /// corner pixel) will be at the location x, y of the parent surface
    /// coordinate system. The coordinates are not restricted to the parent
    /// surface area. Negative values are allowed.
    ///
    /// The scheduled coordinates will take effect whenever the state of the
    /// parent surface is applied.
    ///
    /// If more than one set_position request is invoked by the client before
    /// the commit of the parent surface, the position of a new request always
    /// replaces the scheduled position from any previous request.
    ///
    /// The initial position is 0, 0.
    ///
    /// # Arguments
    ///
    /// - `x`: x coordinate in the parent surface
    /// - `y`: y coordinate in the parent surface
    #[inline]
    pub fn send_set_position(
        &self,
        x: i32,
        y: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            x,
            y,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// Since when the place_above message is available.
    #[allow(dead_code)]
    pub const MSG__PLACE_ABOVE__SINCE: u32 = 1;

    /// restack the sub-surface
    ///
    /// This sub-surface is taken from the stack, and put back just
    /// above the reference surface, changing the z-order of the sub-surfaces.
    /// The reference surface must be one of the sibling surfaces, or the
    /// parent surface. Using any other surface, including this sub-surface,
    /// will cause a protocol error.
    ///
    /// The z-order is double-buffered. Requests are handled in order and
    /// applied immediately to a pending state. The final pending state is
    /// copied to the active state the next time the state of the parent
    /// surface is applied.
    ///
    /// A new sub-surface is initially added as the top-most in the stack
    /// of its siblings and parent.
    ///
    /// # Arguments
    ///
    /// - `sibling`: the reference surface
    #[inline]
    pub fn send_place_above(
        &self,
        sibling: &Rc<MetaWlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            sibling,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg0 = match arg0.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the place_below message is available.
    #[allow(dead_code)]
    pub const MSG__PLACE_BELOW__SINCE: u32 = 1;

    /// restack the sub-surface
    ///
    /// The sub-surface is placed just below the reference surface.
    /// See wl_subsurface.place_above.
    ///
    /// # Arguments
    ///
    /// - `sibling`: the reference surface
    #[inline]
    pub fn send_place_below(
        &self,
        sibling: &Rc<MetaWlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            sibling,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg0 = match arg0.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            3,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the set_sync message is available.
    #[allow(dead_code)]
    pub const MSG__SET_SYNC__SINCE: u32 = 1;

    /// set sub-surface to synchronized mode
    ///
    /// Change the commit behaviour of the sub-surface to synchronized
    /// mode, also described as the parent dependent mode.
    ///
    /// In synchronized mode, wl_surface.commit on a sub-surface will
    /// accumulate the committed state in a cache, but the state will
    /// not be applied and hence will not change the compositor output.
    /// The cached state is applied to the sub-surface immediately after
    /// the parent surface's state is applied. This ensures atomic
    /// updates of the parent and all its synchronized sub-surfaces.
    /// Applying the cached state will invalidate the cache, so further
    /// parent surface commits do not (re-)apply old state.
    ///
    /// See wl_subsurface for the recursive effect of this mode.
    #[inline]
    pub fn send_set_sync(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            4,
        ]);
        Ok(())
    }

    /// Since when the set_desync message is available.
    #[allow(dead_code)]
    pub const MSG__SET_DESYNC__SINCE: u32 = 1;

    /// set sub-surface to desynchronized mode
    ///
    /// Change the commit behaviour of the sub-surface to desynchronized
    /// mode, also described as independent or freely running mode.
    ///
    /// In desynchronized mode, wl_surface.commit on a sub-surface will
    /// apply the pending state directly, without caching, as happens
    /// normally with a wl_surface. Calling wl_surface.commit on the
    /// parent surface has no effect on the sub-surface's wl_surface
    /// state. This mode allows a sub-surface to be updated on its own.
    ///
    /// If cached state exists when wl_surface.commit is called in
    /// desynchronized mode, the pending state is added to the cached
    /// state, and applied as a whole. This invalidates the cache.
    ///
    /// Note: even if a sub-surface is set to desynchronized, a parent
    /// sub-surface may override it to behave as synchronized. For details,
    /// see wl_subsurface.
    ///
    /// If a surface's parent surface behaves as desynchronized, then
    /// the cached state is applied on set_desync.
    #[inline]
    pub fn send_set_desync(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            5,
        ]);
        Ok(())
    }
}

/// A message handler for [WlSubsurface] proxies.
#[allow(dead_code)]
pub trait MetaWlSubsurfaceMessageHandler {
    /// remove sub-surface interface
    ///
    /// The sub-surface interface is removed from the wl_surface object
    /// that was turned into a sub-surface with a
    /// wl_subcompositor.get_subsurface request. The wl_surface's association
    /// to the parent is deleted. The wl_surface is unmapped immediately.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWlSubsurface>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_subsurface.destroy message: {}", Report::new(e));
        }
    }

    /// reposition the sub-surface
    ///
    /// This schedules a sub-surface position change.
    /// The sub-surface will be moved so that its origin (top left
    /// corner pixel) will be at the location x, y of the parent surface
    /// coordinate system. The coordinates are not restricted to the parent
    /// surface area. Negative values are allowed.
    ///
    /// The scheduled coordinates will take effect whenever the state of the
    /// parent surface is applied.
    ///
    /// If more than one set_position request is invoked by the client before
    /// the commit of the parent surface, the position of a new request always
    /// replaces the scheduled position from any previous request.
    ///
    /// The initial position is 0, 0.
    ///
    /// # Arguments
    ///
    /// - `x`: x coordinate in the parent surface
    /// - `y`: y coordinate in the parent surface
    #[inline]
    fn set_position(
        &mut self,
        _slf: &Rc<MetaWlSubsurface>,
        x: i32,
        y: i32,
    ) {
        let res = _slf.send_set_position(
            x,
            y,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_subsurface.set_position message: {}", Report::new(e));
        }
    }

    /// restack the sub-surface
    ///
    /// This sub-surface is taken from the stack, and put back just
    /// above the reference surface, changing the z-order of the sub-surfaces.
    /// The reference surface must be one of the sibling surfaces, or the
    /// parent surface. Using any other surface, including this sub-surface,
    /// will cause a protocol error.
    ///
    /// The z-order is double-buffered. Requests are handled in order and
    /// applied immediately to a pending state. The final pending state is
    /// copied to the active state the next time the state of the parent
    /// surface is applied.
    ///
    /// A new sub-surface is initially added as the top-most in the stack
    /// of its siblings and parent.
    ///
    /// # Arguments
    ///
    /// - `sibling`: the reference surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn place_above(
        &mut self,
        _slf: &Rc<MetaWlSubsurface>,
        sibling: &Rc<MetaWlSurface>,
    ) {
        let res = _slf.send_place_above(
            sibling,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_subsurface.place_above message: {}", Report::new(e));
        }
    }

    /// restack the sub-surface
    ///
    /// The sub-surface is placed just below the reference surface.
    /// See wl_subsurface.place_above.
    ///
    /// # Arguments
    ///
    /// - `sibling`: the reference surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn place_below(
        &mut self,
        _slf: &Rc<MetaWlSubsurface>,
        sibling: &Rc<MetaWlSurface>,
    ) {
        let res = _slf.send_place_below(
            sibling,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_subsurface.place_below message: {}", Report::new(e));
        }
    }

    /// set sub-surface to synchronized mode
    ///
    /// Change the commit behaviour of the sub-surface to synchronized
    /// mode, also described as the parent dependent mode.
    ///
    /// In synchronized mode, wl_surface.commit on a sub-surface will
    /// accumulate the committed state in a cache, but the state will
    /// not be applied and hence will not change the compositor output.
    /// The cached state is applied to the sub-surface immediately after
    /// the parent surface's state is applied. This ensures atomic
    /// updates of the parent and all its synchronized sub-surfaces.
    /// Applying the cached state will invalidate the cache, so further
    /// parent surface commits do not (re-)apply old state.
    ///
    /// See wl_subsurface for the recursive effect of this mode.
    #[inline]
    fn set_sync(
        &mut self,
        _slf: &Rc<MetaWlSubsurface>,
    ) {
        let res = _slf.send_set_sync(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_subsurface.set_sync message: {}", Report::new(e));
        }
    }

    /// set sub-surface to desynchronized mode
    ///
    /// Change the commit behaviour of the sub-surface to desynchronized
    /// mode, also described as independent or freely running mode.
    ///
    /// In desynchronized mode, wl_surface.commit on a sub-surface will
    /// apply the pending state directly, without caching, as happens
    /// normally with a wl_surface. Calling wl_surface.commit on the
    /// parent surface has no effect on the sub-surface's wl_surface
    /// state. This mode allows a sub-surface to be updated on its own.
    ///
    /// If cached state exists when wl_surface.commit is called in
    /// desynchronized mode, the pending state is added to the cached
    /// state, and applied as a whole. This invalidates the cache.
    ///
    /// Note: even if a sub-surface is set to desynchronized, a parent
    /// sub-surface may override it to behave as synchronized. For details,
    /// see wl_subsurface.
    ///
    /// If a surface's parent surface behaves as desynchronized, then
    /// the cached state is applied on set_desync.
    #[inline]
    fn set_desync(
        &mut self,
        _slf: &Rc<MetaWlSubsurface>,
    ) {
        let res = _slf.send_set_desync(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_subsurface.set_desync message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWlSubsurface {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                if let Some(handler) = handler {
                    (**handler).set_position(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.set_position(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let Some(arg0) = client.lookup(arg0) else {
                    return Err(ObjectError);
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).place_above(&self, arg0);
                } else {
                    DefaultMessageHandler.place_above(&self, arg0);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let Some(arg0) = client.lookup(arg0) else {
                    return Err(ObjectError);
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).place_below(&self, arg0);
                } else {
                    DefaultMessageHandler.place_below(&self, arg0);
                }
            }
            4 => {
                if let Some(handler) = handler {
                    (**handler).set_sync(&self);
                } else {
                    DefaultMessageHandler.set_sync(&self);
                }
            }
            5 => {
                if let Some(handler) = handler {
                    (**handler).set_desync(&self);
                } else {
                    DefaultMessageHandler.set_desync(&self);
                }
            }
            _ => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
    }
}

impl MetaWlSubsurface {
    /// Since when the error.bad_surface enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_BAD_SURFACE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWlSubsurfaceError(pub u32);

impl MetaWlSubsurfaceError {
    /// wl_surface is not a sibling or the parent
    #[allow(dead_code)]
    pub const BAD_SURFACE: Self = Self(0);
}

impl Debug for MetaWlSubsurfaceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::BAD_SURFACE => "BAD_SURFACE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
