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

impl ConcreteObject for OrgKdeKwinServerDecorationManager {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::OrgKdeKwinServerDecorationManager;
    const INTERFACE_NAME: &str = "org_kde_kwin_server_decoration_manager";
}

impl OrgKdeKwinServerDecorationManager {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl OrgKdeKwinServerDecorationManagerHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
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
    pub fn try_send_create(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= org_kde_kwin_server_decoration_manager#{}.create(id: org_kde_kwin_server_decoration#{}, surface: wl_surface#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id);
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
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

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
    ) {
        let res = self.try_send_create(
            id,
            surface,
        );
        if let Err(e) = res {
            log_send("org_kde_kwin_server_decoration_manager.create", &e);
        }
    }

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
    /// - `surface`:
    #[inline]
    pub fn new_try_send_create(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Result<Rc<OrgKdeKwinServerDecoration>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_create(
            &id,
            surface,
        )?;
        Ok(id)
    }

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
    /// - `surface`:
    #[inline]
    pub fn new_send_create(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Rc<OrgKdeKwinServerDecoration> {
        let id = self.core.create_child();
        self.send_create(
            &id,
            surface,
        );
        id
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
    pub fn try_send_default_mode(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= org_kde_kwin_server_decoration_manager#{}.default_mode(mode: {})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
    ) {
        let res = self.try_send_default_mode(
            mode,
        );
        if let Err(e) = res {
            log_send("org_kde_kwin_server_decoration_manager.default_mode", &e);
        }
    }
}

/// A message handler for [`OrgKdeKwinServerDecorationManager`] proxies.
pub trait OrgKdeKwinServerDecorationManagerHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<OrgKdeKwinServerDecorationManager>) {
        slf.core.delete_id();
    }

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
    fn handle_create(
        &mut self,
        slf: &Rc<OrgKdeKwinServerDecorationManager>,
        id: &Rc<OrgKdeKwinServerDecoration>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create(
            id,
            surface,
        );
        if let Err(e) = res {
            log_forward("org_kde_kwin_server_decoration_manager.create", &e);
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
    fn handle_default_mode(
        &mut self,
        slf: &Rc<OrgKdeKwinServerDecorationManager>,
        mode: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_default_mode(
            mode,
        );
        if let Err(e) = res {
            log_forward("org_kde_kwin_server_decoration_manager.default_mode", &e);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> org_kde_kwin_server_decoration_manager#{}.create(id: org_kde_kwin_server_decoration#{}, surface: wl_surface#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = OrgKdeKwinServerDecoration::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_create(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_create(&self, arg0, arg1);
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
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> org_kde_kwin_server_decoration_manager#{}.default_mode(mode: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_default_mode(&self, arg0);
                } else {
                    DefaultHandler.handle_default_mode(&self, arg0);
                }
            }
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
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
