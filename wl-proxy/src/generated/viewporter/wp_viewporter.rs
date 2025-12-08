//! surface cropping and scaling
//!
//! The global interface exposing surface cropping and scaling
//! capabilities is used to instantiate an interface extension for a
//! wl_surface object. This extended interface will then allow
//! cropping and scaling the surface contents, effectively
//! disconnecting the direct relationship between the buffer and the
//! surface size.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_viewporter proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpViewporter {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpViewporterMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpViewporterMessageHandler for DefaultMessageHandler { }

impl MetaWpViewporter {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWpViewporter {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WpViewporter, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpViewporter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpViewporter")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpViewporter {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// unbind from the cropping and scaling interface
    ///
    /// Informs the server that the client will not be using this
    /// protocol object anymore. This does not affect any other objects,
    /// wp_viewport objects included.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= wp_viewporter#{}.destroy()", id);
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

    /// Since when the get_viewport message is available.
    #[allow(dead_code)]
    pub const MSG__GET_VIEWPORT__SINCE: u32 = 1;

    /// extend surface interface for crop and scale
    ///
    /// Instantiate an interface extension for the given wl_surface to
    /// crop and scale its content. If the given wl_surface already has
    /// a wp_viewport object associated, the viewport_exists
    /// protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `id`: the new viewport interface id
    /// - `surface`: the surface
    #[inline]
    pub fn send_get_viewport(
        &self,
        id: &Rc<MetaWpViewport>,
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
        eprintln!("server      <= wp_viewporter#{}.get_viewport(id: wp_viewport#{}, surface: wl_surface#{})", id, arg0_id, arg1_id);
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
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }
}

/// A message handler for [WpViewporter] proxies.
#[allow(dead_code)]
pub trait MetaWpViewporterMessageHandler {
    /// unbind from the cropping and scaling interface
    ///
    /// Informs the server that the client will not be using this
    /// protocol object anymore. This does not affect any other objects,
    /// wp_viewport objects included.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpViewporter>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_viewporter.destroy message: {}", Report::new(e));
        }
    }

    /// extend surface interface for crop and scale
    ///
    /// Instantiate an interface extension for the given wl_surface to
    /// crop and scale its content. If the given wl_surface already has
    /// a wp_viewport object associated, the viewport_exists
    /// protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `id`: the new viewport interface id
    /// - `surface`: the surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_viewport(
        &mut self,
        _slf: &Rc<MetaWpViewporter>,
        id: &Rc<MetaWpViewport>,
        surface: &Rc<MetaWlSurface>,
    ) {
        let res = _slf.send_get_viewport(
            id,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_viewporter.get_viewport message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpViewporter {
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
                eprintln!("client#{:04} -> wp_viewporter#{}.destroy()", client.endpoint.id, msg[0]);
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
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                eprintln!("client#{:04} -> wp_viewporter#{}.get_viewport(id: wp_viewport#{}, surface: wl_surface#{})", client.endpoint.id, msg[0], arg0, arg1);
                let arg0_id = arg0;
                let arg0 = MetaWpViewport::new(&self.core.state, self.core.version);
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
                    (**handler).get_viewport(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_viewport(&self, arg0, arg1);
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
            1 => "get_viewport",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl MetaWpViewporter {
    /// Since when the error.viewport_exists enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_VIEWPORT_EXISTS__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWpViewporterError(pub u32);

impl MetaWpViewporterError {
    /// the surface already has a viewport object associated
    #[allow(dead_code)]
    pub const VIEWPORT_EXISTS: Self = Self(0);
}

impl Debug for MetaWpViewporterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::VIEWPORT_EXISTS => "VIEWPORT_EXISTS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
