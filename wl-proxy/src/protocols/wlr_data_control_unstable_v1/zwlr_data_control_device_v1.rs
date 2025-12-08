//! manage a data device for a seat
//!
//! This interface allows a client to manage a seat's selection.
//!
//! When the seat is destroyed, this object becomes inert.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwlr_data_control_device_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwlrDataControlDeviceV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwlrDataControlDeviceV1Handler>,
}

struct DefaultHandler;

impl ZwlrDataControlDeviceV1Handler for DefaultHandler { }

impl ConcreteObject for ZwlrDataControlDeviceV1 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwlrDataControlDeviceV1;
    const INTERFACE_NAME: &str = "zwlr_data_control_device_v1";
}

impl ZwlrDataControlDeviceV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwlrDataControlDeviceV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwlrDataControlDeviceV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwlrDataControlDeviceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwlrDataControlDeviceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwlrDataControlDeviceV1 {
    /// Since when the set_selection message is available.
    pub const MSG__SET_SELECTION__SINCE: u32 = 1;

    /// copy data to the selection
    ///
    /// This request asks the compositor to set the selection to the data from
    /// the source on behalf of the client.
    ///
    /// The given source may not be used in any further set_selection or
    /// set_primary_selection requests. Attempting to use a previously used
    /// source is a protocol error.
    ///
    /// To unset the selection, set the source to NULL.
    ///
    /// # Arguments
    ///
    /// - `source`:
    #[inline]
    pub fn try_send_set_selection(
        &self,
        source: Option<&Rc<ZwlrDataControlSourceV1>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            source,
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
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_data_control_device_v1#{}.set_selection(source: zwlr_data_control_source_v1#{})\n", id, arg0);
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

    /// copy data to the selection
    ///
    /// This request asks the compositor to set the selection to the data from
    /// the source on behalf of the client.
    ///
    /// The given source may not be used in any further set_selection or
    /// set_primary_selection requests. Attempting to use a previously used
    /// source is a protocol error.
    ///
    /// To unset the selection, set the source to NULL.
    ///
    /// # Arguments
    ///
    /// - `source`:
    #[inline]
    pub fn send_set_selection(
        &self,
        source: Option<&Rc<ZwlrDataControlSourceV1>>,
    ) {
        let res = self.try_send_set_selection(
            source,
        );
        if let Err(e) = res {
            log_send("zwlr_data_control_device_v1.set_selection", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy this data device
    ///
    /// Destroys the data device object.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_data_control_device_v1#{}.destroy()\n", id);
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

    /// destroy this data device
    ///
    /// Destroys the data device object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwlr_data_control_device_v1.destroy", &e);
        }
    }

    /// Since when the data_offer message is available.
    pub const MSG__DATA_OFFER__SINCE: u32 = 1;

    /// introduce a new wlr_data_control_offer
    ///
    /// The data_offer event introduces a new wlr_data_control_offer object,
    /// which will subsequently be used in either the
    /// wlr_data_control_device.selection event (for the regular clipboard
    /// selections) or the wlr_data_control_device.primary_selection event (for
    /// the primary clipboard selections). Immediately following the
    /// wlr_data_control_device.data_offer event, the new data_offer object
    /// will send out wlr_data_control_offer.offer events to describe the MIME
    /// types it offers.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_data_offer(
        &self,
        id: &Rc<ZwlrDataControlOfferV1>,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateClientId("id", e)))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_data_control_device_v1#{}.data_offer(id: zwlr_data_control_offer_v1#{})\n", client_id, id, arg0);
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

