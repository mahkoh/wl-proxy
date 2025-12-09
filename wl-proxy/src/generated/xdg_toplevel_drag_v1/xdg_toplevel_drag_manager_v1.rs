//! Move a window during a drag
//!
//! This protocol enhances normal drag and drop with the ability to move a
//! window at the same time. This allows having detachable parts of a window
//! that when dragged out of it become a new window and can be dragged over
//! an existing window to be reattached.
//!
//! A typical workflow would be when the user starts dragging on top of a
//! detachable part of a window, the client would create a wl_data_source and
//! a xdg_toplevel_drag_v1 object and start the drag as normal via
//! wl_data_device.start_drag. Once the client determines that the detachable
//! window contents should be detached from the originating window, it creates
//! a new xdg_toplevel with these contents and issues a
//! xdg_toplevel_drag_v1.attach request before mapping it. From now on the new
//! window is moved by the compositor during the drag as if the client called
//! xdg_toplevel.move.
//!
//! Dragging an existing window is similar. The client creates a
//! xdg_toplevel_drag_v1 object and attaches the existing toplevel before
//! starting the drag.
//!
//! Clients use the existing drag and drop mechanism to detect when a window
//! can be docked or undocked. If the client wants to snap a window into a
//! parent window it should delete or unmap the dragged top-level. If the
//! contents should be detached again it attaches a new toplevel as described
//! above. If a drag operation is cancelled without being dropped, clients
//! should revert to the previous state, deleting any newly created windows
//! as appropriate. When a drag operation ends as indicated by
//! wl_data_source.dnd_drop_performed the dragged toplevel window's final
//! position is determined as if a xdg_toplevel_move operation ended.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A xdg_toplevel_drag_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaXdgToplevelDragManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaXdgToplevelDragManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaXdgToplevelDragManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaXdgToplevelDragManagerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaXdgToplevelDragManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::XdgToplevelDragManagerV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaXdgToplevelDragManagerV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaXdgToplevelDragManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaXdgToplevelDragManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaXdgToplevelDragManagerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the xdg_toplevel_drag_manager_v1 object
    ///
    /// Destroy this xdg_toplevel_drag_manager_v1 object. Other objects,
    /// including xdg_toplevel_drag_v1 objects created by this factory, are not
    /// affected by this request.
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

    /// Since when the get_xdg_toplevel_drag message is available.
    #[allow(dead_code)]
    pub const MSG__GET_XDG_TOPLEVEL_DRAG__SINCE: u32 = 1;

    /// get an xdg_toplevel_drag for a wl_data_source
    ///
    /// Create an xdg_toplevel_drag for a drag and drop operation that is going
    /// to be started with data_source.
    ///
    /// This request can only be made on sources used in drag-and-drop, so it
    /// must be performed before wl_data_device.start_drag. Attempting to use
    /// the source other than for drag-and-drop such as in
    /// wl_data_device.set_selection will raise an invalid_source error.
    ///
    /// Destroying data_source while a toplevel is attached to the
    /// xdg_toplevel_drag is undefined.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `data_source`:
    #[inline]
    pub fn send_get_xdg_toplevel_drag(
        &self,
        id: &Rc<MetaXdgToplevelDragV1>,
        data_source: &Rc<MetaWlDataSource>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            data_source,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("data_source")),
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
        ]);
        Ok(())
    }
}

/// A message handler for [XdgToplevelDragManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaXdgToplevelDragManagerV1MessageHandler {
    /// destroy the xdg_toplevel_drag_manager_v1 object
    ///
    /// Destroy this xdg_toplevel_drag_manager_v1 object. Other objects,
    /// including xdg_toplevel_drag_v1 objects created by this factory, are not
    /// affected by this request.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaXdgToplevelDragManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel_drag_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// get an xdg_toplevel_drag for a wl_data_source
    ///
    /// Create an xdg_toplevel_drag for a drag and drop operation that is going
    /// to be started with data_source.
    ///
    /// This request can only be made on sources used in drag-and-drop, so it
    /// must be performed before wl_data_device.start_drag. Attempting to use
    /// the source other than for drag-and-drop such as in
    /// wl_data_device.set_selection will raise an invalid_source error.
    ///
    /// Destroying data_source while a toplevel is attached to the
    /// xdg_toplevel_drag is undefined.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `data_source`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_xdg_toplevel_drag(
        &mut self,
        _slf: &Rc<MetaXdgToplevelDragManagerV1>,
        id: &Rc<MetaXdgToplevelDragV1>,
        data_source: &Rc<MetaWlDataSource>,
    ) {
        let res = _slf.send_get_xdg_toplevel_drag(
            id,
            data_source,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel_drag_manager_v1.get_xdg_toplevel_drag message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaXdgToplevelDragManagerV1 {
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
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg0_id = arg0;
                let arg0 = MetaXdgToplevelDragV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlDataSource>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("data_source", o.core().interface, ProxyInterface::WlDataSource));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_xdg_toplevel_drag(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_xdg_toplevel_drag(&self, arg0, arg1);
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
            1 => "get_xdg_toplevel_drag",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl MetaXdgToplevelDragManagerV1 {
    /// Since when the error.invalid_source enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_SOURCE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaXdgToplevelDragManagerV1Error(pub u32);

impl MetaXdgToplevelDragManagerV1Error {
    /// data_source already used for toplevel drag
    #[allow(dead_code)]
    pub const INVALID_SOURCE: Self = Self(0);
}

impl Debug for MetaXdgToplevelDragManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_SOURCE => "INVALID_SOURCE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
