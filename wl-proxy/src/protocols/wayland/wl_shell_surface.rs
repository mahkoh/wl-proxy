//! desktop-style metadata interface
//!
//! An interface that may be implemented by a wl_surface, for
//! implementations that provide a desktop-style user interface.
//!
//! It provides requests to treat surfaces like toplevel, fullscreen
//! or popup windows, move, resize or maximize them, associate
//! metadata like title and class, etc.
//!
//! On the server side the object is automatically destroyed when
//! the related wl_surface is destroyed. On the client side,
//! wl_shell_surface_destroy() must be called before destroying
//! the wl_surface object.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wl_shell_surface object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlShellSurface {
    core: ObjectCore,
    handler: HandlerHolder<dyn WlShellSurfaceHandler>,
}

struct DefaultHandler;

impl WlShellSurfaceHandler for DefaultHandler { }

impl ConcreteObject for WlShellSurface {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WlShellSurface;
    const INTERFACE_NAME: &str = "wl_shell_surface";
}

impl WlShellSurface {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WlShellSurfaceHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WlShellSurfaceHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlShellSurface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlShellSurface")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlShellSurface {
    /// Since when the pong message is available.
    pub const MSG__PONG__SINCE: u32 = 1;

    /// respond to a ping event
    ///
    /// A client must respond to a ping event with a pong request or
    /// the client may be deemed unresponsive.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the ping event
    #[inline]
    pub fn try_send_pong(
        &self,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            serial,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_shell_surface#{}.pong(serial: {})\n", id, arg0);
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
            0,
            arg0,
        ]);
        Ok(())
    }

    /// respond to a ping event
    ///
    /// A client must respond to a ping event with a pong request or
    /// the client may be deemed unresponsive.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the ping event
    #[inline]
    pub fn send_pong(
        &self,
        serial: u32,
    ) {
        let res = self.try_send_pong(
            serial,
        );
        if let Err(e) = res {
            log_send("wl_shell_surface.pong", &e);
        }
    }

    /// Since when the move message is available.
    pub const MSG__MOVE__SINCE: u32 = 1;

    /// start an interactive move
    ///
    /// Start a pointer-driven move of the surface.
    ///
    /// This request must be used in response to a button press event.
    /// The server may ignore move requests depending on the state of
    /// the surface (e.g. fullscreen or maximized).
    ///
    /// # Arguments
    ///
    /// - `seat`: seat whose pointer is used
    /// - `serial`: serial number of the implicit grab on the pointer
    #[inline]
    pub fn try_send_move(
        &self,
        seat: &Rc<WlSeat>,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            seat,
            serial,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("seat"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_shell_surface#{}.move(seat: wl_seat#{}, serial: {})\n", id, arg0, arg1);
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
            1,
            arg0_id,
            arg1,
        ]);
        Ok(())
    }

    /// start an interactive move
    ///
    /// Start a pointer-driven move of the surface.
    ///
    /// This request must be used in response to a button press event.
    /// The server may ignore move requests depending on the state of
    /// the surface (e.g. fullscreen or maximized).
    ///
    /// # Arguments
    ///
    /// - `seat`: seat whose pointer is used
    /// - `serial`: serial number of the implicit grab on the pointer
    #[inline]
    pub fn send_move(
        &self,
        seat: &Rc<WlSeat>,
        serial: u32,
    ) {
        let res = self.try_send_move(
            seat,
            serial,
        );
        if let Err(e) = res {
            log_send("wl_shell_surface.move", &e);
        }
    }

    /// Since when the resize message is available.
    pub const MSG__RESIZE__SINCE: u32 = 1;

    /// start an interactive resize
    ///
    /// Start a pointer-driven resizing of the surface.
    ///
    /// This request must be used in response to a button press event.
    /// The server may ignore resize requests depending on the state of
    /// the surface (e.g. fullscreen or maximized).
    ///
    /// # Arguments
    ///
    /// - `seat`: seat whose pointer is used
    /// - `serial`: serial number of the implicit grab on the pointer
    /// - `edges`: which edge or corner is being dragged
    #[inline]
    pub fn try_send_resize(
        &self,
        seat: &Rc<WlSeat>,
        serial: u32,
        edges: WlShellSurfaceResize,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            seat,
            serial,
            edges,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("seat"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: WlShellSurfaceResize) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_shell_surface#{}.resize(seat: wl_seat#{}, serial: {}, edges: {:?})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1, arg2);
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
            arg1,
            arg2.0,
        ]);
        Ok(())
    }

    /// start an interactive resize
    ///
    /// Start a pointer-driven resizing of the surface.
    ///
    /// This request must be used in response to a button press event.
    /// The server may ignore resize requests depending on the state of
    /// the surface (e.g. fullscreen or maximized).
    ///
    /// # Arguments
    ///
    /// - `seat`: seat whose pointer is used
    /// - `serial`: serial number of the implicit grab on the pointer
    /// - `edges`: which edge or corner is being dragged
    #[inline]
    pub fn send_resize(
        &self,
        seat: &Rc<WlSeat>,
        serial: u32,
        edges: WlShellSurfaceResize,
    ) {
        let res = self.try_send_resize(
            seat,
            serial,
            edges,
        );
        if let Err(e) = res {
            log_send("wl_shell_surface.resize", &e);
        }
    }

    /// Since when the set_toplevel message is available.
    pub const MSG__SET_TOPLEVEL__SINCE: u32 = 1;

    /// make the surface a toplevel surface
    ///
    /// Map the surface as a toplevel surface.
    ///
    /// A toplevel surface is not fullscreen, maximized or transient.
    #[inline]
    pub fn try_send_set_toplevel(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_shell_surface#{}.set_toplevel()\n", id);
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

    /// make the surface a toplevel surface
    ///
    /// Map the surface as a toplevel surface.
    ///
    /// A toplevel surface is not fullscreen, maximized or transient.
    #[inline]
    pub fn send_set_toplevel(
        &self,
    ) {
        let res = self.try_send_set_toplevel(
        );
        if let Err(e) = res {
            log_send("wl_shell_surface.set_toplevel", &e);
        }
    }

    /// Since when the set_transient message is available.
    pub const MSG__SET_TRANSIENT__SINCE: u32 = 1;

    /// make the surface a transient surface
    ///
    /// Map the surface relative to an existing surface.
    ///
    /// The x and y arguments specify the location of the upper left
    /// corner of the surface relative to the upper left corner of the
    /// parent surface, in surface-local coordinates.
    ///
    /// The flags argument controls details of the transient behaviour.
    ///
    /// # Arguments
    ///
    /// - `parent`: parent surface
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    /// - `flags`: transient surface behavior
    #[inline]
    pub fn try_send_set_transient(
        &self,
        parent: &Rc<WlSurface>,
        x: i32,
        y: i32,
        flags: WlShellSurfaceTransient,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            parent,
            x,
            y,
            flags,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("parent"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: i32, arg2: i32, arg3: WlShellSurfaceTransient) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_shell_surface#{}.set_transient(parent: wl_surface#{}, x: {}, y: {}, flags: {:?})\n", id, arg0, arg1, arg2, arg3);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1, arg2, arg3);
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
            arg1 as u32,
            arg2 as u32,
            arg3.0,
        ]);
        Ok(())
    }

    /// make the surface a transient surface
    ///
    /// Map the surface relative to an existing surface.
    ///
    /// The x and y arguments specify the location of the upper left
    /// corner of the surface relative to the upper left corner of the
    /// parent surface, in surface-local coordinates.
    ///
    /// The flags argument controls details of the transient behaviour.
    ///
    /// # Arguments
    ///
    /// - `parent`: parent surface
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    /// - `flags`: transient surface behavior
    #[inline]
    pub fn send_set_transient(
        &self,
        parent: &Rc<WlSurface>,
        x: i32,
        y: i32,
        flags: WlShellSurfaceTransient,
    ) {
        let res = self.try_send_set_transient(
            parent,
            x,
            y,
            flags,
        );
        if let Err(e) = res {
            log_send("wl_shell_surface.set_transient", &e);
        }
    }

    /// Since when the set_fullscreen message is available.
    pub const MSG__SET_FULLSCREEN__SINCE: u32 = 1;

    /// make the surface a fullscreen surface
    ///
    /// Map the surface as a fullscreen surface.
    ///
    /// If an output parameter is given then the surface will be made
    /// fullscreen on that output. If the client does not specify the
    /// output then the compositor will apply its policy - usually
    /// choosing the output on which the surface has the biggest surface
    /// area.
    ///
    /// The client may specify a method to resolve a size conflict
    /// between the output size and the surface size - this is provided
    /// through the method parameter.
    ///
    /// The framerate parameter is used only when the method is set
    /// to "driver", to indicate the preferred framerate. A value of 0
    /// indicates that the client does not care about framerate.  The
    /// framerate is specified in mHz, that is framerate of 60000 is 60Hz.
    ///
    /// A method of "scale" or "driver" implies a scaling operation of
    /// the surface, either via a direct scaling operation or a change of
    /// the output mode. This will override any kind of output scaling, so
    /// that mapping a surface with a buffer size equal to the mode can
    /// fill the screen independent of buffer_scale.
    ///
    /// A method of "fill" means we don't scale up the buffer, however
    /// any output scale is applied. This means that you may run into
    /// an edge case where the application maps a buffer with the same
    /// size of the output mode but buffer_scale 1 (thus making a
    /// surface larger than the output). In this case it is allowed to
    /// downscale the results to fit the screen.
    ///
    /// The compositor must reply to this request with a configure event
    /// with the dimensions for the output on which the surface will
    /// be made fullscreen.
    ///
    /// # Arguments
    ///
    /// - `method`: method for resolving size conflict
    /// - `framerate`: framerate in mHz
    /// - `output`: output on which the surface is to be fullscreen
    #[inline]
    pub fn try_send_set_fullscreen(
        &self,
        method: WlShellSurfaceFullscreenMethod,
        framerate: u32,
        output: Option<&Rc<WlOutput>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            method,
            framerate,
            output,
        );
        let arg2 = arg2.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg2_id = match arg2 {
            None => 0,
            Some(arg2) => match arg2.server_obj_id.get() {
                None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("output"))),
                Some(id) => id,
            },
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: WlShellSurfaceFullscreenMethod, arg1: u32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_shell_surface#{}.set_fullscreen(method: {:?}, framerate: {}, output: wl_output#{})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2_id);
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
            arg0.0,
            arg1,
            arg2_id,
        ]);
        Ok(())
    }

    /// make the surface a fullscreen surface
    ///
    /// Map the surface as a fullscreen surface.
    ///
    /// If an output parameter is given then the surface will be made
    /// fullscreen on that output. If the client does not specify the
    /// output then the compositor will apply its policy - usually
    /// choosing the output on which the surface has the biggest surface
    /// area.
    ///
    /// The client may specify a method to resolve a size conflict
    /// between the output size and the surface size - this is provided
    /// through the method parameter.
    ///
    /// The framerate parameter is used only when the method is set
    /// to "driver", to indicate the preferred framerate. A value of 0
    /// indicates that the client does not care about framerate.  The
    /// framerate is specified in mHz, that is framerate of 60000 is 60Hz.
    ///
    /// A method of "scale" or "driver" implies a scaling operation of
    /// the surface, either via a direct scaling operation or a change of
    /// the output mode. This will override any kind of output scaling, so
    /// that mapping a surface with a buffer size equal to the mode can
    /// fill the screen independent of buffer_scale.
    ///
    /// A method of "fill" means we don't scale up the buffer, however
    /// any output scale is applied. This means that you may run into
    /// an edge case where the application maps a buffer with the same
    /// size of the output mode but buffer_scale 1 (thus making a
    /// surface larger than the output). In this case it is allowed to
    /// downscale the results to fit the screen.
    ///
    /// The compositor must reply to this request with a configure event
    /// with the dimensions for the output on which the surface will
    /// be made fullscreen.
    ///
    /// # Arguments
    ///
    /// - `method`: method for resolving size conflict
    /// - `framerate`: framerate in mHz
    /// - `output`: output on which the surface is to be fullscreen
    #[inline]
    pub fn send_set_fullscreen(
        &self,
        method: WlShellSurfaceFullscreenMethod,
        framerate: u32,
        output: Option<&Rc<WlOutput>>,
    ) {
        let res = self.try_send_set_fullscreen(
            method,
            framerate,
            output,
        );
        if let Err(e) = res {
            log_send("wl_shell_surface.set_fullscreen", &e);
        }
    }

    /// Since when the set_popup message is available.
    pub const MSG__SET_POPUP__SINCE: u32 = 1;

    /// make the surface a popup surface
    ///
    /// Map the surface as a popup.
    ///
    /// A popup surface is a transient surface with an added pointer
    /// grab.
    ///
    /// An existing implicit grab will be changed to owner-events mode,
    /// and the popup grab will continue after the implicit grab ends
    /// (i.e. releasing the mouse button does not cause the popup to
    /// be unmapped).
    ///
    /// The popup grab continues until the window is destroyed or a
    /// mouse button is pressed in any other client's window. A click
    /// in any of the client's surfaces is reported as normal, however,
    /// clicks in other clients' surfaces will be discarded and trigger
    /// the callback.
    ///
    /// The x and y arguments specify the location of the upper left
    /// corner of the surface relative to the upper left corner of the
    /// parent surface, in surface-local coordinates.
    ///
    /// # Arguments
    ///
    /// - `seat`: seat whose pointer is used
    /// - `serial`: serial number of the implicit grab on the pointer
    /// - `parent`: parent surface
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    /// - `flags`: transient surface behavior
    #[inline]
    pub fn try_send_set_popup(
        &self,
        seat: &Rc<WlSeat>,
        serial: u32,
        parent: &Rc<WlSurface>,
        x: i32,
        y: i32,
        flags: WlShellSurfaceTransient,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
        ) = (
            seat,
            serial,
            parent,
            x,
            y,
            flags,
        );
        let arg0 = arg0.core();
        let arg2 = arg2.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("seat"))),
            Some(id) => id,
        };
        let arg2_id = match arg2.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("parent"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: i32, arg4: i32, arg5: WlShellSurfaceTransient) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_shell_surface#{}.set_popup(seat: wl_seat#{}, serial: {}, parent: wl_surface#{}, x: {}, y: {}, flags: {:?})\n", id, arg0, arg1, arg2, arg3, arg4, arg5);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1, arg2_id, arg3, arg4, arg5);
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
            arg0_id,
            arg1,
            arg2_id,
            arg3 as u32,
            arg4 as u32,
            arg5.0,
        ]);
        Ok(())
    }

    /// make the surface a popup surface
    ///
    /// Map the surface as a popup.
    ///
    /// A popup surface is a transient surface with an added pointer
    /// grab.
    ///
    /// An existing implicit grab will be changed to owner-events mode,
    /// and the popup grab will continue after the implicit grab ends
    /// (i.e. releasing the mouse button does not cause the popup to
    /// be unmapped).
    ///
    /// The popup grab continues until the window is destroyed or a
    /// mouse button is pressed in any other client's window. A click
    /// in any of the client's surfaces is reported as normal, however,
    /// clicks in other clients' surfaces will be discarded and trigger
    /// the callback.
    ///
    /// The x and y arguments specify the location of the upper left
    /// corner of the surface relative to the upper left corner of the
    /// parent surface, in surface-local coordinates.
    ///
    /// # Arguments
    ///
    /// - `seat`: seat whose pointer is used
    /// - `serial`: serial number of the implicit grab on the pointer
    /// - `parent`: parent surface
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    /// - `flags`: transient surface behavior
    #[inline]
    pub fn send_set_popup(
        &self,
        seat: &Rc<WlSeat>,
        serial: u32,
        parent: &Rc<WlSurface>,
        x: i32,
        y: i32,
        flags: WlShellSurfaceTransient,
    ) {
        let res = self.try_send_set_popup(
            seat,
            serial,
            parent,
            x,
            y,
            flags,
        );
        if let Err(e) = res {
            log_send("wl_shell_surface.set_popup", &e);
        }
    }

    /// Since when the set_maximized message is available.
    pub const MSG__SET_MAXIMIZED__SINCE: u32 = 1;

    /// make the surface a maximized surface
    ///
    /// Map the surface as a maximized surface.
    ///
    /// If an output parameter is given then the surface will be
    /// maximized on that output. If the client does not specify the
    /// output then the compositor will apply its policy - usually
    /// choosing the output on which the surface has the biggest surface
    /// area.
    ///
    /// The compositor will reply with a configure event telling
    /// the expected new surface size. The operation is completed
    /// on the next buffer attach to this surface.
    ///
    /// A maximized surface typically fills the entire output it is
    /// bound to, except for desktop elements such as panels. This is
    /// the main difference between a maximized shell surface and a
    /// fullscreen shell surface.
    ///
    /// The details depend on the compositor implementation.
    ///
    /// # Arguments
    ///
    /// - `output`: output on which the surface is to be maximized
    #[inline]
    pub fn try_send_set_maximized(
        &self,
        output: Option<&Rc<WlOutput>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            output,
        );
        let arg0 = arg0.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("output"))),
                Some(id) => id,
            },
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_shell_surface#{}.set_maximized(output: wl_output#{})\n", id, arg0);
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
            7,
            arg0_id,
        ]);
        Ok(())
    }

    /// make the surface a maximized surface
    ///
    /// Map the surface as a maximized surface.
    ///
    /// If an output parameter is given then the surface will be
    /// maximized on that output. If the client does not specify the
    /// output then the compositor will apply its policy - usually
    /// choosing the output on which the surface has the biggest surface
    /// area.
    ///
    /// The compositor will reply with a configure event telling
    /// the expected new surface size. The operation is completed
    /// on the next buffer attach to this surface.
    ///
    /// A maximized surface typically fills the entire output it is
    /// bound to, except for desktop elements such as panels. This is
    /// the main difference between a maximized shell surface and a
    /// fullscreen shell surface.
    ///
    /// The details depend on the compositor implementation.
    ///
    /// # Arguments
    ///
    /// - `output`: output on which the surface is to be maximized
    #[inline]
    pub fn send_set_maximized(
        &self,
        output: Option<&Rc<WlOutput>>,
    ) {
        let res = self.try_send_set_maximized(
            output,
        );
        if let Err(e) = res {
            log_send("wl_shell_surface.set_maximized", &e);
        }
    }

    /// Since when the set_title message is available.
    pub const MSG__SET_TITLE__SINCE: u32 = 1;

    /// set surface title
    ///
    /// Set a short title for the surface.
    ///
    /// This string may be used to identify the surface in a task bar,
    /// window list, or other user interface elements provided by the
    /// compositor.
    ///
    /// The string must be encoded in UTF-8.
    ///
    /// # Arguments
    ///
    /// - `title`: surface title
    #[inline]
    pub fn try_send_set_title(
        &self,
        title: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            title,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_shell_surface#{}.set_title(title: {:?})\n", id, arg0);
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
            8,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// set surface title
    ///
    /// Set a short title for the surface.
    ///
    /// This string may be used to identify the surface in a task bar,
    /// window list, or other user interface elements provided by the
    /// compositor.
    ///
    /// The string must be encoded in UTF-8.
    ///
    /// # Arguments
    ///
    /// - `title`: surface title
    #[inline]
    pub fn send_set_title(
        &self,
        title: &str,
    ) {
        let res = self.try_send_set_title(
            title,
        );
        if let Err(e) = res {
            log_send("wl_shell_surface.set_title", &e);
        }
    }

    /// Since when the set_class message is available.
    pub const MSG__SET_CLASS__SINCE: u32 = 1;

    /// set surface class
    ///
    /// Set a class for the surface.
    ///
    /// The surface class identifies the general class of applications
    /// to which the surface belongs. A common convention is to use the
    /// file name (or the full path if it is a non-standard location) of
    /// the application's .desktop file as the class.
    ///
    /// # Arguments
    ///
    /// - `class_`: surface class
    #[inline]
    pub fn try_send_set_class(
        &self,
        class_: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            class_,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_shell_surface#{}.set_class(class_: {:?})\n", id, arg0);
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
            9,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// set surface class
    ///
    /// Set a class for the surface.
    ///
    /// The surface class identifies the general class of applications
    /// to which the surface belongs. A common convention is to use the
    /// file name (or the full path if it is a non-standard location) of
    /// the application's .desktop file as the class.
    ///
    /// # Arguments
    ///
    /// - `class_`: surface class
    #[inline]
    pub fn send_set_class(
        &self,
        class_: &str,
    ) {
        let res = self.try_send_set_class(
            class_,
        );
        if let Err(e) = res {
            log_send("wl_shell_surface.set_class", &e);
        }
    }

    /// Since when the ping message is available.
    pub const MSG__PING__SINCE: u32 = 1;

    /// ping client
    ///
    /// Ping a client to check if it is receiving events and sending
    /// requests. A client is expected to reply with a pong request.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the ping
    #[inline]
    pub fn try_send_ping(
        &self,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            serial,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_shell_surface#{}.ping(serial: {})\n", client_id, id, arg0);
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
            arg0,
        ]);
        Ok(())
    }

    /// ping client
    ///
    /// Ping a client to check if it is receiving events and sending
    /// requests. A client is expected to reply with a pong request.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the ping
    #[inline]
    pub fn send_ping(
        &self,
        serial: u32,
    ) {
        let res = self.try_send_ping(
            serial,
        );
        if let Err(e) = res {
            log_send("wl_shell_surface.ping", &e);
        }
    }

    /// Since when the configure message is available.
    pub const MSG__CONFIGURE__SINCE: u32 = 1;

    /// suggest resize
    ///
    /// The configure event asks the client to resize its surface.
    ///
    /// The size is a hint, in the sense that the client is free to
    /// ignore it if it doesn't resize, pick a smaller size (to
    /// satisfy aspect ratio or resize in steps of NxM pixels).
    ///
    /// The edges parameter provides a hint about how the surface
    /// was resized. The client may use this information to decide
    /// how to adjust its content to the new size (e.g. a scrolling
    /// area might adjust its content position to leave the viewable
    /// content unmoved).
    ///
    /// The client is free to dismiss all but the last configure
    /// event it received.
    ///
    /// The width and height arguments specify the size of the window
    /// in surface-local coordinates.
    ///
    /// # Arguments
    ///
    /// - `edges`: how the surface was resized
    /// - `width`: new width of the surface
    /// - `height`: new height of the surface
    #[inline]
    pub fn try_send_configure(
        &self,
        edges: WlShellSurfaceResize,
        width: i32,
        height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            edges,
            width,
            height,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: WlShellSurfaceResize, arg1: i32, arg2: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_shell_surface#{}.configure(edges: {:?}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1, arg2);
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
            arg1 as u32,
            arg2 as u32,
        ]);
        Ok(())
    }

    /// suggest resize
    ///
    /// The configure event asks the client to resize its surface.
    ///
    /// The size is a hint, in the sense that the client is free to
    /// ignore it if it doesn't resize, pick a smaller size (to
    /// satisfy aspect ratio or resize in steps of NxM pixels).
    ///
    /// The edges parameter provides a hint about how the surface
    /// was resized. The client may use this information to decide
    /// how to adjust its content to the new size (e.g. a scrolling
    /// area might adjust its content position to leave the viewable
    /// content unmoved).
    ///
    /// The client is free to dismiss all but the last configure
    /// event it received.
    ///
    /// The width and height arguments specify the size of the window
    /// in surface-local coordinates.
    ///
    /// # Arguments
    ///
    /// - `edges`: how the surface was resized
    /// - `width`: new width of the surface
    /// - `height`: new height of the surface
    #[inline]
    pub fn send_configure(
        &self,
        edges: WlShellSurfaceResize,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_configure(
            edges,
            width,
            height,
        );
        if let Err(e) = res {
            log_send("wl_shell_surface.configure", &e);
        }
    }

    /// Since when the popup_done message is available.
    pub const MSG__POPUP_DONE__SINCE: u32 = 1;

    /// popup interaction is done
    ///
    /// The popup_done event is sent out when a popup grab is broken,
    /// that is, when the user clicks a surface that doesn't belong
    /// to the client owning the popup surface.
    #[inline]
    pub fn try_send_popup_done(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_shell_surface#{}.popup_done()\n", client_id, id);
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

    /// popup interaction is done
    ///
    /// The popup_done event is sent out when a popup grab is broken,
    /// that is, when the user clicks a surface that doesn't belong
    /// to the client owning the popup surface.
    #[inline]
    pub fn send_popup_done(
        &self,
    ) {
        let res = self.try_send_popup_done(
        );
        if let Err(e) = res {
            log_send("wl_shell_surface.popup_done", &e);
        }
    }
}

