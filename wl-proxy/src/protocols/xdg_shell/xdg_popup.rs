//! short-lived, popup surfaces for menus
//!
//! A popup surface is a short-lived, temporary surface. It can be used to
//! implement for example menus, popovers, tooltips and other similar user
//! interface concepts.
//!
//! A popup can be made to take an explicit grab. See xdg_popup.grab for
//! details.
//!
//! When the popup is dismissed, a popup_done event will be sent out, and at
//! the same time the surface will be unmapped. See the xdg_popup.popup_done
//! event for details.
//!
//! Explicitly destroying the xdg_popup object will also dismiss the popup and
//! unmap the surface. Clients that want to dismiss the popup when another
//! surface of their own is clicked should dismiss the popup using the destroy
//! request.
//!
//! A newly created xdg_popup will be stacked on top of all previously created
//! xdg_popup surfaces associated with the same xdg_toplevel.
//!
//! The parent of an xdg_popup must be mapped (see the xdg_surface
//! description) before the xdg_popup itself.
//!
//! The client must call wl_surface.commit on the corresponding wl_surface
//! for the xdg_popup state to take effect.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A xdg_popup object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XdgPopup {
    core: ObjectCore,
    handler: HandlerHolder<dyn XdgPopupHandler>,
}

struct DefaultHandler;

impl XdgPopupHandler for DefaultHandler { }

impl ConcreteObject for XdgPopup {
    const XML_VERSION: u32 = 7;
    const INTERFACE: ObjectInterface = ObjectInterface::XdgPopup;
    const INTERFACE_NAME: &str = "xdg_popup";
}

impl XdgPopup {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl XdgPopupHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn XdgPopupHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XdgPopup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XdgPopup")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XdgPopup {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// remove xdg_popup interface
    ///
    /// This destroys the popup. Explicitly destroying the xdg_popup
    /// object will also dismiss the popup, and unmap the surface.
    ///
    /// If this xdg_popup is not the "topmost" popup, the
    /// xdg_wm_base.not_the_topmost_popup protocol error will be sent.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_popup#{}.destroy()\n", id);
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

