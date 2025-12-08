//! keyboard grab
//!
//! The zwp_input_method_keyboard_grab_v2 interface represents an exclusive
//! grab of the wl_keyboard interface associated with the seat.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_input_method_keyboard_grab_v2 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpInputMethodKeyboardGrabV2 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpInputMethodKeyboardGrabV2Handler>,
}

struct DefaultHandler;

impl ZwpInputMethodKeyboardGrabV2Handler for DefaultHandler { }

impl ConcreteObject for ZwpInputMethodKeyboardGrabV2 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpInputMethodKeyboardGrabV2;
    const INTERFACE_NAME: &str = "zwp_input_method_keyboard_grab_v2";
}

impl ZwpInputMethodKeyboardGrabV2 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpInputMethodKeyboardGrabV2Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpInputMethodKeyboardGrabV2Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpInputMethodKeyboardGrabV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpInputMethodKeyboardGrabV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpInputMethodKeyboardGrabV2 {
    /// Since when the keymap message is available.
    pub const MSG__KEYMAP__SINCE: u32 = 1;

    /// keyboard mapping
    ///
    /// This event provides a file descriptor to the client which can be
    /// memory-mapped to provide a keyboard mapping description.
    ///
    /// # Arguments
    ///
    /// - `format`: keymap format
    /// - `fd`: keymap file descriptor
    /// - `size`: keymap size, in bytes
    #[inline]
    pub fn try_send_keymap(
        &self,
        format: WlKeyboardKeymapFormat,
        fd: &Rc<OwnedFd>,
        size: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            format,
            fd,
            size,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: WlKeyboardKeymapFormat, arg1: i32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_keyboard_grab_v2#{}.keymap(format: {:?}, fd: {}, size: {})\n", client_id, id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1.as_raw_fd(), arg2);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.fds.push_back(arg1.clone());
        fmt.words([
            id,
            0,
            arg0.0,
            arg2,
        ]);
        Ok(())
    }

    /// keyboard mapping
    ///
    /// This event provides a file descriptor to the client which can be
    /// memory-mapped to provide a keyboard mapping description.
    ///
    /// # Arguments
    ///
    /// - `format`: keymap format
    /// - `fd`: keymap file descriptor
    /// - `size`: keymap size, in bytes
    #[inline]
    pub fn send_keymap(
        &self,
        format: WlKeyboardKeymapFormat,
        fd: &Rc<OwnedFd>,
        size: u32,
    ) {
        let res = self.try_send_keymap(
            format,
            fd,
            size,
        );
        if let Err(e) = res {
            log_send("zwp_input_method_keyboard_grab_v2.keymap", &e);
        }
    }

    /// Since when the key message is available.
    pub const MSG__KEY__SINCE: u32 = 1;

