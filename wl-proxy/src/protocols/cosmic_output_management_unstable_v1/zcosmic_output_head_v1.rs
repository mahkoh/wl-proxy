//! Output extension object
//!
//! Extension to zwlr_output_head_v1.
//!
//! Adds additional read-only properties.
//!
//! Properties sent via this interface are applied atomically via the wlr_output_manager.done event.
//! No guarantees are made regarding the order in which properties are sent.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zcosmic_output_head_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZcosmicOutputHeadV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZcosmicOutputHeadV1Handler>,
}

struct DefaultHandler;

impl ZcosmicOutputHeadV1Handler for DefaultHandler { }

impl ConcreteObject for ZcosmicOutputHeadV1 {
    const XML_VERSION: u32 = 3;
    const INTERFACE: ObjectInterface = ObjectInterface::ZcosmicOutputHeadV1;
    const INTERFACE_NAME: &str = "zcosmic_output_head_v1";
}

impl ZcosmicOutputHeadV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZcosmicOutputHeadV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZcosmicOutputHeadV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZcosmicOutputHeadV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZcosmicOutputHeadV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZcosmicOutputHeadV1 {
    /// Since when the scale_1000 message is available.
    pub const MSG__SCALE_1000__SINCE: u32 = 1;

    /// current scale
    ///
    /// This events describes the scale of the head in the global compositor
    /// space multiplied by 1000 for additional precision.
    ///
    /// It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `scale_1000`:
    #[inline]
    pub fn try_send_scale_1000(
        &self,
        scale_1000: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            scale_1000,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zcosmic_output_head_v1#{}.scale_1000(scale_1000: {})\n", client_id, id, arg0);
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
            arg0 as u32,
        ]);
        Ok(())
    }

    /// current scale
    ///
    /// This events describes the scale of the head in the global compositor
    /// space multiplied by 1000 for additional precision.
    ///
    /// It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `scale_1000`:
    #[inline]
    pub fn send_scale_1000(
        &self,
        scale_1000: i32,
    ) {
        let res = self.try_send_scale_1000(
            scale_1000,
        );
        if let Err(e) = res {
            log_send("zcosmic_output_head_v1.scale_1000", &e);
        }
    }

    /// Since when the mirroring message is available.
    pub const MSG__MIRRORING__SINCE: u32 = 1;

    /// mirroring other output
    ///
    /// This events describes that the head is mirroring another.
    /// In these cases `name` contains the unique name of the matching `zwlr_output_head_v1`.
    /// If the name is null, no head is being mirrored onto this one.
    ///
    /// For mirrored heads the `position`-event is meaningless.
    ///
    /// It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    pub fn try_send_mirroring(
        &self,
        name: Option<&str>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            name,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: Option<&str>) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zcosmic_output_head_v1#{}.mirroring(name: {:?})\n", client_id, id, arg0);
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
        ]);
        if let Some(arg0) = arg0 {
            fmt.string(arg0);
        } else {
            fmt.words([0]);
        }
        Ok(())
    }

    /// mirroring other output
    ///
    /// This events describes that the head is mirroring another.
    /// In these cases `name` contains the unique name of the matching `zwlr_output_head_v1`.
    /// If the name is null, no head is being mirrored onto this one.
    ///
    /// For mirrored heads the `position`-event is meaningless.
    ///
    /// It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    pub fn send_mirroring(
        &self,
        name: Option<&str>,
    ) {
        let res = self.try_send_mirroring(
            name,
        );
        if let Err(e) = res {
            log_send("zcosmic_output_head_v1.mirroring", &e);
        }
    }

    /// Since when the release message is available.
    pub const MSG__RELEASE__SINCE: u32 = 1;

    /// destroy the output head
    ///
    /// Using this request a client can tell the compositor that it is not interested
    /// in the head object anymore.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_output_head_v1#{}.release()\n", id);
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
            0,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the output head
    ///
    /// Using this request a client can tell the compositor that it is not interested
    /// in the head object anymore.
    #[inline]
    pub fn send_release(
        &self,
    ) {
        let res = self.try_send_release(
        );
        if let Err(e) = res {
            log_send("zcosmic_output_head_v1.release", &e);
        }
    }

    /// Since when the adaptive_sync_available message is available.
    pub const MSG__ADAPTIVE_SYNC_AVAILABLE__SINCE: u32 = 2;

    /// is adaptive_sync available for this head
    ///
    /// This events describes if adaptive_sync is available for this head.
    ///
    /// It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `available`:
    #[inline]
    pub fn try_send_adaptive_sync_available(
        &self,
        available: ZcosmicOutputHeadV1AdaptiveSyncAvailability,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            available,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: ZcosmicOutputHeadV1AdaptiveSyncAvailability) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zcosmic_output_head_v1#{}.adaptive_sync_available(available: {:?})\n", client_id, id, arg0);
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
            2,
            arg0.0,
        ]);
        Ok(())
    }

    /// is adaptive_sync available for this head
    ///
    /// This events describes if adaptive_sync is available for this head.
    ///
    /// It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `available`:
    #[inline]
    pub fn send_adaptive_sync_available(
        &self,
        available: ZcosmicOutputHeadV1AdaptiveSyncAvailability,
    ) {
        let res = self.try_send_adaptive_sync_available(
            available,
        );
        if let Err(e) = res {
            log_send("zcosmic_output_head_v1.adaptive_sync_available", &e);
        }
    }

    /// Since when the adaptive_sync_ext message is available.
    pub const MSG__ADAPTIVE_SYNC_EXT__SINCE: u32 = 2;

    /// current adaptive_sync state
    ///
    /// This events describes the adaptive_sync state of this head.
    ///
    /// It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_adaptive_sync_ext(
        &self,
        state: ZcosmicOutputHeadV1AdaptiveSyncStateExt,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            state,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: ZcosmicOutputHeadV1AdaptiveSyncStateExt) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zcosmic_output_head_v1#{}.adaptive_sync_ext(state: {:?})\n", client_id, id, arg0);
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
            3,
            arg0.0,
        ]);
        Ok(())
    }

    /// current adaptive_sync state
    ///
    /// This events describes the adaptive_sync state of this head.
    ///
    /// It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_adaptive_sync_ext(
        &self,
        state: ZcosmicOutputHeadV1AdaptiveSyncStateExt,
    ) {
        let res = self.try_send_adaptive_sync_ext(
            state,
        );
        if let Err(e) = res {
            log_send("zcosmic_output_head_v1.adaptive_sync_ext", &e);
        }
    }

    /// Since when the xwayland_primary message is available.
    pub const MSG__XWAYLAND_PRIMARY__SINCE: u32 = 3;

    /// is this head configured as the primary for xwayland
    ///
    /// This event describes if this head is advertised as the primary output via randr to Xwayland.
    ///
    /// At most one output is marked primary, but it is not guaranteed that any output is marked.
    /// It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `state`: boolean if primary or not
    #[inline]
    pub fn try_send_xwayland_primary(
        &self,
        state: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            state,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zcosmic_output_head_v1#{}.xwayland_primary(state: {})\n", client_id, id, arg0);
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
            4,
            arg0,
        ]);
        Ok(())
    }

    /// is this head configured as the primary for xwayland
    ///
    /// This event describes if this head is advertised as the primary output via randr to Xwayland.
    ///
    /// At most one output is marked primary, but it is not guaranteed that any output is marked.
    /// It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `state`: boolean if primary or not
    #[inline]
    pub fn send_xwayland_primary(
        &self,
        state: u32,
    ) {
        let res = self.try_send_xwayland_primary(
            state,
        );
        if let Err(e) = res {
            log_send("zcosmic_output_head_v1.xwayland_primary", &e);
        }
    }
}

