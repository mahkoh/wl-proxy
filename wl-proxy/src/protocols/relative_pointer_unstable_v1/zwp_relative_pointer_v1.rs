//! relative pointer object
//!
//! A wp_relative_pointer object is an extension to the wl_pointer interface
//! used for emitting relative pointer events. It shares the same focus as
//! wl_pointer objects of the same seat and will only emit events when it has
//! focus.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_relative_pointer_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpRelativePointerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpRelativePointerV1Handler>,
}

struct DefaultHandler;

impl ZwpRelativePointerV1Handler for DefaultHandler { }

impl ConcreteObject for ZwpRelativePointerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpRelativePointerV1;
    const INTERFACE_NAME: &str = "zwp_relative_pointer_v1";
}

impl ZwpRelativePointerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpRelativePointerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpRelativePointerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpRelativePointerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpRelativePointerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpRelativePointerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// release the relative pointer object
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_relative_pointer_v1#{}.destroy()\n", id);
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

    /// release the relative pointer object
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwp_relative_pointer_v1.destroy", &e);
        }
    }

    /// Since when the relative_motion message is available.
    pub const MSG__RELATIVE_MOTION__SINCE: u32 = 1;

    /// relative pointer motion
    ///
    /// Relative x/y pointer motion from the pointer of the seat associated with
    /// this object.
    ///
    /// A relative motion is in the same dimension as regular wl_pointer motion
    /// events, except they do not represent an absolute position. For example,
    /// moving a pointer from (x, y) to (x', y') would have the equivalent
    /// relative motion (x' - x, y' - y). If a pointer motion caused the
    /// absolute pointer position to be clipped by for example the edge of the
    /// monitor, the relative motion is unaffected by the clipping and will
    /// represent the unclipped motion.
    ///
    /// This event also contains non-accelerated motion deltas. The
    /// non-accelerated delta is, when applicable, the regular pointer motion
    /// delta as it was before having applied motion acceleration and other
    /// transformations such as normalization.
    ///
    /// Note that the non-accelerated delta does not represent 'raw' events as
    /// they were read from some device. Pointer motion acceleration is device-
    /// and configuration-specific and non-accelerated deltas and accelerated
    /// deltas may have the same value on some devices.
    ///
    /// Relative motions are not coupled to wl_pointer.motion events, and can be
    /// sent in combination with such events, but also independently. There may
    /// also be scenarios where wl_pointer.motion is sent, but there is no
    /// relative motion. The order of an absolute and relative motion event
    /// originating from the same physical motion is not guaranteed.
    ///
    /// If the client needs button events or focus state, it can receive them
    /// from a wl_pointer object of the same seat that the wp_relative_pointer
    /// object is associated with.
    ///
    /// # Arguments
    ///
    /// - `utime_hi`: high 32 bits of a 64 bit timestamp with microsecond granularity
    /// - `utime_lo`: low 32 bits of a 64 bit timestamp with microsecond granularity
    /// - `dx`: the x component of the motion vector
    /// - `dy`: the y component of the motion vector
    /// - `dx_unaccel`: the x component of the unaccelerated motion vector
    /// - `dy_unaccel`: the y component of the unaccelerated motion vector
    #[inline]
    pub fn try_send_relative_motion(
        &self,
        utime_hi: u32,
        utime_lo: u32,
        dx: Fixed,
        dy: Fixed,
        dx_unaccel: Fixed,
        dy_unaccel: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
        ) = (
            utime_hi,
            utime_lo,
            dx,
            dy,
            dx_unaccel,
            dy_unaccel,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: Fixed, arg3: Fixed, arg4: Fixed, arg5: Fixed) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_relative_pointer_v1#{}.relative_motion(utime_hi: {}, utime_lo: {}, dx: {}, dy: {}, dx_unaccel: {}, dy_unaccel: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4, arg5);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1, arg2, arg3, arg4, arg5);
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
            arg2.to_wire() as u32,
            arg3.to_wire() as u32,
            arg4.to_wire() as u32,
            arg5.to_wire() as u32,
        ]);
        Ok(())
    }

    /// relative pointer motion
    ///
    /// Relative x/y pointer motion from the pointer of the seat associated with
    /// this object.
    ///
    /// A relative motion is in the same dimension as regular wl_pointer motion
    /// events, except they do not represent an absolute position. For example,
    /// moving a pointer from (x, y) to (x', y') would have the equivalent
    /// relative motion (x' - x, y' - y). If a pointer motion caused the
    /// absolute pointer position to be clipped by for example the edge of the
    /// monitor, the relative motion is unaffected by the clipping and will
    /// represent the unclipped motion.
    ///
    /// This event also contains non-accelerated motion deltas. The
    /// non-accelerated delta is, when applicable, the regular pointer motion
    /// delta as it was before having applied motion acceleration and other
    /// transformations such as normalization.
    ///
    /// Note that the non-accelerated delta does not represent 'raw' events as
    /// they were read from some device. Pointer motion acceleration is device-
    /// and configuration-specific and non-accelerated deltas and accelerated
    /// deltas may have the same value on some devices.
    ///
    /// Relative motions are not coupled to wl_pointer.motion events, and can be
    /// sent in combination with such events, but also independently. There may
    /// also be scenarios where wl_pointer.motion is sent, but there is no
    /// relative motion. The order of an absolute and relative motion event
    /// originating from the same physical motion is not guaranteed.
    ///
    /// If the client needs button events or focus state, it can receive them
    /// from a wl_pointer object of the same seat that the wp_relative_pointer
    /// object is associated with.
    ///
    /// # Arguments
    ///
    /// - `utime_hi`: high 32 bits of a 64 bit timestamp with microsecond granularity
    /// - `utime_lo`: low 32 bits of a 64 bit timestamp with microsecond granularity
    /// - `dx`: the x component of the motion vector
    /// - `dy`: the y component of the motion vector
    /// - `dx_unaccel`: the x component of the unaccelerated motion vector
    /// - `dy_unaccel`: the y component of the unaccelerated motion vector
    #[inline]
    pub fn send_relative_motion(
        &self,
        utime_hi: u32,
        utime_lo: u32,
        dx: Fixed,
        dy: Fixed,
        dx_unaccel: Fixed,
        dy_unaccel: Fixed,
    ) {
        let res = self.try_send_relative_motion(
            utime_hi,
            utime_lo,
            dx,
            dy,
            dx_unaccel,
            dy_unaccel,
        );
        if let Err(e) = res {
            log_send("zwp_relative_pointer_v1.relative_motion", &e);
        }
    }
}

