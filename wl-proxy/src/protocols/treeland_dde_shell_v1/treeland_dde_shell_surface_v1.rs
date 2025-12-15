//! metadata interface
//!
//! An interface that may be implemented by a wl_surface, for
//! implementations that provide the shell user interface.
//!
//! It provides requests to set surface role, set skip, set the position
//! set auto placement in output coordinates.
//!
//! On the server side the object is automatically destroyed when
//! the related wl_surface is destroyed.  On client side,
//! treeland_dde_shell_surface_v1.destroy() must be called before
//! destroying the wl_surface object.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A treeland_dde_shell_surface_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandDdeShellSurfaceV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn TreelandDdeShellSurfaceV1Handler>,
}

struct DefaultHandler;

impl TreelandDdeShellSurfaceV1Handler for DefaultHandler { }

impl TreelandDdeShellSurfaceV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::TreelandDdeShellSurfaceV1;
    pub const INTERFACE_NAME: &str = "treeland_dde_shell_surface_v1";
}

impl TreelandDdeShellSurfaceV1 {
    pub fn set_handler(&self, handler: impl TreelandDdeShellSurfaceV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandDdeShellSurfaceV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandDdeShellSurfaceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandDdeShellSurfaceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandDdeShellSurfaceV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the treeland_dde_shell_surface_v1 object
    ///
    /// The treeland_dde_shell_surface_v1 interface is removed from the
    /// wl_surface object that was turned into a shell surface with the
    /// treeland_shell_v1.get_treeland_dde_shell_surface request.
    ///
    /// The shell surface role is lost and wl_surface is unmapped.
    #[inline]
    pub fn try_send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_dde_shell_surface_v1#{}.destroy()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
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

    /// destroy the treeland_dde_shell_surface_v1 object
    ///
    /// The treeland_dde_shell_surface_v1 interface is removed from the
    /// wl_surface object that was turned into a shell surface with the
    /// treeland_shell_v1.get_treeland_dde_shell_surface request.
    ///
    /// The shell surface role is lost and wl_surface is unmapped.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("treeland_dde_shell_surface_v1.destroy", &e);
        }
    }

    /// Since when the set_surface_position message is available.
    pub const MSG__SET_SURFACE_POSITION__SINCE: u32 = 1;

    /// change the shell surface position
    ///
    /// Move the surface to new coordinates.
    ///
    /// Coordinates are global, for example 50,50 for a 1920,0+1920x1080 output
    /// is 1970,50 in global coordinates space.
    ///
    /// # Arguments
    ///
    /// - `x`: x coordinate in global space
    /// - `y`: y coordinate in global space
    #[inline]
    pub fn try_send_set_surface_position(
        &self,
        x: i32,
        y: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            x,
            y,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_dde_shell_surface_v1#{}.set_surface_position(x: {}, y: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1);
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
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// change the shell surface position
    ///
    /// Move the surface to new coordinates.
    ///
    /// Coordinates are global, for example 50,50 for a 1920,0+1920x1080 output
    /// is 1970,50 in global coordinates space.
    ///
    /// # Arguments
    ///
    /// - `x`: x coordinate in global space
    /// - `y`: y coordinate in global space
    #[inline]
    pub fn send_set_surface_position(
        &self,
        x: i32,
        y: i32,
    ) {
        let res = self.try_send_set_surface_position(
            x,
            y,
        );
        if let Err(e) = res {
            log_send("treeland_dde_shell_surface_v1.set_surface_position", &e);
        }
    }

    /// Since when the set_role message is available.
    pub const MSG__SET_ROLE__SINCE: u32 = 1;

    /// assign a role to this surface
    ///
    /// Assign a role to a shell surface.
    ///
    /// # Arguments
    ///
    /// - `role`:
    #[inline]
    pub fn try_send_set_role(
        &self,
        role: TreelandDdeShellSurfaceV1Role,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            role,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: TreelandDdeShellSurfaceV1Role) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_dde_shell_surface_v1#{}.set_role(role: {:?})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// assign a role to this surface
    ///
    /// Assign a role to a shell surface.
    ///
    /// # Arguments
    ///
    /// - `role`:
    #[inline]
    pub fn send_set_role(
        &self,
        role: TreelandDdeShellSurfaceV1Role,
    ) {
        let res = self.try_send_set_role(
            role,
        );
        if let Err(e) = res {
            log_send("treeland_dde_shell_surface_v1.set_role", &e);
        }
    }

    /// Since when the set_auto_placement message is available.
    pub const MSG__SET_AUTO_PLACEMENT__SINCE: u32 = 1;

    /// Place the surface at the bottom of the cursor area
    ///
    /// Set the vertical alignment of the surface within the cursor width.
    ///
    /// Do not use it together with set_surface_position to avoid exceptions.
    ///
    /// The position of the surface will be controlled by the compositor after the
    /// request, including preventing it from being displayed beyond the edge of
    /// the output.
    ///
    /// # Arguments
    ///
    /// - `y_offset`: y position is relative to the cursor bottom
    #[inline]
    pub fn try_send_set_auto_placement(
        &self,
        y_offset: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            y_offset,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_dde_shell_surface_v1#{}.set_auto_placement(y_offset: {})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            arg0,
        ]);
        Ok(())
    }

    /// Place the surface at the bottom of the cursor area
    ///
    /// Set the vertical alignment of the surface within the cursor width.
    ///
    /// Do not use it together with set_surface_position to avoid exceptions.
    ///
    /// The position of the surface will be controlled by the compositor after the
    /// request, including preventing it from being displayed beyond the edge of
    /// the output.
    ///
    /// # Arguments
    ///
    /// - `y_offset`: y position is relative to the cursor bottom
    #[inline]
    pub fn send_set_auto_placement(
        &self,
        y_offset: u32,
    ) {
        let res = self.try_send_set_auto_placement(
            y_offset,
        );
        if let Err(e) = res {
            log_send("treeland_dde_shell_surface_v1.set_auto_placement", &e);
        }
    }

    /// Since when the set_skip_switcher message is available.
    pub const MSG__SET_SKIP_SWITCHER__SINCE: u32 = 1;

    /// make the window not appear in a switcher
    ///
    /// Setting this bit will indicate that the window prefers not to be listed in a switcher.
    ///
    /// # Arguments
    ///
    /// - `skip`: Boolean value that sets whether to skip the window switcher.
    #[inline]
    pub fn try_send_set_skip_switcher(
        &self,
        skip: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            skip,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_dde_shell_surface_v1#{}.set_skip_switcher(skip: {})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            arg0,
        ]);
        Ok(())
    }

    /// make the window not appear in a switcher
    ///
    /// Setting this bit will indicate that the window prefers not to be listed in a switcher.
    ///
    /// # Arguments
    ///
    /// - `skip`: Boolean value that sets whether to skip the window switcher.
    #[inline]
    pub fn send_set_skip_switcher(
        &self,
        skip: u32,
    ) {
        let res = self.try_send_set_skip_switcher(
            skip,
        );
        if let Err(e) = res {
            log_send("treeland_dde_shell_surface_v1.set_skip_switcher", &e);
        }
    }

    /// Since when the set_skip_dock_preview message is available.
    pub const MSG__SET_SKIP_DOCK_PREVIEW__SINCE: u32 = 1;

    /// make the window not appear in a dock preview
    ///
    /// Setting this bit will indicate that the window prefers not to be listed in a dock preview.
    ///
    /// # Arguments
    ///
    /// - `skip`: Boolean value that sets whether to skip the dock preview.
    #[inline]
    pub fn try_send_set_skip_dock_preview(
        &self,
        skip: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            skip,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_dde_shell_surface_v1#{}.set_skip_dock_preview(skip: {})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            arg0,
        ]);
        Ok(())
    }

    /// make the window not appear in a dock preview
    ///
    /// Setting this bit will indicate that the window prefers not to be listed in a dock preview.
    ///
    /// # Arguments
    ///
    /// - `skip`: Boolean value that sets whether to skip the dock preview.
    #[inline]
    pub fn send_set_skip_dock_preview(
        &self,
        skip: u32,
    ) {
        let res = self.try_send_set_skip_dock_preview(
            skip,
        );
        if let Err(e) = res {
            log_send("treeland_dde_shell_surface_v1.set_skip_dock_preview", &e);
        }
    }

    /// Since when the set_skip_muti_task_view message is available.
    pub const MSG__SET_SKIP_MUTI_TASK_VIEW__SINCE: u32 = 1;

    /// make the window not appear in a mutitask view
    ///
    /// Setting this bit will indicate that the window prefers not to be listed in a mutitask view.
    ///
    /// # Arguments
    ///
    /// - `skip`: Boolean value that sets whether to skip the mutitask view.
    #[inline]
    pub fn try_send_set_skip_muti_task_view(
        &self,
        skip: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            skip,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_dde_shell_surface_v1#{}.set_skip_muti_task_view(skip: {})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            6,
            arg0,
        ]);
        Ok(())
    }

    /// make the window not appear in a mutitask view
    ///
    /// Setting this bit will indicate that the window prefers not to be listed in a mutitask view.
    ///
    /// # Arguments
    ///
    /// - `skip`: Boolean value that sets whether to skip the mutitask view.
    #[inline]
    pub fn send_set_skip_muti_task_view(
        &self,
        skip: u32,
    ) {
        let res = self.try_send_set_skip_muti_task_view(
            skip,
        );
        if let Err(e) = res {
            log_send("treeland_dde_shell_surface_v1.set_skip_muti_task_view", &e);
        }
    }

    /// Since when the set_accept_keyboard_focus message is available.
    pub const MSG__SET_ACCEPT_KEYBOARD_FOCUS__SINCE: u32 = 1;

    /// control whether the surface accepts keyboard focus
    ///
    /// Setting this will determine whether the surface can receive keyboard focus.
    /// When set to 0, the surface will not receive keyboard focus even when clicked or activated.
    /// When set to 1 (default), the surface will receive keyboard focus normally.
    ///
    /// # Arguments
    ///
    /// - `accept`: Boolean value that sets whether to accept keyboard focus
    #[inline]
    pub fn try_send_set_accept_keyboard_focus(
        &self,
        accept: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            accept,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_dde_shell_surface_v1#{}.set_accept_keyboard_focus(accept: {})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            7,
            arg0,
        ]);
        Ok(())
    }

    /// control whether the surface accepts keyboard focus
    ///
    /// Setting this will determine whether the surface can receive keyboard focus.
    /// When set to 0, the surface will not receive keyboard focus even when clicked or activated.
    /// When set to 1 (default), the surface will receive keyboard focus normally.
    ///
    /// # Arguments
    ///
    /// - `accept`: Boolean value that sets whether to accept keyboard focus
    #[inline]
    pub fn send_set_accept_keyboard_focus(
        &self,
        accept: u32,
    ) {
        let res = self.try_send_set_accept_keyboard_focus(
            accept,
        );
        if let Err(e) = res {
            log_send("treeland_dde_shell_surface_v1.set_accept_keyboard_focus", &e);
        }
    }
}

