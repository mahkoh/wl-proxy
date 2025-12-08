//! Output configuration head extension object
//!
//! Extension to zwlr_output_configuration_head_v1.
//!
//! Adds additional/alternative parameters to the original zwlr_output_configuration_head_v1.
//!
//! Once the original `zwlr_output_configuration_head_v1` is destroyed this object will
//! become inert and all requests except `release` will be ignored.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zcosmic_output_configuration_head_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZcosmicOutputConfigurationHeadV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZcosmicOutputConfigurationHeadV1Handler>,
}

struct DefaultHandler;

impl ZcosmicOutputConfigurationHeadV1Handler for DefaultHandler { }

impl ConcreteObject for ZcosmicOutputConfigurationHeadV1 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::ZcosmicOutputConfigurationHeadV1;
    const INTERFACE_NAME: &str = "zcosmic_output_configuration_head_v1";
}

impl ZcosmicOutputConfigurationHeadV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZcosmicOutputConfigurationHeadV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZcosmicOutputConfigurationHeadV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZcosmicOutputConfigurationHeadV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZcosmicOutputConfigurationHeadV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZcosmicOutputConfigurationHeadV1 {
    /// Since when the set_scale_1000 message is available.
    pub const MSG__SET_SCALE_1000__SINCE: u32 = 1;

    /// set the scale multiplied by 1000
    ///
    /// This request sets the head's scale multiplied by 1000 for additional precision.
    ///
    /// This request is meant to be used in place of `zwlr_output_configuration_head_v1::set_scale`.
    /// Using `set_scale` and `set_scale_1000` at once will thus raise an `already_set` error on the
    /// original `zwlr_output_configuration_head_v1`.
    ///
    /// Any request conflicting with `set_scale` will also conflict with `set_scale_1000`.
    ///
    /// # Arguments
    ///
    /// - `scale_1000`:
    #[inline]
    pub fn try_send_set_scale_1000(
        &self,
        scale_1000: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            scale_1000,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_output_configuration_head_v1#{}.set_scale_1000(scale_1000: {})\n", id, arg0);
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
            arg0 as u32,
        ]);
        Ok(())
    }

    /// set the scale multiplied by 1000
    ///
    /// This request sets the head's scale multiplied by 1000 for additional precision.
    ///
    /// This request is meant to be used in place of `zwlr_output_configuration_head_v1::set_scale`.
    /// Using `set_scale` and `set_scale_1000` at once will thus raise an `already_set` error on the
    /// original `zwlr_output_configuration_head_v1`.
    ///
    /// Any request conflicting with `set_scale` will also conflict with `set_scale_1000`.
    ///
    /// # Arguments
    ///
    /// - `scale_1000`:
    #[inline]
    pub fn send_set_scale_1000(
        &self,
        scale_1000: i32,
    ) {
        let res = self.try_send_set_scale_1000(
            scale_1000,
        );
        if let Err(e) = res {
            log_send("zcosmic_output_configuration_head_v1.set_scale_1000", &e);
        }
    }

    /// Since when the release message is available.
    pub const MSG__RELEASE__SINCE: u32 = 1;

    /// destroy the output configuration head
    ///
    /// Using this request a client can tell the compositor that it is not going
    /// to use the configuration object anymore. Already issued requests will
    /// still be attached to the original `zwlr_output_configuration_head_v1`
    /// until it is destroyed.
    #[inline]
    pub fn try_send_release(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_output_configuration_head_v1#{}.release()\n", id);
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

    /// destroy the output configuration head
    ///
    /// Using this request a client can tell the compositor that it is not going
    /// to use the configuration object anymore. Already issued requests will
    /// still be attached to the original `zwlr_output_configuration_head_v1`
    /// until it is destroyed.
    #[inline]
    pub fn send_release(
        &self,
    ) {
        let res = self.try_send_release(
        );
        if let Err(e) = res {
            log_send("zcosmic_output_configuration_head_v1.release", &e);
        }
    }

    /// Since when the set_adaptive_sync_ext message is available.
    pub const MSG__SET_ADAPTIVE_SYNC_EXT__SINCE: u32 = 2;

    /// set adaptive sync state
    ///
    /// This request requests a new adaptive sync state.
    ///
    /// This request is meant to be used in place of `zwlr_output_configuration_head_v1::set_adaptive_sync`.
    /// Using `set_adaptive_sync` and `set_adaptive_sync_ext` at once will thus raise an `already_set` error on the
    /// original `zwlr_output_configuration_head_v1`.
    ///
    /// Any request conflicting with `set_adaptive_sync` will also conflict with `set_adaptive_sync_ext`.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_set_adaptive_sync_ext(
        &self,
        state: ZcosmicOutputHeadV1AdaptiveSyncStateExt,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            state,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: ZcosmicOutputHeadV1AdaptiveSyncStateExt) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_output_configuration_head_v1#{}.set_adaptive_sync_ext(state: {:?})\n", id, arg0);
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
            2,
            arg0.0,
        ]);
        Ok(())
    }

    /// set adaptive sync state
    ///
    /// This request requests a new adaptive sync state.
    ///
    /// This request is meant to be used in place of `zwlr_output_configuration_head_v1::set_adaptive_sync`.
    /// Using `set_adaptive_sync` and `set_adaptive_sync_ext` at once will thus raise an `already_set` error on the
    /// original `zwlr_output_configuration_head_v1`.
    ///
    /// Any request conflicting with `set_adaptive_sync` will also conflict with `set_adaptive_sync_ext`.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_set_adaptive_sync_ext(
        &self,
        state: ZcosmicOutputHeadV1AdaptiveSyncStateExt,
    ) {
        let res = self.try_send_set_adaptive_sync_ext(
            state,
        );
        if let Err(e) = res {
            log_send("zcosmic_output_configuration_head_v1.set_adaptive_sync_ext", &e);
        }
    }
}

