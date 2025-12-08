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

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwlr_output_configuration_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwlrOutputConfigurationV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwlrOutputConfigurationV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwlrOutputConfigurationV1MessageHandler for DefaultMessageHandler { }

impl MetaZwlrOutputConfigurationV1 {
    pub const XML_VERSION: u32 = 4;
}

impl MetaZwlrOutputConfigurationV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwlrOutputConfigurationV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwlrOutputConfigurationV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwlrOutputConfigurationV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwlrOutputConfigurationV1 {
    /// Since when the enable_head message is available.
    #[allow(dead_code)]
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
        id: &Rc<MetaZwlrOutputConfigurationHeadV1>,
        head: &Rc<MetaZwlrOutputHeadV1>,
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
            return Err(ObjectError);
        };
        let arg1 = match arg1.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
        ]);
        Ok(())
    }

    /// Since when the disable_head message is available.
    #[allow(dead_code)]
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
        head: &Rc<MetaZwlrOutputHeadV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            head,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg0 = match arg0.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the apply message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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
    #[allow(dead_code)]
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
            0,
        ]);
        Ok(())
    }

    /// Since when the failed message is available.
    #[allow(dead_code)]
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
        Ok(())
    }

    /// Since when the cancelled message is available.
    #[allow(dead_code)]
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
            2,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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
#[allow(dead_code)]
pub trait MetaZwlrOutputConfigurationV1MessageHandler {
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
    fn enable_head(
        &mut self,
        _slf: &Rc<MetaZwlrOutputConfigurationV1>,
        id: &Rc<MetaZwlrOutputConfigurationHeadV1>,
        head: &Rc<MetaZwlrOutputHeadV1>,
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
    fn disable_head(
        &mut self,
        _slf: &Rc<MetaZwlrOutputConfigurationV1>,
        head: &Rc<MetaZwlrOutputHeadV1>,
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
    fn apply(
        &mut self,
        _slf: &Rc<MetaZwlrOutputConfigurationV1>,
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
    fn test(
        &mut self,
        _slf: &Rc<MetaZwlrOutputConfigurationV1>,
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
    fn succeeded(
        &mut self,
        _slf: &Rc<MetaZwlrOutputConfigurationV1>,
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
    fn failed(
        &mut self,
        _slf: &Rc<MetaZwlrOutputConfigurationV1>,
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
    fn cancelled(
        &mut self,
        _slf: &Rc<MetaZwlrOutputConfigurationV1>,
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
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwlrOutputConfigurationV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_configuration_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwlrOutputConfigurationV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaZwlrOutputConfigurationHeadV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.endpoint.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaZwlrOutputHeadV1>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).enable_head(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.enable_head(&self, arg0, arg1);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let Some(arg0) = client.endpoint.lookup(arg0) else {
                    return Err(ObjectError);
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaZwlrOutputHeadV1>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).disable_head(&self, arg0);
                } else {
                    DefaultMessageHandler.disable_head(&self, arg0);
                }
            }
            2 => {
                if let Some(handler) = handler {
                    (**handler).apply(&self);
                } else {
                    DefaultMessageHandler.apply(&self);
                }
            }
            3 => {
                if let Some(handler) = handler {
                    (**handler).test(&self);
                } else {
                    DefaultMessageHandler.test(&self);
                }
            }
            4 => {
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            _ => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if let Some(handler) = handler {
                    (**handler).succeeded(&self);
                } else {
                    DefaultMessageHandler.succeeded(&self);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).failed(&self);
                } else {
                    DefaultMessageHandler.failed(&self);
                }
            }
            2 => {
                if let Some(handler) = handler {
                    (**handler).cancelled(&self);
                } else {
                    DefaultMessageHandler.cancelled(&self);
                }
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

impl MetaZwlrOutputConfigurationV1 {
    /// Since when the error.already_configured_head enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ALREADY_CONFIGURED_HEAD__SINCE: u32 = 1;
    /// Since when the error.unconfigured_head enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_UNCONFIGURED_HEAD__SINCE: u32 = 1;
    /// Since when the error.already_used enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ALREADY_USED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaZwlrOutputConfigurationV1Error(pub u32);

impl MetaZwlrOutputConfigurationV1Error {
    /// head has been configured twice
    #[allow(dead_code)]
    pub const ALREADY_CONFIGURED_HEAD: Self = Self(1);

    /// head has not been configured
    #[allow(dead_code)]
    pub const UNCONFIGURED_HEAD: Self = Self(2);

    /// request sent after configuration has been applied or tested
    #[allow(dead_code)]
    pub const ALREADY_USED: Self = Self(3);
}

impl Debug for MetaZwlrOutputConfigurationV1Error {
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
