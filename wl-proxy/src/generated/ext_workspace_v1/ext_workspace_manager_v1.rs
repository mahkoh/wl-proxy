//! list and control workspaces
//!
//! Workspaces, also called virtual desktops, are groups of surfaces. A
//! compositor with a concept of workspaces may only show some such groups of
//! surfaces (those of 'active' workspaces) at a time. 'Activating' a
//! workspace is a request for the compositor to display that workspace's
//! surfaces as normal, whereas the compositor may hide or otherwise
//! de-emphasise surfaces that are associated only with 'inactive' workspaces.
//! Workspaces are grouped by which sets of outputs they correspond to, and
//! may contain surfaces only from those outputs. In this way, it is possible
//! for each output to have its own set of workspaces, or for all outputs (or
//! any other arbitrary grouping) to share workspaces. Compositors may
//! optionally conceptually arrange each group of workspaces in an
//! N-dimensional grid.
//!
//! The purpose of this protocol is to enable the creation of taskbars and
//! docks by providing them with a list of workspaces and their properties,
//! and allowing them to activate and deactivate workspaces.
//!
//! After a client binds the ext_workspace_manager_v1, each workspace will be
//! sent via the workspace event.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_workspace_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ExtWorkspaceManagerV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn ExtWorkspaceManagerV1Handler>,
}

struct DefaultHandler;

impl ExtWorkspaceManagerV1Handler for DefaultHandler { }

impl ExtWorkspaceManagerV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "ext_workspace_manager_v1";
}

impl ExtWorkspaceManagerV1 {
    pub fn set_handler(&self, handler: impl ExtWorkspaceManagerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ExtWorkspaceManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ExtWorkspaceManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtWorkspaceManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ExtWorkspaceManagerV1 {
    /// Since when the workspace_group message is available.
    pub const MSG__WORKSPACE_GROUP__SINCE: u32 = 1;

    /// a workspace group has been created
    ///
    /// This event is emitted whenever a new workspace group has been created.
    ///
    /// All initial details of the workspace group (outputs) will be
    /// sent immediately after this event via the corresponding events in
    /// ext_workspace_group_handle_v1 and ext_workspace_handle_v1.
    #[inline]
    pub fn send_workspace_group(
        &self,
        workspace_group: &Rc<ExtWorkspaceGroupHandleV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            workspace_group,
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
            .map_err(|e| ObjectError::GenerateClientId("workspace_group", e))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_workspace_manager_v1#{}.workspace_group(workspace_group: ext_workspace_group_handle_v1#{})\n", client.endpoint.id, id, arg0_id);
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

    /// Since when the workspace message is available.
    pub const MSG__WORKSPACE__SINCE: u32 = 1;

    /// workspace has been created
    ///
    /// This event is emitted whenever a new workspace has been created.
    ///
    /// All initial details of the workspace (name, coordinates, state) will
    /// be sent immediately after this event via the corresponding events in
    /// ext_workspace_handle_v1.
    ///
    /// Workspaces start off unassigned to any workspace group.
    #[inline]
    pub fn send_workspace(
        &self,
        workspace: &Rc<ExtWorkspaceHandleV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            workspace,
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
            .map_err(|e| ObjectError::GenerateClientId("workspace", e))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_workspace_manager_v1#{}.workspace(workspace: ext_workspace_handle_v1#{})\n", client.endpoint.id, id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the commit message is available.
    pub const MSG__COMMIT__SINCE: u32 = 1;

    /// all requests about the workspaces have been sent
    ///
    /// The client must send this request after it has finished sending other
    /// requests. The compositor must process a series of requests preceding a
    /// commit request atomically.
    ///
    /// This allows changes to the workspace properties to be seen as atomic,
    /// even if they happen via multiple events, and even if they involve
    /// multiple ext_workspace_handle_v1 objects, for example, deactivating one
    /// workspace and activating another.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_workspace_manager_v1#{}.commit()\n", id);
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

    /// Since when the done message is available.
    pub const MSG__DONE__SINCE: u32 = 1;

    /// all information about the workspaces and workspace groups has been sent
    ///
    /// This event is sent after all changes in all workspaces and workspace groups have been
    /// sent.
    ///
    /// This allows changes to one or more ext_workspace_group_handle_v1
    /// properties and ext_workspace_handle_v1 properties
    /// to be seen as atomic, even if they happen via multiple events.
    /// In particular, an output moving from one workspace group to
    /// another sends an output_enter event and an output_leave event to the two
    /// ext_workspace_group_handle_v1 objects in question. The compositor sends
    /// the done event only after updating the output information in both
    /// workspace groups.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_workspace_manager_v1#{}.done()\n", client.endpoint.id, id);
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
            2,
        ]);
        Ok(())
    }

    /// Since when the finished message is available.
    pub const MSG__FINISHED__SINCE: u32 = 1;

    /// the compositor has finished with the workspace_manager
    ///
    /// This event indicates that the compositor is done sending events to the
    /// ext_workspace_manager_v1. The server will destroy the object
    /// immediately after sending this request.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_workspace_manager_v1#{}.finished()\n", client.endpoint.id, id);
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
            3,
        ]);
        drop(fmt);
        drop(outgoing_ref);
        drop(client_ref);
        self.core.handle_client_destroy();
        Ok(())
    }

    /// Since when the stop message is available.
    pub const MSG__STOP__SINCE: u32 = 1;

    /// stop sending events
    ///
    /// Indicates the client no longer wishes to receive events for new
    /// workspace groups. However the compositor may emit further workspace
    /// events, until the finished event is emitted. The compositor is expected
    /// to send the finished event eventually once the stop request has been processed.
    ///
    /// The client must not send any requests after this one, doing so will raise a wl_display
    /// invalid_object error.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_workspace_manager_v1#{}.stop()\n", id);
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
        ]);
        Ok(())
    }
}

