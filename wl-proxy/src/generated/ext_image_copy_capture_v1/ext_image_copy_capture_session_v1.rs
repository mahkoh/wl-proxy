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

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_image_copy_capture_session_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaExtImageCopyCaptureSessionV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaExtImageCopyCaptureSessionV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaExtImageCopyCaptureSessionV1MessageHandler for DefaultMessageHandler { }

impl MetaExtImageCopyCaptureSessionV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaExtImageCopyCaptureSessionV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaExtImageCopyCaptureSessionV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaExtImageCopyCaptureSessionV1 {
    /// Since when the buffer_size message is available.
    #[allow(dead_code)]
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
    pub fn send_buffer_size(
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
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// Since when the shm_format message is available.
    #[allow(dead_code)]
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
    pub fn send_shm_format(
        &self,
        format: MetaWlShmFormat,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            format,
        );
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            1,
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the dmabuf_device message is available.
    #[allow(dead_code)]
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
    pub fn send_dmabuf_device(
        &self,
        device: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            device,
        );
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            2,
        ]);
        fmt.array(arg0);
        Ok(())
    }

    /// Since when the dmabuf_format message is available.
    #[allow(dead_code)]
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
    pub fn send_dmabuf_format(
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
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            3,
            arg0,
        ]);
        fmt.array(arg1);
        Ok(())
    }

    /// Since when the done message is available.
    #[allow(dead_code)]
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
    pub fn send_done(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            4,
        ]);
        Ok(())
    }

    /// Since when the stopped message is available.
    #[allow(dead_code)]
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
    pub fn send_stopped(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            5,
        ]);
        Ok(())
    }

    /// Since when the create_frame message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE_FRAME__SINCE: u32 = 1;

    /// create a frame
    ///
    /// Create a capture frame for this session.
    ///
    /// At most one frame object can exist for a given session at any time. If
    /// a client sends a create_frame request before a previous frame object
    /// has been destroyed, the duplicate_frame protocol error is raised.
    #[inline]
    pub fn send_create_frame(
        &self,
        frame: &Rc<MetaExtImageCopyCaptureFrameV1>,
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
            return Err(ObjectError);
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0.server_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

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
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
        ]);
        Ok(())
    }
}

/// A message handler for [ExtImageCopyCaptureSessionV1] proxies.
#[allow(dead_code)]
pub trait MetaExtImageCopyCaptureSessionV1MessageHandler {
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
    fn buffer_size(
        &mut self,
        _slf: &Rc<MetaExtImageCopyCaptureSessionV1>,
        width: u32,
        height: u32,
    ) {
        let res = _slf.send_buffer_size(
            width,
            height,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_session_v1.buffer_size message: {}", Report::new(e));
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
    fn shm_format(
        &mut self,
        _slf: &Rc<MetaExtImageCopyCaptureSessionV1>,
        format: MetaWlShmFormat,
    ) {
        let res = _slf.send_shm_format(
            format,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_session_v1.shm_format message: {}", Report::new(e));
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
    fn dmabuf_device(
        &mut self,
        _slf: &Rc<MetaExtImageCopyCaptureSessionV1>,
        device: &[u8],
    ) {
        let res = _slf.send_dmabuf_device(
            device,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_session_v1.dmabuf_device message: {}", Report::new(e));
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
    fn dmabuf_format(
        &mut self,
        _slf: &Rc<MetaExtImageCopyCaptureSessionV1>,
        format: u32,
        modifiers: &[u8],
    ) {
        let res = _slf.send_dmabuf_format(
            format,
            modifiers,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_session_v1.dmabuf_format message: {}", Report::new(e));
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
    fn done(
        &mut self,
        _slf: &Rc<MetaExtImageCopyCaptureSessionV1>,
    ) {
        let res = _slf.send_done(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_session_v1.done message: {}", Report::new(e));
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
    fn stopped(
        &mut self,
        _slf: &Rc<MetaExtImageCopyCaptureSessionV1>,
    ) {
        let res = _slf.send_stopped(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_session_v1.stopped message: {}", Report::new(e));
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
    fn create_frame(
        &mut self,
        _slf: &Rc<MetaExtImageCopyCaptureSessionV1>,
        frame: &Rc<MetaExtImageCopyCaptureFrameV1>,
    ) {
        let res = _slf.send_create_frame(
            frame,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_session_v1.create_frame message: {}", Report::new(e));
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
    fn destroy(
        &mut self,
        _slf: &Rc<MetaExtImageCopyCaptureSessionV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_session_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaExtImageCopyCaptureSessionV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaExtImageCopyCaptureFrameV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).create_frame(&self, arg0);
                } else {
                    DefaultMessageHandler.create_frame(&self, arg0);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
            }
            _ => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
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
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).buffer_size(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.buffer_size(&self, arg0, arg1);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = MetaWlShmFormat(arg0);
                if let Some(handler) = handler {
                    (**handler).shm_format(&self, arg0);
                } else {
                    DefaultMessageHandler.shm_format(&self, arg0);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError);
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError);
                    }
                    let start = offset;
                    offset += words;
                    &uapi::as_bytes(&msg[start..])[..len]
                };
                if offset != msg.len() {
                    return Err(ObjectError);
                }
                if let Some(handler) = handler {
                    (**handler).dmabuf_device(&self, arg0);
                } else {
                    DefaultMessageHandler.dmabuf_device(&self, arg0);
                }
            }
            3 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError);
                };
                offset += 1;
                let arg1 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError);
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError);
                    }
                    let start = offset;
                    offset += words;
                    &uapi::as_bytes(&msg[start..])[..len]
                };
                if offset != msg.len() {
                    return Err(ObjectError);
                }
                if let Some(handler) = handler {
                    (**handler).dmabuf_format(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.dmabuf_format(&self, arg0, arg1);
                }
            }
            4 => {
                if let Some(handler) = handler {
                    (**handler).done(&self);
                } else {
                    DefaultMessageHandler.done(&self);
                }
            }
            5 => {
                if let Some(handler) = handler {
                    (**handler).stopped(&self);
                } else {
                    DefaultMessageHandler.stopped(&self);
                }
            }
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }
}

impl MetaExtImageCopyCaptureSessionV1 {
    /// Since when the error.duplicate_frame enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_DUPLICATE_FRAME__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaExtImageCopyCaptureSessionV1Error(pub u32);

impl MetaExtImageCopyCaptureSessionV1Error {
    /// create_frame sent before destroying previous frame
    #[allow(dead_code)]
    pub const DUPLICATE_FRAME: Self = Self(1);
}

impl Debug for MetaExtImageCopyCaptureSessionV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DUPLICATE_FRAME => "DUPLICATE_FRAME",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
