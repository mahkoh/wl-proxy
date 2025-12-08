//! image copy capture session
//!
//! This object represents an active image copy capture session.
//!
//! After a capture session is created, buffer constraint events will be
//! emitted from the compositor to tell the client which buffer types and
//! formats are supported for reading from the session. The compositor may
//! re-send buffer constraint events whenever they change.
//!
//! To advertise buffer constraints, the compositor must send in no
//! particular order: zero or more shm_format and dmabuf_format events, zero
//! or one dmabuf_device event, and exactly one buffer_size event. Then the
//! compositor must send a done event.
//!
//! When the client has received all the buffer constraints, it can create a
//! buffer accordingly, attach it to the capture session using the
//! attach_buffer request, set the buffer damage using the damage_buffer
//! request and then send the capture request.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A ext_image_copy_capture_session_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ExtImageCopyCaptureSessionV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ExtImageCopyCaptureSessionV1Handler>,
}

struct DefaultHandler;

impl ExtImageCopyCaptureSessionV1Handler for DefaultHandler { }

impl ConcreteObject for ExtImageCopyCaptureSessionV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ExtImageCopyCaptureSessionV1;
    const INTERFACE_NAME: &str = "ext_image_copy_capture_session_v1";
}

impl ExtImageCopyCaptureSessionV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ExtImageCopyCaptureSessionV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ExtImageCopyCaptureSessionV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ExtImageCopyCaptureSessionV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtImageCopyCaptureSessionV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ExtImageCopyCaptureSessionV1 {
    /// Since when the buffer_size message is available.
    pub const MSG__BUFFER_SIZE__SINCE: u32 = 1;

