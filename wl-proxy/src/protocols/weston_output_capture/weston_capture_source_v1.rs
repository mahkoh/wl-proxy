//! image capturing source
//!
//! An object representing image capturing functionality for a single
//! source. When created, it sends the initial events if and only if the
//! output still exists and the specified pixel source is available on
//! the output.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A weston_capture_source_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WestonCaptureSourceV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WestonCaptureSourceV1Handler>,
}

struct DefaultHandler;

impl WestonCaptureSourceV1Handler for DefaultHandler { }

impl ConcreteObject for WestonCaptureSourceV1 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::WestonCaptureSourceV1;
    const INTERFACE_NAME: &str = "weston_capture_source_v1";
}

impl WestonCaptureSourceV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WestonCaptureSourceV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WestonCaptureSourceV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WestonCaptureSourceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WestonCaptureSourceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WestonCaptureSourceV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// cancel the capture, and destroy
    ///
    /// If a capture is on-going on this object, this will cancel it and
    /// make the image buffer contents undefined.
    ///
    /// This object is destroyed.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_capture_source_v1#{}.destroy()\n", id);
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

    /// cancel the capture, and destroy
    ///
    /// If a capture is on-going on this object, this will cancel it and
    /// make the image buffer contents undefined.
    ///
    /// This object is destroyed.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("weston_capture_source_v1.destroy", &e);
        }
    }

    /// Since when the capture message is available.
    pub const MSG__CAPTURE__SINCE: u32 = 1;

    /// capture an image
    ///
    /// If the given wl_buffer is compatible, the associated output will go
    /// through a repaint some time after this request has been processed,
    /// and that repaint will execute the capture.
    /// Once the capture is complete, 'complete' event is emitted.
    ///
    /// If the given wl_buffer is incompatible, the event 'retry' is
    /// emitted.
    ///
    /// If the capture fails or the buffer type is unsupported, the event
    /// 'failed' is emitted.
    ///
    /// The client must wait for one of these events before attempting
    /// 'capture' on this object again. If 'capture' is requested again before
    /// any of those events, 'sequence' protocol error is raised.
    ///
    /// The wl_buffer object will not emit wl_buffer.release event due to
    /// this request.
    ///
    /// The wl_buffer must refer to compositor-writable storage. If buffer
    /// storage is not writable, either the protocol error bad_buffer or
    /// wl_shm.error.invalid_fd is raised.
    ///
    /// If the wl_buffer is destroyed before any event is emitted, the buffer
    /// contents become undefined.
    ///
    /// A compositor is required to implement capture into wl_shm buffers.
    /// Other buffer types may or may not be supported.
    ///
    /// # Arguments
    ///
    /// - `buffer`: a writable image buffer
    #[inline]
    pub fn try_send_capture(
        &self,
        buffer: &Rc<WlBuffer>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            buffer,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("buffer"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_capture_source_v1#{}.capture(buffer: wl_buffer#{})\n", id, arg0);
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

    /// capture an image
    ///
    /// If the given wl_buffer is compatible, the associated output will go
    /// through a repaint some time after this request has been processed,
    /// and that repaint will execute the capture.
    /// Once the capture is complete, 'complete' event is emitted.
    ///
    /// If the given wl_buffer is incompatible, the event 'retry' is
    /// emitted.
    ///
    /// If the capture fails or the buffer type is unsupported, the event
    /// 'failed' is emitted.
    ///
    /// The client must wait for one of these events before attempting
    /// 'capture' on this object again. If 'capture' is requested again before
    /// any of those events, 'sequence' protocol error is raised.
    ///
    /// The wl_buffer object will not emit wl_buffer.release event due to
    /// this request.
    ///
    /// The wl_buffer must refer to compositor-writable storage. If buffer
    /// storage is not writable, either the protocol error bad_buffer or
    /// wl_shm.error.invalid_fd is raised.
    ///
    /// If the wl_buffer is destroyed before any event is emitted, the buffer
    /// contents become undefined.
    ///
    /// A compositor is required to implement capture into wl_shm buffers.
    /// Other buffer types may or may not be supported.
    ///
    /// # Arguments
    ///
    /// - `buffer`: a writable image buffer
    #[inline]
    pub fn send_capture(
        &self,
        buffer: &Rc<WlBuffer>,
    ) {
        let res = self.try_send_capture(
            buffer,
        );
        if let Err(e) = res {
            log_send("weston_capture_source_v1.capture", &e);
        }
    }

    /// Since when the format message is available.
    pub const MSG__FORMAT__SINCE: u32 = 1;

    /// pixel format for a buffer
    ///
    /// This event delivers one pixel format that can be used for the
    /// image buffer. Any buffer is incompatible if it does not have
    /// a pixel format delivered by one of this events.
    ///
    /// The format modifier is linear (DRM_FORMAT_MOD_LINEAR).
    ///
    /// This is an initial event, and sent whenever the supported formats
    /// change.
    ///
    /// This event may be send multiple times, followed by a format_done event.
    ///
    /// # Arguments
    ///
    /// - `drm_format`: DRM pixel format code
    #[inline]
    pub fn try_send_format(
        &self,
        drm_format: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            drm_format,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_capture_source_v1#{}.format(drm_format: {})\n", client_id, id, arg0);
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
            arg0,
        ]);
        Ok(())
    }

    /// pixel format for a buffer
    ///
    /// This event delivers one pixel format that can be used for the
    /// image buffer. Any buffer is incompatible if it does not have
    /// a pixel format delivered by one of this events.
    ///
    /// The format modifier is linear (DRM_FORMAT_MOD_LINEAR).
    ///
    /// This is an initial event, and sent whenever the supported formats
    /// change.
    ///
    /// This event may be send multiple times, followed by a format_done event.
    ///
    /// # Arguments
    ///
    /// - `drm_format`: DRM pixel format code
    #[inline]
    pub fn send_format(
        &self,
        drm_format: u32,
    ) {
        let res = self.try_send_format(
            drm_format,
        );
        if let Err(e) = res {
            log_send("weston_capture_source_v1.format", &e);
        }
    }

    /// Since when the size message is available.
    pub const MSG__SIZE__SINCE: u32 = 1;

    /// dimensions for a buffer
    ///
    /// This event delivers the size that should be used for the
    /// image buffer. Any buffer is incompatible if it does not have
    /// this size.
    ///
    /// For wl_shm the row alignment of the buffer must be 4 bytes, and it must
    /// not contain further row padding. Otherwise the buffer is unsupported.
    ///
    /// This is an initial event, and sent whenever the required size
    /// changes.
    ///
    /// # Arguments
    ///
    /// - `width`: width in pixels
    /// - `height`: height in pixels
    #[inline]
    pub fn try_send_size(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_capture_source_v1#{}.size(width: {}, height: {})\n", client_id, id, arg0, arg1);
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

    /// dimensions for a buffer
    ///
    /// This event delivers the size that should be used for the
    /// image buffer. Any buffer is incompatible if it does not have
    /// this size.
    ///
    /// For wl_shm the row alignment of the buffer must be 4 bytes, and it must
    /// not contain further row padding. Otherwise the buffer is unsupported.
    ///
    /// This is an initial event, and sent whenever the required size
    /// changes.
    ///
    /// # Arguments
    ///
    /// - `width`: width in pixels
    /// - `height`: height in pixels
    #[inline]
    pub fn send_size(
        &self,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_size(
            width,
            height,
        );
        if let Err(e) = res {
            log_send("weston_capture_source_v1.size", &e);
        }
    }

    /// Since when the complete message is available.
    pub const MSG__COMPLETE__SINCE: u32 = 1;

    /// capture has completed
    ///
    /// This event is emitted as a response to 'capture' request when it
    /// has successfully completed.
    ///
    /// If the buffer used in the shot is a dmabuf, the client also needs to
    /// wait for any implicit fences on it before accessing the contents.
    #[inline]
    pub fn try_send_complete(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_capture_source_v1#{}.complete()\n", client_id, id);
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

    /// capture has completed
    ///
    /// This event is emitted as a response to 'capture' request when it
    /// has successfully completed.
    ///
    /// If the buffer used in the shot is a dmabuf, the client also needs to
    /// wait for any implicit fences on it before accessing the contents.
    #[inline]
    pub fn send_complete(
        &self,
    ) {
        let res = self.try_send_complete(
        );
        if let Err(e) = res {
            log_send("weston_capture_source_v1.complete", &e);
        }
    }

    /// Since when the retry message is available.
    pub const MSG__RETRY__SINCE: u32 = 1;

    /// retry image capture with a different buffer
    ///
    /// This event is emitted as a response to 'capture' request when it
    /// cannot succeed due to an incompatible buffer. The client has already
    /// received the events delivering the new buffer parameters. The client
    /// should retry the capture with the new buffer parameters.
    #[inline]
    pub fn try_send_retry(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_capture_source_v1#{}.retry()\n", client_id, id);
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

    /// retry image capture with a different buffer
    ///
    /// This event is emitted as a response to 'capture' request when it
    /// cannot succeed due to an incompatible buffer. The client has already
    /// received the events delivering the new buffer parameters. The client
    /// should retry the capture with the new buffer parameters.
    #[inline]
    pub fn send_retry(
        &self,
    ) {
        let res = self.try_send_retry(
        );
        if let Err(e) = res {
            log_send("weston_capture_source_v1.retry", &e);
        }
    }

    /// Since when the failed message is available.
    pub const MSG__FAILED__SINCE: u32 = 1;

    /// capture failed
    ///
    /// This event is emitted as a response to 'capture' request when it
    /// has failed for reasons other than an incompatible buffer. The reasons
    /// may include: unsupported buffer type, unsupported buffer stride,
    /// unsupported image source, the image source (output) was removed, or
    /// compositor policy denied the capture.
    ///
    /// The string 'msg' may contain a human-readable explanation of the
    /// failure to aid debugging.
    ///
    /// # Arguments
    ///
    /// - `msg`: human-readable hint
    #[inline]
    pub fn try_send_failed(
        &self,
        msg: Option<&str>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            msg,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: Option<&str>) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_capture_source_v1#{}.failed(msg: {:?})\n", client_id, id, arg0);
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
        if let Some(arg0) = arg0 {
            fmt.string(arg0);
        } else {
            fmt.words([0]);
        }
        Ok(())
    }

    /// capture failed
    ///
    /// This event is emitted as a response to 'capture' request when it
    /// has failed for reasons other than an incompatible buffer. The reasons
    /// may include: unsupported buffer type, unsupported buffer stride,
    /// unsupported image source, the image source (output) was removed, or
    /// compositor policy denied the capture.
    ///
    /// The string 'msg' may contain a human-readable explanation of the
    /// failure to aid debugging.
    ///
    /// # Arguments
    ///
    /// - `msg`: human-readable hint
    #[inline]
    pub fn send_failed(
        &self,
        msg: Option<&str>,
    ) {
        let res = self.try_send_failed(
            msg,
        );
        if let Err(e) = res {
            log_send("weston_capture_source_v1.failed", &e);
        }
    }

    /// Since when the formats_done message is available.
    pub const MSG__FORMATS_DONE__SINCE: u32 = 2;

    /// sending formats is complete
    ///
    /// This event is sent after all formats have been sent.
    #[inline]
    pub fn try_send_formats_done(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_capture_source_v1#{}.formats_done()\n", client_id, id);
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

    /// sending formats is complete
    ///
    /// This event is sent after all formats have been sent.
    #[inline]
    pub fn send_formats_done(
        &self,
    ) {
        let res = self.try_send_formats_done(
        );
        if let Err(e) = res {
            log_send("weston_capture_source_v1.formats_done", &e);
        }
    }
}

/// A message handler for [`WestonCaptureSourceV1`] proxies.
pub trait WestonCaptureSourceV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WestonCaptureSourceV1>) {
        slf.core.delete_id();
    }

    /// cancel the capture, and destroy
    ///
    /// If a capture is on-going on this object, this will cancel it and
    /// make the image buffer contents undefined.
    ///
    /// This object is destroyed.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WestonCaptureSourceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("weston_capture_source_v1.destroy", &e);
        }
    }

    /// capture an image
    ///
    /// If the given wl_buffer is compatible, the associated output will go
    /// through a repaint some time after this request has been processed,
    /// and that repaint will execute the capture.
    /// Once the capture is complete, 'complete' event is emitted.
    ///
    /// If the given wl_buffer is incompatible, the event 'retry' is
    /// emitted.
    ///
    /// If the capture fails or the buffer type is unsupported, the event
    /// 'failed' is emitted.
    ///
    /// The client must wait for one of these events before attempting
    /// 'capture' on this object again. If 'capture' is requested again before
    /// any of those events, 'sequence' protocol error is raised.
    ///
    /// The wl_buffer object will not emit wl_buffer.release event due to
    /// this request.
    ///
    /// The wl_buffer must refer to compositor-writable storage. If buffer
    /// storage is not writable, either the protocol error bad_buffer or
    /// wl_shm.error.invalid_fd is raised.
    ///
    /// If the wl_buffer is destroyed before any event is emitted, the buffer
    /// contents become undefined.
    ///
    /// A compositor is required to implement capture into wl_shm buffers.
    /// Other buffer types may or may not be supported.
    ///
    /// # Arguments
    ///
    /// - `buffer`: a writable image buffer
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_capture(
        &mut self,
        slf: &Rc<WestonCaptureSourceV1>,
        buffer: &Rc<WlBuffer>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_capture(
            buffer,
        );
        if let Err(e) = res {
            log_forward("weston_capture_source_v1.capture", &e);
        }
    }

    /// pixel format for a buffer
    ///
    /// This event delivers one pixel format that can be used for the
    /// image buffer. Any buffer is incompatible if it does not have
    /// a pixel format delivered by one of this events.
    ///
    /// The format modifier is linear (DRM_FORMAT_MOD_LINEAR).
    ///
    /// This is an initial event, and sent whenever the supported formats
    /// change.
    ///
    /// This event may be send multiple times, followed by a format_done event.
    ///
    /// # Arguments
    ///
    /// - `drm_format`: DRM pixel format code
    #[inline]
    fn handle_format(
        &mut self,
        slf: &Rc<WestonCaptureSourceV1>,
        drm_format: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_format(
            drm_format,
        );
        if let Err(e) = res {
            log_forward("weston_capture_source_v1.format", &e);
        }
    }

    /// dimensions for a buffer
    ///
    /// This event delivers the size that should be used for the
    /// image buffer. Any buffer is incompatible if it does not have
    /// this size.
    ///
    /// For wl_shm the row alignment of the buffer must be 4 bytes, and it must
    /// not contain further row padding. Otherwise the buffer is unsupported.
    ///
    /// This is an initial event, and sent whenever the required size
    /// changes.
    ///
    /// # Arguments
    ///
    /// - `width`: width in pixels
    /// - `height`: height in pixels
    #[inline]
    fn handle_size(
        &mut self,
        slf: &Rc<WestonCaptureSourceV1>,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_size(
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("weston_capture_source_v1.size", &e);
        }
    }

    /// capture has completed
    ///
    /// This event is emitted as a response to 'capture' request when it
    /// has successfully completed.
    ///
    /// If the buffer used in the shot is a dmabuf, the client also needs to
    /// wait for any implicit fences on it before accessing the contents.
    #[inline]
    fn handle_complete(
        &mut self,
        slf: &Rc<WestonCaptureSourceV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_complete(
        );
        if let Err(e) = res {
            log_forward("weston_capture_source_v1.complete", &e);
        }
    }

    /// retry image capture with a different buffer
    ///
    /// This event is emitted as a response to 'capture' request when it
    /// cannot succeed due to an incompatible buffer. The client has already
    /// received the events delivering the new buffer parameters. The client
    /// should retry the capture with the new buffer parameters.
    #[inline]
    fn handle_retry(
        &mut self,
        slf: &Rc<WestonCaptureSourceV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_retry(
        );
        if let Err(e) = res {
            log_forward("weston_capture_source_v1.retry", &e);
        }
    }

    /// capture failed
    ///
    /// This event is emitted as a response to 'capture' request when it
    /// has failed for reasons other than an incompatible buffer. The reasons
    /// may include: unsupported buffer type, unsupported buffer stride,
    /// unsupported image source, the image source (output) was removed, or
    /// compositor policy denied the capture.
    ///
    /// The string 'msg' may contain a human-readable explanation of the
    /// failure to aid debugging.
    ///
    /// # Arguments
    ///
    /// - `msg`: human-readable hint
    #[inline]
    fn handle_failed(
        &mut self,
        slf: &Rc<WestonCaptureSourceV1>,
        msg: Option<&str>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_failed(
            msg,
        );
        if let Err(e) = res {
            log_forward("weston_capture_source_v1.failed", &e);
        }
    }

    /// sending formats is complete
    ///
    /// This event is sent after all formats have been sent.
    #[inline]
    fn handle_formats_done(
        &mut self,
        slf: &Rc<WestonCaptureSourceV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_formats_done(
        );
        if let Err(e) = res {
            log_forward("weston_capture_source_v1.formats_done", &e);
        }
    }
}

