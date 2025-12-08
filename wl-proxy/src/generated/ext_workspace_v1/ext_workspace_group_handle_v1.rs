//! a workspace group assigned to a set of outputs
//!
//! A ext_workspace_group_handle_v1 object represents a workspace group
//! that is assigned a set of outputs and contains a number of workspaces.
//!
//! The set of outputs assigned to the workspace group is conveyed to the client via
//! output_enter and output_leave events, and its workspaces are conveyed with
//! workspace events.
//!
//! For example, a compositor which has a set of workspaces for each output may
//! advertise a workspace group (and its workspaces) per output, whereas a compositor
//! where a workspace spans all outputs may advertise a single workspace group for all
//! outputs.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_workspace_group_handle_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaExtWorkspaceGroupHandleV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaExtWorkspaceGroupHandleV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaExtWorkspaceGroupHandleV1MessageHandler for DefaultMessageHandler { }

impl MetaExtWorkspaceGroupHandleV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaExtWorkspaceGroupHandleV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ExtWorkspaceGroupHandleV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaExtWorkspaceGroupHandleV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaExtWorkspaceGroupHandleV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaExtWorkspaceGroupHandleV1 {
    /// Since when the capabilities message is available.
    #[allow(dead_code)]
    pub const MSG__CAPABILITIES__SINCE: u32 = 1;

