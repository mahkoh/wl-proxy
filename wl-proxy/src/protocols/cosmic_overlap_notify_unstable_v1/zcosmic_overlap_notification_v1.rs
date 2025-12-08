//! Subscription for overlapping toplevels on a layer-surface

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zcosmic_overlap_notification_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZcosmicOverlapNotificationV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZcosmicOverlapNotificationV1Handler>,
}

struct DefaultHandler;

impl ZcosmicOverlapNotificationV1Handler for DefaultHandler { }

impl ConcreteObject for ZcosmicOverlapNotificationV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZcosmicOverlapNotificationV1;
    const INTERFACE_NAME: &str = "zcosmic_overlap_notification_v1";
}

impl ZcosmicOverlapNotificationV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZcosmicOverlapNotificationV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZcosmicOverlapNotificationV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZcosmicOverlapNotificationV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZcosmicOverlapNotificationV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZcosmicOverlapNotificationV1 {
    /// Since when the toplevel_enter message is available.
    pub const MSG__TOPLEVEL_ENTER__SINCE: u32 = 1;

    /// Toplevel entered the surface area
    ///
    /// A ext_foreign_toplevel_handle_v1 has entered the surface area.
    ///
    /// This event will be emitted once for every ext_foreign_toplevel_handle_v1
    /// representing this toplevel.
    ///
    /// Compositors are free to update the overlapping area by sending additional
    /// `toplevel_enter` events for the same toplevel without sending `toplevel_leave`
    /// in between.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    /// - `x`: x coordinate of the upper-left corner of the overlapping area
    /// - `y`: y coordinate of the upper-left corner of the overlapping area
    /// - `width`: width of the overlapping area
    /// - `height`: height of the overlapping area
    #[inline]
    pub fn try_send_toplevel_enter(
        &self,
        toplevel: &Rc<ExtForeignToplevelHandleV1>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ) = (
            toplevel,
            x,
            y,
            width,
            height,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("toplevel", client.endpoint.id)));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: i32, arg2: i32, arg3: i32, arg4: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zcosmic_overlap_notification_v1#{}.toplevel_enter(toplevel: ext_foreign_toplevel_handle_v1#{}, x: {}, y: {}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0_id, arg1, arg2, arg3, arg4);
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
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
            arg4 as u32,
        ]);
        Ok(())
    }

    /// Toplevel entered the surface area
    ///
    /// A ext_foreign_toplevel_handle_v1 has entered the surface area.
    ///
    /// This event will be emitted once for every ext_foreign_toplevel_handle_v1
    /// representing this toplevel.
    ///
    /// Compositors are free to update the overlapping area by sending additional
    /// `toplevel_enter` events for the same toplevel without sending `toplevel_leave`
    /// in between.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    /// - `x`: x coordinate of the upper-left corner of the overlapping area
    /// - `y`: y coordinate of the upper-left corner of the overlapping area
    /// - `width`: width of the overlapping area
    /// - `height`: height of the overlapping area
    #[inline]
    pub fn send_toplevel_enter(
        &self,
        toplevel: &Rc<ExtForeignToplevelHandleV1>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_toplevel_enter(
            toplevel,
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_send("zcosmic_overlap_notification_v1.toplevel_enter", &e);
        }
    }

    /// Since when the toplevel_leave message is available.
    pub const MSG__TOPLEVEL_LEAVE__SINCE: u32 = 1;

    /// Toplevel entered the surface area
    ///
    /// A ext_foreign_toplevel_handle_v1 has left the surface area.
    ///
    /// This event will be emitted once for every ext_foreign_toplevel_handle_v1
    /// representing this toplevel.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    #[inline]
    pub fn try_send_toplevel_leave(
        &self,
        toplevel: &Rc<ExtForeignToplevelHandleV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            toplevel,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("toplevel", client.endpoint.id)));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zcosmic_overlap_notification_v1#{}.toplevel_leave(toplevel: ext_foreign_toplevel_handle_v1#{})\n", client_id, id, arg0);
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
            1,
            arg0_id,
        ]);
        Ok(())
    }

    /// Toplevel entered the surface area
    ///
    /// A ext_foreign_toplevel_handle_v1 has left the surface area.
    ///
    /// This event will be emitted once for every ext_foreign_toplevel_handle_v1
    /// representing this toplevel.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    #[inline]
    pub fn send_toplevel_leave(
        &self,
        toplevel: &Rc<ExtForeignToplevelHandleV1>,
    ) {
        let res = self.try_send_toplevel_leave(
            toplevel,
        );
        if let Err(e) = res {
            log_send("zcosmic_overlap_notification_v1.toplevel_leave", &e);
        }
    }

    /// Since when the layer_enter message is available.
    pub const MSG__LAYER_ENTER__SINCE: u32 = 1;

    /// Layer surface entered the surface area
    ///
    /// A zwlr_layer_surface_v1 has entered the surface area.
    ///
    /// Compositors are free to update the overlapping area by sending additional
    /// `layer_enter` events for the same surface without sending `layer_leave`
    /// in between.
    ///
    /// The overlapping region is given surface-relative to the zwlr_layer_surface_v1
    /// used to create this notification object.
    ///
    /// # Arguments
    ///
    /// - `identifier`: unique identifier for the overlapping layer-surface
    /// - `namespace`: namespace for the layer surface
    /// - `exclusive`: if the overlapping surface is requesting an exclusive area or not
    /// - `layer`: layer the overlapping surface sits on
    /// - `x`: x coordinate of the upper-left corner of the overlapping area
    /// - `y`: y coordinate of the upper-left corner of the overlapping area
    /// - `width`: width of the overlapping area
    /// - `height`: height of the overlapping area
    #[inline]
    pub fn try_send_layer_enter(
        &self,
        identifier: &str,
        namespace: &str,
        exclusive: u32,
        layer: ZwlrLayerShellV1Layer,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
            arg6,
            arg7,
        ) = (
            identifier,
            namespace,
            exclusive,
            layer,
            x,
            y,
            width,
            height,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: &str, arg1: &str, arg2: u32, arg3: ZwlrLayerShellV1Layer, arg4: i32, arg5: i32, arg6: i32, arg7: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zcosmic_overlap_notification_v1#{}.layer_enter(identifier: {:?}, namespace: {:?}, exclusive: {}, layer: {:?}, x: {}, y: {}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
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
        fmt.string(arg0);
        fmt.string(arg1);
        fmt.words([
            arg2,
            arg3.0,
            arg4 as u32,
            arg5 as u32,
            arg6 as u32,
            arg7 as u32,
        ]);
        Ok(())
    }

    /// Layer surface entered the surface area
    ///
    /// A zwlr_layer_surface_v1 has entered the surface area.
    ///
    /// Compositors are free to update the overlapping area by sending additional
    /// `layer_enter` events for the same surface without sending `layer_leave`
    /// in between.
    ///
    /// The overlapping region is given surface-relative to the zwlr_layer_surface_v1
    /// used to create this notification object.
    ///
    /// # Arguments
    ///
    /// - `identifier`: unique identifier for the overlapping layer-surface
    /// - `namespace`: namespace for the layer surface
    /// - `exclusive`: if the overlapping surface is requesting an exclusive area or not
    /// - `layer`: layer the overlapping surface sits on
    /// - `x`: x coordinate of the upper-left corner of the overlapping area
    /// - `y`: y coordinate of the upper-left corner of the overlapping area
    /// - `width`: width of the overlapping area
    /// - `height`: height of the overlapping area
    #[inline]
    pub fn send_layer_enter(
        &self,
        identifier: &str,
        namespace: &str,
        exclusive: u32,
        layer: ZwlrLayerShellV1Layer,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_layer_enter(
            identifier,
            namespace,
            exclusive,
            layer,
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_send("zcosmic_overlap_notification_v1.layer_enter", &e);
        }
    }

    /// Since when the layer_leave message is available.
    pub const MSG__LAYER_LEAVE__SINCE: u32 = 1;

    /// Layer surface left the surface area
    ///
    /// A zwlr_layer_surface_v1 has left the surface area.
    ///
    /// # Arguments
    ///
    /// - `identifier`: unique identifier for the overlapping layer-surface
    #[inline]
    pub fn try_send_layer_leave(
        &self,
        identifier: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            identifier,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zcosmic_overlap_notification_v1#{}.layer_leave(identifier: {:?})\n", client_id, id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Layer surface left the surface area
    ///
    /// A zwlr_layer_surface_v1 has left the surface area.
    ///
    /// # Arguments
    ///
    /// - `identifier`: unique identifier for the overlapping layer-surface
    #[inline]
    pub fn send_layer_leave(
        &self,
        identifier: &str,
    ) {
        let res = self.try_send_layer_leave(
            identifier,
        );
        if let Err(e) = res {
            log_send("zcosmic_overlap_notification_v1.layer_leave", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the notification object
    ///
    /// This request should be called when the client has no interest in overlap
    /// notifications anymore.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_overlap_notification_v1#{}.destroy()\n", id);
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

    /// destroy the notification object
    ///
    /// This request should be called when the client has no interest in overlap
    /// notifications anymore.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zcosmic_overlap_notification_v1.destroy", &e);
        }
    }
}

/// A message handler for [`ZcosmicOverlapNotificationV1`] proxies.
pub trait ZcosmicOverlapNotificationV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZcosmicOverlapNotificationV1>) {
        slf.core.delete_id();
    }

    /// Toplevel entered the surface area
    ///
    /// A ext_foreign_toplevel_handle_v1 has entered the surface area.
    ///
    /// This event will be emitted once for every ext_foreign_toplevel_handle_v1
    /// representing this toplevel.
    ///
    /// Compositors are free to update the overlapping area by sending additional
    /// `toplevel_enter` events for the same toplevel without sending `toplevel_leave`
    /// in between.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    /// - `x`: x coordinate of the upper-left corner of the overlapping area
    /// - `y`: y coordinate of the upper-left corner of the overlapping area
    /// - `width`: width of the overlapping area
    /// - `height`: height of the overlapping area
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_toplevel_enter(
        &mut self,
        slf: &Rc<ZcosmicOverlapNotificationV1>,
        toplevel: &Rc<ExtForeignToplevelHandleV1>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = toplevel.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = slf.try_send_toplevel_enter(
            toplevel,
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("zcosmic_overlap_notification_v1.toplevel_enter", &e);
        }
    }

    /// Toplevel entered the surface area
    ///
    /// A ext_foreign_toplevel_handle_v1 has left the surface area.
    ///
    /// This event will be emitted once for every ext_foreign_toplevel_handle_v1
    /// representing this toplevel.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_toplevel_leave(
        &mut self,
        slf: &Rc<ZcosmicOverlapNotificationV1>,
        toplevel: &Rc<ExtForeignToplevelHandleV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = toplevel.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = slf.try_send_toplevel_leave(
            toplevel,
        );
        if let Err(e) = res {
            log_forward("zcosmic_overlap_notification_v1.toplevel_leave", &e);
        }
    }

    /// Layer surface entered the surface area
    ///
    /// A zwlr_layer_surface_v1 has entered the surface area.
    ///
    /// Compositors are free to update the overlapping area by sending additional
    /// `layer_enter` events for the same surface without sending `layer_leave`
    /// in between.
    ///
    /// The overlapping region is given surface-relative to the zwlr_layer_surface_v1
    /// used to create this notification object.
    ///
    /// # Arguments
    ///
    /// - `identifier`: unique identifier for the overlapping layer-surface
    /// - `namespace`: namespace for the layer surface
    /// - `exclusive`: if the overlapping surface is requesting an exclusive area or not
    /// - `layer`: layer the overlapping surface sits on
    /// - `x`: x coordinate of the upper-left corner of the overlapping area
    /// - `y`: y coordinate of the upper-left corner of the overlapping area
    /// - `width`: width of the overlapping area
    /// - `height`: height of the overlapping area
    #[inline]
    fn handle_layer_enter(
        &mut self,
        slf: &Rc<ZcosmicOverlapNotificationV1>,
        identifier: &str,
        namespace: &str,
        exclusive: u32,
        layer: ZwlrLayerShellV1Layer,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_layer_enter(
            identifier,
            namespace,
            exclusive,
            layer,
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("zcosmic_overlap_notification_v1.layer_enter", &e);
        }
    }

    /// Layer surface left the surface area
    ///
    /// A zwlr_layer_surface_v1 has left the surface area.
    ///
    /// # Arguments
    ///
    /// - `identifier`: unique identifier for the overlapping layer-surface
    #[inline]
    fn handle_layer_leave(
        &mut self,
        slf: &Rc<ZcosmicOverlapNotificationV1>,
        identifier: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_layer_leave(
            identifier,
        );
        if let Err(e) = res {
            log_forward("zcosmic_overlap_notification_v1.layer_leave", &e);
        }
    }

    /// destroy the notification object
    ///
    /// This request should be called when the client has no interest in overlap
    /// notifications anymore.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZcosmicOverlapNotificationV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zcosmic_overlap_notification_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for ZcosmicOverlapNotificationV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZcosmicOverlapNotificationV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_overlap_notification_v1#{}.destroy()\n", client_id, id);
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
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 28)));
                };
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                let arg4 = arg4 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: i32, arg2: i32, arg3: i32, arg4: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zcosmic_overlap_notification_v1#{}.toplevel_enter(toplevel: ext_foreign_toplevel_handle_v1#{}, x: {}, y: {}, width: {}, height: {})\n", id, arg0, arg1, arg2, arg3, arg4);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2, arg3, arg4);
                }
                let arg0_id = arg0;
                let Some(arg0) = server.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ExtForeignToplevelHandleV1>() else {
                    let o = server.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("toplevel", o.core().interface, ObjectInterface::ExtForeignToplevelHandleV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_toplevel_enter(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultHandler.handle_toplevel_enter(&self, arg0, arg1, arg2, arg3, arg4);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zcosmic_overlap_notification_v1#{}.toplevel_leave(toplevel: ext_foreign_toplevel_handle_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = server.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ExtForeignToplevelHandleV1>() else {
                    let o = server.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("toplevel", o.core().interface, ObjectInterface::ExtForeignToplevelHandleV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_toplevel_leave(&self, arg0);
                } else {
                    DefaultHandler.handle_toplevel_leave(&self, arg0);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "identifier")?;
                let arg1;
                (arg1, offset) = parse_string::<NonNullString>(msg, offset, "namespace")?;
                let Some(&arg2) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("exclusive")));
                };
                offset += 1;
                let Some(&arg3) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("layer")));
                };
                offset += 1;
                let Some(&arg4) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("x")));
                };
                offset += 1;
                let Some(&arg5) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("y")));
                };
                offset += 1;
                let Some(&arg6) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("width")));
                };
                offset += 1;
                let Some(&arg7) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("height")));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                let arg3 = ZwlrLayerShellV1Layer(arg3);
                let arg4 = arg4 as i32;
                let arg5 = arg5 as i32;
                let arg6 = arg6 as i32;
                let arg7 = arg7 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str, arg1: &str, arg2: u32, arg3: ZwlrLayerShellV1Layer, arg4: i32, arg5: i32, arg6: i32, arg7: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zcosmic_overlap_notification_v1#{}.layer_enter(identifier: {:?}, namespace: {:?}, exclusive: {}, layer: {:?}, x: {}, y: {}, width: {}, height: {})\n", id, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
                }
                if let Some(handler) = handler {
                    (**handler).handle_layer_enter(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
                } else {
                    DefaultHandler.handle_layer_enter(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
                }
            }
            3 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "identifier")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zcosmic_overlap_notification_v1#{}.layer_leave(identifier: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_layer_leave(&self, arg0);
                } else {
                    DefaultHandler.handle_layer_leave(&self, arg0);
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
            0 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "toplevel_enter",
            1 => "toplevel_leave",
            2 => "layer_enter",
            3 => "layer_leave",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZcosmicOverlapNotificationV1 {
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

