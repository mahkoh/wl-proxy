//! hyprland-specific wl_surface properties
//!
//! This interface allows access to hyprland-specific properties of a wl_surface.
//!
//! Once the wl_surface has been destroyed, the hyprland surface object must be
//! destroyed as well. All other operations are a protocol error.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A hyprland_surface_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct HyprlandSurfaceV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn HyprlandSurfaceV1Handler>,
}

struct DefaultHandler;

impl HyprlandSurfaceV1Handler for DefaultHandler { }

impl ConcreteObject for HyprlandSurfaceV1 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::HyprlandSurfaceV1;
    const INTERFACE_NAME: &str = "hyprland_surface_v1";
}

impl HyprlandSurfaceV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl HyprlandSurfaceV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn HyprlandSurfaceV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for HyprlandSurfaceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HyprlandSurfaceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl HyprlandSurfaceV1 {
    /// Since when the set_opacity message is available.
    pub const MSG__SET_OPACITY__SINCE: u32 = 1;

    /// set the overall opacity of the surface
    ///
    /// Sets a multiplier for the overall opacity of the surface.
    /// This multiplier applies to visual effects such as blur behind the surface
    /// in addition to the surface's content.
    ///
    /// The default value is 1.0.
    /// Setting a value outside of the range 0.0 - 1.0 (inclusive) is a protocol error.
    /// Does not take effect until wl_surface.commit is called.
    ///
    /// # Arguments
    ///
    /// - `opacity`:
    #[inline]
    pub fn try_send_set_opacity(
        &self,
        opacity: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            opacity,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: Fixed) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_surface_v1#{}.set_opacity(opacity: {})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0.to_wire() as u32,
        ]);
        Ok(())
    }

    /// set the overall opacity of the surface
    ///
    /// Sets a multiplier for the overall opacity of the surface.
    /// This multiplier applies to visual effects such as blur behind the surface
    /// in addition to the surface's content.
    ///
    /// The default value is 1.0.
    /// Setting a value outside of the range 0.0 - 1.0 (inclusive) is a protocol error.
    /// Does not take effect until wl_surface.commit is called.
    ///
    /// # Arguments
    ///
    /// - `opacity`:
    #[inline]
    pub fn send_set_opacity(
        &self,
        opacity: Fixed,
    ) {
        let res = self.try_send_set_opacity(
            opacity,
        );
        if let Err(e) = res {
            log_send("hyprland_surface_v1.set_opacity", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the hyprland surface interface
    ///
    /// Destroy the hyprland surface object, resetting properties provided
    /// by this interface to their default values on the next wl_surface.commit.
    #[inline]
    pub fn try_send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_surface_v1#{}.destroy()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the hyprland surface interface
    ///
    /// Destroy the hyprland surface object, resetting properties provided
    /// by this interface to their default values on the next wl_surface.commit.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("hyprland_surface_v1.destroy", &e);
        }
    }

    /// Since when the set_visible_region message is available.
    pub const MSG__SET_VISIBLE_REGION__SINCE: u32 = 2;

    /// set the visible region of the surface
    ///
    /// This request sets the region of the surface that contains visible content.
    /// Visible content refers to content that has an alpha value greater than zero.
    ///
    /// The visible region is an optimization hint for the compositor that lets it
    /// avoid drawing parts of the surface that are not visible. Setting a visible region
    /// that does not contain all content in the surface may result in missing content
    /// not being drawn.
    ///
    /// The visible region is specified in buffer-local coordinates.
    ///
    /// The compositor ignores the parts of the visible region that fall outside of the surface.
    /// When all parts of the region fall outside of the buffer geometry, the compositor may
    /// avoid rendering the surface entirely.
    ///
    /// The initial value for the visible region is empty. Setting the
    /// visible region has copy semantics, and the wl_region object can be destroyed immediately.
    /// A NULL wl_region causes the visible region to be set to empty.
    ///
    /// Does not take effect until wl_surface.commit is called.
    ///
    /// # Arguments
    ///
    /// - `region`:
    #[inline]
    pub fn try_send_set_visible_region(
        &self,
        region: Option<&Rc<WlRegion>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            region,
        );
        let arg0 = arg0.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("region"))),
                Some(id) => id,
            },
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_surface_v1#{}.set_visible_region(region: wl_region#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
            arg0_id,
        ]);
        Ok(())
    }

    /// set the visible region of the surface
    ///
    /// This request sets the region of the surface that contains visible content.
    /// Visible content refers to content that has an alpha value greater than zero.
    ///
    /// The visible region is an optimization hint for the compositor that lets it
    /// avoid drawing parts of the surface that are not visible. Setting a visible region
    /// that does not contain all content in the surface may result in missing content
    /// not being drawn.
    ///
    /// The visible region is specified in buffer-local coordinates.
    ///
    /// The compositor ignores the parts of the visible region that fall outside of the surface.
    /// When all parts of the region fall outside of the buffer geometry, the compositor may
    /// avoid rendering the surface entirely.
    ///
    /// The initial value for the visible region is empty. Setting the
    /// visible region has copy semantics, and the wl_region object can be destroyed immediately.
    /// A NULL wl_region causes the visible region to be set to empty.
    ///
    /// Does not take effect until wl_surface.commit is called.
    ///
    /// # Arguments
    ///
    /// - `region`:
    #[inline]
    pub fn send_set_visible_region(
        &self,
        region: Option<&Rc<WlRegion>>,
    ) {
        let res = self.try_send_set_visible_region(
            region,
        );
        if let Err(e) = res {
            log_send("hyprland_surface_v1.set_visible_region", &e);
        }
    }
}