/// A message handler for [`ZcosmicOutputConfigurationHeadV1`] proxies.
pub trait ZcosmicOutputConfigurationHeadV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZcosmicOutputConfigurationHeadV1>) {
        slf.core.delete_id();
    }

    /// set the scale multiplied by 1000
    ///
    /// This request sets the head's scale multiplied by 1000 for additional precision.
    ///
    /// This request is meant to be used in place of `zwlr_output_configuration_head_v1::set_scale`.
    /// Using `set_scale` and `set_scale_1000` at once will thus raise an `already_set` error on the
    /// original `zwlr_output_configuration_head_v1`.
    ///
    /// Any request conflicting with `set_scale` will also conflict with `set_scale_1000`.
    ///
    /// # Arguments
    ///
    /// - `scale_1000`:
    #[inline]
    fn handle_set_scale_1000(
        &mut self,
        slf: &Rc<ZcosmicOutputConfigurationHeadV1>,
        scale_1000: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_scale_1000(
            scale_1000,
        );
        if let Err(e) = res {
            log_forward("zcosmic_output_configuration_head_v1.set_scale_1000", &e);
        }
    }

    /// destroy the output configuration head
    ///
    /// Using this request a client can tell the compositor that it is not going
    /// to use the configuration object anymore. Already issued requests will
    /// still be attached to the original `zwlr_output_configuration_head_v1`
    /// until it is destroyed.
    #[inline]
    fn handle_release(
        &mut self,
        slf: &Rc<ZcosmicOutputConfigurationHeadV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_release(
        );
        if let Err(e) = res {
            log_forward("zcosmic_output_configuration_head_v1.release", &e);
        }
    }

    /// set adaptive sync state
    ///
    /// This request requests a new adaptive sync state.
    ///
    /// This request is meant to be used in place of `zwlr_output_configuration_head_v1::set_adaptive_sync`.
    /// Using `set_adaptive_sync` and `set_adaptive_sync_ext` at once will thus raise an `already_set` error on the
    /// original `zwlr_output_configuration_head_v1`.
    ///
    /// Any request conflicting with `set_adaptive_sync` will also conflict with `set_adaptive_sync_ext`.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_set_adaptive_sync_ext(
        &mut self,
        slf: &Rc<ZcosmicOutputConfigurationHeadV1>,
        state: ZcosmicOutputHeadV1AdaptiveSyncStateExt,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_adaptive_sync_ext(
            state,
        );
        if let Err(e) = res {
            log_forward("zcosmic_output_configuration_head_v1.set_adaptive_sync_ext", &e);
        }
    }
}

impl ObjectPrivate for ZcosmicOutputConfigurationHeadV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZcosmicOutputConfigurationHeadV1, version),
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
                let arg0 = arg0 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_output_configuration_head_v1#{}.set_scale_1000(scale_1000: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_scale_1000(&self, arg0);
                } else {
                    DefaultHandler.handle_set_scale_1000(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_output_configuration_head_v1#{}.release()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_release(&self);
                } else {
                    DefaultHandler.handle_release(&self);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = ZcosmicOutputHeadV1AdaptiveSyncStateExt(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: ZcosmicOutputHeadV1AdaptiveSyncStateExt) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_output_configuration_head_v1#{}.set_adaptive_sync_ext(state: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_adaptive_sync_ext(&self, arg0);
                } else {
                    DefaultHandler.handle_set_adaptive_sync_ext(&self, arg0);
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
            0 => "set_scale_1000",
            1 => "release",
            2 => "set_adaptive_sync_ext",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for ZcosmicOutputConfigurationHeadV1 {
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

