//! a workspace handing a group of surfaces
//!
//! A zcosmic_workspace_handle_v2 object represents a a workspace that handles a
//! group of surfaces.
//!
//! Each workspace has a name, conveyed to the client with the name event; a
//! list of states, conveyed to the client with the state event; and
//! optionally a set of coordinates, conveyed to the client with the
//! coordinates event. The client may request that the compositor activate or
//! deactivate the workspace.
//!
//! Each workspace can belong to only a single workspace group.
//! Depepending on the compositor policy, there might be workspaces with
//! the same name in different workspace groups, but these workspaces are still
//! separate (e.g. one of them might be active while the other is not).

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zcosmic_workspace_handle_v2 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZcosmicWorkspaceHandleV2 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZcosmicWorkspaceHandleV2Handler>,
}

struct DefaultHandler;

impl ZcosmicWorkspaceHandleV2Handler for DefaultHandler { }

impl ConcreteObject for ZcosmicWorkspaceHandleV2 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::ZcosmicWorkspaceHandleV2;
    const INTERFACE_NAME: &str = "zcosmic_workspace_handle_v2";
}

impl ZcosmicWorkspaceHandleV2 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZcosmicWorkspaceHandleV2Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZcosmicWorkspaceHandleV2Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZcosmicWorkspaceHandleV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZcosmicWorkspaceHandleV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZcosmicWorkspaceHandleV2 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the zcosmic_workspace_handle_v1 object
    ///
    /// This request should be called either when the client will no longer
    /// use the `zcosmic_workspace_handle_v1`.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_workspace_handle_v2#{}.destroy()\n", id);
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

    /// destroy the zcosmic_workspace_handle_v1 object
    ///
    /// This request should be called either when the client will no longer
    /// use the `zcosmic_workspace_handle_v1`.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zcosmic_workspace_handle_v2.destroy", &e);
        }
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
    /// Compositors must send this event once after creation of a
    /// `zcosmic_workspace_handle_v2`. When the capabilities change, compositors
    /// must send this event again.
    ///
    /// # Arguments
    ///
    /// - `capabilities`: capabilities
    #[inline]
    pub fn try_send_capabilities(
        &self,
        capabilities: ZcosmicWorkspaceHandleV2WorkspaceCapabilities,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            capabilities,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: ZcosmicWorkspaceHandleV2WorkspaceCapabilities) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zcosmic_workspace_handle_v2#{}.capabilities(capabilities: {:?})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
    /// Compositors must send this event once after creation of a
    /// `zcosmic_workspace_handle_v2`. When the capabilities change, compositors
    /// must send this event again.
    ///
    /// # Arguments
    ///
    /// - `capabilities`: capabilities
    #[inline]
    pub fn send_capabilities(
        &self,
        capabilities: ZcosmicWorkspaceHandleV2WorkspaceCapabilities,
    ) {
        let res = self.try_send_capabilities(
            capabilities,
        );
        if let Err(e) = res {
            log_send("zcosmic_workspace_handle_v2.capabilities", &e);
        }
    }

    /// Since when the tiling_state message is available.
    pub const MSG__TILING_STATE__SINCE: u32 = 1;

    /// indicates if tiling behavior is enabled for this workspace
    ///
    /// This event is emitted immediately after the zcosmic_workspace_handle_v2 is created
    /// and each time the workspace tiling state changes, either because of a
    /// compositor action or because of a request in this protocol.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_tiling_state(
        &self,
        state: ZcosmicWorkspaceHandleV2TilingState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            state,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: ZcosmicWorkspaceHandleV2TilingState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zcosmic_workspace_handle_v2#{}.tiling_state(state: {:?})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// indicates if tiling behavior is enabled for this workspace
    ///
    /// This event is emitted immediately after the zcosmic_workspace_handle_v2 is created
    /// and each time the workspace tiling state changes, either because of a
    /// compositor action or because of a request in this protocol.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_tiling_state(
        &self,
        state: ZcosmicWorkspaceHandleV2TilingState,
    ) {
        let res = self.try_send_tiling_state(
            state,
        );
        if let Err(e) = res {
            log_send("zcosmic_workspace_handle_v2.tiling_state", &e);
        }
    }

    /// Since when the state message is available.
    pub const MSG__STATE__SINCE: u32 = 2;

    /// the state of the workspace changed
    ///
    /// This event is emitted immediately after the zcosmic_workspace_handle_v2 is
    /// created and each time the workspace state changes, either because of a
    /// compositor action or because of a request in this protocol.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_state(
        &self,
        state: ZcosmicWorkspaceHandleV2State,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            state,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: ZcosmicWorkspaceHandleV2State) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zcosmic_workspace_handle_v2#{}.state(state: {:?})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// the state of the workspace changed
    ///
    /// This event is emitted immediately after the zcosmic_workspace_handle_v2 is
    /// created and each time the workspace state changes, either because of a
    /// compositor action or because of a request in this protocol.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_state(
        &self,
        state: ZcosmicWorkspaceHandleV2State,
    ) {
        let res = self.try_send_state(
            state,
        );
        if let Err(e) = res {
            log_send("zcosmic_workspace_handle_v2.state", &e);
        }
    }

    /// Since when the rename message is available.
    pub const MSG__RENAME__SINCE: u32 = 1;

    /// rename this workspace
    ///
    /// Request that this workspace is renamed.
    ///
    /// There is no guarantee the workspace will actually be renamed.
    ///
    /// # Arguments
    ///
    /// - `name`: new name of the workspace
    #[inline]
    pub fn try_send_rename(
        &self,
        name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            name,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_workspace_handle_v2#{}.rename(name: {:?})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// rename this workspace
    ///
    /// Request that this workspace is renamed.
    ///
    /// There is no guarantee the workspace will actually be renamed.
    ///
    /// # Arguments
    ///
    /// - `name`: new name of the workspace
    #[inline]
    pub fn send_rename(
        &self,
        name: &str,
    ) {
        let res = self.try_send_rename(
            name,
        );
        if let Err(e) = res {
            log_send("zcosmic_workspace_handle_v2.rename", &e);
        }
    }

    /// Since when the set_tiling_state message is available.
    pub const MSG__SET_TILING_STATE__SINCE: u32 = 1;

    /// change the tiling state of this workspace
    ///
    /// Request that this workspace's tiling state is changed.
    ///
    /// There is no guarantee the workspace will actually change it's tiling state.
    ///
    /// # Arguments
    ///
    /// - `state`: the new tiling state of the workspace
    #[inline]
    pub fn try_send_set_tiling_state(
        &self,
        state: ZcosmicWorkspaceHandleV2TilingState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            state,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: ZcosmicWorkspaceHandleV2TilingState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_workspace_handle_v2#{}.set_tiling_state(state: {:?})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            2,
            arg0.0,
        ]);
        Ok(())
    }

    /// change the tiling state of this workspace
    ///
    /// Request that this workspace's tiling state is changed.
    ///
    /// There is no guarantee the workspace will actually change it's tiling state.
    ///
    /// # Arguments
    ///
    /// - `state`: the new tiling state of the workspace
    #[inline]
    pub fn send_set_tiling_state(
        &self,
        state: ZcosmicWorkspaceHandleV2TilingState,
    ) {
        let res = self.try_send_set_tiling_state(
            state,
        );
        if let Err(e) = res {
            log_send("zcosmic_workspace_handle_v2.set_tiling_state", &e);
        }
    }

    /// Since when the move_before message is available.
    pub const MSG__MOVE_BEFORE__SINCE: u32 = 2;

    /// move before a different workspace
    ///
    /// Move a workspace to be before another workspace along a given axis.
    ///
    /// `other_workspace` may be on the same workspace group, or on a different group.
    /// If it's a different set, the workspace will also be moved to that group.
    ///
    /// `axis` should be a valid index in the coordinates on the workspace group
    /// `other_workspace` is on. The workspace will be positioned on the target group
    /// to have a coordinate with this component less than the value of the component for
    /// `other_workspace`. The exact coordinate values, or how other workspaces are moved
    /// to accommodate the workspace, is unspecified.
    ///
    /// The request will be ignored if `axis` is invalid or the compositor is otherwise
    /// unable to move the workspace.
    ///
    /// There is no guarantee the workspace will actually be moved.
    ///
    /// # Arguments
    ///
    /// - `other_workspace`:
    /// - `axis`:
    #[inline]
    pub fn try_send_move_before(
        &self,
        other_workspace: &Rc<ExtWorkspaceHandleV1>,
        axis: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            other_workspace,
            axis,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("other_workspace"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_workspace_handle_v2#{}.move_before(other_workspace: ext_workspace_handle_v1#{}, axis: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            3,
            arg0_id,
            arg1,
        ]);
        Ok(())
    }

    /// move before a different workspace
    ///
    /// Move a workspace to be before another workspace along a given axis.
    ///
    /// `other_workspace` may be on the same workspace group, or on a different group.
    /// If it's a different set, the workspace will also be moved to that group.
    ///
    /// `axis` should be a valid index in the coordinates on the workspace group
    /// `other_workspace` is on. The workspace will be positioned on the target group
    /// to have a coordinate with this component less than the value of the component for
    /// `other_workspace`. The exact coordinate values, or how other workspaces are moved
    /// to accommodate the workspace, is unspecified.
    ///
    /// The request will be ignored if `axis` is invalid or the compositor is otherwise
    /// unable to move the workspace.
    ///
    /// There is no guarantee the workspace will actually be moved.
    ///
    /// # Arguments
    ///
    /// - `other_workspace`:
    /// - `axis`:
    #[inline]
    pub fn send_move_before(
        &self,
        other_workspace: &Rc<ExtWorkspaceHandleV1>,
        axis: u32,
    ) {
        let res = self.try_send_move_before(
            other_workspace,
            axis,
        );
        if let Err(e) = res {
            log_send("zcosmic_workspace_handle_v2.move_before", &e);
        }
    }

    /// Since when the move_after message is available.
    pub const MSG__MOVE_AFTER__SINCE: u32 = 2;

    /// move after a different workspace
    ///
    /// Move a workspace to be after another workspace along a given axis.
    ///
    /// See `move_before`.
    ///
    /// # Arguments
    ///
    /// - `other_workspace`:
    /// - `axis`:
    #[inline]
    pub fn try_send_move_after(
        &self,
        other_workspace: &Rc<ExtWorkspaceHandleV1>,
        axis: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            other_workspace,
            axis,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("other_workspace"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_workspace_handle_v2#{}.move_after(other_workspace: ext_workspace_handle_v1#{}, axis: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            4,
            arg0_id,
            arg1,
        ]);
        Ok(())
    }

    /// move after a different workspace
    ///
    /// Move a workspace to be after another workspace along a given axis.
    ///
    /// See `move_before`.
    ///
    /// # Arguments
    ///
    /// - `other_workspace`:
    /// - `axis`:
    #[inline]
    pub fn send_move_after(
        &self,
        other_workspace: &Rc<ExtWorkspaceHandleV1>,
        axis: u32,
    ) {
        let res = self.try_send_move_after(
            other_workspace,
            axis,
        );
        if let Err(e) = res {
            log_send("zcosmic_workspace_handle_v2.move_after", &e);
        }
    }

    /// Since when the pin message is available.
    pub const MSG__PIN__SINCE: u32 = 2;

    /// pin the workspace
    ///
    /// Request that this workspace be pinned.
    ///
    /// There is no guarantee the workspace will be actually pinned.
    #[inline]
    pub fn try_send_pin(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_workspace_handle_v2#{}.pin()\n", id);
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
            5,
        ]);
        Ok(())
    }

    /// pin the workspace
    ///
    /// Request that this workspace be pinned.
    ///
    /// There is no guarantee the workspace will be actually pinned.
    #[inline]
    pub fn send_pin(
        &self,
    ) {
        let res = self.try_send_pin(
        );
        if let Err(e) = res {
            log_send("zcosmic_workspace_handle_v2.pin", &e);
        }
    }

    /// Since when the unpin message is available.
    pub const MSG__UNPIN__SINCE: u32 = 2;

    /// pin the workspace
    ///
    /// Request that this workspace be unpinned.
    ///
    /// There is no guarantee the workspace will be actually unpinned.
    #[inline]
    pub fn try_send_unpin(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_workspace_handle_v2#{}.unpin()\n", id);
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
            6,
        ]);
        Ok(())
    }

    /// pin the workspace
    ///
    /// Request that this workspace be unpinned.
    ///
    /// There is no guarantee the workspace will be actually unpinned.
    #[inline]
    pub fn send_unpin(
        &self,
    ) {
        let res = self.try_send_unpin(
        );
        if let Err(e) = res {
            log_send("zcosmic_workspace_handle_v2.unpin", &e);
        }
    }
}

