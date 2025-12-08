//! data transfer device
//!
//! There is one wl_data_device per seat which can be obtained
//! from the global wl_data_device_manager singleton.
//!
//! A wl_data_device provides access to inter-client data transfer
//! mechanisms such as copy-and-paste and drag-and-drop.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wl_data_device object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlDataDevice {
    core: ObjectCore,
    handler: HandlerHolder<dyn WlDataDeviceHandler>,
}

struct DefaultHandler;

impl WlDataDeviceHandler for DefaultHandler { }

impl ConcreteObject for WlDataDevice {
    const XML_VERSION: u32 = 3;
    const INTERFACE: ObjectInterface = ObjectInterface::WlDataDevice;
    const INTERFACE_NAME: &str = "wl_data_device";
}

impl WlDataDevice {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WlDataDeviceHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WlDataDeviceHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlDataDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlDataDevice")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlDataDevice {
    /// Since when the start_drag message is available.
    pub const MSG__START_DRAG__SINCE: u32 = 1;

    /// start drag-and-drop operation
    ///
    /// This request asks the compositor to start a drag-and-drop
    /// operation on behalf of the client.
    ///
    /// The source argument is the data source that provides the data
    /// for the eventual data transfer. If source is NULL, enter, leave
    /// and motion events are sent only to the client that initiated the
    /// drag and the client is expected to handle the data passing
    /// internally. If source is destroyed, the drag-and-drop session will be
    /// cancelled.
    ///
    /// The origin surface is the surface where the drag originates and
    /// the client must have an active implicit grab that matches the
    /// serial.
    ///
    /// The icon surface is an optional (can be NULL) surface that
    /// provides an icon to be moved around with the cursor.  Initially,
    /// the top-left corner of the icon surface is placed at the cursor
    /// hotspot, but subsequent wl_surface.offset requests can move the
    /// relative position. Attach requests must be confirmed with
    /// wl_surface.commit as usual. The icon surface is given the role of
    /// a drag-and-drop icon. If the icon surface already has another role,
    /// it raises a protocol error.
    ///
    /// The input region is ignored for wl_surfaces with the role of a
    /// drag-and-drop icon.
    ///
    /// The given source may not be used in any further set_selection or
    /// start_drag requests. Attempting to reuse a previously-used source
    /// may send a used_source error.
    ///
    /// # Arguments
    ///
    /// - `source`: data source for the eventual transfer
    /// - `origin`: surface where the drag originates
    /// - `icon`: drag-and-drop icon surface
    /// - `serial`: serial number of the implicit grab on the origin
    #[inline]
    pub fn try_send_start_drag(
        &self,
        source: Option<&Rc<WlDataSource>>,
        origin: &Rc<WlSurface>,
        icon: Option<&Rc<WlSurface>>,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            source,
            origin,
            icon,
            serial,
        );
        let arg0 = arg0.map(|a| a.core());
        let arg1 = arg1.core();
        let arg2 = arg2.map(|a| a.core());
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
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("origin"))),
            Some(id) => id,
        };
        let arg2_id = match arg2 {
            None => 0,
            Some(arg2) => match arg2.server_obj_id.get() {
                None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("icon"))),
                Some(id) => id,
            },
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_data_device#{}.start_drag(source: wl_data_source#{}, origin: wl_surface#{}, icon: wl_surface#{}, serial: {})\n", id, arg0, arg1, arg2, arg3);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id, arg2_id, arg3);
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
            arg1_id,
            arg2_id,
            arg3,
        ]);
        Ok(())
    }

    /// start drag-and-drop operation
    ///
    /// This request asks the compositor to start a drag-and-drop
    /// operation on behalf of the client.
    ///
    /// The source argument is the data source that provides the data
    /// for the eventual data transfer. If source is NULL, enter, leave
    /// and motion events are sent only to the client that initiated the
    /// drag and the client is expected to handle the data passing
    /// internally. If source is destroyed, the drag-and-drop session will be
    /// cancelled.
    ///
    /// The origin surface is the surface where the drag originates and
    /// the client must have an active implicit grab that matches the
    /// serial.
    ///
    /// The icon surface is an optional (can be NULL) surface that
    /// provides an icon to be moved around with the cursor.  Initially,
    /// the top-left corner of the icon surface is placed at the cursor
    /// hotspot, but subsequent wl_surface.offset requests can move the
    /// relative position. Attach requests must be confirmed with
    /// wl_surface.commit as usual. The icon surface is given the role of
    /// a drag-and-drop icon. If the icon surface already has another role,
    /// it raises a protocol error.
    ///
    /// The input region is ignored for wl_surfaces with the role of a
    /// drag-and-drop icon.
    ///
    /// The given source may not be used in any further set_selection or
    /// start_drag requests. Attempting to reuse a previously-used source
    /// may send a used_source error.
    ///
    /// # Arguments
    ///
    /// - `source`: data source for the eventual transfer
    /// - `origin`: surface where the drag originates
    /// - `icon`: drag-and-drop icon surface
    /// - `serial`: serial number of the implicit grab on the origin
    #[inline]
    pub fn send_start_drag(
        &self,
        source: Option<&Rc<WlDataSource>>,
        origin: &Rc<WlSurface>,
        icon: Option<&Rc<WlSurface>>,
        serial: u32,
    ) {
        let res = self.try_send_start_drag(
            source,
            origin,
            icon,
            serial,
        );
        if let Err(e) = res {
            log_send("wl_data_device.start_drag", &e);
        }
    }

    /// Since when the set_selection message is available.
    pub const MSG__SET_SELECTION__SINCE: u32 = 1;

    /// copy data to the selection
    ///
    /// This request asks the compositor to set the selection
    /// to the data from the source on behalf of the client.
    ///
    /// To unset the selection, set the source to NULL.
    ///
    /// The given source may not be used in any further set_selection or
    /// start_drag requests. Attempting to reuse a previously-used source
    /// may send a used_source error.
    ///
    /// # Arguments
    ///
    /// - `source`: data source for the selection
    /// - `serial`: serial number of the event that triggered this request
    #[inline]
    pub fn try_send_set_selection(
        &self,
        source: Option<&Rc<WlDataSource>>,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_data_device#{}.set_selection(source: wl_data_source#{}, serial: {})\n", id, arg0, arg1);
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
            1,
            arg0_id,
            arg1,
        ]);
        Ok(())
    }

    /// copy data to the selection
    ///
    /// This request asks the compositor to set the selection
    /// to the data from the source on behalf of the client.
    ///
    /// To unset the selection, set the source to NULL.
    ///
    /// The given source may not be used in any further set_selection or
    /// start_drag requests. Attempting to reuse a previously-used source
    /// may send a used_source error.
    ///
    /// # Arguments
    ///
    /// - `source`: data source for the selection
    /// - `serial`: serial number of the event that triggered this request
    #[inline]
    pub fn send_set_selection(
        &self,
        source: Option<&Rc<WlDataSource>>,
        serial: u32,
    ) {
        let res = self.try_send_set_selection(
            source,
            serial,
        );
        if let Err(e) = res {
            log_send("wl_data_device.set_selection", &e);
        }
    }

    /// Since when the data_offer message is available.
    pub const MSG__DATA_OFFER__SINCE: u32 = 1;

    /// introduce a new wl_data_offer
    ///
    /// The data_offer event introduces a new wl_data_offer object,
    /// which will subsequently be used in either the
    /// data_device.enter event (for drag-and-drop) or the
    /// data_device.selection event (for selections).  Immediately
    /// following the data_device.data_offer event, the new data_offer
    /// object will send out data_offer.offer events to describe the
    /// mime types it offers.
    ///
    /// # Arguments
    ///
    /// - `id`: the new data_offer object
    #[inline]
    pub fn try_send_data_offer(
        &self,
        id: &Rc<WlDataOffer>,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_data_device#{}.data_offer(id: wl_data_offer#{})\n", client_id, id, arg0);
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

    /// introduce a new wl_data_offer
    ///
    /// The data_offer event introduces a new wl_data_offer object,
    /// which will subsequently be used in either the
    /// data_device.enter event (for drag-and-drop) or the
    /// data_device.selection event (for selections).  Immediately
    /// following the data_device.data_offer event, the new data_offer
    /// object will send out data_offer.offer events to describe the
    /// mime types it offers.
    ///
    /// # Arguments
    ///
    /// - `id`: the new data_offer object
    #[inline]
    pub fn send_data_offer(
        &self,
        id: &Rc<WlDataOffer>,
    ) {
        let res = self.try_send_data_offer(
            id,
        );
        if let Err(e) = res {
            log_send("wl_data_device.data_offer", &e);
        }
    }

    /// introduce a new wl_data_offer
    ///
    /// The data_offer event introduces a new wl_data_offer object,
    /// which will subsequently be used in either the
    /// data_device.enter event (for drag-and-drop) or the
    /// data_device.selection event (for selections).  Immediately
    /// following the data_device.data_offer event, the new data_offer
    /// object will send out data_offer.offer events to describe the
    /// mime types it offers.
    #[inline]
    pub fn new_try_send_data_offer(
        &self,
    ) -> Result<Rc<WlDataOffer>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_data_offer(
            &id,
        )?;
        Ok(id)
    }

    /// introduce a new wl_data_offer
    ///
    /// The data_offer event introduces a new wl_data_offer object,
    /// which will subsequently be used in either the
    /// data_device.enter event (for drag-and-drop) or the
    /// data_device.selection event (for selections).  Immediately
    /// following the data_device.data_offer event, the new data_offer
    /// object will send out data_offer.offer events to describe the
    /// mime types it offers.
    #[inline]
    pub fn new_send_data_offer(
        &self,
    ) -> Rc<WlDataOffer> {
        let id = self.core.create_child();
        self.send_data_offer(
            &id,
        );
        id
    }

    /// Since when the enter message is available.
    pub const MSG__ENTER__SINCE: u32 = 1;

    /// initiate drag-and-drop session
    ///
    /// This event is sent when an active drag-and-drop pointer enters
    /// a surface owned by the client.  The position of the pointer at
    /// enter time is provided by the x and y arguments, in surface-local
    /// coordinates.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the enter event
    /// - `surface`: client surface entered
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    /// - `id`: source data_offer object
    #[inline]
    pub fn try_send_enter(
        &self,
        serial: u32,
        surface: &Rc<WlSurface>,
        x: Fixed,
        y: Fixed,
        id: Option<&Rc<WlDataOffer>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ) = (
            serial,
            surface,
            x,
            y,
            id,
        );
        let arg1 = arg1.core();
        let arg4 = arg4.map(|a| a.core());
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg1.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("surface", client.endpoint.id)));
        }
        if let Some(arg4) = arg4 {
            if arg4.client_id.get() != Some(client.endpoint.id) {
                return Err(ObjectError(ObjectErrorKind::ArgNoClientId("id", client.endpoint.id)));
            }
        }
        let arg1_id = arg1.client_obj_id.get().unwrap_or(0);
        let arg4_id = arg4.and_then(|arg4| arg4.client_obj_id.get()).unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: Fixed, arg3: Fixed, arg4: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_data_device#{}.enter(serial: {}, surface: wl_surface#{}, x: {}, y: {}, id: wl_data_offer#{})\n", client_id, id, arg0, arg1, arg2, arg3, arg4);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1_id, arg2, arg3, arg4_id);
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
            arg0,
            arg1_id,
            arg2.to_wire() as u32,
            arg3.to_wire() as u32,
            arg4_id,
        ]);
        Ok(())
    }

    /// initiate drag-and-drop session
    ///
    /// This event is sent when an active drag-and-drop pointer enters
    /// a surface owned by the client.  The position of the pointer at
    /// enter time is provided by the x and y arguments, in surface-local
    /// coordinates.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the enter event
    /// - `surface`: client surface entered
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    /// - `id`: source data_offer object
    #[inline]
    pub fn send_enter(
        &self,
        serial: u32,
        surface: &Rc<WlSurface>,
        x: Fixed,
        y: Fixed,
        id: Option<&Rc<WlDataOffer>>,
    ) {
        let res = self.try_send_enter(
            serial,
            surface,
            x,
            y,
            id,
        );
        if let Err(e) = res {
            log_send("wl_data_device.enter", &e);
        }
    }

    /// Since when the leave message is available.
    pub const MSG__LEAVE__SINCE: u32 = 1;

    /// end drag-and-drop session
    ///
    /// This event is sent when the drag-and-drop pointer leaves the
    /// surface and the session ends.  The client must destroy the
    /// wl_data_offer introduced at enter time at this point.
    #[inline]
    pub fn try_send_leave(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_data_device#{}.leave()\n", client_id, id);
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

    /// end drag-and-drop session
    ///
    /// This event is sent when the drag-and-drop pointer leaves the
    /// surface and the session ends.  The client must destroy the
    /// wl_data_offer introduced at enter time at this point.
    #[inline]
    pub fn send_leave(
        &self,
    ) {
        let res = self.try_send_leave(
        );
        if let Err(e) = res {
            log_send("wl_data_device.leave", &e);
        }
    }

    /// Since when the motion message is available.
    pub const MSG__MOTION__SINCE: u32 = 1;

    /// drag-and-drop session motion
    ///
    /// This event is sent when the drag-and-drop pointer moves within
    /// the currently focused surface. The new position of the pointer
    /// is provided by the x and y arguments, in surface-local
    /// coordinates.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    #[inline]
    pub fn try_send_motion(
        &self,
        time: u32,
        x: Fixed,
        y: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            time,
            x,
            y,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: Fixed, arg2: Fixed) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_data_device#{}.motion(time: {}, x: {}, y: {})\n", client_id, id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1, arg2);
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
            arg0,
            arg1.to_wire() as u32,
            arg2.to_wire() as u32,
        ]);
        Ok(())
    }

    /// drag-and-drop session motion
    ///
    /// This event is sent when the drag-and-drop pointer moves within
    /// the currently focused surface. The new position of the pointer
    /// is provided by the x and y arguments, in surface-local
    /// coordinates.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    #[inline]
    pub fn send_motion(
        &self,
        time: u32,
        x: Fixed,
        y: Fixed,
    ) {
        let res = self.try_send_motion(
            time,
            x,
            y,
        );
        if let Err(e) = res {
            log_send("wl_data_device.motion", &e);
        }
    }

    /// Since when the drop message is available.
    pub const MSG__DROP__SINCE: u32 = 1;

    /// end drag-and-drop session successfully
    ///
    /// The event is sent when a drag-and-drop operation is ended
    /// because the implicit grab is removed.
    ///
    /// The drag-and-drop destination is expected to honor the last action
    /// received through wl_data_offer.action, if the resulting action is
    /// "copy" or "move", the destination can still perform
    /// wl_data_offer.receive requests, and is expected to end all
    /// transfers with a wl_data_offer.finish request.
    ///
    /// If the resulting action is "ask", the action will not be considered
    /// final. The drag-and-drop destination is expected to perform one last
    /// wl_data_offer.set_actions request, or wl_data_offer.destroy in order
    /// to cancel the operation.
    #[inline]
    pub fn try_send_drop(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_data_device#{}.drop()\n", client_id, id);
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

    /// end drag-and-drop session successfully
    ///
    /// The event is sent when a drag-and-drop operation is ended
    /// because the implicit grab is removed.
    ///
    /// The drag-and-drop destination is expected to honor the last action
    /// received through wl_data_offer.action, if the resulting action is
    /// "copy" or "move", the destination can still perform
    /// wl_data_offer.receive requests, and is expected to end all
    /// transfers with a wl_data_offer.finish request.
    ///
    /// If the resulting action is "ask", the action will not be considered
    /// final. The drag-and-drop destination is expected to perform one last
    /// wl_data_offer.set_actions request, or wl_data_offer.destroy in order
    /// to cancel the operation.
    #[inline]
    pub fn send_drop(
        &self,
    ) {
        let res = self.try_send_drop(
        );
        if let Err(e) = res {
            log_send("wl_data_device.drop", &e);
        }
    }

    /// Since when the selection message is available.
    pub const MSG__SELECTION__SINCE: u32 = 1;

    /// advertise new selection
    ///
    /// The selection event is sent out to notify the client of a new
    /// wl_data_offer for the selection for this device.  The
    /// data_device.data_offer and the data_offer.offer events are
    /// sent out immediately before this event to introduce the data
    /// offer object.  The selection event is sent to a client
    /// immediately before receiving keyboard focus and when a new
    /// selection is set while the client has keyboard focus.  The
    /// data_offer is valid until a new data_offer or NULL is received
    /// or until the client loses keyboard focus.  Switching surface with
    /// keyboard focus within the same client doesn't mean a new selection
    /// will be sent.  The client must destroy the previous selection
    /// data_offer, if any, upon receiving this event.
    ///
    /// # Arguments
    ///
    /// - `id`: selection data_offer object
    #[inline]
    pub fn try_send_selection(
        &self,
        id: Option<&Rc<WlDataOffer>>,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_data_device#{}.selection(id: wl_data_offer#{})\n", client_id, id, arg0);
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
            5,
            arg0_id,
        ]);
        Ok(())
    }

    /// advertise new selection
    ///
    /// The selection event is sent out to notify the client of a new
    /// wl_data_offer for the selection for this device.  The
    /// data_device.data_offer and the data_offer.offer events are
    /// sent out immediately before this event to introduce the data
    /// offer object.  The selection event is sent to a client
    /// immediately before receiving keyboard focus and when a new
    /// selection is set while the client has keyboard focus.  The
    /// data_offer is valid until a new data_offer or NULL is received
    /// or until the client loses keyboard focus.  Switching surface with
    /// keyboard focus within the same client doesn't mean a new selection
    /// will be sent.  The client must destroy the previous selection
    /// data_offer, if any, upon receiving this event.
    ///
    /// # Arguments
    ///
    /// - `id`: selection data_offer object
    #[inline]
    pub fn send_selection(
        &self,
        id: Option<&Rc<WlDataOffer>>,
    ) {
        let res = self.try_send_selection(
            id,
        );
        if let Err(e) = res {
            log_send("wl_data_device.selection", &e);
        }
    }

    /// Since when the release message is available.
    pub const MSG__RELEASE__SINCE: u32 = 2;

    /// destroy data device
    ///
    /// This request destroys the data device.
    #[inline]
    pub fn try_send_release(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_data_device#{}.release()\n", id);
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
            2,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy data device
    ///
    /// This request destroys the data device.
    #[inline]
    pub fn send_release(
        &self,
    ) {
        let res = self.try_send_release(
        );
        if let Err(e) = res {
            log_send("wl_data_device.release", &e);
        }
    }
}

