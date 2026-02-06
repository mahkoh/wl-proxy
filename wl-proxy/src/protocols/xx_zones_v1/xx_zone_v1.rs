//! area for a client in which it can set window positioning preferences
//!
//! An 'xx_zone' describes a display area provided by the compositor in
//! which a client can place windows and move them around.
//!
//! A zone's area could, for example, correspond to the space usable for
//! placing windows on a specific output (space without panels or other
//! restricted elements) or it could be an area of the output the compositor
//! has specifically chosen for a client to place its surfaces in.
//!
//! Clients should make no assumptions about how a zone is presented to the
//! user (e.g. compositors may visually distinguish what makes up a zone).
//!
//! Items are added to a zone as 'xx_zone_item' objects.
//!
//! All item surface position coordinates (x, y) are relative to the selected
//! zone.
//! They are using the 'size' of the respective zone as coordinate system,
//! with (0, 0) being in the top left corner.
//!
//! If a zone item is moved out of the top/left boundaries of the zone by
//! user interaction, its coordinates must become negative, relative to the
//! zone's top-left coordinate origin. A client may position an item at negative
//! coordinates.
//!
//! The compositor must ensure that any item positioned by the client is
//! visible and accessible to the user, and is not moved into invisible space
//! outside of a zone.
//! Positioning requests may be rejected or altered by the compositor, depending
//! on its policy.
//!
//! The absolute position of the zone within the compositor's coordinate space
//! is opaque to the client and the compositor may move the entire zone without
//! the client noticing it. A zone may also be arbitrarily resized, in which
//! case the respective 'size' event must be emitted again to notify the client.
//!
//! A zone is always tied to an output and does not extend beyond it.
//!
//! A zone may be "invalid". An invalid zone is created with a negative
//! 'size' and must not be used for item arrangement.
//!
//! Upon creation the compositor must emit 'size' and 'handle' events for the
//! newly created 'xx_zone', followed by 'done'.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A xx_zone_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XxZoneV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn XxZoneV1Handler>,
}

struct DefaultHandler;

impl XxZoneV1Handler for DefaultHandler { }

impl ConcreteObject for XxZoneV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::XxZoneV1;
    const INTERFACE_NAME: &str = "xx_zone_v1";
}

