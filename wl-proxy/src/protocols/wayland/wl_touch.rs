//! touchscreen input device
//!
//! The wl_touch interface represents a touchscreen
//! associated with a seat.
//!
//! Touch interactions can consist of one or more contacts.
//! For each contact, a series of events is generated, starting
//! with a down event, followed by zero or more motion events,
//! and ending with an up event. Events relating to the same
//! contact point can be identified by the ID of the sequence.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wl_touch object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlTouch {
    core: ObjectCore,
    handler: HandlerHolder<dyn WlTouchHandler>,
}

struct DefaultHandler;

impl WlTouchHandler for DefaultHandler { }

impl ConcreteObject for WlTouch {
    const XML_VERSION: u32 = 10;
    const INTERFACE: ObjectInterface = ObjectInterface::WlTouch;
    const INTERFACE_NAME: &str = "wl_touch";
}

impl WlTouch {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WlTouchHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WlTouchHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlTouch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlTouch")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlTouch {
    /// Since when the down message is available.
    pub const MSG__DOWN__SINCE: u32 = 1;

    /// touch down event and beginning of a touch sequence
    ///
    /// A new touch point has appeared on the surface. This touch point is
    /// assigned a unique ID. Future events from this touch point reference
    /// this ID. The ID ceases to be valid after a touch up event and may be
    /// reused in the future.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the touch down event
    /// - `time`: timestamp with millisecond granularity
    /// - `surface`: surface touched
    /// - `id`: the unique ID of this touch point
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    #[inline]
    pub fn try_send_down(
        &self,
        serial: u32,
        time: u32,
        surface: &Rc<WlSurface>,
        id: i32,
        x: Fixed,
        y: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
        ) = (
            serial,
            time,
            surface,
            id,
            x,
            y,
        );
        let arg2 = arg2.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg2.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("surface", client.endpoint.id)));
        }
        let arg2_id = arg2.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: i32, arg4: Fixed, arg5: Fixed) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_touch#{}.down(serial: {}, time: {}, surface: wl_surface#{}, id: {}, x: {}, y: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4, arg5);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1, arg2_id, arg3, arg4, arg5);
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
            arg0,
            arg1,
            arg2_id,
            arg3 as u32,
            arg4.to_wire() as u32,
            arg5.to_wire() as u32,
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
    /// # Arguments
    ///
    /// - `serial`: serial number of the touch down event
    /// - `time`: timestamp with millisecond granularity
    /// - `surface`: surface touched
    /// - `id`: the unique ID of this touch point
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    #[inline]
    pub fn send_down(
        &self,
        serial: u32,
        time: u32,
        surface: &Rc<WlSurface>,
        id: i32,
        x: Fixed,
        y: Fixed,
    ) {
        let res = self.try_send_down(
            serial,
            time,
            surface,
            id,
            x,
            y,
        );
        if let Err(e) = res {
            log_send("wl_touch.down", &e);
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
    /// - `serial`: serial number of the touch up event
    /// - `time`: timestamp with millisecond granularity
    /// - `id`: the unique ID of this touch point
    #[inline]
    pub fn try_send_up(
        &self,
        serial: u32,
        time: u32,
        id: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            serial,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_touch#{}.up(serial: {}, time: {}, id: {})\n", client_id, id, arg0, arg1, arg2);
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
            1,
            arg0,
            arg1,
            arg2 as u32,
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
    /// - `serial`: serial number of the touch up event
    /// - `time`: timestamp with millisecond granularity
    /// - `id`: the unique ID of this touch point
    #[inline]
    pub fn send_up(
        &self,
        serial: u32,
        time: u32,
        id: i32,
    ) {
        let res = self.try_send_up(
            serial,
            time,
            id,
        );
        if let Err(e) = res {
            log_send("wl_touch.up", &e);
        }
    }

    /// Since when the motion message is available.
    pub const MSG__MOTION__SINCE: u32 = 1;

    /// update of touch point coordinates
    ///
    /// A touch point has changed coordinates.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `id`: the unique ID of this touch point
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    #[inline]
    pub fn try_send_motion(
        &self,
        time: u32,
        id: i32,
        x: Fixed,
        y: Fixed,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: i32, arg2: Fixed, arg3: Fixed) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_touch#{}.motion(time: {}, id: {}, x: {}, y: {})\n", client_id, id, arg0, arg1, arg2, arg3);
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
            2,
            arg0,
            arg1 as u32,
            arg2.to_wire() as u32,
            arg3.to_wire() as u32,
        ]);
        Ok(())
    }

    /// update of touch point coordinates
    ///
    /// A touch point has changed coordinates.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `id`: the unique ID of this touch point
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    #[inline]
    pub fn send_motion(
        &self,
        time: u32,
        id: i32,
        x: Fixed,
        y: Fixed,
    ) {
        let res = self.try_send_motion(
            time,
            id,
            x,
            y,
        );
        if let Err(e) = res {
            log_send("wl_touch.motion", &e);
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_touch#{}.frame()\n", client_id, id);
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
            3,
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
            log_send("wl_touch.frame", &e);
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
    ///
    /// No frame event is required after the cancel event.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_touch#{}.cancel()\n", client_id, id);
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

    /// touch session cancelled
    ///
    /// Sent if the compositor decides the touch stream is a global
    /// gesture. No further events are sent to the clients from that
    /// particular gesture. Touch cancellation applies to all touch points
    /// currently active on this client's surface. The client is
    /// responsible for finalizing the touch points, future touch points on
    /// this surface may reuse the touch point ID.
    ///
    /// No frame event is required after the cancel event.
    #[inline]
    pub fn send_cancel(
        &self,
    ) {
        let res = self.try_send_cancel(
        );
        if let Err(e) = res {
            log_send("wl_touch.cancel", &e);
        }
    }

    /// Since when the release message is available.
    pub const MSG__RELEASE__SINCE: u32 = 3;

    /// release the touch object
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_touch#{}.release()\n", id);
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

    /// release the touch object
    #[inline]
    pub fn send_release(
        &self,
    ) {
        let res = self.try_send_release(
        );
        if let Err(e) = res {
            log_send("wl_touch.release", &e);
        }
    }

    /// Since when the shape message is available.
    pub const MSG__SHAPE__SINCE: u32 = 6;

    /// update shape of touch point
    ///
    /// Sent when a touchpoint has changed its shape.
    ///
    /// This event does not occur on its own. It is sent before a
    /// wl_touch.frame event and carries the new shape information for
    /// any previously reported, or new touch points of that frame.
    ///
    /// Other events describing the touch point such as wl_touch.down,
    /// wl_touch.motion or wl_touch.orientation may be sent within the
    /// same wl_touch.frame. A client should treat these events as a single
    /// logical touch point update. The order of wl_touch.shape,
    /// wl_touch.orientation and wl_touch.motion is not guaranteed.
    /// A wl_touch.down event is guaranteed to occur before the first
    /// wl_touch.shape event for this touch ID but both events may occur within
    /// the same wl_touch.frame.
    ///
    /// A touchpoint shape is approximated by an ellipse through the major and
    /// minor axis length. The major axis length describes the longer diameter
    /// of the ellipse, while the minor axis length describes the shorter
    /// diameter. Major and minor are orthogonal and both are specified in
    /// surface-local coordinates. The center of the ellipse is always at the
    /// touchpoint location as reported by wl_touch.down or wl_touch.move.
    ///
    /// This event is only sent by the compositor if the touch device supports
    /// shape reports. The client has to make reasonable assumptions about the
    /// shape if it did not receive this event.
    ///
    /// # Arguments
    ///
    /// - `id`: the unique ID of this touch point
    /// - `major`: length of the major axis in surface-local coordinates
    /// - `minor`: length of the minor axis in surface-local coordinates
    #[inline]
    pub fn try_send_shape(
        &self,
        id: i32,
        major: Fixed,
        minor: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            id,
            major,
            minor,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: Fixed, arg2: Fixed) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_touch#{}.shape(id: {}, major: {}, minor: {})\n", client_id, id, arg0, arg1, arg2);
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
            5,
            arg0 as u32,
            arg1.to_wire() as u32,
            arg2.to_wire() as u32,
        ]);
        Ok(())
    }

    /// update shape of touch point
    ///
    /// Sent when a touchpoint has changed its shape.
    ///
    /// This event does not occur on its own. It is sent before a
    /// wl_touch.frame event and carries the new shape information for
    /// any previously reported, or new touch points of that frame.
    ///
    /// Other events describing the touch point such as wl_touch.down,
    /// wl_touch.motion or wl_touch.orientation may be sent within the
    /// same wl_touch.frame. A client should treat these events as a single
    /// logical touch point update. The order of wl_touch.shape,
    /// wl_touch.orientation and wl_touch.motion is not guaranteed.
    /// A wl_touch.down event is guaranteed to occur before the first
    /// wl_touch.shape event for this touch ID but both events may occur within
    /// the same wl_touch.frame.
    ///
    /// A touchpoint shape is approximated by an ellipse through the major and
    /// minor axis length. The major axis length describes the longer diameter
    /// of the ellipse, while the minor axis length describes the shorter
    /// diameter. Major and minor are orthogonal and both are specified in
    /// surface-local coordinates. The center of the ellipse is always at the
    /// touchpoint location as reported by wl_touch.down or wl_touch.move.
    ///
    /// This event is only sent by the compositor if the touch device supports
    /// shape reports. The client has to make reasonable assumptions about the
    /// shape if it did not receive this event.
    ///
    /// # Arguments
    ///
    /// - `id`: the unique ID of this touch point
    /// - `major`: length of the major axis in surface-local coordinates
    /// - `minor`: length of the minor axis in surface-local coordinates
    #[inline]
    pub fn send_shape(
        &self,
        id: i32,
        major: Fixed,
        minor: Fixed,
    ) {
        let res = self.try_send_shape(
            id,
            major,
            minor,
        );
        if let Err(e) = res {
            log_send("wl_touch.shape", &e);
        }
    }

    /// Since when the orientation message is available.
    pub const MSG__ORIENTATION__SINCE: u32 = 6;

    /// update orientation of touch point
    ///
    /// Sent when a touchpoint has changed its orientation.
    ///
    /// This event does not occur on its own. It is sent before a
    /// wl_touch.frame event and carries the new shape information for
    /// any previously reported, or new touch points of that frame.
    ///
    /// Other events describing the touch point such as wl_touch.down,
    /// wl_touch.motion or wl_touch.shape may be sent within the
    /// same wl_touch.frame. A client should treat these events as a single
    /// logical touch point update. The order of wl_touch.shape,
    /// wl_touch.orientation and wl_touch.motion is not guaranteed.
    /// A wl_touch.down event is guaranteed to occur before the first
    /// wl_touch.orientation event for this touch ID but both events may occur
    /// within the same wl_touch.frame.
    ///
    /// The orientation describes the clockwise angle of a touchpoint's major
    /// axis to the positive surface y-axis and is normalized to the -180 to
    /// +180 degree range. The granularity of orientation depends on the touch
    /// device, some devices only support binary rotation values between 0 and
    /// 90 degrees.
    ///
    /// This event is only sent by the compositor if the touch device supports
    /// orientation reports.
    ///
    /// # Arguments
    ///
    /// - `id`: the unique ID of this touch point
    /// - `orientation`: angle between major axis and positive surface y-axis in degrees
    #[inline]
    pub fn try_send_orientation(
        &self,
        id: i32,
        orientation: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            orientation,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: Fixed) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_touch#{}.orientation(id: {}, orientation: {})\n", client_id, id, arg0, arg1);
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
            6,
            arg0 as u32,
            arg1.to_wire() as u32,
        ]);
        Ok(())
    }

    /// update orientation of touch point
    ///
    /// Sent when a touchpoint has changed its orientation.
    ///
    /// This event does not occur on its own. It is sent before a
    /// wl_touch.frame event and carries the new shape information for
    /// any previously reported, or new touch points of that frame.
    ///
    /// Other events describing the touch point such as wl_touch.down,
    /// wl_touch.motion or wl_touch.shape may be sent within the
    /// same wl_touch.frame. A client should treat these events as a single
    /// logical touch point update. The order of wl_touch.shape,
    /// wl_touch.orientation and wl_touch.motion is not guaranteed.
    /// A wl_touch.down event is guaranteed to occur before the first
    /// wl_touch.orientation event for this touch ID but both events may occur
    /// within the same wl_touch.frame.
    ///
    /// The orientation describes the clockwise angle of a touchpoint's major
    /// axis to the positive surface y-axis and is normalized to the -180 to
    /// +180 degree range. The granularity of orientation depends on the touch
    /// device, some devices only support binary rotation values between 0 and
    /// 90 degrees.
    ///
    /// This event is only sent by the compositor if the touch device supports
    /// orientation reports.
    ///
    /// # Arguments
    ///
    /// - `id`: the unique ID of this touch point
    /// - `orientation`: angle between major axis and positive surface y-axis in degrees
    #[inline]
    pub fn send_orientation(
        &self,
        id: i32,
        orientation: Fixed,
    ) {
        let res = self.try_send_orientation(
            id,
            orientation,
        );
        if let Err(e) = res {
            log_send("wl_touch.orientation", &e);
        }
    }
}

