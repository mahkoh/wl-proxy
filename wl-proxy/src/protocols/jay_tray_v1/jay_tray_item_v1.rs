//! tray item interface
//!
//! This interface represents an item in a tray. The underlying surface will
//! be displayed in the tray.
//!
//! Unless the global has been removed, the compositor will immediately emit a
//! configuration sequence after this object has been created. If applicable,
//! the sequence will also contain wl_surface.preferred_buffer_scale and
//! wp_fractional_scale_v1.preferred_scale events.
//!
//! After receiving the configuration sequence, the client must ack the
//! configuration and commit the surface. The item will not be displayed
//! before this.
//!
//! The compositor can send configuration sequences at any point.
//!
//! If a null buffer is committed, the item will not be displayed.
//! If the client wants the item to be displayed, it must ack and commit the
//! latest configuration sequence and attach and commit a non-null buffer.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A jay_tray_item_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct JayTrayItemV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn JayTrayItemV1Handler>,
}

struct DefaultHandler;

impl JayTrayItemV1Handler for DefaultHandler { }

impl ConcreteObject for JayTrayItemV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::JayTrayItemV1;
    const INTERFACE_NAME: &str = "jay_tray_item_v1";
}

impl JayTrayItemV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl JayTrayItemV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn JayTrayItemV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for JayTrayItemV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JayTrayItemV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl JayTrayItemV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy this object
    ///
    /// Destroy this object. The item is immediately removed from the tray.
    ///
    /// The client must destroy all popups before this. Otherwise the has_popups
    /// error is emitted.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= jay_tray_item_v1#{}.destroy()\n", id);
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
            0,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy this object
    ///
    /// Destroy this object. The item is immediately removed from the tray.
    ///
    /// The client must destroy all popups before this. Otherwise the has_popups
    /// error is emitted.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("jay_tray_item_v1.destroy", &e);
        }
    }

    /// Since when the ack_configure message is available.
    pub const MSG__ACK_CONFIGURE__SINCE: u32 = 1;

    /// ack a configuration sequence
    ///
    /// Ack a configuration sequence. The acked configuration sequence is
    /// double-buffered state, see wl_surface.commit. If the compositor has
    /// never sent this serial, an invalid_configure_serial error is emitted.
    ///
    /// # Arguments
    ///
    /// - `serial`: the serial
    #[inline]
    pub fn try_send_ack_configure(
        &self,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            serial,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= jay_tray_item_v1#{}.ack_configure(serial: {})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            arg0,
        ]);
        Ok(())
    }

    /// ack a configuration sequence
    ///
    /// Ack a configuration sequence. The acked configuration sequence is
    /// double-buffered state, see wl_surface.commit. If the compositor has
    /// never sent this serial, an invalid_configure_serial error is emitted.
    ///
    /// # Arguments
    ///
    /// - `serial`: the serial
    #[inline]
    pub fn send_ack_configure(
        &self,
        serial: u32,
    ) {
        let res = self.try_send_ack_configure(
            serial,
        );
        if let Err(e) = res {
            log_send("jay_tray_item_v1.ack_configure", &e);
        }
    }

    /// Since when the get_popup message is available.
    pub const MSG__GET_POPUP__SINCE: u32 = 1;

    /// create a popup for tray item
    ///
    /// Create a popup for a tray item.
    ///
    /// The popup should have been created with a null parent. If the popup
    /// already has a parent, the has_parent error is emitted.
    ///
    /// The seat and serial indicate the interaction that causes this popup to
    /// be shown. If the compositor has never sent this serial, the compositor
    /// might emit the invalid_seat_serial error. This is compositor policy.
    /// If the focus_hint is invalid, the invalid_keyboard_focus_hint error is
    /// emitted.
    ///
    /// The focus hint indicates how the client wants keyboard focus to be
    /// handled for the popup. The compositor may ignore the hint. This hint has
    /// no effect on nested popups.
    ///
    /// The compositor may dismiss the popup at any point.
    ///
    /// # Arguments
    ///
    /// - `popup`: the popup to be shown
    /// - `seat`: the causal seat
    /// - `serial`: the causal input serial
    /// - `focus_hint`: a hint for keyboard focus handling
    #[inline]
    pub fn try_send_get_popup(
        &self,
        popup: &Rc<XdgPopup>,
        seat: &Rc<WlSeat>,
        serial: u32,
        focus_hint: JayTrayItemV1KeyboardFocusHint,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            popup,
            seat,
            serial,
            focus_hint,
        );
        let arg0 = arg0.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("popup"))),
            Some(id) => id,
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("seat"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: JayTrayItemV1KeyboardFocusHint) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= jay_tray_item_v1#{}.get_popup(popup: xdg_popup#{}, seat: wl_seat#{}, serial: {}, focus_hint: {:?})\n", id, arg0, arg1, arg2, arg3);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id, arg2, arg3);
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
            arg1_id,
            arg2,
            arg3.0,
        ]);
        Ok(())
    }

    /// create a popup for tray item
    ///
    /// Create a popup for a tray item.
    ///
    /// The popup should have been created with a null parent. If the popup
    /// already has a parent, the has_parent error is emitted.
    ///
    /// The seat and serial indicate the interaction that causes this popup to
    /// be shown. If the compositor has never sent this serial, the compositor
    /// might emit the invalid_seat_serial error. This is compositor policy.
    /// If the focus_hint is invalid, the invalid_keyboard_focus_hint error is
    /// emitted.
    ///
    /// The focus hint indicates how the client wants keyboard focus to be
    /// handled for the popup. The compositor may ignore the hint. This hint has
    /// no effect on nested popups.
    ///
    /// The compositor may dismiss the popup at any point.
    ///
    /// # Arguments
    ///
    /// - `popup`: the popup to be shown
    /// - `seat`: the causal seat
    /// - `serial`: the causal input serial
    /// - `focus_hint`: a hint for keyboard focus handling
    #[inline]
    pub fn send_get_popup(
        &self,
        popup: &Rc<XdgPopup>,
        seat: &Rc<WlSeat>,
        serial: u32,
        focus_hint: JayTrayItemV1KeyboardFocusHint,
    ) {
        let res = self.try_send_get_popup(
            popup,
            seat,
            serial,
            focus_hint,
        );
        if let Err(e) = res {
            log_send("jay_tray_item_v1.get_popup", &e);
        }
    }

    /// Since when the configure_size message is available.
    pub const MSG__CONFIGURE_SIZE__SINCE: u32 = 1;

    /// optimal tray item size has changed
    ///
    /// This event is sent when the optimal size for the item has changed.
    /// This event is part of a configuration sequence that is terminated with
    /// a configure event. The client should not act on it immediately but wait
    /// for the configure event.
    ///
    /// When the client receives this event, it should reconfigure the surface
    /// for the new size, ack the sequence, and commit the surface.
    ///
    /// If the surface has a different size, the compositor might crop or
    /// stretch the surface. If the surface has subsurfaces that extend beyond
    /// the edges of the surface, the compositor might crop them.
    ///
    /// The width and height are at least 1.
    ///
    /// If a configuration sequence does not contain this event, the client
    /// should assume that the value is unchanged. The first configuration
    /// sequence must contain this event.
    ///
    /// # Arguments
    ///
    /// - `width`: the optimal width in surface coordinates
    /// - `height`: the optimal height in surface coordinates
    #[inline]
    pub fn try_send_configure_size(
        &self,
        width: i32,
        height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            width,
            height,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= jay_tray_item_v1#{}.configure_size(width: {}, height: {})\n", client_id, id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1);
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
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// optimal tray item size has changed
    ///
    /// This event is sent when the optimal size for the item has changed.
    /// This event is part of a configuration sequence that is terminated with
    /// a configure event. The client should not act on it immediately but wait
    /// for the configure event.
    ///
    /// When the client receives this event, it should reconfigure the surface
    /// for the new size, ack the sequence, and commit the surface.
    ///
    /// If the surface has a different size, the compositor might crop or
    /// stretch the surface. If the surface has subsurfaces that extend beyond
    /// the edges of the surface, the compositor might crop them.
    ///
    /// The width and height are at least 1.
    ///
    /// If a configuration sequence does not contain this event, the client
    /// should assume that the value is unchanged. The first configuration
    /// sequence must contain this event.
    ///
    /// # Arguments
    ///
    /// - `width`: the optimal width in surface coordinates
    /// - `height`: the optimal height in surface coordinates
    #[inline]
    pub fn send_configure_size(
        &self,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_configure_size(
            width,
            height,
        );
        if let Err(e) = res {
            log_send("jay_tray_item_v1.configure_size", &e);
        }
    }

    /// Since when the preferred_anchor message is available.
    pub const MSG__PREFERRED_ANCHOR__SINCE: u32 = 1;

    /// preferred anchor has changed
    ///
    /// This events is sent when the preferred anchor for popup windows changes.
    /// This event is part of a configuration sequence that is terminated with
    /// a configure event. The client should not act on it immediately but wait
    /// for the configure event.
    ///
    /// If a configuration sequence does not contain this event, the client
    /// should assume that the value is unchanged. The first configuration
    /// sequence must contain this event.
    ///
    /// # Arguments
    ///
    /// - `anchor`: the preferred anchor
    #[inline]
    pub fn try_send_preferred_anchor(
        &self,
        anchor: XdgPositionerAnchor,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            anchor,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: XdgPositionerAnchor) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= jay_tray_item_v1#{}.preferred_anchor(anchor: {:?})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// preferred anchor has changed
    ///
    /// This events is sent when the preferred anchor for popup windows changes.
    /// This event is part of a configuration sequence that is terminated with
    /// a configure event. The client should not act on it immediately but wait
    /// for the configure event.
    ///
    /// If a configuration sequence does not contain this event, the client
    /// should assume that the value is unchanged. The first configuration
    /// sequence must contain this event.
    ///
    /// # Arguments
    ///
    /// - `anchor`: the preferred anchor
    #[inline]
    pub fn send_preferred_anchor(
        &self,
        anchor: XdgPositionerAnchor,
    ) {
        let res = self.try_send_preferred_anchor(
            anchor,
        );
        if let Err(e) = res {
            log_send("jay_tray_item_v1.preferred_anchor", &e);
        }
    }

    /// Since when the preferred_gravity message is available.
    pub const MSG__PREFERRED_GRAVITY__SINCE: u32 = 1;

    /// preferred gravity has changed
    ///
    /// This events is sent when the preferred gravity for popup windows changes.
    /// This event is part of a configuration sequence that is terminated with
    /// a configure event. The client should not act on it immediately but wait
    /// for the configure event.
    ///
    /// If a configuration sequence does not contain this event, the client
    /// should assume that the value is unchanged. The first configuration
    /// sequence must contain this event.
    ///
    /// # Arguments
    ///
    /// - `gravity`: the preferred gravity
    #[inline]
    pub fn try_send_preferred_gravity(
        &self,
        gravity: XdgPositionerGravity,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            gravity,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: XdgPositionerGravity) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= jay_tray_item_v1#{}.preferred_gravity(gravity: {:?})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// preferred gravity has changed
    ///
    /// This events is sent when the preferred gravity for popup windows changes.
    /// This event is part of a configuration sequence that is terminated with
    /// a configure event. The client should not act on it immediately but wait
    /// for the configure event.
    ///
    /// If a configuration sequence does not contain this event, the client
    /// should assume that the value is unchanged. The first configuration
    /// sequence must contain this event.
    ///
    /// # Arguments
    ///
    /// - `gravity`: the preferred gravity
    #[inline]
    pub fn send_preferred_gravity(
        &self,
        gravity: XdgPositionerGravity,
    ) {
        let res = self.try_send_preferred_gravity(
            gravity,
        );
        if let Err(e) = res {
            log_send("jay_tray_item_v1.preferred_gravity", &e);
        }
    }

    /// Since when the configure message is available.
    pub const MSG__CONFIGURE__SINCE: u32 = 1;

    /// marks the end of a configuration sequence
    ///
    /// This event marks the end of a configuration sequence. The client should
    /// act on the new parameters, ack the sequence, and commit the surface.
    ///
    /// Note that this serial is not related to the wl_seat serial used in
    /// get_popup requests.
    ///
    /// # Arguments
    ///
    /// - `serial`: the serial
    #[inline]
    pub fn try_send_configure(
        &self,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            serial,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= jay_tray_item_v1#{}.configure(serial: {})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
        ]);
        Ok(())
    }

    /// marks the end of a configuration sequence
    ///
    /// This event marks the end of a configuration sequence. The client should
    /// act on the new parameters, ack the sequence, and commit the surface.
    ///
    /// Note that this serial is not related to the wl_seat serial used in
    /// get_popup requests.
    ///
    /// # Arguments
    ///
    /// - `serial`: the serial
    #[inline]
    pub fn send_configure(
        &self,
        serial: u32,
    ) {
        let res = self.try_send_configure(
            serial,
        );
        if let Err(e) = res {
            log_send("jay_tray_item_v1.configure", &e);
        }
    }
}

