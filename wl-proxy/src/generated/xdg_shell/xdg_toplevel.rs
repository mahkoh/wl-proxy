//! toplevel surface
//!
//! This interface defines an xdg_surface role which allows a surface to,
//! among other things, set window-like properties such as maximize,
//! fullscreen, and minimize, set application-specific metadata like title and
//! id, and well as trigger user interactive operations such as interactive
//! resize and move.
//!
//! A xdg_toplevel by default is responsible for providing the full intended
//! visual representation of the toplevel, which depending on the window
//! state, may mean things like a title bar, window controls and drop shadow.
//!
//! Unmapping an xdg_toplevel means that the surface cannot be shown
//! by the compositor until it is explicitly mapped again.
//! All active operations (e.g., move, resize) are canceled and all
//! attributes (e.g. title, state, stacking, ...) are discarded for
//! an xdg_toplevel surface when it is unmapped. The xdg_toplevel returns to
//! the state it had right after xdg_surface.get_toplevel. The client
//! can re-map the toplevel by performing a commit without any buffer
//! attached, waiting for a configure event and handling it as usual (see
//! xdg_surface description).
//!
//! Attaching a null buffer to a toplevel unmaps the surface.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A xdg_toplevel proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaXdgToplevel {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaXdgToplevelMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaXdgToplevelMessageHandler for DefaultMessageHandler { }

impl MetaXdgToplevel {
    pub const XML_VERSION: u32 = 7;
}

impl MetaXdgToplevel {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::XdgToplevel, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaXdgToplevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaXdgToplevel")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaXdgToplevel {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the xdg_toplevel
    ///
    /// This request destroys the role surface and unmaps the surface;
    /// see "Unmapping" behavior in interface section for details.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= xdg_toplevel#{}.destroy()", id);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the set_parent message is available.
    #[allow(dead_code)]
    pub const MSG__SET_PARENT__SINCE: u32 = 1;