/// A message handler for [`ZcosmicOutputHeadV1`] proxies.
pub trait ZcosmicOutputHeadV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZcosmicOutputHeadV1>) {
        slf.core.delete_id();
    }

    /// current scale
    ///
    /// This events describes the scale of the head in the global compositor
    /// space multiplied by 1000 for additional precision.
    ///
    /// It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `scale_1000`:
    #[inline]
    fn handle_scale_1000(
        &mut self,
        slf: &Rc<ZcosmicOutputHeadV1>,
        scale_1000: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_scale_1000(
            scale_1000,
        );
        if let Err(e) = res {
            log_forward("zcosmic_output_head_v1.scale_1000", &e);
        }
    }

    /// mirroring other output
    ///
    /// This events describes that the head is mirroring another.
    /// In these cases `name` contains the unique name of the matching `zwlr_output_head_v1`.
    /// If the name is null, no head is being mirrored onto this one.
    ///
    /// For mirrored heads the `position`-event is meaningless.
    ///
    /// It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    fn handle_mirroring(
        &mut self,
        slf: &Rc<ZcosmicOutputHeadV1>,
        name: Option<&str>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_mirroring(
            name,
        );
        if let Err(e) = res {
            log_forward("zcosmic_output_head_v1.mirroring", &e);
        }
    }

    /// destroy the output head
    ///
    /// Using this request a client can tell the compositor that it is not interested
    /// in the head object anymore.
    #[inline]
    fn handle_release(
        &mut self,
        slf: &Rc<ZcosmicOutputHeadV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_release(
        );
        if let Err(e) = res {
            log_forward("zcosmic_output_head_v1.release", &e);
        }
    }

    /// is adaptive_sync available for this head
    ///
    /// This events describes if adaptive_sync is available for this head.
    ///
    /// It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `available`:
    #[inline]
    fn handle_adaptive_sync_available(
        &mut self,
        slf: &Rc<ZcosmicOutputHeadV1>,
        available: ZcosmicOutputHeadV1AdaptiveSyncAvailability,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_adaptive_sync_available(
            available,
        );
        if let Err(e) = res {
            log_forward("zcosmic_output_head_v1.adaptive_sync_available", &e);
        }
    }

    /// current adaptive_sync state
    ///
    /// This events describes the adaptive_sync state of this head.
    ///
    /// It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_adaptive_sync_ext(
        &mut self,
        slf: &Rc<ZcosmicOutputHeadV1>,
        state: ZcosmicOutputHeadV1AdaptiveSyncStateExt,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_adaptive_sync_ext(
            state,
        );
        if let Err(e) = res {
            log_forward("zcosmic_output_head_v1.adaptive_sync_ext", &e);
        }
    }

    /// is this head configured as the primary for xwayland
    ///
    /// This event describes if this head is advertised as the primary output via randr to Xwayland.
    ///
    /// At most one output is marked primary, but it is not guaranteed that any output is marked.
    /// It is only sent if the output is enabled.
    ///
    /// # Arguments
    ///
    /// - `state`: boolean if primary or not
    #[inline]
    fn handle_xwayland_primary(
        &mut self,
        slf: &Rc<ZcosmicOutputHeadV1>,
        state: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_xwayland_primary(
            state,
        );
        if let Err(e) = res {
            log_forward("zcosmic_output_head_v1.xwayland_primary", &e);
        }
    }
}

