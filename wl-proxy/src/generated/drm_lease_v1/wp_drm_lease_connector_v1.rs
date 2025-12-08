//! a leasable DRM connector
//!
//! Represents a DRM connector which is available for lease. These objects are
//! created via wp_drm_lease_device_v1.connector events, and should be passed
//! to lease requests via wp_drm_lease_request_v1.request_connector.
//! Immediately after the wp_drm_lease_connector_v1 object is created the
//! compositor will send a name, a description, a connector_id and a done
//! event. When the description is updated the compositor will send a
//! description event followed by a done event.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_drm_lease_connector_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpDrmLeaseConnectorV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpDrmLeaseConnectorV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpDrmLeaseConnectorV1MessageHandler for DefaultMessageHandler { }

impl MetaWpDrmLeaseConnectorV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWpDrmLeaseConnectorV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WpDrmLeaseConnectorV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpDrmLeaseConnectorV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpDrmLeaseConnectorV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpDrmLeaseConnectorV1 {
    /// Since when the name message is available.
    #[allow(dead_code)]
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
    pub fn send_name(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= wp_drm_lease_connector_v1#{}.name(name: {:?})", client.endpoint.id, id, arg0);
        let endpoint = &client.endpoint;
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
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the description message is available.
    #[allow(dead_code)]
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
    pub fn send_description(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= wp_drm_lease_connector_v1#{}.description(description: {:?})", client.endpoint.id, id, arg0);
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
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the connector_id message is available.
    #[allow(dead_code)]
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
    pub fn send_connector_id(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= wp_drm_lease_connector_v1#{}.connector_id(connector_id: {})", client.endpoint.id, id, arg0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the done message is available.
    #[allow(dead_code)]
    pub const MSG__DONE__SINCE: u32 = 1;

    /// all properties have been sent
    ///
    /// This event is sent after all properties of a connector have been sent.
    /// This allows changes to the properties to be seen as atomic even if they
    /// happen via multiple events.
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
        eprintln!("client#{:04} <= wp_drm_lease_connector_v1#{}.done()", client.endpoint.id, id);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the withdrawn message is available.
    #[allow(dead_code)]
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
    pub fn send_withdrawn(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= wp_drm_lease_connector_v1#{}.withdrawn()", client.endpoint.id, id);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

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
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= wp_drm_lease_connector_v1#{}.destroy()", id);
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

/// A message handler for [WpDrmLeaseConnectorV1] proxies.
#[allow(dead_code)]
pub trait MetaWpDrmLeaseConnectorV1MessageHandler {
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
    fn name(
        &mut self,
        _slf: &Rc<MetaWpDrmLeaseConnectorV1>,
        name: &str,
    ) {
        let res = _slf.send_name(
            name,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_drm_lease_connector_v1.name message: {}", Report::new(e));
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
    fn description(
        &mut self,
        _slf: &Rc<MetaWpDrmLeaseConnectorV1>,
        description: &str,
    ) {
        let res = _slf.send_description(
            description,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_drm_lease_connector_v1.description message: {}", Report::new(e));
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
    fn connector_id(
        &mut self,
        _slf: &Rc<MetaWpDrmLeaseConnectorV1>,
        connector_id: u32,
    ) {
        let res = _slf.send_connector_id(
            connector_id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_drm_lease_connector_v1.connector_id message: {}", Report::new(e));
        }
    }

    /// all properties have been sent
    ///
    /// This event is sent after all properties of a connector have been sent.
    /// This allows changes to the properties to be seen as atomic even if they
    /// happen via multiple events.
    #[inline]
    fn done(
        &mut self,
        _slf: &Rc<MetaWpDrmLeaseConnectorV1>,
    ) {
        let res = _slf.send_done(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_drm_lease_connector_v1.done message: {}", Report::new(e));
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
    fn withdrawn(
        &mut self,
        _slf: &Rc<MetaWpDrmLeaseConnectorV1>,
    ) {
        let res = _slf.send_withdrawn(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_drm_lease_connector_v1.withdrawn message: {}", Report::new(e));
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
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpDrmLeaseConnectorV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_drm_lease_connector_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpDrmLeaseConnectorV1 {
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
                eprintln!("client#{:04} -> wp_drm_lease_connector_v1#{}.destroy()", client.endpoint.id, msg[0]);
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
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("name"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("name"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("name"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("name"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                eprintln!("server      -> wp_drm_lease_connector_v1#{}.name(name: {:?})", msg[0], arg0);
                if let Some(handler) = handler {
                    (**handler).name(&self, arg0);
                } else {
                    DefaultMessageHandler.name(&self, arg0);
                }
            }
            1 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("description"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("description"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("description"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("description"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                eprintln!("server      -> wp_drm_lease_connector_v1#{}.description(description: {:?})", msg[0], arg0);
                if let Some(handler) = handler {
                    (**handler).description(&self, arg0);
                } else {
                    DefaultMessageHandler.description(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                eprintln!("server      -> wp_drm_lease_connector_v1#{}.connector_id(connector_id: {})", msg[0], arg0);
                if let Some(handler) = handler {
                    (**handler).connector_id(&self, arg0);
                } else {
                    DefaultMessageHandler.connector_id(&self, arg0);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("server      -> wp_drm_lease_connector_v1#{}.done()", msg[0]);
                if let Some(handler) = handler {
                    (**handler).done(&self);
                } else {
                    DefaultMessageHandler.done(&self);
                }
            }
            4 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("server      -> wp_drm_lease_connector_v1#{}.withdrawn()", msg[0]);
                if let Some(handler) = handler {
                    (**handler).withdrawn(&self);
                } else {
                    DefaultMessageHandler.withdrawn(&self);
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

