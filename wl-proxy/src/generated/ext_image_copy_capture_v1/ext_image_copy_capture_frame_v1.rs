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

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_image_copy_capture_frame_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ExtImageCopyCaptureFrameV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn ExtImageCopyCaptureFrameV1Handler>,
}

struct DefaultHandler;

impl ExtImageCopyCaptureFrameV1Handler for DefaultHandler { }

impl ExtImageCopyCaptureFrameV1 {
    pub const XML_VERSION: u32 = 1;
}

impl ExtImageCopyCaptureFrameV1 {
    pub fn set_handler(&self, handler: impl ExtImageCopyCaptureFrameV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

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
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy this object
    ///
    /// Destroys the frame. This request can be sent at any time by the
    /// client.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let args = format_args!("[{millis:7}.{micros:03}] server      <= ext_image_copy_capture_frame_v1#{}.destroy()\n", id);
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

    /// Since when the attach_buffer message is available.
    #[allow(dead_code)]
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
    pub fn send_attach_buffer(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("buffer")),
            Some(id) => id,
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let args = format_args!("[{millis:7}.{micros:03}] server      <= ext_image_copy_capture_frame_v1#{}.attach_buffer(buffer: wl_buffer#{})\n", id, arg0_id);
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
            1,
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the damage_buffer message is available.
    #[allow(dead_code)]
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
    pub fn send_damage_buffer(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let args = format_args!("[{millis:7}.{micros:03}] server      <= ext_image_copy_capture_frame_v1#{}.damage_buffer(x: {}, y: {}, width: {}, height: {})\n", id, arg0, arg1, arg2, arg3);
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
            2,
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// Since when the capture message is available.
    #[allow(dead_code)]
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
    pub fn send_capture(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let args = format_args!("[{millis:7}.{micros:03}] server      <= ext_image_copy_capture_frame_v1#{}.capture()\n", id);
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
            3,
        ]);
        Ok(())
    }

