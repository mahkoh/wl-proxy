//! region interface
//!
//! A region object describes an area.
//!
//! Region objects are used to describe the opaque and input
//! regions of a surface.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wl_region proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWlRegion {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWlRegionMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWlRegionMessageHandler for DefaultMessageHandler { }

impl MetaWlRegion {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWlRegion {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WlRegion, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWlRegion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWlRegion")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWlRegion {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy region
    ///
    /// Destroy the region.  This will invalidate the object ID.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= wl_region#{}.destroy()", id);
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

    /// Since when the add message is available.
    #[allow(dead_code)]
    pub const MSG__ADD__SINCE: u32 = 1;

    /// add rectangle to region
    ///
    /// Add the specified rectangle to the region.
    ///
    /// # Arguments
    ///
    /// - `x`: region-local x coordinate
    /// - `y`: region-local y coordinate
    /// - `width`: rectangle width
    /// - `height`: rectangle height
    #[inline]
    pub fn send_add(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            x,
            y,
            width,
            height,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= wl_region#{}.add(x: {}, y: {}, width: {}, height: {})", id, arg0, arg1, arg2, arg3);
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
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// Since when the subtract message is available.
    #[allow(dead_code)]
    pub const MSG__SUBTRACT__SINCE: u32 = 1;

    /// subtract rectangle from region
    ///
    /// Subtract the specified rectangle from the region.
    ///
    /// # Arguments
    ///
    /// - `x`: region-local x coordinate
    /// - `y`: region-local y coordinate
    /// - `width`: rectangle width
    /// - `height`: rectangle height
    #[inline]
    pub fn send_subtract(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            x,
            y,
            width,
            height,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= wl_region#{}.subtract(x: {}, y: {}, width: {}, height: {})", id, arg0, arg1, arg2, arg3);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }
}

/// A message handler for [WlRegion] proxies.
#[allow(dead_code)]
pub trait MetaWlRegionMessageHandler {
    /// destroy region
    ///
    /// Destroy the region.  This will invalidate the object ID.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWlRegion>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_region.destroy message: {}", Report::new(e));
        }
    }

    /// add rectangle to region
    ///
    /// Add the specified rectangle to the region.
    ///
    /// # Arguments
    ///
    /// - `x`: region-local x coordinate
    /// - `y`: region-local y coordinate
    /// - `width`: rectangle width
    /// - `height`: rectangle height
    #[inline]
    fn add(
        &mut self,
        _slf: &Rc<MetaWlRegion>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = _slf.send_add(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_region.add message: {}", Report::new(e));
        }
    }

    /// subtract rectangle from region
    ///
    /// Subtract the specified rectangle from the region.
    ///
    /// # Arguments
    ///
    /// - `x`: region-local x coordinate
    /// - `y`: region-local y coordinate
    /// - `width`: rectangle width
    /// - `height`: rectangle height
    #[inline]
    fn subtract(
        &mut self,
        _slf: &Rc<MetaWlRegion>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = _slf.send_subtract(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_region.subtract message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWlRegion {
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
                eprintln!("client#{:04} -> wl_region#{}.destroy()", client.endpoint.id, msg[0]);
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            1 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 24));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                eprintln!("client#{:04} -> wl_region#{}.add(x: {}, y: {}, width: {}, height: {})", client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                if let Some(handler) = handler {
                    (**handler).add(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultMessageHandler.add(&self, arg0, arg1, arg2, arg3);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 24));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                eprintln!("client#{:04} -> wl_region#{}.subtract(x: {}, y: {}, width: {}, height: {})", client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                if let Some(handler) = handler {
                    (**handler).subtract(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultMessageHandler.subtract(&self, arg0, arg1, arg2, arg3);
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
            0 => "destroy",
            1 => "add",
            2 => "subtract",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

