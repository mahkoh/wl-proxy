//! fifo interface
//!
//! A fifo object for a surface that may be used to add
//! display refresh constraints to content updates.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_fifo_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpFifoV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpFifoV1Handler>,
}

struct DefaultHandler;

impl WpFifoV1Handler for DefaultHandler { }

impl ConcreteObject for WpFifoV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WpFifoV1;
    const INTERFACE_NAME: &str = "wp_fifo_v1";
}

impl WpFifoV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WpFifoV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WpFifoV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpFifoV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpFifoV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpFifoV1 {
    /// Since when the set_barrier message is available.
    pub const MSG__SET_BARRIER__SINCE: u32 = 1;

    /// sets the start point for a fifo constraint
    ///
    /// When the content update containing the "set_barrier" is applied,
    /// it sets a "fifo_barrier" condition on the surface associated with
    /// the fifo object. The condition is cleared immediately after the
    /// following latching deadline for non-tearing presentation.
    ///
    /// The compositor may clear the condition early if it must do so to
    /// ensure client forward progress assumptions.
    ///
    /// To wait for this condition to clear, use the "wait_barrier" request.
    ///
    /// "set_barrier" is double-buffered state, see wl_surface.commit.
    ///
    /// Requesting set_barrier after the fifo object's surface is
    /// destroyed will generate a "surface_destroyed" error.
    #[inline]
    pub fn try_send_set_barrier(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_fifo_v1#{}.set_barrier()\n", id);
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
        Ok(())
    }

    /// sets the start point for a fifo constraint
    ///
    /// When the content update containing the "set_barrier" is applied,
    /// it sets a "fifo_barrier" condition on the surface associated with
    /// the fifo object. The condition is cleared immediately after the
    /// following latching deadline for non-tearing presentation.
    ///
    /// The compositor may clear the condition early if it must do so to
    /// ensure client forward progress assumptions.
    ///
    /// To wait for this condition to clear, use the "wait_barrier" request.
    ///
    /// "set_barrier" is double-buffered state, see wl_surface.commit.
    ///
    /// Requesting set_barrier after the fifo object's surface is
    /// destroyed will generate a "surface_destroyed" error.
    #[inline]
    pub fn send_set_barrier(
        &self,
    ) {
        let res = self.try_send_set_barrier(
        );
        if let Err(e) = res {
            log_send("wp_fifo_v1.set_barrier", &e);
        }
    }

    /// Since when the wait_barrier message is available.
    pub const MSG__WAIT_BARRIER__SINCE: u32 = 1;

    /// adds a fifo constraint to a content update
    ///
    /// Indicate that this content update is not ready while a
    /// "fifo_barrier" condition is present on the surface.
    ///
    /// This means that when the content update containing "set_barrier"
    /// was made active at a latching deadline, it will be active for
    /// at least one refresh cycle. A content update which is allowed to
    /// tear might become active after a latching deadline if no content
    /// update became active at the deadline.
    ///
    /// The constraint must be ignored if the surface is a subsurface in
    /// synchronized mode. If the surface is not being updated by the
    /// compositor (off-screen, occluded) the compositor may ignore the
    /// constraint. Clients must use an additional mechanism such as
    /// frame callbacks or timestamps to ensure throttling occurs under
    /// all conditions.
    ///
    /// "wait_barrier" is double-buffered state, see wl_surface.commit.
    ///
    /// Requesting "wait_barrier" after the fifo object's surface is
    /// destroyed will generate a "surface_destroyed" error.
    #[inline]
    pub fn try_send_wait_barrier(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_fifo_v1#{}.wait_barrier()\n", id);
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
            1,
        ]);
        Ok(())
    }

    /// adds a fifo constraint to a content update
    ///
    /// Indicate that this content update is not ready while a
    /// "fifo_barrier" condition is present on the surface.
    ///
    /// This means that when the content update containing "set_barrier"
    /// was made active at a latching deadline, it will be active for
    /// at least one refresh cycle. A content update which is allowed to
    /// tear might become active after a latching deadline if no content
    /// update became active at the deadline.
    ///
    /// The constraint must be ignored if the surface is a subsurface in
    /// synchronized mode. If the surface is not being updated by the
    /// compositor (off-screen, occluded) the compositor may ignore the
    /// constraint. Clients must use an additional mechanism such as
    /// frame callbacks or timestamps to ensure throttling occurs under
    /// all conditions.
    ///
    /// "wait_barrier" is double-buffered state, see wl_surface.commit.
    ///
    /// Requesting "wait_barrier" after the fifo object's surface is
    /// destroyed will generate a "surface_destroyed" error.
    #[inline]
    pub fn send_wait_barrier(
        &self,
    ) {
        let res = self.try_send_wait_barrier(
        );
        if let Err(e) = res {
            log_send("wp_fifo_v1.wait_barrier", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the fifo interface
    ///
    /// Informs the server that the client will no longer be using
    /// this protocol object.
    ///
    /// Surface state changes previously made by this protocol are
    /// unaffected by this object's destruction.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_fifo_v1#{}.destroy()\n", id);
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
            2,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the fifo interface
    ///
    /// Informs the server that the client will no longer be using
    /// this protocol object.
    ///
    /// Surface state changes previously made by this protocol are
    /// unaffected by this object's destruction.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wp_fifo_v1.destroy", &e);
        }
    }
}

