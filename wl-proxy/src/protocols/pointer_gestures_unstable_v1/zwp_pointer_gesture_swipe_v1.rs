//! a swipe gesture object
//!
//! A swipe gesture object notifies a client about a multi-finger swipe
//! gesture detected on an indirect input device such as a touchpad.
//! The gesture is usually initiated by multiple fingers moving in the
//! same direction but once initiated the direction may change.
//! The precise conditions of when such a gesture is detected are
//! implementation-dependent.
//!
//! A gesture consists of three stages: begin, update (optional) and end.
//! There cannot be multiple simultaneous hold, pinch or swipe gestures on a
//! same pointer/seat, how compositors prevent these situations is
//! implementation-dependent.
//!
//! A gesture may be cancelled by the compositor or the hardware.
//! Clients should not consider performing permanent or irreversible
//! actions until the end of a gesture has been received.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_pointer_gesture_swipe_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpPointerGestureSwipeV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpPointerGestureSwipeV1Handler>,
}

struct DefaultHandler;

impl ZwpPointerGestureSwipeV1Handler for DefaultHandler { }

impl ConcreteObject for ZwpPointerGestureSwipeV1 {
    const XML_VERSION: u32 = 3;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpPointerGestureSwipeV1;
    const INTERFACE_NAME: &str = "zwp_pointer_gesture_swipe_v1";
}

impl ZwpPointerGestureSwipeV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpPointerGestureSwipeV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpPointerGestureSwipeV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpPointerGestureSwipeV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpPointerGestureSwipeV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpPointerGestureSwipeV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the pointer swipe gesture object
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_pointer_gesture_swipe_v1#{}.destroy()\n", id);
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

    /// destroy the pointer swipe gesture object
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwp_pointer_gesture_swipe_v1.destroy", &e);
        }
    }

    /// Since when the begin message is available.
    pub const MSG__BEGIN__SINCE: u32 = 1;

    /// multi-finger swipe begin
    ///
    /// This event is sent when a multi-finger swipe gesture is detected
    /// on the device.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    /// - `time`: timestamp with millisecond granularity
    /// - `surface`:
    /// - `fingers`: number of fingers
    #[inline]
    pub fn try_send_begin(
        &self,
        serial: u32,
        time: u32,
        surface: &Rc<WlSurface>,
        fingers: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            serial,
            time,
            surface,
            fingers,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_pointer_gesture_swipe_v1#{}.begin(serial: {}, time: {}, surface: wl_surface#{}, fingers: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1, arg2_id, arg3);
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
            arg3,
        ]);
        Ok(())
    }

    /// multi-finger swipe begin
    ///
    /// This event is sent when a multi-finger swipe gesture is detected
    /// on the device.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    /// - `time`: timestamp with millisecond granularity
    /// - `surface`:
    /// - `fingers`: number of fingers
    #[inline]
    pub fn send_begin(
        &self,
        serial: u32,
        time: u32,
        surface: &Rc<WlSurface>,
        fingers: u32,
    ) {
        let res = self.try_send_begin(
            serial,
            time,
            surface,
            fingers,
        );
        if let Err(e) = res {
            log_send("zwp_pointer_gesture_swipe_v1.begin", &e);
        }
    }

    /// Since when the update message is available.
    pub const MSG__UPDATE__SINCE: u32 = 1;

    /// multi-finger swipe motion
    ///
    /// This event is sent when a multi-finger swipe gesture changes the
    /// position of the logical center.
    ///
    /// The dx and dy coordinates are relative coordinates of the logical
    /// center of the gesture compared to the previous event.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `dx`: delta x coordinate in surface coordinate space
    /// - `dy`: delta y coordinate in surface coordinate space
    #[inline]
    pub fn try_send_update(
        &self,
        time: u32,
        dx: Fixed,
        dy: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            time,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: Fixed, arg2: Fixed) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_pointer_gesture_swipe_v1#{}.update(time: {}, dx: {}, dy: {})\n", client_id, id, arg0, arg1, arg2);
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
            arg1.to_wire() as u32,
            arg2.to_wire() as u32,
        ]);
        Ok(())
    }

    /// multi-finger swipe motion
    ///
    /// This event is sent when a multi-finger swipe gesture changes the
    /// position of the logical center.
    ///
    /// The dx and dy coordinates are relative coordinates of the logical
    /// center of the gesture compared to the previous event.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `dx`: delta x coordinate in surface coordinate space
    /// - `dy`: delta y coordinate in surface coordinate space
    #[inline]
    pub fn send_update(
        &self,
        time: u32,
        dx: Fixed,
        dy: Fixed,
    ) {
        let res = self.try_send_update(
            time,
            dx,
            dy,
        );
        if let Err(e) = res {
            log_send("zwp_pointer_gesture_swipe_v1.update", &e);
        }
    }

    /// Since when the end message is available.
    pub const MSG__END__SINCE: u32 = 1;

    /// multi-finger swipe end
    ///
    /// This event is sent when a multi-finger swipe gesture ceases to
    /// be valid. This may happen when one or more fingers are lifted or
    /// the gesture is cancelled.
    ///
    /// When a gesture is cancelled, the client should undo state changes
    /// caused by this gesture. What causes a gesture to be cancelled is
    /// implementation-dependent.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    /// - `time`: timestamp with millisecond granularity
    /// - `cancelled`: 1 if the gesture was cancelled, 0 otherwise
    #[inline]
    pub fn try_send_end(
        &self,
        serial: u32,
        time: u32,
        cancelled: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            serial,
            time,
            cancelled,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_pointer_gesture_swipe_v1#{}.end(serial: {}, time: {}, cancelled: {})\n", client_id, id, arg0, arg1, arg2);
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
            2,
            arg0,
            arg1,
            arg2 as u32,
        ]);
        Ok(())
    }

    /// multi-finger swipe end
    ///
    /// This event is sent when a multi-finger swipe gesture ceases to
    /// be valid. This may happen when one or more fingers are lifted or
    /// the gesture is cancelled.
    ///
    /// When a gesture is cancelled, the client should undo state changes
    /// caused by this gesture. What causes a gesture to be cancelled is
    /// implementation-dependent.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    /// - `time`: timestamp with millisecond granularity
    /// - `cancelled`: 1 if the gesture was cancelled, 0 otherwise
    #[inline]
    pub fn send_end(
        &self,
        serial: u32,
        time: u32,
        cancelled: i32,
    ) {
        let res = self.try_send_end(
            serial,
            time,
            cancelled,
        );
        if let Err(e) = res {
            log_send("zwp_pointer_gesture_swipe_v1.end", &e);
        }
    }
}

