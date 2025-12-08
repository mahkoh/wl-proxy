//! calibrator surface for a specific touch device
//!
//! On creation, this object is tied to a specific touch device. The
//! compositor sends a configure event which the client must obey with the
//! associated wl_surface.
//!
//! Once the client has committed content to the surface, the compositor can
//! grab the touch input device, prevent it from emitting normal touch
//! events, show the surface on the correct output, and relay input events
//! from the touch device via this protocol object.
//!
//! Touch events from other touch devices than the one tied to this object
//! must generate wrong_touch events on at least touch-down and must not
//! generate normal or calibration touch events.
//!
//! At any time, the compositor can choose to cancel the calibration
//! procedure by sending the cancel_calibration event. This should also be
//! used if the touch device disappears or anything else prevents the
//! calibration from continuing on the compositor side.
//!
//! If the wl_surface is destroyed, the compositor must cancel the
//! calibration.
//!
//! The touch event coordinates and conversion results are delivered in
//! calibration units. The calibration units cover the device coordinate
//! range exactly. Calibration units are in the closed interval [0.0, 1.0]
//! mapped into 32-bit unsigned integers. An integer can be converted into a
//! real value by dividing by 2^32-1. A calibration matrix must be computed
//! from the [0.0, 1.0] real values, but the matrix elements do not need to
//! fall into that range.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A weston_touch_calibrator object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WestonTouchCalibrator {
    core: ObjectCore,
    handler: HandlerHolder<dyn WestonTouchCalibratorHandler>,
}

struct DefaultHandler;

impl WestonTouchCalibratorHandler for DefaultHandler { }

impl ConcreteObject for WestonTouchCalibrator {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WestonTouchCalibrator;
    const INTERFACE_NAME: &str = "weston_touch_calibrator";
}

impl WestonTouchCalibrator {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WestonTouchCalibratorHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WestonTouchCalibratorHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WestonTouchCalibrator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WestonTouchCalibrator")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WestonTouchCalibrator {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the calibrator
    ///
    /// This unmaps the surface if it was mapped. The input device grab
    /// is dropped, if it was present. The surface loses its role as a
    /// calibrator.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_touch_calibrator#{}.destroy()\n", id);
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

