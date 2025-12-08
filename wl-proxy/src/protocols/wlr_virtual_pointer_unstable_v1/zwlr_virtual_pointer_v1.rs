//! virtual pointer
//!
//! This protocol allows clients to emulate a physical pointer device. The
//! requests are mostly mirror opposites of those specified in wl_pointer.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwlr_virtual_pointer_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwlrVirtualPointerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwlrVirtualPointerV1Handler>,
}

struct DefaultHandler;

impl ZwlrVirtualPointerV1Handler for DefaultHandler { }

impl ConcreteObject for ZwlrVirtualPointerV1 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwlrVirtualPointerV1;
    const INTERFACE_NAME: &str = "zwlr_virtual_pointer_v1";
}

impl ZwlrVirtualPointerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwlrVirtualPointerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwlrVirtualPointerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwlrVirtualPointerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwlrVirtualPointerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwlrVirtualPointerV1 {
    /// Since when the motion message is available.
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
    pub fn try_send_motion(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: Fixed, arg2: Fixed) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_virtual_pointer_v1#{}.motion(time: {}, dx: {}, dy: {})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2);
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
            arg0,
            arg1.to_wire() as u32,
            arg2.to_wire() as u32,
        ]);
        Ok(())
    }

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
    ) {
        let res = self.try_send_motion(
            time,
            dx,
            dy,
        );
        if let Err(e) = res {
            log_send("zwlr_virtual_pointer_v1.motion", &e);
        }
    }

    /// Since when the motion_absolute message is available.
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
    pub fn try_send_motion_absolute(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_virtual_pointer_v1#{}.motion_absolute(time: {}, x: {}, y: {}, x_extent: {}, y_extent: {})\n", id, arg0, arg1, arg2, arg3, arg4);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2, arg3, arg4);
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
            arg1,
            arg2,
            arg3,
            arg4,
        ]);
        Ok(())
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
    pub fn send_motion_absolute(
        &self,
        time: u32,
        x: u32,
        y: u32,
        x_extent: u32,
        y_extent: u32,
    ) {
        let res = self.try_send_motion_absolute(
            time,
            x,
            y,
            x_extent,
            y_extent,
        );
        if let Err(e) = res {
            log_send("zwlr_virtual_pointer_v1.motion_absolute", &e);
        }
    }

    /// Since when the button message is available.
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
    pub fn try_send_button(
        &self,
        time: u32,
        button: u32,
        state: WlPointerButtonState,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: WlPointerButtonState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_virtual_pointer_v1#{}.button(time: {}, button: {}, state: {:?})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2);
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
            arg0,
            arg1,
            arg2.0,
        ]);
        Ok(())
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
    pub fn send_button(
        &self,
        time: u32,
        button: u32,
        state: WlPointerButtonState,
    ) {
        let res = self.try_send_button(
            time,
            button,
            state,
        );
        if let Err(e) = res {
            log_send("zwlr_virtual_pointer_v1.button", &e);
        }
    }

    /// Since when the axis message is available.
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
    pub fn try_send_axis(
        &self,
        time: u32,
        axis: WlPointerAxis,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: WlPointerAxis, arg2: Fixed) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_virtual_pointer_v1#{}.axis(time: {}, axis: {:?}, value: {})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2);
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
            arg0,
            arg1.0,
            arg2.to_wire() as u32,
        ]);
        Ok(())
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
    pub fn send_axis(
        &self,
        time: u32,
        axis: WlPointerAxis,
        value: Fixed,
    ) {
        let res = self.try_send_axis(
            time,
            axis,
            value,
        );
        if let Err(e) = res {
            log_send("zwlr_virtual_pointer_v1.axis", &e);
        }
    }

    /// Since when the frame message is available.
    pub const MSG__FRAME__SINCE: u32 = 1;

    /// end of a pointer event sequence
    ///
    /// Indicates the set of events that logically belong together.
    #[inline]
    pub fn try_send_frame(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_virtual_pointer_v1#{}.frame()\n", id);
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
            4,
        ]);
        Ok(())
    }

    /// end of a pointer event sequence
    ///
    /// Indicates the set of events that logically belong together.
    #[inline]
    pub fn send_frame(
        &self,
    ) {
        let res = self.try_send_frame(
        );
        if let Err(e) = res {
            log_send("zwlr_virtual_pointer_v1.frame", &e);
        }
    }

    /// Since when the axis_source message is available.
    pub const MSG__AXIS_SOURCE__SINCE: u32 = 1;

    /// axis source event
    ///
    /// Source information for scroll and other axis.
    ///
    /// # Arguments
    ///
    /// - `axis_source`: source of the axis event
    #[inline]
    pub fn try_send_axis_source(
        &self,
        axis_source: WlPointerAxisSource,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            axis_source,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: WlPointerAxisSource) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_virtual_pointer_v1#{}.axis_source(axis_source: {:?})\n", id, arg0);
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
            5,
            arg0.0,
        ]);
        Ok(())
    }

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
        axis_source: WlPointerAxisSource,
    ) {
        let res = self.try_send_axis_source(
            axis_source,
        );
        if let Err(e) = res {
            log_send("zwlr_virtual_pointer_v1.axis_source", &e);
        }
    }

    /// Since when the axis_stop message is available.
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
    pub fn try_send_axis_stop(
        &self,
        time: u32,
        axis: WlPointerAxis,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: WlPointerAxis) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_virtual_pointer_v1#{}.axis_stop(time: {}, axis: {:?})\n", id, arg0, arg1);
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
            arg0,
            arg1.0,
        ]);
        Ok(())
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
    pub fn send_axis_stop(
        &self,
        time: u32,
        axis: WlPointerAxis,
    ) {
        let res = self.try_send_axis_stop(
            time,
            axis,
        );
        if let Err(e) = res {
            log_send("zwlr_virtual_pointer_v1.axis_stop", &e);
        }
    }

    /// Since when the axis_discrete message is available.
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
    pub fn try_send_axis_discrete(
        &self,
        time: u32,
        axis: WlPointerAxis,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: WlPointerAxis, arg2: Fixed, arg3: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_virtual_pointer_v1#{}.axis_discrete(time: {}, axis: {:?}, value: {}, discrete: {})\n", id, arg0, arg1, arg2, arg3);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2, arg3);
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
            7,
            arg0,
            arg1.0,
            arg2.to_wire() as u32,
            arg3 as u32,
        ]);
        Ok(())
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
    pub fn send_axis_discrete(
        &self,
        time: u32,
        axis: WlPointerAxis,
        value: Fixed,
        discrete: i32,
    ) {
        let res = self.try_send_axis_discrete(
            time,
            axis,
            value,
            discrete,
        );
        if let Err(e) = res {
            log_send("zwlr_virtual_pointer_v1.axis_discrete", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the virtual pointer object
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_virtual_pointer_v1#{}.destroy()\n", id);
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
            8,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the virtual pointer object
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwlr_virtual_pointer_v1.destroy", &e);
        }
    }
}

