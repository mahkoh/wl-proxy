//! Output configuration manager
//!
//! This interface provides extension points for wlr-output-management types.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zcosmic_output_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZcosmicOutputManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZcosmicOutputManagerV1Handler>,
}

struct DefaultHandler;

impl ZcosmicOutputManagerV1Handler for DefaultHandler { }

impl ConcreteObject for ZcosmicOutputManagerV1 {
    const XML_VERSION: u32 = 3;
    const INTERFACE: ObjectInterface = ObjectInterface::ZcosmicOutputManagerV1;
    const INTERFACE_NAME: &str = "zcosmic_output_manager_v1";
}

impl ZcosmicOutputManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZcosmicOutputManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZcosmicOutputManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZcosmicOutputManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZcosmicOutputManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZcosmicOutputManagerV1 {
    /// Since when the get_head message is available.
    pub const MSG__GET_HEAD__SINCE: u32 = 1;

    /// Get a zcosmic_output_head_v1 for an existing zwlr_output_head_v1
    ///
    /// Gets an extension object for zwlr_output_head_v1.
    ///
    /// As soon as the extended output is created, events will be dispatched with an accompanying
    /// `done`-event delivered to the matching `zwlr_output_manager_v1` afterwards.
    ///
    /// Any further updates will produce new events, if properties of the zcosmic_output_head_v1 change,
    /// just like for the original `zwlr_output_head_v1`. Events should be handled as atomic, as denoted
    /// by `zwlr_output_manager_v1::done`.
    ///
    /// Trying to create more than one zcosmic_output_head_v1 per zwlr_output_head_v1 will raise an
    /// "already_extended" error.
    ///
    /// # Arguments
    ///
    /// - `extended`:
    /// - `head`:
    #[inline]
    pub fn try_send_get_head(
        &self,
        extended: &Rc<ZcosmicOutputHeadV1>,
        head: &Rc<ZwlrOutputHeadV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            extended,
            head,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("head"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("extended", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_output_manager_v1#{}.get_head(extended: zcosmic_output_head_v1#{}, head: zwlr_output_head_v1#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id);
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
        ]);
        Ok(())
    }

    /// Get a zcosmic_output_head_v1 for an existing zwlr_output_head_v1
    ///
    /// Gets an extension object for zwlr_output_head_v1.
    ///
    /// As soon as the extended output is created, events will be dispatched with an accompanying
    /// `done`-event delivered to the matching `zwlr_output_manager_v1` afterwards.
    ///
    /// Any further updates will produce new events, if properties of the zcosmic_output_head_v1 change,
    /// just like for the original `zwlr_output_head_v1`. Events should be handled as atomic, as denoted
    /// by `zwlr_output_manager_v1::done`.
    ///
    /// Trying to create more than one zcosmic_output_head_v1 per zwlr_output_head_v1 will raise an
    /// "already_extended" error.
    ///
    /// # Arguments
    ///
    /// - `extended`:
    /// - `head`:
    #[inline]
    pub fn send_get_head(
        &self,
        extended: &Rc<ZcosmicOutputHeadV1>,
        head: &Rc<ZwlrOutputHeadV1>,
    ) {
        let res = self.try_send_get_head(
            extended,
            head,
        );
        if let Err(e) = res {
            log_send("zcosmic_output_manager_v1.get_head", &e);
        }
    }

    /// Get a zcosmic_output_head_v1 for an existing zwlr_output_head_v1
    ///
    /// Gets an extension object for zwlr_output_head_v1.
    ///
    /// As soon as the extended output is created, events will be dispatched with an accompanying
    /// `done`-event delivered to the matching `zwlr_output_manager_v1` afterwards.
    ///
    /// Any further updates will produce new events, if properties of the zcosmic_output_head_v1 change,
    /// just like for the original `zwlr_output_head_v1`. Events should be handled as atomic, as denoted
    /// by `zwlr_output_manager_v1::done`.
    ///
    /// Trying to create more than one zcosmic_output_head_v1 per zwlr_output_head_v1 will raise an
    /// "already_extended" error.
    ///
    /// # Arguments
    ///
    /// - `head`:
    #[inline]
    pub fn new_try_send_get_head(
        &self,
        head: &Rc<ZwlrOutputHeadV1>,
    ) -> Result<Rc<ZcosmicOutputHeadV1>, ObjectError> {
        let extended = self.core.create_child();
        self.try_send_get_head(
            &extended,
            head,
        )?;
        Ok(extended)
    }

