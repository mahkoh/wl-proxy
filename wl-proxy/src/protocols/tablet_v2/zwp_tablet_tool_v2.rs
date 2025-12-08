//! a physical tablet tool
//!
//! An object that represents a physical tool that has been, or is
//! currently in use with a tablet in this seat. Each zwp_tablet_tool_v2
//! object stays valid until the client destroys it; the compositor
//! reuses the zwp_tablet_tool_v2 object to indicate that the object's
//! respective physical tool has come into proximity of a tablet again.
//!
//! A zwp_tablet_tool_v2 object's relation to a physical tool depends on the
//! tablet's ability to report serial numbers. If the tablet supports
//! this capability, then the object represents a specific physical tool
//! and can be identified even when used on multiple tablets.
//!
//! A tablet tool has a number of static characteristics, e.g. tool type,
//! hardware_serial and capabilities. These capabilities are sent in an
//! event sequence after the zwp_tablet_seat_v2.tool_added event before any
//! actual events from this tool. This initial event sequence is
//! terminated by a zwp_tablet_tool_v2.done event.
//!
//! Tablet tool events are grouped by zwp_tablet_tool_v2.frame events.
//! Any events received before a zwp_tablet_tool_v2.frame event should be
//! considered part of the same hardware state change.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_tablet_tool_v2 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpTabletToolV2 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpTabletToolV2Handler>,
}

struct DefaultHandler;

impl ZwpTabletToolV2Handler for DefaultHandler { }

impl ConcreteObject for ZwpTabletToolV2 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpTabletToolV2;
    const INTERFACE_NAME: &str = "zwp_tablet_tool_v2";
}

impl ZwpTabletToolV2 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpTabletToolV2Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpTabletToolV2Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpTabletToolV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpTabletToolV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpTabletToolV2 {
    /// Since when the set_cursor message is available.
    pub const MSG__SET_CURSOR__SINCE: u32 = 1;

    /// set the tablet tool's surface
    ///
    /// Sets the surface of the cursor used for this tool on the given
    /// tablet. This request only takes effect if the tool is in proximity
    /// of one of the requesting client's surfaces or the surface parameter
    /// is the current pointer surface. If there was a previous surface set
    /// with this request it is replaced. If surface is NULL, the cursor
    /// image is hidden.
    ///
    /// The parameters hotspot_x and hotspot_y define the position of the
    /// pointer surface relative to the pointer location. Its top-left corner
    /// is always at (x, y) - (hotspot_x, hotspot_y), where (x, y) are the
    /// coordinates of the pointer location, in surface-local coordinates.
    ///
    /// On surface.attach requests to the pointer surface, hotspot_x and
    /// hotspot_y are decremented by the x and y parameters passed to the
    /// request. Attach must be confirmed by wl_surface.commit as usual.
    ///
    /// The hotspot can also be updated by passing the currently set pointer
    /// surface to this request with new values for hotspot_x and hotspot_y.
    ///
    /// The current and pending input regions of the wl_surface are cleared,
    /// and wl_surface.set_input_region is ignored until the wl_surface is no
    /// longer used as the cursor. When the use as a cursor ends, the current
    /// and pending input regions become undefined, and the wl_surface is
    /// unmapped.
    ///
    /// This request gives the surface the role of a zwp_tablet_tool_v2 cursor. A
    /// surface may only ever be used as the cursor surface for one
    /// zwp_tablet_tool_v2. If the surface already has another role or has
    /// previously been used as cursor surface for a different tool, a
    /// protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the proximity_in event
    /// - `surface`:
    /// - `hotspot_x`: surface-local x coordinate
    /// - `hotspot_y`: surface-local y coordinate
    #[inline]
    pub fn try_send_set_cursor(
        &self,
        serial: u32,
        surface: Option<&Rc<WlSurface>>,
        hotspot_x: i32,
        hotspot_y: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            serial,
            surface,
            hotspot_x,
            hotspot_y,
        );
        let arg1 = arg1.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1 {
            None => 0,
            Some(arg1) => match arg1.server_obj_id.get() {
                None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
                Some(id) => id,
            },
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: i32, arg3: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_tablet_tool_v2#{}.set_cursor(serial: {}, surface: wl_surface#{}, hotspot_x: {}, hotspot_y: {})\n", id, arg0, arg1, arg2, arg3);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1_id, arg2, arg3);
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
            arg1_id,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// set the tablet tool's surface
    ///
    /// Sets the surface of the cursor used for this tool on the given
    /// tablet. This request only takes effect if the tool is in proximity
    /// of one of the requesting client's surfaces or the surface parameter
    /// is the current pointer surface. If there was a previous surface set
    /// with this request it is replaced. If surface is NULL, the cursor
    /// image is hidden.
    ///
    /// The parameters hotspot_x and hotspot_y define the position of the
    /// pointer surface relative to the pointer location. Its top-left corner
    /// is always at (x, y) - (hotspot_x, hotspot_y), where (x, y) are the
    /// coordinates of the pointer location, in surface-local coordinates.
    ///
    /// On surface.attach requests to the pointer surface, hotspot_x and
    /// hotspot_y are decremented by the x and y parameters passed to the
    /// request. Attach must be confirmed by wl_surface.commit as usual.
    ///
    /// The hotspot can also be updated by passing the currently set pointer
    /// surface to this request with new values for hotspot_x and hotspot_y.
    ///
    /// The current and pending input regions of the wl_surface are cleared,
    /// and wl_surface.set_input_region is ignored until the wl_surface is no
    /// longer used as the cursor. When the use as a cursor ends, the current
    /// and pending input regions become undefined, and the wl_surface is
    /// unmapped.
    ///
    /// This request gives the surface the role of a zwp_tablet_tool_v2 cursor. A
    /// surface may only ever be used as the cursor surface for one
    /// zwp_tablet_tool_v2. If the surface already has another role or has
    /// previously been used as cursor surface for a different tool, a
    /// protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the proximity_in event
    /// - `surface`:
    /// - `hotspot_x`: surface-local x coordinate
    /// - `hotspot_y`: surface-local y coordinate
    #[inline]
    pub fn send_set_cursor(
        &self,
        serial: u32,
        surface: Option<&Rc<WlSurface>>,
        hotspot_x: i32,
        hotspot_y: i32,
    ) {
        let res = self.try_send_set_cursor(
            serial,
            surface,
            hotspot_x,
            hotspot_y,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_tool_v2.set_cursor", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the tool object
    ///
    /// This destroys the client's resource for this tool object.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_tablet_tool_v2#{}.destroy()\n", id);
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
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the tool object
    ///
    /// This destroys the client's resource for this tool object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwp_tablet_tool_v2.destroy", &e);
        }
    }

    /// Since when the type message is available.
    pub const MSG__TYPE__SINCE: u32 = 1;

    /// tool type
    ///
    /// The tool type is the high-level type of the tool and usually decides
    /// the interaction expected from this tool.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_tool_v2.done event.
    ///
    /// # Arguments
    ///
    /// - `tool_type`: the physical tool type
    #[inline]
    pub fn try_send_type(
        &self,
        tool_type: ZwpTabletToolV2Type,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            tool_type,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: ZwpTabletToolV2Type) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_tool_v2#{}.type(tool_type: {:?})\n", client_id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// tool type
    ///
    /// The tool type is the high-level type of the tool and usually decides
    /// the interaction expected from this tool.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_tool_v2.done event.
    ///
    /// # Arguments
    ///
    /// - `tool_type`: the physical tool type
    #[inline]
    pub fn send_type(
        &self,
        tool_type: ZwpTabletToolV2Type,
    ) {
        let res = self.try_send_type(
            tool_type,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_tool_v2.type", &e);
        }
    }

    /// Since when the hardware_serial message is available.
    pub const MSG__HARDWARE_SERIAL__SINCE: u32 = 1;

    /// unique hardware serial number of the tool
    ///
    /// If the physical tool can be identified by a unique 64-bit serial
    /// number, this event notifies the client of this serial number.
    ///
    /// If multiple tablets are available in the same seat and the tool is
    /// uniquely identifiable by the serial number, that tool may move
    /// between tablets.
    ///
    /// Otherwise, if the tool has no serial number and this event is
    /// missing, the tool is tied to the tablet it first comes into
    /// proximity with. Even if the physical tool is used on multiple
    /// tablets, separate zwp_tablet_tool_v2 objects will be created, one per
    /// tablet.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_tool_v2.done event.
    ///
    /// # Arguments
    ///
    /// - `hardware_serial_hi`: the unique serial number of the tool, most significant bits
    /// - `hardware_serial_lo`: the unique serial number of the tool, least significant bits
    #[inline]
    pub fn try_send_hardware_serial(
        &self,
        hardware_serial_hi: u32,
        hardware_serial_lo: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            hardware_serial_hi,
            hardware_serial_lo,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_tool_v2#{}.hardware_serial(hardware_serial_hi: {}, hardware_serial_lo: {})\n", client_id, id, arg0, arg1);
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
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// unique hardware serial number of the tool
    ///
    /// If the physical tool can be identified by a unique 64-bit serial
    /// number, this event notifies the client of this serial number.
    ///
    /// If multiple tablets are available in the same seat and the tool is
    /// uniquely identifiable by the serial number, that tool may move
    /// between tablets.
    ///
    /// Otherwise, if the tool has no serial number and this event is
    /// missing, the tool is tied to the tablet it first comes into
    /// proximity with. Even if the physical tool is used on multiple
    /// tablets, separate zwp_tablet_tool_v2 objects will be created, one per
    /// tablet.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_tool_v2.done event.
    ///
    /// # Arguments
    ///
    /// - `hardware_serial_hi`: the unique serial number of the tool, most significant bits
    /// - `hardware_serial_lo`: the unique serial number of the tool, least significant bits
    #[inline]
    pub fn send_hardware_serial(
        &self,
        hardware_serial_hi: u32,
        hardware_serial_lo: u32,
    ) {
        let res = self.try_send_hardware_serial(
            hardware_serial_hi,
            hardware_serial_lo,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_tool_v2.hardware_serial", &e);
        }
    }

    /// Since when the hardware_id_wacom message is available.
    pub const MSG__HARDWARE_ID_WACOM__SINCE: u32 = 1;

    /// hardware id notification in Wacom's format
    ///
    /// This event notifies the client of a hardware id available on this tool.
    ///
    /// The hardware id is a device-specific 64-bit id that provides extra
    /// information about the tool in use, beyond the wl_tool.type
    /// enumeration. The format of the id is specific to tablets made by
    /// Wacom Inc. For example, the hardware id of a Wacom Grip
    /// Pen (a stylus) is 0x802.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_tool_v2.done event.
    ///
    /// # Arguments
    ///
    /// - `hardware_id_hi`: the hardware id, most significant bits
    /// - `hardware_id_lo`: the hardware id, least significant bits
    #[inline]
    pub fn try_send_hardware_id_wacom(
        &self,
        hardware_id_hi: u32,
        hardware_id_lo: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            hardware_id_hi,
            hardware_id_lo,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_tool_v2#{}.hardware_id_wacom(hardware_id_hi: {}, hardware_id_lo: {})\n", client_id, id, arg0, arg1);
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
            2,
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// hardware id notification in Wacom's format
    ///
    /// This event notifies the client of a hardware id available on this tool.
    ///
    /// The hardware id is a device-specific 64-bit id that provides extra
    /// information about the tool in use, beyond the wl_tool.type
    /// enumeration. The format of the id is specific to tablets made by
    /// Wacom Inc. For example, the hardware id of a Wacom Grip
    /// Pen (a stylus) is 0x802.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_tool_v2.done event.
    ///
    /// # Arguments
    ///
    /// - `hardware_id_hi`: the hardware id, most significant bits
    /// - `hardware_id_lo`: the hardware id, least significant bits
    #[inline]
    pub fn send_hardware_id_wacom(
        &self,
        hardware_id_hi: u32,
        hardware_id_lo: u32,
    ) {
        let res = self.try_send_hardware_id_wacom(
            hardware_id_hi,
            hardware_id_lo,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_tool_v2.hardware_id_wacom", &e);
        }
    }

    /// Since when the capability message is available.
    pub const MSG__CAPABILITY__SINCE: u32 = 1;

    /// tool capability notification
    ///
    /// This event notifies the client of any capabilities of this tool,
    /// beyond the main set of x/y axes and tip up/down detection.
    ///
    /// One event is sent for each extra capability available on this tool.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_tool_v2.done event.
    ///
    /// # Arguments
    ///
    /// - `capability`: the capability
    #[inline]
    pub fn try_send_capability(
        &self,
        capability: ZwpTabletToolV2Capability,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            capability,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: ZwpTabletToolV2Capability) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_tool_v2#{}.capability(capability: {:?})\n", client_id, id, arg0);
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
            3,
            arg0.0,
        ]);
        Ok(())
    }