    /// image capture source dimensions
    ///
    /// Provides the dimensions of the source image in buffer pixel coordinates.
    ///
    /// The client must attach buffers that match this size.
    ///
    /// # Arguments
    ///
    /// - `width`: buffer width
    /// - `height`: buffer height
    #[inline]
    pub fn try_send_buffer_size(
        &self,
        width: u32,
        height: u32,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_image_copy_capture_session_v1#{}.buffer_size(width: {}, height: {})\n", client_id, id, arg0, arg1);
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
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// image capture source dimensions
    ///
    /// Provides the dimensions of the source image in buffer pixel coordinates.
    ///
    /// The client must attach buffers that match this size.
    ///
    /// # Arguments
    ///
    /// - `width`: buffer width
    /// - `height`: buffer height
    #[inline]
    pub fn send_buffer_size(
        &self,
        width: u32,
        height: u32,
    ) {
        let res = self.try_send_buffer_size(
            width,
            height,
        );
        if let Err(e) = res {
            log_send("ext_image_copy_capture_session_v1.buffer_size", &e);
        }
    }

    /// Since when the shm_format message is available.
    pub const MSG__SHM_FORMAT__SINCE: u32 = 1;

    /// shm buffer format
    ///
    /// Provides the format that must be used for shared-memory buffers.
    ///
    /// This event may be emitted multiple times, in which case the client may
    /// choose any given format.
    ///
    /// # Arguments
    ///
    /// - `format`: shm format
    #[inline]
    pub fn try_send_shm_format(
        &self,
        format: WlShmFormat,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            format,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: WlShmFormat) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_image_copy_capture_session_v1#{}.shm_format(format: {:?})\n", client_id, id, arg0);
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

    /// shm buffer format
    ///
    /// Provides the format that must be used for shared-memory buffers.
    ///
    /// This event may be emitted multiple times, in which case the client may
    /// choose any given format.
    ///
    /// # Arguments
    ///
    /// - `format`: shm format
    #[inline]
    pub fn send_shm_format(
        &self,
        format: WlShmFormat,
    ) {
        let res = self.try_send_shm_format(
            format,
        );
        if let Err(e) = res {
            log_send("ext_image_copy_capture_session_v1.shm_format", &e);
        }
    }

    /// Since when the dmabuf_device message is available.
    pub const MSG__DMABUF_DEVICE__SINCE: u32 = 1;

    /// dma-buf device
    ///
    /// This event advertises the device buffers must be allocated on for
    /// dma-buf buffers.
    ///
    /// In general the device is a DRM node. The DRM node type (primary vs.
    /// render) is unspecified. Clients must not rely on the compositor sending
    /// a particular node type. Clients cannot check two devices for equality
    /// by comparing the dev_t value.
    ///
    /// # Arguments
    ///
    /// - `device`: device dev_t value
    #[inline]
    pub fn try_send_dmabuf_device(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_image_copy_capture_session_v1#{}.dmabuf_device(device: {})\n", client_id, id, debug_array(arg0));
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

    /// dma-buf device
    ///
    /// This event advertises the device buffers must be allocated on for
    /// dma-buf buffers.
    ///
    /// In general the device is a DRM node. The DRM node type (primary vs.
    /// render) is unspecified. Clients must not rely on the compositor sending
    /// a particular node type. Clients cannot check two devices for equality
    /// by comparing the dev_t value.
    ///
    /// # Arguments
    ///
    /// - `device`: device dev_t value
    #[inline]
    pub fn send_dmabuf_device(
        &self,
        device: &[u8],
    ) {
        let res = self.try_send_dmabuf_device(
            device,
        );
        if let Err(e) = res {
            log_send("ext_image_copy_capture_session_v1.dmabuf_device", &e);
        }
    }

    /// Since when the dmabuf_format message is available.
    pub const MSG__DMABUF_FORMAT__SINCE: u32 = 1;

    /// dma-buf format
    ///
    /// Provides the format that must be used for dma-buf buffers.
    ///
    /// The client may choose any of the modifiers advertised in the array of
    /// 64-bit unsigned integers.
    ///
    /// This event may be emitted multiple times, in which case the client may
    /// choose any given format.
    ///
    /// # Arguments
    ///
    /// - `format`: drm format code
    /// - `modifiers`: drm format modifiers
    #[inline]
    pub fn try_send_dmabuf_format(
        &self,
        format: u32,
        modifiers: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            format,
            modifiers,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: &[u8]) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_image_copy_capture_session_v1#{}.dmabuf_format(format: {}, modifiers: {})\n", client_id, id, arg0, debug_array(arg1));
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
            3,
            arg0,
        ]);
        fmt.array(arg1);
        Ok(())
    }

    /// dma-buf format
    ///
    /// Provides the format that must be used for dma-buf buffers.
    ///
    /// The client may choose any of the modifiers advertised in the array of
    /// 64-bit unsigned integers.
    ///
    /// This event may be emitted multiple times, in which case the client may
    /// choose any given format.
    ///
    /// # Arguments
    ///
    /// - `format`: drm format code
    /// - `modifiers`: drm format modifiers
    #[inline]
    pub fn send_dmabuf_format(
        &self,
        format: u32,
        modifiers: &[u8],
    ) {
        let res = self.try_send_dmabuf_format(
            format,
            modifiers,
        );
        if let Err(e) = res {
            log_send("ext_image_copy_capture_session_v1.dmabuf_format", &e);
        }
    }

    /// Since when the done message is available.
    pub const MSG__DONE__SINCE: u32 = 1;

    /// all constraints have been sent
    ///
    /// This event is sent once when all buffer constraint events have been
    /// sent.
    ///
    /// The compositor must always end a batch of buffer constraint events with
    /// this event, regardless of whether it sends the initial constraints or
    /// an update.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_image_copy_capture_session_v1#{}.done()\n", client_id, id);
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

    /// all constraints have been sent
    ///
    /// This event is sent once when all buffer constraint events have been
    /// sent.
    ///
    /// The compositor must always end a batch of buffer constraint events with
    /// this event, regardless of whether it sends the initial constraints or
    /// an update.
    #[inline]
    pub fn send_done(
        &self,
    ) {
        let res = self.try_send_done(
        );
        if let Err(e) = res {
            log_send("ext_image_copy_capture_session_v1.done", &e);
        }
    }

    /// Since when the stopped message is available.
    pub const MSG__STOPPED__SINCE: u32 = 1;

    /// session is no longer available
    ///
    /// This event indicates that the capture session has stopped and is no
    /// longer available. This can happen in a number of cases, e.g. when the
    /// underlying source is destroyed, if the user decides to end the image
    /// capture, or if an unrecoverable runtime error has occurred.
    ///
    /// The client should destroy the session after receiving this event.
    #[inline]
    pub fn try_send_stopped(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_image_copy_capture_session_v1#{}.stopped()\n", client_id, id);
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

    /// session is no longer available
    ///
    /// This event indicates that the capture session has stopped and is no
    /// longer available. This can happen in a number of cases, e.g. when the
    /// underlying source is destroyed, if the user decides to end the image
    /// capture, or if an unrecoverable runtime error has occurred.
    ///
    /// The client should destroy the session after receiving this event.
    #[inline]
    pub fn send_stopped(
        &self,
    ) {
        let res = self.try_send_stopped(
        );
        if let Err(e) = res {
            log_send("ext_image_copy_capture_session_v1.stopped", &e);
        }
    }

    /// Since when the create_frame message is available.
    pub const MSG__CREATE_FRAME__SINCE: u32 = 1;

    /// create a frame
    ///
    /// Create a capture frame for this session.
    ///
    /// At most one frame object can exist for a given session at any time. If
    /// a client sends a create_frame request before a previous frame object
    /// has been destroyed, the duplicate_frame protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `frame`:
    #[inline]
    pub fn try_send_create_frame(
        &self,
        frame: &Rc<ExtImageCopyCaptureFrameV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            frame,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("frame", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_image_copy_capture_session_v1#{}.create_frame(frame: ext_image_copy_capture_frame_v1#{})\n", id, arg0);
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
            0,
            arg0_id,
        ]);
        Ok(())
    }

    /// create a frame
    ///
    /// Create a capture frame for this session.
    ///
    /// At most one frame object can exist for a given session at any time. If
    /// a client sends a create_frame request before a previous frame object
    /// has been destroyed, the duplicate_frame protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `frame`:
    #[inline]
    pub fn send_create_frame(
        &self,
        frame: &Rc<ExtImageCopyCaptureFrameV1>,
    ) {
        let res = self.try_send_create_frame(
            frame,
        );
        if let Err(e) = res {
            log_send("ext_image_copy_capture_session_v1.create_frame", &e);
        }
    }

    /// create a frame
    ///
    /// Create a capture frame for this session.
    ///
    /// At most one frame object can exist for a given session at any time. If
    /// a client sends a create_frame request before a previous frame object
    /// has been destroyed, the duplicate_frame protocol error is raised.
    #[inline]
    pub fn new_try_send_create_frame(
        &self,
    ) -> Result<Rc<ExtImageCopyCaptureFrameV1>, ObjectError> {
        let frame = self.core.create_child();
        self.try_send_create_frame(
            &frame,
        )?;
        Ok(frame)
    }

    /// create a frame
    ///
    /// Create a capture frame for this session.
    ///
    /// At most one frame object can exist for a given session at any time. If
    /// a client sends a create_frame request before a previous frame object
    /// has been destroyed, the duplicate_frame protocol error is raised.
    #[inline]
    pub fn new_send_create_frame(
        &self,
    ) -> Rc<ExtImageCopyCaptureFrameV1> {
        let frame = self.core.create_child();
        self.send_create_frame(
            &frame,
        );
        frame
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// delete this object
    ///
    /// Destroys the session. This request can be sent at any time by the
    /// client.
    ///
    /// This request doesn't affect ext_image_copy_capture_frame_v1 objects created by
    /// this object.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_image_copy_capture_session_v1#{}.destroy()\n", id);
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
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// delete this object
    ///
    /// Destroys the session. This request can be sent at any time by the
    /// client.
    ///
    /// This request doesn't affect ext_image_copy_capture_frame_v1 objects created by
    /// this object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("ext_image_copy_capture_session_v1.destroy", &e);
        }
    }
}

