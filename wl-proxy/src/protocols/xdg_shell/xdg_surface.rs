//! desktop user interface surface base interface
//!
//! An interface that may be implemented by a wl_surface, for
//! implementations that provide a desktop-style user interface.
//!
//! It provides a base set of functionality required to construct user
//! interface elements requiring management by the compositor, such as
//! toplevel windows, menus, etc. The types of functionality are split into
//! xdg_surface roles.
//!
//! Creating an xdg_surface does not set the role for a wl_surface. In order
//! to map an xdg_surface, the client must create a role-specific object
//! using, e.g., get_toplevel, get_popup. The wl_surface for any given
//! xdg_surface can have at most one role, and may not be assigned any role
//! not based on xdg_surface.
//!
//! A role must be assigned before any other requests are made to the
//! xdg_surface object.
//!
//! The client must call wl_surface.commit on the corresponding wl_surface
//! for the xdg_surface state to take effect.
//!
//! Creating an xdg_surface from a wl_surface which has a buffer attached or
//! committed is a client error, and any attempts by a client to attach or
//! manipulate a buffer prior to the first xdg_surface.configure call must
//! also be treated as errors.
//!
//! After creating a role-specific object and setting it up (e.g. by sending
//! the title, app ID, size constraints, parent, etc), the client must
//! perform an initial commit without any buffer attached. The compositor
//! will reply with initial wl_surface state such as
//! wl_surface.preferred_buffer_scale followed by an xdg_surface.configure
//! event. The client must acknowledge it and is then allowed to attach a
//! buffer to map the surface.
//!
//! Mapping an xdg_surface-based role surface is defined as making it
//! possible for the surface to be shown by the compositor. Note that
//! a mapped surface is not guaranteed to be visible once it is mapped.
//!
//! For an xdg_surface to be mapped by the compositor, the following
//! conditions must be met:
//! (1) the client has assigned an xdg_surface-based role to the surface
//! (2) the client has set and committed the xdg_surface state and the
//!     role-dependent state to the surface
//! (3) the client has committed a buffer to the surface
//!
//! A newly-unmapped surface is considered to have met condition (1) out
//! of the 3 required conditions for mapping a surface if its role surface
//! has not been destroyed, i.e. the client must perform the initial commit
//! again before attaching a buffer.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A xdg_surface object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XdgSurface {
    core: ObjectCore,
    handler: HandlerHolder<dyn XdgSurfaceHandler>,
}

struct DefaultHandler;

impl XdgSurfaceHandler for DefaultHandler { }

impl ConcreteObject for XdgSurface {
    const XML_VERSION: u32 = 7;
    const INTERFACE: ObjectInterface = ObjectInterface::XdgSurface;
    const INTERFACE_NAME: &str = "xdg_surface";
}

impl XdgSurface {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl XdgSurfaceHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn XdgSurfaceHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XdgSurface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XdgSurface")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XdgSurface {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the xdg_surface
    ///
    /// Destroy the xdg_surface object. An xdg_surface must only be destroyed
    /// after its role object has been destroyed, otherwise
    /// a defunct_role_object error is raised.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_surface#{}.destroy()\n", id);
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

