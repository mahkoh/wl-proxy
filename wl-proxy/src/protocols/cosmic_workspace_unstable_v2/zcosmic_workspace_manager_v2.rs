//! list and control workspaces
//!
//! This protocol extends `ext-workspace-v1` with addtional requests and events.
//!
//! The caller should call `get_cosmic_workspace` whenever a new ext workspace is
//! created.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zcosmic_workspace_manager_v2 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZcosmicWorkspaceManagerV2 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZcosmicWorkspaceManagerV2Handler>,
}

struct DefaultHandler;

impl ZcosmicWorkspaceManagerV2Handler for DefaultHandler { }

impl ConcreteObject for ZcosmicWorkspaceManagerV2 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::ZcosmicWorkspaceManagerV2;
    const INTERFACE_NAME: &str = "zcosmic_workspace_manager_v2";
}

impl ZcosmicWorkspaceManagerV2 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZcosmicWorkspaceManagerV2Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZcosmicWorkspaceManagerV2Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZcosmicWorkspaceManagerV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZcosmicWorkspaceManagerV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZcosmicWorkspaceManagerV2 {
    /// Since when the get_cosmic_workspace message is available.
    pub const MSG__GET_COSMIC_WORKSPACE__SINCE: u32 = 2;

    /// get cosmic workspace extension object
    ///
    /// Request a `zcosmic_workspace_handle_v2` extension object for an existing
    /// `ext_workspace_handle_v1`.
    ///
    /// If a `zcosmic_workspace_handle_v2` already exists for the `ext_workspace_handle_v1`, this
    /// will raise a `workspace_exists` protocol error.
    ///
    /// # Arguments
    ///
    /// - `cosmic_workspace`:
    /// - `workspace`:
    #[inline]
    pub fn try_send_get_cosmic_workspace(
        &self,
        cosmic_workspace: &Rc<ZcosmicWorkspaceHandleV2>,
        workspace: &Rc<ExtWorkspaceHandleV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            cosmic_workspace,
            workspace,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("workspace"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("cosmic_workspace", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_workspace_manager_v2#{}.get_cosmic_workspace(cosmic_workspace: zcosmic_workspace_handle_v2#{}, workspace: ext_workspace_handle_v1#{})\n", id, arg0, arg1);
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
            0,
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// get cosmic workspace extension object
    ///
    /// Request a `zcosmic_workspace_handle_v2` extension object for an existing
    /// `ext_workspace_handle_v1`.
    ///
    /// If a `zcosmic_workspace_handle_v2` already exists for the `ext_workspace_handle_v1`, this
    /// will raise a `workspace_exists` protocol error.
    ///
    /// # Arguments
    ///
    /// - `cosmic_workspace`:
    /// - `workspace`:
    #[inline]
    pub fn send_get_cosmic_workspace(
        &self,
        cosmic_workspace: &Rc<ZcosmicWorkspaceHandleV2>,
        workspace: &Rc<ExtWorkspaceHandleV1>,
    ) {
        let res = self.try_send_get_cosmic_workspace(
            cosmic_workspace,
            workspace,
        );
        if let Err(e) = res {
            log_send("zcosmic_workspace_manager_v2.get_cosmic_workspace", &e);
        }
    }

    /// get cosmic workspace extension object
    ///
    /// Request a `zcosmic_workspace_handle_v2` extension object for an existing
    /// `ext_workspace_handle_v1`.
    ///
    /// If a `zcosmic_workspace_handle_v2` already exists for the `ext_workspace_handle_v1`, this
    /// will raise a `workspace_exists` protocol error.
    ///
    /// # Arguments
    ///
    /// - `workspace`:
    #[inline]
    pub fn new_try_send_get_cosmic_workspace(
        &self,
        workspace: &Rc<ExtWorkspaceHandleV1>,
    ) -> Result<Rc<ZcosmicWorkspaceHandleV2>, ObjectError> {
        let cosmic_workspace = self.core.create_child();
        self.try_send_get_cosmic_workspace(
            &cosmic_workspace,
            workspace,
        )?;
        Ok(cosmic_workspace)
    }

    /// get cosmic workspace extension object
    ///
    /// Request a `zcosmic_workspace_handle_v2` extension object for an existing
    /// `ext_workspace_handle_v1`.
    ///
    /// If a `zcosmic_workspace_handle_v2` already exists for the `ext_workspace_handle_v1`, this
    /// will raise a `workspace_exists` protocol error.
    ///
    /// # Arguments
    ///
    /// - `workspace`:
    #[inline]
    pub fn new_send_get_cosmic_workspace(
        &self,
        workspace: &Rc<ExtWorkspaceHandleV1>,
    ) -> Rc<ZcosmicWorkspaceHandleV2> {
        let cosmic_workspace = self.core.create_child();
        self.send_get_cosmic_workspace(
            &cosmic_workspace,
            workspace,
        );
        cosmic_workspace
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the zcosmic_workspace_manager_v2 object
    ///
    /// This request should be called either when the client will no longer
    /// use the `zcosmic_workspace_manager_v2`.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_workspace_manager_v2#{}.destroy()\n", id);
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
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the zcosmic_workspace_manager_v2 object
    ///
    /// This request should be called either when the client will no longer
    /// use the `zcosmic_workspace_manager_v2`.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zcosmic_workspace_manager_v2.destroy", &e);
        }
    }
}

/// A message handler for [`ZcosmicWorkspaceManagerV2`] proxies.
pub trait ZcosmicWorkspaceManagerV2Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZcosmicWorkspaceManagerV2>) {
        slf.core.delete_id();
    }