    /// key event
    ///
    /// A key was pressed or released.
    /// The time argument is a timestamp with millisecond granularity, with an
    /// undefined base.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the key event
    /// - `time`: timestamp with millisecond granularity
    /// - `key`: key that produced the event
    /// - `state`: physical state of the key
    #[inline]
    pub fn try_send_key(
        &self,
        serial: u32,
        time: u32,
        key: u32,
        state: WlKeyboardKeyState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            serial,
            time,
            key,
            state,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: WlKeyboardKeyState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_keyboard_grab_v2#{}.key(serial: {}, time: {}, key: {}, state: {:?})\n", client_id, id, arg0, arg1, arg2, arg3);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1, arg2, arg3);
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
            arg1,
            arg2,
            arg3.0,
        ]);
        Ok(())
    }

    /// key event
    ///
    /// A key was pressed or released.
    /// The time argument is a timestamp with millisecond granularity, with an
    /// undefined base.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the key event
    /// - `time`: timestamp with millisecond granularity
    /// - `key`: key that produced the event
    /// - `state`: physical state of the key
    #[inline]
    pub fn send_key(
        &self,
        serial: u32,
        time: u32,
        key: u32,
        state: WlKeyboardKeyState,
    ) {
        let res = self.try_send_key(
            serial,
            time,
            key,
            state,
        );
        if let Err(e) = res {
            log_send("zwp_input_method_keyboard_grab_v2.key", &e);
        }
    }

    /// Since when the modifiers message is available.
    pub const MSG__MODIFIERS__SINCE: u32 = 1;

    /// modifier and group state
    ///
    /// Notifies clients that the modifier and/or group state has changed, and
    /// it should update its local state.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the modifiers event
    /// - `mods_depressed`: depressed modifiers
    /// - `mods_latched`: latched modifiers
    /// - `mods_locked`: locked modifiers
    /// - `group`: keyboard layout
    #[inline]
    pub fn try_send_modifiers(
        &self,
        serial: u32,
        mods_depressed: u32,
        mods_latched: u32,
        mods_locked: u32,
        group: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ) = (
            serial,
            mods_depressed,
            mods_latched,
            mods_locked,
            group,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_keyboard_grab_v2#{}.modifiers(serial: {}, mods_depressed: {}, mods_latched: {}, mods_locked: {}, group: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1, arg2, arg3, arg4);
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
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ]);
        Ok(())
    }

    /// modifier and group state
    ///
    /// Notifies clients that the modifier and/or group state has changed, and
    /// it should update its local state.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the modifiers event
    /// - `mods_depressed`: depressed modifiers
    /// - `mods_latched`: latched modifiers
    /// - `mods_locked`: locked modifiers
    /// - `group`: keyboard layout
    #[inline]
    pub fn send_modifiers(
        &self,
        serial: u32,
        mods_depressed: u32,
        mods_latched: u32,
        mods_locked: u32,
        group: u32,
    ) {
        let res = self.try_send_modifiers(
            serial,
            mods_depressed,
            mods_latched,
            mods_locked,
            group,
        );
        if let Err(e) = res {
            log_send("zwp_input_method_keyboard_grab_v2.modifiers", &e);
        }
    }

    /// Since when the release message is available.
    pub const MSG__RELEASE__SINCE: u32 = 1;

    /// release the grab object
    #[inline]
    pub fn try_send_release(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_keyboard_grab_v2#{}.release()\n", id);
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

    /// release the grab object
    #[inline]
    pub fn send_release(
        &self,
    ) {
        let res = self.try_send_release(
        );
        if let Err(e) = res {
            log_send("zwp_input_method_keyboard_grab_v2.release", &e);
        }
    }

    /// Since when the repeat_info message is available.
    pub const MSG__REPEAT_INFO__SINCE: u32 = 1;

    /// repeat rate and delay
    ///
    /// Informs the client about the keyboard's repeat rate and delay.
    ///
    /// This event is sent as soon as the zwp_input_method_keyboard_grab_v2
    /// object has been created, and is guaranteed to be received by the
    /// client before any key press event.
    ///
    /// Negative values for either rate or delay are illegal. A rate of zero
    /// will disable any repeating (regardless of the value of delay).
    ///
    /// This event can be sent later on as well with a new value if necessary,
    /// so clients should continue listening for the event past the creation
    /// of zwp_input_method_keyboard_grab_v2.
    ///
    /// # Arguments
    ///
    /// - `rate`: the rate of repeating keys in characters per second
    /// - `delay`: delay in milliseconds since key down until repeating starts
    #[inline]
    pub fn try_send_repeat_info(
        &self,
        rate: i32,
        delay: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            rate,
            delay,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_keyboard_grab_v2#{}.repeat_info(rate: {}, delay: {})\n", client_id, id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1);
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
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// repeat rate and delay
    ///
    /// Informs the client about the keyboard's repeat rate and delay.
    ///
    /// This event is sent as soon as the zwp_input_method_keyboard_grab_v2
    /// object has been created, and is guaranteed to be received by the
    /// client before any key press event.
    ///
    /// Negative values for either rate or delay are illegal. A rate of zero
    /// will disable any repeating (regardless of the value of delay).
    ///
    /// This event can be sent later on as well with a new value if necessary,
    /// so clients should continue listening for the event past the creation
    /// of zwp_input_method_keyboard_grab_v2.
    ///
    /// # Arguments
    ///
    /// - `rate`: the rate of repeating keys in characters per second
    /// - `delay`: delay in milliseconds since key down until repeating starts
    #[inline]
    pub fn send_repeat_info(
        &self,
        rate: i32,
        delay: i32,
    ) {
        let res = self.try_send_repeat_info(
            rate,
            delay,
        );
        if let Err(e) = res {
            log_send("zwp_input_method_keyboard_grab_v2.repeat_info", &e);
        }
    }
}

/// A message handler for [`ZwpInputMethodKeyboardGrabV2`] proxies.
pub trait ZwpInputMethodKeyboardGrabV2Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpInputMethodKeyboardGrabV2>) {
        slf.core.delete_id();
    }

    /// keyboard mapping
    ///
    /// This event provides a file descriptor to the client which can be
    /// memory-mapped to provide a keyboard mapping description.
    ///
    /// # Arguments
    ///
    /// - `format`: keymap format
    /// - `fd`: keymap file descriptor
    /// - `size`: keymap size, in bytes
    #[inline]
    fn handle_keymap(
        &mut self,
        slf: &Rc<ZwpInputMethodKeyboardGrabV2>,
        format: WlKeyboardKeymapFormat,
        fd: &Rc<OwnedFd>,
        size: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_keymap(
            format,
            fd,
            size,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_keyboard_grab_v2.keymap", &e);
        }
    }

    /// key event
    ///
    /// A key was pressed or released.
    /// The time argument is a timestamp with millisecond granularity, with an
    /// undefined base.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the key event
    /// - `time`: timestamp with millisecond granularity
    /// - `key`: key that produced the event
    /// - `state`: physical state of the key
    #[inline]
    fn handle_key(
        &mut self,
        slf: &Rc<ZwpInputMethodKeyboardGrabV2>,
        serial: u32,
        time: u32,
        key: u32,
        state: WlKeyboardKeyState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_key(
            serial,
            time,
            key,
            state,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_keyboard_grab_v2.key", &e);
        }
    }

    /// modifier and group state
    ///
    /// Notifies clients that the modifier and/or group state has changed, and
    /// it should update its local state.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the modifiers event
    /// - `mods_depressed`: depressed modifiers
    /// - `mods_latched`: latched modifiers
    /// - `mods_locked`: locked modifiers
    /// - `group`: keyboard layout
    #[inline]
    fn handle_modifiers(
        &mut self,
        slf: &Rc<ZwpInputMethodKeyboardGrabV2>,
        serial: u32,
        mods_depressed: u32,
        mods_latched: u32,
        mods_locked: u32,
        group: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_modifiers(
            serial,
            mods_depressed,
            mods_latched,
            mods_locked,
            group,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_keyboard_grab_v2.modifiers", &e);
        }
    }

    /// release the grab object
    #[inline]
    fn handle_release(
        &mut self,
        slf: &Rc<ZwpInputMethodKeyboardGrabV2>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_release(
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_keyboard_grab_v2.release", &e);
        }
    }

    /// repeat rate and delay
    ///
    /// Informs the client about the keyboard's repeat rate and delay.
    ///
    /// This event is sent as soon as the zwp_input_method_keyboard_grab_v2
    /// object has been created, and is guaranteed to be received by the
    /// client before any key press event.
    ///
    /// Negative values for either rate or delay are illegal. A rate of zero
    /// will disable any repeating (regardless of the value of delay).
    ///
    /// This event can be sent later on as well with a new value if necessary,
    /// so clients should continue listening for the event past the creation
    /// of zwp_input_method_keyboard_grab_v2.
    ///
    /// # Arguments
    ///
    /// - `rate`: the rate of repeating keys in characters per second
    /// - `delay`: delay in milliseconds since key down until repeating starts
    #[inline]
    fn handle_repeat_info(
        &mut self,
        slf: &Rc<ZwpInputMethodKeyboardGrabV2>,
        rate: i32,
        delay: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_repeat_info(
            rate,
            delay,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_keyboard_grab_v2.repeat_info", &e);
        }
    }
}

