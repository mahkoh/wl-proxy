//! adjust power management mode for an output
//!
//! This object offers requests to set the power management mode of
//! an output.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwlr_output_power_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwlrOutputPowerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwlrOutputPowerV1Handler>,
}

struct DefaultHandler;

impl ZwlrOutputPowerV1Handler for DefaultHandler { }

impl ConcreteObject for ZwlrOutputPowerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwlrOutputPowerV1;
    const INTERFACE_NAME: &str = "zwlr_output_power_v1";
}

impl ZwlrOutputPowerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwlrOutputPowerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwlrOutputPowerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwlrOutputPowerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwlrOutputPowerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwlrOutputPowerV1 {
    /// Since when the set_mode message is available.
    pub const MSG__SET_MODE__SINCE: u32 = 1;

    /// Set an outputs power save mode
    ///
    /// Set an output's power save mode to the given mode. The mode change
    /// is effective immediately. If the output does not support the given
    /// mode a failed event is sent.
    ///
    /// # Arguments
    ///
    /// - `mode`: the power save mode to set
    #[inline]
    pub fn try_send_set_mode(
        &self,
        mode: ZwlrOutputPowerV1Mode,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            mode,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: ZwlrOutputPowerV1Mode) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_power_v1#{}.set_mode(mode: {:?})\n", id, arg0);
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
            0,
            arg0.0,
        ]);
        Ok(())
    }

    /// Set an outputs power save mode
    ///
    /// Set an output's power save mode to the given mode. The mode change
    /// is effective immediately. If the output does not support the given
    /// mode a failed event is sent.
    ///
    /// # Arguments
    ///
    /// - `mode`: the power save mode to set
    #[inline]
    pub fn send_set_mode(
        &self,
        mode: ZwlrOutputPowerV1Mode,
    ) {
        let res = self.try_send_set_mode(
            mode,
        );
        if let Err(e) = res {
            log_send("zwlr_output_power_v1.set_mode", &e);
        }
    }

    /// Since when the mode message is available.
    pub const MSG__MODE__SINCE: u32 = 1;

    /// Report a power management mode change
    ///
    /// Report the power management mode change of an output.
    ///
    /// The mode event is sent after an output changed its power
    /// management mode. The reason can be a client using set_mode or the
    /// compositor deciding to change an output's mode.
    /// This event is also sent immediately when the object is created
    /// so the client is informed about the current power management mode.
    ///
    /// # Arguments
    ///
    /// - `mode`: the output's new power management mode
    #[inline]
    pub fn try_send_mode(
        &self,
        mode: ZwlrOutputPowerV1Mode,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            mode,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: ZwlrOutputPowerV1Mode) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_power_v1#{}.mode(mode: {:?})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// Report a power management mode change
    ///
    /// Report the power management mode change of an output.
    ///
    /// The mode event is sent after an output changed its power
    /// management mode. The reason can be a client using set_mode or the
    /// compositor deciding to change an output's mode.
    /// This event is also sent immediately when the object is created
    /// so the client is informed about the current power management mode.
    ///
    /// # Arguments
    ///
    /// - `mode`: the output's new power management mode
    #[inline]
    pub fn send_mode(
        &self,
        mode: ZwlrOutputPowerV1Mode,
    ) {
        let res = self.try_send_mode(
            mode,
        );
        if let Err(e) = res {
            log_send("zwlr_output_power_v1.mode", &e);
        }
    }

    /// Since when the failed message is available.
    pub const MSG__FAILED__SINCE: u32 = 1;

    /// object no longer valid
    ///
    /// This event indicates that the output power management mode control
    /// is no longer valid. This can happen for a number of reasons,
    /// including:
    /// - The output doesn't support power management
    /// - Another client already has exclusive power management mode control
    ///   for this output
    /// - The output disappeared
    ///
    /// Upon receiving this event, the client should destroy this object.
    #[inline]
    pub fn try_send_failed(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_power_v1#{}.failed()\n", client_id, id);
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
        Ok(())
    }

    /// object no longer valid
    ///
    /// This event indicates that the output power management mode control
    /// is no longer valid. This can happen for a number of reasons,
    /// including:
    /// - The output doesn't support power management
    /// - Another client already has exclusive power management mode control
    ///   for this output
    /// - The output disappeared
    ///
    /// Upon receiving this event, the client should destroy this object.
    #[inline]
    pub fn send_failed(
        &self,
    ) {
        let res = self.try_send_failed(
        );
        if let Err(e) = res {
            log_send("zwlr_output_power_v1.failed", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy this power management
    ///
    /// Destroys the output power management mode control object.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_power_v1#{}.destroy()\n", id);
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

    /// destroy this power management
    ///
    /// Destroys the output power management mode control object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwlr_output_power_v1.destroy", &e);
        }
    }
}

