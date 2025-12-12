//! a workspace handing a group of surfaces
//!
//! A ext_workspace_handle_v1 object represents a workspace that handles a
//! group of surfaces.
//!
//! Each workspace has:
//! - a name, conveyed to the client with the name event
//! - potentially an id conveyed with the id event
//! - a list of states, conveyed to the client with the state event
//! - and optionally a set of coordinates, conveyed to the client with the
//! coordinates event
//!
//! The client may request that the compositor activate or deactivate the workspace.
//!
//! Each workspace can belong to only a single workspace group.
//! Depending on the compositor policy, there might be workspaces with
//! the same name in different workspace groups, but these workspaces are still
//! separate (e.g. one of them might be active while the other is not).

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A ext_workspace_handle_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ExtWorkspaceHandleV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn ExtWorkspaceHandleV1Handler>,
}

struct DefaultHandler;

impl ExtWorkspaceHandleV1Handler for DefaultHandler { }

impl ExtWorkspaceHandleV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ProxyInterface = ProxyInterface::ExtWorkspaceHandleV1;
    pub const INTERFACE_NAME: &str = "ext_workspace_handle_v1";
}

impl ExtWorkspaceHandleV1 {
    pub fn set_handler(&self, handler: impl ExtWorkspaceHandleV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ExtWorkspaceHandleV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ExtWorkspaceHandleV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtWorkspaceHandleV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ExtWorkspaceHandleV1 {
    /// Since when the id message is available.
    pub const MSG__ID__SINCE: u32 = 1;

    /// workspace id
    ///
    /// If this event is emitted, it will be send immediately after the
    /// ext_workspace_handle_v1 is created or when an id is assigned to
    /// a workspace (at most once during it's lifetime).
    ///
    /// An id will never change during the lifetime of the `ext_workspace_handle_v1`
    /// and is guaranteed to be unique during it's lifetime.
    ///
    /// Ids are not human-readable and shouldn't be displayed, use `name` for that purpose.
    ///
    /// Compositors are expected to only send ids for workspaces likely stable across multiple
    /// sessions and can be used by clients to store preferences for workspaces. Workspaces without
    /// ids should be considered temporary and any data associated with them should be deleted once
    /// the respective object is lost.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_id(
        &self,
        id: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_workspace_handle_v1#{}.id(id: {:?})\n", client.endpoint.id, id, arg0);
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
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the name message is available.
    pub const MSG__NAME__SINCE: u32 = 1;

    /// workspace name changed
    ///
    /// This event is emitted immediately after the ext_workspace_handle_v1 is
    /// created and whenever the name of the workspace changes.
    ///
    /// A name is meant to be human-readable and can be displayed to a user.
    /// Unlike the id it is neither stable nor unique.
    ///
    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    pub fn send_name(
        &self,
        name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            name,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_workspace_handle_v1#{}.name(name: {:?})\n", client.endpoint.id, id, arg0);
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
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the coordinates message is available.
    pub const MSG__COORDINATES__SINCE: u32 = 1;

    /// workspace coordinates changed
    ///
    /// This event is used to organize workspaces into an N-dimensional grid
    /// within a workspace group, and if supported, is emitted immediately after
    /// the ext_workspace_handle_v1 is created and whenever the coordinates of
    /// the workspace change. Compositors may not send this event if they do not
    /// conceptually arrange workspaces in this way. If compositors simply
    /// number workspaces, without any geometric interpretation, they may send
    /// 1D coordinates, which clients should not interpret as implying any
    /// geometry. Sending an empty array means that the compositor no longer
    /// orders the workspace geometrically.
    ///
    /// Coordinates have an arbitrary number of dimensions N with an uint32
    /// position along each dimension. By convention if N > 1, the first
    /// dimension is X, the second Y, the third Z, and so on. The compositor may
    /// chose to utilize these events for a more novel workspace layout
    /// convention, however. No guarantee is made about the grid being filled or
    /// bounded; there may be a workspace at coordinate 1 and another at
    /// coordinate 1000 and none in between. Within a workspace group, however,
    /// workspaces must have unique coordinates of equal dimensionality.
    ///
    /// # Arguments
    ///
    /// - `coordinates`:
    #[inline]
    pub fn send_coordinates(
        &self,
        coordinates: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            coordinates,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_workspace_handle_v1#{}.coordinates(coordinates: {})\n", client.endpoint.id, id, debug_array(arg0));
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
        fmt.array(arg0);
        Ok(())
    }

    /// Since when the state message is available.
    pub const MSG__STATE__SINCE: u32 = 1;

    /// the state of the workspace changed
    ///
    /// This event is emitted immediately after the ext_workspace_handle_v1 is
    /// created and each time the workspace state changes, either because of a
    /// compositor action or because of a request in this protocol.
    ///
    /// Missing states convey the opposite meaning, e.g. an unset active bit
    /// means the workspace is currently inactive.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_state(
        &self,
        state: ExtWorkspaceHandleV1State,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            state,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_workspace_handle_v1#{}.state(state: {:?})\n", client.endpoint.id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the capabilities message is available.
    pub const MSG__CAPABILITIES__SINCE: u32 = 1;

    /// compositor capabilities
    ///
    /// This event advertises the capabilities supported by the compositor. If
    /// a capability isn't supported, clients should hide or disable the UI
    /// elements that expose this functionality. For instance, if the
    /// compositor doesn't advertise support for removing workspaces, a button
    /// triggering the remove request should not be displayed.
    ///
    /// The compositor will ignore requests it doesn't support. For instance,
    /// a compositor which doesn't advertise support for remove will ignore
    /// remove requests.
    ///
    /// Compositors must send this event once after creation of an
    /// ext_workspace_handle_v1 . When the capabilities change, compositors
    /// must send this event again.
    ///
    /// # Arguments
    ///
    /// - `capabilities`: capabilities
    #[inline]
    pub fn send_capabilities(
        &self,
        capabilities: ExtWorkspaceHandleV1WorkspaceCapabilities,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_workspace_handle_v1#{}.capabilities(capabilities: {:?})\n", client.endpoint.id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the removed message is available.
    pub const MSG__REMOVED__SINCE: u32 = 1;

    /// this workspace has been removed
    ///
    /// This event is send when the workspace associated with the ext_workspace_handle_v1
    /// has been removed. After sending this request, the compositor will immediately consider
    /// the object inert. Any requests will be ignored except the destroy request.
    ///
    /// It is guaranteed there won't be any more events referencing this
    /// ext_workspace_handle_v1.
    ///
    /// The compositor must only remove a workspaces not currently belonging to any
    /// workspace_group.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_workspace_handle_v1#{}.removed()\n", client.endpoint.id, id);
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

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the ext_workspace_handle_v1 object
    ///
    /// Destroys the ext_workspace_handle_v1 object.
    ///
    /// This request should be made either when the client does not want to
    /// use the workspace object any more or after the remove event to finalize
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_workspace_handle_v1#{}.destroy()\n", id);
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

    /// Since when the activate message is available.
    pub const MSG__ACTIVATE__SINCE: u32 = 1;

    /// activate the workspace
    ///
    /// Request that this workspace be activated.
    ///
    /// There is no guarantee the workspace will be actually activated, and
    /// behaviour may be compositor-dependent. For example, activating a
    /// workspace may or may not deactivate all other workspaces in the same
    /// group.
    #[inline]
    pub fn send_activate(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_workspace_handle_v1#{}.activate()\n", id);
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

    /// Since when the deactivate message is available.
    pub const MSG__DEACTIVATE__SINCE: u32 = 1;

    /// deactivate the workspace
    ///
    /// Request that this workspace be deactivated.
    ///
    /// There is no guarantee the workspace will be actually deactivated.
    #[inline]
    pub fn send_deactivate(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_workspace_handle_v1#{}.deactivate()\n", id);
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

    /// Since when the assign message is available.
    pub const MSG__ASSIGN__SINCE: u32 = 1;

    /// assign workspace to group
    ///
    /// Requests that this workspace is assigned to the given workspace group.
    ///
    /// There is no guarantee the workspace will be assigned.
    ///
    /// # Arguments
    ///
    /// - `workspace_group`:
    #[inline]
    pub fn send_assign(
        &self,
        workspace_group: &Rc<ExtWorkspaceGroupHandleV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            workspace_group,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("workspace_group")),
            Some(id) => id,
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_workspace_handle_v1#{}.assign(workspace_group: ext_workspace_group_handle_v1#{})\n", id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the remove message is available.
    pub const MSG__REMOVE__SINCE: u32 = 1;

    /// remove the workspace
    ///
    /// Request that this workspace be removed.
    ///
    /// There is no guarantee the workspace will be actually removed.
    #[inline]
    pub fn send_remove(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_workspace_handle_v1#{}.remove()\n", id);
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
            4,
        ]);
        Ok(())
    }
}

/// A message handler for [ExtWorkspaceHandleV1] proxies.
pub trait ExtWorkspaceHandleV1Handler: Any {
    /// workspace id
    ///
    /// If this event is emitted, it will be send immediately after the
    /// ext_workspace_handle_v1 is created or when an id is assigned to
    /// a workspace (at most once during it's lifetime).
    ///
    /// An id will never change during the lifetime of the `ext_workspace_handle_v1`
    /// and is guaranteed to be unique during it's lifetime.
    ///
    /// Ids are not human-readable and shouldn't be displayed, use `name` for that purpose.
    ///
    /// Compositors are expected to only send ids for workspaces likely stable across multiple
    /// sessions and can be used by clients to store preferences for workspaces. Workspaces without
    /// ids should be considered temporary and any data associated with them should be deleted once
    /// the respective object is lost.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn id(
        &mut self,
        _slf: &Rc<ExtWorkspaceHandleV1>,
        id: &str,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        let res = _slf.send_id(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_handle_v1.id message: {}", Report::new(e));
        }
    }

    /// workspace name changed
    ///
    /// This event is emitted immediately after the ext_workspace_handle_v1 is
    /// created and whenever the name of the workspace changes.
    ///
    /// A name is meant to be human-readable and can be displayed to a user.
    /// Unlike the id it is neither stable nor unique.
    ///
    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    fn name(
        &mut self,
        _slf: &Rc<ExtWorkspaceHandleV1>,
        name: &str,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        let res = _slf.send_name(
            name,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_handle_v1.name message: {}", Report::new(e));
        }
    }

    /// workspace coordinates changed
    ///
    /// This event is used to organize workspaces into an N-dimensional grid
    /// within a workspace group, and if supported, is emitted immediately after
    /// the ext_workspace_handle_v1 is created and whenever the coordinates of
    /// the workspace change. Compositors may not send this event if they do not
    /// conceptually arrange workspaces in this way. If compositors simply
    /// number workspaces, without any geometric interpretation, they may send
    /// 1D coordinates, which clients should not interpret as implying any
    /// geometry. Sending an empty array means that the compositor no longer
    /// orders the workspace geometrically.
    ///
    /// Coordinates have an arbitrary number of dimensions N with an uint32
    /// position along each dimension. By convention if N > 1, the first
    /// dimension is X, the second Y, the third Z, and so on. The compositor may
    /// chose to utilize these events for a more novel workspace layout
    /// convention, however. No guarantee is made about the grid being filled or
    /// bounded; there may be a workspace at coordinate 1 and another at
    /// coordinate 1000 and none in between. Within a workspace group, however,
    /// workspaces must have unique coordinates of equal dimensionality.
    ///
    /// # Arguments
    ///
    /// - `coordinates`:
    #[inline]
    fn coordinates(
        &mut self,
        _slf: &Rc<ExtWorkspaceHandleV1>,
        coordinates: &[u8],
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        let res = _slf.send_coordinates(
            coordinates,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_handle_v1.coordinates message: {}", Report::new(e));
        }
    }

    /// the state of the workspace changed
    ///
    /// This event is emitted immediately after the ext_workspace_handle_v1 is
    /// created and each time the workspace state changes, either because of a
    /// compositor action or because of a request in this protocol.
    ///
    /// Missing states convey the opposite meaning, e.g. an unset active bit
    /// means the workspace is currently inactive.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn state(
        &mut self,
        _slf: &Rc<ExtWorkspaceHandleV1>,
        state: ExtWorkspaceHandleV1State,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        let res = _slf.send_state(
            state,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_handle_v1.state message: {}", Report::new(e));
        }
    }

    /// compositor capabilities
    ///
    /// This event advertises the capabilities supported by the compositor. If
    /// a capability isn't supported, clients should hide or disable the UI
    /// elements that expose this functionality. For instance, if the
    /// compositor doesn't advertise support for removing workspaces, a button
    /// triggering the remove request should not be displayed.
    ///
    /// The compositor will ignore requests it doesn't support. For instance,
    /// a compositor which doesn't advertise support for remove will ignore
    /// remove requests.
    ///
    /// Compositors must send this event once after creation of an
    /// ext_workspace_handle_v1 . When the capabilities change, compositors
    /// must send this event again.
    ///
    /// # Arguments
    ///
    /// - `capabilities`: capabilities
    #[inline]
    fn capabilities(
        &mut self,
        _slf: &Rc<ExtWorkspaceHandleV1>,
        capabilities: ExtWorkspaceHandleV1WorkspaceCapabilities,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        let res = _slf.send_capabilities(
            capabilities,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_handle_v1.capabilities message: {}", Report::new(e));
        }
    }

    /// this workspace has been removed
    ///
    /// This event is send when the workspace associated with the ext_workspace_handle_v1
    /// has been removed. After sending this request, the compositor will immediately consider
    /// the object inert. Any requests will be ignored except the destroy request.
    ///
    /// It is guaranteed there won't be any more events referencing this
    /// ext_workspace_handle_v1.
    ///
    /// The compositor must only remove a workspaces not currently belonging to any
    /// workspace_group.
    #[inline]
    fn removed(
        &mut self,
        _slf: &Rc<ExtWorkspaceHandleV1>,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        let res = _slf.send_removed(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_handle_v1.removed message: {}", Report::new(e));
        }
    }

    /// destroy the ext_workspace_handle_v1 object
    ///
    /// Destroys the ext_workspace_handle_v1 object.
    ///
    /// This request should be made either when the client does not want to
    /// use the workspace object any more or after the remove event to finalize
    /// the destruction of the object.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<ExtWorkspaceHandleV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_handle_v1.destroy message: {}", Report::new(e));
        }
    }

