//! global factory for single-pixel buffers
//!
//! The wp_single_pixel_buffer_manager_v1 interface is a factory for
//! single-pixel buffers.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_single_pixel_buffer_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpSinglePixelBufferManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpSinglePixelBufferManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpSinglePixelBufferManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaWpSinglePixelBufferManagerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWpSinglePixelBufferManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WpSinglePixelBufferManagerV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpSinglePixelBufferManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpSinglePixelBufferManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpSinglePixelBufferManagerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the manager
    ///
    /// Destroy the wp_single_pixel_buffer_manager_v1 object.
    ///
    /// The child objects created via this interface are unaffected.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= wp_single_pixel_buffer_manager_v1#{}.destroy()", id);
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

    /// Since when the create_u32_rgba_buffer message is available.
    #[allow(dead_code)]
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
    pub fn send_create_u32_rgba_buffer(
        &self,
        id: &Rc<MetaWlBuffer>,
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        eprintln!("server      <= wp_single_pixel_buffer_manager_v1#{}.create_u32_rgba_buffer(id: wl_buffer#{}, r: {}, g: {}, b: {}, a: {})", id, arg0_id, arg1, arg2, arg3, arg4);
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
            arg0_id,
            arg1,
            arg2,
            arg3,
            arg4,
        ]);
        Ok(())
    }
}

/// A message handler for [WpSinglePixelBufferManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaWpSinglePixelBufferManagerV1MessageHandler {
    /// destroy the manager
    ///
    /// Destroy the wp_single_pixel_buffer_manager_v1 object.
    ///
    /// The child objects created via this interface are unaffected.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpSinglePixelBufferManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_single_pixel_buffer_manager_v1.destroy message: {}", Report::new(e));
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
    fn create_u32_rgba_buffer(
        &mut self,
        _slf: &Rc<MetaWpSinglePixelBufferManagerV1>,
        id: &Rc<MetaWlBuffer>,
        r: u32,
        g: u32,
        b: u32,
        a: u32,
    ) {
        let res = _slf.send_create_u32_rgba_buffer(
            id,
            r,
            g,
            b,
            a,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_single_pixel_buffer_manager_v1.create_u32_rgba_buffer message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpSinglePixelBufferManagerV1 {
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
                eprintln!("client#{:04} -> wp_single_pixel_buffer_manager_v1#{}.destroy()", client.endpoint.id, msg[0]);
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            1 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 28));
                };
                eprintln!("client#{:04} -> wp_single_pixel_buffer_manager_v1#{}.create_u32_rgba_buffer(id: wl_buffer#{}, r: {}, g: {}, b: {}, a: {})", client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4);
                let arg0_id = arg0;
                let arg0 = MetaWlBuffer::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).create_u32_rgba_buffer(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultMessageHandler.create_u32_rgba_buffer(&self, arg0, arg1, arg2, arg3, arg4);
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

