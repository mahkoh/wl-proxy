//! a shared memory pool
//!
//! The wl_shm_pool object encapsulates a piece of memory shared
//! between the compositor and client.  Through the wl_shm_pool
//! object, the client can allocate shared memory wl_buffer objects.
//! All objects created through the same pool share the same
//! underlying mapped memory. Reusing the mapped memory avoids the
//! setup/teardown overhead and is useful when interactively resizing
//! a surface or for many small buffers.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wl_shm_pool proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWlShmPool {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWlShmPoolMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWlShmPoolMessageHandler for DefaultMessageHandler { }

impl MetaWlShmPool {
    pub const XML_VERSION: u32 = 2;
}

impl MetaWlShmPool {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WlShmPool, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaWlShmPoolMessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaWlShmPool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWlShmPool")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWlShmPool {
    /// Since when the create_buffer message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE_BUFFER__SINCE: u32 = 1;

    /// create a buffer from the pool
    ///
    /// Create a wl_buffer object from the pool.
    ///
    /// The buffer is created offset bytes into the pool and has
    /// width and height as specified.  The stride argument specifies
    /// the number of bytes from the beginning of one row to the beginning
    /// of the next.  The format is the pixel format of the buffer and
    /// must be one of those advertised through the wl_shm.format event.
    ///
    /// A buffer will keep a reference to the pool it was created from
    /// so it is valid to destroy the pool immediately after creating
    /// a buffer from it.
    ///
    /// # Arguments
    ///
    /// - `id`: buffer to create
    /// - `offset`: buffer byte offset within the pool
    /// - `width`: buffer width, in pixels
    /// - `height`: buffer height, in pixels
    /// - `stride`: number of bytes from the beginning of one row to the beginning of the next row
    /// - `format`: buffer pixel format
    #[inline]
    pub fn send_create_buffer(
        &self,
        id: &Rc<MetaWlBuffer>,
        offset: i32,
        width: i32,
        height: i32,
        stride: i32,
        format: MetaWlShmFormat,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
        ) = (
            id,
            offset,
            width,
            height,
            stride,
            format,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
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
            0,
            arg0_id,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
            arg4 as u32,
            arg5.0,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the pool
    ///
    /// Destroy the shared memory pool.
    ///
    /// The mmapped memory will be released when all
    /// buffers that have been created from this pool
    /// are gone.
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
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the resize message is available.
    #[allow(dead_code)]
    pub const MSG__RESIZE__SINCE: u32 = 1;

    /// change the size of the pool mapping
    ///
    /// This request will cause the server to remap the backing memory
    /// for the pool from the file descriptor passed when the pool was
    /// created, but using the new size.  This request can only be
    /// used to make the pool bigger.
    ///
    /// This request only changes the amount of bytes that are mmapped
    /// by the server and does not touch the file corresponding to the
    /// file descriptor passed at creation time. It is the client's
    /// responsibility to ensure that the file is at least as big as
    /// the new pool size.
    ///
    /// # Arguments
    ///
    /// - `size`: new size of the pool, in bytes
    #[inline]
    pub fn send_resize(
        &self,
        size: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            size,
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
        ]);
        Ok(())
    }
}

/// A message handler for [WlShmPool] proxies.
#[allow(dead_code)]
pub trait MetaWlShmPoolMessageHandler {
    /// create a buffer from the pool
    ///
    /// Create a wl_buffer object from the pool.
    ///
    /// The buffer is created offset bytes into the pool and has
    /// width and height as specified.  The stride argument specifies
    /// the number of bytes from the beginning of one row to the beginning
    /// of the next.  The format is the pixel format of the buffer and
    /// must be one of those advertised through the wl_shm.format event.
    ///
    /// A buffer will keep a reference to the pool it was created from
    /// so it is valid to destroy the pool immediately after creating
    /// a buffer from it.
    ///
    /// # Arguments
    ///
    /// - `id`: buffer to create
    /// - `offset`: buffer byte offset within the pool
    /// - `width`: buffer width, in pixels
    /// - `height`: buffer height, in pixels
    /// - `stride`: number of bytes from the beginning of one row to the beginning of the next row
    /// - `format`: buffer pixel format
    #[inline]
    fn create_buffer(
        &mut self,
        _slf: &Rc<MetaWlShmPool>,
        id: &Rc<MetaWlBuffer>,
        offset: i32,
        width: i32,
        height: i32,
        stride: i32,
        format: MetaWlShmFormat,
    ) {
        let res = _slf.send_create_buffer(
            id,
            offset,
            width,
            height,
            stride,
            format,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_shm_pool.create_buffer message: {}", Report::new(e));
        }
    }

    /// destroy the pool
    ///
    /// Destroy the shared memory pool.
    ///
    /// The mmapped memory will be released when all
    /// buffers that have been created from this pool
    /// are gone.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWlShmPool>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_shm_pool.destroy message: {}", Report::new(e));
        }
    }

    /// change the size of the pool mapping
    ///
    /// This request will cause the server to remap the backing memory
    /// for the pool from the file descriptor passed when the pool was
    /// created, but using the new size.  This request can only be
    /// used to make the pool bigger.
    ///
    /// This request only changes the amount of bytes that are mmapped
    /// by the server and does not touch the file corresponding to the
    /// file descriptor passed at creation time. It is the client's
    /// responsibility to ensure that the file is at least as big as
    /// the new pool size.
    ///
    /// # Arguments
    ///
    /// - `size`: new size of the pool, in bytes
    #[inline]
    fn resize(
        &mut self,
        _slf: &Rc<MetaWlShmPool>,
        size: i32,
    ) {
        let res = _slf.send_resize(
            size,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_shm_pool.resize message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWlShmPool {
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
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                    arg5,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 32));
                };
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                let arg4 = arg4 as i32;
                let arg5 = MetaWlShmFormat(arg5);
                let arg0_id = arg0;
                let arg0 = MetaWlBuffer::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).create_buffer(&self, arg0, arg1, arg2, arg3, arg4, arg5);
                } else {
                    DefaultMessageHandler.create_buffer(&self, arg0, arg1, arg2, arg3, arg4, arg5);
                }
            }
            1 => {
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
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = arg0 as i32;
                if let Some(handler) = handler {
                    (**handler).resize(&self, arg0);
                } else {
                    DefaultMessageHandler.resize(&self, arg0);
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
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "create_buffer",
            1 => "destroy",
            2 => "resize",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

