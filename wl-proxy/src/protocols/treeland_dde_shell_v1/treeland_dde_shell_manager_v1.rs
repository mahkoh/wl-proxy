//! dde shell manager
//!
//! This interface allows DDE change some treeland function.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A treeland_dde_shell_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandDdeShellManagerV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn TreelandDdeShellManagerV1Handler>,
}

struct DefaultHandler;

impl TreelandDdeShellManagerV1Handler for DefaultHandler { }

impl TreelandDdeShellManagerV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "treeland_dde_shell_manager_v1";
}

impl TreelandDdeShellManagerV1 {
    pub fn set_handler(&self, handler: impl TreelandDdeShellManagerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandDdeShellManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandDdeShellManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandDdeShellManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandDdeShellManagerV1 {
    /// Since when the get_window_overlap_checker message is available.
    pub const MSG__GET_WINDOW_OVERLAP_CHECKER__SINCE: u32 = 1;

    #[inline]
    pub fn send_get_window_overlap_checker(
        &self,
        id: &Rc<TreelandWindowOverlapChecker>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_dde_shell_manager_v1#{}.get_window_overlap_checker(id: treeland_window_overlap_checker#{})\n", id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the get_shell_surface message is available.
    pub const MSG__GET_SHELL_SURFACE__SINCE: u32 = 1;

    /// create a shell surface from a surface
    ///
    /// Create a shell surface for an existing wl_surface.
    ///
    /// Only one shell surface can be associated with a given surface.
    ///
    /// Recommended for use with xdg_surface.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn send_get_shell_surface(
        &self,
        id: &Rc<TreelandDdeShellSurfaceV1>,
        surface: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            surface,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("surface")),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_dde_shell_manager_v1#{}.get_shell_surface(id: treeland_dde_shell_surface_v1#{}, surface: wl_surface#{})\n", id, arg0_id, arg1_id);
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

    /// Since when the get_treeland_dde_active message is available.
    pub const MSG__GET_TREELAND_DDE_ACTIVE__SINCE: u32 = 1;

    /// create a new dde active
    ///
    /// Create a new dde active for a given seat.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `seat`: seat associated with the dde_active
    #[inline]
    pub fn send_get_treeland_dde_active(
        &self,
        id: &Rc<TreelandDdeActiveV1>,
        seat: &Rc<WlSeat>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            seat,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("seat")),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_dde_shell_manager_v1#{}.get_treeland_dde_active(id: treeland_dde_active_v1#{}, seat: wl_seat#{})\n", id, arg0_id, arg1_id);
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
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// Since when the get_treeland_multitaskview message is available.
    pub const MSG__GET_TREELAND_MULTITASKVIEW__SINCE: u32 = 1;

    /// create a new multitaskview context
    ///
    /// Create a new multitaskview context for toggle.
    #[inline]
    pub fn send_get_treeland_multitaskview(
        &self,
        id: &Rc<TreelandMultitaskviewV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_dde_shell_manager_v1#{}.get_treeland_multitaskview(id: treeland_multitaskview_v1#{})\n", id, arg0_id);
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

    /// Since when the get_treeland_window_picker message is available.
    pub const MSG__GET_TREELAND_WINDOW_PICKER__SINCE: u32 = 1;

    /// create a new window picker
    ///
    /// Create a new window picker to pick window.
    #[inline]
    pub fn send_get_treeland_window_picker(
        &self,
        id: &Rc<TreelandWindowPickerV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_dde_shell_manager_v1#{}.get_treeland_window_picker(id: treeland_window_picker_v1#{})\n", id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the get_treeland_lockscreen message is available.
    pub const MSG__GET_TREELAND_LOCKSCREEN__SINCE: u32 = 1;

    /// create a new lockscreen context
    ///
    /// Create a new lockscreen context for toggle.
    #[inline]
    pub fn send_get_treeland_lockscreen(
        &self,
        id: &Rc<TreelandLockscreenV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_dde_shell_manager_v1#{}.get_treeland_lockscreen(id: treeland_lockscreen_v1#{})\n", id, arg0_id);
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
            5,
            arg0_id,
        ]);
        Ok(())
    }
}

/// A message handler for [TreelandDdeShellManagerV1] proxies.
pub trait TreelandDdeShellManagerV1Handler: Any {
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn get_window_overlap_checker(
        &mut self,
        _slf: &Rc<TreelandDdeShellManagerV1>,
        id: &Rc<TreelandWindowOverlapChecker>,
    ) {
        let res = _slf.send_get_window_overlap_checker(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_dde_shell_manager_v1.get_window_overlap_checker message: {}", Report::new(e));
        }
    }

    /// create a shell surface from a surface
    ///
    /// Create a shell surface for an existing wl_surface.
    ///
    /// Only one shell surface can be associated with a given surface.
    ///
    /// Recommended for use with xdg_surface.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_shell_surface(
        &mut self,
        _slf: &Rc<TreelandDdeShellManagerV1>,
        id: &Rc<TreelandDdeShellSurfaceV1>,
        surface: &Rc<WlSurface>,
    ) {
        let res = _slf.send_get_shell_surface(
            id,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_dde_shell_manager_v1.get_shell_surface message: {}", Report::new(e));
        }
    }

    /// create a new dde active
    ///
    /// Create a new dde active for a given seat.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `seat`: seat associated with the dde_active
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_treeland_dde_active(
        &mut self,
        _slf: &Rc<TreelandDdeShellManagerV1>,
        id: &Rc<TreelandDdeActiveV1>,
        seat: &Rc<WlSeat>,
    ) {
        let res = _slf.send_get_treeland_dde_active(
            id,
            seat,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_dde_shell_manager_v1.get_treeland_dde_active message: {}", Report::new(e));
        }
    }

    /// create a new multitaskview context
    ///
    /// Create a new multitaskview context for toggle.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn get_treeland_multitaskview(
        &mut self,
        _slf: &Rc<TreelandDdeShellManagerV1>,
        id: &Rc<TreelandMultitaskviewV1>,
    ) {
        let res = _slf.send_get_treeland_multitaskview(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_dde_shell_manager_v1.get_treeland_multitaskview message: {}", Report::new(e));
        }
    }