impl ObjectPrivate for WestonCaptureSourceV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WestonCaptureSourceV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_capture_source_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_capture_source_v1#{}.capture(buffer: wl_buffer#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlBuffer>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("buffer", o.core().interface, ObjectInterface::WlBuffer)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_capture(&self, arg0);
                } else {
                    DefaultHandler.handle_capture(&self, arg0);
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_capture_source_v1#{}.format(drm_format: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_format(&self, arg0);
                } else {
                    DefaultHandler.handle_format(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_capture_source_v1#{}.size(width: {}, height: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_size(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_size(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_capture_source_v1#{}.complete()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_complete(&self);
                } else {
                    DefaultHandler.handle_complete(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_capture_source_v1#{}.retry()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_retry(&self);
                } else {
                    DefaultHandler.handle_retry(&self);
                }
            }
            4 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NullableString>(msg, offset, "msg")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: Option<&str>) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_capture_source_v1#{}.failed(msg: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_failed(&self, arg0);
                } else {
                    DefaultHandler.handle_failed(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_capture_source_v1#{}.formats_done()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_formats_done(&self);
                } else {
                    DefaultHandler.handle_formats_done(&self);
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
            1 => "capture",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "format",
            1 => "size",
            2 => "complete",
            3 => "retry",
            4 => "failed",
            5 => "formats_done",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WestonCaptureSourceV1 {
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

impl WestonCaptureSourceV1 {
    /// Since when the error.bad_buffer enum variant is available.
    pub const ENM__ERROR_BAD_BUFFER__SINCE: u32 = 1;
    /// Since when the error.sequence enum variant is available.
    pub const ENM__ERROR_SEQUENCE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WestonCaptureSourceV1Error(pub u32);

impl WestonCaptureSourceV1Error {
    /// the wl_buffer is not writable
    pub const BAD_BUFFER: Self = Self(0);

    /// capture requested again before previous retired
    pub const SEQUENCE: Self = Self(1);
}

impl Debug for WestonCaptureSourceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::BAD_BUFFER => "BAD_BUFFER",
            Self::SEQUENCE => "SEQUENCE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
