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
//! A sub-surface can be in one of two modes. The possible modes are
//! synchronized and desynchronized, see methods wl_subsurface.set_sync and
//! wl_subsurface.set_desync.
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
//! If a surface behaves as in synchronized mode, it is effectively
//! synchronized, otherwise it is effectively desynchronized.
//!
//! A sub-surface is initially in the synchronized mode.
//!
//! The wl_subsurface interface has requests which modify double-buffered
//! state of the parent surface (wl_subsurface.set_position, .place_above and
//! .place_below).
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

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wl_subsurface object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlSubsurface {
    core: ObjectCore,
    handler: HandlerHolder<dyn WlSubsurfaceHandler>,
}

struct DefaultHandler;

impl WlSubsurfaceHandler for DefaultHandler { }

impl ConcreteObject for WlSubsurface {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WlSubsurface;
    const INTERFACE_NAME: &str = "wl_subsurface";
}

impl WlSubsurface {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WlSubsurfaceHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WlSubsurfaceHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlSubsurface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlSubsurface")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlSubsurface {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// remove sub-surface interface
    ///
    /// The sub-surface interface is removed from the wl_surface object
    /// that was turned into a sub-surface with a
    /// wl_subcompositor.get_subsurface request. The wl_surface's association
    /// to the parent is deleted. The wl_surface is unmapped immediately.
    #[inline]
    pub fn try_send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_subsurface#{}.destroy()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// remove sub-surface interface
    ///
    /// The sub-surface interface is removed from the wl_surface object
    /// that was turned into a sub-surface with a
    /// wl_subcompositor.get_subsurface request. The wl_surface's association
    /// to the parent is deleted. The wl_surface is unmapped immediately.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wl_subsurface.destroy", &e);
        }
    }

    /// Since when the set_position message is available.
    pub const MSG__SET_POSITION__SINCE: u32 = 1;

    /// reposition the sub-surface
    ///
    /// This sets the position of the sub-surface, relative to the parent
    /// surface.
    ///
    /// The sub-surface will be moved so that its origin (top left
    /// corner pixel) will be at the location x, y of the parent surface
    /// coordinate system. The coordinates are not restricted to the parent
    /// surface area. Negative values are allowed.
    ///
    /// The initial position is 0, 0.
    ///
    /// Position is double-buffered state on the parent surface, see
    /// wl_subsurface and wl_surface.commit for more information.
    ///
    /// # Arguments
    ///
    /// - `x`: x coordinate in the parent surface
    /// - `y`: y coordinate in the parent surface
    #[inline]
    pub fn try_send_set_position(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_subsurface#{}.set_position(x: {}, y: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// reposition the sub-surface
    ///
    /// This sets the position of the sub-surface, relative to the parent
    /// surface.
    ///
    /// The sub-surface will be moved so that its origin (top left
    /// corner pixel) will be at the location x, y of the parent surface
    /// coordinate system. The coordinates are not restricted to the parent
    /// surface area. Negative values are allowed.
    ///
    /// The initial position is 0, 0.
    ///
    /// Position is double-buffered state on the parent surface, see
    /// wl_subsurface and wl_surface.commit for more information.
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
    ) {
        let res = self.try_send_set_position(
            x,
            y,
        );
        if let Err(e) = res {
            log_send("wl_subsurface.set_position", &e);
        }
    }

    /// Since when the place_above message is available.
    pub const MSG__PLACE_ABOVE__SINCE: u32 = 1;

    /// restack the sub-surface
    ///
    /// This sub-surface is taken from the stack, and put back just
    /// above the reference surface, changing the z-order of the sub-surfaces.
    /// The reference surface must be one of the sibling surfaces, or the
    /// parent surface. Using any other surface, including this sub-surface,
    /// will cause a protocol error.
    ///
    /// A new sub-surface is initially added as the top-most in the stack
    /// of its siblings and parent.
    ///
    /// Z-order is double-buffered state on the parent surface, see
    /// wl_subsurface and wl_surface.commit for more information.
    ///
    /// # Arguments
    ///
    /// - `sibling`: the reference surface
    #[inline]
    pub fn try_send_place_above(
        &self,
        sibling: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            sibling,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("sibling"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_subsurface#{}.place_above(sibling: wl_surface#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
            arg0_id,
        ]);
        Ok(())
    }

    /// restack the sub-surface
    ///
    /// This sub-surface is taken from the stack, and put back just
    /// above the reference surface, changing the z-order of the sub-surfaces.
    /// The reference surface must be one of the sibling surfaces, or the
    /// parent surface. Using any other surface, including this sub-surface,
    /// will cause a protocol error.
    ///
    /// A new sub-surface is initially added as the top-most in the stack
    /// of its siblings and parent.
    ///
    /// Z-order is double-buffered state on the parent surface, see
    /// wl_subsurface and wl_surface.commit for more information.
    ///
    /// # Arguments
    ///
    /// - `sibling`: the reference surface
    #[inline]
    pub fn send_place_above(
        &self,
        sibling: &Rc<WlSurface>,
    ) {
        let res = self.try_send_place_above(
            sibling,
        );
        if let Err(e) = res {
            log_send("wl_subsurface.place_above", &e);
        }
    }

    /// Since when the place_below message is available.
    pub const MSG__PLACE_BELOW__SINCE: u32 = 1;

    /// restack the sub-surface
    ///
    /// The sub-surface is placed just below the reference surface.
    ///
    /// See wl_subsurface.place_above.
    ///
    /// # Arguments
    ///
    /// - `sibling`: the reference surface
    #[inline]
    pub fn try_send_place_below(
        &self,
        sibling: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            sibling,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("sibling"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_subsurface#{}.place_below(sibling: wl_surface#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            3,
            arg0_id,
        ]);
        Ok(())
    }

    /// restack the sub-surface
    ///
    /// The sub-surface is placed just below the reference surface.
    ///
    /// See wl_subsurface.place_above.
    ///
    /// # Arguments
    ///
    /// - `sibling`: the reference surface
    #[inline]
    pub fn send_place_below(
        &self,
        sibling: &Rc<WlSurface>,
    ) {
        let res = self.try_send_place_below(
            sibling,
        );
        if let Err(e) = res {
            log_send("wl_subsurface.place_below", &e);
        }
    }

    /// Since when the set_sync message is available.
    pub const MSG__SET_SYNC__SINCE: u32 = 1;

    /// set sub-surface to synchronized mode
    ///
    /// Change the commit behaviour of the sub-surface to synchronized
    /// mode.
    ///
    /// See wl_subsurface and wl_surface.commit for more information.
    #[inline]
    pub fn try_send_set_sync(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_subsurface#{}.set_sync()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            4,
        ]);
        Ok(())
    }

    /// set sub-surface to synchronized mode
    ///
    /// Change the commit behaviour of the sub-surface to synchronized
    /// mode.
    ///
    /// See wl_subsurface and wl_surface.commit for more information.
    #[inline]
    pub fn send_set_sync(
        &self,
    ) {
        let res = self.try_send_set_sync(
        );
        if let Err(e) = res {
            log_send("wl_subsurface.set_sync", &e);
        }
    }

    /// Since when the set_desync message is available.
    pub const MSG__SET_DESYNC__SINCE: u32 = 1;

    /// set sub-surface to desynchronized mode
    ///
    /// Change the commit behaviour of the sub-surface to desynchronized
    /// mode.
    ///
    /// See wl_subsurface and wl_surface.commit for more information.
    #[inline]
    pub fn try_send_set_desync(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_subsurface#{}.set_desync()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            5,
        ]);
        Ok(())
    }

    /// set sub-surface to desynchronized mode
    ///
    /// Change the commit behaviour of the sub-surface to desynchronized
    /// mode.
    ///
    /// See wl_subsurface and wl_surface.commit for more information.
    #[inline]
    pub fn send_set_desync(
        &self,
    ) {
        let res = self.try_send_set_desync(
        );
        if let Err(e) = res {
            log_send("wl_subsurface.set_desync", &e);
        }
    }
}

