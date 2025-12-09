use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_primary_selection_device_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpPrimarySelectionDeviceV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpPrimarySelectionDeviceV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpPrimarySelectionDeviceV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpPrimarySelectionDeviceV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZwpPrimarySelectionDeviceV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpPrimarySelectionDeviceV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaZwpPrimarySelectionDeviceV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaZwpPrimarySelectionDeviceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpPrimarySelectionDeviceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpPrimarySelectionDeviceV1 {
    /// Since when the set_selection message is available.
    #[allow(dead_code)]
    pub const MSG__SET_SELECTION__SINCE: u32 = 1;

    /// set the primary selection
    ///
    /// Replaces the current selection. The previous owner of the primary
    /// selection will receive a wp_primary_selection_source.cancelled event.
    ///
    /// To unset the selection, set the source to NULL.
    ///
    /// # Arguments
    ///
    /// - `source`:
    /// - `serial`: serial of the event that triggered this request
    #[inline]
    pub fn send_set_selection(
        &self,
        source: Option<&Rc<MetaZwpPrimarySelectionSourceV1>>,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            source,
            serial,
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
            arg1,
        ]);
        Ok(())
    }

    /// Since when the data_offer message is available.
    #[allow(dead_code)]
    pub const MSG__DATA_OFFER__SINCE: u32 = 1;

    /// introduce a new wp_primary_selection_offer
    ///
    /// Introduces a new wp_primary_selection_offer object that may be used
    /// to receive the current primary selection. Immediately following this
    /// event, the new wp_primary_selection_offer object will send
    /// wp_primary_selection_offer.offer events to describe the offered mime
    /// types.
    #[inline]
    pub fn send_data_offer(
        &self,
        offer: &Rc<MetaZwpPrimarySelectionOfferV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            offer,
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
            .map_err(|e| ObjectError::GenerateClientId("offer", e))?;
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

    /// advertise a new primary selection
    ///
    /// The wp_primary_selection_device.selection event is sent to notify the
    /// client of a new primary selection. This event is sent after the
    /// wp_primary_selection.data_offer event introducing this object, and after
    /// the offer has announced its mimetypes through
    /// wp_primary_selection_offer.offer.
    ///
    /// The data_offer is valid until a new offer or NULL is received
    /// or until the client loses keyboard focus. The client must destroy the
    /// previous selection data_offer, if any, upon receiving this event.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_selection(
        &self,
        id: Option<&Rc<MetaZwpPrimarySelectionOfferV1>>,
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

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the primary selection device
    ///
    /// Destroy the primary selection device.
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
}

/// A message handler for [ZwpPrimarySelectionDeviceV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpPrimarySelectionDeviceV1MessageHandler {
    /// set the primary selection
    ///
    /// Replaces the current selection. The previous owner of the primary
    /// selection will receive a wp_primary_selection_source.cancelled event.
    ///
    /// To unset the selection, set the source to NULL.
    ///
    /// # Arguments
    ///
    /// - `source`:
    /// - `serial`: serial of the event that triggered this request
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn set_selection(
        &mut self,
        _slf: &Rc<MetaZwpPrimarySelectionDeviceV1>,
        source: Option<&Rc<MetaZwpPrimarySelectionSourceV1>>,
        serial: u32,
    ) {
        let res = _slf.send_set_selection(
            source,
            serial,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_primary_selection_device_v1.set_selection message: {}", Report::new(e));
        }
    }

    /// introduce a new wp_primary_selection_offer
    ///
    /// Introduces a new wp_primary_selection_offer object that may be used
    /// to receive the current primary selection. Immediately following this
    /// event, the new wp_primary_selection_offer object will send
    /// wp_primary_selection_offer.offer events to describe the offered mime
    /// types.
    ///
    /// # Arguments
    ///
    /// - `offer`:
    #[inline]
    fn data_offer(
        &mut self,
        _slf: &Rc<MetaZwpPrimarySelectionDeviceV1>,
        offer: &Rc<MetaZwpPrimarySelectionOfferV1>,
    ) {
        let res = _slf.send_data_offer(
            offer,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_primary_selection_device_v1.data_offer message: {}", Report::new(e));
        }
    }

    /// advertise a new primary selection
    ///
    /// The wp_primary_selection_device.selection event is sent to notify the
    /// client of a new primary selection. This event is sent after the
    /// wp_primary_selection.data_offer event introducing this object, and after
    /// the offer has announced its mimetypes through
    /// wp_primary_selection_offer.offer.
    ///
    /// The data_offer is valid until a new offer or NULL is received
    /// or until the client loses keyboard focus. The client must destroy the
    /// previous selection data_offer, if any, upon receiving this event.
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
        _slf: &Rc<MetaZwpPrimarySelectionDeviceV1>,
        id: Option<&Rc<MetaZwpPrimarySelectionOfferV1>>,
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
            log::warn!("Could not forward a zwp_primary_selection_device_v1.selection message: {}", Report::new(e));
        }
    }

    /// destroy the primary selection device
    ///
    /// Destroy the primary selection device.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpPrimarySelectionDeviceV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_primary_selection_device_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpPrimarySelectionDeviceV1 {
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
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaZwpPrimarySelectionSourceV1>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError::WrongObjectType("source", o.core().interface, ProxyInterface::ZwpPrimarySelectionSourceV1));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).set_selection(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.set_selection(&self, arg0, arg1);
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
                let arg0 = MetaZwpPrimarySelectionOfferV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetServerId(arg0_id, "offer", e))?;
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
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaZwpPrimarySelectionOfferV1>() else {
                        let o = self.core.state.server.lookup(arg0_id).unwrap();
                        return Err(ObjectError::WrongObjectType("id", o.core().interface, ProxyInterface::ZwpPrimarySelectionOfferV1));
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
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "data_offer",
            1 => "selection",
            _ => return None,
        };
        Some(name)
    }
}

