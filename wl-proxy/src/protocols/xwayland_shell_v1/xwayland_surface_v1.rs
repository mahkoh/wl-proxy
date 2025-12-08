//! interface for associating Xwayland windows to wl_surfaces
//!
//! An Xwayland surface is a surface managed by an Xwayland server.
//! It is used for associating surfaces to Xwayland windows.
//!
//! The Xwayland server associated with actions in this interface is
//! determined by the Wayland client making the request.
//!
//! The client must call wl_surface.commit on the corresponding wl_surface
//! for the xwayland_surface_v1 state to take effect.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A xwayland_surface_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XwaylandSurfaceV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn XwaylandSurfaceV1Handler>,
}

struct DefaultHandler;

impl XwaylandSurfaceV1Handler for DefaultHandler { }

impl ConcreteObject for XwaylandSurfaceV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::XwaylandSurfaceV1;
    const INTERFACE_NAME: &str = "xwayland_surface_v1";
}

impl XwaylandSurfaceV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl XwaylandSurfaceV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn XwaylandSurfaceV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XwaylandSurfaceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XwaylandSurfaceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XwaylandSurfaceV1 {
    /// Since when the set_serial message is available.
    pub const MSG__SET_SERIAL__SINCE: u32 = 1;

    /// associates a Xwayland window to a wl_surface
    ///
    /// Associates an Xwayland window to a wl_surface.
    /// The association state is double-buffered, see wl_surface.commit.
    ///
    /// The `serial_lo` and `serial_hi` parameters specify a non-zero
    /// monotonic serial number which is entirely unique and provided by the
    /// Xwayland server equal to the serial value provided by a client message
    /// with a message type of the `WL_SURFACE_SERIAL` atom on the X11 window
    /// for this surface to be associated to.
    ///
    /// The serial value in the `WL_SURFACE_SERIAL` client message is specified
    /// as having the lo-bits specified in `l[0]` and the hi-bits specified
    /// in `l[1]`.
    ///
    /// If the serial value provided by `serial_lo` and `serial_hi` is not
    /// valid, the `invalid_serial` protocol error will be raised.
    ///
    /// An X11 window may be associated with multiple surfaces throughout its
    /// lifespan. (eg. unmapping and remapping a window).
    ///
    /// For each wl_surface, this state must not be committed more than once,
    /// otherwise the `already_associated` protocol error will be raised.
    ///
    /// # Arguments
    ///
    /// - `serial_lo`: The lower 32-bits of the serial number associated with the X11 window
    /// - `serial_hi`: The upper 32-bits of the serial number associated with the X11 window
    #[inline]
    pub fn try_send_set_serial(
        &self,
        serial_lo: u32,
        serial_hi: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            serial_lo,
            serial_hi,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xwayland_surface_v1#{}.set_serial(serial_lo: {}, serial_hi: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1);
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
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// associates a Xwayland window to a wl_surface
    ///
    /// Associates an Xwayland window to a wl_surface.
    /// The association state is double-buffered, see wl_surface.commit.
    ///
    /// The `serial_lo` and `serial_hi` parameters specify a non-zero
    /// monotonic serial number which is entirely unique and provided by the
    /// Xwayland server equal to the serial value provided by a client message
    /// with a message type of the `WL_SURFACE_SERIAL` atom on the X11 window
    /// for this surface to be associated to.
    ///
    /// The serial value in the `WL_SURFACE_SERIAL` client message is specified
    /// as having the lo-bits specified in `l[0]` and the hi-bits specified
    /// in `l[1]`.
    ///
    /// If the serial value provided by `serial_lo` and `serial_hi` is not
    /// valid, the `invalid_serial` protocol error will be raised.
    ///
    /// An X11 window may be associated with multiple surfaces throughout its
    /// lifespan. (eg. unmapping and remapping a window).
    ///
    /// For each wl_surface, this state must not be committed more than once,
    /// otherwise the `already_associated` protocol error will be raised.
    ///
    /// # Arguments
    ///
    /// - `serial_lo`: The lower 32-bits of the serial number associated with the X11 window
    /// - `serial_hi`: The upper 32-bits of the serial number associated with the X11 window
    #[inline]
    pub fn send_set_serial(
        &self,
        serial_lo: u32,
        serial_hi: u32,
    ) {
        let res = self.try_send_set_serial(
            serial_lo,
            serial_hi,
        );
        if let Err(e) = res {
            log_send("xwayland_surface_v1.set_serial", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the Xwayland surface object
    ///
    /// Destroy the xwayland_surface_v1 object.
    ///
    /// Any already existing associations are unaffected by this action.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xwayland_surface_v1#{}.destroy()\n", id);
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

    /// destroy the Xwayland surface object
    ///
    /// Destroy the xwayland_surface_v1 object.
    ///
    /// Any already existing associations are unaffected by this action.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("xwayland_surface_v1.destroy", &e);
        }
    }
}

/// A message handler for [`XwaylandSurfaceV1`] proxies.
pub trait XwaylandSurfaceV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<XwaylandSurfaceV1>) {
        slf.core.delete_id();
    }

