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

impl ConcreteObject for WpDrmLeaseV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WpDrmLeaseV1;
    const INTERFACE_NAME: &str = "wp_drm_lease_v1";
}

impl WpDrmLeaseV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WpDrmLeaseV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
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
    pub fn try_send_lease_fd(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_drm_lease_v1#{}.lease_fd(leased_fd: {})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0.as_raw_fd());
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
    ) {
        let res = self.try_send_lease_fd(
            leased_fd,
        );
        if let Err(e) = res {
            log_send("wp_drm_lease_v1.lease_fd", &e);
        }
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
    pub fn try_send_finished(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_drm_lease_v1#{}.finished()\n", client_id, id);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id);
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
    ) {
        let res = self.try_send_finished(
        );
        if let Err(e) = res {
            log_send("wp_drm_lease_v1.finished", &e);
        }
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_drm_lease_v1#{}.destroy()\n", id);
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
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wp_drm_lease_v1.destroy", &e);
        }
    }
}

/// A message handler for [`WpDrmLeaseV1`] proxies.
pub trait WpDrmLeaseV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpDrmLeaseV1>) {
        slf.core.delete_id();
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
        slf: &Rc<WpDrmLeaseV1>,
        leased_fd: &Rc<OwnedFd>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_lease_fd(
            leased_fd,
        );
        if let Err(e) = res {
            log_forward("wp_drm_lease_v1.lease_fd", &e);
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
        slf: &Rc<WpDrmLeaseV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_finished(
        );
        if let Err(e) = res {
            log_forward("wp_drm_lease_v1.finished", &e);
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
        slf: &Rc<WpDrmLeaseV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wp_drm_lease_v1.destroy", &e);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_drm_lease_v1#{}.destroy()\n", client_id, id);
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
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                let Some(arg0) = fds.pop_front() else {
                    return Err(ObjectError(ObjectErrorKind::MissingFd("leased_fd")));
                };
                let arg0 = &arg0;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_drm_lease_v1#{}.lease_fd(leased_fd: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0.as_raw_fd());
                }
                if let Some(handler) = handler {
                    (**handler).handle_lease_fd(&self, arg0);
                } else {
                    DefaultHandler.handle_lease_fd(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_drm_lease_v1#{}.finished()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_finished(&self);
                } else {
                    DefaultHandler.handle_finished(&self);
                }
            }
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
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

