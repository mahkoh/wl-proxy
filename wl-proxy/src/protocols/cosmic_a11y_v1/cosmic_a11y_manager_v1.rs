//! a11y global
//!
//! Manager to toggle accessibility features.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A cosmic_a11y_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct CosmicA11yManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn CosmicA11yManagerV1Handler>,
}

struct DefaultHandler;

impl CosmicA11yManagerV1Handler for DefaultHandler { }

impl ConcreteObject for CosmicA11yManagerV1 {
    const XML_VERSION: u32 = 3;
    const INTERFACE: ObjectInterface = ObjectInterface::CosmicA11yManagerV1;
    const INTERFACE_NAME: &str = "cosmic_a11y_manager_v1";
}

impl CosmicA11yManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl CosmicA11yManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn CosmicA11yManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for CosmicA11yManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CosmicA11yManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl CosmicA11yManagerV1 {
    /// Since when the magnifier message is available.
    pub const MSG__MAGNIFIER__SINCE: u32 = 1;

    /// State of the screen magnifier
    ///
    /// State of the screen magnifier.
    ///
    /// This event will be emitted by the compositor when binding the protocol
    /// and whenever the state changes.
    ///
    /// # Arguments
    ///
    /// - `active`: If the screen magnifier is enabled
    #[inline]
    pub fn try_send_magnifier(
        &self,
        active: CosmicA11yManagerV1ActiveState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            active,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: CosmicA11yManagerV1ActiveState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= cosmic_a11y_manager_v1#{}.magnifier(active: {:?})\n", client_id, id, arg0);
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

    /// State of the screen magnifier
    ///
    /// State of the screen magnifier.
    ///
    /// This event will be emitted by the compositor when binding the protocol
    /// and whenever the state changes.
    ///
    /// # Arguments
    ///
    /// - `active`: If the screen magnifier is enabled
    #[inline]
    pub fn send_magnifier(
        &self,
        active: CosmicA11yManagerV1ActiveState,
    ) {
        let res = self.try_send_magnifier(
            active,
        );
        if let Err(e) = res {
            log_send("cosmic_a11y_manager_v1.magnifier", &e);
        }
    }

    /// Since when the set_magnifier message is available.
    pub const MSG__SET_MAGNIFIER__SINCE: u32 = 1;

    /// Set the screen magnifier on or off
    ///
    /// Sets the state of the screen magnifier.
    ///
    /// The client must not assume any requested changes are actually applied and should wait
    /// until the next magnifier event before updating it's UI.
    ///
    /// # Arguments
    ///
    /// - `active`: If the screen magnifier should be enabled
    #[inline]
    pub fn try_send_set_magnifier(
        &self,
        active: CosmicA11yManagerV1ActiveState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            active,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: CosmicA11yManagerV1ActiveState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= cosmic_a11y_manager_v1#{}.set_magnifier(active: {:?})\n", id, arg0);
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

    /// Set the screen magnifier on or off
    ///
    /// Sets the state of the screen magnifier.
    ///
    /// The client must not assume any requested changes are actually applied and should wait
    /// until the next magnifier event before updating it's UI.
    ///
    /// # Arguments
    ///
    /// - `active`: If the screen magnifier should be enabled
    #[inline]
    pub fn send_set_magnifier(
        &self,
        active: CosmicA11yManagerV1ActiveState,
    ) {
        let res = self.try_send_set_magnifier(
            active,
        );
        if let Err(e) = res {
            log_send("cosmic_a11y_manager_v1.set_magnifier", &e);
        }
    }

    /// Since when the screen_filter message is available.
    pub const MSG__SCREEN_FILTER__SINCE: u32 = 2;

    /// Since when the screen_filter message is deprecated.
    pub const MSG__SCREEN_FILTER__DEPRECATED_SINCE: u32 = 3;

    /// State of screen filtering
    ///
    /// Parameters used for screen filtering.
    ///
    /// This event will be emitted by the compositor when binding the protocol
    /// and whenever the state changes.
    ///
    /// If a screen filter is used not known to the protocol or the bound version
    /// filter will be set to unknown.
    ///
    /// Since version 3 this event will not be emitted anymore, instead use `screen_filter2`.
    ///
    /// # Arguments
    ///
    /// - `inverted`: If the screen colors are inverted
    /// - `filter`: Which if any screen filter is enabled
    #[inline]
    pub fn try_send_screen_filter(
        &self,
        inverted: CosmicA11yManagerV1ActiveState,
        filter: CosmicA11yManagerV1Filter,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            inverted,
            filter,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: CosmicA11yManagerV1ActiveState, arg1: CosmicA11yManagerV1Filter) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= cosmic_a11y_manager_v1#{}.screen_filter(inverted: {:?}, filter: {:?})\n", client_id, id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1);
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
            arg0.0,
            arg1.0,
        ]);
        Ok(())
    }

    /// State of screen filtering
    ///
    /// Parameters used for screen filtering.
    ///
    /// This event will be emitted by the compositor when binding the protocol
    /// and whenever the state changes.
    ///
    /// If a screen filter is used not known to the protocol or the bound version
    /// filter will be set to unknown.
    ///
    /// Since version 3 this event will not be emitted anymore, instead use `screen_filter2`.
    ///
    /// # Arguments
    ///
    /// - `inverted`: If the screen colors are inverted
    /// - `filter`: Which if any screen filter is enabled
    #[inline]
    pub fn send_screen_filter(
        &self,
        inverted: CosmicA11yManagerV1ActiveState,
        filter: CosmicA11yManagerV1Filter,
    ) {
        let res = self.try_send_screen_filter(
            inverted,
            filter,
        );
        if let Err(e) = res {
            log_send("cosmic_a11y_manager_v1.screen_filter", &e);
        }
    }

    /// Since when the set_screen_filter message is available.
    pub const MSG__SET_SCREEN_FILTER__SINCE: u32 = 2;

    /// Since when the set_screen_filter message is deprecated.
    pub const MSG__SET_SCREEN_FILTER__DEPRECATED_SINCE: u32 = 3;

    /// Set screen filtering
    ///
    /// Set the parameters for screen filtering.
    ///
    /// If the filter is set to unknown, the compositor MUST not change the current state
    /// of the filter. This is to allow clients to update the inverted state, even if they
    /// don't know about the current active filter.
    ///
    /// The client must not assume any requested changes are actually applied and should wait
    /// until the next screen_filter event before updating it's UI.
    ///
    /// Send this request will raised a "deprecated" protocol error, if version 3 or higher was bound.
    /// Use `set_screen_filter2` instead.
    ///
    /// # Arguments
    ///
    /// - `inverted`: If the screen colors should be inverted
    /// - `filter`: Which if any filter should be used
    #[inline]
    pub fn try_send_set_screen_filter(
        &self,
        inverted: CosmicA11yManagerV1ActiveState,
        filter: CosmicA11yManagerV1Filter,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            inverted,
            filter,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: CosmicA11yManagerV1ActiveState, arg1: CosmicA11yManagerV1Filter) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= cosmic_a11y_manager_v1#{}.set_screen_filter(inverted: {:?}, filter: {:?})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1);
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
            arg0.0,
            arg1.0,
        ]);
        Ok(())
    }

    /// Set screen filtering
    ///
    /// Set the parameters for screen filtering.
    ///
    /// If the filter is set to unknown, the compositor MUST not change the current state
    /// of the filter. This is to allow clients to update the inverted state, even if they
    /// don't know about the current active filter.
    ///
    /// The client must not assume any requested changes are actually applied and should wait
    /// until the next screen_filter event before updating it's UI.
    ///
    /// Send this request will raised a "deprecated" protocol error, if version 3 or higher was bound.
    /// Use `set_screen_filter2` instead.
    ///
    /// # Arguments
    ///
    /// - `inverted`: If the screen colors should be inverted
    /// - `filter`: Which if any filter should be used
    #[inline]
    pub fn send_set_screen_filter(
        &self,
        inverted: CosmicA11yManagerV1ActiveState,
        filter: CosmicA11yManagerV1Filter,
    ) {
        let res = self.try_send_set_screen_filter(
            inverted,
            filter,
        );
        if let Err(e) = res {
            log_send("cosmic_a11y_manager_v1.set_screen_filter", &e);
        }
    }

    /// Since when the screen_filter2 message is available.
    pub const MSG__SCREEN_FILTER2__SINCE: u32 = 3;

    /// State of screen filtering
    ///
    /// Parameters used for screen filtering.
    ///
    /// This event will be emitted by the compositor when binding the protocol
    /// and whenever the state changes.
    ///
    /// If a screen filter is used not known to the protocol or the bound version
    /// filter will be set to unknown.
    ///
    /// The compositor must never send "disabled" as the "filter" argument.
    ///
    /// # Arguments
    ///
    /// - `inverted`: If the screen colors are inverted
    /// - `filter`: Which if any screen filter is selected
    /// - `filter_state`: If the screen filter is active
    #[inline]
    pub fn try_send_screen_filter2(
        &self,
        inverted: CosmicA11yManagerV1ActiveState,
        filter: CosmicA11yManagerV1Filter,
        filter_state: CosmicA11yManagerV1ActiveState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            inverted,
            filter,
            filter_state,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: CosmicA11yManagerV1ActiveState, arg1: CosmicA11yManagerV1Filter, arg2: CosmicA11yManagerV1ActiveState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= cosmic_a11y_manager_v1#{}.screen_filter2(inverted: {:?}, filter: {:?}, filter_state: {:?})\n", client_id, id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1, arg2);
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
            arg0.0,
            arg1.0,
            arg2.0,
        ]);
        Ok(())
    }

    /// State of screen filtering
    ///
    /// Parameters used for screen filtering.
    ///
    /// This event will be emitted by the compositor when binding the protocol
    /// and whenever the state changes.
    ///
    /// If a screen filter is used not known to the protocol or the bound version
    /// filter will be set to unknown.
    ///
    /// The compositor must never send "disabled" as the "filter" argument.
    ///
    /// # Arguments
    ///
    /// - `inverted`: If the screen colors are inverted
    /// - `filter`: Which if any screen filter is selected
    /// - `filter_state`: If the screen filter is active
    #[inline]
    pub fn send_screen_filter2(
        &self,
        inverted: CosmicA11yManagerV1ActiveState,
        filter: CosmicA11yManagerV1Filter,
        filter_state: CosmicA11yManagerV1ActiveState,
    ) {
        let res = self.try_send_screen_filter2(
            inverted,
            filter,
            filter_state,
        );
        if let Err(e) = res {
            log_send("cosmic_a11y_manager_v1.screen_filter2", &e);
        }
    }

    /// Since when the set_screen_filter2 message is available.
    pub const MSG__SET_SCREEN_FILTER2__SINCE: u32 = 3;

    /// Set screen filtering
    ///
    /// Set the parameters for screen filtering.
    ///
    /// If the filter is set to unknown, the compositor MUST not change the currently set
    /// filter. This is to allow clients to update the inverted state or toggle the screen filter,
    /// even if they don't know about the currently selected filter.
    ///
    /// The client must not assume any requested changes are actually applied and should wait
    /// until the next screen_filter event before updating it's UI.
    ///
    /// The "deprecated" protocol error is raised, if "disabled" is set for "filter".
    ///
    /// # Arguments
    ///
    /// - `inverted`: If the screen colors should be inverted
    /// - `filter`: Which if filter should be used
    /// - `filter_state`: If the screen filter should be active
    #[inline]
    pub fn try_send_set_screen_filter2(
        &self,
        inverted: CosmicA11yManagerV1ActiveState,
        filter: CosmicA11yManagerV1Filter,
        filter_state: CosmicA11yManagerV1ActiveState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            inverted,
            filter,
            filter_state,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: CosmicA11yManagerV1ActiveState, arg1: CosmicA11yManagerV1Filter, arg2: CosmicA11yManagerV1ActiveState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= cosmic_a11y_manager_v1#{}.set_screen_filter2(inverted: {:?}, filter: {:?}, filter_state: {:?})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2);
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
            arg1.0,
            arg2.0,
        ]);
        Ok(())
    }

    /// Set screen filtering
    ///
    /// Set the parameters for screen filtering.
    ///
    /// If the filter is set to unknown, the compositor MUST not change the currently set
    /// filter. This is to allow clients to update the inverted state or toggle the screen filter,
    /// even if they don't know about the currently selected filter.
    ///
    /// The client must not assume any requested changes are actually applied and should wait
    /// until the next screen_filter event before updating it's UI.
    ///
    /// The "deprecated" protocol error is raised, if "disabled" is set for "filter".
    ///
    /// # Arguments
    ///
    /// - `inverted`: If the screen colors should be inverted
    /// - `filter`: Which if filter should be used
    /// - `filter_state`: If the screen filter should be active
    #[inline]
    pub fn send_set_screen_filter2(
        &self,
        inverted: CosmicA11yManagerV1ActiveState,
        filter: CosmicA11yManagerV1Filter,
        filter_state: CosmicA11yManagerV1ActiveState,
    ) {
        let res = self.try_send_set_screen_filter2(
            inverted,
            filter,
            filter_state,
        );
        if let Err(e) = res {
            log_send("cosmic_a11y_manager_v1.set_screen_filter2", &e);
        }
    }
}

