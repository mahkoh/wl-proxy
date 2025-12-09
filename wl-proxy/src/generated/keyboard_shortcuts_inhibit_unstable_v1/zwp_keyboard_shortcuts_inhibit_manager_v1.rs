//! context object for keyboard grab_manager
//!
//! A global interface used for inhibiting the compositor keyboard shortcuts.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_keyboard_shortcuts_inhibit_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpKeyboardShortcutsInhibitManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpKeyboardShortcutsInhibitManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpKeyboardShortcutsInhibitManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpKeyboardShortcutsInhibitManagerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZwpKeyboardShortcutsInhibitManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpKeyboardShortcutsInhibitManagerV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaZwpKeyboardShortcutsInhibitManagerV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaZwpKeyboardShortcutsInhibitManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpKeyboardShortcutsInhibitManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpKeyboardShortcutsInhibitManagerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the keyboard shortcuts inhibitor object
    ///
    /// Destroy the keyboard shortcuts inhibitor manager.
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

    /// Since when the inhibit_shortcuts message is available.
    #[allow(dead_code)]
    pub const MSG__INHIBIT_SHORTCUTS__SINCE: u32 = 1;

    /// create a new keyboard shortcuts inhibitor object
    ///
    /// Create a new keyboard shortcuts inhibitor object associated with
    /// the given surface for the given seat.
    ///
    /// If shortcuts are already inhibited for the specified seat and surface,
    /// a protocol error "already_inhibited" is raised by the compositor.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`: the surface that inhibits the keyboard shortcuts behavior
    /// - `seat`: the wl_seat for which keyboard shortcuts should be disabled
    #[inline]
    pub fn send_inhibit_shortcuts(
        &self,
        id: &Rc<MetaZwpKeyboardShortcutsInhibitorV1>,
        surface: &Rc<MetaWlSurface>,
        seat: &Rc<MetaWlSeat>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            id,
            surface,
            seat,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let arg2 = arg2.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("surface")),
            Some(id) => id,
        };
        let arg2_id = match arg2.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("seat")),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
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
            arg0_id,
            arg1_id,
            arg2_id,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpKeyboardShortcutsInhibitManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpKeyboardShortcutsInhibitManagerV1MessageHandler {
    /// destroy the keyboard shortcuts inhibitor object
    ///
    /// Destroy the keyboard shortcuts inhibitor manager.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpKeyboardShortcutsInhibitManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_keyboard_shortcuts_inhibit_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// create a new keyboard shortcuts inhibitor object
    ///
    /// Create a new keyboard shortcuts inhibitor object associated with
    /// the given surface for the given seat.
    ///
    /// If shortcuts are already inhibited for the specified seat and surface,
    /// a protocol error "already_inhibited" is raised by the compositor.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`: the surface that inhibits the keyboard shortcuts behavior
    /// - `seat`: the wl_seat for which keyboard shortcuts should be disabled
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn inhibit_shortcuts(
        &mut self,
        _slf: &Rc<MetaZwpKeyboardShortcutsInhibitManagerV1>,
        id: &Rc<MetaZwpKeyboardShortcutsInhibitorV1>,
        surface: &Rc<MetaWlSurface>,
        seat: &Rc<MetaWlSeat>,
    ) {
        let res = _slf.send_inhibit_shortcuts(
            id,
            surface,
            seat,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_keyboard_shortcuts_inhibit_manager_v1.inhibit_shortcuts message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpKeyboardShortcutsInhibitManagerV1 {
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
            1 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                let arg0_id = arg0;
                let arg0 = MetaZwpKeyboardShortcutsInhibitorV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ProxyInterface::WlSurface));
                };
                let arg2_id = arg2;
                let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg2_id));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<MetaWlSeat>() else {
                    let o = client.endpoint.lookup(arg2_id).unwrap();
                    return Err(ObjectError::WrongObjectType("seat", o.core().interface, ProxyInterface::WlSeat));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).inhibit_shortcuts(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.inhibit_shortcuts(&self, arg0, arg1, arg2);
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
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            1 => "inhibit_shortcuts",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl MetaZwpKeyboardShortcutsInhibitManagerV1 {
    /// Since when the error.already_inhibited enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ALREADY_INHIBITED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaZwpKeyboardShortcutsInhibitManagerV1Error(pub u32);

impl MetaZwpKeyboardShortcutsInhibitManagerV1Error {
    /// the shortcuts are already inhibited for this surface
    #[allow(dead_code)]
    pub const ALREADY_INHIBITED: Self = Self(0);
}

impl Debug for MetaZwpKeyboardShortcutsInhibitManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ALREADY_INHIBITED => "ALREADY_INHIBITED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