/// A message handler for [TreelandDdeShellSurfaceV1] proxies.
pub trait TreelandDdeShellSurfaceV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<TreelandDdeShellSurfaceV1>) {
        slf.core.delete_id();
    }

    /// destroy the treeland_dde_shell_surface_v1 object
    ///
    /// The treeland_dde_shell_surface_v1 interface is removed from the
    /// wl_surface object that was turned into a shell surface with the
    /// treeland_shell_v1.get_treeland_dde_shell_surface request.
    ///
    /// The shell surface role is lost and wl_surface is unmapped.
    #[inline]
    fn handle_destroy(
        &mut self,
        _slf: &Rc<TreelandDdeShellSurfaceV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("treeland_dde_shell_surface_v1.destroy", &e);
        }
    }

    /// change the shell surface position
    ///
    /// Move the surface to new coordinates.
    ///
    /// Coordinates are global, for example 50,50 for a 1920,0+1920x1080 output
    /// is 1970,50 in global coordinates space.
    ///
    /// # Arguments
    ///
    /// - `x`: x coordinate in global space
    /// - `y`: y coordinate in global space
    #[inline]
    fn handle_set_surface_position(
        &mut self,
        _slf: &Rc<TreelandDdeShellSurfaceV1>,
        x: i32,
        y: i32,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_set_surface_position(
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("treeland_dde_shell_surface_v1.set_surface_position", &e);
        }
    }

    /// assign a role to this surface
    ///
    /// Assign a role to a shell surface.
    ///
    /// # Arguments
    ///
    /// - `role`:
    #[inline]
    fn handle_set_role(
        &mut self,
        _slf: &Rc<TreelandDdeShellSurfaceV1>,
        role: TreelandDdeShellSurfaceV1Role,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_set_role(
            role,
        );
        if let Err(e) = res {
            log_forward("treeland_dde_shell_surface_v1.set_role", &e);
        }
    }

    /// Place the surface at the bottom of the cursor area
    ///
    /// Set the vertical alignment of the surface within the cursor width.
    ///
    /// Do not use it together with set_surface_position to avoid exceptions.
    ///
    /// The position of the surface will be controlled by the compositor after the
    /// request, including preventing it from being displayed beyond the edge of
    /// the output.
    ///
    /// # Arguments
    ///
    /// - `y_offset`: y position is relative to the cursor bottom
    #[inline]
    fn handle_set_auto_placement(
        &mut self,
        _slf: &Rc<TreelandDdeShellSurfaceV1>,
        y_offset: u32,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_set_auto_placement(
            y_offset,
        );
        if let Err(e) = res {
            log_forward("treeland_dde_shell_surface_v1.set_auto_placement", &e);
        }
    }

    /// make the window not appear in a switcher
    ///
    /// Setting this bit will indicate that the window prefers not to be listed in a switcher.
    ///
    /// # Arguments
    ///
    /// - `skip`: Boolean value that sets whether to skip the window switcher.
    #[inline]
    fn handle_set_skip_switcher(
        &mut self,
        _slf: &Rc<TreelandDdeShellSurfaceV1>,
        skip: u32,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_set_skip_switcher(
            skip,
        );
        if let Err(e) = res {
            log_forward("treeland_dde_shell_surface_v1.set_skip_switcher", &e);
        }
    }

    /// make the window not appear in a dock preview
    ///
    /// Setting this bit will indicate that the window prefers not to be listed in a dock preview.
    ///
    /// # Arguments
    ///
    /// - `skip`: Boolean value that sets whether to skip the dock preview.
    #[inline]
    fn handle_set_skip_dock_preview(
        &mut self,
        _slf: &Rc<TreelandDdeShellSurfaceV1>,
        skip: u32,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_set_skip_dock_preview(
            skip,
        );
        if let Err(e) = res {
            log_forward("treeland_dde_shell_surface_v1.set_skip_dock_preview", &e);
        }
    }

    /// make the window not appear in a mutitask view
    ///
    /// Setting this bit will indicate that the window prefers not to be listed in a mutitask view.
    ///
    /// # Arguments
    ///
    /// - `skip`: Boolean value that sets whether to skip the mutitask view.
    #[inline]
    fn handle_set_skip_muti_task_view(
        &mut self,
        _slf: &Rc<TreelandDdeShellSurfaceV1>,
        skip: u32,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_set_skip_muti_task_view(
            skip,
        );
        if let Err(e) = res {
            log_forward("treeland_dde_shell_surface_v1.set_skip_muti_task_view", &e);
        }
    }

    /// control whether the surface accepts keyboard focus
    ///
    /// Setting this will determine whether the surface can receive keyboard focus.
    /// When set to 0, the surface will not receive keyboard focus even when clicked or activated.
    /// When set to 1 (default), the surface will receive keyboard focus normally.
    ///
    /// # Arguments
    ///
    /// - `accept`: Boolean value that sets whether to accept keyboard focus
    #[inline]
    fn handle_set_accept_keyboard_focus(
        &mut self,
        _slf: &Rc<TreelandDdeShellSurfaceV1>,
        accept: u32,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_set_accept_keyboard_focus(
            accept,
        );
        if let Err(e) = res {
            log_forward("treeland_dde_shell_surface_v1.set_accept_keyboard_focus", &e);
        }
    }
}