/// A message handler for [`CosmicA11yManagerV1`] proxies.
pub trait CosmicA11yManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<CosmicA11yManagerV1>) {
        slf.core.delete_id();
    }

    /// State of the screen magnifier
    ///
    /// State of the screen magnifier.
    ///
    /// This event will be emitted by the compositor when binding the protocol
    /// and whenever the state changes.
    ///
    /// # Arguments
    ///
    /// - `active`: If the screen magnifier is enabled
    #[inline]
    fn handle_magnifier(
        &mut self,
        slf: &Rc<CosmicA11yManagerV1>,
        active: CosmicA11yManagerV1ActiveState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_magnifier(
            active,
        );
        if let Err(e) = res {
            log_forward("cosmic_a11y_manager_v1.magnifier", &e);
        }
    }

    /// Set the screen magnifier on or off
    ///
    /// Sets the state of the screen magnifier.
    ///
    /// The client must not assume any requested changes are actually applied and should wait
    /// until the next magnifier event before updating it's UI.
    ///
    /// # Arguments
    ///
    /// - `active`: If the screen magnifier should be enabled
    #[inline]
    fn handle_set_magnifier(
        &mut self,
        slf: &Rc<CosmicA11yManagerV1>,
        active: CosmicA11yManagerV1ActiveState,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_magnifier(
            active,
        );
        if let Err(e) = res {
            log_forward("cosmic_a11y_manager_v1.set_magnifier", &e);
        }
    }

    /// State of screen filtering
    ///
    /// Parameters used for screen filtering.
    ///
    /// This event will be emitted by the compositor when binding the protocol
    /// and whenever the state changes.
    ///
    /// If a screen filter is used not known to the protocol or the bound version
    /// filter will be set to unknown.
    ///
    /// Since version 3 this event will not be emitted anymore, instead use `screen_filter2`.
    ///
    /// # Arguments
    ///
    /// - `inverted`: If the screen colors are inverted
    /// - `filter`: Which if any screen filter is enabled
    #[inline]
    fn handle_screen_filter(
        &mut self,
        slf: &Rc<CosmicA11yManagerV1>,
        inverted: CosmicA11yManagerV1ActiveState,
        filter: CosmicA11yManagerV1Filter,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_screen_filter(
            inverted,
            filter,
        );
        if let Err(e) = res {
            log_forward("cosmic_a11y_manager_v1.screen_filter", &e);
        }
    }

    /// Set screen filtering
    ///
    /// Set the parameters for screen filtering.
    ///
    /// If the filter is set to unknown, the compositor MUST not change the current state
    /// of the filter. This is to allow clients to update the inverted state, even if they
    /// don't know about the current active filter.
    ///
    /// The client must not assume any requested changes are actually applied and should wait
    /// until the next screen_filter event before updating it's UI.
    ///
    /// Send this request will raised a "deprecated" protocol error, if version 3 or higher was bound.
    /// Use `set_screen_filter2` instead.
    ///
    /// # Arguments
    ///
    /// - `inverted`: If the screen colors should be inverted
    /// - `filter`: Which if any filter should be used
    #[inline]
    fn handle_set_screen_filter(
        &mut self,
        slf: &Rc<CosmicA11yManagerV1>,
        inverted: CosmicA11yManagerV1ActiveState,
        filter: CosmicA11yManagerV1Filter,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_screen_filter(
            inverted,
            filter,
        );
        if let Err(e) = res {
            log_forward("cosmic_a11y_manager_v1.set_screen_filter", &e);
        }
    }

    /// State of screen filtering
    ///
    /// Parameters used for screen filtering.
    ///
    /// This event will be emitted by the compositor when binding the protocol
    /// and whenever the state changes.
    ///
    /// If a screen filter is used not known to the protocol or the bound version
    /// filter will be set to unknown.
    ///
    /// The compositor must never send "disabled" as the "filter" argument.
    ///
    /// # Arguments
    ///
    /// - `inverted`: If the screen colors are inverted
    /// - `filter`: Which if any screen filter is selected
    /// - `filter_state`: If the screen filter is active
    #[inline]
    fn handle_screen_filter2(
        &mut self,
        slf: &Rc<CosmicA11yManagerV1>,
        inverted: CosmicA11yManagerV1ActiveState,
        filter: CosmicA11yManagerV1Filter,
        filter_state: CosmicA11yManagerV1ActiveState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_screen_filter2(
            inverted,
            filter,
            filter_state,
        );
        if let Err(e) = res {
            log_forward("cosmic_a11y_manager_v1.screen_filter2", &e);
        }
    }

    /// Set screen filtering
    ///
    /// Set the parameters for screen filtering.
    ///
    /// If the filter is set to unknown, the compositor MUST not change the currently set
    /// filter. This is to allow clients to update the inverted state or toggle the screen filter,
    /// even if they don't know about the currently selected filter.
    ///
    /// The client must not assume any requested changes are actually applied and should wait
    /// until the next screen_filter event before updating it's UI.
    ///
    /// The "deprecated" protocol error is raised, if "disabled" is set for "filter".
    ///
    /// # Arguments
    ///
    /// - `inverted`: If the screen colors should be inverted
    /// - `filter`: Which if filter should be used
    /// - `filter_state`: If the screen filter should be active
    #[inline]
    fn handle_set_screen_filter2(
        &mut self,
        slf: &Rc<CosmicA11yManagerV1>,
        inverted: CosmicA11yManagerV1ActiveState,
        filter: CosmicA11yManagerV1Filter,
        filter_state: CosmicA11yManagerV1ActiveState,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_screen_filter2(
            inverted,
            filter,
            filter_state,
        );
        if let Err(e) = res {
            log_forward("cosmic_a11y_manager_v1.set_screen_filter2", &e);
        }
    }
}

