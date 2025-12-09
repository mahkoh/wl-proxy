//! color representation manager singleton
//!
//! A singleton global interface used for getting color representation
//! extensions for wl_surface. The extension interfaces allow setting the
//! color representation of surfaces.
//!
//! Compositors should never remove this global.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_color_representation_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpColorRepresentationManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpColorRepresentationManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpColorRepresentationManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaWpColorRepresentationManagerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWpColorRepresentationManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WpColorRepresentationManagerV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaWpColorRepresentationManagerV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaWpColorRepresentationManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpColorRepresentationManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpColorRepresentationManagerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the manager
    ///
    /// Destroy the wp_color_representation_manager_v1 object. This does not
    /// affect any other objects in any way.
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

    /// Since when the get_surface message is available.
    #[allow(dead_code)]
    pub const MSG__GET_SURFACE__SINCE: u32 = 1;

    /// create a color representation interface for a wl_surface
    ///
    /// If a wp_color_representation_surface_v1 object already exists for the
    /// given wl_surface, the protocol error surface_exists is raised.
    ///
    /// This creates a new color wp_color_representation_surface_v1 object for
    /// the given wl_surface.
    ///
    /// See the wp_color_representation_surface_v1 interface for more details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn send_get_surface(
        &self,
        id: &Rc<MetaWpColorRepresentationSurfaceV1>,
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
            1,
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// Since when the supported_alpha_mode message is available.
    #[allow(dead_code)]
    pub const MSG__SUPPORTED_ALPHA_MODE__SINCE: u32 = 1;

    /// supported alpha modes
    ///
    /// When this object is created, it shall immediately send this event once
    /// for each alpha mode the compositor supports.
    ///
    /// For the definition of the supported values, see the
    /// wp_color_representation_surface_v1::alpha_mode enum.
    ///
    /// # Arguments
    ///
    /// - `alpha_mode`: supported alpha mode
    #[inline]
    pub fn send_supported_alpha_mode(
        &self,
        alpha_mode: MetaWpColorRepresentationSurfaceV1AlphaMode,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            alpha_mode,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the supported_coefficients_and_ranges message is available.
    #[allow(dead_code)]
    pub const MSG__SUPPORTED_COEFFICIENTS_AND_RANGES__SINCE: u32 = 1;

    /// supported matrix coefficients and ranges
    ///
    /// When this object is created, it shall immediately send this event once
    /// for each matrix coefficient and color range combination the compositor
    /// supports.
    ///
    /// For the definition of the supported values, see the
    /// wp_color_representation_surface_v1::coefficients and
    /// wp_color_representation_surface_v1::range enums.
    ///
    /// # Arguments
    ///
    /// - `coefficients`: supported matrix coefficients
    /// - `range`: full range flag
    #[inline]
    pub fn send_supported_coefficients_and_ranges(
        &self,
        coefficients: MetaWpColorRepresentationSurfaceV1Coefficients,
        range: MetaWpColorRepresentationSurfaceV1Range,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            coefficients,
            range,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
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
            arg0.0,
            arg1.0,
        ]);
        Ok(())
    }

    /// Since when the done message is available.
    #[allow(dead_code)]
    pub const MSG__DONE__SINCE: u32 = 1;

    /// all features have been sent
    ///
    /// This event is sent when all supported features have been sent.
    #[inline]
    pub fn send_done(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
        ]);
        Ok(())
    }
}

