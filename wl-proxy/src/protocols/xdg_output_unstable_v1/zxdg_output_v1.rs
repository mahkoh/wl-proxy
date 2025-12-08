//! compositor logical output region
//!
//! An xdg_output describes part of the compositor geometry.
//!
//! This typically corresponds to a monitor that displays part of the
//! compositor space.
//!
//! For objects version 3 onwards, after all xdg_output properties have been
//! sent (when the object is created and when properties are updated), a
//! wl_output.done event is sent. This allows changes to the output
//! properties to be seen as atomic, even if they happen via multiple events.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zxdg_output_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZxdgOutputV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZxdgOutputV1Handler>,
}

struct DefaultHandler;

impl ZxdgOutputV1Handler for DefaultHandler { }

impl ConcreteObject for ZxdgOutputV1 {
    const XML_VERSION: u32 = 3;
    const INTERFACE: ObjectInterface = ObjectInterface::ZxdgOutputV1;
    const INTERFACE_NAME: &str = "zxdg_output_v1";
}

impl ZxdgOutputV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZxdgOutputV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZxdgOutputV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZxdgOutputV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZxdgOutputV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZxdgOutputV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the xdg_output object
    ///
    /// Using this request a client can tell the server that it is not
    /// going to use the xdg_output object anymore.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zxdg_output_v1#{}.destroy()\n", id);
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

    /// destroy the xdg_output object
    ///
    /// Using this request a client can tell the server that it is not
    /// going to use the xdg_output object anymore.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zxdg_output_v1.destroy", &e);
        }
    }

    /// Since when the logical_position message is available.
    pub const MSG__LOGICAL_POSITION__SINCE: u32 = 1;

    /// position of the output within the global compositor space
    ///
    /// The position event describes the location of the wl_output within
    /// the global compositor space.
    ///
    /// The logical_position event is sent after creating an xdg_output
    /// (see xdg_output_manager.get_xdg_output) and whenever the location
    /// of the output changes within the global compositor space.
    ///
    /// # Arguments
    ///
    /// - `x`: x position within the global compositor space
    /// - `y`: y position within the global compositor space
    #[inline]
    pub fn try_send_logical_position(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zxdg_output_v1#{}.logical_position(x: {}, y: {})\n", client_id, id, arg0, arg1);
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
            0,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// position of the output within the global compositor space
    ///
    /// The position event describes the location of the wl_output within
    /// the global compositor space.
    ///
    /// The logical_position event is sent after creating an xdg_output
    /// (see xdg_output_manager.get_xdg_output) and whenever the location
    /// of the output changes within the global compositor space.
    ///
    /// # Arguments
    ///
    /// - `x`: x position within the global compositor space
    /// - `y`: y position within the global compositor space
    #[inline]
    pub fn send_logical_position(
        &self,
        x: i32,
        y: i32,
    ) {
        let res = self.try_send_logical_position(
            x,
            y,
        );
        if let Err(e) = res {
            log_send("zxdg_output_v1.logical_position", &e);
        }
    }

    /// Since when the logical_size message is available.
    pub const MSG__LOGICAL_SIZE__SINCE: u32 = 1;

    /// size of the output in the global compositor space
    ///
    /// The logical_size event describes the size of the output in the
    /// global compositor space.
    ///
    /// Most regular Wayland clients should not pay attention to the
    /// logical size and would rather rely on xdg_shell interfaces.
    ///
    /// Some clients such as Xwayland, however, need this to configure
    /// their surfaces in the global compositor space as the compositor
    /// may apply a different scale from what is advertised by the output
    /// scaling property (to achieve fractional scaling, for example).
    ///
    /// For example, for a wl_output mode 3840×2160 and a scale factor 2:
    ///
    /// - A compositor not scaling the monitor viewport in its compositing space
    ///   will advertise a logical size of 3840×2160,
    ///
    /// - A compositor scaling the monitor viewport with scale factor 2 will
    ///   advertise a logical size of 1920×1080,
    ///
    /// - A compositor scaling the monitor viewport using a fractional scale of
    ///   1.5 will advertise a logical size of 2560×1440.
    ///
    /// For example, for a wl_output mode 1920×1080 and a 90 degree rotation,
    /// the compositor will advertise a logical size of 1080x1920.
    ///
    /// The logical_size event is sent after creating an xdg_output
    /// (see xdg_output_manager.get_xdg_output) and whenever the logical
    /// size of the output changes, either as a result of a change in the
    /// applied scale or because of a change in the corresponding output
    /// mode(see wl_output.mode) or transform (see wl_output.transform).
    ///
    /// # Arguments
    ///
    /// - `width`: width in global compositor space
    /// - `height`: height in global compositor space
    #[inline]
    pub fn try_send_logical_size(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zxdg_output_v1#{}.logical_size(width: {}, height: {})\n", client_id, id, arg0, arg1);
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
            1,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// size of the output in the global compositor space
    ///
    /// The logical_size event describes the size of the output in the
    /// global compositor space.
    ///
    /// Most regular Wayland clients should not pay attention to the
    /// logical size and would rather rely on xdg_shell interfaces.
    ///
    /// Some clients such as Xwayland, however, need this to configure
    /// their surfaces in the global compositor space as the compositor
    /// may apply a different scale from what is advertised by the output
    /// scaling property (to achieve fractional scaling, for example).
    ///
    /// For example, for a wl_output mode 3840×2160 and a scale factor 2:
    ///
    /// - A compositor not scaling the monitor viewport in its compositing space
    ///   will advertise a logical size of 3840×2160,
    ///
    /// - A compositor scaling the monitor viewport with scale factor 2 will
    ///   advertise a logical size of 1920×1080,
    ///
    /// - A compositor scaling the monitor viewport using a fractional scale of
    ///   1.5 will advertise a logical size of 2560×1440.
    ///
    /// For example, for a wl_output mode 1920×1080 and a 90 degree rotation,
    /// the compositor will advertise a logical size of 1080x1920.
    ///
    /// The logical_size event is sent after creating an xdg_output
    /// (see xdg_output_manager.get_xdg_output) and whenever the logical
    /// size of the output changes, either as a result of a change in the
    /// applied scale or because of a change in the corresponding output
    /// mode(see wl_output.mode) or transform (see wl_output.transform).
    ///
    /// # Arguments
    ///
    /// - `width`: width in global compositor space
    /// - `height`: height in global compositor space
    #[inline]
    pub fn send_logical_size(
        &self,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_logical_size(
            width,
            height,
        );
        if let Err(e) = res {
            log_send("zxdg_output_v1.logical_size", &e);
        }
    }

    /// Since when the done message is available.
    pub const MSG__DONE__SINCE: u32 = 1;

    /// Since when the done message is deprecated.
    pub const MSG__DONE__DEPRECATED_SINCE: u32 = 3;

    /// all information about the output have been sent
    ///
    /// This event is sent after all other properties of an xdg_output
    /// have been sent.
    ///
    /// This allows changes to the xdg_output properties to be seen as
    /// atomic, even if they happen via multiple events.
    ///
    /// For objects version 3 onwards, this event is deprecated. Compositors
    /// are not required to send it anymore and must send wl_output.done
    /// instead.
    #[inline]
    pub fn try_send_done(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zxdg_output_v1#{}.done()\n", client_id, id);
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
            2,
        ]);
        Ok(())
    }

    /// all information about the output have been sent
    ///
    /// This event is sent after all other properties of an xdg_output
    /// have been sent.
    ///
    /// This allows changes to the xdg_output properties to be seen as
    /// atomic, even if they happen via multiple events.
    ///
    /// For objects version 3 onwards, this event is deprecated. Compositors
    /// are not required to send it anymore and must send wl_output.done
    /// instead.
    #[inline]
    pub fn send_done(
        &self,
    ) {
        let res = self.try_send_done(
        );
        if let Err(e) = res {
            log_send("zxdg_output_v1.done", &e);
        }
    }

    /// Since when the name message is available.
    pub const MSG__NAME__SINCE: u32 = 2;

    /// name of this output
    ///
    /// Many compositors will assign names to their outputs, show them to the
    /// user, allow them to be configured by name, etc. The client may wish to
    /// know this name as well to offer the user similar behaviors.
    ///
    /// The naming convention is compositor defined, but limited to
    /// alphanumeric characters and dashes (-). Each name is unique among all
    /// wl_output globals, but if a wl_output global is destroyed the same name
    /// may be reused later. The names will also remain consistent across
    /// sessions with the same hardware and software configuration.
    ///
    /// Examples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc. However, do
    /// not assume that the name is a reflection of an underlying DRM
    /// connector, X11 connection, etc.
    ///
    /// The name event is sent after creating an xdg_output (see
    /// xdg_output_manager.get_xdg_output). This event is only sent once per
    /// xdg_output, and the name does not change over the lifetime of the
    /// wl_output global.
    ///
    /// This event is deprecated, instead clients should use wl_output.name.
    /// Compositors must still support this event.
    ///
    /// # Arguments
    ///
    /// - `name`: output name
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zxdg_output_v1#{}.name(name: {:?})\n", client_id, id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// name of this output
    ///
    /// Many compositors will assign names to their outputs, show them to the
    /// user, allow them to be configured by name, etc. The client may wish to
    /// know this name as well to offer the user similar behaviors.
    ///
    /// The naming convention is compositor defined, but limited to
    /// alphanumeric characters and dashes (-). Each name is unique among all
    /// wl_output globals, but if a wl_output global is destroyed the same name
    /// may be reused later. The names will also remain consistent across
    /// sessions with the same hardware and software configuration.
    ///
    /// Examples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc. However, do
    /// not assume that the name is a reflection of an underlying DRM
    /// connector, X11 connection, etc.
    ///
    /// The name event is sent after creating an xdg_output (see
    /// xdg_output_manager.get_xdg_output). This event is only sent once per
    /// xdg_output, and the name does not change over the lifetime of the
    /// wl_output global.
    ///
    /// This event is deprecated, instead clients should use wl_output.name.
    /// Compositors must still support this event.
    ///
    /// # Arguments
    ///
    /// - `name`: output name
    #[inline]
    pub fn send_name(
        &self,
        name: &str,
    ) {
        let res = self.try_send_name(
            name,
        );
        if let Err(e) = res {
            log_send("zxdg_output_v1.name", &e);
        }
    }

    /// Since when the description message is available.
    pub const MSG__DESCRIPTION__SINCE: u32 = 2;

    /// human-readable description of this output
    ///
    /// Many compositors can produce human-readable descriptions of their
    /// outputs.  The client may wish to know this description as well, to
    /// communicate the user for various purposes.
    ///
    /// The description is a UTF-8 string with no convention defined for its
    /// contents. Examples might include 'Foocorp 11" Display' or 'Virtual X11
    /// output via :1'.
    ///
    /// The description event is sent after creating an xdg_output (see
    /// xdg_output_manager.get_xdg_output) and whenever the description
    /// changes. The description is optional, and may not be sent at all.
    ///
    /// For objects of version 2 and lower, this event is only sent once per
    /// xdg_output, and the description does not change over the lifetime of
    /// the wl_output global.
    ///
    /// This event is deprecated, instead clients should use
    /// wl_output.description. Compositors must still support this event.
    ///
    /// # Arguments
    ///
    /// - `description`: output description
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zxdg_output_v1#{}.description(description: {:?})\n", client_id, id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// human-readable description of this output
    ///
    /// Many compositors can produce human-readable descriptions of their
    /// outputs.  The client may wish to know this description as well, to
    /// communicate the user for various purposes.
    ///
    /// The description is a UTF-8 string with no convention defined for its
    /// contents. Examples might include 'Foocorp 11" Display' or 'Virtual X11
    /// output via :1'.
    ///
    /// The description event is sent after creating an xdg_output (see
    /// xdg_output_manager.get_xdg_output) and whenever the description
    /// changes. The description is optional, and may not be sent at all.
    ///
    /// For objects of version 2 and lower, this event is only sent once per
    /// xdg_output, and the description does not change over the lifetime of
    /// the wl_output global.
    ///
    /// This event is deprecated, instead clients should use
    /// wl_output.description. Compositors must still support this event.
    ///
    /// # Arguments
    ///
    /// - `description`: output description
    #[inline]
    pub fn send_description(
        &self,
        description: &str,
    ) {
        let res = self.try_send_description(
            description,
        );
        if let Err(e) = res {
            log_send("zxdg_output_v1.description", &e);
        }
    }
}

