//! output device
//!
//! A head is an output device. The difference between a wl_output object and
//! a head is that heads are advertised even if they are turned off. A head
//! object only advertises properties and cannot be used directly to change
//! them.
//!
//! A head has some read-only properties: modes, name, description and
//! physical_size. These cannot be changed by clients.
//!
//! Other properties can be updated via a wlr_output_configuration object.
//!
//! Properties sent via this interface are applied atomically via the
//! wlr_output_manager.done event. No guarantees are made regarding the order
//! in which properties are sent.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwlr_output_head_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwlrOutputHeadV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwlrOutputHeadV1Handler>,
}

struct DefaultHandler;

impl ZwlrOutputHeadV1Handler for DefaultHandler { }

impl ConcreteObject for ZwlrOutputHeadV1 {
    const XML_VERSION: u32 = 4;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwlrOutputHeadV1;
    const INTERFACE_NAME: &str = "zwlr_output_head_v1";
}

impl ZwlrOutputHeadV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwlrOutputHeadV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwlrOutputHeadV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwlrOutputHeadV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwlrOutputHeadV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwlrOutputHeadV1 {
    /// Since when the name message is available.
    pub const MSG__NAME__SINCE: u32 = 1;

    /// head name
    ///
    /// This event describes the head name.
    ///
    /// The naming convention is compositor defined, but limited to alphanumeric
    /// characters and dashes (-). Each name is unique among all wlr_output_head
    /// objects, but if a wlr_output_head object is destroyed the same name may
    /// be reused later. The names will also remain consistent across sessions
    /// with the same hardware and software configuration.
    ///
    /// Examples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc. However, do
    /// not assume that the name is a reflection of an underlying DRM
    /// connector, X11 connection, etc.
    ///
    /// If this head matches a wl_output, the wl_output.name event must report
    /// the same name.
    ///
    /// The name event is sent after a wlr_output_head object is created. This
    /// event is only sent once per object, and the name does not change over
    /// the lifetime of the wlr_output_head object.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_head_v1#{}.name(name: {:?})\n", client_id, id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// head name
    ///
    /// This event describes the head name.
    ///
    /// The naming convention is compositor defined, but limited to alphanumeric
    /// characters and dashes (-). Each name is unique among all wlr_output_head
    /// objects, but if a wlr_output_head object is destroyed the same name may
    /// be reused later. The names will also remain consistent across sessions
    /// with the same hardware and software configuration.
    ///
    /// Examples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc. However, do
    /// not assume that the name is a reflection of an underlying DRM
    /// connector, X11 connection, etc.
    ///
    /// If this head matches a wl_output, the wl_output.name event must report
    /// the same name.
    ///
    /// The name event is sent after a wlr_output_head object is created. This
    /// event is only sent once per object, and the name does not change over
    /// the lifetime of the wlr_output_head object.
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
            log_send("zwlr_output_head_v1.name", &e);
        }
    }

    /// Since when the description message is available.
    pub const MSG__DESCRIPTION__SINCE: u32 = 1;

    /// head description
    ///
    /// This event describes a human-readable description of the head.
    ///
    /// The description is a UTF-8 string with no convention defined for its
    /// contents. Examples might include 'Foocorp 11" Display' or 'Virtual X11
    /// output via :1'. However, do not assume that the name is a reflection of
    /// the make, model, serial of the underlying DRM connector or the display
    /// name of the underlying X11 connection, etc.
    ///
    /// If this head matches a wl_output, the wl_output.description event must
    /// report the same name.
    ///
    /// The description event is sent after a wlr_output_head object is created.
    /// This event is only sent once per object, and the description does not
    /// change over the lifetime of the wlr_output_head object.
    ///
    /// # Arguments
    ///
    /// - `description`:
    #[inline]
    pub fn try_send_description(
        &self,
        description: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            description,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_head_v1#{}.description(description: {:?})\n", client_id, id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// head description
    ///
    /// This event describes a human-readable description of the head.
    ///
    /// The description is a UTF-8 string with no convention defined for its
    /// contents. Examples might include 'Foocorp 11" Display' or 'Virtual X11
    /// output via :1'. However, do not assume that the name is a reflection of
    /// the make, model, serial of the underlying DRM connector or the display
    /// name of the underlying X11 connection, etc.
    ///
    /// If this head matches a wl_output, the wl_output.description event must
    /// report the same name.
    ///
    /// The description event is sent after a wlr_output_head object is created.
    /// This event is only sent once per object, and the description does not
    /// change over the lifetime of the wlr_output_head object.
    ///
    /// # Arguments
    ///
    /// - `description`:
    #[inline]
    pub fn send_description(
        &self,
        description: &str,
    ) {
        let res = self.try_send_description(
            description,
        );
        if let Err(e) = res {
            log_send("zwlr_output_head_v1.description", &e);
        }
    }

    /// Since when the physical_size message is available.
    pub const MSG__PHYSICAL_SIZE__SINCE: u32 = 1;

    /// head physical size
    ///
    /// This event describes the physical size of the head. This event is only
    /// sent if the head has a physical size (e.g. is not a projector or a
    /// virtual device).
    ///
    /// The physical size event is sent after a wlr_output_head object is created. This
    /// event is only sent once per object, and the physical size does not change over
    /// the lifetime of the wlr_output_head object.
    ///
    /// # Arguments
    ///
    /// - `width`: width in millimeters of the output
    /// - `height`: height in millimeters of the output
    #[inline]
    pub fn try_send_physical_size(
        &self,
        width: i32,
        height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            width,
            height,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_head_v1#{}.physical_size(width: {}, height: {})\n", client_id, id, arg0, arg1);
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
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// head physical size
    ///
    /// This event describes the physical size of the head. This event is only
    /// sent if the head has a physical size (e.g. is not a projector or a
    /// virtual device).
    ///
    /// The physical size event is sent after a wlr_output_head object is created. This
    /// event is only sent once per object, and the physical size does not change over
    /// the lifetime of the wlr_output_head object.
    ///
    /// # Arguments
    ///
    /// - `width`: width in millimeters of the output
    /// - `height`: height in millimeters of the output
    #[inline]
    pub fn send_physical_size(
        &self,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_physical_size(
            width,
            height,
        );
        if let Err(e) = res {
            log_send("zwlr_output_head_v1.physical_size", &e);
        }
    }

    /// Since when the mode message is available.
    pub const MSG__MODE__SINCE: u32 = 1;

    /// introduce a mode
    ///
    /// This event introduces a mode for this head. It is sent once per
    /// supported mode.
    ///
    /// # Arguments
    ///
    /// - `mode`:
    #[inline]
    pub fn try_send_mode(
        &self,
        mode: &Rc<ZwlrOutputModeV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            mode,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateClientId("mode", e)))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_head_v1#{}.mode(mode: zwlr_output_mode_v1#{})\n", client_id, id, arg0);
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
            3,
            arg0_id,
        ]);
        Ok(())
    }

    /// introduce a mode
    ///
    /// This event introduces a mode for this head. It is sent once per
    /// supported mode.
    ///
    /// # Arguments
    ///
    /// - `mode`:
    #[inline]
    pub fn send_mode(
        &self,
        mode: &Rc<ZwlrOutputModeV1>,
    ) {
        let res = self.try_send_mode(
            mode,
        );
        if let Err(e) = res {
            log_send("zwlr_output_head_v1.mode", &e);
        }
    }

    /// introduce a mode
    ///
    /// This event introduces a mode for this head. It is sent once per
    /// supported mode.
    #[inline]
    pub fn new_try_send_mode(
        &self,
    ) -> Result<Rc<ZwlrOutputModeV1>, ObjectError> {
        let mode = self.core.create_child();
        self.try_send_mode(
            &mode,
        )?;
        Ok(mode)
    }

    /// introduce a mode
    ///
    /// This event introduces a mode for this head. It is sent once per
    /// supported mode.
    #[inline]
    pub fn new_send_mode(
        &self,
    ) -> Rc<ZwlrOutputModeV1> {
        let mode = self.core.create_child();
        self.send_mode(
            &mode,
        );
        mode
    }

    /// Since when the enabled message is available.
    pub const MSG__ENABLED__SINCE: u32 = 1;

    /// head is enabled or disabled
    ///
    /// This event describes whether the head is enabled. A disabled head is not
    /// mapped to a region of the global compositor space.
    ///
    /// When a head is disabled, some properties (current_mode, position,
    /// transform and scale) are irrelevant.
    ///
    /// # Arguments
    ///
    /// - `enabled`: zero if disabled, non-zero if enabled
    #[inline]
    pub fn try_send_enabled(
        &self,
        enabled: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            enabled,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_head_v1#{}.enabled(enabled: {})\n", client_id, id, arg0);
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
            arg0 as u32,
        ]);
        Ok(())
    }

    /// head is enabled or disabled
    ///
    /// This event describes whether the head is enabled. A disabled head is not
    /// mapped to a region of the global compositor space.
    ///
    /// When a head is disabled, some properties (current_mode, position,
    /// transform and scale) are irrelevant.
    ///
    /// # Arguments
    ///
    /// - `enabled`: zero if disabled, non-zero if enabled
    #[inline]
    pub fn send_enabled(
        &self,
        enabled: i32,
    ) {
        let res = self.try_send_enabled(
            enabled,
        );
        if let Err(e) = res {
            log_send("zwlr_output_head_v1.enabled", &e);
        }
    }

    /// Since when the current_mode message is available.
    pub const MSG__CURRENT_MODE__SINCE: u32 = 1;

    /// current mode
    ///
    /// This event describes the mode currently in use for this head. It is only
    /// sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `mode`:
    #[inline]
    pub fn try_send_current_mode(
        &self,
        mode: &Rc<ZwlrOutputModeV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            mode,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("mode", client.endpoint.id)));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_head_v1#{}.current_mode(mode: zwlr_output_mode_v1#{})\n", client_id, id, arg0);
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
            5,
            arg0_id,
        ]);
        Ok(())
    }

    /// current mode
    ///
    /// This event describes the mode currently in use for this head. It is only
    /// sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `mode`:
    #[inline]
    pub fn send_current_mode(
        &self,
        mode: &Rc<ZwlrOutputModeV1>,
    ) {
        let res = self.try_send_current_mode(
            mode,
        );
        if let Err(e) = res {
            log_send("zwlr_output_head_v1.current_mode", &e);
        }
    }

    /// Since when the position message is available.
    pub const MSG__POSITION__SINCE: u32 = 1;

    /// current position
    ///
    /// This events describes the position of the head in the global compositor
    /// space. It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `x`: x position within the global compositor space
    /// - `y`: y position within the global compositor space
    #[inline]
    pub fn try_send_position(
        &self,
        x: i32,
        y: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            x,
            y,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_head_v1#{}.position(x: {}, y: {})\n", client_id, id, arg0, arg1);
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
            6,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// current position
    ///
    /// This events describes the position of the head in the global compositor
    /// space. It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `x`: x position within the global compositor space
    /// - `y`: y position within the global compositor space
    #[inline]
    pub fn send_position(
        &self,
        x: i32,
        y: i32,
    ) {
        let res = self.try_send_position(
            x,
            y,
        );
        if let Err(e) = res {
            log_send("zwlr_output_head_v1.position", &e);
        }
    }

    /// Since when the transform message is available.
    pub const MSG__TRANSFORM__SINCE: u32 = 1;

    /// current transformation
    ///
    /// This event describes the transformation currently applied to the head.
    /// It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `transform`:
    #[inline]
    pub fn try_send_transform(
        &self,
        transform: WlOutputTransform,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            transform,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: WlOutputTransform) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_head_v1#{}.transform(transform: {:?})\n", client_id, id, arg0);
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

    /// current transformation
    ///
    /// This event describes the transformation currently applied to the head.
    /// It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `transform`:
    #[inline]
    pub fn send_transform(
        &self,
        transform: WlOutputTransform,
    ) {
        let res = self.try_send_transform(
            transform,
        );
        if let Err(e) = res {
            log_send("zwlr_output_head_v1.transform", &e);
        }
    }

    /// Since when the scale message is available.
    pub const MSG__SCALE__SINCE: u32 = 1;

    /// current scale
    ///
    /// This events describes the scale of the head in the global compositor
    /// space. It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `scale`:
    #[inline]
    pub fn try_send_scale(
        &self,
        scale: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            scale,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: Fixed) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_head_v1#{}.scale(scale: {})\n", client_id, id, arg0);
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
            arg0.to_wire() as u32,
        ]);
        Ok(())
    }

    /// current scale
    ///
    /// This events describes the scale of the head in the global compositor
    /// space. It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `scale`:
    #[inline]
    pub fn send_scale(
        &self,
        scale: Fixed,
    ) {
        let res = self.try_send_scale(
            scale,
        );
        if let Err(e) = res {
            log_send("zwlr_output_head_v1.scale", &e);
        }
    }

    /// Since when the finished message is available.
    pub const MSG__FINISHED__SINCE: u32 = 1;

    /// the head has disappeared
    ///
    /// This event indicates that the head is no longer available. The head
    /// object becomes inert. Clients should send a destroy request and release
    /// any resources associated with it.
    #[inline]
    pub fn try_send_finished(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_head_v1#{}.finished()\n", client_id, id);
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
            9,
        ]);
        Ok(())
    }

    /// the head has disappeared
    ///
    /// This event indicates that the head is no longer available. The head
    /// object becomes inert. Clients should send a destroy request and release
    /// any resources associated with it.
    #[inline]
    pub fn send_finished(
        &self,
    ) {
        let res = self.try_send_finished(
        );
        if let Err(e) = res {
            log_send("zwlr_output_head_v1.finished", &e);
        }
    }

    /// Since when the make message is available.
    pub const MSG__MAKE__SINCE: u32 = 2;

    /// head manufacturer
    ///
    /// This event describes the manufacturer of the head.
    ///
    /// Together with the model and serial_number events the purpose is to
    /// allow clients to recognize heads from previous sessions and for example
    /// load head-specific configurations back.
    ///
    /// It is not guaranteed this event will be ever sent. A reason for that
    /// can be that the compositor does not have information about the make of
    /// the head or the definition of a make is not sensible in the current
    /// setup, for example in a virtual session. Clients can still try to
    /// identify the head by available information from other events but should
    /// be aware that there is an increased risk of false positives.
    ///
    /// If sent, the make event is sent after a wlr_output_head object is
    /// created and only sent once per object. The make does not change over
    /// the lifetime of the wlr_output_head object.
    ///
    /// It is not recommended to display the make string in UI to users. For
    /// that the string provided by the description event should be preferred.
    ///
    /// # Arguments
    ///
    /// - `make`:
    #[inline]
    pub fn try_send_make(
        &self,
        make: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            make,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_head_v1#{}.make(make: {:?})\n", client_id, id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// head manufacturer
    ///
    /// This event describes the manufacturer of the head.
    ///
    /// Together with the model and serial_number events the purpose is to
    /// allow clients to recognize heads from previous sessions and for example
    /// load head-specific configurations back.
    ///
    /// It is not guaranteed this event will be ever sent. A reason for that
    /// can be that the compositor does not have information about the make of
    /// the head or the definition of a make is not sensible in the current
    /// setup, for example in a virtual session. Clients can still try to
    /// identify the head by available information from other events but should
    /// be aware that there is an increased risk of false positives.
    ///
    /// If sent, the make event is sent after a wlr_output_head object is
    /// created and only sent once per object. The make does not change over
    /// the lifetime of the wlr_output_head object.
    ///
    /// It is not recommended to display the make string in UI to users. For
    /// that the string provided by the description event should be preferred.
    ///
    /// # Arguments
    ///
    /// - `make`:
    #[inline]
    pub fn send_make(
        &self,
        make: &str,
    ) {
        let res = self.try_send_make(
            make,
        );
        if let Err(e) = res {
            log_send("zwlr_output_head_v1.make", &e);
        }
    }

    /// Since when the model message is available.
    pub const MSG__MODEL__SINCE: u32 = 2;

    /// head model
    ///
    /// This event describes the model of the head.
    ///
    /// Together with the make and serial_number events the purpose is to
    /// allow clients to recognize heads from previous sessions and for example
    /// load head-specific configurations back.
    ///
    /// It is not guaranteed this event will be ever sent. A reason for that
    /// can be that the compositor does not have information about the model of
    /// the head or the definition of a model is not sensible in the current
    /// setup, for example in a virtual session. Clients can still try to
    /// identify the head by available information from other events but should
    /// be aware that there is an increased risk of false positives.
    ///
    /// If sent, the model event is sent after a wlr_output_head object is
    /// created and only sent once per object. The model does not change over
    /// the lifetime of the wlr_output_head object.
    ///
    /// It is not recommended to display the model string in UI to users. For
    /// that the string provided by the description event should be preferred.
    ///
    /// # Arguments
    ///
    /// - `model`:
    #[inline]
    pub fn try_send_model(
        &self,
        model: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            model,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_head_v1#{}.model(model: {:?})\n", client_id, id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// head model
    ///
    /// This event describes the model of the head.
    ///
    /// Together with the make and serial_number events the purpose is to
    /// allow clients to recognize heads from previous sessions and for example
    /// load head-specific configurations back.
    ///
    /// It is not guaranteed this event will be ever sent. A reason for that
    /// can be that the compositor does not have information about the model of
    /// the head or the definition of a model is not sensible in the current
    /// setup, for example in a virtual session. Clients can still try to
    /// identify the head by available information from other events but should
    /// be aware that there is an increased risk of false positives.
    ///
    /// If sent, the model event is sent after a wlr_output_head object is
    /// created and only sent once per object. The model does not change over
    /// the lifetime of the wlr_output_head object.
    ///
    /// It is not recommended to display the model string in UI to users. For
    /// that the string provided by the description event should be preferred.
    ///
    /// # Arguments
    ///
    /// - `model`:
    #[inline]
    pub fn send_model(
        &self,
        model: &str,
    ) {
        let res = self.try_send_model(
            model,
        );
        if let Err(e) = res {
            log_send("zwlr_output_head_v1.model", &e);
        }
    }

    /// Since when the serial_number message is available.
    pub const MSG__SERIAL_NUMBER__SINCE: u32 = 2;

    /// head serial number
    ///
    /// This event describes the serial number of the head.
    ///
    /// Together with the make and model events the purpose is to allow clients
    /// to recognize heads from previous sessions and for example load head-
    /// specific configurations back.
    ///
    /// It is not guaranteed this event will be ever sent. A reason for that
    /// can be that the compositor does not have information about the serial
    /// number of the head or the definition of a serial number is not sensible
    /// in the current setup. Clients can still try to identify the head by
    /// available information from other events but should be aware that there
    /// is an increased risk of false positives.
    ///
    /// If sent, the serial number event is sent after a wlr_output_head object
    /// is created and only sent once per object. The serial number does not
    /// change over the lifetime of the wlr_output_head object.
    ///
    /// It is not recommended to display the serial_number string in UI to
    /// users. For that the string provided by the description event should be
    /// preferred.
    ///
    /// # Arguments
    ///
    /// - `serial_number`:
    #[inline]
    pub fn try_send_serial_number(
        &self,
        serial_number: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            serial_number,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_head_v1#{}.serial_number(serial_number: {:?})\n", client_id, id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// head serial number
    ///
    /// This event describes the serial number of the head.
    ///
    /// Together with the make and model events the purpose is to allow clients
    /// to recognize heads from previous sessions and for example load head-
    /// specific configurations back.
    ///
    /// It is not guaranteed this event will be ever sent. A reason for that
    /// can be that the compositor does not have information about the serial
    /// number of the head or the definition of a serial number is not sensible
    /// in the current setup. Clients can still try to identify the head by
    /// available information from other events but should be aware that there
    /// is an increased risk of false positives.
    ///
    /// If sent, the serial number event is sent after a wlr_output_head object
    /// is created and only sent once per object. The serial number does not
    /// change over the lifetime of the wlr_output_head object.
    ///
    /// It is not recommended to display the serial_number string in UI to
    /// users. For that the string provided by the description event should be
    /// preferred.
    ///
    /// # Arguments
    ///
    /// - `serial_number`:
    #[inline]
    pub fn send_serial_number(
        &self,
        serial_number: &str,
    ) {
        let res = self.try_send_serial_number(
            serial_number,
        );
        if let Err(e) = res {
            log_send("zwlr_output_head_v1.serial_number", &e);
        }
    }

    /// Since when the release message is available.
    pub const MSG__RELEASE__SINCE: u32 = 3;

    /// destroy the head object
    ///
    /// This request indicates that the client will no longer use this head
    /// object.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_head_v1#{}.release()\n", id);
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

    /// destroy the head object
    ///
    /// This request indicates that the client will no longer use this head
    /// object.
    #[inline]
    pub fn send_release(
        &self,
    ) {
        let res = self.try_send_release(
        );
        if let Err(e) = res {
            log_send("zwlr_output_head_v1.release", &e);
        }
    }

    /// Since when the adaptive_sync message is available.
    pub const MSG__ADAPTIVE_SYNC__SINCE: u32 = 4;

    /// current adaptive sync state
    ///
    /// This event describes whether adaptive sync is currently enabled for
    /// the head or not. Adaptive sync is also known as Variable Refresh
    /// Rate or VRR.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_adaptive_sync(
        &self,
        state: ZwlrOutputHeadV1AdaptiveSyncState,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: ZwlrOutputHeadV1AdaptiveSyncState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_head_v1#{}.adaptive_sync(state: {:?})\n", client_id, id, arg0);
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

    /// current adaptive sync state
    ///
    /// This event describes whether adaptive sync is currently enabled for
    /// the head or not. Adaptive sync is also known as Variable Refresh
    /// Rate or VRR.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_adaptive_sync(
        &self,
        state: ZwlrOutputHeadV1AdaptiveSyncState,
    ) {
        let res = self.try_send_adaptive_sync(
            state,
        );
        if let Err(e) = res {
            log_send("zwlr_output_head_v1.adaptive_sync", &e);
        }
    }
}

/// A message handler for [`ZwlrOutputHeadV1`] proxies.
pub trait ZwlrOutputHeadV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwlrOutputHeadV1>) {
        slf.core.delete_id();
    }

    /// head name
    ///
    /// This event describes the head name.
    ///
    /// The naming convention is compositor defined, but limited to alphanumeric
    /// characters and dashes (-). Each name is unique among all wlr_output_head
    /// objects, but if a wlr_output_head object is destroyed the same name may
    /// be reused later. The names will also remain consistent across sessions
    /// with the same hardware and software configuration.
    ///
    /// Examples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc. However, do
    /// not assume that the name is a reflection of an underlying DRM
    /// connector, X11 connection, etc.
    ///
    /// If this head matches a wl_output, the wl_output.name event must report
    /// the same name.
    ///
    /// The name event is sent after a wlr_output_head object is created. This
    /// event is only sent once per object, and the name does not change over
    /// the lifetime of the wlr_output_head object.
    ///
    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    fn handle_name(
        &mut self,
        slf: &Rc<ZwlrOutputHeadV1>,
        name: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_name(
            name,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_head_v1.name", &e);
        }
    }

    /// head description
    ///
    /// This event describes a human-readable description of the head.
    ///
    /// The description is a UTF-8 string with no convention defined for its
    /// contents. Examples might include 'Foocorp 11" Display' or 'Virtual X11
    /// output via :1'. However, do not assume that the name is a reflection of
    /// the make, model, serial of the underlying DRM connector or the display
    /// name of the underlying X11 connection, etc.
    ///
    /// If this head matches a wl_output, the wl_output.description event must
    /// report the same name.
    ///
    /// The description event is sent after a wlr_output_head object is created.
    /// This event is only sent once per object, and the description does not
    /// change over the lifetime of the wlr_output_head object.
    ///
    /// # Arguments
    ///
    /// - `description`:
    #[inline]
    fn handle_description(
        &mut self,
        slf: &Rc<ZwlrOutputHeadV1>,
        description: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_description(
            description,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_head_v1.description", &e);
        }
    }

    /// head physical size
    ///
    /// This event describes the physical size of the head. This event is only
    /// sent if the head has a physical size (e.g. is not a projector or a
    /// virtual device).
    ///
    /// The physical size event is sent after a wlr_output_head object is created. This
    /// event is only sent once per object, and the physical size does not change over
    /// the lifetime of the wlr_output_head object.
    ///
    /// # Arguments
    ///
    /// - `width`: width in millimeters of the output
    /// - `height`: height in millimeters of the output
    #[inline]
    fn handle_physical_size(
        &mut self,
        slf: &Rc<ZwlrOutputHeadV1>,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_physical_size(
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_head_v1.physical_size", &e);
        }
    }

    /// introduce a mode
    ///
    /// This event introduces a mode for this head. It is sent once per
    /// supported mode.
    ///
    /// # Arguments
    ///
    /// - `mode`:
    #[inline]
    fn handle_mode(
        &mut self,
        slf: &Rc<ZwlrOutputHeadV1>,
        mode: &Rc<ZwlrOutputModeV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_mode(
            mode,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_head_v1.mode", &e);
        }
    }

    /// head is enabled or disabled
    ///
    /// This event describes whether the head is enabled. A disabled head is not
    /// mapped to a region of the global compositor space.
    ///
    /// When a head is disabled, some properties (current_mode, position,
    /// transform and scale) are irrelevant.
    ///
    /// # Arguments
    ///
    /// - `enabled`: zero if disabled, non-zero if enabled
    #[inline]
    fn handle_enabled(
        &mut self,
        slf: &Rc<ZwlrOutputHeadV1>,
        enabled: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_enabled(
            enabled,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_head_v1.enabled", &e);
        }
    }

    /// current mode
    ///
    /// This event describes the mode currently in use for this head. It is only
    /// sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `mode`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_current_mode(
        &mut self,
        slf: &Rc<ZwlrOutputHeadV1>,
        mode: &Rc<ZwlrOutputModeV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = mode.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = slf.try_send_current_mode(
            mode,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_head_v1.current_mode", &e);
        }
    }

    /// current position
    ///
    /// This events describes the position of the head in the global compositor
    /// space. It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `x`: x position within the global compositor space
    /// - `y`: y position within the global compositor space
    #[inline]
    fn handle_position(
        &mut self,
        slf: &Rc<ZwlrOutputHeadV1>,
        x: i32,
        y: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_position(
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_head_v1.position", &e);
        }
    }

    /// current transformation
    ///
    /// This event describes the transformation currently applied to the head.
    /// It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `transform`:
    #[inline]
    fn handle_transform(
        &mut self,
        slf: &Rc<ZwlrOutputHeadV1>,
        transform: WlOutputTransform,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_transform(
            transform,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_head_v1.transform", &e);
        }
    }

    /// current scale
    ///
    /// This events describes the scale of the head in the global compositor
    /// space. It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `scale`:
    #[inline]
    fn handle_scale(
        &mut self,
        slf: &Rc<ZwlrOutputHeadV1>,
        scale: Fixed,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_scale(
            scale,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_head_v1.scale", &e);
        }
    }

    /// the head has disappeared
    ///
    /// This event indicates that the head is no longer available. The head
    /// object becomes inert. Clients should send a destroy request and release
    /// any resources associated with it.
    #[inline]
    fn handle_finished(
        &mut self,
        slf: &Rc<ZwlrOutputHeadV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_finished(
        );
        if let Err(e) = res {
            log_forward("zwlr_output_head_v1.finished", &e);
        }
    }

    /// head manufacturer
    ///
    /// This event describes the manufacturer of the head.
    ///
    /// Together with the model and serial_number events the purpose is to
    /// allow clients to recognize heads from previous sessions and for example
    /// load head-specific configurations back.
    ///
    /// It is not guaranteed this event will be ever sent. A reason for that
    /// can be that the compositor does not have information about the make of
    /// the head or the definition of a make is not sensible in the current
    /// setup, for example in a virtual session. Clients can still try to
    /// identify the head by available information from other events but should
    /// be aware that there is an increased risk of false positives.
    ///
    /// If sent, the make event is sent after a wlr_output_head object is
    /// created and only sent once per object. The make does not change over
    /// the lifetime of the wlr_output_head object.
    ///
    /// It is not recommended to display the make string in UI to users. For
    /// that the string provided by the description event should be preferred.
    ///
    /// # Arguments
    ///
    /// - `make`:
    #[inline]
    fn handle_make(
        &mut self,
        slf: &Rc<ZwlrOutputHeadV1>,
        make: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_make(
            make,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_head_v1.make", &e);
        }
    }

    /// head model
    ///
    /// This event describes the model of the head.
    ///
    /// Together with the make and serial_number events the purpose is to
    /// allow clients to recognize heads from previous sessions and for example
    /// load head-specific configurations back.
    ///
    /// It is not guaranteed this event will be ever sent. A reason for that
    /// can be that the compositor does not have information about the model of
    /// the head or the definition of a model is not sensible in the current
    /// setup, for example in a virtual session. Clients can still try to
    /// identify the head by available information from other events but should
    /// be aware that there is an increased risk of false positives.
    ///
    /// If sent, the model event is sent after a wlr_output_head object is
    /// created and only sent once per object. The model does not change over
    /// the lifetime of the wlr_output_head object.
    ///
    /// It is not recommended to display the model string in UI to users. For
    /// that the string provided by the description event should be preferred.
    ///
    /// # Arguments
    ///
    /// - `model`:
    #[inline]
    fn handle_model(
        &mut self,
        slf: &Rc<ZwlrOutputHeadV1>,
        model: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_model(
            model,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_head_v1.model", &e);
        }
    }

    /// head serial number
    ///
    /// This event describes the serial number of the head.
    ///
    /// Together with the make and model events the purpose is to allow clients
    /// to recognize heads from previous sessions and for example load head-
    /// specific configurations back.
    ///
    /// It is not guaranteed this event will be ever sent. A reason for that
    /// can be that the compositor does not have information about the serial
    /// number of the head or the definition of a serial number is not sensible
    /// in the current setup. Clients can still try to identify the head by
    /// available information from other events but should be aware that there
    /// is an increased risk of false positives.
    ///
    /// If sent, the serial number event is sent after a wlr_output_head object
    /// is created and only sent once per object. The serial number does not
    /// change over the lifetime of the wlr_output_head object.
    ///
    /// It is not recommended to display the serial_number string in UI to
    /// users. For that the string provided by the description event should be
    /// preferred.
    ///
    /// # Arguments
    ///
    /// - `serial_number`:
    #[inline]
    fn handle_serial_number(
        &mut self,
        slf: &Rc<ZwlrOutputHeadV1>,
        serial_number: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_serial_number(
            serial_number,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_head_v1.serial_number", &e);
        }
    }

    /// destroy the head object
    ///
    /// This request indicates that the client will no longer use this head
    /// object.
    #[inline]
    fn handle_release(
        &mut self,
        slf: &Rc<ZwlrOutputHeadV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_release(
        );
        if let Err(e) = res {
            log_forward("zwlr_output_head_v1.release", &e);
        }
    }

    /// current adaptive sync state
    ///
    /// This event describes whether adaptive sync is currently enabled for
    /// the head or not. Adaptive sync is also known as Variable Refresh
    /// Rate or VRR.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_adaptive_sync(
        &mut self,
        slf: &Rc<ZwlrOutputHeadV1>,
        state: ZwlrOutputHeadV1AdaptiveSyncState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_adaptive_sync(
            state,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_head_v1.adaptive_sync", &e);
        }
    }
}

