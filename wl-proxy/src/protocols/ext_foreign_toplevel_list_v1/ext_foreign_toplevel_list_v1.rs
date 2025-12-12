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

impl ExtForeignToplevelListV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::ExtForeignToplevelListV1;
    pub const INTERFACE_NAME: &str = "ext_foreign_toplevel_list_v1";
}

impl ExtForeignToplevelListV1 {
    pub fn set_handler(&self, handler: impl ExtForeignToplevelListV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

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
    #[inline]
    pub fn send_toplevel(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateClientId("toplevel", e))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_foreign_toplevel_list_v1#{}.toplevel(toplevel: ext_foreign_toplevel_handle_v1#{})\n", client.endpoint.id, id, arg0_id);
            self.core.state.log(args);
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
    pub fn send_finished(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_foreign_toplevel_list_v1#{}.finished()\n", client.endpoint.id, id);
            self.core.state.log(args);
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
    pub fn send_stop(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_foreign_toplevel_list_v1#{}.stop()\n", id);
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
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
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_foreign_toplevel_list_v1#{}.destroy()\n", id);
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
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
}

/// A message handler for [ExtForeignToplevelListV1] proxies.
pub trait ExtForeignToplevelListV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ExtForeignToplevelListV1>) {
        let _ = slf.core.delete_id();
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
        _slf: &Rc<ExtForeignToplevelListV1>,
        toplevel: &Rc<ExtForeignToplevelHandleV1>,
    ) {
        let res = _slf.send_toplevel(
            toplevel,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_foreign_toplevel_list_v1.toplevel message: {}", Report::new(e));
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
        _slf: &Rc<ExtForeignToplevelListV1>,
    ) {
        let res = _slf.send_finished(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_foreign_toplevel_list_v1.finished message: {}", Report::new(e));
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
        _slf: &Rc<ExtForeignToplevelListV1>,
    ) {
        let res = _slf.send_stop(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_foreign_toplevel_list_v1.stop message: {}", Report::new(e));
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
        _slf: &Rc<ExtForeignToplevelListV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_foreign_toplevel_list_v1.destroy message: {}", Report::new(e));
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
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err((ObjectError::HandlerBorrowed, self));
        };
        if let Some(handler) = &mut *handler {
            handler.delete_id(&self);
        } else {
            let _ = self.core.delete_id();
        }
        Ok(())
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_foreign_toplevel_list_v1#{}.stop()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_stop(&self);
                } else {
                    DefaultHandler.handle_stop(&self);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_foreign_toplevel_list_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
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
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_foreign_toplevel_list_v1#{}.toplevel(toplevel: ext_foreign_toplevel_handle_v1#{})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ExtForeignToplevelHandleV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetServerId(arg0_id, "toplevel", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_toplevel(&self, arg0);
                } else {
                    DefaultHandler.handle_toplevel(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_foreign_toplevel_list_v1#{}.finished()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_finished(&self);
                } else {
                    DefaultHandler.handle_finished(&self);
                }
            }
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
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

    fn get_handler_any_ref(&self) -> Result<Ref<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(Ref::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<RefMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow_mut().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(RefMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

