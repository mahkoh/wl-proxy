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

impl ConcreteObject for ZwpKeyboardShortcutsInhibitManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpKeyboardShortcutsInhibitManagerV1;
    const INTERFACE_NAME: &str = "zwp_keyboard_shortcuts_inhibit_manager_v1";
}

impl ZwpKeyboardShortcutsInhibitManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpKeyboardShortcutsInhibitManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_keyboard_shortcuts_inhibit_manager_v1#{}.destroy()\n", id);
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
    /// Destroy the keyboard shortcuts inhibitor manager.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwp_keyboard_shortcuts_inhibit_manager_v1.destroy", &e);
        }
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
    pub fn try_send_inhibit_shortcuts(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        let arg2_id = match arg2.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("seat"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_keyboard_shortcuts_inhibit_manager_v1#{}.inhibit_shortcuts(id: zwp_keyboard_shortcuts_inhibitor_v1#{}, surface: wl_surface#{}, seat: wl_seat#{})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id, arg2_id);
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
            arg2_id,
        ]);
        Ok(())
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
    #[inline]
    pub fn send_inhibit_shortcuts(
        &self,
        id: &Rc<ZwpKeyboardShortcutsInhibitorV1>,
        surface: &Rc<WlSurface>,
        seat: &Rc<WlSeat>,
    ) {
        let res = self.try_send_inhibit_shortcuts(
            id,
            surface,
            seat,
        );
        if let Err(e) = res {
            log_send("zwp_keyboard_shortcuts_inhibit_manager_v1.inhibit_shortcuts", &e);
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
    /// - `surface`: the surface that inhibits the keyboard shortcuts behavior
    /// - `seat`: the wl_seat for which keyboard shortcuts should be disabled
    #[inline]
    pub fn new_try_send_inhibit_shortcuts(
        &self,
        surface: &Rc<WlSurface>,
        seat: &Rc<WlSeat>,
    ) -> Result<Rc<ZwpKeyboardShortcutsInhibitorV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_inhibit_shortcuts(
            &id,
            surface,
            seat,
        )?;
        Ok(id)
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
    /// - `surface`: the surface that inhibits the keyboard shortcuts behavior
    /// - `seat`: the wl_seat for which keyboard shortcuts should be disabled
    #[inline]
    pub fn new_send_inhibit_shortcuts(
        &self,
        surface: &Rc<WlSurface>,
        seat: &Rc<WlSeat>,
    ) -> Rc<ZwpKeyboardShortcutsInhibitorV1> {
        let id = self.core.create_child();
        self.send_inhibit_shortcuts(
            &id,
            surface,
            seat,
        );
        id
    }
}

/// A message handler for [`ZwpKeyboardShortcutsInhibitManagerV1`] proxies.
pub trait ZwpKeyboardShortcutsInhibitManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpKeyboardShortcutsInhibitManagerV1>) {
        slf.core.delete_id();
    }

    /// destroy the keyboard shortcuts inhibitor object
    ///
    /// Destroy the keyboard shortcuts inhibitor manager.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwpKeyboardShortcutsInhibitManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwp_keyboard_shortcuts_inhibit_manager_v1.destroy", &e);
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
    fn handle_inhibit_shortcuts(
        &mut self,
        slf: &Rc<ZwpKeyboardShortcutsInhibitManagerV1>,
        id: &Rc<ZwpKeyboardShortcutsInhibitorV1>,
        surface: &Rc<WlSurface>,
        seat: &Rc<WlSeat>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_inhibit_shortcuts(
            id,
            surface,
            seat,
        );
        if let Err(e) = res {
            log_forward("zwp_keyboard_shortcuts_inhibit_manager_v1.inhibit_shortcuts", &e);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_keyboard_shortcuts_inhibit_manager_v1#{}.destroy()\n", client_id, id);
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
            1 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_keyboard_shortcuts_inhibit_manager_v1#{}.inhibit_shortcuts(id: zwp_keyboard_shortcuts_inhibitor_v1#{}, surface: wl_surface#{}, seat: wl_seat#{})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let arg0 = ZwpKeyboardShortcutsInhibitorV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg2_id = arg2;
                let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg2_id)));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg2_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("seat", o.core().interface, ObjectInterface::WlSeat)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).handle_inhibit_shortcuts(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_inhibit_shortcuts(&self, arg0, arg1, arg2);
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
