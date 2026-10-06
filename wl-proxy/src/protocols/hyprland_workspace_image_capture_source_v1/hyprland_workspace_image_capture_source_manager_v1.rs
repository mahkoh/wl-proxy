//! image capture source manager for workspaces
//!
//! This manager can create ICC capture sources from ext-workspace handles.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A hyprland_workspace_image_capture_source_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct HyprlandWorkspaceImageCaptureSourceManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn HyprlandWorkspaceImageCaptureSourceManagerV1Handler>,
}

struct DefaultHandler;

impl HyprlandWorkspaceImageCaptureSourceManagerV1Handler for DefaultHandler { }

impl ConcreteObject for HyprlandWorkspaceImageCaptureSourceManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::HyprlandWorkspaceImageCaptureSourceManagerV1;
    const INTERFACE_NAME: &str = "hyprland_workspace_image_capture_source_manager_v1";
}

impl HyprlandWorkspaceImageCaptureSourceManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl HyprlandWorkspaceImageCaptureSourceManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn HyprlandWorkspaceImageCaptureSourceManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for HyprlandWorkspaceImageCaptureSourceManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HyprlandWorkspaceImageCaptureSourceManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl HyprlandWorkspaceImageCaptureSourceManagerV1 {
    /// Since when the create_source message is available.
    pub const MSG__CREATE_SOURCE__SINCE: u32 = 1;

    /// create an image capture source for a workspace
    ///
    /// Create an ICC capture source from a given workspace handle.
    ///
    /// This source does not own the workspace handle. If the workspace is
    /// removed, further attempts to share this source's contents will produce
    /// a transparent, empty image.
    ///
    /// # Arguments
    ///
    /// - `source`: new workspace image capture source
    /// - `workspace`: workspace to capture
    /// - `mode`: content to include in the source image
    #[inline]
    pub fn try_send_create_source(
        &self,
        source: &Rc<ExtImageCaptureSourceV1>,
        workspace: &Rc<ExtWorkspaceHandleV1>,
        mode: HyprlandWorkspaceImageCaptureSourceManagerV1CaptureMode,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            source,
            workspace,
            mode,
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
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("source", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: HyprlandWorkspaceImageCaptureSourceManagerV1CaptureMode) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_workspace_image_capture_source_manager_v1#{}.create_source(source: ext_image_capture_source_v1#{}, workspace: ext_workspace_handle_v1#{}, mode: {:?})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id, arg2);
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
            arg2.0,
        ]);
        Ok(())
    }

    /// create an image capture source for a workspace
    ///
    /// Create an ICC capture source from a given workspace handle.
    ///
    /// This source does not own the workspace handle. If the workspace is
    /// removed, further attempts to share this source's contents will produce
    /// a transparent, empty image.
    ///
    /// # Arguments
    ///
    /// - `source`: new workspace image capture source
    /// - `workspace`: workspace to capture
    /// - `mode`: content to include in the source image
    #[inline]
    pub fn send_create_source(
        &self,
        source: &Rc<ExtImageCaptureSourceV1>,
        workspace: &Rc<ExtWorkspaceHandleV1>,
        mode: HyprlandWorkspaceImageCaptureSourceManagerV1CaptureMode,
    ) {
        let res = self.try_send_create_source(
            source,
            workspace,
            mode,
        );
        if let Err(e) = res {
            log_send("hyprland_workspace_image_capture_source_manager_v1.create_source", &e);
        }
    }

    /// create an image capture source for a workspace
    ///
    /// Create an ICC capture source from a given workspace handle.
    ///
    /// This source does not own the workspace handle. If the workspace is
    /// removed, further attempts to share this source's contents will produce
    /// a transparent, empty image.
    ///
    /// # Arguments
    ///
    /// - `workspace`: workspace to capture
    /// - `mode`: content to include in the source image
    #[inline]
    pub fn new_try_send_create_source(
        &self,
        workspace: &Rc<ExtWorkspaceHandleV1>,
        mode: HyprlandWorkspaceImageCaptureSourceManagerV1CaptureMode,
    ) -> Result<Rc<ExtImageCaptureSourceV1>, ObjectError> {
        let source = self.core.create_child();
        self.try_send_create_source(
            &source,
            workspace,
            mode,
        )?;
        Ok(source)
    }

    /// create an image capture source for a workspace
    ///
    /// Create an ICC capture source from a given workspace handle.
    ///
    /// This source does not own the workspace handle. If the workspace is
    /// removed, further attempts to share this source's contents will produce
    /// a transparent, empty image.
    ///
    /// # Arguments
    ///
    /// - `workspace`: workspace to capture
    /// - `mode`: content to include in the source image
    #[inline]
    pub fn new_send_create_source(
        &self,
        workspace: &Rc<ExtWorkspaceHandleV1>,
        mode: HyprlandWorkspaceImageCaptureSourceManagerV1CaptureMode,
    ) -> Rc<ExtImageCaptureSourceV1> {
        let source = self.core.create_child();
        self.send_create_source(
            &source,
            workspace,
            mode,
        );
        source
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the manager
    ///
    /// Destroy this manager. Sources and capture sessions created through
    /// this manager remain valid after its destruction.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_workspace_image_capture_source_manager_v1#{}.destroy()\n", id);
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

    /// destroy the manager
    ///
    /// Destroy this manager. Sources and capture sessions created through
    /// this manager remain valid after its destruction.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("hyprland_workspace_image_capture_source_manager_v1.destroy", &e);
        }
    }
}

