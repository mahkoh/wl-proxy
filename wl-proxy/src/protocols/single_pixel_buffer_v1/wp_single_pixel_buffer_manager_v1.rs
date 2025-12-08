//! global factory for single-pixel buffers
//!
//! The wp_single_pixel_buffer_manager_v1 interface is a factory for
//! single-pixel buffers.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_single_pixel_buffer_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpSinglePixelBufferManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpSinglePixelBufferManagerV1Handler>,
}

struct DefaultHandler;

impl WpSinglePixelBufferManagerV1Handler for DefaultHandler { }

impl ConcreteObject for WpSinglePixelBufferManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WpSinglePixelBufferManagerV1;
    const INTERFACE_NAME: &str = "wp_single_pixel_buffer_manager_v1";
}

impl WpSinglePixelBufferManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WpSinglePixelBufferManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WpSinglePixelBufferManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpSinglePixelBufferManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpSinglePixelBufferManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpSinglePixelBufferManagerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the manager
    ///
    /// Destroy the wp_single_pixel_buffer_manager_v1 object.
    ///
    /// The child objects created via this interface are unaffected.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_single_pixel_buffer_manager_v1#{}.destroy()\n", id);
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

    /// destroy the manager
    ///
    /// Destroy the wp_single_pixel_buffer_manager_v1 object.
    ///
    /// The child objects created via this interface are unaffected.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wp_single_pixel_buffer_manager_v1.destroy", &e);
        }
    }

    /// Since when the create_u32_rgba_buffer message is available.
    pub const MSG__CREATE_U32_RGBA_BUFFER__SINCE: u32 = 1;

    /// create a 1×1 buffer from 32-bit RGBA values
    ///
    /// Create a single-pixel buffer from four 32-bit RGBA values.
    ///
    /// Unless specified in another protocol extension, the RGBA values use
    /// pre-multiplied alpha.
    ///
    /// The width and height of the buffer are 1.
    ///
    /// The r, g, b and a arguments valid range is from UINT32_MIN (0)
    /// to UINT32_MAX (0xffffffff).
    ///
    /// These arguments should be interpreted as a percentage, i.e.
    /// - UINT32_MIN = 0% of the given color component
    /// - UINT32_MAX = 100% of the given color component
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `r`: value of the buffer's red channel
    /// - `g`: value of the buffer's green channel
    /// - `b`: value of the buffer's blue channel
    /// - `a`: value of the buffer's alpha channel
    #[inline]
    pub fn try_send_create_u32_rgba_buffer(
        &self,
        id: &Rc<WlBuffer>,
        r: u32,
        g: u32,
        b: u32,
        a: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ) = (
            id,
            r,
            g,
            b,
            a,
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
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_single_pixel_buffer_manager_v1#{}.create_u32_rgba_buffer(id: wl_buffer#{}, r: {}, g: {}, b: {}, a: {})\n", id, arg0, arg1, arg2, arg3, arg4);
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
            1,
            arg0_id,
            arg1,
            arg2,
            arg3,
            arg4,
        ]);
        Ok(())
    }

    /// create a 1×1 buffer from 32-bit RGBA values
    ///
    /// Create a single-pixel buffer from four 32-bit RGBA values.
    ///
    /// Unless specified in another protocol extension, the RGBA values use
    /// pre-multiplied alpha.
    ///
    /// The width and height of the buffer are 1.
    ///
    /// The r, g, b and a arguments valid range is from UINT32_MIN (0)
    /// to UINT32_MAX (0xffffffff).
    ///
    /// These arguments should be interpreted as a percentage, i.e.
    /// - UINT32_MIN = 0% of the given color component
    /// - UINT32_MAX = 100% of the given color component
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `r`: value of the buffer's red channel
    /// - `g`: value of the buffer's green channel
    /// - `b`: value of the buffer's blue channel
    /// - `a`: value of the buffer's alpha channel
    #[inline]
    pub fn send_create_u32_rgba_buffer(
        &self,
        id: &Rc<WlBuffer>,
        r: u32,
        g: u32,
        b: u32,
        a: u32,
    ) {
        let res = self.try_send_create_u32_rgba_buffer(
            id,
            r,
            g,
            b,
            a,
        );
        if let Err(e) = res {
            log_send("wp_single_pixel_buffer_manager_v1.create_u32_rgba_buffer", &e);
        }
    }

    /// create a 1×1 buffer from 32-bit RGBA values
    ///
    /// Create a single-pixel buffer from four 32-bit RGBA values.
    ///
    /// Unless specified in another protocol extension, the RGBA values use
    /// pre-multiplied alpha.
    ///
    /// The width and height of the buffer are 1.
    ///
    /// The r, g, b and a arguments valid range is from UINT32_MIN (0)
    /// to UINT32_MAX (0xffffffff).
    ///
    /// These arguments should be interpreted as a percentage, i.e.
    /// - UINT32_MIN = 0% of the given color component
    /// - UINT32_MAX = 100% of the given color component
    ///
    /// # Arguments
    ///
    /// - `r`: value of the buffer's red channel
    /// - `g`: value of the buffer's green channel
    /// - `b`: value of the buffer's blue channel
    /// - `a`: value of the buffer's alpha channel
    #[inline]
    pub fn new_try_send_create_u32_rgba_buffer(
        &self,
        r: u32,
        g: u32,
        b: u32,
        a: u32,
    ) -> Result<Rc<WlBuffer>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_create_u32_rgba_buffer(
            &id,
            r,
            g,
            b,
            a,
        )?;
        Ok(id)
    }

    /// create a 1×1 buffer from 32-bit RGBA values
    ///
    /// Create a single-pixel buffer from four 32-bit RGBA values.
    ///
    /// Unless specified in another protocol extension, the RGBA values use
    /// pre-multiplied alpha.
    ///
    /// The width and height of the buffer are 1.
    ///
    /// The r, g, b and a arguments valid range is from UINT32_MIN (0)
    /// to UINT32_MAX (0xffffffff).
    ///
    /// These arguments should be interpreted as a percentage, i.e.
    /// - UINT32_MIN = 0% of the given color component
    /// - UINT32_MAX = 100% of the given color component
    ///
    /// # Arguments
    ///
    /// - `r`: value of the buffer's red channel
    /// - `g`: value of the buffer's green channel
    /// - `b`: value of the buffer's blue channel
    /// - `a`: value of the buffer's alpha channel
    #[inline]
    pub fn new_send_create_u32_rgba_buffer(
        &self,
        r: u32,
        g: u32,
        b: u32,
        a: u32,
    ) -> Rc<WlBuffer> {
        let id = self.core.create_child();
        self.send_create_u32_rgba_buffer(
            &id,
            r,
            g,
            b,
            a,
        );
        id
    }
}

