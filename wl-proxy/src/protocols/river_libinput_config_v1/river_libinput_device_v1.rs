//! a libinput device
//!
//! In general, *_support events will be sent exactly once directly after the
//! river_libinput_device_v1 is created. *_default events will be sent after
//! *_support events if the config option is supported, and *_current events
//! willl be sent after the *_default events and again whenever the config
//! option is changed.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A river_libinput_device_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct RiverLibinputDeviceV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn RiverLibinputDeviceV1Handler>,
}

struct DefaultHandler;

impl RiverLibinputDeviceV1Handler for DefaultHandler { }

impl ConcreteObject for RiverLibinputDeviceV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::RiverLibinputDeviceV1;
    const INTERFACE_NAME: &str = "river_libinput_device_v1";
}

impl RiverLibinputDeviceV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl RiverLibinputDeviceV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn RiverLibinputDeviceV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for RiverLibinputDeviceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RiverLibinputDeviceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl RiverLibinputDeviceV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the libinput device object
    ///
    /// This request indicates that the client will no longer use the input
    /// device object and that it may be safely destroyed.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_device_v1#{}.destroy()\n", id);
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

    /// destroy the libinput device object
    ///
    /// This request indicates that the client will no longer use the input
    /// device object and that it may be safely destroyed.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.destroy", &e);
        }
    }

    /// Since when the removed message is available.
    pub const MSG__REMOVED__SINCE: u32 = 1;

    /// the libinput device is removed
    ///
    /// This event indicates that the libinput device has been removed.
    ///
    /// The server will send no further events on this object and ignore any
    /// request (other than river_libinput_device_v1.destroy) made after this
    /// event is sent. The client should destroy this object with the
    /// river_libinput_device_v1.destroy request to free up resources.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.removed()\n", client_id, id);
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

    /// the libinput device is removed
    ///
    /// This event indicates that the libinput device has been removed.
    ///
    /// The server will send no further events on this object and ignore any
    /// request (other than river_libinput_device_v1.destroy) made after this
    /// event is sent. The client should destroy this object with the
    /// river_libinput_device_v1.destroy request to free up resources.
    #[inline]
    pub fn send_removed(
        &self,
    ) {
        let res = self.try_send_removed(
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.removed", &e);
        }
    }

    /// Since when the input_device message is available.
    pub const MSG__INPUT_DEVICE__SINCE: u32 = 1;

    /// corresponding river input device
    ///
    /// The river_input_device_v1 corresponding to this libinput device.
    /// This event will always be the first event sent on the
    /// river_libinput_device_v1 object, and it will be sent exactly once.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.input_device(device: river_input_device_v1#{})\n", client_id, id, arg0);
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
    /// The river_input_device_v1 corresponding to this libinput device.
    /// This event will always be the first event sent on the
    /// river_libinput_device_v1 object, and it will be sent exactly once.
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
            log_send("river_libinput_device_v1.input_device", &e);
        }
    }

    /// Since when the send_events_support message is available.
    pub const MSG__SEND_EVENTS_SUPPORT__SINCE: u32 = 1;

    /// supported send events modes
    ///
    /// Supported send events modes.
    ///
    /// # Arguments
    ///
    /// - `modes`:
    #[inline]
    pub fn try_send_send_events_support(
        &self,
        modes: RiverLibinputDeviceV1SendEventsModes,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            modes,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1SendEventsModes) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.send_events_support(modes: {:?})\n", client_id, id, arg0);
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
            2,
            arg0.0,
        ]);
        Ok(())
    }

    /// supported send events modes
    ///
    /// Supported send events modes.
    ///
    /// # Arguments
    ///
    /// - `modes`:
    #[inline]
    pub fn send_send_events_support(
        &self,
        modes: RiverLibinputDeviceV1SendEventsModes,
    ) {
        let res = self.try_send_send_events_support(
            modes,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.send_events_support", &e);
        }
    }

    /// Since when the send_events_default message is available.
    pub const MSG__SEND_EVENTS_DEFAULT__SINCE: u32 = 1;

    /// default send events mode
    ///
    /// Default send events mode.
    ///
    /// # Arguments
    ///
    /// - `mode`:
    #[inline]
    pub fn try_send_send_events_default(
        &self,
        mode: RiverLibinputDeviceV1SendEventsModes,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            mode,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1SendEventsModes) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.send_events_default(mode: {:?})\n", client_id, id, arg0);
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
            3,
            arg0.0,
        ]);
        Ok(())
    }

    /// default send events mode
    ///
    /// Default send events mode.
    ///
    /// # Arguments
    ///
    /// - `mode`:
    #[inline]
    pub fn send_send_events_default(
        &self,
        mode: RiverLibinputDeviceV1SendEventsModes,
    ) {
        let res = self.try_send_send_events_default(
            mode,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.send_events_default", &e);
        }
    }

    /// Since when the send_events_current message is available.
    pub const MSG__SEND_EVENTS_CURRENT__SINCE: u32 = 1;

    /// current send events mode
    ///
    /// Current send events mode.
    ///
    /// # Arguments
    ///
    /// - `mode`:
    #[inline]
    pub fn try_send_send_events_current(
        &self,
        mode: RiverLibinputDeviceV1SendEventsModes,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            mode,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1SendEventsModes) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.send_events_current(mode: {:?})\n", client_id, id, arg0);
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
            4,
            arg0.0,
        ]);
        Ok(())
    }

    /// current send events mode
    ///
    /// Current send events mode.
    ///
    /// # Arguments
    ///
    /// - `mode`:
    #[inline]
    pub fn send_send_events_current(
        &self,
        mode: RiverLibinputDeviceV1SendEventsModes,
    ) {
        let res = self.try_send_send_events_current(
            mode,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.send_events_current", &e);
        }
    }

    /// Since when the set_send_events message is available.
    pub const MSG__SET_SEND_EVENTS__SINCE: u32 = 1;

    /// set send events mode
    ///
    /// Set the send events mode for the device.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `mode`:
    #[inline]
    pub fn try_send_set_send_events(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        mode: RiverLibinputDeviceV1SendEventsModes,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            result,
            mode,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("result", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1SendEventsModes) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_device_v1#{}.set_send_events(result: river_libinput_result_v1#{}, mode: {:?})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            arg1.0,
        ]);
        Ok(())
    }

    /// set send events mode
    ///
    /// Set the send events mode for the device.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `mode`:
    #[inline]
    pub fn send_set_send_events(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        mode: RiverLibinputDeviceV1SendEventsModes,
    ) {
        let res = self.try_send_set_send_events(
            result,
            mode,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.set_send_events", &e);
        }
    }

    /// set send events mode
    ///
    /// Set the send events mode for the device.
    ///
    /// # Arguments
    ///
    /// - `mode`:
    #[inline]
    pub fn new_try_send_set_send_events(
        &self,
        mode: RiverLibinputDeviceV1SendEventsModes,
    ) -> Result<Rc<RiverLibinputResultV1>, ObjectError> {
        let result = self.core.create_child();
        self.try_send_set_send_events(
            &result,
            mode,
        )?;
        Ok(result)
    }

    /// set send events mode
    ///
    /// Set the send events mode for the device.
    ///
    /// # Arguments
    ///
    /// - `mode`:
    #[inline]
    pub fn new_send_set_send_events(
        &self,
        mode: RiverLibinputDeviceV1SendEventsModes,
    ) -> Rc<RiverLibinputResultV1> {
        let result = self.core.create_child();
        self.send_set_send_events(
            &result,
            mode,
        );
        result
    }

    /// Since when the tap_support message is available.
    pub const MSG__TAP_SUPPORT__SINCE: u32 = 1;

    /// tap-to-click/drag support
    ///
    /// The number of fingers supported for tap-to-click/drag.
    /// If finger_count is 0, tap-to-click and drag are unsupported.
    ///
    /// # Arguments
    ///
    /// - `finger_count`:
    #[inline]
    pub fn try_send_tap_support(
        &self,
        finger_count: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            finger_count,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.tap_support(finger_count: {})\n", client_id, id, arg0);
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
            5,
            arg0 as u32,
        ]);
        Ok(())
    }

    /// tap-to-click/drag support
    ///
    /// The number of fingers supported for tap-to-click/drag.
    /// If finger_count is 0, tap-to-click and drag are unsupported.
    ///
    /// # Arguments
    ///
    /// - `finger_count`:
    #[inline]
    pub fn send_tap_support(
        &self,
        finger_count: i32,
    ) {
        let res = self.try_send_tap_support(
            finger_count,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.tap_support", &e);
        }
    }

    /// Since when the tap_default message is available.
    pub const MSG__TAP_DEFAULT__SINCE: u32 = 1;

    /// default tap-to-click state
    ///
    /// Default tap-to-click state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_tap_default(
        &self,
        state: RiverLibinputDeviceV1TapState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1TapState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.tap_default(state: {:?})\n", client_id, id, arg0);
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
            6,
            arg0.0,
        ]);
        Ok(())
    }

    /// default tap-to-click state
    ///
    /// Default tap-to-click state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_tap_default(
        &self,
        state: RiverLibinputDeviceV1TapState,
    ) {
        let res = self.try_send_tap_default(
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.tap_default", &e);
        }
    }

    /// Since when the tap_current message is available.
    pub const MSG__TAP_CURRENT__SINCE: u32 = 1;

    /// current tap-to-click state
    ///
    /// Current tap-to-click state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_tap_current(
        &self,
        state: RiverLibinputDeviceV1TapState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1TapState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.tap_current(state: {:?})\n", client_id, id, arg0);
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
            7,
            arg0.0,
        ]);
        Ok(())
    }

    /// current tap-to-click state
    ///
    /// Current tap-to-click state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_tap_current(
        &self,
        state: RiverLibinputDeviceV1TapState,
    ) {
        let res = self.try_send_tap_current(
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.tap_current", &e);
        }
    }

    /// Since when the set_tap message is available.
    pub const MSG__SET_TAP__SINCE: u32 = 1;

    /// enable/disable tap-to-click
    ///
    /// Configure tap-to-click on this device, with a default mapping of
    /// 1, 2, 3 finger tap mapping to left, right, middle click, respectively.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    pub fn try_send_set_tap(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1TapState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            result,
            state,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("result", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1TapState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_device_v1#{}.set_tap(result: river_libinput_result_v1#{}, state: {:?})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            arg0_id,
            arg1.0,
        ]);
        Ok(())
    }

    /// enable/disable tap-to-click
    ///
    /// Configure tap-to-click on this device, with a default mapping of
    /// 1, 2, 3 finger tap mapping to left, right, middle click, respectively.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    pub fn send_set_tap(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1TapState,
    ) {
        let res = self.try_send_set_tap(
            result,
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.set_tap", &e);
        }
    }

    /// enable/disable tap-to-click
    ///
    /// Configure tap-to-click on this device, with a default mapping of
    /// 1, 2, 3 finger tap mapping to left, right, middle click, respectively.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn new_try_send_set_tap(
        &self,
        state: RiverLibinputDeviceV1TapState,
    ) -> Result<Rc<RiverLibinputResultV1>, ObjectError> {
        let result = self.core.create_child();
        self.try_send_set_tap(
            &result,
            state,
        )?;
        Ok(result)
    }

    /// enable/disable tap-to-click
    ///
    /// Configure tap-to-click on this device, with a default mapping of
    /// 1, 2, 3 finger tap mapping to left, right, middle click, respectively.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn new_send_set_tap(
        &self,
        state: RiverLibinputDeviceV1TapState,
    ) -> Rc<RiverLibinputResultV1> {
        let result = self.core.create_child();
        self.send_set_tap(
            &result,
            state,
        );
        result
    }

    /// Since when the tap_button_map_default message is available.
    pub const MSG__TAP_BUTTON_MAP_DEFAULT__SINCE: u32 = 1;

    /// default tap-to-click button map
    ///
    /// Default tap-to-click button map.
    ///
    /// # Arguments
    ///
    /// - `button_map`:
    #[inline]
    pub fn try_send_tap_button_map_default(
        &self,
        button_map: RiverLibinputDeviceV1TapButtonMap,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            button_map,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1TapButtonMap) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.tap_button_map_default(button_map: {:?})\n", client_id, id, arg0);
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
            8,
            arg0.0,
        ]);
        Ok(())
    }

    /// default tap-to-click button map
    ///
    /// Default tap-to-click button map.
    ///
    /// # Arguments
    ///
    /// - `button_map`:
    #[inline]
    pub fn send_tap_button_map_default(
        &self,
        button_map: RiverLibinputDeviceV1TapButtonMap,
    ) {
        let res = self.try_send_tap_button_map_default(
            button_map,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.tap_button_map_default", &e);
        }
    }

    /// Since when the tap_button_map_current message is available.
    pub const MSG__TAP_BUTTON_MAP_CURRENT__SINCE: u32 = 1;

    /// current tap-to-click button map
    ///
    /// Current tap-to-click button map.
    ///
    /// # Arguments
    ///
    /// - `button_map`:
    #[inline]
    pub fn try_send_tap_button_map_current(
        &self,
        button_map: RiverLibinputDeviceV1TapButtonMap,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            button_map,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1TapButtonMap) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.tap_button_map_current(button_map: {:?})\n", client_id, id, arg0);
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
            9,
            arg0.0,
        ]);
        Ok(())
    }

    /// current tap-to-click button map
    ///
    /// Current tap-to-click button map.
    ///
    /// # Arguments
    ///
    /// - `button_map`:
    #[inline]
    pub fn send_tap_button_map_current(
        &self,
        button_map: RiverLibinputDeviceV1TapButtonMap,
    ) {
        let res = self.try_send_tap_button_map_current(
            button_map,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.tap_button_map_current", &e);
        }
    }

    /// Since when the set_tap_button_map message is available.
    pub const MSG__SET_TAP_BUTTON_MAP__SINCE: u32 = 1;

    /// set tap-to-click button map
    ///
    /// Set the finger number to button number mapping for tap-to-click. The
    /// default mapping on most devices is to have a 1, 2 and 3 finger tap to
    /// map to the left, right and middle button, respectively.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `button_map`:
    #[inline]
    pub fn try_send_set_tap_button_map(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        button_map: RiverLibinputDeviceV1TapButtonMap,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            result,
            button_map,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("result", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1TapButtonMap) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_device_v1#{}.set_tap_button_map(result: river_libinput_result_v1#{}, button_map: {:?})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            arg0_id,
            arg1.0,
        ]);
        Ok(())
    }

    /// set tap-to-click button map
    ///
    /// Set the finger number to button number mapping for tap-to-click. The
    /// default mapping on most devices is to have a 1, 2 and 3 finger tap to
    /// map to the left, right and middle button, respectively.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `button_map`:
    #[inline]
    pub fn send_set_tap_button_map(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        button_map: RiverLibinputDeviceV1TapButtonMap,
    ) {
        let res = self.try_send_set_tap_button_map(
            result,
            button_map,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.set_tap_button_map", &e);
        }
    }

    /// set tap-to-click button map
    ///
    /// Set the finger number to button number mapping for tap-to-click. The
    /// default mapping on most devices is to have a 1, 2 and 3 finger tap to
    /// map to the left, right and middle button, respectively.
    ///
    /// # Arguments
    ///
    /// - `button_map`:
    #[inline]
    pub fn new_try_send_set_tap_button_map(
        &self,
        button_map: RiverLibinputDeviceV1TapButtonMap,
    ) -> Result<Rc<RiverLibinputResultV1>, ObjectError> {
        let result = self.core.create_child();
        self.try_send_set_tap_button_map(
            &result,
            button_map,
        )?;
        Ok(result)
    }

    /// set tap-to-click button map
    ///
    /// Set the finger number to button number mapping for tap-to-click. The
    /// default mapping on most devices is to have a 1, 2 and 3 finger tap to
    /// map to the left, right and middle button, respectively.
    ///
    /// # Arguments
    ///
    /// - `button_map`:
    #[inline]
    pub fn new_send_set_tap_button_map(
        &self,
        button_map: RiverLibinputDeviceV1TapButtonMap,
    ) -> Rc<RiverLibinputResultV1> {
        let result = self.core.create_child();
        self.send_set_tap_button_map(
            &result,
            button_map,
        );
        result
    }

    /// Since when the drag_default message is available.
    pub const MSG__DRAG_DEFAULT__SINCE: u32 = 1;

    /// default tap-and-drag state
    ///
    /// Default tap-and-drag state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_drag_default(
        &self,
        state: RiverLibinputDeviceV1DragState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1DragState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.drag_default(state: {:?})\n", client_id, id, arg0);
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
            10,
            arg0.0,
        ]);
        Ok(())
    }

    /// default tap-and-drag state
    ///
    /// Default tap-and-drag state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_drag_default(
        &self,
        state: RiverLibinputDeviceV1DragState,
    ) {
        let res = self.try_send_drag_default(
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.drag_default", &e);
        }
    }

    /// Since when the drag_current message is available.
    pub const MSG__DRAG_CURRENT__SINCE: u32 = 1;

    /// current tap-and-drag state
    ///
    /// Current tap-and-drag state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_drag_current(
        &self,
        state: RiverLibinputDeviceV1DragState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1DragState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.drag_current(state: {:?})\n", client_id, id, arg0);
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
            11,
            arg0.0,
        ]);
        Ok(())
    }

    /// current tap-and-drag state
    ///
    /// Current tap-and-drag state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_drag_current(
        &self,
        state: RiverLibinputDeviceV1DragState,
    ) {
        let res = self.try_send_drag_current(
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.drag_current", &e);
        }
    }

    /// Since when the set_drag message is available.
    pub const MSG__SET_DRAG__SINCE: u32 = 1;

    /// set tap-and-drag state
    ///
    /// Configure tap-and-drag functionality on the device.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    pub fn try_send_set_drag(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1DragState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            result,
            state,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("result", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1DragState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_device_v1#{}.set_drag(result: river_libinput_result_v1#{}, state: {:?})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            arg0_id,
            arg1.0,
        ]);
        Ok(())
    }

    /// set tap-and-drag state
    ///
    /// Configure tap-and-drag functionality on the device.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    pub fn send_set_drag(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1DragState,
    ) {
        let res = self.try_send_set_drag(
            result,
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.set_drag", &e);
        }
    }

    /// set tap-and-drag state
    ///
    /// Configure tap-and-drag functionality on the device.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn new_try_send_set_drag(
        &self,
        state: RiverLibinputDeviceV1DragState,
    ) -> Result<Rc<RiverLibinputResultV1>, ObjectError> {
        let result = self.core.create_child();
        self.try_send_set_drag(
            &result,
            state,
        )?;
        Ok(result)
    }

    /// set tap-and-drag state
    ///
    /// Configure tap-and-drag functionality on the device.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn new_send_set_drag(
        &self,
        state: RiverLibinputDeviceV1DragState,
    ) -> Rc<RiverLibinputResultV1> {
        let result = self.core.create_child();
        self.send_set_drag(
            &result,
            state,
        );
        result
    }

    /// Since when the drag_lock_default message is available.
    pub const MSG__DRAG_LOCK_DEFAULT__SINCE: u32 = 1;

    /// default drag lock state
    ///
    /// Default drag lock state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_drag_lock_default(
        &self,
        state: RiverLibinputDeviceV1DragLockState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1DragLockState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.drag_lock_default(state: {:?})\n", client_id, id, arg0);
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
            12,
            arg0.0,
        ]);
        Ok(())
    }

    /// default drag lock state
    ///
    /// Default drag lock state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_drag_lock_default(
        &self,
        state: RiverLibinputDeviceV1DragLockState,
    ) {
        let res = self.try_send_drag_lock_default(
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.drag_lock_default", &e);
        }
    }

    /// Since when the drag_lock_current message is available.
    pub const MSG__DRAG_LOCK_CURRENT__SINCE: u32 = 1;

    /// current drag lock state
    ///
    /// Current drag lock state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_drag_lock_current(
        &self,
        state: RiverLibinputDeviceV1DragLockState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1DragLockState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.drag_lock_current(state: {:?})\n", client_id, id, arg0);
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
            13,
            arg0.0,
        ]);
        Ok(())
    }

    /// current drag lock state
    ///
    /// Current drag lock state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_drag_lock_current(
        &self,
        state: RiverLibinputDeviceV1DragLockState,
    ) {
        let res = self.try_send_drag_lock_current(
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.drag_lock_current", &e);
        }
    }

    /// Since when the set_drag_lock message is available.
    pub const MSG__SET_DRAG_LOCK__SINCE: u32 = 1;

    /// set drag lock state
    ///
    /// Configure drag-lock during tapping on this device. When enabled, a
    /// finger may be lifted and put back on the touchpad and the drag process
    /// continues. A timeout for lifting the finger is optional. When disabled,
    /// lifting the finger during a tap-and-drag will immediately stop the drag.
    /// See the libinput documentation for more details.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    pub fn try_send_set_drag_lock(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1DragLockState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            result,
            state,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("result", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1DragLockState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_device_v1#{}.set_drag_lock(result: river_libinput_result_v1#{}, state: {:?})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            arg0_id,
            arg1.0,
        ]);
        Ok(())
    }

    /// set drag lock state
    ///
    /// Configure drag-lock during tapping on this device. When enabled, a
    /// finger may be lifted and put back on the touchpad and the drag process
    /// continues. A timeout for lifting the finger is optional. When disabled,
    /// lifting the finger during a tap-and-drag will immediately stop the drag.
    /// See the libinput documentation for more details.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    pub fn send_set_drag_lock(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1DragLockState,
    ) {
        let res = self.try_send_set_drag_lock(
            result,
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.set_drag_lock", &e);
        }
    }

    /// set drag lock state
    ///
    /// Configure drag-lock during tapping on this device. When enabled, a
    /// finger may be lifted and put back on the touchpad and the drag process
    /// continues. A timeout for lifting the finger is optional. When disabled,
    /// lifting the finger during a tap-and-drag will immediately stop the drag.
    /// See the libinput documentation for more details.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn new_try_send_set_drag_lock(
        &self,
        state: RiverLibinputDeviceV1DragLockState,
    ) -> Result<Rc<RiverLibinputResultV1>, ObjectError> {
        let result = self.core.create_child();
        self.try_send_set_drag_lock(
            &result,
            state,
        )?;
        Ok(result)
    }

    /// set drag lock state
    ///
    /// Configure drag-lock during tapping on this device. When enabled, a
    /// finger may be lifted and put back on the touchpad and the drag process
    /// continues. A timeout for lifting the finger is optional. When disabled,
    /// lifting the finger during a tap-and-drag will immediately stop the drag.
    /// See the libinput documentation for more details.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn new_send_set_drag_lock(
        &self,
        state: RiverLibinputDeviceV1DragLockState,
    ) -> Rc<RiverLibinputResultV1> {
        let result = self.core.create_child();
        self.send_set_drag_lock(
            &result,
            state,
        );
        result
    }

    /// Since when the three_finger_drag_support message is available.
    pub const MSG__THREE_FINGER_DRAG_SUPPORT__SINCE: u32 = 1;

    /// three finger drag support
    ///
    /// The number of fingers supported for three/four finger drag.
    /// If finger_count is less than 3, three finger drag is unsupported.
    ///
    /// # Arguments
    ///
    /// - `finger_count`:
    #[inline]
    pub fn try_send_three_finger_drag_support(
        &self,
        finger_count: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            finger_count,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.three_finger_drag_support(finger_count: {})\n", client_id, id, arg0);
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
            14,
            arg0 as u32,
        ]);
        Ok(())
    }

    /// three finger drag support
    ///
    /// The number of fingers supported for three/four finger drag.
    /// If finger_count is less than 3, three finger drag is unsupported.
    ///
    /// # Arguments
    ///
    /// - `finger_count`:
    #[inline]
    pub fn send_three_finger_drag_support(
        &self,
        finger_count: i32,
    ) {
        let res = self.try_send_three_finger_drag_support(
            finger_count,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.three_finger_drag_support", &e);
        }
    }

    /// Since when the three_finger_drag_default message is available.
    pub const MSG__THREE_FINGER_DRAG_DEFAULT__SINCE: u32 = 1;

    /// default three finger drag state
    ///
    /// Default three finger drag state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_three_finger_drag_default(
        &self,
        state: RiverLibinputDeviceV1ThreeFingerDragState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1ThreeFingerDragState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.three_finger_drag_default(state: {:?})\n", client_id, id, arg0);
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
            15,
            arg0.0,
        ]);
        Ok(())
    }

    /// default three finger drag state
    ///
    /// Default three finger drag state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_three_finger_drag_default(
        &self,
        state: RiverLibinputDeviceV1ThreeFingerDragState,
    ) {
        let res = self.try_send_three_finger_drag_default(
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.three_finger_drag_default", &e);
        }
    }

    /// Since when the three_finger_drag_current message is available.
    pub const MSG__THREE_FINGER_DRAG_CURRENT__SINCE: u32 = 1;

    /// current three finger drag state
    ///
    /// Current three finger drag state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_three_finger_drag_current(
        &self,
        state: RiverLibinputDeviceV1ThreeFingerDragState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1ThreeFingerDragState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.three_finger_drag_current(state: {:?})\n", client_id, id, arg0);
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
            16,
            arg0.0,
        ]);
        Ok(())
    }

    /// current three finger drag state
    ///
    /// Current three finger drag state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_three_finger_drag_current(
        &self,
        state: RiverLibinputDeviceV1ThreeFingerDragState,
    ) {
        let res = self.try_send_three_finger_drag_current(
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.three_finger_drag_current", &e);
        }
    }

    /// Since when the set_three_finger_drag message is available.
    pub const MSG__SET_THREE_FINGER_DRAG__SINCE: u32 = 1;

    /// set three finger drag state
    ///
    /// Configure three finger drag functionality for the device.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    pub fn try_send_set_three_finger_drag(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1ThreeFingerDragState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            result,
            state,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("result", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1ThreeFingerDragState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_device_v1#{}.set_three_finger_drag(result: river_libinput_result_v1#{}, state: {:?})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            arg0_id,
            arg1.0,
        ]);
        Ok(())
    }

    /// set three finger drag state
    ///
    /// Configure three finger drag functionality for the device.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    pub fn send_set_three_finger_drag(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1ThreeFingerDragState,
    ) {
        let res = self.try_send_set_three_finger_drag(
            result,
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.set_three_finger_drag", &e);
        }
    }

    /// set three finger drag state
    ///
    /// Configure three finger drag functionality for the device.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn new_try_send_set_three_finger_drag(
        &self,
        state: RiverLibinputDeviceV1ThreeFingerDragState,
    ) -> Result<Rc<RiverLibinputResultV1>, ObjectError> {
        let result = self.core.create_child();
        self.try_send_set_three_finger_drag(
            &result,
            state,
        )?;
        Ok(result)
    }

    /// set three finger drag state
    ///
    /// Configure three finger drag functionality for the device.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn new_send_set_three_finger_drag(
        &self,
        state: RiverLibinputDeviceV1ThreeFingerDragState,
    ) -> Rc<RiverLibinputResultV1> {
        let result = self.core.create_child();
        self.send_set_three_finger_drag(
            &result,
            state,
        );
        result
    }

    /// Since when the calibration_matrix_support message is available.
    pub const MSG__CALIBRATION_MATRIX_SUPPORT__SINCE: u32 = 1;

    /// support for a calibration matrix
    ///
    /// A calibration matrix is supported if the supported argument is non-zero.
    ///
    /// # Arguments
    ///
    /// - `supported`: boolean
    #[inline]
    pub fn try_send_calibration_matrix_support(
        &self,
        supported: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            supported,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.calibration_matrix_support(supported: {})\n", client_id, id, arg0);
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
            17,
            arg0 as u32,
        ]);
        Ok(())
    }

    /// support for a calibration matrix
    ///
    /// A calibration matrix is supported if the supported argument is non-zero.
    ///
    /// # Arguments
    ///
    /// - `supported`: boolean
    #[inline]
    pub fn send_calibration_matrix_support(
        &self,
        supported: i32,
    ) {
        let res = self.try_send_calibration_matrix_support(
            supported,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.calibration_matrix_support", &e);
        }
    }

    /// Since when the calibration_matrix_default message is available.
    pub const MSG__CALIBRATION_MATRIX_DEFAULT__SINCE: u32 = 1;

    /// default calibration matrix
    ///
    /// Default calibration matrix.
    ///
    /// # Arguments
    ///
    /// - `matrix`: array of 6 floats
    #[inline]
    pub fn try_send_calibration_matrix_default(
        &self,
        matrix: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            matrix,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: &[u8]) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.calibration_matrix_default(matrix: {})\n", client_id, id, debug_array(arg0));
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
            18,
        ]);
        fmt.array(arg0);
        Ok(())
    }

    /// default calibration matrix
    ///
    /// Default calibration matrix.
    ///
    /// # Arguments
    ///
    /// - `matrix`: array of 6 floats
    #[inline]
    pub fn send_calibration_matrix_default(
        &self,
        matrix: &[u8],
    ) {
        let res = self.try_send_calibration_matrix_default(
            matrix,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.calibration_matrix_default", &e);
        }
    }

    /// Since when the calibration_matrix_current message is available.
    pub const MSG__CALIBRATION_MATRIX_CURRENT__SINCE: u32 = 1;

    /// current calibration matrix
    ///
    /// Current calibration matrix.
    ///
    /// # Arguments
    ///
    /// - `matrix`: array of 6 floats
    #[inline]
    pub fn try_send_calibration_matrix_current(
        &self,
        matrix: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            matrix,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: &[u8]) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.calibration_matrix_current(matrix: {})\n", client_id, id, debug_array(arg0));
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
            19,
        ]);
        fmt.array(arg0);
        Ok(())
    }

    /// current calibration matrix
    ///
    /// Current calibration matrix.
    ///
    /// # Arguments
    ///
    /// - `matrix`: array of 6 floats
    #[inline]
    pub fn send_calibration_matrix_current(
        &self,
        matrix: &[u8],
    ) {
        let res = self.try_send_calibration_matrix_current(
            matrix,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.calibration_matrix_current", &e);
        }
    }

    /// Since when the set_calibration_matrix message is available.
    pub const MSG__SET_CALIBRATION_MATRIX__SINCE: u32 = 1;

    /// set calibration matrix
    ///
    /// Set calibration matrix.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `matrix`: array of 6 floats
    #[inline]
    pub fn try_send_set_calibration_matrix(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        matrix: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            result,
            matrix,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("result", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: &[u8]) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_device_v1#{}.set_calibration_matrix(result: river_libinput_result_v1#{}, matrix: {})\n", id, arg0, debug_array(arg1));
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            arg0_id,
        ]);
        fmt.array(arg1);
        Ok(())
    }

    /// set calibration matrix
    ///
    /// Set calibration matrix.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `matrix`: array of 6 floats
    #[inline]
    pub fn send_set_calibration_matrix(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        matrix: &[u8],
    ) {
        let res = self.try_send_set_calibration_matrix(
            result,
            matrix,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.set_calibration_matrix", &e);
        }
    }

    /// set calibration matrix
    ///
    /// Set calibration matrix.
    ///
    /// # Arguments
    ///
    /// - `matrix`: array of 6 floats
    #[inline]
    pub fn new_try_send_set_calibration_matrix(
        &self,
        matrix: &[u8],
    ) -> Result<Rc<RiverLibinputResultV1>, ObjectError> {
        let result = self.core.create_child();
        self.try_send_set_calibration_matrix(
            &result,
            matrix,
        )?;
        Ok(result)
    }

    /// set calibration matrix
    ///
    /// Set calibration matrix.
    ///
    /// # Arguments
    ///
    /// - `matrix`: array of 6 floats
    #[inline]
    pub fn new_send_set_calibration_matrix(
        &self,
        matrix: &[u8],
    ) -> Rc<RiverLibinputResultV1> {
        let result = self.core.create_child();
        self.send_set_calibration_matrix(
            &result,
            matrix,
        );
        result
    }

    /// Since when the accel_profiles_support message is available.
    pub const MSG__ACCEL_PROFILES_SUPPORT__SINCE: u32 = 1;

    /// supported acceleration profiles
    ///
    /// Supported acceleration profiles.
    ///
    /// # Arguments
    ///
    /// - `profiles`:
    #[inline]
    pub fn try_send_accel_profiles_support(
        &self,
        profiles: RiverLibinputDeviceV1AccelProfiles,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            profiles,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1AccelProfiles) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.accel_profiles_support(profiles: {:?})\n", client_id, id, arg0);
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
            20,
            arg0.0,
        ]);
        Ok(())
    }

    /// supported acceleration profiles
    ///
    /// Supported acceleration profiles.
    ///
    /// # Arguments
    ///
    /// - `profiles`:
    #[inline]
    pub fn send_accel_profiles_support(
        &self,
        profiles: RiverLibinputDeviceV1AccelProfiles,
    ) {
        let res = self.try_send_accel_profiles_support(
            profiles,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.accel_profiles_support", &e);
        }
    }

    /// Since when the accel_profile_default message is available.
    pub const MSG__ACCEL_PROFILE_DEFAULT__SINCE: u32 = 1;

    /// default acceleration profile
    ///
    /// Default acceleration profile.
    ///
    /// # Arguments
    ///
    /// - `profile`:
    #[inline]
    pub fn try_send_accel_profile_default(
        &self,
        profile: RiverLibinputDeviceV1AccelProfile,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            profile,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1AccelProfile) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.accel_profile_default(profile: {:?})\n", client_id, id, arg0);
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
            21,
            arg0.0,
        ]);
        Ok(())
    }

    /// default acceleration profile
    ///
    /// Default acceleration profile.
    ///
    /// # Arguments
    ///
    /// - `profile`:
    #[inline]
    pub fn send_accel_profile_default(
        &self,
        profile: RiverLibinputDeviceV1AccelProfile,
    ) {
        let res = self.try_send_accel_profile_default(
            profile,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.accel_profile_default", &e);
        }
    }

    /// Since when the accel_profile_current message is available.
    pub const MSG__ACCEL_PROFILE_CURRENT__SINCE: u32 = 1;

    /// current send events mode
    ///
    /// Current acceleration profile.
    ///
    /// # Arguments
    ///
    /// - `profile`:
    #[inline]
    pub fn try_send_accel_profile_current(
        &self,
        profile: RiverLibinputDeviceV1AccelProfile,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            profile,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1AccelProfile) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.accel_profile_current(profile: {:?})\n", client_id, id, arg0);
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
            22,
            arg0.0,
        ]);
        Ok(())
    }

    /// current send events mode
    ///
    /// Current acceleration profile.
    ///
    /// # Arguments
    ///
    /// - `profile`:
    #[inline]
    pub fn send_accel_profile_current(
        &self,
        profile: RiverLibinputDeviceV1AccelProfile,
    ) {
        let res = self.try_send_accel_profile_current(
            profile,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.accel_profile_current", &e);
        }
    }

    /// Since when the set_accel_profile message is available.
    pub const MSG__SET_ACCEL_PROFILE__SINCE: u32 = 1;

    /// set send events mode
    ///
    /// Set the acceleration profile.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `profile`:
    #[inline]
    pub fn try_send_set_accel_profile(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        profile: RiverLibinputDeviceV1AccelProfile,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            result,
            profile,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("result", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1AccelProfile) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_device_v1#{}.set_accel_profile(result: river_libinput_result_v1#{}, profile: {:?})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            8,
            arg0_id,
            arg1.0,
        ]);
        Ok(())
    }

    /// set send events mode
    ///
    /// Set the acceleration profile.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `profile`:
    #[inline]
    pub fn send_set_accel_profile(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        profile: RiverLibinputDeviceV1AccelProfile,
    ) {
        let res = self.try_send_set_accel_profile(
            result,
            profile,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.set_accel_profile", &e);
        }
    }

    /// set send events mode
    ///
    /// Set the acceleration profile.
    ///
    /// # Arguments
    ///
    /// - `profile`:
    #[inline]
    pub fn new_try_send_set_accel_profile(
        &self,
        profile: RiverLibinputDeviceV1AccelProfile,
    ) -> Result<Rc<RiverLibinputResultV1>, ObjectError> {
        let result = self.core.create_child();
        self.try_send_set_accel_profile(
            &result,
            profile,
        )?;
        Ok(result)
    }

    /// set send events mode
    ///
    /// Set the acceleration profile.
    ///
    /// # Arguments
    ///
    /// - `profile`:
    #[inline]
    pub fn new_send_set_accel_profile(
        &self,
        profile: RiverLibinputDeviceV1AccelProfile,
    ) -> Rc<RiverLibinputResultV1> {
        let result = self.core.create_child();
        self.send_set_accel_profile(
            &result,
            profile,
        );
        result
    }

    /// Since when the accel_speed_default message is available.
    pub const MSG__ACCEL_SPEED_DEFAULT__SINCE: u32 = 1;

    /// default acceleration speed
    ///
    /// Default acceleration speed.
    ///
    /// # Arguments
    ///
    /// - `speed`: double
    #[inline]
    pub fn try_send_accel_speed_default(
        &self,
        speed: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            speed,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: &[u8]) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.accel_speed_default(speed: {})\n", client_id, id, debug_array(arg0));
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
            23,
        ]);
        fmt.array(arg0);
        Ok(())
    }

    /// default acceleration speed
    ///
    /// Default acceleration speed.
    ///
    /// # Arguments
    ///
    /// - `speed`: double
    #[inline]
    pub fn send_accel_speed_default(
        &self,
        speed: &[u8],
    ) {
        let res = self.try_send_accel_speed_default(
            speed,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.accel_speed_default", &e);
        }
    }

    /// Since when the accel_speed_current message is available.
    pub const MSG__ACCEL_SPEED_CURRENT__SINCE: u32 = 1;

    /// current acceleration speed
    ///
    /// Current acceleration speed.
    ///
    /// # Arguments
    ///
    /// - `speed`: double
    #[inline]
    pub fn try_send_accel_speed_current(
        &self,
        speed: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            speed,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: &[u8]) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.accel_speed_current(speed: {})\n", client_id, id, debug_array(arg0));
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
            24,
        ]);
        fmt.array(arg0);
        Ok(())
    }

    /// current acceleration speed
    ///
    /// Current acceleration speed.
    ///
    /// # Arguments
    ///
    /// - `speed`: double
    #[inline]
    pub fn send_accel_speed_current(
        &self,
        speed: &[u8],
    ) {
        let res = self.try_send_accel_speed_current(
            speed,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.accel_speed_current", &e);
        }
    }

    /// Since when the set_accel_speed message is available.
    pub const MSG__SET_ACCEL_SPEED__SINCE: u32 = 1;

    /// set acceleration speed
    ///
    /// Set the acceleration speed within a range of [-1, 1], where 0 is
    /// the default acceleration for this device, -1 is the slowest acceleration
    /// and 1 is the maximum acceleration available on this device.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `speed`: double
    #[inline]
    pub fn try_send_set_accel_speed(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        speed: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            result,
            speed,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("result", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: &[u8]) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_device_v1#{}.set_accel_speed(result: river_libinput_result_v1#{}, speed: {})\n", id, arg0, debug_array(arg1));
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            9,
            arg0_id,
        ]);
        fmt.array(arg1);
        Ok(())
    }

    /// set acceleration speed
    ///
    /// Set the acceleration speed within a range of [-1, 1], where 0 is
    /// the default acceleration for this device, -1 is the slowest acceleration
    /// and 1 is the maximum acceleration available on this device.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `speed`: double
    #[inline]
    pub fn send_set_accel_speed(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        speed: &[u8],
    ) {
        let res = self.try_send_set_accel_speed(
            result,
            speed,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.set_accel_speed", &e);
        }
    }

    /// set acceleration speed
    ///
    /// Set the acceleration speed within a range of [-1, 1], where 0 is
    /// the default acceleration for this device, -1 is the slowest acceleration
    /// and 1 is the maximum acceleration available on this device.
    ///
    /// # Arguments
    ///
    /// - `speed`: double
    #[inline]
    pub fn new_try_send_set_accel_speed(
        &self,
        speed: &[u8],
    ) -> Result<Rc<RiverLibinputResultV1>, ObjectError> {
        let result = self.core.create_child();
        self.try_send_set_accel_speed(
            &result,
            speed,
        )?;
        Ok(result)
    }

    /// set acceleration speed
    ///
    /// Set the acceleration speed within a range of [-1, 1], where 0 is
    /// the default acceleration for this device, -1 is the slowest acceleration
    /// and 1 is the maximum acceleration available on this device.
    ///
    /// # Arguments
    ///
    /// - `speed`: double
    #[inline]
    pub fn new_send_set_accel_speed(
        &self,
        speed: &[u8],
    ) -> Rc<RiverLibinputResultV1> {
        let result = self.core.create_child();
        self.send_set_accel_speed(
            &result,
            speed,
        );
        result
    }

    /// Since when the apply_accel_config message is available.
    pub const MSG__APPLY_ACCEL_CONFIG__SINCE: u32 = 1;

    /// apply acceleration config
    ///
    /// Apply a pointer accleration config.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `config`:
    #[inline]
    pub fn try_send_apply_accel_config(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        config: &Rc<RiverLibinputAccelConfigV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            result,
            config,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("config"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("result", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_device_v1#{}.apply_accel_config(result: river_libinput_result_v1#{}, config: river_libinput_accel_config_v1#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id);
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
            10,
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// apply acceleration config
    ///
    /// Apply a pointer accleration config.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `config`:
    #[inline]
    pub fn send_apply_accel_config(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        config: &Rc<RiverLibinputAccelConfigV1>,
    ) {
        let res = self.try_send_apply_accel_config(
            result,
            config,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.apply_accel_config", &e);
        }
    }

    /// apply acceleration config
    ///
    /// Apply a pointer accleration config.
    ///
    /// # Arguments
    ///
    /// - `config`:
    #[inline]
    pub fn new_try_send_apply_accel_config(
        &self,
        config: &Rc<RiverLibinputAccelConfigV1>,
    ) -> Result<Rc<RiverLibinputResultV1>, ObjectError> {
        let result = self.core.create_child();
        self.try_send_apply_accel_config(
            &result,
            config,
        )?;
        Ok(result)
    }

    /// apply acceleration config
    ///
    /// Apply a pointer accleration config.
    ///
    /// # Arguments
    ///
    /// - `config`:
    #[inline]
    pub fn new_send_apply_accel_config(
        &self,
        config: &Rc<RiverLibinputAccelConfigV1>,
    ) -> Rc<RiverLibinputResultV1> {
        let result = self.core.create_child();
        self.send_apply_accel_config(
            &result,
            config,
        );
        result
    }

    /// Since when the natural_scroll_support message is available.
    pub const MSG__NATURAL_SCROLL_SUPPORT__SINCE: u32 = 1;

    /// support for natural scroll
    ///
    /// Natural scroll is supported if the supported argument is non-zero.
    ///
    /// # Arguments
    ///
    /// - `supported`: boolean
    #[inline]
    pub fn try_send_natural_scroll_support(
        &self,
        supported: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            supported,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.natural_scroll_support(supported: {})\n", client_id, id, arg0);
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
            25,
            arg0 as u32,
        ]);
        Ok(())
    }

    /// support for natural scroll
    ///
    /// Natural scroll is supported if the supported argument is non-zero.
    ///
    /// # Arguments
    ///
    /// - `supported`: boolean
    #[inline]
    pub fn send_natural_scroll_support(
        &self,
        supported: i32,
    ) {
        let res = self.try_send_natural_scroll_support(
            supported,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.natural_scroll_support", &e);
        }
    }

    /// Since when the natural_scroll_default message is available.
    pub const MSG__NATURAL_SCROLL_DEFAULT__SINCE: u32 = 1;

    /// default natural scroll
    ///
    /// Default natural scroll.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_natural_scroll_default(
        &self,
        state: RiverLibinputDeviceV1NaturalScrollState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1NaturalScrollState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.natural_scroll_default(state: {:?})\n", client_id, id, arg0);
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
            26,
            arg0.0,
        ]);
        Ok(())
    }

    /// default natural scroll
    ///
    /// Default natural scroll.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_natural_scroll_default(
        &self,
        state: RiverLibinputDeviceV1NaturalScrollState,
    ) {
        let res = self.try_send_natural_scroll_default(
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.natural_scroll_default", &e);
        }
    }

    /// Since when the natural_scroll_current message is available.
    pub const MSG__NATURAL_SCROLL_CURRENT__SINCE: u32 = 1;

    /// current natural scroll state
    ///
    /// Current natural scroll.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_natural_scroll_current(
        &self,
        state: RiverLibinputDeviceV1NaturalScrollState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1NaturalScrollState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.natural_scroll_current(state: {:?})\n", client_id, id, arg0);
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
            27,
            arg0.0,
        ]);
        Ok(())
    }

    /// current natural scroll state
    ///
    /// Current natural scroll.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_natural_scroll_current(
        &self,
        state: RiverLibinputDeviceV1NaturalScrollState,
    ) {
        let res = self.try_send_natural_scroll_current(
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.natural_scroll_current", &e);
        }
    }

    /// Since when the set_natural_scroll message is available.
    pub const MSG__SET_NATURAL_SCROLL__SINCE: u32 = 1;

    /// set natural scroll state
    ///
    /// Set natural scroll state.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    pub fn try_send_set_natural_scroll(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1NaturalScrollState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            result,
            state,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("result", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1NaturalScrollState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_device_v1#{}.set_natural_scroll(result: river_libinput_result_v1#{}, state: {:?})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            11,
            arg0_id,
            arg1.0,
        ]);
        Ok(())
    }

    /// set natural scroll state
    ///
    /// Set natural scroll state.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    pub fn send_set_natural_scroll(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1NaturalScrollState,
    ) {
        let res = self.try_send_set_natural_scroll(
            result,
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.set_natural_scroll", &e);
        }
    }

    /// set natural scroll state
    ///
    /// Set natural scroll state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn new_try_send_set_natural_scroll(
        &self,
        state: RiverLibinputDeviceV1NaturalScrollState,
    ) -> Result<Rc<RiverLibinputResultV1>, ObjectError> {
        let result = self.core.create_child();
        self.try_send_set_natural_scroll(
            &result,
            state,
        )?;
        Ok(result)
    }

    /// set natural scroll state
    ///
    /// Set natural scroll state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn new_send_set_natural_scroll(
        &self,
        state: RiverLibinputDeviceV1NaturalScrollState,
    ) -> Rc<RiverLibinputResultV1> {
        let result = self.core.create_child();
        self.send_set_natural_scroll(
            &result,
            state,
        );
        result
    }

    /// Since when the left_handed_support message is available.
    pub const MSG__LEFT_HANDED_SUPPORT__SINCE: u32 = 1;

    /// support for left-handed mode
    ///
    /// Left-handed mode is supported if the supported argument is non-zero.
    ///
    /// # Arguments
    ///
    /// - `supported`: boolean
    #[inline]
    pub fn try_send_left_handed_support(
        &self,
        supported: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            supported,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.left_handed_support(supported: {})\n", client_id, id, arg0);
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
            28,
            arg0 as u32,
        ]);
        Ok(())
    }

    /// support for left-handed mode
    ///
    /// Left-handed mode is supported if the supported argument is non-zero.
    ///
    /// # Arguments
    ///
    /// - `supported`: boolean
    #[inline]
    pub fn send_left_handed_support(
        &self,
        supported: i32,
    ) {
        let res = self.try_send_left_handed_support(
            supported,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.left_handed_support", &e);
        }
    }

    /// Since when the left_handed_default message is available.
    pub const MSG__LEFT_HANDED_DEFAULT__SINCE: u32 = 1;

    /// default left-handed mode
    ///
    /// Default left-handed mode.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_left_handed_default(
        &self,
        state: RiverLibinputDeviceV1LeftHandedState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1LeftHandedState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.left_handed_default(state: {:?})\n", client_id, id, arg0);
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
            29,
            arg0.0,
        ]);
        Ok(())
    }

    /// default left-handed mode
    ///
    /// Default left-handed mode.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_left_handed_default(
        &self,
        state: RiverLibinputDeviceV1LeftHandedState,
    ) {
        let res = self.try_send_left_handed_default(
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.left_handed_default", &e);
        }
    }

    /// Since when the left_handed_current message is available.
    pub const MSG__LEFT_HANDED_CURRENT__SINCE: u32 = 1;

    /// current left-handed mode state
    ///
    /// Current left-handed mode.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_left_handed_current(
        &self,
        state: RiverLibinputDeviceV1LeftHandedState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1LeftHandedState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.left_handed_current(state: {:?})\n", client_id, id, arg0);
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
            30,
            arg0.0,
        ]);
        Ok(())
    }

    /// current left-handed mode state
    ///
    /// Current left-handed mode.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_left_handed_current(
        &self,
        state: RiverLibinputDeviceV1LeftHandedState,
    ) {
        let res = self.try_send_left_handed_current(
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.left_handed_current", &e);
        }
    }

    /// Since when the set_left_handed message is available.
    pub const MSG__SET_LEFT_HANDED__SINCE: u32 = 1;

    /// set left-handed mode state
    ///
    /// Set left-handed mode state.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    pub fn try_send_set_left_handed(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1LeftHandedState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            result,
            state,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("result", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1LeftHandedState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_device_v1#{}.set_left_handed(result: river_libinput_result_v1#{}, state: {:?})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            12,
            arg0_id,
            arg1.0,
        ]);
        Ok(())
    }

    /// set left-handed mode state
    ///
    /// Set left-handed mode state.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    pub fn send_set_left_handed(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1LeftHandedState,
    ) {
        let res = self.try_send_set_left_handed(
            result,
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.set_left_handed", &e);
        }
    }

    /// set left-handed mode state
    ///
    /// Set left-handed mode state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn new_try_send_set_left_handed(
        &self,
        state: RiverLibinputDeviceV1LeftHandedState,
    ) -> Result<Rc<RiverLibinputResultV1>, ObjectError> {
        let result = self.core.create_child();
        self.try_send_set_left_handed(
            &result,
            state,
        )?;
        Ok(result)
    }

    /// set left-handed mode state
    ///
    /// Set left-handed mode state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn new_send_set_left_handed(
        &self,
        state: RiverLibinputDeviceV1LeftHandedState,
    ) -> Rc<RiverLibinputResultV1> {
        let result = self.core.create_child();
        self.send_set_left_handed(
            &result,
            state,
        );
        result
    }

    /// Since when the click_method_support message is available.
    pub const MSG__CLICK_METHOD_SUPPORT__SINCE: u32 = 1;

    /// supported click methods
    ///
    /// The click methods suppported by the device.
    ///
    /// # Arguments
    ///
    /// - `methods`:
    #[inline]
    pub fn try_send_click_method_support(
        &self,
        methods: RiverLibinputDeviceV1ClickMethods,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            methods,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1ClickMethods) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.click_method_support(methods: {:?})\n", client_id, id, arg0);
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
            31,
            arg0.0,
        ]);
        Ok(())
    }

    /// supported click methods
    ///
    /// The click methods suppported by the device.
    ///
    /// # Arguments
    ///
    /// - `methods`:
    #[inline]
    pub fn send_click_method_support(
        &self,
        methods: RiverLibinputDeviceV1ClickMethods,
    ) {
        let res = self.try_send_click_method_support(
            methods,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.click_method_support", &e);
        }
    }

    /// Since when the click_method_default message is available.
    pub const MSG__CLICK_METHOD_DEFAULT__SINCE: u32 = 1;

    /// default click method
    ///
    /// Default click method.
    ///
    /// # Arguments
    ///
    /// - `method`:
    #[inline]
    pub fn try_send_click_method_default(
        &self,
        method: RiverLibinputDeviceV1ClickMethod,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            method,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1ClickMethod) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.click_method_default(method: {:?})\n", client_id, id, arg0);
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
            32,
            arg0.0,
        ]);
        Ok(())
    }

    /// default click method
    ///
    /// Default click method.
    ///
    /// # Arguments
    ///
    /// - `method`:
    #[inline]
    pub fn send_click_method_default(
        &self,
        method: RiverLibinputDeviceV1ClickMethod,
    ) {
        let res = self.try_send_click_method_default(
            method,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.click_method_default", &e);
        }
    }

    /// Since when the click_method_current message is available.
    pub const MSG__CLICK_METHOD_CURRENT__SINCE: u32 = 1;

    /// current click method
    ///
    /// Current click method.
    ///
    /// # Arguments
    ///
    /// - `method`:
    #[inline]
    pub fn try_send_click_method_current(
        &self,
        method: RiverLibinputDeviceV1ClickMethod,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            method,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1ClickMethod) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.click_method_current(method: {:?})\n", client_id, id, arg0);
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
            33,
            arg0.0,
        ]);
        Ok(())
    }

    /// current click method
    ///
    /// Current click method.
    ///
    /// # Arguments
    ///
    /// - `method`:
    #[inline]
    pub fn send_click_method_current(
        &self,
        method: RiverLibinputDeviceV1ClickMethod,
    ) {
        let res = self.try_send_click_method_current(
            method,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.click_method_current", &e);
        }
    }

    /// Since when the set_click_method message is available.
    pub const MSG__SET_CLICK_METHOD__SINCE: u32 = 1;

    /// set click method
    ///
    /// Set click method.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `method`:
    #[inline]
    pub fn try_send_set_click_method(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        method: RiverLibinputDeviceV1ClickMethod,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            result,
            method,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("result", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1ClickMethod) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_device_v1#{}.set_click_method(result: river_libinput_result_v1#{}, method: {:?})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            13,
            arg0_id,
            arg1.0,
        ]);
        Ok(())
    }

    /// set click method
    ///
    /// Set click method.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `method`:
    #[inline]
    pub fn send_set_click_method(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        method: RiverLibinputDeviceV1ClickMethod,
    ) {
        let res = self.try_send_set_click_method(
            result,
            method,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.set_click_method", &e);
        }
    }

    /// set click method
    ///
    /// Set click method.
    ///
    /// # Arguments
    ///
    /// - `method`:
    #[inline]
    pub fn new_try_send_set_click_method(
        &self,
        method: RiverLibinputDeviceV1ClickMethod,
    ) -> Result<Rc<RiverLibinputResultV1>, ObjectError> {
        let result = self.core.create_child();
        self.try_send_set_click_method(
            &result,
            method,
        )?;
        Ok(result)
    }

    /// set click method
    ///
    /// Set click method.
    ///
    /// # Arguments
    ///
    /// - `method`:
    #[inline]
    pub fn new_send_set_click_method(
        &self,
        method: RiverLibinputDeviceV1ClickMethod,
    ) -> Rc<RiverLibinputResultV1> {
        let result = self.core.create_child();
        self.send_set_click_method(
            &result,
            method,
        );
        result
    }

    /// Since when the clickfinger_button_map_default message is available.
    pub const MSG__CLICKFINGER_BUTTON_MAP_DEFAULT__SINCE: u32 = 1;

    /// default clickfinger button map
    ///
    /// Default clickfinger button map.
    /// Supported if click_methods.clickfinger is supported.
    ///
    /// # Arguments
    ///
    /// - `button_map`:
    #[inline]
    pub fn try_send_clickfinger_button_map_default(
        &self,
        button_map: RiverLibinputDeviceV1ClickfingerButtonMap,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            button_map,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1ClickfingerButtonMap) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.clickfinger_button_map_default(button_map: {:?})\n", client_id, id, arg0);
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
            34,
            arg0.0,
        ]);
        Ok(())
    }

    /// default clickfinger button map
    ///
    /// Default clickfinger button map.
    /// Supported if click_methods.clickfinger is supported.
    ///
    /// # Arguments
    ///
    /// - `button_map`:
    #[inline]
    pub fn send_clickfinger_button_map_default(
        &self,
        button_map: RiverLibinputDeviceV1ClickfingerButtonMap,
    ) {
        let res = self.try_send_clickfinger_button_map_default(
            button_map,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.clickfinger_button_map_default", &e);
        }
    }

    /// Since when the clickfinger_button_map_current message is available.
    pub const MSG__CLICKFINGER_BUTTON_MAP_CURRENT__SINCE: u32 = 1;

    /// current clickfinger button map
    ///
    /// Current clickfinger button map.
    /// Supported if click_methods.clickfinger is supported.
    ///
    /// # Arguments
    ///
    /// - `button_map`:
    #[inline]
    pub fn try_send_clickfinger_button_map_current(
        &self,
        button_map: RiverLibinputDeviceV1ClickfingerButtonMap,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            button_map,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1ClickfingerButtonMap) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.clickfinger_button_map_current(button_map: {:?})\n", client_id, id, arg0);
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
            35,
            arg0.0,
        ]);
        Ok(())
    }

    /// current clickfinger button map
    ///
    /// Current clickfinger button map.
    /// Supported if click_methods.clickfinger is supported.
    ///
    /// # Arguments
    ///
    /// - `button_map`:
    #[inline]
    pub fn send_clickfinger_button_map_current(
        &self,
        button_map: RiverLibinputDeviceV1ClickfingerButtonMap,
    ) {
        let res = self.try_send_clickfinger_button_map_current(
            button_map,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.clickfinger_button_map_current", &e);
        }
    }

    /// Since when the set_clickfinger_button_map message is available.
    pub const MSG__SET_CLICKFINGER_BUTTON_MAP__SINCE: u32 = 1;

    /// set clickfinger button map
    ///
    /// Set clickfinger button map.
    /// Supported if click_methods.clickfinger is supported.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `button_map`:
    #[inline]
    pub fn try_send_set_clickfinger_button_map(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        button_map: RiverLibinputDeviceV1ClickfingerButtonMap,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            result,
            button_map,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("result", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1ClickfingerButtonMap) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_device_v1#{}.set_clickfinger_button_map(result: river_libinput_result_v1#{}, button_map: {:?})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            14,
            arg0_id,
            arg1.0,
        ]);
        Ok(())
    }

    /// set clickfinger button map
    ///
    /// Set clickfinger button map.
    /// Supported if click_methods.clickfinger is supported.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `button_map`:
    #[inline]
    pub fn send_set_clickfinger_button_map(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        button_map: RiverLibinputDeviceV1ClickfingerButtonMap,
    ) {
        let res = self.try_send_set_clickfinger_button_map(
            result,
            button_map,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.set_clickfinger_button_map", &e);
        }
    }

    /// set clickfinger button map
    ///
    /// Set clickfinger button map.
    /// Supported if click_methods.clickfinger is supported.
    ///
    /// # Arguments
    ///
    /// - `button_map`:
    #[inline]
    pub fn new_try_send_set_clickfinger_button_map(
        &self,
        button_map: RiverLibinputDeviceV1ClickfingerButtonMap,
    ) -> Result<Rc<RiverLibinputResultV1>, ObjectError> {
        let result = self.core.create_child();
        self.try_send_set_clickfinger_button_map(
            &result,
            button_map,
        )?;
        Ok(result)
    }

    /// set clickfinger button map
    ///
    /// Set clickfinger button map.
    /// Supported if click_methods.clickfinger is supported.
    ///
    /// # Arguments
    ///
    /// - `button_map`:
    #[inline]
    pub fn new_send_set_clickfinger_button_map(
        &self,
        button_map: RiverLibinputDeviceV1ClickfingerButtonMap,
    ) -> Rc<RiverLibinputResultV1> {
        let result = self.core.create_child();
        self.send_set_clickfinger_button_map(
            &result,
            button_map,
        );
        result
    }

    /// Since when the middle_emulation_support message is available.
    pub const MSG__MIDDLE_EMULATION_SUPPORT__SINCE: u32 = 1;

    /// support for middle mouse button emulation
    ///
    /// Middle mouse button emulation is supported if the supported argument is
    /// non-zero.
    ///
    /// # Arguments
    ///
    /// - `supported`: boolean
    #[inline]
    pub fn try_send_middle_emulation_support(
        &self,
        supported: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            supported,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.middle_emulation_support(supported: {})\n", client_id, id, arg0);
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
            36,
            arg0 as u32,
        ]);
        Ok(())
    }

    /// support for middle mouse button emulation
    ///
    /// Middle mouse button emulation is supported if the supported argument is
    /// non-zero.
    ///
    /// # Arguments
    ///
    /// - `supported`: boolean
    #[inline]
    pub fn send_middle_emulation_support(
        &self,
        supported: i32,
    ) {
        let res = self.try_send_middle_emulation_support(
            supported,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.middle_emulation_support", &e);
        }
    }

    /// Since when the middle_emulation_default message is available.
    pub const MSG__MIDDLE_EMULATION_DEFAULT__SINCE: u32 = 1;

    /// default middle mouse button emulation
    ///
    /// Default middle mouse button emulation.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_middle_emulation_default(
        &self,
        state: RiverLibinputDeviceV1MiddleEmulationState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1MiddleEmulationState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.middle_emulation_default(state: {:?})\n", client_id, id, arg0);
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
            37,
            arg0.0,
        ]);
        Ok(())
    }

    /// default middle mouse button emulation
    ///
    /// Default middle mouse button emulation.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_middle_emulation_default(
        &self,
        state: RiverLibinputDeviceV1MiddleEmulationState,
    ) {
        let res = self.try_send_middle_emulation_default(
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.middle_emulation_default", &e);
        }
    }

    /// Since when the middle_emulation_current message is available.
    pub const MSG__MIDDLE_EMULATION_CURRENT__SINCE: u32 = 1;

    /// current middle mouse button emulation state
    ///
    /// Current middle mouse button emulation.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_middle_emulation_current(
        &self,
        state: RiverLibinputDeviceV1MiddleEmulationState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1MiddleEmulationState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.middle_emulation_current(state: {:?})\n", client_id, id, arg0);
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
            38,
            arg0.0,
        ]);
        Ok(())
    }

    /// current middle mouse button emulation state
    ///
    /// Current middle mouse button emulation.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_middle_emulation_current(
        &self,
        state: RiverLibinputDeviceV1MiddleEmulationState,
    ) {
        let res = self.try_send_middle_emulation_current(
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.middle_emulation_current", &e);
        }
    }

    /// Since when the set_middle_emulation message is available.
    pub const MSG__SET_MIDDLE_EMULATION__SINCE: u32 = 1;

    /// set middle mouse button emulation state
    ///
    /// Set middle mouse button emulation state.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    pub fn try_send_set_middle_emulation(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1MiddleEmulationState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            result,
            state,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("result", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1MiddleEmulationState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_device_v1#{}.set_middle_emulation(result: river_libinput_result_v1#{}, state: {:?})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            15,
            arg0_id,
            arg1.0,
        ]);
        Ok(())
    }

    /// set middle mouse button emulation state
    ///
    /// Set middle mouse button emulation state.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    pub fn send_set_middle_emulation(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1MiddleEmulationState,
    ) {
        let res = self.try_send_set_middle_emulation(
            result,
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.set_middle_emulation", &e);
        }
    }

    /// set middle mouse button emulation state
    ///
    /// Set middle mouse button emulation state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn new_try_send_set_middle_emulation(
        &self,
        state: RiverLibinputDeviceV1MiddleEmulationState,
    ) -> Result<Rc<RiverLibinputResultV1>, ObjectError> {
        let result = self.core.create_child();
        self.try_send_set_middle_emulation(
            &result,
            state,
        )?;
        Ok(result)
    }

    /// set middle mouse button emulation state
    ///
    /// Set middle mouse button emulation state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn new_send_set_middle_emulation(
        &self,
        state: RiverLibinputDeviceV1MiddleEmulationState,
    ) -> Rc<RiverLibinputResultV1> {
        let result = self.core.create_child();
        self.send_set_middle_emulation(
            &result,
            state,
        );
        result
    }

    /// Since when the scroll_method_support message is available.
    pub const MSG__SCROLL_METHOD_SUPPORT__SINCE: u32 = 1;

    /// supported scroll methods
    ///
    /// The scroll methods suppported by the device.
    ///
    /// # Arguments
    ///
    /// - `methods`:
    #[inline]
    pub fn try_send_scroll_method_support(
        &self,
        methods: RiverLibinputDeviceV1ScrollMethods,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            methods,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1ScrollMethods) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.scroll_method_support(methods: {:?})\n", client_id, id, arg0);
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
            39,
            arg0.0,
        ]);
        Ok(())
    }

    /// supported scroll methods
    ///
    /// The scroll methods suppported by the device.
    ///
    /// # Arguments
    ///
    /// - `methods`:
    #[inline]
    pub fn send_scroll_method_support(
        &self,
        methods: RiverLibinputDeviceV1ScrollMethods,
    ) {
        let res = self.try_send_scroll_method_support(
            methods,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.scroll_method_support", &e);
        }
    }

    /// Since when the scroll_method_default message is available.
    pub const MSG__SCROLL_METHOD_DEFAULT__SINCE: u32 = 1;

    /// default scroll method
    ///
    /// Default scroll method.
    ///
    /// # Arguments
    ///
    /// - `method`:
    #[inline]
    pub fn try_send_scroll_method_default(
        &self,
        method: RiverLibinputDeviceV1ScrollMethod,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            method,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1ScrollMethod) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.scroll_method_default(method: {:?})\n", client_id, id, arg0);
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
            40,
            arg0.0,
        ]);
        Ok(())
    }

    /// default scroll method
    ///
    /// Default scroll method.
    ///
    /// # Arguments
    ///
    /// - `method`:
    #[inline]
    pub fn send_scroll_method_default(
        &self,
        method: RiverLibinputDeviceV1ScrollMethod,
    ) {
        let res = self.try_send_scroll_method_default(
            method,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.scroll_method_default", &e);
        }
    }

    /// Since when the scroll_method_current message is available.
    pub const MSG__SCROLL_METHOD_CURRENT__SINCE: u32 = 1;

    /// current scroll method
    ///
    /// Current scroll method.
    ///
    /// # Arguments
    ///
    /// - `method`:
    #[inline]
    pub fn try_send_scroll_method_current(
        &self,
        method: RiverLibinputDeviceV1ScrollMethod,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            method,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1ScrollMethod) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.scroll_method_current(method: {:?})\n", client_id, id, arg0);
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
            41,
            arg0.0,
        ]);
        Ok(())
    }

    /// current scroll method
    ///
    /// Current scroll method.
    ///
    /// # Arguments
    ///
    /// - `method`:
    #[inline]
    pub fn send_scroll_method_current(
        &self,
        method: RiverLibinputDeviceV1ScrollMethod,
    ) {
        let res = self.try_send_scroll_method_current(
            method,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.scroll_method_current", &e);
        }
    }

    /// Since when the set_scroll_method message is available.
    pub const MSG__SET_SCROLL_METHOD__SINCE: u32 = 1;

    /// set scroll method
    ///
    /// Set scroll method.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `method`:
    #[inline]
    pub fn try_send_set_scroll_method(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        method: RiverLibinputDeviceV1ScrollMethod,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            result,
            method,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("result", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1ScrollMethod) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_device_v1#{}.set_scroll_method(result: river_libinput_result_v1#{}, method: {:?})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            16,
            arg0_id,
            arg1.0,
        ]);
        Ok(())
    }

    /// set scroll method
    ///
    /// Set scroll method.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `method`:
    #[inline]
    pub fn send_set_scroll_method(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        method: RiverLibinputDeviceV1ScrollMethod,
    ) {
        let res = self.try_send_set_scroll_method(
            result,
            method,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.set_scroll_method", &e);
        }
    }

    /// set scroll method
    ///
    /// Set scroll method.
    ///
    /// # Arguments
    ///
    /// - `method`:
    #[inline]
    pub fn new_try_send_set_scroll_method(
        &self,
        method: RiverLibinputDeviceV1ScrollMethod,
    ) -> Result<Rc<RiverLibinputResultV1>, ObjectError> {
        let result = self.core.create_child();
        self.try_send_set_scroll_method(
            &result,
            method,
        )?;
        Ok(result)
    }

    /// set scroll method
    ///
    /// Set scroll method.
    ///
    /// # Arguments
    ///
    /// - `method`:
    #[inline]
    pub fn new_send_set_scroll_method(
        &self,
        method: RiverLibinputDeviceV1ScrollMethod,
    ) -> Rc<RiverLibinputResultV1> {
        let result = self.core.create_child();
        self.send_set_scroll_method(
            &result,
            method,
        );
        result
    }

    /// Since when the scroll_button_default message is available.
    pub const MSG__SCROLL_BUTTON_DEFAULT__SINCE: u32 = 1;

    /// default scroll button
    ///
    /// Default scroll button.
    /// Supported if scroll_methods.on_button_down is supported.
    ///
    /// # Arguments
    ///
    /// - `button`:
    #[inline]
    pub fn try_send_scroll_button_default(
        &self,
        button: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            button,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.scroll_button_default(button: {})\n", client_id, id, arg0);
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
            42,
            arg0,
        ]);
        Ok(())
    }

    /// default scroll button
    ///
    /// Default scroll button.
    /// Supported if scroll_methods.on_button_down is supported.
    ///
    /// # Arguments
    ///
    /// - `button`:
    #[inline]
    pub fn send_scroll_button_default(
        &self,
        button: u32,
    ) {
        let res = self.try_send_scroll_button_default(
            button,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.scroll_button_default", &e);
        }
    }

    /// Since when the scroll_button_current message is available.
    pub const MSG__SCROLL_BUTTON_CURRENT__SINCE: u32 = 1;

    /// current scroll button
    ///
    /// Current scroll button.
    /// Supported if scroll_methods.on_button_down is supported.
    ///
    /// # Arguments
    ///
    /// - `button`:
    #[inline]
    pub fn try_send_scroll_button_current(
        &self,
        button: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            button,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.scroll_button_current(button: {})\n", client_id, id, arg0);
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
            43,
            arg0,
        ]);
        Ok(())
    }

    /// current scroll button
    ///
    /// Current scroll button.
    /// Supported if scroll_methods.on_button_down is supported.
    ///
    /// # Arguments
    ///
    /// - `button`:
    #[inline]
    pub fn send_scroll_button_current(
        &self,
        button: u32,
    ) {
        let res = self.try_send_scroll_button_current(
            button,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.scroll_button_current", &e);
        }
    }

    /// Since when the set_scroll_button message is available.
    pub const MSG__SET_SCROLL_BUTTON__SINCE: u32 = 1;

    /// set scroll button
    ///
    /// Set scroll button.
    /// Supported if scroll_methods.on_button_down is supported.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `button`:
    #[inline]
    pub fn try_send_set_scroll_button(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        button: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            result,
            button,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("result", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_device_v1#{}.set_scroll_button(result: river_libinput_result_v1#{}, button: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            17,
            arg0_id,
            arg1,
        ]);
        Ok(())
    }

    /// set scroll button
    ///
    /// Set scroll button.
    /// Supported if scroll_methods.on_button_down is supported.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `button`:
    #[inline]
    pub fn send_set_scroll_button(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        button: u32,
    ) {
        let res = self.try_send_set_scroll_button(
            result,
            button,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.set_scroll_button", &e);
        }
    }

    /// set scroll button
    ///
    /// Set scroll button.
    /// Supported if scroll_methods.on_button_down is supported.
    ///
    /// # Arguments
    ///
    /// - `button`:
    #[inline]
    pub fn new_try_send_set_scroll_button(
        &self,
        button: u32,
    ) -> Result<Rc<RiverLibinputResultV1>, ObjectError> {
        let result = self.core.create_child();
        self.try_send_set_scroll_button(
            &result,
            button,
        )?;
        Ok(result)
    }

    /// set scroll button
    ///
    /// Set scroll button.
    /// Supported if scroll_methods.on_button_down is supported.
    ///
    /// # Arguments
    ///
    /// - `button`:
    #[inline]
    pub fn new_send_set_scroll_button(
        &self,
        button: u32,
    ) -> Rc<RiverLibinputResultV1> {
        let result = self.core.create_child();
        self.send_set_scroll_button(
            &result,
            button,
        );
        result
    }

    /// Since when the scroll_button_lock_default message is available.
    pub const MSG__SCROLL_BUTTON_LOCK_DEFAULT__SINCE: u32 = 1;

    /// default scroll button lock state
    ///
    /// Default scroll button lock state.
    /// Supported if scroll_methods.on_button_down is supported.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_scroll_button_lock_default(
        &self,
        state: RiverLibinputDeviceV1ScrollButtonLockState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1ScrollButtonLockState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.scroll_button_lock_default(state: {:?})\n", client_id, id, arg0);
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
            44,
            arg0.0,
        ]);
        Ok(())
    }

    /// default scroll button lock state
    ///
    /// Default scroll button lock state.
    /// Supported if scroll_methods.on_button_down is supported.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_scroll_button_lock_default(
        &self,
        state: RiverLibinputDeviceV1ScrollButtonLockState,
    ) {
        let res = self.try_send_scroll_button_lock_default(
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.scroll_button_lock_default", &e);
        }
    }

    /// Since when the scroll_button_lock_current message is available.
    pub const MSG__SCROLL_BUTTON_LOCK_CURRENT__SINCE: u32 = 1;

    /// current scroll button lock state
    ///
    /// Current scroll button lock state.
    /// Supported if scroll_methods.on_button_down is supported.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_scroll_button_lock_current(
        &self,
        state: RiverLibinputDeviceV1ScrollButtonLockState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1ScrollButtonLockState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.scroll_button_lock_current(state: {:?})\n", client_id, id, arg0);
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
            45,
            arg0.0,
        ]);
        Ok(())
    }

    /// current scroll button lock state
    ///
    /// Current scroll button lock state.
    /// Supported if scroll_methods.on_button_down is supported.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_scroll_button_lock_current(
        &self,
        state: RiverLibinputDeviceV1ScrollButtonLockState,
    ) {
        let res = self.try_send_scroll_button_lock_current(
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.scroll_button_lock_current", &e);
        }
    }

    /// Since when the set_scroll_button_lock message is available.
    pub const MSG__SET_SCROLL_BUTTON_LOCK__SINCE: u32 = 1;

    /// set scroll button lock state
    ///
    /// Set scroll button lock state.
    /// Supported if scroll_methods.on_button_down is supported.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    pub fn try_send_set_scroll_button_lock(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1ScrollButtonLockState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            result,
            state,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("result", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1ScrollButtonLockState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_device_v1#{}.set_scroll_button_lock(result: river_libinput_result_v1#{}, state: {:?})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            18,
            arg0_id,
            arg1.0,
        ]);
        Ok(())
    }

    /// set scroll button lock state
    ///
    /// Set scroll button lock state.
    /// Supported if scroll_methods.on_button_down is supported.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    pub fn send_set_scroll_button_lock(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1ScrollButtonLockState,
    ) {
        let res = self.try_send_set_scroll_button_lock(
            result,
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.set_scroll_button_lock", &e);
        }
    }

    /// set scroll button lock state
    ///
    /// Set scroll button lock state.
    /// Supported if scroll_methods.on_button_down is supported.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn new_try_send_set_scroll_button_lock(
        &self,
        state: RiverLibinputDeviceV1ScrollButtonLockState,
    ) -> Result<Rc<RiverLibinputResultV1>, ObjectError> {
        let result = self.core.create_child();
        self.try_send_set_scroll_button_lock(
            &result,
            state,
        )?;
        Ok(result)
    }

    /// set scroll button lock state
    ///
    /// Set scroll button lock state.
    /// Supported if scroll_methods.on_button_down is supported.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn new_send_set_scroll_button_lock(
        &self,
        state: RiverLibinputDeviceV1ScrollButtonLockState,
    ) -> Rc<RiverLibinputResultV1> {
        let result = self.core.create_child();
        self.send_set_scroll_button_lock(
            &result,
            state,
        );
        result
    }

    /// Since when the dwt_support message is available.
    pub const MSG__DWT_SUPPORT__SINCE: u32 = 1;

    /// support for disable-while-typing
    ///
    /// Disable-while-typing is supported if the supported argument is
    /// non-zero.
    ///
    /// # Arguments
    ///
    /// - `supported`: boolean
    #[inline]
    pub fn try_send_dwt_support(
        &self,
        supported: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            supported,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.dwt_support(supported: {})\n", client_id, id, arg0);
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
            46,
            arg0 as u32,
        ]);
        Ok(())
    }

    /// support for disable-while-typing
    ///
    /// Disable-while-typing is supported if the supported argument is
    /// non-zero.
    ///
    /// # Arguments
    ///
    /// - `supported`: boolean
    #[inline]
    pub fn send_dwt_support(
        &self,
        supported: i32,
    ) {
        let res = self.try_send_dwt_support(
            supported,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.dwt_support", &e);
        }
    }

    /// Since when the dwt_default message is available.
    pub const MSG__DWT_DEFAULT__SINCE: u32 = 1;

    /// default disable-while-typing state
    ///
    /// Default disable-while-typing state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_dwt_default(
        &self,
        state: RiverLibinputDeviceV1DwtState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1DwtState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.dwt_default(state: {:?})\n", client_id, id, arg0);
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
            47,
            arg0.0,
        ]);
        Ok(())
    }

    /// default disable-while-typing state
    ///
    /// Default disable-while-typing state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_dwt_default(
        &self,
        state: RiverLibinputDeviceV1DwtState,
    ) {
        let res = self.try_send_dwt_default(
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.dwt_default", &e);
        }
    }

    /// Since when the dwt_current message is available.
    pub const MSG__DWT_CURRENT__SINCE: u32 = 1;

    /// current disable-while-typing state
    ///
    /// Current disable-while-typing state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_dwt_current(
        &self,
        state: RiverLibinputDeviceV1DwtState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1DwtState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.dwt_current(state: {:?})\n", client_id, id, arg0);
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
            48,
            arg0.0,
        ]);
        Ok(())
    }

    /// current disable-while-typing state
    ///
    /// Current disable-while-typing state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_dwt_current(
        &self,
        state: RiverLibinputDeviceV1DwtState,
    ) {
        let res = self.try_send_dwt_current(
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.dwt_current", &e);
        }
    }

    /// Since when the set_dwt message is available.
    pub const MSG__SET_DWT__SINCE: u32 = 1;

    /// set disable-while-typing state
    ///
    /// Set disable-while-typing state.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    pub fn try_send_set_dwt(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1DwtState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            result,
            state,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("result", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1DwtState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_device_v1#{}.set_dwt(result: river_libinput_result_v1#{}, state: {:?})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            19,
            arg0_id,
            arg1.0,
        ]);
        Ok(())
    }

    /// set disable-while-typing state
    ///
    /// Set disable-while-typing state.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    pub fn send_set_dwt(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1DwtState,
    ) {
        let res = self.try_send_set_dwt(
            result,
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.set_dwt", &e);
        }
    }

    /// set disable-while-typing state
    ///
    /// Set disable-while-typing state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn new_try_send_set_dwt(
        &self,
        state: RiverLibinputDeviceV1DwtState,
    ) -> Result<Rc<RiverLibinputResultV1>, ObjectError> {
        let result = self.core.create_child();
        self.try_send_set_dwt(
            &result,
            state,
        )?;
        Ok(result)
    }

    /// set disable-while-typing state
    ///
    /// Set disable-while-typing state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn new_send_set_dwt(
        &self,
        state: RiverLibinputDeviceV1DwtState,
    ) -> Rc<RiverLibinputResultV1> {
        let result = self.core.create_child();
        self.send_set_dwt(
            &result,
            state,
        );
        result
    }

    /// Since when the dwtp_support message is available.
    pub const MSG__DWTP_SUPPORT__SINCE: u32 = 1;

    /// support for disable-while-trackpointing
    ///
    /// Disable-while-trackpointing is supported if the supported argument is
    /// non-zero.
    ///
    /// # Arguments
    ///
    /// - `supported`: boolean
    #[inline]
    pub fn try_send_dwtp_support(
        &self,
        supported: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            supported,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.dwtp_support(supported: {})\n", client_id, id, arg0);
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
            49,
            arg0 as u32,
        ]);
        Ok(())
    }

    /// support for disable-while-trackpointing
    ///
    /// Disable-while-trackpointing is supported if the supported argument is
    /// non-zero.
    ///
    /// # Arguments
    ///
    /// - `supported`: boolean
    #[inline]
    pub fn send_dwtp_support(
        &self,
        supported: i32,
    ) {
        let res = self.try_send_dwtp_support(
            supported,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.dwtp_support", &e);
        }
    }

    /// Since when the dwtp_default message is available.
    pub const MSG__DWTP_DEFAULT__SINCE: u32 = 1;

    /// default disable-while-trackpointing state
    ///
    /// Default disable-while-trackpointing state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_dwtp_default(
        &self,
        state: RiverLibinputDeviceV1DwtpState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1DwtpState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.dwtp_default(state: {:?})\n", client_id, id, arg0);
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
            50,
            arg0.0,
        ]);
        Ok(())
    }

    /// default disable-while-trackpointing state
    ///
    /// Default disable-while-trackpointing state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_dwtp_default(
        &self,
        state: RiverLibinputDeviceV1DwtpState,
    ) {
        let res = self.try_send_dwtp_default(
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.dwtp_default", &e);
        }
    }

    /// Since when the dwtp_current message is available.
    pub const MSG__DWTP_CURRENT__SINCE: u32 = 1;

    /// current disable-while-trackpointing state
    ///
    /// Current disable-while-trackpointing state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_dwtp_current(
        &self,
        state: RiverLibinputDeviceV1DwtpState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverLibinputDeviceV1DwtpState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.dwtp_current(state: {:?})\n", client_id, id, arg0);
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
            51,
            arg0.0,
        ]);
        Ok(())
    }

    /// current disable-while-trackpointing state
    ///
    /// Current disable-while-trackpointing state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_dwtp_current(
        &self,
        state: RiverLibinputDeviceV1DwtpState,
    ) {
        let res = self.try_send_dwtp_current(
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.dwtp_current", &e);
        }
    }

    /// Since when the set_dwtp message is available.
    pub const MSG__SET_DWTP__SINCE: u32 = 1;

    /// set disable-while-trackpointing state
    ///
    /// Set disable-while-trackpointing state.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    pub fn try_send_set_dwtp(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1DwtpState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            result,
            state,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("result", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1DwtpState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_device_v1#{}.set_dwtp(result: river_libinput_result_v1#{}, state: {:?})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            20,
            arg0_id,
            arg1.0,
        ]);
        Ok(())
    }

    /// set disable-while-trackpointing state
    ///
    /// Set disable-while-trackpointing state.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    pub fn send_set_dwtp(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1DwtpState,
    ) {
        let res = self.try_send_set_dwtp(
            result,
            state,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.set_dwtp", &e);
        }
    }

    /// set disable-while-trackpointing state
    ///
    /// Set disable-while-trackpointing state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn new_try_send_set_dwtp(
        &self,
        state: RiverLibinputDeviceV1DwtpState,
    ) -> Result<Rc<RiverLibinputResultV1>, ObjectError> {
        let result = self.core.create_child();
        self.try_send_set_dwtp(
            &result,
            state,
        )?;
        Ok(result)
    }

    /// set disable-while-trackpointing state
    ///
    /// Set disable-while-trackpointing state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn new_send_set_dwtp(
        &self,
        state: RiverLibinputDeviceV1DwtpState,
    ) -> Rc<RiverLibinputResultV1> {
        let result = self.core.create_child();
        self.send_set_dwtp(
            &result,
            state,
        );
        result
    }

    /// Since when the rotation_support message is available.
    pub const MSG__ROTATION_SUPPORT__SINCE: u32 = 1;

    /// support for rotation
    ///
    /// Rotation is supported if the supported argument is non-zero.
    ///
    /// # Arguments
    ///
    /// - `supported`: boolean
    #[inline]
    pub fn try_send_rotation_support(
        &self,
        supported: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            supported,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.rotation_support(supported: {})\n", client_id, id, arg0);
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
            52,
            arg0 as u32,
        ]);
        Ok(())
    }

    /// support for rotation
    ///
    /// Rotation is supported if the supported argument is non-zero.
    ///
    /// # Arguments
    ///
    /// - `supported`: boolean
    #[inline]
    pub fn send_rotation_support(
        &self,
        supported: i32,
    ) {
        let res = self.try_send_rotation_support(
            supported,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.rotation_support", &e);
        }
    }

    /// Since when the rotation_default message is available.
    pub const MSG__ROTATION_DEFAULT__SINCE: u32 = 1;

    /// default rotation angle
    ///
    /// Default rotation angle.
    ///
    /// # Arguments
    ///
    /// - `angle`:
    #[inline]
    pub fn try_send_rotation_default(
        &self,
        angle: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            angle,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.rotation_default(angle: {})\n", client_id, id, arg0);
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
            53,
            arg0,
        ]);
        Ok(())
    }

    /// default rotation angle
    ///
    /// Default rotation angle.
    ///
    /// # Arguments
    ///
    /// - `angle`:
    #[inline]
    pub fn send_rotation_default(
        &self,
        angle: u32,
    ) {
        let res = self.try_send_rotation_default(
            angle,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.rotation_default", &e);
        }
    }

    /// Since when the rotation_current message is available.
    pub const MSG__ROTATION_CURRENT__SINCE: u32 = 1;

    /// current rotation angle
    ///
    /// Current rotation angle.
    ///
    /// # Arguments
    ///
    /// - `angle`:
    #[inline]
    pub fn try_send_rotation_current(
        &self,
        angle: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            angle,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_libinput_device_v1#{}.rotation_current(angle: {})\n", client_id, id, arg0);
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
            54,
            arg0,
        ]);
        Ok(())
    }

    /// current rotation angle
    ///
    /// Current rotation angle.
    ///
    /// # Arguments
    ///
    /// - `angle`:
    #[inline]
    pub fn send_rotation_current(
        &self,
        angle: u32,
    ) {
        let res = self.try_send_rotation_current(
            angle,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.rotation_current", &e);
        }
    }

    /// Since when the set_rotation message is available.
    pub const MSG__SET_ROTATION__SINCE: u32 = 1;

    /// set rotation angle
    ///
    /// Set rotation angle in degrees clockwise off the logical neutral
    /// position. Angle must be in the range [0-360).
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `angle`:
    #[inline]
    pub fn try_send_set_rotation(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        angle: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            result,
            angle,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("result", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_device_v1#{}.set_rotation(result: river_libinput_result_v1#{}, angle: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            21,
            arg0_id,
            arg1,
        ]);
        Ok(())
    }

    /// set rotation angle
    ///
    /// Set rotation angle in degrees clockwise off the logical neutral
    /// position. Angle must be in the range [0-360).
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `angle`:
    #[inline]
    pub fn send_set_rotation(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        angle: u32,
    ) {
        let res = self.try_send_set_rotation(
            result,
            angle,
        );
        if let Err(e) = res {
            log_send("river_libinput_device_v1.set_rotation", &e);
        }
    }

    /// set rotation angle
    ///
    /// Set rotation angle in degrees clockwise off the logical neutral
    /// position. Angle must be in the range [0-360).
    ///
    /// # Arguments
    ///
    /// - `angle`:
    #[inline]
    pub fn new_try_send_set_rotation(
        &self,
        angle: u32,
    ) -> Result<Rc<RiverLibinputResultV1>, ObjectError> {
        let result = self.core.create_child();
        self.try_send_set_rotation(
            &result,
            angle,
        )?;
        Ok(result)
    }

    /// set rotation angle
    ///
    /// Set rotation angle in degrees clockwise off the logical neutral
    /// position. Angle must be in the range [0-360).
    ///
    /// # Arguments
    ///
    /// - `angle`:
    #[inline]
    pub fn new_send_set_rotation(
        &self,
        angle: u32,
    ) -> Rc<RiverLibinputResultV1> {
        let result = self.core.create_child();
        self.send_set_rotation(
            &result,
            angle,
        );
        result
    }
}

