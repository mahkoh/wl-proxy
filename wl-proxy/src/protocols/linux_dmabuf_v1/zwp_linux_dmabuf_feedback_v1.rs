//! dmabuf feedback
//!
//! This object advertises dmabuf parameters feedback. This includes the
//! preferred devices and the supported formats/modifiers.
//!
//! The parameters are sent once when this object is created and whenever they
//! change. The done event is always sent once after all parameters have been
//! sent. When a single parameter changes, all parameters are re-sent by the
//! compositor.
//!
//! Compositors can re-send the parameters when the current client buffer
//! allocations are sub-optimal. Compositors should not re-send the
//! parameters if re-allocating the buffers would not result in a more optimal
//! configuration. In particular, compositors should avoid sending the exact
//! same parameters multiple times in a row.
//!
//! The tranche_target_device and tranche_formats events are grouped by
//! tranches of preference. For each tranche, a tranche_target_device, one
//! tranche_flags and one or more tranche_formats events are sent, followed
//! by a tranche_done event finishing the list. The tranches are sent in
//! descending order of preference. All formats and modifiers in the same
//! tranche have the same preference.
//!
//! To send parameters, the compositor sends one main_device event, tranches
//! (each consisting of one tranche_target_device event, one tranche_flags
//! event, tranche_formats events and then a tranche_done event), then one
//! done event.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_linux_dmabuf_feedback_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpLinuxDmabufFeedbackV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpLinuxDmabufFeedbackV1Handler>,
}

struct DefaultHandler;

impl ZwpLinuxDmabufFeedbackV1Handler for DefaultHandler { }

impl ConcreteObject for ZwpLinuxDmabufFeedbackV1 {
    const XML_VERSION: u32 = 5;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpLinuxDmabufFeedbackV1;
    const INTERFACE_NAME: &str = "zwp_linux_dmabuf_feedback_v1";
}

