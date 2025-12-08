//! DRM lease request
//!
//! A client that wishes to lease DRM resources will attach the list of
//! connectors advertised with wp_drm_lease_device_v1.connector that they
//! wish to lease, then use wp_drm_lease_request_v1.submit to submit the
//! request.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_drm_lease_request_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpDrmLeaseRequestV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpDrmLeaseRequestV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpDrmLeaseRequestV1MessageHandler for DefaultMessageHandler { }

impl MetaWpDrmLeaseRequestV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWpDrmLeaseRequestV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WpDrmLeaseRequestV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpDrmLeaseRequestV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpDrmLeaseRequestV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpDrmLeaseRequestV1 {
    /// Since when the request_connector message is available.
    #[allow(dead_code)]
    pub const MSG__REQUEST_CONNECTOR__SINCE: u32 = 1;

    /// request a connector for this lease
    ///
    /// Indicates that the client would like to lease the given connector.
    /// This is only used as a suggestion, the compositor may choose to
    /// include any resources in the lease it issues, or change the set of
    /// leased resources at any time. Compositors are however encouraged to
    /// include the requested connector and other resources necessary
    /// to drive the connected output in the lease.
    ///
    /// Requesting a connector that was created from a different lease device
    /// than this lease request raises the wrong_device error. Requesting a
    /// connector twice will raise the duplicate_connector error.
    ///
    /// # Arguments
    ///
    /// - `connector`:
    #[inline]
    pub fn send_request_connector(
        &self,
        connector: &Rc<MetaWpDrmLeaseConnectorV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            connector,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("connector")),
            Some(id) => id,
        };
        eprintln!("server      <= wp_drm_lease_request_v1#{}.request_connector(connector: wp_drm_lease_connector_v1#{})", id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the submit message is available.
    #[allow(dead_code)]
    pub const MSG__SUBMIT__SINCE: u32 = 1;

    /// submit the lease request
    ///
    /// Submits the lease request and creates a new wp_drm_lease_v1 object.
    /// After calling submit the compositor will immediately destroy this
    /// object, issuing any more requests will cause a wl_display error.
    /// The compositor doesn't make any guarantees about the events of the
    /// lease object, clients cannot expect an immediate response.
    /// Not requesting any connectors before submitting the lease request
    /// will raise the empty_lease error.
    #[inline]
    pub fn send_submit(
        &self,
        id: &Rc<MetaWpDrmLeaseV1>,
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
        eprintln!("server      <= wp_drm_lease_request_v1#{}.submit(id: wp_drm_lease_v1#{})", id, arg0_id);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0_id,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [WpDrmLeaseRequestV1] proxies.
#[allow(dead_code)]
pub trait MetaWpDrmLeaseRequestV1MessageHandler {
    /// request a connector for this lease
    ///
    /// Indicates that the client would like to lease the given connector.
    /// This is only used as a suggestion, the compositor may choose to
    /// include any resources in the lease it issues, or change the set of
    /// leased resources at any time. Compositors are however encouraged to
    /// include the requested connector and other resources necessary
    /// to drive the connected output in the lease.
    ///
    /// Requesting a connector that was created from a different lease device
    /// than this lease request raises the wrong_device error. Requesting a
    /// connector twice will raise the duplicate_connector error.
    ///
    /// # Arguments
    ///
    /// - `connector`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn request_connector(
        &mut self,
        _slf: &Rc<MetaWpDrmLeaseRequestV1>,
        connector: &Rc<MetaWpDrmLeaseConnectorV1>,
    ) {
        let res = _slf.send_request_connector(
            connector,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_drm_lease_request_v1.request_connector message: {}", Report::new(e));
        }
    }

    /// submit the lease request
    ///
    /// Submits the lease request and creates a new wp_drm_lease_v1 object.
    /// After calling submit the compositor will immediately destroy this
    /// object, issuing any more requests will cause a wl_display error.
    /// The compositor doesn't make any guarantees about the events of the
    /// lease object, clients cannot expect an immediate response.
    /// Not requesting any connectors before submitting the lease request
    /// will raise the empty_lease error.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn submit(
        &mut self,
        _slf: &Rc<MetaWpDrmLeaseRequestV1>,
        id: &Rc<MetaWpDrmLeaseV1>,
    ) {
        let res = _slf.send_submit(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_drm_lease_request_v1.submit message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpDrmLeaseRequestV1 {
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
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                eprintln!("client#{:04} -> wp_drm_lease_request_v1#{}.request_connector(connector: wp_drm_lease_connector_v1#{})", client.endpoint.id, msg[0], arg0);
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWpDrmLeaseConnectorV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("connector", o.core().interface, ProxyInterface::WpDrmLeaseConnectorV1));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).request_connector(&self, arg0);
                } else {
                    DefaultMessageHandler.request_connector(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                eprintln!("client#{:04} -> wp_drm_lease_request_v1#{}.submit(id: wp_drm_lease_v1#{})", client.endpoint.id, msg[0], arg0);
                let arg0_id = arg0;
                let arg0 = MetaWpDrmLeaseV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).submit(&self, arg0);
                } else {
                    DefaultMessageHandler.submit(&self, arg0);
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
            0 => "request_connector",
            1 => "submit",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl MetaWpDrmLeaseRequestV1 {
    /// Since when the error.wrong_device enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_WRONG_DEVICE__SINCE: u32 = 1;
    /// Since when the error.duplicate_connector enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_DUPLICATE_CONNECTOR__SINCE: u32 = 1;
    /// Since when the error.empty_lease enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_EMPTY_LEASE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWpDrmLeaseRequestV1Error(pub u32);

impl MetaWpDrmLeaseRequestV1Error {
    /// requested a connector from a different lease device
    #[allow(dead_code)]
    pub const WRONG_DEVICE: Self = Self(0);

    /// requested a connector twice
    #[allow(dead_code)]
    pub const DUPLICATE_CONNECTOR: Self = Self(1);

    /// requested a lease without requesting a connector
    #[allow(dead_code)]
    pub const EMPTY_LEASE: Self = Self(2);
}

impl Debug for MetaWpDrmLeaseRequestV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::WRONG_DEVICE => "WRONG_DEVICE",
            Self::DUPLICATE_CONNECTOR => "DUPLICATE_CONNECTOR",
            Self::EMPTY_LEASE => "EMPTY_LEASE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
