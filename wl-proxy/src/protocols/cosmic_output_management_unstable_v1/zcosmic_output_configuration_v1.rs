//! Output configuration extension object
//!
//! Extension to zwlr_output_configuration_v1.
//!
//! Adds additional parameters to be tested/applyed via the original zwlr_output_configuration_v1.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zcosmic_output_configuration_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZcosmicOutputConfigurationV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZcosmicOutputConfigurationV1Handler>,
}

struct DefaultHandler;

impl ZcosmicOutputConfigurationV1Handler for DefaultHandler { }

impl ConcreteObject for ZcosmicOutputConfigurationV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZcosmicOutputConfigurationV1;
    const INTERFACE_NAME: &str = "zcosmic_output_configuration_v1";
}

impl ZcosmicOutputConfigurationV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZcosmicOutputConfigurationV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZcosmicOutputConfigurationV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZcosmicOutputConfigurationV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZcosmicOutputConfigurationV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZcosmicOutputConfigurationV1 {
    /// Since when the mirror_head message is available.
    pub const MSG__MIRROR_HEAD__SINCE: u32 = 1;

    /// enable and configure a head to mirror another head
    ///
    /// Enable a head mirroring another.
    ///
    /// This request creates a head configuration object that can be used to change the head's properties.
    ///
    /// This is an alternative to `zwlr_output_configuration_v1::enable_head` or `zwlr_output_configuration_v1::disable_head`
    /// Using either with the same `head` argument will result in an `already_configured_head` error on the original
    /// `zwlr_output_configuration_v1` object.
    ///
    /// All properties are still required to be set to the resulting `zwlr_output_configuration_head` by the client
    /// as denoted in the original protocol. Some like `set_position` however might be ignored in mirroring configurations.
    ///
    /// Trying to set a disabled or mirroring head as `mirroring` or calling `disable_head`/`mirror_head` after using a head
    /// as a `mirroring` argument will raise a `mirrored_head_busy` protocol error.
    ///
    /// # Arguments
    ///
    /// - `id`: a new object to configure the head
    /// - `head`: the head to be enabled
    /// - `mirroring`: the head to be mirrored
    #[inline]
    pub fn try_send_mirror_head(
        &self,
        id: &Rc<ZwlrOutputConfigurationHeadV1>,
        head: &Rc<ZwlrOutputHeadV1>,
        mirroring: &Rc<ZwlrOutputHeadV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            id,
            head,
            mirroring,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let arg2 = arg2.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("head"))),
            Some(id) => id,
        };
        let arg2_id = match arg2.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("mirroring"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_output_configuration_v1#{}.mirror_head(id: zwlr_output_configuration_head_v1#{}, head: zwlr_output_head_v1#{}, mirroring: zwlr_output_head_v1#{})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id, arg2_id);
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
            arg1_id,
            arg2_id,
        ]);
        Ok(())
    }

    /// enable and configure a head to mirror another head
    ///
    /// Enable a head mirroring another.
    ///
    /// This request creates a head configuration object that can be used to change the head's properties.
    ///
    /// This is an alternative to `zwlr_output_configuration_v1::enable_head` or `zwlr_output_configuration_v1::disable_head`
    /// Using either with the same `head` argument will result in an `already_configured_head` error on the original
    /// `zwlr_output_configuration_v1` object.
    ///
    /// All properties are still required to be set to the resulting `zwlr_output_configuration_head` by the client
    /// as denoted in the original protocol. Some like `set_position` however might be ignored in mirroring configurations.
    ///
    /// Trying to set a disabled or mirroring head as `mirroring` or calling `disable_head`/`mirror_head` after using a head
    /// as a `mirroring` argument will raise a `mirrored_head_busy` protocol error.
    ///
    /// # Arguments
    ///
    /// - `id`: a new object to configure the head
    /// - `head`: the head to be enabled
    /// - `mirroring`: the head to be mirrored
    #[inline]
    pub fn send_mirror_head(
        &self,
        id: &Rc<ZwlrOutputConfigurationHeadV1>,
        head: &Rc<ZwlrOutputHeadV1>,
        mirroring: &Rc<ZwlrOutputHeadV1>,
    ) {
        let res = self.try_send_mirror_head(
            id,
            head,
            mirroring,
        );
        if let Err(e) = res {
            log_send("zcosmic_output_configuration_v1.mirror_head", &e);
        }
    }

    /// enable and configure a head to mirror another head
    ///
    /// Enable a head mirroring another.
    ///
    /// This request creates a head configuration object that can be used to change the head's properties.
    ///
    /// This is an alternative to `zwlr_output_configuration_v1::enable_head` or `zwlr_output_configuration_v1::disable_head`
    /// Using either with the same `head` argument will result in an `already_configured_head` error on the original
    /// `zwlr_output_configuration_v1` object.
    ///
    /// All properties are still required to be set to the resulting `zwlr_output_configuration_head` by the client
    /// as denoted in the original protocol. Some like `set_position` however might be ignored in mirroring configurations.
    ///
    /// Trying to set a disabled or mirroring head as `mirroring` or calling `disable_head`/`mirror_head` after using a head
    /// as a `mirroring` argument will raise a `mirrored_head_busy` protocol error.
    ///
    /// # Arguments
    ///
    /// - `head`: the head to be enabled
    /// - `mirroring`: the head to be mirrored
    #[inline]
    pub fn new_try_send_mirror_head(
        &self,
        head: &Rc<ZwlrOutputHeadV1>,
        mirroring: &Rc<ZwlrOutputHeadV1>,
    ) -> Result<Rc<ZwlrOutputConfigurationHeadV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_mirror_head(
            &id,
            head,
            mirroring,
        )?;
        Ok(id)
    }

    /// enable and configure a head to mirror another head
    ///
    /// Enable a head mirroring another.
    ///
    /// This request creates a head configuration object that can be used to change the head's properties.
    ///
    /// This is an alternative to `zwlr_output_configuration_v1::enable_head` or `zwlr_output_configuration_v1::disable_head`
    /// Using either with the same `head` argument will result in an `already_configured_head` error on the original
    /// `zwlr_output_configuration_v1` object.
    ///
    /// All properties are still required to be set to the resulting `zwlr_output_configuration_head` by the client
    /// as denoted in the original protocol. Some like `set_position` however might be ignored in mirroring configurations.
    ///
    /// Trying to set a disabled or mirroring head as `mirroring` or calling `disable_head`/`mirror_head` after using a head
    /// as a `mirroring` argument will raise a `mirrored_head_busy` protocol error.
    ///
    /// # Arguments
    ///
    /// - `head`: the head to be enabled
    /// - `mirroring`: the head to be mirrored
    #[inline]
    pub fn new_send_mirror_head(
        &self,
        head: &Rc<ZwlrOutputHeadV1>,
        mirroring: &Rc<ZwlrOutputHeadV1>,
    ) -> Rc<ZwlrOutputConfigurationHeadV1> {
        let id = self.core.create_child();
        self.send_mirror_head(
            &id,
            head,
            mirroring,
        );
        id
    }

    /// Since when the finished message is available.
    pub const MSG__FINISHED__SINCE: u32 = 1;

    /// the configuration was used
    ///
    /// This event indicates that the configuration is no longer available.
    ///
    /// This usually happens when the original configuration was `cancelled`, `suceeded` or `failed`.
    ///
    /// Upon receiving this event, the client should destroy this object.
    ///
    /// The configration object becomes inert and any requests other than `destroy` will be ignored.
    #[inline]
    pub fn try_send_finished(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zcosmic_output_configuration_v1#{}.finished()\n", client_id, id);
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
            0,
        ]);
        Ok(())
    }

    /// the configuration was used
    ///
    /// This event indicates that the configuration is no longer available.
    ///
    /// This usually happens when the original configuration was `cancelled`, `suceeded` or `failed`.
    ///
    /// Upon receiving this event, the client should destroy this object.
    ///
    /// The configration object becomes inert and any requests other than `destroy` will be ignored.
    #[inline]
    pub fn send_finished(
        &self,
    ) {
        let res = self.try_send_finished(
        );
        if let Err(e) = res {
            log_send("zcosmic_output_configuration_v1.finished", &e);
        }
    }

    /// Since when the release message is available.
    pub const MSG__RELEASE__SINCE: u32 = 1;

    /// destroy the output configuration
    ///
    /// Using this request a client can tell the compositor that it is not going
    /// to use the configuration object anymore. Any changes to the outputs
    /// will still be attached to the original `zwlr_output_configuration_head_v1`
    /// if it isn't destroyed.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_output_configuration_v1#{}.release()\n", id);
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

    /// destroy the output configuration
    ///
    /// Using this request a client can tell the compositor that it is not going
    /// to use the configuration object anymore. Any changes to the outputs
    /// will still be attached to the original `zwlr_output_configuration_head_v1`
    /// if it isn't destroyed.
    #[inline]
    pub fn send_release(
        &self,
    ) {
        let res = self.try_send_release(
        );
        if let Err(e) = res {
            log_send("zcosmic_output_configuration_v1.release", &e);
        }
    }
}

