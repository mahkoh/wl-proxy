//! transient seat manager
//!
//! The transient seat manager creates short-lived seats.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A ext_transient_seat_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ExtTransientSeatManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ExtTransientSeatManagerV1Handler>,
}

struct DefaultHandler;

impl ExtTransientSeatManagerV1Handler for DefaultHandler { }

impl ConcreteObject for ExtTransientSeatManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ExtTransientSeatManagerV1;
    const INTERFACE_NAME: &str = "ext_transient_seat_manager_v1";
}

impl ExtTransientSeatManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ExtTransientSeatManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ExtTransientSeatManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ExtTransientSeatManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtTransientSeatManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ExtTransientSeatManagerV1 {
    /// Since when the create message is available.
    pub const MSG__CREATE__SINCE: u32 = 1;

    /// create a transient seat
    ///
    /// Create a new seat that is removed when the client side transient seat
    /// object is destroyed.
    ///
    /// The actual seat may be removed sooner, in which case the transient seat
    /// object shall become inert.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    #[inline]
    pub fn try_send_create(
        &self,
        seat: &Rc<ExtTransientSeatV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            seat,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("seat", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_transient_seat_manager_v1#{}.create(seat: ext_transient_seat_v1#{})\n", id, arg0);
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
            0,
            arg0_id,
        ]);
        Ok(())
    }

    /// create a transient seat
    ///
    /// Create a new seat that is removed when the client side transient seat
    /// object is destroyed.
    ///
    /// The actual seat may be removed sooner, in which case the transient seat
    /// object shall become inert.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    #[inline]
    pub fn send_create(
        &self,
        seat: &Rc<ExtTransientSeatV1>,
    ) {
        let res = self.try_send_create(
            seat,
        );
        if let Err(e) = res {
            log_send("ext_transient_seat_manager_v1.create", &e);
        }
    }

    /// create a transient seat
    ///
    /// Create a new seat that is removed when the client side transient seat
    /// object is destroyed.
    ///
    /// The actual seat may be removed sooner, in which case the transient seat
    /// object shall become inert.
    #[inline]
    pub fn new_try_send_create(
        &self,
    ) -> Result<Rc<ExtTransientSeatV1>, ObjectError> {
        let seat = self.core.create_child();
        self.try_send_create(
            &seat,
        )?;
        Ok(seat)
    }

    /// create a transient seat
    ///
    /// Create a new seat that is removed when the client side transient seat
    /// object is destroyed.
    ///
    /// The actual seat may be removed sooner, in which case the transient seat
    /// object shall become inert.
    #[inline]
    pub fn new_send_create(
        &self,
    ) -> Rc<ExtTransientSeatV1> {
        let seat = self.core.create_child();
        self.send_create(
            &seat,
        );
        seat
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the manager
    ///
    /// Destroy the manager.
    ///
    /// All objects created by the manager will remain valid until they are
    /// destroyed themselves.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_transient_seat_manager_v1#{}.destroy()\n", id);
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
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the manager
    ///
    /// Destroy the manager.
    ///
    /// All objects created by the manager will remain valid until they are
    /// destroyed themselves.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("ext_transient_seat_manager_v1.destroy", &e);
        }
    }
}

/// A message handler for [`ExtTransientSeatManagerV1`] proxies.
pub trait ExtTransientSeatManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ExtTransientSeatManagerV1>) {
        slf.core.delete_id();
    }

    /// create a transient seat
    ///
    /// Create a new seat that is removed when the client side transient seat
    /// object is destroyed.
    ///
    /// The actual seat may be removed sooner, in which case the transient seat
    /// object shall become inert.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    #[inline]
    fn handle_create(
        &mut self,
        slf: &Rc<ExtTransientSeatManagerV1>,
        seat: &Rc<ExtTransientSeatV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create(
            seat,
        );
        if let Err(e) = res {
            log_forward("ext_transient_seat_manager_v1.create", &e);
        }
    }

    /// destroy the manager
    ///
    /// Destroy the manager.
    ///
    /// All objects created by the manager will remain valid until they are
    /// destroyed themselves.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ExtTransientSeatManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("ext_transient_seat_manager_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for ExtTransientSeatManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ExtTransientSeatManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_transient_seat_manager_v1#{}.create(seat: ext_transient_seat_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = ExtTransientSeatV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "seat", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create(&self, arg0);
                } else {
                    DefaultHandler.handle_create(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_transient_seat_manager_v1#{}.destroy()\n", client_id, id);
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
            0 => "create",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for ExtTransientSeatManagerV1 {
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