/// A message handler for [ExtWorkspaceManagerV1] proxies.
pub trait ExtWorkspaceManagerV1Handler: Any {
    /// a workspace group has been created
    ///
    /// This event is emitted whenever a new workspace group has been created.
    ///
    /// All initial details of the workspace group (outputs) will be
    /// sent immediately after this event via the corresponding events in
    /// ext_workspace_group_handle_v1 and ext_workspace_handle_v1.
    ///
    /// # Arguments
    ///
    /// - `workspace_group`:
    #[inline]
    fn workspace_group(
        &mut self,
        _slf: &Rc<ExtWorkspaceManagerV1>,
        workspace_group: &Rc<ExtWorkspaceGroupHandleV1>,
    ) {
        let res = _slf.send_workspace_group(
            workspace_group,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_manager_v1.workspace_group message: {}", Report::new(e));
        }
    }

    /// workspace has been created
    ///
    /// This event is emitted whenever a new workspace has been created.
    ///
    /// All initial details of the workspace (name, coordinates, state) will
    /// be sent immediately after this event via the corresponding events in
    /// ext_workspace_handle_v1.
    ///
    /// Workspaces start off unassigned to any workspace group.
    ///
    /// # Arguments
    ///
    /// - `workspace`:
    #[inline]
    fn workspace(
        &mut self,
        _slf: &Rc<ExtWorkspaceManagerV1>,
        workspace: &Rc<ExtWorkspaceHandleV1>,
    ) {
        let res = _slf.send_workspace(
            workspace,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_manager_v1.workspace message: {}", Report::new(e));
        }
    }

    /// all requests about the workspaces have been sent
    ///
    /// The client must send this request after it has finished sending other
    /// requests. The compositor must process a series of requests preceding a
    /// commit request atomically.
    ///
    /// This allows changes to the workspace properties to be seen as atomic,
    /// even if they happen via multiple events, and even if they involve
    /// multiple ext_workspace_handle_v1 objects, for example, deactivating one
    /// workspace and activating another.
    #[inline]
    fn commit(
        &mut self,
        _slf: &Rc<ExtWorkspaceManagerV1>,
    ) {
        let res = _slf.send_commit(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_manager_v1.commit message: {}", Report::new(e));
        }
    }

    /// all information about the workspaces and workspace groups has been sent
    ///
    /// This event is sent after all changes in all workspaces and workspace groups have been
    /// sent.
    ///
    /// This allows changes to one or more ext_workspace_group_handle_v1
    /// properties and ext_workspace_handle_v1 properties
    /// to be seen as atomic, even if they happen via multiple events.
    /// In particular, an output moving from one workspace group to
    /// another sends an output_enter event and an output_leave event to the two
    /// ext_workspace_group_handle_v1 objects in question. The compositor sends
    /// the done event only after updating the output information in both
    /// workspace groups.
    #[inline]
    fn done(
        &mut self,
        _slf: &Rc<ExtWorkspaceManagerV1>,
    ) {
        let res = _slf.send_done(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_manager_v1.done message: {}", Report::new(e));
        }
    }

    /// the compositor has finished with the workspace_manager
    ///
    /// This event indicates that the compositor is done sending events to the
    /// ext_workspace_manager_v1. The server will destroy the object
    /// immediately after sending this request.
    #[inline]
    fn finished(
        &mut self,
        _slf: &Rc<ExtWorkspaceManagerV1>,
    ) {
        let res = _slf.send_finished(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_manager_v1.finished message: {}", Report::new(e));
        }
    }

    /// stop sending events
    ///
    /// Indicates the client no longer wishes to receive events for new
    /// workspace groups. However the compositor may emit further workspace
    /// events, until the finished event is emitted. The compositor is expected
    /// to send the finished event eventually once the stop request has been processed.
    ///
    /// The client must not send any requests after this one, doing so will raise a wl_display
    /// invalid_object error.
    #[inline]
    fn stop(
        &mut self,
        _slf: &Rc<ExtWorkspaceManagerV1>,
    ) {
        let res = _slf.send_stop(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_manager_v1.stop message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for ExtWorkspaceManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::ExtWorkspaceManagerV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_workspace_manager_v1#{}.commit()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).commit(&self);
                } else {
                    DefaultHandler.commit(&self);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_workspace_manager_v1#{}.stop()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).stop(&self);
                } else {
                    DefaultHandler.stop(&self);
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
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_workspace_manager_v1#{}.workspace_group(workspace_group: ext_workspace_group_handle_v1#{})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ExtWorkspaceGroupHandleV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetServerId(arg0_id, "workspace_group", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).workspace_group(&self, arg0);
                } else {
                    DefaultHandler.workspace_group(&self, arg0);
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_workspace_manager_v1#{}.workspace(workspace: ext_workspace_handle_v1#{})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ExtWorkspaceHandleV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetServerId(arg0_id, "workspace", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).workspace(&self, arg0);
                } else {
                    DefaultHandler.workspace(&self, arg0);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_workspace_manager_v1#{}.done()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).done(&self);
                } else {
                    DefaultHandler.done(&self);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_workspace_manager_v1#{}.finished()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).finished(&self);
                } else {
                    DefaultHandler.finished(&self);
                }
                self.core.handle_server_destroy();
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
            0 => "commit",
            1 => "stop",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "workspace_group",
            1 => "workspace",
            2 => "done",
            3 => "finished",
            _ => return None,
        };
        Some(name)
    }
}

impl Proxy for ExtWorkspaceManagerV1 {
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

