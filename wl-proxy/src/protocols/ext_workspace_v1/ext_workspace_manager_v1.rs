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

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A ext_workspace_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ExtWorkspaceManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ExtWorkspaceManagerV1Handler>,
}

struct DefaultHandler;

impl ExtWorkspaceManagerV1Handler for DefaultHandler { }

impl ConcreteObject for ExtWorkspaceManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ExtWorkspaceManagerV1;
    const INTERFACE_NAME: &str = "ext_workspace_manager_v1";
}

impl ExtWorkspaceManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ExtWorkspaceManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
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
    ///
    /// # Arguments
    ///
    /// - `workspace_group`:
    #[inline]
    pub fn try_send_workspace_group(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateClientId("workspace_group", e)))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_workspace_manager_v1#{}.workspace_group(workspace_group: ext_workspace_group_handle_v1#{})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0_id);
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
    pub fn send_workspace_group(
        &self,
        workspace_group: &Rc<ExtWorkspaceGroupHandleV1>,
    ) {
        let res = self.try_send_workspace_group(
            workspace_group,
        );
        if let Err(e) = res {
            log_send("ext_workspace_manager_v1.workspace_group", &e);
        }
    }

    /// a workspace group has been created
    ///
    /// This event is emitted whenever a new workspace group has been created.
    ///
    /// All initial details of the workspace group (outputs) will be
    /// sent immediately after this event via the corresponding events in
    /// ext_workspace_group_handle_v1 and ext_workspace_handle_v1.
    #[inline]
    pub fn new_try_send_workspace_group(
        &self,
    ) -> Result<Rc<ExtWorkspaceGroupHandleV1>, ObjectError> {
        let workspace_group = self.core.create_child();
        self.try_send_workspace_group(
            &workspace_group,
        )?;
        Ok(workspace_group)
    }

    /// a workspace group has been created
    ///
    /// This event is emitted whenever a new workspace group has been created.
    ///
    /// All initial details of the workspace group (outputs) will be
    /// sent immediately after this event via the corresponding events in
    /// ext_workspace_group_handle_v1 and ext_workspace_handle_v1.
    #[inline]
    pub fn new_send_workspace_group(
        &self,
    ) -> Rc<ExtWorkspaceGroupHandleV1> {
        let workspace_group = self.core.create_child();
        self.send_workspace_group(
            &workspace_group,
        );
        workspace_group
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
    ///
    /// # Arguments
    ///
    /// - `workspace`:
    #[inline]
    pub fn try_send_workspace(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateClientId("workspace", e)))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_workspace_manager_v1#{}.workspace(workspace: ext_workspace_handle_v1#{})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0_id);
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
    pub fn send_workspace(
        &self,
        workspace: &Rc<ExtWorkspaceHandleV1>,
    ) {
        let res = self.try_send_workspace(
            workspace,
        );
        if let Err(e) = res {
            log_send("ext_workspace_manager_v1.workspace", &e);
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
    #[inline]
    pub fn new_try_send_workspace(
        &self,
    ) -> Result<Rc<ExtWorkspaceHandleV1>, ObjectError> {
        let workspace = self.core.create_child();
        self.try_send_workspace(
            &workspace,
        )?;
        Ok(workspace)
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
    #[inline]
    pub fn new_send_workspace(
        &self,
    ) -> Rc<ExtWorkspaceHandleV1> {
        let workspace = self.core.create_child();
        self.send_workspace(
            &workspace,
        );
        workspace
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
    pub fn try_send_commit(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_workspace_manager_v1#{}.commit()\n", id);
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
        Ok(())
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
    pub fn send_commit(
        &self,
    ) {
        let res = self.try_send_commit(
        );
        if let Err(e) = res {
            log_send("ext_workspace_manager_v1.commit", &e);
        }
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
    pub fn try_send_done(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_workspace_manager_v1#{}.done()\n", client_id, id);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id);
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
    ) {
        let res = self.try_send_done(
        );
        if let Err(e) = res {
            log_send("ext_workspace_manager_v1.done", &e);
        }
    }

    /// Since when the finished message is available.
    pub const MSG__FINISHED__SINCE: u32 = 1;

    /// the compositor has finished with the workspace_manager
    ///
    /// This event indicates that the compositor is done sending events to the
    /// ext_workspace_manager_v1. The server will destroy the object
    /// immediately after sending this request.
    #[inline]
    pub fn try_send_finished(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_workspace_manager_v1#{}.finished()\n", client_id, id);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id);
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

    /// the compositor has finished with the workspace_manager
    ///
    /// This event indicates that the compositor is done sending events to the
    /// ext_workspace_manager_v1. The server will destroy the object
    /// immediately after sending this request.
    #[inline]
    pub fn send_finished(
        &self,
    ) {
        let res = self.try_send_finished(
        );
        if let Err(e) = res {
            log_send("ext_workspace_manager_v1.finished", &e);
        }
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
    pub fn try_send_stop(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_workspace_manager_v1#{}.stop()\n", id);
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
            1,
        ]);
        Ok(())
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
    pub fn send_stop(
        &self,
    ) {
        let res = self.try_send_stop(
        );
        if let Err(e) = res {
            log_send("ext_workspace_manager_v1.stop", &e);
        }
    }
}

/// A message handler for [`ExtWorkspaceManagerV1`] proxies.
pub trait ExtWorkspaceManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ExtWorkspaceManagerV1>) {
        slf.core.delete_id();
    }

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
    fn handle_workspace_group(
        &mut self,
        slf: &Rc<ExtWorkspaceManagerV1>,
        workspace_group: &Rc<ExtWorkspaceGroupHandleV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_workspace_group(
            workspace_group,
        );
        if let Err(e) = res {
            log_forward("ext_workspace_manager_v1.workspace_group", &e);
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
    fn handle_workspace(
        &mut self,
        slf: &Rc<ExtWorkspaceManagerV1>,
        workspace: &Rc<ExtWorkspaceHandleV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_workspace(
            workspace,
        );
        if let Err(e) = res {
            log_forward("ext_workspace_manager_v1.workspace", &e);
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
    fn handle_commit(
        &mut self,
        slf: &Rc<ExtWorkspaceManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_commit(
        );
        if let Err(e) = res {
            log_forward("ext_workspace_manager_v1.commit", &e);
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
    fn handle_done(
        &mut self,
        slf: &Rc<ExtWorkspaceManagerV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_done(
        );
        if let Err(e) = res {
            log_forward("ext_workspace_manager_v1.done", &e);
        }
    }

    /// the compositor has finished with the workspace_manager
    ///
    /// This event indicates that the compositor is done sending events to the
    /// ext_workspace_manager_v1. The server will destroy the object
    /// immediately after sending this request.
    #[inline]
    fn handle_finished(
        &mut self,
        slf: &Rc<ExtWorkspaceManagerV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_finished(
        );
        if let Err(e) = res {
            log_forward("ext_workspace_manager_v1.finished", &e);
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
    fn handle_stop(
        &mut self,
        slf: &Rc<ExtWorkspaceManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_stop(
        );
        if let Err(e) = res {
            log_forward("ext_workspace_manager_v1.stop", &e);
        }
    }
}

impl ObjectPrivate for ExtWorkspaceManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ExtWorkspaceManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_workspace_manager_v1#{}.commit()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_commit(&self);
                } else {
                    DefaultHandler.handle_commit(&self);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_workspace_manager_v1#{}.stop()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_stop(&self);
                } else {
                    DefaultHandler.handle_stop(&self);
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
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_workspace_manager_v1#{}.workspace_group(workspace_group: ext_workspace_group_handle_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = ExtWorkspaceGroupHandleV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetServerId(arg0_id, "workspace_group", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_workspace_group(&self, arg0);
                } else {
                    DefaultHandler.handle_workspace_group(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_workspace_manager_v1#{}.workspace(workspace: ext_workspace_handle_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = ExtWorkspaceHandleV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetServerId(arg0_id, "workspace", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_workspace(&self, arg0);
                } else {
                    DefaultHandler.handle_workspace(&self, arg0);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_workspace_manager_v1#{}.done()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_done(&self);
                } else {
                    DefaultHandler.handle_done(&self);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_workspace_manager_v1#{}.finished()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                self.core.handle_server_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_finished(&self);
                } else {
                    DefaultHandler.handle_finished(&self);
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

impl Object for ExtWorkspaceManagerV1 {
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

