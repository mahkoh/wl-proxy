//! a leasable DRM connector
//!
//! Represents a DRM connector which is available for lease. These objects are
//! created via wp_drm_lease_device_v1.connector events, and should be passed
//! to lease requests via wp_drm_lease_request_v1.request_connector.
//! Immediately after the wp_drm_lease_connector_v1 object is created the
//! compositor will send a name, a description, a connector_id and a done
//! event. When the description is updated the compositor will send a
//! description event followed by a done event.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_drm_lease_connector_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpDrmLeaseConnectorV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpDrmLeaseConnectorV1Handler>,
}

struct DefaultHandler;

impl WpDrmLeaseConnectorV1Handler for DefaultHandler { }

impl ConcreteObject for WpDrmLeaseConnectorV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WpDrmLeaseConnectorV1;
    const INTERFACE_NAME: &str = "wp_drm_lease_connector_v1";
}

impl WpDrmLeaseConnectorV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WpDrmLeaseConnectorV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WpDrmLeaseConnectorV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpDrmLeaseConnectorV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpDrmLeaseConnectorV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpDrmLeaseConnectorV1 {
    /// Since when the name message is available.
    pub const MSG__NAME__SINCE: u32 = 1;

    /// name
    ///
    /// The compositor sends this event once the connector is created to
    /// indicate the name of this connector. This will not change for the
    /// duration of the Wayland session, but is not guaranteed to be consistent
    /// between sessions.
    ///
    /// If the compositor supports wl_output version 4 and this connector
    /// corresponds to a wl_output, the compositor should use the same name as
    /// for the wl_output.
    ///
    /// # Arguments
    ///
    /// - `name`: connector name
    #[inline]
    pub fn try_send_name(
        &self,
        name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            name,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_drm_lease_connector_v1#{}.name(name: {:?})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
            0,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// name
    ///
    /// The compositor sends this event once the connector is created to
    /// indicate the name of this connector. This will not change for the
    /// duration of the Wayland session, but is not guaranteed to be consistent
    /// between sessions.
    ///
    /// If the compositor supports wl_output version 4 and this connector
    /// corresponds to a wl_output, the compositor should use the same name as
    /// for the wl_output.
    ///
    /// # Arguments
    ///
    /// - `name`: connector name
    #[inline]
    pub fn send_name(
        &self,
        name: &str,
    ) {
        let res = self.try_send_name(
            name,
        );
        if let Err(e) = res {
            log_send("wp_drm_lease_connector_v1.name", &e);
        }
    }

    /// Since when the description message is available.
    pub const MSG__DESCRIPTION__SINCE: u32 = 1;

    /// description
    ///
    /// The compositor sends this event once the connector is created to provide
    /// a human-readable description for this connector, which may be presented
    /// to the user. The compositor may send this event multiple times over the
    /// lifetime of this object to reflect changes in the description.
    ///
    /// # Arguments
    ///
    /// - `description`: connector description
    #[inline]
    pub fn try_send_description(
        &self,
        description: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            description,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_drm_lease_connector_v1#{}.description(description: {:?})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
        fmt.string(arg0);
        Ok(())
    }

    /// description
    ///
    /// The compositor sends this event once the connector is created to provide
    /// a human-readable description for this connector, which may be presented
    /// to the user. The compositor may send this event multiple times over the
    /// lifetime of this object to reflect changes in the description.
    ///
    /// # Arguments
    ///
    /// - `description`: connector description
    #[inline]
    pub fn send_description(
        &self,
        description: &str,
    ) {
        let res = self.try_send_description(
            description,
        );
        if let Err(e) = res {
            log_send("wp_drm_lease_connector_v1.description", &e);
        }
    }

    /// Since when the connector_id message is available.
    pub const MSG__CONNECTOR_ID__SINCE: u32 = 1;

    /// connector_id
    ///
    /// The compositor sends this event once the connector is created to
    /// indicate the DRM object ID which represents the underlying connector
    /// that is being offered. Note that the final lease may include additional
    /// object IDs, such as CRTCs and planes.
    ///
    /// # Arguments
    ///
    /// - `connector_id`: DRM connector ID
    #[inline]
    pub fn try_send_connector_id(
        &self,
        connector_id: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            connector_id,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_drm_lease_connector_v1#{}.connector_id(connector_id: {})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
            arg0,
        ]);
        Ok(())
    }

    /// connector_id
    ///
    /// The compositor sends this event once the connector is created to
    /// indicate the DRM object ID which represents the underlying connector
    /// that is being offered. Note that the final lease may include additional
    /// object IDs, such as CRTCs and planes.
    ///
    /// # Arguments
    ///
    /// - `connector_id`: DRM connector ID
    #[inline]
    pub fn send_connector_id(
        &self,
        connector_id: u32,
    ) {
        let res = self.try_send_connector_id(
            connector_id,
        );
        if let Err(e) = res {
            log_send("wp_drm_lease_connector_v1.connector_id", &e);
        }
    }

    /// Since when the done message is available.
    pub const MSG__DONE__SINCE: u32 = 1;

    /// all properties have been sent
    ///
    /// This event is sent after all properties of a connector have been sent.
    /// This allows changes to the properties to be seen as atomic even if they
    /// happen via multiple events.
    #[inline]
    pub fn try_send_done(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_drm_lease_connector_v1#{}.done()\n", client_id, id);
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
            3,
        ]);
        Ok(())
    }

    /// all properties have been sent
    ///
    /// This event is sent after all properties of a connector have been sent.
    /// This allows changes to the properties to be seen as atomic even if they
    /// happen via multiple events.
    #[inline]
    pub fn send_done(
        &self,
    ) {
        let res = self.try_send_done(
        );
        if let Err(e) = res {
            log_send("wp_drm_lease_connector_v1.done", &e);
        }
    }

    /// Since when the withdrawn message is available.
    pub const MSG__WITHDRAWN__SINCE: u32 = 1;

    /// lease offer withdrawn
    ///
    /// Sent to indicate that the compositor will no longer honor requests for
    /// DRM leases which include this connector. The client may still issue a
    /// lease request including this connector, but the compositor will send
    /// wp_drm_lease_v1.finished without issuing a lease fd. Compositors are
    /// encouraged to send this event when they lose access to connector, for
    /// example when the connector is hot-unplugged, when the connector gets
    /// leased to a client or when the compositor loses DRM master.
    ///
    /// If a client holds a lease for the connector, the status of the lease
    /// remains the same. The client should destroy the object after receiving
    /// this event.
    #[inline]
    pub fn try_send_withdrawn(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_drm_lease_connector_v1#{}.withdrawn()\n", client_id, id);
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
            4,
        ]);
        Ok(())
    }

    /// lease offer withdrawn
    ///
    /// Sent to indicate that the compositor will no longer honor requests for
    /// DRM leases which include this connector. The client may still issue a
    /// lease request including this connector, but the compositor will send
    /// wp_drm_lease_v1.finished without issuing a lease fd. Compositors are
    /// encouraged to send this event when they lose access to connector, for
    /// example when the connector is hot-unplugged, when the connector gets
    /// leased to a client or when the compositor loses DRM master.
    ///
    /// If a client holds a lease for the connector, the status of the lease
    /// remains the same. The client should destroy the object after receiving
    /// this event.
    #[inline]
    pub fn send_withdrawn(
        &self,
    ) {
        let res = self.try_send_withdrawn(
        );
        if let Err(e) = res {
            log_send("wp_drm_lease_connector_v1.withdrawn", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy connector
    ///
    /// The client may send this request to indicate that it will not use this
    /// connector. Clients are encouraged to send this after receiving the
    /// "withdrawn" event so that the server can release the resources
    /// associated with this connector offer. Neither existing lease requests
    /// nor leases will be affected.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_drm_lease_connector_v1#{}.destroy()\n", id);
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

    /// destroy connector
    ///
    /// The client may send this request to indicate that it will not use this
    /// connector. Clients are encouraged to send this after receiving the
    /// "withdrawn" event so that the server can release the resources
    /// associated with this connector offer. Neither existing lease requests
    /// nor leases will be affected.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wp_drm_lease_connector_v1.destroy", &e);
        }
    }
}