/// A message handler for [`WlShellSurface`] proxies.
pub trait WlShellSurfaceHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WlShellSurface>) {
        slf.core.delete_id();
    }

    /// respond to a ping event
    ///
    /// A client must respond to a ping event with a pong request or
    /// the client may be deemed unresponsive.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the ping event
    #[inline]
    fn handle_pong(
        &mut self,
        slf: &Rc<WlShellSurface>,
        serial: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_pong(
            serial,
        );
        if let Err(e) = res {
            log_forward("wl_shell_surface.pong", &e);
        }
    }

    /// start an interactive move
    ///
    /// Start a pointer-driven move of the surface.
    ///
    /// This request must be used in response to a button press event.
    /// The server may ignore move requests depending on the state of
    /// the surface (e.g. fullscreen or maximized).
    ///
    /// # Arguments
    ///
    /// - `seat`: seat whose pointer is used
    /// - `serial`: serial number of the implicit grab on the pointer
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_move(
        &mut self,
        slf: &Rc<WlShellSurface>,
        seat: &Rc<WlSeat>,
        serial: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_move(
            seat,
            serial,
        );
        if let Err(e) = res {
            log_forward("wl_shell_surface.move", &e);
        }
    }

    /// start an interactive resize
    ///
    /// Start a pointer-driven resizing of the surface.
    ///
    /// This request must be used in response to a button press event.
    /// The server may ignore resize requests depending on the state of
    /// the surface (e.g. fullscreen or maximized).
    ///
    /// # Arguments
    ///
    /// - `seat`: seat whose pointer is used
    /// - `serial`: serial number of the implicit grab on the pointer
    /// - `edges`: which edge or corner is being dragged
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_resize(
        &mut self,
        slf: &Rc<WlShellSurface>,
        seat: &Rc<WlSeat>,
        serial: u32,
        edges: WlShellSurfaceResize,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_resize(
            seat,
            serial,
            edges,
        );
        if let Err(e) = res {
            log_forward("wl_shell_surface.resize", &e);
        }
    }

    /// make the surface a toplevel surface
    ///
    /// Map the surface as a toplevel surface.
    ///
    /// A toplevel surface is not fullscreen, maximized or transient.
    #[inline]
    fn handle_set_toplevel(
        &mut self,
        slf: &Rc<WlShellSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_toplevel(
        );
        if let Err(e) = res {
            log_forward("wl_shell_surface.set_toplevel", &e);
        }
    }

    /// make the surface a transient surface
    ///
    /// Map the surface relative to an existing surface.
    ///
    /// The x and y arguments specify the location of the upper left
    /// corner of the surface relative to the upper left corner of the
    /// parent surface, in surface-local coordinates.
    ///
    /// The flags argument controls details of the transient behaviour.
    ///
    /// # Arguments
    ///
    /// - `parent`: parent surface
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    /// - `flags`: transient surface behavior
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_transient(
        &mut self,
        slf: &Rc<WlShellSurface>,
        parent: &Rc<WlSurface>,
        x: i32,
        y: i32,
        flags: WlShellSurfaceTransient,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_transient(
            parent,
            x,
            y,
            flags,
        );
        if let Err(e) = res {
            log_forward("wl_shell_surface.set_transient", &e);
        }
    }

    /// make the surface a fullscreen surface
    ///
    /// Map the surface as a fullscreen surface.
    ///
    /// If an output parameter is given then the surface will be made
    /// fullscreen on that output. If the client does not specify the
    /// output then the compositor will apply its policy - usually
    /// choosing the output on which the surface has the biggest surface
    /// area.
    ///
    /// The client may specify a method to resolve a size conflict
    /// between the output size and the surface size - this is provided
    /// through the method parameter.
    ///
    /// The framerate parameter is used only when the method is set
    /// to "driver", to indicate the preferred framerate. A value of 0
    /// indicates that the client does not care about framerate.  The
    /// framerate is specified in mHz, that is framerate of 60000 is 60Hz.
    ///
    /// A method of "scale" or "driver" implies a scaling operation of
    /// the surface, either via a direct scaling operation or a change of
    /// the output mode. This will override any kind of output scaling, so
    /// that mapping a surface with a buffer size equal to the mode can
    /// fill the screen independent of buffer_scale.
    ///
    /// A method of "fill" means we don't scale up the buffer, however
    /// any output scale is applied. This means that you may run into
    /// an edge case where the application maps a buffer with the same
    /// size of the output mode but buffer_scale 1 (thus making a
    /// surface larger than the output). In this case it is allowed to
    /// downscale the results to fit the screen.
    ///
    /// The compositor must reply to this request with a configure event
    /// with the dimensions for the output on which the surface will
    /// be made fullscreen.
    ///
    /// # Arguments
    ///
    /// - `method`: method for resolving size conflict
    /// - `framerate`: framerate in mHz
    /// - `output`: output on which the surface is to be fullscreen
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_fullscreen(
        &mut self,
        slf: &Rc<WlShellSurface>,
        method: WlShellSurfaceFullscreenMethod,
        framerate: u32,
        output: Option<&Rc<WlOutput>>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_fullscreen(
            method,
            framerate,
            output,
        );
        if let Err(e) = res {
            log_forward("wl_shell_surface.set_fullscreen", &e);
        }
    }

    /// make the surface a popup surface
    ///
    /// Map the surface as a popup.
    ///
    /// A popup surface is a transient surface with an added pointer
    /// grab.
    ///
    /// An existing implicit grab will be changed to owner-events mode,
    /// and the popup grab will continue after the implicit grab ends
    /// (i.e. releasing the mouse button does not cause the popup to
    /// be unmapped).
    ///
    /// The popup grab continues until the window is destroyed or a
    /// mouse button is pressed in any other client's window. A click
    /// in any of the client's surfaces is reported as normal, however,
    /// clicks in other clients' surfaces will be discarded and trigger
    /// the callback.
    ///
    /// The x and y arguments specify the location of the upper left
    /// corner of the surface relative to the upper left corner of the
    /// parent surface, in surface-local coordinates.
    ///
    /// # Arguments
    ///
    /// - `seat`: seat whose pointer is used
    /// - `serial`: serial number of the implicit grab on the pointer
    /// - `parent`: parent surface
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    /// - `flags`: transient surface behavior
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_popup(
        &mut self,
        slf: &Rc<WlShellSurface>,
        seat: &Rc<WlSeat>,
        serial: u32,
        parent: &Rc<WlSurface>,
        x: i32,
        y: i32,
        flags: WlShellSurfaceTransient,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_popup(
            seat,
            serial,
            parent,
            x,
            y,
            flags,
        );
        if let Err(e) = res {
            log_forward("wl_shell_surface.set_popup", &e);
        }
    }

    /// make the surface a maximized surface
    ///
    /// Map the surface as a maximized surface.
    ///
    /// If an output parameter is given then the surface will be
    /// maximized on that output. If the client does not specify the
    /// output then the compositor will apply its policy - usually
    /// choosing the output on which the surface has the biggest surface
    /// area.
    ///
    /// The compositor will reply with a configure event telling
    /// the expected new surface size. The operation is completed
    /// on the next buffer attach to this surface.
    ///
    /// A maximized surface typically fills the entire output it is
    /// bound to, except for desktop elements such as panels. This is
    /// the main difference between a maximized shell surface and a
    /// fullscreen shell surface.
    ///
    /// The details depend on the compositor implementation.
    ///
    /// # Arguments
    ///
    /// - `output`: output on which the surface is to be maximized
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_maximized(
        &mut self,
        slf: &Rc<WlShellSurface>,
        output: Option<&Rc<WlOutput>>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_maximized(
            output,
        );
        if let Err(e) = res {
            log_forward("wl_shell_surface.set_maximized", &e);
        }
    }

    /// set surface title
    ///
    /// Set a short title for the surface.
    ///
    /// This string may be used to identify the surface in a task bar,
    /// window list, or other user interface elements provided by the
    /// compositor.
    ///
    /// The string must be encoded in UTF-8.
    ///
    /// # Arguments
    ///
    /// - `title`: surface title
    #[inline]
    fn handle_set_title(
        &mut self,
        slf: &Rc<WlShellSurface>,
        title: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_title(
            title,
        );
        if let Err(e) = res {
            log_forward("wl_shell_surface.set_title", &e);
        }
    }

    /// set surface class
    ///
    /// Set a class for the surface.
    ///
    /// The surface class identifies the general class of applications
    /// to which the surface belongs. A common convention is to use the
    /// file name (or the full path if it is a non-standard location) of
    /// the application's .desktop file as the class.
    ///
    /// # Arguments
    ///
    /// - `class_`: surface class
    #[inline]
    fn handle_set_class(
        &mut self,
        slf: &Rc<WlShellSurface>,
        class_: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_class(
            class_,
        );
        if let Err(e) = res {
            log_forward("wl_shell_surface.set_class", &e);
        }
    }

    /// ping client
    ///
    /// Ping a client to check if it is receiving events and sending
    /// requests. A client is expected to reply with a pong request.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the ping
    #[inline]
    fn handle_ping(
        &mut self,
        slf: &Rc<WlShellSurface>,
        serial: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_ping(
            serial,
        );
        if let Err(e) = res {
            log_forward("wl_shell_surface.ping", &e);
        }
    }

    /// suggest resize
    ///
    /// The configure event asks the client to resize its surface.
    ///
    /// The size is a hint, in the sense that the client is free to
    /// ignore it if it doesn't resize, pick a smaller size (to
    /// satisfy aspect ratio or resize in steps of NxM pixels).
    ///
    /// The edges parameter provides a hint about how the surface
    /// was resized. The client may use this information to decide
    /// how to adjust its content to the new size (e.g. a scrolling
    /// area might adjust its content position to leave the viewable
    /// content unmoved).
    ///
    /// The client is free to dismiss all but the last configure
    /// event it received.
    ///
    /// The width and height arguments specify the size of the window
    /// in surface-local coordinates.
    ///
    /// # Arguments
    ///
    /// - `edges`: how the surface was resized
    /// - `width`: new width of the surface
    /// - `height`: new height of the surface
    #[inline]
    fn handle_configure(
        &mut self,
        slf: &Rc<WlShellSurface>,
        edges: WlShellSurfaceResize,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_configure(
            edges,
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("wl_shell_surface.configure", &e);
        }
    }

    /// popup interaction is done
    ///
    /// The popup_done event is sent out when a popup grab is broken,
    /// that is, when the user clicks a surface that doesn't belong
    /// to the client owning the popup surface.
    #[inline]
    fn handle_popup_done(
        &mut self,
        slf: &Rc<WlShellSurface>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_popup_done(
        );
        if let Err(e) = res {
            log_forward("wl_shell_surface.popup_done", &e);
        }
    }
}

