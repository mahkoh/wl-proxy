//! layer shell seat state
//!
//! The lifetime of this object is tied to the corresponding river_seat_v1.
//! This object is made inert when the river_seat_v1.removed event is sent and
//! should be destroyed.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A river_layer_shell_seat_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct RiverLayerShellSeatV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn RiverLayerShellSeatV1Handler>,
}

struct DefaultHandler;

impl RiverLayerShellSeatV1Handler for DefaultHandler { }

impl ConcreteObject for RiverLayerShellSeatV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::RiverLayerShellSeatV1;
    const INTERFACE_NAME: &str = "river_layer_shell_seat_v1";
}

impl RiverLayerShellSeatV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl RiverLayerShellSeatV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn RiverLayerShellSeatV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for RiverLayerShellSeatV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RiverLayerShellSeatV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl RiverLayerShellSeatV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the object
    ///
    /// This request indicates that the client will no longer use the
    /// river_layer_shell_seat_v1 object and that it may be safely destroyed.
    ///
    /// This request should be made after the river_seat_v1.removed event is
    /// received to complete destruction of the seat.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_layer_shell_seat_v1#{}.destroy()\n", id);
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

    /// destroy the object
    ///
    /// This request indicates that the client will no longer use the
    /// river_layer_shell_seat_v1 object and that it may be safely destroyed.
    ///
    /// This request should be made after the river_seat_v1.removed event is
    /// received to complete destruction of the seat.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("river_layer_shell_seat_v1.destroy", &e);
        }
    }

    /// Since when the focus_exclusive message is available.
    pub const MSG__FOCUS_EXCLUSIVE__SINCE: u32 = 1;

    ///
    /// A layer shell surface will be given exclusive keyboard focus at the end
    /// of the manage sequence in which this event is sent. The window manager
    /// may want to update window decorations or similar to indicate that no
    /// window is focused.
    ///
    /// Until the focus_non_exclusive or focus_none event is sent, all window
    /// manager requests to change focus are ignored.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn try_send_focus_exclusive(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_layer_shell_seat_v1#{}.focus_exclusive()\n", client_id, id);
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

    ///
    /// A layer shell surface will be given exclusive keyboard focus at the end
    /// of the manage sequence in which this event is sent. The window manager
    /// may want to update window decorations or similar to indicate that no
    /// window is focused.
    ///
    /// Until the focus_non_exclusive or focus_none event is sent, all window
    /// manager requests to change focus are ignored.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn send_focus_exclusive(
        &self,
    ) {
        let res = self.try_send_focus_exclusive(
        );
        if let Err(e) = res {
            log_send("river_layer_shell_seat_v1.focus_exclusive", &e);
        }
    }

    /// Since when the focus_non_exclusive message is available.
    pub const MSG__FOCUS_NON_EXCLUSIVE__SINCE: u32 = 1;

    ///
    /// A layer shell surface will be given non-exclusive keyboard focus at the
    /// end of the manage sequence in which this event is sent. The window
    /// manager may want to update window decorations or similar to indicate
    /// that no window is focused.
    ///
    /// The window manager continues to control focus and may choose to focus a
    /// different window/shell surface at any time. If the window manager sets
    /// focus during the same manage sequence in which this event is sent, the
    /// layer surface will not be focused.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn try_send_focus_non_exclusive(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_layer_shell_seat_v1#{}.focus_non_exclusive()\n", client_id, id);
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

    ///
    /// A layer shell surface will be given non-exclusive keyboard focus at the
    /// end of the manage sequence in which this event is sent. The window
    /// manager may want to update window decorations or similar to indicate
    /// that no window is focused.
    ///
    /// The window manager continues to control focus and may choose to focus a
    /// different window/shell surface at any time. If the window manager sets
    /// focus during the same manage sequence in which this event is sent, the
    /// layer surface will not be focused.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn send_focus_non_exclusive(
        &self,
    ) {
        let res = self.try_send_focus_non_exclusive(
        );
        if let Err(e) = res {
            log_send("river_layer_shell_seat_v1.focus_non_exclusive", &e);
        }
    }

    /// Since when the focus_none message is available.
    pub const MSG__FOCUS_NONE__SINCE: u32 = 1;

    ///
    /// No layer shell surface will have keyboard focus at the end of the manage
    /// sequence in which this event is sent. The window manager may want to
    /// return focus to whichever window last had focus, for example.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn try_send_focus_none(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_layer_shell_seat_v1#{}.focus_none()\n", client_id, id);
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
        Ok(())
    }

    ///
    /// No layer shell surface will have keyboard focus at the end of the manage
    /// sequence in which this event is sent. The window manager may want to
    /// return focus to whichever window last had focus, for example.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn send_focus_none(
        &self,
    ) {
        let res = self.try_send_focus_none(
        );
        if let Err(e) = res {
            log_send("river_layer_shell_seat_v1.focus_none", &e);
        }
    }
}

