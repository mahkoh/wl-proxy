//! child surface positioner
//!
//! The xdg_positioner provides a collection of rules for the placement of a
//! child surface relative to a parent surface. Rules can be defined to ensure
//! the child surface remains within the visible area's borders, and to
//! specify how the child surface changes its position, such as sliding along
//! an axis, or flipping around a rectangle. These positioner-created rules are
//! constrained by the requirement that a child surface must intersect with or
//! be at least partially adjacent to its parent surface.
//!
//! See the various requests for details about possible rules.
//!
//! At the time of the request, the compositor makes a copy of the rules
//! specified by the xdg_positioner. Thus, after the request is complete the
//! xdg_positioner object can be destroyed or reused; further changes to the
//! object will have no effect on previous usages.
//!
//! For an xdg_positioner object to be considered complete, it must have a
//! non-zero size set by set_size, and a non-zero anchor rectangle set by
//! set_anchor_rect. Passing an incomplete xdg_positioner object when
//! positioning a surface raises an invalid_positioner error.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A xdg_positioner object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XdgPositioner {
    core: ObjectCore,
    handler: HandlerHolder<dyn XdgPositionerHandler>,
}

struct DefaultHandler;

impl XdgPositionerHandler for DefaultHandler { }

impl ConcreteObject for XdgPositioner {
    const XML_VERSION: u32 = 7;
    const INTERFACE: ObjectInterface = ObjectInterface::XdgPositioner;
    const INTERFACE_NAME: &str = "xdg_positioner";
}

impl XdgPositioner {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl XdgPositionerHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn XdgPositionerHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XdgPositioner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XdgPositioner")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XdgPositioner {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the xdg_positioner object
    ///
    /// Notify the compositor that the xdg_positioner will no longer be used.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_positioner#{}.destroy()\n", id);
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

