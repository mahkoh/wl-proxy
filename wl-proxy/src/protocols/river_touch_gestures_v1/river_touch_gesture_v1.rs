//! define a touch gesture, receive trigger events
//!
//! Define the requirements for a gesture to be triggered and receive
//! information on e.g. motion and pinch scale when triggered.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A river_touch_gesture_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct RiverTouchGestureV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn RiverTouchGestureV1Handler>,
}

struct DefaultHandler;

impl RiverTouchGestureV1Handler for DefaultHandler { }

impl ConcreteObject for RiverTouchGestureV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::RiverTouchGestureV1;
    const INTERFACE_NAME: &str = "river_touch_gesture_v1";
}

impl RiverTouchGestureV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl RiverTouchGestureV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn RiverTouchGestureV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for RiverTouchGestureV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RiverTouchGestureV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl RiverTouchGestureV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the touch gesture object
    ///
    /// This request indicates that the client will no longer use the object and
    /// that it may be safely destroyed.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_touch_gesture_v1#{}.destroy()\n", id);
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

    /// destroy the touch gesture object
    ///
    /// This request indicates that the client will no longer use the object and
    /// that it may be safely destroyed.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("river_touch_gesture_v1.destroy", &e);
        }
    }

    /// Since when the enable message is available.
    pub const MSG__ENABLE__SINCE: u32 = 1;

    /// enable the gesture
    ///
    /// This request should be made after all initial configuration has been
    /// completed and the window manager wishes the gesture to be able to be
    /// triggered.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_enable(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_touch_gesture_v1#{}.enable()\n", id);
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
        Ok(())
    }

    /// enable the gesture
    ///
    /// This request should be made after all initial configuration has been
    /// completed and the window manager wishes the gesture to be able to be
    /// triggered.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_enable(
        &self,
    ) {
        let res = self.try_send_enable(
        );
        if let Err(e) = res {
            log_send("river_touch_gesture_v1.enable", &e);
        }
    }

    /// Since when the disable message is available.
    pub const MSG__DISABLE__SINCE: u32 = 1;

    /// disable the gesture
    ///
    /// This request may be used to temporarily disable the gesture. It may
    /// be later re-enabled with the enable request.
    ///
    /// This request does not affect an in progress gesture that has already
    /// been started. It will prevent the gesture from being triggered again.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_disable(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_touch_gesture_v1#{}.disable()\n", id);
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
        Ok(())
    }

    /// disable the gesture
    ///
    /// This request may be used to temporarily disable the gesture. It may
    /// be later re-enabled with the enable request.
    ///
    /// This request does not affect an in progress gesture that has already
    /// been started. It will prevent the gesture from being triggered again.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_disable(
        &self,
    ) {
        let res = self.try_send_disable(
        );
        if let Err(e) = res {
            log_send("river_touch_gesture_v1.disable", &e);
        }
    }

    /// Since when the start message is available.
    pub const MSG__START__SINCE: u32 = 1;

    /// gesture started
    ///
    /// This event is sent when the required finger_count number of touch points
    /// are active and all threshold requirements are met. After this event is
    /// sent, no other gesture will be triggered until all touch points are
    /// released and the river_touch_gesture_v1.end event is sent.
    ///
    /// If the threshold requirements for multiple river_touch_gesture_v1
    /// objects are met at the same time, it is compositor policy which gesture
    /// is triggered.
    ///
    /// The wl_touch.cancel event is sent to all surfaces with touch focus, all
    /// touch input is eaten till end of gesture.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn try_send_start(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_touch_gesture_v1#{}.start()\n", client_id, id);
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
            0,
        ]);
        Ok(())
    }

    /// gesture started
    ///
    /// This event is sent when the required finger_count number of touch points
    /// are active and all threshold requirements are met. After this event is
    /// sent, no other gesture will be triggered until all touch points are
    /// released and the river_touch_gesture_v1.end event is sent.
    ///
    /// If the threshold requirements for multiple river_touch_gesture_v1
    /// objects are met at the same time, it is compositor policy which gesture
    /// is triggered.
    ///
    /// The wl_touch.cancel event is sent to all surfaces with touch focus, all
    /// touch input is eaten till end of gesture.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn send_start(
        &self,
    ) {
        let res = self.try_send_start(
        );
        if let Err(e) = res {
            log_send("river_touch_gesture_v1.start", &e);
        }
    }

    /// Since when the end message is available.
    pub const MSG__END__SINCE: u32 = 1;

    /// gesture ended
    ///
    /// All touch points have been released and the gesture is ended.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn try_send_end(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_touch_gesture_v1#{}.end()\n", client_id, id);
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

    /// gesture ended
    ///
    /// All touch points have been released and the gesture is ended.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn send_end(
        &self,
    ) {
        let res = self.try_send_end(
        );
        if let Err(e) = res {
            log_send("river_touch_gesture_v1.end", &e);
        }
    }

    /// Since when the cancel message is available.
    pub const MSG__CANCEL__SINCE: u32 = 1;

    /// gesture canceled
    ///
    /// The gesture is canceled, for example because the hardware palm detection
    /// decided that the touch input should have been ignored.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_touch_gesture_v1#{}.cancel()\n", client_id, id);
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

    /// gesture canceled
    ///
    /// The gesture is canceled, for example because the hardware palm detection
    /// decided that the touch input should have been ignored.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn send_cancel(
        &self,
    ) {
        let res = self.try_send_cancel(
        );
        if let Err(e) = res {
            log_send("river_touch_gesture_v1.cancel", &e);
        }
    }

    /// Since when the finger_count message is available.
    pub const MSG__FINGER_COUNT__SINCE: u32 = 1;

    /// number of touch points changed
    ///
    /// The number of active touch points changed since the gesture was started.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `finger_count`: number of touch points
    #[inline]
    pub fn try_send_finger_count(
        &self,
        finger_count: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            finger_count,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_touch_gesture_v1#{}.finger_count(finger_count: {})\n", client_id, id, arg0);
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

    /// number of touch points changed
    ///
    /// The number of active touch points changed since the gesture was started.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `finger_count`: number of touch points
    #[inline]
    pub fn send_finger_count(
        &self,
        finger_count: u32,
    ) {
        let res = self.try_send_finger_count(
            finger_count,
        );
        if let Err(e) = res {
            log_send("river_touch_gesture_v1.finger_count", &e);
        }
    }

    /// Since when the delta_motion message is available.
    pub const MSG__DELTA_MOTION__SINCE: u32 = 1;

    /// total change in centroid position
    ///
    /// The total change in centroid position since gesture start.
    ///
    /// The centroid is defined as the mean position of all touch points.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `dx`: centroid change in x
    /// - `dy`: centroid change in y
    #[inline]
    pub fn try_send_delta_motion(
        &self,
        dx: i32,
        dy: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            dx,
            dy,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_touch_gesture_v1#{}.delta_motion(dx: {}, dy: {})\n", client_id, id, arg0, arg1);
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
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// total change in centroid position
    ///
    /// The total change in centroid position since gesture start.
    ///
    /// The centroid is defined as the mean position of all touch points.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `dx`: centroid change in x
    /// - `dy`: centroid change in y
    #[inline]
    pub fn send_delta_motion(
        &self,
        dx: i32,
        dy: i32,
    ) {
        let res = self.try_send_delta_motion(
            dx,
            dy,
        );
        if let Err(e) = res {
            log_send("river_touch_gesture_v1.delta_motion", &e);
        }
    }

    /// Since when the scale message is available.
    pub const MSG__SCALE__SINCE: u32 = 1;

    /// current pinch scale
    ///
    /// The pinch distance is defined as the diagonal of the bounding box of all
    /// touch points. The pinch scale is defined as the ratio of the current
    /// pinch distance to the distance at gesture start.
    ///
    /// Thus, the scale is initially 1.0. If the touch points are moved closer
    /// together, the scale decreases and if the touch points are moved further
    /// apart it increases.
    ///
    /// This event is only sent if there are 2 or more touch points.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `scale`: current scale
    #[inline]
    pub fn try_send_scale(
        &self,
        scale: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            scale,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: Fixed) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_touch_gesture_v1#{}.scale(scale: {})\n", client_id, id, arg0);
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
            5,
            arg0.to_wire() as u32,
        ]);
        Ok(())
    }

    /// current pinch scale
    ///
    /// The pinch distance is defined as the diagonal of the bounding box of all
    /// touch points. The pinch scale is defined as the ratio of the current
    /// pinch distance to the distance at gesture start.
    ///
    /// Thus, the scale is initially 1.0. If the touch points are moved closer
    /// together, the scale decreases and if the touch points are moved further
    /// apart it increases.
    ///
    /// This event is only sent if there are 2 or more touch points.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `scale`: current scale
    #[inline]
    pub fn send_scale(
        &self,
        scale: Fixed,
    ) {
        let res = self.try_send_scale(
            scale,
        );
        if let Err(e) = res {
            log_send("river_touch_gesture_v1.scale", &e);
        }
    }

    /// Since when the set_threshold_motion message is available.
    pub const MSG__SET_THRESHOLD_MOTION__SINCE: u32 = 1;

    /// set centroid motion threshold to trigger
    ///
    /// Set the minimum centroid motion required to trigger the gesture.
    ///
    /// Distance is measured between the current centroid and the centroid of
    /// the touch down events.
    ///
    /// If this request is never made, the threshold is considered to be 0.
    ///
    /// The threshold argument must be greater than or equal to 0.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `min_distance`: minimum distance to trigger
    #[inline]
    pub fn try_send_set_threshold_motion(
        &self,
        min_distance: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            min_distance,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_touch_gesture_v1#{}.set_threshold_motion(min_distance: {})\n", id, arg0);
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
            3,
            arg0 as u32,
        ]);
        Ok(())
    }

    /// set centroid motion threshold to trigger
    ///
    /// Set the minimum centroid motion required to trigger the gesture.
    ///
    /// Distance is measured between the current centroid and the centroid of
    /// the touch down events.
    ///
    /// If this request is never made, the threshold is considered to be 0.
    ///
    /// The threshold argument must be greater than or equal to 0.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `min_distance`: minimum distance to trigger
    #[inline]
    pub fn send_set_threshold_motion(
        &self,
        min_distance: i32,
    ) {
        let res = self.try_send_set_threshold_motion(
            min_distance,
        );
        if let Err(e) = res {
            log_send("river_touch_gesture_v1.set_threshold_motion", &e);
        }
    }

    /// Since when the set_direction message is available.
    pub const MSG__SET_DIRECTION__SINCE: u32 = 1;

    /// set centroid motion direction to trigger
    ///
    /// Set the direction and minimum centroid motion distance required to
    /// trigger the gesture.
    ///
    /// If direction is none, there is no directional motion required to trigger
    /// the gesture and the min_distance argument is ignored.
    ///
    /// If this request is never made, the direction is considered to be none.
    ///
    /// The min_distance argument must be greater than or equal to 0.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `direction`: motion direction
    /// - `min_distance`: minimum distance to trigger
    #[inline]
    pub fn try_send_set_direction(
        &self,
        direction: RiverTouchGestureV1Direction,
        min_distance: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            direction,
            min_distance,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: RiverTouchGestureV1Direction, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_touch_gesture_v1#{}.set_direction(direction: {:?}, min_distance: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1);
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
            4,
            arg0.0,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// set centroid motion direction to trigger
    ///
    /// Set the direction and minimum centroid motion distance required to
    /// trigger the gesture.
    ///
    /// If direction is none, there is no directional motion required to trigger
    /// the gesture and the min_distance argument is ignored.
    ///
    /// If this request is never made, the direction is considered to be none.
    ///
    /// The min_distance argument must be greater than or equal to 0.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `direction`: motion direction
    /// - `min_distance`: minimum distance to trigger
    #[inline]
    pub fn send_set_direction(
        &self,
        direction: RiverTouchGestureV1Direction,
        min_distance: i32,
    ) {
        let res = self.try_send_set_direction(
            direction,
            min_distance,
        );
        if let Err(e) = res {
            log_send("river_touch_gesture_v1.set_direction", &e);
        }
    }

    /// Since when the set_threshold_scale message is available.
    pub const MSG__SET_THRESHOLD_SCALE__SINCE: u32 = 1;

    /// set pinch scale threshold to trigger
    ///
    /// Set the pinch scale threshold required to trigger the gesture.
    ///
    /// If threshold_scale is less than 1.0 then the scale must be less than
    /// threshold_scale for the gesture to be triggered. If threshold_scale is
    /// greater than 1.0 then the scale must be greater than threshold_scale for
    /// the gesture to be triggered.
    ///
    /// If this request is never made, threshold_scale is considered to be 1.0.
    ///
    /// This request is ignored if the gesture's finger_count is less than 2.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `r#in`: inward pinch scale threshold
    /// - `out`: outward pinch scale threshold
    #[inline]
    pub fn try_send_set_threshold_scale(
        &self,
        r#in: Fixed,
        out: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            r#in,
            out,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: Fixed, arg1: Fixed) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_touch_gesture_v1#{}.set_threshold_scale(in: {}, out: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1);
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
            5,
            arg0.to_wire() as u32,
            arg1.to_wire() as u32,
        ]);
        Ok(())
    }

    /// set pinch scale threshold to trigger
    ///
    /// Set the pinch scale threshold required to trigger the gesture.
    ///
    /// If threshold_scale is less than 1.0 then the scale must be less than
    /// threshold_scale for the gesture to be triggered. If threshold_scale is
    /// greater than 1.0 then the scale must be greater than threshold_scale for
    /// the gesture to be triggered.
    ///
    /// If this request is never made, threshold_scale is considered to be 1.0.
    ///
    /// This request is ignored if the gesture's finger_count is less than 2.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `r#in`: inward pinch scale threshold
    /// - `out`: outward pinch scale threshold
    #[inline]
    pub fn send_set_threshold_scale(
        &self,
        r#in: Fixed,
        out: Fixed,
    ) {
        let res = self.try_send_set_threshold_scale(
            r#in,
            out,
        );
        if let Err(e) = res {
            log_send("river_touch_gesture_v1.set_threshold_scale", &e);
        }
    }

    /// Since when the set_edge message is available.
    pub const MSG__SET_EDGE__SINCE: u32 = 1;

    /// set edge from which the gesture must start
    ///
    /// Set the touchscreen edge from which the gesture must start and the
    /// maximum allowed distance from that edge.
    ///
    /// If edge is none, then there is no required edge to start from and the
    /// max_distance argument is ignored.
    ///
    /// If this request is never made, the edge is considered to be none.
    ///
    /// The max_distance argument must be greater than or equal to 0.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `edge`: edge to start from
    /// - `max_distance`: max distance from edge
    #[inline]
    pub fn try_send_set_edge(
        &self,
        edge: RiverTouchGestureV1Edge,
        max_distance: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            edge,
            max_distance,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: RiverTouchGestureV1Edge, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_touch_gesture_v1#{}.set_edge(edge: {:?}, max_distance: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1);
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
            6,
            arg0.0,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// set edge from which the gesture must start
    ///
    /// Set the touchscreen edge from which the gesture must start and the
    /// maximum allowed distance from that edge.
    ///
    /// If edge is none, then there is no required edge to start from and the
    /// max_distance argument is ignored.
    ///
    /// If this request is never made, the edge is considered to be none.
    ///
    /// The max_distance argument must be greater than or equal to 0.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `edge`: edge to start from
    /// - `max_distance`: max distance from edge
    #[inline]
    pub fn send_set_edge(
        &self,
        edge: RiverTouchGestureV1Edge,
        max_distance: i32,
    ) {
        let res = self.try_send_set_edge(
            edge,
            max_distance,
        );
        if let Err(e) = res {
            log_send("river_touch_gesture_v1.set_edge", &e);
        }
    }
}

