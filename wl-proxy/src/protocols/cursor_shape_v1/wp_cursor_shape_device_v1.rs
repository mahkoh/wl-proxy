//! cursor shape for a device
//!
//! This interface allows clients to set the cursor shape.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_cursor_shape_device_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpCursorShapeDeviceV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpCursorShapeDeviceV1Handler>,
}

struct DefaultHandler;

impl WpCursorShapeDeviceV1Handler for DefaultHandler { }

impl ConcreteObject for WpCursorShapeDeviceV1 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::WpCursorShapeDeviceV1;
    const INTERFACE_NAME: &str = "wp_cursor_shape_device_v1";
}

impl WpCursorShapeDeviceV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WpCursorShapeDeviceV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WpCursorShapeDeviceV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpCursorShapeDeviceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpCursorShapeDeviceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpCursorShapeDeviceV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the cursor shape device
    ///
    /// Destroy the cursor shape device.
    ///
    /// The device cursor shape remains unchanged.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_cursor_shape_device_v1#{}.destroy()\n", id);
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

    /// destroy the cursor shape device
    ///
    /// Destroy the cursor shape device.
    ///
    /// The device cursor shape remains unchanged.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wp_cursor_shape_device_v1.destroy", &e);
        }
    }

    /// Since when the set_shape message is available.
    pub const MSG__SET_SHAPE__SINCE: u32 = 1;

    /// set device cursor to the shape
    ///
    /// Sets the device cursor to the specified shape. The compositor will
    /// change the cursor image based on the specified shape.
    ///
    /// The cursor actually changes only if the input device focus is one of
    /// the requesting client's surfaces. If any, the previous cursor image
    /// (surface or shape) is replaced.
    ///
    /// The "shape" argument must be a valid enum entry, otherwise the
    /// invalid_shape protocol error is raised.
    ///
    /// This is similar to the wl_pointer.set_cursor and
    /// zwp_tablet_tool_v2.set_cursor requests, but this request accepts a
    /// shape instead of contents in the form of a surface. Clients can mix
    /// set_cursor and set_shape requests.
    ///
    /// The serial parameter must match the latest wl_pointer.enter or
    /// zwp_tablet_tool_v2.proximity_in serial number sent to the client.
    /// Otherwise the request will be ignored.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the enter event
    /// - `shape`:
    #[inline]
    pub fn try_send_set_shape(
        &self,
        serial: u32,
        shape: WpCursorShapeDeviceV1Shape,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            serial,
            shape,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: WpCursorShapeDeviceV1Shape) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_cursor_shape_device_v1#{}.set_shape(serial: {}, shape: {:?})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1);
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
            arg0,
            arg1.0,
        ]);
        Ok(())
    }

    /// set device cursor to the shape
    ///
    /// Sets the device cursor to the specified shape. The compositor will
    /// change the cursor image based on the specified shape.
    ///
    /// The cursor actually changes only if the input device focus is one of
    /// the requesting client's surfaces. If any, the previous cursor image
    /// (surface or shape) is replaced.
    ///
    /// The "shape" argument must be a valid enum entry, otherwise the
    /// invalid_shape protocol error is raised.
    ///
    /// This is similar to the wl_pointer.set_cursor and
    /// zwp_tablet_tool_v2.set_cursor requests, but this request accepts a
    /// shape instead of contents in the form of a surface. Clients can mix
    /// set_cursor and set_shape requests.
    ///
    /// The serial parameter must match the latest wl_pointer.enter or
    /// zwp_tablet_tool_v2.proximity_in serial number sent to the client.
    /// Otherwise the request will be ignored.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the enter event
    /// - `shape`:
    #[inline]
    pub fn send_set_shape(
        &self,
        serial: u32,
        shape: WpCursorShapeDeviceV1Shape,
    ) {
        let res = self.try_send_set_shape(
            serial,
            shape,
        );
        if let Err(e) = res {
            log_send("wp_cursor_shape_device_v1.set_shape", &e);
        }
    }
}

