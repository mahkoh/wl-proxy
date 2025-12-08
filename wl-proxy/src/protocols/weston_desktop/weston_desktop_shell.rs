//! create desktop widgets and helpers
//!
//! Traditional user interfaces can rely on this interface to define the
//! foundations of typical desktops. Currently it's possible to set up
//! background, panels and locking surfaces.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A weston_desktop_shell object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WestonDesktopShell {
    core: ObjectCore,
    handler: HandlerHolder<dyn WestonDesktopShellHandler>,
}

struct DefaultHandler;

impl WestonDesktopShellHandler for DefaultHandler { }

impl ConcreteObject for WestonDesktopShell {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WestonDesktopShell;
    const INTERFACE_NAME: &str = "weston_desktop_shell";
}

impl WestonDesktopShell {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WestonDesktopShellHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WestonDesktopShellHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WestonDesktopShell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WestonDesktopShell")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WestonDesktopShell {
    /// Since when the set_background message is available.
    pub const MSG__SET_BACKGROUND__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `output`:
    /// - `surface`:
    #[inline]
    pub fn try_send_set_background(
        &self,
        output: &Rc<WlOutput>,
        surface: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            output,
            surface,
        );
        let arg0 = arg0.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("output"))),
            Some(id) => id,
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_desktop_shell#{}.set_background(output: wl_output#{}, surface: wl_surface#{})\n", id, arg0, arg1);
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

    /// # Arguments
    ///
    /// - `output`:
    /// - `surface`:
    #[inline]
    pub fn send_set_background(
        &self,
        output: &Rc<WlOutput>,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_set_background(
            output,
            surface,
        );
        if let Err(e) = res {
            log_send("weston_desktop_shell.set_background", &e);
        }
    }

    /// Since when the set_panel message is available.
    pub const MSG__SET_PANEL__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `output`:
    /// - `surface`:
    #[inline]
    pub fn try_send_set_panel(
        &self,
        output: &Rc<WlOutput>,
        surface: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            output,
            surface,
        );
        let arg0 = arg0.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("output"))),
            Some(id) => id,
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_desktop_shell#{}.set_panel(output: wl_output#{}, surface: wl_surface#{})\n", id, arg0, arg1);
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
            1,
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `output`:
    /// - `surface`:
    #[inline]
    pub fn send_set_panel(
        &self,
        output: &Rc<WlOutput>,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_set_panel(
            output,
            surface,
        );
        if let Err(e) = res {
            log_send("weston_desktop_shell.set_panel", &e);
        }
    }

    /// Since when the set_lock_surface message is available.
    pub const MSG__SET_LOCK_SURFACE__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn try_send_set_lock_surface(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            surface,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_desktop_shell#{}.set_lock_surface(surface: wl_surface#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn send_set_lock_surface(
        &self,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_set_lock_surface(
            surface,
        );
        if let Err(e) = res {
            log_send("weston_desktop_shell.set_lock_surface", &e);
        }
    }

    /// Since when the unlock message is available.
    pub const MSG__UNLOCK__SINCE: u32 = 1;

    #[inline]
    pub fn try_send_unlock(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_desktop_shell#{}.unlock()\n", id);
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
            3,
        ]);
        Ok(())
    }

    #[inline]
    pub fn send_unlock(
        &self,
    ) {
        let res = self.try_send_unlock(
        );
        if let Err(e) = res {
            log_send("weston_desktop_shell.unlock", &e);
        }
    }

    /// Since when the set_grab_surface message is available.
    pub const MSG__SET_GRAB_SURFACE__SINCE: u32 = 1;

    /// set grab surface
    ///
    /// The surface set by this request will receive a fake
    /// pointer.enter event during grabs at position 0, 0 and is
    /// expected to set an appropriate cursor image as described by
    /// the grab_cursor event sent just before the enter event.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn try_send_set_grab_surface(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            surface,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_desktop_shell#{}.set_grab_surface(surface: wl_surface#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
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
        ]);
        Ok(())
    }

    /// set grab surface
    ///
    /// The surface set by this request will receive a fake
    /// pointer.enter event during grabs at position 0, 0 and is
    /// expected to set an appropriate cursor image as described by
    /// the grab_cursor event sent just before the enter event.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn send_set_grab_surface(
        &self,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_set_grab_surface(
            surface,
        );
        if let Err(e) = res {
            log_send("weston_desktop_shell.set_grab_surface", &e);
        }
    }

    /// Since when the configure message is available.
    pub const MSG__CONFIGURE__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `edges`:
    /// - `surface`:
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn try_send_configure(
        &self,
        edges: u32,
        surface: &Rc<WlSurface>,
        width: i32,
        height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            edges,
            surface,
            width,
            height,
        );
        let arg1 = arg1.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg1.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("surface", client.endpoint.id)));
        }
        let arg1_id = arg1.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: i32, arg3: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_desktop_shell#{}.configure(edges: {}, surface: wl_surface#{}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1_id, arg2, arg3);
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
            arg0,
            arg1_id,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `edges`:
    /// - `surface`:
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn send_configure(
        &self,
        edges: u32,
        surface: &Rc<WlSurface>,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_configure(
            edges,
            surface,
            width,
            height,
        );
        if let Err(e) = res {
            log_send("weston_desktop_shell.configure", &e);
        }
    }

    /// Since when the prepare_lock_surface message is available.
    pub const MSG__PREPARE_LOCK_SURFACE__SINCE: u32 = 1;

    /// tell the client to create, set the lock surface
    ///
    /// Tell the client we want it to create and set the lock surface, which is
    /// a GUI asking the user to unlock the screen. The lock surface is
    /// announced with 'set_lock_surface'. Whether or not the client actually
    /// implements locking, it MUST send 'unlock' request to let the normal
    /// desktop resume.
    #[inline]
    pub fn try_send_prepare_lock_surface(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_desktop_shell#{}.prepare_lock_surface()\n", client_id, id);
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
            1,
        ]);
        Ok(())
    }

    /// tell the client to create, set the lock surface
    ///
    /// Tell the client we want it to create and set the lock surface, which is
    /// a GUI asking the user to unlock the screen. The lock surface is
    /// announced with 'set_lock_surface'. Whether or not the client actually
    /// implements locking, it MUST send 'unlock' request to let the normal
    /// desktop resume.
    #[inline]
    pub fn send_prepare_lock_surface(
        &self,
    ) {
        let res = self.try_send_prepare_lock_surface(
        );
        if let Err(e) = res {
            log_send("weston_desktop_shell.prepare_lock_surface", &e);
        }
    }

    /// Since when the grab_cursor message is available.
    pub const MSG__GRAB_CURSOR__SINCE: u32 = 1;

    /// tell client what cursor to show during a grab
    ///
    /// This event will be sent immediately before a fake enter event on the
    /// grab surface.
    ///
    /// # Arguments
    ///
    /// - `cursor`:
    #[inline]
    pub fn try_send_grab_cursor(
        &self,
        cursor: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            cursor,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_desktop_shell#{}.grab_cursor(cursor: {})\n", client_id, id, arg0);
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
            arg0,
        ]);
        Ok(())
    }

    /// tell client what cursor to show during a grab
    ///
    /// This event will be sent immediately before a fake enter event on the
    /// grab surface.
    ///
    /// # Arguments
    ///
    /// - `cursor`:
    #[inline]
    pub fn send_grab_cursor(
        &self,
        cursor: u32,
    ) {
        let res = self.try_send_grab_cursor(
            cursor,
        );
        if let Err(e) = res {
            log_send("weston_desktop_shell.grab_cursor", &e);
        }
    }

    /// Since when the desktop_ready message is available.
    pub const MSG__DESKTOP_READY__SINCE: u32 = 1;

    /// desktop is ready to be shown
    ///
    /// Tell the server, that enough desktop elements have been drawn
    /// to make the desktop look ready for use. During start-up, the
    /// server can wait for this request with a black screen before
    /// starting to fade in the desktop, for instance. If the client
    /// parts of a desktop take a long time to initialize, we avoid
    /// showing temporary garbage.
    #[inline]
    pub fn try_send_desktop_ready(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_desktop_shell#{}.desktop_ready()\n", id);
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

    /// desktop is ready to be shown
    ///
    /// Tell the server, that enough desktop elements have been drawn
    /// to make the desktop look ready for use. During start-up, the
    /// server can wait for this request with a black screen before
    /// starting to fade in the desktop, for instance. If the client
    /// parts of a desktop take a long time to initialize, we avoid
    /// showing temporary garbage.
    #[inline]
    pub fn send_desktop_ready(
        &self,
    ) {
        let res = self.try_send_desktop_ready(
        );
        if let Err(e) = res {
            log_send("weston_desktop_shell.desktop_ready", &e);
        }
    }

    /// Since when the set_panel_position message is available.
    pub const MSG__SET_PANEL_POSITION__SINCE: u32 = 1;

    /// set panel position
    ///
    /// Tell the shell which side of the screen the panel is
    /// located. This is so that new windows do not overlap the panel
    /// and maximized windows maximize properly.
    ///
    /// # Arguments
    ///
    /// - `position`:
    #[inline]
    pub fn try_send_set_panel_position(
        &self,
        position: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            position,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_desktop_shell#{}.set_panel_position(position: {})\n", id, arg0);
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
            6,
            arg0,
        ]);
        Ok(())
    }

    /// set panel position
    ///
    /// Tell the shell which side of the screen the panel is
    /// located. This is so that new windows do not overlap the panel
    /// and maximized windows maximize properly.
    ///
    /// # Arguments
    ///
    /// - `position`:
    #[inline]
    pub fn send_set_panel_position(
        &self,
        position: u32,
    ) {
        let res = self.try_send_set_panel_position(
            position,
        );
        if let Err(e) = res {
            log_send("weston_desktop_shell.set_panel_position", &e);
        }
    }
}