    /// destroy the xdg_positioner object
    ///
    /// Notify the compositor that the xdg_positioner will no longer be used.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("xdg_positioner.destroy", &e);
        }
    }

    /// Since when the set_size message is available.
    pub const MSG__SET_SIZE__SINCE: u32 = 1;

    /// set the size of the to-be positioned rectangle
    ///
    /// Set the size of the surface that is to be positioned with the positioner
    /// object. The size is in surface-local coordinates and corresponds to the
    /// window geometry. See xdg_surface.set_window_geometry.
    ///
    /// If a zero or negative size is set the invalid_input error is raised.
    ///
    /// # Arguments
    ///
    /// - `width`: width of positioned rectangle
    /// - `height`: height of positioned rectangle
    #[inline]
    pub fn try_send_set_size(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_positioner#{}.set_size(width: {}, height: {})\n", id, arg0, arg1);
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
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// set the size of the to-be positioned rectangle
    ///
    /// Set the size of the surface that is to be positioned with the positioner
    /// object. The size is in surface-local coordinates and corresponds to the
    /// window geometry. See xdg_surface.set_window_geometry.
    ///
    /// If a zero or negative size is set the invalid_input error is raised.
    ///
    /// # Arguments
    ///
    /// - `width`: width of positioned rectangle
    /// - `height`: height of positioned rectangle
    #[inline]
    pub fn send_set_size(
        &self,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_set_size(
            width,
            height,
        );
        if let Err(e) = res {
            log_send("xdg_positioner.set_size", &e);
        }
    }

    /// Since when the set_anchor_rect message is available.
    pub const MSG__SET_ANCHOR_RECT__SINCE: u32 = 1;

    /// set the anchor rectangle within the parent surface
    ///
    /// Specify the anchor rectangle within the parent surface that the child
    /// surface will be placed relative to. The rectangle is relative to the
    /// window geometry as defined by xdg_surface.set_window_geometry of the
    /// parent surface.
    ///
    /// When the xdg_positioner object is used to position a child surface, the
    /// anchor rectangle may not extend outside the window geometry of the
    /// positioned child's parent surface.
    ///
    /// If a negative size is set the invalid_input error is raised.
    ///
    /// # Arguments
    ///
    /// - `x`: x position of anchor rectangle
    /// - `y`: y position of anchor rectangle
    /// - `width`: width of anchor rectangle
    /// - `height`: height of anchor rectangle
    #[inline]
    pub fn try_send_set_anchor_rect(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_positioner#{}.set_anchor_rect(x: {}, y: {}, width: {}, height: {})\n", id, arg0, arg1, arg2, arg3);
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
            2,
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// set the anchor rectangle within the parent surface
    ///
    /// Specify the anchor rectangle within the parent surface that the child
    /// surface will be placed relative to. The rectangle is relative to the
    /// window geometry as defined by xdg_surface.set_window_geometry of the
    /// parent surface.
    ///
    /// When the xdg_positioner object is used to position a child surface, the
    /// anchor rectangle may not extend outside the window geometry of the
    /// positioned child's parent surface.
    ///
    /// If a negative size is set the invalid_input error is raised.
    ///
    /// # Arguments
    ///
    /// - `x`: x position of anchor rectangle
    /// - `y`: y position of anchor rectangle
    /// - `width`: width of anchor rectangle
    /// - `height`: height of anchor rectangle
    #[inline]
    pub fn send_set_anchor_rect(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_set_anchor_rect(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_send("xdg_positioner.set_anchor_rect", &e);
        }
    }

    /// Since when the set_anchor message is available.
    pub const MSG__SET_ANCHOR__SINCE: u32 = 1;

    /// set anchor rectangle anchor
    ///
    /// Defines the anchor point for the anchor rectangle. The specified anchor
    /// is used derive an anchor point that the child surface will be
    /// positioned relative to. If a corner anchor is set (e.g. 'top_left' or
    /// 'bottom_right'), the anchor point will be at the specified corner;
    /// otherwise, the derived anchor point will be centered on the specified
    /// edge, or in the center of the anchor rectangle if no edge is specified.
    ///
    /// # Arguments
    ///
    /// - `anchor`: anchor
    #[inline]
    pub fn try_send_set_anchor(
        &self,
        anchor: XdgPositionerAnchor,
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
            fn log(state: &State, id: u32, arg0: XdgPositionerAnchor) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_positioner#{}.set_anchor(anchor: {:?})\n", id, arg0);
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
            3,
            arg0.0,
        ]);
        Ok(())
    }

    /// set anchor rectangle anchor
    ///
    /// Defines the anchor point for the anchor rectangle. The specified anchor
    /// is used derive an anchor point that the child surface will be
    /// positioned relative to. If a corner anchor is set (e.g. 'top_left' or
    /// 'bottom_right'), the anchor point will be at the specified corner;
    /// otherwise, the derived anchor point will be centered on the specified
    /// edge, or in the center of the anchor rectangle if no edge is specified.
    ///
    /// # Arguments
    ///
    /// - `anchor`: anchor
    #[inline]
    pub fn send_set_anchor(
        &self,
        anchor: XdgPositionerAnchor,
    ) {
        let res = self.try_send_set_anchor(
            anchor,
        );
        if let Err(e) = res {
            log_send("xdg_positioner.set_anchor", &e);
        }
    }

    /// Since when the set_gravity message is available.
    pub const MSG__SET_GRAVITY__SINCE: u32 = 1;

    /// set child surface gravity
    ///
    /// Defines in what direction a surface should be positioned, relative to
    /// the anchor point of the parent surface. If a corner gravity is
    /// specified (e.g. 'bottom_right' or 'top_left'), then the child surface
    /// will be placed towards the specified gravity; otherwise, the child
    /// surface will be centered over the anchor point on any axis that had no
    /// gravity specified. If the gravity is not in the ‘gravity’ enum, an
    /// invalid_input error is raised.
    ///
    /// # Arguments
    ///
    /// - `gravity`: gravity direction
    #[inline]
    pub fn try_send_set_gravity(
        &self,
        gravity: XdgPositionerGravity,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            gravity,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: XdgPositionerGravity) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_positioner#{}.set_gravity(gravity: {:?})\n", id, arg0);
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

    /// set child surface gravity
    ///
    /// Defines in what direction a surface should be positioned, relative to
    /// the anchor point of the parent surface. If a corner gravity is
    /// specified (e.g. 'bottom_right' or 'top_left'), then the child surface
    /// will be placed towards the specified gravity; otherwise, the child
    /// surface will be centered over the anchor point on any axis that had no
    /// gravity specified. If the gravity is not in the ‘gravity’ enum, an
    /// invalid_input error is raised.
    ///
    /// # Arguments
    ///
    /// - `gravity`: gravity direction
    #[inline]
    pub fn send_set_gravity(
        &self,
        gravity: XdgPositionerGravity,
    ) {
        let res = self.try_send_set_gravity(
            gravity,
        );
        if let Err(e) = res {
            log_send("xdg_positioner.set_gravity", &e);
        }
    }

    /// Since when the set_constraint_adjustment message is available.
    pub const MSG__SET_CONSTRAINT_ADJUSTMENT__SINCE: u32 = 1;

    /// set the adjustment to be done when constrained
    ///
    /// Specify how the window should be positioned if the originally intended
    /// position caused the surface to be constrained, meaning at least
    /// partially outside positioning boundaries set by the compositor. The
    /// adjustment is set by constructing a bitmask describing the adjustment to
    /// be made when the surface is constrained on that axis.
    ///
    /// If no bit for one axis is set, the compositor will assume that the child
    /// surface should not change its position on that axis when constrained.
    ///
    /// If more than one bit for one axis is set, the order of how adjustments
    /// are applied is specified in the corresponding adjustment descriptions.
    ///
    /// The default adjustment is none.
    ///
    /// # Arguments
    ///
    /// - `constraint_adjustment`: bit mask of constraint adjustments
    #[inline]
    pub fn try_send_set_constraint_adjustment(
        &self,
        constraint_adjustment: XdgPositionerConstraintAdjustment,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            constraint_adjustment,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: XdgPositionerConstraintAdjustment) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_positioner#{}.set_constraint_adjustment(constraint_adjustment: {:?})\n", id, arg0);
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
            5,
            arg0.0,
        ]);
        Ok(())
    }

    /// set the adjustment to be done when constrained
    ///
    /// Specify how the window should be positioned if the originally intended
    /// position caused the surface to be constrained, meaning at least
    /// partially outside positioning boundaries set by the compositor. The
    /// adjustment is set by constructing a bitmask describing the adjustment to
    /// be made when the surface is constrained on that axis.
    ///
    /// If no bit for one axis is set, the compositor will assume that the child
    /// surface should not change its position on that axis when constrained.
    ///
    /// If more than one bit for one axis is set, the order of how adjustments
    /// are applied is specified in the corresponding adjustment descriptions.
    ///
    /// The default adjustment is none.
    ///
    /// # Arguments
    ///
    /// - `constraint_adjustment`: bit mask of constraint adjustments
    #[inline]
    pub fn send_set_constraint_adjustment(
        &self,
        constraint_adjustment: XdgPositionerConstraintAdjustment,
    ) {
        let res = self.try_send_set_constraint_adjustment(
            constraint_adjustment,
        );
        if let Err(e) = res {
            log_send("xdg_positioner.set_constraint_adjustment", &e);
        }
    }

    /// Since when the set_offset message is available.
    pub const MSG__SET_OFFSET__SINCE: u32 = 1;

    /// set surface position offset
    ///
    /// Specify the surface position offset relative to the position of the
    /// anchor on the anchor rectangle and the anchor on the surface. For
    /// example if the anchor of the anchor rectangle is at (x, y), the surface
    /// has the gravity bottom|right, and the offset is (ox, oy), the calculated
    /// surface position will be (x + ox, y + oy). The offset position of the
    /// surface is the one used for constraint testing. See
    /// set_constraint_adjustment.
    ///
    /// An example use case is placing a popup menu on top of a user interface
    /// element, while aligning the user interface element of the parent surface
    /// with some user interface element placed somewhere in the popup surface.
    ///
    /// # Arguments
    ///
    /// - `x`: surface position x offset
    /// - `y`: surface position y offset
    #[inline]
    pub fn try_send_set_offset(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_positioner#{}.set_offset(x: {}, y: {})\n", id, arg0, arg1);
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
            6,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// set surface position offset
    ///
    /// Specify the surface position offset relative to the position of the
    /// anchor on the anchor rectangle and the anchor on the surface. For
    /// example if the anchor of the anchor rectangle is at (x, y), the surface
    /// has the gravity bottom|right, and the offset is (ox, oy), the calculated
    /// surface position will be (x + ox, y + oy). The offset position of the
    /// surface is the one used for constraint testing. See
    /// set_constraint_adjustment.
    ///
    /// An example use case is placing a popup menu on top of a user interface
    /// element, while aligning the user interface element of the parent surface
    /// with some user interface element placed somewhere in the popup surface.
    ///
    /// # Arguments
    ///
    /// - `x`: surface position x offset
    /// - `y`: surface position y offset
    #[inline]
    pub fn send_set_offset(
        &self,
        x: i32,
        y: i32,
    ) {
        let res = self.try_send_set_offset(
            x,
            y,
        );
        if let Err(e) = res {
            log_send("xdg_positioner.set_offset", &e);
        }
    }

    /// Since when the set_reactive message is available.
    pub const MSG__SET_REACTIVE__SINCE: u32 = 3;

    /// continuously reconstrain the surface
    ///
    /// When set reactive, the surface is reconstrained if the conditions used
    /// for constraining changed, e.g. the parent window moved.
    ///
    /// If the conditions changed and the popup was reconstrained, an
    /// xdg_popup.configure event is sent with updated geometry, followed by an
    /// xdg_surface.configure event.
    #[inline]
    pub fn try_send_set_reactive(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_positioner#{}.set_reactive()\n", id);
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
        Ok(())
    }

    /// continuously reconstrain the surface
    ///
    /// When set reactive, the surface is reconstrained if the conditions used
    /// for constraining changed, e.g. the parent window moved.
    ///
    /// If the conditions changed and the popup was reconstrained, an
    /// xdg_popup.configure event is sent with updated geometry, followed by an
    /// xdg_surface.configure event.
    #[inline]
    pub fn send_set_reactive(
        &self,
    ) {
        let res = self.try_send_set_reactive(
        );
        if let Err(e) = res {
            log_send("xdg_positioner.set_reactive", &e);
        }
    }

    /// Since when the set_parent_size message is available.
    pub const MSG__SET_PARENT_SIZE__SINCE: u32 = 3;

    ///
    /// Set the parent window geometry the compositor should use when
    /// positioning the popup. The compositor may use this information to
    /// determine the future state the popup should be constrained using. If
    /// this doesn't match the dimension of the parent the popup is eventually
    /// positioned against, the behavior is undefined.
    ///
    /// The arguments are given in the surface-local coordinate space.
    ///
    /// # Arguments
    ///
    /// - `parent_width`: future window geometry width of parent
    /// - `parent_height`: future window geometry height of parent
    #[inline]
    pub fn try_send_set_parent_size(
        &self,
        parent_width: i32,
        parent_height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            parent_width,
            parent_height,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_positioner#{}.set_parent_size(parent_width: {}, parent_height: {})\n", id, arg0, arg1);
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
            8,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    ///
    /// Set the parent window geometry the compositor should use when
    /// positioning the popup. The compositor may use this information to
    /// determine the future state the popup should be constrained using. If
    /// this doesn't match the dimension of the parent the popup is eventually
    /// positioned against, the behavior is undefined.
    ///
    /// The arguments are given in the surface-local coordinate space.
    ///
    /// # Arguments
    ///
    /// - `parent_width`: future window geometry width of parent
    /// - `parent_height`: future window geometry height of parent
    #[inline]
    pub fn send_set_parent_size(
        &self,
        parent_width: i32,
        parent_height: i32,
    ) {
        let res = self.try_send_set_parent_size(
            parent_width,
            parent_height,
        );
        if let Err(e) = res {
            log_send("xdg_positioner.set_parent_size", &e);
        }
    }

    /// Since when the set_parent_configure message is available.
    pub const MSG__SET_PARENT_CONFIGURE__SINCE: u32 = 3;

    /// set parent configure this is a response to
    ///
    /// Set the serial of an xdg_surface.configure event this positioner will be
    /// used in response to. The compositor may use this information together
    /// with set_parent_size to determine what future state the popup should be
    /// constrained using.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of parent configure event
    #[inline]
    pub fn try_send_set_parent_configure(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_positioner#{}.set_parent_configure(serial: {})\n", id, arg0);
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
            arg0,
        ]);
        Ok(())
    }

    /// set parent configure this is a response to
    ///
    /// Set the serial of an xdg_surface.configure event this positioner will be
    /// used in response to. The compositor may use this information together
    /// with set_parent_size to determine what future state the popup should be
    /// constrained using.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of parent configure event
    #[inline]
    pub fn send_set_parent_configure(
        &self,
        serial: u32,
    ) {
        let res = self.try_send_set_parent_configure(
            serial,
        );
        if let Err(e) = res {
            log_send("xdg_positioner.set_parent_configure", &e);
        }
    }
}