    /// activate the workspace
    ///
    /// Request that this workspace be activated.
    ///
    /// There is no guarantee the workspace will be actually activated, and
    /// behaviour may be compositor-dependent. For example, activating a
    /// workspace may or may not deactivate all other workspaces in the same
    /// group.
    #[inline]
    fn activate(
        &mut self,
        _slf: &Rc<ExtWorkspaceHandleV1>,
    ) {
        let res = _slf.send_activate(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_handle_v1.activate message: {}", Report::new(e));
        }
    }

    /// deactivate the workspace
    ///
    /// Request that this workspace be deactivated.
    ///
    /// There is no guarantee the workspace will be actually deactivated.
    #[inline]
    fn deactivate(
        &mut self,
        _slf: &Rc<ExtWorkspaceHandleV1>,
    ) {
        let res = _slf.send_deactivate(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_handle_v1.deactivate message: {}", Report::new(e));
        }
    }

    /// assign workspace to group
    ///
    /// Requests that this workspace is assigned to the given workspace group.
    ///
    /// There is no guarantee the workspace will be assigned.
    ///
    /// # Arguments
    ///
    /// - `workspace_group`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn assign(
        &mut self,
        _slf: &Rc<ExtWorkspaceHandleV1>,
        workspace_group: &Rc<ExtWorkspaceGroupHandleV1>,
    ) {
        let res = _slf.send_assign(
            workspace_group,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_handle_v1.assign message: {}", Report::new(e));
        }
    }

    /// remove the workspace
    ///
    /// Request that this workspace be removed.
    ///
    /// There is no guarantee the workspace will be actually removed.
    #[inline]
    fn remove(
        &mut self,
        _slf: &Rc<ExtWorkspaceHandleV1>,
    ) {
        let res = _slf.send_remove(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_workspace_handle_v1.remove message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for ExtWorkspaceHandleV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::ExtWorkspaceHandleV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_workspace_handle_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_workspace_handle_v1#{}.activate()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).activate(&self);
                } else {
                    DefaultHandler.activate(&self);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_workspace_handle_v1#{}.deactivate()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).deactivate(&self);
                } else {
                    DefaultHandler.deactivate(&self);
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_workspace_handle_v1#{}.assign(workspace_group: ext_workspace_group_handle_v1#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ExtWorkspaceGroupHandleV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("workspace_group", o.core().interface, ProxyInterface::ExtWorkspaceGroupHandleV1));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).assign(&self, arg0);
                } else {
                    DefaultHandler.assign(&self, arg0);
                }
            }
            4 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_workspace_handle_v1#{}.remove()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).remove(&self);
                } else {
                    DefaultHandler.remove(&self);
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
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("id"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("id"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("id"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("id"));
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_workspace_handle_v1#{}.id(id: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).id(&self, arg0);
                } else {
                    DefaultHandler.id(&self, arg0);
                }
            }
            1 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("name"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("name"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("name"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("name"));
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_workspace_handle_v1#{}.name(name: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).name(&self, arg0);
                } else {
                    DefaultHandler.name(&self, arg0);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("coordinates"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("coordinates"));
                    }
                    let start = offset;
                    offset += words;
                    &uapi::as_bytes(&msg[start..])[..len]
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_workspace_handle_v1#{}.coordinates(coordinates: {})\n", msg[0], debug_array(arg0));
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).coordinates(&self, arg0);
                } else {
                    DefaultHandler.coordinates(&self, arg0);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = ExtWorkspaceHandleV1State(arg0);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_workspace_handle_v1#{}.state(state: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).state(&self, arg0);
                } else {
                    DefaultHandler.state(&self, arg0);
                }
            }
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = ExtWorkspaceHandleV1WorkspaceCapabilities(arg0);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_workspace_handle_v1#{}.capabilities(capabilities: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).capabilities(&self, arg0);
                } else {
                    DefaultHandler.capabilities(&self, arg0);
                }
            }
            5 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_workspace_handle_v1#{}.removed()\n", msg[0]);
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
            0 => "destroy",
            1 => "activate",
            2 => "deactivate",
            3 => "assign",
            4 => "remove",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "id",
            1 => "name",
            2 => "coordinates",
            3 => "state",
            4 => "capabilities",
            5 => "removed",
            _ => return None,
        };
        Some(name)
    }
}