/// A message handler for [`RiverLibinputDeviceV1`] proxies.
pub trait RiverLibinputDeviceV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<RiverLibinputDeviceV1>) {
        slf.core.delete_id();
    }

    /// destroy the libinput device object
    ///
    /// This request indicates that the client will no longer use the input
    /// device object and that it may be safely destroyed.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.destroy", &e);
        }
    }

    /// the libinput device is removed
    ///
    /// This event indicates that the libinput device has been removed.
    ///
    /// The server will send no further events on this object and ignore any
    /// request (other than river_libinput_device_v1.destroy) made after this
    /// event is sent. The client should destroy this object with the
    /// river_libinput_device_v1.destroy request to free up resources.
    #[inline]
    fn handle_removed(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_removed(
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.removed", &e);
        }
    }

    /// corresponding river input device
    ///
    /// The river_input_device_v1 corresponding to this libinput device.
    /// This event will always be the first event sent on the
    /// river_libinput_device_v1 object, and it will be sent exactly once.
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
        slf: &Rc<RiverLibinputDeviceV1>,
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
            log_forward("river_libinput_device_v1.input_device", &e);
        }
    }

    /// supported send events modes
    ///
    /// Supported send events modes.
    ///
    /// # Arguments
    ///
    /// - `modes`:
    #[inline]
    fn handle_send_events_support(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        modes: RiverLibinputDeviceV1SendEventsModes,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_send_events_support(
            modes,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.send_events_support", &e);
        }
    }

    /// default send events mode
    ///
    /// Default send events mode.
    ///
    /// # Arguments
    ///
    /// - `mode`:
    #[inline]
    fn handle_send_events_default(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        mode: RiverLibinputDeviceV1SendEventsModes,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_send_events_default(
            mode,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.send_events_default", &e);
        }
    }

    /// current send events mode
    ///
    /// Current send events mode.
    ///
    /// # Arguments
    ///
    /// - `mode`:
    #[inline]
    fn handle_send_events_current(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        mode: RiverLibinputDeviceV1SendEventsModes,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_send_events_current(
            mode,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.send_events_current", &e);
        }
    }

    /// set send events mode
    ///
    /// Set the send events mode for the device.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `mode`:
    #[inline]
    fn handle_set_send_events(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        result: &Rc<RiverLibinputResultV1>,
        mode: RiverLibinputDeviceV1SendEventsModes,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_send_events(
            result,
            mode,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.set_send_events", &e);
        }
    }

    /// tap-to-click/drag support
    ///
    /// The number of fingers supported for tap-to-click/drag.
    /// If finger_count is 0, tap-to-click and drag are unsupported.
    ///
    /// # Arguments
    ///
    /// - `finger_count`:
    #[inline]
    fn handle_tap_support(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        finger_count: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_tap_support(
            finger_count,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.tap_support", &e);
        }
    }

    /// default tap-to-click state
    ///
    /// Default tap-to-click state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_tap_default(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        state: RiverLibinputDeviceV1TapState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_tap_default(
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.tap_default", &e);
        }
    }

    /// current tap-to-click state
    ///
    /// Current tap-to-click state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_tap_current(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        state: RiverLibinputDeviceV1TapState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_tap_current(
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.tap_current", &e);
        }
    }

    /// enable/disable tap-to-click
    ///
    /// Configure tap-to-click on this device, with a default mapping of
    /// 1, 2, 3 finger tap mapping to left, right, middle click, respectively.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    fn handle_set_tap(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1TapState,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_tap(
            result,
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.set_tap", &e);
        }
    }

    /// default tap-to-click button map
    ///
    /// Default tap-to-click button map.
    ///
    /// # Arguments
    ///
    /// - `button_map`:
    #[inline]
    fn handle_tap_button_map_default(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        button_map: RiverLibinputDeviceV1TapButtonMap,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_tap_button_map_default(
            button_map,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.tap_button_map_default", &e);
        }
    }

    /// current tap-to-click button map
    ///
    /// Current tap-to-click button map.
    ///
    /// # Arguments
    ///
    /// - `button_map`:
    #[inline]
    fn handle_tap_button_map_current(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        button_map: RiverLibinputDeviceV1TapButtonMap,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_tap_button_map_current(
            button_map,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.tap_button_map_current", &e);
        }
    }

    /// set tap-to-click button map
    ///
    /// Set the finger number to button number mapping for tap-to-click. The
    /// default mapping on most devices is to have a 1, 2 and 3 finger tap to
    /// map to the left, right and middle button, respectively.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `button_map`:
    #[inline]
    fn handle_set_tap_button_map(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        result: &Rc<RiverLibinputResultV1>,
        button_map: RiverLibinputDeviceV1TapButtonMap,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_tap_button_map(
            result,
            button_map,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.set_tap_button_map", &e);
        }
    }

    /// default tap-and-drag state
    ///
    /// Default tap-and-drag state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_drag_default(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        state: RiverLibinputDeviceV1DragState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_drag_default(
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.drag_default", &e);
        }
    }

    /// current tap-and-drag state
    ///
    /// Current tap-and-drag state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_drag_current(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        state: RiverLibinputDeviceV1DragState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_drag_current(
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.drag_current", &e);
        }
    }

    /// set tap-and-drag state
    ///
    /// Configure tap-and-drag functionality on the device.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    fn handle_set_drag(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1DragState,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_drag(
            result,
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.set_drag", &e);
        }
    }

    /// default drag lock state
    ///
    /// Default drag lock state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_drag_lock_default(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        state: RiverLibinputDeviceV1DragLockState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_drag_lock_default(
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.drag_lock_default", &e);
        }
    }

    /// current drag lock state
    ///
    /// Current drag lock state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_drag_lock_current(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        state: RiverLibinputDeviceV1DragLockState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_drag_lock_current(
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.drag_lock_current", &e);
        }
    }

    /// set drag lock state
    ///
    /// Configure drag-lock during tapping on this device. When enabled, a
    /// finger may be lifted and put back on the touchpad and the drag process
    /// continues. A timeout for lifting the finger is optional. When disabled,
    /// lifting the finger during a tap-and-drag will immediately stop the drag.
    /// See the libinput documentation for more details.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    fn handle_set_drag_lock(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1DragLockState,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_drag_lock(
            result,
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.set_drag_lock", &e);
        }
    }

    /// three finger drag support
    ///
    /// The number of fingers supported for three/four finger drag.
    /// If finger_count is less than 3, three finger drag is unsupported.
    ///
    /// # Arguments
    ///
    /// - `finger_count`:
    #[inline]
    fn handle_three_finger_drag_support(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        finger_count: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_three_finger_drag_support(
            finger_count,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.three_finger_drag_support", &e);
        }
    }

    /// default three finger drag state
    ///
    /// Default three finger drag state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_three_finger_drag_default(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        state: RiverLibinputDeviceV1ThreeFingerDragState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_three_finger_drag_default(
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.three_finger_drag_default", &e);
        }
    }

    /// current three finger drag state
    ///
    /// Current three finger drag state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_three_finger_drag_current(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        state: RiverLibinputDeviceV1ThreeFingerDragState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_three_finger_drag_current(
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.three_finger_drag_current", &e);
        }
    }

    /// set three finger drag state
    ///
    /// Configure three finger drag functionality for the device.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    fn handle_set_three_finger_drag(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1ThreeFingerDragState,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_three_finger_drag(
            result,
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.set_three_finger_drag", &e);
        }
    }

    /// support for a calibration matrix
    ///
    /// A calibration matrix is supported if the supported argument is non-zero.
    ///
    /// # Arguments
    ///
    /// - `supported`: boolean
    #[inline]
    fn handle_calibration_matrix_support(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        supported: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_calibration_matrix_support(
            supported,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.calibration_matrix_support", &e);
        }
    }

    /// default calibration matrix
    ///
    /// Default calibration matrix.
    ///
    /// # Arguments
    ///
    /// - `matrix`: array of 6 floats
    #[inline]
    fn handle_calibration_matrix_default(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        matrix: &[u8],
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_calibration_matrix_default(
            matrix,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.calibration_matrix_default", &e);
        }
    }

    /// current calibration matrix
    ///
    /// Current calibration matrix.
    ///
    /// # Arguments
    ///
    /// - `matrix`: array of 6 floats
    #[inline]
    fn handle_calibration_matrix_current(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        matrix: &[u8],
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_calibration_matrix_current(
            matrix,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.calibration_matrix_current", &e);
        }
    }

    /// set calibration matrix
    ///
    /// Set calibration matrix.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `matrix`: array of 6 floats
    #[inline]
    fn handle_set_calibration_matrix(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        result: &Rc<RiverLibinputResultV1>,
        matrix: &[u8],
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_calibration_matrix(
            result,
            matrix,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.set_calibration_matrix", &e);
        }
    }

    /// supported acceleration profiles
    ///
    /// Supported acceleration profiles.
    ///
    /// # Arguments
    ///
    /// - `profiles`:
    #[inline]
    fn handle_accel_profiles_support(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        profiles: RiverLibinputDeviceV1AccelProfiles,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_accel_profiles_support(
            profiles,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.accel_profiles_support", &e);
        }
    }

    /// default acceleration profile
    ///
    /// Default acceleration profile.
    ///
    /// # Arguments
    ///
    /// - `profile`:
    #[inline]
    fn handle_accel_profile_default(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        profile: RiverLibinputDeviceV1AccelProfile,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_accel_profile_default(
            profile,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.accel_profile_default", &e);
        }
    }

    /// current send events mode
    ///
    /// Current acceleration profile.
    ///
    /// # Arguments
    ///
    /// - `profile`:
    #[inline]
    fn handle_accel_profile_current(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        profile: RiverLibinputDeviceV1AccelProfile,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_accel_profile_current(
            profile,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.accel_profile_current", &e);
        }
    }

    /// set send events mode
    ///
    /// Set the acceleration profile.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `profile`:
    #[inline]
    fn handle_set_accel_profile(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        result: &Rc<RiverLibinputResultV1>,
        profile: RiverLibinputDeviceV1AccelProfile,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_accel_profile(
            result,
            profile,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.set_accel_profile", &e);
        }
    }

    /// default acceleration speed
    ///
    /// Default acceleration speed.
    ///
    /// # Arguments
    ///
    /// - `speed`: double
    #[inline]
    fn handle_accel_speed_default(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        speed: &[u8],
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_accel_speed_default(
            speed,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.accel_speed_default", &e);
        }
    }

    /// current acceleration speed
    ///
    /// Current acceleration speed.
    ///
    /// # Arguments
    ///
    /// - `speed`: double
    #[inline]
    fn handle_accel_speed_current(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        speed: &[u8],
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_accel_speed_current(
            speed,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.accel_speed_current", &e);
        }
    }

    /// set acceleration speed
    ///
    /// Set the acceleration speed within a range of [-1, 1], where 0 is
    /// the default acceleration for this device, -1 is the slowest acceleration
    /// and 1 is the maximum acceleration available on this device.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `speed`: double
    #[inline]
    fn handle_set_accel_speed(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        result: &Rc<RiverLibinputResultV1>,
        speed: &[u8],
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_accel_speed(
            result,
            speed,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.set_accel_speed", &e);
        }
    }

    /// apply acceleration config
    ///
    /// Apply a pointer accleration config.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `config`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_apply_accel_config(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        result: &Rc<RiverLibinputResultV1>,
        config: &Rc<RiverLibinputAccelConfigV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_apply_accel_config(
            result,
            config,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.apply_accel_config", &e);
        }
    }

    /// support for natural scroll
    ///
    /// Natural scroll is supported if the supported argument is non-zero.
    ///
    /// # Arguments
    ///
    /// - `supported`: boolean
    #[inline]
    fn handle_natural_scroll_support(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        supported: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_natural_scroll_support(
            supported,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.natural_scroll_support", &e);
        }
    }

    /// default natural scroll
    ///
    /// Default natural scroll.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_natural_scroll_default(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        state: RiverLibinputDeviceV1NaturalScrollState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_natural_scroll_default(
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.natural_scroll_default", &e);
        }
    }

    /// current natural scroll state
    ///
    /// Current natural scroll.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_natural_scroll_current(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        state: RiverLibinputDeviceV1NaturalScrollState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_natural_scroll_current(
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.natural_scroll_current", &e);
        }
    }

    /// set natural scroll state
    ///
    /// Set natural scroll state.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    fn handle_set_natural_scroll(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1NaturalScrollState,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_natural_scroll(
            result,
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.set_natural_scroll", &e);
        }
    }

    /// support for left-handed mode
    ///
    /// Left-handed mode is supported if the supported argument is non-zero.
    ///
    /// # Arguments
    ///
    /// - `supported`: boolean
    #[inline]
    fn handle_left_handed_support(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        supported: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_left_handed_support(
            supported,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.left_handed_support", &e);
        }
    }

    /// default left-handed mode
    ///
    /// Default left-handed mode.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_left_handed_default(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        state: RiverLibinputDeviceV1LeftHandedState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_left_handed_default(
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.left_handed_default", &e);
        }
    }

    /// current left-handed mode state
    ///
    /// Current left-handed mode.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_left_handed_current(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        state: RiverLibinputDeviceV1LeftHandedState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_left_handed_current(
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.left_handed_current", &e);
        }
    }

    /// set left-handed mode state
    ///
    /// Set left-handed mode state.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    fn handle_set_left_handed(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1LeftHandedState,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_left_handed(
            result,
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.set_left_handed", &e);
        }
    }

    /// supported click methods
    ///
    /// The click methods suppported by the device.
    ///
    /// # Arguments
    ///
    /// - `methods`:
    #[inline]
    fn handle_click_method_support(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        methods: RiverLibinputDeviceV1ClickMethods,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_click_method_support(
            methods,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.click_method_support", &e);
        }
    }

    /// default click method
    ///
    /// Default click method.
    ///
    /// # Arguments
    ///
    /// - `method`:
    #[inline]
    fn handle_click_method_default(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        method: RiverLibinputDeviceV1ClickMethod,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_click_method_default(
            method,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.click_method_default", &e);
        }
    }

    /// current click method
    ///
    /// Current click method.
    ///
    /// # Arguments
    ///
    /// - `method`:
    #[inline]
    fn handle_click_method_current(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        method: RiverLibinputDeviceV1ClickMethod,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_click_method_current(
            method,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.click_method_current", &e);
        }
    }

    /// set click method
    ///
    /// Set click method.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `method`:
    #[inline]
    fn handle_set_click_method(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        result: &Rc<RiverLibinputResultV1>,
        method: RiverLibinputDeviceV1ClickMethod,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_click_method(
            result,
            method,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.set_click_method", &e);
        }
    }

    /// default clickfinger button map
    ///
    /// Default clickfinger button map.
    /// Supported if click_methods.clickfinger is supported.
    ///
    /// # Arguments
    ///
    /// - `button_map`:
    #[inline]
    fn handle_clickfinger_button_map_default(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        button_map: RiverLibinputDeviceV1ClickfingerButtonMap,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_clickfinger_button_map_default(
            button_map,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.clickfinger_button_map_default", &e);
        }
    }

    /// current clickfinger button map
    ///
    /// Current clickfinger button map.
    /// Supported if click_methods.clickfinger is supported.
    ///
    /// # Arguments
    ///
    /// - `button_map`:
    #[inline]
    fn handle_clickfinger_button_map_current(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        button_map: RiverLibinputDeviceV1ClickfingerButtonMap,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_clickfinger_button_map_current(
            button_map,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.clickfinger_button_map_current", &e);
        }
    }

    /// set clickfinger button map
    ///
    /// Set clickfinger button map.
    /// Supported if click_methods.clickfinger is supported.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `button_map`:
    #[inline]
    fn handle_set_clickfinger_button_map(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        result: &Rc<RiverLibinputResultV1>,
        button_map: RiverLibinputDeviceV1ClickfingerButtonMap,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_clickfinger_button_map(
            result,
            button_map,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.set_clickfinger_button_map", &e);
        }
    }

    /// support for middle mouse button emulation
    ///
    /// Middle mouse button emulation is supported if the supported argument is
    /// non-zero.
    ///
    /// # Arguments
    ///
    /// - `supported`: boolean
    #[inline]
    fn handle_middle_emulation_support(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        supported: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_middle_emulation_support(
            supported,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.middle_emulation_support", &e);
        }
    }

    /// default middle mouse button emulation
    ///
    /// Default middle mouse button emulation.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_middle_emulation_default(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        state: RiverLibinputDeviceV1MiddleEmulationState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_middle_emulation_default(
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.middle_emulation_default", &e);
        }
    }

    /// current middle mouse button emulation state
    ///
    /// Current middle mouse button emulation.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_middle_emulation_current(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        state: RiverLibinputDeviceV1MiddleEmulationState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_middle_emulation_current(
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.middle_emulation_current", &e);
        }
    }

    /// set middle mouse button emulation state
    ///
    /// Set middle mouse button emulation state.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    fn handle_set_middle_emulation(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1MiddleEmulationState,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_middle_emulation(
            result,
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.set_middle_emulation", &e);
        }
    }

    /// supported scroll methods
    ///
    /// The scroll methods suppported by the device.
    ///
    /// # Arguments
    ///
    /// - `methods`:
    #[inline]
    fn handle_scroll_method_support(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        methods: RiverLibinputDeviceV1ScrollMethods,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_scroll_method_support(
            methods,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.scroll_method_support", &e);
        }
    }

    /// default scroll method
    ///
    /// Default scroll method.
    ///
    /// # Arguments
    ///
    /// - `method`:
    #[inline]
    fn handle_scroll_method_default(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        method: RiverLibinputDeviceV1ScrollMethod,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_scroll_method_default(
            method,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.scroll_method_default", &e);
        }
    }

    /// current scroll method
    ///
    /// Current scroll method.
    ///
    /// # Arguments
    ///
    /// - `method`:
    #[inline]
    fn handle_scroll_method_current(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        method: RiverLibinputDeviceV1ScrollMethod,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_scroll_method_current(
            method,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.scroll_method_current", &e);
        }
    }

    /// set scroll method
    ///
    /// Set scroll method.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `method`:
    #[inline]
    fn handle_set_scroll_method(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        result: &Rc<RiverLibinputResultV1>,
        method: RiverLibinputDeviceV1ScrollMethod,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_scroll_method(
            result,
            method,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.set_scroll_method", &e);
        }
    }

    /// default scroll button
    ///
    /// Default scroll button.
    /// Supported if scroll_methods.on_button_down is supported.
    ///
    /// # Arguments
    ///
    /// - `button`:
    #[inline]
    fn handle_scroll_button_default(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        button: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_scroll_button_default(
            button,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.scroll_button_default", &e);
        }
    }

    /// current scroll button
    ///
    /// Current scroll button.
    /// Supported if scroll_methods.on_button_down is supported.
    ///
    /// # Arguments
    ///
    /// - `button`:
    #[inline]
    fn handle_scroll_button_current(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        button: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_scroll_button_current(
            button,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.scroll_button_current", &e);
        }
    }

    /// set scroll button
    ///
    /// Set scroll button.
    /// Supported if scroll_methods.on_button_down is supported.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `button`:
    #[inline]
    fn handle_set_scroll_button(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        result: &Rc<RiverLibinputResultV1>,
        button: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_scroll_button(
            result,
            button,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.set_scroll_button", &e);
        }
    }

    /// default scroll button lock state
    ///
    /// Default scroll button lock state.
    /// Supported if scroll_methods.on_button_down is supported.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_scroll_button_lock_default(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        state: RiverLibinputDeviceV1ScrollButtonLockState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_scroll_button_lock_default(
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.scroll_button_lock_default", &e);
        }
    }

    /// current scroll button lock state
    ///
    /// Current scroll button lock state.
    /// Supported if scroll_methods.on_button_down is supported.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_scroll_button_lock_current(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        state: RiverLibinputDeviceV1ScrollButtonLockState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_scroll_button_lock_current(
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.scroll_button_lock_current", &e);
        }
    }

    /// set scroll button lock state
    ///
    /// Set scroll button lock state.
    /// Supported if scroll_methods.on_button_down is supported.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    fn handle_set_scroll_button_lock(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1ScrollButtonLockState,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_scroll_button_lock(
            result,
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.set_scroll_button_lock", &e);
        }
    }

    /// support for disable-while-typing
    ///
    /// Disable-while-typing is supported if the supported argument is
    /// non-zero.
    ///
    /// # Arguments
    ///
    /// - `supported`: boolean
    #[inline]
    fn handle_dwt_support(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        supported: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_dwt_support(
            supported,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.dwt_support", &e);
        }
    }

    /// default disable-while-typing state
    ///
    /// Default disable-while-typing state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_dwt_default(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        state: RiverLibinputDeviceV1DwtState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_dwt_default(
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.dwt_default", &e);
        }
    }

    /// current disable-while-typing state
    ///
    /// Current disable-while-typing state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_dwt_current(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        state: RiverLibinputDeviceV1DwtState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_dwt_current(
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.dwt_current", &e);
        }
    }

    /// set disable-while-typing state
    ///
    /// Set disable-while-typing state.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    fn handle_set_dwt(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1DwtState,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_dwt(
            result,
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.set_dwt", &e);
        }
    }

    /// support for disable-while-trackpointing
    ///
    /// Disable-while-trackpointing is supported if the supported argument is
    /// non-zero.
    ///
    /// # Arguments
    ///
    /// - `supported`: boolean
    #[inline]
    fn handle_dwtp_support(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        supported: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_dwtp_support(
            supported,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.dwtp_support", &e);
        }
    }

    /// default disable-while-trackpointing state
    ///
    /// Default disable-while-trackpointing state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_dwtp_default(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        state: RiverLibinputDeviceV1DwtpState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_dwtp_default(
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.dwtp_default", &e);
        }
    }

    /// current disable-while-trackpointing state
    ///
    /// Current disable-while-trackpointing state.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_dwtp_current(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        state: RiverLibinputDeviceV1DwtpState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_dwtp_current(
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.dwtp_current", &e);
        }
    }

    /// set disable-while-trackpointing state
    ///
    /// Set disable-while-trackpointing state.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `state`:
    #[inline]
    fn handle_set_dwtp(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        result: &Rc<RiverLibinputResultV1>,
        state: RiverLibinputDeviceV1DwtpState,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_dwtp(
            result,
            state,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.set_dwtp", &e);
        }
    }

    /// support for rotation
    ///
    /// Rotation is supported if the supported argument is non-zero.
    ///
    /// # Arguments
    ///
    /// - `supported`: boolean
    #[inline]
    fn handle_rotation_support(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        supported: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_rotation_support(
            supported,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.rotation_support", &e);
        }
    }

    /// default rotation angle
    ///
    /// Default rotation angle.
    ///
    /// # Arguments
    ///
    /// - `angle`:
    #[inline]
    fn handle_rotation_default(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        angle: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_rotation_default(
            angle,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.rotation_default", &e);
        }
    }

    /// current rotation angle
    ///
    /// Current rotation angle.
    ///
    /// # Arguments
    ///
    /// - `angle`:
    #[inline]
    fn handle_rotation_current(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        angle: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_rotation_current(
            angle,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.rotation_current", &e);
        }
    }

    /// set rotation angle
    ///
    /// Set rotation angle in degrees clockwise off the logical neutral
    /// position. Angle must be in the range [0-360).
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `angle`:
    #[inline]
    fn handle_set_rotation(
        &mut self,
        slf: &Rc<RiverLibinputDeviceV1>,
        result: &Rc<RiverLibinputResultV1>,
        angle: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_rotation(
            result,
            angle,
        );
        if let Err(e) = res {
            log_forward("river_libinput_device_v1.set_rotation", &e);
        }
    }
}

