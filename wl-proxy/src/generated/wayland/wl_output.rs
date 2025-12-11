//! compositor output region
//!
//! An output describes part of the compositor geometry.  The
//! compositor works in the 'compositor coordinate system' and an
//! output corresponds to a rectangular area in that space that is
//! actually visible.  This typically corresponds to a monitor that
//! displays part of the compositor space.  This object is published
//! as global during start up, or when a monitor is hotplugged.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wl_output proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlOutput {
    core: ProxyCore,
    handler: HandlerHolder<dyn WlOutputHandler>,
}

struct DefaultHandler;

impl WlOutputHandler for DefaultHandler { }

impl WlOutput {
    pub const XML_VERSION: u32 = 4;
    pub const INTERFACE: &str = "wl_output";
}

impl WlOutput {
    pub fn set_handler(&self, handler: impl WlOutputHandler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn WlOutputHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlOutput")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlOutput {
    /// Since when the geometry message is available.
    pub const MSG__GEOMETRY__SINCE: u32 = 1;

    /// properties of the output
    ///
    /// The geometry event describes geometric properties of the output.
    /// The event is sent when binding to the output object and whenever
    /// any of the properties change.
    ///
    /// The physical size can be set to zero if it doesn't make sense for this
    /// output (e.g. for projectors or virtual outputs).
    ///
    /// The geometry event will be followed by a done event (starting from
    /// version 2).
    ///
    /// Clients should use wl_surface.preferred_buffer_transform instead of the
    /// transform advertised by this event to find the preferred buffer
    /// transform to use for a surface.
    ///
    /// Note: wl_output only advertises partial information about the output
    /// position and identification. Some compositors, for instance those not
    /// implementing a desktop-style output layout or those exposing virtual
    /// outputs, might fake this information. Instead of using x and y, clients
    /// should use xdg_output.logical_position. Instead of using make and model,
    /// clients should use name and description.
    ///
    /// # Arguments
    ///
    /// - `x`: x position within the global compositor space
    /// - `y`: y position within the global compositor space
    /// - `physical_width`: width in millimeters of the output
    /// - `physical_height`: height in millimeters of the output
    /// - `subpixel`: subpixel orientation of the output
    /// - `make`: textual description of the manufacturer
    /// - `model`: textual description of the model
    /// - `transform`: additional transformation applied to buffer contents during presentation
    #[inline]
    pub fn send_geometry(
        &self,
        x: i32,
        y: i32,
        physical_width: i32,
        physical_height: i32,
        subpixel: WlOutputSubpixel,
        make: &str,
        model: &str,
        transform: WlOutputTransform,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
            arg6,
            arg7,
        ) = (
            x,
            y,
            physical_width,
            physical_height,
            subpixel,
            make,
            model,
            transform,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_output#{}.geometry(x: {}, y: {}, physical_width: {}, physical_height: {}, subpixel: {:?}, make: {:?}, model: {:?}, transform: {:?})\n", client.endpoint.id, id, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
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
            0,
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
            arg4.0 as u32,
        ]);
        fmt.string(arg5);
        fmt.string(arg6);
        fmt.words([
            arg7.0 as u32,
        ]);
        Ok(())
    }

    /// Since when the mode message is available.
    pub const MSG__MODE__SINCE: u32 = 1;

    /// advertise available modes for the output
    ///
    /// The mode event describes an available mode for the output.
    ///
    /// The event is sent when binding to the output object and there
    /// will always be one mode, the current mode.  The event is sent
    /// again if an output changes mode, for the mode that is now
    /// current.  In other words, the current mode is always the last
    /// mode that was received with the current flag set.
    ///
    /// Non-current modes are deprecated. A compositor can decide to only
    /// advertise the current mode and never send other modes. Clients
    /// should not rely on non-current modes.
    ///
    /// The size of a mode is given in physical hardware units of
    /// the output device. This is not necessarily the same as
    /// the output size in the global compositor space. For instance,
    /// the output may be scaled, as described in wl_output.scale,
    /// or transformed, as described in wl_output.transform. Clients
    /// willing to retrieve the output size in the global compositor
    /// space should use xdg_output.logical_size instead.
    ///
    /// The vertical refresh rate can be set to zero if it doesn't make
    /// sense for this output (e.g. for virtual outputs).
    ///
    /// The mode event will be followed by a done event (starting from
    /// version 2).
    ///
    /// Clients should not use the refresh rate to schedule frames. Instead,
    /// they should use the wl_surface.frame event or the presentation-time
    /// protocol.
    ///
    /// Note: this information is not always meaningful for all outputs. Some
    /// compositors, such as those exposing virtual outputs, might fake the
    /// refresh rate or the size.
    ///
    /// # Arguments
    ///
    /// - `flags`: bitfield of mode flags
    /// - `width`: width of the mode in hardware units
    /// - `height`: height of the mode in hardware units
    /// - `refresh`: vertical refresh rate in mHz
    #[inline]
    pub fn send_mode(
        &self,
        flags: WlOutputMode,
        width: i32,
        height: i32,
        refresh: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            flags,
            width,
            height,
            refresh,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_output#{}.mode(flags: {:?}, width: {}, height: {}, refresh: {})\n", client.endpoint.id, id, arg0, arg1, arg2, arg3);
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
            arg0.0,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// Since when the done message is available.
    pub const MSG__DONE__SINCE: u32 = 2;

    /// sent all information about output
    ///
    /// This event is sent after all other properties have been
    /// sent after binding to the output object and after any
    /// other property changes done after that. This allows
    /// changes to the output properties to be seen as
    /// atomic, even if they happen via multiple events.
    #[inline]
    pub fn send_done(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_output#{}.done()\n", client.endpoint.id, id);
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
        ]);
        Ok(())
    }

    /// Since when the scale message is available.
    pub const MSG__SCALE__SINCE: u32 = 2;

    /// output scaling properties
    ///
    /// This event contains scaling geometry information
    /// that is not in the geometry event. It may be sent after
    /// binding the output object or if the output scale changes
    /// later. The compositor will emit a non-zero, positive
    /// value for scale. If it is not sent, the client should
    /// assume a scale of 1.
    ///
    /// A scale larger than 1 means that the compositor will
    /// automatically scale surface buffers by this amount
    /// when rendering. This is used for very high resolution
    /// displays where applications rendering at the native
    /// resolution would be too small to be legible.
    ///
    /// Clients should use wl_surface.preferred_buffer_scale
    /// instead of this event to find the preferred buffer
    /// scale to use for a surface.
    ///
    /// The scale event will be followed by a done event.
    ///
    /// # Arguments
    ///
    /// - `factor`: scaling factor of output
    #[inline]
    pub fn send_scale(
        &self,
        factor: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            factor,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_output#{}.scale(factor: {})\n", client.endpoint.id, id, arg0);
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
        ]);
        Ok(())
    }

    /// Since when the release message is available.
    pub const MSG__RELEASE__SINCE: u32 = 3;

    /// release the output object
    ///
    /// Using this request a client can tell the server that it is not going to
    /// use the output object anymore.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_output#{}.release()\n", id);
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

    /// Since when the name message is available.
    pub const MSG__NAME__SINCE: u32 = 4;

    /// name of this output
    ///
    /// Many compositors will assign user-friendly names to their outputs, show
    /// them to the user, allow the user to refer to an output, etc. The client
    /// may wish to know this name as well to offer the user similar behaviors.
    ///
    /// The name is a UTF-8 string with no convention defined for its contents.
    /// Each name is unique among all wl_output globals. The name is only
    /// guaranteed to be unique for the compositor instance.
    ///
    /// The same output name is used for all clients for a given wl_output
    /// global. Thus, the name can be shared across processes to refer to a
    /// specific wl_output global.
    ///
    /// The name is not guaranteed to be persistent across sessions, thus cannot
    /// be used to reliably identify an output in e.g. configuration files.
    ///
    /// Examples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc. However, do
    /// not assume that the name is a reflection of an underlying DRM connector,
    /// X11 connection, etc.
    ///
    /// The name event is sent after binding the output object. This event is
    /// only sent once per output object, and the name does not change over the
    /// lifetime of the wl_output global.
    ///
    /// Compositors may re-use the same output name if the wl_output global is
    /// destroyed and re-created later. Compositors should avoid re-using the
    /// same name if possible.
    ///
    /// The name event will be followed by a done event.
    ///
    /// # Arguments
    ///
    /// - `name`: output name
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_output#{}.name(name: {:?})\n", client.endpoint.id, id, arg0);
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
            4,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the description message is available.
    pub const MSG__DESCRIPTION__SINCE: u32 = 4;

    /// human-readable description of this output
    ///
    /// Many compositors can produce human-readable descriptions of their
    /// outputs. The client may wish to know this description as well, e.g. for
    /// output selection purposes.
    ///
    /// The description is a UTF-8 string with no convention defined for its
    /// contents. The description is not guaranteed to be unique among all
    /// wl_output globals. Examples might include 'Foocorp 11" Display' or
    /// 'Virtual X11 output via :1'.
    ///
    /// The description event is sent after binding the output object and
    /// whenever the description changes. The description is optional, and may
    /// not be sent at all.
    ///
    /// The description event will be followed by a done event.
    ///
    /// # Arguments
    ///
    /// - `description`: output description
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_output#{}.description(description: {:?})\n", client.endpoint.id, id, arg0);
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
            5,
        ]);
        fmt.string(arg0);
        Ok(())
    }
}

