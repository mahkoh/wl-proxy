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

/// A hyprland_focus_grab_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct HyprlandFocusGrabV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn HyprlandFocusGrabV1Handler>,
}

struct DefaultHandler;

impl HyprlandFocusGrabV1Handler for DefaultHandler { }

impl HyprlandFocusGrabV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "hyprland_focus_grab_v1";
}

impl HyprlandFocusGrabV1 {
    pub fn set_handler(&self, handler: impl HyprlandFocusGrabV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

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
    pub fn send_add_surface(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("surface")),
            Some(id) => id,
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_focus_grab_v1#{}.add_surface(surface: wl_surface#{})\n", id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
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
    pub fn send_remove_surface(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("surface")),
            Some(id) => id,
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_focus_grab_v1#{}.remove_surface(surface: wl_surface#{})\n", id, arg0_id);
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
        ]);
        Ok(())
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
    pub fn send_commit(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_focus_grab_v1#{}.commit()\n", id);
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
            2,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the focus grab
    ///
    /// Destroy the grab object and remove the grab if active.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_focus_grab_v1#{}.destroy()\n", id);
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
            3,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the cleared message is available.
    pub const MSG__CLEARED__SINCE: u32 = 1;

    /// the focus grab was cleared
    ///
    /// Sent when an active grab is cancelled by the compositor,
    /// regardless of cause.
    #[inline]
    pub fn send_cleared(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= hyprland_focus_grab_v1#{}.cleared()\n", client.endpoint.id, id);
            self.core.state.log(args);
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
}

/// A message handler for [HyprlandFocusGrabV1] proxies.
pub trait HyprlandFocusGrabV1Handler: Any {
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
    fn add_surface(
        &mut self,
        _slf: &Rc<HyprlandFocusGrabV1>,
        surface: &Rc<WlSurface>,
    ) {
        let res = _slf.send_add_surface(
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a hyprland_focus_grab_v1.add_surface message: {}", Report::new(e));
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
    fn remove_surface(
        &mut self,
        _slf: &Rc<HyprlandFocusGrabV1>,
        surface: &Rc<WlSurface>,
    ) {
        let res = _slf.send_remove_surface(
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a hyprland_focus_grab_v1.remove_surface message: {}", Report::new(e));
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
    fn commit(
        &mut self,
        _slf: &Rc<HyprlandFocusGrabV1>,
    ) {
        let res = _slf.send_commit(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a hyprland_focus_grab_v1.commit message: {}", Report::new(e));
        }
    }

    /// destroy the focus grab
    ///
    /// Destroy the grab object and remove the grab if active.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<HyprlandFocusGrabV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a hyprland_focus_grab_v1.destroy message: {}", Report::new(e));
        }
    }

    /// the focus grab was cleared
    ///
    /// Sent when an active grab is cancelled by the compositor,
    /// regardless of cause.
    #[inline]
    fn cleared(
        &mut self,
        _slf: &Rc<HyprlandFocusGrabV1>,
    ) {
        let res = _slf.send_cleared(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a hyprland_focus_grab_v1.cleared message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for HyprlandFocusGrabV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::HyprlandFocusGrabV1, version),
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_focus_grab_v1#{}.add_surface(surface: wl_surface#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ProxyInterface::WlSurface));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).add_surface(&self, arg0);
                } else {
                    DefaultHandler.add_surface(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_focus_grab_v1#{}.remove_surface(surface: wl_surface#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ProxyInterface::WlSurface));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).remove_surface(&self, arg0);
                } else {
                    DefaultHandler.remove_surface(&self, arg0);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_focus_grab_v1#{}.commit()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).commit(&self);
                } else {
                    DefaultHandler.commit(&self);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_focus_grab_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> hyprland_focus_grab_v1#{}.cleared()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).cleared(&self);
                } else {
                    DefaultHandler.cleared(&self);
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

impl Proxy for HyprlandFocusGrabV1 {
    fn core(&self) -> &ProxyCore {
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

