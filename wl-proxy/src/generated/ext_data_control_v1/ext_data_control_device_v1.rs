//! manage a data device for a seat
//!
//! This interface allows a client to manage a seat's selection.
//!
//! When the seat is destroyed, this object becomes inert.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_data_control_device_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaExtDataControlDeviceV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaExtDataControlDeviceV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaExtDataControlDeviceV1MessageHandler for DefaultMessageHandler { }

impl MetaExtDataControlDeviceV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaExtDataControlDeviceV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ExtDataControlDeviceV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaExtDataControlDeviceV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaExtDataControlDeviceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaExtDataControlDeviceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaExtDataControlDeviceV1 {
    /// Since when the set_selection message is available.
    #[allow(dead_code)]
    pub const MSG__SET_SELECTION__SINCE: u32 = 1;

    /// copy data to the selection
    ///
    /// This request asks the compositor to set the selection to the data from
    /// the source on behalf of the client.
    ///
    /// The given source may not be used in any further set_selection or
    /// set_primary_selection requests. Attempting to use a previously used
    /// source triggers the used_source protocol error.
    ///
    /// To unset the selection, set the source to NULL.
    ///
    /// # Arguments
    ///
    /// - `source`:
    #[inline]
    pub fn send_set_selection(
        &self,
        source: Option<&Rc<MetaExtDataControlSourceV1>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            source,
        );
        let arg0 = arg0.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError::ArgNoServerId("source")),
                Some(id) => id,
            },
        };
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

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy this data device
    ///
    /// Destroys the data device object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
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
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the data_offer message is available.
    #[allow(dead_code)]
    pub const MSG__DATA_OFFER__SINCE: u32 = 1;

    /// introduce a new ext_data_control_offer
    ///
    /// The data_offer event introduces a new ext_data_control_offer object,
    /// which will subsequently be used in either the
    /// ext_data_control_device.selection event (for the regular clipboard
    /// selections) or the ext_data_control_device.primary_selection event (for
    /// the primary clipboard selections). Immediately following the
    /// ext_data_control_device.data_offer event, the new data_offer object
    /// will send out ext_data_control_offer.offer events to describe the MIME
    /// types it offers.
    #[inline]
    pub fn send_data_offer(
        &self,
        id: &Rc<MetaExtDataControlOfferV1>,
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
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the selection message is available.
    #[allow(dead_code)]
    pub const MSG__SELECTION__SINCE: u32 = 1;

    /// advertise new selection
    ///
    /// The selection event is sent out to notify the client of a new
    /// ext_data_control_offer for the selection for this device. The
    /// ext_data_control_device.data_offer and the ext_data_control_offer.offer
    /// events are sent out immediately before this event to introduce the data
    /// offer object. The selection event is sent to a client when a new
    /// selection is set. The ext_data_control_offer is valid until a new
    /// ext_data_control_offer or NULL is received. The client must destroy the
    /// previous selection ext_data_control_offer, if any, upon receiving this
    /// event. Regardless, the previous selection will be ignored once a new
    /// selection ext_data_control_offer is received.
    ///
    /// The first selection event is sent upon binding the
    /// ext_data_control_device object.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_selection(
        &self,
        id: Option<&Rc<MetaExtDataControlOfferV1>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0 = arg0.map(|a| a.core());
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if let Some(arg0) = arg0 {
            if arg0.client_id.get() != Some(client.endpoint.id) {
                return Err(ObjectError::ArgNoClientId("id", client.endpoint.id));
            }
        }
        let arg0_id = arg0.map(|arg0| arg0.client_obj_id.get()).flatten().unwrap_or(0);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the finished message is available.
    #[allow(dead_code)]
    pub const MSG__FINISHED__SINCE: u32 = 1;

    /// this data control is no longer valid
    ///
    /// This data control object is no longer valid and should be destroyed by
    /// the client.
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
        ]);
        Ok(())
    }

    /// Since when the primary_selection message is available.
    #[allow(dead_code)]
    pub const MSG__PRIMARY_SELECTION__SINCE: u32 = 1;

    /// advertise new primary selection
    ///
    /// The primary_selection event is sent out to notify the client of a new
    /// ext_data_control_offer for the primary selection for this device. The
    /// ext_data_control_device.data_offer and the ext_data_control_offer.offer
    /// events are sent out immediately before this event to introduce the data
    /// offer object. The primary_selection event is sent to a client when a
    /// new primary selection is set. The ext_data_control_offer is valid until
    /// a new ext_data_control_offer or NULL is received. The client must
    /// destroy the previous primary selection ext_data_control_offer, if any,
    /// upon receiving this event. Regardless, the previous primary selection
    /// will be ignored once a new primary selection ext_data_control_offer is
    /// received.
    ///
    /// If the compositor supports primary selection, the first
    /// primary_selection event is sent upon binding the
    /// ext_data_control_device object.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_primary_selection(
        &self,
        id: Option<&Rc<MetaExtDataControlOfferV1>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0 = arg0.map(|a| a.core());
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if let Some(arg0) = arg0 {
            if arg0.client_id.get() != Some(client.endpoint.id) {
                return Err(ObjectError::ArgNoClientId("id", client.endpoint.id));
            }
        }
        let arg0_id = arg0.map(|arg0| arg0.client_obj_id.get()).flatten().unwrap_or(0);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the set_primary_selection message is available.
    #[allow(dead_code)]
    pub const MSG__SET_PRIMARY_SELECTION__SINCE: u32 = 1;

    /// copy data to the primary selection
    ///
    /// This request asks the compositor to set the primary selection to the
    /// data from the source on behalf of the client.
    ///
    /// The given source may not be used in any further set_selection or
    /// set_primary_selection requests. Attempting to use a previously used
    /// source triggers the used_source protocol error.
    ///
    /// To unset the primary selection, set the source to NULL.
    ///
    /// The compositor will ignore this request if it does not support primary
    /// selection.
    ///
    /// # Arguments
    ///
    /// - `source`:
    #[inline]
    pub fn send_set_primary_selection(
        &self,
        source: Option<&Rc<MetaExtDataControlSourceV1>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            source,
        );
        let arg0 = arg0.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError::ArgNoServerId("source")),
                Some(id) => id,
            },
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
            arg0_id,
        ]);
        Ok(())
    }
}