/// A message handler for [`RiverTouchGestureV1`] proxies.
pub trait RiverTouchGestureV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<RiverTouchGestureV1>) {
        slf.core.delete_id();
    }

    /// destroy the touch gesture object
    ///
    /// This request indicates that the client will no longer use the object and
    /// that it may be safely destroyed.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<RiverTouchGestureV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("river_touch_gesture_v1.destroy", &e);
        }
    }

    /// enable the gesture
    ///
    /// This request should be made after all initial configuration has been
    /// completed and the window manager wishes the gesture to be able to be
    /// triggered.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_enable(
        &mut self,
        slf: &Rc<RiverTouchGestureV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_enable(
        );
        if let Err(e) = res {
            log_forward("river_touch_gesture_v1.enable", &e);
        }
    }

    /// disable the gesture
    ///
    /// This request may be used to temporarily disable the gesture. It may
    /// be later re-enabled with the enable request.
    ///
    /// This request does not affect an in progress gesture that has already
    /// been started. It will prevent the gesture from being triggered again.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_disable(
        &mut self,
        slf: &Rc<RiverTouchGestureV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_disable(
        );
        if let Err(e) = res {
            log_forward("river_touch_gesture_v1.disable", &e);
        }
    }

    /// gesture started
    ///
    /// This event is sent when the required finger_count number of touch points
    /// are active and all threshold requirements are met. After this event is
    /// sent, no other gesture will be triggered until all touch points are
    /// released and the river_touch_gesture_v1.end event is sent.
    ///
    /// If the threshold requirements for multiple river_touch_gesture_v1
    /// objects are met at the same time, it is compositor policy which gesture
    /// is triggered.
    ///
    /// The wl_touch.cancel event is sent to all surfaces with touch focus, all
    /// touch input is eaten till end of gesture.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    fn handle_start(
        &mut self,
        slf: &Rc<RiverTouchGestureV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_start(
        );
        if let Err(e) = res {
            log_forward("river_touch_gesture_v1.start", &e);
        }
    }

    /// gesture ended
    ///
    /// All touch points have been released and the gesture is ended.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    fn handle_end(
        &mut self,
        slf: &Rc<RiverTouchGestureV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_end(
        );
        if let Err(e) = res {
            log_forward("river_touch_gesture_v1.end", &e);
        }
    }

    /// gesture canceled
    ///
    /// The gesture is canceled, for example because the hardware palm detection
    /// decided that the touch input should have been ignored.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    fn handle_cancel(
        &mut self,
        slf: &Rc<RiverTouchGestureV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_cancel(
        );
        if let Err(e) = res {
            log_forward("river_touch_gesture_v1.cancel", &e);
        }
    }

    /// number of touch points changed
    ///
    /// The number of active touch points changed since the gesture was started.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `finger_count`: number of touch points
    #[inline]
    fn handle_finger_count(
        &mut self,
        slf: &Rc<RiverTouchGestureV1>,
        finger_count: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_finger_count(
            finger_count,
        );
        if let Err(e) = res {
            log_forward("river_touch_gesture_v1.finger_count", &e);
        }
    }

    /// total change in centroid position
    ///
    /// The total change in centroid position since gesture start.
    ///
    /// The centroid is defined as the mean position of all touch points.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `dx`: centroid change in x
    /// - `dy`: centroid change in y
    #[inline]
    fn handle_delta_motion(
        &mut self,
        slf: &Rc<RiverTouchGestureV1>,
        dx: i32,
        dy: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_delta_motion(
            dx,
            dy,
        );
        if let Err(e) = res {
            log_forward("river_touch_gesture_v1.delta_motion", &e);
        }
    }

    /// current pinch scale
    ///
    /// The pinch distance is defined as the diagonal of the bounding box of all
    /// touch points. The pinch scale is defined as the ratio of the current
    /// pinch distance to the distance at gesture start.
    ///
    /// Thus, the scale is initially 1.0. If the touch points are moved closer
    /// together, the scale decreases and if the touch points are moved further
    /// apart it increases.
    ///
    /// This event is only sent if there are 2 or more touch points.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `scale`: current scale
    #[inline]
    fn handle_scale(
        &mut self,
        slf: &Rc<RiverTouchGestureV1>,
        scale: Fixed,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_scale(
            scale,
        );
        if let Err(e) = res {
            log_forward("river_touch_gesture_v1.scale", &e);
        }
    }

    /// set centroid motion threshold to trigger
    ///
    /// Set the minimum centroid motion required to trigger the gesture.
    ///
    /// Distance is measured between the current centroid and the centroid of
    /// the touch down events.
    ///
    /// If this request is never made, the threshold is considered to be 0.
    ///
    /// The threshold argument must be greater than or equal to 0.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `min_distance`: minimum distance to trigger
    #[inline]
    fn handle_set_threshold_motion(
        &mut self,
        slf: &Rc<RiverTouchGestureV1>,
        min_distance: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_threshold_motion(
            min_distance,
        );
        if let Err(e) = res {
            log_forward("river_touch_gesture_v1.set_threshold_motion", &e);
        }
    }

    /// set centroid motion direction to trigger
    ///
    /// Set the direction and minimum centroid motion distance required to
    /// trigger the gesture.
    ///
    /// If direction is none, there is no directional motion required to trigger
    /// the gesture and the min_distance argument is ignored.
    ///
    /// If this request is never made, the direction is considered to be none.
    ///
    /// The min_distance argument must be greater than or equal to 0.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `direction`: motion direction
    /// - `min_distance`: minimum distance to trigger
    #[inline]
    fn handle_set_direction(
        &mut self,
        slf: &Rc<RiverTouchGestureV1>,
        direction: RiverTouchGestureV1Direction,
        min_distance: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_direction(
            direction,
            min_distance,
        );
        if let Err(e) = res {
            log_forward("river_touch_gesture_v1.set_direction", &e);
        }
    }

    /// set pinch scale threshold to trigger
    ///
    /// Set the pinch scale threshold required to trigger the gesture.
    ///
    /// If threshold_scale is less than 1.0 then the scale must be less than
    /// threshold_scale for the gesture to be triggered. If threshold_scale is
    /// greater than 1.0 then the scale must be greater than threshold_scale for
    /// the gesture to be triggered.
    ///
    /// If this request is never made, threshold_scale is considered to be 1.0.
    ///
    /// This request is ignored if the gesture's finger_count is less than 2.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `r#in`: inward pinch scale threshold
    /// - `out`: outward pinch scale threshold
    #[inline]
    fn handle_set_threshold_scale(
        &mut self,
        slf: &Rc<RiverTouchGestureV1>,
        r#in: Fixed,
        out: Fixed,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_threshold_scale(
            r#in,
            out,
        );
        if let Err(e) = res {
            log_forward("river_touch_gesture_v1.set_threshold_scale", &e);
        }
    }

    /// set edge from which the gesture must start
    ///
    /// Set the touchscreen edge from which the gesture must start and the
    /// maximum allowed distance from that edge.
    ///
    /// If edge is none, then there is no required edge to start from and the
    /// max_distance argument is ignored.
    ///
    /// If this request is never made, the edge is considered to be none.
    ///
    /// The max_distance argument must be greater than or equal to 0.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `edge`: edge to start from
    /// - `max_distance`: max distance from edge
    #[inline]
    fn handle_set_edge(
        &mut self,
        slf: &Rc<RiverTouchGestureV1>,
        edge: RiverTouchGestureV1Edge,
        max_distance: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_edge(
            edge,
            max_distance,
        );
        if let Err(e) = res {
            log_forward("river_touch_gesture_v1.set_edge", &e);
        }
    }
}

