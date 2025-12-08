//! a set of buttons, rings and strips
//!
//! A pad group describes a distinct (sub)set of buttons, rings and strips
//! present in the tablet. The criteria of this grouping is usually positional,
//! eg. if a tablet has buttons on the left and right side, 2 groups will be
//! presented. The physical arrangement of groups is undisclosed and may
//! change on the fly.
//!
//! Pad groups will announce their features during pad initialization. Between
//! the corresponding zwp_tablet_pad_v2.group event and zwp_tablet_pad_group_v2.done, the
//! pad group will announce the buttons, rings and strips contained in it,
//! plus the number of supported modes.
//!
//! Modes are a mechanism to allow multiple groups of actions for every element
//! in the pad group. The number of groups and available modes in each is
//! persistent across device plugs. The current mode is user-switchable, it
//! will be announced through the zwp_tablet_pad_group_v2.mode_switch event both
//! whenever it is switched, and after zwp_tablet_pad_v2.enter.
//!
//! The current mode logically applies to all elements in the pad group,
//! although it is at clients' discretion whether to actually perform different
//! actions, and/or issue the respective .set_feedback requests to notify the
//! compositor. See the zwp_tablet_pad_group_v2.mode_switch event for more details.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_tablet_pad_group_v2 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpTabletPadGroupV2 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpTabletPadGroupV2Handler>,
}

struct DefaultHandler;

impl ZwpTabletPadGroupV2Handler for DefaultHandler { }

impl ConcreteObject for ZwpTabletPadGroupV2 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpTabletPadGroupV2;
    const INTERFACE_NAME: &str = "zwp_tablet_pad_group_v2";
}

