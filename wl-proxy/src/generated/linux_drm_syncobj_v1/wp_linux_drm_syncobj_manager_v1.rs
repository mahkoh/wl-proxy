//! global for providing explicit synchronization
//!
//! This global is a factory interface, allowing clients to request
//! explicit synchronization for buffers on a per-surface basis.
//!
//! See wp_linux_drm_syncobj_surface_v1 for more information.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_linux_drm_syncobj_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpLinuxDrmSyncobjManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpLinuxDrmSyncobjManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpLinuxDrmSyncobjManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaWpLinuxDrmSyncobjManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpLinuxDrmSyncobjManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpLinuxDrmSyncobjManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpLinuxDrmSyncobjManagerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
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

    /// Since when the get_surface message is available.
    #[allow(dead_code)]
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
        id: &Rc<MetaWpLinuxDrmSyncobjSurfaceV1>,
        surface: &Rc<MetaWlSurface>,
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
            return Err(ObjectError);
        };
        let arg1 = match arg1.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
        ]);
        Ok(())
    }

    /// Since when the import_timeline message is available.
    #[allow(dead_code)]
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
        id: &Rc<MetaWpLinuxDrmSyncobjTimelineV1>,
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
            return Err(ObjectError);
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.fds.push_back(arg1.clone());
        fmt.words([
            id,
            2,
            arg0.server_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }
}

/// A message handler for [WpLinuxDrmSyncobjManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaWpLinuxDrmSyncobjManagerV1MessageHandler {
    /// destroy explicit synchronization factory object
    ///
    /// Destroy this explicit synchronization factory object. Other objects
    /// shall not be affected by this request.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpLinuxDrmSyncobjManagerV1>,
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
        _slf: &Rc<MetaWpLinuxDrmSyncobjManagerV1>,
        id: &Rc<MetaWpLinuxDrmSyncobjSurfaceV1>,
        surface: &Rc<MetaWlSurface>,
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
        _slf: &Rc<MetaWpLinuxDrmSyncobjManagerV1>,
        id: &Rc<MetaWpLinuxDrmSyncobjTimelineV1>,
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

impl Proxy for MetaWpLinuxDrmSyncobjManagerV1 {
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
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaWpLinuxDrmSyncobjSurfaceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_surface(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_surface(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let Some(arg1) = fds.pop_front() else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaWpLinuxDrmSyncobjTimelineV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).import_timeline(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.import_timeline(&self, arg0, arg1);
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

impl MetaWpLinuxDrmSyncobjManagerV1 {
    /// Since when the error.surface_exists enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_SURFACE_EXISTS__SINCE: u32 = 1;
    /// Since when the error.invalid_timeline enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_TIMELINE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWpLinuxDrmSyncobjManagerV1Error(pub u32);

impl MetaWpLinuxDrmSyncobjManagerV1Error {
    /// the surface already has a synchronization object associated
    #[allow(dead_code)]
    pub const SURFACE_EXISTS: Self = Self(0);

    /// the timeline object could not be imported
    #[allow(dead_code)]
    pub const INVALID_TIMELINE: Self = Self(1);
}

impl Debug for MetaWpLinuxDrmSyncobjManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::SURFACE_EXISTS => "SURFACE_EXISTS",
            Self::INVALID_TIMELINE => "INVALID_TIMELINE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