impl ObjectPrivate for RiverTouchGestureV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::RiverTouchGestureV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_touch_gesture_v1#{}.destroy()\n", client_id, id);
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
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_touch_gesture_v1#{}.enable()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_enable(&self);
                } else {
                    DefaultHandler.handle_enable(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_touch_gesture_v1#{}.disable()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_disable(&self);
                } else {
                    DefaultHandler.handle_disable(&self);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = arg0 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_touch_gesture_v1#{}.set_threshold_motion(min_distance: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_threshold_motion(&self, arg0);
                } else {
                    DefaultHandler.handle_set_threshold_motion(&self, arg0);
                }
            }
            4 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = RiverTouchGestureV1Direction(arg0);
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: RiverTouchGestureV1Direction, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_touch_gesture_v1#{}.set_direction(direction: {:?}, min_distance: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_direction(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_direction(&self, arg0, arg1);
                }
            }
            5 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = Fixed::from_wire(arg0 as i32);
                let arg1 = Fixed::from_wire(arg1 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: Fixed, arg1: Fixed) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_touch_gesture_v1#{}.set_threshold_scale(in: {}, out: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_threshold_scale(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_threshold_scale(&self, arg0, arg1);
                }
            }
            6 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = RiverTouchGestureV1Edge(arg0);
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: RiverTouchGestureV1Edge, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_touch_gesture_v1#{}.set_edge(edge: {:?}, max_distance: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_edge(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_edge(&self, arg0, arg1);
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
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_touch_gesture_v1#{}.start()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_start(&self);
                } else {
                    DefaultHandler.handle_start(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_touch_gesture_v1#{}.end()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_end(&self);
                } else {
                    DefaultHandler.handle_end(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_touch_gesture_v1#{}.cancel()\n", id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_touch_gesture_v1#{}.finger_count(finger_count: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_finger_count(&self, arg0);
                } else {
                    DefaultHandler.handle_finger_count(&self, arg0);
                }
            }
            4 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_touch_gesture_v1#{}.delta_motion(dx: {}, dy: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_delta_motion(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_delta_motion(&self, arg0, arg1);
                }
            }
            5 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = Fixed::from_wire(arg0 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: Fixed) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_touch_gesture_v1#{}.scale(scale: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_scale(&self, arg0);
                } else {
                    DefaultHandler.handle_scale(&self, arg0);
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
            1 => "enable",
            2 => "disable",
            3 => "set_threshold_motion",
            4 => "set_direction",
            5 => "set_threshold_scale",
            6 => "set_edge",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "start",
            1 => "end",
            2 => "cancel",
            3 => "finger_count",
            4 => "delta_motion",
            5 => "scale",
            _ => return None,
        };
        Some(name)
    }

    fn create_zombie(&self) -> Rc<dyn Object> {
        let slf = Self::new(&self.core.state, self.core.version);
        slf.core.make_zombie();
        slf
    }
}

