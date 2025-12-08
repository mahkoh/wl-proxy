//! a frame ready for copy
//!
//! This object represents a single frame.
//!
//! When created, a series of buffer events will be sent, each representing a
//! supported buffer type. The "buffer_done" event is sent afterwards to
//! indicate that all supported buffer types have been enumerated. The client
//! will then be able to send a "copy" request. If the capture is successful,
//! the compositor will send a "flags" followed by a "ready" event.
//!
//! wl_shm buffers are always supported, ie. the "buffer" event is guaranteed to be sent.
//!
//! If the capture failed, the "failed" event is sent. This can happen anytime
//! before the "ready" event.
//!
//! Once either a "ready" or a "failed" event is received, the client should
//! destroy the frame.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A hyprland_toplevel_export_frame_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct HyprlandToplevelExportFrameV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn HyprlandToplevelExportFrameV1Handler>,
}

struct DefaultHandler;

impl HyprlandToplevelExportFrameV1Handler for DefaultHandler { }

impl ConcreteObject for HyprlandToplevelExportFrameV1 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::HyprlandToplevelExportFrameV1;
    const INTERFACE_NAME: &str = "hyprland_toplevel_export_frame_v1";
}

impl HyprlandToplevelExportFrameV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl HyprlandToplevelExportFrameV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn HyprlandToplevelExportFrameV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for HyprlandToplevelExportFrameV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HyprlandToplevelExportFrameV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl HyprlandToplevelExportFrameV1 {
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
    pub fn try_send_buffer(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: WlShmFormat, arg1: u32, arg2: u32, arg3: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= hyprland_toplevel_export_frame_v1#{}.buffer(format: {:?}, width: {}, height: {}, stride: {})\n", client_id, id, arg0, arg1, arg2, arg3);
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
            0,
            arg0.0,
            arg1,
            arg2,
            arg3,
        ]);
        Ok(())
    }

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
    ) {
        let res = self.try_send_buffer(
            format,
            width,
            height,
            stride,
        );
        if let Err(e) = res {
            log_send("hyprland_toplevel_export_frame_v1.buffer", &e);
        }
    }

    /// Since when the copy message is available.
    pub const MSG__COPY__SINCE: u32 = 1;

    /// copy the frame
    ///
    /// Copy the frame to the supplied buffer. The buffer must have the
    /// correct size, see hyprland_toplevel_export_frame_v1.buffer and
    /// hyprland_toplevel_export_frame_v1.linux_dmabuf. The buffer needs to have a
    /// supported format.
    ///
    /// If the frame is successfully copied, a "flags" and a "ready" event is
    /// sent. Otherwise, a "failed" event is sent.
    ///
    /// This event will wait for appropriate damage to be copied, unless the ignore_damage
    /// arg is set to a non-zero value.
    ///
    /// # Arguments
    ///
    /// - `buffer`:
    /// - `ignore_damage`:
    #[inline]
    pub fn try_send_copy(
        &self,
        buffer: &Rc<WlBuffer>,
        ignore_damage: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            buffer,
            ignore_damage,
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
            fn log(state: &State, id: u32, arg0: u32, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_toplevel_export_frame_v1#{}.copy(buffer: wl_buffer#{}, ignore_damage: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            arg1 as u32,
        ]);
        Ok(())
    }

    /// copy the frame
    ///
    /// Copy the frame to the supplied buffer. The buffer must have the
    /// correct size, see hyprland_toplevel_export_frame_v1.buffer and
    /// hyprland_toplevel_export_frame_v1.linux_dmabuf. The buffer needs to have a
    /// supported format.
    ///
    /// If the frame is successfully copied, a "flags" and a "ready" event is
    /// sent. Otherwise, a "failed" event is sent.
    ///
    /// This event will wait for appropriate damage to be copied, unless the ignore_damage
    /// arg is set to a non-zero value.
    ///
    /// # Arguments
    ///
    /// - `buffer`:
    /// - `ignore_damage`:
    #[inline]
    pub fn send_copy(
        &self,
        buffer: &Rc<WlBuffer>,
        ignore_damage: i32,
    ) {
        let res = self.try_send_copy(
            buffer,
            ignore_damage,
        );
        if let Err(e) = res {
            log_send("hyprland_toplevel_export_frame_v1.copy", &e);
        }
    }

    /// Since when the damage message is available.
    pub const MSG__DAMAGE__SINCE: u32 = 1;

    /// carries the coordinates of the damaged region
    ///
    /// This event is sent right before the ready event when ignore_damage was
    /// not set. It may be generated multiple times for each copy
    /// request.
    ///
    /// The arguments describe a box around an area that has changed since the
    /// last copy request that was derived from the current screencopy manager
    /// instance.
    ///
    /// The union of all regions received between the call to copy
    /// and a ready event is the total damage since the prior ready event.
    ///
    /// # Arguments
    ///
    /// - `x`: damaged x coordinates
    /// - `y`: damaged y coordinates
    /// - `width`: current width
    /// - `height`: current height
    #[inline]
    pub fn try_send_damage(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= hyprland_toplevel_export_frame_v1#{}.damage(x: {}, y: {}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2, arg3);
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
            arg0,
            arg1,
            arg2,
            arg3,
        ]);
        Ok(())
    }

    /// carries the coordinates of the damaged region
    ///
    /// This event is sent right before the ready event when ignore_damage was
    /// not set. It may be generated multiple times for each copy
    /// request.
    ///
    /// The arguments describe a box around an area that has changed since the
    /// last copy request that was derived from the current screencopy manager
    /// instance.
    ///
    /// The union of all regions received between the call to copy
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
    ) {
        let res = self.try_send_damage(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_send("hyprland_toplevel_export_frame_v1.damage", &e);
        }
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
    pub fn try_send_flags(
        &self,
        flags: HyprlandToplevelExportFrameV1Flags,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: HyprlandToplevelExportFrameV1Flags) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= hyprland_toplevel_export_frame_v1#{}.flags(flags: {:?})\n", client_id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
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
    pub fn send_flags(
        &self,
        flags: HyprlandToplevelExportFrameV1Flags,
    ) {
        let res = self.try_send_flags(
            flags,
        );
        if let Err(e) = res {
            log_send("hyprland_toplevel_export_frame_v1.flags", &e);
        }
    }

    /// Since when the ready message is available.
    pub const MSG__READY__SINCE: u32 = 1;

    /// indicates frame is available for reading
    ///
    /// Called as soon as the frame is copied, indicating it is available
    /// for reading. This event includes the time at which presentation happened
    /// at.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= hyprland_toplevel_export_frame_v1#{}.ready(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {})\n", client_id, id, arg0, arg1, arg2);
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
            3,
            arg0,
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// indicates frame is available for reading
    ///
    /// Called as soon as the frame is copied, indicating it is available
    /// for reading. This event includes the time at which presentation happened
    /// at.
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
    ) {
        let res = self.try_send_ready(
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
        );
        if let Err(e) = res {
            log_send("hyprland_toplevel_export_frame_v1.ready", &e);
        }
    }

    /// Since when the failed message is available.
    pub const MSG__FAILED__SINCE: u32 = 1;

    /// frame copy failed
    ///
    /// This event indicates that the attempted frame copy has failed.
    ///
    /// After receiving this event, the client should destroy the object.
    #[inline]
    pub fn try_send_failed(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= hyprland_toplevel_export_frame_v1#{}.failed()\n", client_id, id);
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

    /// frame copy failed
    ///
    /// This event indicates that the attempted frame copy has failed.
    ///
    /// After receiving this event, the client should destroy the object.
    #[inline]
    pub fn send_failed(
        &self,
    ) {
        let res = self.try_send_failed(
        );
        if let Err(e) = res {
            log_send("hyprland_toplevel_export_frame_v1.failed", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// delete this object, used or not
    ///
    /// Destroys the frame. This request can be sent at any time by the client.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_toplevel_export_frame_v1#{}.destroy()\n", id);
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

    /// delete this object, used or not
    ///
    /// Destroys the frame. This request can be sent at any time by the client.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("hyprland_toplevel_export_frame_v1.destroy", &e);
        }
    }

    /// Since when the linux_dmabuf message is available.
    pub const MSG__LINUX_DMABUF__SINCE: u32 = 1;

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
    pub fn try_send_linux_dmabuf(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= hyprland_toplevel_export_frame_v1#{}.linux_dmabuf(format: {}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2);
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
            5,
            arg0,
            arg1,
            arg2,
        ]);
        Ok(())
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
    pub fn send_linux_dmabuf(
        &self,
        format: u32,
        width: u32,
        height: u32,
    ) {
        let res = self.try_send_linux_dmabuf(
            format,
            width,
            height,
        );
        if let Err(e) = res {
            log_send("hyprland_toplevel_export_frame_v1.linux_dmabuf", &e);
        }
    }

    /// Since when the buffer_done message is available.
    pub const MSG__BUFFER_DONE__SINCE: u32 = 1;

    /// all buffer types reported
    ///
    /// This event is sent once after all buffer events have been sent.
    ///
    /// The client should proceed to create a buffer of one of the supported
    /// types, and send a "copy" request.
    #[inline]
    pub fn try_send_buffer_done(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= hyprland_toplevel_export_frame_v1#{}.buffer_done()\n", client_id, id);
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
            6,
        ]);
        Ok(())
    }

    /// all buffer types reported
    ///
    /// This event is sent once after all buffer events have been sent.
    ///
    /// The client should proceed to create a buffer of one of the supported
    /// types, and send a "copy" request.
    #[inline]
    pub fn send_buffer_done(
        &self,
    ) {
        let res = self.try_send_buffer_done(
        );
        if let Err(e) = res {
            log_send("hyprland_toplevel_export_frame_v1.buffer_done", &e);
        }
    }
}