    /// remove xdg_popup interface
    ///
    /// This destroys the popup. Explicitly destroying the xdg_popup
    /// object will also dismiss the popup, and unmap the surface.
    ///
    /// If this xdg_popup is not the "topmost" popup, the
    /// xdg_wm_base.not_the_topmost_popup protocol error will be sent.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("xdg_popup.destroy", &e);
        }
    }

    /// Since when the grab message is available.
    pub const MSG__GRAB__SINCE: u32 = 1;

    /// make the popup take an explicit grab
    ///
    /// This request makes the created popup take an explicit grab. An explicit
    /// grab will be dismissed when the user dismisses the popup, or when the
    /// client destroys the xdg_popup. This can be done by the user clicking
    /// outside the surface, using the keyboard, or even locking the screen
    /// through closing the lid or a timeout.
    ///
    /// If the compositor denies the grab, the popup will be immediately
    /// dismissed.
    ///
    /// This request must be used in response to some sort of user action like a
    /// button press, key press, or touch down event. The serial number of the
    /// event should be passed as 'serial'.
    ///
    /// The parent of a grabbing popup must either be an xdg_toplevel surface or
    /// another xdg_popup with an explicit grab. If the parent is another
    /// xdg_popup it means that the popups are nested, with this popup now being
    /// the topmost popup.
    ///
    /// Nested popups must be destroyed in the reverse order they were created
    /// in, e.g. the only popup you are allowed to destroy at all times is the
    /// topmost one.
    ///
    /// When compositors choose to dismiss a popup, they may dismiss every
    /// nested grabbing popup as well. When a compositor dismisses popups, it
    /// will follow the same dismissing order as required from the client.
    ///
    /// If the topmost grabbing popup is destroyed, the grab will be returned to
    /// the parent of the popup, if that parent previously had an explicit grab.
    ///
    /// If the parent is a grabbing popup which has already been dismissed, this
    /// popup will be immediately dismissed. If the parent is a popup that did
    /// not take an explicit grab, an error will be raised.
    ///
    /// During a popup grab, the client owning the grab will receive pointer
    /// and touch events for all their surfaces as normal (similar to an
    /// "owner-events" grab in X11 parlance), while the top most grabbing popup
    /// will always have keyboard focus.
    ///
    /// # Arguments
    ///
    /// - `seat`: the wl_seat of the user event
    /// - `serial`: the serial of the user event
    #[inline]
    pub fn try_send_grab(
        &self,
        seat: &Rc<WlSeat>,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            seat,
            serial,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("seat"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_popup#{}.grab(seat: wl_seat#{}, serial: {})\n", id, arg0, arg1);
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

    /// make the popup take an explicit grab
    ///
    /// This request makes the created popup take an explicit grab. An explicit
    /// grab will be dismissed when the user dismisses the popup, or when the
    /// client destroys the xdg_popup. This can be done by the user clicking
    /// outside the surface, using the keyboard, or even locking the screen
    /// through closing the lid or a timeout.
    ///
    /// If the compositor denies the grab, the popup will be immediately
    /// dismissed.
    ///
    /// This request must be used in response to some sort of user action like a
    /// button press, key press, or touch down event. The serial number of the
    /// event should be passed as 'serial'.
    ///
    /// The parent of a grabbing popup must either be an xdg_toplevel surface or
    /// another xdg_popup with an explicit grab. If the parent is another
    /// xdg_popup it means that the popups are nested, with this popup now being
    /// the topmost popup.
    ///
    /// Nested popups must be destroyed in the reverse order they were created
    /// in, e.g. the only popup you are allowed to destroy at all times is the
    /// topmost one.
    ///
    /// When compositors choose to dismiss a popup, they may dismiss every
    /// nested grabbing popup as well. When a compositor dismisses popups, it
    /// will follow the same dismissing order as required from the client.
    ///
    /// If the topmost grabbing popup is destroyed, the grab will be returned to
    /// the parent of the popup, if that parent previously had an explicit grab.
    ///
    /// If the parent is a grabbing popup which has already been dismissed, this
    /// popup will be immediately dismissed. If the parent is a popup that did
    /// not take an explicit grab, an error will be raised.
    ///
    /// During a popup grab, the client owning the grab will receive pointer
    /// and touch events for all their surfaces as normal (similar to an
    /// "owner-events" grab in X11 parlance), while the top most grabbing popup
    /// will always have keyboard focus.
    ///
    /// # Arguments
    ///
    /// - `seat`: the wl_seat of the user event
    /// - `serial`: the serial of the user event
    #[inline]
    pub fn send_grab(
        &self,
        seat: &Rc<WlSeat>,
        serial: u32,
    ) {
        let res = self.try_send_grab(
            seat,
            serial,
        );
        if let Err(e) = res {
            log_send("xdg_popup.grab", &e);
        }
    }

    /// Since when the configure message is available.
    pub const MSG__CONFIGURE__SINCE: u32 = 1;

    /// configure the popup surface
    ///
    /// This event asks the popup surface to configure itself given the
    /// configuration. The configured state should not be applied immediately.
    /// See xdg_surface.configure for details.
    ///
    /// The x and y arguments represent the position the popup was placed at
    /// given the xdg_positioner rule, relative to the upper left corner of the
    /// window geometry of the parent surface.
    ///
    /// For version 2 or older, the configure event for an xdg_popup is only
    /// ever sent once for the initial configuration. Starting with version 3,
    /// it may be sent again if the popup is setup with an xdg_positioner with
    /// set_reactive requested, or in response to xdg_popup.reposition requests.
    ///
    /// # Arguments
    ///
    /// - `x`: x position relative to parent surface window geometry
    /// - `y`: y position relative to parent surface window geometry
    /// - `width`: window geometry width
    /// - `height`: window geometry height
    #[inline]
    pub fn try_send_configure(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            x,
            y,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xdg_popup#{}.configure(x: {}, y: {}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1, arg2, arg3);
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
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// configure the popup surface
    ///
    /// This event asks the popup surface to configure itself given the
    /// configuration. The configured state should not be applied immediately.
    /// See xdg_surface.configure for details.
    ///
    /// The x and y arguments represent the position the popup was placed at
    /// given the xdg_positioner rule, relative to the upper left corner of the
    /// window geometry of the parent surface.
    ///
    /// For version 2 or older, the configure event for an xdg_popup is only
    /// ever sent once for the initial configuration. Starting with version 3,
    /// it may be sent again if the popup is setup with an xdg_positioner with
    /// set_reactive requested, or in response to xdg_popup.reposition requests.
    ///
    /// # Arguments
    ///
    /// - `x`: x position relative to parent surface window geometry
    /// - `y`: y position relative to parent surface window geometry
    /// - `width`: window geometry width
    /// - `height`: window geometry height
    #[inline]
    pub fn send_configure(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_configure(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_send("xdg_popup.configure", &e);
        }
    }

    /// Since when the popup_done message is available.
    pub const MSG__POPUP_DONE__SINCE: u32 = 1;

    /// popup interaction is done
    ///
    /// The popup_done event is sent out when a popup is dismissed by the
    /// compositor. The client should destroy the xdg_popup object at this
    /// point.
    #[inline]
    pub fn try_send_popup_done(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xdg_popup#{}.popup_done()\n", client_id, id);
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
            1,
        ]);
        Ok(())
    }

    /// popup interaction is done
    ///
    /// The popup_done event is sent out when a popup is dismissed by the
    /// compositor. The client should destroy the xdg_popup object at this
    /// point.
    #[inline]
    pub fn send_popup_done(
        &self,
    ) {
        let res = self.try_send_popup_done(
        );
        if let Err(e) = res {
            log_send("xdg_popup.popup_done", &e);
        }
    }

    /// Since when the reposition message is available.
    pub const MSG__REPOSITION__SINCE: u32 = 3;

    /// recalculate the popup's location
    ///
    /// Reposition an already-mapped popup. The popup will be placed given the
    /// details in the passed xdg_positioner object, and a
    /// xdg_popup.repositioned followed by xdg_popup.configure and
    /// xdg_surface.configure will be emitted in response. Any parameters set
    /// by the previous positioner will be discarded.
    ///
    /// The passed token will be sent in the corresponding
    /// xdg_popup.repositioned event. The new popup position will not take
    /// effect until the corresponding configure event is acknowledged by the
    /// client. See xdg_popup.repositioned for details. The token itself is
    /// opaque, and has no other special meaning.
    ///
    /// If multiple reposition requests are sent, the compositor may skip all
    /// but the last one.
    ///
    /// If the popup is repositioned in response to a configure event for its
    /// parent, the client should send an xdg_positioner.set_parent_configure
    /// and possibly an xdg_positioner.set_parent_size request to allow the
    /// compositor to properly constrain the popup.
    ///
    /// If the popup is repositioned together with a parent that is being
    /// resized, but not in response to a configure event, the client should
    /// send an xdg_positioner.set_parent_size request.
    ///
    /// # Arguments
    ///
    /// - `positioner`:
    /// - `token`: reposition request token
    #[inline]
    pub fn try_send_reposition(
        &self,
        positioner: &Rc<XdgPositioner>,
        token: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            positioner,
            token,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("positioner"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_popup#{}.reposition(positioner: xdg_positioner#{}, token: {})\n", id, arg0, arg1);
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
            2,
            arg0_id,
            arg1,
        ]);
        Ok(())
    }

    /// recalculate the popup's location
    ///
    /// Reposition an already-mapped popup. The popup will be placed given the
    /// details in the passed xdg_positioner object, and a
    /// xdg_popup.repositioned followed by xdg_popup.configure and
    /// xdg_surface.configure will be emitted in response. Any parameters set
    /// by the previous positioner will be discarded.
    ///
    /// The passed token will be sent in the corresponding
    /// xdg_popup.repositioned event. The new popup position will not take
    /// effect until the corresponding configure event is acknowledged by the
    /// client. See xdg_popup.repositioned for details. The token itself is
    /// opaque, and has no other special meaning.
    ///
    /// If multiple reposition requests are sent, the compositor may skip all
    /// but the last one.
    ///
    /// If the popup is repositioned in response to a configure event for its
    /// parent, the client should send an xdg_positioner.set_parent_configure
    /// and possibly an xdg_positioner.set_parent_size request to allow the
    /// compositor to properly constrain the popup.
    ///
    /// If the popup is repositioned together with a parent that is being
    /// resized, but not in response to a configure event, the client should
    /// send an xdg_positioner.set_parent_size request.
    ///
    /// # Arguments
    ///
    /// - `positioner`:
    /// - `token`: reposition request token
    #[inline]
    pub fn send_reposition(
        &self,
        positioner: &Rc<XdgPositioner>,
        token: u32,
    ) {
        let res = self.try_send_reposition(
            positioner,
            token,
        );
        if let Err(e) = res {
            log_send("xdg_popup.reposition", &e);
        }
    }

    /// Since when the repositioned message is available.
    pub const MSG__REPOSITIONED__SINCE: u32 = 3;

    /// signal the completion of a repositioned request
    ///
    /// The repositioned event is sent as part of a popup configuration
    /// sequence, together with xdg_popup.configure and lastly
    /// xdg_surface.configure to notify the completion of a reposition request.
    ///
    /// The repositioned event is to notify about the completion of a
    /// xdg_popup.reposition request. The token argument is the token passed
    /// in the xdg_popup.reposition request.
    ///
    /// Immediately after this event is emitted, xdg_popup.configure and
    /// xdg_surface.configure will be sent with the updated size and position,
    /// as well as a new configure serial.
    ///
    /// The client should optionally update the content of the popup, but must
    /// acknowledge the new popup configuration for the new position to take
    /// effect. See xdg_surface.ack_configure for details.
    ///
    /// # Arguments
    ///
    /// - `token`: reposition request token
    #[inline]
    pub fn try_send_repositioned(
        &self,
        token: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            token,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xdg_popup#{}.repositioned(token: {})\n", client_id, id, arg0);
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
            arg0,
        ]);
        Ok(())
    }

    /// signal the completion of a repositioned request
    ///
    /// The repositioned event is sent as part of a popup configuration
    /// sequence, together with xdg_popup.configure and lastly
    /// xdg_surface.configure to notify the completion of a reposition request.
    ///
    /// The repositioned event is to notify about the completion of a
    /// xdg_popup.reposition request. The token argument is the token passed
    /// in the xdg_popup.reposition request.
    ///
    /// Immediately after this event is emitted, xdg_popup.configure and
    /// xdg_surface.configure will be sent with the updated size and position,
    /// as well as a new configure serial.
    ///
    /// The client should optionally update the content of the popup, but must
    /// acknowledge the new popup configuration for the new position to take
    /// effect. See xdg_surface.ack_configure for details.
    ///
    /// # Arguments
    ///
    /// - `token`: reposition request token
    #[inline]
    pub fn send_repositioned(
        &self,
        token: u32,
    ) {
        let res = self.try_send_repositioned(
            token,
        );
        if let Err(e) = res {
            log_send("xdg_popup.repositioned", &e);
        }
    }
}