impl ObjectPrivate for CosmicA11yManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::CosmicA11yManagerV1, version),
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
                let arg0 = CosmicA11yManagerV1ActiveState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: CosmicA11yManagerV1ActiveState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> cosmic_a11y_manager_v1#{}.set_magnifier(active: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_magnifier(&self, arg0);
                } else {
                    DefaultHandler.handle_set_magnifier(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = CosmicA11yManagerV1ActiveState(arg0);
                let arg1 = CosmicA11yManagerV1Filter(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: CosmicA11yManagerV1ActiveState, arg1: CosmicA11yManagerV1Filter) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> cosmic_a11y_manager_v1#{}.set_screen_filter(inverted: {:?}, filter: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_screen_filter(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_screen_filter(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                let arg0 = CosmicA11yManagerV1ActiveState(arg0);
                let arg1 = CosmicA11yManagerV1Filter(arg1);
                let arg2 = CosmicA11yManagerV1ActiveState(arg2);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: CosmicA11yManagerV1ActiveState, arg1: CosmicA11yManagerV1Filter, arg2: CosmicA11yManagerV1ActiveState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> cosmic_a11y_manager_v1#{}.set_screen_filter2(inverted: {:?}, filter: {:?}, filter_state: {:?})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_screen_filter2(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_set_screen_filter2(&self, arg0, arg1, arg2);
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
                let arg0 = CosmicA11yManagerV1ActiveState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: CosmicA11yManagerV1ActiveState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> cosmic_a11y_manager_v1#{}.magnifier(active: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_magnifier(&self, arg0);
                } else {
                    DefaultHandler.handle_magnifier(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = CosmicA11yManagerV1ActiveState(arg0);
                let arg1 = CosmicA11yManagerV1Filter(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: CosmicA11yManagerV1ActiveState, arg1: CosmicA11yManagerV1Filter) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> cosmic_a11y_manager_v1#{}.screen_filter(inverted: {:?}, filter: {:?})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_screen_filter(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_screen_filter(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                let arg0 = CosmicA11yManagerV1ActiveState(arg0);
                let arg1 = CosmicA11yManagerV1Filter(arg1);
                let arg2 = CosmicA11yManagerV1ActiveState(arg2);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: CosmicA11yManagerV1ActiveState, arg1: CosmicA11yManagerV1Filter, arg2: CosmicA11yManagerV1ActiveState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> cosmic_a11y_manager_v1#{}.screen_filter2(inverted: {:?}, filter: {:?}, filter_state: {:?})\n", id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_screen_filter2(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_screen_filter2(&self, arg0, arg1, arg2);
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
            0 => "set_magnifier",
            1 => "set_screen_filter",
            2 => "set_screen_filter2",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "magnifier",
            1 => "screen_filter",
            2 => "screen_filter2",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for CosmicA11yManagerV1 {
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

impl CosmicA11yManagerV1 {
    /// Since when the active_state.disabled enum variant is available.
    pub const ENM__ACTIVE_STATE_DISABLED__SINCE: u32 = 1;
    /// Since when the active_state.enabled enum variant is available.
    pub const ENM__ACTIVE_STATE_ENABLED__SINCE: u32 = 1;

    /// Since when the filter.disabled enum variant is available.
    pub const ENM__FILTER_DISABLED__SINCE: u32 = 1;

    /// Since when the filter.disabled enum variant is deprecated.
    pub const ENM__FILTER_DISABLED__DEPRECATED_SINCE: u32 = 3;
    /// Since when the filter.unknown enum variant is available.
    pub const ENM__FILTER_UNKNOWN__SINCE: u32 = 1;
    /// Since when the filter.greyscale enum variant is available.
    pub const ENM__FILTER_GREYSCALE__SINCE: u32 = 1;
    /// Since when the filter.daltonize_protanopia enum variant is available.
    pub const ENM__FILTER_DALTONIZE_PROTANOPIA__SINCE: u32 = 1;
    /// Since when the filter.daltonize_deuteranopia enum variant is available.
    pub const ENM__FILTER_DALTONIZE_DEUTERANOPIA__SINCE: u32 = 1;
    /// Since when the filter.daltonize_tritanopia enum variant is available.
    pub const ENM__FILTER_DALTONIZE_TRITANOPIA__SINCE: u32 = 1;

    /// Since when the error.deprecated enum variant is available.
    pub const ENM__ERROR_DEPRECATED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CosmicA11yManagerV1ActiveState(pub u32);

impl CosmicA11yManagerV1ActiveState {
    /// function is disabled
    pub const DISABLED: Self = Self(0);

    /// function is enabled
    pub const ENABLED: Self = Self(1);
}

impl Debug for CosmicA11yManagerV1ActiveState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DISABLED => "DISABLED",
            Self::ENABLED => "ENABLED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CosmicA11yManagerV1Filter(pub u32);

impl CosmicA11yManagerV1Filter {
    /// No screen filter is set
    pub const DISABLED: Self = Self(0);

    /// A custom or unknown screen filter
    pub const UNKNOWN: Self = Self(1);

    /// Greyscale colors
    pub const GREYSCALE: Self = Self(2);

    /// Daltonize for Protanopia
    pub const DALTONIZE_PROTANOPIA: Self = Self(3);

    /// Daltonize for Deuteranopia
    pub const DALTONIZE_DEUTERANOPIA: Self = Self(4);

    /// Daltonize for Tritanopia
    pub const DALTONIZE_TRITANOPIA: Self = Self(5);
}

impl Debug for CosmicA11yManagerV1Filter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DISABLED => "DISABLED",
            Self::UNKNOWN => "UNKNOWN",
            Self::GREYSCALE => "GREYSCALE",
            Self::DALTONIZE_PROTANOPIA => "DALTONIZE_PROTANOPIA",
            Self::DALTONIZE_DEUTERANOPIA => "DALTONIZE_DEUTERANOPIA",
            Self::DALTONIZE_TRITANOPIA => "DALTONIZE_TRITANOPIA",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CosmicA11yManagerV1Error(pub u32);

impl CosmicA11yManagerV1Error {
    /// A deprecated request or value was used
    pub const DEPRECATED: Self = Self(0);
}

impl Debug for CosmicA11yManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DEPRECATED => "DEPRECATED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