/// A message handler for [`HyprlandToplevelExportFrameV1`] proxies.
pub trait HyprlandToplevelExportFrameV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<HyprlandToplevelExportFrameV1>) {
        slf.core.delete_id();
    }

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
    fn handle_buffer(
        &mut self,
        slf: &Rc<HyprlandToplevelExportFrameV1>,
        format: WlShmFormat,
        width: u32,
        height: u32,
        stride: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_buffer(
            format,
            width,
            height,
            stride,
        );
        if let Err(e) = res {
            log_forward("hyprland_toplevel_export_frame_v1.buffer", &e);
        }
    }

    /// copy the frame
    ///
    /// Copy the frame to the supplied buffer. The buffer must have the
    /// correct size, see hyprland_toplevel_export_frame_v1.buffer and
    /// hyprland_toplevel_export_frame_v1.linux_dmabuf. The buffer needs to have a
    /// supported format.
    ///
    /// If the frame is successfully copied, a "flags" and a "ready" event is
    /// sent. Otherwise, a "failed" event is sent.
    ///
    /// This event will wait for appropriate damage to be copied, unless the ignore_damage
    /// arg is set to a non-zero value.
    ///
    /// # Arguments
    ///
    /// - `buffer`:
    /// - `ignore_damage`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_copy(
        &mut self,
        slf: &Rc<HyprlandToplevelExportFrameV1>,
        buffer: &Rc<WlBuffer>,
        ignore_damage: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_copy(
            buffer,
            ignore_damage,
        );
        if let Err(e) = res {
            log_forward("hyprland_toplevel_export_frame_v1.copy", &e);
        }
    }

    /// carries the coordinates of the damaged region
    ///
    /// This event is sent right before the ready event when ignore_damage was
    /// not set. It may be generated multiple times for each copy
    /// request.
    ///
    /// The arguments describe a box around an area that has changed since the
    /// last copy request that was derived from the current screencopy manager
    /// instance.
    ///
    /// The union of all regions received between the call to copy
    /// and a ready event is the total damage since the prior ready event.
    ///
    /// # Arguments
    ///
    /// - `x`: damaged x coordinates
    /// - `y`: damaged y coordinates
    /// - `width`: current width
    /// - `height`: current height
    #[inline]
    fn handle_damage(
        &mut self,
        slf: &Rc<HyprlandToplevelExportFrameV1>,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
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
            log_forward("hyprland_toplevel_export_frame_v1.damage", &e);
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
    fn handle_flags(
        &mut self,
        slf: &Rc<HyprlandToplevelExportFrameV1>,
        flags: HyprlandToplevelExportFrameV1Flags,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_flags(
            flags,
        );
        if let Err(e) = res {
            log_forward("hyprland_toplevel_export_frame_v1.flags", &e);
        }
    }

    /// indicates frame is available for reading
    ///
    /// Called as soon as the frame is copied, indicating it is available
    /// for reading. This event includes the time at which presentation happened
    /// at.
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
    fn handle_ready(
        &mut self,
        slf: &Rc<HyprlandToplevelExportFrameV1>,
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
            log_forward("hyprland_toplevel_export_frame_v1.ready", &e);
        }
    }

    /// frame copy failed
    ///
    /// This event indicates that the attempted frame copy has failed.
    ///
    /// After receiving this event, the client should destroy the object.
    #[inline]
    fn handle_failed(
        &mut self,
        slf: &Rc<HyprlandToplevelExportFrameV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_failed(
        );
        if let Err(e) = res {
            log_forward("hyprland_toplevel_export_frame_v1.failed", &e);
        }
    }

    /// delete this object, used or not
    ///
    /// Destroys the frame. This request can be sent at any time by the client.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<HyprlandToplevelExportFrameV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("hyprland_toplevel_export_frame_v1.destroy", &e);
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
    fn handle_linux_dmabuf(
        &mut self,
        slf: &Rc<HyprlandToplevelExportFrameV1>,
        format: u32,
        width: u32,
        height: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_linux_dmabuf(
            format,
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("hyprland_toplevel_export_frame_v1.linux_dmabuf", &e);
        }
    }

    /// all buffer types reported
    ///
    /// This event is sent once after all buffer events have been sent.
    ///
    /// The client should proceed to create a buffer of one of the supported
    /// types, and send a "copy" request.
    #[inline]
    fn handle_buffer_done(
        &mut self,
        slf: &Rc<HyprlandToplevelExportFrameV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_buffer_done(
        );
        if let Err(e) = res {
            log_forward("hyprland_toplevel_export_frame_v1.buffer_done", &e);
        }
    }
}

