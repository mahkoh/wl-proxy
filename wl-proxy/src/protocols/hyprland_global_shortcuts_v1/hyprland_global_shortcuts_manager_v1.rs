//! manager to register global shortcuts
//!
//! This object is a manager which offers requests to create global shortcuts.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A hyprland_global_shortcuts_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct HyprlandGlobalShortcutsManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn HyprlandGlobalShortcutsManagerV1Handler>,
}

struct DefaultHandler;

impl HyprlandGlobalShortcutsManagerV1Handler for DefaultHandler { }

impl ConcreteObject for HyprlandGlobalShortcutsManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::HyprlandGlobalShortcutsManagerV1;
    const INTERFACE_NAME: &str = "hyprland_global_shortcuts_manager_v1";
}

impl HyprlandGlobalShortcutsManagerV1 {
    pub fn set_handler(&self, handler: impl HyprlandGlobalShortcutsManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn HyprlandGlobalShortcutsManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for HyprlandGlobalShortcutsManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HyprlandGlobalShortcutsManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl HyprlandGlobalShortcutsManagerV1 {
    /// Since when the register_shortcut message is available.
    pub const MSG__REGISTER_SHORTCUT__SINCE: u32 = 1;

    /// register a shortcut
    ///
    /// Register a new global shortcut.
    ///
    /// A global shortcut is anonymous, meaning the app does not know what key(s) trigger it.
    ///
    /// The shortcut's keybinding shall be dealt with by the compositor.
    ///
    /// In the case of a duplicate app_id + id combination, the already_taken protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `shortcut`:
    /// - `id`: a unique id for the shortcut
    /// - `app_id`: the app_id of the application requesting the shortcut
    /// - `description`: user-readable text describing what the shortcut does.
    /// - `trigger_description`: user-readable text describing how to trigger the shortcut for the client to render.
    #[inline]
    pub fn try_send_register_shortcut(
        &self,
        shortcut: &Rc<HyprlandGlobalShortcutV1>,
        id: &str,
        app_id: &str,
        description: &str,
        trigger_description: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ) = (
            shortcut,
            id,
            app_id,
            description,
            trigger_description,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("shortcut", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: &str, arg2: &str, arg3: &str, arg4: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_global_shortcuts_manager_v1#{}.register_shortcut(shortcut: hyprland_global_shortcut_v1#{}, id: {:?}, app_id: {:?}, description: {:?}, trigger_description: {:?})\n", id, arg0, arg1, arg2, arg3, arg4);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1, arg2, arg3, arg4);
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
            arg0_id,
        ]);
        fmt.string(arg1);
        fmt.string(arg2);
        fmt.string(arg3);
        fmt.string(arg4);
        Ok(())
    }

    /// register a shortcut
    ///
    /// Register a new global shortcut.
    ///
    /// A global shortcut is anonymous, meaning the app does not know what key(s) trigger it.
    ///
    /// The shortcut's keybinding shall be dealt with by the compositor.
    ///
    /// In the case of a duplicate app_id + id combination, the already_taken protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `shortcut`:
    /// - `id`: a unique id for the shortcut
    /// - `app_id`: the app_id of the application requesting the shortcut
    /// - `description`: user-readable text describing what the shortcut does.
    /// - `trigger_description`: user-readable text describing how to trigger the shortcut for the client to render.
    #[inline]
    pub fn send_register_shortcut(
        &self,
        shortcut: &Rc<HyprlandGlobalShortcutV1>,
        id: &str,
        app_id: &str,
        description: &str,
        trigger_description: &str,
    ) {
        let res = self.try_send_register_shortcut(
            shortcut,
            id,
            app_id,
            description,
            trigger_description,
        );
        if let Err(e) = res {
            log_send("hyprland_global_shortcuts_manager_v1.register_shortcut", &e);
        }
    }

    /// register a shortcut
    ///
    /// Register a new global shortcut.
    ///
    /// A global shortcut is anonymous, meaning the app does not know what key(s) trigger it.
    ///
    /// The shortcut's keybinding shall be dealt with by the compositor.
    ///
    /// In the case of a duplicate app_id + id combination, the already_taken protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `shortcut`:
    /// - `id`: a unique id for the shortcut
    /// - `app_id`: the app_id of the application requesting the shortcut
    /// - `description`: user-readable text describing what the shortcut does.
    /// - `trigger_description`: user-readable text describing how to trigger the shortcut for the client to render.
    #[inline]
    pub fn new_try_send_register_shortcut(
        &self,
        id: &str,
        app_id: &str,
        description: &str,
        trigger_description: &str,
    ) -> Result<Rc<HyprlandGlobalShortcutV1>, ObjectError> {
        let shortcut = self.core.create_child();
        self.try_send_register_shortcut(
            &shortcut,
            id,
            app_id,
            description,
            trigger_description,
        )?;
        Ok(shortcut)
    }

    /// register a shortcut
    ///
    /// Register a new global shortcut.
    ///
    /// A global shortcut is anonymous, meaning the app does not know what key(s) trigger it.
    ///
    /// The shortcut's keybinding shall be dealt with by the compositor.
    ///
    /// In the case of a duplicate app_id + id combination, the already_taken protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `shortcut`:
    /// - `id`: a unique id for the shortcut
    /// - `app_id`: the app_id of the application requesting the shortcut
    /// - `description`: user-readable text describing what the shortcut does.
    /// - `trigger_description`: user-readable text describing how to trigger the shortcut for the client to render.
    #[inline]
    pub fn new_send_register_shortcut(
        &self,
        id: &str,
        app_id: &str,
        description: &str,
        trigger_description: &str,
    ) -> Rc<HyprlandGlobalShortcutV1> {
        let shortcut = self.core.create_child();
        self.send_register_shortcut(
            &shortcut,
            id,
            app_id,
            description,
            trigger_description,
        );
        shortcut
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the manager
    ///
    /// All objects created by the manager will still remain valid, until their
    /// appropriate destroy request has been called.
    #[inline]
    pub fn try_send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_global_shortcuts_manager_v1#{}.destroy()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
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
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the manager
    ///
    /// All objects created by the manager will still remain valid, until their
    /// appropriate destroy request has been called.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("hyprland_global_shortcuts_manager_v1.destroy", &e);
        }
    }
}

