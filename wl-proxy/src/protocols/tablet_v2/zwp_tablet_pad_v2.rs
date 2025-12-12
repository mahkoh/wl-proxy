//! a set of buttons, rings, strips and dials
//!
//! A pad device is a set of buttons, rings, strips and dials
//! usually physically present on the tablet device itself. Some
//! exceptions exist where the pad device is physically detached, e.g. the
//! Wacom ExpressKey Remote.
//!
//! Pad devices have no axes that control the cursor and are generally
//! auxiliary devices to the tool devices used on the tablet surface.
//!
//! A pad device has a number of static characteristics, e.g. the number
//! of rings. These capabilities are sent in an event sequence after the
//! zwp_tablet_seat_v2.pad_added event before any actual events from this pad.
//! This initial event sequence is terminated by a zwp_tablet_pad_v2.done
//! event.
//!
//! All pad features (buttons, rings, strips and dials) are logically divided into
//! groups and all pads have at least one group. The available groups are
//! notified through the zwp_tablet_pad_v2.group event; the compositor will
//! emit one event per group before emitting zwp_tablet_pad_v2.done.
//!
//! Groups may have multiple modes. Modes allow clients to map multiple
//! actions to a single pad feature. Only one mode can be active per group,
//! although different groups may have different active modes.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_tablet_pad_v2 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpTabletPadV2 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpTabletPadV2Handler>,
}

struct DefaultHandler;

impl ZwpTabletPadV2Handler for DefaultHandler { }

impl ZwpTabletPadV2 {
    pub const XML_VERSION: u32 = 2;
    pub const INTERFACE: ObjectInterface = ObjectInterface::ZwpTabletPadV2;
    pub const INTERFACE_NAME: &str = "zwp_tablet_pad_v2";
}