/// A message handler for [`WlTouch`] proxies.
pub trait WlTouchHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WlTouch>) {
        slf.core.delete_id();
    }

    /// touch down event and beginning of a touch sequence
    ///
    /// A new touch point has appeared on the surface. This touch point is
    /// assigned a unique ID. Future events from this touch point reference
    /// this ID. The ID ceases to be valid after a touch up event and may be
    /// reused in the future.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the touch down event
    /// - `time`: timestamp with millisecond granularity
    /// - `surface`: surface touched
    /// - `id`: the unique ID of this touch point
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_down(
        &mut self,
        slf: &Rc<WlTouch>,
        serial: u32,
        time: u32,
        surface: &Rc<WlSurface>,
        id: i32,
        x: Fixed,
        y: Fixed,
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
        }
        let res = slf.try_send_down(
            serial,
            time,
            surface,
            id,
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("wl_touch.down", &e);
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
    /// - `serial`: serial number of the touch up event
    /// - `time`: timestamp with millisecond granularity
    /// - `id`: the unique ID of this touch point
    #[inline]
    fn handle_up(
        &mut self,
        slf: &Rc<WlTouch>,
        serial: u32,
        time: u32,
        id: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_up(
            serial,
            time,
            id,
        );
        if let Err(e) = res {
            log_forward("wl_touch.up", &e);
        }
    }

    /// update of touch point coordinates
    ///
    /// A touch point has changed coordinates.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `id`: the unique ID of this touch point
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    #[inline]
    fn handle_motion(
        &mut self,
        slf: &Rc<WlTouch>,
        time: u32,
        id: i32,
        x: Fixed,
        y: Fixed,
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
            log_forward("wl_touch.motion", &e);
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
        slf: &Rc<WlTouch>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_frame(
        );
        if let Err(e) = res {
            log_forward("wl_touch.frame", &e);
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
    ///
    /// No frame event is required after the cancel event.
    #[inline]
    fn handle_cancel(
        &mut self,
        slf: &Rc<WlTouch>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_cancel(
        );
        if let Err(e) = res {
            log_forward("wl_touch.cancel", &e);
        }
    }

    /// release the touch object
    #[inline]
    fn handle_release(
        &mut self,
        slf: &Rc<WlTouch>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_release(
        );
        if let Err(e) = res {
            log_forward("wl_touch.release", &e);
        }
    }

    /// update shape of touch point
    ///
    /// Sent when a touchpoint has changed its shape.
    ///
    /// This event does not occur on its own. It is sent before a
    /// wl_touch.frame event and carries the new shape information for
    /// any previously reported, or new touch points of that frame.
    ///
    /// Other events describing the touch point such as wl_touch.down,
    /// wl_touch.motion or wl_touch.orientation may be sent within the
    /// same wl_touch.frame. A client should treat these events as a single
    /// logical touch point update. The order of wl_touch.shape,
    /// wl_touch.orientation and wl_touch.motion is not guaranteed.
    /// A wl_touch.down event is guaranteed to occur before the first
    /// wl_touch.shape event for this touch ID but both events may occur within
    /// the same wl_touch.frame.
    ///
    /// A touchpoint shape is approximated by an ellipse through the major and
    /// minor axis length. The major axis length describes the longer diameter
    /// of the ellipse, while the minor axis length describes the shorter
    /// diameter. Major and minor are orthogonal and both are specified in
    /// surface-local coordinates. The center of the ellipse is always at the
    /// touchpoint location as reported by wl_touch.down or wl_touch.move.
    ///
    /// This event is only sent by the compositor if the touch device supports
    /// shape reports. The client has to make reasonable assumptions about the
    /// shape if it did not receive this event.
    ///
    /// # Arguments
    ///
    /// - `id`: the unique ID of this touch point
    /// - `major`: length of the major axis in surface-local coordinates
    /// - `minor`: length of the minor axis in surface-local coordinates
    #[inline]
    fn handle_shape(
        &mut self,
        slf: &Rc<WlTouch>,
        id: i32,
        major: Fixed,
        minor: Fixed,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_shape(
            id,
            major,
            minor,
        );
        if let Err(e) = res {
            log_forward("wl_touch.shape", &e);
        }
    }

    /// update orientation of touch point
    ///
    /// Sent when a touchpoint has changed its orientation.
    ///
    /// This event does not occur on its own. It is sent before a
    /// wl_touch.frame event and carries the new shape information for
    /// any previously reported, or new touch points of that frame.
    ///
    /// Other events describing the touch point such as wl_touch.down,
    /// wl_touch.motion or wl_touch.shape may be sent within the
    /// same wl_touch.frame. A client should treat these events as a single
    /// logical touch point update. The order of wl_touch.shape,
    /// wl_touch.orientation and wl_touch.motion is not guaranteed.
    /// A wl_touch.down event is guaranteed to occur before the first
    /// wl_touch.orientation event for this touch ID but both events may occur
    /// within the same wl_touch.frame.
    ///
    /// The orientation describes the clockwise angle of a touchpoint's major
    /// axis to the positive surface y-axis and is normalized to the -180 to
    /// +180 degree range. The granularity of orientation depends on the touch
    /// device, some devices only support binary rotation values between 0 and
    /// 90 degrees.
    ///
    /// This event is only sent by the compositor if the touch device supports
    /// orientation reports.
    ///
    /// # Arguments
    ///
    /// - `id`: the unique ID of this touch point
    /// - `orientation`: angle between major axis and positive surface y-axis in degrees
    #[inline]
    fn handle_orientation(
        &mut self,
        slf: &Rc<WlTouch>,
        id: i32,
        orientation: Fixed,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_orientation(
            id,
            orientation,
        );
        if let Err(e) = res {
            log_forward("wl_touch.orientation", &e);
        }
    }
}

impl ObjectPrivate for WlTouch {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WlTouch, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_touch#{}.release()\n", client_id, id);
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
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                    arg5,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 32)));
                };
                let arg3 = arg3 as i32;
                let arg4 = Fixed::from_wire(arg4 as i32);
                let arg5 = Fixed::from_wire(arg5 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: i32, arg4: Fixed, arg5: Fixed) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_touch#{}.down(serial: {}, time: {}, surface: wl_surface#{}, id: {}, x: {}, y: {})\n", id, arg0, arg1, arg2, arg3, arg4, arg5);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2, arg3, arg4, arg5);
                }
                let arg2_id = arg2;
                let Some(arg2) = server.lookup(arg2_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg2_id)));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = server.lookup(arg2_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).handle_down(&self, arg0, arg1, arg2, arg3, arg4, arg5);
                } else {
                    DefaultHandler.handle_down(&self, arg0, arg1, arg2, arg3, arg4, arg5);
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
                let arg2 = arg2 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_touch#{}.up(serial: {}, time: {}, id: {})\n", id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_up(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_up(&self, arg0, arg1, arg2);
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
                let arg1 = arg1 as i32;
                let arg2 = Fixed::from_wire(arg2 as i32);
                let arg3 = Fixed::from_wire(arg3 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: i32, arg2: Fixed, arg3: Fixed) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_touch#{}.motion(time: {}, id: {}, x: {}, y: {})\n", id, arg0, arg1, arg2, arg3);
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
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_touch#{}.frame()\n", id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_touch#{}.cancel()\n", id);
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
            5 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                let arg0 = arg0 as i32;
                let arg1 = Fixed::from_wire(arg1 as i32);
                let arg2 = Fixed::from_wire(arg2 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32, arg1: Fixed, arg2: Fixed) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_touch#{}.shape(id: {}, major: {}, minor: {})\n", id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_shape(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_shape(&self, arg0, arg1, arg2);
                }
            }
            6 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = arg0 as i32;
                let arg1 = Fixed::from_wire(arg1 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32, arg1: Fixed) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_touch#{}.orientation(id: {}, orientation: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_orientation(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_orientation(&self, arg0, arg1);
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
            0 => "release",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "down",
            1 => "up",
            2 => "motion",
            3 => "frame",
            4 => "cancel",
            5 => "shape",
            6 => "orientation",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WlTouch {
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

