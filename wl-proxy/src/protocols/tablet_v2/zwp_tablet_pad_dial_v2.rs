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

impl ConcreteObject for ZwpTabletPadDialV2 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpTabletPadDialV2;
    const INTERFACE_NAME: &str = "zwp_tablet_pad_dial_v2";
}

impl ZwpTabletPadDialV2 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpTabletPadDialV2Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_tablet_pad_dial_v2#{}.set_feedback(description: {:?}, serial: {})\n", id, arg0, arg1);
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
    ) {
        let res = self.try_send_set_feedback(
            description,
            serial,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_pad_dial_v2.set_feedback", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the dial object
    ///
    /// This destroys the client's resource for this dial object.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_tablet_pad_dial_v2#{}.destroy()\n", id);
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

    /// destroy the dial object
    ///
    /// This destroys the client's resource for this dial object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwp_tablet_pad_dial_v2.destroy", &e);
        }
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
    pub fn try_send_delta(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_pad_dial_v2#{}.delta(value120: {})\n", client_id, id, arg0);
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
            arg0 as u32,
        ]);
        Ok(())
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
    pub fn send_delta(
        &self,
        value120: i32,
    ) {
        let res = self.try_send_delta(
            value120,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_pad_dial_v2.delta", &e);
        }
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_pad_dial_v2#{}.frame(time: {})\n", client_id, id, arg0);
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
            arg0,
        ]);
        Ok(())
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
    pub fn send_frame(
        &self,
        time: u32,
    ) {
        let res = self.try_send_frame(
            time,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_pad_dial_v2.frame", &e);
        }
    }
}

/// A message handler for [`ZwpTabletPadDialV2`] proxies.
pub trait ZwpTabletPadDialV2Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpTabletPadDialV2>) {
        slf.core.delete_id();
    }

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
    fn handle_set_feedback(
        &mut self,
        slf: &Rc<ZwpTabletPadDialV2>,
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
            log_forward("zwp_tablet_pad_dial_v2.set_feedback", &e);
        }
    }

    /// destroy the dial object
    ///
    /// This destroys the client's resource for this dial object.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwpTabletPadDialV2>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_pad_dial_v2.destroy", &e);
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
    fn handle_delta(
        &mut self,
        slf: &Rc<ZwpTabletPadDialV2>,
        value120: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_delta(
            value120,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_pad_dial_v2.delta", &e);
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
    fn handle_frame(
        &mut self,
        slf: &Rc<ZwpTabletPadDialV2>,
        time: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_frame(
            time,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_pad_dial_v2.frame", &e);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_tablet_pad_dial_v2#{}.set_feedback(description: {:?}, serial: {})\n", client_id, id, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_tablet_pad_dial_v2#{}.destroy()\n", client_id, id);
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
                let arg0 = arg0 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_pad_dial_v2#{}.delta(value120: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_delta(&self, arg0);
                } else {
                    DefaultHandler.handle_delta(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_pad_dial_v2#{}.frame(time: {})\n", id, arg0);
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

