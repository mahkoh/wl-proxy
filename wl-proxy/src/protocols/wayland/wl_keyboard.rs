//! keyboard input device
//!
//! The wl_keyboard interface represents one or more keyboards
//! associated with a seat.
//!
//! Each wl_keyboard has the following logical state:
//!
//! - an active surface (possibly null),
//! - the keys currently logically down,
//! - the active modifiers,
//! - the active group.
//!
//! By default, the active surface is null, the keys currently logically down
//! are empty, the active modifiers and the active group are 0.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wl_keyboard object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlKeyboard {
    core: ObjectCore,
    handler: HandlerHolder<dyn WlKeyboardHandler>,
}

struct DefaultHandler;

impl WlKeyboardHandler for DefaultHandler { }

impl ConcreteObject for WlKeyboard {
    const XML_VERSION: u32 = 10;
    const INTERFACE: ObjectInterface = ObjectInterface::WlKeyboard;
    const INTERFACE_NAME: &str = "wl_keyboard";
}

impl WlKeyboard {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WlKeyboardHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WlKeyboardHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlKeyboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlKeyboard")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlKeyboard {
    /// Since when the keymap message is available.
    pub const MSG__KEYMAP__SINCE: u32 = 1;

    /// keyboard mapping
    ///
    /// This event provides a file descriptor to the client which can be
    /// memory-mapped in read-only mode to provide a keyboard mapping
    /// description.
    ///
    /// From version 7 onwards, the fd must be mapped with MAP_PRIVATE by
    /// the recipient, as MAP_SHARED may fail.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_keyboard#{}.keymap(format: {:?}, fd: {}, size: {})\n", client_id, id, arg0, arg1, arg2);
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
    /// memory-mapped in read-only mode to provide a keyboard mapping
    /// description.
    ///
    /// From version 7 onwards, the fd must be mapped with MAP_PRIVATE by
    /// the recipient, as MAP_SHARED may fail.
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
            log_send("wl_keyboard.keymap", &e);
        }
    }

    /// Since when the enter message is available.
    pub const MSG__ENTER__SINCE: u32 = 1;

    /// enter event
    ///
    /// Notification that this seat's keyboard focus is on a certain
    /// surface.
    ///
    /// The compositor must send the wl_keyboard.modifiers event after this
    /// event.
    ///
    /// In the wl_keyboard logical state, this event sets the active surface to
    /// the surface argument and the keys currently logically down to the keys
    /// in the keys argument. The compositor must not send this event if the
    /// wl_keyboard already had an active surface immediately before this event.
    ///
    /// Clients should not use the list of pressed keys to emulate key-press
    /// events. The order of keys in the list is unspecified.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the enter event
    /// - `surface`: surface gaining keyboard focus
    /// - `keys`: the keys currently logically down
    #[inline]
    pub fn try_send_enter(
        &self,
        serial: u32,
        surface: &Rc<WlSurface>,
        keys: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            serial,
            surface,
            keys,
        );
        let arg1 = arg1.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg1.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("surface", client.endpoint.id)));
        }
        let arg1_id = arg1.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: &[u8]) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_keyboard#{}.enter(serial: {}, surface: wl_surface#{}, keys: {})\n", client_id, id, arg0, arg1, debug_array(arg2));
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1_id, arg2);
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
            arg1_id,
        ]);
        fmt.array(arg2);
        Ok(())
    }

    /// enter event
    ///
    /// Notification that this seat's keyboard focus is on a certain
    /// surface.
    ///
    /// The compositor must send the wl_keyboard.modifiers event after this
    /// event.
    ///
    /// In the wl_keyboard logical state, this event sets the active surface to
    /// the surface argument and the keys currently logically down to the keys
    /// in the keys argument. The compositor must not send this event if the
    /// wl_keyboard already had an active surface immediately before this event.
    ///
    /// Clients should not use the list of pressed keys to emulate key-press
    /// events. The order of keys in the list is unspecified.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the enter event
    /// - `surface`: surface gaining keyboard focus
    /// - `keys`: the keys currently logically down
    #[inline]
    pub fn send_enter(
        &self,
        serial: u32,
        surface: &Rc<WlSurface>,
        keys: &[u8],
    ) {
        let res = self.try_send_enter(
            serial,
            surface,
            keys,
        );
        if let Err(e) = res {
            log_send("wl_keyboard.enter", &e);
        }
    }

    /// Since when the leave message is available.
    pub const MSG__LEAVE__SINCE: u32 = 1;

    /// leave event
    ///
    /// Notification that this seat's keyboard focus is no longer on
    /// a certain surface.
    ///
    /// The leave notification is sent before the enter notification
    /// for the new focus.
    ///
    /// In the wl_keyboard logical state, this event resets all values to their
    /// defaults. The compositor must not send this event if the active surface
    /// of the wl_keyboard was not equal to the surface argument immediately
    /// before this event.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the leave event
    /// - `surface`: surface that lost keyboard focus
    #[inline]
    pub fn try_send_leave(
        &self,
        serial: u32,
        surface: &Rc<WlSurface>,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg1.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("surface", client.endpoint.id)));
        }
        let arg1_id = arg1.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_keyboard#{}.leave(serial: {}, surface: wl_surface#{})\n", client_id, id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1_id);
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
            arg1_id,
        ]);
        Ok(())
    }

    /// leave event
    ///
    /// Notification that this seat's keyboard focus is no longer on
    /// a certain surface.
    ///
    /// The leave notification is sent before the enter notification
    /// for the new focus.
    ///
    /// In the wl_keyboard logical state, this event resets all values to their
    /// defaults. The compositor must not send this event if the active surface
    /// of the wl_keyboard was not equal to the surface argument immediately
    /// before this event.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the leave event
    /// - `surface`: surface that lost keyboard focus
    #[inline]
    pub fn send_leave(
        &self,
        serial: u32,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_leave(
            serial,
            surface,
        );
        if let Err(e) = res {
            log_send("wl_keyboard.leave", &e);
        }
    }

    /// Since when the key message is available.
    pub const MSG__KEY__SINCE: u32 = 1;

    /// key event
    ///
    /// A key was pressed or released.
    /// The time argument is a timestamp with millisecond
    /// granularity, with an undefined base.
    ///
    /// The key is a platform-specific key code that can be interpreted
    /// by feeding it to the keyboard mapping (see the keymap event).
    ///
    /// If this event produces a change in modifiers, then the resulting
    /// wl_keyboard.modifiers event must be sent after this event.
    ///
    /// In the wl_keyboard logical state, this event adds the key to the keys
    /// currently logically down (if the state argument is pressed) or removes
    /// the key from the keys currently logically down (if the state argument is
    /// released). The compositor must not send this event if the wl_keyboard
    /// did not have an active surface immediately before this event. The
    /// compositor must not send this event if state is pressed (resp. released)
    /// and the key was already logically down (resp. was not logically down)
    /// immediately before this event.
    ///
    /// Since version 10, compositors may send key events with the "repeated"
    /// key state when a wl_keyboard.repeat_info event with a rate argument of
    /// 0 has been received. This allows the compositor to take over the
    /// responsibility of key repetition.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_keyboard#{}.key(serial: {}, time: {}, key: {}, state: {:?})\n", client_id, id, arg0, arg1, arg2, arg3);
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
            3,
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
    /// The time argument is a timestamp with millisecond
    /// granularity, with an undefined base.
    ///
    /// The key is a platform-specific key code that can be interpreted
    /// by feeding it to the keyboard mapping (see the keymap event).
    ///
    /// If this event produces a change in modifiers, then the resulting
    /// wl_keyboard.modifiers event must be sent after this event.
    ///
    /// In the wl_keyboard logical state, this event adds the key to the keys
    /// currently logically down (if the state argument is pressed) or removes
    /// the key from the keys currently logically down (if the state argument is
    /// released). The compositor must not send this event if the wl_keyboard
    /// did not have an active surface immediately before this event. The
    /// compositor must not send this event if state is pressed (resp. released)
    /// and the key was already logically down (resp. was not logically down)
    /// immediately before this event.
    ///
    /// Since version 10, compositors may send key events with the "repeated"
    /// key state when a wl_keyboard.repeat_info event with a rate argument of
    /// 0 has been received. This allows the compositor to take over the
    /// responsibility of key repetition.
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
            log_send("wl_keyboard.key", &e);
        }
    }

    /// Since when the modifiers message is available.
    pub const MSG__MODIFIERS__SINCE: u32 = 1;

    /// modifier and group state
    ///
    /// Notifies clients that the modifier and/or group state has
    /// changed, and it should update its local state.
    ///
    /// The compositor may send this event without a surface of the client
    /// having keyboard focus, for example to tie modifier information to
    /// pointer focus instead. If a modifier event with pressed modifiers is sent
    /// without a prior enter event, the client can assume the modifier state is
    /// valid until it receives the next wl_keyboard.modifiers event. In order to
    /// reset the modifier state again, the compositor can send a
    /// wl_keyboard.modifiers event with no pressed modifiers.
    ///
    /// In the wl_keyboard logical state, this event updates the modifiers and
    /// group.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_keyboard#{}.modifiers(serial: {}, mods_depressed: {}, mods_latched: {}, mods_locked: {}, group: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4);
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
            4,
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
    /// Notifies clients that the modifier and/or group state has
    /// changed, and it should update its local state.
    ///
    /// The compositor may send this event without a surface of the client
    /// having keyboard focus, for example to tie modifier information to
    /// pointer focus instead. If a modifier event with pressed modifiers is sent
    /// without a prior enter event, the client can assume the modifier state is
    /// valid until it receives the next wl_keyboard.modifiers event. In order to
    /// reset the modifier state again, the compositor can send a
    /// wl_keyboard.modifiers event with no pressed modifiers.
    ///
    /// In the wl_keyboard logical state, this event updates the modifiers and
    /// group.
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
            log_send("wl_keyboard.modifiers", &e);
        }
    }

    /// Since when the release message is available.
    pub const MSG__RELEASE__SINCE: u32 = 3;

    /// release the keyboard object
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_keyboard#{}.release()\n", id);
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

    /// release the keyboard object
    #[inline]
    pub fn send_release(
        &self,
    ) {
        let res = self.try_send_release(
        );
        if let Err(e) = res {
            log_send("wl_keyboard.release", &e);
        }
    }

    /// Since when the repeat_info message is available.
    pub const MSG__REPEAT_INFO__SINCE: u32 = 4;

    /// repeat rate and delay
    ///
    /// Informs the client about the keyboard's repeat rate and delay.
    ///
    /// This event is sent as soon as the wl_keyboard object has been created,
    /// and is guaranteed to be received by the client before any key press
    /// event.
    ///
    /// Negative values for either rate or delay are illegal. A rate of zero
    /// will disable any repeating (regardless of the value of delay).
    ///
    /// This event can be sent later on as well with a new value if necessary,
    /// so clients should continue listening for the event past the creation
    /// of wl_keyboard.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_keyboard#{}.repeat_info(rate: {}, delay: {})\n", client_id, id, arg0, arg1);
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
            5,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// repeat rate and delay
    ///
    /// Informs the client about the keyboard's repeat rate and delay.
    ///
    /// This event is sent as soon as the wl_keyboard object has been created,
    /// and is guaranteed to be received by the client before any key press
    /// event.
    ///
    /// Negative values for either rate or delay are illegal. A rate of zero
    /// will disable any repeating (regardless of the value of delay).
    ///
    /// This event can be sent later on as well with a new value if necessary,
    /// so clients should continue listening for the event past the creation
    /// of wl_keyboard.
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
            log_send("wl_keyboard.repeat_info", &e);
        }
    }
}

