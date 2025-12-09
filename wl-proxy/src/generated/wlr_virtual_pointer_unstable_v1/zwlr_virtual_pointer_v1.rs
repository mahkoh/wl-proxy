//! virtual pointer
//!
//! This protocol allows clients to emulate a physical pointer device. The
//! requests are mostly mirror opposites of those specified in wl_pointer.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwlr_virtual_pointer_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwlrVirtualPointerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwlrVirtualPointerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwlrVirtualPointerV1MessageHandler for DefaultMessageHandler { }

impl MetaZwlrVirtualPointerV1 {
    pub const XML_VERSION: u32 = 2;
}

impl MetaZwlrVirtualPointerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwlrVirtualPointerV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaZwlrVirtualPointerV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaZwlrVirtualPointerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwlrVirtualPointerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwlrVirtualPointerV1 {
    /// Since when the motion message is available.
    #[allow(dead_code)]
    pub const MSG__MOTION__SINCE: u32 = 1;

    /// pointer relative motion event
    ///
    /// The pointer has moved by a relative amount to the previous request.
    ///
    /// Values are in the global compositor space.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `dx`: displacement on the x-axis
    /// - `dy`: displacement on the y-axis
    #[inline]
    pub fn send_motion(
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
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
            arg0,
            arg1.to_wire() as u32,
            arg2.to_wire() as u32,
        ]);
        Ok(())
    }

    /// Since when the motion_absolute message is available.
    #[allow(dead_code)]
    pub const MSG__MOTION_ABSOLUTE__SINCE: u32 = 1;

    /// pointer absolute motion event
    ///
    /// The pointer has moved in an absolute coordinate frame.
    ///
    /// Value of x can range from 0 to x_extent, value of y can range from 0
    /// to y_extent.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `x`: position on the x-axis
    /// - `y`: position on the y-axis
    /// - `x_extent`: extent of the x-axis
    /// - `y_extent`: extent of the y-axis
    #[inline]
    pub fn send_motion_absolute(
        &self,
        time: u32,
        x: u32,
        y: u32,
        x_extent: u32,
        y_extent: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ) = (
            time,
            x,
            y,
            x_extent,
            y_extent,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
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
            1,
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ]);
        Ok(())
    }

    /// Since when the button message is available.
    #[allow(dead_code)]
    pub const MSG__BUTTON__SINCE: u32 = 1;

    /// button event
    ///
    /// A button was pressed or released.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `button`: button that produced the event
    /// - `state`: physical state of the button
    #[inline]
    pub fn send_button(
        &self,
        time: u32,
        button: u32,
        state: MetaWlPointerButtonState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            time,
            button,
            state,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
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
            arg0,
            arg1,
            arg2.0,
        ]);
        Ok(())
    }

    /// Since when the axis message is available.
    #[allow(dead_code)]
    pub const MSG__AXIS__SINCE: u32 = 1;

    /// axis event
    ///
    /// Scroll and other axis requests.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `axis`: axis type
    /// - `value`: length of vector in touchpad coordinates
    #[inline]
    pub fn send_axis(
        &self,
        time: u32,
        axis: MetaWlPointerAxis,
        value: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            time,
            axis,
            value,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
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
            3,
            arg0,
            arg1.0,
            arg2.to_wire() as u32,
        ]);
        Ok(())
    }

    /// Since when the frame message is available.
    #[allow(dead_code)]
    pub const MSG__FRAME__SINCE: u32 = 1;

    /// end of a pointer event sequence
    ///
    /// Indicates the set of events that logically belong together.
    #[inline]
    pub fn send_frame(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
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
            4,
        ]);
        Ok(())
    }

    /// Since when the axis_source message is available.
    #[allow(dead_code)]
    pub const MSG__AXIS_SOURCE__SINCE: u32 = 1;

    /// axis source event
    ///
    /// Source information for scroll and other axis.
    ///
    /// # Arguments
    ///
    /// - `axis_source`: source of the axis event
    #[inline]
    pub fn send_axis_source(
        &self,
        axis_source: MetaWlPointerAxisSource,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            axis_source,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
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
            5,
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the axis_stop message is available.
    #[allow(dead_code)]
    pub const MSG__AXIS_STOP__SINCE: u32 = 1;

    /// axis stop event
    ///
    /// Stop notification for scroll and other axes.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `axis`: the axis stopped with this event
    #[inline]
    pub fn send_axis_stop(
        &self,
        time: u32,
        axis: MetaWlPointerAxis,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            time,
            axis,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
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
            6,
            arg0,
            arg1.0,
        ]);
        Ok(())
    }

    /// Since when the axis_discrete message is available.
    #[allow(dead_code)]
    pub const MSG__AXIS_DISCRETE__SINCE: u32 = 1;

    /// axis click event
    ///
    /// Discrete step information for scroll and other axes.
    ///
    /// This event allows the client to extend data normally sent using the axis
    /// event with discrete value.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `axis`: axis type
    /// - `value`: length of vector in touchpad coordinates
    /// - `discrete`: number of steps
    #[inline]
    pub fn send_axis_discrete(
        &self,
        time: u32,
        axis: MetaWlPointerAxis,
        value: Fixed,
        discrete: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            time,
            axis,
            value,
            discrete,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
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
            7,
            arg0,
            arg1.0,
            arg2.to_wire() as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the virtual pointer object
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
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
            8,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [ZwlrVirtualPointerV1] proxies.
#[allow(dead_code)]
pub trait MetaZwlrVirtualPointerV1MessageHandler {
    /// pointer relative motion event
    ///
    /// The pointer has moved by a relative amount to the previous request.
    ///
    /// Values are in the global compositor space.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `dx`: displacement on the x-axis
    /// - `dy`: displacement on the y-axis
    #[inline]
    fn motion(
        &mut self,
        _slf: &Rc<MetaZwlrVirtualPointerV1>,
        time: u32,
        dx: Fixed,
        dy: Fixed,
    ) {
        let res = _slf.send_motion(
            time,
            dx,
            dy,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_virtual_pointer_v1.motion message: {}", Report::new(e));
        }
    }

    /// pointer absolute motion event
    ///
    /// The pointer has moved in an absolute coordinate frame.
    ///
    /// Value of x can range from 0 to x_extent, value of y can range from 0
    /// to y_extent.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `x`: position on the x-axis
    /// - `y`: position on the y-axis
    /// - `x_extent`: extent of the x-axis
    /// - `y_extent`: extent of the y-axis
    #[inline]
    fn motion_absolute(
        &mut self,
        _slf: &Rc<MetaZwlrVirtualPointerV1>,
        time: u32,
        x: u32,
        y: u32,
        x_extent: u32,
        y_extent: u32,
    ) {
        let res = _slf.send_motion_absolute(
            time,
            x,
            y,
            x_extent,
            y_extent,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_virtual_pointer_v1.motion_absolute message: {}", Report::new(e));
        }
    }

    /// button event
    ///
    /// A button was pressed or released.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `button`: button that produced the event
    /// - `state`: physical state of the button
    #[inline]
    fn button(
        &mut self,
        _slf: &Rc<MetaZwlrVirtualPointerV1>,
        time: u32,
        button: u32,
        state: MetaWlPointerButtonState,
    ) {
        let res = _slf.send_button(
            time,
            button,
            state,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_virtual_pointer_v1.button message: {}", Report::new(e));
        }
    }

    /// axis event
    ///
    /// Scroll and other axis requests.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `axis`: axis type
    /// - `value`: length of vector in touchpad coordinates
    #[inline]
    fn axis(
        &mut self,
        _slf: &Rc<MetaZwlrVirtualPointerV1>,
        time: u32,
        axis: MetaWlPointerAxis,
        value: Fixed,
    ) {
        let res = _slf.send_axis(
            time,
            axis,
            value,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_virtual_pointer_v1.axis message: {}", Report::new(e));
        }
    }

    /// end of a pointer event sequence
    ///
    /// Indicates the set of events that logically belong together.
    #[inline]
    fn frame(
        &mut self,
        _slf: &Rc<MetaZwlrVirtualPointerV1>,
    ) {
        let res = _slf.send_frame(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_virtual_pointer_v1.frame message: {}", Report::new(e));
        }
    }

    /// axis source event
    ///
    /// Source information for scroll and other axis.
    ///
    /// # Arguments
    ///
    /// - `axis_source`: source of the axis event
    #[inline]
    fn axis_source(
        &mut self,
        _slf: &Rc<MetaZwlrVirtualPointerV1>,
        axis_source: MetaWlPointerAxisSource,
    ) {
        let res = _slf.send_axis_source(
            axis_source,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_virtual_pointer_v1.axis_source message: {}", Report::new(e));
        }
    }

    /// axis stop event
    ///
    /// Stop notification for scroll and other axes.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `axis`: the axis stopped with this event
    #[inline]
    fn axis_stop(
        &mut self,
        _slf: &Rc<MetaZwlrVirtualPointerV1>,
        time: u32,
        axis: MetaWlPointerAxis,
    ) {
        let res = _slf.send_axis_stop(
            time,
            axis,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_virtual_pointer_v1.axis_stop message: {}", Report::new(e));
        }
    }

    /// axis click event
    ///
    /// Discrete step information for scroll and other axes.
    ///
    /// This event allows the client to extend data normally sent using the axis
    /// event with discrete value.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `axis`: axis type
    /// - `value`: length of vector in touchpad coordinates
    /// - `discrete`: number of steps
    #[inline]
    fn axis_discrete(
        &mut self,
        _slf: &Rc<MetaZwlrVirtualPointerV1>,
        time: u32,
        axis: MetaWlPointerAxis,
        value: Fixed,
        discrete: i32,
    ) {
        let res = _slf.send_axis_discrete(
            time,
            axis,
            value,
            discrete,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_virtual_pointer_v1.axis_discrete message: {}", Report::new(e));
        }
    }

    /// destroy the virtual pointer object
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwlrVirtualPointerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_virtual_pointer_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwlrVirtualPointerV1 {
    fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Self::new(state, version)
    }

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
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                let arg1 = Fixed::from_wire(arg1 as i32);
                let arg2 = Fixed::from_wire(arg2 as i32);
                if let Some(handler) = handler {
                    (**handler).motion(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.motion(&self, arg0, arg1, arg2);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 28));
                };
                if let Some(handler) = handler {
                    (**handler).motion_absolute(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultMessageHandler.motion_absolute(&self, arg0, arg1, arg2, arg3, arg4);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                let arg2 = MetaWlPointerButtonState(arg2);
                if let Some(handler) = handler {
                    (**handler).button(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.button(&self, arg0, arg1, arg2);
                }
            }
            3 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                let arg1 = MetaWlPointerAxis(arg1);
                let arg2 = Fixed::from_wire(arg2 as i32);
                if let Some(handler) = handler {
                    (**handler).axis(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.axis(&self, arg0, arg1, arg2);
                }
            }
            4 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).frame(&self);
                } else {
                    DefaultMessageHandler.frame(&self);
                }
            }
            5 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = MetaWlPointerAxisSource(arg0);
                if let Some(handler) = handler {
                    (**handler).axis_source(&self, arg0);
                } else {
                    DefaultMessageHandler.axis_source(&self, arg0);
                }
            }
            6 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg1 = MetaWlPointerAxis(arg1);
                if let Some(handler) = handler {
                    (**handler).axis_stop(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.axis_stop(&self, arg0, arg1);
                }
            }
            7 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 24));
                };
                let arg1 = MetaWlPointerAxis(arg1);
                let arg2 = Fixed::from_wire(arg2 as i32);
                let arg3 = arg3 as i32;
                if let Some(handler) = handler {
                    (**handler).axis_discrete(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultMessageHandler.axis_discrete(&self, arg0, arg1, arg2, arg3);
                }
            }
            8 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
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
        let handler = &mut *self.handler.borrow();
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
            0 => "motion",
            1 => "motion_absolute",
            2 => "button",
            3 => "axis",
            4 => "frame",
            5 => "axis_source",
            6 => "axis_stop",
            7 => "axis_discrete",
            8 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl MetaZwlrVirtualPointerV1 {
    /// Since when the error.invalid_axis enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_AXIS__SINCE: u32 = 1;
    /// Since when the error.invalid_axis_source enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_AXIS_SOURCE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaZwlrVirtualPointerV1Error(pub u32);

impl MetaZwlrVirtualPointerV1Error {
    /// client sent invalid axis enumeration value
    #[allow(dead_code)]
    pub const INVALID_AXIS: Self = Self(0);

    /// client sent invalid axis source enumeration value
    #[allow(dead_code)]
    pub const INVALID_AXIS_SOURCE: Self = Self(1);
}

impl Debug for MetaZwlrVirtualPointerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_AXIS => "INVALID_AXIS",
            Self::INVALID_AXIS_SOURCE => "INVALID_AXIS_SOURCE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