/// A message handler for [`ZwpRelativePointerV1`] proxies.
pub trait ZwpRelativePointerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpRelativePointerV1>) {
        slf.core.delete_id();
    }

    /// release the relative pointer object
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwpRelativePointerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwp_relative_pointer_v1.destroy", &e);
        }
    }

    /// relative pointer motion
    ///
    /// Relative x/y pointer motion from the pointer of the seat associated with
    /// this object.
    ///
    /// A relative motion is in the same dimension as regular wl_pointer motion
    /// events, except they do not represent an absolute position. For example,
    /// moving a pointer from (x, y) to (x', y') would have the equivalent
    /// relative motion (x' - x, y' - y). If a pointer motion caused the
    /// absolute pointer position to be clipped by for example the edge of the
    /// monitor, the relative motion is unaffected by the clipping and will
    /// represent the unclipped motion.
    ///
    /// This event also contains non-accelerated motion deltas. The
    /// non-accelerated delta is, when applicable, the regular pointer motion
    /// delta as it was before having applied motion acceleration and other
    /// transformations such as normalization.
    ///
    /// Note that the non-accelerated delta does not represent 'raw' events as
    /// they were read from some device. Pointer motion acceleration is device-
    /// and configuration-specific and non-accelerated deltas and accelerated
    /// deltas may have the same value on some devices.
    ///
    /// Relative motions are not coupled to wl_pointer.motion events, and can be
    /// sent in combination with such events, but also independently. There may
    /// also be scenarios where wl_pointer.motion is sent, but there is no
    /// relative motion. The order of an absolute and relative motion event
    /// originating from the same physical motion is not guaranteed.
    ///
    /// If the client needs button events or focus state, it can receive them
    /// from a wl_pointer object of the same seat that the wp_relative_pointer
    /// object is associated with.
    ///
    /// # Arguments
    ///
    /// - `utime_hi`: high 32 bits of a 64 bit timestamp with microsecond granularity
    /// - `utime_lo`: low 32 bits of a 64 bit timestamp with microsecond granularity
    /// - `dx`: the x component of the motion vector
    /// - `dy`: the y component of the motion vector
    /// - `dx_unaccel`: the x component of the unaccelerated motion vector
    /// - `dy_unaccel`: the y component of the unaccelerated motion vector
    #[inline]
    fn handle_relative_motion(
        &mut self,
        slf: &Rc<ZwpRelativePointerV1>,
        utime_hi: u32,
        utime_lo: u32,
        dx: Fixed,
        dy: Fixed,
        dx_unaccel: Fixed,
        dy_unaccel: Fixed,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_relative_motion(
            utime_hi,
            utime_lo,
            dx,
            dy,
            dx_unaccel,
            dy_unaccel,
        );
        if let Err(e) = res {
            log_forward("zwp_relative_pointer_v1.relative_motion", &e);
        }
    }
}

impl ObjectPrivate for ZwpRelativePointerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpRelativePointerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_relative_pointer_v1#{}.destroy()\n", client_id, id);
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
                    arg4,
                    arg5,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 32)));
                };
                let arg2 = Fixed::from_wire(arg2 as i32);
                let arg3 = Fixed::from_wire(arg3 as i32);
                let arg4 = Fixed::from_wire(arg4 as i32);
                let arg5 = Fixed::from_wire(arg5 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: Fixed, arg3: Fixed, arg4: Fixed, arg5: Fixed) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_relative_pointer_v1#{}.relative_motion(utime_hi: {}, utime_lo: {}, dx: {}, dy: {}, dx_unaccel: {}, dy_unaccel: {})\n", id, arg0, arg1, arg2, arg3, arg4, arg5);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2, arg3, arg4, arg5);
                }
                if let Some(handler) = handler {
                    (**handler).handle_relative_motion(&self, arg0, arg1, arg2, arg3, arg4, arg5);
                } else {
                    DefaultHandler.handle_relative_motion(&self, arg0, arg1, arg2, arg3, arg4, arg5);
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
            0 => "relative_motion",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwpRelativePointerV1 {
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

