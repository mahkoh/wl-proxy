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

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_foreign_toplevel_list_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaExtForeignToplevelListV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaExtForeignToplevelListV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaExtForeignToplevelListV1MessageHandler for DefaultMessageHandler { }

impl MetaExtForeignToplevelListV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaExtForeignToplevelListV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaExtForeignToplevelListV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaExtForeignToplevelListV1 {
    /// Since when the toplevel message is available.
    #[allow(dead_code)]
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
        toplevel: &Rc<MetaExtForeignToplevelHandleV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            toplevel,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        arg0.generate_client_id(client, arg0_obj.clone())?;
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
            arg0.client_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }

    /// Since when the finished message is available.
    #[allow(dead_code)]
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
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            1,
        ]);
        Ok(())
    }

    /// Since when the stop message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
        ]);
        Ok(())
    }
}

/// A message handler for [ExtForeignToplevelListV1] proxies.
#[allow(dead_code)]
pub trait MetaExtForeignToplevelListV1MessageHandler {
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
    fn toplevel(
        &mut self,
        _slf: &Rc<MetaExtForeignToplevelListV1>,
        toplevel: &Rc<MetaExtForeignToplevelHandleV1>,
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
    fn finished(
        &mut self,
        _slf: &Rc<MetaExtForeignToplevelListV1>,
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
    fn stop(
        &mut self,
        _slf: &Rc<MetaExtForeignToplevelListV1>,
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
    fn destroy(
        &mut self,
        _slf: &Rc<MetaExtForeignToplevelListV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_foreign_toplevel_list_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaExtForeignToplevelListV1 {
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
            1 => {
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
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
                let arg0 = MetaExtForeignToplevelHandleV1::new(&self.core.state, self.core.version);
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