    /// introduce a new wlr_data_control_offer
    ///
    /// The data_offer event introduces a new wlr_data_control_offer object,
    /// which will subsequently be used in either the
    /// wlr_data_control_device.selection event (for the regular clipboard
    /// selections) or the wlr_data_control_device.primary_selection event (for
    /// the primary clipboard selections). Immediately following the
    /// wlr_data_control_device.data_offer event, the new data_offer object
    /// will send out wlr_data_control_offer.offer events to describe the MIME
    /// types it offers.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_data_offer(
        &self,
        id: &Rc<ZwlrDataControlOfferV1>,
    ) {
        let res = self.try_send_data_offer(
            id,
        );
        if let Err(e) = res {
            log_send("zwlr_data_control_device_v1.data_offer", &e);
        }
    }

    /// introduce a new wlr_data_control_offer
    ///
    /// The data_offer event introduces a new wlr_data_control_offer object,
    /// which will subsequently be used in either the
    /// wlr_data_control_device.selection event (for the regular clipboard
    /// selections) or the wlr_data_control_device.primary_selection event (for
    /// the primary clipboard selections). Immediately following the
    /// wlr_data_control_device.data_offer event, the new data_offer object
    /// will send out wlr_data_control_offer.offer events to describe the MIME
    /// types it offers.
    #[inline]
    pub fn new_try_send_data_offer(
        &self,
    ) -> Result<Rc<ZwlrDataControlOfferV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_data_offer(
            &id,
        )?;
        Ok(id)
    }

    /// introduce a new wlr_data_control_offer
    ///
    /// The data_offer event introduces a new wlr_data_control_offer object,
    /// which will subsequently be used in either the
    /// wlr_data_control_device.selection event (for the regular clipboard
    /// selections) or the wlr_data_control_device.primary_selection event (for
    /// the primary clipboard selections). Immediately following the
    /// wlr_data_control_device.data_offer event, the new data_offer object
    /// will send out wlr_data_control_offer.offer events to describe the MIME
    /// types it offers.
    #[inline]
    pub fn new_send_data_offer(
        &self,
    ) -> Rc<ZwlrDataControlOfferV1> {
        let id = self.core.create_child();
        self.send_data_offer(
            &id,
        );
        id
    }

    /// Since when the selection message is available.
    pub const MSG__SELECTION__SINCE: u32 = 1;

    /// advertise new selection
    ///
    /// The selection event is sent out to notify the client of a new
    /// wlr_data_control_offer for the selection for this device. The
    /// wlr_data_control_device.data_offer and the wlr_data_control_offer.offer
    /// events are sent out immediately before this event to introduce the data
    /// offer object. The selection event is sent to a client when a new
    /// selection is set. The wlr_data_control_offer is valid until a new
    /// wlr_data_control_offer or NULL is received. The client must destroy the
    /// previous selection wlr_data_control_offer, if any, upon receiving this
    /// event.
    ///
    /// The first selection event is sent upon binding the
    /// wlr_data_control_device object.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_selection(
        &self,
        id: Option<&Rc<ZwlrDataControlOfferV1>>,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_data_control_device_v1#{}.selection(id: zwlr_data_control_offer_v1#{})\n", client_id, id, arg0);
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

    /// advertise new selection
    ///
    /// The selection event is sent out to notify the client of a new
    /// wlr_data_control_offer for the selection for this device. The
    /// wlr_data_control_device.data_offer and the wlr_data_control_offer.offer
    /// events are sent out immediately before this event to introduce the data
    /// offer object. The selection event is sent to a client when a new
    /// selection is set. The wlr_data_control_offer is valid until a new
    /// wlr_data_control_offer or NULL is received. The client must destroy the
    /// previous selection wlr_data_control_offer, if any, upon receiving this
    /// event.
    ///
    /// The first selection event is sent upon binding the
    /// wlr_data_control_device object.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_selection(
        &self,
        id: Option<&Rc<ZwlrDataControlOfferV1>>,
    ) {
        let res = self.try_send_selection(
            id,
        );
        if let Err(e) = res {
            log_send("zwlr_data_control_device_v1.selection", &e);
        }
    }

    /// Since when the finished message is available.
    pub const MSG__FINISHED__SINCE: u32 = 1;

    /// this data control is no longer valid
    ///
    /// This data control object is no longer valid and should be destroyed by
    /// the client.
    #[inline]
    pub fn try_send_finished(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_data_control_device_v1#{}.finished()\n", client_id, id);
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
            2,
        ]);
        Ok(())
    }

    /// this data control is no longer valid
    ///
    /// This data control object is no longer valid and should be destroyed by
    /// the client.
    #[inline]
    pub fn send_finished(
        &self,
    ) {
        let res = self.try_send_finished(
        );
        if let Err(e) = res {
            log_send("zwlr_data_control_device_v1.finished", &e);
        }
    }

    /// Since when the primary_selection message is available.
    pub const MSG__PRIMARY_SELECTION__SINCE: u32 = 2;

    /// advertise new primary selection
    ///
    /// The primary_selection event is sent out to notify the client of a new
    /// wlr_data_control_offer for the primary selection for this device. The
    /// wlr_data_control_device.data_offer and the wlr_data_control_offer.offer
    /// events are sent out immediately before this event to introduce the data
    /// offer object. The primary_selection event is sent to a client when a
    /// new primary selection is set. The wlr_data_control_offer is valid until
    /// a new wlr_data_control_offer or NULL is received. The client must
    /// destroy the previous primary selection wlr_data_control_offer, if any,
    /// upon receiving this event.
    ///
    /// If the compositor supports primary selection, the first
    /// primary_selection event is sent upon binding the
    /// wlr_data_control_device object.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_primary_selection(
        &self,
        id: Option<&Rc<ZwlrDataControlOfferV1>>,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_data_control_device_v1#{}.primary_selection(id: zwlr_data_control_offer_v1#{})\n", client_id, id, arg0);
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
            3,
            arg0_id,
        ]);
        Ok(())
    }

    /// advertise new primary selection
    ///
    /// The primary_selection event is sent out to notify the client of a new
    /// wlr_data_control_offer for the primary selection for this device. The
    /// wlr_data_control_device.data_offer and the wlr_data_control_offer.offer
    /// events are sent out immediately before this event to introduce the data
    /// offer object. The primary_selection event is sent to a client when a
    /// new primary selection is set. The wlr_data_control_offer is valid until
    /// a new wlr_data_control_offer or NULL is received. The client must
    /// destroy the previous primary selection wlr_data_control_offer, if any,
    /// upon receiving this event.
    ///
    /// If the compositor supports primary selection, the first
    /// primary_selection event is sent upon binding the
    /// wlr_data_control_device object.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_primary_selection(
        &self,
        id: Option<&Rc<ZwlrDataControlOfferV1>>,
    ) {
        let res = self.try_send_primary_selection(
            id,
        );
        if let Err(e) = res {
            log_send("zwlr_data_control_device_v1.primary_selection", &e);
        }
    }

    /// Since when the set_primary_selection message is available.
    pub const MSG__SET_PRIMARY_SELECTION__SINCE: u32 = 2;

    /// copy data to the primary selection
    ///
    /// This request asks the compositor to set the primary selection to the
    /// data from the source on behalf of the client.
    ///
    /// The given source may not be used in any further set_selection or
    /// set_primary_selection requests. Attempting to use a previously used
    /// source is a protocol error.
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
    pub fn try_send_set_primary_selection(
        &self,
        source: Option<&Rc<ZwlrDataControlSourceV1>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            source,
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
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_data_control_device_v1#{}.set_primary_selection(source: zwlr_data_control_source_v1#{})\n", id, arg0);
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
            2,
            arg0_id,
        ]);
        Ok(())
    }

    /// copy data to the primary selection
    ///
    /// This request asks the compositor to set the primary selection to the
    /// data from the source on behalf of the client.
    ///
    /// The given source may not be used in any further set_selection or
    /// set_primary_selection requests. Attempting to use a previously used
    /// source is a protocol error.
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
        source: Option<&Rc<ZwlrDataControlSourceV1>>,
    ) {
        let res = self.try_send_set_primary_selection(
            source,
        );
        if let Err(e) = res {
            log_send("zwlr_data_control_device_v1.set_primary_selection", &e);
        }
    }
}

