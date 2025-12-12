//! Server side window decoration manager
//!
//! This interface allows to coordinate whether the server should create
//! a server-side window decoration around a wl_surface representing a
//! shell surface (wl_shell_surface or similar). By announcing support
//! for this interface the server indicates that it supports server
//! side decorations.
//!
//! Use in conjunction with zxdg_decoration_manager_v1 is undefined.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A org_kde_kwin_server_decoration_manager object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct OrgKdeKwinServerDecorationManager {
    core: ObjectCore,
    handler: HandlerHolder<dyn OrgKdeKwinServerDecorationManagerHandler>,
}

struct DefaultHandler;

impl OrgKdeKwinServerDecorationManagerHandler for DefaultHandler { }

impl OrgKdeKwinServerDecorationManager {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::OrgKdeKwinServerDecorationManager;
    pub const INTERFACE_NAME: &str = "org_kde_kwin_server_decoration_manager";
}

impl OrgKdeKwinServerDecorationManager {
    pub fn set_handler(&self, handler: impl OrgKdeKwinServerDecorationManagerHandler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn OrgKdeKwinServerDecorationManagerHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for OrgKdeKwinServerDecorationManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OrgKdeKwinServerDecorationManager")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl OrgKdeKwinServerDecorationManager {
    /// Since when the create message is available.
    pub const MSG__CREATE__SINCE: u32 = 1;

    /// Create a server-side decoration object for a given surface
    ///
    /// When a client creates a server-side decoration object it indicates
    /// that it supports the protocol. The client is supposed to tell the
    /// server whether it wants server-side decorations or will provide
    /// client-side decorations.
    ///
    /// If the client does not create a server-side decoration object for
    /// a surface the server interprets this as lack of support for this
    /// protocol and considers it as client-side decorated. Nevertheless a
    /// client-side decorated surface should use this protocol to indicate
    /// to the server that it does not want a server-side deco.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn send_create(
        &self,
        id: &Rc<OrgKdeKwinServerDecoration>,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= org_kde_kwin_server_decoration_manager#{}.create(id: org_kde_kwin_server_decoration#{}, surface: wl_surface#{})\n", id, arg0_id, arg1_id);
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
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// Since when the default_mode message is available.
    pub const MSG__DEFAULT_MODE__SINCE: u32 = 1;

    /// The default mode used on the server
    ///
    /// This event is emitted directly after binding the interface. It contains
    /// the default mode for the decoration. When a new server decoration object
    /// is created this new object will be in the default mode until the first
    /// request_mode is requested.
    ///
    /// The server may change the default mode at any time.
    ///
    /// # Arguments
    ///
    /// - `mode`: The default decoration mode applied to newly created server decorations.
    #[inline]
    pub fn send_default_mode(
        &self,
        mode: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            mode,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= org_kde_kwin_server_decoration_manager#{}.default_mode(mode: {})\n", client.endpoint.id, id, arg0);
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

/// A message handler for [OrgKdeKwinServerDecorationManager] proxies.
pub trait OrgKdeKwinServerDecorationManagerHandler: Any {
    /// Create a server-side decoration object for a given surface
    ///
    /// When a client creates a server-side decoration object it indicates
    /// that it supports the protocol. The client is supposed to tell the
    /// server whether it wants server-side decorations or will provide
    /// client-side decorations.
    ///
    /// If the client does not create a server-side decoration object for
    /// a surface the server interprets this as lack of support for this
    /// protocol and considers it as client-side decorated. Nevertheless a
    /// client-side decorated surface should use this protocol to indicate
    /// to the server that it does not want a server-side deco.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn create(
        &mut self,
        _slf: &Rc<OrgKdeKwinServerDecorationManager>,
        id: &Rc<OrgKdeKwinServerDecoration>,
        surface: &Rc<WlSurface>,
    ) {
        let res = _slf.send_create(
            id,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a org_kde_kwin_server_decoration_manager.create message: {}", Report::new(e));
        }
    }

    /// The default mode used on the server
    ///
    /// This event is emitted directly after binding the interface. It contains
    /// the default mode for the decoration. When a new server decoration object
    /// is created this new object will be in the default mode until the first
    /// request_mode is requested.
    ///
    /// The server may change the default mode at any time.
    ///
    /// # Arguments
    ///
    /// - `mode`: The default decoration mode applied to newly created server decorations.
    #[inline]
    fn default_mode(
        &mut self,
        _slf: &Rc<OrgKdeKwinServerDecorationManager>,
        mode: u32,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        let res = _slf.send_default_mode(
            mode,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a org_kde_kwin_server_decoration_manager.default_mode message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for OrgKdeKwinServerDecorationManager {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::OrgKdeKwinServerDecorationManager, version),
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
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> org_kde_kwin_server_decoration_manager#{}.create(id: org_kde_kwin_server_decoration#{}, surface: wl_surface#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = OrgKdeKwinServerDecoration::new(&self.core.state, self.core.version);
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
                    (**handler).create(&self, arg0, arg1);
                } else {
                    DefaultHandler.create(&self, arg0, arg1);
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> org_kde_kwin_server_decoration_manager#{}.default_mode(mode: {})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).default_mode(&self, arg0);
                } else {
                    DefaultHandler.default_mode(&self, arg0);
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
            0 => "create",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "default_mode",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for OrgKdeKwinServerDecorationManager {
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

impl OrgKdeKwinServerDecorationManager {
    /// Since when the mode.None enum variant is available.
    pub const ENM__MODE_NONE__SINCE: u32 = 1;
    /// Since when the mode.Client enum variant is available.
    pub const ENM__MODE_CLIENT__SINCE: u32 = 1;
    /// Since when the mode.Server enum variant is available.
    pub const ENM__MODE_SERVER__SINCE: u32 = 1;
}

/// Possible values to use in request_mode and the event mode.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OrgKdeKwinServerDecorationManagerMode(pub u32);

impl OrgKdeKwinServerDecorationManagerMode {
    /// Undecorated: The surface is not decorated at all, neither server nor client-side. An example is a popup surface which should not be decorated.
    pub const NONE: Self = Self(0);

    /// Client-side decoration: The decoration is part of the surface and the client.
    pub const CLIENT: Self = Self(1);

    /// Server-side decoration: The server embeds the surface into a decoration frame.
    pub const SERVER: Self = Self(2);
}

impl Debug for OrgKdeKwinServerDecorationManagerMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NONE => "NONE",
            Self::CLIENT => "CLIENT",
            Self::SERVER => "SERVER",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
