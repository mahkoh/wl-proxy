//! client security context
//!
//! The security context allows a client to register a new client and attach
//! security context metadata to the connections.
//!
//! When both are set, the combination of the application ID and the sandbox
//! engine must uniquely identify an application. The same application ID
//! will be used across instances (e.g. if the application is restarted, or
//! if the application is started multiple times).
//!
//! When both are set, the combination of the instance ID and the sandbox
//! engine must uniquely identify a running instance of an application.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_security_context_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpSecurityContextV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn WpSecurityContextV1Handler>,
}

struct DefaultHandler;

impl WpSecurityContextV1Handler for DefaultHandler { }

impl WpSecurityContextV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "wp_security_context_v1";
}

impl WpSecurityContextV1 {
    pub fn set_handler(&self, handler: impl WpSecurityContextV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn WpSecurityContextV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpSecurityContextV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpSecurityContextV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpSecurityContextV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the security context object
    ///
    /// Destroy the security context object.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_security_context_v1#{}.destroy()\n", id);
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

    /// Since when the set_sandbox_engine message is available.
    pub const MSG__SET_SANDBOX_ENGINE__SINCE: u32 = 1;

    /// set the sandbox engine
    ///
    /// Attach a unique sandbox engine name to the security context. The name
    /// should follow the reverse-DNS style (e.g. "org.flatpak").
    ///
    /// A list of well-known engines is maintained at:
    /// https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/security-context/engines.md
    ///
    /// It is a protocol error to call this request twice. The already_set
    /// error is sent in this case.
    ///
    /// # Arguments
    ///
    /// - `name`: the sandbox engine name
    #[inline]
    pub fn send_set_sandbox_engine(
        &self,
        name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            name,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_security_context_v1#{}.set_sandbox_engine(name: {:?})\n", id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the set_app_id message is available.
    pub const MSG__SET_APP_ID__SINCE: u32 = 1;

    /// set the application ID
    ///
    /// Attach an application ID to the security context.
    ///
    /// The application ID is an opaque, sandbox-specific identifier for an
    /// application. See the well-known engines document for more details:
    /// https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/security-context/engines.md
    ///
    /// The compositor may use the application ID to group clients belonging to
    /// the same security context application.
    ///
    /// Whether this request is optional or not depends on the sandbox engine used.
    ///
    /// It is a protocol error to call this request twice. The already_set
    /// error is sent in this case.
    ///
    /// # Arguments
    ///
    /// - `app_id`: the application ID
    #[inline]
    pub fn send_set_app_id(
        &self,
        app_id: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            app_id,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_security_context_v1#{}.set_app_id(app_id: {:?})\n", id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the set_instance_id message is available.
    pub const MSG__SET_INSTANCE_ID__SINCE: u32 = 1;

    /// set the instance ID
    ///
    /// Attach an instance ID to the security context.
    ///
    /// The instance ID is an opaque, sandbox-specific identifier for a running
    /// instance of an application. See the well-known engines document for
    /// more details:
    /// https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/security-context/engines.md
    ///
    /// Whether this request is optional or not depends on the sandbox engine used.
    ///
    /// It is a protocol error to call this request twice. The already_set
    /// error is sent in this case.
    ///
    /// # Arguments
    ///
    /// - `instance_id`: the instance ID
    #[inline]
    pub fn send_set_instance_id(
        &self,
        instance_id: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            instance_id,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_security_context_v1#{}.set_instance_id(instance_id: {:?})\n", id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the commit message is available.
    pub const MSG__COMMIT__SINCE: u32 = 1;

    /// register the security context
    ///
    /// Atomically register the new client and attach the security context
    /// metadata.
    ///
    /// If the provided metadata is inconsistent or does not match with out of
    /// band metadata (see
    /// https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/security-context/engines.md),
    /// the invalid_metadata error may be sent eventually.
    ///
    /// It's a protocol error to send any request other than "destroy" after
    /// this request. In this case, the already_used error is sent.
    #[inline]
    pub fn send_commit(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_security_context_v1#{}.commit()\n", id);
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
            4,
        ]);
        Ok(())
    }
}

/// A message handler for [WpSecurityContextV1] proxies.
pub trait WpSecurityContextV1Handler: Any {
    /// destroy the security context object
    ///
    /// Destroy the security context object.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<WpSecurityContextV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_security_context_v1.destroy message: {}", Report::new(e));
        }
    }

    /// set the sandbox engine
    ///
    /// Attach a unique sandbox engine name to the security context. The name
    /// should follow the reverse-DNS style (e.g. "org.flatpak").
    ///
    /// A list of well-known engines is maintained at:
    /// https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/security-context/engines.md
    ///
    /// It is a protocol error to call this request twice. The already_set
    /// error is sent in this case.
    ///
    /// # Arguments
    ///
    /// - `name`: the sandbox engine name
    #[inline]
    fn set_sandbox_engine(
        &mut self,
        _slf: &Rc<WpSecurityContextV1>,
        name: &str,
    ) {
        let res = _slf.send_set_sandbox_engine(
            name,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_security_context_v1.set_sandbox_engine message: {}", Report::new(e));
        }
    }