/// A message handler for [`ZxdgOutputV1`] proxies.
pub trait ZxdgOutputV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZxdgOutputV1>) {
        slf.core.delete_id();
    }

    /// destroy the xdg_output object
    ///
    /// Using this request a client can tell the server that it is not
    /// going to use the xdg_output object anymore.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZxdgOutputV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zxdg_output_v1.destroy", &e);
        }
    }

    /// position of the output within the global compositor space
    ///
    /// The position event describes the location of the wl_output within
    /// the global compositor space.
    ///
    /// The logical_position event is sent after creating an xdg_output
    /// (see xdg_output_manager.get_xdg_output) and whenever the location
    /// of the output changes within the global compositor space.
    ///
    /// # Arguments
    ///
    /// - `x`: x position within the global compositor space
    /// - `y`: y position within the global compositor space
    #[inline]
    fn handle_logical_position(
        &mut self,
        slf: &Rc<ZxdgOutputV1>,
        x: i32,
        y: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_logical_position(
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("zxdg_output_v1.logical_position", &e);
        }
    }

    /// size of the output in the global compositor space
    ///
    /// The logical_size event describes the size of the output in the
    /// global compositor space.
    ///
    /// Most regular Wayland clients should not pay attention to the
    /// logical size and would rather rely on xdg_shell interfaces.
    ///
    /// Some clients such as Xwayland, however, need this to configure
    /// their surfaces in the global compositor space as the compositor
    /// may apply a different scale from what is advertised by the output
    /// scaling property (to achieve fractional scaling, for example).
    ///
    /// For example, for a wl_output mode 3840×2160 and a scale factor 2:
    ///
    /// - A compositor not scaling the monitor viewport in its compositing space
    ///   will advertise a logical size of 3840×2160,
    ///
    /// - A compositor scaling the monitor viewport with scale factor 2 will
    ///   advertise a logical size of 1920×1080,
    ///
    /// - A compositor scaling the monitor viewport using a fractional scale of
    ///   1.5 will advertise a logical size of 2560×1440.
    ///
    /// For example, for a wl_output mode 1920×1080 and a 90 degree rotation,
    /// the compositor will advertise a logical size of 1080x1920.
    ///
    /// The logical_size event is sent after creating an xdg_output
    /// (see xdg_output_manager.get_xdg_output) and whenever the logical
    /// size of the output changes, either as a result of a change in the
    /// applied scale or because of a change in the corresponding output
    /// mode(see wl_output.mode) or transform (see wl_output.transform).
    ///
    /// # Arguments
    ///
    /// - `width`: width in global compositor space
    /// - `height`: height in global compositor space
    #[inline]
    fn handle_logical_size(
        &mut self,
        slf: &Rc<ZxdgOutputV1>,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_logical_size(
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("zxdg_output_v1.logical_size", &e);
        }
    }

    /// all information about the output have been sent
    ///
    /// This event is sent after all other properties of an xdg_output
    /// have been sent.
    ///
    /// This allows changes to the xdg_output properties to be seen as
    /// atomic, even if they happen via multiple events.
    ///
    /// For objects version 3 onwards, this event is deprecated. Compositors
    /// are not required to send it anymore and must send wl_output.done
    /// instead.
    #[inline]
    fn handle_done(
        &mut self,
        slf: &Rc<ZxdgOutputV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_done(
        );
        if let Err(e) = res {
            log_forward("zxdg_output_v1.done", &e);
        }
    }

    /// name of this output
    ///
    /// Many compositors will assign names to their outputs, show them to the
    /// user, allow them to be configured by name, etc. The client may wish to
    /// know this name as well to offer the user similar behaviors.
    ///
    /// The naming convention is compositor defined, but limited to
    /// alphanumeric characters and dashes (-). Each name is unique among all
    /// wl_output globals, but if a wl_output global is destroyed the same name
    /// may be reused later. The names will also remain consistent across
    /// sessions with the same hardware and software configuration.
    ///
    /// Examples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc. However, do
    /// not assume that the name is a reflection of an underlying DRM
    /// connector, X11 connection, etc.
    ///
    /// The name event is sent after creating an xdg_output (see
    /// xdg_output_manager.get_xdg_output). This event is only sent once per
    /// xdg_output, and the name does not change over the lifetime of the
    /// wl_output global.
    ///
    /// This event is deprecated, instead clients should use wl_output.name.
    /// Compositors must still support this event.
    ///
    /// # Arguments
    ///
    /// - `name`: output name
    #[inline]
    fn handle_name(
        &mut self,
        slf: &Rc<ZxdgOutputV1>,
        name: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_name(
            name,
        );
        if let Err(e) = res {
            log_forward("zxdg_output_v1.name", &e);
        }
    }

    /// human-readable description of this output
    ///
    /// Many compositors can produce human-readable descriptions of their
    /// outputs.  The client may wish to know this description as well, to
    /// communicate the user for various purposes.
    ///
    /// The description is a UTF-8 string with no convention defined for its
    /// contents. Examples might include 'Foocorp 11" Display' or 'Virtual X11
    /// output via :1'.
    ///
    /// The description event is sent after creating an xdg_output (see
    /// xdg_output_manager.get_xdg_output) and whenever the description
    /// changes. The description is optional, and may not be sent at all.
    ///
    /// For objects of version 2 and lower, this event is only sent once per
    /// xdg_output, and the description does not change over the lifetime of
    /// the wl_output global.
    ///
    /// This event is deprecated, instead clients should use
    /// wl_output.description. Compositors must still support this event.
    ///
    /// # Arguments
    ///
    /// - `description`: output description
    #[inline]
    fn handle_description(
        &mut self,
        slf: &Rc<ZxdgOutputV1>,
        description: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_description(
            description,
        );
        if let Err(e) = res {
            log_forward("zxdg_output_v1.description", &e);
        }
    }
}

impl ObjectPrivate for ZxdgOutputV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZxdgOutputV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zxdg_output_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zxdg_output_v1#{}.logical_position(x: {}, y: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_logical_position(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_logical_position(&self, arg0, arg1);
                }
            }
            1 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zxdg_output_v1#{}.logical_size(width: {}, height: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_logical_size(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_logical_size(&self, arg0, arg1);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zxdg_output_v1#{}.done()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_done(&self);
                } else {
                    DefaultHandler.handle_done(&self);
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
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zxdg_output_v1#{}.name(name: {:?})\n", id, arg0);
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
            4 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zxdg_output_v1#{}.description(description: {:?})\n", id, arg0);
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
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "logical_position",
            1 => "logical_size",
            2 => "done",
            3 => "name",
            4 => "description",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZxdgOutputV1 {
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

