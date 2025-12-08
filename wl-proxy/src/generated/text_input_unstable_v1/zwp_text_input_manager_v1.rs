//! text input manager
//!
//! A factory for text_input objects. This object is a global singleton.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_text_input_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpTextInputManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpTextInputManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpTextInputManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpTextInputManagerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZwpTextInputManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpTextInputManagerV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwpTextInputManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpTextInputManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpTextInputManagerV1 {
    /// Since when the create_text_input message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE_TEXT_INPUT__SINCE: u32 = 1;

    /// create text input
    ///
    /// Creates a new text_input object.
    #[inline]
    pub fn send_create_text_input(
        &self,
        id: &Rc<MetaZwpTextInputV1>,
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
        eprintln!("server      <= zwp_text_input_manager_v1#{}.create_text_input(id: zwp_text_input_v1#{})", id, arg0_id);
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

/// A message handler for [ZwpTextInputManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpTextInputManagerV1MessageHandler {
    /// create text input
    ///
    /// Creates a new text_input object.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn create_text_input(
        &mut self,
        _slf: &Rc<MetaZwpTextInputManagerV1>,
        id: &Rc<MetaZwpTextInputV1>,
    ) {
        let res = _slf.send_create_text_input(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_manager_v1.create_text_input message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpTextInputManagerV1 {
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
                eprintln!("client#{:04} -> zwp_text_input_manager_v1#{}.create_text_input(id: zwp_text_input_v1#{})", client.endpoint.id, msg[0], arg0);
                let arg0_id = arg0;
                let arg0 = MetaZwpTextInputV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).create_text_input(&self, arg0);
                } else {
                    DefaultMessageHandler.create_text_input(&self, arg0);
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
            0 => "create_text_input",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