impl Proxy for ExtWorkspaceHandleV1 {
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

impl ExtWorkspaceHandleV1 {
    /// Since when the state.active enum variant is available.
    pub const ENM__STATE_ACTIVE__SINCE: u32 = 1;
    /// Since when the state.urgent enum variant is available.
    pub const ENM__STATE_URGENT__SINCE: u32 = 1;
    /// Since when the state.hidden enum variant is available.
    pub const ENM__STATE_HIDDEN__SINCE: u32 = 1;

    /// Since when the workspace_capabilities.activate enum variant is available.
    pub const ENM__WORKSPACE_CAPABILITIES_ACTIVATE__SINCE: u32 = 1;
    /// Since when the workspace_capabilities.deactivate enum variant is available.
    pub const ENM__WORKSPACE_CAPABILITIES_DEACTIVATE__SINCE: u32 = 1;
    /// Since when the workspace_capabilities.remove enum variant is available.
    pub const ENM__WORKSPACE_CAPABILITIES_REMOVE__SINCE: u32 = 1;
    /// Since when the workspace_capabilities.assign enum variant is available.
    pub const ENM__WORKSPACE_CAPABILITIES_ASSIGN__SINCE: u32 = 1;
}

/// types of states on the workspace
///
/// The different states that a workspace can have.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct ExtWorkspaceHandleV1State(pub u32);

/// An iterator over the set bits in a [ExtWorkspaceHandleV1State].
///
/// You can construct this with the `IntoIterator` implementation of `ExtWorkspaceHandleV1State`.
#[derive(Clone, Debug)]
pub struct ExtWorkspaceHandleV1StateIter(pub u32);

impl ExtWorkspaceHandleV1State {
    /// the workspace is active
    pub const ACTIVE: Self = Self(1);

