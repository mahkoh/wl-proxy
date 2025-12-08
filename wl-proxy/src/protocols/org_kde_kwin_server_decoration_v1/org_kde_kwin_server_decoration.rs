use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A org_kde_kwin_server_decoration object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct OrgKdeKwinServerDecoration {
    core: ObjectCore,
    handler: HandlerHolder<dyn OrgKdeKwinServerDecorationHandler>,
}

struct DefaultHandler;

impl OrgKdeKwinServerDecorationHandler for DefaultHandler { }

impl ConcreteObject for OrgKdeKwinServerDecoration {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::OrgKdeKwinServerDecoration;
    const INTERFACE_NAME: &str = "org_kde_kwin_server_decoration";
}

impl OrgKdeKwinServerDecoration {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl OrgKdeKwinServerDecorationHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn OrgKdeKwinServerDecorationHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for OrgKdeKwinServerDecoration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OrgKdeKwinServerDecoration")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl OrgKdeKwinServerDecoration {
    /// Since when the release message is available.
    pub const MSG__RELEASE__SINCE: u32 = 1;

    /// release the server decoration object
    #[inline]
    pub fn try_send_release(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= org_kde_kwin_server_decoration#{}.release()\n", id);
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
            0,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// release the server decoration object
    #[inline]
    pub fn send_release(
        &self,
    ) {
        let res = self.try_send_release(
        );
        if let Err(e) = res {
            log_send("org_kde_kwin_server_decoration.release", &e);
        }
    }

    /// Since when the request_mode message is available.
    pub const MSG__REQUEST_MODE__SINCE: u32 = 1;

    /// The decoration mode the surface wants to use.
    ///
    /// # Arguments
    ///
    /// - `mode`: The mode this surface wants to use.
    #[inline]
    pub fn try_send_request_mode(
        &self,
        mode: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            mode,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= org_kde_kwin_server_decoration#{}.request_mode(mode: {})\n", id, arg0);
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
            1,
            arg0,
        ]);
        Ok(())
    }

    /// The decoration mode the surface wants to use.
    ///
    /// # Arguments
    ///
    /// - `mode`: The mode this surface wants to use.
    #[inline]
    pub fn send_request_mode(
        &self,
        mode: u32,
    ) {
        let res = self.try_send_request_mode(
            mode,
        );
        if let Err(e) = res {
            log_send("org_kde_kwin_server_decoration.request_mode", &e);
        }
    }

    /// Since when the mode message is available.
    pub const MSG__MODE__SINCE: u32 = 1;

    /// The new decoration mode applied by the server
    ///
    /// This event is emitted directly after the decoration is created and
    /// represents the base decoration policy by the server. E.g. a server
    /// which wants all surfaces to be client-side decorated will send Client,
    /// a server which wants server-side decoration will send Server.
    ///
    /// The client can request a different mode through the decoration request.
    /// The server will acknowledge this by another event with the same mode. So
    /// even if a server prefers server-side decoration it's possible to force a
    /// client-side decoration.
    ///
    /// The server may emit this event at any time. In this case the client can
    /// again request a different mode. It's the responsibility of the server to
    /// prevent a feedback loop.
    ///
    /// # Arguments
    ///
    /// - `mode`: The decoration mode applied to the surface by the server.
    #[inline]
    pub fn try_send_mode(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= org_kde_kwin_server_decoration#{}.mode(mode: {})\n", client_id, id, arg0);
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

    /// The new decoration mode applied by the server
    ///
    /// This event is emitted directly after the decoration is created and
    /// represents the base decoration policy by the server. E.g. a server
    /// which wants all surfaces to be client-side decorated will send Client,
    /// a server which wants server-side decoration will send Server.
    ///
    /// The client can request a different mode through the decoration request.
    /// The server will acknowledge this by another event with the same mode. So
    /// even if a server prefers server-side decoration it's possible to force a
    /// client-side decoration.
    ///
    /// The server may emit this event at any time. In this case the client can
    /// again request a different mode. It's the responsibility of the server to
    /// prevent a feedback loop.
    ///
    /// # Arguments
    ///
    /// - `mode`: The decoration mode applied to the surface by the server.
    #[inline]
    pub fn send_mode(
        &self,
        mode: u32,
    ) {
        let res = self.try_send_mode(
            mode,
        );
        if let Err(e) = res {
            log_send("org_kde_kwin_server_decoration.mode", &e);
        }
    }
}