/// A message handler for [`ExtImageCopyCaptureSessionV1`] proxies.
pub trait ExtImageCopyCaptureSessionV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ExtImageCopyCaptureSessionV1>) {
        slf.core.delete_id();
    }

    /// image capture source dimensions
    ///
    /// Provides the dimensions of the source image in buffer pixel coordinates.
    ///
    /// The client must attach buffers that match this size.
    ///
    /// # Arguments
    ///
    /// - `width`: buffer width
    /// - `height`: buffer height
    #[inline]
    fn handle_buffer_size(
        &mut self,
        slf: &Rc<ExtImageCopyCaptureSessionV1>,
        width: u32,
        height: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_buffer_size(
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("ext_image_copy_capture_session_v1.buffer_size", &e);
        }
    }

    /// shm buffer format
    ///
    /// Provides the format that must be used for shared-memory buffers.
    ///
    /// This event may be emitted multiple times, in which case the client may
    /// choose any given format.
    ///
    /// # Arguments
    ///
    /// - `format`: shm format
    #[inline]
    fn handle_shm_format(
        &mut self,
        slf: &Rc<ExtImageCopyCaptureSessionV1>,
        format: WlShmFormat,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_shm_format(
            format,
        );
        if let Err(e) = res {
            log_forward("ext_image_copy_capture_session_v1.shm_format", &e);
        }
    }

    /// dma-buf device
    ///
    /// This event advertises the device buffers must be allocated on for
    /// dma-buf buffers.
    ///
    /// In general the device is a DRM node. The DRM node type (primary vs.
    /// render) is unspecified. Clients must not rely on the compositor sending
    /// a particular node type. Clients cannot check two devices for equality
    /// by comparing the dev_t value.
    ///
    /// # Arguments
    ///
    /// - `device`: device dev_t value
    #[inline]
    fn handle_dmabuf_device(
        &mut self,
        slf: &Rc<ExtImageCopyCaptureSessionV1>,
        device: &[u8],
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_dmabuf_device(
            device,
        );
        if let Err(e) = res {
            log_forward("ext_image_copy_capture_session_v1.dmabuf_device", &e);
        }
    }

    /// dma-buf format
    ///
    /// Provides the format that must be used for dma-buf buffers.
    ///
    /// The client may choose any of the modifiers advertised in the array of
    /// 64-bit unsigned integers.
    ///
    /// This event may be emitted multiple times, in which case the client may
    /// choose any given format.
    ///
    /// # Arguments
    ///
    /// - `format`: drm format code
    /// - `modifiers`: drm format modifiers
    #[inline]
    fn handle_dmabuf_format(
        &mut self,
        slf: &Rc<ExtImageCopyCaptureSessionV1>,
        format: u32,
        modifiers: &[u8],
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_dmabuf_format(
            format,
            modifiers,
        );
        if let Err(e) = res {
            log_forward("ext_image_copy_capture_session_v1.dmabuf_format", &e);
        }
    }

    /// all constraints have been sent
    ///
    /// This event is sent once when all buffer constraint events have been
    /// sent.
    ///
    /// The compositor must always end a batch of buffer constraint events with
    /// this event, regardless of whether it sends the initial constraints or
    /// an update.
    #[inline]
    fn handle_done(
        &mut self,
        slf: &Rc<ExtImageCopyCaptureSessionV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_done(
        );
        if let Err(e) = res {
            log_forward("ext_image_copy_capture_session_v1.done", &e);
        }
    }

    /// session is no longer available
    ///
    /// This event indicates that the capture session has stopped and is no
    /// longer available. This can happen in a number of cases, e.g. when the
    /// underlying source is destroyed, if the user decides to end the image
    /// capture, or if an unrecoverable runtime error has occurred.
    ///
    /// The client should destroy the session after receiving this event.
    #[inline]
    fn handle_stopped(
        &mut self,
        slf: &Rc<ExtImageCopyCaptureSessionV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_stopped(
        );
        if let Err(e) = res {
            log_forward("ext_image_copy_capture_session_v1.stopped", &e);
        }
    }

    /// create a frame
    ///
    /// Create a capture frame for this session.
    ///
    /// At most one frame object can exist for a given session at any time. If
    /// a client sends a create_frame request before a previous frame object
    /// has been destroyed, the duplicate_frame protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `frame`:
    #[inline]
    fn handle_create_frame(
        &mut self,
        slf: &Rc<ExtImageCopyCaptureSessionV1>,
        frame: &Rc<ExtImageCopyCaptureFrameV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_frame(
            frame,
        );
        if let Err(e) = res {
            log_forward("ext_image_copy_capture_session_v1.create_frame", &e);
        }
    }

    /// delete this object
    ///
    /// Destroys the session. This request can be sent at any time by the
    /// client.
    ///
    /// This request doesn't affect ext_image_copy_capture_frame_v1 objects created by
    /// this object.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ExtImageCopyCaptureSessionV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("ext_image_copy_capture_session_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for ExtImageCopyCaptureSessionV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ExtImageCopyCaptureSessionV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_image_copy_capture_session_v1#{}.create_frame(frame: ext_image_copy_capture_frame_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = ExtImageCopyCaptureFrameV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "frame", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_frame(&self, arg0);
                } else {
                    DefaultHandler.handle_create_frame(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_image_copy_capture_session_v1#{}.destroy()\n", client_id, id);
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
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_image_copy_capture_session_v1#{}.buffer_size(width: {}, height: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_buffer_size(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_buffer_size(&self, arg0, arg1);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = WlShmFormat(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: WlShmFormat) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_image_copy_capture_session_v1#{}.shm_format(format: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_shm_format(&self, arg0);
                } else {
                    DefaultHandler.handle_shm_format(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_image_copy_capture_session_v1#{}.dmabuf_device(device: {})\n", id, debug_array(arg0));
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_dmabuf_device(&self, arg0);
                } else {
                    DefaultHandler.handle_dmabuf_device(&self, arg0);
                }
            }
            3 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("format")));
                };
                offset += 1;
                let arg1;
                (arg1, offset) = parse_array(msg, offset, "modifiers")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_image_copy_capture_session_v1#{}.dmabuf_format(format: {}, modifiers: {})\n", id, arg0, debug_array(arg1));
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_dmabuf_format(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_dmabuf_format(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_image_copy_capture_session_v1#{}.done()\n", id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_image_copy_capture_session_v1#{}.stopped()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_stopped(&self);
                } else {
                    DefaultHandler.handle_stopped(&self);
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
            0 => "create_frame",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "buffer_size",
            1 => "shm_format",
            2 => "dmabuf_device",
            3 => "dmabuf_format",
            4 => "done",
            5 => "stopped",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ExtImageCopyCaptureSessionV1 {
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

impl ExtImageCopyCaptureSessionV1 {
    /// Since when the error.duplicate_frame enum variant is available.
    pub const ENM__ERROR_DUPLICATE_FRAME__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtImageCopyCaptureSessionV1Error(pub u32);

impl ExtImageCopyCaptureSessionV1Error {
    /// create_frame sent before destroying previous frame
    pub const DUPLICATE_FRAME: Self = Self(1);
}

impl Debug for ExtImageCopyCaptureSessionV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DUPLICATE_FRAME => "DUPLICATE_FRAME",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