impl ZwpLinuxDmabufFeedbackV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpLinuxDmabufFeedbackV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpLinuxDmabufFeedbackV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpLinuxDmabufFeedbackV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpLinuxDmabufFeedbackV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpLinuxDmabufFeedbackV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the feedback object
    ///
    /// Using this request a client can tell the server that it is not going to
    /// use the wp_linux_dmabuf_feedback object anymore.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_linux_dmabuf_feedback_v1#{}.destroy()\n", id);
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

    /// destroy the feedback object
    ///
    /// Using this request a client can tell the server that it is not going to
    /// use the wp_linux_dmabuf_feedback object anymore.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwp_linux_dmabuf_feedback_v1.destroy", &e);
        }
    }

    /// Since when the done message is available.
    pub const MSG__DONE__SINCE: u32 = 1;

    /// all feedback has been sent
    ///
    /// This event is sent after all parameters of a wp_linux_dmabuf_feedback
    /// object have been sent.
    ///
    /// This allows changes to the wp_linux_dmabuf_feedback parameters to be
    /// seen as atomic, even if they happen via multiple events.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_linux_dmabuf_feedback_v1#{}.done()\n", client_id, id);
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

    /// all feedback has been sent
    ///
    /// This event is sent after all parameters of a wp_linux_dmabuf_feedback
    /// object have been sent.
    ///
    /// This allows changes to the wp_linux_dmabuf_feedback parameters to be
    /// seen as atomic, even if they happen via multiple events.
    #[inline]
    pub fn send_done(
        &self,
    ) {
        let res = self.try_send_done(
        );
        if let Err(e) = res {
            log_send("zwp_linux_dmabuf_feedback_v1.done", &e);
        }
    }

    /// Since when the format_table message is available.
    pub const MSG__FORMAT_TABLE__SINCE: u32 = 1;

    /// format and modifier table
    ///
    /// This event provides a file descriptor which can be memory-mapped to
    /// access the format and modifier table.
    ///
    /// The table contains a tightly packed array of consecutive format +
    /// modifier pairs. Each pair is 16 bytes wide. It contains a format as a
    /// 32-bit unsigned integer, followed by 4 bytes of unused padding, and a
    /// modifier as a 64-bit unsigned integer. The native endianness is used.
    ///
    /// The client must map the file descriptor in read-only private mode.
    ///
    /// Compositors are not allowed to mutate the table file contents once this
    /// event has been sent. Instead, compositors must create a new, separate
    /// table file and re-send feedback parameters. Compositors are allowed to
    /// store duplicate format + modifier pairs in the table.
    ///
    /// # Arguments
    ///
    /// - `fd`: table file descriptor
    /// - `size`: table size, in bytes
    #[inline]
    pub fn try_send_format_table(
        &self,
        fd: &Rc<OwnedFd>,
        size: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_linux_dmabuf_feedback_v1#{}.format_table(fd: {}, size: {})\n", client_id, id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0.as_raw_fd(), arg1);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.fds.push_back(arg0.clone());
        fmt.words([
            id,
            1,
            arg1,
        ]);
        Ok(())
    }

    /// format and modifier table
    ///
    /// This event provides a file descriptor which can be memory-mapped to
    /// access the format and modifier table.
    ///
    /// The table contains a tightly packed array of consecutive format +
    /// modifier pairs. Each pair is 16 bytes wide. It contains a format as a
    /// 32-bit unsigned integer, followed by 4 bytes of unused padding, and a
    /// modifier as a 64-bit unsigned integer. The native endianness is used.
    ///
    /// The client must map the file descriptor in read-only private mode.
    ///
    /// Compositors are not allowed to mutate the table file contents once this
    /// event has been sent. Instead, compositors must create a new, separate
    /// table file and re-send feedback parameters. Compositors are allowed to
    /// store duplicate format + modifier pairs in the table.
    ///
    /// # Arguments
    ///
    /// - `fd`: table file descriptor
    /// - `size`: table size, in bytes
    #[inline]
    pub fn send_format_table(
        &self,
        fd: &Rc<OwnedFd>,
        size: u32,
    ) {
        let res = self.try_send_format_table(
            fd,
            size,
        );
        if let Err(e) = res {
            log_send("zwp_linux_dmabuf_feedback_v1.format_table", &e);
        }
    }

    /// Since when the main_device message is available.
    pub const MSG__MAIN_DEVICE__SINCE: u32 = 1;

    /// preferred main device
    ///
    /// This event advertises the main device that the server prefers to use
    /// when direct scan-out to the target device isn't possible. The
    /// advertised main device may be different for each
    /// wp_linux_dmabuf_feedback object, and may change over time.
    ///
    /// There is exactly one main device. The compositor must send at least
    /// one preference tranche with tranche_target_device equal to main_device.
    ///
    /// Clients need to create buffers that the main device can import and
    /// read from, otherwise creating the dmabuf wl_buffer will fail (see the
    /// wp_linux_buffer_params.create and create_immed requests for details).
    /// The main device will also likely be kept active by the compositor,
    /// so clients can use it instead of waking up another device for power
    /// savings.
    ///
    /// In general the device is a DRM node. The DRM node type (primary vs.
    /// render) is unspecified. Clients must not rely on the compositor sending
    /// a particular node type. Clients cannot check two devices for equality
    /// by comparing the dev_t value.
    ///
    /// If explicit modifiers are not supported and the client performs buffer
    /// allocations on a different device than the main device, then the client
    /// must force the buffer to have a linear layout.
    ///
    /// # Arguments
    ///
    /// - `device`: device dev_t value
    #[inline]
    pub fn try_send_main_device(
        &self,
        device: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            device,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_linux_dmabuf_feedback_v1#{}.main_device(device: {})\n", client_id, id, debug_array(arg0));
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
        fmt.array(arg0);
        Ok(())
    }

    /// preferred main device
    ///
    /// This event advertises the main device that the server prefers to use
    /// when direct scan-out to the target device isn't possible. The
    /// advertised main device may be different for each
    /// wp_linux_dmabuf_feedback object, and may change over time.
    ///
    /// There is exactly one main device. The compositor must send at least
    /// one preference tranche with tranche_target_device equal to main_device.
    ///
    /// Clients need to create buffers that the main device can import and
    /// read from, otherwise creating the dmabuf wl_buffer will fail (see the
    /// wp_linux_buffer_params.create and create_immed requests for details).
    /// The main device will also likely be kept active by the compositor,
    /// so clients can use it instead of waking up another device for power
    /// savings.
    ///
    /// In general the device is a DRM node. The DRM node type (primary vs.
    /// render) is unspecified. Clients must not rely on the compositor sending
    /// a particular node type. Clients cannot check two devices for equality
    /// by comparing the dev_t value.
    ///
    /// If explicit modifiers are not supported and the client performs buffer
    /// allocations on a different device than the main device, then the client
    /// must force the buffer to have a linear layout.
    ///
    /// # Arguments
    ///
    /// - `device`: device dev_t value
    #[inline]
    pub fn send_main_device(
        &self,
        device: &[u8],
    ) {
        let res = self.try_send_main_device(
            device,
        );
        if let Err(e) = res {
            log_send("zwp_linux_dmabuf_feedback_v1.main_device", &e);
        }
    }

    /// Since when the tranche_done message is available.
    pub const MSG__TRANCHE_DONE__SINCE: u32 = 1;

    /// a preference tranche has been sent
    ///
    /// This event splits tranche_target_device and tranche_formats events in
    /// preference tranches. It is sent after a set of tranche_target_device
    /// and tranche_formats events; it represents the end of a tranche. The
    /// next tranche will have a lower preference.
    #[inline]
    pub fn try_send_tranche_done(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_linux_dmabuf_feedback_v1#{}.tranche_done()\n", client_id, id);
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

    /// a preference tranche has been sent
    ///
    /// This event splits tranche_target_device and tranche_formats events in
    /// preference tranches. It is sent after a set of tranche_target_device
    /// and tranche_formats events; it represents the end of a tranche. The
    /// next tranche will have a lower preference.
    #[inline]
    pub fn send_tranche_done(
        &self,
    ) {
        let res = self.try_send_tranche_done(
        );
        if let Err(e) = res {
            log_send("zwp_linux_dmabuf_feedback_v1.tranche_done", &e);
        }
    }

    /// Since when the tranche_target_device message is available.
    pub const MSG__TRANCHE_TARGET_DEVICE__SINCE: u32 = 1;

    /// target device
    ///
    /// This event advertises the target device that the server prefers to use
    /// for a buffer created given this tranche. The advertised target device
    /// may be different for each preference tranche, and may change over time.
    ///
    /// There is exactly one target device per tranche.
    ///
    /// The target device may be a scan-out device, for example if the
    /// compositor prefers to directly scan-out a buffer created given this
    /// tranche. The target device may be a rendering device, for example if
    /// the compositor prefers to texture from said buffer.
    ///
    /// The client can use this hint to allocate the buffer in a way that makes
    /// it accessible from the target device, ideally directly. The buffer must
    /// still be accessible from the main device, either through direct import
    /// or through a potentially more expensive fallback path. If the buffer
    /// can't be directly imported from the main device then clients must be
    /// prepared for the compositor changing the tranche priority or making
    /// wl_buffer creation fail (see the wp_linux_buffer_params.create and
    /// create_immed requests for details).
    ///
    /// If the device is a DRM node, the DRM node type (primary vs. render) is
    /// unspecified. Clients must not rely on the compositor sending a
    /// particular node type. Clients cannot check two devices for equality by
    /// comparing the dev_t value.
    ///
    /// This event is tied to a preference tranche, see the tranche_done event.
    ///
    /// # Arguments
    ///
    /// - `device`: device dev_t value
    #[inline]
    pub fn try_send_tranche_target_device(
        &self,
        device: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            device,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_linux_dmabuf_feedback_v1#{}.tranche_target_device(device: {})\n", client_id, id, debug_array(arg0));
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
        fmt.array(arg0);
        Ok(())
    }

    /// target device
    ///
    /// This event advertises the target device that the server prefers to use
    /// for a buffer created given this tranche. The advertised target device
    /// may be different for each preference tranche, and may change over time.
    ///
    /// There is exactly one target device per tranche.
    ///
    /// The target device may be a scan-out device, for example if the
    /// compositor prefers to directly scan-out a buffer created given this
    /// tranche. The target device may be a rendering device, for example if
    /// the compositor prefers to texture from said buffer.
    ///
    /// The client can use this hint to allocate the buffer in a way that makes
    /// it accessible from the target device, ideally directly. The buffer must
    /// still be accessible from the main device, either through direct import
    /// or through a potentially more expensive fallback path. If the buffer
    /// can't be directly imported from the main device then clients must be
    /// prepared for the compositor changing the tranche priority or making
    /// wl_buffer creation fail (see the wp_linux_buffer_params.create and
    /// create_immed requests for details).
    ///
    /// If the device is a DRM node, the DRM node type (primary vs. render) is
    /// unspecified. Clients must not rely on the compositor sending a
    /// particular node type. Clients cannot check two devices for equality by
    /// comparing the dev_t value.
    ///
    /// This event is tied to a preference tranche, see the tranche_done event.
    ///
    /// # Arguments
    ///
    /// - `device`: device dev_t value
    #[inline]
    pub fn send_tranche_target_device(
        &self,
        device: &[u8],
    ) {
        let res = self.try_send_tranche_target_device(
            device,
        );
        if let Err(e) = res {
            log_send("zwp_linux_dmabuf_feedback_v1.tranche_target_device", &e);
        }
    }

    /// Since when the tranche_formats message is available.
    pub const MSG__TRANCHE_FORMATS__SINCE: u32 = 1;

    /// supported buffer format modifier
    ///
    /// This event advertises the format + modifier combinations that the
    /// compositor supports.
    ///
    /// It carries an array of indices, each referring to a format + modifier
    /// pair in the last received format table (see the format_table event).
    /// Each index is a 16-bit unsigned integer in native endianness.
    ///
    /// For legacy support, DRM_FORMAT_MOD_INVALID is an allowed modifier.
    /// It indicates that the server can support the format with an implicit
    /// modifier. When a buffer has DRM_FORMAT_MOD_INVALID as its modifier, it
    /// is as if no explicit modifier is specified. The effective modifier
    /// will be derived from the dmabuf.
    ///
    /// A compositor that sends valid modifiers and DRM_FORMAT_MOD_INVALID for
    /// a given format supports both explicit modifiers and implicit modifiers.
    ///
    /// Compositors must not send duplicate format + modifier pairs within the
    /// same tranche or across two different tranches with the same target
    /// device and flags.
    ///
    /// This event is tied to a preference tranche, see the tranche_done event.
    ///
    /// For the definition of the format and modifier codes, see the
    /// wp_linux_buffer_params.create request.
    ///
    /// # Arguments
    ///
    /// - `indices`: array of 16-bit indexes
    #[inline]
    pub fn try_send_tranche_formats(
        &self,
        indices: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            indices,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_linux_dmabuf_feedback_v1#{}.tranche_formats(indices: {})\n", client_id, id, debug_array(arg0));
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
        ]);
        fmt.array(arg0);
        Ok(())
    }

    /// supported buffer format modifier
    ///
    /// This event advertises the format + modifier combinations that the
    /// compositor supports.
    ///
    /// It carries an array of indices, each referring to a format + modifier
    /// pair in the last received format table (see the format_table event).
    /// Each index is a 16-bit unsigned integer in native endianness.
    ///
    /// For legacy support, DRM_FORMAT_MOD_INVALID is an allowed modifier.
    /// It indicates that the server can support the format with an implicit
    /// modifier. When a buffer has DRM_FORMAT_MOD_INVALID as its modifier, it
    /// is as if no explicit modifier is specified. The effective modifier
    /// will be derived from the dmabuf.
    ///
    /// A compositor that sends valid modifiers and DRM_FORMAT_MOD_INVALID for
    /// a given format supports both explicit modifiers and implicit modifiers.
    ///
    /// Compositors must not send duplicate format + modifier pairs within the
    /// same tranche or across two different tranches with the same target
    /// device and flags.
    ///
    /// This event is tied to a preference tranche, see the tranche_done event.
    ///
    /// For the definition of the format and modifier codes, see the
    /// wp_linux_buffer_params.create request.
    ///
    /// # Arguments
    ///
    /// - `indices`: array of 16-bit indexes
    #[inline]
    pub fn send_tranche_formats(
        &self,
        indices: &[u8],
    ) {
        let res = self.try_send_tranche_formats(
            indices,
        );
        if let Err(e) = res {
            log_send("zwp_linux_dmabuf_feedback_v1.tranche_formats", &e);
        }
    }

    /// Since when the tranche_flags message is available.
    pub const MSG__TRANCHE_FLAGS__SINCE: u32 = 1;

    /// tranche flags
    ///
    /// This event sets tranche-specific flags.
    ///
    /// The scanout flag is a hint that direct scan-out may be attempted by the
    /// compositor on the target device if the client appropriately allocates a
    /// buffer. How to allocate a buffer that can be scanned out on the target
    /// device is implementation-defined.
    ///
    /// This event is tied to a preference tranche, see the tranche_done event.
    ///
    /// # Arguments
    ///
    /// - `flags`: tranche flags
    #[inline]
    pub fn try_send_tranche_flags(
        &self,
        flags: ZwpLinuxDmabufFeedbackV1TrancheFlags,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            flags,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: ZwpLinuxDmabufFeedbackV1TrancheFlags) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_linux_dmabuf_feedback_v1#{}.tranche_flags(flags: {:?})\n", client_id, id, arg0);
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

    /// tranche flags
    ///
    /// This event sets tranche-specific flags.
    ///
    /// The scanout flag is a hint that direct scan-out may be attempted by the
    /// compositor on the target device if the client appropriately allocates a
    /// buffer. How to allocate a buffer that can be scanned out on the target
    /// device is implementation-defined.
    ///
    /// This event is tied to a preference tranche, see the tranche_done event.
    ///
    /// # Arguments
    ///
    /// - `flags`: tranche flags
    #[inline]
    pub fn send_tranche_flags(
        &self,
        flags: ZwpLinuxDmabufFeedbackV1TrancheFlags,
    ) {
        let res = self.try_send_tranche_flags(
            flags,
        );
        if let Err(e) = res {
            log_send("zwp_linux_dmabuf_feedback_v1.tranche_flags", &e);
        }
    }
}