impl ObjectPrivate for RiverLibinputDeviceV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::RiverLibinputDeviceV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_device_v1#{}.destroy()\n", client_id, id);
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
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = RiverLibinputDeviceV1SendEventsModes(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1SendEventsModes) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_device_v1#{}.set_send_events(result: river_libinput_result_v1#{}, mode: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverLibinputResultV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "result", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_send_events(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_send_events(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = RiverLibinputDeviceV1TapState(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1TapState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_device_v1#{}.set_tap(result: river_libinput_result_v1#{}, state: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverLibinputResultV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "result", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_tap(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_tap(&self, arg0, arg1);
                }
            }
            3 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = RiverLibinputDeviceV1TapButtonMap(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1TapButtonMap) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_device_v1#{}.set_tap_button_map(result: river_libinput_result_v1#{}, button_map: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverLibinputResultV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "result", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_tap_button_map(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_tap_button_map(&self, arg0, arg1);
                }
            }
            4 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = RiverLibinputDeviceV1DragState(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1DragState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_device_v1#{}.set_drag(result: river_libinput_result_v1#{}, state: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverLibinputResultV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "result", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_drag(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_drag(&self, arg0, arg1);
                }
            }
            5 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = RiverLibinputDeviceV1DragLockState(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1DragLockState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_device_v1#{}.set_drag_lock(result: river_libinput_result_v1#{}, state: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverLibinputResultV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "result", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_drag_lock(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_drag_lock(&self, arg0, arg1);
                }
            }
            6 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = RiverLibinputDeviceV1ThreeFingerDragState(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1ThreeFingerDragState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_device_v1#{}.set_three_finger_drag(result: river_libinput_result_v1#{}, state: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverLibinputResultV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "result", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_three_finger_drag(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_three_finger_drag(&self, arg0, arg1);
                }
            }
            7 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("result")));
                };
                offset += 1;
                let arg1;
                (arg1, offset) = parse_array(msg, offset, "matrix")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_device_v1#{}.set_calibration_matrix(result: river_libinput_result_v1#{}, matrix: {})\n", client_id, id, arg0, debug_array(arg1));
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverLibinputResultV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "result", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_calibration_matrix(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_calibration_matrix(&self, arg0, arg1);
                }
            }
            8 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = RiverLibinputDeviceV1AccelProfile(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1AccelProfile) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_device_v1#{}.set_accel_profile(result: river_libinput_result_v1#{}, profile: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverLibinputResultV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "result", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_accel_profile(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_accel_profile(&self, arg0, arg1);
                }
            }
            9 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("result")));
                };
                offset += 1;
                let arg1;
                (arg1, offset) = parse_array(msg, offset, "speed")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_device_v1#{}.set_accel_speed(result: river_libinput_result_v1#{}, speed: {})\n", client_id, id, arg0, debug_array(arg1));
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverLibinputResultV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "result", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_accel_speed(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_accel_speed(&self, arg0, arg1);
                }
            }
            10 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_device_v1#{}.apply_accel_config(result: river_libinput_result_v1#{}, config: river_libinput_accel_config_v1#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverLibinputResultV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "result", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<RiverLibinputAccelConfigV1>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("config", o.core().interface, ObjectInterface::RiverLibinputAccelConfigV1)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_apply_accel_config(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_apply_accel_config(&self, arg0, arg1);
                }
            }
            11 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = RiverLibinputDeviceV1NaturalScrollState(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1NaturalScrollState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_device_v1#{}.set_natural_scroll(result: river_libinput_result_v1#{}, state: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverLibinputResultV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "result", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_natural_scroll(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_natural_scroll(&self, arg0, arg1);
                }
            }
            12 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = RiverLibinputDeviceV1LeftHandedState(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1LeftHandedState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_device_v1#{}.set_left_handed(result: river_libinput_result_v1#{}, state: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverLibinputResultV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "result", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_left_handed(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_left_handed(&self, arg0, arg1);
                }
            }
            13 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = RiverLibinputDeviceV1ClickMethod(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1ClickMethod) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_device_v1#{}.set_click_method(result: river_libinput_result_v1#{}, method: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverLibinputResultV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "result", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_click_method(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_click_method(&self, arg0, arg1);
                }
            }
            14 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = RiverLibinputDeviceV1ClickfingerButtonMap(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1ClickfingerButtonMap) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_device_v1#{}.set_clickfinger_button_map(result: river_libinput_result_v1#{}, button_map: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverLibinputResultV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "result", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_clickfinger_button_map(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_clickfinger_button_map(&self, arg0, arg1);
                }
            }
            15 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = RiverLibinputDeviceV1MiddleEmulationState(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1MiddleEmulationState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_device_v1#{}.set_middle_emulation(result: river_libinput_result_v1#{}, state: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverLibinputResultV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "result", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_middle_emulation(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_middle_emulation(&self, arg0, arg1);
                }
            }
            16 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = RiverLibinputDeviceV1ScrollMethod(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1ScrollMethod) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_device_v1#{}.set_scroll_method(result: river_libinput_result_v1#{}, method: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverLibinputResultV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "result", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_scroll_method(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_scroll_method(&self, arg0, arg1);
                }
            }
            17 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_device_v1#{}.set_scroll_button(result: river_libinput_result_v1#{}, button: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverLibinputResultV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "result", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_scroll_button(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_scroll_button(&self, arg0, arg1);
                }
            }
            18 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = RiverLibinputDeviceV1ScrollButtonLockState(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1ScrollButtonLockState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_device_v1#{}.set_scroll_button_lock(result: river_libinput_result_v1#{}, state: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverLibinputResultV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "result", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_scroll_button_lock(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_scroll_button_lock(&self, arg0, arg1);
                }
            }
            19 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = RiverLibinputDeviceV1DwtState(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1DwtState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_device_v1#{}.set_dwt(result: river_libinput_result_v1#{}, state: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverLibinputResultV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "result", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_dwt(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_dwt(&self, arg0, arg1);
                }
            }
            20 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = RiverLibinputDeviceV1DwtpState(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: RiverLibinputDeviceV1DwtpState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_device_v1#{}.set_dwtp(result: river_libinput_result_v1#{}, state: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverLibinputResultV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "result", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_dwtp(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_dwtp(&self, arg0, arg1);
                }
            }
            21 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_device_v1#{}.set_rotation(result: river_libinput_result_v1#{}, angle: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverLibinputResultV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "result", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_rotation(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_rotation(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.removed()\n", id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.input_device(device: river_input_device_v1#{})\n", id, arg0);
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1SendEventsModes(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1SendEventsModes) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.send_events_support(modes: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_send_events_support(&self, arg0);
                } else {
                    DefaultHandler.handle_send_events_support(&self, arg0);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1SendEventsModes(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1SendEventsModes) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.send_events_default(mode: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_send_events_default(&self, arg0);
                } else {
                    DefaultHandler.handle_send_events_default(&self, arg0);
                }
            }
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1SendEventsModes(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1SendEventsModes) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.send_events_current(mode: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_send_events_current(&self, arg0);
                } else {
                    DefaultHandler.handle_send_events_current(&self, arg0);
                }
            }
            5 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.tap_support(finger_count: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_tap_support(&self, arg0);
                } else {
                    DefaultHandler.handle_tap_support(&self, arg0);
                }
            }
            6 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1TapState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1TapState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.tap_default(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_tap_default(&self, arg0);
                } else {
                    DefaultHandler.handle_tap_default(&self, arg0);
                }
            }
            7 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1TapState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1TapState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.tap_current(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_tap_current(&self, arg0);
                } else {
                    DefaultHandler.handle_tap_current(&self, arg0);
                }
            }
            8 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1TapButtonMap(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1TapButtonMap) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.tap_button_map_default(button_map: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_tap_button_map_default(&self, arg0);
                } else {
                    DefaultHandler.handle_tap_button_map_default(&self, arg0);
                }
            }
            9 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1TapButtonMap(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1TapButtonMap) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.tap_button_map_current(button_map: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_tap_button_map_current(&self, arg0);
                } else {
                    DefaultHandler.handle_tap_button_map_current(&self, arg0);
                }
            }
            10 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1DragState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1DragState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.drag_default(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_drag_default(&self, arg0);
                } else {
                    DefaultHandler.handle_drag_default(&self, arg0);
                }
            }
            11 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1DragState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1DragState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.drag_current(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_drag_current(&self, arg0);
                } else {
                    DefaultHandler.handle_drag_current(&self, arg0);
                }
            }
            12 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1DragLockState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1DragLockState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.drag_lock_default(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_drag_lock_default(&self, arg0);
                } else {
                    DefaultHandler.handle_drag_lock_default(&self, arg0);
                }
            }
            13 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1DragLockState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1DragLockState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.drag_lock_current(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_drag_lock_current(&self, arg0);
                } else {
                    DefaultHandler.handle_drag_lock_current(&self, arg0);
                }
            }
            14 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.three_finger_drag_support(finger_count: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_three_finger_drag_support(&self, arg0);
                } else {
                    DefaultHandler.handle_three_finger_drag_support(&self, arg0);
                }
            }
            15 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1ThreeFingerDragState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1ThreeFingerDragState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.three_finger_drag_default(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_three_finger_drag_default(&self, arg0);
                } else {
                    DefaultHandler.handle_three_finger_drag_default(&self, arg0);
                }
            }
            16 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1ThreeFingerDragState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1ThreeFingerDragState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.three_finger_drag_current(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_three_finger_drag_current(&self, arg0);
                } else {
                    DefaultHandler.handle_three_finger_drag_current(&self, arg0);
                }
            }
            17 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.calibration_matrix_support(supported: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_calibration_matrix_support(&self, arg0);
                } else {
                    DefaultHandler.handle_calibration_matrix_support(&self, arg0);
                }
            }
            18 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_array(msg, offset, "matrix")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.calibration_matrix_default(matrix: {})\n", id, debug_array(arg0));
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_calibration_matrix_default(&self, arg0);
                } else {
                    DefaultHandler.handle_calibration_matrix_default(&self, arg0);
                }
            }
            19 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_array(msg, offset, "matrix")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.calibration_matrix_current(matrix: {})\n", id, debug_array(arg0));
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_calibration_matrix_current(&self, arg0);
                } else {
                    DefaultHandler.handle_calibration_matrix_current(&self, arg0);
                }
            }
            20 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1AccelProfiles(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1AccelProfiles) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.accel_profiles_support(profiles: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_accel_profiles_support(&self, arg0);
                } else {
                    DefaultHandler.handle_accel_profiles_support(&self, arg0);
                }
            }
            21 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1AccelProfile(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1AccelProfile) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.accel_profile_default(profile: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_accel_profile_default(&self, arg0);
                } else {
                    DefaultHandler.handle_accel_profile_default(&self, arg0);
                }
            }
            22 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1AccelProfile(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1AccelProfile) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.accel_profile_current(profile: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_accel_profile_current(&self, arg0);
                } else {
                    DefaultHandler.handle_accel_profile_current(&self, arg0);
                }
            }
            23 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_array(msg, offset, "speed")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.accel_speed_default(speed: {})\n", id, debug_array(arg0));
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_accel_speed_default(&self, arg0);
                } else {
                    DefaultHandler.handle_accel_speed_default(&self, arg0);
                }
            }
            24 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_array(msg, offset, "speed")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.accel_speed_current(speed: {})\n", id, debug_array(arg0));
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_accel_speed_current(&self, arg0);
                } else {
                    DefaultHandler.handle_accel_speed_current(&self, arg0);
                }
            }
            25 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.natural_scroll_support(supported: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_natural_scroll_support(&self, arg0);
                } else {
                    DefaultHandler.handle_natural_scroll_support(&self, arg0);
                }
            }
            26 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1NaturalScrollState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1NaturalScrollState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.natural_scroll_default(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_natural_scroll_default(&self, arg0);
                } else {
                    DefaultHandler.handle_natural_scroll_default(&self, arg0);
                }
            }
            27 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1NaturalScrollState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1NaturalScrollState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.natural_scroll_current(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_natural_scroll_current(&self, arg0);
                } else {
                    DefaultHandler.handle_natural_scroll_current(&self, arg0);
                }
            }
            28 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.left_handed_support(supported: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_left_handed_support(&self, arg0);
                } else {
                    DefaultHandler.handle_left_handed_support(&self, arg0);
                }
            }
            29 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1LeftHandedState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1LeftHandedState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.left_handed_default(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_left_handed_default(&self, arg0);
                } else {
                    DefaultHandler.handle_left_handed_default(&self, arg0);
                }
            }
            30 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1LeftHandedState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1LeftHandedState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.left_handed_current(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_left_handed_current(&self, arg0);
                } else {
                    DefaultHandler.handle_left_handed_current(&self, arg0);
                }
            }
            31 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1ClickMethods(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1ClickMethods) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.click_method_support(methods: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_click_method_support(&self, arg0);
                } else {
                    DefaultHandler.handle_click_method_support(&self, arg0);
                }
            }
            32 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1ClickMethod(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1ClickMethod) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.click_method_default(method: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_click_method_default(&self, arg0);
                } else {
                    DefaultHandler.handle_click_method_default(&self, arg0);
                }
            }
            33 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1ClickMethod(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1ClickMethod) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.click_method_current(method: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_click_method_current(&self, arg0);
                } else {
                    DefaultHandler.handle_click_method_current(&self, arg0);
                }
            }
            34 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1ClickfingerButtonMap(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1ClickfingerButtonMap) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.clickfinger_button_map_default(button_map: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_clickfinger_button_map_default(&self, arg0);
                } else {
                    DefaultHandler.handle_clickfinger_button_map_default(&self, arg0);
                }
            }
            35 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1ClickfingerButtonMap(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1ClickfingerButtonMap) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.clickfinger_button_map_current(button_map: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_clickfinger_button_map_current(&self, arg0);
                } else {
                    DefaultHandler.handle_clickfinger_button_map_current(&self, arg0);
                }
            }
            36 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.middle_emulation_support(supported: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_middle_emulation_support(&self, arg0);
                } else {
                    DefaultHandler.handle_middle_emulation_support(&self, arg0);
                }
            }
            37 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1MiddleEmulationState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1MiddleEmulationState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.middle_emulation_default(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_middle_emulation_default(&self, arg0);
                } else {
                    DefaultHandler.handle_middle_emulation_default(&self, arg0);
                }
            }
            38 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1MiddleEmulationState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1MiddleEmulationState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.middle_emulation_current(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_middle_emulation_current(&self, arg0);
                } else {
                    DefaultHandler.handle_middle_emulation_current(&self, arg0);
                }
            }
            39 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1ScrollMethods(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1ScrollMethods) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.scroll_method_support(methods: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_scroll_method_support(&self, arg0);
                } else {
                    DefaultHandler.handle_scroll_method_support(&self, arg0);
                }
            }
            40 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1ScrollMethod(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1ScrollMethod) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.scroll_method_default(method: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_scroll_method_default(&self, arg0);
                } else {
                    DefaultHandler.handle_scroll_method_default(&self, arg0);
                }
            }
            41 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1ScrollMethod(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1ScrollMethod) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.scroll_method_current(method: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_scroll_method_current(&self, arg0);
                } else {
                    DefaultHandler.handle_scroll_method_current(&self, arg0);
                }
            }
            42 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.scroll_button_default(button: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_scroll_button_default(&self, arg0);
                } else {
                    DefaultHandler.handle_scroll_button_default(&self, arg0);
                }
            }
            43 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.scroll_button_current(button: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_scroll_button_current(&self, arg0);
                } else {
                    DefaultHandler.handle_scroll_button_current(&self, arg0);
                }
            }
            44 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1ScrollButtonLockState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1ScrollButtonLockState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.scroll_button_lock_default(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_scroll_button_lock_default(&self, arg0);
                } else {
                    DefaultHandler.handle_scroll_button_lock_default(&self, arg0);
                }
            }
            45 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1ScrollButtonLockState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1ScrollButtonLockState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.scroll_button_lock_current(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_scroll_button_lock_current(&self, arg0);
                } else {
                    DefaultHandler.handle_scroll_button_lock_current(&self, arg0);
                }
            }
            46 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.dwt_support(supported: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_dwt_support(&self, arg0);
                } else {
                    DefaultHandler.handle_dwt_support(&self, arg0);
                }
            }
            47 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1DwtState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1DwtState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.dwt_default(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_dwt_default(&self, arg0);
                } else {
                    DefaultHandler.handle_dwt_default(&self, arg0);
                }
            }
            48 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1DwtState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1DwtState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.dwt_current(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_dwt_current(&self, arg0);
                } else {
                    DefaultHandler.handle_dwt_current(&self, arg0);
                }
            }
            49 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.dwtp_support(supported: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_dwtp_support(&self, arg0);
                } else {
                    DefaultHandler.handle_dwtp_support(&self, arg0);
                }
            }
            50 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1DwtpState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1DwtpState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.dwtp_default(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_dwtp_default(&self, arg0);
                } else {
                    DefaultHandler.handle_dwtp_default(&self, arg0);
                }
            }
            51 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverLibinputDeviceV1DwtpState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverLibinputDeviceV1DwtpState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.dwtp_current(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_dwtp_current(&self, arg0);
                } else {
                    DefaultHandler.handle_dwtp_current(&self, arg0);
                }
            }
            52 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.rotation_support(supported: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_rotation_support(&self, arg0);
                } else {
                    DefaultHandler.handle_rotation_support(&self, arg0);
                }
            }
            53 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.rotation_default(angle: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_rotation_default(&self, arg0);
                } else {
                    DefaultHandler.handle_rotation_default(&self, arg0);
                }
            }
            54 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_libinput_device_v1#{}.rotation_current(angle: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_rotation_current(&self, arg0);
                } else {
                    DefaultHandler.handle_rotation_current(&self, arg0);
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
            1 => "set_send_events",
            2 => "set_tap",
            3 => "set_tap_button_map",
            4 => "set_drag",
            5 => "set_drag_lock",
            6 => "set_three_finger_drag",
            7 => "set_calibration_matrix",
            8 => "set_accel_profile",
            9 => "set_accel_speed",
            10 => "apply_accel_config",
            11 => "set_natural_scroll",
            12 => "set_left_handed",
            13 => "set_click_method",
            14 => "set_clickfinger_button_map",
            15 => "set_middle_emulation",
            16 => "set_scroll_method",
            17 => "set_scroll_button",
            18 => "set_scroll_button_lock",
            19 => "set_dwt",
            20 => "set_dwtp",
            21 => "set_rotation",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "removed",
            1 => "input_device",
            2 => "send_events_support",
            3 => "send_events_default",
            4 => "send_events_current",
            5 => "tap_support",
            6 => "tap_default",
            7 => "tap_current",
            8 => "tap_button_map_default",
            9 => "tap_button_map_current",
            10 => "drag_default",
            11 => "drag_current",
            12 => "drag_lock_default",
            13 => "drag_lock_current",
            14 => "three_finger_drag_support",
            15 => "three_finger_drag_default",
            16 => "three_finger_drag_current",
            17 => "calibration_matrix_support",
            18 => "calibration_matrix_default",
            19 => "calibration_matrix_current",
            20 => "accel_profiles_support",
            21 => "accel_profile_default",
            22 => "accel_profile_current",
            23 => "accel_speed_default",
            24 => "accel_speed_current",
            25 => "natural_scroll_support",
            26 => "natural_scroll_default",
            27 => "natural_scroll_current",
            28 => "left_handed_support",
            29 => "left_handed_default",
            30 => "left_handed_current",
            31 => "click_method_support",
            32 => "click_method_default",
            33 => "click_method_current",
            34 => "clickfinger_button_map_default",
            35 => "clickfinger_button_map_current",
            36 => "middle_emulation_support",
            37 => "middle_emulation_default",
            38 => "middle_emulation_current",
            39 => "scroll_method_support",
            40 => "scroll_method_default",
            41 => "scroll_method_current",
            42 => "scroll_button_default",
            43 => "scroll_button_current",
            44 => "scroll_button_lock_default",
            45 => "scroll_button_lock_current",
            46 => "dwt_support",
            47 => "dwt_default",
            48 => "dwt_current",
            49 => "dwtp_support",
            50 => "dwtp_default",
            51 => "dwtp_current",
            52 => "rotation_support",
            53 => "rotation_default",
            54 => "rotation_current",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for RiverLibinputDeviceV1 {
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

impl RiverLibinputDeviceV1 {
    /// Since when the error.invalid_arg enum variant is available.
    pub const ENM__ERROR_INVALID_ARG__SINCE: u32 = 1;

    /// Since when the send_events_modes.enabled enum variant is available.
    pub const ENM__SEND_EVENTS_MODES_ENABLED__SINCE: u32 = 1;
    /// Since when the send_events_modes.disabled enum variant is available.
    pub const ENM__SEND_EVENTS_MODES_DISABLED__SINCE: u32 = 1;
    /// Since when the send_events_modes.disabled_on_external_mouse enum variant is available.
    pub const ENM__SEND_EVENTS_MODES_DISABLED_ON_EXTERNAL_MOUSE__SINCE: u32 = 1;

    /// Since when the tap_state.disabled enum variant is available.
    pub const ENM__TAP_STATE_DISABLED__SINCE: u32 = 1;
    /// Since when the tap_state.enabled enum variant is available.
    pub const ENM__TAP_STATE_ENABLED__SINCE: u32 = 1;

    /// Since when the tap_button_map.lrm enum variant is available.
    pub const ENM__TAP_BUTTON_MAP_LRM__SINCE: u32 = 1;
    /// Since when the tap_button_map.lmr enum variant is available.
    pub const ENM__TAP_BUTTON_MAP_LMR__SINCE: u32 = 1;

    /// Since when the drag_state.disabled enum variant is available.
    pub const ENM__DRAG_STATE_DISABLED__SINCE: u32 = 1;
    /// Since when the drag_state.enabled enum variant is available.
    pub const ENM__DRAG_STATE_ENABLED__SINCE: u32 = 1;

    /// Since when the drag_lock_state.disabled enum variant is available.
    pub const ENM__DRAG_LOCK_STATE_DISABLED__SINCE: u32 = 1;
    /// Since when the drag_lock_state.enabled_timeout enum variant is available.
    pub const ENM__DRAG_LOCK_STATE_ENABLED_TIMEOUT__SINCE: u32 = 1;
    /// Since when the drag_lock_state.enabled_sticky enum variant is available.
    pub const ENM__DRAG_LOCK_STATE_ENABLED_STICKY__SINCE: u32 = 1;

    /// Since when the three_finger_drag_state.disabled enum variant is available.
    pub const ENM__THREE_FINGER_DRAG_STATE_DISABLED__SINCE: u32 = 1;
    /// Since when the three_finger_drag_state.enabled_3fg enum variant is available.
    pub const ENM__THREE_FINGER_DRAG_STATE_ENABLED_3FG__SINCE: u32 = 1;
    /// Since when the three_finger_drag_state.enabled_4fg enum variant is available.
    pub const ENM__THREE_FINGER_DRAG_STATE_ENABLED_4FG__SINCE: u32 = 1;

    /// Since when the accel_profile.none enum variant is available.
    pub const ENM__ACCEL_PROFILE_NONE__SINCE: u32 = 1;
    /// Since when the accel_profile.flat enum variant is available.
    pub const ENM__ACCEL_PROFILE_FLAT__SINCE: u32 = 1;
    /// Since when the accel_profile.adaptive enum variant is available.
    pub const ENM__ACCEL_PROFILE_ADAPTIVE__SINCE: u32 = 1;
    /// Since when the accel_profile.custom enum variant is available.
    pub const ENM__ACCEL_PROFILE_CUSTOM__SINCE: u32 = 1;

    /// Since when the accel_profiles.none enum variant is available.
    pub const ENM__ACCEL_PROFILES_NONE__SINCE: u32 = 1;
    /// Since when the accel_profiles.flat enum variant is available.
    pub const ENM__ACCEL_PROFILES_FLAT__SINCE: u32 = 1;
    /// Since when the accel_profiles.adaptive enum variant is available.
    pub const ENM__ACCEL_PROFILES_ADAPTIVE__SINCE: u32 = 1;
    /// Since when the accel_profiles.custom enum variant is available.
    pub const ENM__ACCEL_PROFILES_CUSTOM__SINCE: u32 = 1;

    /// Since when the natural_scroll_state.disabled enum variant is available.
    pub const ENM__NATURAL_SCROLL_STATE_DISABLED__SINCE: u32 = 1;
    /// Since when the natural_scroll_state.enabled enum variant is available.
    pub const ENM__NATURAL_SCROLL_STATE_ENABLED__SINCE: u32 = 1;

    /// Since when the left_handed_state.disabled enum variant is available.
    pub const ENM__LEFT_HANDED_STATE_DISABLED__SINCE: u32 = 1;
    /// Since when the left_handed_state.enabled enum variant is available.
    pub const ENM__LEFT_HANDED_STATE_ENABLED__SINCE: u32 = 1;

    /// Since when the click_method.none enum variant is available.
    pub const ENM__CLICK_METHOD_NONE__SINCE: u32 = 1;
    /// Since when the click_method.button_areas enum variant is available.
    pub const ENM__CLICK_METHOD_BUTTON_AREAS__SINCE: u32 = 1;
    /// Since when the click_method.clickfinger enum variant is available.
    pub const ENM__CLICK_METHOD_CLICKFINGER__SINCE: u32 = 1;

    /// Since when the click_methods.none enum variant is available.
    pub const ENM__CLICK_METHODS_NONE__SINCE: u32 = 1;
    /// Since when the click_methods.button_areas enum variant is available.
    pub const ENM__CLICK_METHODS_BUTTON_AREAS__SINCE: u32 = 1;
    /// Since when the click_methods.clickfinger enum variant is available.
    pub const ENM__CLICK_METHODS_CLICKFINGER__SINCE: u32 = 1;

    /// Since when the clickfinger_button_map.lrm enum variant is available.
    pub const ENM__CLICKFINGER_BUTTON_MAP_LRM__SINCE: u32 = 1;
    /// Since when the clickfinger_button_map.lmr enum variant is available.
    pub const ENM__CLICKFINGER_BUTTON_MAP_LMR__SINCE: u32 = 1;

    /// Since when the middle_emulation_state.disabled enum variant is available.
    pub const ENM__MIDDLE_EMULATION_STATE_DISABLED__SINCE: u32 = 1;
    /// Since when the middle_emulation_state.enabled enum variant is available.
    pub const ENM__MIDDLE_EMULATION_STATE_ENABLED__SINCE: u32 = 1;

    /// Since when the scroll_method.no_scroll enum variant is available.
    pub const ENM__SCROLL_METHOD_NO_SCROLL__SINCE: u32 = 1;
    /// Since when the scroll_method.two_finger enum variant is available.
    pub const ENM__SCROLL_METHOD_TWO_FINGER__SINCE: u32 = 1;
    /// Since when the scroll_method.edge enum variant is available.
    pub const ENM__SCROLL_METHOD_EDGE__SINCE: u32 = 1;
    /// Since when the scroll_method.on_button_down enum variant is available.
    pub const ENM__SCROLL_METHOD_ON_BUTTON_DOWN__SINCE: u32 = 1;

    /// Since when the scroll_methods.no_scroll enum variant is available.
    pub const ENM__SCROLL_METHODS_NO_SCROLL__SINCE: u32 = 1;
    /// Since when the scroll_methods.two_finger enum variant is available.
    pub const ENM__SCROLL_METHODS_TWO_FINGER__SINCE: u32 = 1;
    /// Since when the scroll_methods.edge enum variant is available.
    pub const ENM__SCROLL_METHODS_EDGE__SINCE: u32 = 1;
    /// Since when the scroll_methods.on_button_down enum variant is available.
    pub const ENM__SCROLL_METHODS_ON_BUTTON_DOWN__SINCE: u32 = 1;

    /// Since when the scroll_button_lock_state.disabled enum variant is available.
    pub const ENM__SCROLL_BUTTON_LOCK_STATE_DISABLED__SINCE: u32 = 1;
    /// Since when the scroll_button_lock_state.enabled enum variant is available.
    pub const ENM__SCROLL_BUTTON_LOCK_STATE_ENABLED__SINCE: u32 = 1;

    /// Since when the dwt_state.disabled enum variant is available.
    pub const ENM__DWT_STATE_DISABLED__SINCE: u32 = 1;
    /// Since when the dwt_state.enabled enum variant is available.
    pub const ENM__DWT_STATE_ENABLED__SINCE: u32 = 1;

    /// Since when the dwtp_state.disabled enum variant is available.
    pub const ENM__DWTP_STATE_DISABLED__SINCE: u32 = 1;
    /// Since when the dwtp_state.enabled enum variant is available.
    pub const ENM__DWTP_STATE_ENABLED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverLibinputDeviceV1Error(pub u32);

impl RiverLibinputDeviceV1Error {
    /// invalid enum value or similar
    pub const INVALID_ARG: Self = Self(0);
}

impl Debug for RiverLibinputDeviceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_ARG => "INVALID_ARG",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct RiverLibinputDeviceV1SendEventsModes(pub u32);

/// An iterator over the set bits in a [`RiverLibinputDeviceV1SendEventsModes`].
///
/// You can construct this with the `IntoIterator` implementation of `RiverLibinputDeviceV1SendEventsModes`.
#[derive(Clone, Debug)]
pub struct RiverLibinputDeviceV1SendEventsModesIter(pub u32);

impl RiverLibinputDeviceV1SendEventsModes {
    pub const ENABLED: Self = Self(0);

    pub const DISABLED: Self = Self(1);

    pub const DISABLED_ON_EXTERNAL_MOUSE: Self = Self(2);
}

impl RiverLibinputDeviceV1SendEventsModes {
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    #[inline]
    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    #[inline]
    pub const fn insert(&mut self, other: Self) {
        *self = self.union(other);
    }

    #[inline]
    pub const fn remove(&mut self, other: Self) {
        *self = self.difference(other);
    }

    #[inline]
    pub const fn toggle(&mut self, other: Self) {
        *self = self.symmetric_difference(other);
    }

    #[inline]
    pub const fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }

    #[inline]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline]
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    #[inline]
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(!self.0)
    }

    #[inline]
    #[must_use]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    #[inline]
    pub const fn all_known() -> Self {
        #[allow(clippy::eq_op, clippy::identity_op)]
        Self(0 | 0 | 1 | 2)
    }
}

impl Iterator for RiverLibinputDeviceV1SendEventsModesIter {
    type Item = RiverLibinputDeviceV1SendEventsModes;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(RiverLibinputDeviceV1SendEventsModes(bit))
    }
}

