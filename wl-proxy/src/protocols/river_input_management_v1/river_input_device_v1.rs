//! an input device
//!
//! An input device represents a physical keyboard, mouse, touchscreen, or
//! drawing tablet tool. It is assigned to exactly one seat at a time.
//! By default, all input devices are assigned to the default seat.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A river_input_device_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct RiverInputDeviceV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn RiverInputDeviceV1Handler>,
}

struct DefaultHandler;

impl RiverInputDeviceV1Handler for DefaultHandler { }

impl ConcreteObject for RiverInputDeviceV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::RiverInputDeviceV1;
    const INTERFACE_NAME: &str = "river_input_device_v1";
}

impl RiverInputDeviceV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl RiverInputDeviceV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn RiverInputDeviceV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for RiverInputDeviceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RiverInputDeviceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl RiverInputDeviceV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the input device object
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_input_device_v1#{}.destroy()\n", id);
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

    /// destroy the input device object
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
            log_send("river_input_device_v1.destroy", &e);
        }
    }

    /// Since when the removed message is available.
    pub const MSG__REMOVED__SINCE: u32 = 1;

    /// the input device is removed
    ///
    /// This event indicates that the input device has been removed.
    ///
    /// The server will send no further events on this object and ignore any
    /// request (other than river_input_device_v1.destroy) made after this event is
    /// sent. The client should destroy this object with the
    /// river_input_device_v1.destroy request to free up resources.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_input_device_v1#{}.removed()\n", client_id, id);
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

    /// the input device is removed
    ///
    /// This event indicates that the input device has been removed.
    ///
    /// The server will send no further events on this object and ignore any
    /// request (other than river_input_device_v1.destroy) made after this event is
    /// sent. The client should destroy this object with the
    /// river_input_device_v1.destroy request to free up resources.
    #[inline]
    pub fn send_removed(
        &self,
    ) {
        let res = self.try_send_removed(
        );
        if let Err(e) = res {
            log_send("river_input_device_v1.removed", &e);
        }
    }

    /// Since when the type message is available.
    pub const MSG__TYPE__SINCE: u32 = 1;

    /// the type of the input device
    ///
    /// The type of the input device. This event is sent once when the
    /// river_input_device_v1 object is created. The device type cannot
    /// change during the lifetime of the object.
    ///
    /// # Arguments
    ///
    /// - `r#type`:
    #[inline]
    pub fn try_send_type(
        &self,
        r#type: RiverInputDeviceV1Type,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            r#type,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverInputDeviceV1Type) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_input_device_v1#{}.type(type: {:?})\n", client_id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// the type of the input device
    ///
    /// The type of the input device. This event is sent once when the
    /// river_input_device_v1 object is created. The device type cannot
    /// change during the lifetime of the object.
    ///
    /// # Arguments
    ///
    /// - `r#type`:
    #[inline]
    pub fn send_type(
        &self,
        r#type: RiverInputDeviceV1Type,
    ) {
        let res = self.try_send_type(
            r#type,
        );
        if let Err(e) = res {
            log_send("river_input_device_v1.type", &e);
        }
    }

    /// Since when the name message is available.
    pub const MSG__NAME__SINCE: u32 = 1;

    /// the name of the input device
    ///
    /// The name of the input device. This event is sent once when the
    /// river_input_device_v1 object is created. The device name cannot
    /// change during the lifetime of the object.
    ///
    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    pub fn try_send_name(
        &self,
        name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_input_device_v1#{}.name(name: {:?})\n", client_id, id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// the name of the input device
    ///
    /// The name of the input device. This event is sent once when the
    /// river_input_device_v1 object is created. The device name cannot
    /// change during the lifetime of the object.
    ///
    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    pub fn send_name(
        &self,
        name: &str,
    ) {
        let res = self.try_send_name(
            name,
        );
        if let Err(e) = res {
            log_send("river_input_device_v1.name", &e);
        }
    }

    /// Since when the assign_to_seat message is available.
    pub const MSG__ASSIGN_TO_SEAT__SINCE: u32 = 1;

    /// assign the input device to a seat
    ///
    /// Assign the input device to a seat. All input devices not explicitly
    /// assigned to a seat are considered assigned to the default seat.
    ///
    /// Has no effect if a seat with the given name does not exist.
    ///
    /// # Arguments
    ///
    /// - `name`: name of the seat
    #[inline]
    pub fn try_send_assign_to_seat(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_input_device_v1#{}.assign_to_seat(name: {:?})\n", id, arg0);
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
            1,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// assign the input device to a seat
    ///
    /// Assign the input device to a seat. All input devices not explicitly
    /// assigned to a seat are considered assigned to the default seat.
    ///
    /// Has no effect if a seat with the given name does not exist.
    ///
    /// # Arguments
    ///
    /// - `name`: name of the seat
    #[inline]
    pub fn send_assign_to_seat(
        &self,
        name: &str,
    ) {
        let res = self.try_send_assign_to_seat(
            name,
        );
        if let Err(e) = res {
            log_send("river_input_device_v1.assign_to_seat", &e);
        }
    }

    /// Since when the set_repeat_info message is available.
    pub const MSG__SET_REPEAT_INFO__SINCE: u32 = 1;

    /// set keyboard repeat rate and delay
    ///
    /// Set repeat rate and delay for a keyboard input device. Has no effect if
    /// the device is not a keyboard.
    ///
    /// Negative values for either rate or delay are illegal. A rate of zero
    /// will disable any repeating (regardless of the value of delay).
    ///
    /// # Arguments
    ///
    /// - `rate`: rate in key repeats per second
    /// - `delay`: delay in milliseconds
    #[inline]
    pub fn try_send_set_repeat_info(
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_input_device_v1#{}.set_repeat_info(rate: {}, delay: {})\n", id, arg0, arg1);
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
            2,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// set keyboard repeat rate and delay
    ///
    /// Set repeat rate and delay for a keyboard input device. Has no effect if
    /// the device is not a keyboard.
    ///
    /// Negative values for either rate or delay are illegal. A rate of zero
    /// will disable any repeating (regardless of the value of delay).
    ///
    /// # Arguments
    ///
    /// - `rate`: rate in key repeats per second
    /// - `delay`: delay in milliseconds
    #[inline]
    pub fn send_set_repeat_info(
        &self,
        rate: i32,
        delay: i32,
    ) {
        let res = self.try_send_set_repeat_info(
            rate,
            delay,
        );
        if let Err(e) = res {
            log_send("river_input_device_v1.set_repeat_info", &e);
        }
    }

    /// Since when the set_scroll_factor message is available.
    pub const MSG__SET_SCROLL_FACTOR__SINCE: u32 = 1;

    /// set scroll factor
    ///
    /// Set the scroll factor for a pointer input device. Has no effect if the
    /// device is not a pointer.
    ///
    /// For example, a factor of 0.5 will make scrolling twice as slow while a
    /// factor of 3.0 will make scrolling 3 times as fast.
    ///
    /// Negative values for either rate or delay are illegal. A rate of zero
    /// will disable any repeating (regardless of the value of delay).
    ///
    /// # Arguments
    ///
    /// - `factor`:
    #[inline]
    pub fn try_send_set_scroll_factor(
        &self,
        factor: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            factor,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: Fixed) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_input_device_v1#{}.set_scroll_factor(factor: {})\n", id, arg0);
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
            arg0.to_wire() as u32,
        ]);
        Ok(())
    }

    /// set scroll factor
    ///
    /// Set the scroll factor for a pointer input device. Has no effect if the
    /// device is not a pointer.
    ///
    /// For example, a factor of 0.5 will make scrolling twice as slow while a
    /// factor of 3.0 will make scrolling 3 times as fast.
    ///
    /// Negative values for either rate or delay are illegal. A rate of zero
    /// will disable any repeating (regardless of the value of delay).
    ///
    /// # Arguments
    ///
    /// - `factor`:
    #[inline]
    pub fn send_set_scroll_factor(
        &self,
        factor: Fixed,
    ) {
        let res = self.try_send_set_scroll_factor(
            factor,
        );
        if let Err(e) = res {
            log_send("river_input_device_v1.set_scroll_factor", &e);
        }
    }

    /// Since when the map_to_output message is available.
    pub const MSG__MAP_TO_OUTPUT__SINCE: u32 = 1;

    /// map input device to the given output
    ///
    /// Map the input device to the given output. Has no effect if the device is
    /// not a pointer, touch, or tablet device.
    ///
    /// If mapped to both an output and a rectangle, the rectangle has priority.
    ///
    /// Passing null clears an existing mapping.
    ///
    /// # Arguments
    ///
    /// - `output`:
    #[inline]
    pub fn try_send_map_to_output(
        &self,
        output: Option<&Rc<WlOutput>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            output,
        );
        let arg0 = arg0.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("output"))),
                Some(id) => id,
            },
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_input_device_v1#{}.map_to_output(output: wl_output#{})\n", id, arg0);
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
            4,
            arg0_id,
        ]);
        Ok(())
    }

    /// map input device to the given output
    ///
    /// Map the input device to the given output. Has no effect if the device is
    /// not a pointer, touch, or tablet device.
    ///
    /// If mapped to both an output and a rectangle, the rectangle has priority.
    ///
    /// Passing null clears an existing mapping.
    ///
    /// # Arguments
    ///
    /// - `output`:
    #[inline]
    pub fn send_map_to_output(
        &self,
        output: Option<&Rc<WlOutput>>,
    ) {
        let res = self.try_send_map_to_output(
            output,
        );
        if let Err(e) = res {
            log_send("river_input_device_v1.map_to_output", &e);
        }
    }

    /// Since when the map_to_rectangle message is available.
    pub const MSG__MAP_TO_RECTANGLE__SINCE: u32 = 1;

    /// map input device to the given rectangle
    ///
    /// Map the input device to the given rectangle in the global compositor
    /// coordinate space. Has no effect if the device is not a pointer, touch,
    /// or tablet device.
    ///
    /// If mapped to both an output and a rectangle, the rectangle has priority.
    ///
    /// Width and height must be greater than or equal to 0.
    ///
    /// Passing 0 for width or height clears an existing mapping.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn try_send_map_to_rectangle(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            x,
            y,
            width,
            height,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_input_device_v1#{}.map_to_rectangle(x: {}, y: {}, width: {}, height: {})\n", id, arg0, arg1, arg2, arg3);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2, arg3);
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
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// map input device to the given rectangle
    ///
    /// Map the input device to the given rectangle in the global compositor
    /// coordinate space. Has no effect if the device is not a pointer, touch,
    /// or tablet device.
    ///
    /// If mapped to both an output and a rectangle, the rectangle has priority.
    ///
    /// Width and height must be greater than or equal to 0.
    ///
    /// Passing 0 for width or height clears an existing mapping.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn send_map_to_rectangle(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_map_to_rectangle(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_send("river_input_device_v1.map_to_rectangle", &e);
        }
    }
}

