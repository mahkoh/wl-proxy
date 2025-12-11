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
pub struct ExtWorkspaceGroupHandleV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn ExtWorkspaceGroupHandleV1Handler>,
}

struct DefaultHandler;

impl ExtWorkspaceGroupHandleV1Handler for DefaultHandler { }

impl ExtWorkspaceGroupHandleV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "ext_workspace_group_handle_v1";
}

impl ExtWorkspaceGroupHandleV1 {
    pub fn set_handler(&self, handler: impl ExtWorkspaceGroupHandleV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ExtWorkspaceGroupHandleV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ExtWorkspaceGroupHandleV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtWorkspaceGroupHandleV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ExtWorkspaceGroupHandleV1 {
    /// Since when the capabilities message is available.
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
        capabilities: ExtWorkspaceGroupHandleV1GroupCapabilities,
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_workspace_group_handle_v1#{}.capabilities(capabilities: {:?})\n", client.endpoint.id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the output_enter message is available.
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
        output: &Rc<WlOutput>,
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_workspace_group_handle_v1#{}.output_enter(output: wl_output#{})\n", client.endpoint.id, id, arg0_id);
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

    /// Since when the output_leave message is available.
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
        output: &Rc<WlOutput>,
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_workspace_group_handle_v1#{}.output_leave(output: wl_output#{})\n", client.endpoint.id, id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the workspace_enter message is available.
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
        workspace: &Rc<ExtWorkspaceHandleV1>,
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_workspace_group_handle_v1#{}.workspace_enter(workspace: ext_workspace_handle_v1#{})\n", client.endpoint.id, id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the workspace_leave message is available.
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
        workspace: &Rc<ExtWorkspaceHandleV1>,
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_workspace_group_handle_v1#{}.workspace_leave(workspace: ext_workspace_handle_v1#{})\n", client.endpoint.id, id, arg0_id);
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
            4,
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the removed message is available.
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_workspace_group_handle_v1#{}.removed()\n", client.endpoint.id, id);
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
            5,
        ]);
        Ok(())
    }

    /// Since when the create_workspace message is available.
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_workspace_group_handle_v1#{}.create_workspace(workspace: {:?})\n", id, arg0);
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
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the destroy message is available.
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_workspace_group_handle_v1#{}.destroy()\n", id);
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
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [ExtWorkspaceGroupHandleV1] proxies.
pub trait ExtWorkspaceGroupHandleV1Handler: Any {
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
        _slf: &Rc<ExtWorkspaceGroupHandleV1>,
        capabilities: ExtWorkspaceGroupHandleV1GroupCapabilities,
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
        _slf: &Rc<ExtWorkspaceGroupHandleV1>,
        output: &Rc<WlOutput>,
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
        _slf: &Rc<ExtWorkspaceGroupHandleV1>,
        output: &Rc<WlOutput>,
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
        _slf: &Rc<ExtWorkspaceGroupHandleV1>,
        workspace: &Rc<ExtWorkspaceHandleV1>,
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
        _slf: &Rc<ExtWorkspaceGroupHandleV1>,
        workspace: &Rc<ExtWorkspaceHandleV1>,
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
        _slf: &Rc<ExtWorkspaceGroupHandleV1>,
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
        _slf: &Rc<ExtWorkspaceGroupHandleV1>,
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
        _slf: &Rc<ExtWorkspaceGroupHandleV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_group_handle_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for ExtWorkspaceGroupHandleV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::ExtWorkspaceGroupHandleV1, version),
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
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_workspace_group_handle_v1#{}.create_workspace(workspace: {:?})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).create_workspace(&self, arg0);
                } else {
                    DefaultHandler.create_workspace(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_workspace_group_handle_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = ExtWorkspaceGroupHandleV1GroupCapabilities(arg0);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_workspace_group_handle_v1#{}.capabilities(capabilities: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).capabilities(&self, arg0);
                } else {
                    DefaultHandler.capabilities(&self, arg0);
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_workspace_group_handle_v1#{}.output_enter(output: wl_output#{})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = self.core.state.server.lookup(arg0_id) else {
                    return Err(ObjectError::NoServerObject(arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlOutput>() else {
                    let o = self.core.state.server.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("output", o.core().interface, ProxyInterface::WlOutput));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).output_enter(&self, arg0);
                } else {
                    DefaultHandler.output_enter(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_workspace_group_handle_v1#{}.output_leave(output: wl_output#{})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = self.core.state.server.lookup(arg0_id) else {
                    return Err(ObjectError::NoServerObject(arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlOutput>() else {
                    let o = self.core.state.server.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("output", o.core().interface, ProxyInterface::WlOutput));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).output_leave(&self, arg0);
                } else {
                    DefaultHandler.output_leave(&self, arg0);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_workspace_group_handle_v1#{}.workspace_enter(workspace: ext_workspace_handle_v1#{})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = self.core.state.server.lookup(arg0_id) else {
                    return Err(ObjectError::NoServerObject(arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ExtWorkspaceHandleV1>() else {
                    let o = self.core.state.server.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("workspace", o.core().interface, ProxyInterface::ExtWorkspaceHandleV1));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).workspace_enter(&self, arg0);
                } else {
                    DefaultHandler.workspace_enter(&self, arg0);
                }
            }
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_workspace_group_handle_v1#{}.workspace_leave(workspace: ext_workspace_handle_v1#{})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = self.core.state.server.lookup(arg0_id) else {
                    return Err(ObjectError::NoServerObject(arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ExtWorkspaceHandleV1>() else {
                    let o = self.core.state.server.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("workspace", o.core().interface, ProxyInterface::ExtWorkspaceHandleV1));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).workspace_leave(&self, arg0);
                } else {
                    DefaultHandler.workspace_leave(&self, arg0);
                }
            }
            5 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_workspace_group_handle_v1#{}.removed()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).removed(&self);
                } else {
                    DefaultHandler.removed(&self);
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

impl Proxy for ExtWorkspaceGroupHandleV1 {
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

impl ExtWorkspaceGroupHandleV1 {
    /// Since when the group_capabilities.create_workspace enum variant is available.
    pub const ENM__GROUP_CAPABILITIES_CREATE_WORKSPACE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct ExtWorkspaceGroupHandleV1GroupCapabilities(pub u32);

/// An iterator over the set bits in a [ExtWorkspaceGroupHandleV1GroupCapabilities].
///
/// You can construct this with the `IntoIterator` implementation of `ExtWorkspaceGroupHandleV1GroupCapabilities`.
#[derive(Clone, Debug)]
pub struct ExtWorkspaceGroupHandleV1GroupCapabilitiesIter(pub u32);

impl ExtWorkspaceGroupHandleV1GroupCapabilities {
    /// create_workspace request is available
    pub const CREATE_WORKSPACE: Self = Self(1);
}

impl ExtWorkspaceGroupHandleV1GroupCapabilities {
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

impl Iterator for ExtWorkspaceGroupHandleV1GroupCapabilitiesIter {
    type Item = ExtWorkspaceGroupHandleV1GroupCapabilities;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(ExtWorkspaceGroupHandleV1GroupCapabilities(bit))
    }
}

impl IntoIterator for ExtWorkspaceGroupHandleV1GroupCapabilities {
    type Item = ExtWorkspaceGroupHandleV1GroupCapabilities;
    type IntoIter = ExtWorkspaceGroupHandleV1GroupCapabilitiesIter;

    fn into_iter(self) -> Self::IntoIter {
        ExtWorkspaceGroupHandleV1GroupCapabilitiesIter(self.0)
    }
}

impl BitAnd for ExtWorkspaceGroupHandleV1GroupCapabilities {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for ExtWorkspaceGroupHandleV1GroupCapabilities {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for ExtWorkspaceGroupHandleV1GroupCapabilities {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for ExtWorkspaceGroupHandleV1GroupCapabilities {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for ExtWorkspaceGroupHandleV1GroupCapabilities {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for ExtWorkspaceGroupHandleV1GroupCapabilities {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for ExtWorkspaceGroupHandleV1GroupCapabilities {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for ExtWorkspaceGroupHandleV1GroupCapabilities {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for ExtWorkspaceGroupHandleV1GroupCapabilities {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for ExtWorkspaceGroupHandleV1GroupCapabilities {
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
