//! layer metadata interface
//!
//! An interface that may be implemented by a wl_surface, for surfaces that
//! are designed to be rendered as a layer of a stacked desktop-like
//! environment.
//!
//! Layer surface state (layer, size, anchor, exclusive zone,
//! margin, interactivity) is double-buffered, and will be applied at the
//! time wl_surface.commit of the corresponding wl_surface is called.
//!
//! Attaching a null buffer to a layer surface unmaps it.
//!
//! Unmapping a layer_surface means that the surface cannot be shown by the
//! compositor until it is explicitly mapped again. The layer_surface
//! returns to the state it had right after layer_shell.get_layer_surface.
//! The client can re-map the surface by performing a commit without any
//! buffer attached, waiting for a configure event and handling it as usual.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwlr_layer_surface_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwlrLayerSurfaceV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwlrLayerSurfaceV1Handler>,
}

struct DefaultHandler;

impl ZwlrLayerSurfaceV1Handler for DefaultHandler { }

impl ConcreteObject for ZwlrLayerSurfaceV1 {
    const XML_VERSION: u32 = 5;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwlrLayerSurfaceV1;
    const INTERFACE_NAME: &str = "zwlr_layer_surface_v1";
}

impl ZwlrLayerSurfaceV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwlrLayerSurfaceV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwlrLayerSurfaceV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwlrLayerSurfaceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwlrLayerSurfaceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwlrLayerSurfaceV1 {
    /// Since when the set_size message is available.
    pub const MSG__SET_SIZE__SINCE: u32 = 1;

