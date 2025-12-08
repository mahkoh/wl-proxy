//! pointer input device
//!
//! The wl_pointer interface represents one or more input devices,
//! such as mice, which control the pointer location and pointer_focus
//! of a seat.
//!
//! The wl_pointer interface generates motion, enter and leave
//! events for the surfaces that the pointer is located over,
//! and button and axis events for button presses, button releases
//! and scrolling.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wl_pointer proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWlPointer {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWlPointerMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWlPointerMessageHandler for DefaultMessageHandler { }

impl MetaWlPointer {
    pub const XML_VERSION: u32 = 10;
}

impl MetaWlPointer {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WlPointer, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWlPointer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWlPointer")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWlPointer {
    /// Since when the set_cursor message is available.
    #[allow(dead_code)]
    pub const MSG__SET_CURSOR__SINCE: u32 = 1;

    /// set the pointer surface
    ///
    /// Set the pointer surface, i.e., the surface that contains the
    /// pointer image (cursor). This request gives the surface the role
    /// of a cursor. If the surface already has another role, it raises
    /// a protocol error.
    ///
    /// The cursor actually changes only if the pointer
    /// focus for this device is one of the requesting client's surfaces
    /// or the surface parameter is the current pointer surface. If
    /// there was a previous surface set with this request it is
    /// replaced. If surface is NULL, the pointer image is hidden.
    ///
    /// The parameters hotspot_x and hotspot_y define the position of
    /// the pointer surface relative to the pointer location. Its
    /// top-left corner is always at (x, y) - (hotspot_x, hotspot_y),
    /// where (x, y) are the coordinates of the pointer location, in
    /// surface-local coordinates.
    ///
    /// On wl_surface.offset requests to the pointer surface, hotspot_x
    /// and hotspot_y are decremented by the x and y parameters
    /// passed to the request. The offset must be applied by
    /// wl_surface.commit as usual.
    ///
    /// The hotspot can also be updated by passing the currently set
    /// pointer surface to this request with new values for hotspot_x
    /// and hotspot_y.
    ///
    /// The input region is ignored for wl_surfaces with the role of
    /// a cursor. When the use as a cursor ends, the wl_surface is
    /// unmapped.
    ///
    /// The serial parameter must match the latest wl_pointer.enter
    /// serial number sent to the client. Otherwise the request will be
    /// ignored.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the enter event
    /// - `surface`: pointer surface
    /// - `hotspot_x`: surface-local x coordinate
    /// - `hotspot_y`: surface-local y coordinate
    #[inline]
    pub fn send_set_cursor(
        &self,
        serial: u32,
        surface: Option<&Rc<MetaWlSurface>>,
        hotspot_x: i32,
        hotspot_y: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            serial,
            surface,
            hotspot_x,
            hotspot_y,
        );
        let arg1 = arg1.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg1 = match arg1 {
            None => 0,
            Some(arg1) => match arg1.server_obj_id.get() {
                None => return Err(ObjectError),
                Some(id) => id,
            },
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
            arg1,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// Since when the enter message is available.
    #[allow(dead_code)]
    pub const MSG__ENTER__SINCE: u32 = 1;

    /// enter event
    ///
    /// Notification that this seat's pointer is focused on a certain
    /// surface.
    ///
    /// When a seat's focus enters a surface, the pointer image
    /// is undefined and a client should respond to this event by setting
    /// an appropriate pointer image with the set_cursor request.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the enter event
    /// - `surface`: surface entered by the pointer
    /// - `surface_x`: surface-local x coordinate
    /// - `surface_y`: surface-local y coordinate
    #[inline]
    pub fn send_enter(
        &self,
        serial: u32,
        surface: &Rc<MetaWlSurface>,
        surface_x: Fixed,
        surface_y: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            serial,
            surface,
            surface_x,
            surface_y,
        );
        let arg1 = arg1.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError);
        };
        if arg1.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError);
        }
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
            arg1.client_obj_id.get().unwrap_or(0),
            arg2.to_wire() as u32,
            arg3.to_wire() as u32,
        ]);
        Ok(())
    }

    /// Since when the leave message is available.
    #[allow(dead_code)]
    pub const MSG__LEAVE__SINCE: u32 = 1;

    /// leave event
    ///
    /// Notification that this seat's pointer is no longer focused on
    /// a certain surface.
    ///
    /// The leave notification is sent before the enter notification
    /// for the new focus.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the leave event
    /// - `surface`: surface left by the pointer
    #[inline]
    pub fn send_leave(
        &self,
        serial: u32,
        surface: &Rc<MetaWlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            serial,
            surface,
        );
        let arg1 = arg1.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError);
        };
        if arg1.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError);
        }
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            1,
            arg0,
            arg1.client_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }

    /// Since when the motion message is available.
    #[allow(dead_code)]
    pub const MSG__MOTION__SINCE: u32 = 1;

    /// pointer motion event
    ///
    /// Notification of pointer location change. The arguments
    /// surface_x and surface_y are the location relative to the
    /// focused surface.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `surface_x`: surface-local x coordinate
    /// - `surface_y`: surface-local y coordinate
    #[inline]
    pub fn send_motion(
        &self,
        time: u32,
        surface_x: Fixed,
        surface_y: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            time,
            surface_x,
            surface_y,
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
            2,
            arg0,
            arg1.to_wire() as u32,
            arg2.to_wire() as u32,
        ]);
        Ok(())
    }

    /// Since when the button message is available.
    #[allow(dead_code)]
    pub const MSG__BUTTON__SINCE: u32 = 1;

    /// pointer button event
    ///
    /// Mouse button click and release notifications.
    ///
    /// The location of the click is given by the last motion or
    /// enter event.
    /// The time argument is a timestamp with millisecond
    /// granularity, with an undefined base.
    ///
    /// The button is a button code as defined in the Linux kernel's
    /// linux/input-event-codes.h header file, e.g. BTN_LEFT.
    ///
    /// Any 16-bit button code value is reserved for future additions to the
    /// kernel's event code list. All other button codes above 0xFFFF are
    /// currently undefined but may be used in future versions of this
    /// protocol.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the button event
    /// - `time`: timestamp with millisecond granularity
    /// - `button`: button that produced the event
    /// - `state`: physical state of the button
    #[inline]
    pub fn send_button(
        &self,
        serial: u32,
        time: u32,
        button: u32,
        state: MetaWlPointerButtonState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            serial,
            time,
            button,
            state,
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
            3,
            arg0,
            arg1,
            arg2,
            arg3.0,
        ]);
        Ok(())
    }

    /// Since when the axis message is available.
    #[allow(dead_code)]
    pub const MSG__AXIS__SINCE: u32 = 1;

    /// axis event
    ///
    /// Scroll and other axis notifications.
    ///
    /// For scroll events (vertical and horizontal scroll axes), the
    /// value parameter is the length of a vector along the specified
    /// axis in a coordinate space identical to those of motion events,
    /// representing a relative movement along the specified axis.
    ///
    /// For devices that support movements non-parallel to axes multiple
    /// axis events will be emitted.
    ///
    /// When applicable, for example for touch pads, the server can
    /// choose to emit scroll events where the motion vector is
    /// equivalent to a motion event vector.
    ///
    /// When applicable, a client can transform its content relative to the
    /// scroll distance.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `axis`: axis type
    /// - `value`: length of vector in surface-local coordinate space
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
            4,
            arg0,
            arg1.0,
            arg2.to_wire() as u32,
        ]);
        Ok(())
    }

    /// Since when the release message is available.
    #[allow(dead_code)]
    pub const MSG__RELEASE__SINCE: u32 = 3;

    /// release the pointer object
    ///
    /// Using this request a client can tell the server that it is not going to
    /// use the pointer object anymore.
    ///
    /// This request destroys the pointer proxy object, so clients must not call
    /// wl_pointer_destroy() after using this request.
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
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the frame message is available.
    #[allow(dead_code)]
    pub const MSG__FRAME__SINCE: u32 = 5;

    /// end of a pointer event sequence
    ///
    /// Indicates the end of a set of events that logically belong together.
    /// A client is expected to accumulate the data in all events within the
    /// frame before proceeding.
    ///
    /// All wl_pointer events before a wl_pointer.frame event belong
    /// logically together. For example, in a diagonal scroll motion the
    /// compositor will send an optional wl_pointer.axis_source event, two
    /// wl_pointer.axis events (horizontal and vertical) and finally a
    /// wl_pointer.frame event. The client may use this information to
    /// calculate a diagonal vector for scrolling.
    ///
    /// When multiple wl_pointer.axis events occur within the same frame,
    /// the motion vector is the combined motion of all events.
    /// When a wl_pointer.axis and a wl_pointer.axis_stop event occur within
    /// the same frame, this indicates that axis movement in one axis has
    /// stopped but continues in the other axis.
    /// When multiple wl_pointer.axis_stop events occur within the same
    /// frame, this indicates that these axes stopped in the same instance.
    ///
    /// A wl_pointer.frame event is sent for every logical event group,
    /// even if the group only contains a single wl_pointer event.
    /// Specifically, a client may get a sequence: motion, frame, button,
    /// frame, axis, frame, axis_stop, frame.
    ///
    /// The wl_pointer.enter and wl_pointer.leave events are logical events
    /// generated by the compositor and not the hardware. These events are
    /// also grouped by a wl_pointer.frame. When a pointer moves from one
    /// surface to another, a compositor should group the
    /// wl_pointer.leave event within the same wl_pointer.frame.
    /// However, a client must not rely on wl_pointer.leave and
    /// wl_pointer.enter being in the same wl_pointer.frame.
    /// Compositor-specific policies may require the wl_pointer.leave and
    /// wl_pointer.enter event being split across multiple wl_pointer.frame
    /// groups.
    #[inline]
    pub fn send_frame(
        &self,
    ) -> Result<(), ObjectError> {
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
            5,
        ]);
        Ok(())
    }

    /// Since when the axis_source message is available.
    #[allow(dead_code)]
    pub const MSG__AXIS_SOURCE__SINCE: u32 = 5;

    /// axis source event
    ///
    /// Source information for scroll and other axes.
    ///
    /// This event does not occur on its own. It is sent before a
    /// wl_pointer.frame event and carries the source information for
    /// all events within that frame.
    ///
    /// The source specifies how this event was generated. If the source is
    /// wl_pointer.axis_source.finger, a wl_pointer.axis_stop event will be
    /// sent when the user lifts the finger off the device.
    ///
    /// If the source is wl_pointer.axis_source.wheel,
    /// wl_pointer.axis_source.wheel_tilt or
    /// wl_pointer.axis_source.continuous, a wl_pointer.axis_stop event may
    /// or may not be sent. Whether a compositor sends an axis_stop event
    /// for these sources is hardware-specific and implementation-dependent;
    /// clients must not rely on receiving an axis_stop event for these
    /// scroll sources and should treat scroll sequences from these scroll
    /// sources as unterminated by default.
    ///
    /// This event is optional. If the source is unknown for a particular
    /// axis event sequence, no event is sent.
    /// Only one wl_pointer.axis_source event is permitted per frame.
    ///
    /// The order of wl_pointer.axis_discrete and wl_pointer.axis_source is
    /// not guaranteed.
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
            6,
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the axis_stop message is available.
    #[allow(dead_code)]
    pub const MSG__AXIS_STOP__SINCE: u32 = 5;

    /// axis stop event
    ///
    /// Stop notification for scroll and other axes.
    ///
    /// For some wl_pointer.axis_source types, a wl_pointer.axis_stop event
    /// is sent to notify a client that the axis sequence has terminated.
    /// This enables the client to implement kinetic scrolling.
    /// See the wl_pointer.axis_source documentation for information on when
    /// this event may be generated.
    ///
    /// Any wl_pointer.axis events with the same axis_source after this
    /// event should be considered as the start of a new axis motion.
    ///
    /// The timestamp is to be interpreted identical to the timestamp in the
    /// wl_pointer.axis event. The timestamp value may be the same as a
    /// preceding wl_pointer.axis event.
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
            7,
            arg0,
            arg1.0,
        ]);
        Ok(())
    }

    /// Since when the axis_discrete message is available.
    #[allow(dead_code)]
    pub const MSG__AXIS_DISCRETE__SINCE: u32 = 5;

    /// Since when the axis_discrete message is deprecated.
    #[allow(dead_code)]
    pub const MSG__AXIS_DISCRETE__DEPRECATED_SINCE: u32 = 8;

    /// axis click event
    ///
    /// Discrete step information for scroll and other axes.
    ///
    /// This event carries the axis value of the wl_pointer.axis event in
    /// discrete steps (e.g. mouse wheel clicks).
    ///
    /// This event is deprecated with wl_pointer version 8 - this event is not
    /// sent to clients supporting version 8 or later.
    ///
    /// This event does not occur on its own, it is coupled with a
    /// wl_pointer.axis event that represents this axis value on a
    /// continuous scale. The protocol guarantees that each axis_discrete
    /// event is always followed by exactly one axis event with the same
    /// axis number within the same wl_pointer.frame. Note that the protocol
    /// allows for other events to occur between the axis_discrete and
    /// its coupled axis event, including other axis_discrete or axis
    /// events. A wl_pointer.frame must not contain more than one axis_discrete
    /// event per axis type.
    ///
    /// This event is optional; continuous scrolling devices
    /// like two-finger scrolling on touchpads do not have discrete
    /// steps and do not generate this event.
    ///
    /// The discrete value carries the directional information. e.g. a value
    /// of -2 is two steps towards the negative direction of this axis.
    ///
    /// The axis number is identical to the axis number in the associated
    /// axis event.
    ///
    /// The order of wl_pointer.axis_discrete and wl_pointer.axis_source is
    /// not guaranteed.
    ///
    /// # Arguments
    ///
    /// - `axis`: axis type
    /// - `discrete`: number of steps
    #[inline]
    pub fn send_axis_discrete(
        &self,
        axis: MetaWlPointerAxis,
        discrete: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            axis,
            discrete,
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
            8,
            arg0.0,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// Since when the axis_value120 message is available.
    #[allow(dead_code)]
    pub const MSG__AXIS_VALUE120__SINCE: u32 = 8;

    /// axis high-resolution scroll event
    ///
    /// Discrete high-resolution scroll information.
    ///
    /// This event carries high-resolution wheel scroll information,
    /// with each multiple of 120 representing one logical scroll step
    /// (a wheel detent). For example, an axis_value120 of 30 is one quarter of
    /// a logical scroll step in the positive direction, a value120 of
    /// -240 are two logical scroll steps in the negative direction within the
    /// same hardware event.
    /// Clients that rely on discrete scrolling should accumulate the
    /// value120 to multiples of 120 before processing the event.
    ///
    /// The value120 must not be zero.
    ///
    /// This event replaces the wl_pointer.axis_discrete event in clients
    /// supporting wl_pointer version 8 or later.
    ///
    /// Where a wl_pointer.axis_source event occurs in the same
    /// wl_pointer.frame, the axis source applies to this event.
    ///
    /// The order of wl_pointer.axis_value120 and wl_pointer.axis_source is
    /// not guaranteed.
    ///
    /// # Arguments
    ///
    /// - `axis`: axis type
    /// - `value120`: scroll distance as fraction of 120
    #[inline]
    pub fn send_axis_value120(
        &self,
        axis: MetaWlPointerAxis,
        value120: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            axis,
            value120,
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
            9,
            arg0.0,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// Since when the axis_relative_direction message is available.
    #[allow(dead_code)]
    pub const MSG__AXIS_RELATIVE_DIRECTION__SINCE: u32 = 9;

    /// axis relative physical direction event
    ///
    /// Relative directional information of the entity causing the axis
    /// motion.
    ///
    /// For a wl_pointer.axis event, the wl_pointer.axis_relative_direction
    /// event specifies the movement direction of the entity causing the
    /// wl_pointer.axis event. For example:
    /// - if a user's fingers on a touchpad move down and this
    ///   causes a wl_pointer.axis vertical_scroll down event, the physical
    ///   direction is 'identical'
    /// - if a user's fingers on a touchpad move down and this causes a
    ///   wl_pointer.axis vertical_scroll up scroll up event ('natural
    ///   scrolling'), the physical direction is 'inverted'.
    ///
    /// A client may use this information to adjust scroll motion of
    /// components. Specifically, enabling natural scrolling causes the
    /// content to change direction compared to traditional scrolling.
    /// Some widgets like volume control sliders should usually match the
    /// physical direction regardless of whether natural scrolling is
    /// active. This event enables clients to match the scroll direction of
    /// a widget to the physical direction.
    ///
    /// This event does not occur on its own, it is coupled with a
    /// wl_pointer.axis event that represents this axis value.
    /// The protocol guarantees that each axis_relative_direction event is
    /// always followed by exactly one axis event with the same
    /// axis number within the same wl_pointer.frame. Note that the protocol
    /// allows for other events to occur between the axis_relative_direction
    /// and its coupled axis event.
    ///
    /// The axis number is identical to the axis number in the associated
    /// axis event.
    ///
    /// The order of wl_pointer.axis_relative_direction,
    /// wl_pointer.axis_discrete and wl_pointer.axis_source is not
    /// guaranteed.
    ///
    /// # Arguments
    ///
    /// - `axis`: axis type
    /// - `direction`: physical direction relative to axis motion
    #[inline]
    pub fn send_axis_relative_direction(
        &self,
        axis: MetaWlPointerAxis,
        direction: MetaWlPointerAxisRelativeDirection,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            axis,
            direction,
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
            10,
            arg0.0,
            arg1.0,
        ]);
        Ok(())
    }
}

