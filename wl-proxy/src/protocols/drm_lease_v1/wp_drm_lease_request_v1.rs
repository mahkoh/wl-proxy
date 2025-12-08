//! DRM lease request
//!
//! A client that wishes to lease DRM resources will attach the list of
//! connectors advertised with wp_drm_lease_device_v1.connector that they
//! wish to lease, then use wp_drm_lease_request_v1.submit to submit the
//! request.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_drm_lease_request_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpDrmLeaseRequestV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpDrmLeaseRequestV1Handler>,
}

struct DefaultHandler;

impl WpDrmLeaseRequestV1Handler for DefaultHandler { }

impl ConcreteObject for WpDrmLeaseRequestV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WpDrmLeaseRequestV1;
    const INTERFACE_NAME: &str = "wp_drm_lease_request_v1";
}

impl WpDrmLeaseRequestV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WpDrmLeaseRequestV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
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
    pub fn try_send_request_connector(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("connector"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_drm_lease_request_v1#{}.request_connector(connector: wp_drm_lease_connector_v1#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

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
    ) {
        let res = self.try_send_request_connector(
            connector,
        );
        if let Err(e) = res {
            log_send("wp_drm_lease_request_v1.request_connector", &e);
        }
    }

    /// Since when the submit message is available.
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
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_submit(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_drm_lease_request_v1#{}.submit(id: wp_drm_lease_v1#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
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
            1,
            arg0_id,
        ]);
        self.core.handle_server_destroy();
        Ok(())
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
    pub fn send_submit(
        &self,
        id: &Rc<WpDrmLeaseV1>,
    ) {
        let res = self.try_send_submit(
            id,
        );
        if let Err(e) = res {
            log_send("wp_drm_lease_request_v1.submit", &e);
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
    #[inline]
    pub fn new_try_send_submit(
        &self,
    ) -> Result<Rc<WpDrmLeaseV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_submit(
            &id,
        )?;
        Ok(id)
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
    #[inline]
    pub fn new_send_submit(
        &self,
    ) -> Rc<WpDrmLeaseV1> {
        let id = self.core.create_child();
        self.send_submit(
            &id,
        );
        id
    }
}

/// A message handler for [`WpDrmLeaseRequestV1`] proxies.
pub trait WpDrmLeaseRequestV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpDrmLeaseRequestV1>) {
        slf.core.delete_id();
    }

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
    fn handle_request_connector(
        &mut self,
        slf: &Rc<WpDrmLeaseRequestV1>,
        connector: &Rc<WpDrmLeaseConnectorV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_request_connector(
            connector,
        );
        if let Err(e) = res {
            log_forward("wp_drm_lease_request_v1.request_connector", &e);
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
    fn handle_submit(
        &mut self,
        slf: &Rc<WpDrmLeaseRequestV1>,
        id: &Rc<WpDrmLeaseV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_submit(
            id,
        );
        if let Err(e) = res {
            log_forward("wp_drm_lease_request_v1.submit", &e);
        }
    }
}

impl ObjectPrivate for WpDrmLeaseRequestV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpDrmLeaseRequestV1, version),
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_drm_lease_request_v1#{}.request_connector(connector: wp_drm_lease_connector_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WpDrmLeaseConnectorV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("connector", o.core().interface, ObjectInterface::WpDrmLeaseConnectorV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_request_connector(&self, arg0);
                } else {
                    DefaultHandler.handle_request_connector(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_drm_lease_request_v1#{}.submit(id: wp_drm_lease_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WpDrmLeaseV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_submit(&self, arg0);
                } else {
                    DefaultHandler.handle_submit(&self, arg0);
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
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
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

impl Object for WpDrmLeaseRequestV1 {
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

impl WpDrmLeaseRequestV1 {
    /// Since when the error.wrong_device enum variant is available.
    pub const ENM__ERROR_WRONG_DEVICE__SINCE: u32 = 1;
    /// Since when the error.duplicate_connector enum variant is available.
    pub const ENM__ERROR_DUPLICATE_CONNECTOR__SINCE: u32 = 1;
    /// Since when the error.empty_lease enum variant is available.
    pub const ENM__ERROR_EMPTY_LEASE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpDrmLeaseRequestV1Error(pub u32);

impl WpDrmLeaseRequestV1Error {
    /// requested a connector from a different lease device
    pub const WRONG_DEVICE: Self = Self(0);

    /// requested a connector twice
    pub const DUPLICATE_CONNECTOR: Self = Self(1);

    /// requested a lease without requesting a connector
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