impl ObjectPrivate for ZcosmicOutputHeadV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZcosmicOutputHeadV1, version),
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
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_output_head_v1#{}.release()\n", client_id, id);
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = arg0 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zcosmic_output_head_v1#{}.scale_1000(scale_1000: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_scale_1000(&self, arg0);
                } else {
                    DefaultHandler.handle_scale_1000(&self, arg0);
                }
            }
            1 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NullableString>(msg, offset, "name")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: Option<&str>) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zcosmic_output_head_v1#{}.mirroring(name: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_mirroring(&self, arg0);
                } else {
                    DefaultHandler.handle_mirroring(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = ZcosmicOutputHeadV1AdaptiveSyncAvailability(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: ZcosmicOutputHeadV1AdaptiveSyncAvailability) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zcosmic_output_head_v1#{}.adaptive_sync_available(available: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_adaptive_sync_available(&self, arg0);
                } else {
                    DefaultHandler.handle_adaptive_sync_available(&self, arg0);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = ZcosmicOutputHeadV1AdaptiveSyncStateExt(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: ZcosmicOutputHeadV1AdaptiveSyncStateExt) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zcosmic_output_head_v1#{}.adaptive_sync_ext(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_adaptive_sync_ext(&self, arg0);
                } else {
                    DefaultHandler.handle_adaptive_sync_ext(&self, arg0);
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
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zcosmic_output_head_v1#{}.xwayland_primary(state: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_xwayland_primary(&self, arg0);
                } else {
                    DefaultHandler.handle_xwayland_primary(&self, arg0);
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
            0 => "release",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "scale_1000",
            1 => "mirroring",
            2 => "adaptive_sync_available",
            3 => "adaptive_sync_ext",
            4 => "xwayland_primary",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZcosmicOutputHeadV1 {
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

impl ZcosmicOutputHeadV1 {
    /// Since when the adaptive_sync_availability.unsupported enum variant is available.
    pub const ENM__ADAPTIVE_SYNC_AVAILABILITY_UNSUPPORTED__SINCE: u32 = 1;
    /// Since when the adaptive_sync_availability.requires_modeset enum variant is available.
    pub const ENM__ADAPTIVE_SYNC_AVAILABILITY_REQUIRES_MODESET__SINCE: u32 = 1;
    /// Since when the adaptive_sync_availability.supported enum variant is available.
    pub const ENM__ADAPTIVE_SYNC_AVAILABILITY_SUPPORTED__SINCE: u32 = 1;

    /// Since when the adaptive_sync_state_ext.disabled enum variant is available.
    pub const ENM__ADAPTIVE_SYNC_STATE_EXT_DISABLED__SINCE: u32 = 1;
    /// Since when the adaptive_sync_state_ext.automatic enum variant is available.
    pub const ENM__ADAPTIVE_SYNC_STATE_EXT_AUTOMATIC__SINCE: u32 = 1;
    /// Since when the adaptive_sync_state_ext.always enum variant is available.
    pub const ENM__ADAPTIVE_SYNC_STATE_EXT_ALWAYS__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZcosmicOutputHeadV1AdaptiveSyncAvailability(pub u32);

impl ZcosmicOutputHeadV1AdaptiveSyncAvailability {
    /// adaptive sync is not supported
    pub const UNSUPPORTED: Self = Self(0);

    /// automatic adaptive_sync is unavailable
    pub const REQUIRES_MODESET: Self = Self(1);

    /// adaptive sync is supported in all states
    pub const SUPPORTED: Self = Self(2);
}

impl Debug for ZcosmicOutputHeadV1AdaptiveSyncAvailability {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::UNSUPPORTED => "UNSUPPORTED",
            Self::REQUIRES_MODESET => "REQUIRES_MODESET",
            Self::SUPPORTED => "SUPPORTED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZcosmicOutputHeadV1AdaptiveSyncStateExt(pub u32);

impl ZcosmicOutputHeadV1AdaptiveSyncStateExt {
    /// adaptive sync is disabled
    pub const DISABLED: Self = Self(0);

    /// adaptive sync will be actived automatically
    pub const AUTOMATIC: Self = Self(1);

    /// adaptive sync is forced to be always active
    pub const ALWAYS: Self = Self(2);
}

impl Debug for ZcosmicOutputHeadV1AdaptiveSyncStateExt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DISABLED => "DISABLED",
            Self::AUTOMATIC => "AUTOMATIC",
            Self::ALWAYS => "ALWAYS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
