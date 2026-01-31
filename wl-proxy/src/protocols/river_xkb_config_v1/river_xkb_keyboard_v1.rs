//! xkbcommon keyboard device
//!
//! This object represent a physical keyboard which has its configuration and
//! state managed by xkbcommon.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A river_xkb_keyboard_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct RiverXkbKeyboardV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn RiverXkbKeyboardV1Handler>,
}

struct DefaultHandler;

impl RiverXkbKeyboardV1Handler for DefaultHandler { }

impl ConcreteObject for RiverXkbKeyboardV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::RiverXkbKeyboardV1;
    const INTERFACE_NAME: &str = "river_xkb_keyboard_v1";
}

impl RiverXkbKeyboardV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl RiverXkbKeyboardV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn RiverXkbKeyboardV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for RiverXkbKeyboardV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RiverXkbKeyboardV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl RiverXkbKeyboardV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the xkb keyboard object
    ///
    /// This request indicates that the client will no longer use the keyboard
    /// object and that it may be safely destroyed.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_xkb_keyboard_v1#{}.destroy()\n", id);
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

    /// destroy the xkb keyboard object
    ///
    /// This request indicates that the client will no longer use the keyboard
    /// object and that it may be safely destroyed.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("river_xkb_keyboard_v1.destroy", &e);
        }
    }

    /// Since when the removed message is available.
    pub const MSG__REMOVED__SINCE: u32 = 1;

    /// the xkb keyboard is removed
    ///
    /// This event indicates that the xkb keyboard has been removed.
    ///
    /// The server will send no further events on this object and ignore any
    /// request (other than river_xkb_keyboard_v1.destroy) made after this event
    /// is sent. The client should destroy this object with the
    /// river_xkb_keyboard_v1.destroy request to free up resources.
    #[inline]
    pub fn try_send_removed(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_xkb_keyboard_v1#{}.removed()\n", client_id, id);
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
            0,
        ]);
        Ok(())
    }

    /// the xkb keyboard is removed
    ///
    /// This event indicates that the xkb keyboard has been removed.
    ///
    /// The server will send no further events on this object and ignore any
    /// request (other than river_xkb_keyboard_v1.destroy) made after this event
    /// is sent. The client should destroy this object with the
    /// river_xkb_keyboard_v1.destroy request to free up resources.
    #[inline]
    pub fn send_removed(
        &self,
    ) {
        let res = self.try_send_removed(
        );
        if let Err(e) = res {
            log_send("river_xkb_keyboard_v1.removed", &e);
        }
    }

    /// Since when the input_device message is available.
    pub const MSG__INPUT_DEVICE__SINCE: u32 = 1;

    /// corresponding river input device
    ///
    /// The river_input_device_v1 corresponding to this xkb keyboard. This event
    /// will always be the first event sent on the river_xkb_keyboard_v1 object,
    /// and it will be sent exactly once.
    ///
    /// # Arguments
    ///
    /// - `device`:
    #[inline]
    pub fn try_send_input_device(
        &self,
        device: &Rc<RiverInputDeviceV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            device,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("device", client.endpoint.id)));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_xkb_keyboard_v1#{}.input_device(device: river_input_device_v1#{})\n", client_id, id, arg0);
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

    /// corresponding river input device
    ///
    /// The river_input_device_v1 corresponding to this xkb keyboard. This event
    /// will always be the first event sent on the river_xkb_keyboard_v1 object,
    /// and it will be sent exactly once.
    ///
    /// # Arguments
    ///
    /// - `device`:
    #[inline]
    pub fn send_input_device(
        &self,
        device: &Rc<RiverInputDeviceV1>,
    ) {
        let res = self.try_send_input_device(
            device,
        );
        if let Err(e) = res {
            log_send("river_xkb_keyboard_v1.input_device", &e);
        }
    }

    /// Since when the set_keymap message is available.
    pub const MSG__SET_KEYMAP__SINCE: u32 = 1;

    /// set the keymap
    ///
    /// Set the keymap for the keyboard.
    ///
    /// It is a protocol error to pass a keymap object for which the
    /// river_xkb_keymap_v1.success event was not received.
    ///
    /// # Arguments
    ///
    /// - `keymap`:
    #[inline]
    pub fn try_send_set_keymap(
        &self,
        keymap: &Rc<RiverXkbKeymapV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            keymap,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("keymap"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_xkb_keyboard_v1#{}.set_keymap(keymap: river_xkb_keymap_v1#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// set the keymap
    ///
    /// Set the keymap for the keyboard.
    ///
    /// It is a protocol error to pass a keymap object for which the
    /// river_xkb_keymap_v1.success event was not received.
    ///
    /// # Arguments
    ///
    /// - `keymap`:
    #[inline]
    pub fn send_set_keymap(
        &self,
        keymap: &Rc<RiverXkbKeymapV1>,
    ) {
        let res = self.try_send_set_keymap(
            keymap,
        );
        if let Err(e) = res {
            log_send("river_xkb_keyboard_v1.set_keymap", &e);
        }
    }

    /// Since when the set_layout_by_index message is available.
    pub const MSG__SET_LAYOUT_BY_INDEX__SINCE: u32 = 1;

    /// set the active layout by index
    ///
    /// Set the active layout for the keyboard's keymap. Has no effect if the
    /// layout index is out of bounds for the current keymap.
    ///
    /// # Arguments
    ///
    /// - `index`:
    #[inline]
    pub fn try_send_set_layout_by_index(
        &self,
        index: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            index,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_xkb_keyboard_v1#{}.set_layout_by_index(index: {})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            2,
            arg0 as u32,
        ]);
        Ok(())
    }

    /// set the active layout by index
    ///
    /// Set the active layout for the keyboard's keymap. Has no effect if the
    /// layout index is out of bounds for the current keymap.
    ///
    /// # Arguments
    ///
    /// - `index`:
    #[inline]
    pub fn send_set_layout_by_index(
        &self,
        index: i32,
    ) {
        let res = self.try_send_set_layout_by_index(
            index,
        );
        if let Err(e) = res {
            log_send("river_xkb_keyboard_v1.set_layout_by_index", &e);
        }
    }

    /// Since when the set_layout_by_name message is available.
    pub const MSG__SET_LAYOUT_BY_NAME__SINCE: u32 = 1;

    /// set the active layout by name
    ///
    /// Set the active layout for the keyboard's keymap. Has no effect if there
    /// is no layout with the give name for the keyboard's keymap.
    ///
    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    pub fn try_send_set_layout_by_name(
        &self,
        name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            name,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_xkb_keyboard_v1#{}.set_layout_by_name(name: {:?})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            3,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// set the active layout by name
    ///
    /// Set the active layout for the keyboard's keymap. Has no effect if there
    /// is no layout with the give name for the keyboard's keymap.
    ///
    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    pub fn send_set_layout_by_name(
        &self,
        name: &str,
    ) {
        let res = self.try_send_set_layout_by_name(
            name,
        );
        if let Err(e) = res {
            log_send("river_xkb_keyboard_v1.set_layout_by_name", &e);
        }
    }

    /// Since when the layout message is available.
    pub const MSG__LAYOUT__SINCE: u32 = 1;

    /// currently active layout
    ///
    /// The currently active layout index and name. The name arg may be null if
    /// the active layout does not have a name.
    ///
    /// This event is sent once when the river_xkb_keyboard_v1 is created and
    /// again whenever the layout changes.
    ///
    /// # Arguments
    ///
    /// - `index`:
    /// - `name`:
    #[inline]
    pub fn try_send_layout(
        &self,
        index: u32,
        name: Option<&str>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            index,
            name,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: Option<&str>) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_xkb_keyboard_v1#{}.layout(index: {}, name: {:?})\n", client_id, id, arg0, arg1);
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
            2,
            arg0,
        ]);
        if let Some(arg1) = arg1 {
            fmt.string(arg1);
        } else {
            fmt.words([0]);
        }
        Ok(())
    }

    /// currently active layout
    ///
    /// The currently active layout index and name. The name arg may be null if
    /// the active layout does not have a name.
    ///
    /// This event is sent once when the river_xkb_keyboard_v1 is created and
    /// again whenever the layout changes.
    ///
    /// # Arguments
    ///
    /// - `index`:
    /// - `name`:
    #[inline]
    pub fn send_layout(
        &self,
        index: u32,
        name: Option<&str>,
    ) {
        let res = self.try_send_layout(
            index,
            name,
        );
        if let Err(e) = res {
            log_send("river_xkb_keyboard_v1.layout", &e);
        }
    }

    /// Since when the capslock_enable message is available.
    pub const MSG__CAPSLOCK_ENABLE__SINCE: u32 = 1;

    /// enable capslock
    ///
    /// Enable capslock for the keyboard.
    #[inline]
    pub fn try_send_capslock_enable(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_xkb_keyboard_v1#{}.capslock_enable()\n", id);
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
            4,
        ]);
        Ok(())
    }

    /// enable capslock
    ///
    /// Enable capslock for the keyboard.
    #[inline]
    pub fn send_capslock_enable(
        &self,
    ) {
        let res = self.try_send_capslock_enable(
        );
        if let Err(e) = res {
            log_send("river_xkb_keyboard_v1.capslock_enable", &e);
        }
    }

    /// Since when the capslock_disable message is available.
    pub const MSG__CAPSLOCK_DISABLE__SINCE: u32 = 1;

    /// disable capslock
    ///
    /// Disable capslock for the keyboard.
    #[inline]
    pub fn try_send_capslock_disable(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_xkb_keyboard_v1#{}.capslock_disable()\n", id);
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
            5,
        ]);
        Ok(())
    }

    /// disable capslock
    ///
    /// Disable capslock for the keyboard.
    #[inline]
    pub fn send_capslock_disable(
        &self,
    ) {
        let res = self.try_send_capslock_disable(
        );
        if let Err(e) = res {
            log_send("river_xkb_keyboard_v1.capslock_disable", &e);
        }
    }

    /// Since when the capslock_enabled message is available.
    pub const MSG__CAPSLOCK_ENABLED__SINCE: u32 = 1;

    /// capslock is currently enabled
    ///
    /// Capslock is currently enabled for the keyboard.
    ///
    /// This event is sent once when the river_xkb_keyboard_v1 is created and
    /// again whenever the capslock state changes.
    #[inline]
    pub fn try_send_capslock_enabled(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_xkb_keyboard_v1#{}.capslock_enabled()\n", client_id, id);
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
            3,
        ]);
        Ok(())
    }

    /// capslock is currently enabled
    ///
    /// Capslock is currently enabled for the keyboard.
    ///
    /// This event is sent once when the river_xkb_keyboard_v1 is created and
    /// again whenever the capslock state changes.
    #[inline]
    pub fn send_capslock_enabled(
        &self,
    ) {
        let res = self.try_send_capslock_enabled(
        );
        if let Err(e) = res {
            log_send("river_xkb_keyboard_v1.capslock_enabled", &e);
        }
    }

    /// Since when the capslock_disabled message is available.
    pub const MSG__CAPSLOCK_DISABLED__SINCE: u32 = 1;

    /// capslock is currently disabled
    ///
    /// Capslock is currently disabled for the keyboard.
    ///
    /// This event is sent once when the river_xkb_keyboard_v1 is created and
    /// again whenever the capslock state changes.
    #[inline]
    pub fn try_send_capslock_disabled(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_xkb_keyboard_v1#{}.capslock_disabled()\n", client_id, id);
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

    /// capslock is currently disabled
    ///
    /// Capslock is currently disabled for the keyboard.
    ///
    /// This event is sent once when the river_xkb_keyboard_v1 is created and
    /// again whenever the capslock state changes.
    #[inline]
    pub fn send_capslock_disabled(
        &self,
    ) {
        let res = self.try_send_capslock_disabled(
        );
        if let Err(e) = res {
            log_send("river_xkb_keyboard_v1.capslock_disabled", &e);
        }
    }

    /// Since when the numlock_enable message is available.
    pub const MSG__NUMLOCK_ENABLE__SINCE: u32 = 1;

    /// enable numlock
    ///
    /// Enable numlock for the keyboard.
    #[inline]
    pub fn try_send_numlock_enable(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_xkb_keyboard_v1#{}.numlock_enable()\n", id);
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
            6,
        ]);
        Ok(())
    }

    /// enable numlock
    ///
    /// Enable numlock for the keyboard.
    #[inline]
    pub fn send_numlock_enable(
        &self,
    ) {
        let res = self.try_send_numlock_enable(
        );
        if let Err(e) = res {
            log_send("river_xkb_keyboard_v1.numlock_enable", &e);
        }
    }

    /// Since when the numlock_disable message is available.
    pub const MSG__NUMLOCK_DISABLE__SINCE: u32 = 1;

    /// disable numlock
    ///
    /// Disable numlock for the keyboard.
    #[inline]
    pub fn try_send_numlock_disable(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_xkb_keyboard_v1#{}.numlock_disable()\n", id);
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
            7,
        ]);
        Ok(())
    }

    /// disable numlock
    ///
    /// Disable numlock for the keyboard.
    #[inline]
    pub fn send_numlock_disable(
        &self,
    ) {
        let res = self.try_send_numlock_disable(
        );
        if let Err(e) = res {
            log_send("river_xkb_keyboard_v1.numlock_disable", &e);
        }
    }

    /// Since when the numlock_enabled message is available.
    pub const MSG__NUMLOCK_ENABLED__SINCE: u32 = 1;

    /// numlock is currently enabled
    ///
    /// Numlock is currently enabled for the keyboard.
    ///
    /// This event is sent once when the river_xkb_keyboard_v1 is created and
    /// again whenever the numlock state changes.
    #[inline]
    pub fn try_send_numlock_enabled(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_xkb_keyboard_v1#{}.numlock_enabled()\n", client_id, id);
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
            5,
        ]);
        Ok(())
    }

    /// numlock is currently enabled
    ///
    /// Numlock is currently enabled for the keyboard.
    ///
    /// This event is sent once when the river_xkb_keyboard_v1 is created and
    /// again whenever the numlock state changes.
    #[inline]
    pub fn send_numlock_enabled(
        &self,
    ) {
        let res = self.try_send_numlock_enabled(
        );
        if let Err(e) = res {
            log_send("river_xkb_keyboard_v1.numlock_enabled", &e);
        }
    }

    /// Since when the numlock_disabled message is available.
    pub const MSG__NUMLOCK_DISABLED__SINCE: u32 = 1;

    /// numlock is currently disabled
    ///
    /// Numlock is currently disabled for the keyboard.
    ///
    /// This event is sent once when the river_xkb_keyboard_v1 is created and
    /// again whenever the numlock state changes.
    #[inline]
    pub fn try_send_numlock_disabled(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_xkb_keyboard_v1#{}.numlock_disabled()\n", client_id, id);
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
            6,
        ]);
        Ok(())
    }

    /// numlock is currently disabled
    ///
    /// Numlock is currently disabled for the keyboard.
    ///
    /// This event is sent once when the river_xkb_keyboard_v1 is created and
    /// again whenever the numlock state changes.
    #[inline]
    pub fn send_numlock_disabled(
        &self,
    ) {
        let res = self.try_send_numlock_disabled(
        );
        if let Err(e) = res {
            log_send("river_xkb_keyboard_v1.numlock_disabled", &e);
        }
    }
}