    /// set the parent of this surface
    ///
    /// Set the "parent" of this surface. This surface should be stacked
    /// above the parent surface and all other ancestor surfaces.
    ///
    /// Parent surfaces should be set on dialogs, toolboxes, or other
    /// "auxiliary" surfaces, so that the parent is raised when the dialog
    /// is raised.
    ///
    /// Setting a null parent for a child surface unsets its parent. Setting
    /// a null parent for a surface which currently has no parent is a no-op.
    ///
    /// Only mapped surfaces can have child surfaces. Setting a parent which
    /// is not mapped is equivalent to setting a null parent. If a surface
    /// becomes unmapped, its children's parent is set to the parent of
    /// the now-unmapped surface. If the now-unmapped surface has no parent,
    /// its children's parent is unset. If the now-unmapped surface becomes
    /// mapped again, its parent-child relationship is not restored.
    ///
    /// The parent toplevel must not be one of the child toplevel's
    /// descendants, and the parent must be different from the child toplevel,
    /// otherwise the invalid_parent protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `parent`:
    #[inline]
    pub fn send_set_parent(
        &self,
        parent: Option<&Rc<MetaXdgToplevel>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            parent,
        );
        let arg0 = arg0.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError::ArgNoServerId("parent")),
                Some(id) => id,
            },
        };
        eprintln!("server      <= xdg_toplevel#{}.set_parent(parent: xdg_toplevel#{})", id, arg0_id);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the set_title message is available.
    #[allow(dead_code)]
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
    /// - `title`:
    #[inline]
    pub fn send_set_title(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= xdg_toplevel#{}.set_title(title: {:?})", id, arg0);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the set_app_id message is available.
    #[allow(dead_code)]
    pub const MSG__SET_APP_ID__SINCE: u32 = 1;

    /// set application ID
    ///
    /// Set an application identifier for the surface.
    ///
    /// The app ID identifies the general class of applications to which
    /// the surface belongs. The compositor can use this to group multiple
    /// surfaces together, or to determine how to launch a new application.
    ///
    /// For D-Bus activatable applications, the app ID is used as the D-Bus
    /// service name.
    ///
    /// The compositor shell will try to group application surfaces together
    /// by their app ID. As a best practice, it is suggested to select app
    /// ID's that match the basename of the application's .desktop file.
    /// For example, "org.freedesktop.FooViewer" where the .desktop file is
    /// "org.freedesktop.FooViewer.desktop".
    ///
    /// Like other properties, a set_app_id request can be sent after the
    /// xdg_toplevel has been mapped to update the property.
    ///
    /// See the desktop-entry specification [0] for more details on
    /// application identifiers and how they relate to well-known D-Bus
    /// names and .desktop files.
    ///
    /// [0] https://standards.freedesktop.org/desktop-entry-spec/
    ///
    /// # Arguments
    ///
    /// - `app_id`:
    #[inline]
    pub fn send_set_app_id(
        &self,
        app_id: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            app_id,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= xdg_toplevel#{}.set_app_id(app_id: {:?})", id, arg0);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            3,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the show_window_menu message is available.
    #[allow(dead_code)]
    pub const MSG__SHOW_WINDOW_MENU__SINCE: u32 = 1;

    /// show the window menu
    ///
    /// Clients implementing client-side decorations might want to show
    /// a context menu when right-clicking on the decorations, giving the
    /// user a menu that they can use to maximize or minimize the window.
    ///
    /// This request asks the compositor to pop up such a window menu at
    /// the given position, relative to the local surface coordinates of
    /// the parent surface. There are no guarantees as to what menu items
    /// the window menu contains, or even if a window menu will be drawn
    /// at all.
    ///
    /// This request must be used in response to some sort of user action
    /// like a button press, key press, or touch down event.
    ///
    /// # Arguments
    ///
    /// - `seat`: the wl_seat of the user event
    /// - `serial`: the serial of the user event
    /// - `x`: the x position to pop up the window menu at
    /// - `y`: the y position to pop up the window menu at
    #[inline]
    pub fn send_show_window_menu(
        &self,
        seat: &Rc<MetaWlSeat>,
        serial: u32,
        x: i32,
        y: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            seat,
            serial,
            x,
            y,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("seat")),
            Some(id) => id,
        };
        eprintln!("server      <= xdg_toplevel#{}.show_window_menu(seat: wl_seat#{}, serial: {}, x: {}, y: {})", id, arg0_id, arg1, arg2, arg3);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            4,
            arg0_id,
            arg1,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// Since when the move message is available.
    #[allow(dead_code)]
    pub const MSG__MOVE__SINCE: u32 = 1;

    /// start an interactive move
    ///
    /// Start an interactive, user-driven move of the surface.
    ///
    /// This request must be used in response to some sort of user action
    /// like a button press, key press, or touch down event. The passed
    /// serial is used to determine the type of interactive move (touch,
    /// pointer, etc).
    ///
    /// The server may ignore move requests depending on the state of
    /// the surface (e.g. fullscreen or maximized), or if the passed serial
    /// is no longer valid.
    ///
    /// If triggered, the surface will lose the focus of the device
    /// (wl_pointer, wl_touch, etc) used for the move. It is up to the
    /// compositor to visually indicate that the move is taking place, such as
    /// updating a pointer cursor, during the move. There is no guarantee
    /// that the device focus will return when the move is completed.
    ///
    /// # Arguments
    ///
    /// - `seat`: the wl_seat of the user event
    /// - `serial`: the serial of the user event
    #[inline]
    pub fn send_move(
        &self,
        seat: &Rc<MetaWlSeat>,
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("seat")),
            Some(id) => id,
        };
        eprintln!("server      <= xdg_toplevel#{}.move(seat: wl_seat#{}, serial: {})", id, arg0_id, arg1);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            5,
            arg0_id,
            arg1,
        ]);
        Ok(())
    }

    /// Since when the resize message is available.
    #[allow(dead_code)]
    pub const MSG__RESIZE__SINCE: u32 = 1;

    /// start an interactive resize
    ///
    /// Start a user-driven, interactive resize of the surface.
    ///
    /// This request must be used in response to some sort of user action
    /// like a button press, key press, or touch down event. The passed
    /// serial is used to determine the type of interactive resize (touch,
    /// pointer, etc).
    ///
    /// The server may ignore resize requests depending on the state of
    /// the surface (e.g. fullscreen or maximized).
    ///
    /// If triggered, the client will receive configure events with the
    /// "resize" state enum value and the expected sizes. See the "resize"
    /// enum value for more details about what is required. The client
    /// must also acknowledge configure events using "ack_configure". After
    /// the resize is completed, the client will receive another "configure"
    /// event without the resize state.
    ///
    /// If triggered, the surface also will lose the focus of the device
    /// (wl_pointer, wl_touch, etc) used for the resize. It is up to the
    /// compositor to visually indicate that the resize is taking place,
    /// such as updating a pointer cursor, during the resize. There is no
    /// guarantee that the device focus will return when the resize is
    /// completed.
    ///
    /// The edges parameter specifies how the surface should be resized, and
    /// is one of the values of the resize_edge enum. Values not matching
    /// a variant of the enum will cause the invalid_resize_edge protocol error.
    /// The compositor may use this information to update the surface position
    /// for example when dragging the top left corner. The compositor may also
    /// use this information to adapt its behavior, e.g. choose an appropriate
    /// cursor image.
    ///
    /// # Arguments
    ///
    /// - `seat`: the wl_seat of the user event
    /// - `serial`: the serial of the user event
    /// - `edges`: which edge or corner is being dragged
    #[inline]
    pub fn send_resize(
        &self,
        seat: &Rc<MetaWlSeat>,
        serial: u32,
        edges: MetaXdgToplevelResizeEdge,
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("seat")),
            Some(id) => id,
        };
        eprintln!("server      <= xdg_toplevel#{}.resize(seat: wl_seat#{}, serial: {}, edges: {:?})", id, arg0_id, arg1, arg2);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            6,
            arg0_id,
            arg1,
            arg2.0,
        ]);
        Ok(())
    }

    /// Since when the set_max_size message is available.
    #[allow(dead_code)]
    pub const MSG__SET_MAX_SIZE__SINCE: u32 = 1;

    /// set the maximum size
    ///
    /// Set a maximum size for the window.
    ///
    /// The client can specify a maximum size so that the compositor does
    /// not try to configure the window beyond this size.
    ///
    /// The width and height arguments are in window geometry coordinates.
    /// See xdg_surface.set_window_geometry.
    ///
    /// Values set in this way are double-buffered, see wl_surface.commit.
    ///
    /// The compositor can use this information to allow or disallow
    /// different states like maximize or fullscreen and draw accurate
    /// animations.
    ///
    /// Similarly, a tiling window manager may use this information to
    /// place and resize client windows in a more effective way.
    ///
    /// The client should not rely on the compositor to obey the maximum
    /// size. The compositor may decide to ignore the values set by the
    /// client and request a larger size.
    ///
    /// If never set, or a value of zero in the request, means that the
    /// client has no expected maximum size in the given dimension.
    /// As a result, a client wishing to reset the maximum size
    /// to an unspecified state can use zero for width and height in the
    /// request.
    ///
    /// Requesting a maximum size to be smaller than the minimum size of
    /// a surface is illegal and will result in an invalid_size error.
    ///
    /// The width and height must be greater than or equal to zero. Using
    /// strictly negative values for width or height will result in a
    /// invalid_size error.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn send_set_max_size(
        &self,
        width: i32,
        height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            width,
            height,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= xdg_toplevel#{}.set_max_size(width: {}, height: {})", id, arg0, arg1);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            7,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// Since when the set_min_size message is available.
    #[allow(dead_code)]
    pub const MSG__SET_MIN_SIZE__SINCE: u32 = 1;

    /// set the minimum size
    ///
    /// Set a minimum size for the window.
    ///
    /// The client can specify a minimum size so that the compositor does
    /// not try to configure the window below this size.
    ///
    /// The width and height arguments are in window geometry coordinates.
    /// See xdg_surface.set_window_geometry.
    ///
    /// Values set in this way are double-buffered, see wl_surface.commit.
    ///
    /// The compositor can use this information to allow or disallow
    /// different states like maximize or fullscreen and draw accurate
    /// animations.
    ///
    /// Similarly, a tiling window manager may use this information to
    /// place and resize client windows in a more effective way.
    ///
    /// The client should not rely on the compositor to obey the minimum
    /// size. The compositor may decide to ignore the values set by the
    /// client and request a smaller size.
    ///
    /// If never set, or a value of zero in the request, means that the
    /// client has no expected minimum size in the given dimension.
    /// As a result, a client wishing to reset the minimum size
    /// to an unspecified state can use zero for width and height in the
    /// request.
    ///
    /// Requesting a minimum size to be larger than the maximum size of
    /// a surface is illegal and will result in an invalid_size error.
    ///
    /// The width and height must be greater than or equal to zero. Using
    /// strictly negative values for width and height will result in a
    /// invalid_size error.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn send_set_min_size(
        &self,
        width: i32,
        height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            width,
            height,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= xdg_toplevel#{}.set_min_size(width: {}, height: {})", id, arg0, arg1);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            8,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// Since when the set_maximized message is available.
    #[allow(dead_code)]
    pub const MSG__SET_MAXIMIZED__SINCE: u32 = 1;

    /// maximize the window
    ///
    /// Maximize the surface.
    ///
    /// After requesting that the surface should be maximized, the compositor
    /// will respond by emitting a configure event. Whether this configure
    /// actually sets the window maximized is subject to compositor policies.
    /// The client must then update its content, drawing in the configured
    /// state. The client must also acknowledge the configure when committing
    /// the new content (see ack_configure).
    ///
    /// It is up to the compositor to decide how and where to maximize the
    /// surface, for example which output and what region of the screen should
    /// be used.
    ///
    /// If the surface was already maximized, the compositor will still emit
    /// a configure event with the "maximized" state.
    ///
    /// If the surface is in a fullscreen state, this request has no direct
    /// effect. It may alter the state the surface is returned to when
    /// unmaximized unless overridden by the compositor.
    #[inline]
    pub fn send_set_maximized(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= xdg_toplevel#{}.set_maximized()", id);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            9,
        ]);
        Ok(())
    }

    /// Since when the unset_maximized message is available.
    #[allow(dead_code)]
    pub const MSG__UNSET_MAXIMIZED__SINCE: u32 = 1;

    /// unmaximize the window
    ///
    /// Unmaximize the surface.
    ///
    /// After requesting that the surface should be unmaximized, the compositor
    /// will respond by emitting a configure event. Whether this actually
    /// un-maximizes the window is subject to compositor policies.
    /// If available and applicable, the compositor will include the window
    /// geometry dimensions the window had prior to being maximized in the
    /// configure event. The client must then update its content, drawing it in
    /// the configured state. The client must also acknowledge the configure
    /// when committing the new content (see ack_configure).
    ///
    /// It is up to the compositor to position the surface after it was
    /// unmaximized; usually the position the surface had before maximizing, if
    /// applicable.
    ///
    /// If the surface was already not maximized, the compositor will still
    /// emit a configure event without the "maximized" state.
    ///
    /// If the surface is in a fullscreen state, this request has no direct
    /// effect. It may alter the state the surface is returned to when
    /// unmaximized unless overridden by the compositor.
    #[inline]
    pub fn send_unset_maximized(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= xdg_toplevel#{}.unset_maximized()", id);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            10,
        ]);
        Ok(())
    }

    /// Since when the set_fullscreen message is available.
    #[allow(dead_code)]
    pub const MSG__SET_FULLSCREEN__SINCE: u32 = 1;

    /// set the window as fullscreen on an output
    ///
    /// Make the surface fullscreen.
    ///
    /// After requesting that the surface should be fullscreened, the
    /// compositor will respond by emitting a configure event. Whether the
    /// client is actually put into a fullscreen state is subject to compositor
    /// policies. The client must also acknowledge the configure when
    /// committing the new content (see ack_configure).
    ///
    /// The output passed by the request indicates the client's preference as
    /// to which display it should be set fullscreen on. If this value is NULL,
    /// it's up to the compositor to choose which display will be used to map
    /// this surface.
    ///
    /// If the surface doesn't cover the whole output, the compositor will
    /// position the surface in the center of the output and compensate with
    /// with border fill covering the rest of the output. The content of the
    /// border fill is undefined, but should be assumed to be in some way that
    /// attempts to blend into the surrounding area (e.g. solid black).
    ///
    /// If the fullscreened surface is not opaque, the compositor must make
    /// sure that other screen content not part of the same surface tree (made
    /// up of subsurfaces, popups or similarly coupled surfaces) are not
    /// visible below the fullscreened surface.
    ///
    /// # Arguments
    ///
    /// - `output`:
    #[inline]
    pub fn send_set_fullscreen(
        &self,
        output: Option<&Rc<MetaWlOutput>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            output,
        );
        let arg0 = arg0.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError::ArgNoServerId("output")),
                Some(id) => id,
            },
        };
        eprintln!("server      <= xdg_toplevel#{}.set_fullscreen(output: wl_output#{})", id, arg0_id);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            11,
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the unset_fullscreen message is available.
    #[allow(dead_code)]
    pub const MSG__UNSET_FULLSCREEN__SINCE: u32 = 1;

    /// unset the window as fullscreen
    ///
    /// Make the surface no longer fullscreen.
    ///
    /// After requesting that the surface should be unfullscreened, the
    /// compositor will respond by emitting a configure event.
    /// Whether this actually removes the fullscreen state of the client is
    /// subject to compositor policies.
    ///
    /// Making a surface unfullscreen sets states for the surface based on the following:
    /// * the state(s) it may have had before becoming fullscreen
    /// * any state(s) decided by the compositor
    /// * any state(s) requested by the client while the surface was fullscreen
    ///
    /// The compositor may include the previous window geometry dimensions in
    /// the configure event, if applicable.
    ///
    /// The client must also acknowledge the configure when committing the new
    /// content (see ack_configure).
    #[inline]
    pub fn send_unset_fullscreen(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= xdg_toplevel#{}.unset_fullscreen()", id);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            12,
        ]);
        Ok(())
    }

    /// Since when the set_minimized message is available.
    #[allow(dead_code)]
    pub const MSG__SET_MINIMIZED__SINCE: u32 = 1;

    /// set the window as minimized
    ///
    /// Request that the compositor minimize your surface. There is no
    /// way to know if the surface is currently minimized, nor is there
    /// any way to unset minimization on this surface.
    ///
    /// If you are looking to throttle redrawing when minimized, please
    /// instead use the wl_surface.frame event for this, as this will
    /// also work with live previews on windows in Alt-Tab, Expose or
    /// similar compositor features.
    #[inline]
    pub fn send_set_minimized(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= xdg_toplevel#{}.set_minimized()", id);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            13,
        ]);
        Ok(())
    }

    /// Since when the configure message is available.
    #[allow(dead_code)]
    pub const MSG__CONFIGURE__SINCE: u32 = 1;

    /// suggest a surface change
    ///
    /// This configure event asks the client to resize its toplevel surface or
    /// to change its state. The configured state should not be applied
    /// immediately. See xdg_surface.configure for details.
    ///
    /// The width and height arguments specify a hint to the window
    /// about how its surface should be resized in window geometry
    /// coordinates. See set_window_geometry.
    ///
    /// If the width or height arguments are zero, it means the client
    /// should decide its own window dimension. This may happen when the
    /// compositor needs to configure the state of the surface but doesn't
    /// have any information about any previous or expected dimension.
    ///
    /// The states listed in the event specify how the width/height
    /// arguments should be interpreted, and possibly how it should be
    /// drawn.
    ///
    /// Clients must send an ack_configure in response to this event. See
    /// xdg_surface.configure and xdg_surface.ack_configure for details.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    /// - `states`:
    #[inline]
    pub fn send_configure(
        &self,
        width: i32,
        height: i32,
        states: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            width,
            height,
            states,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= xdg_toplevel#{}.configure(width: {}, height: {}, states: {})", client.endpoint.id, id, arg0, arg1, debug_array(arg2));
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0 as u32,
            arg1 as u32,
        ]);
        fmt.array(arg2);
        Ok(())
    }

    /// Since when the close message is available.
    #[allow(dead_code)]
    pub const MSG__CLOSE__SINCE: u32 = 1;

    /// surface wants to be closed
    ///
    /// The close event is sent by the compositor when the user
    /// wants the surface to be closed. This should be equivalent to
    /// the user clicking the close button in client-side decorations,
    /// if your application has any.
    ///
    /// This is only a request that the user intends to close the
    /// window. The client may choose to ignore this request, or show
    /// a dialog to ask the user to save their data, etc.
    #[inline]
    pub fn send_close(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= xdg_toplevel#{}.close()", client.endpoint.id, id);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the configure_bounds message is available.
    #[allow(dead_code)]
    pub const MSG__CONFIGURE_BOUNDS__SINCE: u32 = 4;

    /// recommended window geometry bounds
    ///
    /// The configure_bounds event may be sent prior to a xdg_toplevel.configure
    /// event to communicate the bounds a window geometry size is recommended
    /// to constrain to.
    ///
    /// The passed width and height are in surface coordinate space. If width
    /// and height are 0, it means bounds is unknown and equivalent to as if no
    /// configure_bounds event was ever sent for this surface.
    ///
    /// The bounds can for example correspond to the size of a monitor excluding
    /// any panels or other shell components, so that a surface isn't created in
    /// a way that it cannot fit.
    ///
    /// The bounds may change at any point, and in such a case, a new
    /// xdg_toplevel.configure_bounds will be sent, followed by
    /// xdg_toplevel.configure and xdg_surface.configure.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn send_configure_bounds(
        &self,
        width: i32,
        height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            width,
            height,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= xdg_toplevel#{}.configure_bounds(width: {}, height: {})", client.endpoint.id, id, arg0, arg1);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// Since when the wm_capabilities message is available.
    #[allow(dead_code)]
    pub const MSG__WM_CAPABILITIES__SINCE: u32 = 5;

    /// compositor capabilities
    ///
    /// This event advertises the capabilities supported by the compositor. If
    /// a capability isn't supported, clients should hide or disable the UI
    /// elements that expose this functionality. For instance, if the
    /// compositor doesn't advertise support for minimized toplevels, a button
    /// triggering the set_minimized request should not be displayed.
    ///
    /// The compositor will ignore requests it doesn't support. For instance,
    /// a compositor which doesn't advertise support for minimized will ignore
    /// set_minimized requests.
    ///
    /// Compositors must send this event once before the first
    /// xdg_surface.configure event. When the capabilities change, compositors
    /// must send this event again and then send an xdg_surface.configure
    /// event.
    ///
    /// The configured state should not be applied immediately. See
    /// xdg_surface.configure for details.
    ///
    /// The capabilities are sent as an array of 32-bit unsigned integers in
    /// native endianness.
    ///
    /// # Arguments
    ///
    /// - `capabilities`: array of 32-bit capabilities
    #[inline]
    pub fn send_wm_capabilities(
        &self,
        capabilities: &[u8],
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
        eprintln!("client#{:04} <= xdg_toplevel#{}.wm_capabilities(capabilities: {})", client.endpoint.id, id, debug_array(arg0));
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            3,
        ]);
        fmt.array(arg0);
        Ok(())
    }
}

