//! decoration object for a toplevel surface
//!
//! The decoration object allows the compositor to toggle server-side window
//! decorations for a toplevel surface. The client can request to switch to
//! another mode.
//!
//! The xdg_toplevel_decoration object must be destroyed before its
//! xdg_toplevel.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zxdg_toplevel_decoration_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZxdgToplevelDecorationV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZxdgToplevelDecorationV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZxdgToplevelDecorationV1MessageHandler for DefaultMessageHandler { }

impl MetaZxdgToplevelDecorationV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZxdgToplevelDecorationV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZxdgToplevelDecorationV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaZxdgToplevelDecorationV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaZxdgToplevelDecorationV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZxdgToplevelDecorationV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZxdgToplevelDecorationV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the decoration object
    ///
    /// Switch back to a mode without any server-side decorations at the next
    /// commit.
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
            0,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the set_mode message is available.
    #[allow(dead_code)]
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
    pub fn send_set_mode(
        &self,
        mode: MetaZxdgToplevelDecorationV1Mode,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            mode,
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
            1,
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the unset_mode message is available.
    #[allow(dead_code)]
    pub const MSG__UNSET_MODE__SINCE: u32 = 1;

    /// unset the decoration mode
    ///
    /// Unset the toplevel surface decoration mode. This informs the compositor
    /// that the client doesn't prefer a particular decoration mode.
    ///
    /// This request has the same semantics as set_mode.
    #[inline]
    pub fn send_unset_mode(
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
            2,
        ]);
        Ok(())
    }

    /// Since when the configure message is available.
    #[allow(dead_code)]
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
    pub fn send_configure(
        &self,
        mode: MetaZxdgToplevelDecorationV1Mode,
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
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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
}

/// A message handler for [ZxdgToplevelDecorationV1] proxies.
#[allow(dead_code)]
pub trait MetaZxdgToplevelDecorationV1MessageHandler {
    /// destroy the decoration object
    ///
    /// Switch back to a mode without any server-side decorations at the next
    /// commit.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZxdgToplevelDecorationV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_toplevel_decoration_v1.destroy message: {}", Report::new(e));
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
    fn set_mode(
        &mut self,
        _slf: &Rc<MetaZxdgToplevelDecorationV1>,
        mode: MetaZxdgToplevelDecorationV1Mode,
    ) {
        let res = _slf.send_set_mode(
            mode,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_toplevel_decoration_v1.set_mode message: {}", Report::new(e));
        }
    }

    /// unset the decoration mode
    ///
    /// Unset the toplevel surface decoration mode. This informs the compositor
    /// that the client doesn't prefer a particular decoration mode.
    ///
    /// This request has the same semantics as set_mode.
    #[inline]
    fn unset_mode(
        &mut self,
        _slf: &Rc<MetaZxdgToplevelDecorationV1>,
    ) {
        let res = _slf.send_unset_mode(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_toplevel_decoration_v1.unset_mode message: {}", Report::new(e));
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
    fn configure(
        &mut self,
        _slf: &Rc<MetaZxdgToplevelDecorationV1>,
        mode: MetaZxdgToplevelDecorationV1Mode,
    ) {
        let res = _slf.send_configure(
            mode,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_toplevel_decoration_v1.configure message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZxdgToplevelDecorationV1 {
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
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = MetaZxdgToplevelDecorationV1Mode(arg0);
                if let Some(handler) = handler {
                    (**handler).set_mode(&self, arg0);
                } else {
                    DefaultMessageHandler.set_mode(&self, arg0);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).unset_mode(&self);
                } else {
                    DefaultMessageHandler.unset_mode(&self);
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
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = MetaZxdgToplevelDecorationV1Mode(arg0);
                if let Some(handler) = handler {
                    (**handler).configure(&self, arg0);
                } else {
                    DefaultMessageHandler.configure(&self, arg0);
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

impl MetaZxdgToplevelDecorationV1 {
    /// Since when the error.unconfigured_buffer enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_UNCONFIGURED_BUFFER__SINCE: u32 = 1;
    /// Since when the error.already_constructed enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ALREADY_CONSTRUCTED__SINCE: u32 = 1;
    /// Since when the error.orphaned enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ORPHANED__SINCE: u32 = 1;
    /// Since when the error.invalid_mode enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_MODE__SINCE: u32 = 1;

    /// Since when the mode.client_side enum variant is available.
    #[allow(dead_code)]
    pub const ENM__MODE_CLIENT_SIDE__SINCE: u32 = 1;
    /// Since when the mode.server_side enum variant is available.
    #[allow(dead_code)]
    pub const ENM__MODE_SERVER_SIDE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaZxdgToplevelDecorationV1Error(pub u32);

impl MetaZxdgToplevelDecorationV1Error {
    /// xdg_toplevel has a buffer attached before configure
    #[allow(dead_code)]
    pub const UNCONFIGURED_BUFFER: Self = Self(0);

    /// xdg_toplevel already has a decoration object
    #[allow(dead_code)]
    pub const ALREADY_CONSTRUCTED: Self = Self(1);

    /// xdg_toplevel destroyed before the decoration object
    #[allow(dead_code)]
    pub const ORPHANED: Self = Self(2);

    /// invalid mode
    #[allow(dead_code)]
    pub const INVALID_MODE: Self = Self(3);
}

impl Debug for MetaZxdgToplevelDecorationV1Error {
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
#[allow(dead_code)]
pub struct MetaZxdgToplevelDecorationV1Mode(pub u32);

impl MetaZxdgToplevelDecorationV1Mode {
    /// no server-side window decoration
    #[allow(dead_code)]
    pub const CLIENT_SIDE: Self = Self(1);

    /// server-side window decoration
    #[allow(dead_code)]
    pub const SERVER_SIDE: Self = Self(2);
}

impl Debug for MetaZxdgToplevelDecorationV1Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::CLIENT_SIDE => "CLIENT_SIDE",
            Self::SERVER_SIDE => "SERVER_SIDE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