/// A message handler for [`ZwpLinuxDmabufFeedbackV1`] proxies.
pub trait ZwpLinuxDmabufFeedbackV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpLinuxDmabufFeedbackV1>) {
        slf.core.delete_id();
    }

    /// destroy the feedback object
    ///
    /// Using this request a client can tell the server that it is not going to
    /// use the wp_linux_dmabuf_feedback object anymore.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwpLinuxDmabufFeedbackV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwp_linux_dmabuf_feedback_v1.destroy", &e);
        }
    }

    /// all feedback has been sent
    ///
    /// This event is sent after all parameters of a wp_linux_dmabuf_feedback
    /// object have been sent.
    ///
    /// This allows changes to the wp_linux_dmabuf_feedback parameters to be
    /// seen as atomic, even if they happen via multiple events.
    #[inline]
    fn handle_done(
        &mut self,
        slf: &Rc<ZwpLinuxDmabufFeedbackV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_done(
        );
        if let Err(e) = res {
            log_forward("zwp_linux_dmabuf_feedback_v1.done", &e);
        }
    }

    /// format and modifier table
    ///
    /// This event provides a file descriptor which can be memory-mapped to
    /// access the format and modifier table.
    ///
    /// The table contains a tightly packed array of consecutive format +
    /// modifier pairs. Each pair is 16 bytes wide. It contains a format as a
    /// 32-bit unsigned integer, followed by 4 bytes of unused padding, and a
    /// modifier as a 64-bit unsigned integer. The native endianness is used.
    ///
    /// The client must map the file descriptor in read-only private mode.
    ///
    /// Compositors are not allowed to mutate the table file contents once this
    /// event has been sent. Instead, compositors must create a new, separate
    /// table file and re-send feedback parameters. Compositors are allowed to
    /// store duplicate format + modifier pairs in the table.
    ///
    /// # Arguments
    ///
    /// - `fd`: table file descriptor
    /// - `size`: table size, in bytes
    #[inline]
    fn handle_format_table(
        &mut self,
        slf: &Rc<ZwpLinuxDmabufFeedbackV1>,
        fd: &Rc<OwnedFd>,
        size: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_format_table(
            fd,
            size,
        );
        if let Err(e) = res {
            log_forward("zwp_linux_dmabuf_feedback_v1.format_table", &e);
        }
    }

    /// preferred main device
    ///
    /// This event advertises the main device that the server prefers to use
    /// when direct scan-out to the target device isn't possible. The
    /// advertised main device may be different for each
    /// wp_linux_dmabuf_feedback object, and may change over time.
    ///
    /// There is exactly one main device. The compositor must send at least
    /// one preference tranche with tranche_target_device equal to main_device.
    ///
    /// Clients need to create buffers that the main device can import and
    /// read from, otherwise creating the dmabuf wl_buffer will fail (see the
    /// wp_linux_buffer_params.create and create_immed requests for details).
    /// The main device will also likely be kept active by the compositor,
    /// so clients can use it instead of waking up another device for power
    /// savings.
    ///
    /// In general the device is a DRM node. The DRM node type (primary vs.
    /// render) is unspecified. Clients must not rely on the compositor sending
    /// a particular node type. Clients cannot check two devices for equality
    /// by comparing the dev_t value.
    ///
    /// If explicit modifiers are not supported and the client performs buffer
    /// allocations on a different device than the main device, then the client
    /// must force the buffer to have a linear layout.
    ///
    /// # Arguments
    ///
    /// - `device`: device dev_t value
    #[inline]
    fn handle_main_device(
        &mut self,
        slf: &Rc<ZwpLinuxDmabufFeedbackV1>,
        device: &[u8],
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_main_device(
            device,
        );
        if let Err(e) = res {
            log_forward("zwp_linux_dmabuf_feedback_v1.main_device", &e);
        }
    }

    /// a preference tranche has been sent
    ///
    /// This event splits tranche_target_device and tranche_formats events in
    /// preference tranches. It is sent after a set of tranche_target_device
    /// and tranche_formats events; it represents the end of a tranche. The
    /// next tranche will have a lower preference.
    #[inline]
    fn handle_tranche_done(
        &mut self,
        slf: &Rc<ZwpLinuxDmabufFeedbackV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_tranche_done(
        );
        if let Err(e) = res {
            log_forward("zwp_linux_dmabuf_feedback_v1.tranche_done", &e);
        }
    }

    /// target device
    ///
    /// This event advertises the target device that the server prefers to use
    /// for a buffer created given this tranche. The advertised target device
    /// may be different for each preference tranche, and may change over time.
    ///
    /// There is exactly one target device per tranche.
    ///
    /// The target device may be a scan-out device, for example if the
    /// compositor prefers to directly scan-out a buffer created given this
    /// tranche. The target device may be a rendering device, for example if
    /// the compositor prefers to texture from said buffer.
    ///
    /// The client can use this hint to allocate the buffer in a way that makes
    /// it accessible from the target device, ideally directly. The buffer must
    /// still be accessible from the main device, either through direct import
    /// or through a potentially more expensive fallback path. If the buffer
    /// can't be directly imported from the main device then clients must be
    /// prepared for the compositor changing the tranche priority or making
    /// wl_buffer creation fail (see the wp_linux_buffer_params.create and
    /// create_immed requests for details).
    ///
    /// If the device is a DRM node, the DRM node type (primary vs. render) is
    /// unspecified. Clients must not rely on the compositor sending a
    /// particular node type. Clients cannot check two devices for equality by
    /// comparing the dev_t value.
    ///
    /// This event is tied to a preference tranche, see the tranche_done event.
    ///
    /// # Arguments
    ///
    /// - `device`: device dev_t value
    #[inline]
    fn handle_tranche_target_device(
        &mut self,
        slf: &Rc<ZwpLinuxDmabufFeedbackV1>,
        device: &[u8],
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_tranche_target_device(
            device,
        );
        if let Err(e) = res {
            log_forward("zwp_linux_dmabuf_feedback_v1.tranche_target_device", &e);
        }
    }

    /// supported buffer format modifier
    ///
    /// This event advertises the format + modifier combinations that the
    /// compositor supports.
    ///
    /// It carries an array of indices, each referring to a format + modifier
    /// pair in the last received format table (see the format_table event).
    /// Each index is a 16-bit unsigned integer in native endianness.
    ///
    /// For legacy support, DRM_FORMAT_MOD_INVALID is an allowed modifier.
    /// It indicates that the server can support the format with an implicit
    /// modifier. When a buffer has DRM_FORMAT_MOD_INVALID as its modifier, it
    /// is as if no explicit modifier is specified. The effective modifier
    /// will be derived from the dmabuf.
    ///
    /// A compositor that sends valid modifiers and DRM_FORMAT_MOD_INVALID for
    /// a given format supports both explicit modifiers and implicit modifiers.
    ///
    /// Compositors must not send duplicate format + modifier pairs within the
    /// same tranche or across two different tranches with the same target
    /// device and flags.
    ///
    /// This event is tied to a preference tranche, see the tranche_done event.
    ///
    /// For the definition of the format and modifier codes, see the
    /// wp_linux_buffer_params.create request.
    ///
    /// # Arguments
    ///
    /// - `indices`: array of 16-bit indexes
    #[inline]
    fn handle_tranche_formats(
        &mut self,
        slf: &Rc<ZwpLinuxDmabufFeedbackV1>,
        indices: &[u8],
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_tranche_formats(
            indices,
        );
        if let Err(e) = res {
            log_forward("zwp_linux_dmabuf_feedback_v1.tranche_formats", &e);
        }
    }

    /// tranche flags
    ///
    /// This event sets tranche-specific flags.
    ///
    /// The scanout flag is a hint that direct scan-out may be attempted by the
    /// compositor on the target device if the client appropriately allocates a
    /// buffer. How to allocate a buffer that can be scanned out on the target
    /// device is implementation-defined.
    ///
    /// This event is tied to a preference tranche, see the tranche_done event.
    ///
    /// # Arguments
    ///
    /// - `flags`: tranche flags
    #[inline]
    fn handle_tranche_flags(
        &mut self,
        slf: &Rc<ZwpLinuxDmabufFeedbackV1>,
        flags: ZwpLinuxDmabufFeedbackV1TrancheFlags,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_tranche_flags(
            flags,
        );
        if let Err(e) = res {
            log_forward("zwp_linux_dmabuf_feedback_v1.tranche_flags", &e);
        }
    }
}