impl ZwpTabletPadGroupV2 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpTabletPadGroupV2Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpTabletPadGroupV2Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpTabletPadGroupV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpTabletPadGroupV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpTabletPadGroupV2 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the pad object
    ///
    /// Destroy the zwp_tablet_pad_group_v2 object. Objects created from this object
    /// are unaffected and should be destroyed separately.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_tablet_pad_group_v2#{}.destroy()\n", id);
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

    /// destroy the pad object
    ///
    /// Destroy the zwp_tablet_pad_group_v2 object. Objects created from this object
    /// are unaffected and should be destroyed separately.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwp_tablet_pad_group_v2.destroy", &e);
        }
    }

    /// Since when the buttons message is available.
    pub const MSG__BUTTONS__SINCE: u32 = 1;

    /// buttons announced
    ///
    /// Sent on zwp_tablet_pad_group_v2 initialization to announce the available
    /// buttons in the group. Button indices start at 0, a button may only be
    /// in one group at a time.
    ///
    /// This event is first sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event.
    ///
    /// Some buttons are reserved by the compositor. These buttons may not be
    /// assigned to any zwp_tablet_pad_group_v2. Compositors may broadcast this
    /// event in the case of changes to the mapping of these reserved buttons.
    /// If the compositor happens to reserve all buttons in a group, this event
    /// will be sent with an empty array.
    ///
    /// # Arguments
    ///
    /// - `buttons`: buttons in this group
    #[inline]
    pub fn try_send_buttons(
        &self,
        buttons: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            buttons,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: &[u8]) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_pad_group_v2#{}.buttons(buttons: {})\n", client_id, id, debug_array(arg0));
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
        ]);
        fmt.array(arg0);
        Ok(())
    }

    /// buttons announced
    ///
    /// Sent on zwp_tablet_pad_group_v2 initialization to announce the available
    /// buttons in the group. Button indices start at 0, a button may only be
    /// in one group at a time.
    ///
    /// This event is first sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event.
    ///
    /// Some buttons are reserved by the compositor. These buttons may not be
    /// assigned to any zwp_tablet_pad_group_v2. Compositors may broadcast this
    /// event in the case of changes to the mapping of these reserved buttons.
    /// If the compositor happens to reserve all buttons in a group, this event
    /// will be sent with an empty array.
    ///
    /// # Arguments
    ///
    /// - `buttons`: buttons in this group
    #[inline]
    pub fn send_buttons(
        &self,
        buttons: &[u8],
    ) {
        let res = self.try_send_buttons(
            buttons,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_pad_group_v2.buttons", &e);
        }
    }

    /// Since when the ring message is available.
    pub const MSG__RING__SINCE: u32 = 1;

    /// ring announced
    ///
    /// Sent on zwp_tablet_pad_group_v2 initialization to announce available rings.
    /// One event is sent for each ring available on this pad group.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event.
    ///
    /// # Arguments
    ///
    /// - `ring`:
    #[inline]
    pub fn try_send_ring(
        &self,
        ring: &Rc<ZwpTabletPadRingV2>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            ring,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateClientId("ring", e)))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_pad_group_v2#{}.ring(ring: zwp_tablet_pad_ring_v2#{})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// ring announced
    ///
    /// Sent on zwp_tablet_pad_group_v2 initialization to announce available rings.
    /// One event is sent for each ring available on this pad group.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event.
    ///
    /// # Arguments
    ///
    /// - `ring`:
    #[inline]
    pub fn send_ring(
        &self,
        ring: &Rc<ZwpTabletPadRingV2>,
    ) {
        let res = self.try_send_ring(
            ring,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_pad_group_v2.ring", &e);
        }
    }

    /// ring announced
    ///
    /// Sent on zwp_tablet_pad_group_v2 initialization to announce available rings.
    /// One event is sent for each ring available on this pad group.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event.
    #[inline]
    pub fn new_try_send_ring(
        &self,
    ) -> Result<Rc<ZwpTabletPadRingV2>, ObjectError> {
        let ring = self.core.create_child();
        self.try_send_ring(
            &ring,
        )?;
        Ok(ring)
    }

    /// ring announced
    ///
    /// Sent on zwp_tablet_pad_group_v2 initialization to announce available rings.
    /// One event is sent for each ring available on this pad group.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event.
    #[inline]
    pub fn new_send_ring(
        &self,
    ) -> Rc<ZwpTabletPadRingV2> {
        let ring = self.core.create_child();
        self.send_ring(
            &ring,
        );
        ring
    }

    /// Since when the strip message is available.
    pub const MSG__STRIP__SINCE: u32 = 1;

    /// strip announced
    ///
    /// Sent on zwp_tablet_pad_v2 initialization to announce available strips.
    /// One event is sent for each strip available on this pad group.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event.
    ///
    /// # Arguments
    ///
    /// - `strip`:
    #[inline]
    pub fn try_send_strip(
        &self,
        strip: &Rc<ZwpTabletPadStripV2>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            strip,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateClientId("strip", e)))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_pad_group_v2#{}.strip(strip: zwp_tablet_pad_strip_v2#{})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// strip announced
    ///
    /// Sent on zwp_tablet_pad_v2 initialization to announce available strips.
    /// One event is sent for each strip available on this pad group.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event.
    ///
    /// # Arguments
    ///
    /// - `strip`:
    #[inline]
    pub fn send_strip(
        &self,
        strip: &Rc<ZwpTabletPadStripV2>,
    ) {
        let res = self.try_send_strip(
            strip,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_pad_group_v2.strip", &e);
        }
    }

    /// strip announced
    ///
    /// Sent on zwp_tablet_pad_v2 initialization to announce available strips.
    /// One event is sent for each strip available on this pad group.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event.
    #[inline]
    pub fn new_try_send_strip(
        &self,
    ) -> Result<Rc<ZwpTabletPadStripV2>, ObjectError> {
        let strip = self.core.create_child();
        self.try_send_strip(
            &strip,
        )?;
        Ok(strip)
    }

    /// strip announced
    ///
    /// Sent on zwp_tablet_pad_v2 initialization to announce available strips.
    /// One event is sent for each strip available on this pad group.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event.
    #[inline]
    pub fn new_send_strip(
        &self,
    ) -> Rc<ZwpTabletPadStripV2> {
        let strip = self.core.create_child();
        self.send_strip(
            &strip,
        );
        strip
    }

    /// Since when the modes message is available.
    pub const MSG__MODES__SINCE: u32 = 1;

    /// mode-switch ability announced
    ///
    /// Sent on zwp_tablet_pad_group_v2 initialization to announce that the pad
    /// group may switch between modes. A client may use a mode to store a
    /// specific configuration for buttons, rings and strips and use the
    /// zwp_tablet_pad_group_v2.mode_switch event to toggle between these
    /// configurations. Mode indices start at 0.
    ///
    /// Switching modes is compositor-dependent. See the
    /// zwp_tablet_pad_group_v2.mode_switch event for more details.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event. This event is only sent when more than
    /// more than one mode is available.
    ///
    /// # Arguments
    ///
    /// - `modes`: the number of modes
    #[inline]
    pub fn try_send_modes(
        &self,
        modes: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            modes,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_pad_group_v2#{}.modes(modes: {})\n", client_id, id, arg0);
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

    /// mode-switch ability announced
    ///
    /// Sent on zwp_tablet_pad_group_v2 initialization to announce that the pad
    /// group may switch between modes. A client may use a mode to store a
    /// specific configuration for buttons, rings and strips and use the
    /// zwp_tablet_pad_group_v2.mode_switch event to toggle between these
    /// configurations. Mode indices start at 0.
    ///
    /// Switching modes is compositor-dependent. See the
    /// zwp_tablet_pad_group_v2.mode_switch event for more details.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event. This event is only sent when more than
    /// more than one mode is available.
    ///
    /// # Arguments
    ///
    /// - `modes`: the number of modes
    #[inline]
    pub fn send_modes(
        &self,
        modes: u32,
    ) {
        let res = self.try_send_modes(
            modes,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_pad_group_v2.modes", &e);
        }
    }

    /// Since when the done message is available.
    pub const MSG__DONE__SINCE: u32 = 1;

    /// tablet group description events sequence complete
    ///
    /// This event is sent immediately to signal the end of the initial
    /// burst of descriptive events. A client may consider the static
    /// description of the tablet to be complete and finalize initialization
    /// of the tablet group.
    #[inline]
    pub fn try_send_done(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_pad_group_v2#{}.done()\n", client_id, id);
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
            4,
        ]);
        Ok(())
    }

    /// tablet group description events sequence complete
    ///
    /// This event is sent immediately to signal the end of the initial
    /// burst of descriptive events. A client may consider the static
    /// description of the tablet to be complete and finalize initialization
    /// of the tablet group.
    #[inline]
    pub fn send_done(
        &self,
    ) {
        let res = self.try_send_done(
        );
        if let Err(e) = res {
            log_send("zwp_tablet_pad_group_v2.done", &e);
        }
    }

    /// Since when the mode_switch message is available.
    pub const MSG__MODE_SWITCH__SINCE: u32 = 1;

    /// mode switch event
    ///
    /// Notification that the mode was switched.
    ///
    /// A mode applies to all buttons, rings, strips and dials in a group
    /// simultaneously, but a client is not required to assign different actions
    /// for each mode. For example, a client may have mode-specific button
    /// mappings but map the ring to vertical scrolling in all modes. Mode
    /// indices start at 0.
    ///
    /// Switching modes is compositor-dependent. The compositor may provide
    /// visual cues to the user about the mode, e.g. by toggling LEDs on
    /// the tablet device. Mode-switching may be software-controlled or
    /// controlled by one or more physical buttons. For example, on a Wacom
    /// Intuos Pro, the button inside the ring may be assigned to switch
    /// between modes.
    ///
    /// The compositor will also send this event after zwp_tablet_pad_v2.enter on
    /// each group in order to notify of the current mode. Groups that only
    /// feature one mode will use mode=0 when emitting this event.
    ///
    /// If a button action in the new mode differs from the action in the
    /// previous mode, the client should immediately issue a
    /// zwp_tablet_pad_v2.set_feedback request for each changed button.
    ///
    /// If a ring, strip or dial action in the new mode differs from the action
    /// in the previous mode, the client should immediately issue a
    /// zwp_tablet_ring_v2.set_feedback, zwp_tablet_strip_v2.set_feedback or
    /// zwp_tablet_dial_v2.set_feedback request for each changed ring, strip or dial.
    ///
    /// # Arguments
    ///
    /// - `time`: the time of the event with millisecond granularity
    /// - `serial`:
    /// - `mode`: the new mode of the pad
    #[inline]
    pub fn try_send_mode_switch(
        &self,
        time: u32,
        serial: u32,
        mode: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            time,
            serial,
            mode,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_pad_group_v2#{}.mode_switch(time: {}, serial: {}, mode: {})\n", client_id, id, arg0, arg1, arg2);
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
            5,
            arg0,
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// mode switch event
    ///
    /// Notification that the mode was switched.
    ///
    /// A mode applies to all buttons, rings, strips and dials in a group
    /// simultaneously, but a client is not required to assign different actions
    /// for each mode. For example, a client may have mode-specific button
    /// mappings but map the ring to vertical scrolling in all modes. Mode
    /// indices start at 0.
    ///
    /// Switching modes is compositor-dependent. The compositor may provide
    /// visual cues to the user about the mode, e.g. by toggling LEDs on
    /// the tablet device. Mode-switching may be software-controlled or
    /// controlled by one or more physical buttons. For example, on a Wacom
    /// Intuos Pro, the button inside the ring may be assigned to switch
    /// between modes.
    ///
    /// The compositor will also send this event after zwp_tablet_pad_v2.enter on
    /// each group in order to notify of the current mode. Groups that only
    /// feature one mode will use mode=0 when emitting this event.
    ///
    /// If a button action in the new mode differs from the action in the
    /// previous mode, the client should immediately issue a
    /// zwp_tablet_pad_v2.set_feedback request for each changed button.
    ///
    /// If a ring, strip or dial action in the new mode differs from the action
    /// in the previous mode, the client should immediately issue a
    /// zwp_tablet_ring_v2.set_feedback, zwp_tablet_strip_v2.set_feedback or
    /// zwp_tablet_dial_v2.set_feedback request for each changed ring, strip or dial.
    ///
    /// # Arguments
    ///
    /// - `time`: the time of the event with millisecond granularity
    /// - `serial`:
    /// - `mode`: the new mode of the pad
    #[inline]
    pub fn send_mode_switch(
        &self,
        time: u32,
        serial: u32,
        mode: u32,
    ) {
        let res = self.try_send_mode_switch(
            time,
            serial,
            mode,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_pad_group_v2.mode_switch", &e);
        }
    }

    /// Since when the dial message is available.
    pub const MSG__DIAL__SINCE: u32 = 2;

    /// dial announced
    ///
    /// Sent on zwp_tablet_pad_v2 initialization to announce available dials.
    /// One event is sent for each dial available on this pad group.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event.
    ///
    /// # Arguments
    ///
    /// - `dial`:
    #[inline]
    pub fn try_send_dial(
        &self,
        dial: &Rc<ZwpTabletPadDialV2>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            dial,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateClientId("dial", e)))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_pad_group_v2#{}.dial(dial: zwp_tablet_pad_dial_v2#{})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0_id);
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
            6,
            arg0_id,
        ]);
        Ok(())
    }

    /// dial announced
    ///
    /// Sent on zwp_tablet_pad_v2 initialization to announce available dials.
    /// One event is sent for each dial available on this pad group.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event.
    ///
    /// # Arguments
    ///
    /// - `dial`:
    #[inline]
    pub fn send_dial(
        &self,
        dial: &Rc<ZwpTabletPadDialV2>,
    ) {
        let res = self.try_send_dial(
            dial,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_pad_group_v2.dial", &e);
        }
    }

    /// dial announced
    ///
    /// Sent on zwp_tablet_pad_v2 initialization to announce available dials.
    /// One event is sent for each dial available on this pad group.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event.
    #[inline]
    pub fn new_try_send_dial(
        &self,
    ) -> Result<Rc<ZwpTabletPadDialV2>, ObjectError> {
        let dial = self.core.create_child();
        self.try_send_dial(
            &dial,
        )?;
        Ok(dial)
    }

    /// dial announced
    ///
    /// Sent on zwp_tablet_pad_v2 initialization to announce available dials.
    /// One event is sent for each dial available on this pad group.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event.
    #[inline]
    pub fn new_send_dial(
        &self,
    ) -> Rc<ZwpTabletPadDialV2> {
        let dial = self.core.create_child();
        self.send_dial(
            &dial,
        );
        dial
    }
}

