//! a shared memory pool
//!
//! The wl_shm_pool object encapsulates a piece of memory shared
//! between the compositor and client.  Through the wl_shm_pool
//! object, the client can allocate shared memory wl_buffer objects.
//! All objects created through the same pool share the same
//! underlying mapped memory. Reusing the mapped memory avoids the
//! setup/teardown overhead and is useful when interactively resizing
//! a surface or for many small buffers.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wl_shm_pool object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlShmPool {
    core: ObjectCore,
    handler: HandlerHolder<dyn WlShmPoolHandler>,
}

struct DefaultHandler;

impl WlShmPoolHandler for DefaultHandler { }

impl ConcreteObject for WlShmPool {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::WlShmPool;
    const INTERFACE_NAME: &str = "wl_shm_pool";
}

impl WlShmPool {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WlShmPoolHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WlShmPoolHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlShmPool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlShmPool")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlShmPool {
    /// Since when the create_buffer message is available.
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
    pub fn try_send_create_buffer(
        &self,
        id: &Rc<WlBuffer>,
        offset: i32,
        width: i32,
        height: i32,
        stride: i32,
        format: WlShmFormat,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: i32, arg2: i32, arg3: i32, arg4: i32, arg5: WlShmFormat) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_shm_pool#{}.create_buffer(id: wl_buffer#{}, offset: {}, width: {}, height: {}, stride: {}, format: {:?})\n", id, arg0, arg1, arg2, arg3, arg4, arg5);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1, arg2, arg3, arg4, arg5);
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
            arg2 as u32,
            arg3 as u32,
            arg4 as u32,
            arg5.0,
        ]);
        Ok(())
    }

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
        id: &Rc<WlBuffer>,
        offset: i32,
        width: i32,
        height: i32,
        stride: i32,
        format: WlShmFormat,
    ) {
        let res = self.try_send_create_buffer(
            id,
            offset,
            width,
            height,
            stride,
            format,
        );
        if let Err(e) = res {
            log_send("wl_shm_pool.create_buffer", &e);
        }
    }

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
    /// - `offset`: buffer byte offset within the pool
    /// - `width`: buffer width, in pixels
    /// - `height`: buffer height, in pixels
    /// - `stride`: number of bytes from the beginning of one row to the beginning of the next row
    /// - `format`: buffer pixel format
    #[inline]
    pub fn new_try_send_create_buffer(
        &self,
        offset: i32,
        width: i32,
        height: i32,
        stride: i32,
        format: WlShmFormat,
    ) -> Result<Rc<WlBuffer>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_create_buffer(
            &id,
            offset,
            width,
            height,
            stride,
            format,
        )?;
        Ok(id)
    }

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
    /// - `offset`: buffer byte offset within the pool
    /// - `width`: buffer width, in pixels
    /// - `height`: buffer height, in pixels
    /// - `stride`: number of bytes from the beginning of one row to the beginning of the next row
    /// - `format`: buffer pixel format
    #[inline]
    pub fn new_send_create_buffer(
        &self,
        offset: i32,
        width: i32,
        height: i32,
        stride: i32,
        format: WlShmFormat,
    ) -> Rc<WlBuffer> {
        let id = self.core.create_child();
        self.send_create_buffer(
            &id,
            offset,
            width,
            height,
            stride,
            format,
        );
        id
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the pool
    ///
    /// Destroy the shared memory pool.
    ///
    /// The mmapped memory will be released when all
    /// buffers that have been created from this pool
    /// are gone.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_shm_pool#{}.destroy()\n", id);
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
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wl_shm_pool.destroy", &e);
        }
    }

    /// Since when the resize message is available.
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
    pub fn try_send_resize(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_shm_pool#{}.resize(size: {})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
        ]);
        Ok(())
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
    pub fn send_resize(
        &self,
        size: i32,
    ) {
        let res = self.try_send_resize(
            size,
        );
        if let Err(e) = res {
            log_send("wl_shm_pool.resize", &e);
        }
    }
}

/// A message handler for [`WlShmPool`] proxies.
pub trait WlShmPoolHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WlShmPool>) {
        slf.core.delete_id();
    }

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
    fn handle_create_buffer(
        &mut self,
        slf: &Rc<WlShmPool>,
        id: &Rc<WlBuffer>,
        offset: i32,
        width: i32,
        height: i32,
        stride: i32,
        format: WlShmFormat,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_buffer(
            id,
            offset,
            width,
            height,
            stride,
            format,
        );
        if let Err(e) = res {
            log_forward("wl_shm_pool.create_buffer", &e);
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
    fn handle_destroy(
        &mut self,
        slf: &Rc<WlShmPool>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wl_shm_pool.destroy", &e);
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
    fn handle_resize(
        &mut self,
        slf: &Rc<WlShmPool>,
        size: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_resize(
            size,
        );
        if let Err(e) = res {
            log_forward("wl_shm_pool.resize", &e);
        }
    }
}

impl ObjectPrivate for WlShmPool {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WlShmPool, version),
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
                    arg2,
                    arg3,
                    arg4,
                    arg5,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 32)));
                };
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                let arg4 = arg4 as i32;
                let arg5 = WlShmFormat(arg5);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: i32, arg2: i32, arg3: i32, arg4: i32, arg5: WlShmFormat) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_shm_pool#{}.create_buffer(id: wl_buffer#{}, offset: {}, width: {}, height: {}, stride: {}, format: {:?})\n", client_id, id, arg0, arg1, arg2, arg3, arg4, arg5);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4, arg5);
                }
                let arg0_id = arg0;
                let arg0 = WlBuffer::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_buffer(&self, arg0, arg1, arg2, arg3, arg4, arg5);
                } else {
                    DefaultHandler.handle_create_buffer(&self, arg0, arg1, arg2, arg3, arg4, arg5);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_shm_pool#{}.destroy()\n", client_id, id);
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
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = arg0 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_shm_pool#{}.resize(size: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_resize(&self, arg0);
                } else {
                    DefaultHandler.handle_resize(&self, arg0);
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
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
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

impl Object for WlShmPool {
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

