//! pad ring
//!
//! A circular interaction area, such as the touch ring on the Wacom Intuos
//! Pro series tablets.
//!
//! Events on a ring are logically grouped by the zwp_tablet_pad_ring_v2.frame
//! event.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_tablet_pad_ring_v2 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpTabletPadRingV2 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpTabletPadRingV2Handler>,
}

struct DefaultHandler;

impl ZwpTabletPadRingV2Handler for DefaultHandler { }

impl ConcreteObject for ZwpTabletPadRingV2 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpTabletPadRingV2;
    const INTERFACE_NAME: &str = "zwp_tablet_pad_ring_v2";
}

impl ZwpTabletPadRingV2 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpTabletPadRingV2Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpTabletPadRingV2Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpTabletPadRingV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpTabletPadRingV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpTabletPadRingV2 {
    /// Since when the set_feedback message is available.
    pub const MSG__SET_FEEDBACK__SINCE: u32 = 1;

    /// set compositor feedback
    ///
    /// Request that the compositor use the provided feedback string
    /// associated with this ring. This request should be issued immediately
    /// after a zwp_tablet_pad_group_v2.mode_switch event from the corresponding
    /// group is received, or whenever the ring is mapped to a different
    /// action. See zwp_tablet_pad_group_v2.mode_switch for more details.
    ///
    /// Clients are encouraged to provide context-aware descriptions for
    /// the actions associated with the ring; compositors may use this
    /// information to offer visual feedback about the button layout
    /// (eg. on-screen displays).
    ///
    /// The provided string 'description' is a UTF-8 encoded string to be
    /// associated with this ring, and is considered user-visible; general
    /// internationalization rules apply.
    ///
    /// The serial argument will be that of the last
    /// zwp_tablet_pad_group_v2.mode_switch event received for the group of this
    /// ring. Requests providing other serials than the most recent one will be
    /// ignored.
    ///
    /// # Arguments
    ///
    /// - `description`: ring description
    /// - `serial`: serial of the mode switch event
    #[inline]
    pub fn try_send_set_feedback(
        &self,
        description: &str,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            description,
            serial,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_tablet_pad_ring_v2#{}.set_feedback(description: {:?}, serial: {})\n", id, arg0, arg1);
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
            0,
        ]);
        fmt.string(arg0);
        fmt.words([
            arg1,
        ]);
        Ok(())
    }

    /// set compositor feedback
    ///
    /// Request that the compositor use the provided feedback string
    /// associated with this ring. This request should be issued immediately
    /// after a zwp_tablet_pad_group_v2.mode_switch event from the corresponding
    /// group is received, or whenever the ring is mapped to a different
    /// action. See zwp_tablet_pad_group_v2.mode_switch for more details.
    ///
    /// Clients are encouraged to provide context-aware descriptions for
    /// the actions associated with the ring; compositors may use this
    /// information to offer visual feedback about the button layout
    /// (eg. on-screen displays).
    ///
    /// The provided string 'description' is a UTF-8 encoded string to be
    /// associated with this ring, and is considered user-visible; general
    /// internationalization rules apply.
    ///
    /// The serial argument will be that of the last
    /// zwp_tablet_pad_group_v2.mode_switch event received for the group of this
    /// ring. Requests providing other serials than the most recent one will be
    /// ignored.
    ///
    /// # Arguments
    ///
    /// - `description`: ring description
    /// - `serial`: serial of the mode switch event
    #[inline]
    pub fn send_set_feedback(
        &self,
        description: &str,
        serial: u32,
    ) {
        let res = self.try_send_set_feedback(
            description,
            serial,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_pad_ring_v2.set_feedback", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the ring object
    ///
    /// This destroys the client's resource for this ring object.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_tablet_pad_ring_v2#{}.destroy()\n", id);
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
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the ring object
    ///
    /// This destroys the client's resource for this ring object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwp_tablet_pad_ring_v2.destroy", &e);
        }
    }

    /// Since when the source message is available.
    pub const MSG__SOURCE__SINCE: u32 = 1;

    /// ring event source
    ///
    /// Source information for ring events.
    ///
    /// This event does not occur on its own. It is sent before a
    /// zwp_tablet_pad_ring_v2.frame event and carries the source information
    /// for all events within that frame.
    ///
    /// The source specifies how this event was generated. If the source is
    /// zwp_tablet_pad_ring_v2.source.finger, a zwp_tablet_pad_ring_v2.stop event
    /// will be sent when the user lifts the finger off the device.
    ///
    /// This event is optional. If the source is unknown for an interaction,
    /// no event is sent.
    ///
    /// # Arguments
    ///
    /// - `source`: the event source
    #[inline]
    pub fn try_send_source(
        &self,
        source: ZwpTabletPadRingV2Source,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            source,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: ZwpTabletPadRingV2Source) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_pad_ring_v2#{}.source(source: {:?})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// ring event source
    ///
    /// Source information for ring events.
    ///
    /// This event does not occur on its own. It is sent before a
    /// zwp_tablet_pad_ring_v2.frame event and carries the source information
    /// for all events within that frame.
    ///
    /// The source specifies how this event was generated. If the source is
    /// zwp_tablet_pad_ring_v2.source.finger, a zwp_tablet_pad_ring_v2.stop event
    /// will be sent when the user lifts the finger off the device.
    ///
    /// This event is optional. If the source is unknown for an interaction,
    /// no event is sent.
    ///
    /// # Arguments
    ///
    /// - `source`: the event source
    #[inline]
    pub fn send_source(
        &self,
        source: ZwpTabletPadRingV2Source,
    ) {
        let res = self.try_send_source(
            source,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_pad_ring_v2.source", &e);
        }
    }

    /// Since when the angle message is available.
    pub const MSG__ANGLE__SINCE: u32 = 1;

    /// angle changed
    ///
    /// Sent whenever the angle on a ring changes.
    ///
    /// The angle is provided in degrees clockwise from the logical
    /// north of the ring in the pad's current rotation.
    ///
    /// # Arguments
    ///
    /// - `degrees`: the current angle in degrees
    #[inline]
    pub fn try_send_angle(
        &self,
        degrees: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            degrees,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: Fixed) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_pad_ring_v2#{}.angle(degrees: {})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
            arg0.to_wire() as u32,
        ]);
        Ok(())
    }

    /// angle changed
    ///
    /// Sent whenever the angle on a ring changes.
    ///
    /// The angle is provided in degrees clockwise from the logical
    /// north of the ring in the pad's current rotation.
    ///
    /// # Arguments
    ///
    /// - `degrees`: the current angle in degrees
    #[inline]
    pub fn send_angle(
        &self,
        degrees: Fixed,
    ) {
        let res = self.try_send_angle(
            degrees,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_pad_ring_v2.angle", &e);
        }
    }

    /// Since when the stop message is available.
    pub const MSG__STOP__SINCE: u32 = 1;

    /// interaction stopped
    ///
    /// Stop notification for ring events.
    ///
    /// For some zwp_tablet_pad_ring_v2.source types, a zwp_tablet_pad_ring_v2.stop
    /// event is sent to notify a client that the interaction with the ring
    /// has terminated. This enables the client to implement kinetic scrolling.
    /// See the zwp_tablet_pad_ring_v2.source documentation for information on
    /// when this event may be generated.
    ///
    /// Any zwp_tablet_pad_ring_v2.angle events with the same source after this
    /// event should be considered as the start of a new interaction.
    #[inline]
    pub fn try_send_stop(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_pad_ring_v2#{}.stop()\n", client_id, id);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id);
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
        ]);
        Ok(())
    }

    /// interaction stopped
    ///
    /// Stop notification for ring events.
    ///
    /// For some zwp_tablet_pad_ring_v2.source types, a zwp_tablet_pad_ring_v2.stop
    /// event is sent to notify a client that the interaction with the ring
    /// has terminated. This enables the client to implement kinetic scrolling.
    /// See the zwp_tablet_pad_ring_v2.source documentation for information on
    /// when this event may be generated.
    ///
    /// Any zwp_tablet_pad_ring_v2.angle events with the same source after this
    /// event should be considered as the start of a new interaction.
    #[inline]
    pub fn send_stop(
        &self,
    ) {
        let res = self.try_send_stop(
        );
        if let Err(e) = res {
            log_send("zwp_tablet_pad_ring_v2.stop", &e);
        }
    }

    /// Since when the frame message is available.
    pub const MSG__FRAME__SINCE: u32 = 1;

    /// end of a ring event sequence
    ///
    /// Indicates the end of a set of ring events that logically belong
    /// together. A client is expected to accumulate the data in all events
    /// within the frame before proceeding.
    ///
    /// All zwp_tablet_pad_ring_v2 events before a zwp_tablet_pad_ring_v2.frame event belong
    /// logically together. For example, on termination of a finger interaction
    /// on a ring the compositor will send a zwp_tablet_pad_ring_v2.source event,
    /// a zwp_tablet_pad_ring_v2.stop event and a zwp_tablet_pad_ring_v2.frame event.
    ///
    /// A zwp_tablet_pad_ring_v2.frame event is sent for every logical event
    /// group, even if the group only contains a single zwp_tablet_pad_ring_v2
    /// event. Specifically, a client may get a sequence: angle, frame,
    /// angle, frame, etc.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    #[inline]
    pub fn try_send_frame(
        &self,
        time: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            time,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_pad_ring_v2#{}.frame(time: {})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
            3,
            arg0,
        ]);
        Ok(())
    }

    /// end of a ring event sequence
    ///
    /// Indicates the end of a set of ring events that logically belong
    /// together. A client is expected to accumulate the data in all events
    /// within the frame before proceeding.
    ///
    /// All zwp_tablet_pad_ring_v2 events before a zwp_tablet_pad_ring_v2.frame event belong
    /// logically together. For example, on termination of a finger interaction
    /// on a ring the compositor will send a zwp_tablet_pad_ring_v2.source event,
    /// a zwp_tablet_pad_ring_v2.stop event and a zwp_tablet_pad_ring_v2.frame event.
    ///
    /// A zwp_tablet_pad_ring_v2.frame event is sent for every logical event
    /// group, even if the group only contains a single zwp_tablet_pad_ring_v2
    /// event. Specifically, a client may get a sequence: angle, frame,
    /// angle, frame, etc.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    #[inline]
    pub fn send_frame(
        &self,
        time: u32,
    ) {
        let res = self.try_send_frame(
            time,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_pad_ring_v2.frame", &e);
        }
    }
}

