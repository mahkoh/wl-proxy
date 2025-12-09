//! context object for inhibiting idle behavior
//!
//! An idle inhibitor prevents the output that the associated surface is
//! visible on from being set to a state where it is not visually usable due
//! to lack of user interaction (e.g. blanked, dimmed, locked, set to power
//! save, etc.)  Any screensaver processes are also blocked from displaying.
//!
//! If the surface is destroyed, unmapped, becomes occluded, loses
//! visibility, or otherwise becomes not visually relevant for the user, the
//! idle inhibitor will not be honored by the compositor; if the surface
//! subsequently regains visibility the inhibitor takes effect once again.
//! Likewise, the inhibitor isn't honored if the system was already idled at
//! the time the inhibitor was established, although if the system later
//! de-idles and re-idles the inhibitor will take effect.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_idle_inhibitor_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpIdleInhibitorV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpIdleInhibitorV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpIdleInhibitorV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpIdleInhibitorV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZwpIdleInhibitorV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpIdleInhibitorV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaZwpIdleInhibitorV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaZwpIdleInhibitorV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpIdleInhibitorV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpIdleInhibitorV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the idle inhibitor object
    ///
    /// Remove the inhibitor effect from the associated wl_surface.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
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
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [ZwpIdleInhibitorV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpIdleInhibitorV1MessageHandler {
    /// destroy the idle inhibitor object
    ///
    /// Remove the inhibitor effect from the associated wl_surface.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpIdleInhibitorV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_idle_inhibitor_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpIdleInhibitorV1 {
    fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Self::new(state, version)
    }

    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
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
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

