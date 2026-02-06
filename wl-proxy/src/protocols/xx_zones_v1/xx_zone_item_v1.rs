//! opaque surface object that can be positioned in a zone
//!
//! The zone item object is an opaque descriptor for a positionable
//! element, such as a toplevel window.
//! It currently can only be created from an 'xdg_toplevel' via the
//! 'get_zone_item' request on a 'xx_zone_manager'.
//!
//! The lifetime of a zone item is tied to its referenced item (usually
//! a toplevel).
//! When the reference is destroyed, the compositor must send a 'closed'
//! event and the zone item becomes inert.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A xx_zone_item_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XxZoneItemV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn XxZoneItemV1Handler>,
}

struct DefaultHandler;

impl XxZoneItemV1Handler for DefaultHandler { }

impl ConcreteObject for XxZoneItemV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::XxZoneItemV1;
    const INTERFACE_NAME: &str = "xx_zone_item_v1";
}

impl XxZoneItemV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl XxZoneItemV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn XxZoneItemV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XxZoneItemV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XxZoneItemV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XxZoneItemV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// delete this object
    ///
    /// Destroys the zone item. This request may be sent at any time by the
    /// client.
    /// By destroying the object, the respective item surface remains at its
    /// last position, but its association with its zone is lost.
    /// This will also cause it to lose any other attached state described by
    /// this protocol.
    ///
    /// If the item was associated with a zone when this request is sent,
    /// the compositor must emit 'item_left' on the respective zone, unless
    /// it had already been emitted before a 'closed' event.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xx_zone_item_v1#{}.destroy()\n", id);
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

    /// delete this object
    ///
    /// Destroys the zone item. This request may be sent at any time by the
    /// client.
    /// By destroying the object, the respective item surface remains at its
    /// last position, but its association with its zone is lost.
    /// This will also cause it to lose any other attached state described by
    /// this protocol.
    ///
    /// If the item was associated with a zone when this request is sent,
    /// the compositor must emit 'item_left' on the respective zone, unless
    /// it had already been emitted before a 'closed' event.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("xx_zone_item_v1.destroy", &e);
        }
    }

    /// Since when the frame_extents message is available.
    pub const MSG__FRAME_EXTENTS__SINCE: u32 = 1;

    /// the extents of the frame bordering the item
    ///
    /// The 'frame_extents' event describes the current extents of the frame
    /// bordering the item's content area.
    ///
    /// This event is sent immediately after the item joins a zone, or if
    /// the item frame extents have been changed by other means (e.g. toggled
    /// by a client request, or compositor involvement). The dimensions are in
    /// the same coordinate space as the item's zone (the surface coordinate
    /// space).
    ///
    /// This event must be followed by a 'position' event, even if the item's
    /// coordinates did not change as a result of the frame extents changing.
    ///
    /// If the item has no associated frame, the event should still be sent,
    /// but extents must be set to zero.
    ///
    /// This event can only be emitted if the item is currently associated
    /// with a zone.
    ///
    /// # Arguments
    ///
    /// - `top`: current height of the frame bordering the top of the item
    /// - `bottom`: current height of the frame bordering the bottom of the item
    /// - `left`: current width of the frame bordering the left of the item
    /// - `right`: current width of the frame bordering the right of the item
    #[inline]
    pub fn try_send_frame_extents(
        &self,
        top: i32,
        bottom: i32,
        left: i32,
        right: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            top,
            bottom,
            left,
            right,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xx_zone_item_v1#{}.frame_extents(top: {}, bottom: {}, left: {}, right: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1, arg2, arg3);
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
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// the extents of the frame bordering the item
    ///
    /// The 'frame_extents' event describes the current extents of the frame
    /// bordering the item's content area.
    ///
    /// This event is sent immediately after the item joins a zone, or if
    /// the item frame extents have been changed by other means (e.g. toggled
    /// by a client request, or compositor involvement). The dimensions are in
    /// the same coordinate space as the item's zone (the surface coordinate
    /// space).
    ///
    /// This event must be followed by a 'position' event, even if the item's
    /// coordinates did not change as a result of the frame extents changing.
    ///
    /// If the item has no associated frame, the event should still be sent,
    /// but extents must be set to zero.
    ///
    /// This event can only be emitted if the item is currently associated
    /// with a zone.
    ///
    /// # Arguments
    ///
    /// - `top`: current height of the frame bordering the top of the item
    /// - `bottom`: current height of the frame bordering the bottom of the item
    /// - `left`: current width of the frame bordering the left of the item
    /// - `right`: current width of the frame bordering the right of the item
    #[inline]
    pub fn send_frame_extents(
        &self,
        top: i32,
        bottom: i32,
        left: i32,
        right: i32,
    ) {
        let res = self.try_send_frame_extents(
            top,
            bottom,
            left,
            right,
        );
        if let Err(e) = res {
            log_send("xx_zone_item_v1.frame_extents", &e);
        }
    }

    /// Since when the set_position message is available.
    pub const MSG__SET_POSITION__SINCE: u32 = 1;

    /// set a preferred item surface position
    ///
    /// Request a preferred position (x, y) for the specified item
    /// surface to be placed at, relative to its associated zone.
    /// This state is double-buffered and is applied on the next
    /// wl_surface.commit of the surface represented by 'item'.
    ///
    /// X and Y coordinates are relative to the zone this item is associated
    /// with, and must not be larger than the dimensions set by the zone size.
    /// They may be smaller than zero, if the item's top-left edge is to be
    /// placed beyond the zone's top-left sides, but clients should expect the
    /// compositor to more aggressively sanitize the coordinate values in that
    /// case.
    /// If a coordinate exceeds the zone's maximum bounds, the compositor must
    /// sanitize it to more appropriate values (e.g. by clamping the values to
    /// the maximum size).
    /// For infinite zones, the client may pick any coordinate.
    ///
    /// Compositors implementing this protocol should try to place an item
    /// at the requested coordinates relative to the item's zone, unless doing
    /// so is not allowed by compositor policy (because e.g. the user has set
    /// custom rules for the surface represented by the respective item, the
    /// surface overlaps with a protected shell component, session management
    /// has loaded previous surface positions or the placement request would
    /// send the item out of bounds).
    ///
    /// Clients should be aware that their placement preferences might not
    /// always be followed and must be prepared to handle the case where the
    /// item is placed at a different position by the compositor.
    ///
    /// Once an item has been mapped, a change to its preferred placement can
    /// still be requested and should be applied, but must not be followed
    /// by the compositor while the user is interacting with the affected item
    /// surface (e.g. clicking  dragging within the window, or resizing it).
    ///
    /// After a call to this request, a 'position' event must be emitted with the
    /// item's new actual position.
    /// If the current item has no zone associated with it, a 'position_failed'
    /// event must be emitted.
    /// If the compositor did not move the item at all, not even with sanitized
    /// values, a 'position_failed' event must be emitted as well.
    ///
    /// # Arguments
    ///
    /// - `x`: x position relative to zone
    /// - `y`: y position relative to zone
    #[inline]
    pub fn try_send_set_position(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xx_zone_item_v1#{}.set_position(x: {}, y: {})\n", id, arg0, arg1);
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

    /// set a preferred item surface position
    ///
    /// Request a preferred position (x, y) for the specified item
    /// surface to be placed at, relative to its associated zone.
    /// This state is double-buffered and is applied on the next
    /// wl_surface.commit of the surface represented by 'item'.
    ///
    /// X and Y coordinates are relative to the zone this item is associated
    /// with, and must not be larger than the dimensions set by the zone size.
    /// They may be smaller than zero, if the item's top-left edge is to be
    /// placed beyond the zone's top-left sides, but clients should expect the
    /// compositor to more aggressively sanitize the coordinate values in that
    /// case.
    /// If a coordinate exceeds the zone's maximum bounds, the compositor must
    /// sanitize it to more appropriate values (e.g. by clamping the values to
    /// the maximum size).
    /// For infinite zones, the client may pick any coordinate.
    ///
    /// Compositors implementing this protocol should try to place an item
    /// at the requested coordinates relative to the item's zone, unless doing
    /// so is not allowed by compositor policy (because e.g. the user has set
    /// custom rules for the surface represented by the respective item, the
    /// surface overlaps with a protected shell component, session management
    /// has loaded previous surface positions or the placement request would
    /// send the item out of bounds).
    ///
    /// Clients should be aware that their placement preferences might not
    /// always be followed and must be prepared to handle the case where the
    /// item is placed at a different position by the compositor.
    ///
    /// Once an item has been mapped, a change to its preferred placement can
    /// still be requested and should be applied, but must not be followed
    /// by the compositor while the user is interacting with the affected item
    /// surface (e.g. clicking  dragging within the window, or resizing it).
    ///
    /// After a call to this request, a 'position' event must be emitted with the
    /// item's new actual position.
    /// If the current item has no zone associated with it, a 'position_failed'
    /// event must be emitted.
    /// If the compositor did not move the item at all, not even with sanitized
    /// values, a 'position_failed' event must be emitted as well.
    ///
    /// # Arguments
    ///
    /// - `x`: x position relative to zone
    /// - `y`: y position relative to zone
    #[inline]
    pub fn send_set_position(
        &self,
        x: i32,
        y: i32,
    ) {
        let res = self.try_send_set_position(
            x,
            y,
        );
        if let Err(e) = res {
            log_send("xx_zone_item_v1.set_position", &e);
        }
    }

    /// Since when the position message is available.
    pub const MSG__POSITION__SINCE: u32 = 1;

    /// notify about the position of an item
    ///
    /// This event notifies the client of the current position (x, y) of
    /// the item relative to its zone.
    /// Coordinates are relative to the zone this item belongs to, and only
    /// valid within it.
    /// Negative coordinates are possible, if the user has moved an item
    /// surface beyond the zone's top-left boundary.
    ///
    /// This event is sent in response to a 'set_position' request,
    /// or if the item position has been changed by other means
    /// (e.g. user interaction or compositor involvement).
    ///
    /// This event can only be emitted if the item is currently associated
    /// with a zone.
    ///
    /// # Arguments
    ///
    /// - `x`: current x position relative to zone
    /// - `y`: current y position relative to zone
    #[inline]
    pub fn try_send_position(
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
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xx_zone_item_v1#{}.position(x: {}, y: {})\n", client_id, id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1);
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
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// notify about the position of an item
    ///
    /// This event notifies the client of the current position (x, y) of
    /// the item relative to its zone.
    /// Coordinates are relative to the zone this item belongs to, and only
    /// valid within it.
    /// Negative coordinates are possible, if the user has moved an item
    /// surface beyond the zone's top-left boundary.
    ///
    /// This event is sent in response to a 'set_position' request,
    /// or if the item position has been changed by other means
    /// (e.g. user interaction or compositor involvement).
    ///
    /// This event can only be emitted if the item is currently associated
    /// with a zone.
    ///
    /// # Arguments
    ///
    /// - `x`: current x position relative to zone
    /// - `y`: current y position relative to zone
    #[inline]
    pub fn send_position(
        &self,
        x: i32,
        y: i32,
    ) {
        let res = self.try_send_position(
            x,
            y,
        );
        if let Err(e) = res {
            log_send("xx_zone_item_v1.position", &e);
        }
    }

    /// Since when the position_failed message is available.
    pub const MSG__POSITION_FAILED__SINCE: u32 = 1;

    /// a set_position request has failed
    ///
    /// The compositor was unable to set the position of this item entirely,
    /// and could not even find sanitized coordinates to place the item at
    /// instead.
    ///
    /// This event will also be emitted if 'set_position' was called while the
    /// item had no zone associated with it.
    #[inline]
    pub fn try_send_position_failed(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xx_zone_item_v1#{}.position_failed()\n", client_id, id);
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

    /// a set_position request has failed
    ///
    /// The compositor was unable to set the position of this item entirely,
    /// and could not even find sanitized coordinates to place the item at
    /// instead.
    ///
    /// This event will also be emitted if 'set_position' was called while the
    /// item had no zone associated with it.
    #[inline]
    pub fn send_position_failed(
        &self,
    ) {
        let res = self.try_send_position_failed(
        );
        if let Err(e) = res {
            log_send("xx_zone_item_v1.position_failed", &e);
        }
    }

    /// Since when the closed message is available.
    pub const MSG__CLOSED__SINCE: u32 = 1;

    /// the underlying surface has been destroyed
    ///
    /// This event indicates that the surface wrapped by this
    /// zone item has been destroyed.
    ///
    /// The 'xx_zone_item_v1' object becomes inert and the client should
    /// destroy it. Any requests made on an inert zone item must be silently
    /// ignored by the compositor, and no further events will be sent for this
    /// item.
    ///
    /// If the item was associated with a zone when this event is sent,
    /// the compositor must also emit 'item_left' on the respective zone
    /// before sending this event.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xx_zone_item_v1#{}.closed()\n", client_id, id);
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
            3,
        ]);
        Ok(())
    }

    /// the underlying surface has been destroyed
    ///
    /// This event indicates that the surface wrapped by this
    /// zone item has been destroyed.
    ///
    /// The 'xx_zone_item_v1' object becomes inert and the client should
    /// destroy it. Any requests made on an inert zone item must be silently
    /// ignored by the compositor, and no further events will be sent for this
    /// item.
    ///
    /// If the item was associated with a zone when this event is sent,
    /// the compositor must also emit 'item_left' on the respective zone
    /// before sending this event.
    #[inline]
    pub fn send_closed(
        &self,
    ) {
        let res = self.try_send_closed(
        );
        if let Err(e) = res {
            log_send("xx_zone_item_v1.closed", &e);
        }
    }
}

