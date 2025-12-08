//! pad dial
//!
//! A rotary control, e.g. a dial or a wheel.
//!
//! Events on a dial are logically grouped by the zwp_tablet_pad_dial_v2.frame
//! event.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_tablet_pad_dial_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpTabletPadDialV2 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpTabletPadDialV2MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpTabletPadDialV2MessageHandler for DefaultMessageHandler { }

impl MetaZwpTabletPadDialV2 {
    pub const XML_VERSION: u32 = 2;
}

impl MetaZwpTabletPadDialV2 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpTabletPadDialV2, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwpTabletPadDialV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpTabletPadDialV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpTabletPadDialV2 {
    /// Since when the set_feedback message is available.
    #[allow(dead_code)]
    pub const MSG__SET_FEEDBACK__SINCE: u32 = 1;

    /// set compositor feedback
    ///
    /// Requests the compositor to use the provided feedback string
    /// associated with this dial. This request should be issued immediately
    /// after a zwp_tablet_pad_group_v2.mode_switch event from the corresponding
    /// group is received, or whenever the dial is mapped to a different
    /// action. See zwp_tablet_pad_group_v2.mode_switch for more details.
    ///
    /// Clients are encouraged to provide context-aware descriptions for
    /// the actions associated with the dial, and compositors may use this
    /// information to offer visual feedback about the button layout
    /// (eg. on-screen displays).
    ///
    /// The provided string 'description' is a UTF-8 encoded string to be
    /// associated with this ring, and is considered user-visible; general
    /// internationalization rules apply.
    ///
    /// The serial argument will be that of the last
    /// zwp_tablet_pad_group_v2.mode_switch event received for the group of this
    /// dial. Requests providing other serials than the most recent one will be
    /// ignored.
    ///
    /// # Arguments
    ///
    /// - `description`: dial description
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= zwp_tablet_pad_dial_v2#{}.set_feedback(description: {:?}, serial: {})", id, arg0, arg1);
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

    /// destroy the dial object
    ///
    /// This destroys the client's resource for this dial object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= zwp_tablet_pad_dial_v2#{}.destroy()", id);
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

    /// Since when the delta message is available.
    #[allow(dead_code)]
    pub const MSG__DELTA__SINCE: u32 = 1;

    /// delta movement
    ///
    /// Sent whenever the position on a dial changes.
    ///
    /// This event carries the wheel delta as multiples or fractions
    /// of 120 with each multiple of 120 representing one logical wheel detent.
    /// For example, an axis_value120 of 30 is one quarter of
    /// a logical wheel step in the positive direction, a value120 of
    /// -240 are two logical wheel steps in the negative direction within the
    /// same hardware event. See the wl_pointer.axis_value120 for more details.
    ///
    /// The value120 must not be zero.
    ///
    /// # Arguments
    ///
    /// - `value120`: rotation distance as fraction of 120
    #[inline]
    pub fn send_delta(
        &self,
        value120: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            value120,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= zwp_tablet_pad_dial_v2#{}.delta(value120: {})", client.endpoint.id, id, arg0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0 as u32,
        ]);
        Ok(())
    }

    /// Since when the frame message is available.
    #[allow(dead_code)]
    pub const MSG__FRAME__SINCE: u32 = 1;

    /// end of a dial event sequence
    ///
    /// Indicates the end of a set of events that represent one logical
    /// hardware dial event. A client is expected to accumulate the data
    /// in all events within the frame before proceeding.
    ///
    /// All zwp_tablet_pad_dial_v2 events before a zwp_tablet_pad_dial_v2.frame event belong
    /// logically together.
    ///
    /// A zwp_tablet_pad_dial_v2.frame event is sent for every logical event
    /// group, even if the group only contains a single zwp_tablet_pad_dial_v2
    /// event. Specifically, a client may get a sequence: delta, frame,
    /// delta, frame, etc.
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= zwp_tablet_pad_dial_v2#{}.frame(time: {})", client.endpoint.id, id, arg0);
        let endpoint = &client.endpoint;
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
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpTabletPadDialV2] proxies.
#[allow(dead_code)]
pub trait MetaZwpTabletPadDialV2MessageHandler {
    /// set compositor feedback
    ///
    /// Requests the compositor to use the provided feedback string
    /// associated with this dial. This request should be issued immediately
    /// after a zwp_tablet_pad_group_v2.mode_switch event from the corresponding
    /// group is received, or whenever the dial is mapped to a different
    /// action. See zwp_tablet_pad_group_v2.mode_switch for more details.
    ///
    /// Clients are encouraged to provide context-aware descriptions for
    /// the actions associated with the dial, and compositors may use this
    /// information to offer visual feedback about the button layout
    /// (eg. on-screen displays).
    ///
    /// The provided string 'description' is a UTF-8 encoded string to be
    /// associated with this ring, and is considered user-visible; general
    /// internationalization rules apply.
    ///
    /// The serial argument will be that of the last
    /// zwp_tablet_pad_group_v2.mode_switch event received for the group of this
    /// dial. Requests providing other serials than the most recent one will be
    /// ignored.
    ///
    /// # Arguments
    ///
    /// - `description`: dial description
    /// - `serial`: serial of the mode switch event
    #[inline]
    fn set_feedback(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadDialV2>,
        description: &str,
        serial: u32,
    ) {
        let res = _slf.send_set_feedback(
            description,
            serial,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_dial_v2.set_feedback message: {}", Report::new(e));
        }
    }