/// A message handler for [`JayTrayItemV1`] proxies.
pub trait JayTrayItemV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<JayTrayItemV1>) {
        slf.core.delete_id();
    }

    /// destroy this object
    ///
    /// Destroy this object. The item is immediately removed from the tray.
    ///
    /// The client must destroy all popups before this. Otherwise the has_popups
    /// error is emitted.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<JayTrayItemV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("jay_tray_item_v1.destroy", &e);
        }
    }

    /// ack a configuration sequence
    ///
    /// Ack a configuration sequence. The acked configuration sequence is
    /// double-buffered state, see wl_surface.commit. If the compositor has
    /// never sent this serial, an invalid_configure_serial error is emitted.
    ///
    /// # Arguments
    ///
    /// - `serial`: the serial
    #[inline]
    fn handle_ack_configure(
        &mut self,
        slf: &Rc<JayTrayItemV1>,
        serial: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_ack_configure(
            serial,
        );
        if let Err(e) = res {
            log_forward("jay_tray_item_v1.ack_configure", &e);
        }
    }

    /// create a popup for tray item
    ///
    /// Create a popup for a tray item.
    ///
    /// The popup should have been created with a null parent. If the popup
    /// already has a parent, the has_parent error is emitted.
    ///
    /// The seat and serial indicate the interaction that causes this popup to
    /// be shown. If the compositor has never sent this serial, the compositor
    /// might emit the invalid_seat_serial error. This is compositor policy.
    /// If the focus_hint is invalid, the invalid_keyboard_focus_hint error is
    /// emitted.
    ///
    /// The focus hint indicates how the client wants keyboard focus to be
    /// handled for the popup. The compositor may ignore the hint. This hint has
    /// no effect on nested popups.
    ///
    /// The compositor may dismiss the popup at any point.
    ///
    /// # Arguments
    ///
    /// - `popup`: the popup to be shown
    /// - `seat`: the causal seat
    /// - `serial`: the causal input serial
    /// - `focus_hint`: a hint for keyboard focus handling
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_popup(
        &mut self,
        slf: &Rc<JayTrayItemV1>,
        popup: &Rc<XdgPopup>,
        seat: &Rc<WlSeat>,
        serial: u32,
        focus_hint: JayTrayItemV1KeyboardFocusHint,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_popup(
            popup,
            seat,
            serial,
            focus_hint,
        );
        if let Err(e) = res {
            log_forward("jay_tray_item_v1.get_popup", &e);
        }
    }

    /// optimal tray item size has changed
    ///
    /// This event is sent when the optimal size for the item has changed.
    /// This event is part of a configuration sequence that is terminated with
    /// a configure event. The client should not act on it immediately but wait
    /// for the configure event.
    ///
    /// When the client receives this event, it should reconfigure the surface
    /// for the new size, ack the sequence, and commit the surface.
    ///
    /// If the surface has a different size, the compositor might crop or
    /// stretch the surface. If the surface has subsurfaces that extend beyond
    /// the edges of the surface, the compositor might crop them.
    ///
    /// The width and height are at least 1.
    ///
    /// If a configuration sequence does not contain this event, the client
    /// should assume that the value is unchanged. The first configuration
    /// sequence must contain this event.
    ///
    /// # Arguments
    ///
    /// - `width`: the optimal width in surface coordinates
    /// - `height`: the optimal height in surface coordinates
    #[inline]
    fn handle_configure_size(
        &mut self,
        slf: &Rc<JayTrayItemV1>,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_configure_size(
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("jay_tray_item_v1.configure_size", &e);
        }
    }

    /// preferred anchor has changed
    ///
    /// This events is sent when the preferred anchor for popup windows changes.
    /// This event is part of a configuration sequence that is terminated with
    /// a configure event. The client should not act on it immediately but wait
    /// for the configure event.
    ///
    /// If a configuration sequence does not contain this event, the client
    /// should assume that the value is unchanged. The first configuration
    /// sequence must contain this event.
    ///
    /// # Arguments
    ///
    /// - `anchor`: the preferred anchor
    #[inline]
    fn handle_preferred_anchor(
        &mut self,
        slf: &Rc<JayTrayItemV1>,
        anchor: XdgPositionerAnchor,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_preferred_anchor(
            anchor,
        );
        if let Err(e) = res {
            log_forward("jay_tray_item_v1.preferred_anchor", &e);
        }
    }

    /// preferred gravity has changed
    ///
    /// This events is sent when the preferred gravity for popup windows changes.
    /// This event is part of a configuration sequence that is terminated with
    /// a configure event. The client should not act on it immediately but wait
    /// for the configure event.
    ///
    /// If a configuration sequence does not contain this event, the client
    /// should assume that the value is unchanged. The first configuration
    /// sequence must contain this event.
    ///
    /// # Arguments
    ///
    /// - `gravity`: the preferred gravity
    #[inline]
    fn handle_preferred_gravity(
        &mut self,
        slf: &Rc<JayTrayItemV1>,
        gravity: XdgPositionerGravity,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_preferred_gravity(
            gravity,
        );
        if let Err(e) = res {
            log_forward("jay_tray_item_v1.preferred_gravity", &e);
        }
    }

    /// marks the end of a configuration sequence
    ///
    /// This event marks the end of a configuration sequence. The client should
    /// act on the new parameters, ack the sequence, and commit the surface.
    ///
    /// Note that this serial is not related to the wl_seat serial used in
    /// get_popup requests.
    ///
    /// # Arguments
    ///
    /// - `serial`: the serial
    #[inline]
    fn handle_configure(
        &mut self,
        slf: &Rc<JayTrayItemV1>,
        serial: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_configure(
            serial,
        );
        if let Err(e) = res {
            log_forward("jay_tray_item_v1.configure", &e);
        }
    }
}