impl XxZoneV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl XxZoneV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn XxZoneV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XxZoneV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XxZoneV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XxZoneV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the xx_zone object
    ///
    /// Using this request a client can tell the compositor that it is not
    /// going to use the 'xx_zone' object anymore.
    /// The zone itself must only be destroyed if no other client
    /// is currently referencing it, so this request may only destroy the
    /// object reference owned by the client.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xx_zone_v1#{}.destroy()\n", id);
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

    /// destroy the xx_zone object
    ///
    /// Using this request a client can tell the compositor that it is not
    /// going to use the 'xx_zone' object anymore.
    /// The zone itself must only be destroyed if no other client
    /// is currently referencing it, so this request may only destroy the
    /// object reference owned by the client.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("xx_zone_v1.destroy", &e);
        }
    }

    /// Since when the size message is available.
    pub const MSG__SIZE__SINCE: u32 = 1;

    /// size of the zone
    ///
    /// The 'size' event describes the size of this zone.
    ///
    /// It is a rectangle with its origin in the top-left corner, using
    /// the surface coordinate space (device pixels divided by the scaling
    /// factor of the output this zone is attached to).
    ///
    /// If a width or height value is zero, the zone is infinite
    /// in that direction.
    ///
    /// If the width and height values are negative, the zone is considered
    /// "invalid" and must not be used.
    /// A size event declaring the zone invalid may only be emitted immediately
    /// after the zone was created.
    /// A zone must not become invalid at a later time by sending a negative
    /// 'size' after the zone has been established.
    ///
    /// The 'size' event is sent immediately after creating an 'xx_zone_v1',
    /// and whenever the size of the zone changes. A zone size can change at
    /// any time, for any reason, for example due to output size or scaling
    /// changes, or by compositor policy.
    ///
    /// Upon subsequent emissions of 'size' after 'xx_zone' has already
    /// been created, the 'done' event does not have to be sent again.
    ///
    /// # Arguments
    ///
    /// - `width`: zone width in logical pixels
    /// - `height`: zone height in logical pixels
    #[inline]
    pub fn try_send_size(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xx_zone_v1#{}.size(width: {}, height: {})\n", client_id, id, arg0, arg1);
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
            0,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// size of the zone
    ///
    /// The 'size' event describes the size of this zone.
    ///
    /// It is a rectangle with its origin in the top-left corner, using
    /// the surface coordinate space (device pixels divided by the scaling
    /// factor of the output this zone is attached to).
    ///
    /// If a width or height value is zero, the zone is infinite
    /// in that direction.
    ///
    /// If the width and height values are negative, the zone is considered
    /// "invalid" and must not be used.
    /// A size event declaring the zone invalid may only be emitted immediately
    /// after the zone was created.
    /// A zone must not become invalid at a later time by sending a negative
    /// 'size' after the zone has been established.
    ///
    /// The 'size' event is sent immediately after creating an 'xx_zone_v1',
    /// and whenever the size of the zone changes. A zone size can change at
    /// any time, for any reason, for example due to output size or scaling
    /// changes, or by compositor policy.
    ///
    /// Upon subsequent emissions of 'size' after 'xx_zone' has already
    /// been created, the 'done' event does not have to be sent again.
    ///
    /// # Arguments
    ///
    /// - `width`: zone width in logical pixels
    /// - `height`: zone height in logical pixels
    #[inline]
    pub fn send_size(
        &self,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_size(
            width,
            height,
        );
        if let Err(e) = res {
            log_send("xx_zone_v1.size", &e);
        }
    }

    /// Since when the handle message is available.
    pub const MSG__HANDLE__SINCE: u32 = 1;

    /// the zone handle
    ///
    /// The handle event provides the unique handle of this zone.
    /// The handle may be shared with any client, which then can use it to
    /// join this client's zone by calling
    /// 'xx_zone_manager.get_zone_from_handle'.
    ///
    /// This event must only be emitted once after the zone was created.
    /// If this zone is invalid, the handle must be an empty string.
    ///
    /// # Arguments
    ///
    /// - `handle`: the exported zone handle
    #[inline]
    pub fn try_send_handle(
        &self,
        handle: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            handle,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xx_zone_v1#{}.handle(handle: {:?})\n", client_id, id, arg0);
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
            1,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// the zone handle
    ///
    /// The handle event provides the unique handle of this zone.
    /// The handle may be shared with any client, which then can use it to
    /// join this client's zone by calling
    /// 'xx_zone_manager.get_zone_from_handle'.
    ///
    /// This event must only be emitted once after the zone was created.
    /// If this zone is invalid, the handle must be an empty string.
    ///
    /// # Arguments
    ///
    /// - `handle`: the exported zone handle
    #[inline]
    pub fn send_handle(
        &self,
        handle: &str,
    ) {
        let res = self.try_send_handle(
            handle,
        );
        if let Err(e) = res {
            log_send("xx_zone_v1.handle", &e);
        }
    }

    /// Since when the done message is available.
    pub const MSG__DONE__SINCE: u32 = 1;

    /// all information about the zone has been sent
    ///
    /// This event is sent after all other properties (size, handle) of an
    /// 'xx_zone' have been sent.
    ///
    /// This allows changes to the xx_zone properties to be seen as
    /// atomic, even if they happen via multiple events.
    #[inline]
    pub fn try_send_done(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xx_zone_v1#{}.done()\n", client_id, id);
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

    /// all information about the zone has been sent
    ///
    /// This event is sent after all other properties (size, handle) of an
    /// 'xx_zone' have been sent.
    ///
    /// This allows changes to the xx_zone properties to be seen as
    /// atomic, even if they happen via multiple events.
    #[inline]
    pub fn send_done(
        &self,
    ) {
        let res = self.try_send_done(
        );
        if let Err(e) = res {
            log_send("xx_zone_v1.done", &e);
        }
    }

    /// Since when the add_item message is available.
    pub const MSG__ADD_ITEM__SINCE: u32 = 1;

    /// associate an item with this zone
    ///
    /// Make 'item' a member of this zone.
    /// This state is double-buffered and is applied on the next
    /// 'wl_surface.commit' of the surface represented by 'item'.
    ///
    /// This request associates an item with this zone.
    /// If this request is called on an item that already has a zone
    /// association with a different zone, the item must leave its old zone
    /// (with 'item_left' being emitted on its old zone) and will instead
    /// be associated with this zone.
    ///
    /// Upon receiving this request and if the target zone is allowed for 'item',
    /// a compositor must emit 'item_entered' to confirm the zone association.
    /// It must even emit this event if the item was already associated with this
    /// zone before.
    ///
    /// The compositor must move the surface represented by 'item' into the
    /// boundary of this zone upon receiving this request and accepting it
    /// (either by extending the zone size, or by moving the item surface).
    ///
    /// If the compositor does not allow the item to switch zone associations,
    /// and wants it to remain in its previous zone, it must emit
    /// 'item_blocked' instead.
    /// Compositors might want to prevent zone associations if they
    /// perform specialized window management (e.g. autotiling) that would
    /// make clients moving items between certain zones undesirable.
    ///
    /// Once the 'item' is added to its zone, the compositor must first send
    /// a 'frame_extents' event on the item, followed by an initial 'position'
    /// event with the item's current position.
    /// The compositor must then send 'position' events when the position
    /// of the item in its zone is changed, for as long as the item is
    /// associated with a zone.
    ///
    /// If the zone is invalid, an 'invalid' error must be raised and the item
    /// must not be associated with the invalid zone.
    /// If the referenced item is inert (its underlying surface has been
    /// destroyed), the request must be silently ignored.
    ///
    /// # Arguments
    ///
    /// - `item`: the zone item
    #[inline]
    pub fn try_send_add_item(
        &self,
        item: &Rc<XxZoneItemV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            item,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("item"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xx_zone_v1#{}.add_item(item: xx_zone_item_v1#{})\n", id, arg0);
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

    /// associate an item with this zone
    ///
    /// Make 'item' a member of this zone.
    /// This state is double-buffered and is applied on the next
    /// 'wl_surface.commit' of the surface represented by 'item'.
    ///
    /// This request associates an item with this zone.
    /// If this request is called on an item that already has a zone
    /// association with a different zone, the item must leave its old zone
    /// (with 'item_left' being emitted on its old zone) and will instead
    /// be associated with this zone.
    ///
    /// Upon receiving this request and if the target zone is allowed for 'item',
    /// a compositor must emit 'item_entered' to confirm the zone association.
    /// It must even emit this event if the item was already associated with this
    /// zone before.
    ///
    /// The compositor must move the surface represented by 'item' into the
    /// boundary of this zone upon receiving this request and accepting it
    /// (either by extending the zone size, or by moving the item surface).
    ///
    /// If the compositor does not allow the item to switch zone associations,
    /// and wants it to remain in its previous zone, it must emit
    /// 'item_blocked' instead.
    /// Compositors might want to prevent zone associations if they
    /// perform specialized window management (e.g. autotiling) that would
    /// make clients moving items between certain zones undesirable.
    ///
    /// Once the 'item' is added to its zone, the compositor must first send
    /// a 'frame_extents' event on the item, followed by an initial 'position'
    /// event with the item's current position.
    /// The compositor must then send 'position' events when the position
    /// of the item in its zone is changed, for as long as the item is
    /// associated with a zone.
    ///
    /// If the zone is invalid, an 'invalid' error must be raised and the item
    /// must not be associated with the invalid zone.
    /// If the referenced item is inert (its underlying surface has been
    /// destroyed), the request must be silently ignored.
    ///
    /// # Arguments
    ///
    /// - `item`: the zone item
    #[inline]
    pub fn send_add_item(
        &self,
        item: &Rc<XxZoneItemV1>,
    ) {
        let res = self.try_send_add_item(
            item,
        );
        if let Err(e) = res {
            log_send("xx_zone_v1.add_item", &e);
        }
    }

    /// Since when the remove_item message is available.
    pub const MSG__REMOVE_ITEM__SINCE: u32 = 1;

    /// disassociate an item from this zone
    ///
    /// Remove 'item' as a member of this zone.
    /// This state is double-buffered and is applied on the next
    /// 'wl_surface.commit' of the surface represented by 'item'.
    ///
    /// This request removes the item from this zone explicitly,
    /// making the client unable to retrieve coordinates again.
    ///
    /// Upon receiving this request, the compositor should not change the
    /// item surface position on screen, and must emit 'item_left' to confirm
    /// the item's removal. It must even emit this event if the
    /// item was never associated with this zone.
    ///
    /// If the referenced item is inert (its underlying surface has been
    /// destroyed), the request must be silently ignored.
    ///
    /// # Arguments
    ///
    /// - `item`: the zone item
    #[inline]
    pub fn try_send_remove_item(
        &self,
        item: &Rc<XxZoneItemV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            item,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("item"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xx_zone_v1#{}.remove_item(item: xx_zone_item_v1#{})\n", id, arg0);
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

    /// disassociate an item from this zone
    ///
    /// Remove 'item' as a member of this zone.
    /// This state is double-buffered and is applied on the next
    /// 'wl_surface.commit' of the surface represented by 'item'.
    ///
    /// This request removes the item from this zone explicitly,
    /// making the client unable to retrieve coordinates again.
    ///
    /// Upon receiving this request, the compositor should not change the
    /// item surface position on screen, and must emit 'item_left' to confirm
    /// the item's removal. It must even emit this event if the
    /// item was never associated with this zone.
    ///
    /// If the referenced item is inert (its underlying surface has been
    /// destroyed), the request must be silently ignored.
    ///
    /// # Arguments
    ///
    /// - `item`: the zone item
    #[inline]
    pub fn send_remove_item(
        &self,
        item: &Rc<XxZoneItemV1>,
    ) {
        let res = self.try_send_remove_item(
            item,
        );
        if let Err(e) = res {
            log_send("xx_zone_v1.remove_item", &e);
        }
    }

    /// Since when the item_blocked message is available.
    pub const MSG__ITEM_BLOCKED__SINCE: u32 = 1;

    /// an item could not be associated with this zone
    ///
    /// This event notifies the client that an item was prevented from
    /// joining this zone.
    ///
    /// It is emitted as a response to 'add_item' if the compositor did not
    /// allow the item to join this particular zone.
    ///
    /// # Arguments
    ///
    /// - `item`: the item that was prevented from joining this zone
    #[inline]
    pub fn try_send_item_blocked(
        &self,
        item: &Rc<XxZoneItemV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            item,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("item", client.endpoint.id)));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xx_zone_v1#{}.item_blocked(item: xx_zone_item_v1#{})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// an item could not be associated with this zone
    ///
    /// This event notifies the client that an item was prevented from
    /// joining this zone.
    ///
    /// It is emitted as a response to 'add_item' if the compositor did not
    /// allow the item to join this particular zone.
    ///
    /// # Arguments
    ///
    /// - `item`: the item that was prevented from joining this zone
    #[inline]
    pub fn send_item_blocked(
        &self,
        item: &Rc<XxZoneItemV1>,
    ) {
        let res = self.try_send_item_blocked(
            item,
        );
        if let Err(e) = res {
            log_send("xx_zone_v1.item_blocked", &e);
        }
    }

    /// Since when the item_entered message is available.
    pub const MSG__ITEM_ENTERED__SINCE: u32 = 1;

    /// notify about an item having joined this zone
    ///
    /// This event notifies the client of an item joining this zone.
    ///
    /// It is emitted as a response to 'add_item' or if the compositor
    /// automatically had the item surface (re)join an existing zone.
    ///
    /// # Arguments
    ///
    /// - `item`: the item that has joined the zone
    #[inline]
    pub fn try_send_item_entered(
        &self,
        item: &Rc<XxZoneItemV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            item,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("item", client.endpoint.id)));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xx_zone_v1#{}.item_entered(item: xx_zone_item_v1#{})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0_id);
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
            4,
            arg0_id,
        ]);
        Ok(())
    }

    /// notify about an item having joined this zone
    ///
    /// This event notifies the client of an item joining this zone.
    ///
    /// It is emitted as a response to 'add_item' or if the compositor
    /// automatically had the item surface (re)join an existing zone.
    ///
    /// # Arguments
    ///
    /// - `item`: the item that has joined the zone
    #[inline]
    pub fn send_item_entered(
        &self,
        item: &Rc<XxZoneItemV1>,
    ) {
        let res = self.try_send_item_entered(
            item,
        );
        if let Err(e) = res {
            log_send("xx_zone_v1.item_entered", &e);
        }
    }

    /// Since when the item_left message is available.
    pub const MSG__ITEM_LEFT__SINCE: u32 = 1;

    /// notify about an item having left this zone
    ///
    /// This event notifies the client of an item leaving this zone, and
    /// therefore the client will no longer receive updated coordinates
    /// or frame extents for this item.
    /// If the client still wishes to adjust the item surface coordinates, it
    /// may associate the item with a zone again by calling 'add_item'.
    ///
    /// This event is emitted for example if the user moved an item surface out
    /// of a smaller zone's boundaries, or onto a different screen where the
    /// previous zone can not expand to. It is also emitted in response to
    /// explicitly removing an item via 'remove_item'.
    ///
    /// # Arguments
    ///
    /// - `item`: the item that has left the zone
    #[inline]
    pub fn try_send_item_left(
        &self,
        item: &Rc<XxZoneItemV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            item,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("item", client.endpoint.id)));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xx_zone_v1#{}.item_left(item: xx_zone_item_v1#{})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0_id);
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
            5,
            arg0_id,
        ]);
        Ok(())
    }

    /// notify about an item having left this zone
    ///
    /// This event notifies the client of an item leaving this zone, and
    /// therefore the client will no longer receive updated coordinates
    /// or frame extents for this item.
    /// If the client still wishes to adjust the item surface coordinates, it
    /// may associate the item with a zone again by calling 'add_item'.
    ///
    /// This event is emitted for example if the user moved an item surface out
    /// of a smaller zone's boundaries, or onto a different screen where the
    /// previous zone can not expand to. It is also emitted in response to
    /// explicitly removing an item via 'remove_item'.
    ///
    /// # Arguments
    ///
    /// - `item`: the item that has left the zone
    #[inline]
    pub fn send_item_left(
        &self,
        item: &Rc<XxZoneItemV1>,
    ) {
        let res = self.try_send_item_left(
            item,
        );
        if let Err(e) = res {
            log_send("xx_zone_v1.item_left", &e);
        }
    }
}