/// A message handler for [`RiverXkbKeyboardV1`] proxies.
pub trait RiverXkbKeyboardV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<RiverXkbKeyboardV1>) {
        slf.core.delete_id();
    }

    /// destroy the xkb keyboard object
    ///
    /// This request indicates that the client will no longer use the keyboard
    /// object and that it may be safely destroyed.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<RiverXkbKeyboardV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("river_xkb_keyboard_v1.destroy", &e);
        }
    }

    /// the xkb keyboard is removed
    ///
    /// This event indicates that the xkb keyboard has been removed.
    ///
    /// The server will send no further events on this object and ignore any
    /// request (other than river_xkb_keyboard_v1.destroy) made after this event
    /// is sent. The client should destroy this object with the
    /// river_xkb_keyboard_v1.destroy request to free up resources.
    #[inline]
    fn handle_removed(
        &mut self,
        slf: &Rc<RiverXkbKeyboardV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_removed(
        );
        if let Err(e) = res {
            log_forward("river_xkb_keyboard_v1.removed", &e);
        }
    }

    /// corresponding river input device
    ///
    /// The river_input_device_v1 corresponding to this xkb keyboard. This event
    /// will always be the first event sent on the river_xkb_keyboard_v1 object,
    /// and it will be sent exactly once.
    ///
    /// # Arguments
    ///
    /// - `device`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_input_device(
        &mut self,
        slf: &Rc<RiverXkbKeyboardV1>,
        device: &Rc<RiverInputDeviceV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = device.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = slf.try_send_input_device(
            device,
        );
        if let Err(e) = res {
            log_forward("river_xkb_keyboard_v1.input_device", &e);
        }
    }

    /// set the keymap
    ///
    /// Set the keymap for the keyboard.
    ///
    /// It is a protocol error to pass a keymap object for which the
    /// river_xkb_keymap_v1.success event was not received.
    ///
    /// # Arguments
    ///
    /// - `keymap`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_keymap(
        &mut self,
        slf: &Rc<RiverXkbKeyboardV1>,
        keymap: &Rc<RiverXkbKeymapV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_keymap(
            keymap,
        );
        if let Err(e) = res {
            log_forward("river_xkb_keyboard_v1.set_keymap", &e);
        }
    }

    /// set the active layout by index
    ///
    /// Set the active layout for the keyboard's keymap. Has no effect if the
    /// layout index is out of bounds for the current keymap.
    ///
    /// # Arguments
    ///
    /// - `index`:
    #[inline]
    fn handle_set_layout_by_index(
        &mut self,
        slf: &Rc<RiverXkbKeyboardV1>,
        index: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_layout_by_index(
            index,
        );
        if let Err(e) = res {
            log_forward("river_xkb_keyboard_v1.set_layout_by_index", &e);
        }
    }

    /// set the active layout by name
    ///
    /// Set the active layout for the keyboard's keymap. Has no effect if there
    /// is no layout with the give name for the keyboard's keymap.
    ///
    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    fn handle_set_layout_by_name(
        &mut self,
        slf: &Rc<RiverXkbKeyboardV1>,
        name: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_layout_by_name(
            name,
        );
        if let Err(e) = res {
            log_forward("river_xkb_keyboard_v1.set_layout_by_name", &e);
        }
    }

    /// currently active layout
    ///
    /// The currently active layout index and name. The name arg may be null if
    /// the active layout does not have a name.
    ///
    /// This event is sent once when the river_xkb_keyboard_v1 is created and
    /// again whenever the layout changes.
    ///
    /// # Arguments
    ///
    /// - `index`:
    /// - `name`:
    #[inline]
    fn handle_layout(
        &mut self,
        slf: &Rc<RiverXkbKeyboardV1>,
        index: u32,
        name: Option<&str>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_layout(
            index,
            name,
        );
        if let Err(e) = res {
            log_forward("river_xkb_keyboard_v1.layout", &e);
        }
    }

    /// enable capslock
    ///
    /// Enable capslock for the keyboard.
    #[inline]
    fn handle_capslock_enable(
        &mut self,
        slf: &Rc<RiverXkbKeyboardV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_capslock_enable(
        );
        if let Err(e) = res {
            log_forward("river_xkb_keyboard_v1.capslock_enable", &e);
        }
    }

    /// disable capslock
    ///
    /// Disable capslock for the keyboard.
    #[inline]
    fn handle_capslock_disable(
        &mut self,
        slf: &Rc<RiverXkbKeyboardV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_capslock_disable(
        );
        if let Err(e) = res {
            log_forward("river_xkb_keyboard_v1.capslock_disable", &e);
        }
    }

    /// capslock is currently enabled
    ///
    /// Capslock is currently enabled for the keyboard.
    ///
    /// This event is sent once when the river_xkb_keyboard_v1 is created and
    /// again whenever the capslock state changes.
    #[inline]
    fn handle_capslock_enabled(
        &mut self,
        slf: &Rc<RiverXkbKeyboardV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_capslock_enabled(
        );
        if let Err(e) = res {
            log_forward("river_xkb_keyboard_v1.capslock_enabled", &e);
        }
    }

    /// capslock is currently disabled
    ///
    /// Capslock is currently disabled for the keyboard.
    ///
    /// This event is sent once when the river_xkb_keyboard_v1 is created and
    /// again whenever the capslock state changes.
    #[inline]
    fn handle_capslock_disabled(
        &mut self,
        slf: &Rc<RiverXkbKeyboardV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_capslock_disabled(
        );
        if let Err(e) = res {
            log_forward("river_xkb_keyboard_v1.capslock_disabled", &e);
        }
    }

    /// enable numlock
    ///
    /// Enable numlock for the keyboard.
    #[inline]
    fn handle_numlock_enable(
        &mut self,
        slf: &Rc<RiverXkbKeyboardV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_numlock_enable(
        );
        if let Err(e) = res {
            log_forward("river_xkb_keyboard_v1.numlock_enable", &e);
        }
    }

    /// disable numlock
    ///
    /// Disable numlock for the keyboard.
    #[inline]
    fn handle_numlock_disable(
        &mut self,
        slf: &Rc<RiverXkbKeyboardV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_numlock_disable(
        );
        if let Err(e) = res {
            log_forward("river_xkb_keyboard_v1.numlock_disable", &e);
        }
    }

    /// numlock is currently enabled
    ///
    /// Numlock is currently enabled for the keyboard.
    ///
    /// This event is sent once when the river_xkb_keyboard_v1 is created and
    /// again whenever the numlock state changes.
    #[inline]
    fn handle_numlock_enabled(
        &mut self,
        slf: &Rc<RiverXkbKeyboardV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_numlock_enabled(
        );
        if let Err(e) = res {
            log_forward("river_xkb_keyboard_v1.numlock_enabled", &e);
        }
    }

    /// numlock is currently disabled
    ///
    /// Numlock is currently disabled for the keyboard.
    ///
    /// This event is sent once when the river_xkb_keyboard_v1 is created and
    /// again whenever the numlock state changes.
    #[inline]
    fn handle_numlock_disabled(
        &mut self,
        slf: &Rc<RiverXkbKeyboardV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_numlock_disabled(
        );
        if let Err(e) = res {
            log_forward("river_xkb_keyboard_v1.numlock_disabled", &e);
        }
    }
}