/// A message handler for [`ZwpPointerGestureSwipeV1`] proxies.
pub trait ZwpPointerGestureSwipeV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpPointerGestureSwipeV1>) {
        slf.core.delete_id();
    }

    /// destroy the pointer swipe gesture object
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwpPointerGestureSwipeV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwp_pointer_gesture_swipe_v1.destroy", &e);
        }
    }

    /// multi-finger swipe begin
    ///
    /// This event is sent when a multi-finger swipe gesture is detected
    /// on the device.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    /// - `time`: timestamp with millisecond granularity
    /// - `surface`:
    /// - `fingers`: number of fingers
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_begin(
        &mut self,
        slf: &Rc<ZwpPointerGestureSwipeV1>,
        serial: u32,
        time: u32,
        surface: &Rc<WlSurface>,
        fingers: u32,
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
        let res = slf.try_send_begin(
            serial,
            time,
            surface,
            fingers,
        );
        if let Err(e) = res {
            log_forward("zwp_pointer_gesture_swipe_v1.begin", &e);
        }
    }

    /// multi-finger swipe motion
    ///
    /// This event is sent when a multi-finger swipe gesture changes the
    /// position of the logical center.
    ///
    /// The dx and dy coordinates are relative coordinates of the logical
    /// center of the gesture compared to the previous event.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `dx`: delta x coordinate in surface coordinate space
    /// - `dy`: delta y coordinate in surface coordinate space
    #[inline]
    fn handle_update(
        &mut self,
        slf: &Rc<ZwpPointerGestureSwipeV1>,
        time: u32,
        dx: Fixed,
        dy: Fixed,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_update(
            time,
            dx,
            dy,
        );
        if let Err(e) = res {
            log_forward("zwp_pointer_gesture_swipe_v1.update", &e);
        }
    }

    /// multi-finger swipe end
    ///
    /// This event is sent when a multi-finger swipe gesture ceases to
    /// be valid. This may happen when one or more fingers are lifted or
    /// the gesture is cancelled.
    ///
    /// When a gesture is cancelled, the client should undo state changes
    /// caused by this gesture. What causes a gesture to be cancelled is
    /// implementation-dependent.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    /// - `time`: timestamp with millisecond granularity
    /// - `cancelled`: 1 if the gesture was cancelled, 0 otherwise
    #[inline]
    fn handle_end(
        &mut self,
        slf: &Rc<ZwpPointerGestureSwipeV1>,
        serial: u32,
        time: u32,
        cancelled: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_end(
            serial,
            time,
            cancelled,
        );
        if let Err(e) = res {
            log_forward("zwp_pointer_gesture_swipe_v1.end", &e);
        }
    }
}

impl ObjectPrivate for ZwpPointerGestureSwipeV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpPointerGestureSwipeV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_pointer_gesture_swipe_v1#{}.destroy()\n", client_id, id);
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
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 24)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_pointer_gesture_swipe_v1#{}.begin(serial: {}, time: {}, surface: wl_surface#{}, fingers: {})\n", id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2, arg3);
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
                    (**handler).handle_begin(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_begin(&self, arg0, arg1, arg2, arg3);
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
                let arg1 = Fixed::from_wire(arg1 as i32);
                let arg2 = Fixed::from_wire(arg2 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: Fixed, arg2: Fixed) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_pointer_gesture_swipe_v1#{}.update(time: {}, dx: {}, dy: {})\n", id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_update(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_update(&self, arg0, arg1, arg2);
                }
            }
            2 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_pointer_gesture_swipe_v1#{}.end(serial: {}, time: {}, cancelled: {})\n", id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_end(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_end(&self, arg0, arg1, arg2);
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
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "begin",
            1 => "update",
            2 => "end",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwpPointerGestureSwipeV1 {
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

