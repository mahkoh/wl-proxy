//! weston restricted buffer
//!
//! Weston extension to instruct the compositor that the dmabuf created by
//! 'linux-dmabuf' protocol is in protected memory. Protected has hardware
//! level access restrictions, and if used improperly will result in
//! compositor crashes, hardware faults, or other manner of undefined
//! behaviour.
//!
//! Restricted buffers may only be imported to the GPU if the GPU is using a
//! restricted context - otherwise if the buffer is to be rendered, it will
//! be censored (displayed as a solid color).
//!
//! Outside of GPU restrictions, the compositor's policy dictates when
//! restricted buffers may be displayed.
//!
//! This extension depends on the 'linux-dmabuf' protocol. After a
//! 'zwp_linux_buffer_params_v1' object is created by 'zwp_linux_buffer_v1'
//! and before it is used to create a buffer, this extension's 'enable'
//! request may be used to indicate that the buffer is restricted.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A weston_restricted_buffer_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WestonRestrictedBufferV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WestonRestrictedBufferV1Handler>,
}

struct DefaultHandler;

impl WestonRestrictedBufferV1Handler for DefaultHandler { }

impl ConcreteObject for WestonRestrictedBufferV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WestonRestrictedBufferV1;
    const INTERFACE_NAME: &str = "weston_restricted_buffer_v1";
}

impl WestonRestrictedBufferV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WestonRestrictedBufferV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WestonRestrictedBufferV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WestonRestrictedBufferV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WestonRestrictedBufferV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WestonRestrictedBufferV1 {
    /// Since when the enable message is available.
    pub const MSG__ENABLE__SINCE: u32 = 1;

    /// Flag the buffer as restricted
    ///
    /// This request tells the compositor that the buffer is restricted.
    ///
    /// Even if the GPU is not using a restricted context and the dmabuf can't
    /// be scanned out directly, the compositor should censor it with a
    /// placeholder instead of refusing to create the wl_buffer.
    ///
    /// Assumes that 'zwp_linux_buffer_params_v1' was already created
    /// by 'zwp_linux_dmabuf_v1_create_params', as stated in
    /// 'weston_restricted_buffer_v1' interface description.
    ///
    /// # Arguments
    ///
    /// - `dmabuf`: enable restricted handling of dmabuf
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_restricted_buffer_v1#{}.enable(dmabuf: zwp_linux_buffer_params_v1#{})\n", id, arg0);
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

    /// Flag the buffer as restricted
    ///
    /// This request tells the compositor that the buffer is restricted.
    ///
    /// Even if the GPU is not using a restricted context and the dmabuf can't
    /// be scanned out directly, the compositor should censor it with a
    /// placeholder instead of refusing to create the wl_buffer.
    ///
    /// Assumes that 'zwp_linux_buffer_params_v1' was already created
    /// by 'zwp_linux_dmabuf_v1_create_params', as stated in
    /// 'weston_restricted_buffer_v1' interface description.
    ///
    /// # Arguments
    ///
    /// - `dmabuf`: enable restricted handling of dmabuf
    #[inline]
    pub fn send_enable(
        &self,
        dmabuf: &Rc<ZwpLinuxBufferParamsV1>,
    ) {
        let res = self.try_send_enable(
            dmabuf,
        );
        if let Err(e) = res {
            log_send("weston_restricted_buffer_v1.enable", &e);
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_restricted_buffer_v1#{}.destroy()\n", id);
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
            log_send("weston_restricted_buffer_v1.destroy", &e);
        }
    }
}

/// A message handler for [`WestonRestrictedBufferV1`] proxies.
pub trait WestonRestrictedBufferV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WestonRestrictedBufferV1>) {
        slf.core.delete_id();
    }

    /// Flag the buffer as restricted
    ///
    /// This request tells the compositor that the buffer is restricted.
    ///
    /// Even if the GPU is not using a restricted context and the dmabuf can't
    /// be scanned out directly, the compositor should censor it with a
    /// placeholder instead of refusing to create the wl_buffer.
    ///
    /// Assumes that 'zwp_linux_buffer_params_v1' was already created
    /// by 'zwp_linux_dmabuf_v1_create_params', as stated in
    /// 'weston_restricted_buffer_v1' interface description.
    ///
    /// # Arguments
    ///
    /// - `dmabuf`: enable restricted handling of dmabuf
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_enable(
        &mut self,
        slf: &Rc<WestonRestrictedBufferV1>,
        dmabuf: &Rc<ZwpLinuxBufferParamsV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_enable(
            dmabuf,
        );
        if let Err(e) = res {
            log_forward("weston_restricted_buffer_v1.enable", &e);
        }
    }

    /// destroy factory object
    ///
    /// Destroys the factory object, but does not affect any other objects.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WestonRestrictedBufferV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("weston_restricted_buffer_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for WestonRestrictedBufferV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WestonRestrictedBufferV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_restricted_buffer_v1#{}.enable(dmabuf: zwp_linux_buffer_params_v1#{})\n", client_id, id, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_restricted_buffer_v1#{}.destroy()\n", client_id, id);
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

    fn create_zombie(&self) -> Rc<dyn Object> {
        let slf = Self::new(&self.core.state, self.core.version);
        slf.core.make_zombie();
        slf
    }
}

impl Object for WestonRestrictedBufferV1 {
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

impl WestonRestrictedBufferV1 {
    /// Since when the error.params_already_used enum variant is available.
    pub const ENM__ERROR_PARAMS_ALREADY_USED__SINCE: u32 = 1;
    /// Since when the error.already_set enum variant is available.
    pub const ENM__ERROR_ALREADY_SET__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WestonRestrictedBufferV1Error(pub u32);

impl WestonRestrictedBufferV1Error {
    /// dmabuf params already used
    pub const PARAMS_ALREADY_USED: Self = Self(0);

    /// restricted already set
    pub const ALREADY_SET: Self = Self(1);
}

impl Debug for WestonRestrictedBufferV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::PARAMS_ALREADY_USED => "PARAMS_ALREADY_USED",
            Self::ALREADY_SET => "ALREADY_SET",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