/// A message handler for [WlPointer] proxies.
#[allow(dead_code)]
pub trait MetaWlPointerMessageHandler {
    /// set the pointer surface
    ///
    /// Set the pointer surface, i.e., the surface that contains the
    /// pointer image (cursor). This request gives the surface the role
    /// of a cursor. If the surface already has another role, it raises
    /// a protocol error.
    ///
    /// The cursor actually changes only if the pointer
    /// focus for this device is one of the requesting client's surfaces
    /// or the surface parameter is the current pointer surface. If
    /// there was a previous surface set with this request it is
    /// replaced. If surface is NULL, the pointer image is hidden.
    ///
    /// The parameters hotspot_x and hotspot_y define the position of
    /// the pointer surface relative to the pointer location. Its
    /// top-left corner is always at (x, y) - (hotspot_x, hotspot_y),
    /// where (x, y) are the coordinates of the pointer location, in
    /// surface-local coordinates.
    ///
    /// On wl_surface.offset requests to the pointer surface, hotspot_x
    /// and hotspot_y are decremented by the x and y parameters
    /// passed to the request. The offset must be applied by
    /// wl_surface.commit as usual.
    ///
    /// The hotspot can also be updated by passing the currently set
    /// pointer surface to this request with new values for hotspot_x
    /// and hotspot_y.
    ///
    /// The input region is ignored for wl_surfaces with the role of
    /// a cursor. When the use as a cursor ends, the wl_surface is
    /// unmapped.
    ///
    /// The serial parameter must match the latest wl_pointer.enter
    /// serial number sent to the client. Otherwise the request will be
    /// ignored.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the enter event
    /// - `surface`: pointer surface
    /// - `hotspot_x`: surface-local x coordinate
    /// - `hotspot_y`: surface-local y coordinate
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn set_cursor(
        &mut self,
        _slf: &Rc<MetaWlPointer>,
        serial: u32,
        surface: Option<&Rc<MetaWlSurface>>,
        hotspot_x: i32,
        hotspot_y: i32,
    ) {
        let res = _slf.send_set_cursor(
            serial,
            surface,
            hotspot_x,
            hotspot_y,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_pointer.set_cursor message: {}", Report::new(e));
        }
    }