/// A message handler for [`WpDrmLeaseConnectorV1`] proxies.
pub trait WpDrmLeaseConnectorV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpDrmLeaseConnectorV1>) {
        slf.core.delete_id();
    }

    /// name
    ///
    /// The compositor sends this event once the connector is created to
    /// indicate the name of this connector. This will not change for the
    /// duration of the Wayland session, but is not guaranteed to be consistent
    /// between sessions.
    ///
    /// If the compositor supports wl_output version 4 and this connector
    /// corresponds to a wl_output, the compositor should use the same name as
    /// for the wl_output.
    ///
    /// # Arguments
    ///
    /// - `name`: connector name
    #[inline]
    fn handle_name(
        &mut self,
        slf: &Rc<WpDrmLeaseConnectorV1>,
        name: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_name(
            name,
        );
        if let Err(e) = res {
            log_forward("wp_drm_lease_connector_v1.name", &e);
        }
    }

    /// description
    ///
    /// The compositor sends this event once the connector is created to provide
    /// a human-readable description for this connector, which may be presented
    /// to the user. The compositor may send this event multiple times over the
    /// lifetime of this object to reflect changes in the description.
    ///
    /// # Arguments
    ///
    /// - `description`: connector description
    #[inline]
    fn handle_description(
        &mut self,
        slf: &Rc<WpDrmLeaseConnectorV1>,
        description: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_description(
            description,
        );
        if let Err(e) = res {
            log_forward("wp_drm_lease_connector_v1.description", &e);
        }
    }

    /// connector_id
    ///
    /// The compositor sends this event once the connector is created to
    /// indicate the DRM object ID which represents the underlying connector
    /// that is being offered. Note that the final lease may include additional
    /// object IDs, such as CRTCs and planes.
    ///
    /// # Arguments
    ///
    /// - `connector_id`: DRM connector ID
    #[inline]
    fn handle_connector_id(
        &mut self,
        slf: &Rc<WpDrmLeaseConnectorV1>,
        connector_id: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_connector_id(
            connector_id,
        );
        if let Err(e) = res {
            log_forward("wp_drm_lease_connector_v1.connector_id", &e);
        }
    }

    /// all properties have been sent
    ///
    /// This event is sent after all properties of a connector have been sent.
    /// This allows changes to the properties to be seen as atomic even if they
    /// happen via multiple events.
    #[inline]
    fn handle_done(
        &mut self,
        slf: &Rc<WpDrmLeaseConnectorV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_done(
        );
        if let Err(e) = res {
            log_forward("wp_drm_lease_connector_v1.done", &e);
        }
    }

    /// lease offer withdrawn
    ///
    /// Sent to indicate that the compositor will no longer honor requests for
    /// DRM leases which include this connector. The client may still issue a
    /// lease request including this connector, but the compositor will send
    /// wp_drm_lease_v1.finished without issuing a lease fd. Compositors are
    /// encouraged to send this event when they lose access to connector, for
    /// example when the connector is hot-unplugged, when the connector gets
    /// leased to a client or when the compositor loses DRM master.
    ///
    /// If a client holds a lease for the connector, the status of the lease
    /// remains the same. The client should destroy the object after receiving
    /// this event.
    #[inline]
    fn handle_withdrawn(
        &mut self,
        slf: &Rc<WpDrmLeaseConnectorV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_withdrawn(
        );
        if let Err(e) = res {
            log_forward("wp_drm_lease_connector_v1.withdrawn", &e);
        }
    }

    /// destroy connector
    ///
    /// The client may send this request to indicate that it will not use this
    /// connector. Clients are encouraged to send this after receiving the
    /// "withdrawn" event so that the server can release the resources
    /// associated with this connector offer. Neither existing lease requests
    /// nor leases will be affected.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WpDrmLeaseConnectorV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wp_drm_lease_connector_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for WpDrmLeaseConnectorV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpDrmLeaseConnectorV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_drm_lease_connector_v1#{}.destroy()\n", client_id, id);
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
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "name")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_drm_lease_connector_v1#{}.name(name: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_name(&self, arg0);
                } else {
                    DefaultHandler.handle_name(&self, arg0);
                }
            }
            1 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "description")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_drm_lease_connector_v1#{}.description(description: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_description(&self, arg0);
                } else {
                    DefaultHandler.handle_description(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_drm_lease_connector_v1#{}.connector_id(connector_id: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_connector_id(&self, arg0);
                } else {
                    DefaultHandler.handle_connector_id(&self, arg0);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_drm_lease_connector_v1#{}.done()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_done(&self);
                } else {
                    DefaultHandler.handle_done(&self);
                }
            }
            4 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_drm_lease_connector_v1#{}.withdrawn()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_withdrawn(&self);
                } else {
                    DefaultHandler.handle_withdrawn(&self);
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
            0 => "name",
            1 => "description",
            2 => "connector_id",
            3 => "done",
            4 => "withdrawn",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WpDrmLeaseConnectorV1 {
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

