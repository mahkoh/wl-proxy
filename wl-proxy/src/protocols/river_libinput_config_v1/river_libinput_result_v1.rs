//! config application result
//!
//! The result returned by libinput on setting configuration for a device.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A river_libinput_result_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct RiverLibinputResultV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn RiverLibinputResultV1Handler>,
}

struct DefaultHandler;

impl RiverLibinputResultV1Handler for DefaultHandler { }

impl ConcreteObject for RiverLibinputResultV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::RiverLibinputResultV1;
    const INTERFACE_NAME: &str = "river_libinput_result_v1";
}

impl RiverLibinputResultV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl RiverLibinputResultV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn RiverLibinputResultV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for RiverLibinputResultV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RiverLibinputResultV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl RiverLibinputResultV1 {
    /// Since when the success message is available.
    pub const MSG__SUCCESS__SINCE: u32 = 1;

    /// config success
    ///
    /// The configuration was successfully applied to the device.
    #[inline]
    pub fn try_send_success(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_result_v1#{}.success()\n", client_id, id);
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
            0,
        ]);
        drop(fmt);
        drop(outgoing_ref);
        drop(client_ref);
        self.core.handle_client_destroy();
        Ok(())
    }

    /// config success
    ///
    /// The configuration was successfully applied to the device.
    #[inline]
    pub fn send_success(
        &self,
    ) {
        let res = self.try_send_success(
        );
        if let Err(e) = res {
            log_send("river_libinput_result_v1.success", &e);
        }
    }

    /// Since when the unsupported message is available.
    pub const MSG__UNSUPPORTED__SINCE: u32 = 1;

    /// config unsupported
    ///
    /// The configuration is unsupported by the device and was ignored.
    #[inline]
    pub fn try_send_unsupported(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_result_v1#{}.unsupported()\n", client_id, id);
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
        drop(fmt);
        drop(outgoing_ref);
        drop(client_ref);
        self.core.handle_client_destroy();
        Ok(())
    }

    /// config unsupported
    ///
    /// The configuration is unsupported by the device and was ignored.
    #[inline]
    pub fn send_unsupported(
        &self,
    ) {
        let res = self.try_send_unsupported(
        );
        if let Err(e) = res {
            log_send("river_libinput_result_v1.unsupported", &e);
        }
    }

    /// Since when the invalid message is available.
    pub const MSG__INVALID__SINCE: u32 = 1;

    /// config invalid
    ///
    /// The configuration is invalid and was ignored.
    #[inline]
    pub fn try_send_invalid(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_result_v1#{}.invalid()\n", client_id, id);
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
            2,
        ]);
        drop(fmt);
        drop(outgoing_ref);
        drop(client_ref);
        self.core.handle_client_destroy();
        Ok(())
    }

    /// config invalid
    ///
    /// The configuration is invalid and was ignored.
    #[inline]
    pub fn send_invalid(
        &self,
    ) {
        let res = self.try_send_invalid(
        );
        if let Err(e) = res {
            log_send("river_libinput_result_v1.invalid", &e);
        }
    }
}

/// A message handler for [`RiverLibinputResultV1`] proxies.
pub trait RiverLibinputResultV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<RiverLibinputResultV1>) {
        slf.core.delete_id();
    }

    /// config success
    ///
    /// The configuration was successfully applied to the device.
    #[inline]
    fn handle_success(
        &mut self,
        slf: &Rc<RiverLibinputResultV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_success(
        );
        if let Err(e) = res {
            log_forward("river_libinput_result_v1.success", &e);
        }
    }

    /// config unsupported
    ///
    /// The configuration is unsupported by the device and was ignored.
    #[inline]
    fn handle_unsupported(
        &mut self,
        slf: &Rc<RiverLibinputResultV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_unsupported(
        );
        if let Err(e) = res {
            log_forward("river_libinput_result_v1.unsupported", &e);
        }
    }

    /// config invalid
    ///
    /// The configuration is invalid and was ignored.
    #[inline]
    fn handle_invalid(
        &mut self,
        slf: &Rc<RiverLibinputResultV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_invalid(
        );
        if let Err(e) = res {
            log_forward("river_libinput_result_v1.invalid", &e);
        }
    }
}

impl ObjectPrivate for RiverLibinputResultV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::RiverLibinputResultV1, version),
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
            n => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
    }

    fn handle_event(self: Rc<Self>, server: &Endpoint, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
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
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_result_v1#{}.success()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                self.core.handle_server_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_success(&self);
                } else {
                    DefaultHandler.handle_success(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_result_v1#{}.unsupported()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                self.core.handle_server_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_unsupported(&self);
                } else {
                    DefaultHandler.handle_unsupported(&self);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_result_v1#{}.invalid()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                self.core.handle_server_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_invalid(&self);
                } else {
                    DefaultHandler.handle_invalid(&self);
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
        let _ = id;
        None
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "success",
            1 => "unsupported",
            2 => "invalid",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for RiverLibinputResultV1 {
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

