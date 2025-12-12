//! color representation manager singleton
//!
//! A singleton global interface used for getting color representation
//! extensions for wl_surface. The extension interfaces allow setting the
//! color representation of surfaces.
//!
//! Compositors should never remove this global.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_color_representation_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpColorRepresentationManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpColorRepresentationManagerV1Handler>,
}

struct DefaultHandler;

impl WpColorRepresentationManagerV1Handler for DefaultHandler { }

impl WpColorRepresentationManagerV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::WpColorRepresentationManagerV1;
    pub const INTERFACE_NAME: &str = "wp_color_representation_manager_v1";
}

impl WpColorRepresentationManagerV1 {
    pub fn set_handler(&self, handler: impl WpColorRepresentationManagerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn WpColorRepresentationManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpColorRepresentationManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpColorRepresentationManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpColorRepresentationManagerV1 {
    /// Since when the destroy message is available.
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_representation_manager_v1#{}.destroy()\n", id);
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
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
        id: &Rc<WpColorRepresentationSurfaceV1>,
        surface: &Rc<WlSurface>,
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_representation_manager_v1#{}.get_surface(id: wp_color_representation_surface_v1#{}, surface: wl_surface#{})\n", id, arg0_id, arg1_id);
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
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
        alpha_mode: WpColorRepresentationSurfaceV1AlphaMode,
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_color_representation_manager_v1#{}.supported_alpha_mode(alpha_mode: {:?})\n", client.endpoint.id, id, arg0);
            self.core.state.log(args);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
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
        coefficients: WpColorRepresentationSurfaceV1Coefficients,
        range: WpColorRepresentationSurfaceV1Range,
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_color_representation_manager_v1#{}.supported_coefficients_and_ranges(coefficients: {:?}, range: {:?})\n", client.endpoint.id, id, arg0, arg1);
            self.core.state.log(args);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_color_representation_manager_v1#{}.done()\n", client.endpoint.id, id);
            self.core.state.log(args);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
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
pub trait WpColorRepresentationManagerV1Handler: Any {
    /// destroy the manager
    ///
    /// Destroy the wp_color_representation_manager_v1 object. This does not
    /// affect any other objects in any way.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<WpColorRepresentationManagerV1>,
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
        _slf: &Rc<WpColorRepresentationManagerV1>,
        id: &Rc<WpColorRepresentationSurfaceV1>,
        surface: &Rc<WlSurface>,
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
        _slf: &Rc<WpColorRepresentationManagerV1>,
        alpha_mode: WpColorRepresentationSurfaceV1AlphaMode,
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
        _slf: &Rc<WpColorRepresentationManagerV1>,
        coefficients: WpColorRepresentationSurfaceV1Coefficients,
        range: WpColorRepresentationSurfaceV1Range,
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
        _slf: &Rc<WpColorRepresentationManagerV1>,
    ) {
        let res = _slf.send_done(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_color_representation_manager_v1.done message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for WpColorRepresentationManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpColorRepresentationManagerV1, version),
            handler: Default::default(),
        })
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_representation_manager_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_representation_manager_v1#{}.get_surface(id: wp_color_representation_surface_v1#{}, surface: wl_surface#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = WpColorRepresentationSurfaceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_surface(&self, arg0, arg1);
                } else {
                    DefaultHandler.get_surface(&self, arg0, arg1);
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
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = WpColorRepresentationSurfaceV1AlphaMode(arg0);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_color_representation_manager_v1#{}.supported_alpha_mode(alpha_mode: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).supported_alpha_mode(&self, arg0);
                } else {
                    DefaultHandler.supported_alpha_mode(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg0 = WpColorRepresentationSurfaceV1Coefficients(arg0);
                let arg1 = WpColorRepresentationSurfaceV1Range(arg1);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_color_representation_manager_v1#{}.supported_coefficients_and_ranges(coefficients: {:?}, range: {:?})\n", msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).supported_coefficients_and_ranges(&self, arg0, arg1);
                } else {
                    DefaultHandler.supported_coefficients_and_ranges(&self, arg0, arg1);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_color_representation_manager_v1#{}.done()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).done(&self);
                } else {
                    DefaultHandler.done(&self);
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

impl Object for WpColorRepresentationManagerV1 {
    fn core(&self) -> &ObjectCore {
        &self.core
    }

    fn unset_handler(&self) {
        self.handler.set(None);
    }

    fn get_handler_any_ref(&self) -> Result<Ref<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(Ref::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<RefMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow_mut().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(RefMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

impl WpColorRepresentationManagerV1 {
    /// Since when the error.surface_exists enum variant is available.
    pub const ENM__ERROR_SURFACE_EXISTS__SINCE: u32 = 1;
}

/// protocol errors
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpColorRepresentationManagerV1Error(pub u32);

impl WpColorRepresentationManagerV1Error {
    /// color representation surface exists already
    pub const SURFACE_EXISTS: Self = Self(1);
}

impl Debug for WpColorRepresentationManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::SURFACE_EXISTS => "SURFACE_EXISTS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