/// A message handler for [WlOutput] proxies.
pub trait WlOutputHandler: Any {
    /// properties of the output
    ///
    /// The geometry event describes geometric properties of the output.
    /// The event is sent when binding to the output object and whenever
    /// any of the properties change.
    ///
    /// The physical size can be set to zero if it doesn't make sense for this
    /// output (e.g. for projectors or virtual outputs).
    ///
    /// The geometry event will be followed by a done event (starting from
    /// version 2).
    ///
    /// Clients should use wl_surface.preferred_buffer_transform instead of the
    /// transform advertised by this event to find the preferred buffer
    /// transform to use for a surface.
    ///
    /// Note: wl_output only advertises partial information about the output
    /// position and identification. Some compositors, for instance those not
    /// implementing a desktop-style output layout or those exposing virtual
    /// outputs, might fake this information. Instead of using x and y, clients
    /// should use xdg_output.logical_position. Instead of using make and model,
    /// clients should use name and description.
    ///
    /// # Arguments
    ///
    /// - `x`: x position within the global compositor space
    /// - `y`: y position within the global compositor space
    /// - `physical_width`: width in millimeters of the output
    /// - `physical_height`: height in millimeters of the output
    /// - `subpixel`: subpixel orientation of the output
    /// - `make`: textual description of the manufacturer
    /// - `model`: textual description of the model
    /// - `transform`: additional transformation applied to buffer contents during presentation
    #[inline]
    fn geometry(
        &mut self,
        _slf: &Rc<WlOutput>,
        x: i32,
        y: i32,
        physical_width: i32,
        physical_height: i32,
        subpixel: WlOutputSubpixel,
        make: &str,
        model: &str,
        transform: WlOutputTransform,
    ) {
        let res = _slf.send_geometry(
            x,
            y,
            physical_width,
            physical_height,
            subpixel,
            make,
            model,
            transform,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_output.geometry message: {}", Report::new(e));
        }
    }