    /// Get a zcosmic_output_head_v1 for an existing zwlr_output_head_v1
    ///
    /// Gets an extension object for zwlr_output_head_v1.
    ///
    /// As soon as the extended output is created, events will be dispatched with an accompanying
    /// `done`-event delivered to the matching `zwlr_output_manager_v1` afterwards.
    ///
    /// Any further updates will produce new events, if properties of the zcosmic_output_head_v1 change,
    /// just like for the original `zwlr_output_head_v1`. Events should be handled as atomic, as denoted
    /// by `zwlr_output_manager_v1::done`.
    ///
    /// Trying to create more than one zcosmic_output_head_v1 per zwlr_output_head_v1 will raise an
    /// "already_extended" error.
    ///
    /// # Arguments
    ///
    /// - `head`:
    #[inline]
    pub fn new_send_get_head(
        &self,
        head: &Rc<ZwlrOutputHeadV1>,
    ) -> Rc<ZcosmicOutputHeadV1> {
        let extended = self.core.create_child();
        self.send_get_head(
            &extended,
            head,
        );
        extended
    }

    /// Since when the get_configuration message is available.
    pub const MSG__GET_CONFIGURATION__SINCE: u32 = 1;

    /// Get a zcosmic_output_configuration_v1 for an existing zwlr_output_configuration_v1
    ///
    /// Gets an extension object for zwlr_output_configuration_v1.
    ///
    /// Trying to create more than one zcosmic_output_configuration_v1 per zwlr_output_configuration_v1
    /// will raise an "already_extended" error.
    ///
    /// # Arguments
    ///
    /// - `extended`:
    /// - `config`:
    #[inline]
    pub fn try_send_get_configuration(
        &self,
        extended: &Rc<ZcosmicOutputConfigurationV1>,
        config: &Rc<ZwlrOutputConfigurationV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            extended,
            config,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("config"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("extended", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_output_manager_v1#{}.get_configuration(extended: zcosmic_output_configuration_v1#{}, config: zwlr_output_configuration_v1#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id);
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
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// Get a zcosmic_output_configuration_v1 for an existing zwlr_output_configuration_v1
    ///
    /// Gets an extension object for zwlr_output_configuration_v1.
    ///
    /// Trying to create more than one zcosmic_output_configuration_v1 per zwlr_output_configuration_v1
    /// will raise an "already_extended" error.
    ///
    /// # Arguments
    ///
    /// - `extended`:
    /// - `config`:
    #[inline]
    pub fn send_get_configuration(
        &self,
        extended: &Rc<ZcosmicOutputConfigurationV1>,
        config: &Rc<ZwlrOutputConfigurationV1>,
    ) {
        let res = self.try_send_get_configuration(
            extended,
            config,
        );
        if let Err(e) = res {
            log_send("zcosmic_output_manager_v1.get_configuration", &e);
        }
    }

    /// Get a zcosmic_output_configuration_v1 for an existing zwlr_output_configuration_v1
    ///
    /// Gets an extension object for zwlr_output_configuration_v1.
    ///
    /// Trying to create more than one zcosmic_output_configuration_v1 per zwlr_output_configuration_v1
    /// will raise an "already_extended" error.
    ///
    /// # Arguments
    ///
    /// - `config`:
    #[inline]
    pub fn new_try_send_get_configuration(
        &self,
        config: &Rc<ZwlrOutputConfigurationV1>,
    ) -> Result<Rc<ZcosmicOutputConfigurationV1>, ObjectError> {
        let extended = self.core.create_child();
        self.try_send_get_configuration(
            &extended,
            config,
        )?;
        Ok(extended)
    }

    /// Get a zcosmic_output_configuration_v1 for an existing zwlr_output_configuration_v1
    ///
    /// Gets an extension object for zwlr_output_configuration_v1.
    ///
    /// Trying to create more than one zcosmic_output_configuration_v1 per zwlr_output_configuration_v1
    /// will raise an "already_extended" error.
    ///
    /// # Arguments
    ///
    /// - `config`:
    #[inline]
    pub fn new_send_get_configuration(
        &self,
        config: &Rc<ZwlrOutputConfigurationV1>,
    ) -> Rc<ZcosmicOutputConfigurationV1> {
        let extended = self.core.create_child();
        self.send_get_configuration(
            &extended,
            config,
        );
        extended
    }

    /// Since when the get_configuration_head message is available.
    pub const MSG__GET_CONFIGURATION_HEAD__SINCE: u32 = 1;

    /// Get a zcosmic_output_configuration_head_v1 for an existing zwlr_output_configuration_head_v1
    ///
    /// Gets an extension object for zwlr_output_configuration_head_v1.
    ///
    /// Trying to create more than one zcosmic_output_configuration_head_v1 per
    /// zwlr_output_configuration_head_v1 will raise an "already_extended" error.
    ///
    /// # Arguments
    ///
    /// - `extended`:
    /// - `config_head`:
    #[inline]
    pub fn try_send_get_configuration_head(
        &self,
        extended: &Rc<ZcosmicOutputConfigurationHeadV1>,
        config_head: &Rc<ZwlrOutputConfigurationHeadV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            extended,
            config_head,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("config_head"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("extended", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_output_manager_v1#{}.get_configuration_head(extended: zcosmic_output_configuration_head_v1#{}, config_head: zwlr_output_configuration_head_v1#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id);
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
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// Get a zcosmic_output_configuration_head_v1 for an existing zwlr_output_configuration_head_v1
    ///
    /// Gets an extension object for zwlr_output_configuration_head_v1.
    ///
    /// Trying to create more than one zcosmic_output_configuration_head_v1 per
    /// zwlr_output_configuration_head_v1 will raise an "already_extended" error.
    ///
    /// # Arguments
    ///
    /// - `extended`:
    /// - `config_head`:
    #[inline]
    pub fn send_get_configuration_head(
        &self,
        extended: &Rc<ZcosmicOutputConfigurationHeadV1>,
        config_head: &Rc<ZwlrOutputConfigurationHeadV1>,
    ) {
        let res = self.try_send_get_configuration_head(
            extended,
            config_head,
        );
        if let Err(e) = res {
            log_send("zcosmic_output_manager_v1.get_configuration_head", &e);
        }
    }

    /// Get a zcosmic_output_configuration_head_v1 for an existing zwlr_output_configuration_head_v1
    ///
    /// Gets an extension object for zwlr_output_configuration_head_v1.
    ///
    /// Trying to create more than one zcosmic_output_configuration_head_v1 per
    /// zwlr_output_configuration_head_v1 will raise an "already_extended" error.
    ///
    /// # Arguments
    ///
    /// - `config_head`:
    #[inline]
    pub fn new_try_send_get_configuration_head(
        &self,
        config_head: &Rc<ZwlrOutputConfigurationHeadV1>,
    ) -> Result<Rc<ZcosmicOutputConfigurationHeadV1>, ObjectError> {
        let extended = self.core.create_child();
        self.try_send_get_configuration_head(
            &extended,
            config_head,
        )?;
        Ok(extended)
    }

    /// Get a zcosmic_output_configuration_head_v1 for an existing zwlr_output_configuration_head_v1
    ///
    /// Gets an extension object for zwlr_output_configuration_head_v1.
    ///
    /// Trying to create more than one zcosmic_output_configuration_head_v1 per
    /// zwlr_output_configuration_head_v1 will raise an "already_extended" error.
    ///
    /// # Arguments
    ///
    /// - `config_head`:
    #[inline]
    pub fn new_send_get_configuration_head(
        &self,
        config_head: &Rc<ZwlrOutputConfigurationHeadV1>,
    ) -> Rc<ZcosmicOutputConfigurationHeadV1> {
        let extended = self.core.create_child();
        self.send_get_configuration_head(
            &extended,
            config_head,
        );
        extended
    }

    /// Since when the release message is available.
    pub const MSG__RELEASE__SINCE: u32 = 1;

    /// Destroy this global
    ///
    /// Destroys this global. All previously created objects remain valid.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_output_manager_v1#{}.release()\n", id);
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
            3,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Destroy this global
    ///
    /// Destroys this global. All previously created objects remain valid.
    #[inline]
    pub fn send_release(
        &self,
    ) {
        let res = self.try_send_release(
        );
        if let Err(e) = res {
            log_send("zcosmic_output_manager_v1.release", &e);
        }
    }

    /// Since when the set_xwayland_primary message is available.
    pub const MSG__SET_XWAYLAND_PRIMARY__SINCE: u32 = 3;

    /// set head as the primary for xwayland
    ///
    /// This requests a head to be advertised as the primary output via randr to Xwayland.
    ///
    /// No head has to be marked primary, if `null` is passed Xwayland won't advertise a primary output.
    /// Sending a disabled head will be ignored to avoid races.
    ///
    /// # Arguments
    ///
    /// - `head`: head to be advertised as primary
    #[inline]
    pub fn try_send_set_xwayland_primary(
        &self,
        head: Option<&Rc<ZcosmicOutputHeadV1>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            head,
        );
        let arg0 = arg0.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("head"))),
                Some(id) => id,
            },
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_output_manager_v1#{}.set_xwayland_primary(head: zcosmic_output_head_v1#{})\n", id, arg0);
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
            4,
            arg0_id,
        ]);
        Ok(())
    }