impl IntoIterator for RiverLibinputDeviceV1SendEventsModes {
    type Item = RiverLibinputDeviceV1SendEventsModes;
    type IntoIter = RiverLibinputDeviceV1SendEventsModesIter;

    fn into_iter(self) -> Self::IntoIter {
        RiverLibinputDeviceV1SendEventsModesIter(self.0)
    }
}

impl BitAnd for RiverLibinputDeviceV1SendEventsModes {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for RiverLibinputDeviceV1SendEventsModes {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for RiverLibinputDeviceV1SendEventsModes {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for RiverLibinputDeviceV1SendEventsModes {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for RiverLibinputDeviceV1SendEventsModes {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for RiverLibinputDeviceV1SendEventsModes {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for RiverLibinputDeviceV1SendEventsModes {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for RiverLibinputDeviceV1SendEventsModes {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for RiverLibinputDeviceV1SendEventsModes {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for RiverLibinputDeviceV1SendEventsModes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = self.0;
        let mut first = true;
        if v & 1 == 1 {
            v &= !1;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("DISABLED")?;
        }
        if v & 2 == 2 {
            v &= !2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("DISABLED_ON_EXTERNAL_MOUSE")?;
        }
        if v != 0 {
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            write!(f, "0x{v:032x}")?;
        }
        if first {
            f.write_str("ENABLED")?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverLibinputDeviceV1TapState(pub u32);

impl RiverLibinputDeviceV1TapState {
    pub const DISABLED: Self = Self(0);

    pub const ENABLED: Self = Self(1);
}

impl Debug for RiverLibinputDeviceV1TapState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DISABLED => "DISABLED",
            Self::ENABLED => "ENABLED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverLibinputDeviceV1TapButtonMap(pub u32);

impl RiverLibinputDeviceV1TapButtonMap {
    /// 1/2/3 finger tap maps to left/right/middle
    pub const LRM: Self = Self(0);

    /// 1/2/3 finger tap maps to left/middle/right
    pub const LMR: Self = Self(1);
}

impl Debug for RiverLibinputDeviceV1TapButtonMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::LRM => "LRM",
            Self::LMR => "LMR",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverLibinputDeviceV1DragState(pub u32);

impl RiverLibinputDeviceV1DragState {
    pub const DISABLED: Self = Self(0);

    pub const ENABLED: Self = Self(1);
}

impl Debug for RiverLibinputDeviceV1DragState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DISABLED => "DISABLED",
            Self::ENABLED => "ENABLED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverLibinputDeviceV1DragLockState(pub u32);

impl RiverLibinputDeviceV1DragLockState {
    pub const DISABLED: Self = Self(0);

    pub const ENABLED_TIMEOUT: Self = Self(1);

    pub const ENABLED_STICKY: Self = Self(2);
}

impl Debug for RiverLibinputDeviceV1DragLockState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DISABLED => "DISABLED",
            Self::ENABLED_TIMEOUT => "ENABLED_TIMEOUT",
            Self::ENABLED_STICKY => "ENABLED_STICKY",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverLibinputDeviceV1ThreeFingerDragState(pub u32);

impl RiverLibinputDeviceV1ThreeFingerDragState {
    pub const DISABLED: Self = Self(0);

    pub const ENABLED_3FG: Self = Self(1);

    pub const ENABLED_4FG: Self = Self(2);
}

impl Debug for RiverLibinputDeviceV1ThreeFingerDragState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DISABLED => "DISABLED",
            Self::ENABLED_3FG => "ENABLED_3FG",
            Self::ENABLED_4FG => "ENABLED_4FG",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverLibinputDeviceV1AccelProfile(pub u32);

impl RiverLibinputDeviceV1AccelProfile {
    pub const NONE: Self = Self(0);

    pub const FLAT: Self = Self(1);

    pub const ADAPTIVE: Self = Self(2);

    pub const CUSTOM: Self = Self(4);
}

impl Debug for RiverLibinputDeviceV1AccelProfile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NONE => "NONE",
            Self::FLAT => "FLAT",
            Self::ADAPTIVE => "ADAPTIVE",
            Self::CUSTOM => "CUSTOM",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct RiverLibinputDeviceV1AccelProfiles(pub u32);

/// An iterator over the set bits in a [`RiverLibinputDeviceV1AccelProfiles`].
///
/// You can construct this with the `IntoIterator` implementation of `RiverLibinputDeviceV1AccelProfiles`.
#[derive(Clone, Debug)]
pub struct RiverLibinputDeviceV1AccelProfilesIter(pub u32);

impl RiverLibinputDeviceV1AccelProfiles {
    pub const NONE: Self = Self(0);

    pub const FLAT: Self = Self(1);

    pub const ADAPTIVE: Self = Self(2);

    pub const CUSTOM: Self = Self(4);
}

impl RiverLibinputDeviceV1AccelProfiles {
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    #[inline]
    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    #[inline]
    pub const fn insert(&mut self, other: Self) {
        *self = self.union(other);
    }

    #[inline]
    pub const fn remove(&mut self, other: Self) {
        *self = self.difference(other);
    }

    #[inline]
    pub const fn toggle(&mut self, other: Self) {
        *self = self.symmetric_difference(other);
    }

    #[inline]
    pub const fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }

    #[inline]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline]
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    #[inline]
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(!self.0)
    }

