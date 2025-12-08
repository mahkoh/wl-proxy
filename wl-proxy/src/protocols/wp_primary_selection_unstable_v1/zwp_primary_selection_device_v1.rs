use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_primary_selection_device_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpPrimarySelectionDeviceV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpPrimarySelectionDeviceV1Handler>,
}

struct DefaultHandler;

impl ZwpPrimarySelectionDeviceV1Handler for DefaultHandler { }

impl ConcreteObject for ZwpPrimarySelectionDeviceV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpPrimarySelectionDeviceV1;
    const INTERFACE_NAME: &str = "zwp_primary_selection_device_v1";
}

impl ZwpPrimarySelectionDeviceV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpPrimarySelectionDeviceV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpPrimarySelectionDeviceV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpPrimarySelectionDeviceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpPrimarySelectionDeviceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpPrimarySelectionDeviceV1 {
    /// Since when the set_selection message is available.
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
    pub fn try_send_set_selection(
        &self,
        source: Option<&Rc<ZwpPrimarySelectionSourceV1>>,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("source"))),
                Some(id) => id,
            },
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_primary_selection_device_v1#{}.set_selection(source: zwp_primary_selection_source_v1#{}, serial: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            arg1,
        ]);
        Ok(())
    }

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
        source: Option<&Rc<ZwpPrimarySelectionSourceV1>>,
        serial: u32,
    ) {
        let res = self.try_send_set_selection(
            source,
            serial,
        );
        if let Err(e) = res {
            log_send("zwp_primary_selection_device_v1.set_selection", &e);
        }
    }

    /// Since when the data_offer message is available.
    pub const MSG__DATA_OFFER__SINCE: u32 = 1;

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
    pub fn try_send_data_offer(
        &self,
        offer: &Rc<ZwpPrimarySelectionOfferV1>,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateClientId("offer", e)))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_primary_selection_device_v1#{}.data_offer(offer: zwp_primary_selection_offer_v1#{})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
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
    pub fn send_data_offer(
        &self,
        offer: &Rc<ZwpPrimarySelectionOfferV1>,
    ) {
        let res = self.try_send_data_offer(
            offer,
        );
        if let Err(e) = res {
            log_send("zwp_primary_selection_device_v1.data_offer", &e);
        }
    }

    /// introduce a new wp_primary_selection_offer
    ///
    /// Introduces a new wp_primary_selection_offer object that may be used
    /// to receive the current primary selection. Immediately following this
    /// event, the new wp_primary_selection_offer object will send
    /// wp_primary_selection_offer.offer events to describe the offered mime
    /// types.
    #[inline]
    pub fn new_try_send_data_offer(
        &self,
    ) -> Result<Rc<ZwpPrimarySelectionOfferV1>, ObjectError> {
        let offer = self.core.create_child();
        self.try_send_data_offer(
            &offer,
        )?;
        Ok(offer)
    }

    /// introduce a new wp_primary_selection_offer
    ///
    /// Introduces a new wp_primary_selection_offer object that may be used
    /// to receive the current primary selection. Immediately following this
    /// event, the new wp_primary_selection_offer object will send
    /// wp_primary_selection_offer.offer events to describe the offered mime
    /// types.
    #[inline]
    pub fn new_send_data_offer(
        &self,
    ) -> Rc<ZwpPrimarySelectionOfferV1> {
        let offer = self.core.create_child();
        self.send_data_offer(
            &offer,
        );
        offer
    }

    /// Since when the selection message is available.
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
    pub fn try_send_selection(
        &self,
        id: Option<&Rc<ZwpPrimarySelectionOfferV1>>,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if let Some(arg0) = arg0 {
            if arg0.client_id.get() != Some(client.endpoint.id) {
                return Err(ObjectError(ObjectErrorKind::ArgNoClientId("id", client.endpoint.id)));
            }
        }
        let arg0_id = arg0.and_then(|arg0| arg0.client_obj_id.get()).unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_primary_selection_device_v1#{}.selection(id: zwp_primary_selection_offer_v1#{})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0_id);
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
        id: Option<&Rc<ZwpPrimarySelectionOfferV1>>,
    ) {
        let res = self.try_send_selection(
            id,
        );
        if let Err(e) = res {
            log_send("zwp_primary_selection_device_v1.selection", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the primary selection device
    ///
    /// Destroy the primary selection device.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_primary_selection_device_v1#{}.destroy()\n", id);
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
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the primary selection device
    ///
    /// Destroy the primary selection device.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwp_primary_selection_device_v1.destroy", &e);
        }
    }
}

/// A message handler for [`ZwpPrimarySelectionDeviceV1`] proxies.
pub trait ZwpPrimarySelectionDeviceV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpPrimarySelectionDeviceV1>) {
        slf.core.delete_id();
    }

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
    fn handle_set_selection(
        &mut self,
        slf: &Rc<ZwpPrimarySelectionDeviceV1>,
        source: Option<&Rc<ZwpPrimarySelectionSourceV1>>,
        serial: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_selection(
            source,
            serial,
        );
        if let Err(e) = res {
            log_forward("zwp_primary_selection_device_v1.set_selection", &e);
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
    fn handle_data_offer(
        &mut self,
        slf: &Rc<ZwpPrimarySelectionDeviceV1>,
        offer: &Rc<ZwpPrimarySelectionOfferV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_data_offer(
            offer,
        );
        if let Err(e) = res {
            log_forward("zwp_primary_selection_device_v1.data_offer", &e);
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
    fn handle_selection(
        &mut self,
        slf: &Rc<ZwpPrimarySelectionDeviceV1>,
        id: Option<&Rc<ZwpPrimarySelectionOfferV1>>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(id) = id {
                if let Some(client_id_2) = id.core().client_id.get() {
                    if client_id != client_id_2 {
                        return;
                    }
                }
            }
        }
        let res = slf.try_send_selection(
            id,
        );
        if let Err(e) = res {
            log_forward("zwp_primary_selection_device_v1.selection", &e);
        }
    }

    /// destroy the primary selection device
    ///
    /// Destroy the primary selection device.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwpPrimarySelectionDeviceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwp_primary_selection_device_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for ZwpPrimarySelectionDeviceV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpPrimarySelectionDeviceV1, version),
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
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_primary_selection_device_v1#{}.set_selection(source: zwp_primary_selection_source_v1#{}, serial: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ZwpPrimarySelectionSourceV1>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("source", o.core().interface, ObjectInterface::ZwpPrimarySelectionSourceV1)));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_set_selection(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_selection(&self, arg0, arg1);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_primary_selection_device_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_primary_selection_device_v1#{}.data_offer(offer: zwp_primary_selection_offer_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = ZwpPrimarySelectionOfferV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetServerId(arg0_id, "offer", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_data_offer(&self, arg0);
                } else {
                    DefaultHandler.handle_data_offer(&self, arg0);
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
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_primary_selection_device_v1#{}.selection(id: zwp_primary_selection_offer_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = server.lookup(arg0_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoServerObject(arg0_id)));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ZwpPrimarySelectionOfferV1>() else {
                        let o = server.lookup(arg0_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("id", o.core().interface, ObjectInterface::ZwpPrimarySelectionOfferV1)));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_selection(&self, arg0);
                } else {
                    DefaultHandler.handle_selection(&self, arg0);
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

impl Object for ZwpPrimarySelectionDeviceV1 {
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