impl ObjectPrivate for WlShellSurface {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WlShellSurface, version),
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_shell_surface#{}.pong(serial: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_pong(&self, arg0);
                } else {
                    DefaultHandler.handle_pong(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_shell_surface#{}.move(seat: wl_seat#{}, serial: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("seat", o.core().interface, ObjectInterface::WlSeat)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_move(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_move(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                let arg2 = WlShellSurfaceResize(arg2);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: WlShellSurfaceResize) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_shell_surface#{}.resize(seat: wl_seat#{}, serial: {}, edges: {:?})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("seat", o.core().interface, ObjectInterface::WlSeat)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_resize(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_resize(&self, arg0, arg1, arg2);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_shell_surface#{}.set_toplevel()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_toplevel(&self);
                } else {
                    DefaultHandler.handle_set_toplevel(&self);
                }
            }
            4 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 24)));
                };
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = WlShellSurfaceTransient(arg3);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: i32, arg2: i32, arg3: WlShellSurfaceTransient) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_shell_surface#{}.set_transient(parent: wl_surface#{}, x: {}, y: {}, flags: {:?})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("parent", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_transient(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_set_transient(&self, arg0, arg1, arg2, arg3);
                }
            }
            5 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                let arg0 = WlShellSurfaceFullscreenMethod(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: WlShellSurfaceFullscreenMethod, arg1: u32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_shell_surface#{}.set_fullscreen(method: {:?}, framerate: {}, output: wl_output#{})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg2 = if arg2 == 0 {
                    None
                } else {
                    let arg2_id = arg2;
                    let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg2_id)));
                    };
                    let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<WlOutput>() else {
                        let o = client.endpoint.lookup(arg2_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("output", o.core().interface, ObjectInterface::WlOutput)));
                    };
                    Some(arg2)
                };
                let arg2 = arg2.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_set_fullscreen(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_set_fullscreen(&self, arg0, arg1, arg2);
                }
            }
            6 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                    arg5,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 32)));
                };
                let arg3 = arg3 as i32;
                let arg4 = arg4 as i32;
                let arg5 = WlShellSurfaceTransient(arg5);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: i32, arg4: i32, arg5: WlShellSurfaceTransient) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_shell_surface#{}.set_popup(seat: wl_seat#{}, serial: {}, parent: wl_surface#{}, x: {}, y: {}, flags: {:?})\n", client_id, id, arg0, arg1, arg2, arg3, arg4, arg5);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4, arg5);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("seat", o.core().interface, ObjectInterface::WlSeat)));
                };
                let arg2_id = arg2;
                let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg2_id)));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg2_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("parent", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).handle_set_popup(&self, arg0, arg1, arg2, arg3, arg4, arg5);
                } else {
                    DefaultHandler.handle_set_popup(&self, arg0, arg1, arg2, arg3, arg4, arg5);
                }
            }
            7 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_shell_surface#{}.set_maximized(output: wl_output#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlOutput>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("output", o.core().interface, ObjectInterface::WlOutput)));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_set_maximized(&self, arg0);
                } else {
                    DefaultHandler.handle_set_maximized(&self, arg0);
                }
            }
            8 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "title")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_shell_surface#{}.set_title(title: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_title(&self, arg0);
                } else {
                    DefaultHandler.handle_set_title(&self, arg0);
                }
            }
            9 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "class_")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_shell_surface#{}.set_class(class_: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_class(&self, arg0);
                } else {
                    DefaultHandler.handle_set_class(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_shell_surface#{}.ping(serial: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_ping(&self, arg0);
                } else {
                    DefaultHandler.handle_ping(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                let arg0 = WlShellSurfaceResize(arg0);
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: WlShellSurfaceResize, arg1: i32, arg2: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_shell_surface#{}.configure(edges: {:?}, width: {}, height: {})\n", id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_configure(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_configure(&self, arg0, arg1, arg2);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_shell_surface#{}.popup_done()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_popup_done(&self);
                } else {
                    DefaultHandler.handle_popup_done(&self);
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
            0 => "pong",
            1 => "move",
            2 => "resize",
            3 => "set_toplevel",
            4 => "set_transient",
            5 => "set_fullscreen",
            6 => "set_popup",
            7 => "set_maximized",
            8 => "set_title",
            9 => "set_class",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "ping",
            1 => "configure",
            2 => "popup_done",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WlShellSurface {
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

impl WlShellSurface {
    /// Since when the resize.none enum variant is available.
    pub const ENM__RESIZE_NONE__SINCE: u32 = 1;
    /// Since when the resize.top enum variant is available.
    pub const ENM__RESIZE_TOP__SINCE: u32 = 1;
    /// Since when the resize.bottom enum variant is available.
    pub const ENM__RESIZE_BOTTOM__SINCE: u32 = 1;
    /// Since when the resize.left enum variant is available.
    pub const ENM__RESIZE_LEFT__SINCE: u32 = 1;
    /// Since when the resize.top_left enum variant is available.
    pub const ENM__RESIZE_TOP_LEFT__SINCE: u32 = 1;
    /// Since when the resize.bottom_left enum variant is available.
    pub const ENM__RESIZE_BOTTOM_LEFT__SINCE: u32 = 1;
    /// Since when the resize.right enum variant is available.
    pub const ENM__RESIZE_RIGHT__SINCE: u32 = 1;
    /// Since when the resize.top_right enum variant is available.
    pub const ENM__RESIZE_TOP_RIGHT__SINCE: u32 = 1;
    /// Since when the resize.bottom_right enum variant is available.
    pub const ENM__RESIZE_BOTTOM_RIGHT__SINCE: u32 = 1;

    /// Since when the transient.inactive enum variant is available.
    pub const ENM__TRANSIENT_INACTIVE__SINCE: u32 = 1;

    /// Since when the fullscreen_method.default enum variant is available.
    pub const ENM__FULLSCREEN_METHOD_DEFAULT__SINCE: u32 = 1;
    /// Since when the fullscreen_method.scale enum variant is available.
    pub const ENM__FULLSCREEN_METHOD_SCALE__SINCE: u32 = 1;
    /// Since when the fullscreen_method.driver enum variant is available.
    pub const ENM__FULLSCREEN_METHOD_DRIVER__SINCE: u32 = 1;
    /// Since when the fullscreen_method.fill enum variant is available.
    pub const ENM__FULLSCREEN_METHOD_FILL__SINCE: u32 = 1;
}

/// edge values for resizing
///
/// These values are used to indicate which edge of a surface
/// is being dragged in a resize operation. The server may
/// use this information to adapt its behavior, e.g. choose
/// an appropriate cursor image.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct WlShellSurfaceResize(pub u32);

/// An iterator over the set bits in a [`WlShellSurfaceResize`].
///
/// You can construct this with the `IntoIterator` implementation of `WlShellSurfaceResize`.
#[derive(Clone, Debug)]
pub struct WlShellSurfaceResizeIter(pub u32);

impl WlShellSurfaceResize {
    /// no edge
    pub const NONE: Self = Self(0);

    /// top edge
    pub const TOP: Self = Self(1);

    /// bottom edge
    pub const BOTTOM: Self = Self(2);

    /// left edge
    pub const LEFT: Self = Self(4);

    /// top and left edges
    pub const TOP_LEFT: Self = Self(5);

    /// bottom and left edges
    pub const BOTTOM_LEFT: Self = Self(6);

    /// right edge
    pub const RIGHT: Self = Self(8);

    /// top and right edges
    pub const TOP_RIGHT: Self = Self(9);

    /// bottom and right edges
    pub const BOTTOM_RIGHT: Self = Self(10);
}

impl WlShellSurfaceResize {
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
        Self(0 | 0 | 1 | 2 | 4 | 5 | 6 | 8 | 9 | 10)
    }
}

impl Iterator for WlShellSurfaceResizeIter {
    type Item = WlShellSurfaceResize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(WlShellSurfaceResize(bit))
    }
}

impl IntoIterator for WlShellSurfaceResize {
    type Item = WlShellSurfaceResize;
    type IntoIter = WlShellSurfaceResizeIter;

    fn into_iter(self) -> Self::IntoIter {
        WlShellSurfaceResizeIter(self.0)
    }
}

impl BitAnd for WlShellSurfaceResize {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for WlShellSurfaceResize {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for WlShellSurfaceResize {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for WlShellSurfaceResize {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for WlShellSurfaceResize {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for WlShellSurfaceResize {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for WlShellSurfaceResize {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for WlShellSurfaceResize {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for WlShellSurfaceResize {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for WlShellSurfaceResize {
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
            f.write_str("TOP")?;
        }
        if v & 2 == 2 {
            v &= !2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("BOTTOM")?;
        }
        if v & 4 == 4 {
            v &= !4;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("LEFT")?;
        }
        if v & 5 == 5 {
            v &= !5;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("TOP_LEFT")?;
        }
        if v & 6 == 6 {
            v &= !6;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("BOTTOM_LEFT")?;
        }
        if v & 8 == 8 {
            v &= !8;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("RIGHT")?;
        }
        if v & 9 == 9 {
            v &= !9;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("TOP_RIGHT")?;
        }
        if v & 10 == 10 {
            v &= !10;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("BOTTOM_RIGHT")?;
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
            f.write_str("NONE")?;
        }
        Ok(())
    }
}

/// details of transient behaviour
///
/// These flags specify details of the expected behaviour
/// of transient surfaces. Used in the set_transient request.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct WlShellSurfaceTransient(pub u32);

/// An iterator over the set bits in a [`WlShellSurfaceTransient`].
///
/// You can construct this with the `IntoIterator` implementation of `WlShellSurfaceTransient`.
#[derive(Clone, Debug)]
pub struct WlShellSurfaceTransientIter(pub u32);

impl WlShellSurfaceTransient {
    /// do not set keyboard focus
    pub const INACTIVE: Self = Self(0x1);
}

impl WlShellSurfaceTransient {
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
        Self(0 | 0x1)
    }
}

impl Iterator for WlShellSurfaceTransientIter {
    type Item = WlShellSurfaceTransient;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(WlShellSurfaceTransient(bit))
    }
}

impl IntoIterator for WlShellSurfaceTransient {
    type Item = WlShellSurfaceTransient;
    type IntoIter = WlShellSurfaceTransientIter;

    fn into_iter(self) -> Self::IntoIter {
        WlShellSurfaceTransientIter(self.0)
    }
}

impl BitAnd for WlShellSurfaceTransient {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for WlShellSurfaceTransient {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for WlShellSurfaceTransient {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for WlShellSurfaceTransient {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for WlShellSurfaceTransient {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for WlShellSurfaceTransient {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for WlShellSurfaceTransient {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for WlShellSurfaceTransient {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for WlShellSurfaceTransient {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for WlShellSurfaceTransient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = self.0;
        let mut first = true;
        if v & 0x1 == 0x1 {
            v &= !0x1;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("INACTIVE")?;
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

/// different method to set the surface fullscreen
///
/// Hints to indicate to the compositor how to deal with a conflict
/// between the dimensions of the surface and the dimensions of the
/// output. The compositor is free to ignore this parameter.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WlShellSurfaceFullscreenMethod(pub u32);

impl WlShellSurfaceFullscreenMethod {
    /// no preference, apply default policy
    pub const DEFAULT: Self = Self(0);

    /// scale, preserve the surface's aspect ratio and center on output
    pub const SCALE: Self = Self(1);

    /// switch output mode to the smallest mode that can fit the surface, add black borders to compensate size mismatch
    pub const DRIVER: Self = Self(2);

    /// no upscaling, center on output and add black borders to compensate size mismatch
    pub const FILL: Self = Self(3);
}

impl Debug for WlShellSurfaceFullscreenMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DEFAULT => "DEFAULT",
            Self::SCALE => "SCALE",
            Self::DRIVER => "DRIVER",
            Self::FILL => "FILL",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