/// A message handler for [`ZwpTabletPadGroupV2`] proxies.
pub trait ZwpTabletPadGroupV2Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpTabletPadGroupV2>) {
        slf.core.delete_id();
    }

    /// destroy the pad object
    ///
    /// Destroy the zwp_tablet_pad_group_v2 object. Objects created from this object
    /// are unaffected and should be destroyed separately.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwpTabletPadGroupV2>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_pad_group_v2.destroy", &e);
        }
    }

    /// buttons announced
    ///
    /// Sent on zwp_tablet_pad_group_v2 initialization to announce the available
    /// buttons in the group. Button indices start at 0, a button may only be
    /// in one group at a time.
    ///
    /// This event is first sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event.
    ///
    /// Some buttons are reserved by the compositor. These buttons may not be
    /// assigned to any zwp_tablet_pad_group_v2. Compositors may broadcast this
    /// event in the case of changes to the mapping of these reserved buttons.
    /// If the compositor happens to reserve all buttons in a group, this event
    /// will be sent with an empty array.
    ///
    /// # Arguments
    ///
    /// - `buttons`: buttons in this group
    #[inline]
    fn handle_buttons(
        &mut self,
        slf: &Rc<ZwpTabletPadGroupV2>,
        buttons: &[u8],
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_buttons(
            buttons,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_pad_group_v2.buttons", &e);
        }
    }

    /// ring announced
    ///
    /// Sent on zwp_tablet_pad_group_v2 initialization to announce available rings.
    /// One event is sent for each ring available on this pad group.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event.
    ///
    /// # Arguments
    ///
    /// - `ring`:
    #[inline]
    fn handle_ring(
        &mut self,
        slf: &Rc<ZwpTabletPadGroupV2>,
        ring: &Rc<ZwpTabletPadRingV2>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_ring(
            ring,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_pad_group_v2.ring", &e);
        }
    }

    /// strip announced
    ///
    /// Sent on zwp_tablet_pad_v2 initialization to announce available strips.
    /// One event is sent for each strip available on this pad group.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event.
    ///
    /// # Arguments
    ///
    /// - `strip`:
    #[inline]
    fn handle_strip(
        &mut self,
        slf: &Rc<ZwpTabletPadGroupV2>,
        strip: &Rc<ZwpTabletPadStripV2>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_strip(
            strip,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_pad_group_v2.strip", &e);
        }
    }

    /// mode-switch ability announced
    ///
    /// Sent on zwp_tablet_pad_group_v2 initialization to announce that the pad
    /// group may switch between modes. A client may use a mode to store a
    /// specific configuration for buttons, rings and strips and use the
    /// zwp_tablet_pad_group_v2.mode_switch event to toggle between these
    /// configurations. Mode indices start at 0.
    ///
    /// Switching modes is compositor-dependent. See the
    /// zwp_tablet_pad_group_v2.mode_switch event for more details.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event. This event is only sent when more than
    /// more than one mode is available.
    ///
    /// # Arguments
    ///
    /// - `modes`: the number of modes
    #[inline]
    fn handle_modes(
        &mut self,
        slf: &Rc<ZwpTabletPadGroupV2>,
        modes: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_modes(
            modes,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_pad_group_v2.modes", &e);
        }
    }

    /// tablet group description events sequence complete
    ///
    /// This event is sent immediately to signal the end of the initial
    /// burst of descriptive events. A client may consider the static
    /// description of the tablet to be complete and finalize initialization
    /// of the tablet group.
    #[inline]
    fn handle_done(
        &mut self,
        slf: &Rc<ZwpTabletPadGroupV2>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_done(
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_pad_group_v2.done", &e);
        }
    }

    /// mode switch event
    ///
    /// Notification that the mode was switched.
    ///
    /// A mode applies to all buttons, rings, strips and dials in a group
    /// simultaneously, but a client is not required to assign different actions
    /// for each mode. For example, a client may have mode-specific button
    /// mappings but map the ring to vertical scrolling in all modes. Mode
    /// indices start at 0.
    ///
    /// Switching modes is compositor-dependent. The compositor may provide
    /// visual cues to the user about the mode, e.g. by toggling LEDs on
    /// the tablet device. Mode-switching may be software-controlled or
    /// controlled by one or more physical buttons. For example, on a Wacom
    /// Intuos Pro, the button inside the ring may be assigned to switch
    /// between modes.
    ///
    /// The compositor will also send this event after zwp_tablet_pad_v2.enter on
    /// each group in order to notify of the current mode. Groups that only
    /// feature one mode will use mode=0 when emitting this event.
    ///
    /// If a button action in the new mode differs from the action in the
    /// previous mode, the client should immediately issue a
    /// zwp_tablet_pad_v2.set_feedback request for each changed button.
    ///
    /// If a ring, strip or dial action in the new mode differs from the action
    /// in the previous mode, the client should immediately issue a
    /// zwp_tablet_ring_v2.set_feedback, zwp_tablet_strip_v2.set_feedback or
    /// zwp_tablet_dial_v2.set_feedback request for each changed ring, strip or dial.
    ///
    /// # Arguments
    ///
    /// - `time`: the time of the event with millisecond granularity
    /// - `serial`:
    /// - `mode`: the new mode of the pad
    #[inline]
    fn handle_mode_switch(
        &mut self,
        slf: &Rc<ZwpTabletPadGroupV2>,
        time: u32,
        serial: u32,
        mode: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_mode_switch(
            time,
            serial,
            mode,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_pad_group_v2.mode_switch", &e);
        }
    }

    /// dial announced
    ///
    /// Sent on zwp_tablet_pad_v2 initialization to announce available dials.
    /// One event is sent for each dial available on this pad group.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event.
    ///
    /// # Arguments
    ///
    /// - `dial`:
    #[inline]
    fn handle_dial(
        &mut self,
        slf: &Rc<ZwpTabletPadGroupV2>,
        dial: &Rc<ZwpTabletPadDialV2>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_dial(
            dial,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_pad_group_v2.dial", &e);
        }
    }
}