    /// tool capability notification
    ///
    /// This event notifies the client of any capabilities of this tool,
    /// beyond the main set of x/y axes and tip up/down detection.
    ///
    /// One event is sent for each extra capability available on this tool.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_tool_v2.done event.
    ///
    /// # Arguments
    ///
    /// - `capability`: the capability
    #[inline]
    pub fn send_capability(
        &self,
        capability: ZwpTabletToolV2Capability,
    ) {
        let res = self.try_send_capability(
            capability,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_tool_v2.capability", &e);
        }
    }

    /// Since when the done message is available.
    pub const MSG__DONE__SINCE: u32 = 1;

    /// tool description events sequence complete
    ///
    /// This event signals the end of the initial burst of descriptive
    /// events. A client may consider the static description of the tool to
    /// be complete and finalize initialization of the tool.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_tool_v2#{}.done()\n", client_id, id);
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
            4,
        ]);
        Ok(())
    }

    /// tool description events sequence complete
    ///
    /// This event signals the end of the initial burst of descriptive
    /// events. A client may consider the static description of the tool to
    /// be complete and finalize initialization of the tool.
    #[inline]
    pub fn send_done(
        &self,
    ) {
        let res = self.try_send_done(
        );
        if let Err(e) = res {
            log_send("zwp_tablet_tool_v2.done", &e);
        }
    }

    /// Since when the removed message is available.
    pub const MSG__REMOVED__SINCE: u32 = 1;

    /// tool removed
    ///
    /// This event is sent when the tool is removed from the system and will
    /// send no further events. Should the physical tool come back into
    /// proximity later, a new zwp_tablet_tool_v2 object will be created.
    ///
    /// It is compositor-dependent when a tool is removed. A compositor may
    /// remove a tool on proximity out, tablet removal or any other reason.
    /// A compositor may also keep a tool alive until shutdown.
    ///
    /// If the tool is currently in proximity, a proximity_out event will be
    /// sent before the removed event. See zwp_tablet_tool_v2.proximity_out for
    /// the handling of any buttons logically down.
    ///
    /// When this event is received, the client must zwp_tablet_tool_v2.destroy
    /// the object.
    #[inline]
    pub fn try_send_removed(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_tool_v2#{}.removed()\n", client_id, id);
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
            5,
        ]);
        Ok(())
    }

    /// tool removed
    ///
    /// This event is sent when the tool is removed from the system and will
    /// send no further events. Should the physical tool come back into
    /// proximity later, a new zwp_tablet_tool_v2 object will be created.
    ///
    /// It is compositor-dependent when a tool is removed. A compositor may
    /// remove a tool on proximity out, tablet removal or any other reason.
    /// A compositor may also keep a tool alive until shutdown.
    ///
    /// If the tool is currently in proximity, a proximity_out event will be
    /// sent before the removed event. See zwp_tablet_tool_v2.proximity_out for
    /// the handling of any buttons logically down.
    ///
    /// When this event is received, the client must zwp_tablet_tool_v2.destroy
    /// the object.
    #[inline]
    pub fn send_removed(
        &self,
    ) {
        let res = self.try_send_removed(
        );
        if let Err(e) = res {
            log_send("zwp_tablet_tool_v2.removed", &e);
        }
    }

    /// Since when the proximity_in message is available.
    pub const MSG__PROXIMITY_IN__SINCE: u32 = 1;

    /// proximity in event
    ///
    /// Notification that this tool is focused on a certain surface.
    ///
    /// This event can be received when the tool has moved from one surface to
    /// another, or when the tool has come back into proximity above the
    /// surface.
    ///
    /// If any button is logically down when the tool comes into proximity,
    /// the respective button event is sent after the proximity_in event but
    /// within the same frame as the proximity_in event.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    /// - `tablet`: The tablet the tool is in proximity of
    /// - `surface`: The current surface the tablet tool is over
    #[inline]
    pub fn try_send_proximity_in(
        &self,
        serial: u32,
        tablet: &Rc<ZwpTabletV2>,
        surface: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            serial,
            tablet,
            surface,
        );
        let arg1 = arg1.core();
        let arg2 = arg2.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg1.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("tablet", client.endpoint.id)));
        }
        if arg2.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("surface", client.endpoint.id)));
        }
        let arg1_id = arg1.client_obj_id.get().unwrap_or(0);
        let arg2_id = arg2.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_tool_v2#{}.proximity_in(serial: {}, tablet: zwp_tablet_v2#{}, surface: wl_surface#{})\n", client_id, id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1_id, arg2_id);
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
            6,
            arg0,
            arg1_id,
            arg2_id,
        ]);
        Ok(())
    }

    /// proximity in event
    ///
    /// Notification that this tool is focused on a certain surface.
    ///
    /// This event can be received when the tool has moved from one surface to
    /// another, or when the tool has come back into proximity above the
    /// surface.
    ///
    /// If any button is logically down when the tool comes into proximity,
    /// the respective button event is sent after the proximity_in event but
    /// within the same frame as the proximity_in event.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    /// - `tablet`: The tablet the tool is in proximity of
    /// - `surface`: The current surface the tablet tool is over
    #[inline]
    pub fn send_proximity_in(
        &self,
        serial: u32,
        tablet: &Rc<ZwpTabletV2>,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_proximity_in(
            serial,
            tablet,
            surface,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_tool_v2.proximity_in", &e);
        }
    }

    /// Since when the proximity_out message is available.
    pub const MSG__PROXIMITY_OUT__SINCE: u32 = 1;

    /// proximity out event
    ///
    /// Notification that this tool has either left proximity, or is no
    /// longer focused on a certain surface.
    ///
    /// When the tablet tool leaves proximity of the tablet, button release
    /// events are sent for each button that was held down at the time of
    /// leaving proximity. These events are sent before the proximity_out
    /// event but within the same zwp_tablet_v2.frame.
    ///
    /// If the tool stays within proximity of the tablet, but the focus
    /// changes from one surface to another, a button release event may not
    /// be sent until the button is actually released or the tool leaves the
    /// proximity of the tablet.
    #[inline]
    pub fn try_send_proximity_out(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_tool_v2#{}.proximity_out()\n", client_id, id);
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
            7,
        ]);
        Ok(())
    }

    /// proximity out event
    ///
    /// Notification that this tool has either left proximity, or is no
    /// longer focused on a certain surface.
    ///
    /// When the tablet tool leaves proximity of the tablet, button release
    /// events are sent for each button that was held down at the time of
    /// leaving proximity. These events are sent before the proximity_out
    /// event but within the same zwp_tablet_v2.frame.
    ///
    /// If the tool stays within proximity of the tablet, but the focus
    /// changes from one surface to another, a button release event may not
    /// be sent until the button is actually released or the tool leaves the
    /// proximity of the tablet.
    #[inline]
    pub fn send_proximity_out(
        &self,
    ) {
        let res = self.try_send_proximity_out(
        );
        if let Err(e) = res {
            log_send("zwp_tablet_tool_v2.proximity_out", &e);
        }
    }

    /// Since when the down message is available.
    pub const MSG__DOWN__SINCE: u32 = 1;

    /// tablet tool is making contact
    ///
    /// Sent whenever the tablet tool comes in contact with the surface of the
    /// tablet.
    ///
    /// If the tool is already in contact with the tablet when entering the
    /// input region, the client owning said region will receive a
    /// zwp_tablet_v2.proximity_in event, followed by a zwp_tablet_v2.down
    /// event and a zwp_tablet_v2.frame event.
    ///
    /// Note that this event describes logical contact, not physical
    /// contact. On some devices, a compositor may not consider a tool in
    /// logical contact until a minimum physical pressure threshold is
    /// exceeded.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    #[inline]
    pub fn try_send_down(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_tool_v2#{}.down(serial: {})\n", client_id, id, arg0);
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
            8,
            arg0,
        ]);
        Ok(())
    }

    /// tablet tool is making contact
    ///
    /// Sent whenever the tablet tool comes in contact with the surface of the
    /// tablet.
    ///
    /// If the tool is already in contact with the tablet when entering the
    /// input region, the client owning said region will receive a
    /// zwp_tablet_v2.proximity_in event, followed by a zwp_tablet_v2.down
    /// event and a zwp_tablet_v2.frame event.
    ///
    /// Note that this event describes logical contact, not physical
    /// contact. On some devices, a compositor may not consider a tool in
    /// logical contact until a minimum physical pressure threshold is
    /// exceeded.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    #[inline]
    pub fn send_down(
        &self,
        serial: u32,
    ) {
        let res = self.try_send_down(
            serial,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_tool_v2.down", &e);
        }
    }

    /// Since when the up message is available.
    pub const MSG__UP__SINCE: u32 = 1;

    /// tablet tool is no longer making contact
    ///
    /// Sent whenever the tablet tool stops making contact with the surface of
    /// the tablet, or when the tablet tool moves out of the input region
    /// and the compositor grab (if any) is dismissed.
    ///
    /// If the tablet tool moves out of the input region while in contact
    /// with the surface of the tablet and the compositor does not have an
    /// ongoing grab on the surface, the client owning said region will
    /// receive a zwp_tablet_v2.up event, followed by a zwp_tablet_v2.proximity_out
    /// event and a zwp_tablet_v2.frame event. If the compositor has an ongoing
    /// grab on this device, this event sequence is sent whenever the grab
    /// is dismissed in the future.
    ///
    /// Note that this event describes logical contact, not physical
    /// contact. On some devices, a compositor may not consider a tool out
    /// of logical contact until physical pressure falls below a specific
    /// threshold.
    #[inline]
    pub fn try_send_up(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_tool_v2#{}.up()\n", client_id, id);
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
            9,
        ]);
        Ok(())
    }

    /// tablet tool is no longer making contact
    ///
    /// Sent whenever the tablet tool stops making contact with the surface of
    /// the tablet, or when the tablet tool moves out of the input region
    /// and the compositor grab (if any) is dismissed.
    ///
    /// If the tablet tool moves out of the input region while in contact
    /// with the surface of the tablet and the compositor does not have an
    /// ongoing grab on the surface, the client owning said region will
    /// receive a zwp_tablet_v2.up event, followed by a zwp_tablet_v2.proximity_out
    /// event and a zwp_tablet_v2.frame event. If the compositor has an ongoing
    /// grab on this device, this event sequence is sent whenever the grab
    /// is dismissed in the future.
    ///
    /// Note that this event describes logical contact, not physical
    /// contact. On some devices, a compositor may not consider a tool out
    /// of logical contact until physical pressure falls below a specific
    /// threshold.
    #[inline]
    pub fn send_up(
        &self,
    ) {
        let res = self.try_send_up(
        );
        if let Err(e) = res {
            log_send("zwp_tablet_tool_v2.up", &e);
        }
    }

    /// Since when the motion message is available.
    pub const MSG__MOTION__SINCE: u32 = 1;

    /// motion event
    ///
    /// Sent whenever a tablet tool moves.
    ///
    /// # Arguments
    ///
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    #[inline]
    pub fn try_send_motion(
        &self,
        x: Fixed,
        y: Fixed,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: Fixed, arg1: Fixed) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_tool_v2#{}.motion(x: {}, y: {})\n", client_id, id, arg0, arg1);
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
            10,
            arg0.to_wire() as u32,
            arg1.to_wire() as u32,
        ]);
        Ok(())
    }

    /// motion event
    ///
    /// Sent whenever a tablet tool moves.
    ///
    /// # Arguments
    ///
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    #[inline]
    pub fn send_motion(
        &self,
        x: Fixed,
        y: Fixed,
    ) {
        let res = self.try_send_motion(
            x,
            y,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_tool_v2.motion", &e);
        }
    }

    /// Since when the pressure message is available.
    pub const MSG__PRESSURE__SINCE: u32 = 1;

    /// pressure change event
    ///
    /// Sent whenever the pressure axis on a tool changes. The value of this
    /// event is normalized to a value between 0 and 65535.
    ///
    /// Note that pressure may be nonzero even when a tool is not in logical
    /// contact. See the down and up events for more details.
    ///
    /// # Arguments
    ///
    /// - `pressure`: The current pressure value
    #[inline]
    pub fn try_send_pressure(
        &self,
        pressure: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            pressure,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_tool_v2#{}.pressure(pressure: {})\n", client_id, id, arg0);
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
            11,
            arg0,
        ]);
        Ok(())
    }

    /// pressure change event
    ///
    /// Sent whenever the pressure axis on a tool changes. The value of this
    /// event is normalized to a value between 0 and 65535.
    ///
    /// Note that pressure may be nonzero even when a tool is not in logical
    /// contact. See the down and up events for more details.
    ///
    /// # Arguments
    ///
    /// - `pressure`: The current pressure value
    #[inline]
    pub fn send_pressure(
        &self,
        pressure: u32,
    ) {
        let res = self.try_send_pressure(
            pressure,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_tool_v2.pressure", &e);
        }
    }

    /// Since when the distance message is available.
    pub const MSG__DISTANCE__SINCE: u32 = 1;

    /// distance change event
    ///
    /// Sent whenever the distance axis on a tool changes. The value of this
    /// event is normalized to a value between 0 and 65535.
    ///
    /// Note that distance may be nonzero even when a tool is not in logical
    /// contact. See the down and up events for more details.
    ///
    /// # Arguments
    ///
    /// - `distance`: The current distance value
    #[inline]
    pub fn try_send_distance(
        &self,
        distance: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            distance,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_tool_v2#{}.distance(distance: {})\n", client_id, id, arg0);
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
            12,
            arg0,
        ]);
        Ok(())
    }

    /// distance change event
    ///
    /// Sent whenever the distance axis on a tool changes. The value of this
    /// event is normalized to a value between 0 and 65535.
    ///
    /// Note that distance may be nonzero even when a tool is not in logical
    /// contact. See the down and up events for more details.
    ///
    /// # Arguments
    ///
    /// - `distance`: The current distance value
    #[inline]
    pub fn send_distance(
        &self,
        distance: u32,
    ) {
        let res = self.try_send_distance(
            distance,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_tool_v2.distance", &e);
        }
    }

    /// Since when the tilt message is available.
    pub const MSG__TILT__SINCE: u32 = 1;

    /// tilt change event
    ///
    /// Sent whenever one or both of the tilt axes on a tool change. Each tilt
    /// value is in degrees, relative to the z-axis of the tablet.
    /// The angle is positive when the top of a tool tilts along the
    /// positive x or y axis.
    ///
    /// # Arguments
    ///
    /// - `tilt_x`: The current value of the X tilt axis
    /// - `tilt_y`: The current value of the Y tilt axis
    #[inline]
    pub fn try_send_tilt(
        &self,
        tilt_x: Fixed,
        tilt_y: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            tilt_x,
            tilt_y,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: Fixed, arg1: Fixed) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_tool_v2#{}.tilt(tilt_x: {}, tilt_y: {})\n", client_id, id, arg0, arg1);
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
            13,
            arg0.to_wire() as u32,
            arg1.to_wire() as u32,
        ]);
        Ok(())
    }

    /// tilt change event
    ///
    /// Sent whenever one or both of the tilt axes on a tool change. Each tilt
    /// value is in degrees, relative to the z-axis of the tablet.
    /// The angle is positive when the top of a tool tilts along the
    /// positive x or y axis.
    ///
    /// # Arguments
    ///
    /// - `tilt_x`: The current value of the X tilt axis
    /// - `tilt_y`: The current value of the Y tilt axis
    #[inline]
    pub fn send_tilt(
        &self,
        tilt_x: Fixed,
        tilt_y: Fixed,
    ) {
        let res = self.try_send_tilt(
            tilt_x,
            tilt_y,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_tool_v2.tilt", &e);
        }
    }

    /// Since when the rotation message is available.
    pub const MSG__ROTATION__SINCE: u32 = 1;

    /// z-rotation change event
    ///
    /// Sent whenever the z-rotation axis on the tool changes. The
    /// rotation value is in degrees clockwise from the tool's
    /// logical neutral position.
    ///
    /// # Arguments
    ///
    /// - `degrees`: The current rotation of the Z axis
    #[inline]
    pub fn try_send_rotation(
        &self,
        degrees: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            degrees,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: Fixed) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_tool_v2#{}.rotation(degrees: {})\n", client_id, id, arg0);
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
            14,
            arg0.to_wire() as u32,
        ]);
        Ok(())
    }

    /// z-rotation change event
    ///
    /// Sent whenever the z-rotation axis on the tool changes. The
    /// rotation value is in degrees clockwise from the tool's
    /// logical neutral position.
    ///
    /// # Arguments
    ///
    /// - `degrees`: The current rotation of the Z axis
    #[inline]
    pub fn send_rotation(
        &self,
        degrees: Fixed,
    ) {
        let res = self.try_send_rotation(
            degrees,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_tool_v2.rotation", &e);
        }
    }

    /// Since when the slider message is available.
    pub const MSG__SLIDER__SINCE: u32 = 1;

    /// Slider position change event
    ///
    /// Sent whenever the slider position on the tool changes. The
    /// value is normalized between -65535 and 65535, with 0 as the logical
    /// neutral position of the slider.
    ///
    /// The slider is available on e.g. the Wacom Airbrush tool.
    ///
    /// # Arguments
    ///
    /// - `position`: The current position of slider
    #[inline]
    pub fn try_send_slider(
        &self,
        position: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            position,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_tool_v2#{}.slider(position: {})\n", client_id, id, arg0);
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
            15,
            arg0 as u32,
        ]);
        Ok(())
    }

    /// Slider position change event
    ///
    /// Sent whenever the slider position on the tool changes. The
    /// value is normalized between -65535 and 65535, with 0 as the logical
    /// neutral position of the slider.
    ///
    /// The slider is available on e.g. the Wacom Airbrush tool.
    ///
    /// # Arguments
    ///
    /// - `position`: The current position of slider
    #[inline]
    pub fn send_slider(
        &self,
        position: i32,
    ) {
        let res = self.try_send_slider(
            position,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_tool_v2.slider", &e);
        }
    }

    /// Since when the wheel message is available.
    pub const MSG__WHEEL__SINCE: u32 = 1;

    /// Wheel delta event
    ///
    /// Sent whenever the wheel on the tool emits an event. This event
    /// contains two values for the same axis change. The degrees value is
    /// in the same orientation as the wl_pointer.vertical_scroll axis. The
    /// clicks value is in discrete logical clicks of the mouse wheel. This
    /// value may be zero if the movement of the wheel was less
    /// than one logical click.
    ///
    /// Clients should choose either value and avoid mixing degrees and
    /// clicks. The compositor may accumulate values smaller than a logical
    /// click and emulate click events when a certain threshold is met.
    /// Thus, zwp_tablet_tool_v2.wheel events with non-zero clicks values may
    /// have different degrees values.
    ///
    /// # Arguments
    ///
    /// - `degrees`: The wheel delta in degrees
    /// - `clicks`: The wheel delta in discrete clicks
    #[inline]
    pub fn try_send_wheel(
        &self,
        degrees: Fixed,
        clicks: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            degrees,
            clicks,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: Fixed, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_tool_v2#{}.wheel(degrees: {}, clicks: {})\n", client_id, id, arg0, arg1);
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
            16,
            arg0.to_wire() as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// Wheel delta event
    ///
    /// Sent whenever the wheel on the tool emits an event. This event
    /// contains two values for the same axis change. The degrees value is
    /// in the same orientation as the wl_pointer.vertical_scroll axis. The
    /// clicks value is in discrete logical clicks of the mouse wheel. This
    /// value may be zero if the movement of the wheel was less
    /// than one logical click.
    ///
    /// Clients should choose either value and avoid mixing degrees and
    /// clicks. The compositor may accumulate values smaller than a logical
    /// click and emulate click events when a certain threshold is met.
    /// Thus, zwp_tablet_tool_v2.wheel events with non-zero clicks values may
    /// have different degrees values.
    ///
    /// # Arguments
    ///
    /// - `degrees`: The wheel delta in degrees
    /// - `clicks`: The wheel delta in discrete clicks
    #[inline]
    pub fn send_wheel(
        &self,
        degrees: Fixed,
        clicks: i32,
    ) {
        let res = self.try_send_wheel(
            degrees,
            clicks,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_tool_v2.wheel", &e);
        }
    }

    /// Since when the button message is available.
    pub const MSG__BUTTON__SINCE: u32 = 1;

    /// button event
    ///
    /// Sent whenever a button on the tool is pressed or released.
    ///
    /// If a button is held down when the tool moves in or out of proximity,
    /// button events are generated by the compositor. See
    /// zwp_tablet_tool_v2.proximity_in and zwp_tablet_tool_v2.proximity_out for
    /// details.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    /// - `button`: The button whose state has changed
    /// - `state`: Whether the button was pressed or released
    #[inline]
    pub fn try_send_button(
        &self,
        serial: u32,
        button: u32,
        state: ZwpTabletToolV2ButtonState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            serial,
            button,
            state,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: ZwpTabletToolV2ButtonState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_tool_v2#{}.button(serial: {}, button: {}, state: {:?})\n", client_id, id, arg0, arg1, arg2);
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
            17,
            arg0,
            arg1,
            arg2.0,
        ]);
        Ok(())
    }

    /// button event
    ///
    /// Sent whenever a button on the tool is pressed or released.
    ///
    /// If a button is held down when the tool moves in or out of proximity,
    /// button events are generated by the compositor. See
    /// zwp_tablet_tool_v2.proximity_in and zwp_tablet_tool_v2.proximity_out for
    /// details.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    /// - `button`: The button whose state has changed
    /// - `state`: Whether the button was pressed or released
    #[inline]
    pub fn send_button(
        &self,
        serial: u32,
        button: u32,
        state: ZwpTabletToolV2ButtonState,
    ) {
        let res = self.try_send_button(
            serial,
            button,
            state,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_tool_v2.button", &e);
        }
    }

    /// Since when the frame message is available.
    pub const MSG__FRAME__SINCE: u32 = 1;

    /// frame event
    ///
    /// Marks the end of a series of axis and/or button updates from the
    /// tablet. The Wayland protocol requires axis updates to be sent
    /// sequentially, however all events within a frame should be considered
    /// one hardware event.
    ///
    /// # Arguments
    ///
    /// - `time`: The time of the event with millisecond granularity
    #[inline]
    pub fn try_send_frame(
        &self,
        time: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            time,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_tool_v2#{}.frame(time: {})\n", client_id, id, arg0);
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
            18,
            arg0,
        ]);
        Ok(())
    }

    /// frame event
    ///
    /// Marks the end of a series of axis and/or button updates from the
    /// tablet. The Wayland protocol requires axis updates to be sent
    /// sequentially, however all events within a frame should be considered
    /// one hardware event.
    ///
    /// # Arguments
    ///
    /// - `time`: The time of the event with millisecond granularity
    #[inline]
    pub fn send_frame(
        &self,
        time: u32,
    ) {
        let res = self.try_send_frame(
            time,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_tool_v2.frame", &e);
        }
    }
}

