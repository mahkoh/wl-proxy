//! a frame ready for copy
//!
//! This object represents a single frame.
//!
//! When created, a series of buffer events will be sent, each representing a
//! supported buffer type. The "buffer_done" event is sent afterwards to
//! indicate that all supported buffer types have been enumerated. The client
//! will then be able to send a "copy" request. If the capture is successful,
//! the compositor will send a "flags" event followed by a "ready" event.
//!
//! For objects version 2 or lower, wl_shm buffers are always supported, ie.
//! the "buffer" event is guaranteed to be sent.
//!
//! If the capture failed, the "failed" event is sent. This can happen anytime
//! before the "ready" event.
//!
//! Once either a "ready" or a "failed" event is received, the client should
//! destroy the frame.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwlr_screencopy_frame_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwlrScreencopyFrameV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwlrScreencopyFrameV1Handler>,
}

struct DefaultHandler;

impl ZwlrScreencopyFrameV1Handler for DefaultHandler { }

impl ZwlrScreencopyFrameV1 {
    pub const XML_VERSION: u32 = 3;
    pub const INTERFACE: ObjectInterface = ObjectInterface::ZwlrScreencopyFrameV1;
    pub const INTERFACE_NAME: &str = "zwlr_screencopy_frame_v1";
}

impl ZwlrScreencopyFrameV1 {
    pub fn set_handler(&self, handler: impl ZwlrScreencopyFrameV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZwlrScreencopyFrameV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwlrScreencopyFrameV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwlrScreencopyFrameV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwlrScreencopyFrameV1 {
    /// Since when the buffer message is available.
    pub const MSG__BUFFER__SINCE: u32 = 1;

    /// wl_shm buffer information
    ///
    /// Provides information about wl_shm buffer parameters that need to be
    /// used for this frame. This event is sent once after the frame is created
    /// if wl_shm buffers are supported.
    ///
    /// # Arguments
    ///
    /// - `format`: buffer format
    /// - `width`: buffer width
    /// - `height`: buffer height
    /// - `stride`: buffer stride
    #[inline]
    pub fn send_buffer(
        &self,
        format: WlShmFormat,
        width: u32,
        height: u32,
        stride: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            format,
            width,
            height,
            stride,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_screencopy_frame_v1#{}.buffer(format: {:?}, width: {}, height: {}, stride: {})\n", client.endpoint.id, id, arg0, arg1, arg2, arg3);
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
            arg1,
            arg2,
            arg3,
        ]);
        Ok(())
    }

    /// Since when the copy message is available.
    pub const MSG__COPY__SINCE: u32 = 1;