/// A message handler for [XdgToplevel] proxies.
#[allow(dead_code)]
pub trait MetaXdgToplevelMessageHandler {
    /// destroy the xdg_toplevel
    ///
    /// This request destroys the role surface and unmaps the surface;
    /// see "Unmapping" behavior in interface section for details.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaXdgToplevel>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel.destroy message: {}", Report::new(e));
        }
    }

    /// set the parent of this surface
    ///
    /// Set the "parent" of this surface. This surface should be stacked
    /// above the parent surface and all other ancestor surfaces.
    ///
    /// Parent surfaces should be set on dialogs, toolboxes, or other
    /// "auxiliary" surfaces, so that the parent is raised when the dialog
    /// is raised.
    ///
    /// Setting a null parent for a child surface unsets its parent. Setting
    /// a null parent for a surface which currently has no parent is a no-op.
    ///
    /// Only mapped surfaces can have child surfaces. Setting a parent which
    /// is not mapped is equivalent to setting a null parent. If a surface
    /// becomes unmapped, its children's parent is set to the parent of
    /// the now-unmapped surface. If the now-unmapped surface has no parent,
    /// its children's parent is unset. If the now-unmapped surface becomes
    /// mapped again, its parent-child relationship is not restored.
    ///
    /// The parent toplevel must not be one of the child toplevel's
    /// descendants, and the parent must be different from the child toplevel,
    /// otherwise the invalid_parent protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `parent`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn set_parent(
        &mut self,
        _slf: &Rc<MetaXdgToplevel>,
        parent: Option<&Rc<MetaXdgToplevel>>,
    ) {
        let res = _slf.send_set_parent(
            parent,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel.set_parent message: {}", Report::new(e));
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
    /// - `title`:
    #[inline]
    fn set_title(
        &mut self,
        _slf: &Rc<MetaXdgToplevel>,
        title: &str,
    ) {
        let res = _slf.send_set_title(
            title,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel.set_title message: {}", Report::new(e));
        }
    }

    /// set application ID
    ///
    /// Set an application identifier for the surface.
    ///
    /// The app ID identifies the general class of applications to which
    /// the surface belongs. The compositor can use this to group multiple
    /// surfaces together, or to determine how to launch a new application.
    ///
    /// For D-Bus activatable applications, the app ID is used as the D-Bus
    /// service name.
    ///
    /// The compositor shell will try to group application surfaces together
    /// by their app ID. As a best practice, it is suggested to select app
    /// ID's that match the basename of the application's .desktop file.
    /// For example, "org.freedesktop.FooViewer" where the .desktop file is
    /// "org.freedesktop.FooViewer.desktop".
    ///
    /// Like other properties, a set_app_id request can be sent after the
    /// xdg_toplevel has been mapped to update the property.
    ///
    /// See the desktop-entry specification [0] for more details on
    /// application identifiers and how they relate to well-known D-Bus
    /// names and .desktop files.
    ///
    /// [0] https://standards.freedesktop.org/desktop-entry-spec/
    ///
    /// # Arguments
    ///
    /// - `app_id`:
    #[inline]
    fn set_app_id(
        &mut self,
        _slf: &Rc<MetaXdgToplevel>,
        app_id: &str,
    ) {
        let res = _slf.send_set_app_id(
            app_id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel.set_app_id message: {}", Report::new(e));
        }
    }

    /// show the window menu
    ///
    /// Clients implementing client-side decorations might want to show
    /// a context menu when right-clicking on the decorations, giving the
    /// user a menu that they can use to maximize or minimize the window.
    ///
    /// This request asks the compositor to pop up such a window menu at
    /// the given position, relative to the local surface coordinates of
    /// the parent surface. There are no guarantees as to what menu items
    /// the window menu contains, or even if a window menu will be drawn
    /// at all.
    ///
    /// This request must be used in response to some sort of user action
    /// like a button press, key press, or touch down event.
    ///
    /// # Arguments
    ///
    /// - `seat`: the wl_seat of the user event
    /// - `serial`: the serial of the user event
    /// - `x`: the x position to pop up the window menu at
    /// - `y`: the y position to pop up the window menu at
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn show_window_menu(
        &mut self,
        _slf: &Rc<MetaXdgToplevel>,
        seat: &Rc<MetaWlSeat>,
        serial: u32,
        x: i32,
        y: i32,
    ) {
        let res = _slf.send_show_window_menu(
            seat,
            serial,
            x,
            y,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel.show_window_menu message: {}", Report::new(e));
        }
    }

    /// start an interactive move
    ///
    /// Start an interactive, user-driven move of the surface.
    ///
    /// This request must be used in response to some sort of user action
    /// like a button press, key press, or touch down event. The passed
    /// serial is used to determine the type of interactive move (touch,
    /// pointer, etc).
    ///
    /// The server may ignore move requests depending on the state of
    /// the surface (e.g. fullscreen or maximized), or if the passed serial
    /// is no longer valid.
    ///
    /// If triggered, the surface will lose the focus of the device
    /// (wl_pointer, wl_touch, etc) used for the move. It is up to the
    /// compositor to visually indicate that the move is taking place, such as
    /// updating a pointer cursor, during the move. There is no guarantee
    /// that the device focus will return when the move is completed.
    ///
    /// # Arguments
    ///
    /// - `seat`: the wl_seat of the user event
    /// - `serial`: the serial of the user event
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn r#move(
        &mut self,
        _slf: &Rc<MetaXdgToplevel>,
        seat: &Rc<MetaWlSeat>,
        serial: u32,
    ) {
        let res = _slf.send_move(
            seat,
            serial,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel.move message: {}", Report::new(e));
        }
    }

    /// start an interactive resize
    ///
    /// Start a user-driven, interactive resize of the surface.
    ///
    /// This request must be used in response to some sort of user action
    /// like a button press, key press, or touch down event. The passed
    /// serial is used to determine the type of interactive resize (touch,
    /// pointer, etc).
    ///
    /// The server may ignore resize requests depending on the state of
    /// the surface (e.g. fullscreen or maximized).
    ///
    /// If triggered, the client will receive configure events with the
    /// "resize" state enum value and the expected sizes. See the "resize"
    /// enum value for more details about what is required. The client
    /// must also acknowledge configure events using "ack_configure". After
    /// the resize is completed, the client will receive another "configure"
    /// event without the resize state.
    ///
    /// If triggered, the surface also will lose the focus of the device
    /// (wl_pointer, wl_touch, etc) used for the resize. It is up to the
    /// compositor to visually indicate that the resize is taking place,
    /// such as updating a pointer cursor, during the resize. There is no
    /// guarantee that the device focus will return when the resize is
    /// completed.
    ///
    /// The edges parameter specifies how the surface should be resized, and
    /// is one of the values of the resize_edge enum. Values not matching
    /// a variant of the enum will cause the invalid_resize_edge protocol error.
    /// The compositor may use this information to update the surface position
    /// for example when dragging the top left corner. The compositor may also
    /// use this information to adapt its behavior, e.g. choose an appropriate
    /// cursor image.
    ///
    /// # Arguments
    ///
    /// - `seat`: the wl_seat of the user event
    /// - `serial`: the serial of the user event
    /// - `edges`: which edge or corner is being dragged
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn resize(
        &mut self,
        _slf: &Rc<MetaXdgToplevel>,
        seat: &Rc<MetaWlSeat>,
        serial: u32,
        edges: MetaXdgToplevelResizeEdge,
    ) {
        let res = _slf.send_resize(
            seat,
            serial,
            edges,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel.resize message: {}", Report::new(e));
        }
    }

    /// set the maximum size
    ///
    /// Set a maximum size for the window.
    ///
    /// The client can specify a maximum size so that the compositor does
    /// not try to configure the window beyond this size.
    ///
    /// The width and height arguments are in window geometry coordinates.
    /// See xdg_surface.set_window_geometry.
    ///
    /// Values set in this way are double-buffered, see wl_surface.commit.
    ///
    /// The compositor can use this information to allow or disallow
    /// different states like maximize or fullscreen and draw accurate
    /// animations.
    ///
    /// Similarly, a tiling window manager may use this information to
    /// place and resize client windows in a more effective way.
    ///
    /// The client should not rely on the compositor to obey the maximum
    /// size. The compositor may decide to ignore the values set by the
    /// client and request a larger size.
    ///
    /// If never set, or a value of zero in the request, means that the
    /// client has no expected maximum size in the given dimension.
    /// As a result, a client wishing to reset the maximum size
    /// to an unspecified state can use zero for width and height in the
    /// request.
    ///
    /// Requesting a maximum size to be smaller than the minimum size of
    /// a surface is illegal and will result in an invalid_size error.
    ///
    /// The width and height must be greater than or equal to zero. Using
    /// strictly negative values for width or height will result in a
    /// invalid_size error.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    #[inline]
    fn set_max_size(
        &mut self,
        _slf: &Rc<MetaXdgToplevel>,
        width: i32,
        height: i32,
    ) {
        let res = _slf.send_set_max_size(
            width,
            height,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel.set_max_size message: {}", Report::new(e));
        }
    }

    /// set the minimum size
    ///
    /// Set a minimum size for the window.
    ///
    /// The client can specify a minimum size so that the compositor does
    /// not try to configure the window below this size.
    ///
    /// The width and height arguments are in window geometry coordinates.
    /// See xdg_surface.set_window_geometry.
    ///
    /// Values set in this way are double-buffered, see wl_surface.commit.
    ///
    /// The compositor can use this information to allow or disallow
    /// different states like maximize or fullscreen and draw accurate
    /// animations.
    ///
    /// Similarly, a tiling window manager may use this information to
    /// place and resize client windows in a more effective way.
    ///
    /// The client should not rely on the compositor to obey the minimum
    /// size. The compositor may decide to ignore the values set by the
    /// client and request a smaller size.
    ///
    /// If never set, or a value of zero in the request, means that the
    /// client has no expected minimum size in the given dimension.
    /// As a result, a client wishing to reset the minimum size
    /// to an unspecified state can use zero for width and height in the
    /// request.
    ///
    /// Requesting a minimum size to be larger than the maximum size of
    /// a surface is illegal and will result in an invalid_size error.
    ///
    /// The width and height must be greater than or equal to zero. Using
    /// strictly negative values for width and height will result in a
    /// invalid_size error.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    #[inline]
    fn set_min_size(
        &mut self,
        _slf: &Rc<MetaXdgToplevel>,
        width: i32,
        height: i32,
    ) {
        let res = _slf.send_set_min_size(
            width,
            height,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel.set_min_size message: {}", Report::new(e));
        }
    }

    /// maximize the window
    ///
    /// Maximize the surface.
    ///
    /// After requesting that the surface should be maximized, the compositor
    /// will respond by emitting a configure event. Whether this configure
    /// actually sets the window maximized is subject to compositor policies.
    /// The client must then update its content, drawing in the configured
    /// state. The client must also acknowledge the configure when committing
    /// the new content (see ack_configure).
    ///
    /// It is up to the compositor to decide how and where to maximize the
    /// surface, for example which output and what region of the screen should
    /// be used.
    ///
    /// If the surface was already maximized, the compositor will still emit
    /// a configure event with the "maximized" state.
    ///
    /// If the surface is in a fullscreen state, this request has no direct
    /// effect. It may alter the state the surface is returned to when
    /// unmaximized unless overridden by the compositor.
    #[inline]
    fn set_maximized(
        &mut self,
        _slf: &Rc<MetaXdgToplevel>,
    ) {
        let res = _slf.send_set_maximized(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel.set_maximized message: {}", Report::new(e));
        }
    }

    /// unmaximize the window
    ///
    /// Unmaximize the surface.
    ///
    /// After requesting that the surface should be unmaximized, the compositor
    /// will respond by emitting a configure event. Whether this actually
    /// un-maximizes the window is subject to compositor policies.
    /// If available and applicable, the compositor will include the window
    /// geometry dimensions the window had prior to being maximized in the
    /// configure event. The client must then update its content, drawing it in
    /// the configured state. The client must also acknowledge the configure
    /// when committing the new content (see ack_configure).
    ///
    /// It is up to the compositor to position the surface after it was
    /// unmaximized; usually the position the surface had before maximizing, if
    /// applicable.
    ///
    /// If the surface was already not maximized, the compositor will still
    /// emit a configure event without the "maximized" state.
    ///
    /// If the surface is in a fullscreen state, this request has no direct
    /// effect. It may alter the state the surface is returned to when
    /// unmaximized unless overridden by the compositor.
    #[inline]
    fn unset_maximized(
        &mut self,
        _slf: &Rc<MetaXdgToplevel>,
    ) {
        let res = _slf.send_unset_maximized(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel.unset_maximized message: {}", Report::new(e));
        }
    }

    /// set the window as fullscreen on an output
    ///
    /// Make the surface fullscreen.
    ///
    /// After requesting that the surface should be fullscreened, the
    /// compositor will respond by emitting a configure event. Whether the
    /// client is actually put into a fullscreen state is subject to compositor
    /// policies. The client must also acknowledge the configure when
    /// committing the new content (see ack_configure).
    ///
    /// The output passed by the request indicates the client's preference as
    /// to which display it should be set fullscreen on. If this value is NULL,
    /// it's up to the compositor to choose which display will be used to map
    /// this surface.
    ///
    /// If the surface doesn't cover the whole output, the compositor will
    /// position the surface in the center of the output and compensate with
    /// with border fill covering the rest of the output. The content of the
    /// border fill is undefined, but should be assumed to be in some way that
    /// attempts to blend into the surrounding area (e.g. solid black).
    ///
    /// If the fullscreened surface is not opaque, the compositor must make
    /// sure that other screen content not part of the same surface tree (made
    /// up of subsurfaces, popups or similarly coupled surfaces) are not
    /// visible below the fullscreened surface.
    ///
    /// # Arguments
    ///
    /// - `output`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn set_fullscreen(
        &mut self,
        _slf: &Rc<MetaXdgToplevel>,
        output: Option<&Rc<MetaWlOutput>>,
    ) {
        let res = _slf.send_set_fullscreen(
            output,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel.set_fullscreen message: {}", Report::new(e));
        }
    }

    /// unset the window as fullscreen
    ///
    /// Make the surface no longer fullscreen.
    ///
    /// After requesting that the surface should be unfullscreened, the
    /// compositor will respond by emitting a configure event.
    /// Whether this actually removes the fullscreen state of the client is
    /// subject to compositor policies.
    ///
    /// Making a surface unfullscreen sets states for the surface based on the following:
    /// * the state(s) it may have had before becoming fullscreen
    /// * any state(s) decided by the compositor
    /// * any state(s) requested by the client while the surface was fullscreen
    ///
    /// The compositor may include the previous window geometry dimensions in
    /// the configure event, if applicable.
    ///
    /// The client must also acknowledge the configure when committing the new
    /// content (see ack_configure).
    #[inline]
    fn unset_fullscreen(
        &mut self,
        _slf: &Rc<MetaXdgToplevel>,
    ) {
        let res = _slf.send_unset_fullscreen(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel.unset_fullscreen message: {}", Report::new(e));
        }
    }

    /// set the window as minimized
    ///
    /// Request that the compositor minimize your surface. There is no
    /// way to know if the surface is currently minimized, nor is there
    /// any way to unset minimization on this surface.
    ///
    /// If you are looking to throttle redrawing when minimized, please
    /// instead use the wl_surface.frame event for this, as this will
    /// also work with live previews on windows in Alt-Tab, Expose or
    /// similar compositor features.
    #[inline]
    fn set_minimized(
        &mut self,
        _slf: &Rc<MetaXdgToplevel>,
    ) {
        let res = _slf.send_set_minimized(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel.set_minimized message: {}", Report::new(e));
        }
    }

    /// suggest a surface change
    ///
    /// This configure event asks the client to resize its toplevel surface or
    /// to change its state. The configured state should not be applied
    /// immediately. See xdg_surface.configure for details.
    ///
    /// The width and height arguments specify a hint to the window
    /// about how its surface should be resized in window geometry
    /// coordinates. See set_window_geometry.
    ///
    /// If the width or height arguments are zero, it means the client
    /// should decide its own window dimension. This may happen when the
    /// compositor needs to configure the state of the surface but doesn't
    /// have any information about any previous or expected dimension.
    ///
    /// The states listed in the event specify how the width/height
    /// arguments should be interpreted, and possibly how it should be
    /// drawn.
    ///
    /// Clients must send an ack_configure in response to this event. See
    /// xdg_surface.configure and xdg_surface.ack_configure for details.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    /// - `states`:
    #[inline]
    fn configure(
        &mut self,
        _slf: &Rc<MetaXdgToplevel>,
        width: i32,
        height: i32,
        states: &[u8],
    ) {
        let res = _slf.send_configure(
            width,
            height,
            states,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel.configure message: {}", Report::new(e));
        }
    }

    /// surface wants to be closed
    ///
    /// The close event is sent by the compositor when the user
    /// wants the surface to be closed. This should be equivalent to
    /// the user clicking the close button in client-side decorations,
    /// if your application has any.
    ///
    /// This is only a request that the user intends to close the
    /// window. The client may choose to ignore this request, or show
    /// a dialog to ask the user to save their data, etc.
    #[inline]
    fn close(
        &mut self,
        _slf: &Rc<MetaXdgToplevel>,
    ) {
        let res = _slf.send_close(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel.close message: {}", Report::new(e));
        }
    }

    /// recommended window geometry bounds
    ///
    /// The configure_bounds event may be sent prior to a xdg_toplevel.configure
    /// event to communicate the bounds a window geometry size is recommended
    /// to constrain to.
    ///
    /// The passed width and height are in surface coordinate space. If width
    /// and height are 0, it means bounds is unknown and equivalent to as if no
    /// configure_bounds event was ever sent for this surface.
    ///
    /// The bounds can for example correspond to the size of a monitor excluding
    /// any panels or other shell components, so that a surface isn't created in
    /// a way that it cannot fit.
    ///
    /// The bounds may change at any point, and in such a case, a new
    /// xdg_toplevel.configure_bounds will be sent, followed by
    /// xdg_toplevel.configure and xdg_surface.configure.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    #[inline]
    fn configure_bounds(
        &mut self,
        _slf: &Rc<MetaXdgToplevel>,
        width: i32,
        height: i32,
    ) {
        let res = _slf.send_configure_bounds(
            width,
            height,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel.configure_bounds message: {}", Report::new(e));
        }
    }

    /// compositor capabilities
    ///
    /// This event advertises the capabilities supported by the compositor. If
    /// a capability isn't supported, clients should hide or disable the UI
    /// elements that expose this functionality. For instance, if the
    /// compositor doesn't advertise support for minimized toplevels, a button
    /// triggering the set_minimized request should not be displayed.
    ///
    /// The compositor will ignore requests it doesn't support. For instance,
    /// a compositor which doesn't advertise support for minimized will ignore
    /// set_minimized requests.
    ///
    /// Compositors must send this event once before the first
    /// xdg_surface.configure event. When the capabilities change, compositors
    /// must send this event again and then send an xdg_surface.configure
    /// event.
    ///
    /// The configured state should not be applied immediately. See
    /// xdg_surface.configure for details.
    ///
    /// The capabilities are sent as an array of 32-bit unsigned integers in
    /// native endianness.
    ///
    /// # Arguments
    ///
    /// - `capabilities`: array of 32-bit capabilities
    #[inline]
    fn wm_capabilities(
        &mut self,
        _slf: &Rc<MetaXdgToplevel>,
        capabilities: &[u8],
    ) {
        let res = _slf.send_wm_capabilities(
            capabilities,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel.wm_capabilities message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaXdgToplevel {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("client#{:04} -> xdg_toplevel#{}.destroy()", client.endpoint.id, msg[0]);
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                eprintln!("client#{:04} -> xdg_toplevel#{}.set_parent(parent: xdg_toplevel#{})", client.endpoint.id, msg[0], arg0);
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaXdgToplevel>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError::WrongObjectType("parent", o.core().interface, ProxyInterface::XdgToplevel));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).set_parent(&self, arg0);
                } else {
                    DefaultMessageHandler.set_parent(&self, arg0);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("title"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("title"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("title"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("title"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                eprintln!("client#{:04} -> xdg_toplevel#{}.set_title(title: {:?})", client.endpoint.id, msg[0], arg0);
                if let Some(handler) = handler {
                    (**handler).set_title(&self, arg0);
                } else {
                    DefaultMessageHandler.set_title(&self, arg0);
                }
            }
            3 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("app_id"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("app_id"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("app_id"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("app_id"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                eprintln!("client#{:04} -> xdg_toplevel#{}.set_app_id(app_id: {:?})", client.endpoint.id, msg[0], arg0);
                if let Some(handler) = handler {
                    (**handler).set_app_id(&self, arg0);
                } else {
                    DefaultMessageHandler.set_app_id(&self, arg0);
                }
            }
            4 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 24));
                };
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                eprintln!("client#{:04} -> xdg_toplevel#{}.show_window_menu(seat: wl_seat#{}, serial: {}, x: {}, y: {})", client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlSeat>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("seat", o.core().interface, ProxyInterface::WlSeat));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).show_window_menu(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultMessageHandler.show_window_menu(&self, arg0, arg1, arg2, arg3);
                }
            }
            5 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                eprintln!("client#{:04} -> xdg_toplevel#{}.move(seat: wl_seat#{}, serial: {})", client.endpoint.id, msg[0], arg0, arg1);
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlSeat>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("seat", o.core().interface, ProxyInterface::WlSeat));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).r#move(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.r#move(&self, arg0, arg1);
                }
            }
            6 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                let arg2 = MetaXdgToplevelResizeEdge(arg2);
                eprintln!("client#{:04} -> xdg_toplevel#{}.resize(seat: wl_seat#{}, serial: {}, edges: {:?})", client.endpoint.id, msg[0], arg0, arg1, arg2);
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlSeat>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("seat", o.core().interface, ProxyInterface::WlSeat));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).resize(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.resize(&self, arg0, arg1, arg2);
                }
            }
            7 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                eprintln!("client#{:04} -> xdg_toplevel#{}.set_max_size(width: {}, height: {})", client.endpoint.id, msg[0], arg0, arg1);
                if let Some(handler) = handler {
                    (**handler).set_max_size(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.set_max_size(&self, arg0, arg1);
                }
            }
            8 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                eprintln!("client#{:04} -> xdg_toplevel#{}.set_min_size(width: {}, height: {})", client.endpoint.id, msg[0], arg0, arg1);
                if let Some(handler) = handler {
                    (**handler).set_min_size(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.set_min_size(&self, arg0, arg1);
                }
            }
            9 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("client#{:04} -> xdg_toplevel#{}.set_maximized()", client.endpoint.id, msg[0]);
                if let Some(handler) = handler {
                    (**handler).set_maximized(&self);
                } else {
                    DefaultMessageHandler.set_maximized(&self);
                }
            }
            10 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("client#{:04} -> xdg_toplevel#{}.unset_maximized()", client.endpoint.id, msg[0]);
                if let Some(handler) = handler {
                    (**handler).unset_maximized(&self);
                } else {
                    DefaultMessageHandler.unset_maximized(&self);
                }
            }
            11 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                eprintln!("client#{:04} -> xdg_toplevel#{}.set_fullscreen(output: wl_output#{})", client.endpoint.id, msg[0], arg0);
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlOutput>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError::WrongObjectType("output", o.core().interface, ProxyInterface::WlOutput));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).set_fullscreen(&self, arg0);
                } else {
                    DefaultMessageHandler.set_fullscreen(&self, arg0);
                }
            }
            12 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("client#{:04} -> xdg_toplevel#{}.unset_fullscreen()", client.endpoint.id, msg[0]);
                if let Some(handler) = handler {
                    (**handler).unset_fullscreen(&self);
                } else {
                    DefaultMessageHandler.unset_fullscreen(&self);
                }
            }
            13 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("client#{:04} -> xdg_toplevel#{}.set_minimized()", client.endpoint.id, msg[0]);
                if let Some(handler) = handler {
                    (**handler).set_minimized(&self);
                } else {
                    DefaultMessageHandler.set_minimized(&self);
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
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("width"));
                };
                offset += 1;
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("height"));
                };
                offset += 1;
                let arg2 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("states"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("states"));
                    }
                    let start = offset;
                    offset += words;
                    &uapi::as_bytes(&msg[start..])[..len]
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                eprintln!("server      -> xdg_toplevel#{}.configure(width: {}, height: {}, states: {})", msg[0], arg0, arg1, debug_array(arg2));
                if let Some(handler) = handler {
                    (**handler).configure(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.configure(&self, arg0, arg1, arg2);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("server      -> xdg_toplevel#{}.close()", msg[0]);
                if let Some(handler) = handler {
                    (**handler).close(&self);
                } else {
                    DefaultMessageHandler.close(&self);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                eprintln!("server      -> xdg_toplevel#{}.configure_bounds(width: {}, height: {})", msg[0], arg0, arg1);
                if let Some(handler) = handler {
                    (**handler).configure_bounds(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.configure_bounds(&self, arg0, arg1);
                }
            }
            3 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("capabilities"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("capabilities"));
                    }
                    let start = offset;
                    offset += words;
                    &uapi::as_bytes(&msg[start..])[..len]
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                eprintln!("server      -> xdg_toplevel#{}.wm_capabilities(capabilities: {})", msg[0], debug_array(arg0));
                if let Some(handler) = handler {
                    (**handler).wm_capabilities(&self, arg0);
                } else {
                    DefaultMessageHandler.wm_capabilities(&self, arg0);
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
            1 => "set_parent",
            2 => "set_title",
            3 => "set_app_id",
            4 => "show_window_menu",
            5 => "move",
            6 => "resize",
            7 => "set_max_size",
            8 => "set_min_size",
            9 => "set_maximized",
            10 => "unset_maximized",
            11 => "set_fullscreen",
            12 => "unset_fullscreen",
            13 => "set_minimized",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "configure",
            1 => "close",
            2 => "configure_bounds",
            3 => "wm_capabilities",
            _ => return None,
        };
        Some(name)
    }
}

