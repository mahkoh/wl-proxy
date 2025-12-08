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

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwlr_output_head_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwlrOutputHeadV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwlrOutputHeadV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwlrOutputHeadV1MessageHandler for DefaultMessageHandler { }

impl MetaZwlrOutputHeadV1 {
    pub const XML_VERSION: u32 = 4;
}

impl MetaZwlrOutputHeadV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwlrOutputHeadV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwlrOutputHeadV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwlrOutputHeadV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwlrOutputHeadV1 {
    /// Since when the name message is available.
    #[allow(dead_code)]
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
    /// If the compositor implements the xdg-output protocol and this head is
    /// enabled, the xdg_output.name event must report the same name.
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
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            name,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= zwlr_output_head_v1#{}.name(name: {:?})", client.endpoint.id, id, arg0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the description message is available.
    #[allow(dead_code)]
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
    /// If the compositor implements xdg-output and this head is enabled,
    /// the xdg_output.description must report the same description.
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
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            description,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= zwlr_output_head_v1#{}.description(description: {:?})", client.endpoint.id, id, arg0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the physical_size message is available.
    #[allow(dead_code)]
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
    pub fn send_physical_size(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= zwlr_output_head_v1#{}.physical_size(width: {}, height: {})", client.endpoint.id, id, arg0, arg1);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the mode message is available.
    #[allow(dead_code)]
    pub const MSG__MODE__SINCE: u32 = 1;

    /// introduce a mode
    ///
    /// This event introduces a mode for this head. It is sent once per
    /// supported mode.
    #[inline]
    pub fn send_mode(
        &self,
        mode: &Rc<MetaZwlrOutputModeV1>,
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateClientId("mode", e))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= zwlr_output_head_v1#{}.mode(mode: zwlr_output_mode_v1#{})", client.endpoint.id, id, arg0_id);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the enabled message is available.
    #[allow(dead_code)]
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
    pub fn send_enabled(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= zwlr_output_head_v1#{}.enabled(enabled: {})", client.endpoint.id, id, arg0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the current_mode message is available.
    #[allow(dead_code)]
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
    pub fn send_current_mode(
        &self,
        mode: &Rc<MetaZwlrOutputModeV1>,
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError::ArgNoClientId("mode", client.endpoint.id));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= zwlr_output_head_v1#{}.current_mode(mode: zwlr_output_mode_v1#{})", client.endpoint.id, id, arg0_id);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the position message is available.
    #[allow(dead_code)]
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
    pub fn send_position(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= zwlr_output_head_v1#{}.position(x: {}, y: {})", client.endpoint.id, id, arg0, arg1);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the transform message is available.
    #[allow(dead_code)]
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
    pub fn send_transform(
        &self,
        transform: MetaWlOutputTransform,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            transform,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= zwlr_output_head_v1#{}.transform(transform: {:?})", client.endpoint.id, id, arg0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            7,
            arg0.0 as u32,
        ]);
        Ok(())
    }

    /// Since when the scale message is available.
    #[allow(dead_code)]
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
    pub fn send_scale(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= zwlr_output_head_v1#{}.scale(scale: {})", client.endpoint.id, id, arg0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the finished message is available.
    #[allow(dead_code)]
    pub const MSG__FINISHED__SINCE: u32 = 1;

    /// the head has disappeared
    ///
    /// This event indicates that the head is no longer available. The head
    /// object becomes inert. Clients should send a destroy request and release
    /// any resources associated with it.
    #[inline]
    pub fn send_finished(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= zwlr_output_head_v1#{}.finished()", client.endpoint.id, id);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the make message is available.
    #[allow(dead_code)]
    pub const MSG__MAKE__SINCE: u32 = 2;

    /// head manufacturer
    ///
    /// This event describes the manufacturer of the head.
    ///
    /// This must report the same make as the wl_output interface does in its
    /// geometry event.
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
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            make,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= zwlr_output_head_v1#{}.make(make: {:?})", client.endpoint.id, id, arg0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the model message is available.
    #[allow(dead_code)]
    pub const MSG__MODEL__SINCE: u32 = 2;

    /// head model
    ///
    /// This event describes the model of the head.
    ///
    /// This must report the same model as the wl_output interface does in its
    /// geometry event.
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
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            model,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= zwlr_output_head_v1#{}.model(model: {:?})", client.endpoint.id, id, arg0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the serial_number message is available.
    #[allow(dead_code)]
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
    pub fn send_serial_number(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= zwlr_output_head_v1#{}.serial_number(serial_number: {:?})", client.endpoint.id, id, arg0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the release message is available.
    #[allow(dead_code)]
    pub const MSG__RELEASE__SINCE: u32 = 3;

    /// destroy the head object
    ///
    /// This request indicates that the client will no longer use this head
    /// object.
    #[inline]
    pub fn send_release(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= zwlr_output_head_v1#{}.release()", id);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the adaptive_sync message is available.
    #[allow(dead_code)]
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
    pub fn send_adaptive_sync(
        &self,
        state: MetaZwlrOutputHeadV1AdaptiveSyncState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            state,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= zwlr_output_head_v1#{}.adaptive_sync(state: {:?})", client.endpoint.id, id, arg0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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
}

/// A message handler for [ZwlrOutputHeadV1] proxies.
#[allow(dead_code)]
pub trait MetaZwlrOutputHeadV1MessageHandler {
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
    /// If the compositor implements the xdg-output protocol and this head is
    /// enabled, the xdg_output.name event must report the same name.
    ///
    /// The name event is sent after a wlr_output_head object is created. This
    /// event is only sent once per object, and the name does not change over
    /// the lifetime of the wlr_output_head object.
    ///
    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    fn name(
        &mut self,
        _slf: &Rc<MetaZwlrOutputHeadV1>,
        name: &str,
    ) {
        let res = _slf.send_name(
            name,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_head_v1.name message: {}", Report::new(e));
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
    /// If the compositor implements xdg-output and this head is enabled,
    /// the xdg_output.description must report the same description.
    ///
    /// The description event is sent after a wlr_output_head object is created.
    /// This event is only sent once per object, and the description does not
    /// change over the lifetime of the wlr_output_head object.
    ///
    /// # Arguments
    ///
    /// - `description`:
    #[inline]
    fn description(
        &mut self,
        _slf: &Rc<MetaZwlrOutputHeadV1>,
        description: &str,
    ) {
        let res = _slf.send_description(
            description,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_head_v1.description message: {}", Report::new(e));
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
    fn physical_size(
        &mut self,
        _slf: &Rc<MetaZwlrOutputHeadV1>,
        width: i32,
        height: i32,
    ) {
        let res = _slf.send_physical_size(
            width,
            height,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_head_v1.physical_size message: {}", Report::new(e));
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
    fn mode(
        &mut self,
        _slf: &Rc<MetaZwlrOutputHeadV1>,
        mode: &Rc<MetaZwlrOutputModeV1>,
    ) {
        let res = _slf.send_mode(
            mode,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_head_v1.mode message: {}", Report::new(e));
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
    fn enabled(
        &mut self,
        _slf: &Rc<MetaZwlrOutputHeadV1>,
        enabled: i32,
    ) {
        let res = _slf.send_enabled(
            enabled,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_head_v1.enabled message: {}", Report::new(e));
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
    fn current_mode(
        &mut self,
        _slf: &Rc<MetaZwlrOutputHeadV1>,
        mode: &Rc<MetaZwlrOutputModeV1>,
    ) {
        if let Some(client_id) = _slf.core.client_id.get() {
            if let Some(client_id_2) = mode.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = _slf.send_current_mode(
            mode,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_head_v1.current_mode message: {}", Report::new(e));
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
    fn position(
        &mut self,
        _slf: &Rc<MetaZwlrOutputHeadV1>,
        x: i32,
        y: i32,
    ) {
        let res = _slf.send_position(
            x,
            y,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_head_v1.position message: {}", Report::new(e));
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
    fn transform(
        &mut self,
        _slf: &Rc<MetaZwlrOutputHeadV1>,
        transform: MetaWlOutputTransform,
    ) {
        let res = _slf.send_transform(
            transform,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_head_v1.transform message: {}", Report::new(e));
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
    fn scale(
        &mut self,
        _slf: &Rc<MetaZwlrOutputHeadV1>,
        scale: Fixed,
    ) {
        let res = _slf.send_scale(
            scale,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_head_v1.scale message: {}", Report::new(e));
        }
    }

    /// the head has disappeared
    ///
    /// This event indicates that the head is no longer available. The head
    /// object becomes inert. Clients should send a destroy request and release
    /// any resources associated with it.
    #[inline]
    fn finished(
        &mut self,
        _slf: &Rc<MetaZwlrOutputHeadV1>,
    ) {
        let res = _slf.send_finished(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_head_v1.finished message: {}", Report::new(e));
        }
    }

    /// head manufacturer
    ///
    /// This event describes the manufacturer of the head.
    ///
    /// This must report the same make as the wl_output interface does in its
    /// geometry event.
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
    fn make(
        &mut self,
        _slf: &Rc<MetaZwlrOutputHeadV1>,
        make: &str,
    ) {
        let res = _slf.send_make(
            make,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_head_v1.make message: {}", Report::new(e));
        }
    }

    /// head model
    ///
    /// This event describes the model of the head.
    ///
    /// This must report the same model as the wl_output interface does in its
    /// geometry event.
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
    fn model(
        &mut self,
        _slf: &Rc<MetaZwlrOutputHeadV1>,
        model: &str,
    ) {
        let res = _slf.send_model(
            model,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_head_v1.model message: {}", Report::new(e));
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
    fn serial_number(
        &mut self,
        _slf: &Rc<MetaZwlrOutputHeadV1>,
        serial_number: &str,
    ) {
        let res = _slf.send_serial_number(
            serial_number,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_head_v1.serial_number message: {}", Report::new(e));
        }
    }

    /// destroy the head object
    ///
    /// This request indicates that the client will no longer use this head
    /// object.
    #[inline]
    fn release(
        &mut self,
        _slf: &Rc<MetaZwlrOutputHeadV1>,
    ) {
        let res = _slf.send_release(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_head_v1.release message: {}", Report::new(e));
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
    fn adaptive_sync(
        &mut self,
        _slf: &Rc<MetaZwlrOutputHeadV1>,
        state: MetaZwlrOutputHeadV1AdaptiveSyncState,
    ) {
        let res = _slf.send_adaptive_sync(
            state,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_head_v1.adaptive_sync message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwlrOutputHeadV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("client#{:04} -> zwlr_output_head_v1#{}.release()", client.endpoint.id, msg[0]);
                if let Some(handler) = handler {
                    (**handler).release(&self);
                } else {
                    DefaultMessageHandler.release(&self);
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
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("name"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("name"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("name"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("name"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                eprintln!("server      -> zwlr_output_head_v1#{}.name(name: {:?})", msg[0], arg0);
                if let Some(handler) = handler {
                    (**handler).name(&self, arg0);
                } else {
                    DefaultMessageHandler.name(&self, arg0);
                }
            }
            1 => {
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
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                eprintln!("server      -> zwlr_output_head_v1#{}.description(description: {:?})", msg[0], arg0);
                if let Some(handler) = handler {
                    (**handler).description(&self, arg0);
                } else {
                    DefaultMessageHandler.description(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                eprintln!("server      -> zwlr_output_head_v1#{}.physical_size(width: {}, height: {})", msg[0], arg0, arg1);
                if let Some(handler) = handler {
                    (**handler).physical_size(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.physical_size(&self, arg0, arg1);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                eprintln!("server      -> zwlr_output_head_v1#{}.mode(mode: zwlr_output_mode_v1#{})", msg[0], arg0);
                let arg0_id = arg0;
                let arg0 = MetaZwlrOutputModeV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetServerId(arg0_id, "mode", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).mode(&self, arg0);
                } else {
                    DefaultMessageHandler.mode(&self, arg0);
                }
            }
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = arg0 as i32;
                eprintln!("server      -> zwlr_output_head_v1#{}.enabled(enabled: {})", msg[0], arg0);
                if let Some(handler) = handler {
                    (**handler).enabled(&self, arg0);
                } else {
                    DefaultMessageHandler.enabled(&self, arg0);
                }
            }
            5 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                eprintln!("server      -> zwlr_output_head_v1#{}.current_mode(mode: zwlr_output_mode_v1#{})", msg[0], arg0);
                let arg0_id = arg0;
                let Some(arg0) = self.core.state.server.lookup(arg0_id) else {
                    return Err(ObjectError::NoServerObject(arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaZwlrOutputModeV1>() else {
                    let o = self.core.state.server.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("mode", o.core().interface, ProxyInterface::ZwlrOutputModeV1));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).current_mode(&self, arg0);
                } else {
                    DefaultMessageHandler.current_mode(&self, arg0);
                }
            }
            6 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                eprintln!("server      -> zwlr_output_head_v1#{}.position(x: {}, y: {})", msg[0], arg0, arg1);
                if let Some(handler) = handler {
                    (**handler).position(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.position(&self, arg0, arg1);
                }
            }
            7 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = MetaWlOutputTransform(arg0);
                eprintln!("server      -> zwlr_output_head_v1#{}.transform(transform: {:?})", msg[0], arg0);
                if let Some(handler) = handler {
                    (**handler).transform(&self, arg0);
                } else {
                    DefaultMessageHandler.transform(&self, arg0);
                }
            }
            8 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = Fixed::from_wire(arg0 as i32);
                eprintln!("server      -> zwlr_output_head_v1#{}.scale(scale: {})", msg[0], arg0);
                if let Some(handler) = handler {
                    (**handler).scale(&self, arg0);
                } else {
                    DefaultMessageHandler.scale(&self, arg0);
                }
            }
            9 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("server      -> zwlr_output_head_v1#{}.finished()", msg[0]);
                if let Some(handler) = handler {
                    (**handler).finished(&self);
                } else {
                    DefaultMessageHandler.finished(&self);
                }
            }
            10 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("make"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("make"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("make"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("make"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                eprintln!("server      -> zwlr_output_head_v1#{}.make(make: {:?})", msg[0], arg0);
                if let Some(handler) = handler {
                    (**handler).make(&self, arg0);
                } else {
                    DefaultMessageHandler.make(&self, arg0);
                }
            }
            11 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("model"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("model"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("model"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("model"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                eprintln!("server      -> zwlr_output_head_v1#{}.model(model: {:?})", msg[0], arg0);
                if let Some(handler) = handler {
                    (**handler).model(&self, arg0);
                } else {
                    DefaultMessageHandler.model(&self, arg0);
                }
            }
            12 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("serial_number"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("serial_number"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("serial_number"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("serial_number"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                eprintln!("server      -> zwlr_output_head_v1#{}.serial_number(serial_number: {:?})", msg[0], arg0);
                if let Some(handler) = handler {
                    (**handler).serial_number(&self, arg0);
                } else {
                    DefaultMessageHandler.serial_number(&self, arg0);
                }
            }
            13 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = MetaZwlrOutputHeadV1AdaptiveSyncState(arg0);
                eprintln!("server      -> zwlr_output_head_v1#{}.adaptive_sync(state: {:?})", msg[0], arg0);
                if let Some(handler) = handler {
                    (**handler).adaptive_sync(&self, arg0);
                } else {
                    DefaultMessageHandler.adaptive_sync(&self, arg0);
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

impl MetaZwlrOutputHeadV1 {
    /// Since when the adaptive_sync_state.disabled enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ADAPTIVE_SYNC_STATE_DISABLED__SINCE: u32 = 1;
    /// Since when the adaptive_sync_state.enabled enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ADAPTIVE_SYNC_STATE_ENABLED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaZwlrOutputHeadV1AdaptiveSyncState(pub u32);

impl MetaZwlrOutputHeadV1AdaptiveSyncState {
    /// adaptive sync is disabled
    #[allow(dead_code)]
    pub const DISABLED: Self = Self(0);

    /// adaptive sync is enabled
    #[allow(dead_code)]
    pub const ENABLED: Self = Self(1);
}

impl Debug for MetaZwlrOutputHeadV1AdaptiveSyncState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DISABLED => "DISABLED",
            Self::ENABLED => "ENABLED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
