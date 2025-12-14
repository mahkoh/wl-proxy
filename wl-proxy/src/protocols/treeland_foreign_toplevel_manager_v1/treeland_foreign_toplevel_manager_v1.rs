//! foreign toplevel manager
//!
//! This interface allows a client to get toplevel some info.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A treeland_foreign_toplevel_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandForeignToplevelManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn TreelandForeignToplevelManagerV1Handler>,
}

struct DefaultHandler;

impl TreelandForeignToplevelManagerV1Handler for DefaultHandler { }

impl TreelandForeignToplevelManagerV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::TreelandForeignToplevelManagerV1;
    pub const INTERFACE_NAME: &str = "treeland_foreign_toplevel_manager_v1";
}

impl TreelandForeignToplevelManagerV1 {
    pub fn set_handler(&self, handler: impl TreelandForeignToplevelManagerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandForeignToplevelManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandForeignToplevelManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandForeignToplevelManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandForeignToplevelManagerV1 {
    /// Since when the toplevel message is available.
    pub const MSG__TOPLEVEL__SINCE: u32 = 1;

    /// a toplevel has been created
    ///
    /// This event is emitted whenever a new toplevel window is created. It
    /// is emitted for all toplevels, regardless of the app that has created
    /// them.
    ///
    /// All initial details of the toplevel(title, app_id, states, etc.) will
    /// be sent immediately after this event via the corresponding events in
    /// treeland_foreign_toplevel_handle_v1.
    #[inline]
    pub fn send_toplevel(
        &self,
        toplevel: &Rc<TreelandForeignToplevelHandleV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            toplevel,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateClientId("toplevel", e))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_foreign_toplevel_manager_v1#{}.toplevel(toplevel: treeland_foreign_toplevel_handle_v1#{})\n", client.endpoint.id, id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the stop message is available.
    pub const MSG__STOP__SINCE: u32 = 1;

    /// Stop sending events
    ///
    /// Indicates the client no longer wishes to receive events for new toplevels.
    /// However the compositor may emit further toplevel_created events, until
    /// the finished event is emitted.
    ///
    /// The client must not send any more requests after this one.
    #[inline]
    pub fn send_stop(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_foreign_toplevel_manager_v1#{}.stop()\n", id);
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
        Ok(())
    }

    /// Since when the finished message is available.
    pub const MSG__FINISHED__SINCE: u32 = 1;

    /// the compositor has finished with the toplevel manager
    ///
    /// This event indicates that the compositor is done sending events to the
    /// treeland_foreign_toplevel_manager_v1. The server will destroy the object
    /// immediately after sending this request, so it will become invalid and
    /// the client should free any resources associated with it.
    #[inline]
    pub fn send_finished(
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_foreign_toplevel_manager_v1#{}.finished()\n", client.endpoint.id, id);
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
        drop(fmt);
        drop(outgoing_ref);
        drop(client_ref);
        self.core.handle_client_destroy();
        Ok(())
    }

    /// Since when the get_dock_preview_context message is available.
    pub const MSG__GET_DOCK_PREVIEW_CONTEXT__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `relative_surface`:
    /// - `id`:
    #[inline]
    pub fn send_get_dock_preview_context(
        &self,
        relative_surface: &Rc<WlSurface>,
        id: &Rc<TreelandDockPreviewContextV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            relative_surface,
            id,
        );
        let arg0 = arg0.core();
        let arg1_obj = arg1;
        let arg1 = arg1_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("relative_surface")),
            Some(id) => id,
        };
        arg1.generate_server_id(arg1_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg1_id = arg1.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_foreign_toplevel_manager_v1#{}.get_dock_preview_context(relative_surface: wl_surface#{}, id: treeland_dock_preview_context_v1#{})\n", id, arg0_id, arg1_id);
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
        ]);
        Ok(())
    }
}

/// A message handler for [TreelandForeignToplevelManagerV1] proxies.
pub trait TreelandForeignToplevelManagerV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<TreelandForeignToplevelManagerV1>) {
        let _ = slf.core.delete_id();
    }

    /// a toplevel has been created
    ///
    /// This event is emitted whenever a new toplevel window is created. It
    /// is emitted for all toplevels, regardless of the app that has created
    /// them.
    ///
    /// All initial details of the toplevel(title, app_id, states, etc.) will
    /// be sent immediately after this event via the corresponding events in
    /// treeland_foreign_toplevel_handle_v1.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    #[inline]
    fn handle_toplevel(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelManagerV1>,
        toplevel: &Rc<TreelandForeignToplevelHandleV1>,
    ) {
        if !_slf.core.forward_to_client.get() {
            return;
        }
        let res = _slf.send_toplevel(
            toplevel,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_manager_v1.toplevel message: {}", Report::new(e));
        }
    }

    /// Stop sending events
    ///
    /// Indicates the client no longer wishes to receive events for new toplevels.
    /// However the compositor may emit further toplevel_created events, until
    /// the finished event is emitted.
    ///
    /// The client must not send any more requests after this one.
    #[inline]
    fn handle_stop(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelManagerV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.send_stop(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_manager_v1.stop message: {}", Report::new(e));
        }
    }

    /// the compositor has finished with the toplevel manager
    ///
    /// This event indicates that the compositor is done sending events to the
    /// treeland_foreign_toplevel_manager_v1. The server will destroy the object
    /// immediately after sending this request, so it will become invalid and
    /// the client should free any resources associated with it.
    #[inline]
    fn handle_finished(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelManagerV1>,
    ) {
        if !_slf.core.forward_to_client.get() {
            return;
        }
        let res = _slf.send_finished(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_manager_v1.finished message: {}", Report::new(e));
        }
    }

    /// # Arguments
    ///
    /// - `relative_surface`:
    /// - `id`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_dock_preview_context(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelManagerV1>,
        relative_surface: &Rc<WlSurface>,
        id: &Rc<TreelandDockPreviewContextV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.send_get_dock_preview_context(
            relative_surface,
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_manager_v1.get_dock_preview_context message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for TreelandForeignToplevelManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::TreelandForeignToplevelManagerV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_foreign_toplevel_manager_v1#{}.stop()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_stop(&self);
                } else {
                    DefaultHandler.handle_stop(&self);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_foreign_toplevel_manager_v1#{}.get_dock_preview_context(relative_surface: wl_surface#{}, id: treeland_dock_preview_context_v1#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("relative_surface", o.core().interface, ObjectInterface::WlSurface));
                };
                let arg1_id = arg1;
                let arg1 = TreelandDockPreviewContextV1::new(&self.core.state, self.core.version);
                arg1.core().set_client_id(client, arg1_id, arg1.clone())
                    .map_err(|e| ObjectError::SetClientId(arg1_id, "id", e))?;
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_dock_preview_context(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_dock_preview_context(&self, arg0, arg1);
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
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_foreign_toplevel_manager_v1#{}.toplevel(toplevel: treeland_foreign_toplevel_handle_v1#{})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = TreelandForeignToplevelHandleV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetServerId(arg0_id, "toplevel", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_toplevel(&self, arg0);
                } else {
                    DefaultHandler.handle_toplevel(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_foreign_toplevel_manager_v1#{}.finished()\n", msg[0]);
                    self.core.state.log(args);
                }
                self.core.handle_server_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_finished(&self);
                } else {
                    DefaultHandler.handle_finished(&self);
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
            0 => "stop",
            1 => "get_dock_preview_context",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "toplevel",
            1 => "finished",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for TreelandForeignToplevelManagerV1 {
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