    /// destroy the dial object
    ///
    /// This destroys the client's resource for this dial object.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadDialV2>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_dial_v2.destroy message: {}", Report::new(e));
        }
    }

    /// delta movement
    ///
    /// Sent whenever the position on a dial changes.
    ///
    /// This event carries the wheel delta as multiples or fractions
    /// of 120 with each multiple of 120 representing one logical wheel detent.
    /// For example, an axis_value120 of 30 is one quarter of
    /// a logical wheel step in the positive direction, a value120 of
    /// -240 are two logical wheel steps in the negative direction within the
    /// same hardware event. See the wl_pointer.axis_value120 for more details.
    ///
    /// The value120 must not be zero.
    ///
    /// # Arguments
    ///
    /// - `value120`: rotation distance as fraction of 120
    #[inline]
    fn delta(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadDialV2>,
        value120: i32,
    ) {
        let res = _slf.send_delta(
            value120,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_dial_v2.delta message: {}", Report::new(e));
        }
    }

    /// end of a dial event sequence
    ///
    /// Indicates the end of a set of events that represent one logical
    /// hardware dial event. A client is expected to accumulate the data
    /// in all events within the frame before proceeding.
    ///
    /// All zwp_tablet_pad_dial_v2 events before a zwp_tablet_pad_dial_v2.frame event belong
    /// logically together.
    ///
    /// A zwp_tablet_pad_dial_v2.frame event is sent for every logical event
    /// group, even if the group only contains a single zwp_tablet_pad_dial_v2
    /// event. Specifically, a client may get a sequence: delta, frame,
    /// delta, frame, etc.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    #[inline]
    fn frame(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadDialV2>,
        time: u32,
    ) {
        let res = _slf.send_frame(
            time,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_dial_v2.frame message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpTabletPadDialV2 {
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
                        return Err(ObjectError::MissingArgument("description"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("description"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("description"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("description"));
                        };
                        s
                    }
                };
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("serial"));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                eprintln!("client#{:04} -> zwp_tablet_pad_dial_v2#{}.set_feedback(description: {:?}, serial: {})", client.endpoint.id, msg[0], arg0, arg1);
                if let Some(handler) = handler {
                    (**handler).set_feedback(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.set_feedback(&self, arg0, arg1);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("client#{:04} -> zwp_tablet_pad_dial_v2#{}.destroy()", client.endpoint.id, msg[0]);
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
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = arg0 as i32;
                eprintln!("server      -> zwp_tablet_pad_dial_v2#{}.delta(value120: {})", msg[0], arg0);
                if let Some(handler) = handler {
                    (**handler).delta(&self, arg0);
                } else {
                    DefaultMessageHandler.delta(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                eprintln!("server      -> zwp_tablet_pad_dial_v2#{}.frame(time: {})", msg[0], arg0);
                if let Some(handler) = handler {
                    (**handler).frame(&self, arg0);
                } else {
                    DefaultMessageHandler.frame(&self, arg0);
                }
            }
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
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
            0 => "delta",
            1 => "frame",
            _ => return None,
        };
        Some(name)
    }
}