impl ObjectPrivate for HyprlandToplevelExportFrameV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::HyprlandToplevelExportFrameV1, version),
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
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_toplevel_export_frame_v1#{}.copy(buffer: wl_buffer#{}, ignore_damage: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
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
                    (**handler).handle_copy(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_copy(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_toplevel_export_frame_v1#{}.destroy()\n", client_id, id);
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 24)));
                };
                let arg0 = WlShmFormat(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: WlShmFormat, arg1: u32, arg2: u32, arg3: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> hyprland_toplevel_export_frame_v1#{}.buffer(format: {:?}, width: {}, height: {}, stride: {})\n", id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_buffer(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_buffer(&self, arg0, arg1, arg2, arg3);
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
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> hyprland_toplevel_export_frame_v1#{}.damage(x: {}, y: {}, width: {}, height: {})\n", id, arg0, arg1, arg2, arg3);
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = HyprlandToplevelExportFrameV1Flags(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: HyprlandToplevelExportFrameV1Flags) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> hyprland_toplevel_export_frame_v1#{}.flags(flags: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_flags(&self, arg0);
                } else {
                    DefaultHandler.handle_flags(&self, arg0);
                }
            }
            3 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> hyprland_toplevel_export_frame_v1#{}.ready(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {})\n", id, arg0, arg1, arg2);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> hyprland_toplevel_export_frame_v1#{}.failed()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_failed(&self);
                } else {
                    DefaultHandler.handle_failed(&self);
                }
            }
            5 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> hyprland_toplevel_export_frame_v1#{}.linux_dmabuf(format: {}, width: {}, height: {})\n", id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_linux_dmabuf(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_linux_dmabuf(&self, arg0, arg1, arg2);
                }
            }
            6 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> hyprland_toplevel_export_frame_v1#{}.buffer_done()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_buffer_done(&self);
                } else {
                    DefaultHandler.handle_buffer_done(&self);
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
            0 => "copy",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "buffer",
            1 => "damage",
            2 => "flags",
            3 => "ready",
            4 => "failed",
            5 => "linux_dmabuf",
            6 => "buffer_done",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for HyprlandToplevelExportFrameV1 {
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

impl HyprlandToplevelExportFrameV1 {
    /// Since when the error.already_used enum variant is available.
    pub const ENM__ERROR_ALREADY_USED__SINCE: u32 = 1;
    /// Since when the error.invalid_buffer enum variant is available.
    pub const ENM__ERROR_INVALID_BUFFER__SINCE: u32 = 1;

    /// Since when the flags.y_invert enum variant is available.
    pub const ENM__FLAGS_Y_INVERT__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HyprlandToplevelExportFrameV1Error(pub u32);

impl HyprlandToplevelExportFrameV1Error {
    /// the object has already been used to copy a wl_buffer
    pub const ALREADY_USED: Self = Self(0);

    /// buffer attributes are invalid
    pub const INVALID_BUFFER: Self = Self(1);
}

impl Debug for HyprlandToplevelExportFrameV1Error {
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
pub struct HyprlandToplevelExportFrameV1Flags(pub u32);

/// An iterator over the set bits in a [`HyprlandToplevelExportFrameV1Flags`].
///
/// You can construct this with the `IntoIterator` implementation of `HyprlandToplevelExportFrameV1Flags`.
#[derive(Clone, Debug)]
pub struct HyprlandToplevelExportFrameV1FlagsIter(pub u32);

impl HyprlandToplevelExportFrameV1Flags {
    /// contents are y-inverted
    pub const Y_INVERT: Self = Self(1);
}

impl HyprlandToplevelExportFrameV1Flags {
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

impl Iterator for HyprlandToplevelExportFrameV1FlagsIter {
    type Item = HyprlandToplevelExportFrameV1Flags;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(HyprlandToplevelExportFrameV1Flags(bit))
    }
}

impl IntoIterator for HyprlandToplevelExportFrameV1Flags {
    type Item = HyprlandToplevelExportFrameV1Flags;
    type IntoIter = HyprlandToplevelExportFrameV1FlagsIter;

    fn into_iter(self) -> Self::IntoIter {
        HyprlandToplevelExportFrameV1FlagsIter(self.0)
    }
}

impl BitAnd for HyprlandToplevelExportFrameV1Flags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for HyprlandToplevelExportFrameV1Flags {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for HyprlandToplevelExportFrameV1Flags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for HyprlandToplevelExportFrameV1Flags {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for HyprlandToplevelExportFrameV1Flags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for HyprlandToplevelExportFrameV1Flags {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for HyprlandToplevelExportFrameV1Flags {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for HyprlandToplevelExportFrameV1Flags {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for HyprlandToplevelExportFrameV1Flags {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for HyprlandToplevelExportFrameV1Flags {
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
