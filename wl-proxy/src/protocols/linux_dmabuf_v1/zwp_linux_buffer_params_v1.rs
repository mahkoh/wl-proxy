//! parameters for creating a dmabuf-based wl_buffer
//!
//! This temporary object is a collection of dmabufs and other
//! parameters that together form a single logical buffer. The temporary
//! object may eventually create one wl_buffer unless cancelled by
//! destroying it before requesting 'create'.
//!
//! Single-planar formats only require one dmabuf, however
//! multi-planar formats may require more than one dmabuf. For all
//! formats, an 'add' request must be called once per plane (even if the
//! underlying dmabuf fd is identical).
//!
//! You must use consecutive plane indices ('plane_idx' argument for 'add')
//! from zero to the number of planes used by the drm_fourcc format code.
//! All planes required by the format must be given exactly once, but can
//! be given in any order. Each plane index can only be set once; subsequent
//! calls with a plane index which has already been set will result in a
//! plane_set error being generated.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_linux_buffer_params_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpLinuxBufferParamsV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpLinuxBufferParamsV1Handler>,
}

struct DefaultHandler;

impl ZwpLinuxBufferParamsV1Handler for DefaultHandler { }

impl ConcreteObject for ZwpLinuxBufferParamsV1 {
    const XML_VERSION: u32 = 5;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpLinuxBufferParamsV1;
    const INTERFACE_NAME: &str = "zwp_linux_buffer_params_v1";
}

