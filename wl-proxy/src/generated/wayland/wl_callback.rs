//! callback object
//!
//! Clients can handle the 'done' event to get notified when
//! the related request is done.
//!
//! Note, because wl_callback objects are created from multiple independent
//! factory interfaces, the wl_callback interface is frozen at version 1.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wl_callback proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWlCallback {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWlCallbackMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWlCallbackMessageHandler for DefaultMessageHandler { }

impl MetaWlCallback {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWlCallback {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WlCallback, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWlCallback {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWlCallback")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWlCallback {
    /// Since when the done message is available.
    #[allow(dead_code)]
    pub const MSG__DONE__SINCE: u32 = 1;

    /// done event
    ///
    /// Notify the client when the related request is done.
    ///
    /// # Arguments
    ///
    /// - `callback_data`: request-specific data for the callback
    #[inline]
    pub fn send_done(
        &self,
        callback_data: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            callback_data,
        );
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
            0,
            arg0,
        ]);
        drop(fmt);
        drop(outgoing_ref);
        drop(client_ref);
        self.core.handle_client_destroy();
        Ok(())
    }
}

/// A message handler for [WlCallback] proxies.
#[allow(dead_code)]
pub trait MetaWlCallbackMessageHandler {
    /// done event
    ///
    /// Notify the client when the related request is done.
    ///
    /// # Arguments
    ///
    /// - `callback_data`: request-specific data for the callback
    #[inline]
    fn done(
        &mut self,
        _slf: &Rc<MetaWlCallback>,
        callback_data: u32,
    ) {
        let res = _slf.send_done(
            callback_data,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_callback.done message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWlCallback {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            _ => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
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
                if let Some(handler) = handler {
                    (**handler).done(&self, arg0);
                } else {
                    DefaultMessageHandler.done(&self, arg0);
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

