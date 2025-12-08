//! lease device
//!
//! This protocol is used by Wayland compositors which act as Direct
//! Rendering Manager (DRM) masters to lease DRM resources to Wayland
//! clients.
//!
//! The compositor will advertise one wp_drm_lease_device_v1 global for each
//! DRM node. Some time after a client binds to the wp_drm_lease_device_v1
//! global, the compositor will send a drm_fd event followed by zero, one or
//! more connector events. After all currently available connectors have been
//! sent, the compositor will send a wp_drm_lease_device_v1.done event.
//!
//! When the list of connectors available for lease changes the compositor
//! will send wp_drm_lease_device_v1.connector events for added connectors and
//! wp_drm_lease_connector_v1.withdrawn events for removed connectors,
//! followed by a wp_drm_lease_device_v1.done event.
//!
//! The compositor will indicate when a device is gone by removing the global
//! via a wl_registry.global_remove event. Upon receiving this event, the
//! client should destroy any matching wp_drm_lease_device_v1 object.
//!
//! To destroy a wp_drm_lease_device_v1 object, the client must first issue
//! a release request. Upon receiving this request, the compositor will
//! immediately send a released event and destroy the object. The client must
//! continue to process and discard drm_fd and connector events until it
//! receives the released event. Upon receiving the released event, the
//! client can safely cleanup any client-side resources.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_drm_lease_device_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpDrmLeaseDeviceV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpDrmLeaseDeviceV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpDrmLeaseDeviceV1MessageHandler for DefaultMessageHandler { }

impl MetaWpDrmLeaseDeviceV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpDrmLeaseDeviceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpDrmLeaseDeviceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpDrmLeaseDeviceV1 {
    /// Since when the create_lease_request message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE_LEASE_REQUEST__SINCE: u32 = 1;

    /// create a lease request object
    ///
    /// Creates a lease request object.
    ///
    /// See the documentation for wp_drm_lease_request_v1 for details.
    #[inline]
    pub fn send_create_lease_request(
        &self,
        id: &Rc<MetaWpDrmLeaseRequestV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
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
        fmt.words([
            id,
            0,
            arg0.server_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }

    /// Since when the release message is available.
    #[allow(dead_code)]
    pub const MSG__RELEASE__SINCE: u32 = 1;

    /// release this object
    ///
    /// Indicates the client no longer wishes to use this object. In response
    /// the compositor will immediately send the released event and destroy
    /// this object. It can however not guarantee that the client won't receive
    /// connector events before the released event. The client must not send any
    /// requests after this one, doing so will raise a wl_display error.
    /// Existing connectors, lease request and leases will not be affected.
    #[inline]
    pub fn send_release(
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
            1,
        ]);
        Ok(())
    }

    /// Since when the drm_fd message is available.
    #[allow(dead_code)]
    pub const MSG__DRM_FD__SINCE: u32 = 1;

    /// open a non-master fd for this DRM node
    ///
    /// The compositor will send this event when the wp_drm_lease_device_v1
    /// global is bound, although there are no guarantees as to how long this
    /// takes - the compositor might need to wait until regaining DRM master.
    /// The included fd is a non-master DRM file descriptor opened for this
    /// device and the compositor must not authenticate it.
    /// The purpose of this event is to give the client the ability to
    /// query DRM and discover information which may help them pick the
    /// appropriate DRM device or select the appropriate connectors therein.
    ///
    /// # Arguments
    ///
    /// - `fd`: DRM file descriptor
    #[inline]
    pub fn send_drm_fd(
        &self,
        fd: &Rc<OwnedFd>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            fd,
        );
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.fds.push_back(arg0.clone());
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
        ]);
        Ok(())
    }

    /// Since when the connector message is available.
    #[allow(dead_code)]
    pub const MSG__CONNECTOR__SINCE: u32 = 1;

    /// advertise connectors available for leases
    ///
    /// The compositor will use this event to advertise connectors available for
    /// lease by clients. This object may be passed into a lease request to
    /// indicate the client would like to lease that connector, see
    /// wp_drm_lease_request_v1.request_connector for details. While the
    /// compositor will make a best effort to not send disconnected connectors,
    /// no guarantees can be made.
    ///
    /// The compositor must send the drm_fd event before sending connectors.
    /// After the drm_fd event it will send all available connectors but may
    /// send additional connectors at any time.
    #[inline]
    pub fn send_connector(
        &self,
        id: &Rc<MetaWpDrmLeaseConnectorV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        arg0.generate_client_id(client, arg0_obj.clone())?;
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            1,
            arg0.client_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }

    /// Since when the done message is available.
    #[allow(dead_code)]
    pub const MSG__DONE__SINCE: u32 = 1;

    /// signals grouping of connectors
    ///
    /// The compositor will send this event to indicate that it has sent all
    /// currently available connectors after the client binds to the global or
    /// when it updates the connector list, for example on hotplug, drm master
    /// change or when a leased connector becomes available again. It will
    /// similarly send this event to group wp_drm_lease_connector_v1.withdrawn
    /// events of connectors of this device.
    #[inline]
    pub fn send_done(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            2,
        ]);
        Ok(())
    }

    /// Since when the released message is available.
    #[allow(dead_code)]
    pub const MSG__RELEASED__SINCE: u32 = 1;

    /// the compositor has finished using the device
    ///
    /// This event is sent in response to the release request and indicates
    /// that the compositor is done sending connector events.
    /// The compositor will destroy this object immediately after sending the
    /// event and it will become invalid. The client should release any
    /// resources associated with this device after receiving this event.
    #[inline]
    pub fn send_released(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            3,
        ]);
        Ok(())
    }
}

