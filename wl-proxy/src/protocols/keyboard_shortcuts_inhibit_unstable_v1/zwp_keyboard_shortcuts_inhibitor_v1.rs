//! context object for keyboard shortcuts inhibitor
//!
//! A keyboard shortcuts inhibitor instructs the compositor to ignore
//! its own keyboard shortcuts when the associated surface has keyboard
//! focus. As a result, when the surface has keyboard focus on the given
//! seat, it will receive all key events originating from the specified
//! seat, even those which would normally be caught by the compositor for
//! its own shortcuts.
//!
//! The Wayland compositor is however under no obligation to disable
//! all of its shortcuts, and may keep some special key combo for its own
//! use, including but not limited to one allowing the user to forcibly
//! restore normal keyboard events routing in the case of an unwilling
//! client. The compositor may also use the same key combo to reactivate
//! an existing shortcut inhibitor that was previously deactivated on
//! user request.
//!
//! When the compositor restores its own keyboard shortcuts, an
//! "inactive" event is emitted to notify the client that the keyboard
//! shortcuts inhibitor is not effectively active for the surface and
//! seat any more, and the client should not expect to receive all
//! keyboard events.
//!
//! When the keyboard shortcuts inhibitor is inactive, the client has
//! no way to forcibly reactivate the keyboard shortcuts inhibitor.
//!
//! The user can chose to re-enable a previously deactivated keyboard
//! shortcuts inhibitor using any mechanism the compositor may offer,
//! in which case the compositor will send an "active" event to notify
//! the client.
//!
//! If the surface is destroyed, unmapped, or loses the seat's keyboard
//! focus, the keyboard shortcuts inhibitor becomes irrelevant and the
//! compositor will restore its own keyboard shortcuts but no "inactive"
//! event is emitted in this case.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_keyboard_shortcuts_inhibitor_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpKeyboardShortcutsInhibitorV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpKeyboardShortcutsInhibitorV1Handler>,
}

struct DefaultHandler;

impl ZwpKeyboardShortcutsInhibitorV1Handler for DefaultHandler { }

impl ConcreteObject for ZwpKeyboardShortcutsInhibitorV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpKeyboardShortcutsInhibitorV1;
    const INTERFACE_NAME: &str = "zwp_keyboard_shortcuts_inhibitor_v1";
}

