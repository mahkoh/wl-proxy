//! output configuration
//!
//! This object is used by the client to describe a full output configuration.
//!
//! First, the client needs to setup the output configuration. Each head can
//! be either enabled (and configured) or disabled. It is a protocol error to
//! send two enable_head or disable_head requests with the same head. It is a
//! protocol error to omit a head in a configuration.
//!
//! Then, the client can apply or test the configuration. The compositor will
//! then reply with a succeeded, failed or cancelled event. Finally the client
//! should destroy the configuration object.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwlr_output_configuration_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwlrOutputConfigurationV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwlrOutputConfigurationV1Handler>,
}

struct DefaultHandler;

impl ZwlrOutputConfigurationV1Handler for DefaultHandler { }

impl ZwlrOutputConfigurationV1 {
    pub const XML_VERSION: u32 = 4;
    pub const INTERFACE: ObjectInterface = ObjectInterface::ZwlrOutputConfigurationV1;
    pub const INTERFACE_NAME: &str = "zwlr_output_configuration_v1";
}

impl ZwlrOutputConfigurationV1 {
    pub fn set_handler(&self, handler: impl ZwlrOutputConfigurationV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZwlrOutputConfigurationV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwlrOutputConfigurationV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwlrOutputConfigurationV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwlrOutputConfigurationV1 {
    /// Since when the enable_head message is available.
    pub const MSG__ENABLE_HEAD__SINCE: u32 = 1;

    /// enable and configure a head
    ///
    /// Enable a head. This request creates a head configuration object that can
    /// be used to change the head's properties.
    ///
    /// # Arguments
    ///
    /// - `id`: a new object to configure the head
    /// - `head`: the head to be enabled
    #[inline]
    pub fn send_enable_head(
        &self,
        id: &Rc<ZwlrOutputConfigurationHeadV1>,
        head: &Rc<ZwlrOutputHeadV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            head,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("head")),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_configuration_v1#{}.enable_head(id: zwlr_output_configuration_head_v1#{}, head: zwlr_output_head_v1#{})\n", id, arg0_id, arg1_id);
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
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// Since when the disable_head message is available.
    pub const MSG__DISABLE_HEAD__SINCE: u32 = 1;

    /// disable a head
    ///
    /// Disable a head.
    ///
    /// # Arguments
    ///
    /// - `head`: the head to be disabled
    #[inline]
    pub fn send_disable_head(
        &self,
        head: &Rc<ZwlrOutputHeadV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            head,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("head")),
            Some(id) => id,
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_configuration_v1#{}.disable_head(head: zwlr_output_head_v1#{})\n", id, arg0_id);
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
            1,
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the apply message is available.
    pub const MSG__APPLY__SINCE: u32 = 1;

    /// apply the configuration
    ///
    /// Apply the new output configuration.
    ///
    /// In case the configuration is successfully applied, there is no guarantee
    /// that the new output state matches completely the requested
    /// configuration. For instance, a compositor might round the scale if it
    /// doesn't support fractional scaling.
    ///
    /// After this request has been sent, the compositor must respond with an
    /// succeeded, failed or cancelled event. Sending a request that isn't the
    /// destructor is a protocol error.
    #[inline]
    pub fn send_apply(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_configuration_v1#{}.apply()\n", id);
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
            2,
        ]);
        Ok(())
    }

    /// Since when the test message is available.
    pub const MSG__TEST__SINCE: u32 = 1;