/// A message handler for [`ZcosmicWorkspaceHandleV2`] proxies.
pub trait ZcosmicWorkspaceHandleV2Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZcosmicWorkspaceHandleV2>) {
        slf.core.delete_id();
    }

    /// destroy the zcosmic_workspace_handle_v1 object
    ///
    /// This request should be called either when the client will no longer
    /// use the `zcosmic_workspace_handle_v1`.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZcosmicWorkspaceHandleV2>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zcosmic_workspace_handle_v2.destroy", &e);
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
    /// Compositors must send this event once after creation of a
    /// `zcosmic_workspace_handle_v2`. When the capabilities change, compositors
    /// must send this event again.
    ///
    /// # Arguments
    ///
    /// - `capabilities`: capabilities
    #[inline]
    fn handle_capabilities(
        &mut self,
        slf: &Rc<ZcosmicWorkspaceHandleV2>,
        capabilities: ZcosmicWorkspaceHandleV2WorkspaceCapabilities,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_capabilities(
            capabilities,
        );
        if let Err(e) = res {
            log_forward("zcosmic_workspace_handle_v2.capabilities", &e);
        }
    }

    /// indicates if tiling behavior is enabled for this workspace
    ///
    /// This event is emitted immediately after the zcosmic_workspace_handle_v2 is created
    /// and each time the workspace tiling state changes, either because of a
    /// compositor action or because of a request in this protocol.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_tiling_state(
        &mut self,
        slf: &Rc<ZcosmicWorkspaceHandleV2>,
        state: ZcosmicWorkspaceHandleV2TilingState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_tiling_state(
            state,
        );
        if let Err(e) = res {
            log_forward("zcosmic_workspace_handle_v2.tiling_state", &e);
        }
    }

    /// the state of the workspace changed
    ///
    /// This event is emitted immediately after the zcosmic_workspace_handle_v2 is
    /// created and each time the workspace state changes, either because of a
    /// compositor action or because of a request in this protocol.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_state(
        &mut self,
        slf: &Rc<ZcosmicWorkspaceHandleV2>,
        state: ZcosmicWorkspaceHandleV2State,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_state(
            state,
        );
        if let Err(e) = res {
            log_forward("zcosmic_workspace_handle_v2.state", &e);
        }
    }

    /// rename this workspace
    ///
    /// Request that this workspace is renamed.
    ///
    /// There is no guarantee the workspace will actually be renamed.
    ///
    /// # Arguments
    ///
    /// - `name`: new name of the workspace
    #[inline]
    fn handle_rename(
        &mut self,
        slf: &Rc<ZcosmicWorkspaceHandleV2>,
        name: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_rename(
            name,
        );
        if let Err(e) = res {
            log_forward("zcosmic_workspace_handle_v2.rename", &e);
        }
    }

    /// change the tiling state of this workspace
    ///
    /// Request that this workspace's tiling state is changed.
    ///
    /// There is no guarantee the workspace will actually change it's tiling state.
    ///
    /// # Arguments
    ///
    /// - `state`: the new tiling state of the workspace
    #[inline]
    fn handle_set_tiling_state(
        &mut self,
        slf: &Rc<ZcosmicWorkspaceHandleV2>,
        state: ZcosmicWorkspaceHandleV2TilingState,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_tiling_state(
            state,
        );
        if let Err(e) = res {
            log_forward("zcosmic_workspace_handle_v2.set_tiling_state", &e);
        }
    }

    /// move before a different workspace
    ///
    /// Move a workspace to be before another workspace along a given axis.
    ///
    /// `other_workspace` may be on the same workspace group, or on a different group.
    /// If it's a different set, the workspace will also be moved to that group.
    ///
    /// `axis` should be a valid index in the coordinates on the workspace group
    /// `other_workspace` is on. The workspace will be positioned on the target group
    /// to have a coordinate with this component less than the value of the component for
    /// `other_workspace`. The exact coordinate values, or how other workspaces are moved
    /// to accommodate the workspace, is unspecified.
    ///
    /// The request will be ignored if `axis` is invalid or the compositor is otherwise
    /// unable to move the workspace.
    ///
    /// There is no guarantee the workspace will actually be moved.
    ///
    /// # Arguments
    ///
    /// - `other_workspace`:
    /// - `axis`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_move_before(
        &mut self,
        slf: &Rc<ZcosmicWorkspaceHandleV2>,
        other_workspace: &Rc<ExtWorkspaceHandleV1>,
        axis: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_move_before(
            other_workspace,
            axis,
        );
        if let Err(e) = res {
            log_forward("zcosmic_workspace_handle_v2.move_before", &e);
        }
    }

    /// move after a different workspace
    ///
    /// Move a workspace to be after another workspace along a given axis.
    ///
    /// See `move_before`.
    ///
    /// # Arguments
    ///
    /// - `other_workspace`:
    /// - `axis`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_move_after(
        &mut self,
        slf: &Rc<ZcosmicWorkspaceHandleV2>,
        other_workspace: &Rc<ExtWorkspaceHandleV1>,
        axis: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_move_after(
            other_workspace,
            axis,
        );
        if let Err(e) = res {
            log_forward("zcosmic_workspace_handle_v2.move_after", &e);
        }
    }

    /// pin the workspace
    ///
    /// Request that this workspace be pinned.
    ///
    /// There is no guarantee the workspace will be actually pinned.
    #[inline]
    fn handle_pin(
        &mut self,
        slf: &Rc<ZcosmicWorkspaceHandleV2>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_pin(
        );
        if let Err(e) = res {
            log_forward("zcosmic_workspace_handle_v2.pin", &e);
        }
    }

    /// pin the workspace
    ///
    /// Request that this workspace be unpinned.
    ///
    /// There is no guarantee the workspace will be actually unpinned.
    #[inline]
    fn handle_unpin(
        &mut self,
        slf: &Rc<ZcosmicWorkspaceHandleV2>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_unpin(
        );
        if let Err(e) = res {
            log_forward("zcosmic_workspace_handle_v2.unpin", &e);
        }
    }
}