impl ObjectPrivate for ZwlrOutputHeadV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwlrOutputHeadV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_head_v1#{}.release()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_head_v1#{}.name(name: {:?})\n", id, arg0);
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
            1 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "description")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_head_v1#{}.description(description: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_description(&self, arg0);
                } else {
                    DefaultHandler.handle_description(&self, arg0);
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
                    fn log(state: &State, id: u32, arg0: i32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_head_v1#{}.physical_size(width: {}, height: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_physical_size(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_physical_size(&self, arg0, arg1);
                }
            }
            3 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_head_v1#{}.mode(mode: zwlr_output_mode_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = ZwlrOutputModeV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetServerId(arg0_id, "mode", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_mode(&self, arg0);
                } else {
                    DefaultHandler.handle_mode(&self, arg0);
                }
            }
            4 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_head_v1#{}.enabled(enabled: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_enabled(&self, arg0);
                } else {
                    DefaultHandler.handle_enabled(&self, arg0);
                }
            }
            5 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_head_v1#{}.current_mode(mode: zwlr_output_mode_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = server.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ZwlrOutputModeV1>() else {
                    let o = server.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("mode", o.core().interface, ObjectInterface::ZwlrOutputModeV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_current_mode(&self, arg0);
                } else {
                    DefaultHandler.handle_current_mode(&self, arg0);
                }
            }
            6 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_head_v1#{}.position(x: {}, y: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_position(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_position(&self, arg0, arg1);
                }
            }
            7 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = WlOutputTransform(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: WlOutputTransform) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_head_v1#{}.transform(transform: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_transform(&self, arg0);
                } else {
                    DefaultHandler.handle_transform(&self, arg0);
                }
            }
            8 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = Fixed::from_wire(arg0 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: Fixed) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_head_v1#{}.scale(scale: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_scale(&self, arg0);
                } else {
                    DefaultHandler.handle_scale(&self, arg0);
                }
            }
            9 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_head_v1#{}.finished()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_finished(&self);
                } else {
                    DefaultHandler.handle_finished(&self);
                }
            }
            10 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "make")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_head_v1#{}.make(make: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_make(&self, arg0);
                } else {
                    DefaultHandler.handle_make(&self, arg0);
                }
            }
            11 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "model")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_head_v1#{}.model(model: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_model(&self, arg0);
                } else {
                    DefaultHandler.handle_model(&self, arg0);
                }
            }
            12 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "serial_number")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_head_v1#{}.serial_number(serial_number: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_serial_number(&self, arg0);
                } else {
                    DefaultHandler.handle_serial_number(&self, arg0);
                }
            }
            13 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = ZwlrOutputHeadV1AdaptiveSyncState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: ZwlrOutputHeadV1AdaptiveSyncState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_head_v1#{}.adaptive_sync(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_adaptive_sync(&self, arg0);
                } else {
                    DefaultHandler.handle_adaptive_sync(&self, arg0);
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
            0 => "name",
            1 => "description",
            2 => "physical_size",
            3 => "mode",
            4 => "enabled",
            5 => "current_mode",
            6 => "position",
            7 => "transform",
            8 => "scale",
            9 => "finished",
            10 => "make",
            11 => "model",
            12 => "serial_number",
            13 => "adaptive_sync",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwlrOutputHeadV1 {
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

impl ZwlrOutputHeadV1 {
    /// Since when the adaptive_sync_state.disabled enum variant is available.
    pub const ENM__ADAPTIVE_SYNC_STATE_DISABLED__SINCE: u32 = 1;
    /// Since when the adaptive_sync_state.enabled enum variant is available.
    pub const ENM__ADAPTIVE_SYNC_STATE_ENABLED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwlrOutputHeadV1AdaptiveSyncState(pub u32);

impl ZwlrOutputHeadV1AdaptiveSyncState {
    /// adaptive sync is disabled
    pub const DISABLED: Self = Self(0);

    /// adaptive sync is enabled
    pub const ENABLED: Self = Self(1);
}

impl Debug for ZwlrOutputHeadV1AdaptiveSyncState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DISABLED => "DISABLED",
            Self::ENABLED => "ENABLED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