    /// destroy the calibrator
    ///
    /// This unmaps the surface if it was mapped. The input device grab
    /// is dropped, if it was present. The surface loses its role as a
    /// calibrator.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("weston_touch_calibrator.destroy", &e);
        }
    }

    /// Since when the convert message is available.
    pub const MSG__CONVERT__SINCE: u32 = 1;

    /// convert from surface to raw coordinates
    ///
    /// This request asks the compositor to convert the surface-local
    /// coordinates into the expected touch input coordinates appropriate for
    /// the associated touch device. The intention is that a client uses this
    /// request to convert marker positions that the user is supposed to touch
    /// during calibration.
    ///
    /// If the compositor has cancelled the calibration, the conversion result
    /// shall be zeroes and no errors will be raised.
    ///
    /// The coordinates given as arguments to this request are relative to
    /// the associated wl_surface.
    ///
    /// If a client asks for conversion before it has committed valid
    /// content to the wl_surface, the not_mapped error is raised.
    ///
    /// If the coordinates x, y are outside of the wl_surface content, the
    /// bad_coordinates error is raised.
    ///
    /// # Arguments
    ///
    /// - `x`: surface-local X coordinate
    /// - `y`: surface-local Y coordinate
    /// - `reply`: object delivering the result
    #[inline]
    pub fn try_send_convert(
        &self,
        x: i32,
        y: i32,
        reply: &Rc<WestonTouchCoordinate>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            x,
            y,
            reply,
        );
        let arg2_obj = arg2;
        let arg2 = arg2_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg2.generate_server_id(arg2_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("reply", e)))?;
        let arg2_id = arg2.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: i32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_touch_calibrator#{}.convert(x: {}, y: {}, reply: weston_touch_coordinate#{})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2_id);
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
            arg0 as u32,
            arg1 as u32,
            arg2_id,
        ]);
        Ok(())
    }

    /// convert from surface to raw coordinates
    ///
    /// This request asks the compositor to convert the surface-local
    /// coordinates into the expected touch input coordinates appropriate for
    /// the associated touch device. The intention is that a client uses this
    /// request to convert marker positions that the user is supposed to touch
    /// during calibration.
    ///
    /// If the compositor has cancelled the calibration, the conversion result
    /// shall be zeroes and no errors will be raised.
    ///
    /// The coordinates given as arguments to this request are relative to
    /// the associated wl_surface.
    ///
    /// If a client asks for conversion before it has committed valid
    /// content to the wl_surface, the not_mapped error is raised.
    ///
    /// If the coordinates x, y are outside of the wl_surface content, the
    /// bad_coordinates error is raised.
    ///
    /// # Arguments
    ///
    /// - `x`: surface-local X coordinate
    /// - `y`: surface-local Y coordinate
    /// - `reply`: object delivering the result
    #[inline]
    pub fn send_convert(
        &self,
        x: i32,
        y: i32,
        reply: &Rc<WestonTouchCoordinate>,
    ) {
        let res = self.try_send_convert(
            x,
            y,
            reply,
        );
        if let Err(e) = res {
            log_send("weston_touch_calibrator.convert", &e);
        }
    }

    /// convert from surface to raw coordinates
    ///
    /// This request asks the compositor to convert the surface-local
    /// coordinates into the expected touch input coordinates appropriate for
    /// the associated touch device. The intention is that a client uses this
    /// request to convert marker positions that the user is supposed to touch
    /// during calibration.
    ///
    /// If the compositor has cancelled the calibration, the conversion result
    /// shall be zeroes and no errors will be raised.
    ///
    /// The coordinates given as arguments to this request are relative to
    /// the associated wl_surface.
    ///
    /// If a client asks for conversion before it has committed valid
    /// content to the wl_surface, the not_mapped error is raised.
    ///
    /// If the coordinates x, y are outside of the wl_surface content, the
    /// bad_coordinates error is raised.
    ///
    /// # Arguments
    ///
    /// - `x`: surface-local X coordinate
    /// - `y`: surface-local Y coordinate
    #[inline]
    pub fn new_try_send_convert(
        &self,
        x: i32,
        y: i32,
    ) -> Result<Rc<WestonTouchCoordinate>, ObjectError> {
        let reply = self.core.create_child();
        self.try_send_convert(
            x,
            y,
            &reply,
        )?;
        Ok(reply)
    }

    /// convert from surface to raw coordinates
    ///
    /// This request asks the compositor to convert the surface-local
    /// coordinates into the expected touch input coordinates appropriate for
    /// the associated touch device. The intention is that a client uses this
    /// request to convert marker positions that the user is supposed to touch
    /// during calibration.
    ///
    /// If the compositor has cancelled the calibration, the conversion result
    /// shall be zeroes and no errors will be raised.
    ///
    /// The coordinates given as arguments to this request are relative to
    /// the associated wl_surface.
    ///
    /// If a client asks for conversion before it has committed valid
    /// content to the wl_surface, the not_mapped error is raised.
    ///
    /// If the coordinates x, y are outside of the wl_surface content, the
    /// bad_coordinates error is raised.
    ///
    /// # Arguments
    ///
    /// - `x`: surface-local X coordinate
    /// - `y`: surface-local Y coordinate
    #[inline]
    pub fn new_send_convert(
        &self,
        x: i32,
        y: i32,
    ) -> Rc<WestonTouchCoordinate> {
        let reply = self.core.create_child();
        self.send_convert(
            x,
            y,
            &reply,
        );
        reply
    }

    /// Since when the configure message is available.
    pub const MSG__CONFIGURE__SINCE: u32 = 1;

    /// surface size
    ///
    /// This event tells the client what size to make the surface. The client
    /// must obey the size exactly on the next commit with a wl_buffer.
    ///
    /// This event shall be sent once as a response to creating a
    /// weston_touch_calibrator object.
    ///
    /// # Arguments
    ///
    /// - `width`: surface width
    /// - `height`: surface height
    #[inline]
    pub fn try_send_configure(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_touch_calibrator#{}.configure(width: {}, height: {})\n", client_id, id, arg0, arg1);
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

    /// surface size
    ///
    /// This event tells the client what size to make the surface. The client
    /// must obey the size exactly on the next commit with a wl_buffer.
    ///
    /// This event shall be sent once as a response to creating a
    /// weston_touch_calibrator object.
    ///
    /// # Arguments
    ///
    /// - `width`: surface width
    /// - `height`: surface height
    #[inline]
    pub fn send_configure(
        &self,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_configure(
            width,
            height,
        );
        if let Err(e) = res {
            log_send("weston_touch_calibrator.configure", &e);
        }
    }

    /// Since when the cancel_calibration message is available.
    pub const MSG__CANCEL_CALIBRATION__SINCE: u32 = 1;

    /// cancel the calibration procedure
    ///
    /// This is sent when the compositor wants to cancel the calibration and
    /// drop the touch device grab. The compositor unmaps the surface, if it
    /// was mapped.
    ///
    /// The weston_touch_calibrator object will not send any more events. The
    /// client should destroy it.
    #[inline]
    pub fn try_send_cancel_calibration(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_touch_calibrator#{}.cancel_calibration()\n", client_id, id);
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

    /// cancel the calibration procedure
    ///
    /// This is sent when the compositor wants to cancel the calibration and
    /// drop the touch device grab. The compositor unmaps the surface, if it
    /// was mapped.
    ///
    /// The weston_touch_calibrator object will not send any more events. The
    /// client should destroy it.
    #[inline]
    pub fn send_cancel_calibration(
        &self,
    ) {
        let res = self.try_send_cancel_calibration(
        );
        if let Err(e) = res {
            log_send("weston_touch_calibrator.cancel_calibration", &e);
        }
    }

    /// Since when the invalid_touch message is available.
    pub const MSG__INVALID_TOUCH__SINCE: u32 = 1;

    /// a user touch that cannot be used for calibration
    ///
    /// For whatever reason, a touch event resulting from a user action cannot
    /// be used for calibration. The client should show feedback to the user
    /// that the touch was rejected.
    ///
    /// Possible causes for this event include the user touching a wrong
    /// touchscreen when there are multiple ones present. This is particularly
    /// useful when the touchscreens are cloned and there is no other way to
    /// identify which screen the user should be touching.
    ///
    /// Another cause could be a touch device that sends coordinates beyond its
    /// declared range. If motion takes a touch point outside the range, the
    /// compositor should also send 'cancel' event to undo the touch-down.
    #[inline]
    pub fn try_send_invalid_touch(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_touch_calibrator#{}.invalid_touch()\n", client_id, id);
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

    /// a user touch that cannot be used for calibration
    ///
    /// For whatever reason, a touch event resulting from a user action cannot
    /// be used for calibration. The client should show feedback to the user
    /// that the touch was rejected.
    ///
    /// Possible causes for this event include the user touching a wrong
    /// touchscreen when there are multiple ones present. This is particularly
    /// useful when the touchscreens are cloned and there is no other way to
    /// identify which screen the user should be touching.
    ///
    /// Another cause could be a touch device that sends coordinates beyond its
    /// declared range. If motion takes a touch point outside the range, the
    /// compositor should also send 'cancel' event to undo the touch-down.
    #[inline]
    pub fn send_invalid_touch(
        &self,
    ) {
        let res = self.try_send_invalid_touch(
        );
        if let Err(e) = res {
            log_send("weston_touch_calibrator.invalid_touch", &e);
        }
    }

    /// Since when the down message is available.
    pub const MSG__DOWN__SINCE: u32 = 1;

    /// touch down event and beginning of a touch sequence
    ///
    /// A new touch point has appeared on the surface. This touch point is
    /// assigned a unique ID. Future events from this touch point reference
    /// this ID. The ID ceases to be valid after a touch up event and may be
    /// reused in the future.
    ///
    /// For the coordinate units, see weston_touch_calibrator.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `id`: the unique ID of this touch point
    /// - `x`: x coordinate in calibration units
    /// - `y`: y coordinate in calibration units
    #[inline]
    pub fn try_send_down(
        &self,
        time: u32,
        id: i32,
        x: u32,
        y: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            time,
            id,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: i32, arg2: u32, arg3: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_touch_calibrator#{}.down(time: {}, id: {}, x: {}, y: {})\n", client_id, id, arg0, arg1, arg2, arg3);
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
            3,
            arg0,
            arg1 as u32,
            arg2,
            arg3,
        ]);
        Ok(())
    }

    /// touch down event and beginning of a touch sequence
    ///
    /// A new touch point has appeared on the surface. This touch point is
    /// assigned a unique ID. Future events from this touch point reference
    /// this ID. The ID ceases to be valid after a touch up event and may be
    /// reused in the future.
    ///
    /// For the coordinate units, see weston_touch_calibrator.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `id`: the unique ID of this touch point
    /// - `x`: x coordinate in calibration units
    /// - `y`: y coordinate in calibration units
    #[inline]
    pub fn send_down(
        &self,
        time: u32,
        id: i32,
        x: u32,
        y: u32,
    ) {
        let res = self.try_send_down(
            time,
            id,
            x,
            y,
        );
        if let Err(e) = res {
            log_send("weston_touch_calibrator.down", &e);
        }
    }

    /// Since when the up message is available.
    pub const MSG__UP__SINCE: u32 = 1;

    /// end of a touch event sequence
    ///
    /// The touch point has disappeared. No further events will be sent for
    /// this touch point and the touch point's ID is released and may be
    /// reused in a future touch down event.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `id`: the unique ID of this touch point
    #[inline]
    pub fn try_send_up(
        &self,
        time: u32,
        id: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            time,
            id,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_touch_calibrator#{}.up(time: {}, id: {})\n", client_id, id, arg0, arg1);
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
            4,
            arg0,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// end of a touch event sequence
    ///
    /// The touch point has disappeared. No further events will be sent for
    /// this touch point and the touch point's ID is released and may be
    /// reused in a future touch down event.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `id`: the unique ID of this touch point
    #[inline]
    pub fn send_up(
        &self,
        time: u32,
        id: i32,
    ) {
        let res = self.try_send_up(
            time,
            id,
        );
        if let Err(e) = res {
            log_send("weston_touch_calibrator.up", &e);
        }
    }

    /// Since when the motion message is available.
    pub const MSG__MOTION__SINCE: u32 = 1;

    /// update of touch point coordinates
    ///
    /// A touch point has changed coordinates.
    ///
    /// For the coordinate units, see weston_touch_calibrator.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `id`: the unique ID of this touch point
    /// - `x`: x coordinate in calibration units
    /// - `y`: y coordinate in calibration units
    #[inline]
    pub fn try_send_motion(
        &self,
        time: u32,
        id: i32,
        x: u32,
        y: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            time,
            id,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: i32, arg2: u32, arg3: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_touch_calibrator#{}.motion(time: {}, id: {}, x: {}, y: {})\n", client_id, id, arg0, arg1, arg2, arg3);
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
            5,
            arg0,
            arg1 as u32,
            arg2,
            arg3,
        ]);
        Ok(())
    }

    /// update of touch point coordinates
    ///
    /// A touch point has changed coordinates.
    ///
    /// For the coordinate units, see weston_touch_calibrator.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `id`: the unique ID of this touch point
    /// - `x`: x coordinate in calibration units
    /// - `y`: y coordinate in calibration units
    #[inline]
    pub fn send_motion(
        &self,
        time: u32,
        id: i32,
        x: u32,
        y: u32,
    ) {
        let res = self.try_send_motion(
            time,
            id,
            x,
            y,
        );
        if let Err(e) = res {
            log_send("weston_touch_calibrator.motion", &e);
        }
    }

    /// Since when the frame message is available.
    pub const MSG__FRAME__SINCE: u32 = 1;

    /// end of touch frame event
    ///
    /// Indicates the end of a set of events that logically belong together.
    /// A client is expected to accumulate the data in all events within the
    /// frame before proceeding.
    ///
    /// A wl_touch.frame terminates at least one event but otherwise no
    /// guarantee is provided about the set of events within a frame. A client
    /// must assume that any state not updated in a frame is unchanged from the
    /// previously known state.
    #[inline]
    pub fn try_send_frame(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_touch_calibrator#{}.frame()\n", client_id, id);
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
            6,
        ]);
        Ok(())
    }

    /// end of touch frame event
    ///
    /// Indicates the end of a set of events that logically belong together.
    /// A client is expected to accumulate the data in all events within the
    /// frame before proceeding.
    ///
    /// A wl_touch.frame terminates at least one event but otherwise no
    /// guarantee is provided about the set of events within a frame. A client
    /// must assume that any state not updated in a frame is unchanged from the
    /// previously known state.
    #[inline]
    pub fn send_frame(
        &self,
    ) {
        let res = self.try_send_frame(
        );
        if let Err(e) = res {
            log_send("weston_touch_calibrator.frame", &e);
        }
    }

    /// Since when the cancel message is available.
    pub const MSG__CANCEL__SINCE: u32 = 1;

    /// touch session cancelled
    ///
    /// Sent if the compositor decides the touch stream is a global
    /// gesture. No further events are sent to the clients from that
    /// particular gesture. Touch cancellation applies to all touch points
    /// currently active on this client's surface. The client is
    /// responsible for finalizing the touch points, future touch points on
    /// this surface may reuse the touch point ID.
    #[inline]
    pub fn try_send_cancel(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_touch_calibrator#{}.cancel()\n", client_id, id);
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
            7,
        ]);
        Ok(())
    }

    /// touch session cancelled
    ///
    /// Sent if the compositor decides the touch stream is a global
    /// gesture. No further events are sent to the clients from that
    /// particular gesture. Touch cancellation applies to all touch points
    /// currently active on this client's surface. The client is
    /// responsible for finalizing the touch points, future touch points on
    /// this surface may reuse the touch point ID.
    #[inline]
    pub fn send_cancel(
        &self,
    ) {
        let res = self.try_send_cancel(
        );
        if let Err(e) = res {
            log_send("weston_touch_calibrator.cancel", &e);
        }
    }
}