/// A message handler for [`WpFifoV1`] proxies.
pub trait WpFifoV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpFifoV1>) {
        slf.core.delete_id();
    }

    /// sets the start point for a fifo constraint
    ///
    /// When the content update containing the "set_barrier" is applied,
    /// it sets a "fifo_barrier" condition on the surface associated with
    /// the fifo object. The condition is cleared immediately after the
    /// following latching deadline for non-tearing presentation.
    ///
    /// The compositor may clear the condition early if it must do so to
    /// ensure client forward progress assumptions.
    ///
    /// To wait for this condition to clear, use the "wait_barrier" request.
    ///
    /// "set_barrier" is double-buffered state, see wl_surface.commit.
    ///
    /// Requesting set_barrier after the fifo object's surface is
    /// destroyed will generate a "surface_destroyed" error.
    #[inline]
    fn handle_set_barrier(
        &mut self,
        slf: &Rc<WpFifoV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_barrier(
        );
        if let Err(e) = res {
            log_forward("wp_fifo_v1.set_barrier", &e);
        }
    }

    /// adds a fifo constraint to a content update
    ///
    /// Indicate that this content update is not ready while a
    /// "fifo_barrier" condition is present on the surface.
    ///
    /// This means that when the content update containing "set_barrier"
    /// was made active at a latching deadline, it will be active for
    /// at least one refresh cycle. A content update which is allowed to
    /// tear might become active after a latching deadline if no content
    /// update became active at the deadline.
    ///
    /// The constraint must be ignored if the surface is a subsurface in
    /// synchronized mode. If the surface is not being updated by the
    /// compositor (off-screen, occluded) the compositor may ignore the
    /// constraint. Clients must use an additional mechanism such as
    /// frame callbacks or timestamps to ensure throttling occurs under
    /// all conditions.
    ///
    /// "wait_barrier" is double-buffered state, see wl_surface.commit.
    ///
    /// Requesting "wait_barrier" after the fifo object's surface is
    /// destroyed will generate a "surface_destroyed" error.
    #[inline]
    fn handle_wait_barrier(
        &mut self,
        slf: &Rc<WpFifoV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_wait_barrier(
        );
        if let Err(e) = res {
            log_forward("wp_fifo_v1.wait_barrier", &e);
        }
    }

    /// destroy the fifo interface
    ///
    /// Informs the server that the client will no longer be using
    /// this protocol object.
    ///
    /// Surface state changes previously made by this protocol are
    /// unaffected by this object's destruction.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WpFifoV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wp_fifo_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for WpFifoV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpFifoV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_fifo_v1#{}.set_barrier()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_barrier(&self);
                } else {
                    DefaultHandler.handle_set_barrier(&self);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_fifo_v1#{}.wait_barrier()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_wait_barrier(&self);
                } else {
                    DefaultHandler.handle_wait_barrier(&self);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_fifo_v1#{}.destroy()\n", client_id, id);
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
            0 => "set_barrier",
            1 => "wait_barrier",
            2 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WpFifoV1 {
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

impl WpFifoV1 {
    /// Since when the error.surface_destroyed enum variant is available.
    pub const ENM__ERROR_SURFACE_DESTROYED__SINCE: u32 = 1;
}

/// fatal error
///
/// These fatal protocol errors may be emitted in response to
/// illegal requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpFifoV1Error(pub u32);

impl WpFifoV1Error {
    /// the associated surface no longer exists
    pub const SURFACE_DESTROYED: Self = Self(0);
}

impl Debug for WpFifoV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::SURFACE_DESTROYED => "SURFACE_DESTROYED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