/// A message handler for [`WlDataDevice`] proxies.
pub trait WlDataDeviceHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WlDataDevice>) {
        slf.core.delete_id();
    }

    /// start drag-and-drop operation
    ///
    /// This request asks the compositor to start a drag-and-drop
    /// operation on behalf of the client.
    ///
    /// The source argument is the data source that provides the data
    /// for the eventual data transfer. If source is NULL, enter, leave
    /// and motion events are sent only to the client that initiated the
    /// drag and the client is expected to handle the data passing
    /// internally. If source is destroyed, the drag-and-drop session will be
    /// cancelled.
    ///
    /// The origin surface is the surface where the drag originates and
    /// the client must have an active implicit grab that matches the
    /// serial.
    ///
    /// The icon surface is an optional (can be NULL) surface that
    /// provides an icon to be moved around with the cursor.  Initially,
    /// the top-left corner of the icon surface is placed at the cursor
    /// hotspot, but subsequent wl_surface.offset requests can move the
    /// relative position. Attach requests must be confirmed with
    /// wl_surface.commit as usual. The icon surface is given the role of
    /// a drag-and-drop icon. If the icon surface already has another role,
    /// it raises a protocol error.
    ///
    /// The input region is ignored for wl_surfaces with the role of a
    /// drag-and-drop icon.
    ///
    /// The given source may not be used in any further set_selection or
    /// start_drag requests. Attempting to reuse a previously-used source
    /// may send a used_source error.
    ///
    /// # Arguments
    ///
    /// - `source`: data source for the eventual transfer
    /// - `origin`: surface where the drag originates
    /// - `icon`: drag-and-drop icon surface
    /// - `serial`: serial number of the implicit grab on the origin
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_start_drag(
        &mut self,
        slf: &Rc<WlDataDevice>,
        source: Option<&Rc<WlDataSource>>,
        origin: &Rc<WlSurface>,
        icon: Option<&Rc<WlSurface>>,
        serial: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_start_drag(
            source,
            origin,
            icon,
            serial,
        );
        if let Err(e) = res {
            log_forward("wl_data_device.start_drag", &e);
        }
    }

    /// copy data to the selection
    ///
    /// This request asks the compositor to set the selection
    /// to the data from the source on behalf of the client.
    ///
    /// To unset the selection, set the source to NULL.
    ///
    /// The given source may not be used in any further set_selection or
    /// start_drag requests. Attempting to reuse a previously-used source
    /// may send a used_source error.
    ///
    /// # Arguments
    ///
    /// - `source`: data source for the selection
    /// - `serial`: serial number of the event that triggered this request
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_selection(
        &mut self,
        slf: &Rc<WlDataDevice>,
        source: Option<&Rc<WlDataSource>>,
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
            log_forward("wl_data_device.set_selection", &e);
        }
    }

    /// introduce a new wl_data_offer
    ///
    /// The data_offer event introduces a new wl_data_offer object,
    /// which will subsequently be used in either the
    /// data_device.enter event (for drag-and-drop) or the
    /// data_device.selection event (for selections).  Immediately
    /// following the data_device.data_offer event, the new data_offer
    /// object will send out data_offer.offer events to describe the
    /// mime types it offers.
    ///
    /// # Arguments
    ///
    /// - `id`: the new data_offer object
    #[inline]
    fn handle_data_offer(
        &mut self,
        slf: &Rc<WlDataDevice>,
        id: &Rc<WlDataOffer>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_data_offer(
            id,
        );
        if let Err(e) = res {
            log_forward("wl_data_device.data_offer", &e);
        }
    }

    /// initiate drag-and-drop session
    ///
    /// This event is sent when an active drag-and-drop pointer enters
    /// a surface owned by the client.  The position of the pointer at
    /// enter time is provided by the x and y arguments, in surface-local
    /// coordinates.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the enter event
    /// - `surface`: client surface entered
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    /// - `id`: source data_offer object
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_enter(
        &mut self,
        slf: &Rc<WlDataDevice>,
        serial: u32,
        surface: &Rc<WlSurface>,
        x: Fixed,
        y: Fixed,
        id: Option<&Rc<WlDataOffer>>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = surface.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
            if let Some(id) = id {
                if let Some(client_id_2) = id.core().client_id.get() {
                    if client_id != client_id_2 {
                        return;
                    }
                }
            }
        }
        let res = slf.try_send_enter(
            serial,
            surface,
            x,
            y,
            id,
        );
        if let Err(e) = res {
            log_forward("wl_data_device.enter", &e);
        }
    }

    /// end drag-and-drop session
    ///
    /// This event is sent when the drag-and-drop pointer leaves the
    /// surface and the session ends.  The client must destroy the
    /// wl_data_offer introduced at enter time at this point.
    #[inline]
    fn handle_leave(
        &mut self,
        slf: &Rc<WlDataDevice>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_leave(
        );
        if let Err(e) = res {
            log_forward("wl_data_device.leave", &e);
        }
    }

    /// drag-and-drop session motion
    ///
    /// This event is sent when the drag-and-drop pointer moves within
    /// the currently focused surface. The new position of the pointer
    /// is provided by the x and y arguments, in surface-local
    /// coordinates.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    #[inline]
    fn handle_motion(
        &mut self,
        slf: &Rc<WlDataDevice>,
        time: u32,
        x: Fixed,
        y: Fixed,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_motion(
            time,
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("wl_data_device.motion", &e);
        }
    }

    /// end drag-and-drop session successfully
    ///
    /// The event is sent when a drag-and-drop operation is ended
    /// because the implicit grab is removed.
    ///
    /// The drag-and-drop destination is expected to honor the last action
    /// received through wl_data_offer.action, if the resulting action is
    /// "copy" or "move", the destination can still perform
    /// wl_data_offer.receive requests, and is expected to end all
    /// transfers with a wl_data_offer.finish request.
    ///
    /// If the resulting action is "ask", the action will not be considered
    /// final. The drag-and-drop destination is expected to perform one last
    /// wl_data_offer.set_actions request, or wl_data_offer.destroy in order
    /// to cancel the operation.
    #[inline]
    fn handle_drop(
        &mut self,
        slf: &Rc<WlDataDevice>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_drop(
        );
        if let Err(e) = res {
            log_forward("wl_data_device.drop", &e);
        }
    }

    /// advertise new selection
    ///
    /// The selection event is sent out to notify the client of a new
    /// wl_data_offer for the selection for this device.  The
    /// data_device.data_offer and the data_offer.offer events are
    /// sent out immediately before this event to introduce the data
    /// offer object.  The selection event is sent to a client
    /// immediately before receiving keyboard focus and when a new
    /// selection is set while the client has keyboard focus.  The
    /// data_offer is valid until a new data_offer or NULL is received
    /// or until the client loses keyboard focus.  Switching surface with
    /// keyboard focus within the same client doesn't mean a new selection
    /// will be sent.  The client must destroy the previous selection
    /// data_offer, if any, upon receiving this event.
    ///
    /// # Arguments
    ///
    /// - `id`: selection data_offer object
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_selection(
        &mut self,
        slf: &Rc<WlDataDevice>,
        id: Option<&Rc<WlDataOffer>>,
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
            log_forward("wl_data_device.selection", &e);
        }
    }

    /// destroy data device
    ///
    /// This request destroys the data device.
    #[inline]
    fn handle_release(
        &mut self,
        slf: &Rc<WlDataDevice>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_release(
        );
        if let Err(e) = res {
            log_forward("wl_data_device.release", &e);
        }
    }
}