/// A message handler for [`ZwlrDataControlDeviceV1`] proxies.
pub trait ZwlrDataControlDeviceV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwlrDataControlDeviceV1>) {
        slf.core.delete_id();
    }

    /// copy data to the selection
    ///
    /// This request asks the compositor to set the selection to the data from
    /// the source on behalf of the client.
    ///
    /// The given source may not be used in any further set_selection or
    /// set_primary_selection requests. Attempting to use a previously used
    /// source is a protocol error.
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
    fn handle_set_selection(
        &mut self,
        slf: &Rc<ZwlrDataControlDeviceV1>,
        source: Option<&Rc<ZwlrDataControlSourceV1>>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_selection(
            source,
        );
        if let Err(e) = res {
            log_forward("zwlr_data_control_device_v1.set_selection", &e);
        }
    }

    /// destroy this data device
    ///
    /// Destroys the data device object.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwlrDataControlDeviceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwlr_data_control_device_v1.destroy", &e);
        }
    }

    /// introduce a new wlr_data_control_offer
    ///
    /// The data_offer event introduces a new wlr_data_control_offer object,
    /// which will subsequently be used in either the
    /// wlr_data_control_device.selection event (for the regular clipboard
    /// selections) or the wlr_data_control_device.primary_selection event (for
    /// the primary clipboard selections). Immediately following the
    /// wlr_data_control_device.data_offer event, the new data_offer object
    /// will send out wlr_data_control_offer.offer events to describe the MIME
    /// types it offers.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn handle_data_offer(
        &mut self,
        slf: &Rc<ZwlrDataControlDeviceV1>,
        id: &Rc<ZwlrDataControlOfferV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_data_offer(
            id,
        );
        if let Err(e) = res {
            log_forward("zwlr_data_control_device_v1.data_offer", &e);
        }
    }

    /// advertise new selection
    ///
    /// The selection event is sent out to notify the client of a new
    /// wlr_data_control_offer for the selection for this device. The
    /// wlr_data_control_device.data_offer and the wlr_data_control_offer.offer
    /// events are sent out immediately before this event to introduce the data
    /// offer object. The selection event is sent to a client when a new
    /// selection is set. The wlr_data_control_offer is valid until a new
    /// wlr_data_control_offer or NULL is received. The client must destroy the
    /// previous selection wlr_data_control_offer, if any, upon receiving this
    /// event.
    ///
    /// The first selection event is sent upon binding the
    /// wlr_data_control_device object.
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
        slf: &Rc<ZwlrDataControlDeviceV1>,
        id: Option<&Rc<ZwlrDataControlOfferV1>>,
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
            log_forward("zwlr_data_control_device_v1.selection", &e);
        }
    }

    /// this data control is no longer valid
    ///
    /// This data control object is no longer valid and should be destroyed by
    /// the client.
    #[inline]
    fn handle_finished(
        &mut self,
        slf: &Rc<ZwlrDataControlDeviceV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_finished(
        );
        if let Err(e) = res {
            log_forward("zwlr_data_control_device_v1.finished", &e);
        }
    }

    /// advertise new primary selection
    ///
    /// The primary_selection event is sent out to notify the client of a new
    /// wlr_data_control_offer for the primary selection for this device. The
    /// wlr_data_control_device.data_offer and the wlr_data_control_offer.offer
    /// events are sent out immediately before this event to introduce the data
    /// offer object. The primary_selection event is sent to a client when a
    /// new primary selection is set. The wlr_data_control_offer is valid until
    /// a new wlr_data_control_offer or NULL is received. The client must
    /// destroy the previous primary selection wlr_data_control_offer, if any,
    /// upon receiving this event.
    ///
    /// If the compositor supports primary selection, the first
    /// primary_selection event is sent upon binding the
    /// wlr_data_control_device object.
    ///
    /// # Arguments
    ///
    /// - `id`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_primary_selection(
        &mut self,
        slf: &Rc<ZwlrDataControlDeviceV1>,
        id: Option<&Rc<ZwlrDataControlOfferV1>>,
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
        let res = slf.try_send_primary_selection(
            id,
        );
        if let Err(e) = res {
            log_forward("zwlr_data_control_device_v1.primary_selection", &e);
        }
    }

    /// copy data to the primary selection
    ///
    /// This request asks the compositor to set the primary selection to the
    /// data from the source on behalf of the client.
    ///
    /// The given source may not be used in any further set_selection or
    /// set_primary_selection requests. Attempting to use a previously used
    /// source is a protocol error.
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
    fn handle_set_primary_selection(
        &mut self,
        slf: &Rc<ZwlrDataControlDeviceV1>,
        source: Option<&Rc<ZwlrDataControlSourceV1>>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_primary_selection(
            source,
        );
        if let Err(e) = res {
            log_forward("zwlr_data_control_device_v1.set_primary_selection", &e);
        }
    }
}