/// A message handler for [`XdgPopup`] proxies.
pub trait XdgPopupHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<XdgPopup>) {
        slf.core.delete_id();
    }

    /// remove xdg_popup interface
    ///
    /// This destroys the popup. Explicitly destroying the xdg_popup
    /// object will also dismiss the popup, and unmap the surface.
    ///
    /// If this xdg_popup is not the "topmost" popup, the
    /// xdg_wm_base.not_the_topmost_popup protocol error will be sent.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<XdgPopup>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("xdg_popup.destroy", &e);
        }
    }

    /// make the popup take an explicit grab
    ///
    /// This request makes the created popup take an explicit grab. An explicit
    /// grab will be dismissed when the user dismisses the popup, or when the
    /// client destroys the xdg_popup. This can be done by the user clicking
    /// outside the surface, using the keyboard, or even locking the screen
    /// through closing the lid or a timeout.
    ///
    /// If the compositor denies the grab, the popup will be immediately
    /// dismissed.
    ///
    /// This request must be used in response to some sort of user action like a
    /// button press, key press, or touch down event. The serial number of the
    /// event should be passed as 'serial'.
    ///
    /// The parent of a grabbing popup must either be an xdg_toplevel surface or
    /// another xdg_popup with an explicit grab. If the parent is another
    /// xdg_popup it means that the popups are nested, with this popup now being
    /// the topmost popup.
    ///
    /// Nested popups must be destroyed in the reverse order they were created
    /// in, e.g. the only popup you are allowed to destroy at all times is the
    /// topmost one.
    ///
    /// When compositors choose to dismiss a popup, they may dismiss every
    /// nested grabbing popup as well. When a compositor dismisses popups, it
    /// will follow the same dismissing order as required from the client.
    ///
    /// If the topmost grabbing popup is destroyed, the grab will be returned to
    /// the parent of the popup, if that parent previously had an explicit grab.
    ///
    /// If the parent is a grabbing popup which has already been dismissed, this
    /// popup will be immediately dismissed. If the parent is a popup that did
    /// not take an explicit grab, an error will be raised.
    ///
    /// During a popup grab, the client owning the grab will receive pointer
    /// and touch events for all their surfaces as normal (similar to an
    /// "owner-events" grab in X11 parlance), while the top most grabbing popup
    /// will always have keyboard focus.
    ///
    /// # Arguments
    ///
    /// - `seat`: the wl_seat of the user event
    /// - `serial`: the serial of the user event
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_grab(
        &mut self,
        slf: &Rc<XdgPopup>,
        seat: &Rc<WlSeat>,
        serial: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_grab(
            seat,
            serial,
        );
        if let Err(e) = res {
            log_forward("xdg_popup.grab", &e);
        }
    }

    /// configure the popup surface
    ///
    /// This event asks the popup surface to configure itself given the
    /// configuration. The configured state should not be applied immediately.
    /// See xdg_surface.configure for details.
    ///
    /// The x and y arguments represent the position the popup was placed at
    /// given the xdg_positioner rule, relative to the upper left corner of the
    /// window geometry of the parent surface.
    ///
    /// For version 2 or older, the configure event for an xdg_popup is only
    /// ever sent once for the initial configuration. Starting with version 3,
    /// it may be sent again if the popup is setup with an xdg_positioner with
    /// set_reactive requested, or in response to xdg_popup.reposition requests.
    ///
    /// # Arguments
    ///
    /// - `x`: x position relative to parent surface window geometry
    /// - `y`: y position relative to parent surface window geometry
    /// - `width`: window geometry width
    /// - `height`: window geometry height
    #[inline]
    fn handle_configure(
        &mut self,
        slf: &Rc<XdgPopup>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_configure(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("xdg_popup.configure", &e);
        }
    }

    /// popup interaction is done
    ///
    /// The popup_done event is sent out when a popup is dismissed by the
    /// compositor. The client should destroy the xdg_popup object at this
    /// point.
    #[inline]
    fn handle_popup_done(
        &mut self,
        slf: &Rc<XdgPopup>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_popup_done(
        );
        if let Err(e) = res {
            log_forward("xdg_popup.popup_done", &e);
        }
    }

    /// recalculate the popup's location
    ///
    /// Reposition an already-mapped popup. The popup will be placed given the
    /// details in the passed xdg_positioner object, and a
    /// xdg_popup.repositioned followed by xdg_popup.configure and
    /// xdg_surface.configure will be emitted in response. Any parameters set
    /// by the previous positioner will be discarded.
    ///
    /// The passed token will be sent in the corresponding
    /// xdg_popup.repositioned event. The new popup position will not take
    /// effect until the corresponding configure event is acknowledged by the
    /// client. See xdg_popup.repositioned for details. The token itself is
    /// opaque, and has no other special meaning.
    ///
    /// If multiple reposition requests are sent, the compositor may skip all
    /// but the last one.
    ///
    /// If the popup is repositioned in response to a configure event for its
    /// parent, the client should send an xdg_positioner.set_parent_configure
    /// and possibly an xdg_positioner.set_parent_size request to allow the
    /// compositor to properly constrain the popup.
    ///
    /// If the popup is repositioned together with a parent that is being
    /// resized, but not in response to a configure event, the client should
    /// send an xdg_positioner.set_parent_size request.
    ///
    /// # Arguments
    ///
    /// - `positioner`:
    /// - `token`: reposition request token
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_reposition(
        &mut self,
        slf: &Rc<XdgPopup>,
        positioner: &Rc<XdgPositioner>,
        token: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_reposition(
            positioner,
            token,
        );
        if let Err(e) = res {
            log_forward("xdg_popup.reposition", &e);
        }
    }

    /// signal the completion of a repositioned request
    ///
    /// The repositioned event is sent as part of a popup configuration
    /// sequence, together with xdg_popup.configure and lastly
    /// xdg_surface.configure to notify the completion of a reposition request.
    ///
    /// The repositioned event is to notify about the completion of a
    /// xdg_popup.reposition request. The token argument is the token passed
    /// in the xdg_popup.reposition request.
    ///
    /// Immediately after this event is emitted, xdg_popup.configure and
    /// xdg_surface.configure will be sent with the updated size and position,
    /// as well as a new configure serial.
    ///
    /// The client should optionally update the content of the popup, but must
    /// acknowledge the new popup configuration for the new position to take
    /// effect. See xdg_surface.ack_configure for details.
    ///
    /// # Arguments
    ///
    /// - `token`: reposition request token
    #[inline]
    fn handle_repositioned(
        &mut self,
        slf: &Rc<XdgPopup>,
        token: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_repositioned(
            token,
        );
        if let Err(e) = res {
            log_forward("xdg_popup.repositioned", &e);
        }
    }
}