    /// advertise available modes for the output
    ///
    /// The mode event describes an available mode for the output.
    ///
    /// The event is sent when binding to the output object and there
    /// will always be one mode, the current mode.  The event is sent
    /// again if an output changes mode, for the mode that is now
    /// current.  In other words, the current mode is always the last
    /// mode that was received with the current flag set.
    ///
    /// Non-current modes are deprecated. A compositor can decide to only
    /// advertise the current mode and never send other modes. Clients
    /// should not rely on non-current modes.
    ///
    /// The size of a mode is given in physical hardware units of
    /// the output device. This is not necessarily the same as
    /// the output size in the global compositor space. For instance,
    /// the output may be scaled, as described in wl_output.scale,
    /// or transformed, as described in wl_output.transform. Clients
    /// willing to retrieve the output size in the global compositor
    /// space should use xdg_output.logical_size instead.
    ///
    /// The vertical refresh rate can be set to zero if it doesn't make
    /// sense for this output (e.g. for virtual outputs).
    ///
    /// The mode event will be followed by a done event (starting from
    /// version 2).
    ///
    /// Clients should not use the refresh rate to schedule frames. Instead,
    /// they should use the wl_surface.frame event or the presentation-time
    /// protocol.
    ///
    /// Note: this information is not always meaningful for all outputs. Some
    /// compositors, such as those exposing virtual outputs, might fake the
    /// refresh rate or the size.
    ///
    /// # Arguments
    ///
    /// - `flags`: bitfield of mode flags
    /// - `width`: width of the mode in hardware units
    /// - `height`: height of the mode in hardware units
    /// - `refresh`: vertical refresh rate in mHz
    #[inline]
    fn mode(
        &mut self,
        _slf: &Rc<WlOutput>,
        flags: WlOutputMode,
        width: i32,
        height: i32,
        refresh: i32,
    ) {
        let res = _slf.send_mode(
            flags,
            width,
            height,
            refresh,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_output.mode message: {}", Report::new(e));
        }
    }

