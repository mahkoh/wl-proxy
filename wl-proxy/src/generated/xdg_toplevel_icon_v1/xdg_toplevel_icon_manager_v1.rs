//! interface to manage toplevel icons
//!
//! This interface allows clients to create toplevel window icons and set
//! them on toplevel windows to be displayed to the user.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A xdg_toplevel_icon_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaXdgToplevelIconManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaXdgToplevelIconManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaXdgToplevelIconManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaXdgToplevelIconManagerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaXdgToplevelIconManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::XdgToplevelIconManagerV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaXdgToplevelIconManagerV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaXdgToplevelIconManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaXdgToplevelIconManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaXdgToplevelIconManagerV1 {
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
        id: &Rc<MetaXdgToplevelIconV1>,
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
        toplevel: &Rc<MetaXdgToplevel>,
        icon: Option<&Rc<MetaXdgToplevelIconV1>>,
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
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

/// A message handler for [XdgToplevelIconManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaXdgToplevelIconManagerV1MessageHandler {
    /// destroy the toplevel icon manager
    ///
    /// Destroy the toplevel icon manager.
    /// This does not destroy objects created with the manager.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaXdgToplevelIconManagerV1>,
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
        _slf: &Rc<MetaXdgToplevelIconManagerV1>,
        id: &Rc<MetaXdgToplevelIconV1>,
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
        _slf: &Rc<MetaXdgToplevelIconManagerV1>,
        toplevel: &Rc<MetaXdgToplevel>,
        icon: Option<&Rc<MetaXdgToplevelIconV1>>,
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
        _slf: &Rc<MetaXdgToplevelIconManagerV1>,
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
        _slf: &Rc<MetaXdgToplevelIconManagerV1>,
    ) {
        let res = _slf.send_done(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel_icon_manager_v1.done message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaXdgToplevelIconManagerV1 {
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
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0_id = arg0;
                let arg0 = MetaXdgToplevelIconV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).create_icon(&self, arg0);
                } else {
                    DefaultMessageHandler.create_icon(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaXdgToplevel>() else {
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
                    let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaXdgToplevelIconV1>() else {
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
                    DefaultMessageHandler.set_icon(&self, arg0, arg1);
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
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = arg0 as i32;
                if let Some(handler) = handler {
                    (**handler).icon_size(&self, arg0);
                } else {
                    DefaultMessageHandler.icon_size(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).done(&self);
                } else {
                    DefaultMessageHandler.done(&self);
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