    /// destroy the xdg_surface
    ///
    /// Destroy the xdg_surface object. An xdg_surface must only be destroyed
    /// after its role object has been destroyed, otherwise
    /// a defunct_role_object error is raised.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("xdg_surface.destroy", &e);
        }
    }

    /// Since when the get_toplevel message is available.
    pub const MSG__GET_TOPLEVEL__SINCE: u32 = 1;

    /// assign the xdg_toplevel surface role
    ///
    /// This creates an xdg_toplevel object for the given xdg_surface and gives
    /// the associated wl_surface the xdg_toplevel role.
    ///
    /// See the documentation of xdg_toplevel for more details about what an
    /// xdg_toplevel is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_get_toplevel(
        &self,
        id: &Rc<XdgToplevel>,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_surface#{}.get_toplevel(id: xdg_toplevel#{})\n", id, arg0);
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
            1,
            arg0_id,
        ]);
        Ok(())
    }

    /// assign the xdg_toplevel surface role
    ///
    /// This creates an xdg_toplevel object for the given xdg_surface and gives
    /// the associated wl_surface the xdg_toplevel role.
    ///
    /// See the documentation of xdg_toplevel for more details about what an
    /// xdg_toplevel is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_get_toplevel(
        &self,
        id: &Rc<XdgToplevel>,
    ) {
        let res = self.try_send_get_toplevel(
            id,
        );
        if let Err(e) = res {
            log_send("xdg_surface.get_toplevel", &e);
        }
    }

    /// assign the xdg_toplevel surface role
    ///
    /// This creates an xdg_toplevel object for the given xdg_surface and gives
    /// the associated wl_surface the xdg_toplevel role.
    ///
    /// See the documentation of xdg_toplevel for more details about what an
    /// xdg_toplevel is and how it is used.
    #[inline]
    pub fn new_try_send_get_toplevel(
        &self,
    ) -> Result<Rc<XdgToplevel>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_toplevel(
            &id,
        )?;
        Ok(id)
    }

    /// assign the xdg_toplevel surface role
    ///
    /// This creates an xdg_toplevel object for the given xdg_surface and gives
    /// the associated wl_surface the xdg_toplevel role.
    ///
    /// See the documentation of xdg_toplevel for more details about what an
    /// xdg_toplevel is and how it is used.
    #[inline]
    pub fn new_send_get_toplevel(
        &self,
    ) -> Rc<XdgToplevel> {
        let id = self.core.create_child();
        self.send_get_toplevel(
            &id,
        );
        id
    }

    /// Since when the get_popup message is available.
    pub const MSG__GET_POPUP__SINCE: u32 = 1;

    /// assign the xdg_popup surface role
    ///
    /// This creates an xdg_popup object for the given xdg_surface and gives
    /// the associated wl_surface the xdg_popup role.
    ///
    /// If null is passed as a parent, a parent surface must be specified using
    /// some other protocol, before committing the initial state.
    ///
    /// See the documentation of xdg_popup for more details about what an
    /// xdg_popup is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `parent`:
    /// - `positioner`:
    #[inline]
    pub fn try_send_get_popup(
        &self,
        id: &Rc<XdgPopup>,
        parent: Option<&Rc<XdgSurface>>,
        positioner: &Rc<XdgPositioner>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            id,
            parent,
            positioner,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.map(|a| a.core());
        let arg2 = arg2.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1 {
            None => 0,
            Some(arg1) => match arg1.server_obj_id.get() {
                None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("parent"))),
                Some(id) => id,
            },
        };
        let arg2_id = match arg2.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("positioner"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_surface#{}.get_popup(id: xdg_popup#{}, parent: xdg_surface#{}, positioner: xdg_positioner#{})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id, arg2_id);
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
            arg1_id,
            arg2_id,
        ]);
        Ok(())
    }

    /// assign the xdg_popup surface role
    ///
    /// This creates an xdg_popup object for the given xdg_surface and gives
    /// the associated wl_surface the xdg_popup role.
    ///
    /// If null is passed as a parent, a parent surface must be specified using
    /// some other protocol, before committing the initial state.
    ///
    /// See the documentation of xdg_popup for more details about what an
    /// xdg_popup is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `parent`:
    /// - `positioner`:
    #[inline]
    pub fn send_get_popup(
        &self,
        id: &Rc<XdgPopup>,
        parent: Option<&Rc<XdgSurface>>,
        positioner: &Rc<XdgPositioner>,
    ) {
        let res = self.try_send_get_popup(
            id,
            parent,
            positioner,
        );
        if let Err(e) = res {
            log_send("xdg_surface.get_popup", &e);
        }
    }

    /// assign the xdg_popup surface role
    ///
    /// This creates an xdg_popup object for the given xdg_surface and gives
    /// the associated wl_surface the xdg_popup role.
    ///
    /// If null is passed as a parent, a parent surface must be specified using
    /// some other protocol, before committing the initial state.
    ///
    /// See the documentation of xdg_popup for more details about what an
    /// xdg_popup is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `parent`:
    /// - `positioner`:
    #[inline]
    pub fn new_try_send_get_popup(
        &self,
        parent: Option<&Rc<XdgSurface>>,
        positioner: &Rc<XdgPositioner>,
    ) -> Result<Rc<XdgPopup>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_popup(
            &id,
            parent,
            positioner,
        )?;
        Ok(id)
    }

    /// assign the xdg_popup surface role
    ///
    /// This creates an xdg_popup object for the given xdg_surface and gives
    /// the associated wl_surface the xdg_popup role.
    ///
    /// If null is passed as a parent, a parent surface must be specified using
    /// some other protocol, before committing the initial state.
    ///
    /// See the documentation of xdg_popup for more details about what an
    /// xdg_popup is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `parent`:
    /// - `positioner`:
    #[inline]
    pub fn new_send_get_popup(
        &self,
        parent: Option<&Rc<XdgSurface>>,
        positioner: &Rc<XdgPositioner>,
    ) -> Rc<XdgPopup> {
        let id = self.core.create_child();
        self.send_get_popup(
            &id,
            parent,
            positioner,
        );
        id
    }

    /// Since when the set_window_geometry message is available.
    pub const MSG__SET_WINDOW_GEOMETRY__SINCE: u32 = 1;

    /// set the new window geometry
    ///
    /// The window geometry of a surface is its "visible bounds" from the
    /// user's perspective. Client-side decorations often have invisible
    /// portions like drop-shadows which should be ignored for the
    /// purposes of aligning, placing and constraining windows. Note that
    /// in some situations, compositors may clip rendering to the window
    /// geometry, so the client should avoid putting functional elements
    /// outside of it.
    ///
    /// The window geometry is double-buffered state, see wl_surface.commit.
    ///
    /// When maintaining a position, the compositor should treat the (x, y)
    /// coordinate of the window geometry as the top left corner of the window.
    /// A client changing the (x, y) window geometry coordinate should in
    /// general not alter the position of the window.
    ///
    /// Once the window geometry of the surface is set, it is not possible to
    /// unset it, and it will remain the same until set_window_geometry is
    /// called again, even if a new subsurface or buffer is attached.
    ///
    /// If never set, the value is the full bounds of the surface,
    /// including any subsurfaces. This updates dynamically on every
    /// commit. This unset is meant for extremely simple clients.
    ///
    /// The arguments are given in the surface-local coordinate space of
    /// the wl_surface associated with this xdg_surface, and may extend outside
    /// of the wl_surface itself to mark parts of the subsurface tree as part of
    /// the window geometry.
    ///
    /// When applied, the effective window geometry will be the set window
    /// geometry clamped to the bounding rectangle of the combined
    /// geometry of the surface of the xdg_surface and the associated
    /// subsurfaces.
    ///
    /// The effective geometry will not be recalculated unless a new call to
    /// set_window_geometry is done and the new pending surface state is
    /// subsequently applied.
    ///
    /// The width and height of the effective window geometry must be
    /// greater than zero. Setting an invalid size will raise an
    /// invalid_size error.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn try_send_set_window_geometry(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            x,
            y,
            width,
            height,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_surface#{}.set_window_geometry(x: {}, y: {}, width: {}, height: {})\n", id, arg0, arg1, arg2, arg3);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2, arg3);
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
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// set the new window geometry
    ///
    /// The window geometry of a surface is its "visible bounds" from the
    /// user's perspective. Client-side decorations often have invisible
    /// portions like drop-shadows which should be ignored for the
    /// purposes of aligning, placing and constraining windows. Note that
    /// in some situations, compositors may clip rendering to the window
    /// geometry, so the client should avoid putting functional elements
    /// outside of it.
    ///
    /// The window geometry is double-buffered state, see wl_surface.commit.
    ///
    /// When maintaining a position, the compositor should treat the (x, y)
    /// coordinate of the window geometry as the top left corner of the window.
    /// A client changing the (x, y) window geometry coordinate should in
    /// general not alter the position of the window.
    ///
    /// Once the window geometry of the surface is set, it is not possible to
    /// unset it, and it will remain the same until set_window_geometry is
    /// called again, even if a new subsurface or buffer is attached.
    ///
    /// If never set, the value is the full bounds of the surface,
    /// including any subsurfaces. This updates dynamically on every
    /// commit. This unset is meant for extremely simple clients.
    ///
    /// The arguments are given in the surface-local coordinate space of
    /// the wl_surface associated with this xdg_surface, and may extend outside
    /// of the wl_surface itself to mark parts of the subsurface tree as part of
    /// the window geometry.
    ///
    /// When applied, the effective window geometry will be the set window
    /// geometry clamped to the bounding rectangle of the combined
    /// geometry of the surface of the xdg_surface and the associated
    /// subsurfaces.
    ///
    /// The effective geometry will not be recalculated unless a new call to
    /// set_window_geometry is done and the new pending surface state is
    /// subsequently applied.
    ///
    /// The width and height of the effective window geometry must be
    /// greater than zero. Setting an invalid size will raise an
    /// invalid_size error.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn send_set_window_geometry(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_set_window_geometry(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_send("xdg_surface.set_window_geometry", &e);
        }
    }

    /// Since when the ack_configure message is available.
    pub const MSG__ACK_CONFIGURE__SINCE: u32 = 1;

    /// ack a configure event
    ///
    /// When a configure event is received, if a client commits the
    /// surface in response to the configure event, then the client
    /// must make an ack_configure request sometime before the commit
    /// request, passing along the serial of the configure event.
    ///
    /// For instance, for toplevel surfaces the compositor might use this
    /// information to move a surface to the top left only when the client has
    /// drawn itself for the maximized or fullscreen state.
    ///
    /// If the client receives multiple configure events before it
    /// can respond to one, it only has to ack the last configure event.
    /// Acking a configure event that was never sent raises an invalid_serial
    /// error.
    ///
    /// A client is not required to commit immediately after sending
    /// an ack_configure request - it may even ack_configure several times
    /// before its next surface commit.
    ///
    /// A client may send multiple ack_configure requests before committing, but
    /// only the last request sent before a commit indicates which configure
    /// event the client really is responding to.
    ///
    /// Sending an ack_configure request consumes the serial number sent with
    /// the request, as well as serial numbers sent by all configure events
    /// sent on this xdg_surface prior to the configure event referenced by
    /// the committed serial.
    ///
    /// It is an error to issue multiple ack_configure requests referencing a
    /// serial from the same configure event, or to issue an ack_configure
    /// request referencing a serial from a configure event issued before the
    /// event identified by the last ack_configure request for the same
    /// xdg_surface. Doing so will raise an invalid_serial error.
    ///
    /// # Arguments
    ///
    /// - `serial`: the serial from the configure event
    #[inline]
    pub fn try_send_ack_configure(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_surface#{}.ack_configure(serial: {})\n", id, arg0);
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
            4,
            arg0,
        ]);
        Ok(())
    }

    /// ack a configure event
    ///
    /// When a configure event is received, if a client commits the
    /// surface in response to the configure event, then the client
    /// must make an ack_configure request sometime before the commit
    /// request, passing along the serial of the configure event.
    ///
    /// For instance, for toplevel surfaces the compositor might use this
    /// information to move a surface to the top left only when the client has
    /// drawn itself for the maximized or fullscreen state.
    ///
    /// If the client receives multiple configure events before it
    /// can respond to one, it only has to ack the last configure event.
    /// Acking a configure event that was never sent raises an invalid_serial
    /// error.
    ///
    /// A client is not required to commit immediately after sending
    /// an ack_configure request - it may even ack_configure several times
    /// before its next surface commit.
    ///
    /// A client may send multiple ack_configure requests before committing, but
    /// only the last request sent before a commit indicates which configure
    /// event the client really is responding to.
    ///
    /// Sending an ack_configure request consumes the serial number sent with
    /// the request, as well as serial numbers sent by all configure events
    /// sent on this xdg_surface prior to the configure event referenced by
    /// the committed serial.
    ///
    /// It is an error to issue multiple ack_configure requests referencing a
    /// serial from the same configure event, or to issue an ack_configure
    /// request referencing a serial from a configure event issued before the
    /// event identified by the last ack_configure request for the same
    /// xdg_surface. Doing so will raise an invalid_serial error.
    ///
    /// # Arguments
    ///
    /// - `serial`: the serial from the configure event
    #[inline]
    pub fn send_ack_configure(
        &self,
        serial: u32,
    ) {
        let res = self.try_send_ack_configure(
            serial,
        );
        if let Err(e) = res {
            log_send("xdg_surface.ack_configure", &e);
        }
    }

    /// Since when the configure message is available.
    pub const MSG__CONFIGURE__SINCE: u32 = 1;

    /// suggest a surface change
    ///
    /// The configure event marks the end of a configure sequence. A configure
    /// sequence is a set of one or more events configuring the state of the
    /// xdg_surface, including the final xdg_surface.configure event.
    ///
    /// Where applicable, xdg_surface surface roles will during a configure
    /// sequence extend this event as a latched state sent as events before the
    /// xdg_surface.configure event. Such events should be considered to make up
    /// a set of atomically applied configuration states, where the
    /// xdg_surface.configure commits the accumulated state.
    ///
    /// Clients should arrange their surface for the new states, and then send
    /// an ack_configure request with the serial sent in this configure event at
    /// some point before committing the new surface.
    ///
    /// If the client receives multiple configure events before it can respond
    /// to one, it is free to discard all but the last event it received.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the configure event
    #[inline]
    pub fn try_send_configure(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xdg_surface#{}.configure(serial: {})\n", client_id, id, arg0);
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

    /// suggest a surface change
    ///
    /// The configure event marks the end of a configure sequence. A configure
    /// sequence is a set of one or more events configuring the state of the
    /// xdg_surface, including the final xdg_surface.configure event.
    ///
    /// Where applicable, xdg_surface surface roles will during a configure
    /// sequence extend this event as a latched state sent as events before the
    /// xdg_surface.configure event. Such events should be considered to make up
    /// a set of atomically applied configuration states, where the
    /// xdg_surface.configure commits the accumulated state.
    ///
    /// Clients should arrange their surface for the new states, and then send
    /// an ack_configure request with the serial sent in this configure event at
    /// some point before committing the new surface.
    ///
    /// If the client receives multiple configure events before it can respond
    /// to one, it is free to discard all but the last event it received.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the configure event
    #[inline]
    pub fn send_configure(
        &self,
        serial: u32,
    ) {
        let res = self.try_send_configure(
            serial,
        );
        if let Err(e) = res {
            log_send("xdg_surface.configure", &e);
        }
    }
}

