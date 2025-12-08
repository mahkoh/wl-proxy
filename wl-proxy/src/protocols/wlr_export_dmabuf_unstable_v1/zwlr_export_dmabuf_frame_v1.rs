//! a DMA-BUF frame
//!
//! This object represents a single DMA-BUF frame.
//!
//! If the capture is successful, the compositor will first send a "frame"
//! event, followed by one or several "object". When the frame is available
//! for readout, the "ready" event is sent.
//!
//! If the capture failed, the "cancel" event is sent. This can happen anytime
//! before the "ready" event.
//!
//! Once either a "ready" or a "cancel" event is received, the client should
//! destroy the frame. Once an "object" event is received, the client is
//! responsible for closing the associated file descriptor.
//!
//! All frames are read-only and may not be written into or altered.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwlr_export_dmabuf_frame_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwlrExportDmabufFrameV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwlrExportDmabufFrameV1Handler>,
}

struct DefaultHandler;

impl ZwlrExportDmabufFrameV1Handler for DefaultHandler { }

impl ConcreteObject for ZwlrExportDmabufFrameV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwlrExportDmabufFrameV1;
    const INTERFACE_NAME: &str = "zwlr_export_dmabuf_frame_v1";
}

impl ZwlrExportDmabufFrameV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwlrExportDmabufFrameV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwlrExportDmabufFrameV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwlrExportDmabufFrameV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwlrExportDmabufFrameV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwlrExportDmabufFrameV1 {
    /// Since when the frame message is available.
    pub const MSG__FRAME__SINCE: u32 = 1;