    /// set head as the primary for xwayland
    ///
    /// This requests a head to be advertised as the primary output via randr to Xwayland.
    ///
    /// No head has to be marked primary, if `null` is passed Xwayland won't advertise a primary output.
    /// Sending a disabled head will be ignored to avoid races.
    ///
    /// # Arguments
    ///
    /// - `head`: head to be advertised as primary
    #[inline]
    pub fn send_set_xwayland_primary(
        &self,
        head: Option<&Rc<ZcosmicOutputHeadV1>>,
    ) {
        let res = self.try_send_set_xwayland_primary(
            head,
        );
        if let Err(e) = res {
            log_send("zcosmic_output_manager_v1.set_xwayland_primary", &e);
        }
    }
}

/// A message handler for [`ZcosmicOutputManagerV1`] proxies.
pub trait ZcosmicOutputManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZcosmicOutputManagerV1>) {
        slf.core.delete_id();
    }

    /// Get a zcosmic_output_head_v1 for an existing zwlr_output_head_v1
    ///
    /// Gets an extension object for zwlr_output_head_v1.
    ///
    /// As soon as the extended output is created, events will be dispatched with an accompanying
    /// `done`-event delivered to the matching `zwlr_output_manager_v1` afterwards.
    ///
    /// Any further updates will produce new events, if properties of the zcosmic_output_head_v1 change,
    /// just like for the original `zwlr_output_head_v1`. Events should be handled as atomic, as denoted
    /// by `zwlr_output_manager_v1::done`.
    ///
    /// Trying to create more than one zcosmic_output_head_v1 per zwlr_output_head_v1 will raise an
    /// "already_extended" error.
    ///
    /// # Arguments
    ///
    /// - `extended`:
    /// - `head`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_head(
        &mut self,
        slf: &Rc<ZcosmicOutputManagerV1>,
        extended: &Rc<ZcosmicOutputHeadV1>,
        head: &Rc<ZwlrOutputHeadV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_head(
            extended,
            head,
        );
        if let Err(e) = res {
            log_forward("zcosmic_output_manager_v1.get_head", &e);
        }
    }

    /// Get a zcosmic_output_configuration_v1 for an existing zwlr_output_configuration_v1
    ///
    /// Gets an extension object for zwlr_output_configuration_v1.
    ///
    /// Trying to create more than one zcosmic_output_configuration_v1 per zwlr_output_configuration_v1
    /// will raise an "already_extended" error.
    ///
    /// # Arguments
    ///
    /// - `extended`:
    /// - `config`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_configuration(
        &mut self,
        slf: &Rc<ZcosmicOutputManagerV1>,
        extended: &Rc<ZcosmicOutputConfigurationV1>,
        config: &Rc<ZwlrOutputConfigurationV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_configuration(
            extended,
            config,
        );
        if let Err(e) = res {
            log_forward("zcosmic_output_manager_v1.get_configuration", &e);
        }
    }

    /// Get a zcosmic_output_configuration_head_v1 for an existing zwlr_output_configuration_head_v1
    ///
    /// Gets an extension object for zwlr_output_configuration_head_v1.
    ///
    /// Trying to create more than one zcosmic_output_configuration_head_v1 per
    /// zwlr_output_configuration_head_v1 will raise an "already_extended" error.
    ///
    /// # Arguments
    ///
    /// - `extended`:
    /// - `config_head`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_configuration_head(
        &mut self,
        slf: &Rc<ZcosmicOutputManagerV1>,
        extended: &Rc<ZcosmicOutputConfigurationHeadV1>,
        config_head: &Rc<ZwlrOutputConfigurationHeadV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_configuration_head(
            extended,
            config_head,
        );
        if let Err(e) = res {
            log_forward("zcosmic_output_manager_v1.get_configuration_head", &e);
        }
    }

    /// Destroy this global
    ///
    /// Destroys this global. All previously created objects remain valid.
    #[inline]
    fn handle_release(
        &mut self,
        slf: &Rc<ZcosmicOutputManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_release(
        );
        if let Err(e) = res {
            log_forward("zcosmic_output_manager_v1.release", &e);
        }
    }

    /// set head as the primary for xwayland
    ///
    /// This requests a head to be advertised as the primary output via randr to Xwayland.
    ///
    /// No head has to be marked primary, if `null` is passed Xwayland won't advertise a primary output.
    /// Sending a disabled head will be ignored to avoid races.
    ///
    /// # Arguments
    ///
    /// - `head`: head to be advertised as primary
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_xwayland_primary(
        &mut self,
        slf: &Rc<ZcosmicOutputManagerV1>,
        head: Option<&Rc<ZcosmicOutputHeadV1>>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_xwayland_primary(
            head,
        );
        if let Err(e) = res {
            log_forward("zcosmic_output_manager_v1.set_xwayland_primary", &e);
        }
    }
}

