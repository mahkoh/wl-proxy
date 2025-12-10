//! list and control opened apps
//!
//! The purpose of this protocol is to enable the creation of taskbars
//! and docks by providing them with a list of opened applications and
//! letting them request certain actions on them, like maximizing, etc.
//!
//! After a client binds the zwlr_foreign_toplevel_manager_v1, each opened
//! toplevel window will be sent via the toplevel event

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwlr_foreign_toplevel_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwlrForeignToplevelManagerV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn ZwlrForeignToplevelManagerV1Handler>,
}

struct DefaultHandler;

impl ZwlrForeignToplevelManagerV1Handler for DefaultHandler { }

impl ZwlrForeignToplevelManagerV1 {
    pub const XML_VERSION: u32 = 3;
}

impl ZwlrForeignToplevelManagerV1 {
    pub fn set_handler(&self, handler: impl ZwlrForeignToplevelManagerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

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
    #[allow(dead_code)]
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
    #[inline]
    pub fn send_toplevel(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateClientId("toplevel", e))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let args = format_args!("[{millis:7}.{micros:03}] client#{:<4} <= zwlr_foreign_toplevel_manager_v1#{}.toplevel(toplevel: zwlr_foreign_toplevel_handle_v1#{})\n", client.endpoint.id, id, arg0_id);
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

    /// Since when the stop message is available.
    #[allow(dead_code)]
    pub const MSG__STOP__SINCE: u32 = 1;

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
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let args = format_args!("[{millis:7}.{micros:03}] server      <= zwlr_foreign_toplevel_manager_v1#{}.stop()\n", id);
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

    /// Since when the finished message is available.
    #[allow(dead_code)]
    pub const MSG__FINISHED__SINCE: u32 = 1;

    /// the compositor has finished with the toplevel manager
    ///
    /// This event indicates that the compositor is done sending events to the
    /// zwlr_foreign_toplevel_manager_v1. The server will destroy the object
    /// immediately after sending this request, so it will become invalid and
    /// the client should free any resources associated with it.
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
            let args = format_args!("[{millis:7}.{micros:03}] client#{:<4} <= zwlr_foreign_toplevel_manager_v1#{}.finished()\n", client.endpoint.id, id);
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
        drop(fmt);
        drop(outgoing_ref);
        drop(client_ref);
        self.core.handle_client_destroy();
        Ok(())
    }
}

/// A message handler for [ZwlrForeignToplevelManagerV1] proxies.
#[allow(dead_code)]
pub trait ZwlrForeignToplevelManagerV1Handler: Any {
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
    fn toplevel(
        &mut self,
        _slf: &Rc<ZwlrForeignToplevelManagerV1>,
        toplevel: &Rc<ZwlrForeignToplevelHandleV1>,
    ) {
        let res = _slf.send_toplevel(
            toplevel,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_foreign_toplevel_manager_v1.toplevel message: {}", Report::new(e));
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
    fn stop(
        &mut self,
        _slf: &Rc<ZwlrForeignToplevelManagerV1>,
    ) {
        let res = _slf.send_stop(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_foreign_toplevel_manager_v1.stop message: {}", Report::new(e));
        }
    }

    /// the compositor has finished with the toplevel manager
    ///
    /// This event indicates that the compositor is done sending events to the
    /// zwlr_foreign_toplevel_manager_v1. The server will destroy the object
    /// immediately after sending this request, so it will become invalid and
    /// the client should free any resources associated with it.
    #[inline]
    fn finished(
        &mut self,
        _slf: &Rc<ZwlrForeignToplevelManagerV1>,
    ) {
        let res = _slf.send_finished(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_foreign_toplevel_manager_v1.finished message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for ZwlrForeignToplevelManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::ZwlrForeignToplevelManagerV1, version),
            handler: Default::default(),
        })
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let args = format_args!("[{millis:7}.{micros:03}] client#{:<4} -> zwlr_foreign_toplevel_manager_v1#{}.stop()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).stop(&self);
                } else {
                    DefaultHandler.stop(&self);
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
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let args = format_args!("[{millis:7}.{micros:03}] server      -> zwlr_foreign_toplevel_manager_v1#{}.toplevel(toplevel: zwlr_foreign_toplevel_handle_v1#{})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ZwlrForeignToplevelHandleV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetServerId(arg0_id, "toplevel", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).toplevel(&self, arg0);
                } else {
                    DefaultHandler.toplevel(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let args = format_args!("[{millis:7}.{micros:03}] server      -> zwlr_foreign_toplevel_manager_v1#{}.finished()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).finished(&self);
                } else {
                    DefaultHandler.finished(&self);
                }
                self.core.handle_server_destroy();
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

impl Proxy for ZwlrForeignToplevelManagerV1 {
    fn core(&self) -> &ProxyCore {
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

