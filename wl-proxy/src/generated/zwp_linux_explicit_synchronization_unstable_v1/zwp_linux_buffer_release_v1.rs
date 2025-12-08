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

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_linux_buffer_release_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpLinuxBufferReleaseV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpLinuxBufferReleaseV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpLinuxBufferReleaseV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpLinuxBufferReleaseV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZwpLinuxBufferReleaseV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpLinuxBufferReleaseV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwpLinuxBufferReleaseV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpLinuxBufferReleaseV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpLinuxBufferReleaseV1 {
    /// Since when the fenced_release message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.fds.push_back(arg0.clone());
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
        ]);
        drop(fmt);
        drop(outgoing_ref);
        drop(client_ref);
        self.core.handle_client_destroy();
        Ok(())
    }

    /// Since when the immediate_release message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
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
#[allow(dead_code)]
pub trait MetaZwpLinuxBufferReleaseV1MessageHandler {
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
        _slf: &Rc<MetaZwpLinuxBufferReleaseV1>,
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
        _slf: &Rc<MetaZwpLinuxBufferReleaseV1>,
    ) {
        let res = _slf.send_immediate_release(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_linux_buffer_release_v1.immediate_release message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpLinuxBufferReleaseV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            _ => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
    }

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let Some(arg0) = fds.pop_front() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).fenced_release(&self, arg0);
                } else {
                    DefaultMessageHandler.fenced_release(&self, arg0);
                }
                self.core.handle_server_destroy();
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).immediate_release(&self);
                } else {
                    DefaultMessageHandler.immediate_release(&self);
                }
                self.core.handle_server_destroy();
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

