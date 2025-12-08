//! create desktop-style surfaces
//!
//! The xdg_wm_base interface is exposed as a global object enabling clients
//! to turn their wl_surfaces into windows in a desktop environment. It
//! defines the basic functionality needed for clients and the compositor to
//! create windows that can be dragged, resized, maximized, etc, as well as
//! creating transient windows such as popup menus.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A xdg_wm_base proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaXdgWmBase {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaXdgWmBaseMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaXdgWmBaseMessageHandler for DefaultMessageHandler { }

impl MetaXdgWmBase {
    pub const XML_VERSION: u32 = 7;
}

impl MetaXdgWmBase {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::XdgWmBase, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaXdgWmBase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaXdgWmBase")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaXdgWmBase {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy xdg_wm_base
    ///
    /// Destroy this xdg_wm_base object.
    ///
    /// Destroying a bound xdg_wm_base object while there are surfaces
    /// still alive created by this xdg_wm_base object instance is illegal
    /// and will result in a defunct_surfaces error.
    #[inline]
    pub fn send_destroy(
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
            0,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the create_positioner message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE_POSITIONER__SINCE: u32 = 1;

    /// create a positioner object
    ///
    /// Create a positioner object. A positioner object is used to position
    /// surfaces relative to some parent surface. See the interface description
    /// and xdg_surface.get_popup for details.
    #[inline]
    pub fn send_create_positioner(
        &self,
        id: &Rc<MetaXdgPositioner>,
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

    /// Since when the get_xdg_surface message is available.
    #[allow(dead_code)]
    pub const MSG__GET_XDG_SURFACE__SINCE: u32 = 1;

    /// create a shell surface from a surface
    ///
    /// This creates an xdg_surface for the given surface. While xdg_surface
    /// itself is not a role, the corresponding surface may only be assigned
    /// a role extending xdg_surface, such as xdg_toplevel or xdg_popup. It is
    /// illegal to create an xdg_surface for a wl_surface which already has an
    /// assigned role and this will result in a role error.
    ///
    /// This creates an xdg_surface for the given surface. An xdg_surface is
    /// used as basis to define a role to a given surface, such as xdg_toplevel
    /// or xdg_popup. It also manages functionality shared between xdg_surface
    /// based surface roles.
    ///
    /// See the documentation of xdg_surface for more details about what an
    /// xdg_surface is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn send_get_xdg_surface(
        &self,
        id: &Rc<MetaXdgSurface>,
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
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
        ]);
        Ok(())
    }

    /// Since when the pong message is available.
    #[allow(dead_code)]
    pub const MSG__PONG__SINCE: u32 = 1;

    /// respond to a ping event
    ///
    /// A client must respond to a ping event with a pong request or
    /// the client may be deemed unresponsive. See xdg_wm_base.ping
    /// and xdg_wm_base.error.unresponsive.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the ping event
    #[inline]
    pub fn send_pong(
        &self,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            serial,
        );
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
            3,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the ping message is available.
    #[allow(dead_code)]
    pub const MSG__PING__SINCE: u32 = 1;

    /// check if the client is alive
    ///
    /// The ping event asks the client if it's still alive. Pass the
    /// serial specified in the event back to the compositor by sending
    /// a "pong" request back with the specified serial. See xdg_wm_base.pong.
    ///
    /// Compositors can use this to determine if the client is still
    /// alive. It's unspecified what will happen if the client doesn't
    /// respond to the ping request, or in what timeframe. Clients should
    /// try to respond in a reasonable amount of time. The “unresponsive”
    /// error is provided for compositors that wish to disconnect unresponsive
    /// clients.
    ///
    /// A compositor is free to ping in any way it wants, but a client must
    /// always respond to any xdg_wm_base object it created.
    ///
    /// # Arguments
    ///
    /// - `serial`: pass this to the pong request
    #[inline]
    pub fn send_ping(
        &self,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            serial,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
            arg0,
        ]);
        Ok(())
    }
}