/// A message handler for [`XxZoneV1`] proxies.
pub trait XxZoneV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<XxZoneV1>) {
        slf.core.delete_id();
    }

    /// destroy the xx_zone object
    ///
    /// Using this request a client can tell the compositor that it is not
    /// going to use the 'xx_zone' object anymore.
    /// The zone itself must only be destroyed if no other client
    /// is currently referencing it, so this request may only destroy the
    /// object reference owned by the client.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<XxZoneV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("xx_zone_v1.destroy", &e);
        }
    }

    /// size of the zone
    ///
    /// The 'size' event describes the size of this zone.
    ///
    /// It is a rectangle with its origin in the top-left corner, using
    /// the surface coordinate space (device pixels divided by the scaling
    /// factor of the output this zone is attached to).
    ///
    /// If a width or height value is zero, the zone is infinite
    /// in that direction.
    ///
    /// If the width and height values are negative, the zone is considered
    /// "invalid" and must not be used.
    /// A size event declaring the zone invalid may only be emitted immediately
    /// after the zone was created.
    /// A zone must not become invalid at a later time by sending a negative
    /// 'size' after the zone has been established.
    ///
    /// The 'size' event is sent immediately after creating an 'xx_zone_v1',
    /// and whenever the size of the zone changes. A zone size can change at
    /// any time, for any reason, for example due to output size or scaling
    /// changes, or by compositor policy.
    ///
    /// Upon subsequent emissions of 'size' after 'xx_zone' has already
    /// been created, the 'done' event does not have to be sent again.
    ///
    /// # Arguments
    ///
    /// - `width`: zone width in logical pixels
    /// - `height`: zone height in logical pixels
    #[inline]
    fn handle_size(
        &mut self,
        slf: &Rc<XxZoneV1>,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_size(
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("xx_zone_v1.size", &e);
        }
    }

    /// the zone handle
    ///
    /// The handle event provides the unique handle of this zone.
    /// The handle may be shared with any client, which then can use it to
    /// join this client's zone by calling
    /// 'xx_zone_manager.get_zone_from_handle'.
    ///
    /// This event must only be emitted once after the zone was created.
    /// If this zone is invalid, the handle must be an empty string.
    ///
    /// # Arguments
    ///
    /// - `handle`: the exported zone handle
    #[inline]
    fn handle_handle(
        &mut self,
        slf: &Rc<XxZoneV1>,
        handle: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_handle(
            handle,
        );
        if let Err(e) = res {
            log_forward("xx_zone_v1.handle", &e);
        }
    }

    /// all information about the zone has been sent
    ///
    /// This event is sent after all other properties (size, handle) of an
    /// 'xx_zone' have been sent.
    ///
    /// This allows changes to the xx_zone properties to be seen as
    /// atomic, even if they happen via multiple events.
    #[inline]
    fn handle_done(
        &mut self,
        slf: &Rc<XxZoneV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_done(
        );
        if let Err(e) = res {
            log_forward("xx_zone_v1.done", &e);
        }
    }

    /// associate an item with this zone
    ///
    /// Make 'item' a member of this zone.
    /// This state is double-buffered and is applied on the next
    /// 'wl_surface.commit' of the surface represented by 'item'.
    ///
    /// This request associates an item with this zone.
    /// If this request is called on an item that already has a zone
    /// association with a different zone, the item must leave its old zone
    /// (with 'item_left' being emitted on its old zone) and will instead
    /// be associated with this zone.
    ///
    /// Upon receiving this request and if the target zone is allowed for 'item',
    /// a compositor must emit 'item_entered' to confirm the zone association.
    /// It must even emit this event if the item was already associated with this
    /// zone before.
    ///
    /// The compositor must move the surface represented by 'item' into the
    /// boundary of this zone upon receiving this request and accepting it
    /// (either by extending the zone size, or by moving the item surface).
    ///
    /// If the compositor does not allow the item to switch zone associations,
    /// and wants it to remain in its previous zone, it must emit
    /// 'item_blocked' instead.
    /// Compositors might want to prevent zone associations if they
    /// perform specialized window management (e.g. autotiling) that would
    /// make clients moving items between certain zones undesirable.
    ///
    /// Once the 'item' is added to its zone, the compositor must first send
    /// a 'frame_extents' event on the item, followed by an initial 'position'
    /// event with the item's current position.
    /// The compositor must then send 'position' events when the position
    /// of the item in its zone is changed, for as long as the item is
    /// associated with a zone.
    ///
    /// If the zone is invalid, an 'invalid' error must be raised and the item
    /// must not be associated with the invalid zone.
    /// If the referenced item is inert (its underlying surface has been
    /// destroyed), the request must be silently ignored.
    ///
    /// # Arguments
    ///
    /// - `item`: the zone item
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_add_item(
        &mut self,
        slf: &Rc<XxZoneV1>,
        item: &Rc<XxZoneItemV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_add_item(
            item,
        );
        if let Err(e) = res {
            log_forward("xx_zone_v1.add_item", &e);
        }
    }

    /// disassociate an item from this zone
    ///
    /// Remove 'item' as a member of this zone.
    /// This state is double-buffered and is applied on the next
    /// 'wl_surface.commit' of the surface represented by 'item'.
    ///
    /// This request removes the item from this zone explicitly,
    /// making the client unable to retrieve coordinates again.
    ///
    /// Upon receiving this request, the compositor should not change the
    /// item surface position on screen, and must emit 'item_left' to confirm
    /// the item's removal. It must even emit this event if the
    /// item was never associated with this zone.
    ///
    /// If the referenced item is inert (its underlying surface has been
    /// destroyed), the request must be silently ignored.
    ///
    /// # Arguments
    ///
    /// - `item`: the zone item
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_remove_item(
        &mut self,
        slf: &Rc<XxZoneV1>,
        item: &Rc<XxZoneItemV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_remove_item(
            item,
        );
        if let Err(e) = res {
            log_forward("xx_zone_v1.remove_item", &e);
        }
    }

    /// an item could not be associated with this zone
    ///
    /// This event notifies the client that an item was prevented from
    /// joining this zone.
    ///
    /// It is emitted as a response to 'add_item' if the compositor did not
    /// allow the item to join this particular zone.
    ///
    /// # Arguments
    ///
    /// - `item`: the item that was prevented from joining this zone
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_item_blocked(
        &mut self,
        slf: &Rc<XxZoneV1>,
        item: &Rc<XxZoneItemV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = item.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = slf.try_send_item_blocked(
            item,
        );
        if let Err(e) = res {
            log_forward("xx_zone_v1.item_blocked", &e);
        }
    }

    /// notify about an item having joined this zone
    ///
    /// This event notifies the client of an item joining this zone.
    ///
    /// It is emitted as a response to 'add_item' or if the compositor
    /// automatically had the item surface (re)join an existing zone.
    ///
    /// # Arguments
    ///
    /// - `item`: the item that has joined the zone
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_item_entered(
        &mut self,
        slf: &Rc<XxZoneV1>,
        item: &Rc<XxZoneItemV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = item.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = slf.try_send_item_entered(
            item,
        );
        if let Err(e) = res {
            log_forward("xx_zone_v1.item_entered", &e);
        }
    }

    /// notify about an item having left this zone
    ///
    /// This event notifies the client of an item leaving this zone, and
    /// therefore the client will no longer receive updated coordinates
    /// or frame extents for this item.
    /// If the client still wishes to adjust the item surface coordinates, it
    /// may associate the item with a zone again by calling 'add_item'.
    ///
    /// This event is emitted for example if the user moved an item surface out
    /// of a smaller zone's boundaries, or onto a different screen where the
    /// previous zone can not expand to. It is also emitted in response to
    /// explicitly removing an item via 'remove_item'.
    ///
    /// # Arguments
    ///
    /// - `item`: the item that has left the zone
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_item_left(
        &mut self,
        slf: &Rc<XxZoneV1>,
        item: &Rc<XxZoneItemV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = item.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = slf.try_send_item_left(
            item,
        );
        if let Err(e) = res {
            log_forward("xx_zone_v1.item_left", &e);
        }
    }
}