/// A message handler for [`WestonDesktopShell`] proxies.
pub trait WestonDesktopShellHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WestonDesktopShell>) {
        slf.core.delete_id();
    }

    /// # Arguments
    ///
    /// - `output`:
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_background(
        &mut self,
        slf: &Rc<WestonDesktopShell>,
        output: &Rc<WlOutput>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_background(
            output,
            surface,
        );
        if let Err(e) = res {
            log_forward("weston_desktop_shell.set_background", &e);
        }
    }

    /// # Arguments
    ///
    /// - `output`:
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_panel(
        &mut self,
        slf: &Rc<WestonDesktopShell>,
        output: &Rc<WlOutput>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_panel(
            output,
            surface,
        );
        if let Err(e) = res {
            log_forward("weston_desktop_shell.set_panel", &e);
        }
    }

    /// # Arguments
    ///
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_lock_surface(
        &mut self,
        slf: &Rc<WestonDesktopShell>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_lock_surface(
            surface,
        );
        if let Err(e) = res {
            log_forward("weston_desktop_shell.set_lock_surface", &e);
        }
    }

    #[inline]
    fn handle_unlock(
        &mut self,
        slf: &Rc<WestonDesktopShell>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_unlock(
        );
        if let Err(e) = res {
            log_forward("weston_desktop_shell.unlock", &e);
        }
    }

    /// set grab surface
    ///
    /// The surface set by this request will receive a fake
    /// pointer.enter event during grabs at position 0, 0 and is
    /// expected to set an appropriate cursor image as described by
    /// the grab_cursor event sent just before the enter event.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_grab_surface(
        &mut self,
        slf: &Rc<WestonDesktopShell>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_grab_surface(
            surface,
        );
        if let Err(e) = res {
            log_forward("weston_desktop_shell.set_grab_surface", &e);
        }
    }

    /// # Arguments
    ///
    /// - `edges`:
    /// - `surface`:
    /// - `width`:
    /// - `height`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_configure(
        &mut self,
        slf: &Rc<WestonDesktopShell>,
        edges: u32,
        surface: &Rc<WlSurface>,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = surface.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = slf.try_send_configure(
            edges,
            surface,
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("weston_desktop_shell.configure", &e);
        }
    }

    /// tell the client to create, set the lock surface
    ///
    /// Tell the client we want it to create and set the lock surface, which is
    /// a GUI asking the user to unlock the screen. The lock surface is
    /// announced with 'set_lock_surface'. Whether or not the client actually
    /// implements locking, it MUST send 'unlock' request to let the normal
    /// desktop resume.
    #[inline]
    fn handle_prepare_lock_surface(
        &mut self,
        slf: &Rc<WestonDesktopShell>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_prepare_lock_surface(
        );
        if let Err(e) = res {
            log_forward("weston_desktop_shell.prepare_lock_surface", &e);
        }
    }

    /// tell client what cursor to show during a grab
    ///
    /// This event will be sent immediately before a fake enter event on the
    /// grab surface.
    ///
    /// # Arguments
    ///
    /// - `cursor`:
    #[inline]
    fn handle_grab_cursor(
        &mut self,
        slf: &Rc<WestonDesktopShell>,
        cursor: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_grab_cursor(
            cursor,
        );
        if let Err(e) = res {
            log_forward("weston_desktop_shell.grab_cursor", &e);
        }
    }

    /// desktop is ready to be shown
    ///
    /// Tell the server, that enough desktop elements have been drawn
    /// to make the desktop look ready for use. During start-up, the
    /// server can wait for this request with a black screen before
    /// starting to fade in the desktop, for instance. If the client
    /// parts of a desktop take a long time to initialize, we avoid
    /// showing temporary garbage.
    #[inline]
    fn handle_desktop_ready(
        &mut self,
        slf: &Rc<WestonDesktopShell>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_desktop_ready(
        );
        if let Err(e) = res {
            log_forward("weston_desktop_shell.desktop_ready", &e);
        }
    }

    /// set panel position
    ///
    /// Tell the shell which side of the screen the panel is
    /// located. This is so that new windows do not overlap the panel
    /// and maximized windows maximize properly.
    ///
    /// # Arguments
    ///
    /// - `position`:
    #[inline]
    fn handle_set_panel_position(
        &mut self,
        slf: &Rc<WestonDesktopShell>,
        position: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_panel_position(
            position,
        );
        if let Err(e) = res {
            log_forward("weston_desktop_shell.set_panel_position", &e);
        }
    }
}