    /// enter event
    ///
    /// Notification that this seat's pointer is focused on a certain
    /// surface.
    ///
    /// When a seat's focus enters a surface, the pointer image
    /// is undefined and a client should respond to this event by setting
    /// an appropriate pointer image with the set_cursor request.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the enter event
    /// - `surface`: surface entered by the pointer
    /// - `surface_x`: surface-local x coordinate
    /// - `surface_y`: surface-local y coordinate
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn enter(
        &mut self,
        _slf: &Rc<MetaWlPointer>,
        serial: u32,
        surface: &Rc<MetaWlSurface>,
        surface_x: Fixed,
        surface_y: Fixed,
    ) {
        if let Some(client_id) = _slf.core.client_id.get() {
            if let Some(client_id_2) = surface.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = _slf.send_enter(
            serial,
            surface,
            surface_x,
            surface_y,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_pointer.enter message: {}", Report::new(e));
        }
    }

    /// leave event
    ///
    /// Notification that this seat's pointer is no longer focused on
    /// a certain surface.
    ///
    /// The leave notification is sent before the enter notification
    /// for the new focus.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the leave event
    /// - `surface`: surface left by the pointer
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn leave(
        &mut self,
        _slf: &Rc<MetaWlPointer>,
        serial: u32,
        surface: &Rc<MetaWlSurface>,
    ) {
        if let Some(client_id) = _slf.core.client_id.get() {
            if let Some(client_id_2) = surface.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = _slf.send_leave(
            serial,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_pointer.leave message: {}", Report::new(e));
        }
    }

    /// pointer motion event
    ///
    /// Notification of pointer location change. The arguments
    /// surface_x and surface_y are the location relative to the
    /// focused surface.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `surface_x`: surface-local x coordinate
    /// - `surface_y`: surface-local y coordinate
    #[inline]
    fn motion(
        &mut self,
        _slf: &Rc<MetaWlPointer>,
        time: u32,
        surface_x: Fixed,
        surface_y: Fixed,
    ) {
        let res = _slf.send_motion(
            time,
            surface_x,
            surface_y,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_pointer.motion message: {}", Report::new(e));
        }
    }

    /// pointer button event
    ///
    /// Mouse button click and release notifications.
    ///
    /// The location of the click is given by the last motion or
    /// enter event.
    /// The time argument is a timestamp with millisecond
    /// granularity, with an undefined base.
    ///
    /// The button is a button code as defined in the Linux kernel's
    /// linux/input-event-codes.h header file, e.g. BTN_LEFT.
    ///
    /// Any 16-bit button code value is reserved for future additions to the
    /// kernel's event code list. All other button codes above 0xFFFF are
    /// currently undefined but may be used in future versions of this
    /// protocol.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the button event
    /// - `time`: timestamp with millisecond granularity
    /// - `button`: button that produced the event
    /// - `state`: physical state of the button
    #[inline]
    fn button(
        &mut self,
        _slf: &Rc<MetaWlPointer>,
        serial: u32,
        time: u32,
        button: u32,
        state: MetaWlPointerButtonState,
    ) {
        let res = _slf.send_button(
            serial,
            time,
            button,
            state,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_pointer.button message: {}", Report::new(e));
        }
    }

    /// axis event
    ///
    /// Scroll and other axis notifications.
    ///
    /// For scroll events (vertical and horizontal scroll axes), the
    /// value parameter is the length of a vector along the specified
    /// axis in a coordinate space identical to those of motion events,
    /// representing a relative movement along the specified axis.
    ///
    /// For devices that support movements non-parallel to axes multiple
    /// axis events will be emitted.
    ///
    /// When applicable, for example for touch pads, the server can
    /// choose to emit scroll events where the motion vector is
    /// equivalent to a motion event vector.
    ///
    /// When applicable, a client can transform its content relative to the
    /// scroll distance.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `axis`: axis type
    /// - `value`: length of vector in surface-local coordinate space
    #[inline]
    fn axis(
        &mut self,
        _slf: &Rc<MetaWlPointer>,
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
            log::warn!("Could not forward a wl_pointer.axis message: {}", Report::new(e));
        }
    }

    /// release the pointer object
    ///
    /// Using this request a client can tell the server that it is not going to
    /// use the pointer object anymore.
    ///
    /// This request destroys the pointer proxy object, so clients must not call
    /// wl_pointer_destroy() after using this request.
    #[inline]
    fn release(
        &mut self,
        _slf: &Rc<MetaWlPointer>,
    ) {
        let res = _slf.send_release(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_pointer.release message: {}", Report::new(e));
        }
    }

    /// end of a pointer event sequence
    ///
    /// Indicates the end of a set of events that logically belong together.
    /// A client is expected to accumulate the data in all events within the
    /// frame before proceeding.
    ///
    /// All wl_pointer events before a wl_pointer.frame event belong
    /// logically together. For example, in a diagonal scroll motion the
    /// compositor will send an optional wl_pointer.axis_source event, two
    /// wl_pointer.axis events (horizontal and vertical) and finally a
    /// wl_pointer.frame event. The client may use this information to
    /// calculate a diagonal vector for scrolling.
    ///
    /// When multiple wl_pointer.axis events occur within the same frame,
    /// the motion vector is the combined motion of all events.
    /// When a wl_pointer.axis and a wl_pointer.axis_stop event occur within
    /// the same frame, this indicates that axis movement in one axis has
    /// stopped but continues in the other axis.
    /// When multiple wl_pointer.axis_stop events occur within the same
    /// frame, this indicates that these axes stopped in the same instance.
    ///
    /// A wl_pointer.frame event is sent for every logical event group,
    /// even if the group only contains a single wl_pointer event.
    /// Specifically, a client may get a sequence: motion, frame, button,
    /// frame, axis, frame, axis_stop, frame.
    ///
    /// The wl_pointer.enter and wl_pointer.leave events are logical events
    /// generated by the compositor and not the hardware. These events are
    /// also grouped by a wl_pointer.frame. When a pointer moves from one
    /// surface to another, a compositor should group the
    /// wl_pointer.leave event within the same wl_pointer.frame.
    /// However, a client must not rely on wl_pointer.leave and
    /// wl_pointer.enter being in the same wl_pointer.frame.
    /// Compositor-specific policies may require the wl_pointer.leave and
    /// wl_pointer.enter event being split across multiple wl_pointer.frame
    /// groups.
    #[inline]
    fn frame(
        &mut self,
        _slf: &Rc<MetaWlPointer>,
    ) {
        let res = _slf.send_frame(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_pointer.frame message: {}", Report::new(e));
        }
    }

    /// axis source event
    ///
    /// Source information for scroll and other axes.
    ///
    /// This event does not occur on its own. It is sent before a
    /// wl_pointer.frame event and carries the source information for
    /// all events within that frame.
    ///
    /// The source specifies how this event was generated. If the source is
    /// wl_pointer.axis_source.finger, a wl_pointer.axis_stop event will be
    /// sent when the user lifts the finger off the device.
    ///
    /// If the source is wl_pointer.axis_source.wheel,
    /// wl_pointer.axis_source.wheel_tilt or
    /// wl_pointer.axis_source.continuous, a wl_pointer.axis_stop event may
    /// or may not be sent. Whether a compositor sends an axis_stop event
    /// for these sources is hardware-specific and implementation-dependent;
    /// clients must not rely on receiving an axis_stop event for these
    /// scroll sources and should treat scroll sequences from these scroll
    /// sources as unterminated by default.
    ///
    /// This event is optional. If the source is unknown for a particular
    /// axis event sequence, no event is sent.
    /// Only one wl_pointer.axis_source event is permitted per frame.
    ///
    /// The order of wl_pointer.axis_discrete and wl_pointer.axis_source is
    /// not guaranteed.
    ///
    /// # Arguments
    ///
    /// - `axis_source`: source of the axis event
    #[inline]
    fn axis_source(
        &mut self,
        _slf: &Rc<MetaWlPointer>,
        axis_source: MetaWlPointerAxisSource,
    ) {
        let res = _slf.send_axis_source(
            axis_source,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_pointer.axis_source message: {}", Report::new(e));
        }
    }

    /// axis stop event
    ///
    /// Stop notification for scroll and other axes.
    ///
    /// For some wl_pointer.axis_source types, a wl_pointer.axis_stop event
    /// is sent to notify a client that the axis sequence has terminated.
    /// This enables the client to implement kinetic scrolling.
    /// See the wl_pointer.axis_source documentation for information on when
    /// this event may be generated.
    ///
    /// Any wl_pointer.axis events with the same axis_source after this
    /// event should be considered as the start of a new axis motion.
    ///
    /// The timestamp is to be interpreted identical to the timestamp in the
    /// wl_pointer.axis event. The timestamp value may be the same as a
    /// preceding wl_pointer.axis event.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `axis`: the axis stopped with this event
    #[inline]
    fn axis_stop(
        &mut self,
        _slf: &Rc<MetaWlPointer>,
        time: u32,
        axis: MetaWlPointerAxis,
    ) {
        let res = _slf.send_axis_stop(
            time,
            axis,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_pointer.axis_stop message: {}", Report::new(e));
        }
    }

    /// axis click event
    ///
    /// Discrete step information for scroll and other axes.
    ///
    /// This event carries the axis value of the wl_pointer.axis event in
    /// discrete steps (e.g. mouse wheel clicks).
    ///
    /// This event is deprecated with wl_pointer version 8 - this event is not
    /// sent to clients supporting version 8 or later.
    ///
    /// This event does not occur on its own, it is coupled with a
    /// wl_pointer.axis event that represents this axis value on a
    /// continuous scale. The protocol guarantees that each axis_discrete
    /// event is always followed by exactly one axis event with the same
    /// axis number within the same wl_pointer.frame. Note that the protocol
    /// allows for other events to occur between the axis_discrete and
    /// its coupled axis event, including other axis_discrete or axis
    /// events. A wl_pointer.frame must not contain more than one axis_discrete
    /// event per axis type.
    ///
    /// This event is optional; continuous scrolling devices
    /// like two-finger scrolling on touchpads do not have discrete
    /// steps and do not generate this event.
    ///
    /// The discrete value carries the directional information. e.g. a value
    /// of -2 is two steps towards the negative direction of this axis.
    ///
    /// The axis number is identical to the axis number in the associated
    /// axis event.
    ///
    /// The order of wl_pointer.axis_discrete and wl_pointer.axis_source is
    /// not guaranteed.
    ///
    /// # Arguments
    ///
    /// - `axis`: axis type
    /// - `discrete`: number of steps
    #[inline]
    fn axis_discrete(
        &mut self,
        _slf: &Rc<MetaWlPointer>,
        axis: MetaWlPointerAxis,
        discrete: i32,
    ) {
        let res = _slf.send_axis_discrete(
            axis,
            discrete,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_pointer.axis_discrete message: {}", Report::new(e));
        }
    }

    /// axis high-resolution scroll event
    ///
    /// Discrete high-resolution scroll information.
    ///
    /// This event carries high-resolution wheel scroll information,
    /// with each multiple of 120 representing one logical scroll step
    /// (a wheel detent). For example, an axis_value120 of 30 is one quarter of
    /// a logical scroll step in the positive direction, a value120 of
    /// -240 are two logical scroll steps in the negative direction within the
    /// same hardware event.
    /// Clients that rely on discrete scrolling should accumulate the
    /// value120 to multiples of 120 before processing the event.
    ///
    /// The value120 must not be zero.
    ///
    /// This event replaces the wl_pointer.axis_discrete event in clients
    /// supporting wl_pointer version 8 or later.
    ///
    /// Where a wl_pointer.axis_source event occurs in the same
    /// wl_pointer.frame, the axis source applies to this event.
    ///
    /// The order of wl_pointer.axis_value120 and wl_pointer.axis_source is
    /// not guaranteed.
    ///
    /// # Arguments
    ///
    /// - `axis`: axis type
    /// - `value120`: scroll distance as fraction of 120
    #[inline]
    fn axis_value120(
        &mut self,
        _slf: &Rc<MetaWlPointer>,
        axis: MetaWlPointerAxis,
        value120: i32,
    ) {
        let res = _slf.send_axis_value120(
            axis,
            value120,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_pointer.axis_value120 message: {}", Report::new(e));
        }
    }

    /// axis relative physical direction event
    ///
    /// Relative directional information of the entity causing the axis
    /// motion.
    ///
    /// For a wl_pointer.axis event, the wl_pointer.axis_relative_direction
    /// event specifies the movement direction of the entity causing the
    /// wl_pointer.axis event. For example:
    /// - if a user's fingers on a touchpad move down and this
    ///   causes a wl_pointer.axis vertical_scroll down event, the physical
    ///   direction is 'identical'
    /// - if a user's fingers on a touchpad move down and this causes a
    ///   wl_pointer.axis vertical_scroll up scroll up event ('natural
    ///   scrolling'), the physical direction is 'inverted'.
    ///
    /// A client may use this information to adjust scroll motion of
    /// components. Specifically, enabling natural scrolling causes the
    /// content to change direction compared to traditional scrolling.
    /// Some widgets like volume control sliders should usually match the
    /// physical direction regardless of whether natural scrolling is
    /// active. This event enables clients to match the scroll direction of
    /// a widget to the physical direction.
    ///
    /// This event does not occur on its own, it is coupled with a
    /// wl_pointer.axis event that represents this axis value.
    /// The protocol guarantees that each axis_relative_direction event is
    /// always followed by exactly one axis event with the same
    /// axis number within the same wl_pointer.frame. Note that the protocol
    /// allows for other events to occur between the axis_relative_direction
    /// and its coupled axis event.
    ///
    /// The axis number is identical to the axis number in the associated
    /// axis event.
    ///
    /// The order of wl_pointer.axis_relative_direction,
    /// wl_pointer.axis_discrete and wl_pointer.axis_source is not
    /// guaranteed.
    ///
    /// # Arguments
    ///
    /// - `axis`: axis type
    /// - `direction`: physical direction relative to axis motion
    #[inline]
    fn axis_relative_direction(
        &mut self,
        _slf: &Rc<MetaWlPointer>,
        axis: MetaWlPointerAxis,
        direction: MetaWlPointerAxisRelativeDirection,
    ) {
        let res = _slf.send_axis_relative_direction(
            axis,
            direction,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_pointer.axis_relative_direction message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWlPointer {
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
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg1 = if arg1 == 0 {
                    None
                } else {
                    let Some(arg1) = client.endpoint.lookup(arg1) else {
                        return Err(ObjectError);
                    };
                    let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                        return Err(ObjectError);
                    };
                    Some(arg1)
                };
                let arg1 = arg1.as_ref();
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                if let Some(handler) = handler {
                    (**handler).set_cursor(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultMessageHandler.set_cursor(&self, arg0, arg1, arg2, arg3);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).release(&self);
                } else {
                    DefaultMessageHandler.release(&self);
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
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let Some(arg1) = self.core.state.server.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    return Err(ObjectError);
                };
                let arg1 = &arg1;
                let arg2 = Fixed::from_wire(arg2 as i32);
                let arg3 = Fixed::from_wire(arg3 as i32);
                if let Some(handler) = handler {
                    (**handler).enter(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultMessageHandler.enter(&self, arg0, arg1, arg2, arg3);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let Some(arg1) = self.core.state.server.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    return Err(ObjectError);
                };
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).leave(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.leave(&self, arg0, arg1);
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
                let arg1 = Fixed::from_wire(arg1 as i32);
                let arg2 = Fixed::from_wire(arg2 as i32);
                if let Some(handler) = handler {
                    (**handler).motion(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.motion(&self, arg0, arg1, arg2);
                }
            }
            3 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg3 = MetaWlPointerButtonState(arg3);
                if let Some(handler) = handler {
                    (**handler).button(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultMessageHandler.button(&self, arg0, arg1, arg2, arg3);
                }
            }
            4 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg1 = MetaWlPointerAxis(arg1);
                let arg2 = Fixed::from_wire(arg2 as i32);
                if let Some(handler) = handler {
                    (**handler).axis(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.axis(&self, arg0, arg1, arg2);
                }
            }
            5 => {
                if let Some(handler) = handler {
                    (**handler).frame(&self);
                } else {
                    DefaultMessageHandler.frame(&self);
                }
            }
            6 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = MetaWlPointerAxisSource(arg0);
                if let Some(handler) = handler {
                    (**handler).axis_source(&self, arg0);
                } else {
                    DefaultMessageHandler.axis_source(&self, arg0);
                }
            }
            7 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg1 = MetaWlPointerAxis(arg1);
                if let Some(handler) = handler {
                    (**handler).axis_stop(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.axis_stop(&self, arg0, arg1);
                }
            }
            8 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = MetaWlPointerAxis(arg0);
                let arg1 = arg1 as i32;
                if let Some(handler) = handler {
                    (**handler).axis_discrete(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.axis_discrete(&self, arg0, arg1);
                }
            }
            9 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = MetaWlPointerAxis(arg0);
                let arg1 = arg1 as i32;
                if let Some(handler) = handler {
                    (**handler).axis_value120(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.axis_value120(&self, arg0, arg1);
                }
            }
            10 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = MetaWlPointerAxis(arg0);
                let arg1 = MetaWlPointerAxisRelativeDirection(arg1);
                if let Some(handler) = handler {
                    (**handler).axis_relative_direction(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.axis_relative_direction(&self, arg0, arg1);
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

impl MetaWlPointer {
    /// Since when the error.role enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ROLE__SINCE: u32 = 1;

    /// Since when the button_state.released enum variant is available.
    #[allow(dead_code)]
    pub const ENM__BUTTON_STATE_RELEASED__SINCE: u32 = 1;
    /// Since when the button_state.pressed enum variant is available.
    #[allow(dead_code)]
    pub const ENM__BUTTON_STATE_PRESSED__SINCE: u32 = 1;

    /// Since when the axis.vertical_scroll enum variant is available.
    #[allow(dead_code)]
    pub const ENM__AXIS_VERTICAL_SCROLL__SINCE: u32 = 1;
    /// Since when the axis.horizontal_scroll enum variant is available.
    #[allow(dead_code)]
    pub const ENM__AXIS_HORIZONTAL_SCROLL__SINCE: u32 = 1;

    /// Since when the axis_source.wheel enum variant is available.
    #[allow(dead_code)]
    pub const ENM__AXIS_SOURCE_WHEEL__SINCE: u32 = 1;
    /// Since when the axis_source.finger enum variant is available.
    #[allow(dead_code)]
    pub const ENM__AXIS_SOURCE_FINGER__SINCE: u32 = 1;
    /// Since when the axis_source.continuous enum variant is available.
    #[allow(dead_code)]
    pub const ENM__AXIS_SOURCE_CONTINUOUS__SINCE: u32 = 1;
    /// Since when the axis_source.wheel_tilt enum variant is available.
    #[allow(dead_code)]
    pub const ENM__AXIS_SOURCE_WHEEL_TILT__SINCE: u32 = 6;

    /// Since when the axis_relative_direction.identical enum variant is available.
    #[allow(dead_code)]
    pub const ENM__AXIS_RELATIVE_DIRECTION_IDENTICAL__SINCE: u32 = 1;
    /// Since when the axis_relative_direction.inverted enum variant is available.
    #[allow(dead_code)]
    pub const ENM__AXIS_RELATIVE_DIRECTION_INVERTED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWlPointerError(pub u32);

impl MetaWlPointerError {
    /// given wl_surface has another role
    #[allow(dead_code)]
    pub const ROLE: Self = Self(0);
}

impl Debug for MetaWlPointerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ROLE => "ROLE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// physical button state
///
/// Describes the physical state of a button that produced the button
/// event.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWlPointerButtonState(pub u32);

impl MetaWlPointerButtonState {
    /// the button is not pressed
    #[allow(dead_code)]
    pub const RELEASED: Self = Self(0);

    /// the button is pressed
    #[allow(dead_code)]
    pub const PRESSED: Self = Self(1);
}

impl Debug for MetaWlPointerButtonState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::RELEASED => "RELEASED",
            Self::PRESSED => "PRESSED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// axis types
///
/// Describes the axis types of scroll events.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWlPointerAxis(pub u32);

impl MetaWlPointerAxis {
    /// vertical axis
    #[allow(dead_code)]
    pub const VERTICAL_SCROLL: Self = Self(0);

    /// horizontal axis
    #[allow(dead_code)]
    pub const HORIZONTAL_SCROLL: Self = Self(1);
}

impl Debug for MetaWlPointerAxis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::VERTICAL_SCROLL => "VERTICAL_SCROLL",
            Self::HORIZONTAL_SCROLL => "HORIZONTAL_SCROLL",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// axis source types
///
/// Describes the source types for axis events. This indicates to the
/// client how an axis event was physically generated; a client may
/// adjust the user interface accordingly. For example, scroll events
/// from a "finger" source may be in a smooth coordinate space with
/// kinetic scrolling whereas a "wheel" source may be in discrete steps
/// of a number of lines.
///
/// The "continuous" axis source is a device generating events in a
/// continuous coordinate space, but using something other than a
/// finger. One example for this source is button-based scrolling where
/// the vertical motion of a device is converted to scroll events while
/// a button is held down.
///
/// The "wheel tilt" axis source indicates that the actual device is a
/// wheel but the scroll event is not caused by a rotation but a
/// (usually sideways) tilt of the wheel.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWlPointerAxisSource(pub u32);

impl MetaWlPointerAxisSource {
    /// a physical wheel rotation
    #[allow(dead_code)]
    pub const WHEEL: Self = Self(0);

    /// finger on a touch surface
    #[allow(dead_code)]
    pub const FINGER: Self = Self(1);

    /// continuous coordinate space
    #[allow(dead_code)]
    pub const CONTINUOUS: Self = Self(2);

    /// a physical wheel tilt
    #[allow(dead_code)]
    pub const WHEEL_TILT: Self = Self(3);
}

impl Debug for MetaWlPointerAxisSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::WHEEL => "WHEEL",
            Self::FINGER => "FINGER",
            Self::CONTINUOUS => "CONTINUOUS",
            Self::WHEEL_TILT => "WHEEL_TILT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// axis relative direction
///
/// This specifies the direction of the physical motion that caused a
/// wl_pointer.axis event, relative to the wl_pointer.axis direction.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWlPointerAxisRelativeDirection(pub u32);

impl MetaWlPointerAxisRelativeDirection {
    /// physical motion matches axis direction
    #[allow(dead_code)]
    pub const IDENTICAL: Self = Self(0);

    /// physical motion is the inverse of the axis direction
    #[allow(dead_code)]
    pub const INVERTED: Self = Self(1);
}

impl Debug for MetaWlPointerAxisRelativeDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::IDENTICAL => "IDENTICAL",
            Self::INVERTED => "INVERTED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
