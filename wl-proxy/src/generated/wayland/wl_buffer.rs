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
pub struct WlBuffer {
    core: ProxyCore,
    handler: HandlerHolder<dyn WlBufferHandler>,
}

struct DefaultHandler;

impl WlBufferHandler for DefaultHandler { }

impl WlBuffer {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "wl_buffer";
}

impl WlBuffer {
    pub fn set_handler(&self, handler: impl WlBufferHandler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn WlBufferHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlBuffer")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlBuffer {
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_buffer#{}.destroy()\n", id);
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_buffer#{}.release()\n", client.endpoint.id, id);
            self.core.state.log(args);
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
        ]);
        Ok(())
    }
}

/// A message handler for [WlBuffer] proxies.
#[allow(dead_code)]
pub trait WlBufferHandler: Any {
    /// destroy a buffer
    ///
    /// Destroy a buffer. If and how you need to release the backing
    /// storage is defined by the buffer factory interface.
    ///
    /// For possible side-effects to a surface, see wl_surface.attach.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<WlBuffer>,
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
        _slf: &Rc<WlBuffer>,
    ) {
        let res = _slf.send_release(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_buffer.release message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for WlBuffer {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::WlBuffer, version),
            handler: Default::default(),
        })
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_buffer#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
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
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_buffer#{}.release()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).release(&self);
                } else {
                    DefaultHandler.release(&self);
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

impl Proxy for WlBuffer {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn unset_handler(&self) {
        self.handler.set(None);
    }

    fn get_handler_any_ref(&self) -> Result<Ref<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(Ref::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<RefMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow_mut().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(RefMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

