//! the compositor singleton
//!
//! A compositor.  This object is a singleton global.  The
//! compositor is in charge of combining the contents of multiple
//! surfaces into one displayable output.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wl_compositor proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWlCompositor {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWlCompositorMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWlCompositorMessageHandler for DefaultMessageHandler { }

impl MetaWlCompositor {
    pub const XML_VERSION: u32 = 6;
}

impl MetaWlCompositor {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WlCompositor, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWlCompositor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWlCompositor")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWlCompositor {
    /// Since when the create_surface message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE_SURFACE__SINCE: u32 = 1;

    /// create new surface
    ///
    /// Ask the compositor to create a new surface.
    #[inline]
    pub fn send_create_surface(
        &self,
        id: &Rc<MetaWlSurface>,
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
            return Err(ObjectError);
        };
        arg0.generate_server_id(arg0_obj.clone())?;
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
            arg0.server_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }

    /// Since when the create_region message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE_REGION__SINCE: u32 = 1;

    /// create new region
    ///
    /// Ask the compositor to create a new region.
    #[inline]
    pub fn send_create_region(
        &self,
        id: &Rc<MetaWlRegion>,
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
            return Err(ObjectError);
        };
        arg0.generate_server_id(arg0_obj.clone())?;
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
            arg0.server_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }
}

/// A message handler for [WlCompositor] proxies.
#[allow(dead_code)]
pub trait MetaWlCompositorMessageHandler {
    /// create new surface
    ///
    /// Ask the compositor to create a new surface.
    ///
    /// # Arguments
    ///
    /// - `id`: the new surface
    #[inline]
    fn create_surface(
        &mut self,
        _slf: &Rc<MetaWlCompositor>,
        id: &Rc<MetaWlSurface>,
    ) {
        let res = _slf.send_create_surface(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_compositor.create_surface message: {}", Report::new(e));
        }
    }

    /// create new region
    ///
    /// Ask the compositor to create a new region.
    ///
    /// # Arguments
    ///
    /// - `id`: the new region
    #[inline]
    fn create_region(
        &mut self,
        _slf: &Rc<MetaWlCompositor>,
        id: &Rc<MetaWlRegion>,
    ) {
        let res = _slf.send_create_region(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_compositor.create_region message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWlCompositor {
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
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaWlSurface::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).create_surface(&self, arg0);
                } else {
                    DefaultMessageHandler.create_surface(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaWlRegion::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).create_region(&self, arg0);
                } else {
                    DefaultMessageHandler.create_region(&self, arg0);
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