impl ObjectPrivate for ZcosmicWorkspaceHandleV2 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZcosmicWorkspaceHandleV2, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_workspace_handle_v2#{}.destroy()\n", client_id, id);
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
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "name")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_workspace_handle_v2#{}.rename(name: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_rename(&self, arg0);
                } else {
                    DefaultHandler.handle_rename(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = ZcosmicWorkspaceHandleV2TilingState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: ZcosmicWorkspaceHandleV2TilingState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_workspace_handle_v2#{}.set_tiling_state(state: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_tiling_state(&self, arg0);
                } else {
                    DefaultHandler.handle_set_tiling_state(&self, arg0);
                }
            }
            3 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_workspace_handle_v2#{}.move_before(other_workspace: ext_workspace_handle_v1#{}, axis: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ExtWorkspaceHandleV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("other_workspace", o.core().interface, ObjectInterface::ExtWorkspaceHandleV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_move_before(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_move_before(&self, arg0, arg1);
                }
            }
            4 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_workspace_handle_v2#{}.move_after(other_workspace: ext_workspace_handle_v1#{}, axis: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ExtWorkspaceHandleV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("other_workspace", o.core().interface, ObjectInterface::ExtWorkspaceHandleV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_move_after(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_move_after(&self, arg0, arg1);
                }
            }
            5 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_workspace_handle_v2#{}.pin()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_pin(&self);
                } else {
                    DefaultHandler.handle_pin(&self);
                }
            }
            6 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_workspace_handle_v2#{}.unpin()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_unpin(&self);
                } else {
                    DefaultHandler.handle_unpin(&self);
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
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = ZcosmicWorkspaceHandleV2WorkspaceCapabilities(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: ZcosmicWorkspaceHandleV2WorkspaceCapabilities) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zcosmic_workspace_handle_v2#{}.capabilities(capabilities: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_capabilities(&self, arg0);
                } else {
                    DefaultHandler.handle_capabilities(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = ZcosmicWorkspaceHandleV2TilingState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: ZcosmicWorkspaceHandleV2TilingState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zcosmic_workspace_handle_v2#{}.tiling_state(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_tiling_state(&self, arg0);
                } else {
                    DefaultHandler.handle_tiling_state(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = ZcosmicWorkspaceHandleV2State(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: ZcosmicWorkspaceHandleV2State) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zcosmic_workspace_handle_v2#{}.state(state: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_state(&self, arg0);
                } else {
                    DefaultHandler.handle_state(&self, arg0);
                }
            }
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
        Ok(())
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            1 => "rename",
            2 => "set_tiling_state",
            3 => "move_before",
            4 => "move_after",
            5 => "pin",
            6 => "unpin",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "capabilities",
            1 => "tiling_state",
            2 => "state",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZcosmicWorkspaceHandleV2 {
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

impl ZcosmicWorkspaceHandleV2 {
    /// Since when the workspace_capabilities.rename enum variant is available.
    pub const ENM__WORKSPACE_CAPABILITIES_RENAME__SINCE: u32 = 1;
    /// Since when the workspace_capabilities.set_tiling_state enum variant is available.
    pub const ENM__WORKSPACE_CAPABILITIES_SET_TILING_STATE__SINCE: u32 = 1;
    /// Since when the workspace_capabilities.pin enum variant is available.
    pub const ENM__WORKSPACE_CAPABILITIES_PIN__SINCE: u32 = 2;
    /// Since when the workspace_capabilities.move enum variant is available.
    pub const ENM__WORKSPACE_CAPABILITIES_MOVE__SINCE: u32 = 2;

    /// Since when the tiling_state.floating_only enum variant is available.
    pub const ENM__TILING_STATE_FLOATING_ONLY__SINCE: u32 = 1;
    /// Since when the tiling_state.tiling_enabled enum variant is available.
    pub const ENM__TILING_STATE_TILING_ENABLED__SINCE: u32 = 1;

    /// Since when the state.pinned enum variant is available.
    pub const ENM__STATE_PINNED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct ZcosmicWorkspaceHandleV2WorkspaceCapabilities(pub u32);

/// An iterator over the set bits in a [`ZcosmicWorkspaceHandleV2WorkspaceCapabilities`].
///
/// You can construct this with the `IntoIterator` implementation of `ZcosmicWorkspaceHandleV2WorkspaceCapabilities`.
#[derive(Clone, Debug)]
pub struct ZcosmicWorkspaceHandleV2WorkspaceCapabilitiesIter(pub u32);

impl ZcosmicWorkspaceHandleV2WorkspaceCapabilities {
    /// rename request is available
    pub const RENAME: Self = Self(1);

    /// set_tiling_state request is available
    pub const SET_TILING_STATE: Self = Self(2);

    /// pin and unpin requests are available
    pub const PIN: Self = Self(3);

    /// move_before and move_after requests are available
    pub const MOVE: Self = Self(4);
}

impl ZcosmicWorkspaceHandleV2WorkspaceCapabilities {
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
        Self(0 | 1 | 2 | 3 | 4)
    }
}

impl Iterator for ZcosmicWorkspaceHandleV2WorkspaceCapabilitiesIter {
    type Item = ZcosmicWorkspaceHandleV2WorkspaceCapabilities;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(ZcosmicWorkspaceHandleV2WorkspaceCapabilities(bit))
    }
}

impl IntoIterator for ZcosmicWorkspaceHandleV2WorkspaceCapabilities {
    type Item = ZcosmicWorkspaceHandleV2WorkspaceCapabilities;
    type IntoIter = ZcosmicWorkspaceHandleV2WorkspaceCapabilitiesIter;

    fn into_iter(self) -> Self::IntoIter {
        ZcosmicWorkspaceHandleV2WorkspaceCapabilitiesIter(self.0)
    }
}

impl BitAnd for ZcosmicWorkspaceHandleV2WorkspaceCapabilities {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for ZcosmicWorkspaceHandleV2WorkspaceCapabilities {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for ZcosmicWorkspaceHandleV2WorkspaceCapabilities {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for ZcosmicWorkspaceHandleV2WorkspaceCapabilities {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for ZcosmicWorkspaceHandleV2WorkspaceCapabilities {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for ZcosmicWorkspaceHandleV2WorkspaceCapabilities {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for ZcosmicWorkspaceHandleV2WorkspaceCapabilities {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for ZcosmicWorkspaceHandleV2WorkspaceCapabilities {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for ZcosmicWorkspaceHandleV2WorkspaceCapabilities {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for ZcosmicWorkspaceHandleV2WorkspaceCapabilities {
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
            f.write_str("RENAME")?;
        }
        if v & 2 == 2 {
            v &= !2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("SET_TILING_STATE")?;
        }
        if v & 3 == 3 {
            v &= !3;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("PIN")?;
        }
        if v & 4 == 4 {
            v &= !4;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("MOVE")?;
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

/// types of tiling state a workspace may have
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZcosmicWorkspaceHandleV2TilingState(pub u32);

impl ZcosmicWorkspaceHandleV2TilingState {
    /// The workspace has no active tiling properties
    pub const FLOATING_ONLY: Self = Self(0);

    /// Tiling behavior is enabled for the workspace
    pub const TILING_ENABLED: Self = Self(1);
}

impl Debug for ZcosmicWorkspaceHandleV2TilingState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::FLOATING_ONLY => "FLOATING_ONLY",
            Self::TILING_ENABLED => "TILING_ENABLED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// state of the workspace, extending the ext workspace state
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct ZcosmicWorkspaceHandleV2State(pub u32);

/// An iterator over the set bits in a [`ZcosmicWorkspaceHandleV2State`].
///
/// You can construct this with the `IntoIterator` implementation of `ZcosmicWorkspaceHandleV2State`.
#[derive(Clone, Debug)]
pub struct ZcosmicWorkspaceHandleV2StateIter(pub u32);

impl ZcosmicWorkspaceHandleV2State {
    /// the workspace is pinned
    pub const PINNED: Self = Self(1);
}

impl ZcosmicWorkspaceHandleV2State {
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

impl Iterator for ZcosmicWorkspaceHandleV2StateIter {
    type Item = ZcosmicWorkspaceHandleV2State;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(ZcosmicWorkspaceHandleV2State(bit))
    }
}

impl IntoIterator for ZcosmicWorkspaceHandleV2State {
    type Item = ZcosmicWorkspaceHandleV2State;
    type IntoIter = ZcosmicWorkspaceHandleV2StateIter;

    fn into_iter(self) -> Self::IntoIter {
        ZcosmicWorkspaceHandleV2StateIter(self.0)
    }
}

impl BitAnd for ZcosmicWorkspaceHandleV2State {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for ZcosmicWorkspaceHandleV2State {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for ZcosmicWorkspaceHandleV2State {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for ZcosmicWorkspaceHandleV2State {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for ZcosmicWorkspaceHandleV2State {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for ZcosmicWorkspaceHandleV2State {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for ZcosmicWorkspaceHandleV2State {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for ZcosmicWorkspaceHandleV2State {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for ZcosmicWorkspaceHandleV2State {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for ZcosmicWorkspaceHandleV2State {
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
            f.write_str("PINNED")?;
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
