//! cursor shape for a device
//!
//! This interface allows clients to set the cursor shape.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_cursor_shape_device_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpCursorShapeDeviceV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpCursorShapeDeviceV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpCursorShapeDeviceV1MessageHandler for DefaultMessageHandler { }

impl MetaWpCursorShapeDeviceV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpCursorShapeDeviceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpCursorShapeDeviceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpCursorShapeDeviceV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the cursor shape device
    ///
    /// Destroy the cursor shape device.
    ///
    /// The device cursor shape remains unchanged.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
        ]);
        Ok(())
    }

    /// Since when the set_shape message is available.
    #[allow(dead_code)]
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
    pub fn send_set_shape(
        &self,
        serial: u32,
        shape: MetaWpCursorShapeDeviceV1Shape,
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
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0,
            arg1.0,
        ]);
        Ok(())
    }
}

/// A message handler for [WpCursorShapeDeviceV1] proxies.
#[allow(dead_code)]
pub trait MetaWpCursorShapeDeviceV1MessageHandler {
    /// destroy the cursor shape device
    ///
    /// Destroy the cursor shape device.
    ///
    /// The device cursor shape remains unchanged.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpCursorShapeDeviceV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_cursor_shape_device_v1.destroy message: {}", Report::new(e));
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
    fn set_shape(
        &mut self,
        _slf: &Rc<MetaWpCursorShapeDeviceV1>,
        serial: u32,
        shape: MetaWpCursorShapeDeviceV1Shape,
    ) {
        let res = _slf.send_set_shape(
            serial,
            shape,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_cursor_shape_device_v1.set_shape message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpCursorShapeDeviceV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg1 = MetaWpCursorShapeDeviceV1Shape(arg1);
                if let Some(handler) = handler {
                    (**handler).set_shape(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.set_shape(&self, arg0, arg1);
                }
            }
            _ => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
    }
}

impl MetaWpCursorShapeDeviceV1 {
    /// Since when the shape.default enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_DEFAULT__SINCE: u32 = 1;
    /// Since when the shape.context_menu enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_CONTEXT_MENU__SINCE: u32 = 1;
    /// Since when the shape.help enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_HELP__SINCE: u32 = 1;
    /// Since when the shape.pointer enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_POINTER__SINCE: u32 = 1;
    /// Since when the shape.progress enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_PROGRESS__SINCE: u32 = 1;
    /// Since when the shape.wait enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_WAIT__SINCE: u32 = 1;
    /// Since when the shape.cell enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_CELL__SINCE: u32 = 1;
    /// Since when the shape.crosshair enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_CROSSHAIR__SINCE: u32 = 1;
    /// Since when the shape.text enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_TEXT__SINCE: u32 = 1;
    /// Since when the shape.vertical_text enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_VERTICAL_TEXT__SINCE: u32 = 1;
    /// Since when the shape.alias enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_ALIAS__SINCE: u32 = 1;
    /// Since when the shape.copy enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_COPY__SINCE: u32 = 1;
    /// Since when the shape.move enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_MOVE__SINCE: u32 = 1;
    /// Since when the shape.no_drop enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_NO_DROP__SINCE: u32 = 1;
    /// Since when the shape.not_allowed enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_NOT_ALLOWED__SINCE: u32 = 1;
    /// Since when the shape.grab enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_GRAB__SINCE: u32 = 1;
    /// Since when the shape.grabbing enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_GRABBING__SINCE: u32 = 1;
    /// Since when the shape.e_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_E_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.n_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_N_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.ne_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_NE_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.nw_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_NW_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.s_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_S_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.se_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_SE_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.sw_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_SW_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.w_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_W_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.ew_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_EW_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.ns_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_NS_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.nesw_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_NESW_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.nwse_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_NWSE_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.col_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_COL_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.row_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_ROW_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.all_scroll enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_ALL_SCROLL__SINCE: u32 = 1;
    /// Since when the shape.zoom_in enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_ZOOM_IN__SINCE: u32 = 1;
    /// Since when the shape.zoom_out enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_ZOOM_OUT__SINCE: u32 = 1;
    /// Since when the shape.dnd_ask enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_DND_ASK__SINCE: u32 = 2;
    /// Since when the shape.all_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_ALL_RESIZE__SINCE: u32 = 2;

    /// Since when the error.invalid_shape enum variant is available.
    #[allow(dead_code)]
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
#[allow(dead_code)]
pub struct MetaWpCursorShapeDeviceV1Shape(pub u32);

impl MetaWpCursorShapeDeviceV1Shape {
    /// default cursor
    #[allow(dead_code)]
    pub const DEFAULT: Self = Self(1);

    /// a context menu is available for the object under the cursor
    #[allow(dead_code)]
    pub const CONTEXT_MENU: Self = Self(2);

    /// help is available for the object under the cursor
    #[allow(dead_code)]
    pub const HELP: Self = Self(3);