    /// a frame description
    ///
    /// Main event supplying the client with information about the frame. If the
    /// capture didn't fail, this event is always emitted first before any other
    /// events.
    ///
    /// This event is followed by a number of "object" as specified by the
    /// "num_objects" argument.
    ///
    /// # Arguments
    ///
    /// - `width`: frame width in pixels
    /// - `height`: frame height in pixels
    /// - `offset_x`: crop offset for the x axis
    /// - `offset_y`: crop offset for the y axis
    /// - `buffer_flags`: flags which indicate properties (invert, interlacing),
    ///                                       has the same values as zwp_linux_buffer_params_v1:flags
    /// - `flags`: indicates special frame features
    /// - `format`: format of the frame (DRM_FORMAT_*)
    /// - `mod_high`: drm format modifier, high
    /// - `mod_low`: drm format modifier, low
    /// - `num_objects`: indicates how many objects (FDs) the frame has (max 4)
    #[inline]
    pub fn try_send_frame(
        &self,
        width: u32,
        height: u32,
        offset_x: u32,
        offset_y: u32,
        buffer_flags: u32,
        flags: ZwlrExportDmabufFrameV1Flags,
        format: u32,
        mod_high: u32,
        mod_low: u32,
        num_objects: u32,
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
            arg8,
            arg9,
        ) = (
            width,
            height,
            offset_x,
            offset_y,
            buffer_flags,
            flags,
            format,
            mod_high,
            mod_low,
            num_objects,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: ZwlrExportDmabufFrameV1Flags, arg6: u32, arg7: u32, arg8: u32, arg9: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_export_dmabuf_frame_v1#{}.frame(width: {}, height: {}, offset_x: {}, offset_y: {}, buffer_flags: {}, flags: {:?}, format: {}, mod_high: {}, mod_low: {}, num_objects: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
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
            arg2,
            arg3,
            arg4,
            arg5.0,
            arg6,
            arg7,
            arg8,
            arg9,
        ]);
        Ok(())
    }

    /// a frame description
    ///
    /// Main event supplying the client with information about the frame. If the
    /// capture didn't fail, this event is always emitted first before any other
    /// events.
    ///
    /// This event is followed by a number of "object" as specified by the
    /// "num_objects" argument.
    ///
    /// # Arguments
    ///
    /// - `width`: frame width in pixels
    /// - `height`: frame height in pixels
    /// - `offset_x`: crop offset for the x axis
    /// - `offset_y`: crop offset for the y axis
    /// - `buffer_flags`: flags which indicate properties (invert, interlacing),
    ///                                       has the same values as zwp_linux_buffer_params_v1:flags
    /// - `flags`: indicates special frame features
    /// - `format`: format of the frame (DRM_FORMAT_*)
    /// - `mod_high`: drm format modifier, high
    /// - `mod_low`: drm format modifier, low
    /// - `num_objects`: indicates how many objects (FDs) the frame has (max 4)
    #[inline]
    pub fn send_frame(
        &self,
        width: u32,
        height: u32,
        offset_x: u32,
        offset_y: u32,
        buffer_flags: u32,
        flags: ZwlrExportDmabufFrameV1Flags,
        format: u32,
        mod_high: u32,
        mod_low: u32,
        num_objects: u32,
    ) {
        let res = self.try_send_frame(
            width,
            height,
            offset_x,
            offset_y,
            buffer_flags,
            flags,
            format,
            mod_high,
            mod_low,
            num_objects,
        );
        if let Err(e) = res {
            log_send("zwlr_export_dmabuf_frame_v1.frame", &e);
        }
    }

    /// Since when the object message is available.
    pub const MSG__OBJECT__SINCE: u32 = 1;

    /// an object description
    ///
    /// Event which serves to supply the client with the file descriptors
    /// containing the data for each object.
    ///
    /// After receiving this event, the client must always close the file
    /// descriptor as soon as they're done with it and even if the frame fails.
    ///
    /// # Arguments
    ///
    /// - `index`: index of the current object
    /// - `fd`: fd of the current object
    /// - `size`: size in bytes for the current object
    /// - `offset`: starting point for the data in the object's fd
    /// - `stride`: line size in bytes
    /// - `plane_index`: index of the plane the data in the object applies to
    #[inline]
    pub fn try_send_object(
        &self,
        index: u32,
        fd: &Rc<OwnedFd>,
        size: u32,
        offset: u32,
        stride: u32,
        plane_index: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
        ) = (
            index,
            fd,
            size,
            offset,
            stride,
            plane_index,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: i32, arg2: u32, arg3: u32, arg4: u32, arg5: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_export_dmabuf_frame_v1#{}.object(index: {}, fd: {}, size: {}, offset: {}, stride: {}, plane_index: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4, arg5);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1.as_raw_fd(), arg2, arg3, arg4, arg5);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.fds.push_back(arg1.clone());
        fmt.words([
            id,
            1,
            arg0,
            arg2,
            arg3,
            arg4,
            arg5,
        ]);
        Ok(())
    }

    /// an object description
    ///
    /// Event which serves to supply the client with the file descriptors
    /// containing the data for each object.
    ///
    /// After receiving this event, the client must always close the file
    /// descriptor as soon as they're done with it and even if the frame fails.
    ///
    /// # Arguments
    ///
    /// - `index`: index of the current object
    /// - `fd`: fd of the current object
    /// - `size`: size in bytes for the current object
    /// - `offset`: starting point for the data in the object's fd
    /// - `stride`: line size in bytes
    /// - `plane_index`: index of the plane the data in the object applies to
    #[inline]
    pub fn send_object(
        &self,
        index: u32,
        fd: &Rc<OwnedFd>,
        size: u32,
        offset: u32,
        stride: u32,
        plane_index: u32,
    ) {
        let res = self.try_send_object(
            index,
            fd,
            size,
            offset,
            stride,
            plane_index,
        );
        if let Err(e) = res {
            log_send("zwlr_export_dmabuf_frame_v1.object", &e);
        }
    }

    /// Since when the ready message is available.
    pub const MSG__READY__SINCE: u32 = 1;

    /// indicates frame is available for reading
    ///
    /// This event is sent as soon as the frame is presented, indicating it is
    /// available for reading. This event includes the time at which
    /// presentation happened at.
    ///
    /// The timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,
    /// each component being an unsigned 32-bit value. Whole seconds are in
    /// tv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,
    /// and the additional fractional part in tv_nsec as nanoseconds. Hence,
    /// for valid timestamps tv_nsec must be in [0, 999999999]. The seconds part
    /// may have an arbitrary offset at start.
    ///
    /// After receiving this event, the client should destroy this object.
    ///
    /// # Arguments
    ///
    /// - `tv_sec_hi`: high 32 bits of the seconds part of the timestamp
    /// - `tv_sec_lo`: low 32 bits of the seconds part of the timestamp
    /// - `tv_nsec`: nanoseconds part of the timestamp
    #[inline]
    pub fn try_send_ready(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_export_dmabuf_frame_v1#{}.ready(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {})\n", client_id, id, arg0, arg1, arg2);
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

    /// indicates frame is available for reading
    ///
    /// This event is sent as soon as the frame is presented, indicating it is
    /// available for reading. This event includes the time at which
    /// presentation happened at.
    ///
    /// The timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,
    /// each component being an unsigned 32-bit value. Whole seconds are in
    /// tv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,
    /// and the additional fractional part in tv_nsec as nanoseconds. Hence,
    /// for valid timestamps tv_nsec must be in [0, 999999999]. The seconds part
    /// may have an arbitrary offset at start.
    ///
    /// After receiving this event, the client should destroy this object.
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
    ) {
        let res = self.try_send_ready(
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
        );
        if let Err(e) = res {
            log_send("zwlr_export_dmabuf_frame_v1.ready", &e);
        }
    }

    /// Since when the cancel message is available.
    pub const MSG__CANCEL__SINCE: u32 = 1;

    /// indicates the frame is no longer valid
    ///
    /// If the capture failed or if the frame is no longer valid after the
    /// "frame" event has been emitted, this event will be used to inform the
    /// client to scrap the frame.
    ///
    /// If the failure is temporary, the client may capture again the same
    /// source. If the failure is permanent, any further attempts to capture the
    /// same source will fail again.
    ///
    /// After receiving this event, the client should destroy this object.
    ///
    /// # Arguments
    ///
    /// - `reason`: indicates a reason for cancelling this frame capture
    #[inline]
    pub fn try_send_cancel(
        &self,
        reason: ZwlrExportDmabufFrameV1CancelReason,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: ZwlrExportDmabufFrameV1CancelReason) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_export_dmabuf_frame_v1#{}.cancel(reason: {:?})\n", client_id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// indicates the frame is no longer valid
    ///
    /// If the capture failed or if the frame is no longer valid after the
    /// "frame" event has been emitted, this event will be used to inform the
    /// client to scrap the frame.
    ///
    /// If the failure is temporary, the client may capture again the same
    /// source. If the failure is permanent, any further attempts to capture the
    /// same source will fail again.
    ///
    /// After receiving this event, the client should destroy this object.
    ///
    /// # Arguments
    ///
    /// - `reason`: indicates a reason for cancelling this frame capture
    #[inline]
    pub fn send_cancel(
        &self,
        reason: ZwlrExportDmabufFrameV1CancelReason,
    ) {
        let res = self.try_send_cancel(
            reason,
        );
        if let Err(e) = res {
            log_send("zwlr_export_dmabuf_frame_v1.cancel", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// delete this object, used or not
    ///
    /// Unreferences the frame. This request must be called as soon as its no
    /// longer used.
    ///
    /// It can be called at any time by the client. The client will still have
    /// to close any FDs it has been given.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_export_dmabuf_frame_v1#{}.destroy()\n", id);
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

    /// delete this object, used or not
    ///
    /// Unreferences the frame. This request must be called as soon as its no
    /// longer used.
    ///
    /// It can be called at any time by the client. The client will still have
    /// to close any FDs it has been given.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwlr_export_dmabuf_frame_v1.destroy", &e);
        }
    }
}