/// A message handler for [`RiverLayerShellSeatV1`] proxies.
pub trait RiverLayerShellSeatV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<RiverLayerShellSeatV1>) {
        slf.core.delete_id();
    }

    /// destroy the object
    ///
    /// This request indicates that the client will no longer use the
    /// river_layer_shell_seat_v1 object and that it may be safely destroyed.
    ///
    /// This request should be made after the river_seat_v1.removed event is
    /// received to complete destruction of the seat.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<RiverLayerShellSeatV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("river_layer_shell_seat_v1.destroy", &e);
        }
    }

    ///
    /// A layer shell surface will be given exclusive keyboard focus at the end
    /// of the manage sequence in which this event is sent. The window manager
    /// may want to update window decorations or similar to indicate that no
    /// window is focused.
    ///
    /// Until the focus_non_exclusive or focus_none event is sent, all window
    /// manager requests to change focus are ignored.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    fn handle_focus_exclusive(
        &mut self,
        slf: &Rc<RiverLayerShellSeatV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_focus_exclusive(
        );
        if let Err(e) = res {
            log_forward("river_layer_shell_seat_v1.focus_exclusive", &e);
        }
    }

    ///
    /// A layer shell surface will be given non-exclusive keyboard focus at the
    /// end of the manage sequence in which this event is sent. The window
    /// manager may want to update window decorations or similar to indicate
    /// that no window is focused.
    ///
    /// The window manager continues to control focus and may choose to focus a
    /// different window/shell surface at any time. If the window manager sets
    /// focus during the same manage sequence in which this event is sent, the
    /// layer surface will not be focused.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    fn handle_focus_non_exclusive(
        &mut self,
        slf: &Rc<RiverLayerShellSeatV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_focus_non_exclusive(
        );
        if let Err(e) = res {
            log_forward("river_layer_shell_seat_v1.focus_non_exclusive", &e);
        }
    }

    ///
    /// No layer shell surface will have keyboard focus at the end of the manage
    /// sequence in which this event is sent. The window manager may want to
    /// return focus to whichever window last had focus, for example.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    fn handle_focus_none(
        &mut self,
        slf: &Rc<RiverLayerShellSeatV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_focus_none(
        );
        if let Err(e) = res {
            log_forward("river_layer_shell_seat_v1.focus_none", &e);
        }
    }
}

impl ObjectPrivate for RiverLayerShellSeatV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::RiverLayerShellSeatV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_layer_shell_seat_v1#{}.destroy()\n", client_id, id);
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
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_layer_shell_seat_v1#{}.focus_exclusive()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_focus_exclusive(&self);
                } else {
                    DefaultHandler.handle_focus_exclusive(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_layer_shell_seat_v1#{}.focus_non_exclusive()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_focus_non_exclusive(&self);
                } else {
                    DefaultHandler.handle_focus_non_exclusive(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_layer_shell_seat_v1#{}.focus_none()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_focus_none(&self);
                } else {
                    DefaultHandler.handle_focus_none(&self);
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
            0 => "focus_exclusive",
            1 => "focus_non_exclusive",
            2 => "focus_none",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for RiverLayerShellSeatV1 {
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