impl ObjectPrivate for ZcosmicOutputManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZcosmicOutputManagerV1, version),
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_output_manager_v1#{}.get_head(extended: zcosmic_output_head_v1#{}, head: zwlr_output_head_v1#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = ZcosmicOutputHeadV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "extended", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<ZwlrOutputHeadV1>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("head", o.core().interface, ObjectInterface::ZwlrOutputHeadV1)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_head(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_head(&self, arg0, arg1);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_output_manager_v1#{}.get_configuration(extended: zcosmic_output_configuration_v1#{}, config: zwlr_output_configuration_v1#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = ZcosmicOutputConfigurationV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "extended", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<ZwlrOutputConfigurationV1>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("config", o.core().interface, ObjectInterface::ZwlrOutputConfigurationV1)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_configuration(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_configuration(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_output_manager_v1#{}.get_configuration_head(extended: zcosmic_output_configuration_head_v1#{}, config_head: zwlr_output_configuration_head_v1#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = ZcosmicOutputConfigurationHeadV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "extended", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<ZwlrOutputConfigurationHeadV1>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("config_head", o.core().interface, ObjectInterface::ZwlrOutputConfigurationHeadV1)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_configuration_head(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_configuration_head(&self, arg0, arg1);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_output_manager_v1#{}.release()\n", client_id, id);
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
            4 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_output_manager_v1#{}.set_xwayland_primary(head: zcosmic_output_head_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ZcosmicOutputHeadV1>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("head", o.core().interface, ObjectInterface::ZcosmicOutputHeadV1)));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_set_xwayland_primary(&self, arg0);
                } else {
                    DefaultHandler.handle_set_xwayland_primary(&self, arg0);
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
            0 => "get_head",
            1 => "get_configuration",
            2 => "get_configuration_head",
            3 => "release",
            4 => "set_xwayland_primary",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for ZcosmicOutputManagerV1 {
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

impl ZcosmicOutputManagerV1 {
    /// Since when the error.already_extended enum variant is available.
    pub const ENM__ERROR_ALREADY_EXTENDED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZcosmicOutputManagerV1Error(pub u32);

impl ZcosmicOutputManagerV1Error {
    /// object already created
    pub const ALREADY_EXTENDED: Self = Self(1);
}

impl Debug for ZcosmicOutputManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ALREADY_EXTENDED => "ALREADY_EXTENDED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
