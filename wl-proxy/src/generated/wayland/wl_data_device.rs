//! data transfer device
//!
//! There is one wl_data_device per seat which can be obtained
//! from the global wl_data_device_manager singleton.
//!
//! A wl_data_device provides access to inter-client data transfer
//! mechanisms such as copy-and-paste and drag-and-drop.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wl_data_device proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWlDataDevice {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWlDataDeviceMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWlDataDeviceMessageHandler for DefaultMessageHandler { }

impl MetaWlDataDevice {
    pub const XML_VERSION: u32 = 3;
}

impl MetaWlDataDevice {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WlDataDevice, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWlDataDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWlDataDevice")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWlDataDevice {
    /// Since when the start_drag message is available.
    #[allow(dead_code)]
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
    pub fn send_start_drag(
        &self,
        source: Option<&Rc<MetaWlDataSource>>,
        origin: &Rc<MetaWlSurface>,
        icon: Option<&Rc<MetaWlSurface>>,
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError::ArgNoServerId("source")),
                Some(id) => id,
            },
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("origin")),
            Some(id) => id,
        };
        let arg2_id = match arg2 {
            None => 0,
            Some(arg2) => match arg2.server_obj_id.get() {
                None => return Err(ObjectError::ArgNoServerId("icon")),
                Some(id) => id,
            },
        };
        eprintln!("server      <= wl_data_device#{}.start_drag(source: wl_data_source#{}, origin: wl_surface#{}, icon: wl_surface#{}, serial: {})", id, arg0_id, arg1_id, arg2_id, arg3);
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
            arg1_id,
            arg2_id,
            arg3,
        ]);
        Ok(())
    }

    /// Since when the set_selection message is available.
    #[allow(dead_code)]
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
    pub fn send_set_selection(
        &self,
        source: Option<&Rc<MetaWlDataSource>>,
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
        eprintln!("server      <= wl_data_device#{}.set_selection(source: wl_data_source#{}, serial: {})", id, arg0_id, arg1);
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
            arg1,
        ]);
        Ok(())
    }

    /// Since when the data_offer message is available.
    #[allow(dead_code)]
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
    #[inline]
    pub fn send_data_offer(
        &self,
        id: &Rc<MetaWlDataOffer>,
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
        eprintln!("client#{:04} <= wl_data_device#{}.data_offer(id: wl_data_offer#{})", client.endpoint.id, id, arg0_id);
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

    /// Since when the enter message is available.
    #[allow(dead_code)]
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
    pub fn send_enter(
        &self,
        serial: u32,
        surface: &Rc<MetaWlSurface>,
        x: Fixed,
        y: Fixed,
        id: Option<&Rc<MetaWlDataOffer>>,
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg1.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError::ArgNoClientId("surface", client.endpoint.id));
        }
        if let Some(arg4) = arg4 {
            if arg4.client_id.get() != Some(client.endpoint.id) {
                return Err(ObjectError::ArgNoClientId("id", client.endpoint.id));
            }
        }
        let arg1_id = arg1.client_obj_id.get().unwrap_or(0);
        let arg4_id = arg4.map(|arg4| arg4.client_obj_id.get()).flatten().unwrap_or(0);
        eprintln!("client#{:04} <= wl_data_device#{}.enter(serial: {}, surface: wl_surface#{}, x: {}, y: {}, id: wl_data_offer#{})", client.endpoint.id, id, arg0, arg1_id, arg2, arg3, arg4_id);
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
            arg0,
            arg1_id,
            arg2.to_wire() as u32,
            arg3.to_wire() as u32,
            arg4_id,
        ]);
        Ok(())
    }

    /// Since when the leave message is available.
    #[allow(dead_code)]
    pub const MSG__LEAVE__SINCE: u32 = 1;

    /// end drag-and-drop session
    ///
    /// This event is sent when the drag-and-drop pointer leaves the
    /// surface and the session ends.  The client must destroy the
    /// wl_data_offer introduced at enter time at this point.
    #[inline]
    pub fn send_leave(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= wl_data_device#{}.leave()", client.endpoint.id, id);
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

    /// Since when the motion message is available.
    #[allow(dead_code)]
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
    pub fn send_motion(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= wl_data_device#{}.motion(time: {}, x: {}, y: {})", client.endpoint.id, id, arg0, arg1, arg2);
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
            arg0,
            arg1.to_wire() as u32,
            arg2.to_wire() as u32,
        ]);
        Ok(())
    }

    /// Since when the drop message is available.
    #[allow(dead_code)]
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
    pub fn send_drop(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= wl_data_device#{}.drop()", client.endpoint.id, id);
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

    /// Since when the selection message is available.
    #[allow(dead_code)]
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
    pub fn send_selection(
        &self,
        id: Option<&Rc<MetaWlDataOffer>>,
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
        eprintln!("client#{:04} <= wl_data_device#{}.selection(id: wl_data_offer#{})", client.endpoint.id, id, arg0_id);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the release message is available.
    #[allow(dead_code)]
    pub const MSG__RELEASE__SINCE: u32 = 2;

    /// destroy data device
    ///
    /// This request destroys the data device.
    #[inline]
    pub fn send_release(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= wl_data_device#{}.release()", id);
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
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [WlDataDevice] proxies.
#[allow(dead_code)]
pub trait MetaWlDataDeviceMessageHandler {
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
    fn start_drag(
        &mut self,
        _slf: &Rc<MetaWlDataDevice>,
        source: Option<&Rc<MetaWlDataSource>>,
        origin: &Rc<MetaWlSurface>,
        icon: Option<&Rc<MetaWlSurface>>,
        serial: u32,
    ) {
        let res = _slf.send_start_drag(
            source,
            origin,
            icon,
            serial,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_device.start_drag message: {}", Report::new(e));
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
    fn set_selection(
        &mut self,
        _slf: &Rc<MetaWlDataDevice>,
        source: Option<&Rc<MetaWlDataSource>>,
        serial: u32,
    ) {
        let res = _slf.send_set_selection(
            source,
            serial,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_device.set_selection message: {}", Report::new(e));
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
    fn data_offer(
        &mut self,
        _slf: &Rc<MetaWlDataDevice>,
        id: &Rc<MetaWlDataOffer>,
    ) {
        let res = _slf.send_data_offer(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_device.data_offer message: {}", Report::new(e));
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
    fn enter(
        &mut self,
        _slf: &Rc<MetaWlDataDevice>,
        serial: u32,
        surface: &Rc<MetaWlSurface>,
        x: Fixed,
        y: Fixed,
        id: Option<&Rc<MetaWlDataOffer>>,
    ) {
        if let Some(client_id) = _slf.core.client_id.get() {
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
        let res = _slf.send_enter(
            serial,
            surface,
            x,
            y,
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_device.enter message: {}", Report::new(e));
        }
    }

    /// end drag-and-drop session
    ///
    /// This event is sent when the drag-and-drop pointer leaves the
    /// surface and the session ends.  The client must destroy the
    /// wl_data_offer introduced at enter time at this point.
    #[inline]
    fn leave(
        &mut self,
        _slf: &Rc<MetaWlDataDevice>,
    ) {
        let res = _slf.send_leave(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_device.leave message: {}", Report::new(e));
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
    fn motion(
        &mut self,
        _slf: &Rc<MetaWlDataDevice>,
        time: u32,
        x: Fixed,
        y: Fixed,
    ) {
        let res = _slf.send_motion(
            time,
            x,
            y,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_device.motion message: {}", Report::new(e));
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
    fn drop(
        &mut self,
        _slf: &Rc<MetaWlDataDevice>,
    ) {
        let res = _slf.send_drop(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_device.drop message: {}", Report::new(e));
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
    fn selection(
        &mut self,
        _slf: &Rc<MetaWlDataDevice>,
        id: Option<&Rc<MetaWlDataOffer>>,
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
            log::warn!("Could not forward a wl_data_device.selection message: {}", Report::new(e));
        }
    }

    /// destroy data device
    ///
    /// This request destroys the data device.
    #[inline]
    fn release(
        &mut self,
        _slf: &Rc<MetaWlDataDevice>,
    ) {
        let res = _slf.send_release(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_device.release message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWlDataDevice {
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
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 24));
                };
                eprintln!("client#{:04} -> wl_data_device#{}.start_drag(source: wl_data_source#{}, origin: wl_surface#{}, icon: wl_surface#{}, serial: {})", client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlDataSource>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError::WrongObjectType("source", o.core().interface, ProxyInterface::WlDataSource));
                    };
                    Some(arg0)
                };
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("origin", o.core().interface, ProxyInterface::WlSurface));
                };
                let arg2 = if arg2 == 0 {
                    None
                } else {
                    let arg2_id = arg2;
                    let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg2_id));
                    };
                    let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                        let o = client.endpoint.lookup(arg2_id).unwrap();
                        return Err(ObjectError::WrongObjectType("icon", o.core().interface, ProxyInterface::WlSurface));
                    };
                    Some(arg2)
                };
                let arg0 = arg0.as_ref();
                let arg1 = &arg1;
                let arg2 = arg2.as_ref();
                if let Some(handler) = handler {
                    (**handler).start_drag(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultMessageHandler.start_drag(&self, arg0, arg1, arg2, arg3);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                eprintln!("client#{:04} -> wl_data_device#{}.set_selection(source: wl_data_source#{}, serial: {})", client.endpoint.id, msg[0], arg0, arg1);
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlDataSource>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError::WrongObjectType("source", o.core().interface, ProxyInterface::WlDataSource));
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
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("client#{:04} -> wl_data_device#{}.release()", client.endpoint.id, msg[0]);
                if let Some(handler) = handler {
                    (**handler).release(&self);
                } else {
                    DefaultMessageHandler.release(&self);
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
                eprintln!("server      -> wl_data_device#{}.data_offer(id: wl_data_offer#{})", msg[0], arg0);
                let arg0_id = arg0;
                let arg0 = MetaWlDataOffer::new(&self.core.state, self.core.version);
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
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 28));
                };
                let arg2 = Fixed::from_wire(arg2 as i32);
                let arg3 = Fixed::from_wire(arg3 as i32);
                eprintln!("server      -> wl_data_device#{}.enter(serial: {}, surface: wl_surface#{}, x: {}, y: {}, id: wl_data_offer#{})", msg[0], arg0, arg1, arg2, arg3, arg4);
                let arg1_id = arg1;
                let Some(arg1) = self.core.state.server.lookup(arg1_id) else {
                    return Err(ObjectError::NoServerObject(arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    let o = self.core.state.server.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ProxyInterface::WlSurface));
                };
                let arg4 = if arg4 == 0 {
                    None
                } else {
                    let arg4_id = arg4;
                    let Some(arg4) = self.core.state.server.lookup(arg4_id) else {
                        return Err(ObjectError::NoServerObject(arg4_id));
                    };
                    let Ok(arg4) = (arg4 as Rc<dyn Any>).downcast::<MetaWlDataOffer>() else {
                        let o = self.core.state.server.lookup(arg4_id).unwrap();
                        return Err(ObjectError::WrongObjectType("id", o.core().interface, ProxyInterface::WlDataOffer));
                    };
                    Some(arg4)
                };
                let arg1 = &arg1;
                let arg4 = arg4.as_ref();
                if let Some(handler) = handler {
                    (**handler).enter(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultMessageHandler.enter(&self, arg0, arg1, arg2, arg3, arg4);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("server      -> wl_data_device#{}.leave()", msg[0]);
                if let Some(handler) = handler {
                    (**handler).leave(&self);
                } else {
                    DefaultMessageHandler.leave(&self);
                }
            }
            3 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                let arg1 = Fixed::from_wire(arg1 as i32);
                let arg2 = Fixed::from_wire(arg2 as i32);
                eprintln!("server      -> wl_data_device#{}.motion(time: {}, x: {}, y: {})", msg[0], arg0, arg1, arg2);
                if let Some(handler) = handler {
                    (**handler).motion(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.motion(&self, arg0, arg1, arg2);
                }
            }
            4 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("server      -> wl_data_device#{}.drop()", msg[0]);
                if let Some(handler) = handler {
                    (**handler).drop(&self);
                } else {
                    DefaultMessageHandler.drop(&self);
                }
            }
            5 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                eprintln!("server      -> wl_data_device#{}.selection(id: wl_data_offer#{})", msg[0], arg0);
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = self.core.state.server.lookup(arg0_id) else {
                        return Err(ObjectError::NoServerObject(arg0_id));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlDataOffer>() else {
                        let o = self.core.state.server.lookup(arg0_id).unwrap();
                        return Err(ObjectError::WrongObjectType("id", o.core().interface, ProxyInterface::WlDataOffer));
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

impl MetaWlDataDevice {
    /// Since when the error.role enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ROLE__SINCE: u32 = 1;
    /// Since when the error.used_source enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_USED_SOURCE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWlDataDeviceError(pub u32);

impl MetaWlDataDeviceError {
    /// given wl_surface has another role
    #[allow(dead_code)]
    pub const ROLE: Self = Self(0);

    /// source has already been used
    #[allow(dead_code)]
    pub const USED_SOURCE: Self = Self(1);
}

impl Debug for MetaWlDataDeviceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ROLE => "ROLE",
            Self::USED_SOURCE => "USED_SOURCE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