/// A message handler for [`ZwpTabletPadRingV2`] proxies.
pub trait ZwpTabletPadRingV2Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpTabletPadRingV2>) {
        slf.core.delete_id();
    }

    /// set compositor feedback
    ///
    /// Request that the compositor use the provided feedback string
    /// associated with this ring. This request should be issued immediately
    /// after a zwp_tablet_pad_group_v2.mode_switch event from the corresponding
    /// group is received, or whenever the ring is mapped to a different
    /// action. See zwp_tablet_pad_group_v2.mode_switch for more details.
    ///
    /// Clients are encouraged to provide context-aware descriptions for
    /// the actions associated with the ring; compositors may use this
    /// information to offer visual feedback about the button layout
    /// (eg. on-screen displays).
    ///
    /// The provided string 'description' is a UTF-8 encoded string to be
    /// associated with this ring, and is considered user-visible; general
    /// internationalization rules apply.
    ///
    /// The serial argument will be that of the last
    /// zwp_tablet_pad_group_v2.mode_switch event received for the group of this
    /// ring. Requests providing other serials than the most recent one will be
    /// ignored.
    ///
    /// # Arguments
    ///
    /// - `description`: ring description
    /// - `serial`: serial of the mode switch event
    #[inline]
    fn handle_set_feedback(
        &mut self,
        slf: &Rc<ZwpTabletPadRingV2>,
        description: &str,
        serial: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_feedback(
            description,
            serial,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_pad_ring_v2.set_feedback", &e);
        }
    }

    /// destroy the ring object
    ///
    /// This destroys the client's resource for this ring object.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwpTabletPadRingV2>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_pad_ring_v2.destroy", &e);
        }
    }

    /// ring event source
    ///
    /// Source information for ring events.
    ///
    /// This event does not occur on its own. It is sent before a
    /// zwp_tablet_pad_ring_v2.frame event and carries the source information
    /// for all events within that frame.
    ///
    /// The source specifies how this event was generated. If the source is
    /// zwp_tablet_pad_ring_v2.source.finger, a zwp_tablet_pad_ring_v2.stop event
    /// will be sent when the user lifts the finger off the device.
    ///
    /// This event is optional. If the source is unknown for an interaction,
    /// no event is sent.
    ///
    /// # Arguments
    ///
    /// - `source`: the event source
    #[inline]
    fn handle_source(
        &mut self,
        slf: &Rc<ZwpTabletPadRingV2>,
        source: ZwpTabletPadRingV2Source,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_source(
            source,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_pad_ring_v2.source", &e);
        }
    }

    /// angle changed
    ///
    /// Sent whenever the angle on a ring changes.
    ///
    /// The angle is provided in degrees clockwise from the logical
    /// north of the ring in the pad's current rotation.
    ///
    /// # Arguments
    ///
    /// - `degrees`: the current angle in degrees
    #[inline]
    fn handle_angle(
        &mut self,
        slf: &Rc<ZwpTabletPadRingV2>,
        degrees: Fixed,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_angle(
            degrees,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_pad_ring_v2.angle", &e);
        }
    }

    /// interaction stopped
    ///
    /// Stop notification for ring events.
    ///
    /// For some zwp_tablet_pad_ring_v2.source types, a zwp_tablet_pad_ring_v2.stop
    /// event is sent to notify a client that the interaction with the ring
    /// has terminated. This enables the client to implement kinetic scrolling.
    /// See the zwp_tablet_pad_ring_v2.source documentation for information on
    /// when this event may be generated.
    ///
    /// Any zwp_tablet_pad_ring_v2.angle events with the same source after this
    /// event should be considered as the start of a new interaction.
    #[inline]
    fn handle_stop(
        &mut self,
        slf: &Rc<ZwpTabletPadRingV2>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_stop(
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_pad_ring_v2.stop", &e);
        }
    }

    /// end of a ring event sequence
    ///
    /// Indicates the end of a set of ring events that logically belong
    /// together. A client is expected to accumulate the data in all events
    /// within the frame before proceeding.
    ///
    /// All zwp_tablet_pad_ring_v2 events before a zwp_tablet_pad_ring_v2.frame event belong
    /// logically together. For example, on termination of a finger interaction
    /// on a ring the compositor will send a zwp_tablet_pad_ring_v2.source event,
    /// a zwp_tablet_pad_ring_v2.stop event and a zwp_tablet_pad_ring_v2.frame event.
    ///
    /// A zwp_tablet_pad_ring_v2.frame event is sent for every logical event
    /// group, even if the group only contains a single zwp_tablet_pad_ring_v2
    /// event. Specifically, a client may get a sequence: angle, frame,
    /// angle, frame, etc.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    #[inline]
    fn handle_frame(
        &mut self,
        slf: &Rc<ZwpTabletPadRingV2>,
        time: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_frame(
            time,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_pad_ring_v2.frame", &e);
        }
    }
}

