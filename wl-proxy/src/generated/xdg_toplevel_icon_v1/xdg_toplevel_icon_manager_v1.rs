//! interface to manage toplevel icons
//!
//! This interface allows clients to create toplevel window icons and set
//! them on toplevel windows to be displayed to the user.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A xdg_toplevel_icon_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XdgToplevelIconManagerV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn XdgToplevelIconManagerV1Handler>,
}

struct DefaultHandler;

impl XdgToplevelIconManagerV1Handler for DefaultHandler { }

impl XdgToplevelIconManagerV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "xdg_toplevel_icon_manager_v1";
}

impl XdgToplevelIconManagerV1 {
    pub fn set_handler(&self, handler: impl XdgToplevelIconManagerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn XdgToplevelIconManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XdgToplevelIconManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XdgToplevelIconManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XdgToplevelIconManagerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the toplevel icon manager
    ///
    /// Destroy the toplevel icon manager.
    /// This does not destroy objects created with the manager.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_toplevel_icon_manager_v1#{}.destroy()\n", id);
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

    /// Since when the create_icon message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE_ICON__SINCE: u32 = 1;

    /// create a new icon instance
    ///
    /// Creates a new icon object. This icon can then be attached to a
    /// xdg_toplevel via the 'set_icon' request.
    #[inline]
    pub fn send_create_icon(
        &self,
        id: &Rc<XdgToplevelIconV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_toplevel_icon_manager_v1#{}.create_icon(id: xdg_toplevel_icon_v1#{})\n", id, arg0_id);
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

    /// Since when the set_icon message is available.
    #[allow(dead_code)]
    pub const MSG__SET_ICON__SINCE: u32 = 1;

    /// set an icon on a toplevel window
    ///
    /// This request assigns the icon 'icon' to 'toplevel', or clears the
    /// toplevel icon if 'icon' was null.
    /// This state is double-buffered and is applied on the next
    /// wl_surface.commit of the toplevel.
    ///
    /// After making this call, the xdg_toplevel_icon_v1 provided as 'icon'
    /// can be destroyed by the client without 'toplevel' losing its icon.
    /// The xdg_toplevel_icon_v1 is immutable from this point, and any
    /// future attempts to change it must raise the
    /// 'xdg_toplevel_icon_v1.immutable' protocol error.
    ///
    /// The compositor must set the toplevel icon from either the pixel data
    /// the icon provides, or by loading a stock icon using the icon name.
    /// See the description of 'xdg_toplevel_icon_v1' for details.
    ///
    /// If 'icon' is set to null, the icon of the respective toplevel is reset
    /// to its default icon (usually the icon of the application, derived from
    /// its desktop-entry file, or a placeholder icon).
    /// If this request is passed an icon with no pixel buffers or icon name
    /// assigned, the icon must be reset just like if 'icon' was null.
    ///
    /// # Arguments
    ///
    /// - `toplevel`: the toplevel to act on
    /// - `icon`:
    #[inline]
    pub fn send_set_icon(
        &self,
        toplevel: &Rc<XdgToplevel>,
        icon: Option<&Rc<XdgToplevelIconV1>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            toplevel,
            icon,
        );
        let arg0 = arg0.core();
        let arg1 = arg1.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("toplevel")),
            Some(id) => id,
        };
        let arg1_id = match arg1 {
            None => 0,
            Some(arg1) => match arg1.server_obj_id.get() {
                None => return Err(ObjectError::ArgNoServerId("icon")),
                Some(id) => id,
            },
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_toplevel_icon_manager_v1#{}.set_icon(toplevel: xdg_toplevel#{}, icon: xdg_toplevel_icon_v1#{})\n", id, arg0_id, arg1_id);
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
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// Since when the icon_size message is available.
    #[allow(dead_code)]
    pub const MSG__ICON_SIZE__SINCE: u32 = 1;

    /// describes a supported & preferred icon size
    ///
    /// This event indicates an icon size the compositor prefers to be
    /// available if the client has scalable icons and can render to any size.
    ///
    /// When the 'xdg_toplevel_icon_manager_v1' object is created, the
    /// compositor may send one or more 'icon_size' events to describe the list
    /// of preferred icon sizes. If the compositor has no size preference, it
    /// may not send any 'icon_size' event, and it is up to the client to
    /// decide a suitable icon size.
    ///
    /// A sequence of 'icon_size' events must be finished with a 'done' event.
    /// If the compositor has no size preferences, it must still send the
    /// 'done' event, without any preceding 'icon_size' events.
    ///
    /// # Arguments
    ///
    /// - `size`: the edge size of the square icon in surface-local coordinates, e.g. 64
    #[inline]
    pub fn send_icon_size(
        &self,
        size: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            size,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xdg_toplevel_icon_manager_v1#{}.icon_size(size: {})\n", client.endpoint.id, id, arg0);
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
            arg0 as u32,
        ]);
        Ok(())
    }

    /// Since when the done message is available.
    #[allow(dead_code)]
    pub const MSG__DONE__SINCE: u32 = 1;

    /// all information has been sent
    ///
    /// This event is sent after all 'icon_size' events have been sent.
    #[inline]
    pub fn send_done(
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xdg_toplevel_icon_manager_v1#{}.done()\n", client.endpoint.id, id);
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
            1,
        ]);
        Ok(())
    }
}