impl ObjectPrivate for XdgPopup {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::XdgPopup, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_popup#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_popup#{}.grab(seat: wl_seat#{}, serial: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("seat", o.core().interface, ObjectInterface::WlSeat)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_grab(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_grab(&self, arg0, arg1);
                }
            }
            2 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_popup#{}.reposition(positioner: xdg_positioner#{}, token: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<XdgPositioner>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("positioner", o.core().interface, ObjectInterface::XdgPositioner)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_reposition(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_reposition(&self, arg0, arg1);
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
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 24)));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xdg_popup#{}.configure(x: {}, y: {}, width: {}, height: {})\n", id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_configure(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_configure(&self, arg0, arg1, arg2, arg3);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xdg_popup#{}.popup_done()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_popup_done(&self);
                } else {
                    DefaultHandler.handle_popup_done(&self);
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
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xdg_popup#{}.repositioned(token: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_repositioned(&self, arg0);
                } else {
                    DefaultHandler.handle_repositioned(&self, arg0);
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
            1 => "grab",
            2 => "reposition",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "configure",
            1 => "popup_done",
            2 => "repositioned",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for XdgPopup {
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

impl XdgPopup {
    /// Since when the error.invalid_grab enum variant is available.
    pub const ENM__ERROR_INVALID_GRAB__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct XdgPopupError(pub u32);

impl XdgPopupError {
    /// tried to grab after being mapped
    pub const INVALID_GRAB: Self = Self(0);
}

impl Debug for XdgPopupError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_GRAB => "INVALID_GRAB",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