impl MetaXdgToplevel {
    /// Since when the error.invalid_resize_edge enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_RESIZE_EDGE__SINCE: u32 = 1;
    /// Since when the error.invalid_parent enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_PARENT__SINCE: u32 = 1;
    /// Since when the error.invalid_size enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_SIZE__SINCE: u32 = 1;

    /// Since when the resize_edge.none enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_EDGE_NONE__SINCE: u32 = 1;
    /// Since when the resize_edge.top enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_EDGE_TOP__SINCE: u32 = 1;
    /// Since when the resize_edge.bottom enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_EDGE_BOTTOM__SINCE: u32 = 1;
    /// Since when the resize_edge.left enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_EDGE_LEFT__SINCE: u32 = 1;
    /// Since when the resize_edge.top_left enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_EDGE_TOP_LEFT__SINCE: u32 = 1;
    /// Since when the resize_edge.bottom_left enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_EDGE_BOTTOM_LEFT__SINCE: u32 = 1;
    /// Since when the resize_edge.right enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_EDGE_RIGHT__SINCE: u32 = 1;
    /// Since when the resize_edge.top_right enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_EDGE_TOP_RIGHT__SINCE: u32 = 1;
    /// Since when the resize_edge.bottom_right enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_EDGE_BOTTOM_RIGHT__SINCE: u32 = 1;