impl ObjectPrivate for TreelandDdeShellSurfaceV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::TreelandDdeShellSurfaceV1, version),
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
            self.core.delete_id();
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
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_dde_shell_surface_v1#{}.destroy()\n", client_id, id);
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
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_dde_shell_surface_v1#{}.set_surface_position(x: {}, y: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_surface_position(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_surface_position(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = TreelandDdeShellSurfaceV1Role(arg0);
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: TreelandDdeShellSurfaceV1Role) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_dde_shell_surface_v1#{}.set_role(role: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_role(&self, arg0);
                } else {
                    DefaultHandler.handle_set_role(&self, arg0);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_dde_shell_surface_v1#{}.set_auto_placement(y_offset: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_auto_placement(&self, arg0);
                } else {
                    DefaultHandler.handle_set_auto_placement(&self, arg0);
                }
            }
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_dde_shell_surface_v1#{}.set_skip_switcher(skip: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_skip_switcher(&self, arg0);
                } else {
                    DefaultHandler.handle_set_skip_switcher(&self, arg0);
                }
            }
            5 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_dde_shell_surface_v1#{}.set_skip_dock_preview(skip: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_skip_dock_preview(&self, arg0);
                } else {
                    DefaultHandler.handle_set_skip_dock_preview(&self, arg0);
                }
            }
            6 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_dde_shell_surface_v1#{}.set_skip_muti_task_view(skip: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_skip_muti_task_view(&self, arg0);
                } else {
                    DefaultHandler.handle_set_skip_muti_task_view(&self, arg0);
                }
            }
            7 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_dde_shell_surface_v1#{}.set_accept_keyboard_focus(accept: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_accept_keyboard_focus(&self, arg0);
                } else {
                    DefaultHandler.handle_set_accept_keyboard_focus(&self, arg0);
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
            0 => "destroy",
            1 => "set_surface_position",
            2 => "set_role",
            3 => "set_auto_placement",
            4 => "set_skip_switcher",
            5 => "set_skip_dock_preview",
            6 => "set_skip_muti_task_view",
            7 => "set_accept_keyboard_focus",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for TreelandDdeShellSurfaceV1 {
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

impl TreelandDdeShellSurfaceV1 {
    /// Since when the role.overlay enum variant is available.
    pub const ENM__ROLE_OVERLAY__SINCE: u32 = 1;
}

/// available roles for surfaces
///
/// These values indicate which roles a surface can be rendered in, They
/// are ordered by z depth.
///
/// Displayed below wlr-layer-shell, at the overlay level of the workspace.
///
/// Multiple surfaces can share a single role, and ordering within a single
/// role is undefined.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TreelandDdeShellSurfaceV1Role(pub u32);

impl TreelandDdeShellSurfaceV1Role {
    pub const OVERLAY: Self = Self(1);
}

impl Debug for TreelandDdeShellSurfaceV1Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::OVERLAY => "OVERLAY",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