/// A message handler for [`RiverInputDeviceV1`] proxies.
pub trait RiverInputDeviceV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<RiverInputDeviceV1>) {
        slf.core.delete_id();
    }

    /// destroy the input device object
    ///
    /// This request indicates that the client will no longer use the input
    /// device object and that it may be safely destroyed.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<RiverInputDeviceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("river_input_device_v1.destroy", &e);
        }
    }

    /// the input device is removed
    ///
    /// This event indicates that the input device has been removed.
    ///
    /// The server will send no further events on this object and ignore any
    /// request (other than river_input_device_v1.destroy) made after this event is
    /// sent. The client should destroy this object with the
    /// river_input_device_v1.destroy request to free up resources.
    #[inline]
    fn handle_removed(
        &mut self,
        slf: &Rc<RiverInputDeviceV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_removed(
        );
        if let Err(e) = res {
            log_forward("river_input_device_v1.removed", &e);
        }
    }

    /// the type of the input device
    ///
    /// The type of the input device. This event is sent once when the
    /// river_input_device_v1 object is created. The device type cannot
    /// change during the lifetime of the object.
    ///
    /// # Arguments
    ///
    /// - `r#type`:
    #[inline]
    fn handle_type(
        &mut self,
        slf: &Rc<RiverInputDeviceV1>,
        r#type: RiverInputDeviceV1Type,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_type(
            r#type,
        );
        if let Err(e) = res {
            log_forward("river_input_device_v1.type", &e);
        }
    }

    /// the name of the input device
    ///
    /// The name of the input device. This event is sent once when the
    /// river_input_device_v1 object is created. The device name cannot
    /// change during the lifetime of the object.
    ///
    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    fn handle_name(
        &mut self,
        slf: &Rc<RiverInputDeviceV1>,
        name: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_name(
            name,
        );
        if let Err(e) = res {
            log_forward("river_input_device_v1.name", &e);
        }
    }

    /// assign the input device to a seat
    ///
    /// Assign the input device to a seat. All input devices not explicitly
    /// assigned to a seat are considered assigned to the default seat.
    ///
    /// Has no effect if a seat with the given name does not exist.
    ///
    /// # Arguments
    ///
    /// - `name`: name of the seat
    #[inline]
    fn handle_assign_to_seat(
        &mut self,
        slf: &Rc<RiverInputDeviceV1>,
        name: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_assign_to_seat(
            name,
        );
        if let Err(e) = res {
            log_forward("river_input_device_v1.assign_to_seat", &e);
        }
    }

    /// set keyboard repeat rate and delay
    ///
    /// Set repeat rate and delay for a keyboard input device. Has no effect if
    /// the device is not a keyboard.
    ///
    /// Negative values for either rate or delay are illegal. A rate of zero
    /// will disable any repeating (regardless of the value of delay).
    ///
    /// # Arguments
    ///
    /// - `rate`: rate in key repeats per second
    /// - `delay`: delay in milliseconds
    #[inline]
    fn handle_set_repeat_info(
        &mut self,
        slf: &Rc<RiverInputDeviceV1>,
        rate: i32,
        delay: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_repeat_info(
            rate,
            delay,
        );
        if let Err(e) = res {
            log_forward("river_input_device_v1.set_repeat_info", &e);
        }
    }

    /// set scroll factor
    ///
    /// Set the scroll factor for a pointer input device. Has no effect if the
    /// device is not a pointer.
    ///
    /// For example, a factor of 0.5 will make scrolling twice as slow while a
    /// factor of 3.0 will make scrolling 3 times as fast.
    ///
    /// Negative values for either rate or delay are illegal. A rate of zero
    /// will disable any repeating (regardless of the value of delay).
    ///
    /// # Arguments
    ///
    /// - `factor`:
    #[inline]
    fn handle_set_scroll_factor(
        &mut self,
        slf: &Rc<RiverInputDeviceV1>,
        factor: Fixed,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_scroll_factor(
            factor,
        );
        if let Err(e) = res {
            log_forward("river_input_device_v1.set_scroll_factor", &e);
        }
    }

    /// map input device to the given output
    ///
    /// Map the input device to the given output. Has no effect if the device is
    /// not a pointer, touch, or tablet device.
    ///
    /// If mapped to both an output and a rectangle, the rectangle has priority.
    ///
    /// Passing null clears an existing mapping.
    ///
    /// # Arguments
    ///
    /// - `output`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_map_to_output(
        &mut self,
        slf: &Rc<RiverInputDeviceV1>,
        output: Option<&Rc<WlOutput>>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_map_to_output(
            output,
        );
        if let Err(e) = res {
            log_forward("river_input_device_v1.map_to_output", &e);
        }
    }

    /// map input device to the given rectangle
    ///
    /// Map the input device to the given rectangle in the global compositor
    /// coordinate space. Has no effect if the device is not a pointer, touch,
    /// or tablet device.
    ///
    /// If mapped to both an output and a rectangle, the rectangle has priority.
    ///
    /// Width and height must be greater than or equal to 0.
    ///
    /// Passing 0 for width or height clears an existing mapping.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    fn handle_map_to_rectangle(
        &mut self,
        slf: &Rc<RiverInputDeviceV1>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_map_to_rectangle(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("river_input_device_v1.map_to_rectangle", &e);
        }
    }
}