    /// Since when the state.maximized enum variant is available.
    #[allow(dead_code)]
    pub const ENM__STATE_MAXIMIZED__SINCE: u32 = 1;
    /// Since when the state.fullscreen enum variant is available.
    #[allow(dead_code)]
    pub const ENM__STATE_FULLSCREEN__SINCE: u32 = 1;
    /// Since when the state.resizing enum variant is available.
    #[allow(dead_code)]
    pub const ENM__STATE_RESIZING__SINCE: u32 = 1;
    /// Since when the state.activated enum variant is available.
    #[allow(dead_code)]
    pub const ENM__STATE_ACTIVATED__SINCE: u32 = 1;
    /// Since when the state.tiled_left enum variant is available.
    #[allow(dead_code)]
    pub const ENM__STATE_TILED_LEFT__SINCE: u32 = 2;
    /// Since when the state.tiled_right enum variant is available.
    #[allow(dead_code)]
    pub const ENM__STATE_TILED_RIGHT__SINCE: u32 = 2;
    /// Since when the state.tiled_top enum variant is available.
    #[allow(dead_code)]
    pub const ENM__STATE_TILED_TOP__SINCE: u32 = 2;
    /// Since when the state.tiled_bottom enum variant is available.
    #[allow(dead_code)]
    pub const ENM__STATE_TILED_BOTTOM__SINCE: u32 = 2;
    /// Since when the state.suspended enum variant is available.
    #[allow(dead_code)]
    pub const ENM__STATE_SUSPENDED__SINCE: u32 = 6;
    /// Since when the state.constrained_left enum variant is available.
    #[allow(dead_code)]
    pub const ENM__STATE_CONSTRAINED_LEFT__SINCE: u32 = 7;
    /// Since when the state.constrained_right enum variant is available.
    #[allow(dead_code)]
    pub const ENM__STATE_CONSTRAINED_RIGHT__SINCE: u32 = 7;
    /// Since when the state.constrained_top enum variant is available.
    #[allow(dead_code)]
    pub const ENM__STATE_CONSTRAINED_TOP__SINCE: u32 = 7;
    /// Since when the state.constrained_bottom enum variant is available.
    #[allow(dead_code)]
    pub const ENM__STATE_CONSTRAINED_BOTTOM__SINCE: u32 = 7;

