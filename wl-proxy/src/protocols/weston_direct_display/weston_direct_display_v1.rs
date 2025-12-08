//! weston direct display
//!
//! Weston extension to instruct the compositor to avoid any import
//! of the dmabuf created by 'linux-dmabuf' protocol other than the display
//! controller.
//!
//! Compositors are already going to use direct scan-out as much as possible but
//! there's no assurance that while doing so, they won't first import the dmabuf
//! in to the GPU. This extension assures the client that the compositor will
//! never attempt to import in to the GPU and pass it directly to the display
//! controller.
//!
//! Clients can make use of this extension to pass the dmabuf buffer to the
//! display controller, potentially increasing the performance and lowering the
//! bandwidth usage.
//!
//! Lastly, clients can make use of this extension in tandem with content-protection
//! one thus avoiding any GPU interaction and providing a secure-content path.
//! Also, in some cases, the memory where dmabuf are allocated are in specially
//! crafted memory zone which would be seen as an illegal memory access when the
//! GPU will attempt to read it.
//!
//! WARNING: This interface by design might break screenshoting functionality
//! as compositing might be involved while doing that. Also, do note, that in
//! case the dmabufer provided can't be imported by KMS, the client connection
//! will be terminated.
//!
//! WARNING: This extension requires 'linux-dmabuf' protocol and
//! 'zwp_linux_buffer_params_v1' be already created by 'zwp_linux_buffer_v1'.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A weston_direct_display_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WestonDirectDisplayV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WestonDirectDisplayV1Handler>,
}

struct DefaultHandler;

impl WestonDirectDisplayV1Handler for DefaultHandler { }

impl ConcreteObject for WestonDirectDisplayV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WestonDirectDisplayV1;
    const INTERFACE_NAME: &str = "weston_direct_display_v1";
}

impl WestonDirectDisplayV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WestonDirectDisplayV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WestonDirectDisplayV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WestonDirectDisplayV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WestonDirectDisplayV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WestonDirectDisplayV1 {
    /// Since when the enable message is available.
    pub const MSG__ENABLE__SINCE: u32 = 1;

    /// forward buffer to display controller
    ///
    /// This request tells the compositor not to import the dmabuf to the GPU
    /// in order to bypass it entirely, such that the buffer will be directly
    /// scanned-out by the display controller. If HW is not capable/or there
    /// aren't any available resources to directly scan-out the buffer, a
    /// placeholder should be installed in-place by the compositor. The
    /// compositor may perform checks on the dmabuf and refuse to create a
    /// wl_buffer if the dmabuf seems unusable for being used directly.
    ///
    /// Assumes that 'zwp_linux_buffer_params_v1' was already created
    /// by 'zwp_linux_dmabuf_v1_create_params'.
    ///
    /// # Arguments
    ///
    /// - `dmabuf`: enable direct-display for dmabuf buffer
    #[inline]
    pub fn try_send_enable(
        &self,
        dmabuf: &Rc<ZwpLinuxBufferParamsV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            dmabuf,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("dmabuf"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_direct_display_v1#{}.enable(dmabuf: zwp_linux_buffer_params_v1#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
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
        ]);
        Ok(())
    }

    /// forward buffer to display controller
    ///
    /// This request tells the compositor not to import the dmabuf to the GPU
    /// in order to bypass it entirely, such that the buffer will be directly
    /// scanned-out by the display controller. If HW is not capable/or there
    /// aren't any available resources to directly scan-out the buffer, a
    /// placeholder should be installed in-place by the compositor. The
    /// compositor may perform checks on the dmabuf and refuse to create a
    /// wl_buffer if the dmabuf seems unusable for being used directly.
    ///
    /// Assumes that 'zwp_linux_buffer_params_v1' was already created
    /// by 'zwp_linux_dmabuf_v1_create_params'.
    ///
    /// # Arguments
    ///
    /// - `dmabuf`: enable direct-display for dmabuf buffer
    #[inline]
    pub fn send_enable(
        &self,
        dmabuf: &Rc<ZwpLinuxBufferParamsV1>,
    ) {
        let res = self.try_send_enable(
            dmabuf,
        );
        if let Err(e) = res {
            log_send("weston_direct_display_v1.enable", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy factory object
    ///
    /// Destroys the factory object, but does not affect any other objects.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_direct_display_v1#{}.destroy()\n", id);
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

    /// destroy factory object
    ///
    /// Destroys the factory object, but does not affect any other objects.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("weston_direct_display_v1.destroy", &e);
        }
    }
}

/// A message handler for [`WestonDirectDisplayV1`] proxies.
pub trait WestonDirectDisplayV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WestonDirectDisplayV1>) {
        slf.core.delete_id();
    }

    /// forward buffer to display controller
    ///
    /// This request tells the compositor not to import the dmabuf to the GPU
    /// in order to bypass it entirely, such that the buffer will be directly
    /// scanned-out by the display controller. If HW is not capable/or there
    /// aren't any available resources to directly scan-out the buffer, a
    /// placeholder should be installed in-place by the compositor. The
    /// compositor may perform checks on the dmabuf and refuse to create a
    /// wl_buffer if the dmabuf seems unusable for being used directly.
    ///
    /// Assumes that 'zwp_linux_buffer_params_v1' was already created
    /// by 'zwp_linux_dmabuf_v1_create_params'.
    ///
    /// # Arguments
    ///
    /// - `dmabuf`: enable direct-display for dmabuf buffer
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_enable(
        &mut self,
        slf: &Rc<WestonDirectDisplayV1>,
        dmabuf: &Rc<ZwpLinuxBufferParamsV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_enable(
            dmabuf,
        );
        if let Err(e) = res {
            log_forward("weston_direct_display_v1.enable", &e);
        }
    }

    /// destroy factory object
    ///
    /// Destroys the factory object, but does not affect any other objects.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WestonDirectDisplayV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("weston_direct_display_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for WestonDirectDisplayV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WestonDirectDisplayV1, version),
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_direct_display_v1#{}.enable(dmabuf: zwp_linux_buffer_params_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ZwpLinuxBufferParamsV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("dmabuf", o.core().interface, ObjectInterface::ZwpLinuxBufferParamsV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_enable(&self, arg0);
                } else {
                    DefaultHandler.handle_enable(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_direct_display_v1#{}.destroy()\n", client_id, id);
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
            0 => "enable",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WestonDirectDisplayV1 {
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