/// A message handler for [`WestonTouchCalibrator`] proxies.
pub trait WestonTouchCalibratorHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WestonTouchCalibrator>) {
        slf.core.delete_id();
    }

    /// destroy the calibrator
    ///
    /// This unmaps the surface if it was mapped. The input device grab
    /// is dropped, if it was present. The surface loses its role as a
    /// calibrator.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WestonTouchCalibrator>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("weston_touch_calibrator.destroy", &e);
        }
    }

    /// convert from surface to raw coordinates
    ///
    /// This request asks the compositor to convert the surface-local
    /// coordinates into the expected touch input coordinates appropriate for
    /// the associated touch device. The intention is that a client uses this
    /// request to convert marker positions that the user is supposed to touch
    /// during calibration.
    ///
    /// If the compositor has cancelled the calibration, the conversion result
    /// shall be zeroes and no errors will be raised.
    ///
    /// The coordinates given as arguments to this request are relative to
    /// the associated wl_surface.
    ///
    /// If a client asks for conversion before it has committed valid
    /// content to the wl_surface, the not_mapped error is raised.
    ///
    /// If the coordinates x, y are outside of the wl_surface content, the
    /// bad_coordinates error is raised.
    ///
    /// # Arguments
    ///
    /// - `x`: surface-local X coordinate
    /// - `y`: surface-local Y coordinate
    /// - `reply`: object delivering the result
    #[inline]
    fn handle_convert(
        &mut self,
        slf: &Rc<WestonTouchCalibrator>,
        x: i32,
        y: i32,
        reply: &Rc<WestonTouchCoordinate>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_convert(
            x,
            y,
            reply,
        );
        if let Err(e) = res {
            log_forward("weston_touch_calibrator.convert", &e);
        }
    }

    /// surface size
    ///
    /// This event tells the client what size to make the surface. The client
    /// must obey the size exactly on the next commit with a wl_buffer.
    ///
    /// This event shall be sent once as a response to creating a
    /// weston_touch_calibrator object.
    ///
    /// # Arguments
    ///
    /// - `width`: surface width
    /// - `height`: surface height
    #[inline]
    fn handle_configure(
        &mut self,
        slf: &Rc<WestonTouchCalibrator>,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_configure(
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("weston_touch_calibrator.configure", &e);
        }
    }

    /// cancel the calibration procedure
    ///
    /// This is sent when the compositor wants to cancel the calibration and
    /// drop the touch device grab. The compositor unmaps the surface, if it
    /// was mapped.
    ///
    /// The weston_touch_calibrator object will not send any more events. The
    /// client should destroy it.
    #[inline]
    fn handle_cancel_calibration(
        &mut self,
        slf: &Rc<WestonTouchCalibrator>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_cancel_calibration(
        );
        if let Err(e) = res {
            log_forward("weston_touch_calibrator.cancel_calibration", &e);
        }
    }

    /// a user touch that cannot be used for calibration
    ///
    /// For whatever reason, a touch event resulting from a user action cannot
    /// be used for calibration. The client should show feedback to the user
    /// that the touch was rejected.
    ///
    /// Possible causes for this event include the user touching a wrong
    /// touchscreen when there are multiple ones present. This is particularly
    /// useful when the touchscreens are cloned and there is no other way to
    /// identify which screen the user should be touching.
    ///
    /// Another cause could be a touch device that sends coordinates beyond its
    /// declared range. If motion takes a touch point outside the range, the
    /// compositor should also send 'cancel' event to undo the touch-down.
    #[inline]
    fn handle_invalid_touch(
        &mut self,
        slf: &Rc<WestonTouchCalibrator>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_invalid_touch(
        );
        if let Err(e) = res {
            log_forward("weston_touch_calibrator.invalid_touch", &e);
        }
    }

    /// touch down event and beginning of a touch sequence
    ///
    /// A new touch point has appeared on the surface. This touch point is
    /// assigned a unique ID. Future events from this touch point reference
    /// this ID. The ID ceases to be valid after a touch up event and may be
    /// reused in the future.
    ///
    /// For the coordinate units, see weston_touch_calibrator.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `id`: the unique ID of this touch point
    /// - `x`: x coordinate in calibration units
    /// - `y`: y coordinate in calibration units
    #[inline]
    fn handle_down(
        &mut self,
        slf: &Rc<WestonTouchCalibrator>,
        time: u32,
        id: i32,
        x: u32,
        y: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_down(
            time,
            id,
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("weston_touch_calibrator.down", &e);
        }
    }

    /// end of a touch event sequence
    ///
    /// The touch point has disappeared. No further events will be sent for
    /// this touch point and the touch point's ID is released and may be
    /// reused in a future touch down event.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `id`: the unique ID of this touch point
    #[inline]
    fn handle_up(
        &mut self,
        slf: &Rc<WestonTouchCalibrator>,
        time: u32,
        id: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_up(
            time,
            id,
        );
        if let Err(e) = res {
            log_forward("weston_touch_calibrator.up", &e);
        }
    }

    /// update of touch point coordinates
    ///
    /// A touch point has changed coordinates.
    ///
    /// For the coordinate units, see weston_touch_calibrator.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `id`: the unique ID of this touch point
    /// - `x`: x coordinate in calibration units
    /// - `y`: y coordinate in calibration units
    #[inline]
    fn handle_motion(
        &mut self,
        slf: &Rc<WestonTouchCalibrator>,
        time: u32,
        id: i32,
        x: u32,
        y: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_motion(
            time,
            id,
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("weston_touch_calibrator.motion", &e);
        }
    }

    /// end of touch frame event
    ///
    /// Indicates the end of a set of events that logically belong together.
    /// A client is expected to accumulate the data in all events within the
    /// frame before proceeding.
    ///
    /// A wl_touch.frame terminates at least one event but otherwise no
    /// guarantee is provided about the set of events within a frame. A client
    /// must assume that any state not updated in a frame is unchanged from the
    /// previously known state.
    #[inline]
    fn handle_frame(
        &mut self,
        slf: &Rc<WestonTouchCalibrator>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_frame(
        );
        if let Err(e) = res {
            log_forward("weston_touch_calibrator.frame", &e);
        }
    }

    /// touch session cancelled
    ///
    /// Sent if the compositor decides the touch stream is a global
    /// gesture. No further events are sent to the clients from that
    /// particular gesture. Touch cancellation applies to all touch points
    /// currently active on this client's surface. The client is
    /// responsible for finalizing the touch points, future touch points on
    /// this surface may reuse the touch point ID.
    #[inline]
    fn handle_cancel(
        &mut self,
        slf: &Rc<WestonTouchCalibrator>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_cancel(
        );
        if let Err(e) = res {
            log_forward("weston_touch_calibrator.cancel", &e);
        }
    }
}