impl ZwpTabletPadV2 {
    pub fn set_handler(&self, handler: impl ZwpTabletPadV2Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpTabletPadV2Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpTabletPadV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpTabletPadV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpTabletPadV2 {
    /// Since when the set_feedback message is available.
    pub const MSG__SET_FEEDBACK__SINCE: u32 = 1;

    /// set compositor feedback
    ///
    /// Requests the compositor to use the provided feedback string
    /// associated with this button. This request should be issued immediately
    /// after a zwp_tablet_pad_group_v2.mode_switch event from the corresponding
    /// group is received, or whenever a button is mapped to a different
    /// action. See zwp_tablet_pad_group_v2.mode_switch for more details.
    ///
    /// Clients are encouraged to provide context-aware descriptions for
    /// the actions associated with each button, and compositors may use
    /// this information to offer visual feedback on the button layout
    /// (e.g. on-screen displays).
    ///
    /// Button indices start at 0. Setting the feedback string on a button
    /// that is reserved by the compositor (i.e. not belonging to any
    /// zwp_tablet_pad_group_v2) does not generate an error but the compositor
    /// is free to ignore the request.
    ///
    /// The provided string 'description' is a UTF-8 encoded string to be
    /// associated with this ring, and is considered user-visible; general
    /// internationalization rules apply.
    ///
    /// The serial argument will be that of the last
    /// zwp_tablet_pad_group_v2.mode_switch event received for the group of this
    /// button. Requests providing other serials than the most recent one will
    /// be ignored.
    ///
    /// # Arguments
    ///
    /// - `button`: button index
    /// - `description`: button description
    /// - `serial`: serial of the mode switch event
    #[inline]
    pub fn send_set_feedback(
        &self,
        button: u32,
        description: &str,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            button,
            description,
            serial,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_tablet_pad_v2#{}.set_feedback(button: {}, description: {:?}, serial: {})\n", id, arg0, arg1, arg2);
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
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
        fmt.string(arg1);
        fmt.words([
            arg2,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the pad object
    ///
    /// Destroy the zwp_tablet_pad_v2 object. Objects created from this object
    /// are unaffected and should be destroyed separately.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_tablet_pad_v2#{}.destroy()\n", id);
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
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

    /// Since when the group message is available.
    pub const MSG__GROUP__SINCE: u32 = 1;

    /// group announced
    ///
    /// Sent on zwp_tablet_pad_v2 initialization to announce available groups.
    /// One event is sent for each pad group available.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_v2.done event. At least one group will be announced.
    #[inline]
    pub fn send_group(
        &self,
        pad_group: &Rc<ZwpTabletPadGroupV2>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            pad_group,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateClientId("pad_group", e))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_pad_v2#{}.group(pad_group: zwp_tablet_pad_group_v2#{})\n", client.endpoint.id, id, arg0_id);
            self.core.state.log(args);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the path message is available.
    pub const MSG__PATH__SINCE: u32 = 1;

    /// path to the device
    ///
    /// A system-specific device path that indicates which device is behind
    /// this zwp_tablet_pad_v2. This information may be used to gather additional
    /// information about the device, e.g. through libwacom.
    ///
    /// The format of the path is unspecified, it may be a device node, a
    /// sysfs path, or some other identifier. It is up to the client to
    /// identify the string provided.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_v2.done event.
    ///
    /// # Arguments
    ///
    /// - `path`: path to local device
    #[inline]
    pub fn send_path(
        &self,
        path: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            path,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_pad_v2#{}.path(path: {:?})\n", client.endpoint.id, id, arg0);
            self.core.state.log(args);
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

    /// Since when the buttons message is available.
    pub const MSG__BUTTONS__SINCE: u32 = 1;

    /// buttons announced
    ///
    /// Sent on zwp_tablet_pad_v2 initialization to announce the available
    /// buttons.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_v2.done event. This event is only sent when at least one
    /// button is available.
    ///
    /// # Arguments
    ///
    /// - `buttons`: the number of buttons
    #[inline]
    pub fn send_buttons(
        &self,
        buttons: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            buttons,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_pad_v2#{}.buttons(buttons: {})\n", client.endpoint.id, id, arg0);
            self.core.state.log(args);
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
        ]);
        Ok(())
    }

    /// Since when the done message is available.
    pub const MSG__DONE__SINCE: u32 = 1;

    /// pad description event sequence complete
    ///
    /// This event signals the end of the initial burst of descriptive
    /// events. A client may consider the static description of the pad to
    /// be complete and finalize initialization of the pad.
    #[inline]
    pub fn send_done(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_pad_v2#{}.done()\n", client.endpoint.id, id);
            self.core.state.log(args);
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

    /// Since when the button message is available.
    pub const MSG__BUTTON__SINCE: u32 = 1;

    /// physical button state
    ///
    /// Sent whenever the physical state of a button changes.
    ///
    /// # Arguments
    ///
    /// - `time`: the time of the event with millisecond granularity
    /// - `button`: the index of the button that changed state
    /// - `state`:
    #[inline]
    pub fn send_button(
        &self,
        time: u32,
        button: u32,
        state: ZwpTabletPadV2ButtonState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            time,
            button,
            state,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_pad_v2#{}.button(time: {}, button: {}, state: {:?})\n", client.endpoint.id, id, arg0, arg1, arg2);
            self.core.state.log(args);
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
            arg0,
            arg1,
            arg2.0,
        ]);
        Ok(())
    }

    /// Since when the enter message is available.
    pub const MSG__ENTER__SINCE: u32 = 1;

    /// enter event
    ///
    /// Notification that this pad is focused on the specified surface.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the enter event
    /// - `tablet`: the tablet the pad is attached to
    /// - `surface`: surface the pad is focused on
    #[inline]
    pub fn send_enter(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg1.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError::ArgNoClientId("tablet", client.endpoint.id));
        }
        if arg2.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError::ArgNoClientId("surface", client.endpoint.id));
        }
        let arg1_id = arg1.client_obj_id.get().unwrap_or(0);
        let arg2_id = arg2.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_pad_v2#{}.enter(serial: {}, tablet: zwp_tablet_v2#{}, surface: wl_surface#{})\n", client.endpoint.id, id, arg0, arg1_id, arg2_id);
            self.core.state.log(args);
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
            arg0,
            arg1_id,
            arg2_id,
        ]);
        Ok(())
    }

    /// Since when the leave message is available.
    pub const MSG__LEAVE__SINCE: u32 = 1;

    /// leave event
    ///
    /// Notification that this pad is no longer focused on the specified
    /// surface.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the leave event
    /// - `surface`: surface the pad is no longer focused on
    #[inline]
    pub fn send_leave(
        &self,
        serial: u32,
        surface: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            serial,
            surface,
        );
        let arg1 = arg1.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg1.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError::ArgNoClientId("surface", client.endpoint.id));
        }
        let arg1_id = arg1.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_pad_v2#{}.leave(serial: {}, surface: wl_surface#{})\n", client.endpoint.id, id, arg0, arg1_id);
            self.core.state.log(args);
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
        ]);
        Ok(())
    }

    /// Since when the removed message is available.
    pub const MSG__REMOVED__SINCE: u32 = 1;

    /// pad removed event
    ///
    /// Sent when the pad has been removed from the system. When a tablet
    /// is removed its pad(s) will be removed too.
    ///
    /// When this event is received, the client must destroy all rings, strips
    /// and groups that were offered by this pad, and issue zwp_tablet_pad_v2.destroy
    /// the pad itself.
    #[inline]
    pub fn send_removed(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_pad_v2#{}.removed()\n", client.endpoint.id, id);
            self.core.state.log(args);
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
}

