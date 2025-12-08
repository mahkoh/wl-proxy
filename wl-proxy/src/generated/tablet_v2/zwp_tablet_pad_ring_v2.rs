//! pad ring
//!
//! A circular interaction area, such as the touch ring on the Wacom Intuos
//! Pro series tablets.
//!
//! Events on a ring are logically grouped by the zwp_tablet_pad_ring_v2.frame
//! event.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_tablet_pad_ring_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpTabletPadRingV2 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpTabletPadRingV2MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpTabletPadRingV2MessageHandler for DefaultMessageHandler { }

impl MetaZwpTabletPadRingV2 {
    pub const XML_VERSION: u32 = 2;
}

impl MetaZwpTabletPadRingV2 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpTabletPadRingV2, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwpTabletPadRingV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpTabletPadRingV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpTabletPadRingV2 {
    /// Since when the set_feedback message is available.
    #[allow(dead_code)]
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
    pub fn send_set_feedback(
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
        fmt.string(arg0);
        fmt.words([
            arg1,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the ring object
    ///
    /// This destroys the client's resource for this ring object.
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
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the source message is available.
    #[allow(dead_code)]
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
    pub fn send_source(
        &self,
        source: MetaZwpTabletPadRingV2Source,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            source,
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
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the angle message is available.
    #[allow(dead_code)]
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
    pub fn send_angle(
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
            1,
            arg0.to_wire() as u32,
        ]);
        Ok(())
    }

    /// Since when the stop message is available.
    #[allow(dead_code)]
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
    pub fn send_stop(
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
            2,
        ]);
        Ok(())
    }

    /// Since when the frame message is available.
    #[allow(dead_code)]
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
    pub fn send_frame(
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
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpTabletPadRingV2] proxies.
#[allow(dead_code)]
pub trait MetaZwpTabletPadRingV2MessageHandler {
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
    fn set_feedback(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadRingV2>,
        description: &str,
        serial: u32,
    ) {
        let res = _slf.send_set_feedback(
            description,
            serial,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_ring_v2.set_feedback message: {}", Report::new(e));
        }
    }

    /// destroy the ring object
    ///
    /// This destroys the client's resource for this ring object.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadRingV2>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_ring_v2.destroy message: {}", Report::new(e));
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
    fn source(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadRingV2>,
        source: MetaZwpTabletPadRingV2Source,
    ) {
        let res = _slf.send_source(
            source,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_ring_v2.source message: {}", Report::new(e));
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
    fn angle(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadRingV2>,
        degrees: Fixed,
    ) {
        let res = _slf.send_angle(
            degrees,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_ring_v2.angle message: {}", Report::new(e));
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
    fn stop(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadRingV2>,
    ) {
        let res = _slf.send_stop(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_ring_v2.stop message: {}", Report::new(e));
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
    fn frame(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadRingV2>,
        time: u32,
    ) {
        let res = _slf.send_frame(
            time,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_ring_v2.frame message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpTabletPadRingV2 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
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
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError);
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError);
                        };
                        s
                    }
                };
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError);
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError);
                }
                if let Some(handler) = handler {
                    (**handler).set_feedback(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.set_feedback(&self, arg0, arg1);
                }
            }
            1 => {
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
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = MetaZwpTabletPadRingV2Source(arg0);
                if let Some(handler) = handler {
                    (**handler).source(&self, arg0);
                } else {
                    DefaultMessageHandler.source(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = Fixed::from_wire(arg0 as i32);
                if let Some(handler) = handler {
                    (**handler).angle(&self, arg0);
                } else {
                    DefaultMessageHandler.angle(&self, arg0);
                }
            }
            2 => {
                if let Some(handler) = handler {
                    (**handler).stop(&self);
                } else {
                    DefaultMessageHandler.stop(&self);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).frame(&self, arg0);
                } else {
                    DefaultMessageHandler.frame(&self, arg0);
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

impl MetaZwpTabletPadRingV2 {
    /// Since when the source.finger enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SOURCE_FINGER__SINCE: u32 = 1;
}

/// ring axis source
///
/// Describes the source types for ring events. This indicates to the
/// client how a ring event was physically generated; a client may
/// adjust the user interface accordingly. For example, events
/// from a "finger" source may trigger kinetic scrolling.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaZwpTabletPadRingV2Source(pub u32);

impl MetaZwpTabletPadRingV2Source {
    /// finger
    #[allow(dead_code)]
    pub const FINGER: Self = Self(1);
}

impl Debug for MetaZwpTabletPadRingV2Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::FINGER => "FINGER",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
