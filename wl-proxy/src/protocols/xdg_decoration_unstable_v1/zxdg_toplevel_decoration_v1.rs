//! decoration object for a toplevel surface
//!
//! The decoration object allows the compositor to toggle server-side window
//! decorations for a toplevel surface. The client can request to switch to
//! another mode.
//!
//! The xdg_toplevel_decoration object must be destroyed before its
//! xdg_toplevel.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zxdg_toplevel_decoration_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZxdgToplevelDecorationV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZxdgToplevelDecorationV1Handler>,
}

struct DefaultHandler;

impl ZxdgToplevelDecorationV1Handler for DefaultHandler { }

impl ConcreteObject for ZxdgToplevelDecorationV1 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::ZxdgToplevelDecorationV1;
    const INTERFACE_NAME: &str = "zxdg_toplevel_decoration_v1";
}

impl ZxdgToplevelDecorationV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZxdgToplevelDecorationV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZxdgToplevelDecorationV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZxdgToplevelDecorationV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZxdgToplevelDecorationV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZxdgToplevelDecorationV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the decoration object
    ///
    /// Switch back to a mode without any server-side decorations at the next
    /// commit, unless a new xdg_toplevel_decoration is created for the surface
    /// first.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zxdg_toplevel_decoration_v1#{}.destroy()\n", id);
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

    /// destroy the decoration object
    ///
    /// Switch back to a mode without any server-side decorations at the next
    /// commit, unless a new xdg_toplevel_decoration is created for the surface
    /// first.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zxdg_toplevel_decoration_v1.destroy", &e);
        }
    }

    /// Since when the set_mode message is available.
    pub const MSG__SET_MODE__SINCE: u32 = 1;

    /// set the decoration mode
    ///
    /// Set the toplevel surface decoration mode. This informs the compositor
    /// that the client prefers the provided decoration mode.
    ///
    /// After requesting a decoration mode, the compositor will respond by
    /// emitting an xdg_surface.configure event. The client should then update
    /// its content, drawing it without decorations if the received mode is
    /// server-side decorations. The client must also acknowledge the configure
    /// when committing the new content (see xdg_surface.ack_configure).
    ///
    /// The compositor can decide not to use the client's mode and enforce a
    /// different mode instead.
    ///
    /// Clients whose decoration mode depend on the xdg_toplevel state may send
    /// a set_mode request in response to an xdg_surface.configure event and wait
    /// for the next xdg_surface.configure event to prevent unwanted state.
    /// Such clients are responsible for preventing configure loops and must
    /// make sure not to send multiple successive set_mode requests with the
    /// same decoration mode.
    ///
    /// If an invalid mode is supplied by the client, the invalid_mode protocol
    /// error is raised by the compositor.
    ///
    /// # Arguments
    ///
    /// - `mode`: the decoration mode
    #[inline]
    pub fn try_send_set_mode(
        &self,
        mode: ZxdgToplevelDecorationV1Mode,
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
            fn log(state: &State, id: u32, arg0: ZxdgToplevelDecorationV1Mode) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zxdg_toplevel_decoration_v1#{}.set_mode(mode: {:?})\n", id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// set the decoration mode
    ///
    /// Set the toplevel surface decoration mode. This informs the compositor
    /// that the client prefers the provided decoration mode.
    ///
    /// After requesting a decoration mode, the compositor will respond by
    /// emitting an xdg_surface.configure event. The client should then update
    /// its content, drawing it without decorations if the received mode is
    /// server-side decorations. The client must also acknowledge the configure
    /// when committing the new content (see xdg_surface.ack_configure).
    ///
    /// The compositor can decide not to use the client's mode and enforce a
    /// different mode instead.
    ///
    /// Clients whose decoration mode depend on the xdg_toplevel state may send
    /// a set_mode request in response to an xdg_surface.configure event and wait
    /// for the next xdg_surface.configure event to prevent unwanted state.
    /// Such clients are responsible for preventing configure loops and must
    /// make sure not to send multiple successive set_mode requests with the
    /// same decoration mode.
    ///
    /// If an invalid mode is supplied by the client, the invalid_mode protocol
    /// error is raised by the compositor.
    ///
    /// # Arguments
    ///
    /// - `mode`: the decoration mode
    #[inline]
    pub fn send_set_mode(
        &self,
        mode: ZxdgToplevelDecorationV1Mode,
    ) {
        let res = self.try_send_set_mode(
            mode,
        );
        if let Err(e) = res {
            log_send("zxdg_toplevel_decoration_v1.set_mode", &e);
        }
    }

    /// Since when the unset_mode message is available.
    pub const MSG__UNSET_MODE__SINCE: u32 = 1;

    /// unset the decoration mode
    ///
    /// Unset the toplevel surface decoration mode. This informs the compositor
    /// that the client doesn't prefer a particular decoration mode.
    ///
    /// This request has the same semantics as set_mode.
    #[inline]
    pub fn try_send_unset_mode(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zxdg_toplevel_decoration_v1#{}.unset_mode()\n", id);
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
            2,
        ]);
        Ok(())
    }

    /// unset the decoration mode
    ///
    /// Unset the toplevel surface decoration mode. This informs the compositor
    /// that the client doesn't prefer a particular decoration mode.
    ///
    /// This request has the same semantics as set_mode.
    #[inline]
    pub fn send_unset_mode(
        &self,
    ) {
        let res = self.try_send_unset_mode(
        );
        if let Err(e) = res {
            log_send("zxdg_toplevel_decoration_v1.unset_mode", &e);
        }
    }

    /// Since when the configure message is available.
    pub const MSG__CONFIGURE__SINCE: u32 = 1;

    /// notify a decoration mode change
    ///
    /// The configure event configures the effective decoration mode. The
    /// configured state should not be applied immediately. Clients must send an
    /// ack_configure in response to this event. See xdg_surface.configure and
    /// xdg_surface.ack_configure for details.
    ///
    /// A configure event can be sent at any time. The specified mode must be
    /// obeyed by the client.
    ///
    /// # Arguments
    ///
    /// - `mode`: the decoration mode
    #[inline]
    pub fn try_send_configure(
        &self,
        mode: ZxdgToplevelDecorationV1Mode,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: ZxdgToplevelDecorationV1Mode) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zxdg_toplevel_decoration_v1#{}.configure(mode: {:?})\n", client_id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// notify a decoration mode change
    ///
    /// The configure event configures the effective decoration mode. The
    /// configured state should not be applied immediately. Clients must send an
    /// ack_configure in response to this event. See xdg_surface.configure and
    /// xdg_surface.ack_configure for details.
    ///
    /// A configure event can be sent at any time. The specified mode must be
    /// obeyed by the client.
    ///
    /// # Arguments
    ///
    /// - `mode`: the decoration mode
    #[inline]
    pub fn send_configure(
        &self,
        mode: ZxdgToplevelDecorationV1Mode,
    ) {
        let res = self.try_send_configure(
            mode,
        );
        if let Err(e) = res {
            log_send("zxdg_toplevel_decoration_v1.configure", &e);
        }
    }
}

