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
pub struct ZwpPointerGesturesV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn ZwpPointerGesturesV1Handler>,
}

struct DefaultHandler;

impl ZwpPointerGesturesV1Handler for DefaultHandler { }

impl ZwpPointerGesturesV1 {
    pub const XML_VERSION: u32 = 3;
    pub const INTERFACE: &str = "zwp_pointer_gestures_v1";
}

impl ZwpPointerGesturesV1 {
    pub fn set_handler(&self, handler: impl ZwpPointerGesturesV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpPointerGesturesV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpPointerGesturesV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpPointerGesturesV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpPointerGesturesV1 {
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
        id: &Rc<ZwpPointerGestureSwipeV1>,
        pointer: &Rc<WlPointer>,
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("pointer")),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_pointer_gestures_v1#{}.get_swipe_gesture(id: zwp_pointer_gesture_swipe_v1#{}, pointer: wl_pointer#{})\n", id, arg0_id, arg1_id);
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0_id,
            arg1_id,
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
        id: &Rc<ZwpPointerGesturePinchV1>,
        pointer: &Rc<WlPointer>,
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("pointer")),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_pointer_gestures_v1#{}.get_pinch_gesture(id: zwp_pointer_gesture_pinch_v1#{}, pointer: wl_pointer#{})\n", id, arg0_id, arg1_id);
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
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
            arg1_id,
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_pointer_gestures_v1#{}.release()\n", id);
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
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
        id: &Rc<ZwpPointerGestureHoldV1>,
        pointer: &Rc<WlPointer>,
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("pointer")),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_pointer_gestures_v1#{}.get_hold_gesture(id: zwp_pointer_gesture_hold_v1#{}, pointer: wl_pointer#{})\n", id, arg0_id, arg1_id);
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            3,
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpPointerGesturesV1] proxies.
#[allow(dead_code)]
pub trait ZwpPointerGesturesV1Handler: Any {
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
        _slf: &Rc<ZwpPointerGesturesV1>,
        id: &Rc<ZwpPointerGestureSwipeV1>,
        pointer: &Rc<WlPointer>,
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
        _slf: &Rc<ZwpPointerGesturesV1>,
        id: &Rc<ZwpPointerGesturePinchV1>,
        pointer: &Rc<WlPointer>,
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
        _slf: &Rc<ZwpPointerGesturesV1>,
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
        _slf: &Rc<ZwpPointerGesturesV1>,
        id: &Rc<ZwpPointerGestureHoldV1>,
        pointer: &Rc<WlPointer>,
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

impl ProxyPrivate for ZwpPointerGesturesV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::ZwpPointerGesturesV1, version),
            handler: Default::default(),
        })
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_pointer_gestures_v1#{}.get_swipe_gesture(id: zwp_pointer_gesture_swipe_v1#{}, pointer: wl_pointer#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ZwpPointerGestureSwipeV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlPointer>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("pointer", o.core().interface, ProxyInterface::WlPointer));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_swipe_gesture(&self, arg0, arg1);
                } else {
                    DefaultHandler.get_swipe_gesture(&self, arg0, arg1);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_pointer_gestures_v1#{}.get_pinch_gesture(id: zwp_pointer_gesture_pinch_v1#{}, pointer: wl_pointer#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ZwpPointerGesturePinchV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlPointer>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("pointer", o.core().interface, ProxyInterface::WlPointer));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_pinch_gesture(&self, arg0, arg1);
                } else {
                    DefaultHandler.get_pinch_gesture(&self, arg0, arg1);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_pointer_gestures_v1#{}.release()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).release(&self);
                } else {
                    DefaultHandler.release(&self);
                }
                self.core.handle_client_destroy();
            }
            3 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_pointer_gestures_v1#{}.get_hold_gesture(id: zwp_pointer_gesture_hold_v1#{}, pointer: wl_pointer#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ZwpPointerGestureHoldV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlPointer>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("pointer", o.core().interface, ProxyInterface::WlPointer));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_hold_gesture(&self, arg0, arg1);
                } else {
                    DefaultHandler.get_hold_gesture(&self, arg0, arg1);
                }
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
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "get_swipe_gesture",
            1 => "get_pinch_gesture",
            2 => "release",
            3 => "get_hold_gesture",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Proxy for ZwpPointerGesturesV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn unset_handler(&self) {
        self.handler.set(None);
    }

    fn get_handler_any_ref(&self) -> Result<Ref<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(Ref::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<RefMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow_mut().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(RefMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