    /// get cosmic workspace extension object
    ///
    /// Request a `zcosmic_workspace_handle_v2` extension object for an existing
    /// `ext_workspace_handle_v1`.
    ///
    /// If a `zcosmic_workspace_handle_v2` already exists for the `ext_workspace_handle_v1`, this
    /// will raise a `workspace_exists` protocol error.
    ///
    /// # Arguments
    ///
    /// - `cosmic_workspace`:
    /// - `workspace`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_cosmic_workspace(
        &mut self,
        slf: &Rc<ZcosmicWorkspaceManagerV2>,
        cosmic_workspace: &Rc<ZcosmicWorkspaceHandleV2>,
        workspace: &Rc<ExtWorkspaceHandleV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_cosmic_workspace(
            cosmic_workspace,
            workspace,
        );
        if let Err(e) = res {
            log_forward("zcosmic_workspace_manager_v2.get_cosmic_workspace", &e);
        }
    }

    /// destroy the zcosmic_workspace_manager_v2 object
    ///
    /// This request should be called either when the client will no longer
    /// use the `zcosmic_workspace_manager_v2`.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZcosmicWorkspaceManagerV2>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zcosmic_workspace_manager_v2.destroy", &e);
        }
    }
}

impl ObjectPrivate for ZcosmicWorkspaceManagerV2 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZcosmicWorkspaceManagerV2, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_workspace_manager_v2#{}.get_cosmic_workspace(cosmic_workspace: zcosmic_workspace_handle_v2#{}, workspace: ext_workspace_handle_v1#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = ZcosmicWorkspaceHandleV2::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "cosmic_workspace", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<ExtWorkspaceHandleV1>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("workspace", o.core().interface, ObjectInterface::ExtWorkspaceHandleV1)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_cosmic_workspace(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_cosmic_workspace(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_workspace_manager_v2#{}.destroy()\n", client_id, id);
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
            0 => "get_cosmic_workspace",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for ZcosmicWorkspaceManagerV2 {
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

impl ZcosmicWorkspaceManagerV2 {
    /// Since when the error.workspace_exists enum variant is available.
    pub const ENM__ERROR_WORKSPACE_EXISTS__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZcosmicWorkspaceManagerV2Error(pub u32);

impl ZcosmicWorkspaceManagerV2Error {
    /// zcosmic_workspace_handle_v2 already exists for ext_workspace_handle_v1
    pub const WORKSPACE_EXISTS: Self = Self(0);
}

impl Debug for ZcosmicWorkspaceManagerV2Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::WORKSPACE_EXISTS => "WORKSPACE_EXISTS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