impl Object for RiverTouchGestureV1 {
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

impl RiverTouchGestureV1 {
    /// Since when the error.invalid_distance enum variant is available.
    pub const ENM__ERROR_INVALID_DISTANCE__SINCE: u32 = 1;
    /// Since when the error.invalid_direction enum variant is available.
    pub const ENM__ERROR_INVALID_DIRECTION__SINCE: u32 = 1;
    /// Since when the error.invalid_edge enum variant is available.
    pub const ENM__ERROR_INVALID_EDGE__SINCE: u32 = 1;

    /// Since when the direction.none enum variant is available.
    pub const ENM__DIRECTION_NONE__SINCE: u32 = 1;
    /// Since when the direction.up enum variant is available.
    pub const ENM__DIRECTION_UP__SINCE: u32 = 1;
    /// Since when the direction.down enum variant is available.
    pub const ENM__DIRECTION_DOWN__SINCE: u32 = 1;
    /// Since when the direction.left enum variant is available.
    pub const ENM__DIRECTION_LEFT__SINCE: u32 = 1;
    /// Since when the direction.right enum variant is available.
    pub const ENM__DIRECTION_RIGHT__SINCE: u32 = 1;

    /// Since when the edge.none enum variant is available.
    pub const ENM__EDGE_NONE__SINCE: u32 = 1;
    /// Since when the edge.top enum variant is available.
    pub const ENM__EDGE_TOP__SINCE: u32 = 1;
    /// Since when the edge.bottom enum variant is available.
    pub const ENM__EDGE_BOTTOM__SINCE: u32 = 1;
    /// Since when the edge.left enum variant is available.
    pub const ENM__EDGE_LEFT__SINCE: u32 = 1;
    /// Since when the edge.right enum variant is available.
    pub const ENM__EDGE_RIGHT__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverTouchGestureV1Error(pub u32);

impl RiverTouchGestureV1Error {
    pub const INVALID_DISTANCE: Self = Self(0);