/// A message handler for [WpColorRepresentationManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaWpColorRepresentationManagerV1MessageHandler {
    /// destroy the manager
    ///
    /// Destroy the wp_color_representation_manager_v1 object. This does not
    /// affect any other objects in any way.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpColorRepresentationManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_color_representation_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// create a color representation interface for a wl_surface
    ///
    /// If a wp_color_representation_surface_v1 object already exists for the
    /// given wl_surface, the protocol error surface_exists is raised.
    ///
    /// This creates a new color wp_color_representation_surface_v1 object for
    /// the given wl_surface.
    ///
    /// See the wp_color_representation_surface_v1 interface for more details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_surface(
        &mut self,
        _slf: &Rc<MetaWpColorRepresentationManagerV1>,
        id: &Rc<MetaWpColorRepresentationSurfaceV1>,
        surface: &Rc<MetaWlSurface>,
    ) {
        let res = _slf.send_get_surface(
            id,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_color_representation_manager_v1.get_surface message: {}", Report::new(e));
        }
    }

    /// supported alpha modes
    ///
    /// When this object is created, it shall immediately send this event once
    /// for each alpha mode the compositor supports.
    ///
    /// For the definition of the supported values, see the
    /// wp_color_representation_surface_v1::alpha_mode enum.
    ///
    /// # Arguments
    ///
    /// - `alpha_mode`: supported alpha mode
    #[inline]
    fn supported_alpha_mode(
        &mut self,
        _slf: &Rc<MetaWpColorRepresentationManagerV1>,
        alpha_mode: MetaWpColorRepresentationSurfaceV1AlphaMode,
    ) {
        let res = _slf.send_supported_alpha_mode(
            alpha_mode,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_color_representation_manager_v1.supported_alpha_mode message: {}", Report::new(e));
        }
    }

    /// supported matrix coefficients and ranges
    ///
    /// When this object is created, it shall immediately send this event once
    /// for each matrix coefficient and color range combination the compositor
    /// supports.
    ///
    /// For the definition of the supported values, see the
    /// wp_color_representation_surface_v1::coefficients and
    /// wp_color_representation_surface_v1::range enums.
    ///
    /// # Arguments
    ///
    /// - `coefficients`: supported matrix coefficients
    /// - `range`: full range flag
    #[inline]
    fn supported_coefficients_and_ranges(
        &mut self,
        _slf: &Rc<MetaWpColorRepresentationManagerV1>,
        coefficients: MetaWpColorRepresentationSurfaceV1Coefficients,
        range: MetaWpColorRepresentationSurfaceV1Range,
    ) {
        let res = _slf.send_supported_coefficients_and_ranges(
            coefficients,
            range,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_color_representation_manager_v1.supported_coefficients_and_ranges message: {}", Report::new(e));
        }
    }

    /// all features have been sent
    ///
    /// This event is sent when all supported features have been sent.
    #[inline]
    fn done(
        &mut self,
        _slf: &Rc<MetaWpColorRepresentationManagerV1>,
    ) {
        let res = _slf.send_done(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_color_representation_manager_v1.done message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpColorRepresentationManagerV1 {
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
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg0_id = arg0;
                let arg0 = MetaWpColorRepresentationSurfaceV1::new(&self.core.state, self.core.version);
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
                    (**handler).get_surface(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_surface(&self, arg0, arg1);
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
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = MetaWpColorRepresentationSurfaceV1AlphaMode(arg0);
                if let Some(handler) = handler {
                    (**handler).supported_alpha_mode(&self, arg0);
                } else {
                    DefaultMessageHandler.supported_alpha_mode(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg0 = MetaWpColorRepresentationSurfaceV1Coefficients(arg0);
                let arg1 = MetaWpColorRepresentationSurfaceV1Range(arg1);
                if let Some(handler) = handler {
                    (**handler).supported_coefficients_and_ranges(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.supported_coefficients_and_ranges(&self, arg0, arg1);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).done(&self);
                } else {
                    DefaultMessageHandler.done(&self);
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
        let name = match id {
            0 => "destroy",
            1 => "get_surface",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "supported_alpha_mode",
            1 => "supported_coefficients_and_ranges",
            2 => "done",
            _ => return None,
        };
        Some(name)
    }
}

impl MetaWpColorRepresentationManagerV1 {
    /// Since when the error.surface_exists enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_SURFACE_EXISTS__SINCE: u32 = 1;
}

/// protocol errors
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWpColorRepresentationManagerV1Error(pub u32);

impl MetaWpColorRepresentationManagerV1Error {
    /// color representation surface exists already
    #[allow(dead_code)]
    pub const SURFACE_EXISTS: Self = Self(1);
}

impl Debug for MetaWpColorRepresentationManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::SURFACE_EXISTS => "SURFACE_EXISTS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
