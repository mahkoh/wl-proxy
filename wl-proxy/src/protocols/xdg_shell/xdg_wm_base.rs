//! create desktop-style surfaces
//!
//! The xdg_wm_base interface is exposed as a global object enabling clients
//! to turn their wl_surfaces into windows in a desktop environment. It
//! defines the basic functionality needed for clients and the compositor to
//! create windows that can be dragged, resized, maximized, etc, as well as
//! creating transient windows such as popup menus.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A xdg_wm_base proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XdgWmBase {
    core: ProxyCore,
    handler: HandlerHolder<dyn XdgWmBaseHandler>,
}

struct DefaultHandler;

impl XdgWmBaseHandler for DefaultHandler { }

impl XdgWmBase {
    pub const XML_VERSION: u32 = 7;
    pub const INTERFACE: &str = "xdg_wm_base";
}

impl XdgWmBase {
    pub fn set_handler(&self, handler: impl XdgWmBaseHandler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn XdgWmBaseHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XdgWmBase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XdgWmBase")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XdgWmBase {
    /// Since when the destroy message is available.
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_wm_base#{}.destroy()\n", id);
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

    /// Since when the create_positioner message is available.
    pub const MSG__CREATE_POSITIONER__SINCE: u32 = 1;

    /// create a positioner object
    ///
    /// Create a positioner object. A positioner object is used to position
    /// surfaces relative to some parent surface. See the interface description
    /// and xdg_surface.get_popup for details.
    #[inline]
    pub fn send_create_positioner(
        &self,
        id: &Rc<XdgPositioner>,
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_wm_base#{}.create_positioner(id: xdg_positioner#{})\n", id, arg0_id);
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
        ]);
        Ok(())
    }

    /// Since when the get_xdg_surface message is available.
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
        id: &Rc<XdgSurface>,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_wm_base#{}.get_xdg_surface(id: xdg_surface#{}, surface: wl_surface#{})\n", id, arg0_id, arg1_id);
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
            2,
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// Since when the pong message is available.
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_wm_base#{}.pong(serial: {})\n", id, arg0);
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
            3,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the ping message is available.
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xdg_wm_base#{}.ping(serial: {})\n", client.endpoint.id, id, arg0);
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
            arg0,
        ]);
        Ok(())
    }
}

/// A message handler for [XdgWmBase] proxies.
pub trait XdgWmBaseHandler: Any {
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
        _slf: &Rc<XdgWmBase>,
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
        _slf: &Rc<XdgWmBase>,
        id: &Rc<XdgPositioner>,
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
        _slf: &Rc<XdgWmBase>,
        id: &Rc<XdgSurface>,
        surface: &Rc<WlSurface>,
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
        _slf: &Rc<XdgWmBase>,
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
        _slf: &Rc<XdgWmBase>,
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

impl ProxyPrivate for XdgWmBase {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::XdgWmBase, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_wm_base#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_wm_base#{}.create_positioner(id: xdg_positioner#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = XdgPositioner::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).create_positioner(&self, arg0);
                } else {
                    DefaultHandler.create_positioner(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_wm_base#{}.get_xdg_surface(id: xdg_surface#{}, surface: wl_surface#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = XdgSurface::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ProxyInterface::WlSurface));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_xdg_surface(&self, arg0, arg1);
                } else {
                    DefaultHandler.get_xdg_surface(&self, arg0, arg1);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_wm_base#{}.pong(serial: {})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).pong(&self, arg0);
                } else {
                    DefaultHandler.pong(&self, arg0);
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
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xdg_wm_base#{}.ping(serial: {})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).ping(&self, arg0);
                } else {
                    DefaultHandler.ping(&self, arg0);
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
            1 => "create_positioner",
            2 => "get_xdg_surface",
            3 => "pong",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "ping",
            _ => return None,
        };
        Some(name)
    }
}

impl Proxy for XdgWmBase {
    fn core(&self) -> &ProxyCore {
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

impl XdgWmBase {
    /// Since when the error.role enum variant is available.
    pub const ENM__ERROR_ROLE__SINCE: u32 = 1;
    /// Since when the error.defunct_surfaces enum variant is available.
    pub const ENM__ERROR_DEFUNCT_SURFACES__SINCE: u32 = 1;
    /// Since when the error.not_the_topmost_popup enum variant is available.
    pub const ENM__ERROR_NOT_THE_TOPMOST_POPUP__SINCE: u32 = 1;
    /// Since when the error.invalid_popup_parent enum variant is available.
    pub const ENM__ERROR_INVALID_POPUP_PARENT__SINCE: u32 = 1;
    /// Since when the error.invalid_surface_state enum variant is available.
    pub const ENM__ERROR_INVALID_SURFACE_STATE__SINCE: u32 = 1;
    /// Since when the error.invalid_positioner enum variant is available.
    pub const ENM__ERROR_INVALID_POSITIONER__SINCE: u32 = 1;
    /// Since when the error.unresponsive enum variant is available.
    pub const ENM__ERROR_UNRESPONSIVE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct XdgWmBaseError(pub u32);

impl XdgWmBaseError {
    /// given wl_surface has another role
    pub const ROLE: Self = Self(0);

    /// xdg_wm_base was destroyed before children
    pub const DEFUNCT_SURFACES: Self = Self(1);

    /// the client tried to map or destroy a non-topmost popup
    pub const NOT_THE_TOPMOST_POPUP: Self = Self(2);

    /// the client specified an invalid popup parent surface
    pub const INVALID_POPUP_PARENT: Self = Self(3);

    /// the client provided an invalid surface state
    pub const INVALID_SURFACE_STATE: Self = Self(4);

    /// the client provided an invalid positioner
    pub const INVALID_POSITIONER: Self = Self(5);

    /// the client didn’t respond to a ping event in time
    pub const UNRESPONSIVE: Self = Self(6);
}

impl Debug for XdgWmBaseError {
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