/// A message handler for [`ZwlrVirtualPointerV1`] proxies.
pub trait ZwlrVirtualPointerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwlrVirtualPointerV1>) {
        slf.core.delete_id();
    }

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
    fn handle_motion(
        &mut self,
        slf: &Rc<ZwlrVirtualPointerV1>,
        time: u32,
        dx: Fixed,
        dy: Fixed,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_motion(
            time,
            dx,
            dy,
        );
        if let Err(e) = res {
            log_forward("zwlr_virtual_pointer_v1.motion", &e);
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
    fn handle_motion_absolute(
        &mut self,
        slf: &Rc<ZwlrVirtualPointerV1>,
        time: u32,
        x: u32,
        y: u32,
        x_extent: u32,
        y_extent: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_motion_absolute(
            time,
            x,
            y,
            x_extent,
            y_extent,
        );
        if let Err(e) = res {
            log_forward("zwlr_virtual_pointer_v1.motion_absolute", &e);
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
    fn handle_button(
        &mut self,
        slf: &Rc<ZwlrVirtualPointerV1>,
        time: u32,
        button: u32,
        state: WlPointerButtonState,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_button(
            time,
            button,
            state,
        );
        if let Err(e) = res {
            log_forward("zwlr_virtual_pointer_v1.button", &e);
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
    fn handle_axis(
        &mut self,
        slf: &Rc<ZwlrVirtualPointerV1>,
        time: u32,
        axis: WlPointerAxis,
        value: Fixed,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_axis(
            time,
            axis,
            value,
        );
        if let Err(e) = res {
            log_forward("zwlr_virtual_pointer_v1.axis", &e);
        }
    }

    /// end of a pointer event sequence
    ///
    /// Indicates the set of events that logically belong together.
    #[inline]
    fn handle_frame(
        &mut self,
        slf: &Rc<ZwlrVirtualPointerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_frame(
        );
        if let Err(e) = res {
            log_forward("zwlr_virtual_pointer_v1.frame", &e);
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
    fn handle_axis_source(
        &mut self,
        slf: &Rc<ZwlrVirtualPointerV1>,
        axis_source: WlPointerAxisSource,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_axis_source(
            axis_source,
        );
        if let Err(e) = res {
            log_forward("zwlr_virtual_pointer_v1.axis_source", &e);
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
    fn handle_axis_stop(
        &mut self,
        slf: &Rc<ZwlrVirtualPointerV1>,
        time: u32,
        axis: WlPointerAxis,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_axis_stop(
            time,
            axis,
        );
        if let Err(e) = res {
            log_forward("zwlr_virtual_pointer_v1.axis_stop", &e);
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
    fn handle_axis_discrete(
        &mut self,
        slf: &Rc<ZwlrVirtualPointerV1>,
        time: u32,
        axis: WlPointerAxis,
        value: Fixed,
        discrete: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_axis_discrete(
            time,
            axis,
            value,
            discrete,
        );
        if let Err(e) = res {
            log_forward("zwlr_virtual_pointer_v1.axis_discrete", &e);
        }
    }

    /// destroy the virtual pointer object
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwlrVirtualPointerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwlr_virtual_pointer_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for ZwlrVirtualPointerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwlrVirtualPointerV1, version),
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
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: Fixed, arg2: Fixed) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_virtual_pointer_v1#{}.motion(time: {}, dx: {}, dy: {})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_motion(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_motion(&self, arg0, arg1, arg2);
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
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 28)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_virtual_pointer_v1#{}.motion_absolute(time: {}, x: {}, y: {}, x_extent: {}, y_extent: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4);
                }
                if let Some(handler) = handler {
                    (**handler).handle_motion_absolute(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultHandler.handle_motion_absolute(&self, arg0, arg1, arg2, arg3, arg4);
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
                let arg2 = WlPointerButtonState(arg2);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: WlPointerButtonState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_virtual_pointer_v1#{}.button(time: {}, button: {}, state: {:?})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_button(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_button(&self, arg0, arg1, arg2);
                }
            }
            3 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                let arg1 = WlPointerAxis(arg1);
                let arg2 = Fixed::from_wire(arg2 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: WlPointerAxis, arg2: Fixed) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_virtual_pointer_v1#{}.axis(time: {}, axis: {:?}, value: {})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_axis(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_axis(&self, arg0, arg1, arg2);
                }
            }
            4 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_virtual_pointer_v1#{}.frame()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_frame(&self);
                } else {
                    DefaultHandler.handle_frame(&self);
                }
            }
            5 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = WlPointerAxisSource(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: WlPointerAxisSource) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_virtual_pointer_v1#{}.axis_source(axis_source: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_axis_source(&self, arg0);
                } else {
                    DefaultHandler.handle_axis_source(&self, arg0);
                }
            }
            6 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = WlPointerAxis(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: WlPointerAxis) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_virtual_pointer_v1#{}.axis_stop(time: {}, axis: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_axis_stop(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_axis_stop(&self, arg0, arg1);
                }
            }
            7 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 24)));
                };
                let arg1 = WlPointerAxis(arg1);
                let arg2 = Fixed::from_wire(arg2 as i32);
                let arg3 = arg3 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: WlPointerAxis, arg2: Fixed, arg3: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_virtual_pointer_v1#{}.axis_discrete(time: {}, axis: {:?}, value: {}, discrete: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_axis_discrete(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_axis_discrete(&self, arg0, arg1, arg2, arg3);
                }
            }
            8 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_virtual_pointer_v1#{}.destroy()\n", client_id, id);
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

impl Object for ZwlrVirtualPointerV1 {
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

impl ZwlrVirtualPointerV1 {
    /// Since when the error.invalid_axis enum variant is available.
    pub const ENM__ERROR_INVALID_AXIS__SINCE: u32 = 1;
    /// Since when the error.invalid_axis_source enum variant is available.
    pub const ENM__ERROR_INVALID_AXIS_SOURCE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwlrVirtualPointerV1Error(pub u32);

impl ZwlrVirtualPointerV1Error {
    /// client sent invalid axis enumeration value
    pub const INVALID_AXIS: Self = Self(0);

    /// client sent invalid axis source enumeration value
    pub const INVALID_AXIS_SOURCE: Self = Self(1);
}

impl Debug for ZwlrVirtualPointerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_AXIS => "INVALID_AXIS",
            Self::INVALID_AXIS_SOURCE => "INVALID_AXIS_SOURCE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
