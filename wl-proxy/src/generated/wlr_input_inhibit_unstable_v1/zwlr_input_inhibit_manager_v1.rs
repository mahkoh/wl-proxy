//! inhibits input events to other clients
//!
//! Clients can use this interface to prevent input events from being sent to
//! any surfaces but its own, which is useful for example in lock screen
//! software. It is assumed that access to this interface will be locked down
//! to whitelisted clients by the compositor.
//!
//! Note! This protocol is deprecated and not intended for production use.
//! For screen lockers, use the ext-session-lock-v1 protocol.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwlr_input_inhibit_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwlrInputInhibitManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwlrInputInhibitManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwlrInputInhibitManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaZwlrInputInhibitManagerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZwlrInputInhibitManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwlrInputInhibitManagerV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaZwlrInputInhibitManagerV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaZwlrInputInhibitManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwlrInputInhibitManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwlrInputInhibitManagerV1 {
    /// Since when the get_inhibitor message is available.
    #[allow(dead_code)]
    pub const MSG__GET_INHIBITOR__SINCE: u32 = 1;

    /// inhibit input to other clients
    ///
    /// Activates the input inhibitor. As long as the inhibitor is active, the
    /// compositor will not send input events to other clients.
    #[inline]
    pub fn send_get_inhibitor(
        &self,
        id: &Rc<MetaZwlrInputInhibitorV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
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
            arg0_id,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwlrInputInhibitManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaZwlrInputInhibitManagerV1MessageHandler {
    /// inhibit input to other clients
    ///
    /// Activates the input inhibitor. As long as the inhibitor is active, the
    /// compositor will not send input events to other clients.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn get_inhibitor(
        &mut self,
        _slf: &Rc<MetaZwlrInputInhibitManagerV1>,
        id: &Rc<MetaZwlrInputInhibitorV1>,
    ) {
        let res = _slf.send_get_inhibitor(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_input_inhibit_manager_v1.get_inhibitor message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwlrInputInhibitManagerV1 {
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0_id = arg0;
                let arg0 = MetaZwlrInputInhibitorV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).get_inhibitor(&self, arg0);
                } else {
                    DefaultMessageHandler.get_inhibitor(&self, arg0);
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
            0 => "get_inhibitor",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl MetaZwlrInputInhibitManagerV1 {
    /// Since when the error.already_inhibited enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ALREADY_INHIBITED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaZwlrInputInhibitManagerV1Error(pub u32);

impl MetaZwlrInputInhibitManagerV1Error {
    /// an input inhibitor is already in use on the compositor
    #[allow(dead_code)]
    pub const ALREADY_INHIBITED: Self = Self(0);
}

impl Debug for MetaZwlrInputInhibitManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ALREADY_INHIBITED => "ALREADY_INHIBITED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