/// A message handler for [`ZxdgToplevelDecorationV1`] proxies.
pub trait ZxdgToplevelDecorationV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZxdgToplevelDecorationV1>) {
        slf.core.delete_id();
    }

    /// destroy the decoration object
    ///
    /// Switch back to a mode without any server-side decorations at the next
    /// commit, unless a new xdg_toplevel_decoration is created for the surface
    /// first.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZxdgToplevelDecorationV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zxdg_toplevel_decoration_v1.destroy", &e);
        }
    }

    /// set the decoration mode
    ///
    /// Set the toplevel surface decoration mode. This informs the compositor
    /// that the client prefers the provided decoration mode.
    ///
    /// After requesting a decoration mode, the compositor will respond by
    /// emitting an xdg_surface.configure event. The client should then update
    /// its content, drawing it without decorations if the received mode is
    /// server-side decorations. The client must also acknowledge the configure
    /// when committing the new content (see xdg_surface.ack_configure).
    ///
    /// The compositor can decide not to use the client's mode and enforce a
    /// different mode instead.
    ///
    /// Clients whose decoration mode depend on the xdg_toplevel state may send
    /// a set_mode request in response to an xdg_surface.configure event and wait
    /// for the next xdg_surface.configure event to prevent unwanted state.
    /// Such clients are responsible for preventing configure loops and must
    /// make sure not to send multiple successive set_mode requests with the
    /// same decoration mode.
    ///
    /// If an invalid mode is supplied by the client, the invalid_mode protocol
    /// error is raised by the compositor.
    ///
    /// # Arguments
    ///
    /// - `mode`: the decoration mode
    #[inline]
    fn handle_set_mode(
        &mut self,
        slf: &Rc<ZxdgToplevelDecorationV1>,
        mode: ZxdgToplevelDecorationV1Mode,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_mode(
            mode,
        );
        if let Err(e) = res {
            log_forward("zxdg_toplevel_decoration_v1.set_mode", &e);
        }
    }

    /// unset the decoration mode
    ///
    /// Unset the toplevel surface decoration mode. This informs the compositor
    /// that the client doesn't prefer a particular decoration mode.
    ///
    /// This request has the same semantics as set_mode.
    #[inline]
    fn handle_unset_mode(
        &mut self,
        slf: &Rc<ZxdgToplevelDecorationV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_unset_mode(
        );
        if let Err(e) = res {
            log_forward("zxdg_toplevel_decoration_v1.unset_mode", &e);
        }
    }

    /// notify a decoration mode change
    ///
    /// The configure event configures the effective decoration mode. The
    /// configured state should not be applied immediately. Clients must send an
    /// ack_configure in response to this event. See xdg_surface.configure and
    /// xdg_surface.ack_configure for details.
    ///
    /// A configure event can be sent at any time. The specified mode must be
    /// obeyed by the client.
    ///
    /// # Arguments
    ///
    /// - `mode`: the decoration mode
    #[inline]
    fn handle_configure(
        &mut self,
        slf: &Rc<ZxdgToplevelDecorationV1>,
        mode: ZxdgToplevelDecorationV1Mode,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_configure(
            mode,
        );
        if let Err(e) = res {
            log_forward("zxdg_toplevel_decoration_v1.configure", &e);
        }
    }
}