impl ZwpLinuxBufferParamsV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpLinuxBufferParamsV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpLinuxBufferParamsV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpLinuxBufferParamsV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpLinuxBufferParamsV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpLinuxBufferParamsV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// delete this object, used or not
    ///
    /// Cleans up the temporary data sent to the server for dmabuf-based
    /// wl_buffer creation.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_linux_buffer_params_v1#{}.destroy()\n", id);
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
    /// Cleans up the temporary data sent to the server for dmabuf-based
    /// wl_buffer creation.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwp_linux_buffer_params_v1.destroy", &e);
        }
    }

    /// Since when the add message is available.
    pub const MSG__ADD__SINCE: u32 = 1;

    /// add a dmabuf to the temporary set
    ///
    /// This request adds one dmabuf to the set in this
    /// zwp_linux_buffer_params_v1.
    ///
    /// The 64-bit unsigned value combined from modifier_hi and modifier_lo
    /// is the dmabuf layout modifier. DRM AddFB2 ioctl calls this the
    /// fb modifier, which is defined in drm_mode.h of Linux UAPI.
    /// This is an opaque token. Drivers use this token to express tiling,
    /// compression, etc. driver-specific modifications to the base format
    /// defined by the DRM fourcc code.
    ///
    /// Starting from version 4, the invalid_format protocol error is sent if
    /// the format + modifier pair was not advertised as supported.
    ///
    /// Starting from version 5, the invalid_format protocol error is sent if
    /// all planes don't use the same modifier.
    ///
    /// This request raises the PLANE_IDX error if plane_idx is too large.
    /// The error PLANE_SET is raised if attempting to set a plane that
    /// was already set.
    ///
    /// # Arguments
    ///
    /// - `fd`: dmabuf fd
    /// - `plane_idx`: plane index
    /// - `offset`: offset in bytes
    /// - `stride`: stride in bytes
    /// - `modifier_hi`: high 32 bits of layout modifier
    /// - `modifier_lo`: low 32 bits of layout modifier
    #[inline]
    pub fn try_send_add(
        &self,
        fd: &Rc<OwnedFd>,
        plane_idx: u32,
        offset: u32,
        stride: u32,
        modifier_hi: u32,
        modifier_lo: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
        ) = (
            fd,
            plane_idx,
            offset,
            stride,
            modifier_hi,
            modifier_lo,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_linux_buffer_params_v1#{}.add(fd: {}, plane_idx: {}, offset: {}, stride: {}, modifier_hi: {}, modifier_lo: {})\n", id, arg0, arg1, arg2, arg3, arg4, arg5);
                state.log(args);
            }
            log(&self.core.state, id, arg0.as_raw_fd(), arg1, arg2, arg3, arg4, arg5);
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
        fmt.fds.push_back(arg0.clone());
        fmt.words([
            id,
            1,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
        ]);
        Ok(())
    }

    /// add a dmabuf to the temporary set
    ///
    /// This request adds one dmabuf to the set in this
    /// zwp_linux_buffer_params_v1.
    ///
    /// The 64-bit unsigned value combined from modifier_hi and modifier_lo
    /// is the dmabuf layout modifier. DRM AddFB2 ioctl calls this the
    /// fb modifier, which is defined in drm_mode.h of Linux UAPI.
    /// This is an opaque token. Drivers use this token to express tiling,
    /// compression, etc. driver-specific modifications to the base format
    /// defined by the DRM fourcc code.
    ///
    /// Starting from version 4, the invalid_format protocol error is sent if
    /// the format + modifier pair was not advertised as supported.
    ///
    /// Starting from version 5, the invalid_format protocol error is sent if
    /// all planes don't use the same modifier.
    ///
    /// This request raises the PLANE_IDX error if plane_idx is too large.
    /// The error PLANE_SET is raised if attempting to set a plane that
    /// was already set.
    ///
    /// # Arguments
    ///
    /// - `fd`: dmabuf fd
    /// - `plane_idx`: plane index
    /// - `offset`: offset in bytes
    /// - `stride`: stride in bytes
    /// - `modifier_hi`: high 32 bits of layout modifier
    /// - `modifier_lo`: low 32 bits of layout modifier
    #[inline]
    pub fn send_add(
        &self,
        fd: &Rc<OwnedFd>,
        plane_idx: u32,
        offset: u32,
        stride: u32,
        modifier_hi: u32,
        modifier_lo: u32,
    ) {
        let res = self.try_send_add(
            fd,
            plane_idx,
            offset,
            stride,
            modifier_hi,
            modifier_lo,
        );
        if let Err(e) = res {
            log_send("zwp_linux_buffer_params_v1.add", &e);
        }
    }

    /// Since when the create message is available.
    pub const MSG__CREATE__SINCE: u32 = 1;

    /// create a wl_buffer from the given dmabufs
    ///
    /// This asks for creation of a wl_buffer from the added dmabuf
    /// buffers. The wl_buffer is not created immediately but returned via
    /// the 'created' event if the dmabuf sharing succeeds. The sharing
    /// may fail at runtime for reasons a client cannot predict, in
    /// which case the 'failed' event is triggered.
    ///
    /// The 'format' argument is a DRM_FORMAT code, as defined by the
    /// libdrm's drm_fourcc.h. The Linux kernel's DRM sub-system is the
    /// authoritative source on how the format codes should work.
    ///
    /// The 'flags' is a bitfield of the flags defined in enum "flags".
    /// 'y_invert' means the that the image needs to be y-flipped.
    ///
    /// Flag 'interlaced' means that the frame in the buffer is not
    /// progressive as usual, but interlaced. An interlaced buffer as
    /// supported here must always contain both top and bottom fields.
    /// The top field always begins on the first pixel row. The temporal
    /// ordering between the two fields is top field first, unless
    /// 'bottom_first' is specified. It is undefined whether 'bottom_first'
    /// is ignored if 'interlaced' is not set.
    ///
    /// This protocol does not convey any information about field rate,
    /// duration, or timing, other than the relative ordering between the
    /// two fields in one buffer. A compositor may have to estimate the
    /// intended field rate from the incoming buffer rate. It is undefined
    /// whether the time of receiving wl_surface.commit with a new buffer
    /// attached, applying the wl_surface state, wl_surface.frame callback
    /// trigger, presentation, or any other point in the compositor cycle
    /// is used to measure the frame or field times. There is no support
    /// for detecting missed or late frames/fields/buffers either, and
    /// there is no support whatsoever for cooperating with interlaced
    /// compositor output.
    ///
    /// The composited image quality resulting from the use of interlaced
    /// buffers is explicitly undefined. A compositor may use elaborate
    /// hardware features or software to deinterlace and create progressive
    /// output frames from a sequence of interlaced input buffers, or it
    /// may produce substandard image quality. However, compositors that
    /// cannot guarantee reasonable image quality in all cases are recommended
    /// to just reject all interlaced buffers.
    ///
    /// Any argument errors, including non-positive width or height,
    /// mismatch between the number of planes and the format, bad
    /// format, bad offset or stride, may be indicated by fatal protocol
    /// errors: INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS,
    /// OUT_OF_BOUNDS.
    ///
    /// Dmabuf import errors in the server that are not obvious client
    /// bugs are returned via the 'failed' event as non-fatal. This
    /// allows attempting dmabuf sharing and falling back in the client
    /// if it fails.
    ///
    /// This request can be sent only once in the object's lifetime, after
    /// which the only legal request is destroy. This object should be
    /// destroyed after issuing a 'create' request. Attempting to use this
    /// object after issuing 'create' raises ALREADY_USED protocol error.
    ///
    /// It is not mandatory to issue 'create'. If a client wants to
    /// cancel the buffer creation, it can just destroy this object.
    ///
    /// # Arguments
    ///
    /// - `width`: base plane width in pixels
    /// - `height`: base plane height in pixels
    /// - `format`: DRM_FORMAT code
    /// - `flags`: see enum flags
    #[inline]
    pub fn try_send_create(
        &self,
        width: i32,
        height: i32,
        format: u32,
        flags: ZwpLinuxBufferParamsV1Flags,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            width,
            height,
            format,
            flags,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: i32, arg2: u32, arg3: ZwpLinuxBufferParamsV1Flags) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_linux_buffer_params_v1#{}.create(width: {}, height: {}, format: {}, flags: {:?})\n", id, arg0, arg1, arg2, arg3);
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
            arg2,
            arg3.0,
        ]);
        Ok(())
    }

    /// create a wl_buffer from the given dmabufs
    ///
    /// This asks for creation of a wl_buffer from the added dmabuf
    /// buffers. The wl_buffer is not created immediately but returned via
    /// the 'created' event if the dmabuf sharing succeeds. The sharing
    /// may fail at runtime for reasons a client cannot predict, in
    /// which case the 'failed' event is triggered.
    ///
    /// The 'format' argument is a DRM_FORMAT code, as defined by the
    /// libdrm's drm_fourcc.h. The Linux kernel's DRM sub-system is the
    /// authoritative source on how the format codes should work.
    ///
    /// The 'flags' is a bitfield of the flags defined in enum "flags".
    /// 'y_invert' means the that the image needs to be y-flipped.
    ///
    /// Flag 'interlaced' means that the frame in the buffer is not
    /// progressive as usual, but interlaced. An interlaced buffer as
    /// supported here must always contain both top and bottom fields.
    /// The top field always begins on the first pixel row. The temporal
    /// ordering between the two fields is top field first, unless
    /// 'bottom_first' is specified. It is undefined whether 'bottom_first'
    /// is ignored if 'interlaced' is not set.
    ///
    /// This protocol does not convey any information about field rate,
    /// duration, or timing, other than the relative ordering between the
    /// two fields in one buffer. A compositor may have to estimate the
    /// intended field rate from the incoming buffer rate. It is undefined
    /// whether the time of receiving wl_surface.commit with a new buffer
    /// attached, applying the wl_surface state, wl_surface.frame callback
    /// trigger, presentation, or any other point in the compositor cycle
    /// is used to measure the frame or field times. There is no support
    /// for detecting missed or late frames/fields/buffers either, and
    /// there is no support whatsoever for cooperating with interlaced
    /// compositor output.
    ///
    /// The composited image quality resulting from the use of interlaced
    /// buffers is explicitly undefined. A compositor may use elaborate
    /// hardware features or software to deinterlace and create progressive
    /// output frames from a sequence of interlaced input buffers, or it
    /// may produce substandard image quality. However, compositors that
    /// cannot guarantee reasonable image quality in all cases are recommended
    /// to just reject all interlaced buffers.
    ///
    /// Any argument errors, including non-positive width or height,
    /// mismatch between the number of planes and the format, bad
    /// format, bad offset or stride, may be indicated by fatal protocol
    /// errors: INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS,
    /// OUT_OF_BOUNDS.
    ///
    /// Dmabuf import errors in the server that are not obvious client
    /// bugs are returned via the 'failed' event as non-fatal. This
    /// allows attempting dmabuf sharing and falling back in the client
    /// if it fails.
    ///
    /// This request can be sent only once in the object's lifetime, after
    /// which the only legal request is destroy. This object should be
    /// destroyed after issuing a 'create' request. Attempting to use this
    /// object after issuing 'create' raises ALREADY_USED protocol error.
    ///
    /// It is not mandatory to issue 'create'. If a client wants to
    /// cancel the buffer creation, it can just destroy this object.
    ///
    /// # Arguments
    ///
    /// - `width`: base plane width in pixels
    /// - `height`: base plane height in pixels
    /// - `format`: DRM_FORMAT code
    /// - `flags`: see enum flags
    #[inline]
    pub fn send_create(
        &self,
        width: i32,
        height: i32,
        format: u32,
        flags: ZwpLinuxBufferParamsV1Flags,
    ) {
        let res = self.try_send_create(
            width,
            height,
            format,
            flags,
        );
        if let Err(e) = res {
            log_send("zwp_linux_buffer_params_v1.create", &e);
        }
    }

    /// Since when the created message is available.
    pub const MSG__CREATED__SINCE: u32 = 1;

    /// buffer creation succeeded
    ///
    /// This event indicates that the attempted buffer creation was
    /// successful. It provides the new wl_buffer referencing the dmabuf(s).
    ///
    /// Upon receiving this event, the client should destroy the
    /// zwp_linux_buffer_params_v1 object.
    ///
    /// # Arguments
    ///
    /// - `buffer`: the newly created wl_buffer
    #[inline]
    pub fn try_send_created(
        &self,
        buffer: &Rc<WlBuffer>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            buffer,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateClientId("buffer", e)))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_linux_buffer_params_v1#{}.created(buffer: wl_buffer#{})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// buffer creation succeeded
    ///
    /// This event indicates that the attempted buffer creation was
    /// successful. It provides the new wl_buffer referencing the dmabuf(s).
    ///
    /// Upon receiving this event, the client should destroy the
    /// zwp_linux_buffer_params_v1 object.
    ///
    /// # Arguments
    ///
    /// - `buffer`: the newly created wl_buffer
    #[inline]
    pub fn send_created(
        &self,
        buffer: &Rc<WlBuffer>,
    ) {
        let res = self.try_send_created(
            buffer,
        );
        if let Err(e) = res {
            log_send("zwp_linux_buffer_params_v1.created", &e);
        }
    }

    /// buffer creation succeeded
    ///
    /// This event indicates that the attempted buffer creation was
    /// successful. It provides the new wl_buffer referencing the dmabuf(s).
    ///
    /// Upon receiving this event, the client should destroy the
    /// zwp_linux_buffer_params_v1 object.
    #[inline]
    pub fn new_try_send_created(
        &self,
    ) -> Result<Rc<WlBuffer>, ObjectError> {
        let buffer = self.core.create_child();
        self.try_send_created(
            &buffer,
        )?;
        Ok(buffer)
    }

    /// buffer creation succeeded
    ///
    /// This event indicates that the attempted buffer creation was
    /// successful. It provides the new wl_buffer referencing the dmabuf(s).
    ///
    /// Upon receiving this event, the client should destroy the
    /// zwp_linux_buffer_params_v1 object.
    #[inline]
    pub fn new_send_created(
        &self,
    ) -> Rc<WlBuffer> {
        let buffer = self.core.create_child();
        self.send_created(
            &buffer,
        );
        buffer
    }

    /// Since when the failed message is available.
    pub const MSG__FAILED__SINCE: u32 = 1;

    /// buffer creation failed
    ///
    /// This event indicates that the attempted buffer creation has
    /// failed. It usually means that one of the dmabuf constraints
    /// has not been fulfilled.
    ///
    /// Upon receiving this event, the client should destroy the
    /// zwp_linux_buffer_params_v1 object.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_linux_buffer_params_v1#{}.failed()\n", client_id, id);
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
            1,
        ]);
        Ok(())
    }

    /// buffer creation failed
    ///
    /// This event indicates that the attempted buffer creation has
    /// failed. It usually means that one of the dmabuf constraints
    /// has not been fulfilled.
    ///
    /// Upon receiving this event, the client should destroy the
    /// zwp_linux_buffer_params_v1 object.
    #[inline]
    pub fn send_failed(
        &self,
    ) {
        let res = self.try_send_failed(
        );
        if let Err(e) = res {
            log_send("zwp_linux_buffer_params_v1.failed", &e);
        }
    }

    /// Since when the create_immed message is available.
    pub const MSG__CREATE_IMMED__SINCE: u32 = 2;

    /// immediately create a wl_buffer from the given
    ///                      dmabufs
    ///
    /// This asks for immediate creation of a wl_buffer by importing the
    /// added dmabufs.
    ///
    /// In case of import success, no event is sent from the server, and the
    /// wl_buffer is ready to be used by the client.
    ///
    /// Upon import failure, either of the following may happen, as seen fit
    /// by the implementation:
    /// - the client is terminated with one of the following fatal protocol
    ///   errors:
    ///   - INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS, OUT_OF_BOUNDS,
    ///     in case of argument errors such as mismatch between the number
    ///     of planes and the format, bad format, non-positive width or
    ///     height, or bad offset or stride.
    ///   - INVALID_WL_BUFFER, in case the cause for failure is unknown or
    ///     platform specific.
    /// - the server creates an invalid wl_buffer, marks it as failed and
    ///   sends a 'failed' event to the client. The result of using this
    ///   invalid wl_buffer as an argument in any request by the client is
    ///   defined by the compositor implementation.
    ///
    /// This takes the same arguments as a 'create' request, and obeys the
    /// same restrictions.
    ///
    /// # Arguments
    ///
    /// - `buffer_id`: id for the newly created wl_buffer
    /// - `width`: base plane width in pixels
    /// - `height`: base plane height in pixels
    /// - `format`: DRM_FORMAT code
    /// - `flags`: see enum flags
    #[inline]
    pub fn try_send_create_immed(
        &self,
        buffer_id: &Rc<WlBuffer>,
        width: i32,
        height: i32,
        format: u32,
        flags: ZwpLinuxBufferParamsV1Flags,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ) = (
            buffer_id,
            width,
            height,
            format,
            flags,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("buffer_id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: i32, arg2: i32, arg3: u32, arg4: ZwpLinuxBufferParamsV1Flags) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_linux_buffer_params_v1#{}.create_immed(buffer_id: wl_buffer#{}, width: {}, height: {}, format: {}, flags: {:?})\n", id, arg0, arg1, arg2, arg3, arg4);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1, arg2, arg3, arg4);
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
            arg0_id,
            arg1 as u32,
            arg2 as u32,
            arg3,
            arg4.0,
        ]);
        Ok(())
    }

    /// immediately create a wl_buffer from the given
    ///                      dmabufs
    ///
    /// This asks for immediate creation of a wl_buffer by importing the
    /// added dmabufs.
    ///
    /// In case of import success, no event is sent from the server, and the
    /// wl_buffer is ready to be used by the client.
    ///
    /// Upon import failure, either of the following may happen, as seen fit
    /// by the implementation:
    /// - the client is terminated with one of the following fatal protocol
    ///   errors:
    ///   - INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS, OUT_OF_BOUNDS,
    ///     in case of argument errors such as mismatch between the number
    ///     of planes and the format, bad format, non-positive width or
    ///     height, or bad offset or stride.
    ///   - INVALID_WL_BUFFER, in case the cause for failure is unknown or
    ///     platform specific.
    /// - the server creates an invalid wl_buffer, marks it as failed and
    ///   sends a 'failed' event to the client. The result of using this
    ///   invalid wl_buffer as an argument in any request by the client is
    ///   defined by the compositor implementation.
    ///
    /// This takes the same arguments as a 'create' request, and obeys the
    /// same restrictions.
    ///
    /// # Arguments
    ///
    /// - `buffer_id`: id for the newly created wl_buffer
    /// - `width`: base plane width in pixels
    /// - `height`: base plane height in pixels
    /// - `format`: DRM_FORMAT code
    /// - `flags`: see enum flags
    #[inline]
    pub fn send_create_immed(
        &self,
        buffer_id: &Rc<WlBuffer>,
        width: i32,
        height: i32,
        format: u32,
        flags: ZwpLinuxBufferParamsV1Flags,
    ) {
        let res = self.try_send_create_immed(
            buffer_id,
            width,
            height,
            format,
            flags,
        );
        if let Err(e) = res {
            log_send("zwp_linux_buffer_params_v1.create_immed", &e);
        }
    }

    /// immediately create a wl_buffer from the given
    ///                      dmabufs
    ///
    /// This asks for immediate creation of a wl_buffer by importing the
    /// added dmabufs.
    ///
    /// In case of import success, no event is sent from the server, and the
    /// wl_buffer is ready to be used by the client.
    ///
    /// Upon import failure, either of the following may happen, as seen fit
    /// by the implementation:
    /// - the client is terminated with one of the following fatal protocol
    ///   errors:
    ///   - INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS, OUT_OF_BOUNDS,
    ///     in case of argument errors such as mismatch between the number
    ///     of planes and the format, bad format, non-positive width or
    ///     height, or bad offset or stride.
    ///   - INVALID_WL_BUFFER, in case the cause for failure is unknown or
    ///     platform specific.
    /// - the server creates an invalid wl_buffer, marks it as failed and
    ///   sends a 'failed' event to the client. The result of using this
    ///   invalid wl_buffer as an argument in any request by the client is
    ///   defined by the compositor implementation.
    ///
    /// This takes the same arguments as a 'create' request, and obeys the
    /// same restrictions.
    ///
    /// # Arguments
    ///
    /// - `width`: base plane width in pixels
    /// - `height`: base plane height in pixels
    /// - `format`: DRM_FORMAT code
    /// - `flags`: see enum flags
    #[inline]
    pub fn new_try_send_create_immed(
        &self,
        width: i32,
        height: i32,
        format: u32,
        flags: ZwpLinuxBufferParamsV1Flags,
    ) -> Result<Rc<WlBuffer>, ObjectError> {
        let buffer_id = self.core.create_child();
        self.try_send_create_immed(
            &buffer_id,
            width,
            height,
            format,
            flags,
        )?;
        Ok(buffer_id)
    }

    /// immediately create a wl_buffer from the given
    ///                      dmabufs
    ///
    /// This asks for immediate creation of a wl_buffer by importing the
    /// added dmabufs.
    ///
    /// In case of import success, no event is sent from the server, and the
    /// wl_buffer is ready to be used by the client.
    ///
    /// Upon import failure, either of the following may happen, as seen fit
    /// by the implementation:
    /// - the client is terminated with one of the following fatal protocol
    ///   errors:
    ///   - INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS, OUT_OF_BOUNDS,
    ///     in case of argument errors such as mismatch between the number
    ///     of planes and the format, bad format, non-positive width or
    ///     height, or bad offset or stride.
    ///   - INVALID_WL_BUFFER, in case the cause for failure is unknown or
    ///     platform specific.
    /// - the server creates an invalid wl_buffer, marks it as failed and
    ///   sends a 'failed' event to the client. The result of using this
    ///   invalid wl_buffer as an argument in any request by the client is
    ///   defined by the compositor implementation.
    ///
    /// This takes the same arguments as a 'create' request, and obeys the
    /// same restrictions.
    ///
    /// # Arguments
    ///
    /// - `width`: base plane width in pixels
    /// - `height`: base plane height in pixels
    /// - `format`: DRM_FORMAT code
    /// - `flags`: see enum flags
    #[inline]
    pub fn new_send_create_immed(
        &self,
        width: i32,
        height: i32,
        format: u32,
        flags: ZwpLinuxBufferParamsV1Flags,
    ) -> Rc<WlBuffer> {
        let buffer_id = self.core.create_child();
        self.send_create_immed(
            &buffer_id,
            width,
            height,
            format,
            flags,
        );
        buffer_id
    }
}