    /// sent all information about output
    ///
    /// This event is sent after all other properties have been
    /// sent after binding to the output object and after any
    /// other property changes done after that. This allows
    /// changes to the output properties to be seen as
    /// atomic, even if they happen via multiple events.
    #[inline]
    fn done(
        &mut self,
        _slf: &Rc<WlOutput>,
    ) {
        let res = _slf.send_done(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_output.done message: {}", Report::new(e));
        }
    }

    /// output scaling properties
    ///
    /// This event contains scaling geometry information
    /// that is not in the geometry event. It may be sent after
    /// binding the output object or if the output scale changes
    /// later. The compositor will emit a non-zero, positive
    /// value for scale. If it is not sent, the client should
    /// assume a scale of 1.
    ///
    /// A scale larger than 1 means that the compositor will
    /// automatically scale surface buffers by this amount
    /// when rendering. This is used for very high resolution
    /// displays where applications rendering at the native
    /// resolution would be too small to be legible.
    ///
    /// Clients should use wl_surface.preferred_buffer_scale
    /// instead of this event to find the preferred buffer
    /// scale to use for a surface.
    ///
    /// The scale event will be followed by a done event.
    ///
    /// # Arguments
    ///
    /// - `factor`: scaling factor of output
    #[inline]
    fn scale(
        &mut self,
        _slf: &Rc<WlOutput>,
        factor: i32,
    ) {
        let res = _slf.send_scale(
            factor,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_output.scale message: {}", Report::new(e));
        }
    }

    /// release the output object
    ///
    /// Using this request a client can tell the server that it is not going to
    /// use the output object anymore.
    #[inline]
    fn release(
        &mut self,
        _slf: &Rc<WlOutput>,
    ) {
        let res = _slf.send_release(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_output.release message: {}", Report::new(e));
        }
    }

    /// name of this output
    ///
    /// Many compositors will assign user-friendly names to their outputs, show
    /// them to the user, allow the user to refer to an output, etc. The client
    /// may wish to know this name as well to offer the user similar behaviors.
    ///
    /// The name is a UTF-8 string with no convention defined for its contents.
    /// Each name is unique among all wl_output globals. The name is only
    /// guaranteed to be unique for the compositor instance.
    ///
    /// The same output name is used for all clients for a given wl_output
    /// global. Thus, the name can be shared across processes to refer to a
    /// specific wl_output global.
    ///
    /// The name is not guaranteed to be persistent across sessions, thus cannot
    /// be used to reliably identify an output in e.g. configuration files.
    ///
    /// Examples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc. However, do
    /// not assume that the name is a reflection of an underlying DRM connector,
    /// X11 connection, etc.
    ///
    /// The name event is sent after binding the output object. This event is
    /// only sent once per output object, and the name does not change over the
    /// lifetime of the wl_output global.
    ///
    /// Compositors may re-use the same output name if the wl_output global is
    /// destroyed and re-created later. Compositors should avoid re-using the
    /// same name if possible.
    ///
    /// The name event will be followed by a done event.
    ///
    /// # Arguments
    ///
    /// - `name`: output name
    #[inline]
    fn name(
        &mut self,
        _slf: &Rc<WlOutput>,
        name: &str,
    ) {
        let res = _slf.send_name(
            name,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_output.name message: {}", Report::new(e));
        }
    }