/// A message handler for [ExtDataControlDeviceV1] proxies.
#[allow(dead_code)]
pub trait MetaExtDataControlDeviceV1MessageHandler {
    /// copy data to the selection
    ///
    /// This request asks the compositor to set the selection to the data from
    /// the source on behalf of the client.
    ///
    /// The given source may not be used in any further set_selection or
    /// set_primary_selection requests. Attempting to use a previously used
    /// source triggers the used_source protocol error.
    ///
    /// To unset the selection, set the source to NULL.
    ///
    /// # Arguments
    ///
    /// - `source`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn set_selection(
        &mut self,
        _slf: &Rc<MetaExtDataControlDeviceV1>,
        source: Option<&Rc<MetaExtDataControlSourceV1>>,
    ) {
        let res = _slf.send_set_selection(
            source,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_data_control_device_v1.set_selection message: {}", Report::new(e));
        }
    }

    /// destroy this data device
    ///
    /// Destroys the data device object.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaExtDataControlDeviceV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_data_control_device_v1.destroy message: {}", Report::new(e));
        }
    }

    /// introduce a new ext_data_control_offer
    ///
    /// The data_offer event introduces a new ext_data_control_offer object,
    /// which will subsequently be used in either the
    /// ext_data_control_device.selection event (for the regular clipboard
    /// selections) or the ext_data_control_device.primary_selection event (for
    /// the primary clipboard selections). Immediately following the
    /// ext_data_control_device.data_offer event, the new data_offer object
    /// will send out ext_data_control_offer.offer events to describe the MIME
    /// types it offers.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn data_offer(
        &mut self,
        _slf: &Rc<MetaExtDataControlDeviceV1>,
        id: &Rc<MetaExtDataControlOfferV1>,
    ) {
        let res = _slf.send_data_offer(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_data_control_device_v1.data_offer message: {}", Report::new(e));
        }
    }

    /// advertise new selection
    ///
    /// The selection event is sent out to notify the client of a new
    /// ext_data_control_offer for the selection for this device. The
    /// ext_data_control_device.data_offer and the ext_data_control_offer.offer
    /// events are sent out immediately before this event to introduce the data
    /// offer object. The selection event is sent to a client when a new
    /// selection is set. The ext_data_control_offer is valid until a new
    /// ext_data_control_offer or NULL is received. The client must destroy the
    /// previous selection ext_data_control_offer, if any, upon receiving this
    /// event. Regardless, the previous selection will be ignored once a new
    /// selection ext_data_control_offer is received.
    ///
    /// The first selection event is sent upon binding the
    /// ext_data_control_device object.
    ///
    /// # Arguments
    ///
    /// - `id`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn selection(
        &mut self,
        _slf: &Rc<MetaExtDataControlDeviceV1>,
        id: Option<&Rc<MetaExtDataControlOfferV1>>,
    ) {
        if let Some(client_id) = _slf.core.client_id.get() {
            if let Some(id) = id {
                if let Some(client_id_2) = id.core().client_id.get() {
                    if client_id != client_id_2 {
                        return;
                    }
                }
            }
        }
        let res = _slf.send_selection(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_data_control_device_v1.selection message: {}", Report::new(e));
        }
    }

    /// this data control is no longer valid
    ///
    /// This data control object is no longer valid and should be destroyed by
    /// the client.
    #[inline]
    fn finished(
        &mut self,
        _slf: &Rc<MetaExtDataControlDeviceV1>,
    ) {
        let res = _slf.send_finished(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_data_control_device_v1.finished message: {}", Report::new(e));
        }
    }

    /// advertise new primary selection
    ///
    /// The primary_selection event is sent out to notify the client of a new
    /// ext_data_control_offer for the primary selection for this device. The
    /// ext_data_control_device.data_offer and the ext_data_control_offer.offer
    /// events are sent out immediately before this event to introduce the data
    /// offer object. The primary_selection event is sent to a client when a
    /// new primary selection is set. The ext_data_control_offer is valid until
    /// a new ext_data_control_offer or NULL is received. The client must
    /// destroy the previous primary selection ext_data_control_offer, if any,
    /// upon receiving this event. Regardless, the previous primary selection
    /// will be ignored once a new primary selection ext_data_control_offer is
    /// received.
    ///
    /// If the compositor supports primary selection, the first
    /// primary_selection event is sent upon binding the
    /// ext_data_control_device object.
    ///
    /// # Arguments
    ///
    /// - `id`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn primary_selection(
        &mut self,
        _slf: &Rc<MetaExtDataControlDeviceV1>,
        id: Option<&Rc<MetaExtDataControlOfferV1>>,
    ) {
        if let Some(client_id) = _slf.core.client_id.get() {
            if let Some(id) = id {
                if let Some(client_id_2) = id.core().client_id.get() {
                    if client_id != client_id_2 {
                        return;
                    }
                }
            }
        }
        let res = _slf.send_primary_selection(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_data_control_device_v1.primary_selection message: {}", Report::new(e));
        }
    }

    /// copy data to the primary selection
    ///
    /// This request asks the compositor to set the primary selection to the
    /// data from the source on behalf of the client.
    ///
    /// The given source may not be used in any further set_selection or
    /// set_primary_selection requests. Attempting to use a previously used
    /// source triggers the used_source protocol error.
    ///
    /// To unset the primary selection, set the source to NULL.
    ///
    /// The compositor will ignore this request if it does not support primary
    /// selection.
    ///
    /// # Arguments
    ///
    /// - `source`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn set_primary_selection(
        &mut self,
        _slf: &Rc<MetaExtDataControlDeviceV1>,
        source: Option<&Rc<MetaExtDataControlSourceV1>>,
    ) {
        let res = _slf.send_set_primary_selection(
            source,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_data_control_device_v1.set_primary_selection message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaExtDataControlDeviceV1 {
    fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Self::new(state, version)
    }

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
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaExtDataControlSourceV1>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError::WrongObjectType("source", o.core().interface, ProxyInterface::ExtDataControlSourceV1));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).set_selection(&self, arg0);
                } else {
                    DefaultMessageHandler.set_selection(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaExtDataControlSourceV1>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError::WrongObjectType("source", o.core().interface, ProxyInterface::ExtDataControlSourceV1));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).set_primary_selection(&self, arg0);
                } else {
                    DefaultMessageHandler.set_primary_selection(&self, arg0);
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
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0_id = arg0;
                let arg0 = MetaExtDataControlOfferV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetServerId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).data_offer(&self, arg0);
                } else {
                    DefaultMessageHandler.data_offer(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = self.core.state.server.lookup(arg0_id) else {
                        return Err(ObjectError::NoServerObject(arg0_id));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaExtDataControlOfferV1>() else {
                        let o = self.core.state.server.lookup(arg0_id).unwrap();
                        return Err(ObjectError::WrongObjectType("id", o.core().interface, ProxyInterface::ExtDataControlOfferV1));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).selection(&self, arg0);
                } else {
                    DefaultMessageHandler.selection(&self, arg0);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).finished(&self);
                } else {
                    DefaultMessageHandler.finished(&self);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = self.core.state.server.lookup(arg0_id) else {
                        return Err(ObjectError::NoServerObject(arg0_id));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaExtDataControlOfferV1>() else {
                        let o = self.core.state.server.lookup(arg0_id).unwrap();
                        return Err(ObjectError::WrongObjectType("id", o.core().interface, ProxyInterface::ExtDataControlOfferV1));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).primary_selection(&self, arg0);
                } else {
                    DefaultMessageHandler.primary_selection(&self, arg0);
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
            0 => "set_selection",
            1 => "destroy",
            2 => "set_primary_selection",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "data_offer",
            1 => "selection",
            2 => "finished",
            3 => "primary_selection",
            _ => return None,
        };
        Some(name)
    }
}

impl MetaExtDataControlDeviceV1 {
    /// Since when the error.used_source enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_USED_SOURCE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaExtDataControlDeviceV1Error(pub u32);

impl MetaExtDataControlDeviceV1Error {
    /// source given to set_selection or set_primary_selection was already used before
    #[allow(dead_code)]
    pub const USED_SOURCE: Self = Self(1);
}

impl Debug for MetaExtDataControlDeviceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::USED_SOURCE => "USED_SOURCE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