impl ObjectPrivate for WestonTouchCalibrator {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WestonTouchCalibrator, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_touch_calibrator#{}.destroy()\n", client_id, id);
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
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_touch_calibrator#{}.convert(x: {}, y: {}, reply: weston_touch_coordinate#{})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg2_id = arg2;
                let arg2 = WestonTouchCoordinate::new(&self.core.state, self.core.version);
                arg2.core().set_client_id(client, arg2_id, arg2.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg2_id, "reply", e)))?;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).handle_convert(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_convert(&self, arg0, arg1, arg2);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_touch_calibrator#{}.configure(width: {}, height: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_configure(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_configure(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_touch_calibrator#{}.cancel_calibration()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_cancel_calibration(&self);
                } else {
                    DefaultHandler.handle_cancel_calibration(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_touch_calibrator#{}.invalid_touch()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_invalid_touch(&self);
                } else {
                    DefaultHandler.handle_invalid_touch(&self);
                }
            }
            3 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 24)));
                };
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: i32, arg2: u32, arg3: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_touch_calibrator#{}.down(time: {}, id: {}, x: {}, y: {})\n", id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_down(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_down(&self, arg0, arg1, arg2, arg3);
                }
            }
            4 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_touch_calibrator#{}.up(time: {}, id: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_up(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_up(&self, arg0, arg1);
                }
            }
            5 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 24)));
                };
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: i32, arg2: u32, arg3: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_touch_calibrator#{}.motion(time: {}, id: {}, x: {}, y: {})\n", id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_motion(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_motion(&self, arg0, arg1, arg2, arg3);
                }
            }
            6 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_touch_calibrator#{}.frame()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_frame(&self);
                } else {
                    DefaultHandler.handle_frame(&self);
                }
            }
            7 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_touch_calibrator#{}.cancel()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_cancel(&self);
                } else {
                    DefaultHandler.handle_cancel(&self);
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
            1 => "convert",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "configure",
            1 => "cancel_calibration",
            2 => "invalid_touch",
            3 => "down",
            4 => "up",
            5 => "motion",
            6 => "frame",
            7 => "cancel",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WestonTouchCalibrator {
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

impl WestonTouchCalibrator {
    /// Since when the error.bad_size enum variant is available.
    pub const ENM__ERROR_BAD_SIZE__SINCE: u32 = 1;
    /// Since when the error.not_mapped enum variant is available.
    pub const ENM__ERROR_NOT_MAPPED__SINCE: u32 = 1;
    /// Since when the error.bad_coordinates enum variant is available.
    pub const ENM__ERROR_BAD_COORDINATES__SINCE: u32 = 1;
}

/// calibrator object errors
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WestonTouchCalibratorError(pub u32);

impl WestonTouchCalibratorError {
    /// surface size does not match
    pub const BAD_SIZE: Self = Self(0);

    /// requested operation is not possible without mapping the surface
    pub const NOT_MAPPED: Self = Self(1);

    /// surface-local coordinates are out of bounds
    pub const BAD_COORDINATES: Self = Self(2);
}

impl Debug for WestonTouchCalibratorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::BAD_SIZE => "BAD_SIZE",
            Self::NOT_MAPPED => "NOT_MAPPED",
            Self::BAD_COORDINATES => "BAD_COORDINATES",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