/// A message handler for [`HyprlandSurfaceV1`] proxies.
pub trait HyprlandSurfaceV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<HyprlandSurfaceV1>) {
        slf.core.delete_id();
    }

    /// set the overall opacity of the surface
    ///
    /// Sets a multiplier for the overall opacity of the surface.
    /// This multiplier applies to visual effects such as blur behind the surface
    /// in addition to the surface's content.
    ///
    /// The default value is 1.0.
    /// Setting a value outside of the range 0.0 - 1.0 (inclusive) is a protocol error.
    /// Does not take effect until wl_surface.commit is called.
    ///
    /// # Arguments
    ///
    /// - `opacity`:
    #[inline]
    fn handle_set_opacity(
        &mut self,
        slf: &Rc<HyprlandSurfaceV1>,
        opacity: Fixed,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_opacity(
            opacity,
        );
        if let Err(e) = res {
            log_forward("hyprland_surface_v1.set_opacity", &e);
        }
    }

    /// destroy the hyprland surface interface
    ///
    /// Destroy the hyprland surface object, resetting properties provided
    /// by this interface to their default values on the next wl_surface.commit.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<HyprlandSurfaceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("hyprland_surface_v1.destroy", &e);
        }
    }

    /// set the visible region of the surface
    ///
    /// This request sets the region of the surface that contains visible content.
    /// Visible content refers to content that has an alpha value greater than zero.
    ///
    /// The visible region is an optimization hint for the compositor that lets it
    /// avoid drawing parts of the surface that are not visible. Setting a visible region
    /// that does not contain all content in the surface may result in missing content
    /// not being drawn.
    ///
    /// The visible region is specified in buffer-local coordinates.
    ///
    /// The compositor ignores the parts of the visible region that fall outside of the surface.
    /// When all parts of the region fall outside of the buffer geometry, the compositor may
    /// avoid rendering the surface entirely.
    ///
    /// The initial value for the visible region is empty. Setting the
    /// visible region has copy semantics, and the wl_region object can be destroyed immediately.
    /// A NULL wl_region causes the visible region to be set to empty.
    ///
    /// Does not take effect until wl_surface.commit is called.
    ///
    /// # Arguments
    ///
    /// - `region`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_visible_region(
        &mut self,
        slf: &Rc<HyprlandSurfaceV1>,
        region: Option<&Rc<WlRegion>>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_visible_region(
            region,
        );
        if let Err(e) = res {
            log_forward("hyprland_surface_v1.set_visible_region", &e);
        }
    }
}

impl ObjectPrivate for HyprlandSurfaceV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::HyprlandSurfaceV1, version),
            handler: Default::default(),
        })
    }

    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err((ObjectError(ObjectErrorKind::HandlerBorrowed), self));
        };
        if let Some(handler) = &mut *handler {
            handler.delete_id(&self);
        } else {
            self.core.delete_id();
        }
        Ok(())
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = Fixed::from_wire(arg0 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: Fixed) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_surface_v1#{}.set_opacity(opacity: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_opacity(&self, arg0);
                } else {
                    DefaultHandler.handle_set_opacity(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_surface_v1#{}.destroy()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_destroy(&self);
                } else {
                    DefaultHandler.handle_destroy(&self);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_surface_v1#{}.set_visible_region(region: wl_region#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlRegion>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("region", o.core().interface, ObjectInterface::WlRegion)));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_set_visible_region(&self, arg0);
                } else {
                    DefaultHandler.handle_set_visible_region(&self, arg0);
                }
            }
            n => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, server: &Endpoint, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "set_opacity",
            1 => "destroy",
            2 => "set_visible_region",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for HyprlandSurfaceV1 {
    fn core(&self) -> &ObjectCore {
        &self.core
    }

    fn unset_handler(&self) {
        self.handler.set(None);
    }

    fn get_handler_any_ref(&self) -> Result<HandlerRef<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.try_borrow().ok_or(HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(HandlerRef::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<HandlerMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.try_borrow_mut().ok_or(HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(HandlerMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

impl HyprlandSurfaceV1 {
    /// Since when the error.no_surface enum variant is available.
    pub const ENM__ERROR_NO_SURFACE__SINCE: u32 = 1;
    /// Since when the error.out_of_range enum variant is available.
    pub const ENM__ERROR_OUT_OF_RANGE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HyprlandSurfaceV1Error(pub u32);

impl HyprlandSurfaceV1Error {
    /// wl_surface was destroyed
    pub const NO_SURFACE: Self = Self(0);

    /// given opacity was not in the range 0.0 - 1.0 (inclusive)
    pub const OUT_OF_RANGE: Self = Self(1);
}

impl Debug for HyprlandSurfaceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NO_SURFACE => "NO_SURFACE",
            Self::OUT_OF_RANGE => "OUT_OF_RANGE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