/// A message handler for [`WpCursorShapeDeviceV1`] proxies.
pub trait WpCursorShapeDeviceV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpCursorShapeDeviceV1>) {
        slf.core.delete_id();
    }

    /// destroy the cursor shape device
    ///
    /// Destroy the cursor shape device.
    ///
    /// The device cursor shape remains unchanged.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WpCursorShapeDeviceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wp_cursor_shape_device_v1.destroy", &e);
        }
    }

    /// set device cursor to the shape
    ///
    /// Sets the device cursor to the specified shape. The compositor will
    /// change the cursor image based on the specified shape.
    ///
    /// The cursor actually changes only if the input device focus is one of
    /// the requesting client's surfaces. If any, the previous cursor image
    /// (surface or shape) is replaced.
    ///
    /// The "shape" argument must be a valid enum entry, otherwise the
    /// invalid_shape protocol error is raised.
    ///
    /// This is similar to the wl_pointer.set_cursor and
    /// zwp_tablet_tool_v2.set_cursor requests, but this request accepts a
    /// shape instead of contents in the form of a surface. Clients can mix
    /// set_cursor and set_shape requests.
    ///
    /// The serial parameter must match the latest wl_pointer.enter or
    /// zwp_tablet_tool_v2.proximity_in serial number sent to the client.
    /// Otherwise the request will be ignored.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the enter event
    /// - `shape`:
    #[inline]
    fn handle_set_shape(
        &mut self,
        slf: &Rc<WpCursorShapeDeviceV1>,
        serial: u32,
        shape: WpCursorShapeDeviceV1Shape,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_shape(
            serial,
            shape,
        );
        if let Err(e) = res {
            log_forward("wp_cursor_shape_device_v1.set_shape", &e);
        }
    }
}

