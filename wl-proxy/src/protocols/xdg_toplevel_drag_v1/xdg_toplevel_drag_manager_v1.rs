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

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A xdg_toplevel_drag_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XdgToplevelDragManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn XdgToplevelDragManagerV1Handler>,
}

struct DefaultHandler;

impl XdgToplevelDragManagerV1Handler for DefaultHandler { }

impl ConcreteObject for XdgToplevelDragManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::XdgToplevelDragManagerV1;
    const INTERFACE_NAME: &str = "xdg_toplevel_drag_manager_v1";
}

impl XdgToplevelDragManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl XdgToplevelDragManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn XdgToplevelDragManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XdgToplevelDragManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XdgToplevelDragManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XdgToplevelDragManagerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the xdg_toplevel_drag_manager_v1 object
    ///
    /// Destroy this xdg_toplevel_drag_manager_v1 object. Other objects,
    /// including xdg_toplevel_drag_v1 objects created by this factory, are not
    /// affected by this request.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_toplevel_drag_manager_v1#{}.destroy()\n", id);
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

    /// destroy the xdg_toplevel_drag_manager_v1 object
    ///
    /// Destroy this xdg_toplevel_drag_manager_v1 object. Other objects,
    /// including xdg_toplevel_drag_v1 objects created by this factory, are not
    /// affected by this request.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("xdg_toplevel_drag_manager_v1.destroy", &e);
        }
    }

    /// Since when the get_xdg_toplevel_drag message is available.
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
    pub fn try_send_get_xdg_toplevel_drag(
        &self,
        id: &Rc<XdgToplevelDragV1>,
        data_source: &Rc<WlDataSource>,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("data_source"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_toplevel_drag_manager_v1#{}.get_xdg_toplevel_drag(id: xdg_toplevel_drag_v1#{}, data_source: wl_data_source#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id);
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
        ]);
        Ok(())
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
    #[inline]
    pub fn send_get_xdg_toplevel_drag(
        &self,
        id: &Rc<XdgToplevelDragV1>,
        data_source: &Rc<WlDataSource>,
    ) {
        let res = self.try_send_get_xdg_toplevel_drag(
            id,
            data_source,
        );
        if let Err(e) = res {
            log_send("xdg_toplevel_drag_manager_v1.get_xdg_toplevel_drag", &e);
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
    /// - `data_source`:
    #[inline]
    pub fn new_try_send_get_xdg_toplevel_drag(
        &self,
        data_source: &Rc<WlDataSource>,
    ) -> Result<Rc<XdgToplevelDragV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_xdg_toplevel_drag(
            &id,
            data_source,
        )?;
        Ok(id)
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
    /// - `data_source`:
    #[inline]
    pub fn new_send_get_xdg_toplevel_drag(
        &self,
        data_source: &Rc<WlDataSource>,
    ) -> Rc<XdgToplevelDragV1> {
        let id = self.core.create_child();
        self.send_get_xdg_toplevel_drag(
            &id,
            data_source,
        );
        id
    }
}

/// A message handler for [`XdgToplevelDragManagerV1`] proxies.
pub trait XdgToplevelDragManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<XdgToplevelDragManagerV1>) {
        slf.core.delete_id();
    }

    /// destroy the xdg_toplevel_drag_manager_v1 object
    ///
    /// Destroy this xdg_toplevel_drag_manager_v1 object. Other objects,
    /// including xdg_toplevel_drag_v1 objects created by this factory, are not
    /// affected by this request.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<XdgToplevelDragManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("xdg_toplevel_drag_manager_v1.destroy", &e);
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
    fn handle_get_xdg_toplevel_drag(
        &mut self,
        slf: &Rc<XdgToplevelDragManagerV1>,
        id: &Rc<XdgToplevelDragV1>,
        data_source: &Rc<WlDataSource>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_xdg_toplevel_drag(
            id,
            data_source,
        );
        if let Err(e) = res {
            log_forward("xdg_toplevel_drag_manager_v1.get_xdg_toplevel_drag", &e);
        }
    }
}

impl ObjectPrivate for XdgToplevelDragManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::XdgToplevelDragManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_toplevel_drag_manager_v1#{}.destroy()\n", client_id, id);
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_toplevel_drag_manager_v1#{}.get_xdg_toplevel_drag(id: xdg_toplevel_drag_v1#{}, data_source: wl_data_source#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = XdgToplevelDragV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlDataSource>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("data_source", o.core().interface, ObjectInterface::WlDataSource)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_xdg_toplevel_drag(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_xdg_toplevel_drag(&self, arg0, arg1);
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

impl Object for XdgToplevelDragManagerV1 {
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

impl XdgToplevelDragManagerV1 {
    /// Since when the error.invalid_source enum variant is available.
    pub const ENM__ERROR_INVALID_SOURCE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct XdgToplevelDragManagerV1Error(pub u32);

impl XdgToplevelDragManagerV1Error {
    /// data_source already used for toplevel drag
    pub const INVALID_SOURCE: Self = Self(0);
}

impl Debug for XdgToplevelDragManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_SOURCE => "INVALID_SOURCE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