impl ObjectPrivate for XxZoneV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::XxZoneV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xx_zone_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xx_zone_v1#{}.add_item(item: xx_zone_item_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<XxZoneItemV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("item", o.core().interface, ObjectInterface::XxZoneItemV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_add_item(&self, arg0);
                } else {
                    DefaultHandler.handle_add_item(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xx_zone_v1#{}.remove_item(item: xx_zone_item_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<XxZoneItemV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("item", o.core().interface, ObjectInterface::XxZoneItemV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_remove_item(&self, arg0);
                } else {
                    DefaultHandler.handle_remove_item(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xx_zone_v1#{}.size(width: {}, height: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_size(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_size(&self, arg0, arg1);
                }
            }
            1 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "handle")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xx_zone_v1#{}.handle(handle: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_handle(&self, arg0);
                } else {
                    DefaultHandler.handle_handle(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xx_zone_v1#{}.done()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_done(&self);
                } else {
                    DefaultHandler.handle_done(&self);
                }
            }
            3 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xx_zone_v1#{}.item_blocked(item: xx_zone_item_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = server.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<XxZoneItemV1>() else {
                    let o = server.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("item", o.core().interface, ObjectInterface::XxZoneItemV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_item_blocked(&self, arg0);
                } else {
                    DefaultHandler.handle_item_blocked(&self, arg0);
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
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xx_zone_v1#{}.item_entered(item: xx_zone_item_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = server.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<XxZoneItemV1>() else {
                    let o = server.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("item", o.core().interface, ObjectInterface::XxZoneItemV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_item_entered(&self, arg0);
                } else {
                    DefaultHandler.handle_item_entered(&self, arg0);
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
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xx_zone_v1#{}.item_left(item: xx_zone_item_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = server.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<XxZoneItemV1>() else {
                    let o = server.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("item", o.core().interface, ObjectInterface::XxZoneItemV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_item_left(&self, arg0);
                } else {
                    DefaultHandler.handle_item_left(&self, arg0);
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
            1 => "add_item",
            2 => "remove_item",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "size",
            1 => "handle",
            2 => "done",
            3 => "item_blocked",
            4 => "item_entered",
            5 => "item_left",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for XxZoneV1 {
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

impl XxZoneV1 {
    /// Since when the error.invalid enum variant is available.
    pub const ENM__ERROR_INVALID__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct XxZoneV1Error(pub u32);

impl XxZoneV1Error {
    /// an invalid value has been submitted
    pub const INVALID: Self = Self(0);
}

impl Debug for XxZoneV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID => "INVALID",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