/// A message handler for [`ZwpLinuxBufferParamsV1`] proxies.
pub trait ZwpLinuxBufferParamsV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpLinuxBufferParamsV1>) {
        slf.core.delete_id();
    }

    /// delete this object, used or not
    ///
    /// Cleans up the temporary data sent to the server for dmabuf-based
    /// wl_buffer creation.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwpLinuxBufferParamsV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwp_linux_buffer_params_v1.destroy", &e);
        }
    }

    /// add a dmabuf to the temporary set
    ///
    /// This request adds one dmabuf to the set in this
    /// zwp_linux_buffer_params_v1.
    ///
    /// The 64-bit unsigned value combined from modifier_hi and modifier_lo
    /// is the dmabuf layout modifier. DRM AddFB2 ioctl calls this the
    /// fb modifier, which is defined in drm_mode.h of Linux UAPI.
    /// This is an opaque token. Drivers use this token to express tiling,
    /// compression, etc. driver-specific modifications to the base format
    /// defined by the DRM fourcc code.
    ///
    /// Starting from version 4, the invalid_format protocol error is sent if
    /// the format + modifier pair was not advertised as supported.
    ///
    /// Starting from version 5, the invalid_format protocol error is sent if
    /// all planes don't use the same modifier.
    ///
    /// This request raises the PLANE_IDX error if plane_idx is too large.
    /// The error PLANE_SET is raised if attempting to set a plane that
    /// was already set.
    ///
    /// # Arguments
    ///
    /// - `fd`: dmabuf fd
    /// - `plane_idx`: plane index
    /// - `offset`: offset in bytes
    /// - `stride`: stride in bytes
    /// - `modifier_hi`: high 32 bits of layout modifier
    /// - `modifier_lo`: low 32 bits of layout modifier
    #[inline]
    fn handle_add(
        &mut self,
        slf: &Rc<ZwpLinuxBufferParamsV1>,
        fd: &Rc<OwnedFd>,
        plane_idx: u32,
        offset: u32,
        stride: u32,
        modifier_hi: u32,
        modifier_lo: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_add(
            fd,
            plane_idx,
            offset,
            stride,
            modifier_hi,
            modifier_lo,
        );
        if let Err(e) = res {
            log_forward("zwp_linux_buffer_params_v1.add", &e);
        }
    }

    /// create a wl_buffer from the given dmabufs
    ///
    /// This asks for creation of a wl_buffer from the added dmabuf
    /// buffers. The wl_buffer is not created immediately but returned via
    /// the 'created' event if the dmabuf sharing succeeds. The sharing
    /// may fail at runtime for reasons a client cannot predict, in
    /// which case the 'failed' event is triggered.
    ///
    /// The 'format' argument is a DRM_FORMAT code, as defined by the
    /// libdrm's drm_fourcc.h. The Linux kernel's DRM sub-system is the
    /// authoritative source on how the format codes should work.
    ///
    /// The 'flags' is a bitfield of the flags defined in enum "flags".
    /// 'y_invert' means the that the image needs to be y-flipped.
    ///
    /// Flag 'interlaced' means that the frame in the buffer is not
    /// progressive as usual, but interlaced. An interlaced buffer as
    /// supported here must always contain both top and bottom fields.
    /// The top field always begins on the first pixel row. The temporal
    /// ordering between the two fields is top field first, unless
    /// 'bottom_first' is specified. It is undefined whether 'bottom_first'
    /// is ignored if 'interlaced' is not set.
    ///
    /// This protocol does not convey any information about field rate,
    /// duration, or timing, other than the relative ordering between the
    /// two fields in one buffer. A compositor may have to estimate the
    /// intended field rate from the incoming buffer rate. It is undefined
    /// whether the time of receiving wl_surface.commit with a new buffer
    /// attached, applying the wl_surface state, wl_surface.frame callback
    /// trigger, presentation, or any other point in the compositor cycle
    /// is used to measure the frame or field times. There is no support
    /// for detecting missed or late frames/fields/buffers either, and
    /// there is no support whatsoever for cooperating with interlaced
    /// compositor output.
    ///
    /// The composited image quality resulting from the use of interlaced
    /// buffers is explicitly undefined. A compositor may use elaborate
    /// hardware features or software to deinterlace and create progressive
    /// output frames from a sequence of interlaced input buffers, or it
    /// may produce substandard image quality. However, compositors that
    /// cannot guarantee reasonable image quality in all cases are recommended
    /// to just reject all interlaced buffers.
    ///
    /// Any argument errors, including non-positive width or height,
    /// mismatch between the number of planes and the format, bad
    /// format, bad offset or stride, may be indicated by fatal protocol
    /// errors: INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS,
    /// OUT_OF_BOUNDS.
    ///
    /// Dmabuf import errors in the server that are not obvious client
    /// bugs are returned via the 'failed' event as non-fatal. This
    /// allows attempting dmabuf sharing and falling back in the client
    /// if it fails.
    ///
    /// This request can be sent only once in the object's lifetime, after
    /// which the only legal request is destroy. This object should be
    /// destroyed after issuing a 'create' request. Attempting to use this
    /// object after issuing 'create' raises ALREADY_USED protocol error.
    ///
    /// It is not mandatory to issue 'create'. If a client wants to
    /// cancel the buffer creation, it can just destroy this object.
    ///
    /// # Arguments
    ///
    /// - `width`: base plane width in pixels
    /// - `height`: base plane height in pixels
    /// - `format`: DRM_FORMAT code
    /// - `flags`: see enum flags
    #[inline]
    fn handle_create(
        &mut self,
        slf: &Rc<ZwpLinuxBufferParamsV1>,
        width: i32,
        height: i32,
        format: u32,
        flags: ZwpLinuxBufferParamsV1Flags,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create(
            width,
            height,
            format,
            flags,
        );
        if let Err(e) = res {
            log_forward("zwp_linux_buffer_params_v1.create", &e);
        }
    }

    /// buffer creation succeeded
    ///
    /// This event indicates that the attempted buffer creation was
    /// successful. It provides the new wl_buffer referencing the dmabuf(s).
    ///
    /// Upon receiving this event, the client should destroy the
    /// zwp_linux_buffer_params_v1 object.
    ///
    /// # Arguments
    ///
    /// - `buffer`: the newly created wl_buffer
    #[inline]
    fn handle_created(
        &mut self,
        slf: &Rc<ZwpLinuxBufferParamsV1>,
        buffer: &Rc<WlBuffer>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_created(
            buffer,
        );
        if let Err(e) = res {
            log_forward("zwp_linux_buffer_params_v1.created", &e);
        }
    }

    /// buffer creation failed
    ///
    /// This event indicates that the attempted buffer creation has
    /// failed. It usually means that one of the dmabuf constraints
    /// has not been fulfilled.
    ///
    /// Upon receiving this event, the client should destroy the
    /// zwp_linux_buffer_params_v1 object.
    #[inline]
    fn handle_failed(
        &mut self,
        slf: &Rc<ZwpLinuxBufferParamsV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_failed(
        );
        if let Err(e) = res {
            log_forward("zwp_linux_buffer_params_v1.failed", &e);
        }
    }

    /// immediately create a wl_buffer from the given
    ///                      dmabufs
    ///
    /// This asks for immediate creation of a wl_buffer by importing the
    /// added dmabufs.
    ///
    /// In case of import success, no event is sent from the server, and the
    /// wl_buffer is ready to be used by the client.
    ///
    /// Upon import failure, either of the following may happen, as seen fit
    /// by the implementation:
    /// - the client is terminated with one of the following fatal protocol
    ///   errors:
    ///   - INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS, OUT_OF_BOUNDS,
    ///     in case of argument errors such as mismatch between the number
    ///     of planes and the format, bad format, non-positive width or
    ///     height, or bad offset or stride.
    ///   - INVALID_WL_BUFFER, in case the cause for failure is unknown or
    ///     platform specific.
    /// - the server creates an invalid wl_buffer, marks it as failed and
    ///   sends a 'failed' event to the client. The result of using this
    ///   invalid wl_buffer as an argument in any request by the client is
    ///   defined by the compositor implementation.
    ///
    /// This takes the same arguments as a 'create' request, and obeys the
    /// same restrictions.
    ///
    /// # Arguments
    ///
    /// - `buffer_id`: id for the newly created wl_buffer
    /// - `width`: base plane width in pixels
    /// - `height`: base plane height in pixels
    /// - `format`: DRM_FORMAT code
    /// - `flags`: see enum flags
    #[inline]
    fn handle_create_immed(
        &mut self,
        slf: &Rc<ZwpLinuxBufferParamsV1>,
        buffer_id: &Rc<WlBuffer>,
        width: i32,
        height: i32,
        format: u32,
        flags: ZwpLinuxBufferParamsV1Flags,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_immed(
            buffer_id,
            width,
            height,
            format,
            flags,
        );
        if let Err(e) = res {
            log_forward("zwp_linux_buffer_params_v1.create_immed", &e);
        }
    }
}