    /// sets the size of the surface
    ///
    /// Sets the size of the surface in surface-local coordinates. The
    /// compositor will display the surface centered with respect to its
    /// anchors.
    ///
    /// If you pass 0 for either value, the compositor will assign it and
    /// inform you of the assignment in the configure event. You must set your
    /// anchor to opposite edges in the dimensions you omit; not doing so is a
    /// protocol error. Both values are 0 by default.
    ///
    /// Size is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn try_send_set_size(
        &self,
        width: u32,
        height: u32,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_layer_surface_v1#{}.set_size(width: {}, height: {})\n", id, arg0, arg1);
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
            0,
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// sets the size of the surface
    ///
    /// Sets the size of the surface in surface-local coordinates. The
    /// compositor will display the surface centered with respect to its
    /// anchors.
    ///
    /// If you pass 0 for either value, the compositor will assign it and
    /// inform you of the assignment in the configure event. You must set your
    /// anchor to opposite edges in the dimensions you omit; not doing so is a
    /// protocol error. Both values are 0 by default.
    ///
    /// Size is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn send_set_size(
        &self,
        width: u32,
        height: u32,
    ) {
        let res = self.try_send_set_size(
            width,
            height,
        );
        if let Err(e) = res {
            log_send("zwlr_layer_surface_v1.set_size", &e);
        }
    }

    /// Since when the set_anchor message is available.
    pub const MSG__SET_ANCHOR__SINCE: u32 = 1;

    /// configures the anchor point of the surface
    ///
    /// Requests that the compositor anchor the surface to the specified edges
    /// and corners. If two orthogonal edges are specified (e.g. 'top' and
    /// 'left'), then the anchor point will be the intersection of the edges
    /// (e.g. the top left corner of the output); otherwise the anchor point
    /// will be centered on that edge, or in the center if none is specified.
    ///
    /// Anchor is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `anchor`:
    #[inline]
    pub fn try_send_set_anchor(
        &self,
        anchor: ZwlrLayerSurfaceV1Anchor,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            anchor,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: ZwlrLayerSurfaceV1Anchor) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_layer_surface_v1#{}.set_anchor(anchor: {:?})\n", id, arg0);
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
            1,
            arg0.0,
        ]);
        Ok(())
    }

    /// configures the anchor point of the surface
    ///
    /// Requests that the compositor anchor the surface to the specified edges
    /// and corners. If two orthogonal edges are specified (e.g. 'top' and
    /// 'left'), then the anchor point will be the intersection of the edges
    /// (e.g. the top left corner of the output); otherwise the anchor point
    /// will be centered on that edge, or in the center if none is specified.
    ///
    /// Anchor is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `anchor`:
    #[inline]
    pub fn send_set_anchor(
        &self,
        anchor: ZwlrLayerSurfaceV1Anchor,
    ) {
        let res = self.try_send_set_anchor(
            anchor,
        );
        if let Err(e) = res {
            log_send("zwlr_layer_surface_v1.set_anchor", &e);
        }
    }

    /// Since when the set_exclusive_zone message is available.
    pub const MSG__SET_EXCLUSIVE_ZONE__SINCE: u32 = 1;

    /// configures the exclusive geometry of this surface
    ///
    /// Requests that the compositor avoids occluding an area with other
    /// surfaces. The compositor's use of this information is
    /// implementation-dependent - do not assume that this region will not
    /// actually be occluded.
    ///
    /// A positive value is only meaningful if the surface is anchored to one
    /// edge or an edge and both perpendicular edges. If the surface is not
    /// anchored, anchored to only two perpendicular edges (a corner), anchored
    /// to only two parallel edges or anchored to all edges, a positive value
    /// will be treated the same as zero.
    ///
    /// A positive zone is the distance from the edge in surface-local
    /// coordinates to consider exclusive.
    ///
    /// Surfaces that do not wish to have an exclusive zone may instead specify
    /// how they should interact with surfaces that do. If set to zero, the
    /// surface indicates that it would like to be moved to avoid occluding
    /// surfaces with a positive exclusive zone. If set to -1, the surface
    /// indicates that it would not like to be moved to accommodate for other
    /// surfaces, and the compositor should extend it all the way to the edges
    /// it is anchored to.
    ///
    /// For example, a panel might set its exclusive zone to 10, so that
    /// maximized shell surfaces are not shown on top of it. A notification
    /// might set its exclusive zone to 0, so that it is moved to avoid
    /// occluding the panel, but shell surfaces are shown underneath it. A
    /// wallpaper or lock screen might set their exclusive zone to -1, so that
    /// they stretch below or over the panel.
    ///
    /// The default value is 0.
    ///
    /// Exclusive zone is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `zone`:
    #[inline]
    pub fn try_send_set_exclusive_zone(
        &self,
        zone: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            zone,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_layer_surface_v1#{}.set_exclusive_zone(zone: {})\n", id, arg0);
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
            2,
            arg0 as u32,
        ]);
        Ok(())
    }

    /// configures the exclusive geometry of this surface
    ///
    /// Requests that the compositor avoids occluding an area with other
    /// surfaces. The compositor's use of this information is
    /// implementation-dependent - do not assume that this region will not
    /// actually be occluded.
    ///
    /// A positive value is only meaningful if the surface is anchored to one
    /// edge or an edge and both perpendicular edges. If the surface is not
    /// anchored, anchored to only two perpendicular edges (a corner), anchored
    /// to only two parallel edges or anchored to all edges, a positive value
    /// will be treated the same as zero.
    ///
    /// A positive zone is the distance from the edge in surface-local
    /// coordinates to consider exclusive.
    ///
    /// Surfaces that do not wish to have an exclusive zone may instead specify
    /// how they should interact with surfaces that do. If set to zero, the
    /// surface indicates that it would like to be moved to avoid occluding
    /// surfaces with a positive exclusive zone. If set to -1, the surface
    /// indicates that it would not like to be moved to accommodate for other
    /// surfaces, and the compositor should extend it all the way to the edges
    /// it is anchored to.
    ///
    /// For example, a panel might set its exclusive zone to 10, so that
    /// maximized shell surfaces are not shown on top of it. A notification
    /// might set its exclusive zone to 0, so that it is moved to avoid
    /// occluding the panel, but shell surfaces are shown underneath it. A
    /// wallpaper or lock screen might set their exclusive zone to -1, so that
    /// they stretch below or over the panel.
    ///
    /// The default value is 0.
    ///
    /// Exclusive zone is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `zone`:
    #[inline]
    pub fn send_set_exclusive_zone(
        &self,
        zone: i32,
    ) {
        let res = self.try_send_set_exclusive_zone(
            zone,
        );
        if let Err(e) = res {
            log_send("zwlr_layer_surface_v1.set_exclusive_zone", &e);
        }
    }

    /// Since when the set_margin message is available.
    pub const MSG__SET_MARGIN__SINCE: u32 = 1;

    /// sets a margin from the anchor point
    ///
    /// Requests that the surface be placed some distance away from the anchor
    /// point on the output, in surface-local coordinates. Setting this value
    /// for edges you are not anchored to has no effect.
    ///
    /// The exclusive zone includes the margin.
    ///
    /// Margin is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `top`:
    /// - `right`:
    /// - `bottom`:
    /// - `left`:
    #[inline]
    pub fn try_send_set_margin(
        &self,
        top: i32,
        right: i32,
        bottom: i32,
        left: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            top,
            right,
            bottom,
            left,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_layer_surface_v1#{}.set_margin(top: {}, right: {}, bottom: {}, left: {})\n", id, arg0, arg1, arg2, arg3);
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

    /// sets a margin from the anchor point
    ///
    /// Requests that the surface be placed some distance away from the anchor
    /// point on the output, in surface-local coordinates. Setting this value
    /// for edges you are not anchored to has no effect.
    ///
    /// The exclusive zone includes the margin.
    ///
    /// Margin is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `top`:
    /// - `right`:
    /// - `bottom`:
    /// - `left`:
    #[inline]
    pub fn send_set_margin(
        &self,
        top: i32,
        right: i32,
        bottom: i32,
        left: i32,
    ) {
        let res = self.try_send_set_margin(
            top,
            right,
            bottom,
            left,
        );
        if let Err(e) = res {
            log_send("zwlr_layer_surface_v1.set_margin", &e);
        }
    }

    /// Since when the set_keyboard_interactivity message is available.
    pub const MSG__SET_KEYBOARD_INTERACTIVITY__SINCE: u32 = 1;

    /// requests keyboard events
    ///
    /// Set how keyboard events are delivered to this surface. By default,
    /// layer shell surfaces do not receive keyboard events; this request can
    /// be used to change this.
    ///
    /// This setting is inherited by child surfaces set by the get_popup
    /// request.
    ///
    /// Layer surfaces receive pointer, touch, and tablet events normally. If
    /// you do not want to receive them, set the input region on your surface
    /// to an empty region.
    ///
    /// Keyboard interactivity is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `keyboard_interactivity`:
    #[inline]
    pub fn try_send_set_keyboard_interactivity(
        &self,
        keyboard_interactivity: ZwlrLayerSurfaceV1KeyboardInteractivity,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            keyboard_interactivity,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: ZwlrLayerSurfaceV1KeyboardInteractivity) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_layer_surface_v1#{}.set_keyboard_interactivity(keyboard_interactivity: {:?})\n", id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// requests keyboard events
    ///
    /// Set how keyboard events are delivered to this surface. By default,
    /// layer shell surfaces do not receive keyboard events; this request can
    /// be used to change this.
    ///
    /// This setting is inherited by child surfaces set by the get_popup
    /// request.
    ///
    /// Layer surfaces receive pointer, touch, and tablet events normally. If
    /// you do not want to receive them, set the input region on your surface
    /// to an empty region.
    ///
    /// Keyboard interactivity is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `keyboard_interactivity`:
    #[inline]
    pub fn send_set_keyboard_interactivity(
        &self,
        keyboard_interactivity: ZwlrLayerSurfaceV1KeyboardInteractivity,
    ) {
        let res = self.try_send_set_keyboard_interactivity(
            keyboard_interactivity,
        );
        if let Err(e) = res {
            log_send("zwlr_layer_surface_v1.set_keyboard_interactivity", &e);
        }
    }

    /// Since when the get_popup message is available.
    pub const MSG__GET_POPUP__SINCE: u32 = 1;

    /// assign this layer_surface as an xdg_popup parent
    ///
    /// This assigns an xdg_popup's parent to this layer_surface.  This popup
    /// should have been created via xdg_surface::get_popup with the parent set
    /// to NULL, and this request must be invoked before committing the popup's
    /// initial state.
    ///
    /// See the documentation of xdg_popup for more details about what an
    /// xdg_popup is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `popup`:
    #[inline]
    pub fn try_send_get_popup(
        &self,
        popup: &Rc<XdgPopup>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            popup,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("popup"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_layer_surface_v1#{}.get_popup(popup: xdg_popup#{})\n", id, arg0);
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
            5,
            arg0_id,
        ]);
        Ok(())
    }

    /// assign this layer_surface as an xdg_popup parent
    ///
    /// This assigns an xdg_popup's parent to this layer_surface.  This popup
    /// should have been created via xdg_surface::get_popup with the parent set
    /// to NULL, and this request must be invoked before committing the popup's
    /// initial state.
    ///
    /// See the documentation of xdg_popup for more details about what an
    /// xdg_popup is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `popup`:
    #[inline]
    pub fn send_get_popup(
        &self,
        popup: &Rc<XdgPopup>,
    ) {
        let res = self.try_send_get_popup(
            popup,
        );
        if let Err(e) = res {
            log_send("zwlr_layer_surface_v1.get_popup", &e);
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
    /// If the client receives multiple configure events before it
    /// can respond to one, it only has to ack the last configure event.
    ///
    /// A client is not required to commit immediately after sending
    /// an ack_configure request - it may even ack_configure several times
    /// before its next surface commit.
    ///
    /// A client may send multiple ack_configure requests before committing, but
    /// only the last request sent before a commit indicates which configure
    /// event the client really is responding to.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_layer_surface_v1#{}.ack_configure(serial: {})\n", id, arg0);
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

    /// ack a configure event
    ///
    /// When a configure event is received, if a client commits the
    /// surface in response to the configure event, then the client
    /// must make an ack_configure request sometime before the commit
    /// request, passing along the serial of the configure event.
    ///
    /// If the client receives multiple configure events before it
    /// can respond to one, it only has to ack the last configure event.
    ///
    /// A client is not required to commit immediately after sending
    /// an ack_configure request - it may even ack_configure several times
    /// before its next surface commit.
    ///
    /// A client may send multiple ack_configure requests before committing, but
    /// only the last request sent before a commit indicates which configure
    /// event the client really is responding to.
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
            log_send("zwlr_layer_surface_v1.ack_configure", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the layer_surface
    ///
    /// This request destroys the layer surface.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_layer_surface_v1#{}.destroy()\n", id);
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
            7,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the layer_surface
    ///
    /// This request destroys the layer surface.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwlr_layer_surface_v1.destroy", &e);
        }
    }

    /// Since when the configure message is available.
    pub const MSG__CONFIGURE__SINCE: u32 = 1;

    /// suggest a surface change
    ///
    /// The configure event asks the client to resize its surface.
    ///
    /// Clients should arrange their surface for the new states, and then send
    /// an ack_configure request with the serial sent in this configure event at
    /// some point before committing the new surface.
    ///
    /// The client is free to dismiss all but the last configure event it
    /// received.
    ///
    /// The width and height arguments specify the size of the window in
    /// surface-local coordinates.
    ///
    /// The size is a hint, in the sense that the client is free to ignore it if
    /// it doesn't resize, pick a smaller size (to satisfy aspect ratio or
    /// resize in steps of NxM pixels). If the client picks a smaller size and
    /// is anchored to two opposite anchors (e.g. 'top' and 'bottom'), the
    /// surface will be centered on this axis.
    ///
    /// If the width or height arguments are zero, it means the client should
    /// decide its own window dimension.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn try_send_configure(
        &self,
        serial: u32,
        width: u32,
        height: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            serial,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_layer_surface_v1#{}.configure(serial: {}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2);
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
            0,
            arg0,
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// suggest a surface change
    ///
    /// The configure event asks the client to resize its surface.
    ///
    /// Clients should arrange their surface for the new states, and then send
    /// an ack_configure request with the serial sent in this configure event at
    /// some point before committing the new surface.
    ///
    /// The client is free to dismiss all but the last configure event it
    /// received.
    ///
    /// The width and height arguments specify the size of the window in
    /// surface-local coordinates.
    ///
    /// The size is a hint, in the sense that the client is free to ignore it if
    /// it doesn't resize, pick a smaller size (to satisfy aspect ratio or
    /// resize in steps of NxM pixels). If the client picks a smaller size and
    /// is anchored to two opposite anchors (e.g. 'top' and 'bottom'), the
    /// surface will be centered on this axis.
    ///
    /// If the width or height arguments are zero, it means the client should
    /// decide its own window dimension.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn send_configure(
        &self,
        serial: u32,
        width: u32,
        height: u32,
    ) {
        let res = self.try_send_configure(
            serial,
            width,
            height,
        );
        if let Err(e) = res {
            log_send("zwlr_layer_surface_v1.configure", &e);
        }
    }

    /// Since when the closed message is available.
    pub const MSG__CLOSED__SINCE: u32 = 1;

    /// surface should be closed
    ///
    /// The closed event is sent by the compositor when the surface will no
    /// longer be shown. The output may have been destroyed or the user may
    /// have asked for it to be removed. Further changes to the surface will be
    /// ignored. The client should destroy the resource after receiving this
    /// event, and create a new surface if they so choose.
    #[inline]
    pub fn try_send_closed(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_layer_surface_v1#{}.closed()\n", client_id, id);
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

    /// surface should be closed
    ///
    /// The closed event is sent by the compositor when the surface will no
    /// longer be shown. The output may have been destroyed or the user may
    /// have asked for it to be removed. Further changes to the surface will be
    /// ignored. The client should destroy the resource after receiving this
    /// event, and create a new surface if they so choose.
    #[inline]
    pub fn send_closed(
        &self,
    ) {
        let res = self.try_send_closed(
        );
        if let Err(e) = res {
            log_send("zwlr_layer_surface_v1.closed", &e);
        }
    }

    /// Since when the set_layer message is available.
    pub const MSG__SET_LAYER__SINCE: u32 = 2;

    /// change the layer of the surface
    ///
    /// Change the layer that the surface is rendered on.
    ///
    /// Layer is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `layer`: layer to move this surface to
    #[inline]
    pub fn try_send_set_layer(
        &self,
        layer: ZwlrLayerShellV1Layer,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            layer,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: ZwlrLayerShellV1Layer) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_layer_surface_v1#{}.set_layer(layer: {:?})\n", id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// change the layer of the surface
    ///
    /// Change the layer that the surface is rendered on.
    ///
    /// Layer is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `layer`: layer to move this surface to
    #[inline]
    pub fn send_set_layer(
        &self,
        layer: ZwlrLayerShellV1Layer,
    ) {
        let res = self.try_send_set_layer(
            layer,
        );
        if let Err(e) = res {
            log_send("zwlr_layer_surface_v1.set_layer", &e);
        }
    }

    /// Since when the set_exclusive_edge message is available.
    pub const MSG__SET_EXCLUSIVE_EDGE__SINCE: u32 = 5;

    /// set the edge the exclusive zone will be applied to
    ///
    /// Requests an edge for the exclusive zone to apply. The exclusive
    /// edge will be automatically deduced from anchor points when possible,
    /// but when the surface is anchored to a corner, it will be necessary
    /// to set it explicitly to disambiguate, as it is not possible to deduce
    /// which one of the two corner edges should be used.
    ///
    /// The edge must be one the surface is anchored to, otherwise the
    /// invalid_exclusive_edge protocol error will be raised.
    ///
    /// # Arguments
    ///
    /// - `edge`:
    #[inline]
    pub fn try_send_set_exclusive_edge(
        &self,
        edge: ZwlrLayerSurfaceV1Anchor,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            edge,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: ZwlrLayerSurfaceV1Anchor) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_layer_surface_v1#{}.set_exclusive_edge(edge: {:?})\n", id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// set the edge the exclusive zone will be applied to
    ///
    /// Requests an edge for the exclusive zone to apply. The exclusive
    /// edge will be automatically deduced from anchor points when possible,
    /// but when the surface is anchored to a corner, it will be necessary
    /// to set it explicitly to disambiguate, as it is not possible to deduce
    /// which one of the two corner edges should be used.
    ///
    /// The edge must be one the surface is anchored to, otherwise the
    /// invalid_exclusive_edge protocol error will be raised.
    ///
    /// # Arguments
    ///
    /// - `edge`:
    #[inline]
    pub fn send_set_exclusive_edge(
        &self,
        edge: ZwlrLayerSurfaceV1Anchor,
    ) {
        let res = self.try_send_set_exclusive_edge(
            edge,
        );
        if let Err(e) = res {
            log_send("zwlr_layer_surface_v1.set_exclusive_edge", &e);
        }
    }
}