/// A message handler for [`ZwlrExportDmabufFrameV1`] proxies.
pub trait ZwlrExportDmabufFrameV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwlrExportDmabufFrameV1>) {
        slf.core.delete_id();
    }

    /// a frame description
    ///
    /// Main event supplying the client with information about the frame. If the
    /// capture didn't fail, this event is always emitted first before any other
    /// events.
    ///
    /// This event is followed by a number of "object" as specified by the
    /// "num_objects" argument.
    ///
    /// # Arguments
    ///
    /// - `width`: frame width in pixels
    /// - `height`: frame height in pixels
    /// - `offset_x`: crop offset for the x axis
    /// - `offset_y`: crop offset for the y axis
    /// - `buffer_flags`: flags which indicate properties (invert, interlacing),
    ///                                       has the same values as zwp_linux_buffer_params_v1:flags
    /// - `flags`: indicates special frame features
    /// - `format`: format of the frame (DRM_FORMAT_*)
    /// - `mod_high`: drm format modifier, high
    /// - `mod_low`: drm format modifier, low
    /// - `num_objects`: indicates how many objects (FDs) the frame has (max 4)
    #[inline]
    fn handle_frame(
        &mut self,
        slf: &Rc<ZwlrExportDmabufFrameV1>,
        width: u32,
        height: u32,
        offset_x: u32,
        offset_y: u32,
        buffer_flags: u32,
        flags: ZwlrExportDmabufFrameV1Flags,
        format: u32,
        mod_high: u32,
        mod_low: u32,
        num_objects: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_frame(
            width,
            height,
            offset_x,
            offset_y,
            buffer_flags,
            flags,
            format,
            mod_high,
            mod_low,
            num_objects,
        );
        if let Err(e) = res {
            log_forward("zwlr_export_dmabuf_frame_v1.frame", &e);
        }
    }

    /// an object description
    ///
    /// Event which serves to supply the client with the file descriptors
    /// containing the data for each object.
    ///
    /// After receiving this event, the client must always close the file
    /// descriptor as soon as they're done with it and even if the frame fails.
    ///
    /// # Arguments
    ///
    /// - `index`: index of the current object
    /// - `fd`: fd of the current object
    /// - `size`: size in bytes for the current object
    /// - `offset`: starting point for the data in the object's fd
    /// - `stride`: line size in bytes
    /// - `plane_index`: index of the plane the data in the object applies to
    #[inline]
    fn handle_object(
        &mut self,
        slf: &Rc<ZwlrExportDmabufFrameV1>,
        index: u32,
        fd: &Rc<OwnedFd>,
        size: u32,
        offset: u32,
        stride: u32,
        plane_index: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_object(
            index,
            fd,
            size,
            offset,
            stride,
            plane_index,
        );
        if let Err(e) = res {
            log_forward("zwlr_export_dmabuf_frame_v1.object", &e);
        }
    }

    /// indicates frame is available for reading
    ///
    /// This event is sent as soon as the frame is presented, indicating it is
    /// available for reading. This event includes the time at which
    /// presentation happened at.
    ///
    /// The timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,
    /// each component being an unsigned 32-bit value. Whole seconds are in
    /// tv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,
    /// and the additional fractional part in tv_nsec as nanoseconds. Hence,
    /// for valid timestamps tv_nsec must be in [0, 999999999]. The seconds part
    /// may have an arbitrary offset at start.
    ///
    /// After receiving this event, the client should destroy this object.
    ///
    /// # Arguments
    ///
    /// - `tv_sec_hi`: high 32 bits of the seconds part of the timestamp
    /// - `tv_sec_lo`: low 32 bits of the seconds part of the timestamp
    /// - `tv_nsec`: nanoseconds part of the timestamp
    #[inline]
    fn handle_ready(
        &mut self,
        slf: &Rc<ZwlrExportDmabufFrameV1>,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_ready(
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
        );
        if let Err(e) = res {
            log_forward("zwlr_export_dmabuf_frame_v1.ready", &e);
        }
    }

    /// indicates the frame is no longer valid
    ///
    /// If the capture failed or if the frame is no longer valid after the
    /// "frame" event has been emitted, this event will be used to inform the
    /// client to scrap the frame.
    ///
    /// If the failure is temporary, the client may capture again the same
    /// source. If the failure is permanent, any further attempts to capture the
    /// same source will fail again.
    ///
    /// After receiving this event, the client should destroy this object.
    ///
    /// # Arguments
    ///
    /// - `reason`: indicates a reason for cancelling this frame capture
    #[inline]
    fn handle_cancel(
        &mut self,
        slf: &Rc<ZwlrExportDmabufFrameV1>,
        reason: ZwlrExportDmabufFrameV1CancelReason,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_cancel(
            reason,
        );
        if let Err(e) = res {
            log_forward("zwlr_export_dmabuf_frame_v1.cancel", &e);
        }
    }

    /// delete this object, used or not
    ///
    /// Unreferences the frame. This request must be called as soon as its no
    /// longer used.
    ///
    /// It can be called at any time by the client. The client will still have
    /// to close any FDs it has been given.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwlrExportDmabufFrameV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwlr_export_dmabuf_frame_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for ZwlrExportDmabufFrameV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwlrExportDmabufFrameV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_export_dmabuf_frame_v1#{}.destroy()\n", client_id, id);
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
                    arg2,
                    arg3,
                    arg4,
                    arg5,
                    arg6,
                    arg7,
                    arg8,
                    arg9,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 48)));
                };
                let arg5 = ZwlrExportDmabufFrameV1Flags(arg5);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: ZwlrExportDmabufFrameV1Flags, arg6: u32, arg7: u32, arg8: u32, arg9: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_export_dmabuf_frame_v1#{}.frame(width: {}, height: {}, offset_x: {}, offset_y: {}, buffer_flags: {}, flags: {:?}, format: {}, mod_high: {}, mod_low: {}, num_objects: {})\n", id, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
                }
                if let Some(handler) = handler {
                    (**handler).handle_frame(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
                } else {
                    DefaultHandler.handle_frame(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
                }
            }
            1 => {
                let [
                    arg0,
                    arg2,
                    arg3,
                    arg4,
                    arg5,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 28)));
                };
                let Some(arg1) = fds.pop_front() else {
                    return Err(ObjectError(ObjectErrorKind::MissingFd("fd")));
                };
                let arg1 = &arg1;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: i32, arg2: u32, arg3: u32, arg4: u32, arg5: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_export_dmabuf_frame_v1#{}.object(index: {}, fd: {}, size: {}, offset: {}, stride: {}, plane_index: {})\n", id, arg0, arg1, arg2, arg3, arg4, arg5);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1.as_raw_fd(), arg2, arg3, arg4, arg5);
                }
                if let Some(handler) = handler {
                    (**handler).handle_object(&self, arg0, arg1, arg2, arg3, arg4, arg5);
                } else {
                    DefaultHandler.handle_object(&self, arg0, arg1, arg2, arg3, arg4, arg5);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_export_dmabuf_frame_v1#{}.ready(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {})\n", id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_ready(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_ready(&self, arg0, arg1, arg2);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = ZwlrExportDmabufFrameV1CancelReason(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: ZwlrExportDmabufFrameV1CancelReason) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_export_dmabuf_frame_v1#{}.cancel(reason: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_cancel(&self, arg0);
                } else {
                    DefaultHandler.handle_cancel(&self, arg0);
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
            0 => "frame",
            1 => "object",
            2 => "ready",
            3 => "cancel",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwlrExportDmabufFrameV1 {
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

impl ZwlrExportDmabufFrameV1 {
    /// Since when the flags.transient enum variant is available.
    pub const ENM__FLAGS_TRANSIENT__SINCE: u32 = 1;

    /// Since when the cancel_reason.temporary enum variant is available.
    pub const ENM__CANCEL_REASON_TEMPORARY__SINCE: u32 = 1;
    /// Since when the cancel_reason.permanent enum variant is available.
    pub const ENM__CANCEL_REASON_PERMANENT__SINCE: u32 = 1;
    /// Since when the cancel_reason.resizing enum variant is available.
    pub const ENM__CANCEL_REASON_RESIZING__SINCE: u32 = 1;
}

/// frame flags
///
/// Special flags that should be respected by the client.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwlrExportDmabufFrameV1Flags(pub u32);

impl ZwlrExportDmabufFrameV1Flags {
    /// clients should copy frame before processing
    pub const TRANSIENT: Self = Self(0x1);
}

impl Debug for ZwlrExportDmabufFrameV1Flags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::TRANSIENT => "TRANSIENT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// cancel reason
///
/// Indicates reason for cancelling the frame.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwlrExportDmabufFrameV1CancelReason(pub u32);

impl ZwlrExportDmabufFrameV1CancelReason {
    /// temporary error, source will produce more frames
    pub const TEMPORARY: Self = Self(0);

    /// fatal error, source will not produce frames
    pub const PERMANENT: Self = Self(1);

    /// temporary error, source will produce more frames
    pub const RESIZING: Self = Self(2);
}

impl Debug for ZwlrExportDmabufFrameV1CancelReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::TEMPORARY => "TEMPORARY",
            Self::PERMANENT => "PERMANENT",
            Self::RESIZING => "RESIZING",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
