//! Object representing a toplevel move during a drag

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A xdg_toplevel_drag_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XdgToplevelDragV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn XdgToplevelDragV1Handler>,
}

struct DefaultHandler;

impl XdgToplevelDragV1Handler for DefaultHandler { }

impl XdgToplevelDragV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::XdgToplevelDragV1;
    pub const INTERFACE_NAME: &str = "xdg_toplevel_drag_v1";
}

impl XdgToplevelDragV1 {
    pub fn set_handler(&self, handler: impl XdgToplevelDragV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn XdgToplevelDragV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XdgToplevelDragV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XdgToplevelDragV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XdgToplevelDragV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy an xdg_toplevel_drag_v1 object
    ///
    /// Destroy this xdg_toplevel_drag_v1 object. This request must only be
    /// called after the underlying wl_data_source drag has ended, as indicated
    /// by the dnd_drop_performed or cancelled events. In any other case an
    /// ongoing_drag error is raised.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_toplevel_drag_v1#{}.destroy()\n", id);
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

    /// Since when the attach message is available.
    pub const MSG__ATTACH__SINCE: u32 = 1;

    /// Move a toplevel with the drag operation
    ///
    /// Request that the window will be moved with the cursor during the drag
    /// operation. The offset is a hint to the compositor how the toplevel
    /// should be positioned relative to the cursor hotspot in surface local
    /// coordinates and relative to the geometry of the toplevel being attached.
    /// See xdg_surface.set_window_geometry. For example it might only
    /// be used when an unmapped window is attached. The attached window
    /// does not participate in the selection of the drag target.
    ///
    /// If the toplevel is unmapped while it is attached, it is automatically
    /// detached from the drag. In this case this request has to be called again
    /// if the window should be attached after it is remapped.
    ///
    /// This request can be called multiple times but issuing it while a
    /// toplevel with an active role is attached raises a toplevel_attached
    /// error.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    /// - `x_offset`: dragged surface x offset
    /// - `y_offset`: dragged surface y offset
    #[inline]
    pub fn send_attach(
        &self,
        toplevel: &Rc<XdgToplevel>,
        x_offset: i32,
        y_offset: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            toplevel,
            x_offset,
            y_offset,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("toplevel")),
            Some(id) => id,
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_toplevel_drag_v1#{}.attach(toplevel: xdg_toplevel#{}, x_offset: {}, y_offset: {})\n", id, arg0_id, arg1, arg2);
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
            arg1 as u32,
            arg2 as u32,
        ]);
        Ok(())
    }
}

/// A message handler for [XdgToplevelDragV1] proxies.
pub trait XdgToplevelDragV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<XdgToplevelDragV1>) {
        let _ = slf.core.delete_id();
    }

    /// destroy an xdg_toplevel_drag_v1 object
    ///
    /// Destroy this xdg_toplevel_drag_v1 object. This request must only be
    /// called after the underlying wl_data_source drag has ended, as indicated
    /// by the dnd_drop_performed or cancelled events. In any other case an
    /// ongoing_drag error is raised.
    #[inline]
    fn handle_destroy(
        &mut self,
        _slf: &Rc<XdgToplevelDragV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel_drag_v1.destroy message: {}", Report::new(e));
        }
    }

    /// Move a toplevel with the drag operation
    ///
    /// Request that the window will be moved with the cursor during the drag
    /// operation. The offset is a hint to the compositor how the toplevel
    /// should be positioned relative to the cursor hotspot in surface local
    /// coordinates and relative to the geometry of the toplevel being attached.
    /// See xdg_surface.set_window_geometry. For example it might only
    /// be used when an unmapped window is attached. The attached window
    /// does not participate in the selection of the drag target.
    ///
    /// If the toplevel is unmapped while it is attached, it is automatically
    /// detached from the drag. In this case this request has to be called again
    /// if the window should be attached after it is remapped.
    ///
    /// This request can be called multiple times but issuing it while a
    /// toplevel with an active role is attached raises a toplevel_attached
    /// error.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    /// - `x_offset`: dragged surface x offset
    /// - `y_offset`: dragged surface y offset
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_attach(
        &mut self,
        _slf: &Rc<XdgToplevelDragV1>,
        toplevel: &Rc<XdgToplevel>,
        x_offset: i32,
        y_offset: i32,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.send_attach(
            toplevel,
            x_offset,
            y_offset,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel_drag_v1.attach message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for XdgToplevelDragV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::XdgToplevelDragV1, version),
            handler: Default::default(),
        })
    }

    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err((ObjectError::HandlerBorrowed, self));
        };
        if let Some(handler) = &mut *handler {
            handler.delete_id(&self);
        } else {
            let _ = self.core.delete_id();
        }
        Ok(())
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_toplevel_drag_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
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
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_toplevel_drag_v1#{}.attach(toplevel: xdg_toplevel#{}, x_offset: {}, y_offset: {})\n", client.endpoint.id, msg[0], arg0, arg1, arg2);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<XdgToplevel>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("toplevel", o.core().interface, ObjectInterface::XdgToplevel));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_attach(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_attach(&self, arg0, arg1, arg2);
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
        let Some(mut handler) = self.handler.try_borrow_mut() else {
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
            1 => "attach",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for XdgToplevelDragV1 {
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

impl XdgToplevelDragV1 {
    /// Since when the error.toplevel_attached enum variant is available.
    pub const ENM__ERROR_TOPLEVEL_ATTACHED__SINCE: u32 = 1;
    /// Since when the error.ongoing_drag enum variant is available.
    pub const ENM__ERROR_ONGOING_DRAG__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct XdgToplevelDragV1Error(pub u32);

impl XdgToplevelDragV1Error {
    /// valid toplevel already attached
    pub const TOPLEVEL_ATTACHED: Self = Self(0);

    /// drag has not ended
    pub const ONGOING_DRAG: Self = Self(1);
}

impl Debug for XdgToplevelDragV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::TOPLEVEL_ATTACHED => "TOPLEVEL_ATTACHED",
            Self::ONGOING_DRAG => "ONGOING_DRAG",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
