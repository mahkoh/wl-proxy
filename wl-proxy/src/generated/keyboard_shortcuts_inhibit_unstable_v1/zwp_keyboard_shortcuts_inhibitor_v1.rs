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

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_keyboard_shortcuts_inhibitor_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpKeyboardShortcutsInhibitorV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpKeyboardShortcutsInhibitorV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpKeyboardShortcutsInhibitorV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpKeyboardShortcutsInhibitorV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZwpKeyboardShortcutsInhibitorV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpKeyboardShortcutsInhibitorV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaZwpKeyboardShortcutsInhibitorV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaZwpKeyboardShortcutsInhibitorV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpKeyboardShortcutsInhibitorV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpKeyboardShortcutsInhibitorV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the keyboard shortcuts inhibitor object
    ///
    /// Remove the keyboard shortcuts inhibitor from the associated wl_surface.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
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
            0,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the active message is available.
    #[allow(dead_code)]
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
    pub fn send_active(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the inactive message is available.
    #[allow(dead_code)]
    pub const MSG__INACTIVE__SINCE: u32 = 1;

    /// shortcuts are restored
    ///
    /// This event indicates that the shortcuts inhibitor is inactive,
    /// normal shortcuts processing is restored by the compositor.
    #[inline]
    pub fn send_inactive(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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
}

/// A message handler for [ZwpKeyboardShortcutsInhibitorV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpKeyboardShortcutsInhibitorV1MessageHandler {
    /// destroy the keyboard shortcuts inhibitor object
    ///
    /// Remove the keyboard shortcuts inhibitor from the associated wl_surface.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpKeyboardShortcutsInhibitorV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_keyboard_shortcuts_inhibitor_v1.destroy message: {}", Report::new(e));
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
    fn active(
        &mut self,
        _slf: &Rc<MetaZwpKeyboardShortcutsInhibitorV1>,
    ) {
        let res = _slf.send_active(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_keyboard_shortcuts_inhibitor_v1.active message: {}", Report::new(e));
        }
    }

    /// shortcuts are restored
    ///
    /// This event indicates that the shortcuts inhibitor is inactive,
    /// normal shortcuts processing is restored by the compositor.
    #[inline]
    fn inactive(
        &mut self,
        _slf: &Rc<MetaZwpKeyboardShortcutsInhibitorV1>,
    ) {
        let res = _slf.send_inactive(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_keyboard_shortcuts_inhibitor_v1.inactive message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpKeyboardShortcutsInhibitorV1 {
    fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Self::new(state, version)
    }

    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
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
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).active(&self);
                } else {
                    DefaultMessageHandler.active(&self);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).inactive(&self);
                } else {
                    DefaultMessageHandler.inactive(&self);
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