impl ObjectPrivate for ZxdgToplevelDecorationV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZxdgToplevelDecorationV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zxdg_toplevel_decoration_v1#{}.destroy()\n", client_id, id);
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
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = ZxdgToplevelDecorationV1Mode(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: ZxdgToplevelDecorationV1Mode) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zxdg_toplevel_decoration_v1#{}.set_mode(mode: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_mode(&self, arg0);
                } else {
                    DefaultHandler.handle_set_mode(&self, arg0);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zxdg_toplevel_decoration_v1#{}.unset_mode()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_unset_mode(&self);
                } else {
                    DefaultHandler.handle_unset_mode(&self);
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
                let arg0 = ZxdgToplevelDecorationV1Mode(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: ZxdgToplevelDecorationV1Mode) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zxdg_toplevel_decoration_v1#{}.configure(mode: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_configure(&self, arg0);
                } else {
                    DefaultHandler.handle_configure(&self, arg0);
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
            0 => "destroy",
            1 => "set_mode",
            2 => "unset_mode",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "configure",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZxdgToplevelDecorationV1 {
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

impl ZxdgToplevelDecorationV1 {
    /// Since when the error.unconfigured_buffer enum variant is available.
    pub const ENM__ERROR_UNCONFIGURED_BUFFER__SINCE: u32 = 1;
    /// Since when the error.already_constructed enum variant is available.
    pub const ENM__ERROR_ALREADY_CONSTRUCTED__SINCE: u32 = 1;
    /// Since when the error.orphaned enum variant is available.
    pub const ENM__ERROR_ORPHANED__SINCE: u32 = 1;
    /// Since when the error.invalid_mode enum variant is available.
    pub const ENM__ERROR_INVALID_MODE__SINCE: u32 = 1;

    /// Since when the mode.client_side enum variant is available.
    pub const ENM__MODE_CLIENT_SIDE__SINCE: u32 = 1;
    /// Since when the mode.server_side enum variant is available.
    pub const ENM__MODE_SERVER_SIDE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZxdgToplevelDecorationV1Error(pub u32);

impl ZxdgToplevelDecorationV1Error {
    /// xdg_toplevel has a buffer attached before configure
    pub const UNCONFIGURED_BUFFER: Self = Self(0);

    /// xdg_toplevel already has a decoration object
    pub const ALREADY_CONSTRUCTED: Self = Self(1);

    /// xdg_toplevel destroyed before the decoration object
    pub const ORPHANED: Self = Self(2);

    /// invalid mode
    pub const INVALID_MODE: Self = Self(3);
}

impl Debug for ZxdgToplevelDecorationV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::UNCONFIGURED_BUFFER => "UNCONFIGURED_BUFFER",
            Self::ALREADY_CONSTRUCTED => "ALREADY_CONSTRUCTED",
            Self::ORPHANED => "ORPHANED",
            Self::INVALID_MODE => "INVALID_MODE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// window decoration modes
///
/// These values describe window decoration modes.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZxdgToplevelDecorationV1Mode(pub u32);

impl ZxdgToplevelDecorationV1Mode {
    /// no server-side window decoration
    pub const CLIENT_SIDE: Self = Self(1);

    /// server-side window decoration
    pub const SERVER_SIDE: Self = Self(2);
}

impl Debug for ZxdgToplevelDecorationV1Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::CLIENT_SIDE => "CLIENT_SIDE",
            Self::SERVER_SIDE => "SERVER_SIDE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