/// A message handler for [`ZwpTabletToolV2`] proxies.
pub trait ZwpTabletToolV2Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpTabletToolV2>) {
        slf.core.delete_id();
    }

    /// set the tablet tool's surface
    ///
    /// Sets the surface of the cursor used for this tool on the given
    /// tablet. This request only takes effect if the tool is in proximity
    /// of one of the requesting client's surfaces or the surface parameter
    /// is the current pointer surface. If there was a previous surface set
    /// with this request it is replaced. If surface is NULL, the cursor
    /// image is hidden.
    ///
    /// The parameters hotspot_x and hotspot_y define the position of the
    /// pointer surface relative to the pointer location. Its top-left corner
    /// is always at (x, y) - (hotspot_x, hotspot_y), where (x, y) are the
    /// coordinates of the pointer location, in surface-local coordinates.
    ///
    /// On surface.attach requests to the pointer surface, hotspot_x and
    /// hotspot_y are decremented by the x and y parameters passed to the
    /// request. Attach must be confirmed by wl_surface.commit as usual.
    ///
    /// The hotspot can also be updated by passing the currently set pointer
    /// surface to this request with new values for hotspot_x and hotspot_y.
    ///
    /// The current and pending input regions of the wl_surface are cleared,
    /// and wl_surface.set_input_region is ignored until the wl_surface is no
    /// longer used as the cursor. When the use as a cursor ends, the current
    /// and pending input regions become undefined, and the wl_surface is
    /// unmapped.
    ///
    /// This request gives the surface the role of a zwp_tablet_tool_v2 cursor. A
    /// surface may only ever be used as the cursor surface for one
    /// zwp_tablet_tool_v2. If the surface already has another role or has
    /// previously been used as cursor surface for a different tool, a
    /// protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the proximity_in event
    /// - `surface`:
    /// - `hotspot_x`: surface-local x coordinate
    /// - `hotspot_y`: surface-local y coordinate
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_cursor(
        &mut self,
        slf: &Rc<ZwpTabletToolV2>,
        serial: u32,
        surface: Option<&Rc<WlSurface>>,
        hotspot_x: i32,
        hotspot_y: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_cursor(
            serial,
            surface,
            hotspot_x,
            hotspot_y,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_tool_v2.set_cursor", &e);
        }
    }

    /// destroy the tool object
    ///
    /// This destroys the client's resource for this tool object.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwpTabletToolV2>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_tool_v2.destroy", &e);
        }
    }

    /// tool type
    ///
    /// The tool type is the high-level type of the tool and usually decides
    /// the interaction expected from this tool.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_tool_v2.done event.
    ///
    /// # Arguments
    ///
    /// - `tool_type`: the physical tool type
    #[inline]
    fn handle_type(
        &mut self,
        slf: &Rc<ZwpTabletToolV2>,
        tool_type: ZwpTabletToolV2Type,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_type(
            tool_type,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_tool_v2.type", &e);
        }
    }

    /// unique hardware serial number of the tool
    ///
    /// If the physical tool can be identified by a unique 64-bit serial
    /// number, this event notifies the client of this serial number.
    ///
    /// If multiple tablets are available in the same seat and the tool is
    /// uniquely identifiable by the serial number, that tool may move
    /// between tablets.
    ///
    /// Otherwise, if the tool has no serial number and this event is
    /// missing, the tool is tied to the tablet it first comes into
    /// proximity with. Even if the physical tool is used on multiple
    /// tablets, separate zwp_tablet_tool_v2 objects will be created, one per
    /// tablet.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_tool_v2.done event.
    ///
    /// # Arguments
    ///
    /// - `hardware_serial_hi`: the unique serial number of the tool, most significant bits
    /// - `hardware_serial_lo`: the unique serial number of the tool, least significant bits
    #[inline]
    fn handle_hardware_serial(
        &mut self,
        slf: &Rc<ZwpTabletToolV2>,
        hardware_serial_hi: u32,
        hardware_serial_lo: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_hardware_serial(
            hardware_serial_hi,
            hardware_serial_lo,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_tool_v2.hardware_serial", &e);
        }
    }

    /// hardware id notification in Wacom's format
    ///
    /// This event notifies the client of a hardware id available on this tool.
    ///
    /// The hardware id is a device-specific 64-bit id that provides extra
    /// information about the tool in use, beyond the wl_tool.type
    /// enumeration. The format of the id is specific to tablets made by
    /// Wacom Inc. For example, the hardware id of a Wacom Grip
    /// Pen (a stylus) is 0x802.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_tool_v2.done event.
    ///
    /// # Arguments
    ///
    /// - `hardware_id_hi`: the hardware id, most significant bits
    /// - `hardware_id_lo`: the hardware id, least significant bits
    #[inline]
    fn handle_hardware_id_wacom(
        &mut self,
        slf: &Rc<ZwpTabletToolV2>,
        hardware_id_hi: u32,
        hardware_id_lo: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_hardware_id_wacom(
            hardware_id_hi,
            hardware_id_lo,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_tool_v2.hardware_id_wacom", &e);
        }
    }

    /// tool capability notification
    ///
    /// This event notifies the client of any capabilities of this tool,
    /// beyond the main set of x/y axes and tip up/down detection.
    ///
    /// One event is sent for each extra capability available on this tool.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_tool_v2.done event.
    ///
    /// # Arguments
    ///
    /// - `capability`: the capability
    #[inline]
    fn handle_capability(
        &mut self,
        slf: &Rc<ZwpTabletToolV2>,
        capability: ZwpTabletToolV2Capability,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_capability(
            capability,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_tool_v2.capability", &e);
        }
    }

    /// tool description events sequence complete
    ///
    /// This event signals the end of the initial burst of descriptive
    /// events. A client may consider the static description of the tool to
    /// be complete and finalize initialization of the tool.
    #[inline]
    fn handle_done(
        &mut self,
        slf: &Rc<ZwpTabletToolV2>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_done(
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_tool_v2.done", &e);
        }
    }

    /// tool removed
    ///
    /// This event is sent when the tool is removed from the system and will
    /// send no further events. Should the physical tool come back into
    /// proximity later, a new zwp_tablet_tool_v2 object will be created.
    ///
    /// It is compositor-dependent when a tool is removed. A compositor may
    /// remove a tool on proximity out, tablet removal or any other reason.
    /// A compositor may also keep a tool alive until shutdown.
    ///
    /// If the tool is currently in proximity, a proximity_out event will be
    /// sent before the removed event. See zwp_tablet_tool_v2.proximity_out for
    /// the handling of any buttons logically down.
    ///
    /// When this event is received, the client must zwp_tablet_tool_v2.destroy
    /// the object.
    #[inline]
    fn handle_removed(
        &mut self,
        slf: &Rc<ZwpTabletToolV2>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_removed(
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_tool_v2.removed", &e);
        }
    }

    /// proximity in event
    ///
    /// Notification that this tool is focused on a certain surface.
    ///
    /// This event can be received when the tool has moved from one surface to
    /// another, or when the tool has come back into proximity above the
    /// surface.
    ///
    /// If any button is logically down when the tool comes into proximity,
    /// the respective button event is sent after the proximity_in event but
    /// within the same frame as the proximity_in event.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    /// - `tablet`: The tablet the tool is in proximity of
    /// - `surface`: The current surface the tablet tool is over
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_proximity_in(
        &mut self,
        slf: &Rc<ZwpTabletToolV2>,
        serial: u32,
        tablet: &Rc<ZwpTabletV2>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = tablet.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
            if let Some(client_id_2) = surface.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = slf.try_send_proximity_in(
            serial,
            tablet,
            surface,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_tool_v2.proximity_in", &e);
        }
    }

    /// proximity out event
    ///
    /// Notification that this tool has either left proximity, or is no
    /// longer focused on a certain surface.
    ///
    /// When the tablet tool leaves proximity of the tablet, button release
    /// events are sent for each button that was held down at the time of
    /// leaving proximity. These events are sent before the proximity_out
    /// event but within the same zwp_tablet_v2.frame.
    ///
    /// If the tool stays within proximity of the tablet, but the focus
    /// changes from one surface to another, a button release event may not
    /// be sent until the button is actually released or the tool leaves the
    /// proximity of the tablet.
    #[inline]
    fn handle_proximity_out(
        &mut self,
        slf: &Rc<ZwpTabletToolV2>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_proximity_out(
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_tool_v2.proximity_out", &e);
        }
    }

    /// tablet tool is making contact
    ///
    /// Sent whenever the tablet tool comes in contact with the surface of the
    /// tablet.
    ///
    /// If the tool is already in contact with the tablet when entering the
    /// input region, the client owning said region will receive a
    /// zwp_tablet_v2.proximity_in event, followed by a zwp_tablet_v2.down
    /// event and a zwp_tablet_v2.frame event.
    ///
    /// Note that this event describes logical contact, not physical
    /// contact. On some devices, a compositor may not consider a tool in
    /// logical contact until a minimum physical pressure threshold is
    /// exceeded.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    #[inline]
    fn handle_down(
        &mut self,
        slf: &Rc<ZwpTabletToolV2>,
        serial: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_down(
            serial,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_tool_v2.down", &e);
        }
    }

    /// tablet tool is no longer making contact
    ///
    /// Sent whenever the tablet tool stops making contact with the surface of
    /// the tablet, or when the tablet tool moves out of the input region
    /// and the compositor grab (if any) is dismissed.
    ///
    /// If the tablet tool moves out of the input region while in contact
    /// with the surface of the tablet and the compositor does not have an
    /// ongoing grab on the surface, the client owning said region will
    /// receive a zwp_tablet_v2.up event, followed by a zwp_tablet_v2.proximity_out
    /// event and a zwp_tablet_v2.frame event. If the compositor has an ongoing
    /// grab on this device, this event sequence is sent whenever the grab
    /// is dismissed in the future.
    ///
    /// Note that this event describes logical contact, not physical
    /// contact. On some devices, a compositor may not consider a tool out
    /// of logical contact until physical pressure falls below a specific
    /// threshold.
    #[inline]
    fn handle_up(
        &mut self,
        slf: &Rc<ZwpTabletToolV2>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_up(
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_tool_v2.up", &e);
        }
    }

    /// motion event
    ///
    /// Sent whenever a tablet tool moves.
    ///
    /// # Arguments
    ///
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    #[inline]
    fn handle_motion(
        &mut self,
        slf: &Rc<ZwpTabletToolV2>,
        x: Fixed,
        y: Fixed,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_motion(
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_tool_v2.motion", &e);
        }
    }

    /// pressure change event
    ///
    /// Sent whenever the pressure axis on a tool changes. The value of this
    /// event is normalized to a value between 0 and 65535.
    ///
    /// Note that pressure may be nonzero even when a tool is not in logical
    /// contact. See the down and up events for more details.
    ///
    /// # Arguments
    ///
    /// - `pressure`: The current pressure value
    #[inline]
    fn handle_pressure(
        &mut self,
        slf: &Rc<ZwpTabletToolV2>,
        pressure: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_pressure(
            pressure,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_tool_v2.pressure", &e);
        }
    }

    /// distance change event
    ///
    /// Sent whenever the distance axis on a tool changes. The value of this
    /// event is normalized to a value between 0 and 65535.
    ///
    /// Note that distance may be nonzero even when a tool is not in logical
    /// contact. See the down and up events for more details.
    ///
    /// # Arguments
    ///
    /// - `distance`: The current distance value
    #[inline]
    fn handle_distance(
        &mut self,
        slf: &Rc<ZwpTabletToolV2>,
        distance: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_distance(
            distance,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_tool_v2.distance", &e);
        }
    }

    /// tilt change event
    ///
    /// Sent whenever one or both of the tilt axes on a tool change. Each tilt
    /// value is in degrees, relative to the z-axis of the tablet.
    /// The angle is positive when the top of a tool tilts along the
    /// positive x or y axis.
    ///
    /// # Arguments
    ///
    /// - `tilt_x`: The current value of the X tilt axis
    /// - `tilt_y`: The current value of the Y tilt axis
    #[inline]
    fn handle_tilt(
        &mut self,
        slf: &Rc<ZwpTabletToolV2>,
        tilt_x: Fixed,
        tilt_y: Fixed,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_tilt(
            tilt_x,
            tilt_y,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_tool_v2.tilt", &e);
        }
    }

    /// z-rotation change event
    ///
    /// Sent whenever the z-rotation axis on the tool changes. The
    /// rotation value is in degrees clockwise from the tool's
    /// logical neutral position.
    ///
    /// # Arguments
    ///
    /// - `degrees`: The current rotation of the Z axis
    #[inline]
    fn handle_rotation(
        &mut self,
        slf: &Rc<ZwpTabletToolV2>,
        degrees: Fixed,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_rotation(
            degrees,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_tool_v2.rotation", &e);
        }
    }

    /// Slider position change event
    ///
    /// Sent whenever the slider position on the tool changes. The
    /// value is normalized between -65535 and 65535, with 0 as the logical
    /// neutral position of the slider.
    ///
    /// The slider is available on e.g. the Wacom Airbrush tool.
    ///
    /// # Arguments
    ///
    /// - `position`: The current position of slider
    #[inline]
    fn handle_slider(
        &mut self,
        slf: &Rc<ZwpTabletToolV2>,
        position: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_slider(
            position,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_tool_v2.slider", &e);
        }
    }

    /// Wheel delta event
    ///
    /// Sent whenever the wheel on the tool emits an event. This event
    /// contains two values for the same axis change. The degrees value is
    /// in the same orientation as the wl_pointer.vertical_scroll axis. The
    /// clicks value is in discrete logical clicks of the mouse wheel. This
    /// value may be zero if the movement of the wheel was less
    /// than one logical click.
    ///
    /// Clients should choose either value and avoid mixing degrees and
    /// clicks. The compositor may accumulate values smaller than a logical
    /// click and emulate click events when a certain threshold is met.
    /// Thus, zwp_tablet_tool_v2.wheel events with non-zero clicks values may
    /// have different degrees values.
    ///
    /// # Arguments
    ///
    /// - `degrees`: The wheel delta in degrees
    /// - `clicks`: The wheel delta in discrete clicks
    #[inline]
    fn handle_wheel(
        &mut self,
        slf: &Rc<ZwpTabletToolV2>,
        degrees: Fixed,
        clicks: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_wheel(
            degrees,
            clicks,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_tool_v2.wheel", &e);
        }
    }

    /// button event
    ///
    /// Sent whenever a button on the tool is pressed or released.
    ///
    /// If a button is held down when the tool moves in or out of proximity,
    /// button events are generated by the compositor. See
    /// zwp_tablet_tool_v2.proximity_in and zwp_tablet_tool_v2.proximity_out for
    /// details.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    /// - `button`: The button whose state has changed
    /// - `state`: Whether the button was pressed or released
    #[inline]
    fn handle_button(
        &mut self,
        slf: &Rc<ZwpTabletToolV2>,
        serial: u32,
        button: u32,
        state: ZwpTabletToolV2ButtonState,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_button(
            serial,
            button,
            state,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_tool_v2.button", &e);
        }
    }

    /// frame event
    ///
    /// Marks the end of a series of axis and/or button updates from the
    /// tablet. The Wayland protocol requires axis updates to be sent
    /// sequentially, however all events within a frame should be considered
    /// one hardware event.
    ///
    /// # Arguments
    ///
    /// - `time`: The time of the event with millisecond granularity
    #[inline]
    fn handle_frame(
        &mut self,
        slf: &Rc<ZwpTabletToolV2>,
        time: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_frame(
            time,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_tool_v2.frame", &e);
        }
    }
}