/// A message handler for [`ZwlrLayerSurfaceV1`] proxies.
pub trait ZwlrLayerSurfaceV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwlrLayerSurfaceV1>) {
        slf.core.delete_id();
    }

    /// sets the size of the surface
    ///
    /// Sets the size of the surface in surface-local coordinates. The
    /// compositor will display the surface centered with respect to its
    /// anchors.
    ///
    /// If you pass 0 for either value, the compositor will assign it and
    /// inform you of the assignment in the configure event. You must set your
    /// anchor to opposite edges in the dimensions you omit; not doing so is a
    /// protocol error. Both values are 0 by default.
    ///
    /// Size is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    #[inline]
    fn handle_set_size(
        &mut self,
        slf: &Rc<ZwlrLayerSurfaceV1>,
        width: u32,
        height: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_size(
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("zwlr_layer_surface_v1.set_size", &e);
        }
    }

    /// configures the anchor point of the surface
    ///
    /// Requests that the compositor anchor the surface to the specified edges
    /// and corners. If two orthogonal edges are specified (e.g. 'top' and
    /// 'left'), then the anchor point will be the intersection of the edges
    /// (e.g. the top left corner of the output); otherwise the anchor point
    /// will be centered on that edge, or in the center if none is specified.
    ///
    /// Anchor is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `anchor`:
    #[inline]
    fn handle_set_anchor(
        &mut self,
        slf: &Rc<ZwlrLayerSurfaceV1>,
        anchor: ZwlrLayerSurfaceV1Anchor,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_anchor(
            anchor,
        );
        if let Err(e) = res {
            log_forward("zwlr_layer_surface_v1.set_anchor", &e);
        }
    }

    /// configures the exclusive geometry of this surface
    ///
    /// Requests that the compositor avoids occluding an area with other
    /// surfaces. The compositor's use of this information is
    /// implementation-dependent - do not assume that this region will not
    /// actually be occluded.
    ///
    /// A positive value is only meaningful if the surface is anchored to one
    /// edge or an edge and both perpendicular edges. If the surface is not
    /// anchored, anchored to only two perpendicular edges (a corner), anchored
    /// to only two parallel edges or anchored to all edges, a positive value
    /// will be treated the same as zero.
    ///
    /// A positive zone is the distance from the edge in surface-local
    /// coordinates to consider exclusive.
    ///
    /// Surfaces that do not wish to have an exclusive zone may instead specify
    /// how they should interact with surfaces that do. If set to zero, the
    /// surface indicates that it would like to be moved to avoid occluding
    /// surfaces with a positive exclusive zone. If set to -1, the surface
    /// indicates that it would not like to be moved to accommodate for other
    /// surfaces, and the compositor should extend it all the way to the edges
    /// it is anchored to.
    ///
    /// For example, a panel might set its exclusive zone to 10, so that
    /// maximized shell surfaces are not shown on top of it. A notification
    /// might set its exclusive zone to 0, so that it is moved to avoid
    /// occluding the panel, but shell surfaces are shown underneath it. A
    /// wallpaper or lock screen might set their exclusive zone to -1, so that
    /// they stretch below or over the panel.
    ///
    /// The default value is 0.
    ///
    /// Exclusive zone is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `zone`:
    #[inline]
    fn handle_set_exclusive_zone(
        &mut self,
        slf: &Rc<ZwlrLayerSurfaceV1>,
        zone: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_exclusive_zone(
            zone,
        );
        if let Err(e) = res {
            log_forward("zwlr_layer_surface_v1.set_exclusive_zone", &e);
        }
    }

    /// sets a margin from the anchor point
    ///
    /// Requests that the surface be placed some distance away from the anchor
    /// point on the output, in surface-local coordinates. Setting this value
    /// for edges you are not anchored to has no effect.
    ///
    /// The exclusive zone includes the margin.
    ///
    /// Margin is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `top`:
    /// - `right`:
    /// - `bottom`:
    /// - `left`:
    #[inline]
    fn handle_set_margin(
        &mut self,
        slf: &Rc<ZwlrLayerSurfaceV1>,
        top: i32,
        right: i32,
        bottom: i32,
        left: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_margin(
            top,
            right,
            bottom,
            left,
        );
        if let Err(e) = res {
            log_forward("zwlr_layer_surface_v1.set_margin", &e);
        }
    }

    /// requests keyboard events
    ///
    /// Set how keyboard events are delivered to this surface. By default,
    /// layer shell surfaces do not receive keyboard events; this request can
    /// be used to change this.
    ///
    /// This setting is inherited by child surfaces set by the get_popup
    /// request.
    ///
    /// Layer surfaces receive pointer, touch, and tablet events normally. If
    /// you do not want to receive them, set the input region on your surface
    /// to an empty region.
    ///
    /// Keyboard interactivity is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `keyboard_interactivity`:
    #[inline]
    fn handle_set_keyboard_interactivity(
        &mut self,
        slf: &Rc<ZwlrLayerSurfaceV1>,
        keyboard_interactivity: ZwlrLayerSurfaceV1KeyboardInteractivity,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_keyboard_interactivity(
            keyboard_interactivity,
        );
        if let Err(e) = res {
            log_forward("zwlr_layer_surface_v1.set_keyboard_interactivity", &e);
        }
    }

    /// assign this layer_surface as an xdg_popup parent
    ///
    /// This assigns an xdg_popup's parent to this layer_surface.  This popup
    /// should have been created via xdg_surface::get_popup with the parent set
    /// to NULL, and this request must be invoked before committing the popup's
    /// initial state.
    ///
    /// See the documentation of xdg_popup for more details about what an
    /// xdg_popup is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `popup`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_popup(
        &mut self,
        slf: &Rc<ZwlrLayerSurfaceV1>,
        popup: &Rc<XdgPopup>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_popup(
            popup,
        );
        if let Err(e) = res {
            log_forward("zwlr_layer_surface_v1.get_popup", &e);
        }
    }

    /// ack a configure event
    ///
    /// When a configure event is received, if a client commits the
    /// surface in response to the configure event, then the client
    /// must make an ack_configure request sometime before the commit
    /// request, passing along the serial of the configure event.
    ///
    /// If the client receives multiple configure events before it
    /// can respond to one, it only has to ack the last configure event.
    ///
    /// A client is not required to commit immediately after sending
    /// an ack_configure request - it may even ack_configure several times
    /// before its next surface commit.
    ///
    /// A client may send multiple ack_configure requests before committing, but
    /// only the last request sent before a commit indicates which configure
    /// event the client really is responding to.
    ///
    /// # Arguments
    ///
    /// - `serial`: the serial from the configure event
    #[inline]
    fn handle_ack_configure(
        &mut self,
        slf: &Rc<ZwlrLayerSurfaceV1>,
        serial: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_ack_configure(
            serial,
        );
        if let Err(e) = res {
            log_forward("zwlr_layer_surface_v1.ack_configure", &e);
        }
    }

    /// destroy the layer_surface
    ///
    /// This request destroys the layer surface.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwlrLayerSurfaceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwlr_layer_surface_v1.destroy", &e);
        }
    }

    /// suggest a surface change
    ///
    /// The configure event asks the client to resize its surface.
    ///
    /// Clients should arrange their surface for the new states, and then send
    /// an ack_configure request with the serial sent in this configure event at
    /// some point before committing the new surface.
    ///
    /// The client is free to dismiss all but the last configure event it
    /// received.
    ///
    /// The width and height arguments specify the size of the window in
    /// surface-local coordinates.
    ///
    /// The size is a hint, in the sense that the client is free to ignore it if
    /// it doesn't resize, pick a smaller size (to satisfy aspect ratio or
    /// resize in steps of NxM pixels). If the client picks a smaller size and
    /// is anchored to two opposite anchors (e.g. 'top' and 'bottom'), the
    /// surface will be centered on this axis.
    ///
    /// If the width or height arguments are zero, it means the client should
    /// decide its own window dimension.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    /// - `width`:
    /// - `height`:
    #[inline]
    fn handle_configure(
        &mut self,
        slf: &Rc<ZwlrLayerSurfaceV1>,
        serial: u32,
        width: u32,
        height: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_configure(
            serial,
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("zwlr_layer_surface_v1.configure", &e);
        }
    }

    /// surface should be closed
    ///
    /// The closed event is sent by the compositor when the surface will no
    /// longer be shown. The output may have been destroyed or the user may
    /// have asked for it to be removed. Further changes to the surface will be
    /// ignored. The client should destroy the resource after receiving this
    /// event, and create a new surface if they so choose.
    #[inline]
    fn handle_closed(
        &mut self,
        slf: &Rc<ZwlrLayerSurfaceV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_closed(
        );
        if let Err(e) = res {
            log_forward("zwlr_layer_surface_v1.closed", &e);
        }
    }

    /// change the layer of the surface
    ///
    /// Change the layer that the surface is rendered on.
    ///
    /// Layer is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `layer`: layer to move this surface to
    #[inline]
    fn handle_set_layer(
        &mut self,
        slf: &Rc<ZwlrLayerSurfaceV1>,
        layer: ZwlrLayerShellV1Layer,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_layer(
            layer,
        );
        if let Err(e) = res {
            log_forward("zwlr_layer_surface_v1.set_layer", &e);
        }
    }

    /// set the edge the exclusive zone will be applied to
    ///
    /// Requests an edge for the exclusive zone to apply. The exclusive
    /// edge will be automatically deduced from anchor points when possible,
    /// but when the surface is anchored to a corner, it will be necessary
    /// to set it explicitly to disambiguate, as it is not possible to deduce
    /// which one of the two corner edges should be used.
    ///
    /// The edge must be one the surface is anchored to, otherwise the
    /// invalid_exclusive_edge protocol error will be raised.
    ///
    /// # Arguments
    ///
    /// - `edge`:
    #[inline]
    fn handle_set_exclusive_edge(
        &mut self,
        slf: &Rc<ZwlrLayerSurfaceV1>,
        edge: ZwlrLayerSurfaceV1Anchor,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_exclusive_edge(
            edge,
        );
        if let Err(e) = res {
            log_forward("zwlr_layer_surface_v1.set_exclusive_edge", &e);
        }
    }
}

