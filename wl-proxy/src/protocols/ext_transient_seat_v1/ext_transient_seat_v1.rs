//! transient seat handle
//!
//! When the transient seat handle is destroyed, the seat itself will also be
//! destroyed.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A ext_transient_seat_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ExtTransientSeatV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ExtTransientSeatV1Handler>,
}

struct DefaultHandler;

impl ExtTransientSeatV1Handler for DefaultHandler { }

impl ConcreteObject for ExtTransientSeatV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ExtTransientSeatV1;
    const INTERFACE_NAME: &str = "ext_transient_seat_v1";
}

impl ExtTransientSeatV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ExtTransientSeatV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ExtTransientSeatV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ExtTransientSeatV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtTransientSeatV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ExtTransientSeatV1 {
    /// Since when the ready message is available.
    pub const MSG__READY__SINCE: u32 = 1;

    /// transient seat is ready
    ///
    /// This event advertises the global name for the wl_seat to be used with
    /// wl_registry_bind.
    ///
    /// It is sent exactly once, immediately after the transient seat is created
    /// and the new "wl_seat" global is advertised, if and only if the creation
    /// of the transient seat was allowed.
    ///
    /// # Arguments
    ///
    /// - `global_name`:
    #[inline]
    pub fn try_send_ready(
        &self,
        global_name: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            global_name,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_transient_seat_v1#{}.ready(global_name: {})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0,
        ]);
        Ok(())
    }

    /// transient seat is ready
    ///
    /// This event advertises the global name for the wl_seat to be used with
    /// wl_registry_bind.
    ///
    /// It is sent exactly once, immediately after the transient seat is created
    /// and the new "wl_seat" global is advertised, if and only if the creation
    /// of the transient seat was allowed.
    ///
    /// # Arguments
    ///
    /// - `global_name`:
    #[inline]
    pub fn send_ready(
        &self,
        global_name: u32,
    ) {
        let res = self.try_send_ready(
            global_name,
        );
        if let Err(e) = res {
            log_send("ext_transient_seat_v1.ready", &e);
        }
    }

    /// Since when the denied message is available.
    pub const MSG__DENIED__SINCE: u32 = 1;

    /// transient seat creation denied
    ///
    /// The event informs the client that the compositor denied its request to
    /// create a transient seat.
    ///
    /// It is sent exactly once, immediately after the transient seat object is
    /// created, if and only if the creation of the transient seat was denied.
    ///
    /// After receiving this event, the client should destroy the object.
    #[inline]
    pub fn try_send_denied(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_transient_seat_v1#{}.denied()\n", client_id, id);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
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

    /// transient seat creation denied
    ///
    /// The event informs the client that the compositor denied its request to
    /// create a transient seat.
    ///
    /// It is sent exactly once, immediately after the transient seat object is
    /// created, if and only if the creation of the transient seat was denied.
    ///
    /// After receiving this event, the client should destroy the object.
    #[inline]
    pub fn send_denied(
        &self,
    ) {
        let res = self.try_send_denied(
        );
        if let Err(e) = res {
            log_send("ext_transient_seat_v1.denied", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy transient seat
    ///
    /// When the transient seat object is destroyed by the client, the
    /// associated seat created by the compositor is also destroyed.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_transient_seat_v1#{}.destroy()\n", id);
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

    /// destroy transient seat
    ///
    /// When the transient seat object is destroyed by the client, the
    /// associated seat created by the compositor is also destroyed.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("ext_transient_seat_v1.destroy", &e);
        }
    }
}

/// A message handler for [`ExtTransientSeatV1`] proxies.
pub trait ExtTransientSeatV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ExtTransientSeatV1>) {
        slf.core.delete_id();
    }

    /// transient seat is ready
    ///
    /// This event advertises the global name for the wl_seat to be used with
    /// wl_registry_bind.
    ///
    /// It is sent exactly once, immediately after the transient seat is created
    /// and the new "wl_seat" global is advertised, if and only if the creation
    /// of the transient seat was allowed.
    ///
    /// # Arguments
    ///
    /// - `global_name`:
    #[inline]
    fn handle_ready(
        &mut self,
        slf: &Rc<ExtTransientSeatV1>,
        global_name: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_ready(
            global_name,
        );
        if let Err(e) = res {
            log_forward("ext_transient_seat_v1.ready", &e);
        }
    }

    /// transient seat creation denied
    ///
    /// The event informs the client that the compositor denied its request to
    /// create a transient seat.
    ///
    /// It is sent exactly once, immediately after the transient seat object is
    /// created, if and only if the creation of the transient seat was denied.
    ///
    /// After receiving this event, the client should destroy the object.
    #[inline]
    fn handle_denied(
        &mut self,
        slf: &Rc<ExtTransientSeatV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_denied(
        );
        if let Err(e) = res {
            log_forward("ext_transient_seat_v1.denied", &e);
        }
    }

    /// destroy transient seat
    ///
    /// When the transient seat object is destroyed by the client, the
    /// associated seat created by the compositor is also destroyed.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ExtTransientSeatV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("ext_transient_seat_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for ExtTransientSeatV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ExtTransientSeatV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_transient_seat_v1#{}.destroy()\n", client_id, id);
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
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_transient_seat_v1#{}.ready(global_name: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_ready(&self, arg0);
                } else {
                    DefaultHandler.handle_ready(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_transient_seat_v1#{}.denied()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_denied(&self);
                } else {
                    DefaultHandler.handle_denied(&self);
                }
            }
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
        Ok(())
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "ready",
            1 => "denied",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ExtTransientSeatV1 {
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