impl ObjectPrivate for WpCursorShapeDeviceV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpCursorShapeDeviceV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_cursor_shape_device_v1#{}.destroy()\n", client_id, id);
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
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = WpCursorShapeDeviceV1Shape(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: WpCursorShapeDeviceV1Shape) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_cursor_shape_device_v1#{}.set_shape(serial: {}, shape: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_shape(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_shape(&self, arg0, arg1);
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
            0 => "destroy",
            1 => "set_shape",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WpCursorShapeDeviceV1 {
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

impl WpCursorShapeDeviceV1 {
    /// Since when the shape.default enum variant is available.
    pub const ENM__SHAPE_DEFAULT__SINCE: u32 = 1;
    /// Since when the shape.context_menu enum variant is available.
    pub const ENM__SHAPE_CONTEXT_MENU__SINCE: u32 = 1;
    /// Since when the shape.help enum variant is available.
    pub const ENM__SHAPE_HELP__SINCE: u32 = 1;
    /// Since when the shape.pointer enum variant is available.
    pub const ENM__SHAPE_POINTER__SINCE: u32 = 1;
    /// Since when the shape.progress enum variant is available.
    pub const ENM__SHAPE_PROGRESS__SINCE: u32 = 1;
    /// Since when the shape.wait enum variant is available.
    pub const ENM__SHAPE_WAIT__SINCE: u32 = 1;
    /// Since when the shape.cell enum variant is available.
    pub const ENM__SHAPE_CELL__SINCE: u32 = 1;
    /// Since when the shape.crosshair enum variant is available.
    pub const ENM__SHAPE_CROSSHAIR__SINCE: u32 = 1;
    /// Since when the shape.text enum variant is available.
    pub const ENM__SHAPE_TEXT__SINCE: u32 = 1;
    /// Since when the shape.vertical_text enum variant is available.
    pub const ENM__SHAPE_VERTICAL_TEXT__SINCE: u32 = 1;
    /// Since when the shape.alias enum variant is available.
    pub const ENM__SHAPE_ALIAS__SINCE: u32 = 1;
    /// Since when the shape.copy enum variant is available.
    pub const ENM__SHAPE_COPY__SINCE: u32 = 1;
    /// Since when the shape.move enum variant is available.
    pub const ENM__SHAPE_MOVE__SINCE: u32 = 1;
    /// Since when the shape.no_drop enum variant is available.
    pub const ENM__SHAPE_NO_DROP__SINCE: u32 = 1;
    /// Since when the shape.not_allowed enum variant is available.
    pub const ENM__SHAPE_NOT_ALLOWED__SINCE: u32 = 1;
    /// Since when the shape.grab enum variant is available.
    pub const ENM__SHAPE_GRAB__SINCE: u32 = 1;
    /// Since when the shape.grabbing enum variant is available.
    pub const ENM__SHAPE_GRABBING__SINCE: u32 = 1;
    /// Since when the shape.e_resize enum variant is available.
    pub const ENM__SHAPE_E_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.n_resize enum variant is available.
    pub const ENM__SHAPE_N_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.ne_resize enum variant is available.
    pub const ENM__SHAPE_NE_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.nw_resize enum variant is available.
    pub const ENM__SHAPE_NW_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.s_resize enum variant is available.
    pub const ENM__SHAPE_S_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.se_resize enum variant is available.
    pub const ENM__SHAPE_SE_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.sw_resize enum variant is available.
    pub const ENM__SHAPE_SW_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.w_resize enum variant is available.
    pub const ENM__SHAPE_W_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.ew_resize enum variant is available.
    pub const ENM__SHAPE_EW_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.ns_resize enum variant is available.
    pub const ENM__SHAPE_NS_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.nesw_resize enum variant is available.
    pub const ENM__SHAPE_NESW_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.nwse_resize enum variant is available.
    pub const ENM__SHAPE_NWSE_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.col_resize enum variant is available.
    pub const ENM__SHAPE_COL_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.row_resize enum variant is available.
    pub const ENM__SHAPE_ROW_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.all_scroll enum variant is available.
    pub const ENM__SHAPE_ALL_SCROLL__SINCE: u32 = 1;
    /// Since when the shape.zoom_in enum variant is available.
    pub const ENM__SHAPE_ZOOM_IN__SINCE: u32 = 1;
    /// Since when the shape.zoom_out enum variant is available.
    pub const ENM__SHAPE_ZOOM_OUT__SINCE: u32 = 1;
    /// Since when the shape.dnd_ask enum variant is available.
    pub const ENM__SHAPE_DND_ASK__SINCE: u32 = 2;
    /// Since when the shape.all_resize enum variant is available.
    pub const ENM__SHAPE_ALL_RESIZE__SINCE: u32 = 2;

    /// Since when the error.invalid_shape enum variant is available.
    pub const ENM__ERROR_INVALID_SHAPE__SINCE: u32 = 1;
}

/// cursor shapes
///
/// This enum describes cursor shapes.
///
/// The names are taken from the CSS W3C specification:
/// https://w3c.github.io/csswg-drafts/css-ui/#cursor
/// with a few additions.
///
/// Note that there are some groups of cursor shapes that are related:
/// The first group is drag-and-drop cursors which are used to indicate
/// the selected action during dnd operations. The second group is resize
/// cursors which are used to indicate resizing and moving possibilities
/// on window borders. It is recommended that the shapes in these groups
/// should use visually compatible images and metaphors.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpCursorShapeDeviceV1Shape(pub u32);

impl WpCursorShapeDeviceV1Shape {
    /// default cursor
    pub const DEFAULT: Self = Self(1);

    /// a context menu is available for the object under the cursor
    pub const CONTEXT_MENU: Self = Self(2);

    /// help is available for the object under the cursor
    pub const HELP: Self = Self(3);

    /// pointer that indicates a link or another interactive element
    pub const POINTER: Self = Self(4);

    /// progress indicator
    pub const PROGRESS: Self = Self(5);

    /// program is busy, user should wait
    pub const WAIT: Self = Self(6);

    /// a cell or set of cells may be selected
    pub const CELL: Self = Self(7);

    /// simple crosshair
    pub const CROSSHAIR: Self = Self(8);

    /// text may be selected
    pub const TEXT: Self = Self(9);

    /// vertical text may be selected
    pub const VERTICAL_TEXT: Self = Self(10);

    /// drag-and-drop: alias of/shortcut to something is to be created
    pub const ALIAS: Self = Self(11);

    /// drag-and-drop: something is to be copied
    pub const COPY: Self = Self(12);

    /// drag-and-drop: something is to be moved
    pub const MOVE: Self = Self(13);

    /// drag-and-drop: the dragged item cannot be dropped at the current cursor location
    pub const NO_DROP: Self = Self(14);

    /// drag-and-drop: the requested action will not be carried out
    pub const NOT_ALLOWED: Self = Self(15);

    /// drag-and-drop: something can be grabbed
    pub const GRAB: Self = Self(16);

    /// drag-and-drop: something is being grabbed
    pub const GRABBING: Self = Self(17);

    /// resizing: the east border is to be moved
    pub const E_RESIZE: Self = Self(18);

    /// resizing: the north border is to be moved
    pub const N_RESIZE: Self = Self(19);

    /// resizing: the north-east corner is to be moved
    pub const NE_RESIZE: Self = Self(20);

    /// resizing: the north-west corner is to be moved
    pub const NW_RESIZE: Self = Self(21);

    /// resizing: the south border is to be moved
    pub const S_RESIZE: Self = Self(22);

    /// resizing: the south-east corner is to be moved
    pub const SE_RESIZE: Self = Self(23);

    /// resizing: the south-west corner is to be moved
    pub const SW_RESIZE: Self = Self(24);

    /// resizing: the west border is to be moved
    pub const W_RESIZE: Self = Self(25);

    /// resizing: the east and west borders are to be moved
    pub const EW_RESIZE: Self = Self(26);

    /// resizing: the north and south borders are to be moved
    pub const NS_RESIZE: Self = Self(27);

    /// resizing: the north-east and south-west corners are to be moved
    pub const NESW_RESIZE: Self = Self(28);

    /// resizing: the north-west and south-east corners are to be moved
    pub const NWSE_RESIZE: Self = Self(29);

    /// resizing: that the item/column can be resized horizontally
    pub const COL_RESIZE: Self = Self(30);

    /// resizing: that the item/row can be resized vertically
    pub const ROW_RESIZE: Self = Self(31);

    /// something can be scrolled in any direction
    pub const ALL_SCROLL: Self = Self(32);

    /// something can be zoomed in
    pub const ZOOM_IN: Self = Self(33);

    /// something can be zoomed out
    pub const ZOOM_OUT: Self = Self(34);

    /// drag-and-drop: the user will select which action will be carried out (non-css value)
    pub const DND_ASK: Self = Self(35);

    /// resizing: something can be moved or resized in any direction (non-css value)
    pub const ALL_RESIZE: Self = Self(36);
}

impl Debug for WpCursorShapeDeviceV1Shape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DEFAULT => "DEFAULT",
            Self::CONTEXT_MENU => "CONTEXT_MENU",
            Self::HELP => "HELP",
            Self::POINTER => "POINTER",
            Self::PROGRESS => "PROGRESS",
            Self::WAIT => "WAIT",
            Self::CELL => "CELL",
            Self::CROSSHAIR => "CROSSHAIR",
            Self::TEXT => "TEXT",
            Self::VERTICAL_TEXT => "VERTICAL_TEXT",
            Self::ALIAS => "ALIAS",
            Self::COPY => "COPY",
            Self::MOVE => "MOVE",
            Self::NO_DROP => "NO_DROP",
            Self::NOT_ALLOWED => "NOT_ALLOWED",
            Self::GRAB => "GRAB",
            Self::GRABBING => "GRABBING",
            Self::E_RESIZE => "E_RESIZE",
            Self::N_RESIZE => "N_RESIZE",
            Self::NE_RESIZE => "NE_RESIZE",
            Self::NW_RESIZE => "NW_RESIZE",
            Self::S_RESIZE => "S_RESIZE",
            Self::SE_RESIZE => "SE_RESIZE",
            Self::SW_RESIZE => "SW_RESIZE",
            Self::W_RESIZE => "W_RESIZE",
            Self::EW_RESIZE => "EW_RESIZE",
            Self::NS_RESIZE => "NS_RESIZE",
            Self::NESW_RESIZE => "NESW_RESIZE",
            Self::NWSE_RESIZE => "NWSE_RESIZE",
            Self::COL_RESIZE => "COL_RESIZE",
            Self::ROW_RESIZE => "ROW_RESIZE",
            Self::ALL_SCROLL => "ALL_SCROLL",
            Self::ZOOM_IN => "ZOOM_IN",
            Self::ZOOM_OUT => "ZOOM_OUT",
            Self::DND_ASK => "DND_ASK",
            Self::ALL_RESIZE => "ALL_RESIZE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpCursorShapeDeviceV1Error(pub u32);

impl WpCursorShapeDeviceV1Error {
    /// the specified shape value is invalid
    pub const INVALID_SHAPE: Self = Self(1);
}

impl Debug for WpCursorShapeDeviceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_SHAPE => "INVALID_SHAPE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
