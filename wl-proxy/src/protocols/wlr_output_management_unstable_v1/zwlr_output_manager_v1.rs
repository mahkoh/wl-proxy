//! output device configuration manager
//!
//! This interface is a manager that allows reading and writing the current
//! output device configuration.
//!
//! Output devices that display pixels (e.g. a physical monitor or a virtual
//! output in a window) are represented as heads. Heads cannot be created nor
//! destroyed by the client, but they can be enabled or disabled and their
//! properties can be changed. Each head may have one or more available modes.
//!
//! Whenever a head appears (e.g. a monitor is plugged in), it will be
//! advertised via the head event. Immediately after the output manager is
//! bound, all current heads are advertised.
//!
//! Whenever a head's properties change, the relevant wlr_output_head events
//! will be sent. Not all head properties will be sent: only properties that
//! have changed need to.
//!
//! Whenever a head disappears (e.g. a monitor is unplugged), a
//! wlr_output_head.finished event will be sent.
//!
//! After one or more heads appear, change or disappear, the done event will
//! be sent. It carries a serial which can be used in a create_configuration
//! request to update heads properties.
//!
//! The information obtained from this protocol should only be used for output
//! configuration purposes. This protocol is not designed to be a generic
//! output property advertisement protocol for regular clients. Instead,
//! protocols such as xdg-output should be used.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwlr_output_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwlrOutputManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwlrOutputManagerV1Handler>,
}

struct DefaultHandler;

impl ZwlrOutputManagerV1Handler for DefaultHandler { }

impl ConcreteObject for ZwlrOutputManagerV1 {
    const XML_VERSION: u32 = 4;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwlrOutputManagerV1;
    const INTERFACE_NAME: &str = "zwlr_output_manager_v1";
}

impl ZwlrOutputManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwlrOutputManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwlrOutputManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwlrOutputManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwlrOutputManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwlrOutputManagerV1 {
    /// Since when the head message is available.
    pub const MSG__HEAD__SINCE: u32 = 1;

