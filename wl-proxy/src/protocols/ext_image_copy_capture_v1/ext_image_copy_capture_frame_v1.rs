//! image capture frame
//!
//! This object represents an image capture frame.
//!
//! The client should attach a buffer, damage the buffer, and then send a
//! capture request.
//!
//! If the capture is successful, the compositor must send the frame metadata
//! (transform, damage, presentation_time in any order) followed by the ready
//! event.
//!
//! If the capture fails, the compositor must send the failed event.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A ext_image_copy_capture_frame_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ExtImageCopyCaptureFrameV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ExtImageCopyCaptureFrameV1Handler>,
}

struct DefaultHandler;

impl ExtImageCopyCaptureFrameV1Handler for DefaultHandler { }

impl ConcreteObject for ExtImageCopyCaptureFrameV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ExtImageCopyCaptureFrameV1;
    const INTERFACE_NAME: &str = "ext_image_copy_capture_frame_v1";
}

impl ExtImageCopyCaptureFrameV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ExtImageCopyCaptureFrameV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ExtImageCopyCaptureFrameV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ExtImageCopyCaptureFrameV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtImageCopyCaptureFrameV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ExtImageCopyCaptureFrameV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy this object
    ///
    /// Destroys the frame. This request can be sent at any time by the
    /// client.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_image_copy_capture_frame_v1#{}.destroy()\n", id);
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

    /// destroy this object
    ///
    /// Destroys the frame. This request can be sent at any time by the
    /// client.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("ext_image_copy_capture_frame_v1.destroy", &e);
        }
    }

    /// Since when the attach_buffer message is available.
    pub const MSG__ATTACH_BUFFER__SINCE: u32 = 1;

    /// attach buffer to session
    ///
    /// Attach a buffer to the session.
    ///
    /// The wl_buffer.release request is unused.
    ///
    /// The new buffer replaces any previously attached buffer.
    ///
    /// This request must not be sent after capture, or else the
    /// already_captured protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `buffer`:
    #[inline]
    pub fn try_send_attach_buffer(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_image_copy_capture_frame_v1#{}.attach_buffer(buffer: wl_buffer#{})\n", id, arg0);
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

    /// attach buffer to session
    ///
    /// Attach a buffer to the session.
    ///
    /// The wl_buffer.release request is unused.
    ///
    /// The new buffer replaces any previously attached buffer.
    ///
    /// This request must not be sent after capture, or else the
    /// already_captured protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `buffer`:
    #[inline]
    pub fn send_attach_buffer(
        &self,
        buffer: &Rc<WlBuffer>,
    ) {
        let res = self.try_send_attach_buffer(
            buffer,
        );
        if let Err(e) = res {
            log_send("ext_image_copy_capture_frame_v1.attach_buffer", &e);
        }
    }

    /// Since when the damage_buffer message is available.
    pub const MSG__DAMAGE_BUFFER__SINCE: u32 = 1;

    /// damage buffer
    ///
    /// Apply damage to the buffer which is to be captured next. This request
    /// may be sent multiple times to describe a region.
    ///
    /// The client indicates the accumulated damage since this wl_buffer was
    /// last captured. During capture, the compositor will update the buffer
    /// with at least the union of the region passed by the client and the
    /// region advertised by ext_image_copy_capture_frame_v1.damage.
    ///
    /// When a wl_buffer is captured for the first time, or when the client
    /// doesn't track damage, the client must damage the whole buffer.
    ///
    /// This is for optimisation purposes. The compositor may use this
    /// information to reduce copying.
    ///
    /// These coordinates originate from the upper left corner of the buffer.
    ///
    /// If x or y are strictly negative, or if width or height are negative or
    /// zero, the invalid_buffer_damage protocol error is raised.
    ///
    /// This request must not be sent after capture, or else the
    /// already_captured protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `x`: region x coordinate
    /// - `y`: region y coordinate
    /// - `width`: region width
    /// - `height`: region height
    #[inline]
    pub fn try_send_damage_buffer(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_image_copy_capture_frame_v1#{}.damage_buffer(x: {}, y: {}, width: {}, height: {})\n", id, arg0, arg1, arg2, arg3);
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
            2,
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// damage buffer
    ///
    /// Apply damage to the buffer which is to be captured next. This request
    /// may be sent multiple times to describe a region.
    ///
    /// The client indicates the accumulated damage since this wl_buffer was
    /// last captured. During capture, the compositor will update the buffer
    /// with at least the union of the region passed by the client and the
    /// region advertised by ext_image_copy_capture_frame_v1.damage.
    ///
    /// When a wl_buffer is captured for the first time, or when the client
    /// doesn't track damage, the client must damage the whole buffer.
    ///
    /// This is for optimisation purposes. The compositor may use this
    /// information to reduce copying.
    ///
    /// These coordinates originate from the upper left corner of the buffer.
    ///
    /// If x or y are strictly negative, or if width or height are negative or
    /// zero, the invalid_buffer_damage protocol error is raised.
    ///
    /// This request must not be sent after capture, or else the
    /// already_captured protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `x`: region x coordinate
    /// - `y`: region y coordinate
    /// - `width`: region width
    /// - `height`: region height
    #[inline]
    pub fn send_damage_buffer(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_damage_buffer(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_send("ext_image_copy_capture_frame_v1.damage_buffer", &e);
        }
    }

    /// Since when the capture message is available.
    pub const MSG__CAPTURE__SINCE: u32 = 1;

    /// capture a frame
    ///
    /// Capture a frame.
    ///
    /// Unless this is the first successful captured frame performed in this
    /// session, the compositor may wait an indefinite amount of time for the
    /// source content to change before performing the copy.
    ///
    /// This request may only be sent once, or else the already_captured
    /// protocol error is raised. A buffer must be attached before this request
    /// is sent, or else the no_buffer protocol error is raised.
    #[inline]
    pub fn try_send_capture(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_image_copy_capture_frame_v1#{}.capture()\n", id);
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
            3,
        ]);
        Ok(())
    }

    /// capture a frame
    ///
    /// Capture a frame.
    ///
    /// Unless this is the first successful captured frame performed in this
    /// session, the compositor may wait an indefinite amount of time for the
    /// source content to change before performing the copy.
    ///
    /// This request may only be sent once, or else the already_captured
    /// protocol error is raised. A buffer must be attached before this request
    /// is sent, or else the no_buffer protocol error is raised.
    #[inline]
    pub fn send_capture(
        &self,
    ) {
        let res = self.try_send_capture(
        );
        if let Err(e) = res {
            log_send("ext_image_copy_capture_frame_v1.capture", &e);
        }
    }

    /// Since when the transform message is available.
    pub const MSG__TRANSFORM__SINCE: u32 = 1;

    /// buffer transform
    ///
    /// This event is sent before the ready event and holds the transform that
    /// the compositor has applied to the buffer contents.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_image_copy_capture_frame_v1#{}.transform(transform: {:?})\n", client_id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// buffer transform
    ///
    /// This event is sent before the ready event and holds the transform that
    /// the compositor has applied to the buffer contents.
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
            log_send("ext_image_copy_capture_frame_v1.transform", &e);
        }
    }

    /// Since when the damage message is available.
    pub const MSG__DAMAGE__SINCE: u32 = 1;

    /// buffer damaged region
    ///
    /// This event is sent before the ready event. It may be generated multiple
    /// times to describe a region.
    ///
    /// The first captured frame in a session will always carry full damage.
    /// Subsequent frames' damaged regions describe which parts of the buffer
    /// have changed since the last ready event.
    ///
    /// These coordinates originate in the upper left corner of the buffer.
    ///
    /// # Arguments
    ///
    /// - `x`: damage x coordinate
    /// - `y`: damage y coordinate
    /// - `width`: damage width
    /// - `height`: damage height
    #[inline]
    pub fn try_send_damage(
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
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_image_copy_capture_frame_v1#{}.damage(x: {}, y: {}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1, arg2, arg3);
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
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// buffer damaged region
    ///
    /// This event is sent before the ready event. It may be generated multiple
    /// times to describe a region.
    ///
    /// The first captured frame in a session will always carry full damage.
    /// Subsequent frames' damaged regions describe which parts of the buffer
    /// have changed since the last ready event.
    ///
    /// These coordinates originate in the upper left corner of the buffer.
    ///
    /// # Arguments
    ///
    /// - `x`: damage x coordinate
    /// - `y`: damage y coordinate
    /// - `width`: damage width
    /// - `height`: damage height
    #[inline]
    pub fn send_damage(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_damage(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_send("ext_image_copy_capture_frame_v1.damage", &e);
        }
    }

    /// Since when the presentation_time message is available.
    pub const MSG__PRESENTATION_TIME__SINCE: u32 = 1;

    /// presentation time of the frame
    ///
    /// This event indicates the time at which the frame is presented to the
    /// output in system monotonic time. This event is sent before the ready
    /// event.
    ///
    /// The timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,
    /// each component being an unsigned 32-bit value. Whole seconds are in
    /// tv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,
    /// and the additional fractional part in tv_nsec as nanoseconds. Hence,
    /// for valid timestamps tv_nsec must be in [0, 999999999].
    ///
    /// # Arguments
    ///
    /// - `tv_sec_hi`: high 32 bits of the seconds part of the timestamp
    /// - `tv_sec_lo`: low 32 bits of the seconds part of the timestamp
    /// - `tv_nsec`: nanoseconds part of the timestamp
    #[inline]
    pub fn try_send_presentation_time(
        &self,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_image_copy_capture_frame_v1#{}.presentation_time(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {})\n", client_id, id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1, arg2);
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
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// presentation time of the frame
    ///
    /// This event indicates the time at which the frame is presented to the
    /// output in system monotonic time. This event is sent before the ready
    /// event.
    ///
    /// The timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,
    /// each component being an unsigned 32-bit value. Whole seconds are in
    /// tv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,
    /// and the additional fractional part in tv_nsec as nanoseconds. Hence,
    /// for valid timestamps tv_nsec must be in [0, 999999999].
    ///
    /// # Arguments
    ///
    /// - `tv_sec_hi`: high 32 bits of the seconds part of the timestamp
    /// - `tv_sec_lo`: low 32 bits of the seconds part of the timestamp
    /// - `tv_nsec`: nanoseconds part of the timestamp
    #[inline]
    pub fn send_presentation_time(
        &self,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
    ) {
        let res = self.try_send_presentation_time(
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
        );
        if let Err(e) = res {
            log_send("ext_image_copy_capture_frame_v1.presentation_time", &e);
        }
    }

    /// Since when the ready message is available.
    pub const MSG__READY__SINCE: u32 = 1;

    /// frame is available for reading
    ///
    /// Called as soon as the frame is copied, indicating it is available
    /// for reading.
    ///
    /// The buffer may be re-used by the client after this event.
    ///
    /// After receiving this event, the client must destroy the object.
    #[inline]
    pub fn try_send_ready(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_image_copy_capture_frame_v1#{}.ready()\n", client_id, id);
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

    /// frame is available for reading
    ///
    /// Called as soon as the frame is copied, indicating it is available
    /// for reading.
    ///
    /// The buffer may be re-used by the client after this event.
    ///
    /// After receiving this event, the client must destroy the object.
    #[inline]
    pub fn send_ready(
        &self,
    ) {
        let res = self.try_send_ready(
        );
        if let Err(e) = res {
            log_send("ext_image_copy_capture_frame_v1.ready", &e);
        }
    }

    /// Since when the failed message is available.
    pub const MSG__FAILED__SINCE: u32 = 1;

    /// capture failed
    ///
    /// This event indicates that the attempted frame copy has failed.
    ///
    /// After receiving this event, the client must destroy the object.
    ///
    /// # Arguments
    ///
    /// - `reason`:
    #[inline]
    pub fn try_send_failed(
        &self,
        reason: ExtImageCopyCaptureFrameV1FailureReason,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            reason,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: ExtImageCopyCaptureFrameV1FailureReason) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_image_copy_capture_frame_v1#{}.failed(reason: {:?})\n", client_id, id, arg0);
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

    /// capture failed
    ///
    /// This event indicates that the attempted frame copy has failed.
    ///
    /// After receiving this event, the client must destroy the object.
    ///
    /// # Arguments
    ///
    /// - `reason`:
    #[inline]
    pub fn send_failed(
        &self,
        reason: ExtImageCopyCaptureFrameV1FailureReason,
    ) {
        let res = self.try_send_failed(
            reason,
        );
        if let Err(e) = res {
            log_send("ext_image_copy_capture_frame_v1.failed", &e);
        }
    }
}

/// A message handler for [`ExtImageCopyCaptureFrameV1`] proxies.
pub trait ExtImageCopyCaptureFrameV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ExtImageCopyCaptureFrameV1>) {
        slf.core.delete_id();
    }

    /// destroy this object
    ///
    /// Destroys the frame. This request can be sent at any time by the
    /// client.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ExtImageCopyCaptureFrameV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("ext_image_copy_capture_frame_v1.destroy", &e);
        }
    }

    /// attach buffer to session
    ///
    /// Attach a buffer to the session.
    ///
    /// The wl_buffer.release request is unused.
    ///
    /// The new buffer replaces any previously attached buffer.
    ///
    /// This request must not be sent after capture, or else the
    /// already_captured protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `buffer`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_attach_buffer(
        &mut self,
        slf: &Rc<ExtImageCopyCaptureFrameV1>,
        buffer: &Rc<WlBuffer>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_attach_buffer(
            buffer,
        );
        if let Err(e) = res {
            log_forward("ext_image_copy_capture_frame_v1.attach_buffer", &e);
        }
    }

    /// damage buffer
    ///
    /// Apply damage to the buffer which is to be captured next. This request
    /// may be sent multiple times to describe a region.
    ///
    /// The client indicates the accumulated damage since this wl_buffer was
    /// last captured. During capture, the compositor will update the buffer
    /// with at least the union of the region passed by the client and the
    /// region advertised by ext_image_copy_capture_frame_v1.damage.
    ///
    /// When a wl_buffer is captured for the first time, or when the client
    /// doesn't track damage, the client must damage the whole buffer.
    ///
    /// This is for optimisation purposes. The compositor may use this
    /// information to reduce copying.
    ///
    /// These coordinates originate from the upper left corner of the buffer.
    ///
    /// If x or y are strictly negative, or if width or height are negative or
    /// zero, the invalid_buffer_damage protocol error is raised.
    ///
    /// This request must not be sent after capture, or else the
    /// already_captured protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `x`: region x coordinate
    /// - `y`: region y coordinate
    /// - `width`: region width
    /// - `height`: region height
    #[inline]
    fn handle_damage_buffer(
        &mut self,
        slf: &Rc<ExtImageCopyCaptureFrameV1>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_damage_buffer(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("ext_image_copy_capture_frame_v1.damage_buffer", &e);
        }
    }

    /// capture a frame
    ///
    /// Capture a frame.
    ///
    /// Unless this is the first successful captured frame performed in this
    /// session, the compositor may wait an indefinite amount of time for the
    /// source content to change before performing the copy.
    ///
    /// This request may only be sent once, or else the already_captured
    /// protocol error is raised. A buffer must be attached before this request
    /// is sent, or else the no_buffer protocol error is raised.
    #[inline]
    fn handle_capture(
        &mut self,
        slf: &Rc<ExtImageCopyCaptureFrameV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_capture(
        );
        if let Err(e) = res {
            log_forward("ext_image_copy_capture_frame_v1.capture", &e);
        }
    }

    /// buffer transform
    ///
    /// This event is sent before the ready event and holds the transform that
    /// the compositor has applied to the buffer contents.
    ///
    /// # Arguments
    ///
    /// - `transform`:
    #[inline]
    fn handle_transform(
        &mut self,
        slf: &Rc<ExtImageCopyCaptureFrameV1>,
        transform: WlOutputTransform,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_transform(
            transform,
        );
        if let Err(e) = res {
            log_forward("ext_image_copy_capture_frame_v1.transform", &e);
        }
    }

    /// buffer damaged region
    ///
    /// This event is sent before the ready event. It may be generated multiple
    /// times to describe a region.
    ///
    /// The first captured frame in a session will always carry full damage.
    /// Subsequent frames' damaged regions describe which parts of the buffer
    /// have changed since the last ready event.
    ///
    /// These coordinates originate in the upper left corner of the buffer.
    ///
    /// # Arguments
    ///
    /// - `x`: damage x coordinate
    /// - `y`: damage y coordinate
    /// - `width`: damage width
    /// - `height`: damage height
    #[inline]
    fn handle_damage(
        &mut self,
        slf: &Rc<ExtImageCopyCaptureFrameV1>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_damage(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("ext_image_copy_capture_frame_v1.damage", &e);
        }
    }

    /// presentation time of the frame
    ///
    /// This event indicates the time at which the frame is presented to the
    /// output in system monotonic time. This event is sent before the ready
    /// event.
    ///
    /// The timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,
    /// each component being an unsigned 32-bit value. Whole seconds are in
    /// tv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,
    /// and the additional fractional part in tv_nsec as nanoseconds. Hence,
    /// for valid timestamps tv_nsec must be in [0, 999999999].
    ///
    /// # Arguments
    ///
    /// - `tv_sec_hi`: high 32 bits of the seconds part of the timestamp
    /// - `tv_sec_lo`: low 32 bits of the seconds part of the timestamp
    /// - `tv_nsec`: nanoseconds part of the timestamp
    #[inline]
    fn handle_presentation_time(
        &mut self,
        slf: &Rc<ExtImageCopyCaptureFrameV1>,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_presentation_time(
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
        );
        if let Err(e) = res {
            log_forward("ext_image_copy_capture_frame_v1.presentation_time", &e);
        }
    }

    /// frame is available for reading
    ///
    /// Called as soon as the frame is copied, indicating it is available
    /// for reading.
    ///
    /// The buffer may be re-used by the client after this event.
    ///
    /// After receiving this event, the client must destroy the object.
    #[inline]
    fn handle_ready(
        &mut self,
        slf: &Rc<ExtImageCopyCaptureFrameV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_ready(
        );
        if let Err(e) = res {
            log_forward("ext_image_copy_capture_frame_v1.ready", &e);
        }
    }

    /// capture failed
    ///
    /// This event indicates that the attempted frame copy has failed.
    ///
    /// After receiving this event, the client must destroy the object.
    ///
    /// # Arguments
    ///
    /// - `reason`:
    #[inline]
    fn handle_failed(
        &mut self,
        slf: &Rc<ExtImageCopyCaptureFrameV1>,
        reason: ExtImageCopyCaptureFrameV1FailureReason,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_failed(
            reason,
        );
        if let Err(e) = res {
            log_forward("ext_image_copy_capture_frame_v1.failed", &e);
        }
    }
}

