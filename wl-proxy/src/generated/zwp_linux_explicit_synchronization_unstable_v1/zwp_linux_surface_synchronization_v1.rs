//! per-surface explicit synchronization support
//!
//! This object implements per-surface explicit synchronization.
//!
//! Synchronization refers to co-ordination of pipelined operations performed
//! on buffers. Most GPU clients will schedule an asynchronous operation to
//! render to the buffer, then immediately send the buffer to the compositor
//! to be attached to a surface.
//!
//! In implicit synchronization, ensuring that the rendering operation is
//! complete before the compositor displays the buffer is an implementation
//! detail handled by either the kernel or userspace graphics driver.
//!
//! By contrast, in explicit synchronization, dma_fence objects mark when the
//! asynchronous operations are complete. When submitting a buffer, the
//! client provides an acquire fence which will be waited on before the
//! compositor accesses the buffer. The Wayland server, through a
//! zwp_linux_buffer_release_v1 object, will inform the client with an event
//! which may be accompanied by a release fence, when the compositor will no
//! longer access the buffer contents due to the specific commit that
//! requested the release event.
//!
//! Each surface can be associated with only one object of this interface at
//! any time.
//!
//! In version 1 of this interface, explicit synchronization is only
//! guaranteed to be supported for buffers created with any version of the
//! wp_linux_dmabuf buffer factory. Version 2 additionally guarantees
//! explicit synchronization support for opaque EGL buffers, which is a type
//! of platform specific buffers described in the EGL_WL_bind_wayland_display
//! extension. Compositors are free to support explicit synchronization for
//! additional buffer types.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_linux_surface_synchronization_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpLinuxSurfaceSynchronizationV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpLinuxSurfaceSynchronizationV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpLinuxSurfaceSynchronizationV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpLinuxSurfaceSynchronizationV1 {
    pub const XML_VERSION: u32 = 2;
}

