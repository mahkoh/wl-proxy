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

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_security_context_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpSecurityContextManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpSecurityContextManagerV1Handler>,
}

struct DefaultHandler;

impl WpSecurityContextManagerV1Handler for DefaultHandler { }

impl WpSecurityContextManagerV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::WpSecurityContextManagerV1;
    pub const INTERFACE_NAME: &str = "wp_security_context_manager_v1";
}

impl WpSecurityContextManagerV1 {
    pub fn set_handler(&self, handler: impl WpSecurityContextManagerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn WpSecurityContextManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpSecurityContextManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpSecurityContextManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpSecurityContextManagerV1 {
    /// Since when the destroy message is available.
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_security_context_manager_v1#{}.destroy()\n", id);
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

    /// Since when the create_listener message is available.
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
        id: &Rc<WpSecurityContextV1>,
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_security_context_manager_v1#{}.create_listener(id: wp_security_context_v1#{}, listen_fd: {}, close_fd: {})\n", id, arg0_id, arg1.as_raw_fd(), arg2.as_raw_fd());
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.fds.push_back(arg1.clone());
        fmt.fds.push_back(arg2.clone());
        fmt.words([
            id,
            1,
            arg0_id,
        ]);
        Ok(())
    }
}

/// A message handler for [WpSecurityContextManagerV1] proxies.
pub trait WpSecurityContextManagerV1Handler: Any {
    /// destroy the manager object
    ///
    /// Destroy the manager. This doesn't destroy objects created with the
    /// manager.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<WpSecurityContextManagerV1>,
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
        _slf: &Rc<WpSecurityContextManagerV1>,
        id: &Rc<WpSecurityContextV1>,
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

impl ObjectPrivate for WpSecurityContextManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpSecurityContextManagerV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_security_context_manager_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
                let Some(arg1) = fds.pop_front() else {
                    return Err(ObjectError::MissingFd("listen_fd"));
                };
                let Some(arg2) = fds.pop_front() else {
                    return Err(ObjectError::MissingFd("close_fd"));
                };
                let arg1 = &arg1;
                let arg2 = &arg2;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_security_context_manager_v1#{}.create_listener(id: wp_security_context_v1#{}, listen_fd: {}, close_fd: {})\n", client.endpoint.id, msg[0], arg0, arg1.as_raw_fd(), arg2.as_raw_fd());
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = WpSecurityContextV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).create_listener(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.create_listener(&self, arg0, arg1, arg2);
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
            0 => "destroy",
            1 => "create_listener",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WpSecurityContextManagerV1 {
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

impl WpSecurityContextManagerV1 {
    /// Since when the error.invalid_listen_fd enum variant is available.
    pub const ENM__ERROR_INVALID_LISTEN_FD__SINCE: u32 = 1;
    /// Since when the error.nested enum variant is available.
    pub const ENM__ERROR_NESTED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpSecurityContextManagerV1Error(pub u32);

impl WpSecurityContextManagerV1Error {
    /// listening socket FD is invalid
    pub const INVALID_LISTEN_FD: Self = Self(1);

    /// nested security contexts are forbidden
    pub const NESTED: Self = Self(2);
}

impl Debug for WpSecurityContextManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_LISTEN_FD => "INVALID_LISTEN_FD",
            Self::NESTED => "NESTED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
