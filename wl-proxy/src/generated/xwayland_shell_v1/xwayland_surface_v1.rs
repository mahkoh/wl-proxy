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

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A xwayland_surface_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaXwaylandSurfaceV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaXwaylandSurfaceV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaXwaylandSurfaceV1MessageHandler for DefaultMessageHandler { }

impl MetaXwaylandSurfaceV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaXwaylandSurfaceV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::XwaylandSurfaceV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaXwaylandSurfaceV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaXwaylandSurfaceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaXwaylandSurfaceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaXwaylandSurfaceV1 {
    /// Since when the set_serial message is available.
    #[allow(dead_code)]
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
    pub fn send_set_serial(
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
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the Xwayland surface object
    ///
    /// Destroy the xwayland_surface_v1 object.
    ///
    /// Any already existing associations are unaffected by this action.
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
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [XwaylandSurfaceV1] proxies.
#[allow(dead_code)]
pub trait MetaXwaylandSurfaceV1MessageHandler {
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
    fn set_serial(
        &mut self,
        _slf: &Rc<MetaXwaylandSurfaceV1>,
        serial_lo: u32,
        serial_hi: u32,
    ) {
        let res = _slf.send_set_serial(
            serial_lo,
            serial_hi,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xwayland_surface_v1.set_serial message: {}", Report::new(e));
        }
    }

    /// destroy the Xwayland surface object
    ///
    /// Destroy the xwayland_surface_v1 object.
    ///
    /// Any already existing associations are unaffected by this action.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaXwaylandSurfaceV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xwayland_surface_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaXwaylandSurfaceV1 {
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
                if let Some(handler) = handler {
                    (**handler).set_serial(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.set_serial(&self, arg0, arg1);
                }
            }
            1 => {
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

impl MetaXwaylandSurfaceV1 {
    /// Since when the error.already_associated enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ALREADY_ASSOCIATED__SINCE: u32 = 1;
    /// Since when the error.invalid_serial enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_SERIAL__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaXwaylandSurfaceV1Error(pub u32);

impl MetaXwaylandSurfaceV1Error {
    /// given wl_surface is already associated with an X11 window
    #[allow(dead_code)]
    pub const ALREADY_ASSOCIATED: Self = Self(0);

    /// serial was not valid
    #[allow(dead_code)]
    pub const INVALID_SERIAL: Self = Self(1);
}

impl Debug for MetaXwaylandSurfaceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ALREADY_ASSOCIATED => "ALREADY_ASSOCIATED",
            Self::INVALID_SERIAL => "INVALID_SERIAL",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