impl ObjectPrivate for JayTrayItemV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::JayTrayItemV1, version),
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
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> jay_tray_item_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> jay_tray_item_v1#{}.ack_configure(serial: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_ack_configure(&self, arg0);
                } else {
                    DefaultHandler.handle_ack_configure(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 24)));
                };
                let arg3 = JayTrayItemV1KeyboardFocusHint(arg3);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: JayTrayItemV1KeyboardFocusHint) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> jay_tray_item_v1#{}.get_popup(popup: xdg_popup#{}, seat: wl_seat#{}, serial: {}, focus_hint: {:?})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<XdgPopup>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("popup", o.core().interface, ObjectInterface::XdgPopup)));
                };
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("seat", o.core().interface, ObjectInterface::WlSeat)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_popup(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_get_popup(&self, arg0, arg1, arg2, arg3);
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
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> jay_tray_item_v1#{}.configure_size(width: {}, height: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_configure_size(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_configure_size(&self, arg0, arg1);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = XdgPositionerAnchor(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: XdgPositionerAnchor) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> jay_tray_item_v1#{}.preferred_anchor(anchor: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_preferred_anchor(&self, arg0);
                } else {
                    DefaultHandler.handle_preferred_anchor(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = XdgPositionerGravity(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: XdgPositionerGravity) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> jay_tray_item_v1#{}.preferred_gravity(gravity: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_preferred_gravity(&self, arg0);
                } else {
                    DefaultHandler.handle_preferred_gravity(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> jay_tray_item_v1#{}.configure(serial: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_configure(&self, arg0);
                } else {
                    DefaultHandler.handle_configure(&self, arg0);
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
            0 => "destroy",
            1 => "ack_configure",
            2 => "get_popup",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "configure_size",
            1 => "preferred_anchor",
            2 => "preferred_gravity",
            3 => "configure",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for JayTrayItemV1 {
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

impl JayTrayItemV1 {
    /// Since when the error.has_popups enum variant is available.
    pub const ENM__ERROR_HAS_POPUPS__SINCE: u32 = 1;
    /// Since when the error.has_parent enum variant is available.
    pub const ENM__ERROR_HAS_PARENT__SINCE: u32 = 1;
    /// Since when the error.invalid_seat_serial enum variant is available.
    pub const ENM__ERROR_INVALID_SEAT_SERIAL__SINCE: u32 = 1;
    /// Since when the error.invalid_configure_serial enum variant is available.
    pub const ENM__ERROR_INVALID_CONFIGURE_SERIAL__SINCE: u32 = 1;
    /// Since when the error.invalid_keyboard_focus_hint enum variant is available.
    pub const ENM__ERROR_INVALID_KEYBOARD_FOCUS_HINT__SINCE: u32 = 1;

    /// Since when the keyboard_focus_hint.none enum variant is available.
    pub const ENM__KEYBOARD_FOCUS_HINT_NONE__SINCE: u32 = 1;
    /// Since when the keyboard_focus_hint.on_demand enum variant is available.
    pub const ENM__KEYBOARD_FOCUS_HINT_ON_DEMAND__SINCE: u32 = 1;
    /// Since when the keyboard_focus_hint.immediate enum variant is available.
    pub const ENM__KEYBOARD_FOCUS_HINT_IMMEDIATE__SINCE: u32 = 1;
}

/// fatal error
///
/// These fatal protocol errors may be emitted in response to
/// invalid requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct JayTrayItemV1Error(pub u32);

impl JayTrayItemV1Error {
    /// the item has popups at destroy time
    pub const HAS_POPUPS: Self = Self(0);

    /// the popup already has a parent
    pub const HAS_PARENT: Self = Self(1);

    /// invalid serial provided to get_popup
    pub const INVALID_SEAT_SERIAL: Self = Self(2);

    /// invalid serial provided to ack_configure
    pub const INVALID_CONFIGURE_SERIAL: Self = Self(3);

    /// invalid keyboard focus hint provided to get_popup
    pub const INVALID_KEYBOARD_FOCUS_HINT: Self = Self(4);
}

impl Debug for JayTrayItemV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::HAS_POPUPS => "HAS_POPUPS",
            Self::HAS_PARENT => "HAS_PARENT",
            Self::INVALID_SEAT_SERIAL => "INVALID_SEAT_SERIAL",
            Self::INVALID_CONFIGURE_SERIAL => "INVALID_CONFIGURE_SERIAL",
            Self::INVALID_KEYBOARD_FOCUS_HINT => "INVALID_KEYBOARD_FOCUS_HINT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// keyboard focus hint
///
/// This enum describes when a popup used in the get_popup request should
/// get keyboard focus.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct JayTrayItemV1KeyboardFocusHint(pub u32);

impl JayTrayItemV1KeyboardFocusHint {
    /// no keyboard focus
    ///
    /// The popup should never get the keyboard focus.
    pub const NONE: Self = Self(0);

    /// on demand keyboard focus
    ///
    /// The popup should get the keyboard focus when the user requests it.
    pub const ON_DEMAND: Self = Self(1);

    /// immediate keyboard focus
    ///
    /// The popup should get the keyboard focus as soon as it becomes visible.
    pub const IMMEDIATE: Self = Self(2);
}

impl Debug for JayTrayItemV1KeyboardFocusHint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NONE => "NONE",
            Self::ON_DEMAND => "ON_DEMAND",
            Self::IMMEDIATE => "IMMEDIATE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