/// A message handler for [`OrgKdeKwinServerDecoration`] proxies.
pub trait OrgKdeKwinServerDecorationHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<OrgKdeKwinServerDecoration>) {
        slf.core.delete_id();
    }

    /// release the server decoration object
    #[inline]
    fn handle_release(
        &mut self,
        slf: &Rc<OrgKdeKwinServerDecoration>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_release(
        );
        if let Err(e) = res {
            log_forward("org_kde_kwin_server_decoration.release", &e);
        }
    }

    /// The decoration mode the surface wants to use.
    ///
    /// # Arguments
    ///
    /// - `mode`: The mode this surface wants to use.
    #[inline]
    fn handle_request_mode(
        &mut self,
        slf: &Rc<OrgKdeKwinServerDecoration>,
        mode: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_request_mode(
            mode,
        );
        if let Err(e) = res {
            log_forward("org_kde_kwin_server_decoration.request_mode", &e);
        }
    }

    /// The new decoration mode applied by the server
    ///
    /// This event is emitted directly after the decoration is created and
    /// represents the base decoration policy by the server. E.g. a server
    /// which wants all surfaces to be client-side decorated will send Client,
    /// a server which wants server-side decoration will send Server.
    ///
    /// The client can request a different mode through the decoration request.
    /// The server will acknowledge this by another event with the same mode. So
    /// even if a server prefers server-side decoration it's possible to force a
    /// client-side decoration.
    ///
    /// The server may emit this event at any time. In this case the client can
    /// again request a different mode. It's the responsibility of the server to
    /// prevent a feedback loop.
    ///
    /// # Arguments
    ///
    /// - `mode`: The decoration mode applied to the surface by the server.
    #[inline]
    fn handle_mode(
        &mut self,
        slf: &Rc<OrgKdeKwinServerDecoration>,
        mode: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_mode(
            mode,
        );
        if let Err(e) = res {
            log_forward("org_kde_kwin_server_decoration.mode", &e);
        }
    }
}

impl ObjectPrivate for OrgKdeKwinServerDecoration {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::OrgKdeKwinServerDecoration, version),
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
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> org_kde_kwin_server_decoration#{}.release()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_release(&self);
                } else {
                    DefaultHandler.handle_release(&self);
                }
            }
            1 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> org_kde_kwin_server_decoration#{}.request_mode(mode: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_request_mode(&self, arg0);
                } else {
                    DefaultHandler.handle_request_mode(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> org_kde_kwin_server_decoration#{}.mode(mode: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_mode(&self, arg0);
                } else {
                    DefaultHandler.handle_mode(&self, arg0);
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
            0 => "release",
            1 => "request_mode",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "mode",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for OrgKdeKwinServerDecoration {
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

impl OrgKdeKwinServerDecoration {
    /// Since when the mode.None enum variant is available.
    pub const ENM__MODE_NONE__SINCE: u32 = 1;
    /// Since when the mode.Client enum variant is available.
    pub const ENM__MODE_CLIENT__SINCE: u32 = 1;
    /// Since when the mode.Server enum variant is available.
    pub const ENM__MODE_SERVER__SINCE: u32 = 1;
}

/// Possible values to use in request_mode and the event mode.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OrgKdeKwinServerDecorationMode(pub u32);

impl OrgKdeKwinServerDecorationMode {
    /// Undecorated: The surface is not decorated at all, neither server nor client-side. An example is a popup surface which should not be decorated.
    pub const NONE: Self = Self(0);

    /// Client-side decoration: The decoration is part of the surface and the client.
    pub const CLIENT: Self = Self(1);

    /// Server-side decoration: The server embeds the surface into a decoration frame.
    pub const SERVER: Self = Self(2);
}

impl Debug for OrgKdeKwinServerDecorationMode {
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