    /// associates a Xwayland window to a wl_surface
    ///
    /// Associates an Xwayland window to a wl_surface.
    /// The association state is double-buffered, see wl_surface.commit.
    ///
    /// The `serial_lo` and `serial_hi` parameters specify a non-zero
    /// monotonic serial number which is entirely unique and provided by the
    /// Xwayland server equal to the serial value provided by a client message
    /// with a message type of the `WL_SURFACE_SERIAL` atom on the X11 window
    /// for this surface to be associated to.
    ///
    /// The serial value in the `WL_SURFACE_SERIAL` client message is specified
    /// as having the lo-bits specified in `l[0]` and the hi-bits specified
    /// in `l[1]`.
    ///
    /// If the serial value provided by `serial_lo` and `serial_hi` is not
    /// valid, the `invalid_serial` protocol error will be raised.
    ///
    /// An X11 window may be associated with multiple surfaces throughout its
    /// lifespan. (eg. unmapping and remapping a window).
    ///
    /// For each wl_surface, this state must not be committed more than once,
    /// otherwise the `already_associated` protocol error will be raised.
    ///
    /// # Arguments
    ///
    /// - `serial_lo`: The lower 32-bits of the serial number associated with the X11 window
    /// - `serial_hi`: The upper 32-bits of the serial number associated with the X11 window
    #[inline]
    fn handle_set_serial(
        &mut self,
        slf: &Rc<XwaylandSurfaceV1>,
        serial_lo: u32,
        serial_hi: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_serial(
            serial_lo,
            serial_hi,
        );
        if let Err(e) = res {
            log_forward("xwayland_surface_v1.set_serial", &e);
        }
    }

    /// destroy the Xwayland surface object
    ///
    /// Destroy the xwayland_surface_v1 object.
    ///
    /// Any already existing associations are unaffected by this action.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<XwaylandSurfaceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("xwayland_surface_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for XwaylandSurfaceV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::XwaylandSurfaceV1, version),
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
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xwayland_surface_v1#{}.set_serial(serial_lo: {}, serial_hi: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_serial(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_serial(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xwayland_surface_v1#{}.destroy()\n", client_id, id);
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
            0 => "set_serial",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for XwaylandSurfaceV1 {
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

impl XwaylandSurfaceV1 {
    /// Since when the error.already_associated enum variant is available.
    pub const ENM__ERROR_ALREADY_ASSOCIATED__SINCE: u32 = 1;
    /// Since when the error.invalid_serial enum variant is available.
    pub const ENM__ERROR_INVALID_SERIAL__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct XwaylandSurfaceV1Error(pub u32);

impl XwaylandSurfaceV1Error {
    /// given wl_surface is already associated with an X11 window
    pub const ALREADY_ASSOCIATED: Self = Self(0);

    /// serial was not valid
    pub const INVALID_SERIAL: Self = Self(1);
}

impl Debug for XwaylandSurfaceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ALREADY_ASSOCIATED => "ALREADY_ASSOCIATED",
            Self::INVALID_SERIAL => "INVALID_SERIAL",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