/// A message handler for [`WlSubsurface`] proxies.
pub trait WlSubsurfaceHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WlSubsurface>) {
        slf.core.delete_id();
    }

    /// remove sub-surface interface
    ///
    /// The sub-surface interface is removed from the wl_surface object
    /// that was turned into a sub-surface with a
    /// wl_subcompositor.get_subsurface request. The wl_surface's association
    /// to the parent is deleted. The wl_surface is unmapped immediately.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WlSubsurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wl_subsurface.destroy", &e);
        }
    }

    /// reposition the sub-surface
    ///
    /// This sets the position of the sub-surface, relative to the parent
    /// surface.
    ///
    /// The sub-surface will be moved so that its origin (top left
    /// corner pixel) will be at the location x, y of the parent surface
    /// coordinate system. The coordinates are not restricted to the parent
    /// surface area. Negative values are allowed.
    ///
    /// The initial position is 0, 0.
    ///
    /// Position is double-buffered state on the parent surface, see
    /// wl_subsurface and wl_surface.commit for more information.
    ///
    /// # Arguments
    ///
    /// - `x`: x coordinate in the parent surface
    /// - `y`: y coordinate in the parent surface
    #[inline]
    fn handle_set_position(
        &mut self,
        slf: &Rc<WlSubsurface>,
        x: i32,
        y: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_position(
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("wl_subsurface.set_position", &e);
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
    /// A new sub-surface is initially added as the top-most in the stack
    /// of its siblings and parent.
    ///
    /// Z-order is double-buffered state on the parent surface, see
    /// wl_subsurface and wl_surface.commit for more information.
    ///
    /// # Arguments
    ///
    /// - `sibling`: the reference surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_place_above(
        &mut self,
        slf: &Rc<WlSubsurface>,
        sibling: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_place_above(
            sibling,
        );
        if let Err(e) = res {
            log_forward("wl_subsurface.place_above", &e);
        }
    }

    /// restack the sub-surface
    ///
    /// The sub-surface is placed just below the reference surface.
    ///
    /// See wl_subsurface.place_above.
    ///
    /// # Arguments
    ///
    /// - `sibling`: the reference surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_place_below(
        &mut self,
        slf: &Rc<WlSubsurface>,
        sibling: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_place_below(
            sibling,
        );
        if let Err(e) = res {
            log_forward("wl_subsurface.place_below", &e);
        }
    }

    /// set sub-surface to synchronized mode
    ///
    /// Change the commit behaviour of the sub-surface to synchronized
    /// mode.
    ///
    /// See wl_subsurface and wl_surface.commit for more information.
    #[inline]
    fn handle_set_sync(
        &mut self,
        slf: &Rc<WlSubsurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_sync(
        );
        if let Err(e) = res {
            log_forward("wl_subsurface.set_sync", &e);
        }
    }

    /// set sub-surface to desynchronized mode
    ///
    /// Change the commit behaviour of the sub-surface to desynchronized
    /// mode.
    ///
    /// See wl_subsurface and wl_surface.commit for more information.
    #[inline]
    fn handle_set_desync(
        &mut self,
        slf: &Rc<WlSubsurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_desync(
        );
        if let Err(e) = res {
            log_forward("wl_subsurface.set_desync", &e);
        }
    }
}