/// A message handler for [`ZcosmicOutputConfigurationV1`] proxies.
pub trait ZcosmicOutputConfigurationV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZcosmicOutputConfigurationV1>) {
        slf.core.delete_id();
    }

    /// enable and configure a head to mirror another head
    ///
    /// Enable a head mirroring another.
    ///
    /// This request creates a head configuration object that can be used to change the head's properties.
    ///
    /// This is an alternative to `zwlr_output_configuration_v1::enable_head` or `zwlr_output_configuration_v1::disable_head`
    /// Using either with the same `head` argument will result in an `already_configured_head` error on the original
    /// `zwlr_output_configuration_v1` object.
    ///
    /// All properties are still required to be set to the resulting `zwlr_output_configuration_head` by the client
    /// as denoted in the original protocol. Some like `set_position` however might be ignored in mirroring configurations.
    ///
    /// Trying to set a disabled or mirroring head as `mirroring` or calling `disable_head`/`mirror_head` after using a head
    /// as a `mirroring` argument will raise a `mirrored_head_busy` protocol error.
    ///
    /// # Arguments
    ///
    /// - `id`: a new object to configure the head
    /// - `head`: the head to be enabled
    /// - `mirroring`: the head to be mirrored
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_mirror_head(
        &mut self,
        slf: &Rc<ZcosmicOutputConfigurationV1>,
        id: &Rc<ZwlrOutputConfigurationHeadV1>,
        head: &Rc<ZwlrOutputHeadV1>,
        mirroring: &Rc<ZwlrOutputHeadV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_mirror_head(
            id,
            head,
            mirroring,
        );
        if let Err(e) = res {
            log_forward("zcosmic_output_configuration_v1.mirror_head", &e);
        }
    }

    /// the configuration was used
    ///
    /// This event indicates that the configuration is no longer available.
    ///
    /// This usually happens when the original configuration was `cancelled`, `suceeded` or `failed`.
    ///
    /// Upon receiving this event, the client should destroy this object.
    ///
    /// The configration object becomes inert and any requests other than `destroy` will be ignored.
    #[inline]
    fn handle_finished(
        &mut self,
        slf: &Rc<ZcosmicOutputConfigurationV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_finished(
        );
        if let Err(e) = res {
            log_forward("zcosmic_output_configuration_v1.finished", &e);
        }
    }

    /// destroy the output configuration
    ///
    /// Using this request a client can tell the compositor that it is not going
    /// to use the configuration object anymore. Any changes to the outputs
    /// will still be attached to the original `zwlr_output_configuration_head_v1`
    /// if it isn't destroyed.
    #[inline]
    fn handle_release(
        &mut self,
        slf: &Rc<ZcosmicOutputConfigurationV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_release(
        );
        if let Err(e) = res {
            log_forward("zcosmic_output_configuration_v1.release", &e);
        }
    }
}