    /// Since when the wm_capabilities.window_menu enum variant is available.
    #[allow(dead_code)]
    pub const ENM__WM_CAPABILITIES_WINDOW_MENU__SINCE: u32 = 1;
    /// Since when the wm_capabilities.maximize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__WM_CAPABILITIES_MAXIMIZE__SINCE: u32 = 1;
    /// Since when the wm_capabilities.fullscreen enum variant is available.
    #[allow(dead_code)]
    pub const ENM__WM_CAPABILITIES_FULLSCREEN__SINCE: u32 = 1;
    /// Since when the wm_capabilities.minimize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__WM_CAPABILITIES_MINIMIZE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaXdgToplevelError(pub u32);

impl MetaXdgToplevelError {
    /// provided value is
    ///         not a valid variant of the resize_edge enum
    #[allow(dead_code)]
    pub const INVALID_RESIZE_EDGE: Self = Self(0);

    /// invalid parent toplevel
    #[allow(dead_code)]
    pub const INVALID_PARENT: Self = Self(1);

    /// client provided an invalid min or max size
    #[allow(dead_code)]
    pub const INVALID_SIZE: Self = Self(2);
}

impl Debug for MetaXdgToplevelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_RESIZE_EDGE => "INVALID_RESIZE_EDGE",
            Self::INVALID_PARENT => "INVALID_PARENT",
            Self::INVALID_SIZE => "INVALID_SIZE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// edge values for resizing
///
/// These values are used to indicate which edge of a surface
/// is being dragged in a resize operation.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaXdgToplevelResizeEdge(pub u32);

impl MetaXdgToplevelResizeEdge {
    #[allow(dead_code)]
    pub const NONE: Self = Self(0);

