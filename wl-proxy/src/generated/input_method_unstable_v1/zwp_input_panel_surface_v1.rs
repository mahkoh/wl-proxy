use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_input_panel_surface_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpInputPanelSurfaceV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpInputPanelSurfaceV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpInputPanelSurfaceV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpInputPanelSurfaceV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZwpInputPanelSurfaceV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpInputPanelSurfaceV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwpInputPanelSurfaceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpInputPanelSurfaceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpInputPanelSurfaceV1 {
    /// Since when the set_toplevel message is available.
    #[allow(dead_code)]
    pub const MSG__SET_TOPLEVEL__SINCE: u32 = 1;

    /// set the surface type as a keyboard
    ///
    /// Set the input_panel_surface type to keyboard.
    ///
    /// A keyboard surface is only shown when a text input is active.
    ///
    /// # Arguments
    ///
    /// - `output`:
    /// - `position`:
    #[inline]
    pub fn send_set_toplevel(
        &self,
        output: &Rc<MetaWlOutput>,
        position: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            output,
            position,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg0 = match arg0.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
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
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// Since when the set_overlay_panel message is available.
    #[allow(dead_code)]
    pub const MSG__SET_OVERLAY_PANEL__SINCE: u32 = 1;

    /// set the surface type as an overlay panel
    ///
    /// Set the input_panel_surface to be an overlay panel.
    ///
    /// This is shown near the input cursor above the application window when
    /// a text input is active.
    #[inline]
    pub fn send_set_overlay_panel(
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
            1,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpInputPanelSurfaceV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpInputPanelSurfaceV1MessageHandler {
    /// set the surface type as a keyboard
    ///
    /// Set the input_panel_surface type to keyboard.
    ///
    /// A keyboard surface is only shown when a text input is active.
    ///
    /// # Arguments
    ///
    /// - `output`:
    /// - `position`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn set_toplevel(
        &mut self,
        _slf: &Rc<MetaZwpInputPanelSurfaceV1>,
        output: &Rc<MetaWlOutput>,
        position: u32,
    ) {
        let res = _slf.send_set_toplevel(
            output,
            position,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_panel_surface_v1.set_toplevel message: {}", Report::new(e));
        }
    }

    /// set the surface type as an overlay panel
    ///
    /// Set the input_panel_surface to be an overlay panel.
    ///
    /// This is shown near the input cursor above the application window when
    /// a text input is active.
    #[inline]
    fn set_overlay_panel(
        &mut self,
        _slf: &Rc<MetaZwpInputPanelSurfaceV1>,
    ) {
        let res = _slf.send_set_overlay_panel(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_panel_surface_v1.set_overlay_panel message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpInputPanelSurfaceV1 {
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
                let Some(arg0) = client.endpoint.lookup(arg0) else {
                    return Err(ObjectError);
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlOutput>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).set_toplevel(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.set_toplevel(&self, arg0, arg1);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).set_overlay_panel(&self);
                } else {
                    DefaultMessageHandler.set_overlay_panel(&self);
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

impl MetaZwpInputPanelSurfaceV1 {
    /// Since when the position.center_bottom enum variant is available.
    #[allow(dead_code)]
    pub const ENM__POSITION_CENTER_BOTTOM__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaZwpInputPanelSurfaceV1Position(pub u32);

impl MetaZwpInputPanelSurfaceV1Position {
    #[allow(dead_code)]
    pub const CENTER_BOTTOM: Self = Self(0);
}

impl Debug for MetaZwpInputPanelSurfaceV1Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::CENTER_BOTTOM => "CENTER_BOTTOM",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