/// A message handler for [HyprlandGlobalShortcutsManagerV1] proxies.
pub trait HyprlandGlobalShortcutsManagerV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<HyprlandGlobalShortcutsManagerV1>) {
        slf.core.delete_id();
    }

    /// register a shortcut
    ///
    /// Register a new global shortcut.
    ///
    /// A global shortcut is anonymous, meaning the app does not know what key(s) trigger it.
    ///
    /// The shortcut's keybinding shall be dealt with by the compositor.
    ///
    /// In the case of a duplicate app_id + id combination, the already_taken protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `shortcut`:
    /// - `id`: a unique id for the shortcut
    /// - `app_id`: the app_id of the application requesting the shortcut
    /// - `description`: user-readable text describing what the shortcut does.
    /// - `trigger_description`: user-readable text describing how to trigger the shortcut for the client to render.
    #[inline]
    fn handle_register_shortcut(
        &mut self,
        _slf: &Rc<HyprlandGlobalShortcutsManagerV1>,
        shortcut: &Rc<HyprlandGlobalShortcutV1>,
        id: &str,
        app_id: &str,
        description: &str,
        trigger_description: &str,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_register_shortcut(
            shortcut,
            id,
            app_id,
            description,
            trigger_description,
        );
        if let Err(e) = res {
            log_forward("hyprland_global_shortcuts_manager_v1.register_shortcut", &e);
        }
    }

    /// destroy the manager
    ///
    /// All objects created by the manager will still remain valid, until their
    /// appropriate destroy request has been called.
    #[inline]
    fn handle_destroy(
        &mut self,
        _slf: &Rc<HyprlandGlobalShortcutsManagerV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("hyprland_global_shortcuts_manager_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for HyprlandGlobalShortcutsManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::HyprlandGlobalShortcutsManagerV1, version),
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
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("shortcut")));
                };
                offset += 1;
                let arg1 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError(ObjectErrorKind::MissingArgument("id")));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError(ObjectErrorKind::MissingArgument("id")));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError(ObjectErrorKind::NullString("id")));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError(ObjectErrorKind::NonUtf8("id")));
                        };
                        s
                    }
                };
                let arg2 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError(ObjectErrorKind::MissingArgument("app_id")));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError(ObjectErrorKind::MissingArgument("app_id")));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError(ObjectErrorKind::NullString("app_id")));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError(ObjectErrorKind::NonUtf8("app_id")));
                        };
                        s
                    }
                };
                let arg3 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError(ObjectErrorKind::MissingArgument("description")));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError(ObjectErrorKind::MissingArgument("description")));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError(ObjectErrorKind::NullString("description")));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError(ObjectErrorKind::NonUtf8("description")));
                        };
                        s
                    }
                };
                let arg4 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError(ObjectErrorKind::MissingArgument("trigger_description")));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError(ObjectErrorKind::MissingArgument("trigger_description")));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError(ObjectErrorKind::NullString("trigger_description")));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError(ObjectErrorKind::NonUtf8("trigger_description")));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: &str, arg2: &str, arg3: &str, arg4: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_global_shortcuts_manager_v1#{}.register_shortcut(shortcut: hyprland_global_shortcut_v1#{}, id: {:?}, app_id: {:?}, description: {:?}, trigger_description: {:?})\n", client_id, id, arg0, arg1, arg2, arg3, arg4);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4);
                }
                let arg0_id = arg0;
                let arg0 = HyprlandGlobalShortcutV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "shortcut", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_register_shortcut(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultHandler.handle_register_shortcut(&self, arg0, arg1, arg2, arg3, arg4);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_global_shortcuts_manager_v1#{}.destroy()\n", client_id, id);
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

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "register_shortcut",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for HyprlandGlobalShortcutsManagerV1 {
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

impl HyprlandGlobalShortcutsManagerV1 {
    /// Since when the error.already_taken enum variant is available.
    pub const ENM__ERROR_ALREADY_TAKEN__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HyprlandGlobalShortcutsManagerV1Error(pub u32);

impl HyprlandGlobalShortcutsManagerV1Error {
    /// the app_id + id combination has already been registered.
    pub const ALREADY_TAKEN: Self = Self(0);
}

impl Debug for HyprlandGlobalShortcutsManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ALREADY_TAKEN => "ALREADY_TAKEN",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