impl ObjectPrivate for RiverInputDeviceV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::RiverInputDeviceV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_input_device_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_input_device_v1#{}.assign_to_seat(name: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_assign_to_seat(&self, arg0);
                } else {
                    DefaultHandler.handle_assign_to_seat(&self, arg0);
                }
            }
            2 => {
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
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_input_device_v1#{}.set_repeat_info(rate: {}, delay: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_repeat_info(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_repeat_info(&self, arg0, arg1);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = Fixed::from_wire(arg0 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: Fixed) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_input_device_v1#{}.set_scroll_factor(factor: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_scroll_factor(&self, arg0);
                } else {
                    DefaultHandler.handle_set_scroll_factor(&self, arg0);
                }
            }
            4 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_input_device_v1#{}.map_to_output(output: wl_output#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlOutput>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("output", o.core().interface, ObjectInterface::WlOutput)));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_map_to_output(&self, arg0);
                } else {
                    DefaultHandler.handle_map_to_output(&self, arg0);
                }
            }
            5 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 24)));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_input_device_v1#{}.map_to_rectangle(x: {}, y: {}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_map_to_rectangle(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_map_to_rectangle(&self, arg0, arg1, arg2, arg3);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_input_device_v1#{}.removed()\n", id);
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
                let arg0 = RiverInputDeviceV1Type(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverInputDeviceV1Type) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_input_device_v1#{}.type(type: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_type(&self, arg0);
                } else {
                    DefaultHandler.handle_type(&self, arg0);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "name")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_input_device_v1#{}.name(name: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_name(&self, arg0);
                } else {
                    DefaultHandler.handle_name(&self, arg0);
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
            1 => "assign_to_seat",
            2 => "set_repeat_info",
            3 => "set_scroll_factor",
            4 => "map_to_output",
            5 => "map_to_rectangle",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "removed",
            1 => "type",
            2 => "name",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for RiverInputDeviceV1 {
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

impl RiverInputDeviceV1 {
    /// Since when the error.invalid_repeat_info enum variant is available.
    pub const ENM__ERROR_INVALID_REPEAT_INFO__SINCE: u32 = 1;
    /// Since when the error.invalid_scroll_factor enum variant is available.
    pub const ENM__ERROR_INVALID_SCROLL_FACTOR__SINCE: u32 = 1;
    /// Since when the error.invalid_map_to_rectangle enum variant is available.
    pub const ENM__ERROR_INVALID_MAP_TO_RECTANGLE__SINCE: u32 = 1;

    /// Since when the type.keyboard enum variant is available.
    pub const ENM__TYPE_KEYBOARD__SINCE: u32 = 1;
    /// Since when the type.pointer enum variant is available.
    pub const ENM__TYPE_POINTER__SINCE: u32 = 1;
    /// Since when the type.touch enum variant is available.
    pub const ENM__TYPE_TOUCH__SINCE: u32 = 1;
    /// Since when the type.tablet enum variant is available.
    pub const ENM__TYPE_TABLET__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverInputDeviceV1Error(pub u32);

impl RiverInputDeviceV1Error {
    pub const INVALID_REPEAT_INFO: Self = Self(0);

    pub const INVALID_SCROLL_FACTOR: Self = Self(1);

    pub const INVALID_MAP_TO_RECTANGLE: Self = Self(2);
}

impl Debug for RiverInputDeviceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_REPEAT_INFO => "INVALID_REPEAT_INFO",
            Self::INVALID_SCROLL_FACTOR => "INVALID_SCROLL_FACTOR",
            Self::INVALID_MAP_TO_RECTANGLE => "INVALID_MAP_TO_RECTANGLE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverInputDeviceV1Type(pub u32);

impl RiverInputDeviceV1Type {
    pub const KEYBOARD: Self = Self(0);

    pub const POINTER: Self = Self(1);

    pub const TOUCH: Self = Self(2);

    pub const TABLET: Self = Self(3);
}

impl Debug for RiverInputDeviceV1Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::KEYBOARD => "KEYBOARD",
            Self::POINTER => "POINTER",
            Self::TOUCH => "TOUCH",
            Self::TABLET => "TABLET",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
