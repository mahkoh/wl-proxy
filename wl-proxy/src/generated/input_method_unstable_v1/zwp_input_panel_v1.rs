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
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
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
            return Err(ObjectError);
        };
        let arg1 = match arg1.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
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
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaZwpInputPanelSurfaceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_input_panel_surface(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_input_panel_surface(&self, arg0, arg1);
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