/// A message handler for [`XxZoneItemV1`] proxies.
pub trait XxZoneItemV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<XxZoneItemV1>) {
        slf.core.delete_id();
    }

    /// delete this object
    ///
    /// Destroys the zone item. This request may be sent at any time by the
    /// client.
    /// By destroying the object, the respective item surface remains at its
    /// last position, but its association with its zone is lost.
    /// This will also cause it to lose any other attached state described by
    /// this protocol.
    ///
    /// If the item was associated with a zone when this request is sent,
    /// the compositor must emit 'item_left' on the respective zone, unless
    /// it had already been emitted before a 'closed' event.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<XxZoneItemV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("xx_zone_item_v1.destroy", &e);
        }
    }

    /// the extents of the frame bordering the item
    ///
    /// The 'frame_extents' event describes the current extents of the frame
    /// bordering the item's content area.
    ///
    /// This event is sent immediately after the item joins a zone, or if
    /// the item frame extents have been changed by other means (e.g. toggled
    /// by a client request, or compositor involvement). The dimensions are in
    /// the same coordinate space as the item's zone (the surface coordinate
    /// space).
    ///
    /// This event must be followed by a 'position' event, even if the item's
    /// coordinates did not change as a result of the frame extents changing.
    ///
    /// If the item has no associated frame, the event should still be sent,
    /// but extents must be set to zero.
    ///
    /// This event can only be emitted if the item is currently associated
    /// with a zone.
    ///
    /// # Arguments
    ///
    /// - `top`: current height of the frame bordering the top of the item
    /// - `bottom`: current height of the frame bordering the bottom of the item
    /// - `left`: current width of the frame bordering the left of the item
    /// - `right`: current width of the frame bordering the right of the item
    #[inline]
    fn handle_frame_extents(
        &mut self,
        slf: &Rc<XxZoneItemV1>,
        top: i32,
        bottom: i32,
        left: i32,
        right: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_frame_extents(
            top,
            bottom,
            left,
            right,
        );
        if let Err(e) = res {
            log_forward("xx_zone_item_v1.frame_extents", &e);
        }
    }

    /// set a preferred item surface position
    ///
    /// Request a preferred position (x, y) for the specified item
    /// surface to be placed at, relative to its associated zone.
    /// This state is double-buffered and is applied on the next
    /// wl_surface.commit of the surface represented by 'item'.
    ///
    /// X and Y coordinates are relative to the zone this item is associated
    /// with, and must not be larger than the dimensions set by the zone size.
    /// They may be smaller than zero, if the item's top-left edge is to be
    /// placed beyond the zone's top-left sides, but clients should expect the
    /// compositor to more aggressively sanitize the coordinate values in that
    /// case.
    /// If a coordinate exceeds the zone's maximum bounds, the compositor must
    /// sanitize it to more appropriate values (e.g. by clamping the values to
    /// the maximum size).
    /// For infinite zones, the client may pick any coordinate.
    ///
    /// Compositors implementing this protocol should try to place an item
    /// at the requested coordinates relative to the item's zone, unless doing
    /// so is not allowed by compositor policy (because e.g. the user has set
    /// custom rules for the surface represented by the respective item, the
    /// surface overlaps with a protected shell component, session management
    /// has loaded previous surface positions or the placement request would
    /// send the item out of bounds).
    ///
    /// Clients should be aware that their placement preferences might not
    /// always be followed and must be prepared to handle the case where the
    /// item is placed at a different position by the compositor.
    ///
    /// Once an item has been mapped, a change to its preferred placement can
    /// still be requested and should be applied, but must not be followed
    /// by the compositor while the user is interacting with the affected item
    /// surface (e.g. clicking  dragging within the window, or resizing it).
    ///
    /// After a call to this request, a 'position' event must be emitted with the
    /// item's new actual position.
    /// If the current item has no zone associated with it, a 'position_failed'
    /// event must be emitted.
    /// If the compositor did not move the item at all, not even with sanitized
    /// values, a 'position_failed' event must be emitted as well.
    ///
    /// # Arguments
    ///
    /// - `x`: x position relative to zone
    /// - `y`: y position relative to zone
    #[inline]
    fn handle_set_position(
        &mut self,
        slf: &Rc<XxZoneItemV1>,
        x: i32,
        y: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_position(
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("xx_zone_item_v1.set_position", &e);
        }
    }

    /// notify about the position of an item
    ///
    /// This event notifies the client of the current position (x, y) of
    /// the item relative to its zone.
    /// Coordinates are relative to the zone this item belongs to, and only
    /// valid within it.
    /// Negative coordinates are possible, if the user has moved an item
    /// surface beyond the zone's top-left boundary.
    ///
    /// This event is sent in response to a 'set_position' request,
    /// or if the item position has been changed by other means
    /// (e.g. user interaction or compositor involvement).
    ///
    /// This event can only be emitted if the item is currently associated
    /// with a zone.
    ///
    /// # Arguments
    ///
    /// - `x`: current x position relative to zone
    /// - `y`: current y position relative to zone
    #[inline]
    fn handle_position(
        &mut self,
        slf: &Rc<XxZoneItemV1>,
        x: i32,
        y: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_position(
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("xx_zone_item_v1.position", &e);
        }
    }

    /// a set_position request has failed
    ///
    /// The compositor was unable to set the position of this item entirely,
    /// and could not even find sanitized coordinates to place the item at
    /// instead.
    ///
    /// This event will also be emitted if 'set_position' was called while the
    /// item had no zone associated with it.
    #[inline]
    fn handle_position_failed(
        &mut self,
        slf: &Rc<XxZoneItemV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_position_failed(
        );
        if let Err(e) = res {
            log_forward("xx_zone_item_v1.position_failed", &e);
        }
    }

    /// the underlying surface has been destroyed
    ///
    /// This event indicates that the surface wrapped by this
    /// zone item has been destroyed.
    ///
    /// The 'xx_zone_item_v1' object becomes inert and the client should
    /// destroy it. Any requests made on an inert zone item must be silently
    /// ignored by the compositor, and no further events will be sent for this
    /// item.
    ///
    /// If the item was associated with a zone when this event is sent,
    /// the compositor must also emit 'item_left' on the respective zone
    /// before sending this event.
    #[inline]
    fn handle_closed(
        &mut self,
        slf: &Rc<XxZoneItemV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_closed(
        );
        if let Err(e) = res {
            log_forward("xx_zone_item_v1.closed", &e);
        }
    }
}

impl ObjectPrivate for XxZoneItemV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::XxZoneItemV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xx_zone_item_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xx_zone_item_v1#{}.set_position(x: {}, y: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_position(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_position(&self, arg0, arg1);
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
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xx_zone_item_v1#{}.frame_extents(top: {}, bottom: {}, left: {}, right: {})\n", id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_frame_extents(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_frame_extents(&self, arg0, arg1, arg2, arg3);
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
                    fn log(state: &State, id: u32, arg0: i32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xx_zone_item_v1#{}.position(x: {}, y: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_position(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_position(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xx_zone_item_v1#{}.position_failed()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_position_failed(&self);
                } else {
                    DefaultHandler.handle_position_failed(&self);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xx_zone_item_v1#{}.closed()\n", id);
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
            0 => "destroy",
            1 => "set_position",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "frame_extents",
            1 => "position",
            2 => "position_failed",
            3 => "closed",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for XxZoneItemV1 {
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