    #[inline]
    #[must_use]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    #[inline]
    pub const fn all_known() -> Self {
        #[allow(clippy::eq_op, clippy::identity_op)]
        Self(0 | 0 | 1 | 2 | 4)
    }
}

impl Iterator for RiverLibinputDeviceV1AccelProfilesIter {
    type Item = RiverLibinputDeviceV1AccelProfiles;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(RiverLibinputDeviceV1AccelProfiles(bit))
    }
}

impl IntoIterator for RiverLibinputDeviceV1AccelProfiles {
    type Item = RiverLibinputDeviceV1AccelProfiles;
    type IntoIter = RiverLibinputDeviceV1AccelProfilesIter;

    fn into_iter(self) -> Self::IntoIter {
        RiverLibinputDeviceV1AccelProfilesIter(self.0)
    }
}

impl BitAnd for RiverLibinputDeviceV1AccelProfiles {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for RiverLibinputDeviceV1AccelProfiles {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for RiverLibinputDeviceV1AccelProfiles {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for RiverLibinputDeviceV1AccelProfiles {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for RiverLibinputDeviceV1AccelProfiles {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for RiverLibinputDeviceV1AccelProfiles {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for RiverLibinputDeviceV1AccelProfiles {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for RiverLibinputDeviceV1AccelProfiles {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for RiverLibinputDeviceV1AccelProfiles {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for RiverLibinputDeviceV1AccelProfiles {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = self.0;
        let mut first = true;
        if v & 1 == 1 {
            v &= !1;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("FLAT")?;
        }
        if v & 2 == 2 {
            v &= !2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("ADAPTIVE")?;
        }
        if v & 4 == 4 {
            v &= !4;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("CUSTOM")?;
        }
        if v != 0 {
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            write!(f, "0x{v:032x}")?;
        }
        if first {
            f.write_str("NONE")?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverLibinputDeviceV1NaturalScrollState(pub u32);

impl RiverLibinputDeviceV1NaturalScrollState {
    pub const DISABLED: Self = Self(0);

    pub const ENABLED: Self = Self(1);
}

impl Debug for RiverLibinputDeviceV1NaturalScrollState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DISABLED => "DISABLED",
            Self::ENABLED => "ENABLED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverLibinputDeviceV1LeftHandedState(pub u32);

impl RiverLibinputDeviceV1LeftHandedState {
    pub const DISABLED: Self = Self(0);

    pub const ENABLED: Self = Self(1);
}

impl Debug for RiverLibinputDeviceV1LeftHandedState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DISABLED => "DISABLED",
            Self::ENABLED => "ENABLED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverLibinputDeviceV1ClickMethod(pub u32);

impl RiverLibinputDeviceV1ClickMethod {
    pub const NONE: Self = Self(0);

    pub const BUTTON_AREAS: Self = Self(1);

    pub const CLICKFINGER: Self = Self(2);
}

impl Debug for RiverLibinputDeviceV1ClickMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NONE => "NONE",
            Self::BUTTON_AREAS => "BUTTON_AREAS",
            Self::CLICKFINGER => "CLICKFINGER",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct RiverLibinputDeviceV1ClickMethods(pub u32);

/// An iterator over the set bits in a [`RiverLibinputDeviceV1ClickMethods`].
///
/// You can construct this with the `IntoIterator` implementation of `RiverLibinputDeviceV1ClickMethods`.
#[derive(Clone, Debug)]
pub struct RiverLibinputDeviceV1ClickMethodsIter(pub u32);

impl RiverLibinputDeviceV1ClickMethods {
    pub const NONE: Self = Self(0);

    pub const BUTTON_AREAS: Self = Self(1);

    pub const CLICKFINGER: Self = Self(2);
}

impl RiverLibinputDeviceV1ClickMethods {
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    #[inline]
    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    #[inline]
    pub const fn insert(&mut self, other: Self) {
        *self = self.union(other);
    }

    #[inline]
    pub const fn remove(&mut self, other: Self) {
        *self = self.difference(other);
    }

    #[inline]
    pub const fn toggle(&mut self, other: Self) {
        *self = self.symmetric_difference(other);
    }

    #[inline]
    pub const fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }

    #[inline]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline]
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    #[inline]
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(!self.0)
    }