    #[allow(dead_code)]
    pub const TOP: Self = Self(1);

    #[allow(dead_code)]
    pub const BOTTOM: Self = Self(2);

    #[allow(dead_code)]
    pub const LEFT: Self = Self(4);

    #[allow(dead_code)]
    pub const TOP_LEFT: Self = Self(5);

    #[allow(dead_code)]
    pub const BOTTOM_LEFT: Self = Self(6);

    #[allow(dead_code)]
    pub const RIGHT: Self = Self(8);

    #[allow(dead_code)]
    pub const TOP_RIGHT: Self = Self(9);

    #[allow(dead_code)]
    pub const BOTTOM_RIGHT: Self = Self(10);
}

impl Debug for MetaXdgToplevelResizeEdge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NONE => "NONE",
            Self::TOP => "TOP",
            Self::BOTTOM => "BOTTOM",
            Self::LEFT => "LEFT",
            Self::TOP_LEFT => "TOP_LEFT",
            Self::BOTTOM_LEFT => "BOTTOM_LEFT",
            Self::RIGHT => "RIGHT",
            Self::TOP_RIGHT => "TOP_RIGHT",
            Self::BOTTOM_RIGHT => "BOTTOM_RIGHT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// types of state on the surface
///
/// The different state values used on the surface. This is designed for
/// state values like maximized, fullscreen. It is paired with the
/// configure event to ensure that both the client and the compositor
/// setting the state can be synchronized.
///
/// States set in this way are double-buffered, see wl_surface.commit.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaXdgToplevelState(pub u32);

impl MetaXdgToplevelState {
    /// the surface is maximized
    ///
    /// the surface is maximized
    ///
    /// The surface is maximized. The window geometry specified in the configure
    /// event must be obeyed by the client, or the xdg_wm_base.invalid_surface_state
    /// error is raised.
    ///
    /// The client should draw without shadow or other
    /// decoration outside of the window geometry.
    #[allow(dead_code)]
    pub const MAXIMIZED: Self = Self(1);

