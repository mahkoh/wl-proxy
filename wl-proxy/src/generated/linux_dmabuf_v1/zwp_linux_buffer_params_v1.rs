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

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_linux_buffer_params_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpLinuxBufferParamsV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpLinuxBufferParamsV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpLinuxBufferParamsV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpLinuxBufferParamsV1 {
    pub const XML_VERSION: u32 = 5;
}

impl MetaZwpLinuxBufferParamsV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpLinuxBufferParamsV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaZwpLinuxBufferParamsV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaZwpLinuxBufferParamsV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpLinuxBufferParamsV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpLinuxBufferParamsV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// delete this object, used or not
    ///
    /// Cleans up the temporary data sent to the server for dmabuf-based
    /// wl_buffer creation.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the add message is available.
    #[allow(dead_code)]
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
    pub fn send_add(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the create message is available.
    #[allow(dead_code)]
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
    pub fn send_create(
        &self,
        width: i32,
        height: i32,
        format: u32,
        flags: MetaZwpLinuxBufferParamsV1Flags,
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the created message is available.
    #[allow(dead_code)]
    pub const MSG__CREATED__SINCE: u32 = 1;

    /// buffer creation succeeded
    ///
    /// This event indicates that the attempted buffer creation was
    /// successful. It provides the new wl_buffer referencing the dmabuf(s).
    ///
    /// Upon receiving this event, the client should destroy the
    /// zwp_linux_buffer_params_v1 object.
    #[inline]
    pub fn send_created(
        &self,
        buffer: &Rc<MetaWlBuffer>,
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateClientId("buffer", e))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the failed message is available.
    #[allow(dead_code)]
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
    pub fn send_failed(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the create_immed message is available.
    #[allow(dead_code)]
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
    pub fn send_create_immed(
        &self,
        buffer_id: &Rc<MetaWlBuffer>,
        width: i32,
        height: i32,
        format: u32,
        flags: MetaZwpLinuxBufferParamsV1Flags,
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("buffer_id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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
}

/// A message handler for [ZwpLinuxBufferParamsV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpLinuxBufferParamsV1MessageHandler {
    /// delete this object, used or not
    ///
    /// Cleans up the temporary data sent to the server for dmabuf-based
    /// wl_buffer creation.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpLinuxBufferParamsV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_linux_buffer_params_v1.destroy message: {}", Report::new(e));
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
    fn add(
        &mut self,
        _slf: &Rc<MetaZwpLinuxBufferParamsV1>,
        fd: &Rc<OwnedFd>,
        plane_idx: u32,
        offset: u32,
        stride: u32,
        modifier_hi: u32,
        modifier_lo: u32,
    ) {
        let res = _slf.send_add(
            fd,
            plane_idx,
            offset,
            stride,
            modifier_hi,
            modifier_lo,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_linux_buffer_params_v1.add message: {}", Report::new(e));
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
    fn create(
        &mut self,
        _slf: &Rc<MetaZwpLinuxBufferParamsV1>,
        width: i32,
        height: i32,
        format: u32,
        flags: MetaZwpLinuxBufferParamsV1Flags,
    ) {
        let res = _slf.send_create(
            width,
            height,
            format,
            flags,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_linux_buffer_params_v1.create message: {}", Report::new(e));
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
    fn created(
        &mut self,
        _slf: &Rc<MetaZwpLinuxBufferParamsV1>,
        buffer: &Rc<MetaWlBuffer>,
    ) {
        let res = _slf.send_created(
            buffer,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_linux_buffer_params_v1.created message: {}", Report::new(e));
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
    fn failed(
        &mut self,
        _slf: &Rc<MetaZwpLinuxBufferParamsV1>,
    ) {
        let res = _slf.send_failed(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_linux_buffer_params_v1.failed message: {}", Report::new(e));
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
    fn create_immed(
        &mut self,
        _slf: &Rc<MetaZwpLinuxBufferParamsV1>,
        buffer_id: &Rc<MetaWlBuffer>,
        width: i32,
        height: i32,
        format: u32,
        flags: MetaZwpLinuxBufferParamsV1Flags,
    ) {
        let res = _slf.send_create_immed(
            buffer_id,
            width,
            height,
            format,
            flags,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_linux_buffer_params_v1.create_immed message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpLinuxBufferParamsV1 {
    fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Self::new(state, version)
    }

    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            1 => {
                let [
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                    arg5,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 28));
                };
                let Some(arg0) = fds.pop_front() else {
                    return Err(ObjectError::MissingFd("fd"));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).add(&self, arg0, arg1, arg2, arg3, arg4, arg5);
                } else {
                    DefaultMessageHandler.add(&self, arg0, arg1, arg2, arg3, arg4, arg5);
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
                let arg3 = MetaZwpLinuxBufferParamsV1Flags(arg3);
                if let Some(handler) = handler {
                    (**handler).create(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultMessageHandler.create(&self, arg0, arg1, arg2, arg3);
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
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 28));
                };
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg4 = MetaZwpLinuxBufferParamsV1Flags(arg4);
                let arg0_id = arg0;
                let arg0 = MetaWlBuffer::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "buffer_id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).create_immed(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultMessageHandler.create_immed(&self, arg0, arg1, arg2, arg3, arg4);
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
                let arg0_id = arg0;
                let arg0 = MetaWlBuffer::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetServerId(arg0_id, "buffer", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).created(&self, arg0);
                } else {
                    DefaultMessageHandler.created(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).failed(&self);
                } else {
                    DefaultMessageHandler.failed(&self);
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

impl MetaZwpLinuxBufferParamsV1 {
    /// Since when the error.already_used enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ALREADY_USED__SINCE: u32 = 1;
    /// Since when the error.plane_idx enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_PLANE_IDX__SINCE: u32 = 1;
    /// Since when the error.plane_set enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_PLANE_SET__SINCE: u32 = 1;
    /// Since when the error.incomplete enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INCOMPLETE__SINCE: u32 = 1;
    /// Since when the error.invalid_format enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_FORMAT__SINCE: u32 = 1;
    /// Since when the error.invalid_dimensions enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_DIMENSIONS__SINCE: u32 = 1;
    /// Since when the error.out_of_bounds enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_OUT_OF_BOUNDS__SINCE: u32 = 1;
    /// Since when the error.invalid_wl_buffer enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_WL_BUFFER__SINCE: u32 = 1;

    /// Since when the flags.y_invert enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FLAGS_Y_INVERT__SINCE: u32 = 1;
    /// Since when the flags.interlaced enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FLAGS_INTERLACED__SINCE: u32 = 1;
    /// Since when the flags.bottom_first enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FLAGS_BOTTOM_FIRST__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaZwpLinuxBufferParamsV1Error(pub u32);

impl MetaZwpLinuxBufferParamsV1Error {
    /// the dmabuf_batch object has already been used to create a wl_buffer
    #[allow(dead_code)]
    pub const ALREADY_USED: Self = Self(0);

    /// plane index out of bounds
    #[allow(dead_code)]
    pub const PLANE_IDX: Self = Self(1);

    /// the plane index was already set
    #[allow(dead_code)]
    pub const PLANE_SET: Self = Self(2);

    /// missing or too many planes to create a buffer
    #[allow(dead_code)]
    pub const INCOMPLETE: Self = Self(3);

    /// format not supported
    #[allow(dead_code)]
    pub const INVALID_FORMAT: Self = Self(4);

    /// invalid width or height
    #[allow(dead_code)]
    pub const INVALID_DIMENSIONS: Self = Self(5);

    /// offset + stride * height goes out of dmabuf bounds
    #[allow(dead_code)]
    pub const OUT_OF_BOUNDS: Self = Self(6);

    /// invalid wl_buffer resulted from importing dmabufs via
    ///                the create_immed request on given buffer_params
    #[allow(dead_code)]
    pub const INVALID_WL_BUFFER: Self = Self(7);
}

impl Debug for MetaZwpLinuxBufferParamsV1Error {
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
#[allow(dead_code)]
pub struct MetaZwpLinuxBufferParamsV1Flags(pub u32);

/// An iterator over the set bits in a [MetaZwpLinuxBufferParamsV1Flags].
///
/// You can construct this with the `IntoIterator` implementation of `MetaZwpLinuxBufferParamsV1Flags`.
#[derive(Clone, Debug)]
pub struct MetaZwpLinuxBufferParamsV1FlagsIter(pub u32);

impl MetaZwpLinuxBufferParamsV1Flags {
    /// contents are y-inverted
    #[allow(dead_code)]
    pub const Y_INVERT: Self = Self(1);

    /// content is interlaced
    #[allow(dead_code)]
    pub const INTERLACED: Self = Self(2);

    /// bottom field first
    #[allow(dead_code)]
    pub const BOTTOM_FIRST: Self = Self(4);
}

#[allow(dead_code)]
impl MetaZwpLinuxBufferParamsV1Flags {
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

impl Iterator for MetaZwpLinuxBufferParamsV1FlagsIter {
    type Item = MetaZwpLinuxBufferParamsV1Flags;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(MetaZwpLinuxBufferParamsV1Flags(bit))
    }
}

impl IntoIterator for MetaZwpLinuxBufferParamsV1Flags {
    type Item = MetaZwpLinuxBufferParamsV1Flags;
    type IntoIter = MetaZwpLinuxBufferParamsV1FlagsIter;

    fn into_iter(self) -> Self::IntoIter {
        MetaZwpLinuxBufferParamsV1FlagsIter(self.0)
    }
}

impl BitAnd for MetaZwpLinuxBufferParamsV1Flags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for MetaZwpLinuxBufferParamsV1Flags {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for MetaZwpLinuxBufferParamsV1Flags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for MetaZwpLinuxBufferParamsV1Flags {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for MetaZwpLinuxBufferParamsV1Flags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for MetaZwpLinuxBufferParamsV1Flags {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for MetaZwpLinuxBufferParamsV1Flags {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for MetaZwpLinuxBufferParamsV1Flags {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for MetaZwpLinuxBufferParamsV1Flags {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for MetaZwpLinuxBufferParamsV1Flags {
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