impl ObjectPrivate for WlSubsurface {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WlSubsurface, version),
            handler: Default::default(),
        })
    }

    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err((ObjectError(ObjectErrorKind::HandlerBorrowed), self));
        };
        if let Some(handler) = &mut *handler {
            handler.delete_id(&self);
        } else {
            self.core.delete_id();
        }
        Ok(())
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_subsurface#{}.destroy()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_destroy(&self);
                } else {
                    DefaultHandler.handle_destroy(&self);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_subsurface#{}.set_position(x: {}, y: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_position(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_position(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_subsurface#{}.place_above(sibling: wl_surface#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("sibling", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_place_above(&self, arg0);
                } else {
                    DefaultHandler.handle_place_above(&self, arg0);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_subsurface#{}.place_below(sibling: wl_surface#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("sibling", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_place_below(&self, arg0);
                } else {
                    DefaultHandler.handle_place_below(&self, arg0);
                }
            }
            4 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_subsurface#{}.set_sync()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_sync(&self);
                } else {
                    DefaultHandler.handle_set_sync(&self);
                }
            }
            5 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_subsurface#{}.set_desync()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_desync(&self);
                } else {
                    DefaultHandler.handle_set_desync(&self);
                }
            }
            n => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, server: &Endpoint, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            1 => "set_position",
            2 => "place_above",
            3 => "place_below",
            4 => "set_sync",
            5 => "set_desync",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WlSubsurface {
    fn core(&self) -> &ObjectCore {
        &self.core
    }

    fn unset_handler(&self) {
        self.handler.set(None);
    }

    fn get_handler_any_ref(&self) -> Result<HandlerRef<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.try_borrow().ok_or(HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(HandlerRef::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<HandlerMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.try_borrow_mut().ok_or(HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(HandlerMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

impl WlSubsurface {
    /// Since when the error.bad_surface enum variant is available.
    pub const ENM__ERROR_BAD_SURFACE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WlSubsurfaceError(pub u32);

impl WlSubsurfaceError {
    /// wl_surface is not a sibling or the parent
    pub const BAD_SURFACE: Self = Self(0);
}

impl Debug for WlSubsurfaceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::BAD_SURFACE => "BAD_SURFACE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
