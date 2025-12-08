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

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_pointer_gesture_swipe_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpPointerGestureSwipeV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpPointerGestureSwipeV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpPointerGestureSwipeV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpPointerGestureSwipeV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwpPointerGestureSwipeV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpPointerGestureSwipeV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpPointerGestureSwipeV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the pointer swipe gesture object
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
        ]);
        Ok(())
    }

    /// Since when the begin message is available.
    #[allow(dead_code)]
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
    pub fn send_begin(
        &self,
        serial: u32,
        time: u32,
        surface: &Rc<MetaWlSurface>,
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
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        if arg2.client_id.get() != Some(client.id) {
            return Err(ObjectError);
        }
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
            arg0,
            arg1,
            arg2.client_obj_id.get().unwrap_or(0),
            arg3,
        ]);
        Ok(())
    }

    /// Since when the update message is available.
    #[allow(dead_code)]
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
    pub fn send_update(
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
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            1,
            arg0,
            arg1.to_wire() as u32,
            arg2.to_wire() as u32,
        ]);
        Ok(())
    }

    /// Since when the end message is available.
    #[allow(dead_code)]
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
    pub fn send_end(
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
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            2,
            arg0,
            arg1,
            arg2 as u32,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpPointerGestureSwipeV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpPointerGestureSwipeV1MessageHandler {
    /// destroy the pointer swipe gesture object
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpPointerGestureSwipeV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_pointer_gesture_swipe_v1.destroy message: {}", Report::new(e));
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
    fn begin(
        &mut self,
        _slf: &Rc<MetaZwpPointerGestureSwipeV1>,
        serial: u32,
        time: u32,
        surface: &Rc<MetaWlSurface>,
        fingers: u32,
    ) {
        if let Some(client_id) = _slf.core.client_id.get() {
            if let Some(client_id_2) = surface.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = _slf.send_begin(
            serial,
            time,
            surface,
            fingers,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_pointer_gesture_swipe_v1.begin message: {}", Report::new(e));
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
    fn update(
        &mut self,
        _slf: &Rc<MetaZwpPointerGestureSwipeV1>,
        time: u32,
        dx: Fixed,
        dy: Fixed,
    ) {
        let res = _slf.send_update(
            time,
            dx,
            dy,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_pointer_gesture_swipe_v1.update message: {}", Report::new(e));
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
    fn end(
        &mut self,
        _slf: &Rc<MetaZwpPointerGestureSwipeV1>,
        serial: u32,
        time: u32,
        cancelled: i32,
    ) {
        let res = _slf.send_end(
            serial,
            time,
            cancelled,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_pointer_gesture_swipe_v1.end message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpPointerGestureSwipeV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
            }
            _ => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
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
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let Some(arg2) = self.core.state.lookup(arg2) else {
                    return Err(ObjectError);
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    return Err(ObjectError);
                };
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).begin(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultMessageHandler.begin(&self, arg0, arg1, arg2, arg3);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg1 = Fixed::from_wire(arg1 as i32);
                let arg2 = Fixed::from_wire(arg2 as i32);
                if let Some(handler) = handler {
                    (**handler).update(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.update(&self, arg0, arg1, arg2);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg2 = arg2 as i32;
                if let Some(handler) = handler {
                    (**handler).end(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.end(&self, arg0, arg1, arg2);
                }
            }
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }
}