impl ObjectPrivate for WestonDesktopShell {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WestonDesktopShell, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_desktop_shell#{}.set_background(output: wl_output#{}, surface: wl_surface#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlOutput>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("output", o.core().interface, ObjectInterface::WlOutput)));
                };
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_set_background(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_background(&self, arg0, arg1);
                }
            }
            1 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_desktop_shell#{}.set_panel(output: wl_output#{}, surface: wl_surface#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlOutput>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("output", o.core().interface, ObjectInterface::WlOutput)));
                };
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_set_panel(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_panel(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_desktop_shell#{}.set_lock_surface(surface: wl_surface#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_lock_surface(&self, arg0);
                } else {
                    DefaultHandler.handle_set_lock_surface(&self, arg0);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_desktop_shell#{}.unlock()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_unlock(&self);
                } else {
                    DefaultHandler.handle_unlock(&self);
                }
            }
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_desktop_shell#{}.set_grab_surface(surface: wl_surface#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_grab_surface(&self, arg0);
                } else {
                    DefaultHandler.handle_set_grab_surface(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_desktop_shell#{}.desktop_ready()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_desktop_ready(&self);
                } else {
                    DefaultHandler.handle_desktop_ready(&self);
                }
            }
            6 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_desktop_shell#{}.set_panel_position(position: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_panel_position(&self, arg0);
                } else {
                    DefaultHandler.handle_set_panel_position(&self, arg0);
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
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 24)));
                };
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: i32, arg3: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_desktop_shell#{}.configure(edges: {}, surface: wl_surface#{}, width: {}, height: {})\n", id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2, arg3);
                }
                let arg1_id = arg1;
                let Some(arg1) = server.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = server.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_configure(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_configure(&self, arg0, arg1, arg2, arg3);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_desktop_shell#{}.prepare_lock_surface()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_prepare_lock_surface(&self);
                } else {
                    DefaultHandler.handle_prepare_lock_surface(&self);
                }
            }
            2 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_desktop_shell#{}.grab_cursor(cursor: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_grab_cursor(&self, arg0);
                } else {
                    DefaultHandler.handle_grab_cursor(&self, arg0);
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
            0 => "set_background",
            1 => "set_panel",
            2 => "set_lock_surface",
            3 => "unlock",
            4 => "set_grab_surface",
            5 => "desktop_ready",
            6 => "set_panel_position",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "configure",
            1 => "prepare_lock_surface",
            2 => "grab_cursor",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WestonDesktopShell {
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

impl WestonDesktopShell {
    /// Since when the cursor.none enum variant is available.
    pub const ENM__CURSOR_NONE__SINCE: u32 = 1;
    /// Since when the cursor.resize_top enum variant is available.
    pub const ENM__CURSOR_RESIZE_TOP__SINCE: u32 = 1;
    /// Since when the cursor.resize_bottom enum variant is available.
    pub const ENM__CURSOR_RESIZE_BOTTOM__SINCE: u32 = 1;
    /// Since when the cursor.arrow enum variant is available.
    pub const ENM__CURSOR_ARROW__SINCE: u32 = 1;
    /// Since when the cursor.resize_left enum variant is available.
    pub const ENM__CURSOR_RESIZE_LEFT__SINCE: u32 = 1;
    /// Since when the cursor.resize_top_left enum variant is available.
    pub const ENM__CURSOR_RESIZE_TOP_LEFT__SINCE: u32 = 1;
    /// Since when the cursor.resize_bottom_left enum variant is available.
    pub const ENM__CURSOR_RESIZE_BOTTOM_LEFT__SINCE: u32 = 1;
    /// Since when the cursor.move enum variant is available.
    pub const ENM__CURSOR_MOVE__SINCE: u32 = 1;
    /// Since when the cursor.resize_right enum variant is available.
    pub const ENM__CURSOR_RESIZE_RIGHT__SINCE: u32 = 1;
    /// Since when the cursor.resize_top_right enum variant is available.
    pub const ENM__CURSOR_RESIZE_TOP_RIGHT__SINCE: u32 = 1;
    /// Since when the cursor.resize_bottom_right enum variant is available.
    pub const ENM__CURSOR_RESIZE_BOTTOM_RIGHT__SINCE: u32 = 1;
    /// Since when the cursor.busy enum variant is available.
    pub const ENM__CURSOR_BUSY__SINCE: u32 = 1;

    /// Since when the panel_position.top enum variant is available.
    pub const ENM__PANEL_POSITION_TOP__SINCE: u32 = 1;
    /// Since when the panel_position.bottom enum variant is available.
    pub const ENM__PANEL_POSITION_BOTTOM__SINCE: u32 = 1;
    /// Since when the panel_position.left enum variant is available.
    pub const ENM__PANEL_POSITION_LEFT__SINCE: u32 = 1;
    /// Since when the panel_position.right enum variant is available.
    pub const ENM__PANEL_POSITION_RIGHT__SINCE: u32 = 1;

    /// Since when the error.invalid_argument enum variant is available.
    pub const ENM__ERROR_INVALID_ARGUMENT__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WestonDesktopShellCursor(pub u32);

impl WestonDesktopShellCursor {
    pub const NONE: Self = Self(0);

    pub const RESIZE_TOP: Self = Self(1);

    pub const RESIZE_BOTTOM: Self = Self(2);

    pub const ARROW: Self = Self(3);

    pub const RESIZE_LEFT: Self = Self(4);

    pub const RESIZE_TOP_LEFT: Self = Self(5);

    pub const RESIZE_BOTTOM_LEFT: Self = Self(6);

    pub const MOVE: Self = Self(7);

    pub const RESIZE_RIGHT: Self = Self(8);

    pub const RESIZE_TOP_RIGHT: Self = Self(9);

    pub const RESIZE_BOTTOM_RIGHT: Self = Self(10);

    pub const BUSY: Self = Self(11);
}

impl Debug for WestonDesktopShellCursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NONE => "NONE",
            Self::RESIZE_TOP => "RESIZE_TOP",
            Self::RESIZE_BOTTOM => "RESIZE_BOTTOM",
            Self::ARROW => "ARROW",
            Self::RESIZE_LEFT => "RESIZE_LEFT",
            Self::RESIZE_TOP_LEFT => "RESIZE_TOP_LEFT",
            Self::RESIZE_BOTTOM_LEFT => "RESIZE_BOTTOM_LEFT",
            Self::MOVE => "MOVE",
            Self::RESIZE_RIGHT => "RESIZE_RIGHT",
            Self::RESIZE_TOP_RIGHT => "RESIZE_TOP_RIGHT",
            Self::RESIZE_BOTTOM_RIGHT => "RESIZE_BOTTOM_RIGHT",
            Self::BUSY => "BUSY",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WestonDesktopShellPanelPosition(pub u32);

impl WestonDesktopShellPanelPosition {
    pub const TOP: Self = Self(0);

    pub const BOTTOM: Self = Self(1);

    pub const LEFT: Self = Self(2);

    pub const RIGHT: Self = Self(3);
}

impl Debug for WestonDesktopShellPanelPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::TOP => "TOP",
            Self::BOTTOM => "BOTTOM",
            Self::LEFT => "LEFT",
            Self::RIGHT => "RIGHT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WestonDesktopShellError(pub u32);

impl WestonDesktopShellError {
    /// an invalid argument was provided in a request
    pub const INVALID_ARGUMENT: Self = Self(0);
}

impl Debug for WestonDesktopShellError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_ARGUMENT => "INVALID_ARGUMENT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