    /// copy the frame
    ///
    /// Copy the frame to the supplied buffer. The buffer must have the
    /// correct size, see zwlr_screencopy_frame_v1.buffer and
    /// zwlr_screencopy_frame_v1.linux_dmabuf. The buffer needs to have a
    /// supported format.
    ///
    /// If the frame is successfully copied, "flags" and "ready" events are
    /// sent. Otherwise, a "failed" event is sent.
    ///
    /// # Arguments
    ///
    /// - `buffer`:
    #[inline]
    pub fn send_copy(
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
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_screencopy_frame_v1#{}.copy(buffer: wl_buffer#{})\n", id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the flags message is available.
    pub const MSG__FLAGS__SINCE: u32 = 1;

    /// frame flags
    ///
    /// Provides flags about the frame. This event is sent once before the
    /// "ready" event.
    ///
    /// # Arguments
    ///
    /// - `flags`: frame flags
    #[inline]
    pub fn send_flags(
        &self,
        flags: ZwlrScreencopyFrameV1Flags,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            flags,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_screencopy_frame_v1#{}.flags(flags: {:?})\n", client.endpoint.id, id, arg0);
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
        ]);
        Ok(())
    }

    /// Since when the ready message is available.
    pub const MSG__READY__SINCE: u32 = 1;

    /// indicates frame is available for reading
    ///
    /// Called as soon as the frame is copied, indicating it is available
    /// for reading. This event includes the time at which the presentation took place.
    ///
    /// The timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,
    /// each component being an unsigned 32-bit value. Whole seconds are in
    /// tv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,
    /// and the additional fractional part in tv_nsec as nanoseconds. Hence,
    /// for valid timestamps tv_nsec must be in [0, 999999999]. The seconds part
    /// may have an arbitrary offset at start.
    ///
    /// After receiving this event, the client should destroy the object.
    ///
    /// # Arguments
    ///
    /// - `tv_sec_hi`: high 32 bits of the seconds part of the timestamp
    /// - `tv_sec_lo`: low 32 bits of the seconds part of the timestamp
    /// - `tv_nsec`: nanoseconds part of the timestamp
    #[inline]
    pub fn send_ready(
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
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_screencopy_frame_v1#{}.ready(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {})\n", client.endpoint.id, id, arg0, arg1, arg2);
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

    /// Since when the failed message is available.
    pub const MSG__FAILED__SINCE: u32 = 1;

    /// frame copy failed
    ///
    /// This event indicates that the attempted frame copy has failed.
    ///
    /// After receiving this event, the client should destroy the object.
    #[inline]
    pub fn send_failed(
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_screencopy_frame_v1#{}.failed()\n", client.endpoint.id, id);
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

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// delete this object, used or not
    ///
    /// Destroys the frame. This request can be sent at any time by the client.
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
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_screencopy_frame_v1#{}.destroy()\n", id);
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
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the copy_with_damage message is available.
    pub const MSG__COPY_WITH_DAMAGE__SINCE: u32 = 2;

    /// copy the frame when it's damaged
    ///
    /// Same as copy, except it waits until there is damage to copy.
    ///
    /// # Arguments
    ///
    /// - `buffer`:
    #[inline]
    pub fn send_copy_with_damage(
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
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_screencopy_frame_v1#{}.copy_with_damage(buffer: wl_buffer#{})\n", id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the damage message is available.
    pub const MSG__DAMAGE__SINCE: u32 = 2;

    /// carries the coordinates of the damaged region
    ///
    /// This event is sent right before the ready event when copy_with_damage is
    /// requested. It may be generated multiple times for each copy_with_damage
    /// request.
    ///
    /// The arguments describe a box around an area that has changed since the
    /// last copy request that was derived from the current screencopy manager
    /// instance.
    ///
    /// The union of all regions received between the call to copy_with_damage
    /// and a ready event is the total damage since the prior ready event.
    ///
    /// # Arguments
    ///
    /// - `x`: damaged x coordinates
    /// - `y`: damaged y coordinates
    /// - `width`: current width
    /// - `height`: current height
    #[inline]
    pub fn send_damage(
        &self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
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
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_screencopy_frame_v1#{}.damage(x: {}, y: {}, width: {}, height: {})\n", client.endpoint.id, id, arg0, arg1, arg2, arg3);
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
            arg0,
            arg1,
            arg2,
            arg3,
        ]);
        Ok(())
    }

    /// Since when the linux_dmabuf message is available.
    pub const MSG__LINUX_DMABUF__SINCE: u32 = 3;

    /// linux-dmabuf buffer information
    ///
    /// Provides information about linux-dmabuf buffer parameters that need to
    /// be used for this frame. This event is sent once after the frame is
    /// created if linux-dmabuf buffers are supported.
    ///
    /// # Arguments
    ///
    /// - `format`: fourcc pixel format
    /// - `width`: buffer width
    /// - `height`: buffer height
    #[inline]
    pub fn send_linux_dmabuf(
        &self,
        format: u32,
        width: u32,
        height: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            format,
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
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_screencopy_frame_v1#{}.linux_dmabuf(format: {}, width: {}, height: {})\n", client.endpoint.id, id, arg0, arg1, arg2);
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
            arg0,
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// Since when the buffer_done message is available.
    pub const MSG__BUFFER_DONE__SINCE: u32 = 3;

    /// all buffer types reported
    ///
    /// This event is sent once after all buffer events have been sent.
    ///
    /// The client should proceed to create a buffer of one of the supported
    /// types, and send a "copy" request.
    #[inline]
    pub fn send_buffer_done(
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_screencopy_frame_v1#{}.buffer_done()\n", client.endpoint.id, id);
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
            6,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwlrScreencopyFrameV1] proxies.
pub trait ZwlrScreencopyFrameV1Handler: Any {
    /// wl_shm buffer information
    ///
    /// Provides information about wl_shm buffer parameters that need to be
    /// used for this frame. This event is sent once after the frame is created
    /// if wl_shm buffers are supported.
    ///
    /// # Arguments
    ///
    /// - `format`: buffer format
    /// - `width`: buffer width
    /// - `height`: buffer height
    /// - `stride`: buffer stride
    #[inline]
    fn buffer(
        &mut self,
        _slf: &Rc<ZwlrScreencopyFrameV1>,
        format: WlShmFormat,
        width: u32,
        height: u32,
        stride: u32,
    ) {
        let res = _slf.send_buffer(
            format,
            width,
            height,
            stride,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_screencopy_frame_v1.buffer message: {}", Report::new(e));
        }
    }

    /// copy the frame
    ///
    /// Copy the frame to the supplied buffer. The buffer must have the
    /// correct size, see zwlr_screencopy_frame_v1.buffer and
    /// zwlr_screencopy_frame_v1.linux_dmabuf. The buffer needs to have a
    /// supported format.
    ///
    /// If the frame is successfully copied, "flags" and "ready" events are
    /// sent. Otherwise, a "failed" event is sent.
    ///
    /// # Arguments
    ///
    /// - `buffer`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn copy(
        &mut self,
        _slf: &Rc<ZwlrScreencopyFrameV1>,
        buffer: &Rc<WlBuffer>,
    ) {
        let res = _slf.send_copy(
            buffer,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_screencopy_frame_v1.copy message: {}", Report::new(e));
        }
    }

    /// frame flags
    ///
    /// Provides flags about the frame. This event is sent once before the
    /// "ready" event.
    ///
    /// # Arguments
    ///
    /// - `flags`: frame flags
    #[inline]
    fn flags(
        &mut self,
        _slf: &Rc<ZwlrScreencopyFrameV1>,
        flags: ZwlrScreencopyFrameV1Flags,
    ) {
        let res = _slf.send_flags(
            flags,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_screencopy_frame_v1.flags message: {}", Report::new(e));
        }
    }

    /// indicates frame is available for reading
    ///
    /// Called as soon as the frame is copied, indicating it is available
    /// for reading. This event includes the time at which the presentation took place.
    ///
    /// The timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,
    /// each component being an unsigned 32-bit value. Whole seconds are in
    /// tv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,
    /// and the additional fractional part in tv_nsec as nanoseconds. Hence,
    /// for valid timestamps tv_nsec must be in [0, 999999999]. The seconds part
    /// may have an arbitrary offset at start.
    ///
    /// After receiving this event, the client should destroy the object.
    ///
    /// # Arguments
    ///
    /// - `tv_sec_hi`: high 32 bits of the seconds part of the timestamp
    /// - `tv_sec_lo`: low 32 bits of the seconds part of the timestamp
    /// - `tv_nsec`: nanoseconds part of the timestamp
    #[inline]
    fn ready(
        &mut self,
        _slf: &Rc<ZwlrScreencopyFrameV1>,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
    ) {
        let res = _slf.send_ready(
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_screencopy_frame_v1.ready message: {}", Report::new(e));
        }
    }

    /// frame copy failed
    ///
    /// This event indicates that the attempted frame copy has failed.
    ///
    /// After receiving this event, the client should destroy the object.
    #[inline]
    fn failed(
        &mut self,
        _slf: &Rc<ZwlrScreencopyFrameV1>,
    ) {
        let res = _slf.send_failed(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_screencopy_frame_v1.failed message: {}", Report::new(e));
        }
    }

    /// delete this object, used or not
    ///
    /// Destroys the frame. This request can be sent at any time by the client.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<ZwlrScreencopyFrameV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_screencopy_frame_v1.destroy message: {}", Report::new(e));
        }
    }

    /// copy the frame when it's damaged
    ///
    /// Same as copy, except it waits until there is damage to copy.
    ///
    /// # Arguments
    ///
    /// - `buffer`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn copy_with_damage(
        &mut self,
        _slf: &Rc<ZwlrScreencopyFrameV1>,
        buffer: &Rc<WlBuffer>,
    ) {
        let res = _slf.send_copy_with_damage(
            buffer,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_screencopy_frame_v1.copy_with_damage message: {}", Report::new(e));
        }
    }

    /// carries the coordinates of the damaged region
    ///
    /// This event is sent right before the ready event when copy_with_damage is
    /// requested. It may be generated multiple times for each copy_with_damage
    /// request.
    ///
    /// The arguments describe a box around an area that has changed since the
    /// last copy request that was derived from the current screencopy manager
    /// instance.
    ///
    /// The union of all regions received between the call to copy_with_damage
    /// and a ready event is the total damage since the prior ready event.
    ///
    /// # Arguments
    ///
    /// - `x`: damaged x coordinates
    /// - `y`: damaged y coordinates
    /// - `width`: current width
    /// - `height`: current height
    #[inline]
    fn damage(
        &mut self,
        _slf: &Rc<ZwlrScreencopyFrameV1>,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) {
        let res = _slf.send_damage(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_screencopy_frame_v1.damage message: {}", Report::new(e));
        }
    }

    /// linux-dmabuf buffer information
    ///
    /// Provides information about linux-dmabuf buffer parameters that need to
    /// be used for this frame. This event is sent once after the frame is
    /// created if linux-dmabuf buffers are supported.
    ///
    /// # Arguments
    ///
    /// - `format`: fourcc pixel format
    /// - `width`: buffer width
    /// - `height`: buffer height
    #[inline]
    fn linux_dmabuf(
        &mut self,
        _slf: &Rc<ZwlrScreencopyFrameV1>,
        format: u32,
        width: u32,
        height: u32,
    ) {
        let res = _slf.send_linux_dmabuf(
            format,
            width,
            height,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_screencopy_frame_v1.linux_dmabuf message: {}", Report::new(e));
        }
    }

    /// all buffer types reported
    ///
    /// This event is sent once after all buffer events have been sent.
    ///
    /// The client should proceed to create a buffer of one of the supported
    /// types, and send a "copy" request.
    #[inline]
    fn buffer_done(
        &mut self,
        _slf: &Rc<ZwlrScreencopyFrameV1>,
    ) {
        let res = _slf.send_buffer_done(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_screencopy_frame_v1.buffer_done message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for ZwlrScreencopyFrameV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwlrScreencopyFrameV1, version),
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_screencopy_frame_v1#{}.copy(buffer: wl_buffer#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlBuffer>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("buffer", o.core().interface, ObjectInterface::WlBuffer));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).copy(&self, arg0);
                } else {
                    DefaultHandler.copy(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_screencopy_frame_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_screencopy_frame_v1#{}.copy_with_damage(buffer: wl_buffer#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlBuffer>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("buffer", o.core().interface, ObjectInterface::WlBuffer));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).copy_with_damage(&self, arg0);
                } else {
                    DefaultHandler.copy_with_damage(&self, arg0);
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
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 24));
                };
                let arg0 = WlShmFormat(arg0);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_screencopy_frame_v1#{}.buffer(format: {:?}, width: {}, height: {}, stride: {})\n", msg[0], arg0, arg1, arg2, arg3);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).buffer(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.buffer(&self, arg0, arg1, arg2, arg3);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = ZwlrScreencopyFrameV1Flags(arg0);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_screencopy_frame_v1#{}.flags(flags: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).flags(&self, arg0);
                } else {
                    DefaultHandler.flags(&self, arg0);
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
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_screencopy_frame_v1#{}.ready(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {})\n", msg[0], arg0, arg1, arg2);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).ready(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.ready(&self, arg0, arg1, arg2);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_screencopy_frame_v1#{}.failed()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).failed(&self);
                } else {
                    DefaultHandler.failed(&self);
                }
            }
            4 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 24));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_screencopy_frame_v1#{}.damage(x: {}, y: {}, width: {}, height: {})\n", msg[0], arg0, arg1, arg2, arg3);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).damage(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.damage(&self, arg0, arg1, arg2, arg3);
                }
            }
            5 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_screencopy_frame_v1#{}.linux_dmabuf(format: {}, width: {}, height: {})\n", msg[0], arg0, arg1, arg2);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).linux_dmabuf(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.linux_dmabuf(&self, arg0, arg1, arg2);
                }
            }
            6 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_screencopy_frame_v1#{}.buffer_done()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).buffer_done(&self);
                } else {
                    DefaultHandler.buffer_done(&self);
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
            0 => "copy",
            1 => "destroy",
            2 => "copy_with_damage",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "buffer",
            1 => "flags",
            2 => "ready",
            3 => "failed",
            4 => "damage",
            5 => "linux_dmabuf",
            6 => "buffer_done",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwlrScreencopyFrameV1 {
    fn core(&self) -> &ObjectCore {
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

impl ZwlrScreencopyFrameV1 {
    /// Since when the error.already_used enum variant is available.
    pub const ENM__ERROR_ALREADY_USED__SINCE: u32 = 1;
    /// Since when the error.invalid_buffer enum variant is available.
    pub const ENM__ERROR_INVALID_BUFFER__SINCE: u32 = 1;

    /// Since when the flags.y_invert enum variant is available.
    pub const ENM__FLAGS_Y_INVERT__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwlrScreencopyFrameV1Error(pub u32);

impl ZwlrScreencopyFrameV1Error {
    /// the object has already been used to copy a wl_buffer
    pub const ALREADY_USED: Self = Self(0);

    /// buffer attributes are invalid
    pub const INVALID_BUFFER: Self = Self(1);
}

impl Debug for ZwlrScreencopyFrameV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ALREADY_USED => "ALREADY_USED",
            Self::INVALID_BUFFER => "INVALID_BUFFER",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct ZwlrScreencopyFrameV1Flags(pub u32);

/// An iterator over the set bits in a [ZwlrScreencopyFrameV1Flags].
///
/// You can construct this with the `IntoIterator` implementation of `ZwlrScreencopyFrameV1Flags`.
#[derive(Clone, Debug)]
pub struct ZwlrScreencopyFrameV1FlagsIter(pub u32);

impl ZwlrScreencopyFrameV1Flags {
    /// contents are y-inverted
    pub const Y_INVERT: Self = Self(1);
}

impl ZwlrScreencopyFrameV1Flags {
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

impl Iterator for ZwlrScreencopyFrameV1FlagsIter {
    type Item = ZwlrScreencopyFrameV1Flags;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(ZwlrScreencopyFrameV1Flags(bit))
    }
}

impl IntoIterator for ZwlrScreencopyFrameV1Flags {
    type Item = ZwlrScreencopyFrameV1Flags;
    type IntoIter = ZwlrScreencopyFrameV1FlagsIter;

    fn into_iter(self) -> Self::IntoIter {
        ZwlrScreencopyFrameV1FlagsIter(self.0)
    }
}

impl BitAnd for ZwlrScreencopyFrameV1Flags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for ZwlrScreencopyFrameV1Flags {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for ZwlrScreencopyFrameV1Flags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for ZwlrScreencopyFrameV1Flags {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for ZwlrScreencopyFrameV1Flags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for ZwlrScreencopyFrameV1Flags {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for ZwlrScreencopyFrameV1Flags {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for ZwlrScreencopyFrameV1Flags {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for ZwlrScreencopyFrameV1Flags {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for ZwlrScreencopyFrameV1Flags {
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
            f.write_str("Y_INVERT")?;
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
