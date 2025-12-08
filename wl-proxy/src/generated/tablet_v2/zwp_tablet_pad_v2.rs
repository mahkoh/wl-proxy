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

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_tablet_pad_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpTabletPadV2 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpTabletPadV2MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpTabletPadV2MessageHandler for DefaultMessageHandler { }

impl MetaZwpTabletPadV2 {
    pub const XML_VERSION: u32 = 2;
}

impl MetaZwpTabletPadV2 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpTabletPadV2, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwpTabletPadV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpTabletPadV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpTabletPadV2 {
    /// Since when the set_feedback message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
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
            arg0,
        ]);
        fmt.string(arg1);
        fmt.words([
            arg2,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
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
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the group message is available.
    #[allow(dead_code)]
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
        pad_group: &Rc<MetaZwpTabletPadGroupV2>,
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
            return Err(ObjectError);
        };
        arg0.generate_client_id(client, arg0_obj.clone())?;
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
            arg0.client_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }

    /// Since when the path message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            1,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the buttons message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            2,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the done message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            3,
        ]);
        Ok(())
    }

    /// Since when the button message is available.
    #[allow(dead_code)]
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
        state: MetaZwpTabletPadV2ButtonState,
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
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            4,
            arg0,
            arg1,
            arg2.0,
        ]);
        Ok(())
    }

    /// Since when the enter message is available.
    #[allow(dead_code)]
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
        tablet: &Rc<MetaZwpTabletV2>,
        surface: &Rc<MetaWlSurface>,
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
            return Err(ObjectError);
        };
        if arg1.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError);
        }
        if arg2.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError);
        }
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            5,
            arg0,
            arg1.client_obj_id.get().unwrap_or(0),
            arg2.client_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }

    /// Since when the leave message is available.
    #[allow(dead_code)]
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
        surface: &Rc<MetaWlSurface>,
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
            return Err(ObjectError);
        };
        if arg1.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError);
        }
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            6,
            arg0,
            arg1.client_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }

    /// Since when the removed message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            7,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpTabletPadV2] proxies.
#[allow(dead_code)]
pub trait MetaZwpTabletPadV2MessageHandler {
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
    fn set_feedback(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadV2>,
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
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadV2>,
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
    fn group(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadV2>,
        pad_group: &Rc<MetaZwpTabletPadGroupV2>,
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
    fn path(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadV2>,
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
    fn buttons(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadV2>,
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
    fn done(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadV2>,
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
    fn button(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadV2>,
        time: u32,
        button: u32,
        state: MetaZwpTabletPadV2ButtonState,
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
    fn enter(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadV2>,
        serial: u32,
        tablet: &Rc<MetaZwpTabletV2>,
        surface: &Rc<MetaWlSurface>,
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
    fn leave(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadV2>,
        serial: u32,
        surface: &Rc<MetaWlSurface>,
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
    fn removed(
        &mut self,
        _slf: &Rc<MetaZwpTabletPadV2>,
    ) {
        let res = _slf.send_removed(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_pad_v2.removed message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpTabletPadV2 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError);
                };
                offset += 1;
                let arg1 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError);
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError);
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError);
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError);
                        };
                        s
                    }
                };
                let Some(&arg2) = msg.get(offset) else {
                    return Err(ObjectError);
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError);
                }
                if let Some(handler) = handler {
                    (**handler).set_feedback(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.set_feedback(&self, arg0, arg1, arg2);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
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
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaZwpTabletPadGroupV2::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).group(&self, arg0);
                } else {
                    DefaultMessageHandler.group(&self, arg0);
                }
            }
            1 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError);
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError);
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError);
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError);
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError);
                }
                if let Some(handler) = handler {
                    (**handler).path(&self, arg0);
                } else {
                    DefaultMessageHandler.path(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).buttons(&self, arg0);
                } else {
                    DefaultMessageHandler.buttons(&self, arg0);
                }
            }
            3 => {
                if let Some(handler) = handler {
                    (**handler).done(&self);
                } else {
                    DefaultMessageHandler.done(&self);
                }
            }
            4 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg2 = MetaZwpTabletPadV2ButtonState(arg2);
                if let Some(handler) = handler {
                    (**handler).button(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.button(&self, arg0, arg1, arg2);
                }
            }
            5 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let Some(arg1) = self.core.state.server.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaZwpTabletV2>() else {
                    return Err(ObjectError);
                };
                let Some(arg2) = self.core.state.server.lookup(arg2) else {
                    return Err(ObjectError);
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    return Err(ObjectError);
                };
                let arg1 = &arg1;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).enter(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.enter(&self, arg0, arg1, arg2);
                }
            }
            6 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let Some(arg1) = self.core.state.server.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    return Err(ObjectError);
                };
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).leave(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.leave(&self, arg0, arg1);
                }
            }
            7 => {
                if let Some(handler) = handler {
                    (**handler).removed(&self);
                } else {
                    DefaultMessageHandler.removed(&self);
                }
            }
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }
}

impl MetaZwpTabletPadV2 {
    /// Since when the button_state.released enum variant is available.
    #[allow(dead_code)]
    pub const ENM__BUTTON_STATE_RELEASED__SINCE: u32 = 1;
    /// Since when the button_state.pressed enum variant is available.
    #[allow(dead_code)]
    pub const ENM__BUTTON_STATE_PRESSED__SINCE: u32 = 1;
}

/// physical button state
///
/// Describes the physical state of a button that caused the button
/// event.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaZwpTabletPadV2ButtonState(pub u32);

impl MetaZwpTabletPadV2ButtonState {
    /// the button is not pressed
    #[allow(dead_code)]
    pub const RELEASED: Self = Self(0);

    /// the button is pressed
    #[allow(dead_code)]
    pub const PRESSED: Self = Self(1);
}

impl Debug for MetaZwpTabletPadV2ButtonState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::RELEASED => "RELEASED",
            Self::PRESSED => "PRESSED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