/// A message handler for [XdgWmBase] proxies.
#[allow(dead_code)]
pub trait MetaXdgWmBaseMessageHandler {
    /// destroy xdg_wm_base
    ///
    /// Destroy this xdg_wm_base object.
    ///
    /// Destroying a bound xdg_wm_base object while there are surfaces
    /// still alive created by this xdg_wm_base object instance is illegal
    /// and will result in a defunct_surfaces error.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaXdgWmBase>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_wm_base.destroy message: {}", Report::new(e));
        }
    }

    /// create a positioner object
    ///
    /// Create a positioner object. A positioner object is used to position
    /// surfaces relative to some parent surface. See the interface description
    /// and xdg_surface.get_popup for details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn create_positioner(
        &mut self,
        _slf: &Rc<MetaXdgWmBase>,
        id: &Rc<MetaXdgPositioner>,
    ) {
        let res = _slf.send_create_positioner(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_wm_base.create_positioner message: {}", Report::new(e));
        }
    }

    /// create a shell surface from a surface
    ///
    /// This creates an xdg_surface for the given surface. While xdg_surface
    /// itself is not a role, the corresponding surface may only be assigned
    /// a role extending xdg_surface, such as xdg_toplevel or xdg_popup. It is
    /// illegal to create an xdg_surface for a wl_surface which already has an
    /// assigned role and this will result in a role error.
    ///
    /// This creates an xdg_surface for the given surface. An xdg_surface is
    /// used as basis to define a role to a given surface, such as xdg_toplevel
    /// or xdg_popup. It also manages functionality shared between xdg_surface
    /// based surface roles.
    ///
    /// See the documentation of xdg_surface for more details about what an
    /// xdg_surface is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_xdg_surface(
        &mut self,
        _slf: &Rc<MetaXdgWmBase>,
        id: &Rc<MetaXdgSurface>,
        surface: &Rc<MetaWlSurface>,
    ) {
        let res = _slf.send_get_xdg_surface(
            id,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_wm_base.get_xdg_surface message: {}", Report::new(e));
        }
    }

    /// respond to a ping event
    ///
    /// A client must respond to a ping event with a pong request or
    /// the client may be deemed unresponsive. See xdg_wm_base.ping
    /// and xdg_wm_base.error.unresponsive.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the ping event
    #[inline]
    fn pong(
        &mut self,
        _slf: &Rc<MetaXdgWmBase>,
        serial: u32,
    ) {
        let res = _slf.send_pong(
            serial,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_wm_base.pong message: {}", Report::new(e));
        }
    }

    /// check if the client is alive
    ///
    /// The ping event asks the client if it's still alive. Pass the
    /// serial specified in the event back to the compositor by sending
    /// a "pong" request back with the specified serial. See xdg_wm_base.pong.
    ///
    /// Compositors can use this to determine if the client is still
    /// alive. It's unspecified what will happen if the client doesn't
    /// respond to the ping request, or in what timeframe. Clients should
    /// try to respond in a reasonable amount of time. The “unresponsive”
    /// error is provided for compositors that wish to disconnect unresponsive
    /// clients.
    ///
    /// A compositor is free to ping in any way it wants, but a client must
    /// always respond to any xdg_wm_base object it created.
    ///
    /// # Arguments
    ///
    /// - `serial`: pass this to the pong request
    #[inline]
    fn ping(
        &mut self,
        _slf: &Rc<MetaXdgWmBase>,
        serial: u32,
    ) {
        let res = _slf.send_ping(
            serial,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_wm_base.ping message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaXdgWmBase {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
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
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaXdgPositioner::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).create_positioner(&self, arg0);
                } else {
                    DefaultMessageHandler.create_positioner(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaXdgSurface::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.endpoint.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_xdg_surface(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_xdg_surface(&self, arg0, arg1);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).pong(&self, arg0);
                } else {
                    DefaultMessageHandler.pong(&self, arg0);
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
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).ping(&self, arg0);
                } else {
                    DefaultMessageHandler.ping(&self, arg0);
                }
            }
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }
}

impl MetaXdgWmBase {
    /// Since when the error.role enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ROLE__SINCE: u32 = 1;
    /// Since when the error.defunct_surfaces enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_DEFUNCT_SURFACES__SINCE: u32 = 1;
    /// Since when the error.not_the_topmost_popup enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_NOT_THE_TOPMOST_POPUP__SINCE: u32 = 1;
    /// Since when the error.invalid_popup_parent enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_POPUP_PARENT__SINCE: u32 = 1;
    /// Since when the error.invalid_surface_state enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_SURFACE_STATE__SINCE: u32 = 1;
    /// Since when the error.invalid_positioner enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_POSITIONER__SINCE: u32 = 1;
    /// Since when the error.unresponsive enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_UNRESPONSIVE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaXdgWmBaseError(pub u32);

impl MetaXdgWmBaseError {
    /// given wl_surface has another role
    #[allow(dead_code)]
    pub const ROLE: Self = Self(0);

    /// xdg_wm_base was destroyed before children
    #[allow(dead_code)]
    pub const DEFUNCT_SURFACES: Self = Self(1);

    /// the client tried to map or destroy a non-topmost popup
    #[allow(dead_code)]
    pub const NOT_THE_TOPMOST_POPUP: Self = Self(2);

    /// the client specified an invalid popup parent surface
    #[allow(dead_code)]
    pub const INVALID_POPUP_PARENT: Self = Self(3);

    /// the client provided an invalid surface state
    #[allow(dead_code)]
    pub const INVALID_SURFACE_STATE: Self = Self(4);

    /// the client provided an invalid positioner
    #[allow(dead_code)]
    pub const INVALID_POSITIONER: Self = Self(5);

    /// the client didn’t respond to a ping event in time
    #[allow(dead_code)]
    pub const UNRESPONSIVE: Self = Self(6);
}

impl Debug for MetaXdgWmBaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ROLE => "ROLE",
            Self::DEFUNCT_SURFACES => "DEFUNCT_SURFACES",
            Self::NOT_THE_TOPMOST_POPUP => "NOT_THE_TOPMOST_POPUP",
            Self::INVALID_POPUP_PARENT => "INVALID_POPUP_PARENT",
            Self::INVALID_SURFACE_STATE => "INVALID_SURFACE_STATE",
            Self::INVALID_POSITIONER => "INVALID_POSITIONER",
            Self::UNRESPONSIVE => "UNRESPONSIVE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