impl ObjectPrivate for ZwlrDataControlDeviceV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwlrDataControlDeviceV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_data_control_device_v1#{}.set_selection(source: zwlr_data_control_source_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ZwlrDataControlSourceV1>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("source", o.core().interface, ObjectInterface::ZwlrDataControlSourceV1)));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_set_selection(&self, arg0);
                } else {
                    DefaultHandler.handle_set_selection(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_data_control_device_v1#{}.destroy()\n", client_id, id);
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
            2 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_data_control_device_v1#{}.set_primary_selection(source: zwlr_data_control_source_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ZwlrDataControlSourceV1>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("source", o.core().interface, ObjectInterface::ZwlrDataControlSourceV1)));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_set_primary_selection(&self, arg0);
                } else {
                    DefaultHandler.handle_set_primary_selection(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_data_control_device_v1#{}.data_offer(id: zwlr_data_control_offer_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = ZwlrDataControlOfferV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetServerId(arg0_id, "id", e)))?;
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_data_control_device_v1#{}.selection(id: zwlr_data_control_offer_v1#{})\n", id, arg0);
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
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ZwlrDataControlOfferV1>() else {
                        let o = server.lookup(arg0_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("id", o.core().interface, ObjectInterface::ZwlrDataControlOfferV1)));
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
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_data_control_device_v1#{}.finished()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_finished(&self);
                } else {
                    DefaultHandler.handle_finished(&self);
                }
            }
            3 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_data_control_device_v1#{}.primary_selection(id: zwlr_data_control_offer_v1#{})\n", id, arg0);
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
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ZwlrDataControlOfferV1>() else {
                        let o = server.lookup(arg0_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("id", o.core().interface, ObjectInterface::ZwlrDataControlOfferV1)));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_primary_selection(&self, arg0);
                } else {
                    DefaultHandler.handle_primary_selection(&self, arg0);
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

impl Object for ZwlrDataControlDeviceV1 {
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

impl ZwlrDataControlDeviceV1 {
    /// Since when the error.used_source enum variant is available.
    pub const ENM__ERROR_USED_SOURCE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwlrDataControlDeviceV1Error(pub u32);

impl ZwlrDataControlDeviceV1Error {
    /// source given to set_selection or set_primary_selection was already used before
    pub const USED_SOURCE: Self = Self(1);
}

impl Debug for ZwlrDataControlDeviceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::USED_SOURCE => "USED_SOURCE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