/// A message handler for [`WlKeyboard`] proxies.
pub trait WlKeyboardHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WlKeyboard>) {
        slf.core.delete_id();
    }

    /// keyboard mapping
    ///
    /// This event provides a file descriptor to the client which can be
    /// memory-mapped in read-only mode to provide a keyboard mapping
    /// description.
    ///
    /// From version 7 onwards, the fd must be mapped with MAP_PRIVATE by
    /// the recipient, as MAP_SHARED may fail.
    ///
    /// # Arguments
    ///
    /// - `format`: keymap format
    /// - `fd`: keymap file descriptor
    /// - `size`: keymap size, in bytes
    #[inline]
    fn handle_keymap(
        &mut self,
        slf: &Rc<WlKeyboard>,
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
            log_forward("wl_keyboard.keymap", &e);
        }
    }

    /// enter event
    ///
    /// Notification that this seat's keyboard focus is on a certain
    /// surface.
    ///
    /// The compositor must send the wl_keyboard.modifiers event after this
    /// event.
    ///
    /// In the wl_keyboard logical state, this event sets the active surface to
    /// the surface argument and the keys currently logically down to the keys
    /// in the keys argument. The compositor must not send this event if the
    /// wl_keyboard already had an active surface immediately before this event.
    ///
    /// Clients should not use the list of pressed keys to emulate key-press
    /// events. The order of keys in the list is unspecified.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the enter event
    /// - `surface`: surface gaining keyboard focus
    /// - `keys`: the keys currently logically down
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_enter(
        &mut self,
        slf: &Rc<WlKeyboard>,
        serial: u32,
        surface: &Rc<WlSurface>,
        keys: &[u8],
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = surface.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = slf.try_send_enter(
            serial,
            surface,
            keys,
        );
        if let Err(e) = res {
            log_forward("wl_keyboard.enter", &e);
        }
    }

    /// leave event
    ///
    /// Notification that this seat's keyboard focus is no longer on
    /// a certain surface.
    ///
    /// The leave notification is sent before the enter notification
    /// for the new focus.
    ///
    /// In the wl_keyboard logical state, this event resets all values to their
    /// defaults. The compositor must not send this event if the active surface
    /// of the wl_keyboard was not equal to the surface argument immediately
    /// before this event.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the leave event
    /// - `surface`: surface that lost keyboard focus
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_leave(
        &mut self,
        slf: &Rc<WlKeyboard>,
        serial: u32,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = surface.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = slf.try_send_leave(
            serial,
            surface,
        );
        if let Err(e) = res {
            log_forward("wl_keyboard.leave", &e);
        }
    }

    /// key event
    ///
    /// A key was pressed or released.
    /// The time argument is a timestamp with millisecond
    /// granularity, with an undefined base.
    ///
    /// The key is a platform-specific key code that can be interpreted
    /// by feeding it to the keyboard mapping (see the keymap event).
    ///
    /// If this event produces a change in modifiers, then the resulting
    /// wl_keyboard.modifiers event must be sent after this event.
    ///
    /// In the wl_keyboard logical state, this event adds the key to the keys
    /// currently logically down (if the state argument is pressed) or removes
    /// the key from the keys currently logically down (if the state argument is
    /// released). The compositor must not send this event if the wl_keyboard
    /// did not have an active surface immediately before this event. The
    /// compositor must not send this event if state is pressed (resp. released)
    /// and the key was already logically down (resp. was not logically down)
    /// immediately before this event.
    ///
    /// Since version 10, compositors may send key events with the "repeated"
    /// key state when a wl_keyboard.repeat_info event with a rate argument of
    /// 0 has been received. This allows the compositor to take over the
    /// responsibility of key repetition.
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
        slf: &Rc<WlKeyboard>,
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
            log_forward("wl_keyboard.key", &e);
        }
    }

    /// modifier and group state
    ///
    /// Notifies clients that the modifier and/or group state has
    /// changed, and it should update its local state.
    ///
    /// The compositor may send this event without a surface of the client
    /// having keyboard focus, for example to tie modifier information to
    /// pointer focus instead. If a modifier event with pressed modifiers is sent
    /// without a prior enter event, the client can assume the modifier state is
    /// valid until it receives the next wl_keyboard.modifiers event. In order to
    /// reset the modifier state again, the compositor can send a
    /// wl_keyboard.modifiers event with no pressed modifiers.
    ///
    /// In the wl_keyboard logical state, this event updates the modifiers and
    /// group.
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
        slf: &Rc<WlKeyboard>,
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
            log_forward("wl_keyboard.modifiers", &e);
        }
    }

    /// release the keyboard object
    #[inline]
    fn handle_release(
        &mut self,
        slf: &Rc<WlKeyboard>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_release(
        );
        if let Err(e) = res {
            log_forward("wl_keyboard.release", &e);
        }
    }

    /// repeat rate and delay
    ///
    /// Informs the client about the keyboard's repeat rate and delay.
    ///
    /// This event is sent as soon as the wl_keyboard object has been created,
    /// and is guaranteed to be received by the client before any key press
    /// event.
    ///
    /// Negative values for either rate or delay are illegal. A rate of zero
    /// will disable any repeating (regardless of the value of delay).
    ///
    /// This event can be sent later on as well with a new value if necessary,
    /// so clients should continue listening for the event past the creation
    /// of wl_keyboard.
    ///
    /// # Arguments
    ///
    /// - `rate`: the rate of repeating keys in characters per second
    /// - `delay`: delay in milliseconds since key down until repeating starts
    #[inline]
    fn handle_repeat_info(
        &mut self,
        slf: &Rc<WlKeyboard>,
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
            log_forward("wl_keyboard.repeat_info", &e);
        }
    }
}