impl ObjectPrivate for ZwlrLayerSurfaceV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwlrLayerSurfaceV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_layer_surface_v1#{}.set_size(width: {}, height: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_size(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_size(&self, arg0, arg1);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = ZwlrLayerSurfaceV1Anchor(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: ZwlrLayerSurfaceV1Anchor) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_layer_surface_v1#{}.set_anchor(anchor: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_anchor(&self, arg0);
                } else {
                    DefaultHandler.handle_set_anchor(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = arg0 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_layer_surface_v1#{}.set_exclusive_zone(zone: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_exclusive_zone(&self, arg0);
                } else {
                    DefaultHandler.handle_set_exclusive_zone(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_layer_surface_v1#{}.set_margin(top: {}, right: {}, bottom: {}, left: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_margin(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_set_margin(&self, arg0, arg1, arg2, arg3);
                }
            }
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = ZwlrLayerSurfaceV1KeyboardInteractivity(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: ZwlrLayerSurfaceV1KeyboardInteractivity) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_layer_surface_v1#{}.set_keyboard_interactivity(keyboard_interactivity: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_keyboard_interactivity(&self, arg0);
                } else {
                    DefaultHandler.handle_set_keyboard_interactivity(&self, arg0);
                }
            }
            5 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_layer_surface_v1#{}.get_popup(popup: xdg_popup#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<XdgPopup>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("popup", o.core().interface, ObjectInterface::XdgPopup)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_get_popup(&self, arg0);
                } else {
                    DefaultHandler.handle_get_popup(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_layer_surface_v1#{}.ack_configure(serial: {})\n", client_id, id, arg0);
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
            7 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_layer_surface_v1#{}.destroy()\n", client_id, id);
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
            8 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = ZwlrLayerShellV1Layer(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: ZwlrLayerShellV1Layer) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_layer_surface_v1#{}.set_layer(layer: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_layer(&self, arg0);
                } else {
                    DefaultHandler.handle_set_layer(&self, arg0);
                }
            }
            9 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = ZwlrLayerSurfaceV1Anchor(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: ZwlrLayerSurfaceV1Anchor) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_layer_surface_v1#{}.set_exclusive_edge(edge: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_exclusive_edge(&self, arg0);
                } else {
                    DefaultHandler.handle_set_exclusive_edge(&self, arg0);
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_layer_surface_v1#{}.configure(serial: {}, width: {}, height: {})\n", id, arg0, arg1, arg2);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_layer_surface_v1#{}.closed()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_closed(&self);
                } else {
                    DefaultHandler.handle_closed(&self);
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
            0 => "set_size",
            1 => "set_anchor",
            2 => "set_exclusive_zone",
            3 => "set_margin",
            4 => "set_keyboard_interactivity",
            5 => "get_popup",
            6 => "ack_configure",
            7 => "destroy",
            8 => "set_layer",
            9 => "set_exclusive_edge",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "configure",
            1 => "closed",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwlrLayerSurfaceV1 {
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

impl ZwlrLayerSurfaceV1 {
    /// Since when the keyboard_interactivity.none enum variant is available.
    pub const ENM__KEYBOARD_INTERACTIVITY_NONE__SINCE: u32 = 1;
    /// Since when the keyboard_interactivity.exclusive enum variant is available.
    pub const ENM__KEYBOARD_INTERACTIVITY_EXCLUSIVE__SINCE: u32 = 1;
    /// Since when the keyboard_interactivity.on_demand enum variant is available.
    pub const ENM__KEYBOARD_INTERACTIVITY_ON_DEMAND__SINCE: u32 = 4;

    /// Since when the error.invalid_surface_state enum variant is available.
    pub const ENM__ERROR_INVALID_SURFACE_STATE__SINCE: u32 = 1;
    /// Since when the error.invalid_size enum variant is available.
    pub const ENM__ERROR_INVALID_SIZE__SINCE: u32 = 1;
    /// Since when the error.invalid_anchor enum variant is available.
    pub const ENM__ERROR_INVALID_ANCHOR__SINCE: u32 = 1;
    /// Since when the error.invalid_keyboard_interactivity enum variant is available.
    pub const ENM__ERROR_INVALID_KEYBOARD_INTERACTIVITY__SINCE: u32 = 1;
    /// Since when the error.invalid_exclusive_edge enum variant is available.
    pub const ENM__ERROR_INVALID_EXCLUSIVE_EDGE__SINCE: u32 = 1;

    /// Since when the anchor.top enum variant is available.
    pub const ENM__ANCHOR_TOP__SINCE: u32 = 1;
    /// Since when the anchor.bottom enum variant is available.
    pub const ENM__ANCHOR_BOTTOM__SINCE: u32 = 1;
    /// Since when the anchor.left enum variant is available.
    pub const ENM__ANCHOR_LEFT__SINCE: u32 = 1;
    /// Since when the anchor.right enum variant is available.
    pub const ENM__ANCHOR_RIGHT__SINCE: u32 = 1;
}

/// types of keyboard interaction possible for a layer shell surface
///
/// Types of keyboard interaction possible for layer shell surfaces. The
/// rationale for this is twofold: (1) some applications are not interested
/// in keyboard events and not allowing them to be focused can improve the
/// desktop experience; (2) some applications will want to take exclusive
/// keyboard focus.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwlrLayerSurfaceV1KeyboardInteractivity(pub u32);

impl ZwlrLayerSurfaceV1KeyboardInteractivity {
    /// no keyboard focus is possible
    ///
    /// This value indicates that this surface is not interested in keyboard
    /// events and the compositor should never assign it the keyboard focus.
    ///
    /// This is the default value, set for newly created layer shell surfaces.
    ///
    /// This is useful for e.g. desktop widgets that display information or
    /// only have interaction with non-keyboard input devices.
    pub const NONE: Self = Self(0);

    /// request exclusive keyboard focus
    ///
    /// Request exclusive keyboard focus if this surface is above the shell surface layer.
    ///
    /// For the top and overlay layers, the seat will always give
    /// exclusive keyboard focus to the top-most layer which has keyboard
    /// interactivity set to exclusive. If this layer contains multiple
    /// surfaces with keyboard interactivity set to exclusive, the compositor
    /// determines the one receiving keyboard events in an implementation-
    /// defined manner. In this case, no guarantee is made when this surface
    /// will receive keyboard focus (if ever).
    ///
    /// For the bottom and background layers, the compositor is allowed to use
    /// normal focus semantics.
    ///
    /// This setting is mainly intended for applications that need to ensure
    /// they receive all keyboard events, such as a lock screen or a password
    /// prompt.
    pub const EXCLUSIVE: Self = Self(1);

    /// request regular keyboard focus semantics
    ///
    /// This requests the compositor to allow this surface to be focused and
    /// unfocused by the user in an implementation-defined manner. The user
    /// should be able to unfocus this surface even regardless of the layer
    /// it is on.
    ///
    /// Typically, the compositor will want to use its normal mechanism to
    /// manage keyboard focus between layer shell surfaces with this setting
    /// and regular toplevels on the desktop layer (e.g. click to focus).
    /// Nevertheless, it is possible for a compositor to require a special
    /// interaction to focus or unfocus layer shell surfaces (e.g. requiring
    /// a click even if focus follows the mouse normally, or providing a
    /// keybinding to switch focus between layers).
    ///
    /// This setting is mainly intended for desktop shell components (e.g.
    /// panels) that allow keyboard interaction. Using this option can allow
    /// implementing a desktop shell that can be fully usable without the
    /// mouse.
    pub const ON_DEMAND: Self = Self(2);
}

impl Debug for ZwlrLayerSurfaceV1KeyboardInteractivity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NONE => "NONE",
            Self::EXCLUSIVE => "EXCLUSIVE",
            Self::ON_DEMAND => "ON_DEMAND",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwlrLayerSurfaceV1Error(pub u32);

impl ZwlrLayerSurfaceV1Error {
    /// provided surface state is invalid
    pub const INVALID_SURFACE_STATE: Self = Self(0);

    /// size is invalid
    pub const INVALID_SIZE: Self = Self(1);

    /// anchor bitfield is invalid
    pub const INVALID_ANCHOR: Self = Self(2);

    /// keyboard interactivity is invalid
    pub const INVALID_KEYBOARD_INTERACTIVITY: Self = Self(3);

    /// exclusive edge is invalid given the surface anchors
    pub const INVALID_EXCLUSIVE_EDGE: Self = Self(4);
}

impl Debug for ZwlrLayerSurfaceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_SURFACE_STATE => "INVALID_SURFACE_STATE",
            Self::INVALID_SIZE => "INVALID_SIZE",
            Self::INVALID_ANCHOR => "INVALID_ANCHOR",
            Self::INVALID_KEYBOARD_INTERACTIVITY => "INVALID_KEYBOARD_INTERACTIVITY",
            Self::INVALID_EXCLUSIVE_EDGE => "INVALID_EXCLUSIVE_EDGE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct ZwlrLayerSurfaceV1Anchor(pub u32);

/// An iterator over the set bits in a [`ZwlrLayerSurfaceV1Anchor`].
///
/// You can construct this with the `IntoIterator` implementation of `ZwlrLayerSurfaceV1Anchor`.
#[derive(Clone, Debug)]
pub struct ZwlrLayerSurfaceV1AnchorIter(pub u32);

impl ZwlrLayerSurfaceV1Anchor {
    /// the top edge of the anchor rectangle
    pub const TOP: Self = Self(1);

    /// the bottom edge of the anchor rectangle
    pub const BOTTOM: Self = Self(2);

    /// the left edge of the anchor rectangle
    pub const LEFT: Self = Self(4);

    /// the right edge of the anchor rectangle
    pub const RIGHT: Self = Self(8);
}

impl ZwlrLayerSurfaceV1Anchor {
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
        Self(0 | 1 | 2 | 4 | 8)
    }
}

impl Iterator for ZwlrLayerSurfaceV1AnchorIter {
    type Item = ZwlrLayerSurfaceV1Anchor;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(ZwlrLayerSurfaceV1Anchor(bit))
    }
}

impl IntoIterator for ZwlrLayerSurfaceV1Anchor {
    type Item = ZwlrLayerSurfaceV1Anchor;
    type IntoIter = ZwlrLayerSurfaceV1AnchorIter;

    fn into_iter(self) -> Self::IntoIter {
        ZwlrLayerSurfaceV1AnchorIter(self.0)
    }
}

impl BitAnd for ZwlrLayerSurfaceV1Anchor {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for ZwlrLayerSurfaceV1Anchor {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for ZwlrLayerSurfaceV1Anchor {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for ZwlrLayerSurfaceV1Anchor {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for ZwlrLayerSurfaceV1Anchor {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for ZwlrLayerSurfaceV1Anchor {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for ZwlrLayerSurfaceV1Anchor {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for ZwlrLayerSurfaceV1Anchor {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for ZwlrLayerSurfaceV1Anchor {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for ZwlrLayerSurfaceV1Anchor {
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
        if v & 8 == 8 {
            v &= !8;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("RIGHT")?;
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