impl ObjectPrivate for ZwpLinuxDmabufFeedbackV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpLinuxDmabufFeedbackV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_linux_dmabuf_feedback_v1#{}.destroy()\n", client_id, id);
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
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_linux_dmabuf_feedback_v1#{}.done()\n", id);
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
            1 => {
                let [
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let Some(arg0) = fds.pop_front() else {
                    return Err(ObjectError(ObjectErrorKind::MissingFd("fd")));
                };
                let arg0 = &arg0;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_linux_dmabuf_feedback_v1#{}.format_table(fd: {}, size: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0.as_raw_fd(), arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_format_table(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_format_table(&self, arg0, arg1);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_array(msg, offset, "device")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_linux_dmabuf_feedback_v1#{}.main_device(device: {})\n", id, debug_array(arg0));
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_main_device(&self, arg0);
                } else {
                    DefaultHandler.handle_main_device(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_linux_dmabuf_feedback_v1#{}.tranche_done()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_tranche_done(&self);
                } else {
                    DefaultHandler.handle_tranche_done(&self);
                }
            }
            4 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_array(msg, offset, "device")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_linux_dmabuf_feedback_v1#{}.tranche_target_device(device: {})\n", id, debug_array(arg0));
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_tranche_target_device(&self, arg0);
                } else {
                    DefaultHandler.handle_tranche_target_device(&self, arg0);
                }
            }
            5 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_array(msg, offset, "indices")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_linux_dmabuf_feedback_v1#{}.tranche_formats(indices: {})\n", id, debug_array(arg0));
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_tranche_formats(&self, arg0);
                } else {
                    DefaultHandler.handle_tranche_formats(&self, arg0);
                }
            }
            6 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = ZwpLinuxDmabufFeedbackV1TrancheFlags(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: ZwpLinuxDmabufFeedbackV1TrancheFlags) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_linux_dmabuf_feedback_v1#{}.tranche_flags(flags: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_tranche_flags(&self, arg0);
                } else {
                    DefaultHandler.handle_tranche_flags(&self, arg0);
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
            0 => "done",
            1 => "format_table",
            2 => "main_device",
            3 => "tranche_done",
            4 => "tranche_target_device",
            5 => "tranche_formats",
            6 => "tranche_flags",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwpLinuxDmabufFeedbackV1 {
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

impl ZwpLinuxDmabufFeedbackV1 {
    /// Since when the tranche_flags.scanout enum variant is available.
    pub const ENM__TRANCHE_FLAGS_SCANOUT__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct ZwpLinuxDmabufFeedbackV1TrancheFlags(pub u32);

/// An iterator over the set bits in a [`ZwpLinuxDmabufFeedbackV1TrancheFlags`].
///
/// You can construct this with the `IntoIterator` implementation of `ZwpLinuxDmabufFeedbackV1TrancheFlags`.
#[derive(Clone, Debug)]
pub struct ZwpLinuxDmabufFeedbackV1TrancheFlagsIter(pub u32);

impl ZwpLinuxDmabufFeedbackV1TrancheFlags {
    /// direct scan-out tranche
    pub const SCANOUT: Self = Self(1);
}

impl ZwpLinuxDmabufFeedbackV1TrancheFlags {
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
        Self(0 | 1)
    }
}

impl Iterator for ZwpLinuxDmabufFeedbackV1TrancheFlagsIter {
    type Item = ZwpLinuxDmabufFeedbackV1TrancheFlags;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(ZwpLinuxDmabufFeedbackV1TrancheFlags(bit))
    }
}

impl IntoIterator for ZwpLinuxDmabufFeedbackV1TrancheFlags {
    type Item = ZwpLinuxDmabufFeedbackV1TrancheFlags;
    type IntoIter = ZwpLinuxDmabufFeedbackV1TrancheFlagsIter;

    fn into_iter(self) -> Self::IntoIter {
        ZwpLinuxDmabufFeedbackV1TrancheFlagsIter(self.0)
    }
}

impl BitAnd for ZwpLinuxDmabufFeedbackV1TrancheFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for ZwpLinuxDmabufFeedbackV1TrancheFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for ZwpLinuxDmabufFeedbackV1TrancheFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for ZwpLinuxDmabufFeedbackV1TrancheFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for ZwpLinuxDmabufFeedbackV1TrancheFlags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for ZwpLinuxDmabufFeedbackV1TrancheFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for ZwpLinuxDmabufFeedbackV1TrancheFlags {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for ZwpLinuxDmabufFeedbackV1TrancheFlags {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for ZwpLinuxDmabufFeedbackV1TrancheFlags {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for ZwpLinuxDmabufFeedbackV1TrancheFlags {
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
            f.write_str("SCANOUT")?;
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
