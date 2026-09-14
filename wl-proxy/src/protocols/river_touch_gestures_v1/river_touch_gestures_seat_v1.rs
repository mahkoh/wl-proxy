//! touch gestures seat
//!
//! This object manages touch gesture state associated with a specific seat.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A river_touch_gestures_seat_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct RiverTouchGesturesSeatV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn RiverTouchGesturesSeatV1Handler>,
}

struct DefaultHandler;

impl RiverTouchGesturesSeatV1Handler for DefaultHandler { }

impl ConcreteObject for RiverTouchGesturesSeatV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::RiverTouchGesturesSeatV1;
    const INTERFACE_NAME: &str = "river_touch_gestures_seat_v1";
}

impl RiverTouchGesturesSeatV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl RiverTouchGesturesSeatV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn RiverTouchGesturesSeatV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for RiverTouchGesturesSeatV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RiverTouchGesturesSeatV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl RiverTouchGesturesSeatV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the gestures seat object
    ///
    /// This request indicates that the client will no longer use the gestures
    /// seat object and that it may be safely destroyed.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_touch_gestures_seat_v1#{}.destroy()\n", id);
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

    /// destroy the gestures seat object
    ///
    /// This request indicates that the client will no longer use the gestures
    /// seat object and that it may be safely destroyed.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("river_touch_gestures_seat_v1.destroy", &e);
        }
    }

    /// Since when the set_arbitration_timeout message is available.
    pub const MSG__SET_ARBITRATION_TIMEOUT__SINCE: u32 = 1;

    /// define a new gesture
    ///
    /// The arbitration timeout defines the maximum time after the first touch
    /// down event the compositor should wait before deciding that the touch
    /// sequence is not a gesture and instead routing the touch input to the
    /// respective clients through the wl_touch interface.
    ///
    /// This timeout should be short enough that it subjectively does not
    /// negatively impact the responsiveness. The default if this request is
    /// never made for the river_touch_gestures_seat_v1 is 100 milliseconds.
    ///
    /// Setting the timeout to 0 milliseconds effectively disables all gestures
    /// with e.g. a finger count greater than 1 or with a non-zero motion
    /// threshold. However, a gesture with finger count 1 and edge trigger area
    /// defined with the river_touch_gesture_v1.set_edge request could still be
    /// triggered even with a 0 millisecond arbitration timeout.
    ///
    /// This request may be made at any time but will only take effect on the
    /// next touch sequence to be started.
    ///
    /// # Arguments
    ///
    /// - `msec`: arbitration timeout in milliseconds
    #[inline]
    pub fn try_send_set_arbitration_timeout(
        &self,
        msec: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            msec,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_touch_gestures_seat_v1#{}.set_arbitration_timeout(msec: {})\n", id, arg0);
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

    /// define a new gesture
    ///
    /// The arbitration timeout defines the maximum time after the first touch
    /// down event the compositor should wait before deciding that the touch
    /// sequence is not a gesture and instead routing the touch input to the
    /// respective clients through the wl_touch interface.
    ///
    /// This timeout should be short enough that it subjectively does not
    /// negatively impact the responsiveness. The default if this request is
    /// never made for the river_touch_gestures_seat_v1 is 100 milliseconds.
    ///
    /// Setting the timeout to 0 milliseconds effectively disables all gestures
    /// with e.g. a finger count greater than 1 or with a non-zero motion
    /// threshold. However, a gesture with finger count 1 and edge trigger area
    /// defined with the river_touch_gesture_v1.set_edge request could still be
    /// triggered even with a 0 millisecond arbitration timeout.
    ///
    /// This request may be made at any time but will only take effect on the
    /// next touch sequence to be started.
    ///
    /// # Arguments
    ///
    /// - `msec`: arbitration timeout in milliseconds
    #[inline]
    pub fn send_set_arbitration_timeout(
        &self,
        msec: u32,
    ) {
        let res = self.try_send_set_arbitration_timeout(
            msec,
        );
        if let Err(e) = res {
            log_send("river_touch_gestures_seat_v1.set_arbitration_timeout", &e);
        }
    }

    /// Since when the get_gesture message is available.
    pub const MSG__GET_GESTURE__SINCE: u32 = 1;

    /// define a new gesture
    ///
    /// Define a new touch gesture.
    ///
    /// The finger_count argument defines the number of simultaneous touch
    /// points necessary to trigger the gesture. The finger_count must be
    /// greater than 0.
    ///
    /// The maximum finger count supported is limited by touchscreen hardware.
    /// If the requested finger count is greater than the maximum supported,
    /// the gesture will never be triggered.
    ///
    /// The new gesture is not enabled until the river_touch_gesture_v1.enable
    /// request is made during a manage sequence.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `finger_count`: number of touch points
    #[inline]
    pub fn try_send_get_gesture(
        &self,
        id: &Rc<RiverTouchGestureV1>,
        finger_count: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            finger_count,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_touch_gestures_seat_v1#{}.get_gesture(id: river_touch_gesture_v1#{}, finger_count: {})\n", id, arg0, arg1);
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

    /// define a new gesture
    ///
    /// Define a new touch gesture.
    ///
    /// The finger_count argument defines the number of simultaneous touch
    /// points necessary to trigger the gesture. The finger_count must be
    /// greater than 0.
    ///
    /// The maximum finger count supported is limited by touchscreen hardware.
    /// If the requested finger count is greater than the maximum supported,
    /// the gesture will never be triggered.
    ///
    /// The new gesture is not enabled until the river_touch_gesture_v1.enable
    /// request is made during a manage sequence.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `finger_count`: number of touch points
    #[inline]
    pub fn send_get_gesture(
        &self,
        id: &Rc<RiverTouchGestureV1>,
        finger_count: u32,
    ) {
        let res = self.try_send_get_gesture(
            id,
            finger_count,
        );
        if let Err(e) = res {
            log_send("river_touch_gestures_seat_v1.get_gesture", &e);
        }
    }

    /// define a new gesture
    ///
    /// Define a new touch gesture.
    ///
    /// The finger_count argument defines the number of simultaneous touch
    /// points necessary to trigger the gesture. The finger_count must be
    /// greater than 0.
    ///
    /// The maximum finger count supported is limited by touchscreen hardware.
    /// If the requested finger count is greater than the maximum supported,
    /// the gesture will never be triggered.
    ///
    /// The new gesture is not enabled until the river_touch_gesture_v1.enable
    /// request is made during a manage sequence.
    ///
    /// # Arguments
    ///
    /// - `finger_count`: number of touch points
    #[inline]
    pub fn new_try_send_get_gesture(
        &self,
        finger_count: u32,
    ) -> Result<Rc<RiverTouchGestureV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_gesture(
            &id,
            finger_count,
        )?;
        Ok(id)
    }

    /// define a new gesture
    ///
    /// Define a new touch gesture.
    ///
    /// The finger_count argument defines the number of simultaneous touch
    /// points necessary to trigger the gesture. The finger_count must be
    /// greater than 0.
    ///
    /// The maximum finger count supported is limited by touchscreen hardware.
    /// If the requested finger count is greater than the maximum supported,
    /// the gesture will never be triggered.
    ///
    /// The new gesture is not enabled until the river_touch_gesture_v1.enable
    /// request is made during a manage sequence.
    ///
    /// # Arguments
    ///
    /// - `finger_count`: number of touch points
    #[inline]
    pub fn new_send_get_gesture(
        &self,
        finger_count: u32,
    ) -> Rc<RiverTouchGestureV1> {
        let id = self.core.create_child();
        self.send_get_gesture(
            &id,
            finger_count,
        );
        id
    }
}