/// A message handler for [WpDrmLeaseDeviceV1] proxies.
#[allow(dead_code)]
pub trait MetaWpDrmLeaseDeviceV1MessageHandler {
    /// create a lease request object
    ///
    /// Creates a lease request object.
    ///
    /// See the documentation for wp_drm_lease_request_v1 for details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn create_lease_request(
        &mut self,
        _slf: &Rc<MetaWpDrmLeaseDeviceV1>,
        id: &Rc<MetaWpDrmLeaseRequestV1>,
    ) {
        let res = _slf.send_create_lease_request(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_drm_lease_device_v1.create_lease_request message: {}", Report::new(e));
        }
    }

    /// release this object
    ///
    /// Indicates the client no longer wishes to use this object. In response
    /// the compositor will immediately send the released event and destroy
    /// this object. It can however not guarantee that the client won't receive
    /// connector events before the released event. The client must not send any
    /// requests after this one, doing so will raise a wl_display error.
    /// Existing connectors, lease request and leases will not be affected.
    #[inline]
    fn release(
        &mut self,
        _slf: &Rc<MetaWpDrmLeaseDeviceV1>,
    ) {
        let res = _slf.send_release(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_drm_lease_device_v1.release message: {}", Report::new(e));
        }
    }

    /// open a non-master fd for this DRM node
    ///
    /// The compositor will send this event when the wp_drm_lease_device_v1
    /// global is bound, although there are no guarantees as to how long this
    /// takes - the compositor might need to wait until regaining DRM master.
    /// The included fd is a non-master DRM file descriptor opened for this
    /// device and the compositor must not authenticate it.
    /// The purpose of this event is to give the client the ability to
    /// query DRM and discover information which may help them pick the
    /// appropriate DRM device or select the appropriate connectors therein.
    ///
    /// # Arguments
    ///
    /// - `fd`: DRM file descriptor
    #[inline]
    fn drm_fd(
        &mut self,
        _slf: &Rc<MetaWpDrmLeaseDeviceV1>,
        fd: &Rc<OwnedFd>,
    ) {
        let res = _slf.send_drm_fd(
            fd,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_drm_lease_device_v1.drm_fd message: {}", Report::new(e));
        }
    }

    /// advertise connectors available for leases
    ///
    /// The compositor will use this event to advertise connectors available for
    /// lease by clients. This object may be passed into a lease request to
    /// indicate the client would like to lease that connector, see
    /// wp_drm_lease_request_v1.request_connector for details. While the
    /// compositor will make a best effort to not send disconnected connectors,
    /// no guarantees can be made.
    ///
    /// The compositor must send the drm_fd event before sending connectors.
    /// After the drm_fd event it will send all available connectors but may
    /// send additional connectors at any time.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn connector(
        &mut self,
        _slf: &Rc<MetaWpDrmLeaseDeviceV1>,
        id: &Rc<MetaWpDrmLeaseConnectorV1>,
    ) {
        let res = _slf.send_connector(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_drm_lease_device_v1.connector message: {}", Report::new(e));
        }
    }

    /// signals grouping of connectors
    ///
    /// The compositor will send this event to indicate that it has sent all
    /// currently available connectors after the client binds to the global or
    /// when it updates the connector list, for example on hotplug, drm master
    /// change or when a leased connector becomes available again. It will
    /// similarly send this event to group wp_drm_lease_connector_v1.withdrawn
    /// events of connectors of this device.
    #[inline]
    fn done(
        &mut self,
        _slf: &Rc<MetaWpDrmLeaseDeviceV1>,
    ) {
        let res = _slf.send_done(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_drm_lease_device_v1.done message: {}", Report::new(e));
        }
    }

    /// the compositor has finished using the device
    ///
    /// This event is sent in response to the release request and indicates
    /// that the compositor is done sending connector events.
    /// The compositor will destroy this object immediately after sending the
    /// event and it will become invalid. The client should release any
    /// resources associated with this device after receiving this event.
    #[inline]
    fn released(
        &mut self,
        _slf: &Rc<MetaWpDrmLeaseDeviceV1>,
    ) {
        let res = _slf.send_released(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_drm_lease_device_v1.released message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpDrmLeaseDeviceV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaWpDrmLeaseRequestV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).create_lease_request(&self, arg0);
                } else {
                    DefaultMessageHandler.create_lease_request(&self, arg0);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).release(&self);
                } else {
                    DefaultMessageHandler.release(&self);
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
            0 => {
                let Some(arg0) = fds.pop_front() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).drm_fd(&self, arg0);
                } else {
                    DefaultMessageHandler.drm_fd(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaWpDrmLeaseConnectorV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).connector(&self, arg0);
                } else {
                    DefaultMessageHandler.connector(&self, arg0);
                }
            }
            2 => {
                if let Some(handler) = handler {
                    (**handler).done(&self);
                } else {
                    DefaultMessageHandler.done(&self);
                }
            }
            3 => {
                if let Some(handler) = handler {
                    (**handler).released(&self);
                } else {
                    DefaultMessageHandler.released(&self);
                }
            }
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }
}