    /// test the configuration
    ///
    /// Test the new output configuration. The configuration won't be applied,
    /// but will only be validated.
    ///
    /// Even if the compositor succeeds to test a configuration, applying it may
    /// fail.
    ///
    /// After this request has been sent, the compositor must respond with an
    /// succeeded, failed or cancelled event. Sending a request that isn't the
    /// destructor is a protocol error.
    #[inline]
    pub fn send_test(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_configuration_v1#{}.test()\n", id);
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
            3,
        ]);
        Ok(())
    }

    /// Since when the succeeded message is available.
    pub const MSG__SUCCEEDED__SINCE: u32 = 1;

    /// configuration changes succeeded
    ///
    /// Sent after the compositor has successfully applied the changes or
    /// tested them.
    ///
    /// Upon receiving this event, the client should destroy this object.
    ///
    /// If the current configuration has changed, events to describe the changes
    /// will be sent followed by a wlr_output_manager.done event.
    #[inline]
    pub fn send_succeeded(
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_configuration_v1#{}.succeeded()\n", client.endpoint.id, id);
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

    /// Since when the failed message is available.
    pub const MSG__FAILED__SINCE: u32 = 1;

    /// configuration changes failed
    ///
    /// Sent if the compositor rejects the changes or failed to apply them. The
    /// compositor should revert any changes made by the apply request that
    /// triggered this event.
    ///
    /// Upon receiving this event, the client should destroy this object.
    #[inline]
    pub fn send_failed(
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_configuration_v1#{}.failed()\n", client.endpoint.id, id);
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
        Ok(())
    }

    /// Since when the cancelled message is available.
    pub const MSG__CANCELLED__SINCE: u32 = 1;

    /// configuration has been cancelled
    ///
    /// Sent if the compositor cancels the configuration because the state of an
    /// output changed and the client has outdated information (e.g. after an
    /// output has been hotplugged).
    ///
    /// The client can create a new configuration with a newer serial and try
    /// again.
    ///
    /// Upon receiving this event, the client should destroy this object.
    #[inline]
    pub fn send_cancelled(
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_configuration_v1#{}.cancelled()\n", client.endpoint.id, id);
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
            2,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the output configuration
    ///
    /// Using this request a client can tell the compositor that it is not going
    /// to use the configuration object anymore. Any changes to the outputs
    /// that have not been applied will be discarded.
    ///
    /// This request also destroys wlr_output_configuration_head objects created
    /// via this object.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_configuration_v1#{}.destroy()\n", id);
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
            4,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [ZwlrOutputConfigurationV1] proxies.
pub trait ZwlrOutputConfigurationV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwlrOutputConfigurationV1>) {
        let _ = slf.core.delete_id();
    }

    /// enable and configure a head
    ///
    /// Enable a head. This request creates a head configuration object that can
    /// be used to change the head's properties.
    ///
    /// # Arguments
    ///
    /// - `id`: a new object to configure the head
    /// - `head`: the head to be enabled
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_enable_head(
        &mut self,
        _slf: &Rc<ZwlrOutputConfigurationV1>,
        id: &Rc<ZwlrOutputConfigurationHeadV1>,
        head: &Rc<ZwlrOutputHeadV1>,
    ) {
        let res = _slf.send_enable_head(
            id,
            head,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_configuration_v1.enable_head message: {}", Report::new(e));
        }
    }

    /// disable a head
    ///
    /// Disable a head.
    ///
    /// # Arguments
    ///
    /// - `head`: the head to be disabled
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_disable_head(
        &mut self,
        _slf: &Rc<ZwlrOutputConfigurationV1>,
        head: &Rc<ZwlrOutputHeadV1>,
    ) {
        let res = _slf.send_disable_head(
            head,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_configuration_v1.disable_head message: {}", Report::new(e));
        }
    }

    /// apply the configuration
    ///
    /// Apply the new output configuration.
    ///
    /// In case the configuration is successfully applied, there is no guarantee
    /// that the new output state matches completely the requested
    /// configuration. For instance, a compositor might round the scale if it
    /// doesn't support fractional scaling.
    ///
    /// After this request has been sent, the compositor must respond with an
    /// succeeded, failed or cancelled event. Sending a request that isn't the
    /// destructor is a protocol error.
    #[inline]
    fn handle_apply(
        &mut self,
        _slf: &Rc<ZwlrOutputConfigurationV1>,
    ) {
        let res = _slf.send_apply(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_configuration_v1.apply message: {}", Report::new(e));
        }
    }

    /// test the configuration
    ///
    /// Test the new output configuration. The configuration won't be applied,
    /// but will only be validated.
    ///
    /// Even if the compositor succeeds to test a configuration, applying it may
    /// fail.
    ///
    /// After this request has been sent, the compositor must respond with an
    /// succeeded, failed or cancelled event. Sending a request that isn't the
    /// destructor is a protocol error.
    #[inline]
    fn handle_test(
        &mut self,
        _slf: &Rc<ZwlrOutputConfigurationV1>,
    ) {
        let res = _slf.send_test(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_configuration_v1.test message: {}", Report::new(e));
        }
    }

    /// configuration changes succeeded
    ///
    /// Sent after the compositor has successfully applied the changes or
    /// tested them.
    ///
    /// Upon receiving this event, the client should destroy this object.
    ///
    /// If the current configuration has changed, events to describe the changes
    /// will be sent followed by a wlr_output_manager.done event.
    #[inline]
    fn handle_succeeded(
        &mut self,
        _slf: &Rc<ZwlrOutputConfigurationV1>,
    ) {
        let res = _slf.send_succeeded(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_configuration_v1.succeeded message: {}", Report::new(e));
        }
    }

    /// configuration changes failed
    ///
    /// Sent if the compositor rejects the changes or failed to apply them. The
    /// compositor should revert any changes made by the apply request that
    /// triggered this event.
    ///
    /// Upon receiving this event, the client should destroy this object.
    #[inline]
    fn handle_failed(
        &mut self,
        _slf: &Rc<ZwlrOutputConfigurationV1>,
    ) {
        let res = _slf.send_failed(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_configuration_v1.failed message: {}", Report::new(e));
        }
    }

    /// configuration has been cancelled
    ///
    /// Sent if the compositor cancels the configuration because the state of an
    /// output changed and the client has outdated information (e.g. after an
    /// output has been hotplugged).
    ///
    /// The client can create a new configuration with a newer serial and try
    /// again.
    ///
    /// Upon receiving this event, the client should destroy this object.
    #[inline]
    fn handle_cancelled(
        &mut self,
        _slf: &Rc<ZwlrOutputConfigurationV1>,
    ) {
        let res = _slf.send_cancelled(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_configuration_v1.cancelled message: {}", Report::new(e));
        }
    }

    /// destroy the output configuration
    ///
    /// Using this request a client can tell the compositor that it is not going
    /// to use the configuration object anymore. Any changes to the outputs
    /// that have not been applied will be discarded.
    ///
    /// This request also destroys wlr_output_configuration_head objects created
    /// via this object.
    #[inline]
    fn handle_destroy(
        &mut self,
        _slf: &Rc<ZwlrOutputConfigurationV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_configuration_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for ZwlrOutputConfigurationV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwlrOutputConfigurationV1, version),
            handler: Default::default(),
        })
    }

    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err((ObjectError::HandlerBorrowed, self));
        };
        if let Some(handler) = &mut *handler {
            handler.delete_id(&self);
        } else {
            let _ = self.core.delete_id();
        }
        Ok(())
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_configuration_v1#{}.enable_head(id: zwlr_output_configuration_head_v1#{}, head: zwlr_output_head_v1#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ZwlrOutputConfigurationHeadV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<ZwlrOutputHeadV1>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("head", o.core().interface, ObjectInterface::ZwlrOutputHeadV1));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_enable_head(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_enable_head(&self, arg0, arg1);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_configuration_v1#{}.disable_head(head: zwlr_output_head_v1#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ZwlrOutputHeadV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("head", o.core().interface, ObjectInterface::ZwlrOutputHeadV1));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_disable_head(&self, arg0);
                } else {
                    DefaultHandler.handle_disable_head(&self, arg0);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_configuration_v1#{}.apply()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_apply(&self);
                } else {
                    DefaultHandler.handle_apply(&self);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_configuration_v1#{}.test()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_test(&self);
                } else {
                    DefaultHandler.handle_test(&self);
                }
            }
            4 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_configuration_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_configuration_v1#{}.succeeded()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_succeeded(&self);
                } else {
                    DefaultHandler.handle_succeeded(&self);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_configuration_v1#{}.failed()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_failed(&self);
                } else {
                    DefaultHandler.handle_failed(&self);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_configuration_v1#{}.cancelled()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_cancelled(&self);
                } else {
                    DefaultHandler.handle_cancelled(&self);
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
            0 => "enable_head",
            1 => "disable_head",
            2 => "apply",
            3 => "test",
            4 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "succeeded",
            1 => "failed",
            2 => "cancelled",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwlrOutputConfigurationV1 {
    fn core(&self) -> &ObjectCore {
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

impl ZwlrOutputConfigurationV1 {
    /// Since when the error.already_configured_head enum variant is available.
    pub const ENM__ERROR_ALREADY_CONFIGURED_HEAD__SINCE: u32 = 1;
    /// Since when the error.unconfigured_head enum variant is available.
    pub const ENM__ERROR_UNCONFIGURED_HEAD__SINCE: u32 = 1;
    /// Since when the error.already_used enum variant is available.
    pub const ENM__ERROR_ALREADY_USED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwlrOutputConfigurationV1Error(pub u32);

impl ZwlrOutputConfigurationV1Error {
    /// head has been configured twice
    pub const ALREADY_CONFIGURED_HEAD: Self = Self(1);

    /// head has not been configured
    pub const UNCONFIGURED_HEAD: Self = Self(2);

    /// request sent after configuration has been applied or tested
    pub const ALREADY_USED: Self = Self(3);
}

impl Debug for ZwlrOutputConfigurationV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ALREADY_CONFIGURED_HEAD => "ALREADY_CONFIGURED_HEAD",
            Self::UNCONFIGURED_HEAD => "UNCONFIGURED_HEAD",
            Self::ALREADY_USED => "ALREADY_USED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