    /// Since when the transform message is available.
    #[allow(dead_code)]
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
    pub fn send_transform(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let args = format_args!("[{millis:7}.{micros:03}] client#{:<4} <= ext_image_copy_capture_frame_v1#{}.transform(transform: {:?})\n", client.endpoint.id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the damage message is available.
    #[allow(dead_code)]
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
    pub fn send_damage(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let args = format_args!("[{millis:7}.{micros:03}] client#{:<4} <= ext_image_copy_capture_frame_v1#{}.damage(x: {}, y: {}, width: {}, height: {})\n", client.endpoint.id, id, arg0, arg1, arg2, arg3);
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
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// Since when the presentation_time message is available.
    #[allow(dead_code)]
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
    pub fn send_presentation_time(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let args = format_args!("[{millis:7}.{micros:03}] client#{:<4} <= ext_image_copy_capture_frame_v1#{}.presentation_time(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {})\n", client.endpoint.id, id, arg0, arg1, arg2);
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
            arg0,
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// Since when the ready message is available.
    #[allow(dead_code)]
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
    pub fn send_ready(
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
            let args = format_args!("[{millis:7}.{micros:03}] client#{:<4} <= ext_image_copy_capture_frame_v1#{}.ready()\n", client.endpoint.id, id);
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
        ]);
        Ok(())
    }

    /// Since when the failed message is available.
    #[allow(dead_code)]
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
    pub fn send_failed(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let args = format_args!("[{millis:7}.{micros:03}] client#{:<4} <= ext_image_copy_capture_frame_v1#{}.failed(reason: {:?})\n", client.endpoint.id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }
}

/// A message handler for [ExtImageCopyCaptureFrameV1] proxies.
#[allow(dead_code)]
pub trait ExtImageCopyCaptureFrameV1Handler: Any {
    /// destroy this object
    ///
    /// Destroys the frame. This request can be sent at any time by the
    /// client.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<ExtImageCopyCaptureFrameV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_frame_v1.destroy message: {}", Report::new(e));
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
    fn attach_buffer(
        &mut self,
        _slf: &Rc<ExtImageCopyCaptureFrameV1>,
        buffer: &Rc<WlBuffer>,
    ) {
        let res = _slf.send_attach_buffer(
            buffer,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_frame_v1.attach_buffer message: {}", Report::new(e));
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
    fn damage_buffer(
        &mut self,
        _slf: &Rc<ExtImageCopyCaptureFrameV1>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = _slf.send_damage_buffer(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_frame_v1.damage_buffer message: {}", Report::new(e));
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
    fn capture(
        &mut self,
        _slf: &Rc<ExtImageCopyCaptureFrameV1>,
    ) {
        let res = _slf.send_capture(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_frame_v1.capture message: {}", Report::new(e));
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
    fn transform(
        &mut self,
        _slf: &Rc<ExtImageCopyCaptureFrameV1>,
        transform: WlOutputTransform,
    ) {
        let res = _slf.send_transform(
            transform,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_frame_v1.transform message: {}", Report::new(e));
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
    fn damage(
        &mut self,
        _slf: &Rc<ExtImageCopyCaptureFrameV1>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = _slf.send_damage(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_frame_v1.damage message: {}", Report::new(e));
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
    fn presentation_time(
        &mut self,
        _slf: &Rc<ExtImageCopyCaptureFrameV1>,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
    ) {
        let res = _slf.send_presentation_time(
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_frame_v1.presentation_time message: {}", Report::new(e));
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
    fn ready(
        &mut self,
        _slf: &Rc<ExtImageCopyCaptureFrameV1>,
    ) {
        let res = _slf.send_ready(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_frame_v1.ready message: {}", Report::new(e));
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
    fn failed(
        &mut self,
        _slf: &Rc<ExtImageCopyCaptureFrameV1>,
        reason: ExtImageCopyCaptureFrameV1FailureReason,
    ) {
        let res = _slf.send_failed(
            reason,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_frame_v1.failed message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for ExtImageCopyCaptureFrameV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::ExtImageCopyCaptureFrameV1, version),
            handler: Default::default(),
        })
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let args = format_args!("[{millis:7}.{micros:03}] client#{:<4} -> ext_image_copy_capture_frame_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let args = format_args!("[{millis:7}.{micros:03}] client#{:<4} -> ext_image_copy_capture_frame_v1#{}.attach_buffer(buffer: wl_buffer#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlBuffer>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("buffer", o.core().interface, ProxyInterface::WlBuffer));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).attach_buffer(&self, arg0);
                } else {
                    DefaultHandler.attach_buffer(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 24));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let args = format_args!("[{millis:7}.{micros:03}] client#{:<4} -> ext_image_copy_capture_frame_v1#{}.damage_buffer(x: {}, y: {}, width: {}, height: {})\n", client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).damage_buffer(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.damage_buffer(&self, arg0, arg1, arg2, arg3);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let args = format_args!("[{millis:7}.{micros:03}] client#{:<4} -> ext_image_copy_capture_frame_v1#{}.capture()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).capture(&self);
                } else {
                    DefaultHandler.capture(&self);
                }
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = WlOutputTransform(arg0);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let args = format_args!("[{millis:7}.{micros:03}] server      -> ext_image_copy_capture_frame_v1#{}.transform(transform: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).transform(&self, arg0);
                } else {
                    DefaultHandler.transform(&self, arg0);
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
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let args = format_args!("[{millis:7}.{micros:03}] server      -> ext_image_copy_capture_frame_v1#{}.damage(x: {}, y: {}, width: {}, height: {})\n", msg[0], arg0, arg1, arg2, arg3);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).damage(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.damage(&self, arg0, arg1, arg2, arg3);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let args = format_args!("[{millis:7}.{micros:03}] server      -> ext_image_copy_capture_frame_v1#{}.presentation_time(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {})\n", msg[0], arg0, arg1, arg2);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).presentation_time(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.presentation_time(&self, arg0, arg1, arg2);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let args = format_args!("[{millis:7}.{micros:03}] server      -> ext_image_copy_capture_frame_v1#{}.ready()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).ready(&self);
                } else {
                    DefaultHandler.ready(&self);
                }
            }
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = ExtImageCopyCaptureFrameV1FailureReason(arg0);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let args = format_args!("[{millis:7}.{micros:03}] server      -> ext_image_copy_capture_frame_v1#{}.failed(reason: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).failed(&self, arg0);
                } else {
                    DefaultHandler.failed(&self, arg0);
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

impl Proxy for ExtImageCopyCaptureFrameV1 {
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

impl ExtImageCopyCaptureFrameV1 {
    /// Since when the error.no_buffer enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_NO_BUFFER__SINCE: u32 = 1;
    /// Since when the error.invalid_buffer_damage enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_BUFFER_DAMAGE__SINCE: u32 = 1;
    /// Since when the error.already_captured enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ALREADY_CAPTURED__SINCE: u32 = 1;

    /// Since when the failure_reason.unknown enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FAILURE_REASON_UNKNOWN__SINCE: u32 = 1;
    /// Since when the failure_reason.buffer_constraints enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FAILURE_REASON_BUFFER_CONSTRAINTS__SINCE: u32 = 1;
    /// Since when the failure_reason.stopped enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FAILURE_REASON_STOPPED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct ExtImageCopyCaptureFrameV1Error(pub u32);

impl ExtImageCopyCaptureFrameV1Error {
    /// capture sent without attach_buffer
    #[allow(dead_code)]
    pub const NO_BUFFER: Self = Self(1);

    /// invalid buffer damage
    #[allow(dead_code)]
    pub const INVALID_BUFFER_DAMAGE: Self = Self(2);

    /// capture request has been sent
    #[allow(dead_code)]
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
#[allow(dead_code)]
pub struct ExtImageCopyCaptureFrameV1FailureReason(pub u32);

impl ExtImageCopyCaptureFrameV1FailureReason {
    /// unknown runtime error
    ///
    /// An unspecified runtime error has occurred. The client may retry.
    #[allow(dead_code)]
    pub const UNKNOWN: Self = Self(0);

    /// buffer constraints mismatch
    ///
    /// The buffer submitted by the client doesn't match the latest session
    /// constraints. The client should re-allocate its buffers and retry.
    #[allow(dead_code)]
    pub const BUFFER_CONSTRAINTS: Self = Self(1);

    /// session is no longer available
    ///
    /// The session has stopped. See ext_image_copy_capture_session_v1.stopped.
    #[allow(dead_code)]
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
