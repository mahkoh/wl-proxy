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

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_drm_lease_device_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpDrmLeaseDeviceV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpDrmLeaseDeviceV1Handler>,
}

struct DefaultHandler;

impl WpDrmLeaseDeviceV1Handler for DefaultHandler { }

impl WpDrmLeaseDeviceV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::WpDrmLeaseDeviceV1;
    pub const INTERFACE_NAME: &str = "wp_drm_lease_device_v1";
}

impl WpDrmLeaseDeviceV1 {
    pub fn set_handler(&self, handler: impl WpDrmLeaseDeviceV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn WpDrmLeaseDeviceV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpDrmLeaseDeviceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpDrmLeaseDeviceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpDrmLeaseDeviceV1 {
    /// Since when the create_lease_request message is available.
    pub const MSG__CREATE_LEASE_REQUEST__SINCE: u32 = 1;

    /// create a lease request object
    ///
    /// Creates a lease request object.
    ///
    /// See the documentation for wp_drm_lease_request_v1 for details.
    #[inline]
    pub fn send_create_lease_request(
        &self,
        id: &Rc<WpDrmLeaseRequestV1>,
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_drm_lease_device_v1#{}.create_lease_request(id: wp_drm_lease_request_v1#{})\n", id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the release message is available.
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_drm_lease_device_v1#{}.release()\n", id);
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
        ]);
        Ok(())
    }

    /// Since when the drm_fd message is available.
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
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_drm_lease_device_v1#{}.drm_fd(fd: {})\n", client.endpoint.id, id, arg0.as_raw_fd());
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

    /// Since when the connector message is available.
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
        id: &Rc<WpDrmLeaseConnectorV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateClientId("id", e))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_drm_lease_device_v1#{}.connector(id: wp_drm_lease_connector_v1#{})\n", client.endpoint.id, id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the done message is available.
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
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_drm_lease_device_v1#{}.done()\n", client.endpoint.id, id);
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
            2,
        ]);
        Ok(())
    }

    /// Since when the released message is available.
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
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_drm_lease_device_v1#{}.released()\n", client.endpoint.id, id);
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
            3,
        ]);
        drop(fmt);
        drop(outgoing_ref);
        drop(client_ref);
        self.core.handle_client_destroy();
        Ok(())
    }
}

/// A message handler for [WpDrmLeaseDeviceV1] proxies.
pub trait WpDrmLeaseDeviceV1Handler: Any {
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
        _slf: &Rc<WpDrmLeaseDeviceV1>,
        id: &Rc<WpDrmLeaseRequestV1>,
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
        _slf: &Rc<WpDrmLeaseDeviceV1>,
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
        _slf: &Rc<WpDrmLeaseDeviceV1>,
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
        _slf: &Rc<WpDrmLeaseDeviceV1>,
        id: &Rc<WpDrmLeaseConnectorV1>,
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
        _slf: &Rc<WpDrmLeaseDeviceV1>,
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
        _slf: &Rc<WpDrmLeaseDeviceV1>,
    ) {
        let res = _slf.send_released(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_drm_lease_device_v1.released message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for WpDrmLeaseDeviceV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpDrmLeaseDeviceV1, version),
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_drm_lease_device_v1#{}.create_lease_request(id: wp_drm_lease_request_v1#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = WpDrmLeaseRequestV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).create_lease_request(&self, arg0);
                } else {
                    DefaultHandler.create_lease_request(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_drm_lease_device_v1#{}.release()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).release(&self);
                } else {
                    DefaultHandler.release(&self);
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
            0 => {
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_drm_lease_device_v1#{}.drm_fd(fd: {})\n", msg[0], arg0.as_raw_fd());
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).drm_fd(&self, arg0);
                } else {
                    DefaultHandler.drm_fd(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_drm_lease_device_v1#{}.connector(id: wp_drm_lease_connector_v1#{})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = WpDrmLeaseConnectorV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetServerId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).connector(&self, arg0);
                } else {
                    DefaultHandler.connector(&self, arg0);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_drm_lease_device_v1#{}.done()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).done(&self);
                } else {
                    DefaultHandler.done(&self);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_drm_lease_device_v1#{}.released()\n", msg[0]);
                    self.core.state.log(args);
                }
                self.core.handle_server_destroy();
                if let Some(handler) = handler {
                    (**handler).released(&self);
                } else {
                    DefaultHandler.released(&self);
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
            0 => "create_lease_request",
            1 => "release",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "drm_fd",
            1 => "connector",
            2 => "done",
            3 => "released",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WpDrmLeaseDeviceV1 {
    fn core(&self) -> &ObjectCore {
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

