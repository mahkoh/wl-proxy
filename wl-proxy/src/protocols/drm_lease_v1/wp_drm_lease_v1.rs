//! a DRM lease
//!
//! A DRM lease object is used to transfer the DRM file descriptor to the
//! client and manage the lifetime of the lease.
//!
//! Some time after the wp_drm_lease_v1 object is created, the compositor
//! will reply with the lease request's result. If the lease request is
//! granted, the compositor will send a lease_fd event. If the lease request
//! is denied, the compositor will send a finished event without a lease_fd
//! event.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_drm_lease_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpDrmLeaseV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpDrmLeaseV1Handler>,
}

struct DefaultHandler;

impl WpDrmLeaseV1Handler for DefaultHandler { }

impl WpDrmLeaseV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::WpDrmLeaseV1;
    pub const INTERFACE_NAME: &str = "wp_drm_lease_v1";
}

impl WpDrmLeaseV1 {
    pub fn set_handler(&self, handler: impl WpDrmLeaseV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn WpDrmLeaseV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpDrmLeaseV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpDrmLeaseV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpDrmLeaseV1 {
    /// Since when the lease_fd message is available.
    pub const MSG__LEASE_FD__SINCE: u32 = 1;

    /// shares the DRM file descriptor
    ///
    /// This event returns a file descriptor suitable for use with DRM-related
    /// ioctls. The client should use drmModeGetLease to enumerate the DRM
    /// objects which have been leased to them. The compositor guarantees it
    /// will not use the leased DRM objects itself until it sends the finished
    /// event. If the compositor cannot or will not grant a lease for the
    /// requested connectors, it will not send this event, instead sending the
    /// finished event.
    ///
    /// The compositor will send this event at most once during this objects
    /// lifetime.
    ///
    /// # Arguments
    ///
    /// - `leased_fd`: leased DRM file descriptor
    #[inline]
    pub fn send_lease_fd(
        &self,
        leased_fd: &Rc<OwnedFd>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            leased_fd,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_drm_lease_v1#{}.lease_fd(leased_fd: {})\n", client.endpoint.id, id, arg0.as_raw_fd());
            self.core.state.log(args);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.fds.push_back(arg0.clone());
        fmt.words([
            id,
            0,
        ]);
        Ok(())
    }

    /// Since when the finished message is available.
    pub const MSG__FINISHED__SINCE: u32 = 1;

    /// sent when the lease has been revoked
    ///
    /// The compositor uses this event to either reject a lease request, or if
    /// it previously sent a lease_fd, to notify the client that the lease has
    /// been revoked. If the client requires a new lease, they should destroy
    /// this object and submit a new lease request. The compositor will send
    /// no further events for this object after sending the finish event.
    /// Compositors should revoke the lease when any of the leased resources
    /// become unavailable, namely when a hot-unplug occurs or when the
    /// compositor loses DRM master. Compositors may advertise the connector
    /// for leasing again, if the resource is available, by sending the
    /// connector event through the wp_drm_lease_device_v1 interface.
    #[inline]
    pub fn send_finished(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_drm_lease_v1#{}.finished()\n", client.endpoint.id, id);
            self.core.state.log(args);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroys the lease object
    ///
    /// The client should send this to indicate that it no longer wishes to use
    /// this lease. The compositor should use drmModeRevokeLease on the
    /// appropriate file descriptor, if necessary.
    ///
    /// Upon destruction, the compositor should advertise the connector for
    /// leasing again by sending the connector event through the
    /// wp_drm_lease_device_v1 interface.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_drm_lease_v1#{}.destroy()\n", id);
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
}

/// A message handler for [WpDrmLeaseV1] proxies.
pub trait WpDrmLeaseV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpDrmLeaseV1>) {
        let _ = slf.core.delete_id();
    }

    /// shares the DRM file descriptor
    ///
    /// This event returns a file descriptor suitable for use with DRM-related
    /// ioctls. The client should use drmModeGetLease to enumerate the DRM
    /// objects which have been leased to them. The compositor guarantees it
    /// will not use the leased DRM objects itself until it sends the finished
    /// event. If the compositor cannot or will not grant a lease for the
    /// requested connectors, it will not send this event, instead sending the
    /// finished event.
    ///
    /// The compositor will send this event at most once during this objects
    /// lifetime.
    ///
    /// # Arguments
    ///
    /// - `leased_fd`: leased DRM file descriptor
    #[inline]
    fn handle_lease_fd(
        &mut self,
        _slf: &Rc<WpDrmLeaseV1>,
        leased_fd: &Rc<OwnedFd>,
    ) {
        let res = _slf.send_lease_fd(
            leased_fd,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_drm_lease_v1.lease_fd message: {}", Report::new(e));
        }
    }

    /// sent when the lease has been revoked
    ///
    /// The compositor uses this event to either reject a lease request, or if
    /// it previously sent a lease_fd, to notify the client that the lease has
    /// been revoked. If the client requires a new lease, they should destroy
    /// this object and submit a new lease request. The compositor will send
    /// no further events for this object after sending the finish event.
    /// Compositors should revoke the lease when any of the leased resources
    /// become unavailable, namely when a hot-unplug occurs or when the
    /// compositor loses DRM master. Compositors may advertise the connector
    /// for leasing again, if the resource is available, by sending the
    /// connector event through the wp_drm_lease_device_v1 interface.
    #[inline]
    fn handle_finished(
        &mut self,
        _slf: &Rc<WpDrmLeaseV1>,
    ) {
        let res = _slf.send_finished(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_drm_lease_v1.finished message: {}", Report::new(e));
        }
    }

    /// destroys the lease object
    ///
    /// The client should send this to indicate that it no longer wishes to use
    /// this lease. The compositor should use drmModeRevokeLease on the
    /// appropriate file descriptor, if necessary.
    ///
    /// Upon destruction, the compositor should advertise the connector for
    /// leasing again by sending the connector event through the
    /// wp_drm_lease_device_v1 interface.
    #[inline]
    fn handle_destroy(
        &mut self,
        _slf: &Rc<WpDrmLeaseV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_drm_lease_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for WpDrmLeaseV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpDrmLeaseV1, version),
            handler: Default::default(),
        })
    }

    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err((ObjectError::HandlerBorrowed, self));
        };
        if let Some(handler) = &mut *handler {
            handler.delete_id(&self);
        } else {
            let _ = self.core.delete_id();
        }
        Ok(())
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_drm_lease_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_destroy(&self);
                } else {
                    DefaultHandler.handle_destroy(&self);
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
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                let Some(arg0) = fds.pop_front() else {
                    return Err(ObjectError::MissingFd("leased_fd"));
                };
                let arg0 = &arg0;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_drm_lease_v1#{}.lease_fd(leased_fd: {})\n", msg[0], arg0.as_raw_fd());
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_lease_fd(&self, arg0);
                } else {
                    DefaultHandler.handle_lease_fd(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_drm_lease_v1#{}.finished()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_finished(&self);
                } else {
                    DefaultHandler.handle_finished(&self);
                }
            }
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
        Ok(())
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "lease_fd",
            1 => "finished",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WpDrmLeaseV1 {
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