    /// human-readable description of this output
    ///
    /// Many compositors can produce human-readable descriptions of their
    /// outputs. The client may wish to know this description as well, e.g. for
    /// output selection purposes.
    ///
    /// The description is a UTF-8 string with no convention defined for its
    /// contents. The description is not guaranteed to be unique among all
    /// wl_output globals. Examples might include 'Foocorp 11" Display' or
    /// 'Virtual X11 output via :1'.
    ///
    /// The description event is sent after binding the output object and
    /// whenever the description changes. The description is optional, and may
    /// not be sent at all.
    ///
    /// The description event will be followed by a done event.
    ///
    /// # Arguments
    ///
    /// - `description`: output description
    #[inline]
    fn description(
        &mut self,
        _slf: &Rc<WlOutput>,
        description: &str,
    ) {
        let res = _slf.send_description(
            description,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_output.description message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for WlOutput {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::WlOutput, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_output#{}.release()\n", client.endpoint.id, msg[0]);
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
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("x"));
                };
                offset += 1;
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("y"));
                };
                offset += 1;
                let Some(&arg2) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("physical_width"));
                };
                offset += 1;
                let Some(&arg3) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("physical_height"));
                };
                offset += 1;
                let Some(&arg4) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("subpixel"));
                };
                offset += 1;
                let arg5 = {
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
                let arg6 = {
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
                let Some(&arg7) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("transform"));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                let arg4 = WlOutputSubpixel(arg4);
                let arg7 = WlOutputTransform(arg7);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_output#{}.geometry(x: {}, y: {}, physical_width: {}, physical_height: {}, subpixel: {:?}, make: {:?}, model: {:?}, transform: {:?})\n", msg[0], arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).geometry(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
                } else {
                    DefaultHandler.geometry(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
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
                let arg0 = WlOutputMode(arg0);
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_output#{}.mode(flags: {:?}, width: {}, height: {}, refresh: {})\n", msg[0], arg0, arg1, arg2, arg3);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).mode(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.mode(&self, arg0, arg1, arg2, arg3);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_output#{}.done()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).done(&self);
                } else {
                    DefaultHandler.done(&self);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = arg0 as i32;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_output#{}.scale(factor: {})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).scale(&self, arg0);
                } else {
                    DefaultHandler.scale(&self, arg0);
                }
            }
            4 => {
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
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_output#{}.name(name: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).name(&self, arg0);
                } else {
                    DefaultHandler.name(&self, arg0);
                }
            }
            5 => {
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
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_output#{}.description(description: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).description(&self, arg0);
                } else {
                    DefaultHandler.description(&self, arg0);
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
            0 => "geometry",
            1 => "mode",
            2 => "done",
            3 => "scale",
            4 => "name",
            5 => "description",
            _ => return None,
        };
        Some(name)
    }
}

impl Proxy for WlOutput {
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

impl WlOutput {
    /// Since when the subpixel.unknown enum variant is available.
    pub const ENM__SUBPIXEL_UNKNOWN__SINCE: u32 = 1;
    /// Since when the subpixel.none enum variant is available.
    pub const ENM__SUBPIXEL_NONE__SINCE: u32 = 1;
    /// Since when the subpixel.horizontal_rgb enum variant is available.
    pub const ENM__SUBPIXEL_HORIZONTAL_RGB__SINCE: u32 = 1;
    /// Since when the subpixel.horizontal_bgr enum variant is available.
    pub const ENM__SUBPIXEL_HORIZONTAL_BGR__SINCE: u32 = 1;
    /// Since when the subpixel.vertical_rgb enum variant is available.
    pub const ENM__SUBPIXEL_VERTICAL_RGB__SINCE: u32 = 1;
    /// Since when the subpixel.vertical_bgr enum variant is available.
    pub const ENM__SUBPIXEL_VERTICAL_BGR__SINCE: u32 = 1;

    /// Since when the transform.normal enum variant is available.
    pub const ENM__TRANSFORM_NORMAL__SINCE: u32 = 1;
    /// Since when the transform.90 enum variant is available.
    pub const ENM__TRANSFORM_90__SINCE: u32 = 1;
    /// Since when the transform.180 enum variant is available.
    pub const ENM__TRANSFORM_180__SINCE: u32 = 1;
    /// Since when the transform.270 enum variant is available.
    pub const ENM__TRANSFORM_270__SINCE: u32 = 1;
    /// Since when the transform.flipped enum variant is available.
    pub const ENM__TRANSFORM_FLIPPED__SINCE: u32 = 1;
    /// Since when the transform.flipped_90 enum variant is available.
    pub const ENM__TRANSFORM_FLIPPED_90__SINCE: u32 = 1;
    /// Since when the transform.flipped_180 enum variant is available.
    pub const ENM__TRANSFORM_FLIPPED_180__SINCE: u32 = 1;
    /// Since when the transform.flipped_270 enum variant is available.
    pub const ENM__TRANSFORM_FLIPPED_270__SINCE: u32 = 1;

