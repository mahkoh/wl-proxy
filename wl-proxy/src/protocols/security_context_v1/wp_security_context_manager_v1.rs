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

impl ConcreteObject for WpSecurityContextManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WpSecurityContextManagerV1;
    const INTERFACE_NAME: &str = "wp_security_context_manager_v1";
}

impl WpSecurityContextManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WpSecurityContextManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_security_context_manager_v1#{}.destroy()\n", id);
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

    /// destroy the manager object
    ///
    /// Destroy the manager. This doesn't destroy objects created with the
    /// manager.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wp_security_context_manager_v1.destroy", &e);
        }
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
    pub fn try_send_create_listener(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: i32, arg2: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_security_context_manager_v1#{}.create_listener(id: wp_security_context_v1#{}, listen_fd: {}, close_fd: {})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1.as_raw_fd(), arg2.as_raw_fd());
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
        fmt.fds.push_back(arg1.clone());
        fmt.fds.push_back(arg2.clone());
        fmt.words([
            id,
            1,
            arg0_id,
        ]);
        Ok(())
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
    pub fn send_create_listener(
        &self,
        id: &Rc<WpSecurityContextV1>,
        listen_fd: &Rc<OwnedFd>,
        close_fd: &Rc<OwnedFd>,
    ) {
        let res = self.try_send_create_listener(
            id,
            listen_fd,
            close_fd,
        );
        if let Err(e) = res {
            log_send("wp_security_context_manager_v1.create_listener", &e);
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
    /// - `listen_fd`: listening socket FD
    /// - `close_fd`: FD signaling when done
    #[inline]
    pub fn new_try_send_create_listener(
        &self,
        listen_fd: &Rc<OwnedFd>,
        close_fd: &Rc<OwnedFd>,
    ) -> Result<Rc<WpSecurityContextV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_create_listener(
            &id,
            listen_fd,
            close_fd,
        )?;
        Ok(id)
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
    /// - `listen_fd`: listening socket FD
    /// - `close_fd`: FD signaling when done
    #[inline]
    pub fn new_send_create_listener(
        &self,
        listen_fd: &Rc<OwnedFd>,
        close_fd: &Rc<OwnedFd>,
    ) -> Rc<WpSecurityContextV1> {
        let id = self.core.create_child();
        self.send_create_listener(
            &id,
            listen_fd,
            close_fd,
        );
        id
    }
}

/// A message handler for [`WpSecurityContextManagerV1`] proxies.
pub trait WpSecurityContextManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpSecurityContextManagerV1>) {
        slf.core.delete_id();
    }

    /// destroy the manager object
    ///
    /// Destroy the manager. This doesn't destroy objects created with the
    /// manager.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WpSecurityContextManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wp_security_context_manager_v1.destroy", &e);
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
    fn handle_create_listener(
        &mut self,
        slf: &Rc<WpSecurityContextManagerV1>,
        id: &Rc<WpSecurityContextV1>,
        listen_fd: &Rc<OwnedFd>,
        close_fd: &Rc<OwnedFd>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_listener(
            id,
            listen_fd,
            close_fd,
        );
        if let Err(e) = res {
            log_forward("wp_security_context_manager_v1.create_listener", &e);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_security_context_manager_v1#{}.destroy()\n", client_id, id);
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
                let Some(arg1) = fds.pop_front() else {
                    return Err(ObjectError(ObjectErrorKind::MissingFd("listen_fd")));
                };
                let Some(arg2) = fds.pop_front() else {
                    return Err(ObjectError(ObjectErrorKind::MissingFd("close_fd")));
                };
                let arg1 = &arg1;
                let arg2 = &arg2;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: i32, arg2: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_security_context_manager_v1#{}.create_listener(id: wp_security_context_v1#{}, listen_fd: {}, close_fd: {})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1.as_raw_fd(), arg2.as_raw_fd());
                }
                let arg0_id = arg0;
                let arg0 = WpSecurityContextV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_listener(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_create_listener(&self, arg0, arg1, arg2);
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
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
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
