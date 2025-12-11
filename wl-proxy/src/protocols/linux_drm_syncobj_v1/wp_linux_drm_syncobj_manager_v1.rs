//! global for providing explicit synchronization
//!
//! This global is a factory interface, allowing clients to request
//! explicit synchronization for buffers on a per-surface basis.
//!
//! See wp_linux_drm_syncobj_surface_v1 for more information.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_linux_drm_syncobj_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpLinuxDrmSyncobjManagerV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn WpLinuxDrmSyncobjManagerV1Handler>,
}

struct DefaultHandler;

impl WpLinuxDrmSyncobjManagerV1Handler for DefaultHandler { }

impl WpLinuxDrmSyncobjManagerV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "wp_linux_drm_syncobj_manager_v1";
}

impl WpLinuxDrmSyncobjManagerV1 {
    pub fn set_handler(&self, handler: impl WpLinuxDrmSyncobjManagerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn WpLinuxDrmSyncobjManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpLinuxDrmSyncobjManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpLinuxDrmSyncobjManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpLinuxDrmSyncobjManagerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy explicit synchronization factory object
    ///
    /// Destroy this explicit synchronization factory object. Other objects
    /// shall not be affected by this request.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_linux_drm_syncobj_manager_v1#{}.destroy()\n", id);
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

    /// Since when the get_surface message is available.
    pub const MSG__GET_SURFACE__SINCE: u32 = 1;

    /// extend surface interface for explicit synchronization
    ///
    /// Instantiate an interface extension for the given wl_surface to provide
    /// explicit synchronization.
    ///
    /// If the given wl_surface already has an explicit synchronization object
    /// associated, the surface_exists protocol error is raised.
    ///
    /// Graphics APIs, like EGL or Vulkan, that manage the buffer queue and
    /// commits of a wl_surface themselves, are likely to be using this
    /// extension internally. If a client is using such an API for a
    /// wl_surface, it should not directly use this extension on that surface,
    /// to avoid raising a surface_exists protocol error.
    ///
    /// # Arguments
    ///
    /// - `id`: the new synchronization surface object id
    /// - `surface`: the surface
    #[inline]
    pub fn send_get_surface(
        &self,
        id: &Rc<WpLinuxDrmSyncobjSurfaceV1>,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_linux_drm_syncobj_manager_v1#{}.get_surface(id: wp_linux_drm_syncobj_surface_v1#{}, surface: wl_surface#{})\n", id, arg0_id, arg1_id);
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

    /// Since when the import_timeline message is available.
    pub const MSG__IMPORT_TIMELINE__SINCE: u32 = 1;

    /// import a DRM syncobj timeline
    ///
    /// Import a DRM synchronization object timeline.
    ///
    /// If the FD cannot be imported, the invalid_timeline error is raised.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `fd`: drm_syncobj file descriptor
    #[inline]
    pub fn send_import_timeline(
        &self,
        id: &Rc<WpLinuxDrmSyncobjTimelineV1>,
        fd: &Rc<OwnedFd>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            fd,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_linux_drm_syncobj_manager_v1#{}.import_timeline(id: wp_linux_drm_syncobj_timeline_v1#{}, fd: {})\n", id, arg0_id, arg1.as_raw_fd());
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
        fmt.words([
            id,
            2,
            arg0_id,
        ]);
        Ok(())
    }
}

/// A message handler for [WpLinuxDrmSyncobjManagerV1] proxies.
pub trait WpLinuxDrmSyncobjManagerV1Handler: Any {
    /// destroy explicit synchronization factory object
    ///
    /// Destroy this explicit synchronization factory object. Other objects
    /// shall not be affected by this request.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<WpLinuxDrmSyncobjManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_linux_drm_syncobj_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// extend surface interface for explicit synchronization
    ///
    /// Instantiate an interface extension for the given wl_surface to provide
    /// explicit synchronization.
    ///
    /// If the given wl_surface already has an explicit synchronization object
    /// associated, the surface_exists protocol error is raised.
    ///
    /// Graphics APIs, like EGL or Vulkan, that manage the buffer queue and
    /// commits of a wl_surface themselves, are likely to be using this
    /// extension internally. If a client is using such an API for a
    /// wl_surface, it should not directly use this extension on that surface,
    /// to avoid raising a surface_exists protocol error.
    ///
    /// # Arguments
    ///
    /// - `id`: the new synchronization surface object id
    /// - `surface`: the surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_surface(
        &mut self,
        _slf: &Rc<WpLinuxDrmSyncobjManagerV1>,
        id: &Rc<WpLinuxDrmSyncobjSurfaceV1>,
        surface: &Rc<WlSurface>,
    ) {
        let res = _slf.send_get_surface(
            id,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_linux_drm_syncobj_manager_v1.get_surface message: {}", Report::new(e));
        }
    }

    /// import a DRM syncobj timeline
    ///
    /// Import a DRM synchronization object timeline.
    ///
    /// If the FD cannot be imported, the invalid_timeline error is raised.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `fd`: drm_syncobj file descriptor
    #[inline]
    fn import_timeline(
        &mut self,
        _slf: &Rc<WpLinuxDrmSyncobjManagerV1>,
        id: &Rc<WpLinuxDrmSyncobjTimelineV1>,
        fd: &Rc<OwnedFd>,
    ) {
        let res = _slf.send_import_timeline(
            id,
            fd,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_linux_drm_syncobj_manager_v1.import_timeline message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for WpLinuxDrmSyncobjManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::WpLinuxDrmSyncobjManagerV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_linux_drm_syncobj_manager_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_linux_drm_syncobj_manager_v1#{}.get_surface(id: wp_linux_drm_syncobj_surface_v1#{}, surface: wl_surface#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = WpLinuxDrmSyncobjSurfaceV1::new(&self.core.state, self.core.version);
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
                    (**handler).get_surface(&self, arg0, arg1);
                } else {
                    DefaultHandler.get_surface(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let Some(arg1) = fds.pop_front() else {
                    return Err(ObjectError::MissingFd("fd"));
                };
                let arg1 = &arg1;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_linux_drm_syncobj_manager_v1#{}.import_timeline(id: wp_linux_drm_syncobj_timeline_v1#{}, fd: {})\n", client.endpoint.id, msg[0], arg0, arg1.as_raw_fd());
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = WpLinuxDrmSyncobjTimelineV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).import_timeline(&self, arg0, arg1);
                } else {
                    DefaultHandler.import_timeline(&self, arg0, arg1);
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
            1 => "get_surface",
            2 => "import_timeline",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Proxy for WpLinuxDrmSyncobjManagerV1 {
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

impl WpLinuxDrmSyncobjManagerV1 {
    /// Since when the error.surface_exists enum variant is available.
    pub const ENM__ERROR_SURFACE_EXISTS__SINCE: u32 = 1;
    /// Since when the error.invalid_timeline enum variant is available.
    pub const ENM__ERROR_INVALID_TIMELINE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpLinuxDrmSyncobjManagerV1Error(pub u32);

impl WpLinuxDrmSyncobjManagerV1Error {
    /// the surface already has a synchronization object associated
    pub const SURFACE_EXISTS: Self = Self(0);

    /// the timeline object could not be imported
    pub const INVALID_TIMELINE: Self = Self(1);
}

impl Debug for WpLinuxDrmSyncobjManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::SURFACE_EXISTS => "SURFACE_EXISTS",
            Self::INVALID_TIMELINE => "INVALID_TIMELINE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
