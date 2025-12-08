//! client security context manager
//!
//! This interface allows a client to register a new Wayland connection to
//! the compositor and attach a security context to it.
//!
//! This is intended to be used by sandboxes. Sandbox engines attach a
//! security context to all connections coming from inside the sandbox. The
//! compositor can then restrict the features that the sandboxed connections
//! can use.
//!
//! Compositors should forbid nesting multiple security contexts by not
//! exposing wp_security_context_manager_v1 global to clients with a security
//! context attached, or by sending the nested protocol error. Nested
//! security contexts are dangerous because they can potentially allow
//! privilege escalation of a sandboxed client.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_security_context_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpSecurityContextManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpSecurityContextManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpSecurityContextManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaWpSecurityContextManagerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWpSecurityContextManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WpSecurityContextManagerV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpSecurityContextManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpSecurityContextManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpSecurityContextManagerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the manager object
    ///
    /// Destroy the manager. This doesn't destroy objects created with the
    /// manager.
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

    /// Since when the create_listener message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE_LISTENER__SINCE: u32 = 1;

    /// create a new security context
    ///
    /// Creates a new security context with a socket listening FD.
    ///
    /// The compositor will accept new client connections on listen_fd.
    /// listen_fd must be ready to accept new connections when this request is
    /// sent by the client. In other words, the client must call bind(2) and
    /// listen(2) before sending the FD.
    ///
    /// close_fd is a FD that will signal hangup when the compositor should stop
    /// accepting new connections on listen_fd.
    ///
    /// The compositor must continue to accept connections on listen_fd when
    /// the Wayland client which created the security context disconnects.
    ///
    /// After sending this request, closing listen_fd and close_fd remains the
    /// only valid operation on them.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `listen_fd`: listening socket FD
    /// - `close_fd`: FD signaling when done
    #[inline]
    pub fn send_create_listener(
        &self,
        id: &Rc<MetaWpSecurityContextV1>,
        listen_fd: &Rc<OwnedFd>,
        close_fd: &Rc<OwnedFd>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            id,
            listen_fd,
            close_fd,
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
        fmt.fds.push_back(arg1.clone());
        fmt.fds.push_back(arg2.clone());
        fmt.words([
            id,
            1,
            arg0.server_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }
}

/// A message handler for [WpSecurityContextManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaWpSecurityContextManagerV1MessageHandler {
    /// destroy the manager object
    ///
    /// Destroy the manager. This doesn't destroy objects created with the
    /// manager.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpSecurityContextManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_security_context_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// create a new security context
    ///
    /// Creates a new security context with a socket listening FD.
    ///
    /// The compositor will accept new client connections on listen_fd.
    /// listen_fd must be ready to accept new connections when this request is
    /// sent by the client. In other words, the client must call bind(2) and
    /// listen(2) before sending the FD.
    ///
    /// close_fd is a FD that will signal hangup when the compositor should stop
    /// accepting new connections on listen_fd.
    ///
    /// The compositor must continue to accept connections on listen_fd when
    /// the Wayland client which created the security context disconnects.
    ///
    /// After sending this request, closing listen_fd and close_fd remains the
    /// only valid operation on them.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `listen_fd`: listening socket FD
    /// - `close_fd`: FD signaling when done
    #[inline]
    fn create_listener(
        &mut self,
        _slf: &Rc<MetaWpSecurityContextManagerV1>,
        id: &Rc<MetaWpSecurityContextV1>,
        listen_fd: &Rc<OwnedFd>,
        close_fd: &Rc<OwnedFd>,
    ) {
        let res = _slf.send_create_listener(
            id,
            listen_fd,
            close_fd,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_security_context_manager_v1.create_listener message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpSecurityContextManagerV1 {
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
                let Some(arg1) = fds.pop_front() else {
                    return Err(ObjectError);
                };
                let Some(arg2) = fds.pop_front() else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaWpSecurityContextV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                let arg1 = &arg1;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).create_listener(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.create_listener(&self, arg0, arg1, arg2);
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
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
    }
}

impl MetaWpSecurityContextManagerV1 {
    /// Since when the error.invalid_listen_fd enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_LISTEN_FD__SINCE: u32 = 1;
    /// Since when the error.nested enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_NESTED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWpSecurityContextManagerV1Error(pub u32);

impl MetaWpSecurityContextManagerV1Error {
    /// listening socket FD is invalid
    #[allow(dead_code)]
    pub const INVALID_LISTEN_FD: Self = Self(1);

    /// nested security contexts are forbidden
    #[allow(dead_code)]
    pub const NESTED: Self = Self(2);
}

impl Debug for MetaWpSecurityContextManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_LISTEN_FD => "INVALID_LISTEN_FD",
            Self::NESTED => "NESTED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
