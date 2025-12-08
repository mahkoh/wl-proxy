//! list toplevels
//!
//! A toplevel is defined as a surface with a role similar to xdg_toplevel.
//! XWayland surfaces may be treated like toplevels in this protocol.
//!
//! After a client binds the ext_foreign_toplevel_list_v1, each mapped
//! toplevel window will be sent using the ext_foreign_toplevel_list_v1.toplevel
//! event.
//!
//! Clients which only care about the current state can perform a roundtrip after
//! binding this global.
//!
//! For each instance of ext_foreign_toplevel_list_v1, the compositor must
//! create a new ext_foreign_toplevel_handle_v1 object for each mapped toplevel.
//!
//! If a compositor implementation sends the ext_foreign_toplevel_list_v1.finished
//! event after the global is bound, the compositor must not send any
//! ext_foreign_toplevel_list_v1.toplevel events.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A ext_foreign_toplevel_list_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ExtForeignToplevelListV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ExtForeignToplevelListV1Handler>,
}

struct DefaultHandler;

impl ExtForeignToplevelListV1Handler for DefaultHandler { }

impl ConcreteObject for ExtForeignToplevelListV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ExtForeignToplevelListV1;
    const INTERFACE_NAME: &str = "ext_foreign_toplevel_list_v1";
}

impl ExtForeignToplevelListV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ExtForeignToplevelListV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ExtForeignToplevelListV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ExtForeignToplevelListV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtForeignToplevelListV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ExtForeignToplevelListV1 {
    /// Since when the toplevel message is available.
    pub const MSG__TOPLEVEL__SINCE: u32 = 1;