impl ObjectPrivate for WlKeyboard {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WlKeyboard, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_keyboard#{}.release()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_keyboard#{}.keymap(format: {:?}, fd: {}, size: {})\n", id, arg0, arg1, arg2);
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
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("serial")));
                };
                offset += 1;
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("surface")));
                };
                offset += 1;
                let arg2;
                (arg2, offset) = parse_array(msg, offset, "keys")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_keyboard#{}.enter(serial: {}, surface: wl_surface#{}, keys: {})\n", id, arg0, arg1, debug_array(arg2));
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2);
                }
                let arg1_id = arg1;
                let Some(arg1) = server.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = server.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_enter(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_enter(&self, arg0, arg1, arg2);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_keyboard#{}.leave(serial: {}, surface: wl_surface#{})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                let arg1_id = arg1;
                let Some(arg1) = server.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = server.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_leave(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_leave(&self, arg0, arg1);
                }
            }
            3 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_keyboard#{}.key(serial: {}, time: {}, key: {}, state: {:?})\n", id, arg0, arg1, arg2, arg3);
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
            4 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_keyboard#{}.modifiers(serial: {}, mods_depressed: {}, mods_latched: {}, mods_locked: {}, group: {})\n", id, arg0, arg1, arg2, arg3, arg4);
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
            5 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_keyboard#{}.repeat_info(rate: {}, delay: {})\n", id, arg0, arg1);
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
            1 => "enter",
            2 => "leave",
            3 => "key",
            4 => "modifiers",
            5 => "repeat_info",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WlKeyboard {
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

impl WlKeyboard {
    /// Since when the keymap_format.no_keymap enum variant is available.
    pub const ENM__KEYMAP_FORMAT_NO_KEYMAP__SINCE: u32 = 1;
    /// Since when the keymap_format.xkb_v1 enum variant is available.
    pub const ENM__KEYMAP_FORMAT_XKB_V1__SINCE: u32 = 1;

    /// Since when the key_state.released enum variant is available.
    pub const ENM__KEY_STATE_RELEASED__SINCE: u32 = 1;
    /// Since when the key_state.pressed enum variant is available.
    pub const ENM__KEY_STATE_PRESSED__SINCE: u32 = 1;
    /// Since when the key_state.repeated enum variant is available.
    pub const ENM__KEY_STATE_REPEATED__SINCE: u32 = 10;
}

/// keyboard mapping format
///
/// This specifies the format of the keymap provided to the
/// client with the wl_keyboard.keymap event.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WlKeyboardKeymapFormat(pub u32);

impl WlKeyboardKeymapFormat {
    /// no keymap; client must understand how to interpret the raw keycode
    pub const NO_KEYMAP: Self = Self(0);

    /// libxkbcommon compatible, null-terminated string; to determine the xkb keycode, clients must add 8 to the key event keycode
    pub const XKB_V1: Self = Self(1);
}

impl Debug for WlKeyboardKeymapFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NO_KEYMAP => "NO_KEYMAP",
            Self::XKB_V1 => "XKB_V1",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// physical key state
///
/// Describes the physical state of a key that produced the key event.
///
/// Since version 10, the key can be in a "repeated" pseudo-state which
/// means the same as "pressed", but is used to signal repetition in the
/// key event.
///
/// The key may only enter the repeated state after entering the pressed
/// state and before entering the released state. This event may be
/// generated multiple times while the key is down.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WlKeyboardKeyState(pub u32);

impl WlKeyboardKeyState {
    /// key is not pressed
    pub const RELEASED: Self = Self(0);

    /// key is pressed
    pub const PRESSED: Self = Self(1);

    /// key was repeated
    pub const REPEATED: Self = Self(2);
}

impl Debug for WlKeyboardKeyState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::RELEASED => "RELEASED",
            Self::PRESSED => "PRESSED",
            Self::REPEATED => "REPEATED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
