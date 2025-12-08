//! content for a wl_surface
//!
//! A buffer provides the content for a wl_surface. Buffers are
//! created through factory interfaces such as wl_shm, wp_linux_buffer_params
//! (from the linux-dmabuf protocol extension) or similar. It has a width and
//! a height and can be attached to a wl_surface, but the mechanism by which a
//! client provides and updates the contents is defined by the buffer factory
//! interface.
//!
//! Color channels are assumed to be electrical rather than optical (in other
//! words, encoded with a transfer function) unless otherwise specified. If
//! the buffer uses a format that has an alpha channel, the alpha channel is
//! assumed to be premultiplied into the electrical color channel values
//! (after transfer function encoding) unless otherwise specified.
//!
//! Note, because wl_buffer objects are created from multiple independent
//! factory interfaces, the wl_buffer interface is frozen at version 1.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wl_buffer proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWlBuffer {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWlBufferMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWlBufferMessageHandler for DefaultMessageHandler { }

impl MetaWlBuffer {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWlBuffer {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WlBuffer, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWlBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWlBuffer")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWlBuffer {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy a buffer
    ///
    /// Destroy a buffer. If and how you need to release the backing
    /// storage is defined by the buffer factory interface.
    ///
    /// For possible side-effects to a surface, see wl_surface.attach.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= wl_buffer#{}.destroy()", id);
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

    /// Since when the release message is available.
    #[allow(dead_code)]
    pub const MSG__RELEASE__SINCE: u32 = 1;

    /// compositor releases buffer
    ///
    /// Sent when this wl_buffer is no longer used by the compositor.
    ///
    /// For more information on when release events may or may not be sent,
    /// and what consequences it has, please see the description of
    /// wl_surface.attach.
    ///
    /// If a client receives a release event before the frame callback
    /// requested in the same wl_surface.commit that attaches this
    /// wl_buffer to a surface, then the client is immediately free to
    /// reuse the buffer and its backing storage, and does not need a
    /// second buffer for the next surface content update. Typically
    /// this is possible, when the compositor maintains a copy of the
    /// wl_surface contents, e.g. as a GL texture. This is an important
    /// optimization for GL(ES) compositors with wl_shm clients.
    #[inline]
    pub fn send_release(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= wl_buffer#{}.release()", client.endpoint.id, id);
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
        ]);
        Ok(())
    }
}

/// A message handler for [WlBuffer] proxies.
#[allow(dead_code)]
pub trait MetaWlBufferMessageHandler {
    /// destroy a buffer
    ///
    /// Destroy a buffer. If and how you need to release the backing
    /// storage is defined by the buffer factory interface.
    ///
    /// For possible side-effects to a surface, see wl_surface.attach.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWlBuffer>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_buffer.destroy message: {}", Report::new(e));
        }
    }

    /// compositor releases buffer
    ///
    /// Sent when this wl_buffer is no longer used by the compositor.
    ///
    /// For more information on when release events may or may not be sent,
    /// and what consequences it has, please see the description of
    /// wl_surface.attach.
    ///
    /// If a client receives a release event before the frame callback
    /// requested in the same wl_surface.commit that attaches this
    /// wl_buffer to a surface, then the client is immediately free to
    /// reuse the buffer and its backing storage, and does not need a
    /// second buffer for the next surface content update. Typically
    /// this is possible, when the compositor maintains a copy of the
    /// wl_surface contents, e.g. as a GL texture. This is an important
    /// optimization for GL(ES) compositors with wl_shm clients.
    #[inline]
    fn release(
        &mut self,
        _slf: &Rc<MetaWlBuffer>,
    ) {
        let res = _slf.send_release(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_buffer.release message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWlBuffer {
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
                eprintln!("client#{:04} -> wl_buffer#{}.destroy()", client.endpoint.id, msg[0]);
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
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
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("server      -> wl_buffer#{}.release()", msg[0]);
                if let Some(handler) = handler {
                    (**handler).release(&self);
                } else {
                    DefaultMessageHandler.release(&self);
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
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "release",
            _ => return None,
        };
        Some(name)
    }
}

