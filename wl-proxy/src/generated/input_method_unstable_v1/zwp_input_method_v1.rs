//! input method
//!
//! An input method object is responsible for composing text in response to
//! input from hardware or virtual keyboards. There is one input method
//! object per seat. On activate there is a new input method context object
//! created which allows the input method to communicate with the text input.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_input_method_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpInputMethodV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpInputMethodV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpInputMethodV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpInputMethodV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZwpInputMethodV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpInputMethodV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwpInputMethodV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpInputMethodV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpInputMethodV1 {
    /// Since when the activate message is available.
    #[allow(dead_code)]
    pub const MSG__ACTIVATE__SINCE: u32 = 1;

    /// activate event
    ///
    /// A text input was activated. Creates an input method context object
    /// which allows communication with the text input.
    #[inline]
    pub fn send_activate(
        &self,
        id: &Rc<MetaZwpInputMethodContextV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
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
            .map_err(|e| ObjectError::GenerateClientId("id", e))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= zwp_input_method_v1#{}.activate(id: zwp_input_method_context_v1#{})", client.endpoint.id, id, arg0_id);
        let endpoint = &client.endpoint;
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

    /// Since when the deactivate message is available.
    #[allow(dead_code)]
    pub const MSG__DEACTIVATE__SINCE: u32 = 1;

    /// deactivate event
    ///
    /// The text input corresponding to the context argument was deactivated.
    /// The input method context should be destroyed after deactivation is
    /// handled.
    ///
    /// # Arguments
    ///
    /// - `context`:
    #[inline]
    pub fn send_deactivate(
        &self,
        context: &Rc<MetaZwpInputMethodContextV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            context,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError::ArgNoClientId("context", client.endpoint.id));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= zwp_input_method_v1#{}.deactivate(context: zwp_input_method_context_v1#{})", client.endpoint.id, id, arg0_id);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0_id,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpInputMethodV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpInputMethodV1MessageHandler {
    /// activate event
    ///
    /// A text input was activated. Creates an input method context object
    /// which allows communication with the text input.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn activate(
        &mut self,
        _slf: &Rc<MetaZwpInputMethodV1>,
        id: &Rc<MetaZwpInputMethodContextV1>,
    ) {
        let res = _slf.send_activate(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_v1.activate message: {}", Report::new(e));
        }
    }

    /// deactivate event
    ///
    /// The text input corresponding to the context argument was deactivated.
    /// The input method context should be destroyed after deactivation is
    /// handled.
    ///
    /// # Arguments
    ///
    /// - `context`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn deactivate(
        &mut self,
        _slf: &Rc<MetaZwpInputMethodV1>,
        context: &Rc<MetaZwpInputMethodContextV1>,
    ) {
        if let Some(client_id) = _slf.core.client_id.get() {
            if let Some(client_id_2) = context.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = _slf.send_deactivate(
            context,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_v1.deactivate message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpInputMethodV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            n => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
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
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                eprintln!("server      -> zwp_input_method_v1#{}.activate(id: zwp_input_method_context_v1#{})", msg[0], arg0);
                let arg0_id = arg0;
                let arg0 = MetaZwpInputMethodContextV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetServerId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).activate(&self, arg0);
                } else {
                    DefaultMessageHandler.activate(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                eprintln!("server      -> zwp_input_method_v1#{}.deactivate(context: zwp_input_method_context_v1#{})", msg[0], arg0);
                let arg0_id = arg0;
                let Some(arg0) = self.core.state.server.lookup(arg0_id) else {
                    return Err(ObjectError::NoServerObject(arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaZwpInputMethodContextV1>() else {
                    let o = self.core.state.server.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("context", o.core().interface, ProxyInterface::ZwpInputMethodContextV1));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).deactivate(&self, arg0);
                } else {
                    DefaultMessageHandler.deactivate(&self, arg0);
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
        let _ = id;
        None
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "activate",
            1 => "deactivate",
            _ => return None,
        };
        Some(name)
    }
}