    #[inline]
    #[must_use]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    #[inline]
    pub const fn all_known() -> Self {
        #[allow(clippy::eq_op, clippy::identity_op)]
        Self(0 | 0 | 1 | 2)
    }
}

impl Iterator for RiverLibinputDeviceV1ClickMethodsIter {
    type Item = RiverLibinputDeviceV1ClickMethods;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(RiverLibinputDeviceV1ClickMethods(bit))
    }
}

impl IntoIterator for RiverLibinputDeviceV1ClickMethods {
    type Item = RiverLibinputDeviceV1ClickMethods;
    type IntoIter = RiverLibinputDeviceV1ClickMethodsIter;

    fn into_iter(self) -> Self::IntoIter {
        RiverLibinputDeviceV1ClickMethodsIter(self.0)
    }
}

impl BitAnd for RiverLibinputDeviceV1ClickMethods {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for RiverLibinputDeviceV1ClickMethods {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for RiverLibinputDeviceV1ClickMethods {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for RiverLibinputDeviceV1ClickMethods {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for RiverLibinputDeviceV1ClickMethods {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for RiverLibinputDeviceV1ClickMethods {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for RiverLibinputDeviceV1ClickMethods {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for RiverLibinputDeviceV1ClickMethods {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for RiverLibinputDeviceV1ClickMethods {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for RiverLibinputDeviceV1ClickMethods {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = self.0;
        let mut first = true;
        if v & 1 == 1 {
            v &= !1;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("BUTTON_AREAS")?;
        }
        if v & 2 == 2 {
            v &= !2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("CLICKFINGER")?;
        }
        if v != 0 {
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            write!(f, "0x{v:032x}")?;
        }
        if first {
            f.write_str("NONE")?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverLibinputDeviceV1ClickfingerButtonMap(pub u32);

impl RiverLibinputDeviceV1ClickfingerButtonMap {
    pub const LRM: Self = Self(0);

    pub const LMR: Self = Self(1);
}

impl Debug for RiverLibinputDeviceV1ClickfingerButtonMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::LRM => "LRM",
            Self::LMR => "LMR",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverLibinputDeviceV1MiddleEmulationState(pub u32);

impl RiverLibinputDeviceV1MiddleEmulationState {
    pub const DISABLED: Self = Self(0);

    pub const ENABLED: Self = Self(1);
}

impl Debug for RiverLibinputDeviceV1MiddleEmulationState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DISABLED => "DISABLED",
            Self::ENABLED => "ENABLED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverLibinputDeviceV1ScrollMethod(pub u32);

impl RiverLibinputDeviceV1ScrollMethod {
    pub const NO_SCROLL: Self = Self(0);

    pub const TWO_FINGER: Self = Self(1);

    pub const EDGE: Self = Self(2);

    pub const ON_BUTTON_DOWN: Self = Self(4);
}

impl Debug for RiverLibinputDeviceV1ScrollMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NO_SCROLL => "NO_SCROLL",
            Self::TWO_FINGER => "TWO_FINGER",
            Self::EDGE => "EDGE",
            Self::ON_BUTTON_DOWN => "ON_BUTTON_DOWN",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct RiverLibinputDeviceV1ScrollMethods(pub u32);

/// An iterator over the set bits in a [`RiverLibinputDeviceV1ScrollMethods`].
///
/// You can construct this with the `IntoIterator` implementation of `RiverLibinputDeviceV1ScrollMethods`.
#[derive(Clone, Debug)]
pub struct RiverLibinputDeviceV1ScrollMethodsIter(pub u32);

impl RiverLibinputDeviceV1ScrollMethods {
    pub const NO_SCROLL: Self = Self(0);

    pub const TWO_FINGER: Self = Self(1);

    pub const EDGE: Self = Self(2);

    pub const ON_BUTTON_DOWN: Self = Self(4);
}

impl RiverLibinputDeviceV1ScrollMethods {
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    #[inline]
    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    #[inline]
    pub const fn insert(&mut self, other: Self) {
        *self = self.union(other);
    }

    #[inline]
    pub const fn remove(&mut self, other: Self) {
        *self = self.difference(other);
    }

    #[inline]
    pub const fn toggle(&mut self, other: Self) {
        *self = self.symmetric_difference(other);
    }

    #[inline]
    pub const fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }

    #[inline]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline]
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    #[inline]
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(!self.0)
    }