    /// introduce a new head
    ///
    /// This event introduces a new head. This happens whenever a new head
    /// appears (e.g. a monitor is plugged in) or after the output manager is
    /// bound.
    ///
    /// # Arguments
    ///
    /// - `head`:
    #[inline]
    pub fn try_send_head(
        &self,
        head: &Rc<ZwlrOutputHeadV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            head,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateClientId("head", e)))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_manager_v1#{}.head(head: zwlr_output_head_v1#{})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// introduce a new head
    ///
    /// This event introduces a new head. This happens whenever a new head
    /// appears (e.g. a monitor is plugged in) or after the output manager is
    /// bound.
    ///
    /// # Arguments
    ///
    /// - `head`:
    #[inline]
    pub fn send_head(
        &self,
        head: &Rc<ZwlrOutputHeadV1>,
    ) {
        let res = self.try_send_head(
            head,
        );
        if let Err(e) = res {
            log_send("zwlr_output_manager_v1.head", &e);
        }
    }

    /// introduce a new head
    ///
    /// This event introduces a new head. This happens whenever a new head
    /// appears (e.g. a monitor is plugged in) or after the output manager is
    /// bound.
    #[inline]
    pub fn new_try_send_head(
        &self,
    ) -> Result<Rc<ZwlrOutputHeadV1>, ObjectError> {
        let head = self.core.create_child();
        self.try_send_head(
            &head,
        )?;
        Ok(head)
    }

    /// introduce a new head
    ///
    /// This event introduces a new head. This happens whenever a new head
    /// appears (e.g. a monitor is plugged in) or after the output manager is
    /// bound.
    #[inline]
    pub fn new_send_head(
        &self,
    ) -> Rc<ZwlrOutputHeadV1> {
        let head = self.core.create_child();
        self.send_head(
            &head,
        );
        head
    }

    /// Since when the done message is available.
    pub const MSG__DONE__SINCE: u32 = 1;

    /// sent all information about current configuration
    ///
    /// This event is sent after all information has been sent after binding to
    /// the output manager object and after any subsequent changes. This applies
    /// to child head and mode objects as well. In other words, this event is
    /// sent whenever a head or mode is created or destroyed and whenever one of
    /// their properties has been changed. Not all state is re-sent each time
    /// the current configuration changes: only the actual changes are sent.
    ///
    /// This allows changes to the output configuration to be seen as atomic,
    /// even if they happen via multiple events.
    ///
    /// A serial is sent to be used in a future create_configuration request.
    ///
    /// # Arguments
    ///
    /// - `serial`: current configuration serial
    #[inline]
    pub fn try_send_done(
        &self,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            serial,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_manager_v1#{}.done(serial: {})\n", client_id, id, arg0);
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
            1,
            arg0,
        ]);
        Ok(())
    }

    /// sent all information about current configuration
    ///
    /// This event is sent after all information has been sent after binding to
    /// the output manager object and after any subsequent changes. This applies
    /// to child head and mode objects as well. In other words, this event is
    /// sent whenever a head or mode is created or destroyed and whenever one of
    /// their properties has been changed. Not all state is re-sent each time
    /// the current configuration changes: only the actual changes are sent.
    ///
    /// This allows changes to the output configuration to be seen as atomic,
    /// even if they happen via multiple events.
    ///
    /// A serial is sent to be used in a future create_configuration request.
    ///
    /// # Arguments
    ///
    /// - `serial`: current configuration serial
    #[inline]
    pub fn send_done(
        &self,
        serial: u32,
    ) {
        let res = self.try_send_done(
            serial,
        );
        if let Err(e) = res {
            log_send("zwlr_output_manager_v1.done", &e);
        }
    }

    /// Since when the create_configuration message is available.
    pub const MSG__CREATE_CONFIGURATION__SINCE: u32 = 1;

    /// create a new output configuration object
    ///
    /// Create a new output configuration object. This allows to update head
    /// properties.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `serial`:
    #[inline]
    pub fn try_send_create_configuration(
        &self,
        id: &Rc<ZwlrOutputConfigurationV1>,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            serial,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_manager_v1#{}.create_configuration(id: zwlr_output_configuration_v1#{}, serial: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            arg1,
        ]);
        Ok(())
    }

    /// create a new output configuration object
    ///
    /// Create a new output configuration object. This allows to update head
    /// properties.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `serial`:
    #[inline]
    pub fn send_create_configuration(
        &self,
        id: &Rc<ZwlrOutputConfigurationV1>,
        serial: u32,
    ) {
        let res = self.try_send_create_configuration(
            id,
            serial,
        );
        if let Err(e) = res {
            log_send("zwlr_output_manager_v1.create_configuration", &e);
        }
    }

    /// create a new output configuration object
    ///
    /// Create a new output configuration object. This allows to update head
    /// properties.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    #[inline]
    pub fn new_try_send_create_configuration(
        &self,
        serial: u32,
    ) -> Result<Rc<ZwlrOutputConfigurationV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_create_configuration(
            &id,
            serial,
        )?;
        Ok(id)
    }

    /// create a new output configuration object
    ///
    /// Create a new output configuration object. This allows to update head
    /// properties.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    #[inline]
    pub fn new_send_create_configuration(
        &self,
        serial: u32,
    ) -> Rc<ZwlrOutputConfigurationV1> {
        let id = self.core.create_child();
        self.send_create_configuration(
            &id,
            serial,
        );
        id
    }

    /// Since when the stop message is available.
    pub const MSG__STOP__SINCE: u32 = 1;

    /// stop sending events
    ///
    /// Indicates the client no longer wishes to receive events for output
    /// configuration changes. However the compositor may emit further events,
    /// until the finished event is emitted.
    ///
    /// The client must not send any more requests after this one.
    #[inline]
    pub fn try_send_stop(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_manager_v1#{}.stop()\n", id);
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
        Ok(())
    }

    /// stop sending events
    ///
    /// Indicates the client no longer wishes to receive events for output
    /// configuration changes. However the compositor may emit further events,
    /// until the finished event is emitted.
    ///
    /// The client must not send any more requests after this one.
    #[inline]
    pub fn send_stop(
        &self,
    ) {
        let res = self.try_send_stop(
        );
        if let Err(e) = res {
            log_send("zwlr_output_manager_v1.stop", &e);
        }
    }

    /// Since when the finished message is available.
    pub const MSG__FINISHED__SINCE: u32 = 1;

    /// the compositor has finished with the manager
    ///
    /// This event indicates that the compositor is done sending manager events.
    /// The compositor will destroy the object immediately after sending this
    /// event, so it will become invalid and the client should release any
    /// resources associated with it.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_manager_v1#{}.finished()\n", client_id, id);
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
            2,
        ]);
        drop(fmt);
        drop(outgoing_ref);
        drop(client_ref);
        self.core.handle_client_destroy();
        Ok(())
    }

    /// the compositor has finished with the manager
    ///
    /// This event indicates that the compositor is done sending manager events.
    /// The compositor will destroy the object immediately after sending this
    /// event, so it will become invalid and the client should release any
    /// resources associated with it.
    #[inline]
    pub fn send_finished(
        &self,
    ) {
        let res = self.try_send_finished(
        );
        if let Err(e) = res {
            log_send("zwlr_output_manager_v1.finished", &e);
        }
    }
}