    /// set the application ID
    ///
    /// Attach an application ID to the security context.
    ///
    /// The application ID is an opaque, sandbox-specific identifier for an
    /// application. See the well-known engines document for more details:
    /// https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/security-context/engines.md
    ///
    /// The compositor may use the application ID to group clients belonging to
    /// the same security context application.
    ///
    /// Whether this request is optional or not depends on the sandbox engine used.
    ///
    /// It is a protocol error to call this request twice. The already_set
    /// error is sent in this case.
    ///
    /// # Arguments
    ///
    /// - `app_id`: the application ID
    #[inline]
    fn set_app_id(
        &mut self,
        _slf: &Rc<WpSecurityContextV1>,
        app_id: &str,
    ) {
        let res = _slf.send_set_app_id(
            app_id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_security_context_v1.set_app_id message: {}", Report::new(e));
        }
    }

    /// set the instance ID
    ///
    /// Attach an instance ID to the security context.
    ///
    /// The instance ID is an opaque, sandbox-specific identifier for a running
    /// instance of an application. See the well-known engines document for
    /// more details:
    /// https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/security-context/engines.md
    ///
    /// Whether this request is optional or not depends on the sandbox engine used.
    ///
    /// It is a protocol error to call this request twice. The already_set
    /// error is sent in this case.
    ///
    /// # Arguments
    ///
    /// - `instance_id`: the instance ID
    #[inline]
    fn set_instance_id(
        &mut self,
        _slf: &Rc<WpSecurityContextV1>,
        instance_id: &str,
    ) {
        let res = _slf.send_set_instance_id(
            instance_id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_security_context_v1.set_instance_id message: {}", Report::new(e));
        }
    }

    /// register the security context
    ///
    /// Atomically register the new client and attach the security context
    /// metadata.
    ///
    /// If the provided metadata is inconsistent or does not match with out of
    /// band metadata (see
    /// https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/security-context/engines.md),
    /// the invalid_metadata error may be sent eventually.
    ///
    /// It's a protocol error to send any request other than "destroy" after
    /// this request. In this case, the already_used error is sent.
    #[inline]
    fn commit(
        &mut self,
        _slf: &Rc<WpSecurityContextV1>,
    ) {
        let res = _slf.send_commit(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_security_context_v1.commit message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for WpSecurityContextV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::WpSecurityContextV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_security_context_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("name"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("name"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("name"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("name"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_security_context_v1#{}.set_sandbox_engine(name: {:?})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_sandbox_engine(&self, arg0);
                } else {
                    DefaultHandler.set_sandbox_engine(&self, arg0);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("app_id"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("app_id"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("app_id"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("app_id"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_security_context_v1#{}.set_app_id(app_id: {:?})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_app_id(&self, arg0);
                } else {
                    DefaultHandler.set_app_id(&self, arg0);
                }
            }
            3 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("instance_id"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("instance_id"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("instance_id"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("instance_id"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_security_context_v1#{}.set_instance_id(instance_id: {:?})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_instance_id(&self, arg0);
                } else {
                    DefaultHandler.set_instance_id(&self, arg0);
                }
            }
            4 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_security_context_v1#{}.commit()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).commit(&self);
                } else {
                    DefaultHandler.commit(&self);
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
            1 => "set_sandbox_engine",
            2 => "set_app_id",
            3 => "set_instance_id",
            4 => "commit",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Proxy for WpSecurityContextV1 {
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

impl WpSecurityContextV1 {
    /// Since when the error.already_used enum variant is available.
    pub const ENM__ERROR_ALREADY_USED__SINCE: u32 = 1;
    /// Since when the error.already_set enum variant is available.
    pub const ENM__ERROR_ALREADY_SET__SINCE: u32 = 1;
    /// Since when the error.invalid_metadata enum variant is available.
    pub const ENM__ERROR_INVALID_METADATA__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpSecurityContextV1Error(pub u32);

impl WpSecurityContextV1Error {
    /// security context has already been committed
    pub const ALREADY_USED: Self = Self(1);

    /// metadata has already been set
    pub const ALREADY_SET: Self = Self(2);

    /// metadata is invalid
    pub const INVALID_METADATA: Self = Self(3);
}

impl Debug for WpSecurityContextV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ALREADY_USED => "ALREADY_USED",
            Self::ALREADY_SET => "ALREADY_SET",
            Self::INVALID_METADATA => "INVALID_METADATA",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
