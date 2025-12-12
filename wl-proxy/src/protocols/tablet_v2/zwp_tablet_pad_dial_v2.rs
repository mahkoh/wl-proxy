//! pad dial
//!
//! A rotary control, e.g. a dial or a wheel.
//!
//! Events on a dial are logically grouped by the zwp_tablet_pad_dial_v2.frame
//! event.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_tablet_pad_dial_v2 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpTabletPadDialV2 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpTabletPadDialV2Handler>,
}

struct DefaultHandler;

impl ZwpTabletPadDialV2Handler for DefaultHandler { }

impl ZwpTabletPadDialV2 {
    pub const XML_VERSION: u32 = 2;
    pub const INTERFACE: ObjectInterface = ObjectInterface::ZwpTabletPadDialV2;
    pub const INTERFACE_NAME: &str = "zwp_tablet_pad_dial_v2";
}

impl ZwpTabletPadDialV2 {
    pub fn set_handler(&self, handler: impl ZwpTabletPadDialV2Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpTabletPadDialV2Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpTabletPadDialV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpTabletPadDialV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpTabletPadDialV2 {
    /// Since when the set_feedback message is available.
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_tablet_pad_dial_v2#{}.set_feedback(description: {:?}, serial: {})\n", id, arg0, arg1);
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
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

    /// Since when the destroy message is available.
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_tablet_pad_dial_v2#{}.destroy()\n", id);
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
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

    /// Since when the delta message is available.
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_pad_dial_v2#{}.delta(value120: {})\n", client.endpoint.id, id, arg0);
            self.core.state.log(args);
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
            arg0 as u32,
        ]);
        Ok(())
    }

    /// Since when the frame message is available.
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_pad_dial_v2#{}.frame(time: {})\n", client.endpoint.id, id, arg0);
            self.core.state.log(args);
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
            arg0,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpTabletPadDialV2] proxies.
pub trait ZwpTabletPadDialV2Handler: Any {
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
        _slf: &Rc<ZwpTabletPadDialV2>,
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
        _slf: &Rc<ZwpTabletPadDialV2>,
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
        _slf: &Rc<ZwpTabletPadDialV2>,
        value120: i32,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
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
        _slf: &Rc<ZwpTabletPadDialV2>,
        time: u32,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        let res = _slf.send_frame(
            time,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_dial_v2.frame message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for ZwpTabletPadDialV2 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpTabletPadDialV2, version),
            handler: Default::default(),
        })
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
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
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_tablet_pad_dial_v2#{}.set_feedback(description: {:?}, serial: {})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_feedback(&self, arg0, arg1);
                } else {
                    DefaultHandler.set_feedback(&self, arg0, arg1);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_tablet_pad_dial_v2#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
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
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = arg0 as i32;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_pad_dial_v2#{}.delta(value120: {})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).delta(&self, arg0);
                } else {
                    DefaultHandler.delta(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_pad_dial_v2#{}.frame(time: {})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).frame(&self, arg0);
                } else {
                    DefaultHandler.frame(&self, arg0);
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

impl Object for ZwpTabletPadDialV2 {
    fn core(&self) -> &ObjectCore {
        &self.core
    }

    fn unset_handler(&self) {
        self.handler.set(None);
    }

    fn get_handler_any_ref(&self) -> Result<Ref<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(Ref::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<RefMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow_mut().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(RefMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