impl ObjectPrivate for ZcosmicOutputConfigurationV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZcosmicOutputConfigurationV1, version),
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
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_output_configuration_v1#{}.mirror_head(id: zwlr_output_configuration_head_v1#{}, head: zwlr_output_head_v1#{}, mirroring: zwlr_output_head_v1#{})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let arg0 = ZwlrOutputConfigurationHeadV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<ZwlrOutputHeadV1>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("head", o.core().interface, ObjectInterface::ZwlrOutputHeadV1)));
                };
                let arg2_id = arg2;
                let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg2_id)));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<ZwlrOutputHeadV1>() else {
                    let o = client.endpoint.lookup(arg2_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("mirroring", o.core().interface, ObjectInterface::ZwlrOutputHeadV1)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).handle_mirror_head(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_mirror_head(&self, arg0, arg1, arg2);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_output_configuration_v1#{}.release()\n", client_id, id);
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
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zcosmic_output_configuration_v1#{}.finished()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_finished(&self);
                } else {
                    DefaultHandler.handle_finished(&self);
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
            0 => "mirror_head",
            1 => "release",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "finished",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZcosmicOutputConfigurationV1 {
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

impl ZcosmicOutputConfigurationV1 {
    /// Since when the error.already_finished enum variant is available.
    pub const ENM__ERROR_ALREADY_FINISHED__SINCE: u32 = 1;
    /// Since when the error.mirrored_head_busy enum variant is available.
    pub const ENM__ERROR_MIRRORED_HEAD_BUSY__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZcosmicOutputConfigurationV1Error(pub u32);

impl ZcosmicOutputConfigurationV1Error {
    /// underlying configuration has already been used
    pub const ALREADY_FINISHED: Self = Self(1);

    /// mirrored head is not enabled
    pub const MIRRORED_HEAD_BUSY: Self = Self(2);
}

impl Debug for ZcosmicOutputConfigurationV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ALREADY_FINISHED => "ALREADY_FINISHED",
            Self::MIRRORED_HEAD_BUSY => "MIRRORED_HEAD_BUSY",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
