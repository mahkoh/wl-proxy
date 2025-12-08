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

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_tablet_pad_group_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpTabletPadGroupV2 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpTabletPadGroupV2MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpTabletPadGroupV2MessageHandler for DefaultMessageHandler { }

impl MetaZwpTabletPadGroupV2 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwpTabletPadGroupV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpTabletPadGroupV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpTabletPadGroupV2 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the pad object
    ///
    /// Destroy the zwp_tablet_pad_group_v2 object. Objects created from this object
    /// are unaffected and should be destroyed separately.
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

    /// Since when the buttons message is available.
    #[allow(dead_code)]
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
    pub fn send_buttons(
        &self,
        buttons: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            buttons,
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
            0,
        ]);
        fmt.array(arg0);
        Ok(())
    }

    /// Since when the ring message is available.
    #[allow(dead_code)]
    pub const MSG__RING__SINCE: u32 = 1;

    /// ring announced
    ///
    /// Sent on zwp_tablet_pad_group_v2 initialization to announce available rings.
    /// One event is sent for each ring available on this pad group.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event.
    #[inline]
    pub fn send_ring(
        &self,
        ring: &Rc<MetaZwpTabletPadRingV2>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            ring,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        arg0.generate_client_id(client, arg0_obj.clone())?;
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            1,
            arg0.client_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }

    /// Since when the strip message is available.
    #[allow(dead_code)]
    pub const MSG__STRIP__SINCE: u32 = 1;

    /// strip announced
    ///
    /// Sent on zwp_tablet_pad_v2 initialization to announce available strips.
    /// One event is sent for each strip available on this pad group.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event.
    #[inline]
    pub fn send_strip(
        &self,
        strip: &Rc<MetaZwpTabletPadStripV2>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            strip,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        arg0.generate_client_id(client, arg0_obj.clone())?;
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            2,
            arg0.client_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }

    /// Since when the modes message is available.
    #[allow(dead_code)]
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
    pub fn send_modes(
        &self,
        modes: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            modes,
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
            3,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the done message is available.
    #[allow(dead_code)]
    pub const MSG__DONE__SINCE: u32 = 1;

    /// tablet group description events sequence complete
    ///
    /// This event is sent immediately to signal the end of the initial
    /// burst of descriptive events. A client may consider the static
    /// description of the tablet to be complete and finalize initialization
    /// of the tablet group.
    #[inline]
    pub fn send_done(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            4,
        ]);
        Ok(())
    }

    /// Since when the mode_switch message is available.
    #[allow(dead_code)]
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
    pub fn send_mode_switch(
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
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            5,
            arg0,
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// Since when the dial message is available.
    #[allow(dead_code)]
    pub const MSG__DIAL__SINCE: u32 = 2;

    /// dial announced
    ///
    /// Sent on zwp_tablet_pad_v2 initialization to announce available dials.
    /// One event is sent for each dial available on this pad group.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_group_v2.done event.
    #[inline]
    pub fn send_dial(
        &self,
        dial: &Rc<MetaZwpTabletPadDialV2>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            dial,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        arg0.generate_client_id(client, arg0_obj.clone())?;
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            6,
            arg0.client_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpTabletPadGroupV2] proxies.
#[allow(dead_code)]
pub trait MetaZwpTabletPadGroupV2MessageHandler {
    /// destroy the pad object
    ///
    /// Destroy the zwp_tablet_pad_group_v2 object. Objects created from this object
    /// are unaffected and should be destroyed separately.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadGroupV2>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_group_v2.destroy message: {}", Report::new(e));
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
    fn buttons(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadGroupV2>,
        buttons: &[u8],
    ) {
        let res = _slf.send_buttons(
            buttons,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_group_v2.buttons message: {}", Report::new(e));
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
    fn ring(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadGroupV2>,
        ring: &Rc<MetaZwpTabletPadRingV2>,
    ) {
        let res = _slf.send_ring(
            ring,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_group_v2.ring message: {}", Report::new(e));
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
    fn strip(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadGroupV2>,
        strip: &Rc<MetaZwpTabletPadStripV2>,
    ) {
        let res = _slf.send_strip(
            strip,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_group_v2.strip message: {}", Report::new(e));
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
    fn modes(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadGroupV2>,
        modes: u32,
    ) {
        let res = _slf.send_modes(
            modes,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_group_v2.modes message: {}", Report::new(e));
        }
    }

    /// tablet group description events sequence complete
    ///
    /// This event is sent immediately to signal the end of the initial
    /// burst of descriptive events. A client may consider the static
    /// description of the tablet to be complete and finalize initialization
    /// of the tablet group.
    #[inline]
    fn done(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadGroupV2>,
    ) {
        let res = _slf.send_done(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_group_v2.done message: {}", Report::new(e));
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
    fn mode_switch(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadGroupV2>,
        time: u32,
        serial: u32,
        mode: u32,
    ) {
        let res = _slf.send_mode_switch(
            time,
            serial,
            mode,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_group_v2.mode_switch message: {}", Report::new(e));
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
    fn dial(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadGroupV2>,
        dial: &Rc<MetaZwpTabletPadDialV2>,
    ) {
        let res = _slf.send_dial(
            dial,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_group_v2.dial message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpTabletPadGroupV2 {
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
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError);
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError);
                    }
                    let start = offset;
                    offset += words;
                    &uapi::as_bytes(&msg[start..])[..len]
                };
                if offset != msg.len() {
                    return Err(ObjectError);
                }
                if let Some(handler) = handler {
                    (**handler).buttons(&self, arg0);
                } else {
                    DefaultMessageHandler.buttons(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaZwpTabletPadRingV2::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).ring(&self, arg0);
                } else {
                    DefaultMessageHandler.ring(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaZwpTabletPadStripV2::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).strip(&self, arg0);
                } else {
                    DefaultMessageHandler.strip(&self, arg0);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).modes(&self, arg0);
                } else {
                    DefaultMessageHandler.modes(&self, arg0);
                }
            }
            4 => {
                if let Some(handler) = handler {
                    (**handler).done(&self);
                } else {
                    DefaultMessageHandler.done(&self);
                }
            }
            5 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).mode_switch(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.mode_switch(&self, arg0, arg1, arg2);
                }
            }
            6 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaZwpTabletPadDialV2::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).dial(&self, arg0);
                } else {
                    DefaultMessageHandler.dial(&self, arg0);
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