/// A message handler for [`HyprlandWorkspaceImageCaptureSourceManagerV1`] proxies.
pub trait HyprlandWorkspaceImageCaptureSourceManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<HyprlandWorkspaceImageCaptureSourceManagerV1>) {
        slf.core.delete_id();
    }

    /// create an image capture source for a workspace
    ///
    /// Create an ICC capture source from a given workspace handle.
    ///
    /// This source does not own the workspace handle. If the workspace is
    /// removed, further attempts to share this source's contents will produce
    /// a transparent, empty image.
    ///
    /// # Arguments
    ///
    /// - `source`: new workspace image capture source
    /// - `workspace`: workspace to capture
    /// - `mode`: content to include in the source image
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_create_source(
        &mut self,
        slf: &Rc<HyprlandWorkspaceImageCaptureSourceManagerV1>,
        source: &Rc<ExtImageCaptureSourceV1>,
        workspace: &Rc<ExtWorkspaceHandleV1>,
        mode: HyprlandWorkspaceImageCaptureSourceManagerV1CaptureMode,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_source(
            source,
            workspace,
            mode,
        );
        if let Err(e) = res {
            log_forward("hyprland_workspace_image_capture_source_manager_v1.create_source", &e);
        }
    }

    /// destroy the manager
    ///
    /// Destroy this manager. Sources and capture sessions created through
    /// this manager remain valid after its destruction.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<HyprlandWorkspaceImageCaptureSourceManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("hyprland_workspace_image_capture_source_manager_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for HyprlandWorkspaceImageCaptureSourceManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::HyprlandWorkspaceImageCaptureSourceManagerV1, version),
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
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                let arg2 = HyprlandWorkspaceImageCaptureSourceManagerV1CaptureMode(arg2);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: HyprlandWorkspaceImageCaptureSourceManagerV1CaptureMode) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_workspace_image_capture_source_manager_v1#{}.create_source(source: ext_image_capture_source_v1#{}, workspace: ext_workspace_handle_v1#{}, mode: {:?})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let arg0 = ExtImageCaptureSourceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "source", e)))?;
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
                    (**handler).handle_create_source(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_create_source(&self, arg0, arg1, arg2);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_workspace_image_capture_source_manager_v1#{}.destroy()\n", client_id, id);
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
            0 => "create_source",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }

    fn create_zombie(&self) -> Rc<dyn Object> {
        let slf = Self::new(&self.core.state, self.core.version);
        slf.core.make_zombie();
        slf
    }
}

impl Object for HyprlandWorkspaceImageCaptureSourceManagerV1 {
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

impl HyprlandWorkspaceImageCaptureSourceManagerV1 {
    /// Since when the capture_mode.windows_only enum variant is available.
    pub const ENM__CAPTURE_MODE_WINDOWS_ONLY__SINCE: u32 = 1;
    /// Since when the capture_mode.everything enum variant is available.
    pub const ENM__CAPTURE_MODE_EVERYTHING__SINCE: u32 = 1;

    /// Since when the error.invalid_mode enum variant is available.
    pub const ENM__ERROR_INVALID_MODE__SINCE: u32 = 1;
}

/// content included in the source image
///
/// This enum describes the allowed capture modes.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HyprlandWorkspaceImageCaptureSourceManagerV1CaptureMode(pub u32);

impl HyprlandWorkspaceImageCaptureSourceManagerV1CaptureMode {
    /// workspace windows only
    ///
    /// Include only the selected workspace's windows and their associated
    /// surfaces and decorations. The background will be transparent.
    pub const WINDOWS_ONLY: Self = Self(0);

    /// workspace windows and output-global desktop elements
    ///
    /// Include the selected workspace's windows together with wallpaper,
    /// panels, pinned windows, and other output-global surfaces, including
    /// layer-shell surfaces, associated with its output.
    pub const EVERYTHING: Self = Self(1);
}

impl Debug for HyprlandWorkspaceImageCaptureSourceManagerV1CaptureMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::WINDOWS_ONLY => "WINDOWS_ONLY",
            Self::EVERYTHING => "EVERYTHING",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HyprlandWorkspaceImageCaptureSourceManagerV1Error(pub u32);

impl HyprlandWorkspaceImageCaptureSourceManagerV1Error {
    /// unknown capture mode
    pub const INVALID_MODE: Self = Self(0);
}

impl Debug for HyprlandWorkspaceImageCaptureSourceManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_MODE => "INVALID_MODE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
