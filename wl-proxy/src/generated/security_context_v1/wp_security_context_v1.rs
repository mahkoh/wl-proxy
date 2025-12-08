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
pub struct MetaWpSecurityContextV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpSecurityContextV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpSecurityContextV1MessageHandler for DefaultMessageHandler { }

impl MetaWpSecurityContextV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpSecurityContextV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpSecurityContextV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpSecurityContextV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
        ]);
        Ok(())
    }

    /// Since when the set_sandbox_engine message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the set_app_id message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the set_instance_id message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            3,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the commit message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            4,
        ]);
        Ok(())
    }
}

/// A message handler for [WpSecurityContextV1] proxies.
#[allow(dead_code)]
pub trait MetaWpSecurityContextV1MessageHandler {
    /// destroy the security context object
    ///
    /// Destroy the security context object.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpSecurityContextV1>,
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
        _slf: &Rc<MetaWpSecurityContextV1>,
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
        _slf: &Rc<MetaWpSecurityContextV1>,
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
        _slf: &Rc<MetaWpSecurityContextV1>,
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
        _slf: &Rc<MetaWpSecurityContextV1>,
    ) {
        let res = _slf.send_commit(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_security_context_v1.commit message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpSecurityContextV1 {
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
            }
            1 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError);
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError);
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError);
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError);
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError);
                }
                if let Some(handler) = handler {
                    (**handler).set_sandbox_engine(&self, arg0);
                } else {
                    DefaultMessageHandler.set_sandbox_engine(&self, arg0);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError);
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError);
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError);
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError);
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError);
                }
                if let Some(handler) = handler {
                    (**handler).set_app_id(&self, arg0);
                } else {
                    DefaultMessageHandler.set_app_id(&self, arg0);
                }
            }
            3 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError);
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError);
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError);
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError);
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError);
                }
                if let Some(handler) = handler {
                    (**handler).set_instance_id(&self, arg0);
                } else {
                    DefaultMessageHandler.set_instance_id(&self, arg0);
                }
            }
            4 => {
                if let Some(handler) = handler {
                    (**handler).commit(&self);
                } else {
                    DefaultMessageHandler.commit(&self);
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

impl MetaWpSecurityContextV1 {
    /// Since when the error.already_used enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ALREADY_USED__SINCE: u32 = 1;
    /// Since when the error.already_set enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ALREADY_SET__SINCE: u32 = 1;
    /// Since when the error.invalid_metadata enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_METADATA__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWpSecurityContextV1Error(pub u32);

impl MetaWpSecurityContextV1Error {
    /// security context has already been committed
    #[allow(dead_code)]
    pub const ALREADY_USED: Self = Self(1);

    /// metadata has already been set
    #[allow(dead_code)]
    pub const ALREADY_SET: Self = Self(2);

    /// metadata is invalid
    #[allow(dead_code)]
    pub const INVALID_METADATA: Self = Self(3);
}

impl Debug for MetaWpSecurityContextV1Error {
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