impl ObjectPrivate for WlDataDevice {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WlDataDevice, version),
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
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 24)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_data_device#{}.start_drag(source: wl_data_source#{}, origin: wl_surface#{}, icon: wl_surface#{}, serial: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlDataSource>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("source", o.core().interface, ObjectInterface::WlDataSource)));
                    };
                    Some(arg0)
                };
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("origin", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg2 = if arg2 == 0 {
                    None
                } else {
                    let arg2_id = arg2;
                    let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg2_id)));
                    };
                    let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<WlSurface>() else {
                        let o = client.endpoint.lookup(arg2_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("icon", o.core().interface, ObjectInterface::WlSurface)));
                    };
                    Some(arg2)
                };
                let arg0 = arg0.as_ref();
                let arg1 = &arg1;
                let arg2 = arg2.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_start_drag(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_start_drag(&self, arg0, arg1, arg2, arg3);
                }
            }
            1 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_data_device#{}.set_selection(source: wl_data_source#{}, serial: {})\n", client_id, id, arg0, arg1);
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
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlDataSource>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("source", o.core().interface, ObjectInterface::WlDataSource)));
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
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_data_device#{}.release()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_release(&self);
                } else {
                    DefaultHandler.handle_release(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_data_device#{}.data_offer(id: wl_data_offer#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WlDataOffer::new(&self.core.state, self.core.version);
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
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 28)));
                };
                let arg2 = Fixed::from_wire(arg2 as i32);
                let arg3 = Fixed::from_wire(arg3 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: Fixed, arg3: Fixed, arg4: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_data_device#{}.enter(serial: {}, surface: wl_surface#{}, x: {}, y: {}, id: wl_data_offer#{})\n", id, arg0, arg1, arg2, arg3, arg4);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2, arg3, arg4);
                }
                let arg1_id = arg1;
                let Some(arg1) = server.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = server.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg4 = if arg4 == 0 {
                    None
                } else {
                    let arg4_id = arg4;
                    let Some(arg4) = server.lookup(arg4_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoServerObject(arg4_id)));
                    };
                    let Ok(arg4) = (arg4 as Rc<dyn Any>).downcast::<WlDataOffer>() else {
                        let o = server.lookup(arg4_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("id", o.core().interface, ObjectInterface::WlDataOffer)));
                    };
                    Some(arg4)
                };
                let arg1 = &arg1;
                let arg4 = arg4.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_enter(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultHandler.handle_enter(&self, arg0, arg1, arg2, arg3, arg4);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_data_device#{}.leave()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_leave(&self);
                } else {
                    DefaultHandler.handle_leave(&self);
                }
            }
            3 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                let arg1 = Fixed::from_wire(arg1 as i32);
                let arg2 = Fixed::from_wire(arg2 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: Fixed, arg2: Fixed) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_data_device#{}.motion(time: {}, x: {}, y: {})\n", id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_motion(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_motion(&self, arg0, arg1, arg2);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_data_device#{}.drop()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_drop(&self);
                } else {
                    DefaultHandler.handle_drop(&self);
                }
            }
            5 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_data_device#{}.selection(id: wl_data_offer#{})\n", id, arg0);
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
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlDataOffer>() else {
                        let o = server.lookup(arg0_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("id", o.core().interface, ObjectInterface::WlDataOffer)));
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
            0 => "start_drag",
            1 => "set_selection",
            2 => "release",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "data_offer",
            1 => "enter",
            2 => "leave",
            3 => "motion",
            4 => "drop",
            5 => "selection",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WlDataDevice {
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

impl WlDataDevice {
    /// Since when the error.role enum variant is available.
    pub const ENM__ERROR_ROLE__SINCE: u32 = 1;
    /// Since when the error.used_source enum variant is available.
    pub const ENM__ERROR_USED_SOURCE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WlDataDeviceError(pub u32);

impl WlDataDeviceError {
    /// given wl_surface has another role
    pub const ROLE: Self = Self(0);

    /// source has already been used
    pub const USED_SOURCE: Self = Self(1);
}

impl Debug for WlDataDeviceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ROLE => "ROLE",
            Self::USED_SOURCE => "USED_SOURCE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
