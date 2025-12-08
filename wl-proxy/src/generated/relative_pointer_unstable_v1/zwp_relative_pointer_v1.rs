//! relative pointer object
//!
//! A wp_relative_pointer object is an extension to the wl_pointer interface
//! used for emitting relative pointer events. It shares the same focus as
//! wl_pointer objects of the same seat and will only emit events when it has
//! focus.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_relative_pointer_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpRelativePointerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpRelativePointerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpRelativePointerV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpRelativePointerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZwpRelativePointerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpRelativePointerV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwpRelativePointerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpRelativePointerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpRelativePointerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// release the relative pointer object
    #[inline]
    pub fn send_destroy(
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
            0,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the relative_motion message is available.
    #[allow(dead_code)]
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
    pub fn send_relative_motion(
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
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
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
}

/// A message handler for [ZwpRelativePointerV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpRelativePointerV1MessageHandler {
    /// release the relative pointer object
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpRelativePointerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_relative_pointer_v1.destroy message: {}", Report::new(e));
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
    fn relative_motion(
        &mut self,
        _slf: &Rc<MetaZwpRelativePointerV1>,
        utime_hi: u32,
        utime_lo: u32,
        dx: Fixed,
        dy: Fixed,
        dx_unaccel: Fixed,
        dy_unaccel: Fixed,
    ) {
        let res = _slf.send_relative_motion(
            utime_hi,
            utime_lo,
            dx,
            dy,
            dx_unaccel,
            dy_unaccel,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_relative_pointer_v1.relative_motion message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpRelativePointerV1 {
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
                self.core.handle_client_destroy();
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
                    arg4,
                    arg5,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg2 = Fixed::from_wire(arg2 as i32);
                let arg3 = Fixed::from_wire(arg3 as i32);
                let arg4 = Fixed::from_wire(arg4 as i32);
                let arg5 = Fixed::from_wire(arg5 as i32);
                if let Some(handler) = handler {
                    (**handler).relative_motion(&self, arg0, arg1, arg2, arg3, arg4, arg5);
                } else {
                    DefaultMessageHandler.relative_motion(&self, arg0, arg1, arg2, arg3, arg4, arg5);
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

