//! protocol for providing explicit synchronization
//!
//! This global is a factory interface, allowing clients to request
//! explicit synchronization for buffers on a per-surface basis.
//!
//! See zwp_linux_surface_synchronization_v1 for more information.
//!
//! This interface is derived from Chromium's
//! zcr_linux_explicit_synchronization_v1.
//!
//! Note: this protocol is superseded by linux-drm-syncobj.
//!
//! Warning! The protocol described in this file is experimental and
//! backward incompatible changes may be made. Backward compatible changes
//! may be added together with the corresponding interface version bump.
//! Backward incompatible changes are done by bumping the version number in
//! the protocol and interface names and resetting the interface version.
//! Once the protocol is to be declared stable, the 'z' prefix and the
//! version number in the protocol and interface names are removed and the
//! interface version number is reset.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_linux_explicit_synchronization_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpLinuxExplicitSynchronizationV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn ZwpLinuxExplicitSynchronizationV1Handler>,
}

struct DefaultHandler;

impl ZwpLinuxExplicitSynchronizationV1Handler for DefaultHandler { }

impl ZwpLinuxExplicitSynchronizationV1 {
    pub const XML_VERSION: u32 = 2;
    pub const INTERFACE: &str = "zwp_linux_explicit_synchronization_v1";
}

impl ZwpLinuxExplicitSynchronizationV1 {
    pub fn set_handler(&self, handler: impl ZwpLinuxExplicitSynchronizationV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpLinuxExplicitSynchronizationV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpLinuxExplicitSynchronizationV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpLinuxExplicitSynchronizationV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpLinuxExplicitSynchronizationV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy explicit synchronization factory object
    ///
    /// Destroy this explicit synchronization factory object. Other objects,
    /// including zwp_linux_surface_synchronization_v1 objects created by this
    /// factory, shall not be affected by this request.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_linux_explicit_synchronization_v1#{}.destroy()\n", id);
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

    /// Since when the get_synchronization message is available.
    pub const MSG__GET_SYNCHRONIZATION__SINCE: u32 = 1;

    /// extend surface interface for explicit synchronization
    ///
    /// Instantiate an interface extension for the given wl_surface to provide
    /// explicit synchronization.
    ///
    /// If the given wl_surface already has an explicit synchronization object
    /// associated, the synchronization_exists protocol error is raised.
    ///
    /// Graphics APIs, like EGL or Vulkan, that manage the buffer queue and
    /// commits of a wl_surface themselves, are likely to be using this
    /// extension internally. If a client is using such an API for a
    /// wl_surface, it should not directly use this extension on that surface,
    /// to avoid raising a synchronization_exists protocol error.
    ///
    /// # Arguments
    ///
    /// - `id`: the new synchronization interface id
    /// - `surface`: the surface
    #[inline]
    pub fn send_get_synchronization(
        &self,
        id: &Rc<ZwpLinuxSurfaceSynchronizationV1>,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_linux_explicit_synchronization_v1#{}.get_synchronization(id: zwp_linux_surface_synchronization_v1#{}, surface: wl_surface#{})\n", id, arg0_id, arg1_id);
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
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpLinuxExplicitSynchronizationV1] proxies.
pub trait ZwpLinuxExplicitSynchronizationV1Handler: Any {
    /// destroy explicit synchronization factory object
    ///
    /// Destroy this explicit synchronization factory object. Other objects,
    /// including zwp_linux_surface_synchronization_v1 objects created by this
    /// factory, shall not be affected by this request.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<ZwpLinuxExplicitSynchronizationV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_linux_explicit_synchronization_v1.destroy message: {}", Report::new(e));
        }
    }

    /// extend surface interface for explicit synchronization
    ///
    /// Instantiate an interface extension for the given wl_surface to provide
    /// explicit synchronization.
    ///
    /// If the given wl_surface already has an explicit synchronization object
    /// associated, the synchronization_exists protocol error is raised.
    ///
    /// Graphics APIs, like EGL or Vulkan, that manage the buffer queue and
    /// commits of a wl_surface themselves, are likely to be using this
    /// extension internally. If a client is using such an API for a
    /// wl_surface, it should not directly use this extension on that surface,
    /// to avoid raising a synchronization_exists protocol error.
    ///
    /// # Arguments
    ///
    /// - `id`: the new synchronization interface id
    /// - `surface`: the surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_synchronization(
        &mut self,
        _slf: &Rc<ZwpLinuxExplicitSynchronizationV1>,
        id: &Rc<ZwpLinuxSurfaceSynchronizationV1>,
        surface: &Rc<WlSurface>,
    ) {
        let res = _slf.send_get_synchronization(
            id,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_linux_explicit_synchronization_v1.get_synchronization message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for ZwpLinuxExplicitSynchronizationV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::ZwpLinuxExplicitSynchronizationV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_linux_explicit_synchronization_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_linux_explicit_synchronization_v1#{}.get_synchronization(id: zwp_linux_surface_synchronization_v1#{}, surface: wl_surface#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ZwpLinuxSurfaceSynchronizationV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ProxyInterface::WlSurface));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_synchronization(&self, arg0, arg1);
                } else {
                    DefaultHandler.get_synchronization(&self, arg0, arg1);
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
            1 => "get_synchronization",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Proxy for ZwpLinuxExplicitSynchronizationV1 {
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

impl ZwpLinuxExplicitSynchronizationV1 {
    /// Since when the error.synchronization_exists enum variant is available.
    pub const ENM__ERROR_SYNCHRONIZATION_EXISTS__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwpLinuxExplicitSynchronizationV1Error(pub u32);

impl ZwpLinuxExplicitSynchronizationV1Error {
    /// the surface already has a synchronization object associated
    pub const SYNCHRONIZATION_EXISTS: Self = Self(0);
}

impl Debug for ZwpLinuxExplicitSynchronizationV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::SYNCHRONIZATION_EXISTS => "SYNCHRONIZATION_EXISTS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