impl ObjectPrivate for RiverXkbKeyboardV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::RiverXkbKeyboardV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_xkb_keyboard_v1#{}.destroy()\n", client_id, id);
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
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_xkb_keyboard_v1#{}.set_keymap(keymap: river_xkb_keymap_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<RiverXkbKeymapV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("keymap", o.core().interface, ObjectInterface::RiverXkbKeymapV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_keymap(&self, arg0);
                } else {
                    DefaultHandler.handle_set_keymap(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = arg0 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_xkb_keyboard_v1#{}.set_layout_by_index(index: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_layout_by_index(&self, arg0);
                } else {
                    DefaultHandler.handle_set_layout_by_index(&self, arg0);
                }
            }
            3 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "name")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_xkb_keyboard_v1#{}.set_layout_by_name(name: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_layout_by_name(&self, arg0);
                } else {
                    DefaultHandler.handle_set_layout_by_name(&self, arg0);
                }
            }
            4 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_xkb_keyboard_v1#{}.capslock_enable()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_capslock_enable(&self);
                } else {
                    DefaultHandler.handle_capslock_enable(&self);
                }
            }
            5 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_xkb_keyboard_v1#{}.capslock_disable()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_capslock_disable(&self);
                } else {
                    DefaultHandler.handle_capslock_disable(&self);
                }
            }
            6 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_xkb_keyboard_v1#{}.numlock_enable()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_numlock_enable(&self);
                } else {
                    DefaultHandler.handle_numlock_enable(&self);
                }
            }
            7 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_xkb_keyboard_v1#{}.numlock_disable()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_numlock_disable(&self);
                } else {
                    DefaultHandler.handle_numlock_disable(&self);
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
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_xkb_keyboard_v1#{}.removed()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_removed(&self);
                } else {
                    DefaultHandler.handle_removed(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_xkb_keyboard_v1#{}.input_device(device: river_input_device_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = server.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<RiverInputDeviceV1>() else {
                    let o = server.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("device", o.core().interface, ObjectInterface::RiverInputDeviceV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_input_device(&self, arg0);
                } else {
                    DefaultHandler.handle_input_device(&self, arg0);
                }
            }
            2 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("index")));
                };
                offset += 1;
                let arg1;
                (arg1, offset) = parse_string::<NullableString>(msg, offset, "name")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: Option<&str>) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_xkb_keyboard_v1#{}.layout(index: {}, name: {:?})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_layout(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_layout(&self, arg0, arg1);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_xkb_keyboard_v1#{}.capslock_enabled()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_capslock_enabled(&self);
                } else {
                    DefaultHandler.handle_capslock_enabled(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_xkb_keyboard_v1#{}.capslock_disabled()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_capslock_disabled(&self);
                } else {
                    DefaultHandler.handle_capslock_disabled(&self);
                }
            }
            5 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_xkb_keyboard_v1#{}.numlock_enabled()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_numlock_enabled(&self);
                } else {
                    DefaultHandler.handle_numlock_enabled(&self);
                }
            }
            6 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_xkb_keyboard_v1#{}.numlock_disabled()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_numlock_disabled(&self);
                } else {
                    DefaultHandler.handle_numlock_disabled(&self);
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
            1 => "set_keymap",
            2 => "set_layout_by_index",
            3 => "set_layout_by_name",
            4 => "capslock_enable",
            5 => "capslock_disable",
            6 => "numlock_enable",
            7 => "numlock_disable",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "removed",
            1 => "input_device",
            2 => "layout",
            3 => "capslock_enabled",
            4 => "capslock_disabled",
            5 => "numlock_enabled",
            6 => "numlock_disabled",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for RiverXkbKeyboardV1 {
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

impl RiverXkbKeyboardV1 {
    /// Since when the error.invalid_keymap enum variant is available.
    pub const ENM__ERROR_INVALID_KEYMAP__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverXkbKeyboardV1Error(pub u32);

impl RiverXkbKeyboardV1Error {
    pub const INVALID_KEYMAP: Self = Self(0);
}

impl Debug for RiverXkbKeyboardV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_KEYMAP => "INVALID_KEYMAP",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