/// A message handler for [`XdgPositioner`] proxies.
pub trait XdgPositionerHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<XdgPositioner>) {
        slf.core.delete_id();
    }

    /// destroy the xdg_positioner object
    ///
    /// Notify the compositor that the xdg_positioner will no longer be used.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<XdgPositioner>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("xdg_positioner.destroy", &e);
        }
    }

    /// set the size of the to-be positioned rectangle
    ///
    /// Set the size of the surface that is to be positioned with the positioner
    /// object. The size is in surface-local coordinates and corresponds to the
    /// window geometry. See xdg_surface.set_window_geometry.
    ///
    /// If a zero or negative size is set the invalid_input error is raised.
    ///
    /// # Arguments
    ///
    /// - `width`: width of positioned rectangle
    /// - `height`: height of positioned rectangle
    #[inline]
    fn handle_set_size(
        &mut self,
        slf: &Rc<XdgPositioner>,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_size(
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("xdg_positioner.set_size", &e);
        }
    }

    /// set the anchor rectangle within the parent surface
    ///
    /// Specify the anchor rectangle within the parent surface that the child
    /// surface will be placed relative to. The rectangle is relative to the
    /// window geometry as defined by xdg_surface.set_window_geometry of the
    /// parent surface.
    ///
    /// When the xdg_positioner object is used to position a child surface, the
    /// anchor rectangle may not extend outside the window geometry of the
    /// positioned child's parent surface.
    ///
    /// If a negative size is set the invalid_input error is raised.
    ///
    /// # Arguments
    ///
    /// - `x`: x position of anchor rectangle
    /// - `y`: y position of anchor rectangle
    /// - `width`: width of anchor rectangle
    /// - `height`: height of anchor rectangle
    #[inline]
    fn handle_set_anchor_rect(
        &mut self,
        slf: &Rc<XdgPositioner>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_anchor_rect(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("xdg_positioner.set_anchor_rect", &e);
        }
    }

    /// set anchor rectangle anchor
    ///
    /// Defines the anchor point for the anchor rectangle. The specified anchor
    /// is used derive an anchor point that the child surface will be
    /// positioned relative to. If a corner anchor is set (e.g. 'top_left' or
    /// 'bottom_right'), the anchor point will be at the specified corner;
    /// otherwise, the derived anchor point will be centered on the specified
    /// edge, or in the center of the anchor rectangle if no edge is specified.
    ///
    /// # Arguments
    ///
    /// - `anchor`: anchor
    #[inline]
    fn handle_set_anchor(
        &mut self,
        slf: &Rc<XdgPositioner>,
        anchor: XdgPositionerAnchor,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_anchor(
            anchor,
        );
        if let Err(e) = res {
            log_forward("xdg_positioner.set_anchor", &e);
        }
    }

    /// set child surface gravity
    ///
    /// Defines in what direction a surface should be positioned, relative to
    /// the anchor point of the parent surface. If a corner gravity is
    /// specified (e.g. 'bottom_right' or 'top_left'), then the child surface
    /// will be placed towards the specified gravity; otherwise, the child
    /// surface will be centered over the anchor point on any axis that had no
    /// gravity specified. If the gravity is not in the ‘gravity’ enum, an
    /// invalid_input error is raised.
    ///
    /// # Arguments
    ///
    /// - `gravity`: gravity direction
    #[inline]
    fn handle_set_gravity(
        &mut self,
        slf: &Rc<XdgPositioner>,
        gravity: XdgPositionerGravity,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_gravity(
            gravity,
        );
        if let Err(e) = res {
            log_forward("xdg_positioner.set_gravity", &e);
        }
    }

    /// set the adjustment to be done when constrained
    ///
    /// Specify how the window should be positioned if the originally intended
    /// position caused the surface to be constrained, meaning at least
    /// partially outside positioning boundaries set by the compositor. The
    /// adjustment is set by constructing a bitmask describing the adjustment to
    /// be made when the surface is constrained on that axis.
    ///
    /// If no bit for one axis is set, the compositor will assume that the child
    /// surface should not change its position on that axis when constrained.
    ///
    /// If more than one bit for one axis is set, the order of how adjustments
    /// are applied is specified in the corresponding adjustment descriptions.
    ///
    /// The default adjustment is none.
    ///
    /// # Arguments
    ///
    /// - `constraint_adjustment`: bit mask of constraint adjustments
    #[inline]
    fn handle_set_constraint_adjustment(
        &mut self,
        slf: &Rc<XdgPositioner>,
        constraint_adjustment: XdgPositionerConstraintAdjustment,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_constraint_adjustment(
            constraint_adjustment,
        );
        if let Err(e) = res {
            log_forward("xdg_positioner.set_constraint_adjustment", &e);
        }
    }

    /// set surface position offset
    ///
    /// Specify the surface position offset relative to the position of the
    /// anchor on the anchor rectangle and the anchor on the surface. For
    /// example if the anchor of the anchor rectangle is at (x, y), the surface
    /// has the gravity bottom|right, and the offset is (ox, oy), the calculated
    /// surface position will be (x + ox, y + oy). The offset position of the
    /// surface is the one used for constraint testing. See
    /// set_constraint_adjustment.
    ///
    /// An example use case is placing a popup menu on top of a user interface
    /// element, while aligning the user interface element of the parent surface
    /// with some user interface element placed somewhere in the popup surface.
    ///
    /// # Arguments
    ///
    /// - `x`: surface position x offset
    /// - `y`: surface position y offset
    #[inline]
    fn handle_set_offset(
        &mut self,
        slf: &Rc<XdgPositioner>,
        x: i32,
        y: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_offset(
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("xdg_positioner.set_offset", &e);
        }
    }

    /// continuously reconstrain the surface
    ///
    /// When set reactive, the surface is reconstrained if the conditions used
    /// for constraining changed, e.g. the parent window moved.
    ///
    /// If the conditions changed and the popup was reconstrained, an
    /// xdg_popup.configure event is sent with updated geometry, followed by an
    /// xdg_surface.configure event.
    #[inline]
    fn handle_set_reactive(
        &mut self,
        slf: &Rc<XdgPositioner>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_reactive(
        );
        if let Err(e) = res {
            log_forward("xdg_positioner.set_reactive", &e);
        }
    }

    ///
    /// Set the parent window geometry the compositor should use when
    /// positioning the popup. The compositor may use this information to
    /// determine the future state the popup should be constrained using. If
    /// this doesn't match the dimension of the parent the popup is eventually
    /// positioned against, the behavior is undefined.
    ///
    /// The arguments are given in the surface-local coordinate space.
    ///
    /// # Arguments
    ///
    /// - `parent_width`: future window geometry width of parent
    /// - `parent_height`: future window geometry height of parent
    #[inline]
    fn handle_set_parent_size(
        &mut self,
        slf: &Rc<XdgPositioner>,
        parent_width: i32,
        parent_height: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_parent_size(
            parent_width,
            parent_height,
        );
        if let Err(e) = res {
            log_forward("xdg_positioner.set_parent_size", &e);
        }
    }

    /// set parent configure this is a response to
    ///
    /// Set the serial of an xdg_surface.configure event this positioner will be
    /// used in response to. The compositor may use this information together
    /// with set_parent_size to determine what future state the popup should be
    /// constrained using.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of parent configure event
    #[inline]
    fn handle_set_parent_configure(
        &mut self,
        slf: &Rc<XdgPositioner>,
        serial: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_parent_configure(
            serial,
        );
        if let Err(e) = res {
            log_forward("xdg_positioner.set_parent_configure", &e);
        }
    }
}

