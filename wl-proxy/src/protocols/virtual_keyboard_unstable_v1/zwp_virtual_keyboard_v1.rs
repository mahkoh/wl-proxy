//! virtual keyboard
//!
//! The virtual keyboard provides an application with requests which emulate
//! the behaviour of a physical keyboard.
//!
//! This interface can be used by clients on its own to provide raw input
//! events, or it can accompany the input method protocol.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_virtual_keyboard_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpVirtualKeyboardV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpVirtualKeyboardV1Handler>,
}

struct DefaultHandler;

impl ZwpVirtualKeyboardV1Handler for DefaultHandler { }

impl ZwpVirtualKeyboardV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::ZwpVirtualKeyboardV1;
    pub const INTERFACE_NAME: &str = "zwp_virtual_keyboard_v1";
}

impl ZwpVirtualKeyboardV1 {
    pub fn set_handler(&self, handler: impl ZwpVirtualKeyboardV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpVirtualKeyboardV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpVirtualKeyboardV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpVirtualKeyboardV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpVirtualKeyboardV1 {
    /// Since when the keymap message is available.
    pub const MSG__KEYMAP__SINCE: u32 = 1;

    /// keyboard mapping
    ///
    /// Provide a file descriptor to the compositor which can be
    /// memory-mapped to provide a keyboard mapping description.
    ///
    /// Format carries a value from the keymap_format enumeration.
    ///
    /// # Arguments
    ///
    /// - `format`: keymap format
    /// - `fd`: keymap file descriptor
    /// - `size`: keymap size, in bytes
    #[inline]
    pub fn send_keymap(
        &self,
        format: u32,
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_virtual_keyboard_v1#{}.keymap(format: {}, fd: {}, size: {})\n", id, arg0, arg1.as_raw_fd(), arg2);
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.fds.push_back(arg1.clone());
        fmt.words([
            id,
            0,
            arg0,
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
    /// undefined base. All requests regarding a single object must share the
    /// same clock.
    ///
    /// Keymap must be set before issuing this request.
    ///
    /// State carries a value from the key_state enumeration.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `key`: key that produced the event
    /// - `state`: physical state of the key
    #[inline]
    pub fn send_key(
        &self,
        time: u32,
        key: u32,
        state: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            time,
            key,
            state,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_virtual_keyboard_v1#{}.key(time: {}, key: {}, state: {})\n", id, arg0, arg1, arg2);
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
            arg0,
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// Since when the modifiers message is available.
    pub const MSG__MODIFIERS__SINCE: u32 = 1;

    /// modifier and group state
    ///
    /// Notifies the compositor that the modifier and/or group state has
    /// changed, and it should update state.
    ///
    /// The client should use wl_keyboard.modifiers event to synchronize its
    /// internal state with seat state.
    ///
    /// Keymap must be set before issuing this request.
    ///
    /// # Arguments
    ///
    /// - `mods_depressed`: depressed modifiers
    /// - `mods_latched`: latched modifiers
    /// - `mods_locked`: locked modifiers
    /// - `group`: keyboard layout
    #[inline]
    pub fn send_modifiers(
        &self,
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
        ) = (
            mods_depressed,
            mods_latched,
            mods_locked,
            group,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_virtual_keyboard_v1#{}.modifiers(mods_depressed: {}, mods_latched: {}, mods_locked: {}, group: {})\n", id, arg0, arg1, arg2, arg3);
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
            2,
            arg0,
            arg1,
            arg2,
            arg3,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the virtual keyboard keyboard object
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_virtual_keyboard_v1#{}.destroy()\n", id);
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
            3,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [ZwpVirtualKeyboardV1] proxies.
pub trait ZwpVirtualKeyboardV1Handler: Any {
    /// keyboard mapping
    ///
    /// Provide a file descriptor to the compositor which can be
    /// memory-mapped to provide a keyboard mapping description.
    ///
    /// Format carries a value from the keymap_format enumeration.
    ///
    /// # Arguments
    ///
    /// - `format`: keymap format
    /// - `fd`: keymap file descriptor
    /// - `size`: keymap size, in bytes
    #[inline]
    fn keymap(
        &mut self,
        _slf: &Rc<ZwpVirtualKeyboardV1>,
        format: u32,
        fd: &Rc<OwnedFd>,
        size: u32,
    ) {
        let res = _slf.send_keymap(
            format,
            fd,
            size,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_virtual_keyboard_v1.keymap message: {}", Report::new(e));
        }
    }

    /// key event
    ///
    /// A key was pressed or released.
    /// The time argument is a timestamp with millisecond granularity, with an
    /// undefined base. All requests regarding a single object must share the
    /// same clock.
    ///
    /// Keymap must be set before issuing this request.
    ///
    /// State carries a value from the key_state enumeration.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `key`: key that produced the event
    /// - `state`: physical state of the key
    #[inline]
    fn key(
        &mut self,
        _slf: &Rc<ZwpVirtualKeyboardV1>,
        time: u32,
        key: u32,
        state: u32,
    ) {
        let res = _slf.send_key(
            time,
            key,
            state,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_virtual_keyboard_v1.key message: {}", Report::new(e));
        }
    }

    /// modifier and group state
    ///
    /// Notifies the compositor that the modifier and/or group state has
    /// changed, and it should update state.
    ///
    /// The client should use wl_keyboard.modifiers event to synchronize its
    /// internal state with seat state.
    ///
    /// Keymap must be set before issuing this request.
    ///
    /// # Arguments
    ///
    /// - `mods_depressed`: depressed modifiers
    /// - `mods_latched`: latched modifiers
    /// - `mods_locked`: locked modifiers
    /// - `group`: keyboard layout
    #[inline]
    fn modifiers(
        &mut self,
        _slf: &Rc<ZwpVirtualKeyboardV1>,
        mods_depressed: u32,
        mods_latched: u32,
        mods_locked: u32,
        group: u32,
    ) {
        let res = _slf.send_modifiers(
            mods_depressed,
            mods_latched,
            mods_locked,
            group,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_virtual_keyboard_v1.modifiers message: {}", Report::new(e));
        }
    }

    /// destroy the virtual keyboard keyboard object
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<ZwpVirtualKeyboardV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_virtual_keyboard_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for ZwpVirtualKeyboardV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpVirtualKeyboardV1, version),
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
                let [
                    arg0,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let Some(arg1) = fds.pop_front() else {
                    return Err(ObjectError::MissingFd("fd"));
                };
                let arg1 = &arg1;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_virtual_keyboard_v1#{}.keymap(format: {}, fd: {}, size: {})\n", client.endpoint.id, msg[0], arg0, arg1.as_raw_fd(), arg2);
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
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_virtual_keyboard_v1#{}.key(time: {}, key: {}, state: {})\n", client.endpoint.id, msg[0], arg0, arg1, arg2);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).key(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.key(&self, arg0, arg1, arg2);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 24));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_virtual_keyboard_v1#{}.modifiers(mods_depressed: {}, mods_latched: {}, mods_locked: {}, group: {})\n", client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).modifiers(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.modifiers(&self, arg0, arg1, arg2, arg3);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_virtual_keyboard_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
                }
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
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "keymap",
            1 => "key",
            2 => "modifiers",
            3 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for ZwpVirtualKeyboardV1 {
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

impl ZwpVirtualKeyboardV1 {
    /// Since when the error.no_keymap enum variant is available.
    pub const ENM__ERROR_NO_KEYMAP__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwpVirtualKeyboardV1Error(pub u32);

impl ZwpVirtualKeyboardV1Error {
    /// No keymap was set
    pub const NO_KEYMAP: Self = Self(0);
}

impl Debug for ZwpVirtualKeyboardV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NO_KEYMAP => "NO_KEYMAP",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