impl ObjectPrivate for ZwpTabletPadRingV2 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpTabletPadRingV2, version),
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
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "description")?;
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("serial")));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_tablet_pad_ring_v2#{}.set_feedback(description: {:?}, serial: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_feedback(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_feedback(&self, arg0, arg1);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_tablet_pad_ring_v2#{}.destroy()\n", client_id, id);
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = ZwpTabletPadRingV2Source(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: ZwpTabletPadRingV2Source) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_pad_ring_v2#{}.source(source: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_source(&self, arg0);
                } else {
                    DefaultHandler.handle_source(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = Fixed::from_wire(arg0 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: Fixed) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_pad_ring_v2#{}.angle(degrees: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_angle(&self, arg0);
                } else {
                    DefaultHandler.handle_angle(&self, arg0);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_pad_ring_v2#{}.stop()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_stop(&self);
                } else {
                    DefaultHandler.handle_stop(&self);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_pad_ring_v2#{}.frame(time: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_frame(&self, arg0);
                } else {
                    DefaultHandler.handle_frame(&self, arg0);
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
            0 => "set_feedback",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "source",
            1 => "angle",
            2 => "stop",
            3 => "frame",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwpTabletPadRingV2 {
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

impl ZwpTabletPadRingV2 {
    /// Since when the source.finger enum variant is available.
    pub const ENM__SOURCE_FINGER__SINCE: u32 = 1;
}

/// ring axis source
///
/// Describes the source types for ring events. This indicates to the
/// client how a ring event was physically generated; a client may
/// adjust the user interface accordingly. For example, events
/// from a "finger" source may trigger kinetic scrolling.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwpTabletPadRingV2Source(pub u32);

impl ZwpTabletPadRingV2Source {
    /// finger
    pub const FINGER: Self = Self(1);
}

impl Debug for ZwpTabletPadRingV2Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::FINGER => "FINGER",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