/// A message handler for [`ZwlrOutputPowerV1`] proxies.
pub trait ZwlrOutputPowerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwlrOutputPowerV1>) {
        slf.core.delete_id();
    }

    /// Set an outputs power save mode
    ///
    /// Set an output's power save mode to the given mode. The mode change
    /// is effective immediately. If the output does not support the given
    /// mode a failed event is sent.
    ///
    /// # Arguments
    ///
    /// - `mode`: the power save mode to set
    #[inline]
    fn handle_set_mode(
        &mut self,
        slf: &Rc<ZwlrOutputPowerV1>,
        mode: ZwlrOutputPowerV1Mode,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_mode(
            mode,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_power_v1.set_mode", &e);
        }
    }

    /// Report a power management mode change
    ///
    /// Report the power management mode change of an output.
    ///
    /// The mode event is sent after an output changed its power
    /// management mode. The reason can be a client using set_mode or the
    /// compositor deciding to change an output's mode.
    /// This event is also sent immediately when the object is created
    /// so the client is informed about the current power management mode.
    ///
    /// # Arguments
    ///
    /// - `mode`: the output's new power management mode
    #[inline]
    fn handle_mode(
        &mut self,
        slf: &Rc<ZwlrOutputPowerV1>,
        mode: ZwlrOutputPowerV1Mode,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_mode(
            mode,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_power_v1.mode", &e);
        }
    }

    /// object no longer valid
    ///
    /// This event indicates that the output power management mode control
    /// is no longer valid. This can happen for a number of reasons,
    /// including:
    /// - The output doesn't support power management
    /// - Another client already has exclusive power management mode control
    ///   for this output
    /// - The output disappeared
    ///
    /// Upon receiving this event, the client should destroy this object.
    #[inline]
    fn handle_failed(
        &mut self,
        slf: &Rc<ZwlrOutputPowerV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_failed(
        );
        if let Err(e) = res {
            log_forward("zwlr_output_power_v1.failed", &e);
        }
    }

    /// destroy this power management
    ///
    /// Destroys the output power management mode control object.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwlrOutputPowerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwlr_output_power_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for ZwlrOutputPowerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwlrOutputPowerV1, version),
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
                let arg0 = ZwlrOutputPowerV1Mode(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: ZwlrOutputPowerV1Mode) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_power_v1#{}.set_mode(mode: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_mode(&self, arg0);
                } else {
                    DefaultHandler.handle_set_mode(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_power_v1#{}.destroy()\n", client_id, id);
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
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = ZwlrOutputPowerV1Mode(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: ZwlrOutputPowerV1Mode) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_power_v1#{}.mode(mode: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_mode(&self, arg0);
                } else {
                    DefaultHandler.handle_mode(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_power_v1#{}.failed()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_failed(&self);
                } else {
                    DefaultHandler.handle_failed(&self);
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
        let name = match id {
            0 => "set_mode",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "mode",
            1 => "failed",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwlrOutputPowerV1 {
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

impl ZwlrOutputPowerV1 {
    /// Since when the mode.off enum variant is available.
    pub const ENM__MODE_OFF__SINCE: u32 = 1;
    /// Since when the mode.on enum variant is available.
    pub const ENM__MODE_ON__SINCE: u32 = 1;

    /// Since when the error.invalid_mode enum variant is available.
    pub const ENM__ERROR_INVALID_MODE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwlrOutputPowerV1Mode(pub u32);

impl ZwlrOutputPowerV1Mode {
    /// Output is turned off.
    pub const OFF: Self = Self(0);

    /// Output is turned on, no power saving
    pub const ON: Self = Self(1);
}

impl Debug for ZwlrOutputPowerV1Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::OFF => "OFF",
            Self::ON => "ON",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwlrOutputPowerV1Error(pub u32);

impl ZwlrOutputPowerV1Error {
    /// nonexistent power save mode
    pub const INVALID_MODE: Self = Self(1);
}

impl Debug for ZwlrOutputPowerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_MODE => "INVALID_MODE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