impl ObjectPrivate for ZwpTabletToolV2 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpTabletToolV2, version),
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
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: i32, arg3: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_tablet_tool_v2#{}.set_cursor(serial: {}, surface: wl_surface#{}, hotspot_x: {}, hotspot_y: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                let arg1 = if arg1 == 0 {
                    None
                } else {
                    let arg1_id = arg1;
                    let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                    };
                    let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                        let o = client.endpoint.lookup(arg1_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                    };
                    Some(arg1)
                };
                let arg1 = arg1.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_set_cursor(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_set_cursor(&self, arg0, arg1, arg2, arg3);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_tablet_tool_v2#{}.destroy()\n", client_id, id);
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
                let arg0 = ZwpTabletToolV2Type(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: ZwpTabletToolV2Type) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_tool_v2#{}.type(tool_type: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_type(&self, arg0);
                } else {
                    DefaultHandler.handle_type(&self, arg0);
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
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_tool_v2#{}.hardware_serial(hardware_serial_hi: {}, hardware_serial_lo: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_hardware_serial(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_hardware_serial(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_tool_v2#{}.hardware_id_wacom(hardware_id_hi: {}, hardware_id_lo: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_hardware_id_wacom(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_hardware_id_wacom(&self, arg0, arg1);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = ZwpTabletToolV2Capability(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: ZwpTabletToolV2Capability) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_tool_v2#{}.capability(capability: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_capability(&self, arg0);
                } else {
                    DefaultHandler.handle_capability(&self, arg0);
                }
            }
            4 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_tool_v2#{}.done()\n", id);
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
            5 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_tool_v2#{}.removed()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_removed(&self);
                } else {
                    DefaultHandler.handle_removed(&self);
                }
            }
            6 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_tool_v2#{}.proximity_in(serial: {}, tablet: zwp_tablet_v2#{}, surface: wl_surface#{})\n", id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2);
                }
                let arg1_id = arg1;
                let Some(arg1) = server.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<ZwpTabletV2>() else {
                    let o = server.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("tablet", o.core().interface, ObjectInterface::ZwpTabletV2)));
                };
                let arg2_id = arg2;
                let Some(arg2) = server.lookup(arg2_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg2_id)));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = server.lookup(arg2_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg1 = &arg1;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).handle_proximity_in(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_proximity_in(&self, arg0, arg1, arg2);
                }
            }
            7 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_tool_v2#{}.proximity_out()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_proximity_out(&self);
                } else {
                    DefaultHandler.handle_proximity_out(&self);
                }
            }
            8 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_tool_v2#{}.down(serial: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_down(&self, arg0);
                } else {
                    DefaultHandler.handle_down(&self, arg0);
                }
            }
            9 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_tool_v2#{}.up()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_up(&self);
                } else {
                    DefaultHandler.handle_up(&self);
                }
            }
            10 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = Fixed::from_wire(arg0 as i32);
                let arg1 = Fixed::from_wire(arg1 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: Fixed, arg1: Fixed) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_tool_v2#{}.motion(x: {}, y: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_motion(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_motion(&self, arg0, arg1);
                }
            }
            11 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_tool_v2#{}.pressure(pressure: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_pressure(&self, arg0);
                } else {
                    DefaultHandler.handle_pressure(&self, arg0);
                }
            }
            12 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_tool_v2#{}.distance(distance: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_distance(&self, arg0);
                } else {
                    DefaultHandler.handle_distance(&self, arg0);
                }
            }
            13 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = Fixed::from_wire(arg0 as i32);
                let arg1 = Fixed::from_wire(arg1 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: Fixed, arg1: Fixed) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_tool_v2#{}.tilt(tilt_x: {}, tilt_y: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_tilt(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_tilt(&self, arg0, arg1);
                }
            }
            14 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = Fixed::from_wire(arg0 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: Fixed) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_tool_v2#{}.rotation(degrees: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_rotation(&self, arg0);
                } else {
                    DefaultHandler.handle_rotation(&self, arg0);
                }
            }
            15 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = arg0 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_tool_v2#{}.slider(position: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_slider(&self, arg0);
                } else {
                    DefaultHandler.handle_slider(&self, arg0);
                }
            }
            16 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = Fixed::from_wire(arg0 as i32);
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: Fixed, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_tool_v2#{}.wheel(degrees: {}, clicks: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_wheel(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_wheel(&self, arg0, arg1);
                }
            }
            17 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                let arg2 = ZwpTabletToolV2ButtonState(arg2);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: ZwpTabletToolV2ButtonState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_tool_v2#{}.button(serial: {}, button: {}, state: {:?})\n", id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_button(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_button(&self, arg0, arg1, arg2);
                }
            }
            18 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_tool_v2#{}.frame(time: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_frame(&self, arg0);
                } else {
                    DefaultHandler.handle_frame(&self, arg0);
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
            0 => "set_cursor",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "type",
            1 => "hardware_serial",
            2 => "hardware_id_wacom",
            3 => "capability",
            4 => "done",
            5 => "removed",
            6 => "proximity_in",
            7 => "proximity_out",
            8 => "down",
            9 => "up",
            10 => "motion",
            11 => "pressure",
            12 => "distance",
            13 => "tilt",
            14 => "rotation",
            15 => "slider",
            16 => "wheel",
            17 => "button",
            18 => "frame",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwpTabletToolV2 {
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

impl ZwpTabletToolV2 {
    /// Since when the type.pen enum variant is available.
    pub const ENM__TYPE_PEN__SINCE: u32 = 1;
    /// Since when the type.eraser enum variant is available.
    pub const ENM__TYPE_ERASER__SINCE: u32 = 1;
    /// Since when the type.brush enum variant is available.
    pub const ENM__TYPE_BRUSH__SINCE: u32 = 1;
    /// Since when the type.pencil enum variant is available.
    pub const ENM__TYPE_PENCIL__SINCE: u32 = 1;
    /// Since when the type.airbrush enum variant is available.
    pub const ENM__TYPE_AIRBRUSH__SINCE: u32 = 1;
    /// Since when the type.finger enum variant is available.
    pub const ENM__TYPE_FINGER__SINCE: u32 = 1;
    /// Since when the type.mouse enum variant is available.
    pub const ENM__TYPE_MOUSE__SINCE: u32 = 1;
    /// Since when the type.lens enum variant is available.
    pub const ENM__TYPE_LENS__SINCE: u32 = 1;

    /// Since when the capability.tilt enum variant is available.
    pub const ENM__CAPABILITY_TILT__SINCE: u32 = 1;
    /// Since when the capability.pressure enum variant is available.
    pub const ENM__CAPABILITY_PRESSURE__SINCE: u32 = 1;
    /// Since when the capability.distance enum variant is available.
    pub const ENM__CAPABILITY_DISTANCE__SINCE: u32 = 1;
    /// Since when the capability.rotation enum variant is available.
    pub const ENM__CAPABILITY_ROTATION__SINCE: u32 = 1;
    /// Since when the capability.slider enum variant is available.
    pub const ENM__CAPABILITY_SLIDER__SINCE: u32 = 1;
    /// Since when the capability.wheel enum variant is available.
    pub const ENM__CAPABILITY_WHEEL__SINCE: u32 = 1;

    /// Since when the button_state.released enum variant is available.
    pub const ENM__BUTTON_STATE_RELEASED__SINCE: u32 = 1;
    /// Since when the button_state.pressed enum variant is available.
    pub const ENM__BUTTON_STATE_PRESSED__SINCE: u32 = 1;

    /// Since when the error.role enum variant is available.
    pub const ENM__ERROR_ROLE__SINCE: u32 = 1;
}

/// a physical tool type
///
/// Describes the physical type of a tool. The physical type of a tool
/// generally defines its base usage.
///
/// The mouse tool represents a mouse-shaped tool that is not a relative
/// device but bound to the tablet's surface, providing absolute
/// coordinates.
///
/// The lens tool is a mouse-shaped tool with an attached lens to
/// provide precision focus.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwpTabletToolV2Type(pub u32);

impl ZwpTabletToolV2Type {
    /// Pen
    pub const PEN: Self = Self(0x140);

    /// Eraser
    pub const ERASER: Self = Self(0x141);

    /// Brush
    pub const BRUSH: Self = Self(0x142);

    /// Pencil
    pub const PENCIL: Self = Self(0x143);

    /// Airbrush
    pub const AIRBRUSH: Self = Self(0x144);

    /// Finger
    pub const FINGER: Self = Self(0x145);

    /// Mouse
    pub const MOUSE: Self = Self(0x146);

    /// Lens
    pub const LENS: Self = Self(0x147);
}

impl Debug for ZwpTabletToolV2Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::PEN => "PEN",
            Self::ERASER => "ERASER",
            Self::BRUSH => "BRUSH",
            Self::PENCIL => "PENCIL",
            Self::AIRBRUSH => "AIRBRUSH",
            Self::FINGER => "FINGER",
            Self::MOUSE => "MOUSE",
            Self::LENS => "LENS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// capability flags for a tool
///
/// Describes extra capabilities on a tablet.
///
/// Any tool must provide x and y values, extra axes are
/// device-specific.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwpTabletToolV2Capability(pub u32);

impl ZwpTabletToolV2Capability {
    /// Tilt axes
    pub const TILT: Self = Self(1);

    /// Pressure axis
    pub const PRESSURE: Self = Self(2);

    /// Distance axis
    pub const DISTANCE: Self = Self(3);

    /// Z-rotation axis
    pub const ROTATION: Self = Self(4);

    /// Slider axis
    pub const SLIDER: Self = Self(5);

    /// Wheel axis
    pub const WHEEL: Self = Self(6);
}

impl Debug for ZwpTabletToolV2Capability {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::TILT => "TILT",
            Self::PRESSURE => "PRESSURE",
            Self::DISTANCE => "DISTANCE",
            Self::ROTATION => "ROTATION",
            Self::SLIDER => "SLIDER",
            Self::WHEEL => "WHEEL",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// physical button state
///
/// Describes the physical state of a button that produced the button event.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwpTabletToolV2ButtonState(pub u32);

impl ZwpTabletToolV2ButtonState {
    /// button is not pressed
    pub const RELEASED: Self = Self(0);

    /// button is pressed
    pub const PRESSED: Self = Self(1);
}

impl Debug for ZwpTabletToolV2ButtonState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::RELEASED => "RELEASED",
            Self::PRESSED => "PRESSED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwpTabletToolV2Error(pub u32);

impl ZwpTabletToolV2Error {
    /// given wl_surface has another role
    pub const ROLE: Self = Self(0);
}

impl Debug for ZwpTabletToolV2Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ROLE => "ROLE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
