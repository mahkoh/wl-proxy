//! global for providing explicit synchronization
//!
//! This global is a factory interface, allowing clients to request
//! explicit synchronization for buffers on a per-surface basis.
//!
//! See wp_linux_drm_syncobj_surface_v1 for more information.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_linux_drm_syncobj_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpLinuxDrmSyncobjManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpLinuxDrmSyncobjManagerV1Handler>,
}

struct DefaultHandler;

impl WpLinuxDrmSyncobjManagerV1Handler for DefaultHandler { }

impl ConcreteObject for WpLinuxDrmSyncobjManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WpLinuxDrmSyncobjManagerV1;
    const INTERFACE_NAME: &str = "wp_linux_drm_syncobj_manager_v1";
}

impl WpLinuxDrmSyncobjManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WpLinuxDrmSyncobjManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_linux_drm_syncobj_manager_v1#{}.destroy()\n", id);
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

    /// destroy explicit synchronization factory object
    ///
    /// Destroy this explicit synchronization factory object. Other objects
    /// shall not be affected by this request.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wp_linux_drm_syncobj_manager_v1.destroy", &e);
        }
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
    pub fn try_send_get_surface(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_linux_drm_syncobj_manager_v1#{}.get_surface(id: wp_linux_drm_syncobj_surface_v1#{}, surface: wl_surface#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id);
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
            arg0_id,
            arg1_id,
        ]);
        Ok(())
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
    #[inline]
    pub fn send_get_surface(
        &self,
        id: &Rc<WpLinuxDrmSyncobjSurfaceV1>,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_get_surface(
            id,
            surface,
        );
        if let Err(e) = res {
            log_send("wp_linux_drm_syncobj_manager_v1.get_surface", &e);
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
    /// - `surface`: the surface
    #[inline]
    pub fn new_try_send_get_surface(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Result<Rc<WpLinuxDrmSyncobjSurfaceV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_surface(
            &id,
            surface,
        )?;
        Ok(id)
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
    /// - `surface`: the surface
    #[inline]
    pub fn new_send_get_surface(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Rc<WpLinuxDrmSyncobjSurfaceV1> {
        let id = self.core.create_child();
        self.send_get_surface(
            &id,
            surface,
        );
        id
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
    pub fn try_send_import_timeline(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_linux_drm_syncobj_manager_v1#{}.import_timeline(id: wp_linux_drm_syncobj_timeline_v1#{}, fd: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1.as_raw_fd());
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
        fmt.words([
            id,
            2,
            arg0_id,
        ]);
        Ok(())
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
    pub fn send_import_timeline(
        &self,
        id: &Rc<WpLinuxDrmSyncobjTimelineV1>,
        fd: &Rc<OwnedFd>,
    ) {
        let res = self.try_send_import_timeline(
            id,
            fd,
        );
        if let Err(e) = res {
            log_send("wp_linux_drm_syncobj_manager_v1.import_timeline", &e);
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
    /// - `fd`: drm_syncobj file descriptor
    #[inline]
    pub fn new_try_send_import_timeline(
        &self,
        fd: &Rc<OwnedFd>,
    ) -> Result<Rc<WpLinuxDrmSyncobjTimelineV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_import_timeline(
            &id,
            fd,
        )?;
        Ok(id)
    }

    /// import a DRM syncobj timeline
    ///
    /// Import a DRM synchronization object timeline.
    ///
    /// If the FD cannot be imported, the invalid_timeline error is raised.
    ///
    /// # Arguments
    ///
    /// - `fd`: drm_syncobj file descriptor
    #[inline]
    pub fn new_send_import_timeline(
        &self,
        fd: &Rc<OwnedFd>,
    ) -> Rc<WpLinuxDrmSyncobjTimelineV1> {
        let id = self.core.create_child();
        self.send_import_timeline(
            &id,
            fd,
        );
        id
    }
}

/// A message handler for [`WpLinuxDrmSyncobjManagerV1`] proxies.
pub trait WpLinuxDrmSyncobjManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpLinuxDrmSyncobjManagerV1>) {
        slf.core.delete_id();
    }

    /// destroy explicit synchronization factory object
    ///
    /// Destroy this explicit synchronization factory object. Other objects
    /// shall not be affected by this request.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WpLinuxDrmSyncobjManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wp_linux_drm_syncobj_manager_v1.destroy", &e);
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
    fn handle_get_surface(
        &mut self,
        slf: &Rc<WpLinuxDrmSyncobjManagerV1>,
        id: &Rc<WpLinuxDrmSyncobjSurfaceV1>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_surface(
            id,
            surface,
        );
        if let Err(e) = res {
            log_forward("wp_linux_drm_syncobj_manager_v1.get_surface", &e);
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
    fn handle_import_timeline(
        &mut self,
        slf: &Rc<WpLinuxDrmSyncobjManagerV1>,
        id: &Rc<WpLinuxDrmSyncobjTimelineV1>,
        fd: &Rc<OwnedFd>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_import_timeline(
            id,
            fd,
        );
        if let Err(e) = res {
            log_forward("wp_linux_drm_syncobj_manager_v1.import_timeline", &e);
        }
    }
}

impl ObjectPrivate for WpLinuxDrmSyncobjManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpLinuxDrmSyncobjManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_linux_drm_syncobj_manager_v1#{}.destroy()\n", client_id, id);
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
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_linux_drm_syncobj_manager_v1#{}.get_surface(id: wp_linux_drm_syncobj_surface_v1#{}, surface: wl_surface#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = WpLinuxDrmSyncobjSurfaceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_surface(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_surface(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let Some(arg1) = fds.pop_front() else {
                    return Err(ObjectError(ObjectErrorKind::MissingFd("fd")));
                };
                let arg1 = &arg1;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_linux_drm_syncobj_manager_v1#{}.import_timeline(id: wp_linux_drm_syncobj_timeline_v1#{}, fd: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1.as_raw_fd());
                }
                let arg0_id = arg0;
                let arg0 = WpLinuxDrmSyncobjTimelineV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_import_timeline(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_import_timeline(&self, arg0, arg1);
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

impl Object for WpLinuxDrmSyncobjManagerV1 {
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