    /// the workspace requests attention
    pub const URGENT: Self = Self(2);

    /// the workspace is not visible
    ///
    /// The workspace is not visible in its workspace group, and clients
    /// attempting to visualize the compositor workspace state should not
    /// display such workspaces.
    pub const HIDDEN: Self = Self(4);
}

impl ExtWorkspaceHandleV1State {
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
        Self(0 | 1 | 2 | 4)
    }
}

impl Iterator for ExtWorkspaceHandleV1StateIter {
    type Item = ExtWorkspaceHandleV1State;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(ExtWorkspaceHandleV1State(bit))
    }
}

impl IntoIterator for ExtWorkspaceHandleV1State {
    type Item = ExtWorkspaceHandleV1State;
    type IntoIter = ExtWorkspaceHandleV1StateIter;

    fn into_iter(self) -> Self::IntoIter {
        ExtWorkspaceHandleV1StateIter(self.0)
    }
}

impl BitAnd for ExtWorkspaceHandleV1State {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for ExtWorkspaceHandleV1State {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for ExtWorkspaceHandleV1State {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for ExtWorkspaceHandleV1State {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for ExtWorkspaceHandleV1State {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for ExtWorkspaceHandleV1State {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for ExtWorkspaceHandleV1State {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for ExtWorkspaceHandleV1State {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for ExtWorkspaceHandleV1State {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for ExtWorkspaceHandleV1State {
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
            f.write_str("ACTIVE")?;
        }
        if v & 2 == 2 {
            v &= !2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("URGENT")?;
        }
        if v & 4 == 4 {
            v &= !4;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("HIDDEN")?;
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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct ExtWorkspaceHandleV1WorkspaceCapabilities(pub u32);

/// An iterator over the set bits in a [ExtWorkspaceHandleV1WorkspaceCapabilities].
///
/// You can construct this with the `IntoIterator` implementation of `ExtWorkspaceHandleV1WorkspaceCapabilities`.
#[derive(Clone, Debug)]
pub struct ExtWorkspaceHandleV1WorkspaceCapabilitiesIter(pub u32);

impl ExtWorkspaceHandleV1WorkspaceCapabilities {
    /// activate request is available
    pub const ACTIVATE: Self = Self(1);

    /// deactivate request is available
    pub const DEACTIVATE: Self = Self(2);

    /// remove request is available
    pub const REMOVE: Self = Self(4);

    /// assign request is available
    pub const ASSIGN: Self = Self(8);
}

impl ExtWorkspaceHandleV1WorkspaceCapabilities {
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
        Self(0 | 1 | 2 | 4 | 8)
    }
}

impl Iterator for ExtWorkspaceHandleV1WorkspaceCapabilitiesIter {
    type Item = ExtWorkspaceHandleV1WorkspaceCapabilities;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(ExtWorkspaceHandleV1WorkspaceCapabilities(bit))
    }
}

impl IntoIterator for ExtWorkspaceHandleV1WorkspaceCapabilities {
    type Item = ExtWorkspaceHandleV1WorkspaceCapabilities;
    type IntoIter = ExtWorkspaceHandleV1WorkspaceCapabilitiesIter;

    fn into_iter(self) -> Self::IntoIter {
        ExtWorkspaceHandleV1WorkspaceCapabilitiesIter(self.0)
    }
}

impl BitAnd for ExtWorkspaceHandleV1WorkspaceCapabilities {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for ExtWorkspaceHandleV1WorkspaceCapabilities {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for ExtWorkspaceHandleV1WorkspaceCapabilities {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for ExtWorkspaceHandleV1WorkspaceCapabilities {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for ExtWorkspaceHandleV1WorkspaceCapabilities {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for ExtWorkspaceHandleV1WorkspaceCapabilities {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for ExtWorkspaceHandleV1WorkspaceCapabilities {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for ExtWorkspaceHandleV1WorkspaceCapabilities {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for ExtWorkspaceHandleV1WorkspaceCapabilities {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for ExtWorkspaceHandleV1WorkspaceCapabilities {
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
            f.write_str("ACTIVATE")?;
        }
        if v & 2 == 2 {
            v &= !2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("DEACTIVATE")?;
        }
        if v & 4 == 4 {
            v &= !4;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("REMOVE")?;
        }
        if v & 8 == 8 {
            v &= !8;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("ASSIGN")?;
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