impl MetaZwpLinuxSurfaceSynchronizationV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpLinuxSurfaceSynchronizationV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwpLinuxSurfaceSynchronizationV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpLinuxSurfaceSynchronizationV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpLinuxSurfaceSynchronizationV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy synchronization object
    ///
    /// Destroy this explicit synchronization object.
    ///
    /// Any fence set by this object with set_acquire_fence since the last
    /// commit will be discarded by the server. Any fences set by this object
    /// before the last commit are not affected.
    ///
    /// zwp_linux_buffer_release_v1 objects created by this object are not
    /// affected by this request.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= zwp_linux_surface_synchronization_v1#{}.destroy()", id);
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

    /// Since when the set_acquire_fence message is available.
    #[allow(dead_code)]
    pub const MSG__SET_ACQUIRE_FENCE__SINCE: u32 = 1;

    /// set the acquire fence
    ///
    /// Set the acquire fence that must be signaled before the compositor
    /// may sample from the buffer attached with wl_surface.attach. The fence
    /// is a dma_fence kernel object.
    ///
    /// The acquire fence is double-buffered state, and will be applied on the
    /// next wl_surface.commit request for the associated surface. Thus, it
    /// applies only to the buffer that is attached to the surface at commit
    /// time.
    ///
    /// If the provided fd is not a valid dma_fence fd, then an INVALID_FENCE
    /// error is raised.
    ///
    /// If a fence has already been attached during the same commit cycle, a
    /// DUPLICATE_FENCE error is raised.
    ///
    /// If the associated wl_surface was destroyed, a NO_SURFACE error is
    /// raised.
    ///
    /// If at surface commit time the attached buffer does not support explicit
    /// synchronization, an UNSUPPORTED_BUFFER error is raised.
    ///
    /// If at surface commit time there is no buffer attached, a NO_BUFFER
    /// error is raised.
    ///
    /// # Arguments
    ///
    /// - `fd`: acquire fence fd
    #[inline]
    pub fn send_set_acquire_fence(
        &self,
        fd: &Rc<OwnedFd>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            fd,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= zwp_linux_surface_synchronization_v1#{}.set_acquire_fence(fd: {})", id, arg0.as_raw_fd());
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.fds.push_back(arg0.clone());
        fmt.words([
            id,
            1,
        ]);
        Ok(())
    }

    /// Since when the get_release message is available.
    #[allow(dead_code)]
    pub const MSG__GET_RELEASE__SINCE: u32 = 1;

    /// release fence for last-attached buffer
    ///
    /// Create a listener for the release of the buffer attached by the
    /// client with wl_surface.attach. See zwp_linux_buffer_release_v1
    /// documentation for more information.
    ///
    /// The release object is double-buffered state, and will be associated
    /// with the buffer that is attached to the surface at wl_surface.commit
    /// time.
    ///
    /// If a zwp_linux_buffer_release_v1 object has already been requested for
    /// the surface in the same commit cycle, a DUPLICATE_RELEASE error is
    /// raised.
    ///
    /// If the associated wl_surface was destroyed, a NO_SURFACE error
    /// is raised.
    ///
    /// If at surface commit time there is no buffer attached, a NO_BUFFER
    /// error is raised.
    #[inline]
    pub fn send_get_release(
        &self,
        release: &Rc<MetaZwpLinuxBufferReleaseV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            release,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("release", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        eprintln!("server      <= zwp_linux_surface_synchronization_v1#{}.get_release(release: zwp_linux_buffer_release_v1#{})", id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpLinuxSurfaceSynchronizationV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpLinuxSurfaceSynchronizationV1MessageHandler {
    /// destroy synchronization object
    ///
    /// Destroy this explicit synchronization object.
    ///
    /// Any fence set by this object with set_acquire_fence since the last
    /// commit will be discarded by the server. Any fences set by this object
    /// before the last commit are not affected.
    ///
    /// zwp_linux_buffer_release_v1 objects created by this object are not
    /// affected by this request.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpLinuxSurfaceSynchronizationV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_linux_surface_synchronization_v1.destroy message: {}", Report::new(e));
        }
    }

    /// set the acquire fence
    ///
    /// Set the acquire fence that must be signaled before the compositor
    /// may sample from the buffer attached with wl_surface.attach. The fence
    /// is a dma_fence kernel object.
    ///
    /// The acquire fence is double-buffered state, and will be applied on the
    /// next wl_surface.commit request for the associated surface. Thus, it
    /// applies only to the buffer that is attached to the surface at commit
    /// time.
    ///
    /// If the provided fd is not a valid dma_fence fd, then an INVALID_FENCE
    /// error is raised.
    ///
    /// If a fence has already been attached during the same commit cycle, a
    /// DUPLICATE_FENCE error is raised.
    ///
    /// If the associated wl_surface was destroyed, a NO_SURFACE error is
    /// raised.
    ///
    /// If at surface commit time the attached buffer does not support explicit
    /// synchronization, an UNSUPPORTED_BUFFER error is raised.
    ///
    /// If at surface commit time there is no buffer attached, a NO_BUFFER
    /// error is raised.
    ///
    /// # Arguments
    ///
    /// - `fd`: acquire fence fd
    #[inline]
    fn set_acquire_fence(
        &mut self,
        _slf: &Rc<MetaZwpLinuxSurfaceSynchronizationV1>,
        fd: &Rc<OwnedFd>,
    ) {
        let res = _slf.send_set_acquire_fence(
            fd,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_linux_surface_synchronization_v1.set_acquire_fence message: {}", Report::new(e));
        }
    }

    /// release fence for last-attached buffer
    ///
    /// Create a listener for the release of the buffer attached by the
    /// client with wl_surface.attach. See zwp_linux_buffer_release_v1
    /// documentation for more information.
    ///
    /// The release object is double-buffered state, and will be associated
    /// with the buffer that is attached to the surface at wl_surface.commit
    /// time.
    ///
    /// If a zwp_linux_buffer_release_v1 object has already been requested for
    /// the surface in the same commit cycle, a DUPLICATE_RELEASE error is
    /// raised.
    ///
    /// If the associated wl_surface was destroyed, a NO_SURFACE error
    /// is raised.
    ///
    /// If at surface commit time there is no buffer attached, a NO_BUFFER
    /// error is raised.
    ///
    /// # Arguments
    ///
    /// - `release`: new zwp_linux_buffer_release_v1 object
    #[inline]
    fn get_release(
        &mut self,
        _slf: &Rc<MetaZwpLinuxSurfaceSynchronizationV1>,
        release: &Rc<MetaZwpLinuxBufferReleaseV1>,
    ) {
        let res = _slf.send_get_release(
            release,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_linux_surface_synchronization_v1.get_release message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpLinuxSurfaceSynchronizationV1 {
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
                eprintln!("client#{:04} -> zwp_linux_surface_synchronization_v1#{}.destroy()", client.endpoint.id, msg[0]);
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                let Some(arg0) = fds.pop_front() else {
                    return Err(ObjectError::MissingFd("fd"));
                };
                let arg0 = &arg0;
                eprintln!("client#{:04} -> zwp_linux_surface_synchronization_v1#{}.set_acquire_fence(fd: {})", client.endpoint.id, msg[0], arg0.as_raw_fd());
                if let Some(handler) = handler {
                    (**handler).set_acquire_fence(&self, arg0);
                } else {
                    DefaultMessageHandler.set_acquire_fence(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                eprintln!("client#{:04} -> zwp_linux_surface_synchronization_v1#{}.get_release(release: zwp_linux_buffer_release_v1#{})", client.endpoint.id, msg[0], arg0);
                let arg0_id = arg0;
                let arg0 = MetaZwpLinuxBufferReleaseV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "release", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).get_release(&self, arg0);
                } else {
                    DefaultMessageHandler.get_release(&self, arg0);
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
            1 => "set_acquire_fence",
            2 => "get_release",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl MetaZwpLinuxSurfaceSynchronizationV1 {
    /// Since when the error.invalid_fence enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_FENCE__SINCE: u32 = 1;
    /// Since when the error.duplicate_fence enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_DUPLICATE_FENCE__SINCE: u32 = 1;
    /// Since when the error.duplicate_release enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_DUPLICATE_RELEASE__SINCE: u32 = 1;
    /// Since when the error.no_surface enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_NO_SURFACE__SINCE: u32 = 1;
    /// Since when the error.unsupported_buffer enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_UNSUPPORTED_BUFFER__SINCE: u32 = 1;
    /// Since when the error.no_buffer enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_NO_BUFFER__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaZwpLinuxSurfaceSynchronizationV1Error(pub u32);

impl MetaZwpLinuxSurfaceSynchronizationV1Error {
    /// the fence specified by the client could not be imported
    #[allow(dead_code)]
    pub const INVALID_FENCE: Self = Self(0);

    /// multiple fences added for a single surface commit
    #[allow(dead_code)]
    pub const DUPLICATE_FENCE: Self = Self(1);

    /// multiple releases added for a single surface commit
    #[allow(dead_code)]
    pub const DUPLICATE_RELEASE: Self = Self(2);

    /// the associated wl_surface was destroyed
    #[allow(dead_code)]
    pub const NO_SURFACE: Self = Self(3);

    /// the buffer does not support explicit synchronization
    #[allow(dead_code)]
    pub const UNSUPPORTED_BUFFER: Self = Self(4);

    /// no buffer was attached
    #[allow(dead_code)]
    pub const NO_BUFFER: Self = Self(5);
}

impl Debug for MetaZwpLinuxSurfaceSynchronizationV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_FENCE => "INVALID_FENCE",
            Self::DUPLICATE_FENCE => "DUPLICATE_FENCE",
            Self::DUPLICATE_RELEASE => "DUPLICATE_RELEASE",
            Self::NO_SURFACE => "NO_SURFACE",
            Self::UNSUPPORTED_BUFFER => "UNSUPPORTED_BUFFER",
            Self::NO_BUFFER => "NO_BUFFER",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
