//! touchpad gestures
//!
//! A global interface to provide semantic touchpad gestures for a given
//! pointer.
//!
//! Three gestures are currently supported: swipe, pinch, and hold.
//! Pinch and swipe gestures follow a three-stage cycle: begin, update,
//! end. Hold gestures follow a two-stage cycle: begin and end. All
//! gestures are identified by a unique id.
//!
//! Warning! The protocol described in this file is experimental and
//! backward incompatible changes may be made. Backward compatible changes
//! may be added together with the corresponding interface version bump.
//! Backward incompatible changes are done by bumping the version number in
//! the protocol and interface names and resetting the interface version.
//! Once the protocol is to be declared stable, the 'z' prefix and the
//! version number in the protocol and interface names are removed and the
//! interface version number is reset.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_pointer_gestures_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpPointerGesturesV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpPointerGesturesV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpPointerGesturesV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpPointerGesturesV1 {
    pub const XML_VERSION: u32 = 3;
}

impl MetaZwpPointerGesturesV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpPointerGesturesV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwpPointerGesturesV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpPointerGesturesV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpPointerGesturesV1 {
    /// Since when the get_swipe_gesture message is available.
    #[allow(dead_code)]
    pub const MSG__GET_SWIPE_GESTURE__SINCE: u32 = 1;

    /// get swipe gesture
    ///
    /// Create a swipe gesture object. See the
    /// wl_pointer_gesture_swipe interface for details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `pointer`:
    #[inline]
    pub fn send_get_swipe_gesture(
        &self,
        id: &Rc<MetaZwpPointerGestureSwipeV1>,
        pointer: &Rc<MetaWlPointer>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            pointer,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg1 = match arg1.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())?;
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
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
        ]);
        Ok(())
    }

    /// Since when the get_pinch_gesture message is available.
    #[allow(dead_code)]
    pub const MSG__GET_PINCH_GESTURE__SINCE: u32 = 1;

    /// get pinch gesture
    ///
    /// Create a pinch gesture object. See the
    /// wl_pointer_gesture_pinch interface for details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `pointer`:
    #[inline]
    pub fn send_get_pinch_gesture(
        &self,
        id: &Rc<MetaZwpPointerGesturePinchV1>,
        pointer: &Rc<MetaWlPointer>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            pointer,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg1 = match arg1.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())?;
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
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
        ]);
        Ok(())
    }

    /// Since when the release message is available.
    #[allow(dead_code)]
    pub const MSG__RELEASE__SINCE: u32 = 2;

    /// destroy the pointer gesture object
    ///
    /// Destroy the pointer gesture object. Swipe, pinch and hold objects
    /// created via this gesture object remain valid.
    #[inline]
    pub fn send_release(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
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

    /// Since when the get_hold_gesture message is available.
    #[allow(dead_code)]
    pub const MSG__GET_HOLD_GESTURE__SINCE: u32 = 3;

    /// get hold gesture
    ///
    /// Create a hold gesture object. See the
    /// wl_pointer_gesture_hold interface for details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `pointer`:
    #[inline]
    pub fn send_get_hold_gesture(
        &self,
        id: &Rc<MetaZwpPointerGestureHoldV1>,
        pointer: &Rc<MetaWlPointer>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            pointer,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg1 = match arg1.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            3,
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpPointerGesturesV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpPointerGesturesV1MessageHandler {
    /// get swipe gesture
    ///
    /// Create a swipe gesture object. See the
    /// wl_pointer_gesture_swipe interface for details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `pointer`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_swipe_gesture(
        &mut self,
        _slf: &Rc<MetaZwpPointerGesturesV1>,
        id: &Rc<MetaZwpPointerGestureSwipeV1>,
        pointer: &Rc<MetaWlPointer>,
    ) {
        let res = _slf.send_get_swipe_gesture(
            id,
            pointer,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_pointer_gestures_v1.get_swipe_gesture message: {}", Report::new(e));
        }
    }

    /// get pinch gesture
    ///
    /// Create a pinch gesture object. See the
    /// wl_pointer_gesture_pinch interface for details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `pointer`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_pinch_gesture(
        &mut self,
        _slf: &Rc<MetaZwpPointerGesturesV1>,
        id: &Rc<MetaZwpPointerGesturePinchV1>,
        pointer: &Rc<MetaWlPointer>,
    ) {
        let res = _slf.send_get_pinch_gesture(
            id,
            pointer,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_pointer_gestures_v1.get_pinch_gesture message: {}", Report::new(e));
        }
    }

    /// destroy the pointer gesture object
    ///
    /// Destroy the pointer gesture object. Swipe, pinch and hold objects
    /// created via this gesture object remain valid.
    #[inline]
    fn release(
        &mut self,
        _slf: &Rc<MetaZwpPointerGesturesV1>,
    ) {
        let res = _slf.send_release(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_pointer_gestures_v1.release message: {}", Report::new(e));
        }
    }

    /// get hold gesture
    ///
    /// Create a hold gesture object. See the
    /// wl_pointer_gesture_hold interface for details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `pointer`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_hold_gesture(
        &mut self,
        _slf: &Rc<MetaZwpPointerGesturesV1>,
        id: &Rc<MetaZwpPointerGestureHoldV1>,
        pointer: &Rc<MetaWlPointer>,
    ) {
        let res = _slf.send_get_hold_gesture(
            id,
            pointer,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_pointer_gestures_v1.get_hold_gesture message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpPointerGesturesV1 {
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
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaZwpPointerGestureSwipeV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.endpoint.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlPointer>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_swipe_gesture(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_swipe_gesture(&self, arg0, arg1);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaZwpPointerGesturePinchV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.endpoint.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlPointer>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_pinch_gesture(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_pinch_gesture(&self, arg0, arg1);
                }
            }
            2 => {
                if let Some(handler) = handler {
                    (**handler).release(&self);
                } else {
                    DefaultMessageHandler.release(&self);
                }
                self.core.handle_client_destroy();
            }
            3 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaZwpPointerGestureHoldV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.endpoint.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlPointer>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_hold_gesture(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_hold_gesture(&self, arg0, arg1);
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
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
    }
}

