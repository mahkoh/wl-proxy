//! input method manager
//!
//! The input method manager allows the client to become the input method on
//! a chosen seat.
//!
//! No more than one input method must be associated with any seat at any
//! given time.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_input_method_manager_v2 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpInputMethodManagerV2 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpInputMethodManagerV2Handler>,
}

struct DefaultHandler;

impl ZwpInputMethodManagerV2Handler for DefaultHandler { }

impl ConcreteObject for ZwpInputMethodManagerV2 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpInputMethodManagerV2;
    const INTERFACE_NAME: &str = "zwp_input_method_manager_v2";
}

impl ZwpInputMethodManagerV2 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpInputMethodManagerV2Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpInputMethodManagerV2Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpInputMethodManagerV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpInputMethodManagerV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpInputMethodManagerV2 {
    /// Since when the get_input_method message is available.
    pub const MSG__GET_INPUT_METHOD__SINCE: u32 = 1;

    /// request an input method object
    ///
    /// Request a new input zwp_input_method_v2 object associated with a given
    /// seat.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `input_method`:
    #[inline]
    pub fn try_send_get_input_method(
        &self,
        seat: &Rc<WlSeat>,
        input_method: &Rc<ZwpInputMethodV2>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            seat,
            input_method,
        );
        let arg0 = arg0.core();
        let arg1_obj = arg1;
        let arg1 = arg1_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("seat"))),
            Some(id) => id,
        };
        arg1.generate_server_id(arg1_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("input_method", e)))?;
        let arg1_id = arg1.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_manager_v2#{}.get_input_method(seat: wl_seat#{}, input_method: zwp_input_method_v2#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id);
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
            arg1_id,
        ]);
        Ok(())
    }

    /// request an input method object
    ///
    /// Request a new input zwp_input_method_v2 object associated with a given
    /// seat.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `input_method`:
    #[inline]
    pub fn send_get_input_method(
        &self,
        seat: &Rc<WlSeat>,
        input_method: &Rc<ZwpInputMethodV2>,
    ) {
        let res = self.try_send_get_input_method(
            seat,
            input_method,
        );
        if let Err(e) = res {
            log_send("zwp_input_method_manager_v2.get_input_method", &e);
        }
    }

    /// request an input method object
    ///
    /// Request a new input zwp_input_method_v2 object associated with a given
    /// seat.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    #[inline]
    pub fn new_try_send_get_input_method(
        &self,
        seat: &Rc<WlSeat>,
    ) -> Result<Rc<ZwpInputMethodV2>, ObjectError> {
        let input_method = self.core.create_child();
        self.try_send_get_input_method(
            seat,
            &input_method,
        )?;
        Ok(input_method)
    }

    /// request an input method object
    ///
    /// Request a new input zwp_input_method_v2 object associated with a given
    /// seat.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    #[inline]
    pub fn new_send_get_input_method(
        &self,
        seat: &Rc<WlSeat>,
    ) -> Rc<ZwpInputMethodV2> {
        let input_method = self.core.create_child();
        self.send_get_input_method(
            seat,
            &input_method,
        );
        input_method
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the input method manager
    ///
    /// Destroys the zwp_input_method_manager_v2 object.
    ///
    /// The zwp_input_method_v2 objects originating from it remain valid.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_manager_v2#{}.destroy()\n", id);
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

    /// destroy the input method manager
    ///
    /// Destroys the zwp_input_method_manager_v2 object.
    ///
    /// The zwp_input_method_v2 objects originating from it remain valid.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwp_input_method_manager_v2.destroy", &e);
        }
    }
}

/// A message handler for [`ZwpInputMethodManagerV2`] proxies.
pub trait ZwpInputMethodManagerV2Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpInputMethodManagerV2>) {
        slf.core.delete_id();
    }

    /// request an input method object
    ///
    /// Request a new input zwp_input_method_v2 object associated with a given
    /// seat.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `input_method`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_input_method(
        &mut self,
        slf: &Rc<ZwpInputMethodManagerV2>,
        seat: &Rc<WlSeat>,
        input_method: &Rc<ZwpInputMethodV2>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_input_method(
            seat,
            input_method,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_manager_v2.get_input_method", &e);
        }
    }

    /// destroy the input method manager
    ///
    /// Destroys the zwp_input_method_manager_v2 object.
    ///
    /// The zwp_input_method_v2 objects originating from it remain valid.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwpInputMethodManagerV2>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_manager_v2.destroy", &e);
        }
    }
}

impl ObjectPrivate for ZwpInputMethodManagerV2 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpInputMethodManagerV2, version),
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
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_manager_v2#{}.get_input_method(seat: wl_seat#{}, input_method: zwp_input_method_v2#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("seat", o.core().interface, ObjectInterface::WlSeat)));
                };
                let arg1_id = arg1;
                let arg1 = ZwpInputMethodV2::new(&self.core.state, self.core.version);
                arg1.core().set_client_id(client, arg1_id, arg1.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg1_id, "input_method", e)))?;
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_input_method(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_input_method(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_manager_v2#{}.destroy()\n", client_id, id);
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
            0 => "get_input_method",
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

impl Object for ZwpInputMethodManagerV2 {
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

