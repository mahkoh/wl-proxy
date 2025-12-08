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

/// A zwp_linux_buffer_release_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpLinuxBufferReleaseV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpLinuxBufferReleaseV1Handler>,
}

struct DefaultHandler;

impl ZwpLinuxBufferReleaseV1Handler for DefaultHandler { }

impl ConcreteObject for ZwpLinuxBufferReleaseV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpLinuxBufferReleaseV1;
    const INTERFACE_NAME: &str = "zwp_linux_buffer_release_v1";
}

impl ZwpLinuxBufferReleaseV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpLinuxBufferReleaseV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
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
    pub fn try_send_fenced_release(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_linux_buffer_release_v1#{}.fenced_release(fence: {})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0.as_raw_fd());
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
    ) {
        let res = self.try_send_fenced_release(
            fence,
        );
        if let Err(e) = res {
            log_send("zwp_linux_buffer_release_v1.fenced_release", &e);
        }
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
    pub fn try_send_immediate_release(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_linux_buffer_release_v1#{}.immediate_release()\n", client_id, id);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id);
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
    ) {
        let res = self.try_send_immediate_release(
        );
        if let Err(e) = res {
            log_send("zwp_linux_buffer_release_v1.immediate_release", &e);
        }
    }
}

/// A message handler for [`ZwpLinuxBufferReleaseV1`] proxies.
pub trait ZwpLinuxBufferReleaseV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpLinuxBufferReleaseV1>) {
        slf.core.delete_id();
    }

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
    fn handle_fenced_release(
        &mut self,
        slf: &Rc<ZwpLinuxBufferReleaseV1>,
        fence: &Rc<OwnedFd>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_fenced_release(
            fence,
        );
        if let Err(e) = res {
            log_forward("zwp_linux_buffer_release_v1.fenced_release", &e);
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
    fn handle_immediate_release(
        &mut self,
        slf: &Rc<ZwpLinuxBufferReleaseV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_immediate_release(
        );
        if let Err(e) = res {
            log_forward("zwp_linux_buffer_release_v1.immediate_release", &e);
        }
    }
}

impl ObjectPrivate for ZwpLinuxBufferReleaseV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpLinuxBufferReleaseV1, version),
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
            n => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
    }

    fn handle_event(self: Rc<Self>, server: &Endpoint, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                let Some(arg0) = fds.pop_front() else {
                    return Err(ObjectError(ObjectErrorKind::MissingFd("fence")));
                };
                let arg0 = &arg0;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_linux_buffer_release_v1#{}.fenced_release(fence: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0.as_raw_fd());
                }
                self.core.handle_server_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_fenced_release(&self, arg0);
                } else {
                    DefaultHandler.handle_fenced_release(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_linux_buffer_release_v1#{}.immediate_release()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                self.core.handle_server_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_immediate_release(&self);
                } else {
                    DefaultHandler.handle_immediate_release(&self);
                }
            }
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
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

impl Object for ZwpLinuxBufferReleaseV1 {
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