/// A message handler for [`XdgSurface`] proxies.
pub trait XdgSurfaceHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<XdgSurface>) {
        slf.core.delete_id();
    }

    /// destroy the xdg_surface
    ///
    /// Destroy the xdg_surface object. An xdg_surface must only be destroyed
    /// after its role object has been destroyed, otherwise
    /// a defunct_role_object error is raised.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<XdgSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("xdg_surface.destroy", &e);
        }
    }

    /// assign the xdg_toplevel surface role
    ///
    /// This creates an xdg_toplevel object for the given xdg_surface and gives
    /// the associated wl_surface the xdg_toplevel role.
    ///
    /// See the documentation of xdg_toplevel for more details about what an
    /// xdg_toplevel is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn handle_get_toplevel(
        &mut self,
        slf: &Rc<XdgSurface>,
        id: &Rc<XdgToplevel>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_toplevel(
            id,
        );
        if let Err(e) = res {
            log_forward("xdg_surface.get_toplevel", &e);
        }
    }

    /// assign the xdg_popup surface role
    ///
    /// This creates an xdg_popup object for the given xdg_surface and gives
    /// the associated wl_surface the xdg_popup role.
    ///
    /// If null is passed as a parent, a parent surface must be specified using
    /// some other protocol, before committing the initial state.
    ///
    /// See the documentation of xdg_popup for more details about what an
    /// xdg_popup is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `parent`:
    /// - `positioner`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_popup(
        &mut self,
        slf: &Rc<XdgSurface>,
        id: &Rc<XdgPopup>,
        parent: Option<&Rc<XdgSurface>>,
        positioner: &Rc<XdgPositioner>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_popup(
            id,
            parent,
            positioner,
        );
        if let Err(e) = res {
            log_forward("xdg_surface.get_popup", &e);
        }
    }

    /// set the new window geometry
    ///
    /// The window geometry of a surface is its "visible bounds" from the
    /// user's perspective. Client-side decorations often have invisible
    /// portions like drop-shadows which should be ignored for the
    /// purposes of aligning, placing and constraining windows. Note that
    /// in some situations, compositors may clip rendering to the window
    /// geometry, so the client should avoid putting functional elements
    /// outside of it.
    ///
    /// The window geometry is double-buffered state, see wl_surface.commit.
    ///
    /// When maintaining a position, the compositor should treat the (x, y)
    /// coordinate of the window geometry as the top left corner of the window.
    /// A client changing the (x, y) window geometry coordinate should in
    /// general not alter the position of the window.
    ///
    /// Once the window geometry of the surface is set, it is not possible to
    /// unset it, and it will remain the same until set_window_geometry is
    /// called again, even if a new subsurface or buffer is attached.
    ///
    /// If never set, the value is the full bounds of the surface,
    /// including any subsurfaces. This updates dynamically on every
    /// commit. This unset is meant for extremely simple clients.
    ///
    /// The arguments are given in the surface-local coordinate space of
    /// the wl_surface associated with this xdg_surface, and may extend outside
    /// of the wl_surface itself to mark parts of the subsurface tree as part of
    /// the window geometry.
    ///
    /// When applied, the effective window geometry will be the set window
    /// geometry clamped to the bounding rectangle of the combined
    /// geometry of the surface of the xdg_surface and the associated
    /// subsurfaces.
    ///
    /// The effective geometry will not be recalculated unless a new call to
    /// set_window_geometry is done and the new pending surface state is
    /// subsequently applied.
    ///
    /// The width and height of the effective window geometry must be
    /// greater than zero. Setting an invalid size will raise an
    /// invalid_size error.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    fn handle_set_window_geometry(
        &mut self,
        slf: &Rc<XdgSurface>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_window_geometry(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("xdg_surface.set_window_geometry", &e);
        }
    }

    /// ack a configure event
    ///
    /// When a configure event is received, if a client commits the
    /// surface in response to the configure event, then the client
    /// must make an ack_configure request sometime before the commit
    /// request, passing along the serial of the configure event.
    ///
    /// For instance, for toplevel surfaces the compositor might use this
    /// information to move a surface to the top left only when the client has
    /// drawn itself for the maximized or fullscreen state.
    ///
    /// If the client receives multiple configure events before it
    /// can respond to one, it only has to ack the last configure event.
    /// Acking a configure event that was never sent raises an invalid_serial
    /// error.
    ///
    /// A client is not required to commit immediately after sending
    /// an ack_configure request - it may even ack_configure several times
    /// before its next surface commit.
    ///
    /// A client may send multiple ack_configure requests before committing, but
    /// only the last request sent before a commit indicates which configure
    /// event the client really is responding to.
    ///
    /// Sending an ack_configure request consumes the serial number sent with
    /// the request, as well as serial numbers sent by all configure events
    /// sent on this xdg_surface prior to the configure event referenced by
    /// the committed serial.
    ///
    /// It is an error to issue multiple ack_configure requests referencing a
    /// serial from the same configure event, or to issue an ack_configure
    /// request referencing a serial from a configure event issued before the
    /// event identified by the last ack_configure request for the same
    /// xdg_surface. Doing so will raise an invalid_serial error.
    ///
    /// # Arguments
    ///
    /// - `serial`: the serial from the configure event
    #[inline]
    fn handle_ack_configure(
        &mut self,
        slf: &Rc<XdgSurface>,
        serial: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_ack_configure(
            serial,
        );
        if let Err(e) = res {
            log_forward("xdg_surface.ack_configure", &e);
        }
    }

    /// suggest a surface change
    ///
    /// The configure event marks the end of a configure sequence. A configure
    /// sequence is a set of one or more events configuring the state of the
    /// xdg_surface, including the final xdg_surface.configure event.
    ///
    /// Where applicable, xdg_surface surface roles will during a configure
    /// sequence extend this event as a latched state sent as events before the
    /// xdg_surface.configure event. Such events should be considered to make up
    /// a set of atomically applied configuration states, where the
    /// xdg_surface.configure commits the accumulated state.
    ///
    /// Clients should arrange their surface for the new states, and then send
    /// an ack_configure request with the serial sent in this configure event at
    /// some point before committing the new surface.
    ///
    /// If the client receives multiple configure events before it can respond
    /// to one, it is free to discard all but the last event it received.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the configure event
    #[inline]
    fn handle_configure(
        &mut self,
        slf: &Rc<XdgSurface>,
        serial: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_configure(
            serial,
        );
        if let Err(e) = res {
            log_forward("xdg_surface.configure", &e);
        }
    }
}