impl ObjectPrivate for ZwpLinuxBufferParamsV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpLinuxBufferParamsV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_linux_buffer_params_v1#{}.destroy()\n", client_id, id);
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
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                    arg5,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 28)));
                };
                let Some(arg0) = fds.pop_front() else {
                    return Err(ObjectError(ObjectErrorKind::MissingFd("fd")));
                };
                let arg0 = &arg0;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_linux_buffer_params_v1#{}.add(fd: {}, plane_idx: {}, offset: {}, stride: {}, modifier_hi: {}, modifier_lo: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4, arg5);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0.as_raw_fd(), arg1, arg2, arg3, arg4, arg5);
                }
                if let Some(handler) = handler {
                    (**handler).handle_add(&self, arg0, arg1, arg2, arg3, arg4, arg5);
                } else {
                    DefaultHandler.handle_add(&self, arg0, arg1, arg2, arg3, arg4, arg5);
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
                let arg3 = ZwpLinuxBufferParamsV1Flags(arg3);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32, arg2: u32, arg3: ZwpLinuxBufferParamsV1Flags) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_linux_buffer_params_v1#{}.create(width: {}, height: {}, format: {}, flags: {:?})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_create(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_create(&self, arg0, arg1, arg2, arg3);
                }
            }
            3 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 28)));
                };
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg4 = ZwpLinuxBufferParamsV1Flags(arg4);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: i32, arg2: i32, arg3: u32, arg4: ZwpLinuxBufferParamsV1Flags) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_linux_buffer_params_v1#{}.create_immed(buffer_id: wl_buffer#{}, width: {}, height: {}, format: {}, flags: {:?})\n", client_id, id, arg0, arg1, arg2, arg3, arg4);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4);
                }
                let arg0_id = arg0;
                let arg0 = WlBuffer::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "buffer_id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_immed(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultHandler.handle_create_immed(&self, arg0, arg1, arg2, arg3, arg4);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_linux_buffer_params_v1#{}.created(buffer: wl_buffer#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WlBuffer::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetServerId(arg0_id, "buffer", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_created(&self, arg0);
                } else {
                    DefaultHandler.handle_created(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_linux_buffer_params_v1#{}.failed()\n", id);
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
            1 => "add",
            2 => "create",
            3 => "create_immed",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "created",
            1 => "failed",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwpLinuxBufferParamsV1 {
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

impl ZwpLinuxBufferParamsV1 {
    /// Since when the error.already_used enum variant is available.
    pub const ENM__ERROR_ALREADY_USED__SINCE: u32 = 1;
    /// Since when the error.plane_idx enum variant is available.
    pub const ENM__ERROR_PLANE_IDX__SINCE: u32 = 1;
    /// Since when the error.plane_set enum variant is available.
    pub const ENM__ERROR_PLANE_SET__SINCE: u32 = 1;
    /// Since when the error.incomplete enum variant is available.
    pub const ENM__ERROR_INCOMPLETE__SINCE: u32 = 1;
    /// Since when the error.invalid_format enum variant is available.
    pub const ENM__ERROR_INVALID_FORMAT__SINCE: u32 = 1;
    /// Since when the error.invalid_dimensions enum variant is available.
    pub const ENM__ERROR_INVALID_DIMENSIONS__SINCE: u32 = 1;
    /// Since when the error.out_of_bounds enum variant is available.
    pub const ENM__ERROR_OUT_OF_BOUNDS__SINCE: u32 = 1;
    /// Since when the error.invalid_wl_buffer enum variant is available.
    pub const ENM__ERROR_INVALID_WL_BUFFER__SINCE: u32 = 1;

    /// Since when the flags.y_invert enum variant is available.
    pub const ENM__FLAGS_Y_INVERT__SINCE: u32 = 1;
    /// Since when the flags.interlaced enum variant is available.
    pub const ENM__FLAGS_INTERLACED__SINCE: u32 = 1;
    /// Since when the flags.bottom_first enum variant is available.
    pub const ENM__FLAGS_BOTTOM_FIRST__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwpLinuxBufferParamsV1Error(pub u32);

impl ZwpLinuxBufferParamsV1Error {
    /// the dmabuf_batch object has already been used to create a wl_buffer
    pub const ALREADY_USED: Self = Self(0);

    /// plane index out of bounds
    pub const PLANE_IDX: Self = Self(1);

    /// the plane index was already set
    pub const PLANE_SET: Self = Self(2);

    /// missing or too many planes to create a buffer
    pub const INCOMPLETE: Self = Self(3);

    /// format not supported
    pub const INVALID_FORMAT: Self = Self(4);

    /// invalid width or height
    pub const INVALID_DIMENSIONS: Self = Self(5);

    /// offset + stride * height goes out of dmabuf bounds
    pub const OUT_OF_BOUNDS: Self = Self(6);

    /// invalid wl_buffer resulted from importing dmabufs via
    ///                the create_immed request on given buffer_params
    pub const INVALID_WL_BUFFER: Self = Self(7);
}

impl Debug for ZwpLinuxBufferParamsV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ALREADY_USED => "ALREADY_USED",
            Self::PLANE_IDX => "PLANE_IDX",
            Self::PLANE_SET => "PLANE_SET",
            Self::INCOMPLETE => "INCOMPLETE",
            Self::INVALID_FORMAT => "INVALID_FORMAT",
            Self::INVALID_DIMENSIONS => "INVALID_DIMENSIONS",
            Self::OUT_OF_BOUNDS => "OUT_OF_BOUNDS",
            Self::INVALID_WL_BUFFER => "INVALID_WL_BUFFER",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct ZwpLinuxBufferParamsV1Flags(pub u32);

/// An iterator over the set bits in a [`ZwpLinuxBufferParamsV1Flags`].
///
/// You can construct this with the `IntoIterator` implementation of `ZwpLinuxBufferParamsV1Flags`.
#[derive(Clone, Debug)]
pub struct ZwpLinuxBufferParamsV1FlagsIter(pub u32);

impl ZwpLinuxBufferParamsV1Flags {
    /// contents are y-inverted
    pub const Y_INVERT: Self = Self(1);

    /// content is interlaced
    pub const INTERLACED: Self = Self(2);

    /// bottom field first
    pub const BOTTOM_FIRST: Self = Self(4);
}

impl ZwpLinuxBufferParamsV1Flags {
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
        Self(0 | 1 | 2 | 4)
    }
}

impl Iterator for ZwpLinuxBufferParamsV1FlagsIter {
    type Item = ZwpLinuxBufferParamsV1Flags;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(ZwpLinuxBufferParamsV1Flags(bit))
    }
}

impl IntoIterator for ZwpLinuxBufferParamsV1Flags {
    type Item = ZwpLinuxBufferParamsV1Flags;
    type IntoIter = ZwpLinuxBufferParamsV1FlagsIter;

    fn into_iter(self) -> Self::IntoIter {
        ZwpLinuxBufferParamsV1FlagsIter(self.0)
    }
}

impl BitAnd for ZwpLinuxBufferParamsV1Flags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for ZwpLinuxBufferParamsV1Flags {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for ZwpLinuxBufferParamsV1Flags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for ZwpLinuxBufferParamsV1Flags {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for ZwpLinuxBufferParamsV1Flags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for ZwpLinuxBufferParamsV1Flags {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for ZwpLinuxBufferParamsV1Flags {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for ZwpLinuxBufferParamsV1Flags {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for ZwpLinuxBufferParamsV1Flags {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for ZwpLinuxBufferParamsV1Flags {
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
        if v & 2 == 2 {
            v &= !2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("INTERLACED")?;
        }
        if v & 4 == 4 {
            v &= !4;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("BOTTOM_FIRST")?;
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
