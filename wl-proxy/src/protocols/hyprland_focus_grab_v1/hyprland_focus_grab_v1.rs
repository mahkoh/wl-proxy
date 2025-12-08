//! input focus limiter
//!
//! This interface restricts input focus to a specified whitelist of
//! surfaces as long as the focus grab object exists and has at least
//! one comitted surface.
//!
//! Mouse and touch events inside a whitelisted surface will be passed
//! to the surface normally, while events outside of a whitelisted surface
//! will clear the grab object. Keyboard events will be passed to the client
//! and a compositor-picked surface in the whitelist will receive a
//! wl_keyboard::enter event if a whitelisted surface is not already entered.
//!
//! Upon meeting implementation-defined criteria usually meaning a mouse or
//! touch input outside of any whitelisted surfaces, the compositor will
//! clear the whitelist, rendering the grab inert and sending the cleared
//! event. The same will happen if another focus grab or similar action
//! is started at the compositor's discretion.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A hyprland_focus_grab_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct HyprlandFocusGrabV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn HyprlandFocusGrabV1Handler>,
}

struct DefaultHandler;

impl HyprlandFocusGrabV1Handler for DefaultHandler { }

impl ConcreteObject for HyprlandFocusGrabV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::HyprlandFocusGrabV1;
    const INTERFACE_NAME: &str = "hyprland_focus_grab_v1";
}

impl HyprlandFocusGrabV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl HyprlandFocusGrabV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn HyprlandFocusGrabV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for HyprlandFocusGrabV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HyprlandFocusGrabV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl HyprlandFocusGrabV1 {
    /// Since when the add_surface message is available.
    pub const MSG__ADD_SURFACE__SINCE: u32 = 1;