impl ObjectPrivate for XdgSurface {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::XdgSurface, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_surface#{}.destroy()\n", client_id, id);
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_surface#{}.get_toplevel(id: xdg_toplevel#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = XdgToplevel::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_get_toplevel(&self, arg0);
                } else {
                    DefaultHandler.handle_get_toplevel(&self, arg0);
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
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_surface#{}.get_popup(id: xdg_popup#{}, parent: xdg_surface#{}, positioner: xdg_positioner#{})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let arg0 = XdgPopup::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1 = if arg1 == 0 {
                    None
                } else {
                    let arg1_id = arg1;
                    let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                    };
                    let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<XdgSurface>() else {
                        let o = client.endpoint.lookup(arg1_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("parent", o.core().interface, ObjectInterface::XdgSurface)));
                    };
                    Some(arg1)
                };
                let arg2_id = arg2;
                let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg2_id)));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<XdgPositioner>() else {
                    let o = client.endpoint.lookup(arg2_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("positioner", o.core().interface, ObjectInterface::XdgPositioner)));
                };
                let arg0 = &arg0;
                let arg1 = arg1.as_ref();
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).handle_get_popup(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_get_popup(&self, arg0, arg1, arg2);
                }
            }
            3 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 24)));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_surface#{}.set_window_geometry(x: {}, y: {}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_window_geometry(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_set_window_geometry(&self, arg0, arg1, arg2, arg3);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_surface#{}.ack_configure(serial: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_ack_configure(&self, arg0);
                } else {
                    DefaultHandler.handle_ack_configure(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xdg_surface#{}.configure(serial: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_configure(&self, arg0);
                } else {
                    DefaultHandler.handle_configure(&self, arg0);
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
            1 => "get_toplevel",
            2 => "get_popup",
            3 => "set_window_geometry",
            4 => "ack_configure",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "configure",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for XdgSurface {
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

impl XdgSurface {
    /// Since when the error.not_constructed enum variant is available.
    pub const ENM__ERROR_NOT_CONSTRUCTED__SINCE: u32 = 1;
    /// Since when the error.already_constructed enum variant is available.
    pub const ENM__ERROR_ALREADY_CONSTRUCTED__SINCE: u32 = 1;
    /// Since when the error.unconfigured_buffer enum variant is available.
    pub const ENM__ERROR_UNCONFIGURED_BUFFER__SINCE: u32 = 1;
    /// Since when the error.invalid_serial enum variant is available.
    pub const ENM__ERROR_INVALID_SERIAL__SINCE: u32 = 1;
    /// Since when the error.invalid_size enum variant is available.
    pub const ENM__ERROR_INVALID_SIZE__SINCE: u32 = 1;
    /// Since when the error.defunct_role_object enum variant is available.
    pub const ENM__ERROR_DEFUNCT_ROLE_OBJECT__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct XdgSurfaceError(pub u32);

impl XdgSurfaceError {
    /// Surface was not fully constructed
    pub const NOT_CONSTRUCTED: Self = Self(1);

    /// Surface was already constructed
    pub const ALREADY_CONSTRUCTED: Self = Self(2);

    /// Attaching a buffer to an unconfigured surface
    pub const UNCONFIGURED_BUFFER: Self = Self(3);

    /// Invalid serial number when acking a configure event
    pub const INVALID_SERIAL: Self = Self(4);

    /// Width or height was zero or negative
    pub const INVALID_SIZE: Self = Self(5);

    /// Surface was destroyed before its role object
    pub const DEFUNCT_ROLE_OBJECT: Self = Self(6);
}

impl Debug for XdgSurfaceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NOT_CONSTRUCTED => "NOT_CONSTRUCTED",
            Self::ALREADY_CONSTRUCTED => "ALREADY_CONSTRUCTED",
            Self::UNCONFIGURED_BUFFER => "UNCONFIGURED_BUFFER",
            Self::INVALID_SERIAL => "INVALID_SERIAL",
            Self::INVALID_SIZE => "INVALID_SIZE",
            Self::DEFUNCT_ROLE_OBJECT => "DEFUNCT_ROLE_OBJECT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