    /// a toplevel has been created
    ///
    /// This event is emitted whenever a new toplevel window is created. It is
    /// emitted for all toplevels, regardless of the app that has created them.
    ///
    /// All initial properties of the toplevel (identifier, title, app_id) will be sent
    /// immediately after this event using the corresponding events for
    /// ext_foreign_toplevel_handle_v1. The compositor will use the
    /// ext_foreign_toplevel_handle_v1.done event to indicate when all data has
    /// been sent.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    #[inline]
    pub fn try_send_toplevel(
        &self,
        toplevel: &Rc<ExtForeignToplevelHandleV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            toplevel,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateClientId("toplevel", e)))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_foreign_toplevel_list_v1#{}.toplevel(toplevel: ext_foreign_toplevel_handle_v1#{})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// a toplevel has been created
    ///
    /// This event is emitted whenever a new toplevel window is created. It is
    /// emitted for all toplevels, regardless of the app that has created them.
    ///
    /// All initial properties of the toplevel (identifier, title, app_id) will be sent
    /// immediately after this event using the corresponding events for
    /// ext_foreign_toplevel_handle_v1. The compositor will use the
    /// ext_foreign_toplevel_handle_v1.done event to indicate when all data has
    /// been sent.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    #[inline]
    pub fn send_toplevel(
        &self,
        toplevel: &Rc<ExtForeignToplevelHandleV1>,
    ) {
        let res = self.try_send_toplevel(
            toplevel,
        );
        if let Err(e) = res {
            log_send("ext_foreign_toplevel_list_v1.toplevel", &e);
        }
    }

    /// a toplevel has been created
    ///
    /// This event is emitted whenever a new toplevel window is created. It is
    /// emitted for all toplevels, regardless of the app that has created them.
    ///
    /// All initial properties of the toplevel (identifier, title, app_id) will be sent
    /// immediately after this event using the corresponding events for
    /// ext_foreign_toplevel_handle_v1. The compositor will use the
    /// ext_foreign_toplevel_handle_v1.done event to indicate when all data has
    /// been sent.
    #[inline]
    pub fn new_try_send_toplevel(
        &self,
    ) -> Result<Rc<ExtForeignToplevelHandleV1>, ObjectError> {
        let toplevel = self.core.create_child();
        self.try_send_toplevel(
            &toplevel,
        )?;
        Ok(toplevel)
    }

    /// a toplevel has been created
    ///
    /// This event is emitted whenever a new toplevel window is created. It is
    /// emitted for all toplevels, regardless of the app that has created them.
    ///
    /// All initial properties of the toplevel (identifier, title, app_id) will be sent
    /// immediately after this event using the corresponding events for
    /// ext_foreign_toplevel_handle_v1. The compositor will use the
    /// ext_foreign_toplevel_handle_v1.done event to indicate when all data has
    /// been sent.
    #[inline]
    pub fn new_send_toplevel(
        &self,
    ) -> Rc<ExtForeignToplevelHandleV1> {
        let toplevel = self.core.create_child();
        self.send_toplevel(
            &toplevel,
        );
        toplevel
    }

    /// Since when the finished message is available.
    pub const MSG__FINISHED__SINCE: u32 = 1;

    /// the compositor has finished with the toplevel manager
    ///
    /// This event indicates that the compositor is done sending events
    /// to this object. The client should destroy the object.
    /// See ext_foreign_toplevel_list_v1.destroy for more information.
    ///
    /// The compositor must not send any more toplevel events after this event.
    #[inline]
    pub fn try_send_finished(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_foreign_toplevel_list_v1#{}.finished()\n", client_id, id);
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

    /// the compositor has finished with the toplevel manager
    ///
    /// This event indicates that the compositor is done sending events
    /// to this object. The client should destroy the object.
    /// See ext_foreign_toplevel_list_v1.destroy for more information.
    ///
    /// The compositor must not send any more toplevel events after this event.
    #[inline]
    pub fn send_finished(
        &self,
    ) {
        let res = self.try_send_finished(
        );
        if let Err(e) = res {
            log_send("ext_foreign_toplevel_list_v1.finished", &e);
        }
    }

    /// Since when the stop message is available.
    pub const MSG__STOP__SINCE: u32 = 1;

    /// stop sending events
    ///
    /// This request indicates that the client no longer wishes to receive
    /// events for new toplevels.
    ///
    /// The Wayland protocol is asynchronous, meaning the compositor may send
    /// further toplevel events until the stop request is processed.
    /// The client should wait for a ext_foreign_toplevel_list_v1.finished
    /// event before destroying this object.
    #[inline]
    pub fn try_send_stop(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_foreign_toplevel_list_v1#{}.stop()\n", id);
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

    /// stop sending events
    ///
    /// This request indicates that the client no longer wishes to receive
    /// events for new toplevels.
    ///
    /// The Wayland protocol is asynchronous, meaning the compositor may send
    /// further toplevel events until the stop request is processed.
    /// The client should wait for a ext_foreign_toplevel_list_v1.finished
    /// event before destroying this object.
    #[inline]
    pub fn send_stop(
        &self,
    ) {
        let res = self.try_send_stop(
        );
        if let Err(e) = res {
            log_send("ext_foreign_toplevel_list_v1.stop", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the ext_foreign_toplevel_list_v1 object
    ///
    /// This request should be called either when the client will no longer
    /// use the ext_foreign_toplevel_list_v1 or after the finished event
    /// has been received to allow destruction of the object.
    ///
    /// If a client wishes to destroy this object it should send a
    /// ext_foreign_toplevel_list_v1.stop request and wait for a ext_foreign_toplevel_list_v1.finished
    /// event, then destroy the handles and then this object.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_foreign_toplevel_list_v1#{}.destroy()\n", id);
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

    /// destroy the ext_foreign_toplevel_list_v1 object
    ///
    /// This request should be called either when the client will no longer
    /// use the ext_foreign_toplevel_list_v1 or after the finished event
    /// has been received to allow destruction of the object.
    ///
    /// If a client wishes to destroy this object it should send a
    /// ext_foreign_toplevel_list_v1.stop request and wait for a ext_foreign_toplevel_list_v1.finished
    /// event, then destroy the handles and then this object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("ext_foreign_toplevel_list_v1.destroy", &e);
        }
    }
}

/// A message handler for [`ExtForeignToplevelListV1`] proxies.
pub trait ExtForeignToplevelListV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ExtForeignToplevelListV1>) {
        slf.core.delete_id();
    }

    /// a toplevel has been created
    ///
    /// This event is emitted whenever a new toplevel window is created. It is
    /// emitted for all toplevels, regardless of the app that has created them.
    ///
    /// All initial properties of the toplevel (identifier, title, app_id) will be sent
    /// immediately after this event using the corresponding events for
    /// ext_foreign_toplevel_handle_v1. The compositor will use the
    /// ext_foreign_toplevel_handle_v1.done event to indicate when all data has
    /// been sent.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    #[inline]
    fn handle_toplevel(
        &mut self,
        slf: &Rc<ExtForeignToplevelListV1>,
        toplevel: &Rc<ExtForeignToplevelHandleV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_toplevel(
            toplevel,
        );
        if let Err(e) = res {
            log_forward("ext_foreign_toplevel_list_v1.toplevel", &e);
        }
    }

    /// the compositor has finished with the toplevel manager
    ///
    /// This event indicates that the compositor is done sending events
    /// to this object. The client should destroy the object.
    /// See ext_foreign_toplevel_list_v1.destroy for more information.
    ///
    /// The compositor must not send any more toplevel events after this event.
    #[inline]
    fn handle_finished(
        &mut self,
        slf: &Rc<ExtForeignToplevelListV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_finished(
        );
        if let Err(e) = res {
            log_forward("ext_foreign_toplevel_list_v1.finished", &e);
        }
    }

    /// stop sending events
    ///
    /// This request indicates that the client no longer wishes to receive
    /// events for new toplevels.
    ///
    /// The Wayland protocol is asynchronous, meaning the compositor may send
    /// further toplevel events until the stop request is processed.
    /// The client should wait for a ext_foreign_toplevel_list_v1.finished
    /// event before destroying this object.
    #[inline]
    fn handle_stop(
        &mut self,
        slf: &Rc<ExtForeignToplevelListV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_stop(
        );
        if let Err(e) = res {
            log_forward("ext_foreign_toplevel_list_v1.stop", &e);
        }
    }

    /// destroy the ext_foreign_toplevel_list_v1 object
    ///
    /// This request should be called either when the client will no longer
    /// use the ext_foreign_toplevel_list_v1 or after the finished event
    /// has been received to allow destruction of the object.
    ///
    /// If a client wishes to destroy this object it should send a
    /// ext_foreign_toplevel_list_v1.stop request and wait for a ext_foreign_toplevel_list_v1.finished
    /// event, then destroy the handles and then this object.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ExtForeignToplevelListV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("ext_foreign_toplevel_list_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for ExtForeignToplevelListV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ExtForeignToplevelListV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_foreign_toplevel_list_v1#{}.stop()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_stop(&self);
                } else {
                    DefaultHandler.handle_stop(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_foreign_toplevel_list_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_foreign_toplevel_list_v1#{}.toplevel(toplevel: ext_foreign_toplevel_handle_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = ExtForeignToplevelHandleV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetServerId(arg0_id, "toplevel", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_toplevel(&self, arg0);
                } else {
                    DefaultHandler.handle_toplevel(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_foreign_toplevel_list_v1#{}.finished()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_finished(&self);
                } else {
                    DefaultHandler.handle_finished(&self);
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
            0 => "stop",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "toplevel",
            1 => "finished",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ExtForeignToplevelListV1 {
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