    /// add a surface to the focus whitelist
    ///
    /// Add a surface to the whitelist. Destroying the surface is treated the
    /// same as an explicit call to remove_surface and duplicate additions are
    /// ignored.
    ///
    /// Does not take effect until commit is called.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn try_send_add_surface(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            surface,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_focus_grab_v1#{}.add_surface(surface: wl_surface#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
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
        ]);
        Ok(())
    }

    /// add a surface to the focus whitelist
    ///
    /// Add a surface to the whitelist. Destroying the surface is treated the
    /// same as an explicit call to remove_surface and duplicate additions are
    /// ignored.
    ///
    /// Does not take effect until commit is called.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn send_add_surface(
        &self,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_add_surface(
            surface,
        );
        if let Err(e) = res {
            log_send("hyprland_focus_grab_v1.add_surface", &e);
        }
    }

    /// Since when the remove_surface message is available.
    pub const MSG__REMOVE_SURFACE__SINCE: u32 = 1;

    /// remove a surface from the focus whitelist
    ///
    /// Remove a surface from the whitelist. Destroying the surface is treated
    /// the same as an explicit call to this function.
    ///
    /// If the grab was active and the removed surface was entered by the
    /// keyboard, another surface will be entered on commit.
    ///
    /// Does not take effect until commit is called.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn try_send_remove_surface(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            surface,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_focus_grab_v1#{}.remove_surface(surface: wl_surface#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
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
        ]);
        Ok(())
    }

    /// remove a surface from the focus whitelist
    ///
    /// Remove a surface from the whitelist. Destroying the surface is treated
    /// the same as an explicit call to this function.
    ///
    /// If the grab was active and the removed surface was entered by the
    /// keyboard, another surface will be entered on commit.
    ///
    /// Does not take effect until commit is called.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn send_remove_surface(
        &self,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_remove_surface(
            surface,
        );
        if let Err(e) = res {
            log_send("hyprland_focus_grab_v1.remove_surface", &e);
        }
    }

    /// Since when the commit message is available.
    pub const MSG__COMMIT__SINCE: u32 = 1;

    /// commit the focus whitelist
    ///
    /// Commit pending changes to the surface whitelist.
    ///
    /// If the list previously had no entries and now has at least one, the grab
    /// will start. If it previously had entries and now has none, the grab will
    /// become inert.
    #[inline]
    pub fn try_send_commit(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_focus_grab_v1#{}.commit()\n", id);
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
            2,
        ]);
        Ok(())
    }

    /// commit the focus whitelist
    ///
    /// Commit pending changes to the surface whitelist.
    ///
    /// If the list previously had no entries and now has at least one, the grab
    /// will start. If it previously had entries and now has none, the grab will
    /// become inert.
    #[inline]
    pub fn send_commit(
        &self,
    ) {
        let res = self.try_send_commit(
        );
        if let Err(e) = res {
            log_send("hyprland_focus_grab_v1.commit", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the focus grab
    ///
    /// Destroy the grab object and remove the grab if active.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_focus_grab_v1#{}.destroy()\n", id);
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
            3,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the focus grab
    ///
    /// Destroy the grab object and remove the grab if active.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("hyprland_focus_grab_v1.destroy", &e);
        }
    }

    /// Since when the cleared message is available.
    pub const MSG__CLEARED__SINCE: u32 = 1;

    /// the focus grab was cleared
    ///
    /// Sent when an active grab is cancelled by the compositor,
    /// regardless of cause.
    #[inline]
    pub fn try_send_cleared(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= hyprland_focus_grab_v1#{}.cleared()\n", client_id, id);
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

    /// the focus grab was cleared
    ///
    /// Sent when an active grab is cancelled by the compositor,
    /// regardless of cause.
    #[inline]
    pub fn send_cleared(
        &self,
    ) {
        let res = self.try_send_cleared(
        );
        if let Err(e) = res {
            log_send("hyprland_focus_grab_v1.cleared", &e);
        }
    }
}

/// A message handler for [`HyprlandFocusGrabV1`] proxies.
pub trait HyprlandFocusGrabV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<HyprlandFocusGrabV1>) {
        slf.core.delete_id();
    }

    /// add a surface to the focus whitelist
    ///
    /// Add a surface to the whitelist. Destroying the surface is treated the
    /// same as an explicit call to remove_surface and duplicate additions are
    /// ignored.
    ///
    /// Does not take effect until commit is called.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_add_surface(
        &mut self,
        slf: &Rc<HyprlandFocusGrabV1>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_add_surface(
            surface,
        );
        if let Err(e) = res {
            log_forward("hyprland_focus_grab_v1.add_surface", &e);
        }
    }

    /// remove a surface from the focus whitelist
    ///
    /// Remove a surface from the whitelist. Destroying the surface is treated
    /// the same as an explicit call to this function.
    ///
    /// If the grab was active and the removed surface was entered by the
    /// keyboard, another surface will be entered on commit.
    ///
    /// Does not take effect until commit is called.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_remove_surface(
        &mut self,
        slf: &Rc<HyprlandFocusGrabV1>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_remove_surface(
            surface,
        );
        if let Err(e) = res {
            log_forward("hyprland_focus_grab_v1.remove_surface", &e);
        }
    }

    /// commit the focus whitelist
    ///
    /// Commit pending changes to the surface whitelist.
    ///
    /// If the list previously had no entries and now has at least one, the grab
    /// will start. If it previously had entries and now has none, the grab will
    /// become inert.
    #[inline]
    fn handle_commit(
        &mut self,
        slf: &Rc<HyprlandFocusGrabV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_commit(
        );
        if let Err(e) = res {
            log_forward("hyprland_focus_grab_v1.commit", &e);
        }
    }

    /// destroy the focus grab
    ///
    /// Destroy the grab object and remove the grab if active.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<HyprlandFocusGrabV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("hyprland_focus_grab_v1.destroy", &e);
        }
    }

    /// the focus grab was cleared
    ///
    /// Sent when an active grab is cancelled by the compositor,
    /// regardless of cause.
    #[inline]
    fn handle_cleared(
        &mut self,
        slf: &Rc<HyprlandFocusGrabV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_cleared(
        );
        if let Err(e) = res {
            log_forward("hyprland_focus_grab_v1.cleared", &e);
        }
    }
}

impl ObjectPrivate for HyprlandFocusGrabV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::HyprlandFocusGrabV1, version),
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_focus_grab_v1#{}.add_surface(surface: wl_surface#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_add_surface(&self, arg0);
                } else {
                    DefaultHandler.handle_add_surface(&self, arg0);
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
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_focus_grab_v1#{}.remove_surface(surface: wl_surface#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_remove_surface(&self, arg0);
                } else {
                    DefaultHandler.handle_remove_surface(&self, arg0);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_focus_grab_v1#{}.commit()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_commit(&self);
                } else {
                    DefaultHandler.handle_commit(&self);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_focus_grab_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> hyprland_focus_grab_v1#{}.cleared()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_cleared(&self);
                } else {
                    DefaultHandler.handle_cleared(&self);
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
            0 => "add_surface",
            1 => "remove_surface",
            2 => "commit",
            3 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "cleared",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for HyprlandFocusGrabV1 {
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

