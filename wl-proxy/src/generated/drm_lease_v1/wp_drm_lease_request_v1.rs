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
pub struct WpDrmLeaseRequestV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn WpDrmLeaseRequestV1Handler>,
}

struct DefaultHandler;

impl WpDrmLeaseRequestV1Handler for DefaultHandler { }

impl WpDrmLeaseRequestV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "wp_drm_lease_request_v1";
}

impl WpDrmLeaseRequestV1 {
    pub fn set_handler(&self, handler: impl WpDrmLeaseRequestV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn WpDrmLeaseRequestV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpDrmLeaseRequestV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpDrmLeaseRequestV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpDrmLeaseRequestV1 {
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
        connector: &Rc<WpDrmLeaseConnectorV1>,
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_drm_lease_request_v1#{}.request_connector(connector: wp_drm_lease_connector_v1#{})\n", id, arg0_id);
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
        id: &Rc<WpDrmLeaseV1>,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_drm_lease_request_v1#{}.submit(id: wp_drm_lease_v1#{})\n", id, arg0_id);
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
            arg0_id,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [WpDrmLeaseRequestV1] proxies.
#[allow(dead_code)]
pub trait WpDrmLeaseRequestV1Handler: Any {
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
        _slf: &Rc<WpDrmLeaseRequestV1>,
        connector: &Rc<WpDrmLeaseConnectorV1>,
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
        _slf: &Rc<WpDrmLeaseRequestV1>,
        id: &Rc<WpDrmLeaseV1>,
    ) {
        let res = _slf.send_submit(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_drm_lease_request_v1.submit message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for WpDrmLeaseRequestV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::WpDrmLeaseRequestV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_drm_lease_request_v1#{}.request_connector(connector: wp_drm_lease_connector_v1#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WpDrmLeaseConnectorV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("connector", o.core().interface, ProxyInterface::WpDrmLeaseConnectorV1));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).request_connector(&self, arg0);
                } else {
                    DefaultHandler.request_connector(&self, arg0);
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_drm_lease_request_v1#{}.submit(id: wp_drm_lease_v1#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = WpDrmLeaseV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).submit(&self, arg0);
                } else {
                    DefaultHandler.submit(&self, arg0);
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

impl Proxy for WpDrmLeaseRequestV1 {
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

impl WpDrmLeaseRequestV1 {
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
pub struct WpDrmLeaseRequestV1Error(pub u32);

impl WpDrmLeaseRequestV1Error {
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

impl Debug for WpDrmLeaseRequestV1Error {
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