    #[inline]
    #[must_use]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    #[inline]
    pub const fn all_known() -> Self {
        #[allow(clippy::eq_op, clippy::identity_op)]
        Self(0 | 0 | 1 | 2 | 4)
    }
}

impl Iterator for RiverLibinputDeviceV1ScrollMethodsIter {
    type Item = RiverLibinputDeviceV1ScrollMethods;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(RiverLibinputDeviceV1ScrollMethods(bit))
    }
}

impl IntoIterator for RiverLibinputDeviceV1ScrollMethods {
    type Item = RiverLibinputDeviceV1ScrollMethods;
    type IntoIter = RiverLibinputDeviceV1ScrollMethodsIter;

    fn into_iter(self) -> Self::IntoIter {
        RiverLibinputDeviceV1ScrollMethodsIter(self.0)
    }
}

impl BitAnd for RiverLibinputDeviceV1ScrollMethods {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for RiverLibinputDeviceV1ScrollMethods {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for RiverLibinputDeviceV1ScrollMethods {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for RiverLibinputDeviceV1ScrollMethods {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for RiverLibinputDeviceV1ScrollMethods {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for RiverLibinputDeviceV1ScrollMethods {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for RiverLibinputDeviceV1ScrollMethods {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for RiverLibinputDeviceV1ScrollMethods {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for RiverLibinputDeviceV1ScrollMethods {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for RiverLibinputDeviceV1ScrollMethods {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = self.0;
        let mut first = true;
        if v & 1 == 1 {
            v &= !1;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("TWO_FINGER")?;
        }
        if v & 2 == 2 {
            v &= !2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("EDGE")?;
        }
        if v & 4 == 4 {
            v &= !4;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("ON_BUTTON_DOWN")?;
        }
        if v != 0 {
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            write!(f, "0x{v:032x}")?;
        }
        if first {
            f.write_str("NO_SCROLL")?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverLibinputDeviceV1ScrollButtonLockState(pub u32);

impl RiverLibinputDeviceV1ScrollButtonLockState {
    pub const DISABLED: Self = Self(0);

    pub const ENABLED: Self = Self(1);
}

impl Debug for RiverLibinputDeviceV1ScrollButtonLockState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DISABLED => "DISABLED",
            Self::ENABLED => "ENABLED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverLibinputDeviceV1DwtState(pub u32);

impl RiverLibinputDeviceV1DwtState {
    pub const DISABLED: Self = Self(0);

    pub const ENABLED: Self = Self(1);
}

impl Debug for RiverLibinputDeviceV1DwtState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DISABLED => "DISABLED",
            Self::ENABLED => "ENABLED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverLibinputDeviceV1DwtpState(pub u32);

impl RiverLibinputDeviceV1DwtpState {
    pub const DISABLED: Self = Self(0);

    pub const ENABLED: Self = Self(1);
}

impl Debug for RiverLibinputDeviceV1DwtpState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DISABLED => "DISABLED",
            Self::ENABLED => "ENABLED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