/// A message handler for [XdgToplevelIconManagerV1] proxies.
#[allow(dead_code)]
pub trait XdgToplevelIconManagerV1Handler: Any {
    /// destroy the toplevel icon manager
    ///
    /// Destroy the toplevel icon manager.
    /// This does not destroy objects created with the manager.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<XdgToplevelIconManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel_icon_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// create a new icon instance
    ///
    /// Creates a new icon object. This icon can then be attached to a
    /// xdg_toplevel via the 'set_icon' request.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn create_icon(
        &mut self,
        _slf: &Rc<XdgToplevelIconManagerV1>,
        id: &Rc<XdgToplevelIconV1>,
    ) {
        let res = _slf.send_create_icon(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel_icon_manager_v1.create_icon message: {}", Report::new(e));
        }
    }

    /// set an icon on a toplevel window
    ///
    /// This request assigns the icon 'icon' to 'toplevel', or clears the
    /// toplevel icon if 'icon' was null.
    /// This state is double-buffered and is applied on the next
    /// wl_surface.commit of the toplevel.
    ///
    /// After making this call, the xdg_toplevel_icon_v1 provided as 'icon'
    /// can be destroyed by the client without 'toplevel' losing its icon.
    /// The xdg_toplevel_icon_v1 is immutable from this point, and any
    /// future attempts to change it must raise the
    /// 'xdg_toplevel_icon_v1.immutable' protocol error.
    ///
    /// The compositor must set the toplevel icon from either the pixel data
    /// the icon provides, or by loading a stock icon using the icon name.
    /// See the description of 'xdg_toplevel_icon_v1' for details.
    ///
    /// If 'icon' is set to null, the icon of the respective toplevel is reset
    /// to its default icon (usually the icon of the application, derived from
    /// its desktop-entry file, or a placeholder icon).
    /// If this request is passed an icon with no pixel buffers or icon name
    /// assigned, the icon must be reset just like if 'icon' was null.
    ///
    /// # Arguments
    ///
    /// - `toplevel`: the toplevel to act on
    /// - `icon`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn set_icon(
        &mut self,
        _slf: &Rc<XdgToplevelIconManagerV1>,
        toplevel: &Rc<XdgToplevel>,
        icon: Option<&Rc<XdgToplevelIconV1>>,
    ) {
        let res = _slf.send_set_icon(
            toplevel,
            icon,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel_icon_manager_v1.set_icon message: {}", Report::new(e));
        }
    }

    /// describes a supported & preferred icon size
    ///
    /// This event indicates an icon size the compositor prefers to be
    /// available if the client has scalable icons and can render to any size.
    ///
    /// When the 'xdg_toplevel_icon_manager_v1' object is created, the
    /// compositor may send one or more 'icon_size' events to describe the list
    /// of preferred icon sizes. If the compositor has no size preference, it
    /// may not send any 'icon_size' event, and it is up to the client to
    /// decide a suitable icon size.
    ///
    /// A sequence of 'icon_size' events must be finished with a 'done' event.
    /// If the compositor has no size preferences, it must still send the
    /// 'done' event, without any preceding 'icon_size' events.
    ///
    /// # Arguments
    ///
    /// - `size`: the edge size of the square icon in surface-local coordinates, e.g. 64
    #[inline]
    fn icon_size(
        &mut self,
        _slf: &Rc<XdgToplevelIconManagerV1>,
        size: i32,
    ) {
        let res = _slf.send_icon_size(
            size,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel_icon_manager_v1.icon_size message: {}", Report::new(e));
        }
    }

    /// all information has been sent
    ///
    /// This event is sent after all 'icon_size' events have been sent.
    #[inline]
    fn done(
        &mut self,
        _slf: &Rc<XdgToplevelIconManagerV1>,
    ) {
        let res = _slf.send_done(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel_icon_manager_v1.done message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for XdgToplevelIconManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::XdgToplevelIconManagerV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_toplevel_icon_manager_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_toplevel_icon_manager_v1#{}.create_icon(id: xdg_toplevel_icon_v1#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = XdgToplevelIconV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).create_icon(&self, arg0);
                } else {
                    DefaultHandler.create_icon(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_toplevel_icon_manager_v1#{}.set_icon(toplevel: xdg_toplevel#{}, icon: xdg_toplevel_icon_v1#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<XdgToplevel>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("toplevel", o.core().interface, ProxyInterface::XdgToplevel));
                };
                let arg1 = if arg1 == 0 {
                    None
                } else {
                    let arg1_id = arg1;
                    let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                    };
                    let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<XdgToplevelIconV1>() else {
                        let o = client.endpoint.lookup(arg1_id).unwrap();
                        return Err(ObjectError::WrongObjectType("icon", o.core().interface, ProxyInterface::XdgToplevelIconV1));
                    };
                    Some(arg1)
                };
                let arg0 = &arg0;
                let arg1 = arg1.as_ref();
                if let Some(handler) = handler {
                    (**handler).set_icon(&self, arg0, arg1);
                } else {
                    DefaultHandler.set_icon(&self, arg0, arg1);
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
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = arg0 as i32;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xdg_toplevel_icon_manager_v1#{}.icon_size(size: {})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).icon_size(&self, arg0);
                } else {
                    DefaultHandler.icon_size(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xdg_toplevel_icon_manager_v1#{}.done()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).done(&self);
                } else {
                    DefaultHandler.done(&self);
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
            1 => "create_icon",
            2 => "set_icon",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "icon_size",
            1 => "done",
            _ => return None,
        };
        Some(name)
    }
}

impl Proxy for XdgToplevelIconManagerV1 {
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