/// A message handler for [`RiverTouchGesturesSeatV1`] proxies.
pub trait RiverTouchGesturesSeatV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<RiverTouchGesturesSeatV1>) {
        slf.core.delete_id();
    }

    /// destroy the gestures seat object
    ///
    /// This request indicates that the client will no longer use the gestures
    /// seat object and that it may be safely destroyed.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<RiverTouchGesturesSeatV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("river_touch_gestures_seat_v1.destroy", &e);
        }
    }

    /// define a new gesture
    ///
    /// The arbitration timeout defines the maximum time after the first touch
    /// down event the compositor should wait before deciding that the touch
    /// sequence is not a gesture and instead routing the touch input to the
    /// respective clients through the wl_touch interface.
    ///
    /// This timeout should be short enough that it subjectively does not
    /// negatively impact the responsiveness. The default if this request is
    /// never made for the river_touch_gestures_seat_v1 is 100 milliseconds.
    ///
    /// Setting the timeout to 0 milliseconds effectively disables all gestures
    /// with e.g. a finger count greater than 1 or with a non-zero motion
    /// threshold. However, a gesture with finger count 1 and edge trigger area
    /// defined with the river_touch_gesture_v1.set_edge request could still be
    /// triggered even with a 0 millisecond arbitration timeout.
    ///
    /// This request may be made at any time but will only take effect on the
    /// next touch sequence to be started.
    ///
    /// # Arguments
    ///
    /// - `msec`: arbitration timeout in milliseconds
    #[inline]
    fn handle_set_arbitration_timeout(
        &mut self,
        slf: &Rc<RiverTouchGesturesSeatV1>,
        msec: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_arbitration_timeout(
            msec,
        );
        if let Err(e) = res {
            log_forward("river_touch_gestures_seat_v1.set_arbitration_timeout", &e);
        }
    }

    /// define a new gesture
    ///
    /// Define a new touch gesture.
    ///
    /// The finger_count argument defines the number of simultaneous touch
    /// points necessary to trigger the gesture. The finger_count must be
    /// greater than 0.
    ///
    /// The maximum finger count supported is limited by touchscreen hardware.
    /// If the requested finger count is greater than the maximum supported,
    /// the gesture will never be triggered.
    ///
    /// The new gesture is not enabled until the river_touch_gesture_v1.enable
    /// request is made during a manage sequence.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `finger_count`: number of touch points
    #[inline]
    fn handle_get_gesture(
        &mut self,
        slf: &Rc<RiverTouchGesturesSeatV1>,
        id: &Rc<RiverTouchGestureV1>,
        finger_count: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_gesture(
            id,
            finger_count,
        );
        if let Err(e) = res {
            log_forward("river_touch_gestures_seat_v1.get_gesture", &e);
        }
    }
}