/// A message handler for [ZwpTabletPadV2] proxies.
pub trait ZwpTabletPadV2Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpTabletPadV2>) {
        let _ = slf.core.delete_id();
    }

    /// set compositor feedback
    ///
    /// Requests the compositor to use the provided feedback string
    /// associated with this button. This request should be issued immediately
    /// after a zwp_tablet_pad_group_v2.mode_switch event from the corresponding
    /// group is received, or whenever a button is mapped to a different
    /// action. See zwp_tablet_pad_group_v2.mode_switch for more details.
    ///
    /// Clients are encouraged to provide context-aware descriptions for
    /// the actions associated with each button, and compositors may use
    /// this information to offer visual feedback on the button layout
    /// (e.g. on-screen displays).
    ///
    /// Button indices start at 0. Setting the feedback string on a button
    /// that is reserved by the compositor (i.e. not belonging to any
    /// zwp_tablet_pad_group_v2) does not generate an error but the compositor
    /// is free to ignore the request.
    ///
    /// The provided string 'description' is a UTF-8 encoded string to be
    /// associated with this ring, and is considered user-visible; general
    /// internationalization rules apply.
    ///
    /// The serial argument will be that of the last
    /// zwp_tablet_pad_group_v2.mode_switch event received for the group of this
    /// button. Requests providing other serials than the most recent one will
    /// be ignored.
    ///
    /// # Arguments
    ///
    /// - `button`: button index
    /// - `description`: button description
    /// - `serial`: serial of the mode switch event
    #[inline]
    fn handle_set_feedback(
        &mut self,
        _slf: &Rc<ZwpTabletPadV2>,
        button: u32,
        description: &str,
        serial: u32,
    ) {
        let res = _slf.send_set_feedback(
            button,
            description,
            serial,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_v2.set_feedback message: {}", Report::new(e));
        }
    }

    /// destroy the pad object
    ///
    /// Destroy the zwp_tablet_pad_v2 object. Objects created from this object
    /// are unaffected and should be destroyed separately.
    #[inline]
    fn handle_destroy(
        &mut self,
        _slf: &Rc<ZwpTabletPadV2>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_v2.destroy message: {}", Report::new(e));
        }
    }

    /// group announced
    ///
    /// Sent on zwp_tablet_pad_v2 initialization to announce available groups.
    /// One event is sent for each pad group available.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_v2.done event. At least one group will be announced.
    ///
    /// # Arguments
    ///
    /// - `pad_group`:
    #[inline]
    fn handle_group(
        &mut self,
        _slf: &Rc<ZwpTabletPadV2>,
        pad_group: &Rc<ZwpTabletPadGroupV2>,
    ) {
        let res = _slf.send_group(
            pad_group,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_v2.group message: {}", Report::new(e));
        }
    }

    /// path to the device
    ///
    /// A system-specific device path that indicates which device is behind
    /// this zwp_tablet_pad_v2. This information may be used to gather additional
    /// information about the device, e.g. through libwacom.
    ///
    /// The format of the path is unspecified, it may be a device node, a
    /// sysfs path, or some other identifier. It is up to the client to
    /// identify the string provided.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_v2.done event.
    ///
    /// # Arguments
    ///
    /// - `path`: path to local device
    #[inline]
    fn handle_path(
        &mut self,
        _slf: &Rc<ZwpTabletPadV2>,
        path: &str,
    ) {
        let res = _slf.send_path(
            path,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_v2.path message: {}", Report::new(e));
        }
    }

    /// buttons announced
    ///
    /// Sent on zwp_tablet_pad_v2 initialization to announce the available
    /// buttons.
    ///
    /// This event is sent in the initial burst of events before the
    /// zwp_tablet_pad_v2.done event. This event is only sent when at least one
    /// button is available.
    ///
    /// # Arguments
    ///
    /// - `buttons`: the number of buttons
    #[inline]
    fn handle_buttons(
        &mut self,
        _slf: &Rc<ZwpTabletPadV2>,
        buttons: u32,
    ) {
        let res = _slf.send_buttons(
            buttons,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_v2.buttons message: {}", Report::new(e));
        }
    }

    /// pad description event sequence complete
    ///
    /// This event signals the end of the initial burst of descriptive
    /// events. A client may consider the static description of the pad to
    /// be complete and finalize initialization of the pad.
    #[inline]
    fn handle_done(
        &mut self,
        _slf: &Rc<ZwpTabletPadV2>,
    ) {
        let res = _slf.send_done(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_v2.done message: {}", Report::new(e));
        }
    }

    /// physical button state
    ///
    /// Sent whenever the physical state of a button changes.
    ///
    /// # Arguments
    ///
    /// - `time`: the time of the event with millisecond granularity
    /// - `button`: the index of the button that changed state
    /// - `state`:
    #[inline]
    fn handle_button(
        &mut self,
        _slf: &Rc<ZwpTabletPadV2>,
        time: u32,
        button: u32,
        state: ZwpTabletPadV2ButtonState,
    ) {
        let res = _slf.send_button(
            time,
            button,
            state,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_v2.button message: {}", Report::new(e));
        }
    }

    /// enter event
    ///
    /// Notification that this pad is focused on the specified surface.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the enter event
    /// - `tablet`: the tablet the pad is attached to
    /// - `surface`: surface the pad is focused on
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_enter(
        &mut self,
        _slf: &Rc<ZwpTabletPadV2>,
        serial: u32,
        tablet: &Rc<ZwpTabletV2>,
        surface: &Rc<WlSurface>,
    ) {
        if let Some(client_id) = _slf.core.client_id.get() {
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
        let res = _slf.send_enter(
            serial,
            tablet,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_v2.enter message: {}", Report::new(e));
        }
    }

    /// leave event
    ///
    /// Notification that this pad is no longer focused on the specified
    /// surface.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the leave event
    /// - `surface`: surface the pad is no longer focused on
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_leave(
        &mut self,
        _slf: &Rc<ZwpTabletPadV2>,
        serial: u32,
        surface: &Rc<WlSurface>,
    ) {
        if let Some(client_id) = _slf.core.client_id.get() {
            if let Some(client_id_2) = surface.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = _slf.send_leave(
            serial,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_v2.leave message: {}", Report::new(e));
        }
    }

    /// pad removed event
    ///
    /// Sent when the pad has been removed from the system. When a tablet
    /// is removed its pad(s) will be removed too.
    ///
    /// When this event is received, the client must destroy all rings, strips
    /// and groups that were offered by this pad, and issue zwp_tablet_pad_v2.destroy
    /// the pad itself.
    #[inline]
    fn handle_removed(
        &mut self,
        _slf: &Rc<ZwpTabletPadV2>,
    ) {
        let res = _slf.send_removed(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_v2.removed message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for ZwpTabletPadV2 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpTabletPadV2, version),
            handler: Default::default(),
        })
    }

    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err((ObjectError::HandlerBorrowed, self));
        };
        if let Some(handler) = &mut *handler {
            handler.delete_id(&self);
        } else {
            let _ = self.core.delete_id();
        }
        Ok(())
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("button"));
                };
                offset += 1;
                let arg1 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("description"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("description"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("description"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("description"));
                        };
                        s
                    }
                };
                let Some(&arg2) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("serial"));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_tablet_pad_v2#{}.set_feedback(button: {}, description: {:?}, serial: {})\n", client.endpoint.id, msg[0], arg0, arg1, arg2);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_feedback(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_set_feedback(&self, arg0, arg1, arg2);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_tablet_pad_v2#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
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
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_pad_v2#{}.group(pad_group: zwp_tablet_pad_group_v2#{})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ZwpTabletPadGroupV2::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetServerId(arg0_id, "pad_group", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_group(&self, arg0);
                } else {
                    DefaultHandler.handle_group(&self, arg0);
                }
            }
            1 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("path"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("path"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("path"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("path"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_pad_v2#{}.path(path: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_path(&self, arg0);
                } else {
                    DefaultHandler.handle_path(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_pad_v2#{}.buttons(buttons: {})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_buttons(&self, arg0);
                } else {
                    DefaultHandler.handle_buttons(&self, arg0);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_pad_v2#{}.done()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_done(&self);
                } else {
                    DefaultHandler.handle_done(&self);
                }
            }
            4 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                let arg2 = ZwpTabletPadV2ButtonState(arg2);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_pad_v2#{}.button(time: {}, button: {}, state: {:?})\n", msg[0], arg0, arg1, arg2);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_button(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_button(&self, arg0, arg1, arg2);
                }
            }
            5 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_pad_v2#{}.enter(serial: {}, tablet: zwp_tablet_v2#{}, surface: wl_surface#{})\n", msg[0], arg0, arg1, arg2);
                    self.core.state.log(args);
                }
                let arg1_id = arg1;
                let Some(arg1) = self.core.state.server.lookup(arg1_id) else {
                    return Err(ObjectError::NoServerObject(arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<ZwpTabletV2>() else {
                    let o = self.core.state.server.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("tablet", o.core().interface, ObjectInterface::ZwpTabletV2));
                };
                let arg2_id = arg2;
                let Some(arg2) = self.core.state.server.lookup(arg2_id) else {
                    return Err(ObjectError::NoServerObject(arg2_id));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = self.core.state.server.lookup(arg2_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface));
                };
                let arg1 = &arg1;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).handle_enter(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_enter(&self, arg0, arg1, arg2);
                }
            }
            6 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_pad_v2#{}.leave(serial: {}, surface: wl_surface#{})\n", msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg1_id = arg1;
                let Some(arg1) = self.core.state.server.lookup(arg1_id) else {
                    return Err(ObjectError::NoServerObject(arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = self.core.state.server.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface));
                };
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_leave(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_leave(&self, arg0, arg1);
                }
            }
            7 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_pad_v2#{}.removed()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_removed(&self);
                } else {
                    DefaultHandler.handle_removed(&self);
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
            0 => "set_feedback",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "group",
            1 => "path",
            2 => "buttons",
            3 => "done",
            4 => "button",
            5 => "enter",
            6 => "leave",
            7 => "removed",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwpTabletPadV2 {
    fn core(&self) -> &ObjectCore {
        &self.core
    }

    fn unset_handler(&self) {
        self.handler.set(None);
    }

    fn get_handler_any_ref(&self) -> Result<Ref<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(Ref::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<RefMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow_mut().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(RefMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

impl ZwpTabletPadV2 {
    /// Since when the button_state.released enum variant is available.
    pub const ENM__BUTTON_STATE_RELEASED__SINCE: u32 = 1;
    /// Since when the button_state.pressed enum variant is available.
    pub const ENM__BUTTON_STATE_PRESSED__SINCE: u32 = 1;
}

/// physical button state
///
/// Describes the physical state of a button that caused the button
/// event.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwpTabletPadV2ButtonState(pub u32);

impl ZwpTabletPadV2ButtonState {
    /// the button is not pressed
    pub const RELEASED: Self = Self(0);

    /// the button is pressed
    pub const PRESSED: Self = Self(1);
}

impl Debug for ZwpTabletPadV2ButtonState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::RELEASED => "RELEASED",
            Self::PRESSED => "PRESSED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
