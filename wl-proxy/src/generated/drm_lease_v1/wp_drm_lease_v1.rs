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

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_drm_lease_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpDrmLeaseV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpDrmLeaseV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpDrmLeaseV1MessageHandler for DefaultMessageHandler { }

impl MetaWpDrmLeaseV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWpDrmLeaseV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WpDrmLeaseV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpDrmLeaseV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpDrmLeaseV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpDrmLeaseV1 {
    /// Since when the lease_fd message is available.
    #[allow(dead_code)]
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
        eprintln!("client#{:04} <= wp_drm_lease_v1#{}.lease_fd(leased_fd: {})", client.endpoint.id, id, arg0.as_raw_fd());
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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
    #[allow(dead_code)]
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
        eprintln!("client#{:04} <= wp_drm_lease_v1#{}.finished()", client.endpoint.id, id);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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
    #[allow(dead_code)]
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
        eprintln!("server      <= wp_drm_lease_v1#{}.destroy()", id);
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
}

/// A message handler for [WpDrmLeaseV1] proxies.
#[allow(dead_code)]
pub trait MetaWpDrmLeaseV1MessageHandler {
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
    fn lease_fd(
        &mut self,
        _slf: &Rc<MetaWpDrmLeaseV1>,
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
    fn finished(
        &mut self,
        _slf: &Rc<MetaWpDrmLeaseV1>,
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
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpDrmLeaseV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_drm_lease_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpDrmLeaseV1 {
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
                eprintln!("client#{:04} -> wp_drm_lease_v1#{}.destroy()", client.endpoint.id, msg[0]);
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
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
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                let Some(arg0) = fds.pop_front() else {
                    return Err(ObjectError::MissingFd("leased_fd"));
                };
                let arg0 = &arg0;
                eprintln!("server      -> wp_drm_lease_v1#{}.lease_fd(leased_fd: {})", msg[0], arg0.as_raw_fd());
                if let Some(handler) = handler {
                    (**handler).lease_fd(&self, arg0);
                } else {
                    DefaultMessageHandler.lease_fd(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("server      -> wp_drm_lease_v1#{}.finished()", msg[0]);
                if let Some(handler) = handler {
                    (**handler).finished(&self);
                } else {
                    DefaultMessageHandler.finished(&self);
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