    /// Since when the mode.current enum variant is available.
    pub const ENM__MODE_CURRENT__SINCE: u32 = 1;
    /// Since when the mode.preferred enum variant is available.
    pub const ENM__MODE_PREFERRED__SINCE: u32 = 1;
}

/// subpixel geometry information
///
/// This enumeration describes how the physical
/// pixels on an output are laid out.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WlOutputSubpixel(pub u32);

impl WlOutputSubpixel {
    /// unknown geometry
    pub const UNKNOWN: Self = Self(0);

    /// no geometry
    pub const NONE: Self = Self(1);

    /// horizontal RGB
    pub const HORIZONTAL_RGB: Self = Self(2);

    /// horizontal BGR
    pub const HORIZONTAL_BGR: Self = Self(3);

    /// vertical RGB
    pub const VERTICAL_RGB: Self = Self(4);

    /// vertical BGR
    pub const VERTICAL_BGR: Self = Self(5);
}

impl Debug for WlOutputSubpixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::UNKNOWN => "UNKNOWN",
            Self::NONE => "NONE",
            Self::HORIZONTAL_RGB => "HORIZONTAL_RGB",
            Self::HORIZONTAL_BGR => "HORIZONTAL_BGR",
            Self::VERTICAL_RGB => "VERTICAL_RGB",
            Self::VERTICAL_BGR => "VERTICAL_BGR",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// transformation applied to buffer contents
///
/// This describes transformations that clients and compositors apply to
/// buffer contents.
///
/// The flipped values correspond to an initial flip around a
/// vertical axis followed by rotation.
///
/// The purpose is mainly to allow clients to render accordingly and
/// tell the compositor, so that for fullscreen surfaces, the
/// compositor will still be able to scan out directly from client
/// surfaces.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WlOutputTransform(pub u32);

impl WlOutputTransform {
    /// no transform
    pub const NORMAL: Self = Self(0);

    /// 90 degrees counter-clockwise
    pub const _90: Self = Self(1);

    /// 180 degrees counter-clockwise
    pub const _180: Self = Self(2);

    /// 270 degrees counter-clockwise
    pub const _270: Self = Self(3);

    /// 180 degree flip around a vertical axis
    pub const FLIPPED: Self = Self(4);

    /// flip and rotate 90 degrees counter-clockwise
    pub const FLIPPED_90: Self = Self(5);

    /// flip and rotate 180 degrees counter-clockwise
    pub const FLIPPED_180: Self = Self(6);

    /// flip and rotate 270 degrees counter-clockwise
    pub const FLIPPED_270: Self = Self(7);
}

impl Debug for WlOutputTransform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NORMAL => "NORMAL",
            Self::_90 => "_90",
            Self::_180 => "_180",
            Self::_270 => "_270",
            Self::FLIPPED => "FLIPPED",
            Self::FLIPPED_90 => "FLIPPED_90",
            Self::FLIPPED_180 => "FLIPPED_180",
            Self::FLIPPED_270 => "FLIPPED_270",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// mode information
///
/// These flags describe properties of an output mode.
/// They are used in the flags bitfield of the mode event.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct WlOutputMode(pub u32);

/// An iterator over the set bits in a [WlOutputMode].
///
/// You can construct this with the `IntoIterator` implementation of `WlOutputMode`.
#[derive(Clone, Debug)]
pub struct WlOutputModeIter(pub u32);

impl WlOutputMode {
    /// indicates this is the current mode
    pub const CURRENT: Self = Self(0x1);

    /// indicates this is the preferred mode
    pub const PREFERRED: Self = Self(0x2);
}

impl WlOutputMode {
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
        Self(0 | 0x1 | 0x2)
    }
}

impl Iterator for WlOutputModeIter {
    type Item = WlOutputMode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(WlOutputMode(bit))
    }
}

impl IntoIterator for WlOutputMode {
    type Item = WlOutputMode;
    type IntoIter = WlOutputModeIter;

    fn into_iter(self) -> Self::IntoIter {
        WlOutputModeIter(self.0)
    }
}

impl BitAnd for WlOutputMode {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for WlOutputMode {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for WlOutputMode {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for WlOutputMode {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for WlOutputMode {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for WlOutputMode {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for WlOutputMode {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for WlOutputMode {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for WlOutputMode {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for WlOutputMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = self.0;
        let mut first = true;
        if v & 0x1 == 0x1 {
            v &= !0x1;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("CURRENT")?;
        }
        if v & 0x2 == 0x2 {
            v &= !0x2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("PREFERRED")?;
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
            f.write_str("0")?;
        }
        Ok(())
    }
}