    /// pointer that indicates a link or another interactive element
    #[allow(dead_code)]
    pub const POINTER: Self = Self(4);

    /// progress indicator
    #[allow(dead_code)]
    pub const PROGRESS: Self = Self(5);

    /// program is busy, user should wait
    #[allow(dead_code)]
    pub const WAIT: Self = Self(6);

    /// a cell or set of cells may be selected
    #[allow(dead_code)]
    pub const CELL: Self = Self(7);

    /// simple crosshair
    #[allow(dead_code)]
    pub const CROSSHAIR: Self = Self(8);

    /// text may be selected
    #[allow(dead_code)]
    pub const TEXT: Self = Self(9);

    /// vertical text may be selected
    #[allow(dead_code)]
    pub const VERTICAL_TEXT: Self = Self(10);

    /// drag-and-drop: alias of/shortcut to something is to be created
    #[allow(dead_code)]
    pub const ALIAS: Self = Self(11);

    /// drag-and-drop: something is to be copied
    #[allow(dead_code)]
    pub const COPY: Self = Self(12);

    /// drag-and-drop: something is to be moved
    #[allow(dead_code)]
    pub const MOVE: Self = Self(13);

    /// drag-and-drop: the dragged item cannot be dropped at the current cursor location
    #[allow(dead_code)]
    pub const NO_DROP: Self = Self(14);

    /// drag-and-drop: the requested action will not be carried out
    #[allow(dead_code)]
    pub const NOT_ALLOWED: Self = Self(15);

    /// drag-and-drop: something can be grabbed
    #[allow(dead_code)]
    pub const GRAB: Self = Self(16);

    /// drag-and-drop: something is being grabbed
    #[allow(dead_code)]
    pub const GRABBING: Self = Self(17);

    /// resizing: the east border is to be moved
    #[allow(dead_code)]
    pub const E_RESIZE: Self = Self(18);

    /// resizing: the north border is to be moved
    #[allow(dead_code)]
    pub const N_RESIZE: Self = Self(19);

    /// resizing: the north-east corner is to be moved
    #[allow(dead_code)]
    pub const NE_RESIZE: Self = Self(20);

    /// resizing: the north-west corner is to be moved
    #[allow(dead_code)]
    pub const NW_RESIZE: Self = Self(21);

    /// resizing: the south border is to be moved
    #[allow(dead_code)]
    pub const S_RESIZE: Self = Self(22);

    /// resizing: the south-east corner is to be moved
    #[allow(dead_code)]
    pub const SE_RESIZE: Self = Self(23);

    /// resizing: the south-west corner is to be moved
    #[allow(dead_code)]
    pub const SW_RESIZE: Self = Self(24);

    /// resizing: the west border is to be moved
    #[allow(dead_code)]
    pub const W_RESIZE: Self = Self(25);

    /// resizing: the east and west borders are to be moved
    #[allow(dead_code)]
    pub const EW_RESIZE: Self = Self(26);

    /// resizing: the north and south borders are to be moved
    #[allow(dead_code)]
    pub const NS_RESIZE: Self = Self(27);

    /// resizing: the north-east and south-west corners are to be moved
    #[allow(dead_code)]
    pub const NESW_RESIZE: Self = Self(28);

    /// resizing: the north-west and south-east corners are to be moved
    #[allow(dead_code)]
    pub const NWSE_RESIZE: Self = Self(29);

    /// resizing: that the item/column can be resized horizontally
    #[allow(dead_code)]
    pub const COL_RESIZE: Self = Self(30);

    /// resizing: that the item/row can be resized vertically
    #[allow(dead_code)]
    pub const ROW_RESIZE: Self = Self(31);

    /// something can be scrolled in any direction
    #[allow(dead_code)]
    pub const ALL_SCROLL: Self = Self(32);

    /// something can be zoomed in
    #[allow(dead_code)]
    pub const ZOOM_IN: Self = Self(33);

    /// something can be zoomed out
    #[allow(dead_code)]
    pub const ZOOM_OUT: Self = Self(34);

    /// drag-and-drop: the user will select which action will be carried out (non-css value)
    #[allow(dead_code)]
    pub const DND_ASK: Self = Self(35);

    /// resizing: something can be moved or resized in any direction (non-css value)
    #[allow(dead_code)]
    pub const ALL_RESIZE: Self = Self(36);
}

impl Debug for MetaWpCursorShapeDeviceV1Shape {
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
#[allow(dead_code)]
pub struct MetaWpCursorShapeDeviceV1Error(pub u32);

impl MetaWpCursorShapeDeviceV1Error {
    /// the specified shape value is invalid
    #[allow(dead_code)]
    pub const INVALID_SHAPE: Self = Self(1);
}

impl Debug for MetaWpCursorShapeDeviceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_SHAPE => "INVALID_SHAPE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