impl ObjectPrivate for ExtImageCopyCaptureFrameV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ExtImageCopyCaptureFrameV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_image_copy_capture_frame_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_image_copy_capture_frame_v1#{}.attach_buffer(buffer: wl_buffer#{})\n", client_id, id, arg0);
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
                    (**handler).handle_attach_buffer(&self, arg0);
                } else {
                    DefaultHandler.handle_attach_buffer(&self, arg0);
                }
            }
            2 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_image_copy_capture_frame_v1#{}.damage_buffer(x: {}, y: {}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_damage_buffer(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_damage_buffer(&self, arg0, arg1, arg2, arg3);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_image_copy_capture_frame_v1#{}.capture()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_capture(&self);
                } else {
                    DefaultHandler.handle_capture(&self);
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
                let arg0 = WlOutputTransform(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: WlOutputTransform) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_image_copy_capture_frame_v1#{}.transform(transform: {:?})\n", id, arg0);
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
            1 => {
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
                    fn log(state: &State, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_image_copy_capture_frame_v1#{}.damage(x: {}, y: {}, width: {}, height: {})\n", id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_damage(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_damage(&self, arg0, arg1, arg2, arg3);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_image_copy_capture_frame_v1#{}.presentation_time(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {})\n", id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_presentation_time(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_presentation_time(&self, arg0, arg1, arg2);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_image_copy_capture_frame_v1#{}.ready()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_ready(&self);
                } else {
                    DefaultHandler.handle_ready(&self);
                }
            }
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = ExtImageCopyCaptureFrameV1FailureReason(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: ExtImageCopyCaptureFrameV1FailureReason) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_image_copy_capture_frame_v1#{}.failed(reason: {:?})\n", id, arg0);
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
            1 => "attach_buffer",
            2 => "damage_buffer",
            3 => "capture",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "transform",
            1 => "damage",
            2 => "presentation_time",
            3 => "ready",
            4 => "failed",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ExtImageCopyCaptureFrameV1 {
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

impl ExtImageCopyCaptureFrameV1 {
    /// Since when the error.no_buffer enum variant is available.
    pub const ENM__ERROR_NO_BUFFER__SINCE: u32 = 1;
    /// Since when the error.invalid_buffer_damage enum variant is available.
    pub const ENM__ERROR_INVALID_BUFFER_DAMAGE__SINCE: u32 = 1;
    /// Since when the error.already_captured enum variant is available.
    pub const ENM__ERROR_ALREADY_CAPTURED__SINCE: u32 = 1;

    /// Since when the failure_reason.unknown enum variant is available.
    pub const ENM__FAILURE_REASON_UNKNOWN__SINCE: u32 = 1;
    /// Since when the failure_reason.buffer_constraints enum variant is available.
    pub const ENM__FAILURE_REASON_BUFFER_CONSTRAINTS__SINCE: u32 = 1;
    /// Since when the failure_reason.stopped enum variant is available.
    pub const ENM__FAILURE_REASON_STOPPED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtImageCopyCaptureFrameV1Error(pub u32);

impl ExtImageCopyCaptureFrameV1Error {
    /// capture sent without attach_buffer
    pub const NO_BUFFER: Self = Self(1);

    /// invalid buffer damage
    pub const INVALID_BUFFER_DAMAGE: Self = Self(2);

    /// capture request has been sent
    pub const ALREADY_CAPTURED: Self = Self(3);
}

impl Debug for ExtImageCopyCaptureFrameV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NO_BUFFER => "NO_BUFFER",
            Self::INVALID_BUFFER_DAMAGE => "INVALID_BUFFER_DAMAGE",
            Self::ALREADY_CAPTURED => "ALREADY_CAPTURED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtImageCopyCaptureFrameV1FailureReason(pub u32);

impl ExtImageCopyCaptureFrameV1FailureReason {
    /// unknown runtime error
    ///
    /// An unspecified runtime error has occurred. The client may retry.
    pub const UNKNOWN: Self = Self(0);

    /// buffer constraints mismatch
    ///
    /// The buffer submitted by the client doesn't match the latest session
    /// constraints. The client should re-allocate its buffers and retry.
    pub const BUFFER_CONSTRAINTS: Self = Self(1);

    /// session is no longer available
    ///
    /// The session has stopped. See ext_image_copy_capture_session_v1.stopped.
    pub const STOPPED: Self = Self(2);
}

impl Debug for ExtImageCopyCaptureFrameV1FailureReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::UNKNOWN => "UNKNOWN",
            Self::BUFFER_CONSTRAINTS => "BUFFER_CONSTRAINTS",
            Self::STOPPED => "STOPPED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
