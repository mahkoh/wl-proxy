//! context object for keyboard grab_manager
//!
//! A global interface used for inhibiting the compositor keyboard shortcuts.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_keyboard_shortcuts_inhibit_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpKeyboardShortcutsInhibitManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpKeyboardShortcutsInhibitManagerV1Handler>,
}

struct DefaultHandler;

impl ZwpKeyboardShortcutsInhibitManagerV1Handler for DefaultHandler { }

impl ZwpKeyboardShortcutsInhibitManagerV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::ZwpKeyboardShortcutsInhibitManagerV1;
    pub const INTERFACE_NAME: &str = "zwp_keyboard_shortcuts_inhibit_manager_v1";
}

impl ZwpKeyboardShortcutsInhibitManagerV1 {
    pub fn set_handler(&self, handler: impl ZwpKeyboardShortcutsInhibitManagerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpKeyboardShortcutsInhibitManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpKeyboardShortcutsInhibitManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpKeyboardShortcutsInhibitManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpKeyboardShortcutsInhibitManagerV1 {
    /// Since when the destroy message is available.
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_keyboard_shortcuts_inhibit_manager_v1#{}.destroy()\n", id);
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
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

    /// Since when the inhibit_shortcuts message is available.
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
        id: &Rc<ZwpKeyboardShortcutsInhibitorV1>,
        surface: &Rc<WlSurface>,
        seat: &Rc<WlSeat>,
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_keyboard_shortcuts_inhibit_manager_v1#{}.inhibit_shortcuts(id: zwp_keyboard_shortcuts_inhibitor_v1#{}, surface: wl_surface#{}, seat: wl_seat#{})\n", id, arg0_id, arg1_id, arg2_id);
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
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
            arg2_id,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpKeyboardShortcutsInhibitManagerV1] proxies.
pub trait ZwpKeyboardShortcutsInhibitManagerV1Handler: Any {
    /// destroy the keyboard shortcuts inhibitor object
    ///
    /// Destroy the keyboard shortcuts inhibitor manager.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<ZwpKeyboardShortcutsInhibitManagerV1>,
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
        _slf: &Rc<ZwpKeyboardShortcutsInhibitManagerV1>,
        id: &Rc<ZwpKeyboardShortcutsInhibitorV1>,
        surface: &Rc<WlSurface>,
        seat: &Rc<WlSeat>,
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

impl ObjectPrivate for ZwpKeyboardShortcutsInhibitManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpKeyboardShortcutsInhibitManagerV1, version),
            handler: Default::default(),
        })
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_keyboard_shortcuts_inhibit_manager_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
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
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_keyboard_shortcuts_inhibit_manager_v1#{}.inhibit_shortcuts(id: zwp_keyboard_shortcuts_inhibitor_v1#{}, surface: wl_surface#{}, seat: wl_seat#{})\n", client.endpoint.id, msg[0], arg0, arg1, arg2);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ZwpKeyboardShortcutsInhibitorV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface));
                };
                let arg2_id = arg2;
                let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg2_id));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg2_id).unwrap();
                    return Err(ObjectError::WrongObjectType("seat", o.core().interface, ObjectInterface::WlSeat));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).inhibit_shortcuts(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.inhibit_shortcuts(&self, arg0, arg1, arg2);
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
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
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

impl Object for ZwpKeyboardShortcutsInhibitManagerV1 {
    fn core(&self) -> &ObjectCore {
        &self.core
    }

    fn unset_handler(&self) {
        self.handler.set(None);
    }

    fn get_handler_any_ref(&self) -> Result<Ref<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(Ref::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<RefMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow_mut().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(RefMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

impl ZwpKeyboardShortcutsInhibitManagerV1 {
    /// Since when the error.already_inhibited enum variant is available.
    pub const ENM__ERROR_ALREADY_INHIBITED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwpKeyboardShortcutsInhibitManagerV1Error(pub u32);

impl ZwpKeyboardShortcutsInhibitManagerV1Error {
    /// the shortcuts are already inhibited for this surface
    pub const ALREADY_INHIBITED: Self = Self(0);
}

impl Debug for ZwpKeyboardShortcutsInhibitManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ALREADY_INHIBITED => "ALREADY_INHIBITED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
