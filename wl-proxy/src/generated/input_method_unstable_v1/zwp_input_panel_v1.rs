//! interface for implementing keyboards
//!
//! Only one client can bind this interface at a time.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_input_panel_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpInputPanelV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpInputPanelV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpInputPanelV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpInputPanelV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZwpInputPanelV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpInputPanelV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaZwpInputPanelV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaZwpInputPanelV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpInputPanelV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpInputPanelV1 {
    /// Since when the get_input_panel_surface message is available.
    #[allow(dead_code)]
    pub const MSG__GET_INPUT_PANEL_SURFACE__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn send_get_input_panel_surface(
        &self,
        id: &Rc<MetaZwpInputPanelSurfaceV1>,
        surface: &Rc<MetaWlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            surface,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("surface")),
            Some(id) => id,
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
            arg1_id,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpInputPanelV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpInputPanelV1MessageHandler {
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_input_panel_surface(
        &mut self,
        _slf: &Rc<MetaZwpInputPanelV1>,
        id: &Rc<MetaZwpInputPanelSurfaceV1>,
        surface: &Rc<MetaWlSurface>,
    ) {
        let res = _slf.send_get_input_panel_surface(
            id,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_panel_v1.get_input_panel_surface message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpInputPanelV1 {
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
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg0_id = arg0;
                let arg0 = MetaZwpInputPanelSurfaceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ProxyInterface::WlSurface));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_input_panel_surface(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_input_panel_surface(&self, arg0, arg1);
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
            0 => "get_input_panel_surface",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

