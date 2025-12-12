//! keyboard grab
//!
//! The zwp_input_method_keyboard_grab_v2 interface represents an exclusive
//! grab of the wl_keyboard interface associated with the seat.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_input_method_keyboard_grab_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpInputMethodKeyboardGrabV2 {
    core: ProxyCore,
    handler: HandlerHolder<dyn ZwpInputMethodKeyboardGrabV2Handler>,
}

struct DefaultHandler;

impl ZwpInputMethodKeyboardGrabV2Handler for DefaultHandler { }

impl ZwpInputMethodKeyboardGrabV2 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ProxyInterface = ProxyInterface::ZwpInputMethodKeyboardGrabV2;
    pub const INTERFACE_NAME: &str = "zwp_input_method_keyboard_grab_v2";
}

impl ZwpInputMethodKeyboardGrabV2 {
    pub fn set_handler(&self, handler: impl ZwpInputMethodKeyboardGrabV2Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

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
    pub fn send_keymap(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_keyboard_grab_v2#{}.keymap(format: {:?}, fd: {}, size: {})\n", client.endpoint.id, id, arg0, arg1.as_raw_fd(), arg2);
            self.core.state.log(args);
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
    pub fn send_key(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_keyboard_grab_v2#{}.key(serial: {}, time: {}, key: {}, state: {:?})\n", client.endpoint.id, id, arg0, arg1, arg2, arg3);
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
            arg1,
            arg2,
            arg3.0,
        ]);
        Ok(())
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
    pub fn send_modifiers(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_keyboard_grab_v2#{}.modifiers(serial: {}, mods_depressed: {}, mods_latched: {}, mods_locked: {}, group: {})\n", client.endpoint.id, id, arg0, arg1, arg2, arg3, arg4);
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
            2,
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ]);
        Ok(())
    }

    /// Since when the release message is available.
    pub const MSG__RELEASE__SINCE: u32 = 1;

    /// release the grab object
    #[inline]
    pub fn send_release(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_keyboard_grab_v2#{}.release()\n", id);
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
        self.core.handle_server_destroy();
        Ok(())
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
    pub fn send_repeat_info(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_keyboard_grab_v2#{}.repeat_info(rate: {}, delay: {})\n", client.endpoint.id, id, arg0, arg1);
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
            3,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpInputMethodKeyboardGrabV2] proxies.
pub trait ZwpInputMethodKeyboardGrabV2Handler: Any {
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
    fn keymap(
        &mut self,
        _slf: &Rc<ZwpInputMethodKeyboardGrabV2>,
        format: WlKeyboardKeymapFormat,
        fd: &Rc<OwnedFd>,
        size: u32,
    ) {
        let res = _slf.send_keymap(
            format,
            fd,
            size,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_keyboard_grab_v2.keymap message: {}", Report::new(e));
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
    fn key(
        &mut self,
        _slf: &Rc<ZwpInputMethodKeyboardGrabV2>,
        serial: u32,
        time: u32,
        key: u32,
        state: WlKeyboardKeyState,
    ) {
        let res = _slf.send_key(
            serial,
            time,
            key,
            state,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_keyboard_grab_v2.key message: {}", Report::new(e));
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
    fn modifiers(
        &mut self,
        _slf: &Rc<ZwpInputMethodKeyboardGrabV2>,
        serial: u32,
        mods_depressed: u32,
        mods_latched: u32,
        mods_locked: u32,
        group: u32,
    ) {
        let res = _slf.send_modifiers(
            serial,
            mods_depressed,
            mods_latched,
            mods_locked,
            group,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_keyboard_grab_v2.modifiers message: {}", Report::new(e));
        }
    }

    /// release the grab object
    #[inline]
    fn release(
        &mut self,
        _slf: &Rc<ZwpInputMethodKeyboardGrabV2>,
    ) {
        let res = _slf.send_release(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_keyboard_grab_v2.release message: {}", Report::new(e));
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
    fn repeat_info(
        &mut self,
        _slf: &Rc<ZwpInputMethodKeyboardGrabV2>,
        rate: i32,
        delay: i32,
    ) {
        let res = _slf.send_repeat_info(
            rate,
            delay,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_keyboard_grab_v2.repeat_info message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for ZwpInputMethodKeyboardGrabV2 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::ZwpInputMethodKeyboardGrabV2, version),
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
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_keyboard_grab_v2#{}.release()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).release(&self);
                } else {
                    DefaultHandler.release(&self);
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
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let Some(arg1) = fds.pop_front() else {
                    return Err(ObjectError::MissingFd("fd"));
                };
                let arg0 = WlKeyboardKeymapFormat(arg0);
                let arg1 = &arg1;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_keyboard_grab_v2#{}.keymap(format: {:?}, fd: {}, size: {})\n", msg[0], arg0, arg1.as_raw_fd(), arg2);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).keymap(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.keymap(&self, arg0, arg1, arg2);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 24));
                };
                let arg3 = WlKeyboardKeyState(arg3);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_keyboard_grab_v2#{}.key(serial: {}, time: {}, key: {}, state: {:?})\n", msg[0], arg0, arg1, arg2, arg3);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).key(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.key(&self, arg0, arg1, arg2, arg3);
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
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 28));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_keyboard_grab_v2#{}.modifiers(serial: {}, mods_depressed: {}, mods_latched: {}, mods_locked: {}, group: {})\n", msg[0], arg0, arg1, arg2, arg3, arg4);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).modifiers(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultHandler.modifiers(&self, arg0, arg1, arg2, arg3, arg4);
                }
            }
            3 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_keyboard_grab_v2#{}.repeat_info(rate: {}, delay: {})\n", msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).repeat_info(&self, arg0, arg1);
                } else {
                    DefaultHandler.repeat_info(&self, arg0, arg1);
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

impl Proxy for ZwpInputMethodKeyboardGrabV2 {
    fn core(&self) -> &ProxyCore {
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

