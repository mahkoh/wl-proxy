//! inhibits input to other clients
//!
//! While this resource exists, input to clients other than the owner of the
//! inhibitor resource will not receive input events. Any client which
//! previously had focus will receive a leave event and will not be given
//! focus again. The client that owns this resource will receive all input
//! events normally. The compositor will also disable all of its own input
//! processing (such as keyboard shortcuts) while the inhibitor is active.
//!
//! The compositor may continue to send input events to selected clients,
//! such as an on-screen keyboard (via the input-method protocol).

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwlr_input_inhibitor_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwlrInputInhibitorV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwlrInputInhibitorV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwlrInputInhibitorV1MessageHandler for DefaultMessageHandler { }

impl MetaZwlrInputInhibitorV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwlrInputInhibitorV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwlrInputInhibitorV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwlrInputInhibitorV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the input inhibitor object
    ///
    /// Destroy the inhibitor and allow other clients to receive input.
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
            0,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwlrInputInhibitorV1] proxies.
#[allow(dead_code)]
pub trait MetaZwlrInputInhibitorV1MessageHandler {
    /// destroy the input inhibitor object
    ///
    /// Destroy the inhibitor and allow other clients to receive input.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwlrInputInhibitorV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_input_inhibitor_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwlrInputInhibitorV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
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
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
    }
}