    pub const INVALID_DIRECTION: Self = Self(1);

    pub const INVALID_EDGE: Self = Self(2);
}

impl Debug for RiverTouchGestureV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_DISTANCE => "INVALID_DISTANCE",
            Self::INVALID_DIRECTION => "INVALID_DIRECTION",
            Self::INVALID_EDGE => "INVALID_EDGE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverTouchGestureV1Direction(pub u32);

impl RiverTouchGestureV1Direction {
    pub const NONE: Self = Self(0);

    pub const UP: Self = Self(1);

    pub const DOWN: Self = Self(2);

    pub const LEFT: Self = Self(3);

    pub const RIGHT: Self = Self(4);
}

impl Debug for RiverTouchGestureV1Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NONE => "NONE",
            Self::UP => "UP",
            Self::DOWN => "DOWN",
            Self::LEFT => "LEFT",
            Self::RIGHT => "RIGHT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverTouchGestureV1Edge(pub u32);

impl RiverTouchGestureV1Edge {
    pub const NONE: Self = Self(0);

    pub const TOP: Self = Self(1);

    pub const BOTTOM: Self = Self(2);

    pub const LEFT: Self = Self(3);

    pub const RIGHT: Self = Self(4);
}

impl Debug for RiverTouchGestureV1Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NONE => "NONE",
            Self::TOP => "TOP",
            Self::BOTTOM => "BOTTOM",
            Self::LEFT => "LEFT",
            Self::RIGHT => "RIGHT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