/// A message handler for [`WpSinglePixelBufferManagerV1`] proxies.
pub trait WpSinglePixelBufferManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpSinglePixelBufferManagerV1>) {
        slf.core.delete_id();
    }

    /// destroy the manager
    ///
    /// Destroy the wp_single_pixel_buffer_manager_v1 object.
    ///
    /// The child objects created via this interface are unaffected.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WpSinglePixelBufferManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wp_single_pixel_buffer_manager_v1.destroy", &e);
        }
    }

    /// create a 1×1 buffer from 32-bit RGBA values
    ///
    /// Create a single-pixel buffer from four 32-bit RGBA values.
    ///
    /// Unless specified in another protocol extension, the RGBA values use
    /// pre-multiplied alpha.
    ///
    /// The width and height of the buffer are 1.
    ///
    /// The r, g, b and a arguments valid range is from UINT32_MIN (0)
    /// to UINT32_MAX (0xffffffff).
    ///
    /// These arguments should be interpreted as a percentage, i.e.
    /// - UINT32_MIN = 0% of the given color component
    /// - UINT32_MAX = 100% of the given color component
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `r`: value of the buffer's red channel
    /// - `g`: value of the buffer's green channel
    /// - `b`: value of the buffer's blue channel
    /// - `a`: value of the buffer's alpha channel
    #[inline]
    fn handle_create_u32_rgba_buffer(
        &mut self,
        slf: &Rc<WpSinglePixelBufferManagerV1>,
        id: &Rc<WlBuffer>,
        r: u32,
        g: u32,
        b: u32,
        a: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_u32_rgba_buffer(
            id,
            r,
            g,
            b,
            a,
        );
        if let Err(e) = res {
            log_forward("wp_single_pixel_buffer_manager_v1.create_u32_rgba_buffer", &e);
        }
    }
}

impl ObjectPrivate for WpSinglePixelBufferManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpSinglePixelBufferManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_single_pixel_buffer_manager_v1#{}.destroy()\n", client_id, id);
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
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 28)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_single_pixel_buffer_manager_v1#{}.create_u32_rgba_buffer(id: wl_buffer#{}, r: {}, g: {}, b: {}, a: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4);
                }
                let arg0_id = arg0;
                let arg0 = WlBuffer::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_u32_rgba_buffer(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultHandler.handle_create_u32_rgba_buffer(&self, arg0, arg1, arg2, arg3, arg4);
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
            0 => "destroy",
            1 => "create_u32_rgba_buffer",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WpSinglePixelBufferManagerV1 {
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