impl ObjectPrivate for RiverTouchGesturesSeatV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::RiverTouchGesturesSeatV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_touch_gestures_seat_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_touch_gestures_seat_v1#{}.set_arbitration_timeout(msec: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_arbitration_timeout(&self, arg0);
                } else {
                    DefaultHandler.handle_set_arbitration_timeout(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_touch_gestures_seat_v1#{}.get_gesture(id: river_touch_gesture_v1#{}, finger_count: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverTouchGestureV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_get_gesture(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_gesture(&self, arg0, arg1);
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
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            1 => "set_arbitration_timeout",
            2 => "get_gesture",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }

    fn create_zombie(&self) -> Rc<dyn Object> {
        let slf = Self::new(&self.core.state, self.core.version);
        slf.core.make_zombie();
        slf
    }
}

impl Object for RiverTouchGesturesSeatV1 {
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

impl RiverTouchGesturesSeatV1 {
    /// Since when the error.invalid_finger_count enum variant is available.
    pub const ENM__ERROR_INVALID_FINGER_COUNT__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverTouchGesturesSeatV1Error(pub u32);

impl RiverTouchGesturesSeatV1Error {
    pub const INVALID_FINGER_COUNT: Self = Self(0);
}

impl Debug for RiverTouchGesturesSeatV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_FINGER_COUNT => "INVALID_FINGER_COUNT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