    /// the surface is fullscreen
    ///
    /// the surface is fullscreen
    ///
    /// The surface is fullscreen. The window geometry specified in the
    /// configure event is a maximum; the client cannot resize beyond it. For
    /// a surface to cover the whole fullscreened area, the geometry
    /// dimensions must be obeyed by the client. For more details, see
    /// xdg_toplevel.set_fullscreen.
    #[allow(dead_code)]
    pub const FULLSCREEN: Self = Self(2);

    /// the surface is being resized
    ///
    /// the surface is being resized
    ///
    /// The surface is being resized. The window geometry specified in the
    /// configure event is a maximum; the client cannot resize beyond it.
    /// Clients that have aspect ratio or cell sizing configuration can use
    /// a smaller size, however.
    #[allow(dead_code)]
    pub const RESIZING: Self = Self(3);

    /// the surface is now activated
    ///
    /// the surface is now activated
    ///
    /// Client window decorations should be painted as if the window is
    /// active. Do not assume this means that the window actually has
    /// keyboard or pointer focus.
    #[allow(dead_code)]
    pub const ACTIVATED: Self = Self(4);

    /// the surface’s left edge is tiled
    ///
    /// The window is currently in a tiled layout and the left edge is
    /// considered to be adjacent to another part of the tiling grid.
    ///
    /// The client should draw without shadow or other decoration outside of
    /// the window geometry on the left edge.
    #[allow(dead_code)]
    pub const TILED_LEFT: Self = Self(5);

    /// the surface’s right edge is tiled
    ///
    /// The window is currently in a tiled layout and the right edge is
    /// considered to be adjacent to another part of the tiling grid.
    ///
    /// The client should draw without shadow or other decoration outside of
    /// the window geometry on the right edge.
    #[allow(dead_code)]
    pub const TILED_RIGHT: Self = Self(6);

    /// the surface’s top edge is tiled
    ///
    /// The window is currently in a tiled layout and the top edge is
    /// considered to be adjacent to another part of the tiling grid.
    ///
    /// The client should draw without shadow or other decoration outside of
    /// the window geometry on the top edge.
    #[allow(dead_code)]
    pub const TILED_TOP: Self = Self(7);

    /// the surface’s bottom edge is tiled
    ///
    /// The window is currently in a tiled layout and the bottom edge is
    /// considered to be adjacent to another part of the tiling grid.
    ///
    /// The client should draw without shadow or other decoration outside of
    /// the window geometry on the bottom edge.
    #[allow(dead_code)]
    pub const TILED_BOTTOM: Self = Self(8);

    /// surface repaint is suspended
    ///
    /// The surface is currently not ordinarily being repainted; for
    /// example because its content is occluded by another window, or its
    /// outputs are switched off due to screen locking.
    #[allow(dead_code)]
    pub const SUSPENDED: Self = Self(9);

    /// the surface’s left edge is constrained
    ///
    /// The left edge of the window is currently constrained, meaning it
    /// shouldn't attempt to resize from that edge. It can for example mean
    /// it's tiled next to a monitor edge on the constrained side of the
    /// window.
    #[allow(dead_code)]
    pub const CONSTRAINED_LEFT: Self = Self(10);

    /// the surface’s right edge is constrained
    ///
    /// The right edge of the window is currently constrained, meaning it
    /// shouldn't attempt to resize from that edge. It can for example mean
    /// it's tiled next to a monitor edge on the constrained side of the
    /// window.
    #[allow(dead_code)]
    pub const CONSTRAINED_RIGHT: Self = Self(11);

    /// the surface’s top edge is constrained
    ///
    /// The top edge of the window is currently constrained, meaning it
    /// shouldn't attempt to resize from that edge. It can for example mean
    /// it's tiled next to a monitor edge on the constrained side of the
    /// window.
    #[allow(dead_code)]
    pub const CONSTRAINED_TOP: Self = Self(12);

    /// the surface’s bottom edge is constrained
    ///
    /// The bottom edge of the window is currently constrained, meaning it
    /// shouldn't attempt to resize from that edge. It can for example mean
    /// it's tiled next to a monitor edge on the constrained side of the
    /// window.
    #[allow(dead_code)]
    pub const CONSTRAINED_BOTTOM: Self = Self(13);
}

impl Debug for MetaXdgToplevelState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::MAXIMIZED => "MAXIMIZED",
            Self::FULLSCREEN => "FULLSCREEN",
            Self::RESIZING => "RESIZING",
            Self::ACTIVATED => "ACTIVATED",
            Self::TILED_LEFT => "TILED_LEFT",
            Self::TILED_RIGHT => "TILED_RIGHT",
            Self::TILED_TOP => "TILED_TOP",
            Self::TILED_BOTTOM => "TILED_BOTTOM",
            Self::SUSPENDED => "SUSPENDED",
            Self::CONSTRAINED_LEFT => "CONSTRAINED_LEFT",
            Self::CONSTRAINED_RIGHT => "CONSTRAINED_RIGHT",
            Self::CONSTRAINED_TOP => "CONSTRAINED_TOP",
            Self::CONSTRAINED_BOTTOM => "CONSTRAINED_BOTTOM",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaXdgToplevelWmCapabilities(pub u32);

impl MetaXdgToplevelWmCapabilities {
    /// show_window_menu is available
    #[allow(dead_code)]
    pub const WINDOW_MENU: Self = Self(1);

    /// set_maximized and unset_maximized are available
    #[allow(dead_code)]
    pub const MAXIMIZE: Self = Self(2);

    /// set_fullscreen and unset_fullscreen are available
    #[allow(dead_code)]
    pub const FULLSCREEN: Self = Self(3);

    /// set_minimized is available
    #[allow(dead_code)]
    pub const MINIMIZE: Self = Self(4);
}

impl Debug for MetaXdgToplevelWmCapabilities {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::WINDOW_MENU => "WINDOW_MENU",
            Self::MAXIMIZE => "MAXIMIZE",
            Self::FULLSCREEN => "FULLSCREEN",
            Self::MINIMIZE => "MINIMIZE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