impl ObjectPrivate for ZwpTabletPadGroupV2 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpTabletPadGroupV2, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_tablet_pad_group_v2#{}.destroy()\n", client_id, id);
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
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_array(msg, offset, "buttons")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_pad_group_v2#{}.buttons(buttons: {})\n", id, debug_array(arg0));
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_buttons(&self, arg0);
                } else {
                    DefaultHandler.handle_buttons(&self, arg0);
                }
            }
            1 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_pad_group_v2#{}.ring(ring: zwp_tablet_pad_ring_v2#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = ZwpTabletPadRingV2::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetServerId(arg0_id, "ring", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_ring(&self, arg0);
                } else {
                    DefaultHandler.handle_ring(&self, arg0);
                }
            }
            2 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_pad_group_v2#{}.strip(strip: zwp_tablet_pad_strip_v2#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = ZwpTabletPadStripV2::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetServerId(arg0_id, "strip", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_strip(&self, arg0);
                } else {
                    DefaultHandler.handle_strip(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_pad_group_v2#{}.modes(modes: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_modes(&self, arg0);
                } else {
                    DefaultHandler.handle_modes(&self, arg0);
                }
            }
            4 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_pad_group_v2#{}.done()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_done(&self);
                } else {
                    DefaultHandler.handle_done(&self);
                }
            }
            5 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_pad_group_v2#{}.mode_switch(time: {}, serial: {}, mode: {})\n", id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_mode_switch(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_mode_switch(&self, arg0, arg1, arg2);
                }
            }
            6 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_pad_group_v2#{}.dial(dial: zwp_tablet_pad_dial_v2#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = ZwpTabletPadDialV2::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetServerId(arg0_id, "dial", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_dial(&self, arg0);
                } else {
                    DefaultHandler.handle_dial(&self, arg0);
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
            0 => "buttons",
            1 => "ring",
            2 => "strip",
            3 => "modes",
            4 => "done",
            5 => "mode_switch",
            6 => "dial",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwpTabletPadGroupV2 {
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