/// A message handler for [`ZwlrOutputManagerV1`] proxies.
pub trait ZwlrOutputManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwlrOutputManagerV1>) {
        slf.core.delete_id();
    }

    /// introduce a new head
    ///
    /// This event introduces a new head. This happens whenever a new head
    /// appears (e.g. a monitor is plugged in) or after the output manager is
    /// bound.
    ///
    /// # Arguments
    ///
    /// - `head`:
    #[inline]
    fn handle_head(
        &mut self,
        slf: &Rc<ZwlrOutputManagerV1>,
        head: &Rc<ZwlrOutputHeadV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_head(
            head,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_manager_v1.head", &e);
        }
    }

    /// sent all information about current configuration
    ///
    /// This event is sent after all information has been sent after binding to
    /// the output manager object and after any subsequent changes. This applies
    /// to child head and mode objects as well. In other words, this event is
    /// sent whenever a head or mode is created or destroyed and whenever one of
    /// their properties has been changed. Not all state is re-sent each time
    /// the current configuration changes: only the actual changes are sent.
    ///
    /// This allows changes to the output configuration to be seen as atomic,
    /// even if they happen via multiple events.
    ///
    /// A serial is sent to be used in a future create_configuration request.
    ///
    /// # Arguments
    ///
    /// - `serial`: current configuration serial
    #[inline]
    fn handle_done(
        &mut self,
        slf: &Rc<ZwlrOutputManagerV1>,
        serial: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_done(
            serial,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_manager_v1.done", &e);
        }
    }

    /// create a new output configuration object
    ///
    /// Create a new output configuration object. This allows to update head
    /// properties.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `serial`:
    #[inline]
    fn handle_create_configuration(
        &mut self,
        slf: &Rc<ZwlrOutputManagerV1>,
        id: &Rc<ZwlrOutputConfigurationV1>,
        serial: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_configuration(
            id,
            serial,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_manager_v1.create_configuration", &e);
        }
    }

    /// stop sending events
    ///
    /// Indicates the client no longer wishes to receive events for output
    /// configuration changes. However the compositor may emit further events,
    /// until the finished event is emitted.
    ///
    /// The client must not send any more requests after this one.
    #[inline]
    fn handle_stop(
        &mut self,
        slf: &Rc<ZwlrOutputManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_stop(
        );
        if let Err(e) = res {
            log_forward("zwlr_output_manager_v1.stop", &e);
        }
    }

    /// the compositor has finished with the manager
    ///
    /// This event indicates that the compositor is done sending manager events.
    /// The compositor will destroy the object immediately after sending this
    /// event, so it will become invalid and the client should release any
    /// resources associated with it.
    #[inline]
    fn handle_finished(
        &mut self,
        slf: &Rc<ZwlrOutputManagerV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_finished(
        );
        if let Err(e) = res {
            log_forward("zwlr_output_manager_v1.finished", &e);
        }
    }
}

impl ObjectPrivate for ZwlrOutputManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwlrOutputManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_manager_v1#{}.create_configuration(id: zwlr_output_configuration_v1#{}, serial: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = ZwlrOutputConfigurationV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_configuration(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_create_configuration(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_manager_v1#{}.stop()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_stop(&self);
                } else {
                    DefaultHandler.handle_stop(&self);
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
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_manager_v1#{}.head(head: zwlr_output_head_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = ZwlrOutputHeadV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetServerId(arg0_id, "head", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_head(&self, arg0);
                } else {
                    DefaultHandler.handle_head(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_manager_v1#{}.done(serial: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_done(&self, arg0);
                } else {
                    DefaultHandler.handle_done(&self, arg0);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_manager_v1#{}.finished()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                self.core.handle_server_destroy();
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
            0 => "create_configuration",
            1 => "stop",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "head",
            1 => "done",
            2 => "finished",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwlrOutputManagerV1 {
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