    /// compositor capabilities
    ///
    /// This event advertises the capabilities supported by the compositor. If
    /// a capability isn't supported, clients should hide or disable the UI
    /// elements that expose this functionality. For instance, if the
    /// compositor doesn't advertise support for creating workspaces, a button
    /// triggering the create_workspace request should not be displayed.
    ///
    /// The compositor will ignore requests it doesn't support. For instance,
    /// a compositor which doesn't advertise support for creating workspaces will ignore
    /// create_workspace requests.
    ///
    /// Compositors must send this event once after creation of an
    /// ext_workspace_group_handle_v1. When the capabilities change, compositors
    /// must send this event again.
    ///
    /// # Arguments
    ///
    /// - `capabilities`: capabilities
    #[inline]
    pub fn send_capabilities(
        &self,
        capabilities: MetaExtWorkspaceGroupHandleV1GroupCapabilities,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            capabilities,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= ext_workspace_group_handle_v1#{}.capabilities(capabilities: {:?})", client.endpoint.id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the output_enter message is available.
    #[allow(dead_code)]
    pub const MSG__OUTPUT_ENTER__SINCE: u32 = 1;

    /// output assigned to workspace group
    ///
    /// This event is emitted whenever an output is assigned to the workspace
    /// group or a new `wl_output` object is bound by the client, which was already
    /// assigned to this workspace_group.
    ///
    /// # Arguments
    ///
    /// - `output`:
    #[inline]
    pub fn send_output_enter(
        &self,
        output: &Rc<MetaWlOutput>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            output,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError::ArgNoClientId("output", client.endpoint.id));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= ext_workspace_group_handle_v1#{}.output_enter(output: wl_output#{})", client.endpoint.id, id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the output_leave message is available.
    #[allow(dead_code)]
    pub const MSG__OUTPUT_LEAVE__SINCE: u32 = 1;

    /// output removed from workspace group
    ///
    /// This event is emitted whenever an output is removed from the workspace
    /// group.
    ///
    /// # Arguments
    ///
    /// - `output`:
    #[inline]
    pub fn send_output_leave(
        &self,
        output: &Rc<MetaWlOutput>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            output,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError::ArgNoClientId("output", client.endpoint.id));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= ext_workspace_group_handle_v1#{}.output_leave(output: wl_output#{})", client.endpoint.id, id, arg0_id);
        let endpoint = &client.endpoint;
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
        ]);
        Ok(())
    }

    /// Since when the workspace_enter message is available.
    #[allow(dead_code)]
    pub const MSG__WORKSPACE_ENTER__SINCE: u32 = 1;

    /// workspace added to workspace group
    ///
    /// This event is emitted whenever a workspace is assigned to this group.
    /// A workspace may only ever be assigned to a single group at a single point
    /// in time, but can be re-assigned during it's lifetime.
    ///
    /// # Arguments
    ///
    /// - `workspace`:
    #[inline]
    pub fn send_workspace_enter(
        &self,
        workspace: &Rc<MetaExtWorkspaceHandleV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            workspace,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError::ArgNoClientId("workspace", client.endpoint.id));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= ext_workspace_group_handle_v1#{}.workspace_enter(workspace: ext_workspace_handle_v1#{})", client.endpoint.id, id, arg0_id);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            3,
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the workspace_leave message is available.
    #[allow(dead_code)]
    pub const MSG__WORKSPACE_LEAVE__SINCE: u32 = 1;

    /// workspace removed from workspace group
    ///
    /// This event is emitted whenever a workspace is removed from this group.
    ///
    /// # Arguments
    ///
    /// - `workspace`:
    #[inline]
    pub fn send_workspace_leave(
        &self,
        workspace: &Rc<MetaExtWorkspaceHandleV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            workspace,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError::ArgNoClientId("workspace", client.endpoint.id));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= ext_workspace_group_handle_v1#{}.workspace_leave(workspace: ext_workspace_handle_v1#{})", client.endpoint.id, id, arg0_id);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            4,
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the removed message is available.
    #[allow(dead_code)]
    pub const MSG__REMOVED__SINCE: u32 = 1;

    /// this workspace group has been removed
    ///
    /// This event is send when the group associated with the ext_workspace_group_handle_v1
    /// has been removed. After sending this request the compositor will immediately consider
    /// the object inert. Any requests will be ignored except the destroy request.
    /// It is guaranteed there won't be any more events referencing this
    /// ext_workspace_group_handle_v1.
    ///
    /// The compositor must remove all workspaces belonging to a workspace group
    /// via a workspace_leave event before removing the workspace group.
    #[inline]
    pub fn send_removed(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= ext_workspace_group_handle_v1#{}.removed()", client.endpoint.id, id);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            5,
        ]);
        Ok(())
    }

    /// Since when the create_workspace message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE_WORKSPACE__SINCE: u32 = 1;

    /// create a new workspace
    ///
    /// Request that the compositor create a new workspace with the given name
    /// and assign it to this group.
    ///
    /// There is no guarantee that the compositor will create a new workspace,
    /// or that the created workspace will have the provided name.
    ///
    /// # Arguments
    ///
    /// - `workspace`:
    #[inline]
    pub fn send_create_workspace(
        &self,
        workspace: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            workspace,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= ext_workspace_group_handle_v1#{}.create_workspace(workspace: {:?})", id, arg0);
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
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the ext_workspace_group_handle_v1 object
    ///
    /// Destroys the ext_workspace_group_handle_v1 object.
    ///
    /// This request should be send either when the client does not want to
    /// use the workspace group object any more or after the removed event to finalize
    /// the destruction of the object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= ext_workspace_group_handle_v1#{}.destroy()", id);
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
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [ExtWorkspaceGroupHandleV1] proxies.
#[allow(dead_code)]
pub trait MetaExtWorkspaceGroupHandleV1MessageHandler {
    /// compositor capabilities
    ///
    /// This event advertises the capabilities supported by the compositor. If
    /// a capability isn't supported, clients should hide or disable the UI
    /// elements that expose this functionality. For instance, if the
    /// compositor doesn't advertise support for creating workspaces, a button
    /// triggering the create_workspace request should not be displayed.
    ///
    /// The compositor will ignore requests it doesn't support. For instance,
    /// a compositor which doesn't advertise support for creating workspaces will ignore
    /// create_workspace requests.
    ///
    /// Compositors must send this event once after creation of an
    /// ext_workspace_group_handle_v1. When the capabilities change, compositors
    /// must send this event again.
    ///
    /// # Arguments
    ///
    /// - `capabilities`: capabilities
    #[inline]
    fn capabilities(
        &mut self,
        _slf: &Rc<MetaExtWorkspaceGroupHandleV1>,
        capabilities: MetaExtWorkspaceGroupHandleV1GroupCapabilities,
    ) {
        let res = _slf.send_capabilities(
            capabilities,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_group_handle_v1.capabilities message: {}", Report::new(e));
        }
    }

    /// output assigned to workspace group
    ///
    /// This event is emitted whenever an output is assigned to the workspace
    /// group or a new `wl_output` object is bound by the client, which was already
    /// assigned to this workspace_group.
    ///
    /// # Arguments
    ///
    /// - `output`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn output_enter(
        &mut self,
        _slf: &Rc<MetaExtWorkspaceGroupHandleV1>,
        output: &Rc<MetaWlOutput>,
    ) {
        if let Some(client_id) = _slf.core.client_id.get() {
            if let Some(client_id_2) = output.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = _slf.send_output_enter(
            output,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_group_handle_v1.output_enter message: {}", Report::new(e));
        }
    }

    /// output removed from workspace group
    ///
    /// This event is emitted whenever an output is removed from the workspace
    /// group.
    ///
    /// # Arguments
    ///
    /// - `output`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn output_leave(
        &mut self,
        _slf: &Rc<MetaExtWorkspaceGroupHandleV1>,
        output: &Rc<MetaWlOutput>,
    ) {
        if let Some(client_id) = _slf.core.client_id.get() {
            if let Some(client_id_2) = output.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = _slf.send_output_leave(
            output,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_group_handle_v1.output_leave message: {}", Report::new(e));
        }
    }

    /// workspace added to workspace group
    ///
    /// This event is emitted whenever a workspace is assigned to this group.
    /// A workspace may only ever be assigned to a single group at a single point
    /// in time, but can be re-assigned during it's lifetime.
    ///
    /// # Arguments
    ///
    /// - `workspace`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn workspace_enter(
        &mut self,
        _slf: &Rc<MetaExtWorkspaceGroupHandleV1>,
        workspace: &Rc<MetaExtWorkspaceHandleV1>,
    ) {
        if let Some(client_id) = _slf.core.client_id.get() {
            if let Some(client_id_2) = workspace.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = _slf.send_workspace_enter(
            workspace,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_group_handle_v1.workspace_enter message: {}", Report::new(e));
        }
    }

    /// workspace removed from workspace group
    ///
    /// This event is emitted whenever a workspace is removed from this group.
    ///
    /// # Arguments
    ///
    /// - `workspace`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn workspace_leave(
        &mut self,
        _slf: &Rc<MetaExtWorkspaceGroupHandleV1>,
        workspace: &Rc<MetaExtWorkspaceHandleV1>,
    ) {
        if let Some(client_id) = _slf.core.client_id.get() {
            if let Some(client_id_2) = workspace.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = _slf.send_workspace_leave(
            workspace,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_group_handle_v1.workspace_leave message: {}", Report::new(e));
        }
    }

    /// this workspace group has been removed
    ///
    /// This event is send when the group associated with the ext_workspace_group_handle_v1
    /// has been removed. After sending this request the compositor will immediately consider
    /// the object inert. Any requests will be ignored except the destroy request.
    /// It is guaranteed there won't be any more events referencing this
    /// ext_workspace_group_handle_v1.
    ///
    /// The compositor must remove all workspaces belonging to a workspace group
    /// via a workspace_leave event before removing the workspace group.
    #[inline]
    fn removed(
        &mut self,
        _slf: &Rc<MetaExtWorkspaceGroupHandleV1>,
    ) {
        let res = _slf.send_removed(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_group_handle_v1.removed message: {}", Report::new(e));
        }
    }

    /// create a new workspace
    ///
    /// Request that the compositor create a new workspace with the given name
    /// and assign it to this group.
    ///
    /// There is no guarantee that the compositor will create a new workspace,
    /// or that the created workspace will have the provided name.
    ///
    /// # Arguments
    ///
    /// - `workspace`:
    #[inline]
    fn create_workspace(
        &mut self,
        _slf: &Rc<MetaExtWorkspaceGroupHandleV1>,
        workspace: &str,
    ) {
        let res = _slf.send_create_workspace(
            workspace,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_group_handle_v1.create_workspace message: {}", Report::new(e));
        }
    }

    /// destroy the ext_workspace_group_handle_v1 object
    ///
    /// Destroys the ext_workspace_group_handle_v1 object.
    ///
    /// This request should be send either when the client does not want to
    /// use the workspace group object any more or after the removed event to finalize
    /// the destruction of the object.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaExtWorkspaceGroupHandleV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_group_handle_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaExtWorkspaceGroupHandleV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("workspace"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("workspace"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("workspace"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("workspace"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                eprintln!("client#{:04} -> ext_workspace_group_handle_v1#{}.create_workspace(workspace: {:?})", client.endpoint.id, msg[0], arg0);
                if let Some(handler) = handler {
                    (**handler).create_workspace(&self, arg0);
                } else {
                    DefaultMessageHandler.create_workspace(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("client#{:04} -> ext_workspace_group_handle_v1#{}.destroy()", client.endpoint.id, msg[0]);
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
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
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = MetaExtWorkspaceGroupHandleV1GroupCapabilities(arg0);
                eprintln!("server      -> ext_workspace_group_handle_v1#{}.capabilities(capabilities: {:?})", msg[0], arg0);
                if let Some(handler) = handler {
                    (**handler).capabilities(&self, arg0);
                } else {
                    DefaultMessageHandler.capabilities(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                eprintln!("server      -> ext_workspace_group_handle_v1#{}.output_enter(output: wl_output#{})", msg[0], arg0);
                let arg0_id = arg0;
                let Some(arg0) = self.core.state.server.lookup(arg0_id) else {
                    return Err(ObjectError::NoServerObject(arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlOutput>() else {
                    let o = self.core.state.server.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("output", o.core().interface, ProxyInterface::WlOutput));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).output_enter(&self, arg0);
                } else {
                    DefaultMessageHandler.output_enter(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                eprintln!("server      -> ext_workspace_group_handle_v1#{}.output_leave(output: wl_output#{})", msg[0], arg0);
                let arg0_id = arg0;
                let Some(arg0) = self.core.state.server.lookup(arg0_id) else {
                    return Err(ObjectError::NoServerObject(arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlOutput>() else {
                    let o = self.core.state.server.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("output", o.core().interface, ProxyInterface::WlOutput));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).output_leave(&self, arg0);
                } else {
                    DefaultMessageHandler.output_leave(&self, arg0);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                eprintln!("server      -> ext_workspace_group_handle_v1#{}.workspace_enter(workspace: ext_workspace_handle_v1#{})", msg[0], arg0);
                let arg0_id = arg0;
                let Some(arg0) = self.core.state.server.lookup(arg0_id) else {
                    return Err(ObjectError::NoServerObject(arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaExtWorkspaceHandleV1>() else {
                    let o = self.core.state.server.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("workspace", o.core().interface, ProxyInterface::ExtWorkspaceHandleV1));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).workspace_enter(&self, arg0);
                } else {
                    DefaultMessageHandler.workspace_enter(&self, arg0);
                }
            }
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                eprintln!("server      -> ext_workspace_group_handle_v1#{}.workspace_leave(workspace: ext_workspace_handle_v1#{})", msg[0], arg0);
                let arg0_id = arg0;
                let Some(arg0) = self.core.state.server.lookup(arg0_id) else {
                    return Err(ObjectError::NoServerObject(arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaExtWorkspaceHandleV1>() else {
                    let o = self.core.state.server.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("workspace", o.core().interface, ProxyInterface::ExtWorkspaceHandleV1));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).workspace_leave(&self, arg0);
                } else {
                    DefaultMessageHandler.workspace_leave(&self, arg0);
                }
            }
            5 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("server      -> ext_workspace_group_handle_v1#{}.removed()", msg[0]);
                if let Some(handler) = handler {
                    (**handler).removed(&self);
                } else {
                    DefaultMessageHandler.removed(&self);
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
            0 => "create_workspace",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "capabilities",
            1 => "output_enter",
            2 => "output_leave",
            3 => "workspace_enter",
            4 => "workspace_leave",
            5 => "removed",
            _ => return None,
        };
        Some(name)
    }
}

impl MetaExtWorkspaceGroupHandleV1 {
    /// Since when the group_capabilities.create_workspace enum variant is available.
    #[allow(dead_code)]
    pub const ENM__GROUP_CAPABILITIES_CREATE_WORKSPACE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
#[allow(dead_code)]
pub struct MetaExtWorkspaceGroupHandleV1GroupCapabilities(pub u32);

/// An iterator over the set bits in a [MetaExtWorkspaceGroupHandleV1GroupCapabilities].
///
/// You can construct this with the `IntoIterator` implementation of `MetaExtWorkspaceGroupHandleV1GroupCapabilities`.
#[derive(Clone, Debug)]
pub struct MetaExtWorkspaceGroupHandleV1GroupCapabilitiesIter(pub u32);

impl MetaExtWorkspaceGroupHandleV1GroupCapabilities {
    /// create_workspace request is available
    #[allow(dead_code)]
    pub const CREATE_WORKSPACE: Self = Self(1);
}

#[allow(dead_code)]
impl MetaExtWorkspaceGroupHandleV1GroupCapabilities {
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    #[inline]
    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    #[inline]
    pub const fn insert(&mut self, other: Self) {
        *self = self.union(other);
    }

    #[inline]
    pub const fn remove(&mut self, other: Self) {
        *self = self.difference(other);
    }

    #[inline]
    pub const fn toggle(&mut self, other: Self) {
        *self = self.symmetric_difference(other);
    }

    #[inline]
    pub const fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }

    #[inline]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline]
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    #[inline]
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(!self.0)
    }

    #[inline]
    #[must_use]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    #[inline]
    pub const fn all_known() -> Self {
        #[allow(clippy::eq_op, clippy::identity_op)]
        Self(0 | 1)
    }
}

impl Iterator for MetaExtWorkspaceGroupHandleV1GroupCapabilitiesIter {
    type Item = MetaExtWorkspaceGroupHandleV1GroupCapabilities;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(MetaExtWorkspaceGroupHandleV1GroupCapabilities(bit))
    }
}

impl IntoIterator for MetaExtWorkspaceGroupHandleV1GroupCapabilities {
    type Item = MetaExtWorkspaceGroupHandleV1GroupCapabilities;
    type IntoIter = MetaExtWorkspaceGroupHandleV1GroupCapabilitiesIter;

    fn into_iter(self) -> Self::IntoIter {
        MetaExtWorkspaceGroupHandleV1GroupCapabilitiesIter(self.0)
    }
}

impl BitAnd for MetaExtWorkspaceGroupHandleV1GroupCapabilities {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for MetaExtWorkspaceGroupHandleV1GroupCapabilities {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for MetaExtWorkspaceGroupHandleV1GroupCapabilities {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for MetaExtWorkspaceGroupHandleV1GroupCapabilities {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for MetaExtWorkspaceGroupHandleV1GroupCapabilities {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for MetaExtWorkspaceGroupHandleV1GroupCapabilities {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for MetaExtWorkspaceGroupHandleV1GroupCapabilities {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for MetaExtWorkspaceGroupHandleV1GroupCapabilities {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for MetaExtWorkspaceGroupHandleV1GroupCapabilities {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for MetaExtWorkspaceGroupHandleV1GroupCapabilities {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = self.0;
        let mut first = true;
        if v & 1 == 1 {
            v &= !1;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("CREATE_WORKSPACE")?;
        }
        if v != 0 {
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            write!(f, "0x{v:032x}")?;
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}
