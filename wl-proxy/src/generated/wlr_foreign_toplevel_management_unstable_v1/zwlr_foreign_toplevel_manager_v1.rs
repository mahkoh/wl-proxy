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
pub struct MetaZwlrForeignToplevelManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwlrForeignToplevelManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwlrForeignToplevelManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaZwlrForeignToplevelManagerV1 {
    pub const XML_VERSION: u32 = 3;
}

impl MetaZwlrForeignToplevelManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwlrForeignToplevelManagerV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwlrForeignToplevelManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwlrForeignToplevelManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwlrForeignToplevelManagerV1 {
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
        toplevel: &Rc<MetaZwlrForeignToplevelHandleV1>,
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
            return Err(ObjectError);
        };
        arg0.generate_client_id(client, arg0_obj.clone())?;
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
            arg0.client_obj_id.get().unwrap_or(0),
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
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
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
pub trait MetaZwlrForeignToplevelManagerV1MessageHandler {
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
        _slf: &Rc<MetaZwlrForeignToplevelManagerV1>,
        toplevel: &Rc<MetaZwlrForeignToplevelHandleV1>,
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
        _slf: &Rc<MetaZwlrForeignToplevelManagerV1>,
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
        _slf: &Rc<MetaZwlrForeignToplevelManagerV1>,
    ) {
        let res = _slf.send_finished(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_foreign_toplevel_manager_v1.finished message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwlrForeignToplevelManagerV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if let Some(handler) = handler {
                    (**handler).stop(&self);
                } else {
                    DefaultMessageHandler.stop(&self);
                }
            }
            _ => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
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
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaZwlrForeignToplevelHandleV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).toplevel(&self, arg0);
                } else {
                    DefaultMessageHandler.toplevel(&self, arg0);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).finished(&self);
                } else {
                    DefaultMessageHandler.finished(&self);
                }
                self.core.handle_server_destroy();
            }
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }
}