impl ObjectPrivate for XdgPositioner {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::XdgPositioner, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_positioner#{}.destroy()\n", client_id, id);
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
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_positioner#{}.set_size(width: {}, height: {})\n", client_id, id, arg0, arg1);
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
            2 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_positioner#{}.set_anchor_rect(x: {}, y: {}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_anchor_rect(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_set_anchor_rect(&self, arg0, arg1, arg2, arg3);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = XdgPositionerAnchor(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: XdgPositionerAnchor) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_positioner#{}.set_anchor(anchor: {:?})\n", client_id, id, arg0);
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
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = XdgPositionerGravity(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: XdgPositionerGravity) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_positioner#{}.set_gravity(gravity: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_gravity(&self, arg0);
                } else {
                    DefaultHandler.handle_set_gravity(&self, arg0);
                }
            }
            5 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = XdgPositionerConstraintAdjustment(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: XdgPositionerConstraintAdjustment) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_positioner#{}.set_constraint_adjustment(constraint_adjustment: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_constraint_adjustment(&self, arg0);
                } else {
                    DefaultHandler.handle_set_constraint_adjustment(&self, arg0);
                }
            }
            6 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_positioner#{}.set_offset(x: {}, y: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_offset(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_offset(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_positioner#{}.set_reactive()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_reactive(&self);
                } else {
                    DefaultHandler.handle_set_reactive(&self);
                }
            }
            8 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_positioner#{}.set_parent_size(parent_width: {}, parent_height: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_parent_size(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_parent_size(&self, arg0, arg1);
                }
            }
            9 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_positioner#{}.set_parent_configure(serial: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_parent_configure(&self, arg0);
                } else {
                    DefaultHandler.handle_set_parent_configure(&self, arg0);
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
            1 => "set_size",
            2 => "set_anchor_rect",
            3 => "set_anchor",
            4 => "set_gravity",
            5 => "set_constraint_adjustment",
            6 => "set_offset",
            7 => "set_reactive",
            8 => "set_parent_size",
            9 => "set_parent_configure",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for XdgPositioner {
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

impl XdgPositioner {
    /// Since when the error.invalid_input enum variant is available.
    pub const ENM__ERROR_INVALID_INPUT__SINCE: u32 = 1;

    /// Since when the anchor.none enum variant is available.
    pub const ENM__ANCHOR_NONE__SINCE: u32 = 1;
    /// Since when the anchor.top enum variant is available.
    pub const ENM__ANCHOR_TOP__SINCE: u32 = 1;
    /// Since when the anchor.bottom enum variant is available.
    pub const ENM__ANCHOR_BOTTOM__SINCE: u32 = 1;
    /// Since when the anchor.left enum variant is available.
    pub const ENM__ANCHOR_LEFT__SINCE: u32 = 1;
    /// Since when the anchor.right enum variant is available.
    pub const ENM__ANCHOR_RIGHT__SINCE: u32 = 1;
    /// Since when the anchor.top_left enum variant is available.
    pub const ENM__ANCHOR_TOP_LEFT__SINCE: u32 = 1;
    /// Since when the anchor.bottom_left enum variant is available.
    pub const ENM__ANCHOR_BOTTOM_LEFT__SINCE: u32 = 1;
    /// Since when the anchor.top_right enum variant is available.
    pub const ENM__ANCHOR_TOP_RIGHT__SINCE: u32 = 1;
    /// Since when the anchor.bottom_right enum variant is available.
    pub const ENM__ANCHOR_BOTTOM_RIGHT__SINCE: u32 = 1;

    /// Since when the gravity.none enum variant is available.
    pub const ENM__GRAVITY_NONE__SINCE: u32 = 1;
    /// Since when the gravity.top enum variant is available.
    pub const ENM__GRAVITY_TOP__SINCE: u32 = 1;
    /// Since when the gravity.bottom enum variant is available.
    pub const ENM__GRAVITY_BOTTOM__SINCE: u32 = 1;
    /// Since when the gravity.left enum variant is available.
    pub const ENM__GRAVITY_LEFT__SINCE: u32 = 1;
    /// Since when the gravity.right enum variant is available.
    pub const ENM__GRAVITY_RIGHT__SINCE: u32 = 1;
    /// Since when the gravity.top_left enum variant is available.
    pub const ENM__GRAVITY_TOP_LEFT__SINCE: u32 = 1;
    /// Since when the gravity.bottom_left enum variant is available.
    pub const ENM__GRAVITY_BOTTOM_LEFT__SINCE: u32 = 1;
    /// Since when the gravity.top_right enum variant is available.
    pub const ENM__GRAVITY_TOP_RIGHT__SINCE: u32 = 1;
    /// Since when the gravity.bottom_right enum variant is available.
    pub const ENM__GRAVITY_BOTTOM_RIGHT__SINCE: u32 = 1;

    /// Since when the constraint_adjustment.none enum variant is available.
    pub const ENM__CONSTRAINT_ADJUSTMENT_NONE__SINCE: u32 = 1;
    /// Since when the constraint_adjustment.slide_x enum variant is available.
    pub const ENM__CONSTRAINT_ADJUSTMENT_SLIDE_X__SINCE: u32 = 1;
    /// Since when the constraint_adjustment.slide_y enum variant is available.
    pub const ENM__CONSTRAINT_ADJUSTMENT_SLIDE_Y__SINCE: u32 = 1;
    /// Since when the constraint_adjustment.flip_x enum variant is available.
    pub const ENM__CONSTRAINT_ADJUSTMENT_FLIP_X__SINCE: u32 = 1;
    /// Since when the constraint_adjustment.flip_y enum variant is available.
    pub const ENM__CONSTRAINT_ADJUSTMENT_FLIP_Y__SINCE: u32 = 1;
    /// Since when the constraint_adjustment.resize_x enum variant is available.
    pub const ENM__CONSTRAINT_ADJUSTMENT_RESIZE_X__SINCE: u32 = 1;
    /// Since when the constraint_adjustment.resize_y enum variant is available.
    pub const ENM__CONSTRAINT_ADJUSTMENT_RESIZE_Y__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct XdgPositionerError(pub u32);

impl XdgPositionerError {
    /// invalid input provided
    pub const INVALID_INPUT: Self = Self(0);
}

impl Debug for XdgPositionerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_INPUT => "INVALID_INPUT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct XdgPositionerAnchor(pub u32);

impl XdgPositionerAnchor {
    pub const NONE: Self = Self(0);

    pub const TOP: Self = Self(1);

    pub const BOTTOM: Self = Self(2);

    pub const LEFT: Self = Self(3);

    pub const RIGHT: Self = Self(4);

    pub const TOP_LEFT: Self = Self(5);

    pub const BOTTOM_LEFT: Self = Self(6);

    pub const TOP_RIGHT: Self = Self(7);

    pub const BOTTOM_RIGHT: Self = Self(8);
}

impl Debug for XdgPositionerAnchor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NONE => "NONE",
            Self::TOP => "TOP",
            Self::BOTTOM => "BOTTOM",
            Self::LEFT => "LEFT",
            Self::RIGHT => "RIGHT",
            Self::TOP_LEFT => "TOP_LEFT",
            Self::BOTTOM_LEFT => "BOTTOM_LEFT",
            Self::TOP_RIGHT => "TOP_RIGHT",
            Self::BOTTOM_RIGHT => "BOTTOM_RIGHT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct XdgPositionerGravity(pub u32);

impl XdgPositionerGravity {
    pub const NONE: Self = Self(0);

    pub const TOP: Self = Self(1);

    pub const BOTTOM: Self = Self(2);

    pub const LEFT: Self = Self(3);

    pub const RIGHT: Self = Self(4);

    pub const TOP_LEFT: Self = Self(5);

    pub const BOTTOM_LEFT: Self = Self(6);

    pub const TOP_RIGHT: Self = Self(7);

    pub const BOTTOM_RIGHT: Self = Self(8);
}

impl Debug for XdgPositionerGravity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NONE => "NONE",
            Self::TOP => "TOP",
            Self::BOTTOM => "BOTTOM",
            Self::LEFT => "LEFT",
            Self::RIGHT => "RIGHT",
            Self::TOP_LEFT => "TOP_LEFT",
            Self::BOTTOM_LEFT => "BOTTOM_LEFT",
            Self::TOP_RIGHT => "TOP_RIGHT",
            Self::BOTTOM_RIGHT => "BOTTOM_RIGHT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// constraint adjustments
///
/// The constraint adjustment value define ways the compositor will adjust
/// the position of the surface, if the unadjusted position would result
/// in the surface being partly constrained.
///
/// Whether a surface is considered 'constrained' is left to the compositor
/// to determine. For example, the surface may be partly outside the
/// compositor's defined 'work area', thus necessitating the child surface's
/// position be adjusted until it is entirely inside the work area.
///
/// The adjustments can be combined, according to a defined precedence: 1)
/// Flip, 2) Slide, 3) Resize.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct XdgPositionerConstraintAdjustment(pub u32);

/// An iterator over the set bits in a [`XdgPositionerConstraintAdjustment`].
///
/// You can construct this with the `IntoIterator` implementation of `XdgPositionerConstraintAdjustment`.
#[derive(Clone, Debug)]
pub struct XdgPositionerConstraintAdjustmentIter(pub u32);

impl XdgPositionerConstraintAdjustment {
    /// don't move the child surface when constrained
    ///
    /// Don't alter the surface position even if it is constrained on some
    /// axis, for example partially outside the edge of an output.
    pub const NONE: Self = Self(0);

    /// move along the x axis until unconstrained
    ///
    /// Slide the surface along the x axis until it is no longer constrained.
    ///
    /// First try to slide towards the direction of the gravity on the x axis
    /// until either the edge in the opposite direction of the gravity is
    /// unconstrained or the edge in the direction of the gravity is
    /// constrained.
    ///
    /// Then try to slide towards the opposite direction of the gravity on the
    /// x axis until either the edge in the direction of the gravity is
    /// unconstrained or the edge in the opposite direction of the gravity is
    /// constrained.
    pub const SLIDE_X: Self = Self(1);

    /// move along the y axis until unconstrained
    ///
    /// Slide the surface along the y axis until it is no longer constrained.
    ///
    /// First try to slide towards the direction of the gravity on the y axis
    /// until either the edge in the opposite direction of the gravity is
    /// unconstrained or the edge in the direction of the gravity is
    /// constrained.
    ///
    /// Then try to slide towards the opposite direction of the gravity on the
    /// y axis until either the edge in the direction of the gravity is
    /// unconstrained or the edge in the opposite direction of the gravity is
    /// constrained.
    pub const SLIDE_Y: Self = Self(2);

    /// invert the anchor and gravity on the x axis
    ///
    /// Invert the anchor and gravity on the x axis if the surface is
    /// constrained on the x axis. For example, if the left edge of the
    /// surface is constrained, the gravity is 'left' and the anchor is
    /// 'left', change the gravity to 'right' and the anchor to 'right'.
    ///
    /// If the adjusted position also ends up being constrained, the resulting
    /// position of the flip_x adjustment will be the one before the
    /// adjustment.
    pub const FLIP_X: Self = Self(4);

    /// invert the anchor and gravity on the y axis
    ///
    /// Invert the anchor and gravity on the y axis if the surface is
    /// constrained on the y axis. For example, if the bottom edge of the
    /// surface is constrained, the gravity is 'bottom' and the anchor is
    /// 'bottom', change the gravity to 'top' and the anchor to 'top'.
    ///
    /// The adjusted position is calculated given the original anchor
    /// rectangle and offset, but with the new flipped anchor and gravity
    /// values.
    ///
    /// If the adjusted position also ends up being constrained, the resulting
    /// position of the flip_y adjustment will be the one before the
    /// adjustment.
    pub const FLIP_Y: Self = Self(8);

    /// horizontally resize the surface
    ///
    /// Resize the surface horizontally so that it is completely
    /// unconstrained.
    pub const RESIZE_X: Self = Self(16);

    /// vertically resize the surface
    ///
    /// Resize the surface vertically so that it is completely unconstrained.
    pub const RESIZE_Y: Self = Self(32);
}

impl XdgPositionerConstraintAdjustment {
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
        Self(0 | 0 | 1 | 2 | 4 | 8 | 16 | 32)
    }
}

impl Iterator for XdgPositionerConstraintAdjustmentIter {
    type Item = XdgPositionerConstraintAdjustment;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(XdgPositionerConstraintAdjustment(bit))
    }
}

impl IntoIterator for XdgPositionerConstraintAdjustment {
    type Item = XdgPositionerConstraintAdjustment;
    type IntoIter = XdgPositionerConstraintAdjustmentIter;

    fn into_iter(self) -> Self::IntoIter {
        XdgPositionerConstraintAdjustmentIter(self.0)
    }
}

impl BitAnd for XdgPositionerConstraintAdjustment {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for XdgPositionerConstraintAdjustment {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for XdgPositionerConstraintAdjustment {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for XdgPositionerConstraintAdjustment {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for XdgPositionerConstraintAdjustment {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for XdgPositionerConstraintAdjustment {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for XdgPositionerConstraintAdjustment {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for XdgPositionerConstraintAdjustment {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for XdgPositionerConstraintAdjustment {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for XdgPositionerConstraintAdjustment {
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
            f.write_str("SLIDE_X")?;
        }
        if v & 2 == 2 {
            v &= !2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("SLIDE_Y")?;
        }
        if v & 4 == 4 {
            v &= !4;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("FLIP_X")?;
        }
        if v & 8 == 8 {
            v &= !8;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("FLIP_Y")?;
        }
        if v & 16 == 16 {
            v &= !16;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("RESIZE_X")?;
        }
        if v & 32 == 32 {
            v &= !32;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("RESIZE_Y")?;
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
