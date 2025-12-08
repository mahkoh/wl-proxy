//! list and control opened apps
//!
//! The purpose of this protocol is to enable the creation of taskbars
//! and docks by providing them with a list of opened applications and
//! letting them request certain actions on them, like maximizing, etc.
//!
//! After a client binds the zwlr_foreign_toplevel_manager_v1, each opened
//! toplevel window will be sent via the toplevel event

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwlr_foreign_toplevel_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwlrForeignToplevelManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwlrForeignToplevelManagerV1Handler>,
}

struct DefaultHandler;

impl ZwlrForeignToplevelManagerV1Handler for DefaultHandler { }

impl ConcreteObject for ZwlrForeignToplevelManagerV1 {
    const XML_VERSION: u32 = 3;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwlrForeignToplevelManagerV1;
    const INTERFACE_NAME: &str = "zwlr_foreign_toplevel_manager_v1";
}

impl ZwlrForeignToplevelManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwlrForeignToplevelManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwlrForeignToplevelManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwlrForeignToplevelManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwlrForeignToplevelManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwlrForeignToplevelManagerV1 {
    /// Since when the toplevel message is available.
    pub const MSG__TOPLEVEL__SINCE: u32 = 1;

    /// a toplevel has been created
    ///
    /// This event is emitted whenever a new toplevel window is created. It
    /// is emitted for all toplevels, regardless of the app that has created
    /// them.
    ///
    /// All initial details of the toplevel(title, app_id, states, etc.) will
    /// be sent immediately after this event via the corresponding events in
    /// zwlr_foreign_toplevel_handle_v1.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    #[inline]
    pub fn try_send_toplevel(
        &self,
        toplevel: &Rc<ZwlrForeignToplevelHandleV1>,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_foreign_toplevel_manager_v1#{}.toplevel(toplevel: zwlr_foreign_toplevel_handle_v1#{})\n", client_id, id, arg0);
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
    /// This event is emitted whenever a new toplevel window is created. It
    /// is emitted for all toplevels, regardless of the app that has created
    /// them.
    ///
    /// All initial details of the toplevel(title, app_id, states, etc.) will
    /// be sent immediately after this event via the corresponding events in
    /// zwlr_foreign_toplevel_handle_v1.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    #[inline]
    pub fn send_toplevel(
        &self,
        toplevel: &Rc<ZwlrForeignToplevelHandleV1>,
    ) {
        let res = self.try_send_toplevel(
            toplevel,
        );
        if let Err(e) = res {
            log_send("zwlr_foreign_toplevel_manager_v1.toplevel", &e);
        }
    }

    /// a toplevel has been created
    ///
    /// This event is emitted whenever a new toplevel window is created. It
    /// is emitted for all toplevels, regardless of the app that has created
    /// them.
    ///
    /// All initial details of the toplevel(title, app_id, states, etc.) will
    /// be sent immediately after this event via the corresponding events in
    /// zwlr_foreign_toplevel_handle_v1.
    #[inline]
    pub fn new_try_send_toplevel(
        &self,
    ) -> Result<Rc<ZwlrForeignToplevelHandleV1>, ObjectError> {
        let toplevel = self.core.create_child();
        self.try_send_toplevel(
            &toplevel,
        )?;
        Ok(toplevel)
    }

    /// a toplevel has been created
    ///
    /// This event is emitted whenever a new toplevel window is created. It
    /// is emitted for all toplevels, regardless of the app that has created
    /// them.
    ///
    /// All initial details of the toplevel(title, app_id, states, etc.) will
    /// be sent immediately after this event via the corresponding events in
    /// zwlr_foreign_toplevel_handle_v1.
    #[inline]
    pub fn new_send_toplevel(
        &self,
    ) -> Rc<ZwlrForeignToplevelHandleV1> {
        let toplevel = self.core.create_child();
        self.send_toplevel(
            &toplevel,
        );
        toplevel
    }

    /// Since when the stop message is available.
    pub const MSG__STOP__SINCE: u32 = 1;

    /// stop sending events
    ///
    /// Indicates the client no longer wishes to receive events for new toplevels.
    /// However the compositor may emit further toplevel_created events, until
    /// the finished event is emitted.
    ///
    /// The client must not send any more requests after this one.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_foreign_toplevel_manager_v1#{}.stop()\n", id);
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
    /// Indicates the client no longer wishes to receive events for new toplevels.
    /// However the compositor may emit further toplevel_created events, until
    /// the finished event is emitted.
    ///
    /// The client must not send any more requests after this one.
    #[inline]
    pub fn send_stop(
        &self,
    ) {
        let res = self.try_send_stop(
        );
        if let Err(e) = res {
            log_send("zwlr_foreign_toplevel_manager_v1.stop", &e);
        }
    }

    /// Since when the finished message is available.
    pub const MSG__FINISHED__SINCE: u32 = 1;

    /// the compositor has finished with the toplevel manager
    ///
    /// This event indicates that the compositor is done sending events to the
    /// zwlr_foreign_toplevel_manager_v1. The server will destroy the object
    /// immediately after sending this request, so it will become invalid and
    /// the client should free any resources associated with it.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_foreign_toplevel_manager_v1#{}.finished()\n", client_id, id);
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

    /// the compositor has finished with the toplevel manager
    ///
    /// This event indicates that the compositor is done sending events to the
    /// zwlr_foreign_toplevel_manager_v1. The server will destroy the object
    /// immediately after sending this request, so it will become invalid and
    /// the client should free any resources associated with it.
    #[inline]
    pub fn send_finished(
        &self,
    ) {
        let res = self.try_send_finished(
        );
        if let Err(e) = res {
            log_send("zwlr_foreign_toplevel_manager_v1.finished", &e);
        }
    }
}

/// A message handler for [`ZwlrForeignToplevelManagerV1`] proxies.
pub trait ZwlrForeignToplevelManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwlrForeignToplevelManagerV1>) {
        slf.core.delete_id();
    }

    /// a toplevel has been created
    ///
    /// This event is emitted whenever a new toplevel window is created. It
    /// is emitted for all toplevels, regardless of the app that has created
    /// them.
    ///
    /// All initial details of the toplevel(title, app_id, states, etc.) will
    /// be sent immediately after this event via the corresponding events in
    /// zwlr_foreign_toplevel_handle_v1.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    #[inline]
    fn handle_toplevel(
        &mut self,
        slf: &Rc<ZwlrForeignToplevelManagerV1>,
        toplevel: &Rc<ZwlrForeignToplevelHandleV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_toplevel(
            toplevel,
        );
        if let Err(e) = res {
            log_forward("zwlr_foreign_toplevel_manager_v1.toplevel", &e);
        }
    }

    /// stop sending events
    ///
    /// Indicates the client no longer wishes to receive events for new toplevels.
    /// However the compositor may emit further toplevel_created events, until
    /// the finished event is emitted.
    ///
    /// The client must not send any more requests after this one.
    #[inline]
    fn handle_stop(
        &mut self,
        slf: &Rc<ZwlrForeignToplevelManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_stop(
        );
        if let Err(e) = res {
            log_forward("zwlr_foreign_toplevel_manager_v1.stop", &e);
        }
    }

    /// the compositor has finished with the toplevel manager
    ///
    /// This event indicates that the compositor is done sending events to the
    /// zwlr_foreign_toplevel_manager_v1. The server will destroy the object
    /// immediately after sending this request, so it will become invalid and
    /// the client should free any resources associated with it.
    #[inline]
    fn handle_finished(
        &mut self,
        slf: &Rc<ZwlrForeignToplevelManagerV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_finished(
        );
        if let Err(e) = res {
            log_forward("zwlr_foreign_toplevel_manager_v1.finished", &e);
        }
    }
}

impl ObjectPrivate for ZwlrForeignToplevelManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwlrForeignToplevelManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_foreign_toplevel_manager_v1#{}.stop()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_foreign_toplevel_manager_v1#{}.toplevel(toplevel: zwlr_foreign_toplevel_handle_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = ZwlrForeignToplevelHandleV1::new(&self.core.state, self.core.version);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_foreign_toplevel_manager_v1#{}.finished()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                self.core.handle_server_destroy();
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

impl Object for ZwlrForeignToplevelManagerV1 {
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

