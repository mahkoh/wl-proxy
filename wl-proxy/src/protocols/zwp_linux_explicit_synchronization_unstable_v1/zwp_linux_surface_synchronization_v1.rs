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

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_linux_surface_synchronization_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpLinuxSurfaceSynchronizationV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn ZwpLinuxSurfaceSynchronizationV1Handler>,
}

struct DefaultHandler;

impl ZwpLinuxSurfaceSynchronizationV1Handler for DefaultHandler { }

impl ZwpLinuxSurfaceSynchronizationV1 {
    pub const XML_VERSION: u32 = 2;
    pub const INTERFACE: ProxyInterface = ProxyInterface::ZwpLinuxSurfaceSynchronizationV1;
    pub const INTERFACE_NAME: &str = "zwp_linux_surface_synchronization_v1";
}

impl ZwpLinuxSurfaceSynchronizationV1 {
    pub fn set_handler(&self, handler: impl ZwpLinuxSurfaceSynchronizationV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpLinuxSurfaceSynchronizationV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpLinuxSurfaceSynchronizationV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpLinuxSurfaceSynchronizationV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpLinuxSurfaceSynchronizationV1 {
    /// Since when the destroy message is available.
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_linux_surface_synchronization_v1#{}.destroy()\n", id);
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

    /// Since when the set_acquire_fence message is available.
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_linux_surface_synchronization_v1#{}.set_acquire_fence(fd: {})\n", id, arg0.as_raw_fd());
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
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
        release: &Rc<ZwpLinuxBufferReleaseV1>,
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_linux_surface_synchronization_v1#{}.get_release(release: zwp_linux_buffer_release_v1#{})\n", id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpLinuxSurfaceSynchronizationV1] proxies.
pub trait ZwpLinuxSurfaceSynchronizationV1Handler: Any {
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
        _slf: &Rc<ZwpLinuxSurfaceSynchronizationV1>,
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
        _slf: &Rc<ZwpLinuxSurfaceSynchronizationV1>,
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
        _slf: &Rc<ZwpLinuxSurfaceSynchronizationV1>,
        release: &Rc<ZwpLinuxBufferReleaseV1>,
    ) {
        let res = _slf.send_get_release(
            release,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_linux_surface_synchronization_v1.get_release message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for ZwpLinuxSurfaceSynchronizationV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::ZwpLinuxSurfaceSynchronizationV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_linux_surface_synchronization_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                let Some(arg0) = fds.pop_front() else {
                    return Err(ObjectError::MissingFd("fd"));
                };
                let arg0 = &arg0;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_linux_surface_synchronization_v1#{}.set_acquire_fence(fd: {})\n", client.endpoint.id, msg[0], arg0.as_raw_fd());
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_acquire_fence(&self, arg0);
                } else {
                    DefaultHandler.set_acquire_fence(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_linux_surface_synchronization_v1#{}.get_release(release: zwp_linux_buffer_release_v1#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ZwpLinuxBufferReleaseV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "release", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).get_release(&self, arg0);
                } else {
                    DefaultHandler.get_release(&self, arg0);
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

impl Proxy for ZwpLinuxSurfaceSynchronizationV1 {
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

impl ZwpLinuxSurfaceSynchronizationV1 {
    /// Since when the error.invalid_fence enum variant is available.
    pub const ENM__ERROR_INVALID_FENCE__SINCE: u32 = 1;
    /// Since when the error.duplicate_fence enum variant is available.
    pub const ENM__ERROR_DUPLICATE_FENCE__SINCE: u32 = 1;
    /// Since when the error.duplicate_release enum variant is available.
    pub const ENM__ERROR_DUPLICATE_RELEASE__SINCE: u32 = 1;
    /// Since when the error.no_surface enum variant is available.
    pub const ENM__ERROR_NO_SURFACE__SINCE: u32 = 1;
    /// Since when the error.unsupported_buffer enum variant is available.
    pub const ENM__ERROR_UNSUPPORTED_BUFFER__SINCE: u32 = 1;
    /// Since when the error.no_buffer enum variant is available.
    pub const ENM__ERROR_NO_BUFFER__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwpLinuxSurfaceSynchronizationV1Error(pub u32);

impl ZwpLinuxSurfaceSynchronizationV1Error {
    /// the fence specified by the client could not be imported
    pub const INVALID_FENCE: Self = Self(0);

    /// multiple fences added for a single surface commit
    pub const DUPLICATE_FENCE: Self = Self(1);

    /// multiple releases added for a single surface commit
    pub const DUPLICATE_RELEASE: Self = Self(2);

    /// the associated wl_surface was destroyed
    pub const NO_SURFACE: Self = Self(3);

    /// the buffer does not support explicit synchronization
    pub const UNSUPPORTED_BUFFER: Self = Self(4);

    /// no buffer was attached
    pub const NO_BUFFER: Self = Self(5);
}

impl Debug for ZwpLinuxSurfaceSynchronizationV1Error {
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