    /// create a new window picker
    ///
    /// Create a new window picker to pick window.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn get_treeland_window_picker(
        &mut self,
        _slf: &Rc<TreelandDdeShellManagerV1>,
        id: &Rc<TreelandWindowPickerV1>,
    ) {
        let res = _slf.send_get_treeland_window_picker(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_dde_shell_manager_v1.get_treeland_window_picker message: {}", Report::new(e));
        }
    }

    /// create a new lockscreen context
    ///
    /// Create a new lockscreen context for toggle.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn get_treeland_lockscreen(
        &mut self,
        _slf: &Rc<TreelandDdeShellManagerV1>,
        id: &Rc<TreelandLockscreenV1>,
    ) {
        let res = _slf.send_get_treeland_lockscreen(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_dde_shell_manager_v1.get_treeland_lockscreen message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for TreelandDdeShellManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::TreelandDdeShellManagerV1, version),
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_dde_shell_manager_v1#{}.get_window_overlap_checker(id: treeland_window_overlap_checker#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = TreelandWindowOverlapChecker::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).get_window_overlap_checker(&self, arg0);
                } else {
                    DefaultHandler.get_window_overlap_checker(&self, arg0);
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_dde_shell_manager_v1#{}.get_shell_surface(id: treeland_dde_shell_surface_v1#{}, surface: wl_surface#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = TreelandDdeShellSurfaceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ProxyInterface::WlSurface));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_shell_surface(&self, arg0, arg1);
                } else {
                    DefaultHandler.get_shell_surface(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_dde_shell_manager_v1#{}.get_treeland_dde_active(id: treeland_dde_active_v1#{}, seat: wl_seat#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = TreelandDdeActiveV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("seat", o.core().interface, ProxyInterface::WlSeat));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_treeland_dde_active(&self, arg0, arg1);
                } else {
                    DefaultHandler.get_treeland_dde_active(&self, arg0, arg1);
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_dde_shell_manager_v1#{}.get_treeland_multitaskview(id: treeland_multitaskview_v1#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = TreelandMultitaskviewV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).get_treeland_multitaskview(&self, arg0);
                } else {
                    DefaultHandler.get_treeland_multitaskview(&self, arg0);
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_dde_shell_manager_v1#{}.get_treeland_window_picker(id: treeland_window_picker_v1#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = TreelandWindowPickerV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).get_treeland_window_picker(&self, arg0);
                } else {
                    DefaultHandler.get_treeland_window_picker(&self, arg0);
                }
            }
            5 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_dde_shell_manager_v1#{}.get_treeland_lockscreen(id: treeland_lockscreen_v1#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = TreelandLockscreenV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).get_treeland_lockscreen(&self, arg0);
                } else {
                    DefaultHandler.get_treeland_lockscreen(&self, arg0);
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
            0 => "get_window_overlap_checker",
            1 => "get_shell_surface",
            2 => "get_treeland_dde_active",
            3 => "get_treeland_multitaskview",
            4 => "get_treeland_window_picker",
            5 => "get_treeland_lockscreen",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Proxy for TreelandDdeShellManagerV1 {
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

