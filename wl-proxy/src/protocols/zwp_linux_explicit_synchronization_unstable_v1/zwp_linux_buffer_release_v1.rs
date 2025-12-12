//! buffer release explicit synchronization
//!
//! This object is instantiated in response to a
//! zwp_linux_surface_synchronization_v1.get_release request.
//!
//! It provides an alternative to wl_buffer.release events, providing a
//! unique release from a single wl_surface.commit request. The release event
//! also supports explicit synchronization, providing a fence FD for the
//! client to synchronize against.
//!
//! Exactly one event, either a fenced_release or an immediate_release, will
//! be emitted for the wl_surface.commit request. The compositor can choose
//! release by release which event it uses.
//!
//! This event does not replace wl_buffer.release events; servers are still
//! required to send those events.
//!
//! Once a buffer release object has delivered a 'fenced_release' or an
//! 'immediate_release' event it is automatically destroyed.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_linux_buffer_release_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpLinuxBufferReleaseV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn ZwpLinuxBufferReleaseV1Handler>,
}

struct DefaultHandler;

impl ZwpLinuxBufferReleaseV1Handler for DefaultHandler { }

impl ZwpLinuxBufferReleaseV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ProxyInterface = ProxyInterface::ZwpLinuxBufferReleaseV1;
    pub const INTERFACE_NAME: &str = "zwp_linux_buffer_release_v1";
}

impl ZwpLinuxBufferReleaseV1 {
    pub fn set_handler(&self, handler: impl ZwpLinuxBufferReleaseV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpLinuxBufferReleaseV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpLinuxBufferReleaseV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpLinuxBufferReleaseV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpLinuxBufferReleaseV1 {
    /// Since when the fenced_release message is available.
    pub const MSG__FENCED_RELEASE__SINCE: u32 = 1;

    /// release buffer with fence
    ///
    /// Sent when the compositor has finalised its usage of the associated
    /// buffer for the relevant commit, providing a dma_fence which will be
    /// signaled when all operations by the compositor on that buffer for that
    /// commit have finished.
    ///
    /// Once the fence has signaled, and assuming the associated buffer is not
    /// pending release from other wl_surface.commit requests, no additional
    /// explicit or implicit synchronization is required to safely reuse or
    /// destroy the buffer.
    ///
    /// This event destroys the zwp_linux_buffer_release_v1 object.
    ///
    /// # Arguments
    ///
    /// - `fence`: fence for last operation on buffer
    #[inline]
    pub fn send_fenced_release(
        &self,
        fence: &Rc<OwnedFd>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            fence,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_linux_buffer_release_v1#{}.fenced_release(fence: {})\n", client.endpoint.id, id, arg0.as_raw_fd());
            self.core.state.log(args);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.fds.push_back(arg0.clone());
        fmt.words([
            id,
            0,
        ]);
        drop(fmt);
        drop(outgoing_ref);
        drop(client_ref);
        self.core.handle_client_destroy();
        Ok(())
    }

    /// Since when the immediate_release message is available.
    pub const MSG__IMMEDIATE_RELEASE__SINCE: u32 = 1;

    /// release buffer immediately
    ///
    /// Sent when the compositor has finalised its usage of the associated
    /// buffer for the relevant commit, and either performed no operations
    /// using it, or has a guarantee that all its operations on that buffer for
    /// that commit have finished.
    ///
    /// Once this event is received, and assuming the associated buffer is not
    /// pending release from other wl_surface.commit requests, no additional
    /// explicit or implicit synchronization is required to safely reuse or
    /// destroy the buffer.
    ///
    /// This event destroys the zwp_linux_buffer_release_v1 object.
    #[inline]
    pub fn send_immediate_release(
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_linux_buffer_release_v1#{}.immediate_release()\n", client.endpoint.id, id);
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
            1,
        ]);
        drop(fmt);
        drop(outgoing_ref);
        drop(client_ref);
        self.core.handle_client_destroy();
        Ok(())
    }
}

/// A message handler for [ZwpLinuxBufferReleaseV1] proxies.
pub trait ZwpLinuxBufferReleaseV1Handler: Any {
    /// release buffer with fence
    ///
    /// Sent when the compositor has finalised its usage of the associated
    /// buffer for the relevant commit, providing a dma_fence which will be
    /// signaled when all operations by the compositor on that buffer for that
    /// commit have finished.
    ///
    /// Once the fence has signaled, and assuming the associated buffer is not
    /// pending release from other wl_surface.commit requests, no additional
    /// explicit or implicit synchronization is required to safely reuse or
    /// destroy the buffer.
    ///
    /// This event destroys the zwp_linux_buffer_release_v1 object.
    ///
    /// # Arguments
    ///
    /// - `fence`: fence for last operation on buffer
    #[inline]
    fn fenced_release(
        &mut self,
        _slf: &Rc<ZwpLinuxBufferReleaseV1>,
        fence: &Rc<OwnedFd>,
    ) {
        let res = _slf.send_fenced_release(
            fence,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_linux_buffer_release_v1.fenced_release message: {}", Report::new(e));
        }
    }

    /// release buffer immediately
    ///
    /// Sent when the compositor has finalised its usage of the associated
    /// buffer for the relevant commit, and either performed no operations
    /// using it, or has a guarantee that all its operations on that buffer for
    /// that commit have finished.
    ///
    /// Once this event is received, and assuming the associated buffer is not
    /// pending release from other wl_surface.commit requests, no additional
    /// explicit or implicit synchronization is required to safely reuse or
    /// destroy the buffer.
    ///
    /// This event destroys the zwp_linux_buffer_release_v1 object.
    #[inline]
    fn immediate_release(
        &mut self,
        _slf: &Rc<ZwpLinuxBufferReleaseV1>,
    ) {
        let res = _slf.send_immediate_release(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_linux_buffer_release_v1.immediate_release message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for ZwpLinuxBufferReleaseV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::ZwpLinuxBufferReleaseV1, version),
            handler: Default::default(),
        })
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            n => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
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
                let Some(arg0) = fds.pop_front() else {
                    return Err(ObjectError::MissingFd("fence"));
                };
                let arg0 = &arg0;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_linux_buffer_release_v1#{}.fenced_release(fence: {})\n", msg[0], arg0.as_raw_fd());
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).fenced_release(&self, arg0);
                } else {
                    DefaultHandler.fenced_release(&self, arg0);
                }
                self.core.handle_server_destroy();
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_linux_buffer_release_v1#{}.immediate_release()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).immediate_release(&self);
                } else {
                    DefaultHandler.immediate_release(&self);
                }
                self.core.handle_server_destroy();
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
        let _ = id;
        None
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "fenced_release",
            1 => "immediate_release",
            _ => return None,
        };
        Some(name)
    }
}

impl Proxy for ZwpLinuxBufferReleaseV1 {
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