impl ZwpKeyboardShortcutsInhibitorV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpKeyboardShortcutsInhibitorV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpKeyboardShortcutsInhibitorV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpKeyboardShortcutsInhibitorV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpKeyboardShortcutsInhibitorV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpKeyboardShortcutsInhibitorV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the keyboard shortcuts inhibitor object
    ///
    /// Remove the keyboard shortcuts inhibitor from the associated wl_surface.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_keyboard_shortcuts_inhibitor_v1#{}.destroy()\n", id);
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

    /// destroy the keyboard shortcuts inhibitor object
    ///
    /// Remove the keyboard shortcuts inhibitor from the associated wl_surface.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwp_keyboard_shortcuts_inhibitor_v1.destroy", &e);
        }
    }

    /// Since when the active message is available.
    pub const MSG__ACTIVE__SINCE: u32 = 1;

    /// shortcuts are inhibited
    ///
    /// This event indicates that the shortcut inhibitor is active.
    ///
    /// The compositor sends this event every time compositor shortcuts
    /// are inhibited on behalf of the surface. When active, the client
    /// may receive input events normally reserved by the compositor
    /// (see zwp_keyboard_shortcuts_inhibitor_v1).
    ///
    /// This occurs typically when the initial request "inhibit_shortcuts"
    /// first becomes active or when the user instructs the compositor to
    /// re-enable and existing shortcuts inhibitor using any mechanism
    /// offered by the compositor.
    #[inline]
    pub fn try_send_active(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_keyboard_shortcuts_inhibitor_v1#{}.active()\n", client_id, id);
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

    /// shortcuts are inhibited
    ///
    /// This event indicates that the shortcut inhibitor is active.
    ///
    /// The compositor sends this event every time compositor shortcuts
    /// are inhibited on behalf of the surface. When active, the client
    /// may receive input events normally reserved by the compositor
    /// (see zwp_keyboard_shortcuts_inhibitor_v1).
    ///
    /// This occurs typically when the initial request "inhibit_shortcuts"
    /// first becomes active or when the user instructs the compositor to
    /// re-enable and existing shortcuts inhibitor using any mechanism
    /// offered by the compositor.
    #[inline]
    pub fn send_active(
        &self,
    ) {
        let res = self.try_send_active(
        );
        if let Err(e) = res {
            log_send("zwp_keyboard_shortcuts_inhibitor_v1.active", &e);
        }
    }

    /// Since when the inactive message is available.
    pub const MSG__INACTIVE__SINCE: u32 = 1;

    /// shortcuts are restored
    ///
    /// This event indicates that the shortcuts inhibitor is inactive,
    /// normal shortcuts processing is restored by the compositor.
    #[inline]
    pub fn try_send_inactive(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_keyboard_shortcuts_inhibitor_v1#{}.inactive()\n", client_id, id);
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

    /// shortcuts are restored
    ///
    /// This event indicates that the shortcuts inhibitor is inactive,
    /// normal shortcuts processing is restored by the compositor.
    #[inline]
    pub fn send_inactive(
        &self,
    ) {
        let res = self.try_send_inactive(
        );
        if let Err(e) = res {
            log_send("zwp_keyboard_shortcuts_inhibitor_v1.inactive", &e);
        }
    }
}

/// A message handler for [`ZwpKeyboardShortcutsInhibitorV1`] proxies.
pub trait ZwpKeyboardShortcutsInhibitorV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpKeyboardShortcutsInhibitorV1>) {
        slf.core.delete_id();
    }

    /// destroy the keyboard shortcuts inhibitor object
    ///
    /// Remove the keyboard shortcuts inhibitor from the associated wl_surface.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwpKeyboardShortcutsInhibitorV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwp_keyboard_shortcuts_inhibitor_v1.destroy", &e);
        }
    }

    /// shortcuts are inhibited
    ///
    /// This event indicates that the shortcut inhibitor is active.
    ///
    /// The compositor sends this event every time compositor shortcuts
    /// are inhibited on behalf of the surface. When active, the client
    /// may receive input events normally reserved by the compositor
    /// (see zwp_keyboard_shortcuts_inhibitor_v1).
    ///
    /// This occurs typically when the initial request "inhibit_shortcuts"
    /// first becomes active or when the user instructs the compositor to
    /// re-enable and existing shortcuts inhibitor using any mechanism
    /// offered by the compositor.
    #[inline]
    fn handle_active(
        &mut self,
        slf: &Rc<ZwpKeyboardShortcutsInhibitorV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_active(
        );
        if let Err(e) = res {
            log_forward("zwp_keyboard_shortcuts_inhibitor_v1.active", &e);
        }
    }

    /// shortcuts are restored
    ///
    /// This event indicates that the shortcuts inhibitor is inactive,
    /// normal shortcuts processing is restored by the compositor.
    #[inline]
    fn handle_inactive(
        &mut self,
        slf: &Rc<ZwpKeyboardShortcutsInhibitorV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_inactive(
        );
        if let Err(e) = res {
            log_forward("zwp_keyboard_shortcuts_inhibitor_v1.inactive", &e);
        }
    }
}

impl ObjectPrivate for ZwpKeyboardShortcutsInhibitorV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpKeyboardShortcutsInhibitorV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_keyboard_shortcuts_inhibitor_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_keyboard_shortcuts_inhibitor_v1#{}.active()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_active(&self);
                } else {
                    DefaultHandler.handle_active(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_keyboard_shortcuts_inhibitor_v1#{}.inactive()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_inactive(&self);
                } else {
                    DefaultHandler.handle_inactive(&self);
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
            0 => "active",
            1 => "inactive",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwpKeyboardShortcutsInhibitorV1 {
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