impl ObjectPrivate for ZwpInputMethodKeyboardGrabV2 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpInputMethodKeyboardGrabV2, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_keyboard_grab_v2#{}.release()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_release(&self);
                } else {
                    DefaultHandler.handle_release(&self);
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
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let Some(arg1) = fds.pop_front() else {
                    return Err(ObjectError(ObjectErrorKind::MissingFd("fd")));
                };
                let arg0 = WlKeyboardKeymapFormat(arg0);
                let arg1 = &arg1;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: WlKeyboardKeymapFormat, arg1: i32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_keyboard_grab_v2#{}.keymap(format: {:?}, fd: {}, size: {})\n", id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1.as_raw_fd(), arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_keymap(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_keymap(&self, arg0, arg1, arg2);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 24)));
                };
                let arg3 = WlKeyboardKeyState(arg3);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: WlKeyboardKeyState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_keyboard_grab_v2#{}.key(serial: {}, time: {}, key: {}, state: {:?})\n", id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_key(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_key(&self, arg0, arg1, arg2, arg3);
                }
            }
            2 => {
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
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_keyboard_grab_v2#{}.modifiers(serial: {}, mods_depressed: {}, mods_latched: {}, mods_locked: {}, group: {})\n", id, arg0, arg1, arg2, arg3, arg4);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2, arg3, arg4);
                }
                if let Some(handler) = handler {
                    (**handler).handle_modifiers(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultHandler.handle_modifiers(&self, arg0, arg1, arg2, arg3, arg4);
                }
            }
            3 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_keyboard_grab_v2#{}.repeat_info(rate: {}, delay: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_repeat_info(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_repeat_info(&self, arg0, arg1);
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
            0 => "release",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "keymap",
            1 => "key",
            2 => "modifiers",
            3 => "repeat_info",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwpInputMethodKeyboardGrabV2 {
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

