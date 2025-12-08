//! controller object for graphic tablet devices of a seat
//!
//! An object that provides access to the graphics tablets available on this
//! seat. After binding to this interface, the compositor sends a set of
//! zwp_tablet_seat_v2.tablet_added and zwp_tablet_seat_v2.tool_added events.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_tablet_seat_v2 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpTabletSeatV2 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpTabletSeatV2Handler>,
}

struct DefaultHandler;

impl ZwpTabletSeatV2Handler for DefaultHandler { }

impl ConcreteObject for ZwpTabletSeatV2 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpTabletSeatV2;
    const INTERFACE_NAME: &str = "zwp_tablet_seat_v2";
}

impl ZwpTabletSeatV2 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpTabletSeatV2Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpTabletSeatV2Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpTabletSeatV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpTabletSeatV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpTabletSeatV2 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// release the memory for the tablet seat object
    ///
    /// Destroy the zwp_tablet_seat_v2 object. Objects created from this
    /// object are unaffected and should be destroyed separately.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_tablet_seat_v2#{}.destroy()\n", id);
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

    /// release the memory for the tablet seat object
    ///
    /// Destroy the zwp_tablet_seat_v2 object. Objects created from this
    /// object are unaffected and should be destroyed separately.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwp_tablet_seat_v2.destroy", &e);
        }
    }

    /// Since when the tablet_added message is available.
    pub const MSG__TABLET_ADDED__SINCE: u32 = 1;

    /// new device notification
    ///
    /// This event is sent whenever a new tablet becomes available on this
    /// seat. This event only provides the object id of the tablet, any
    /// static information about the tablet (device name, vid/pid, etc.) is
    /// sent through the zwp_tablet_v2 interface.
    ///
    /// # Arguments
    ///
    /// - `id`: the newly added graphics tablet
    #[inline]
    pub fn try_send_tablet_added(
        &self,
        id: &Rc<ZwpTabletV2>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateClientId("id", e)))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_seat_v2#{}.tablet_added(id: zwp_tablet_v2#{})\n", client_id, id, arg0);
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
            0,
            arg0_id,
        ]);
        Ok(())
    }

    /// new device notification
    ///
    /// This event is sent whenever a new tablet becomes available on this
    /// seat. This event only provides the object id of the tablet, any
    /// static information about the tablet (device name, vid/pid, etc.) is
    /// sent through the zwp_tablet_v2 interface.
    ///
    /// # Arguments
    ///
    /// - `id`: the newly added graphics tablet
    #[inline]
    pub fn send_tablet_added(
        &self,
        id: &Rc<ZwpTabletV2>,
    ) {
        let res = self.try_send_tablet_added(
            id,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_seat_v2.tablet_added", &e);
        }
    }

    /// new device notification
    ///
    /// This event is sent whenever a new tablet becomes available on this
    /// seat. This event only provides the object id of the tablet, any
    /// static information about the tablet (device name, vid/pid, etc.) is
    /// sent through the zwp_tablet_v2 interface.
    #[inline]
    pub fn new_try_send_tablet_added(
        &self,
    ) -> Result<Rc<ZwpTabletV2>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_tablet_added(
            &id,
        )?;
        Ok(id)
    }

    /// new device notification
    ///
    /// This event is sent whenever a new tablet becomes available on this
    /// seat. This event only provides the object id of the tablet, any
    /// static information about the tablet (device name, vid/pid, etc.) is
    /// sent through the zwp_tablet_v2 interface.
    #[inline]
    pub fn new_send_tablet_added(
        &self,
    ) -> Rc<ZwpTabletV2> {
        let id = self.core.create_child();
        self.send_tablet_added(
            &id,
        );
        id
    }

    /// Since when the tool_added message is available.
    pub const MSG__TOOL_ADDED__SINCE: u32 = 1;

    /// a new tool has been used with a tablet
    ///
    /// This event is sent whenever a tool that has not previously been used
    /// with a tablet comes into use. This event only provides the object id
    /// of the tool; any static information about the tool (capabilities,
    /// type, etc.) is sent through the zwp_tablet_tool_v2 interface.
    ///
    /// # Arguments
    ///
    /// - `id`: the newly added tablet tool
    #[inline]
    pub fn try_send_tool_added(
        &self,
        id: &Rc<ZwpTabletToolV2>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateClientId("id", e)))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_seat_v2#{}.tool_added(id: zwp_tablet_tool_v2#{})\n", client_id, id, arg0);
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
            1,
            arg0_id,
        ]);
        Ok(())
    }

    /// a new tool has been used with a tablet
    ///
    /// This event is sent whenever a tool that has not previously been used
    /// with a tablet comes into use. This event only provides the object id
    /// of the tool; any static information about the tool (capabilities,
    /// type, etc.) is sent through the zwp_tablet_tool_v2 interface.
    ///
    /// # Arguments
    ///
    /// - `id`: the newly added tablet tool
    #[inline]
    pub fn send_tool_added(
        &self,
        id: &Rc<ZwpTabletToolV2>,
    ) {
        let res = self.try_send_tool_added(
            id,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_seat_v2.tool_added", &e);
        }
    }

    /// a new tool has been used with a tablet
    ///
    /// This event is sent whenever a tool that has not previously been used
    /// with a tablet comes into use. This event only provides the object id
    /// of the tool; any static information about the tool (capabilities,
    /// type, etc.) is sent through the zwp_tablet_tool_v2 interface.
    #[inline]
    pub fn new_try_send_tool_added(
        &self,
    ) -> Result<Rc<ZwpTabletToolV2>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_tool_added(
            &id,
        )?;
        Ok(id)
    }

    /// a new tool has been used with a tablet
    ///
    /// This event is sent whenever a tool that has not previously been used
    /// with a tablet comes into use. This event only provides the object id
    /// of the tool; any static information about the tool (capabilities,
    /// type, etc.) is sent through the zwp_tablet_tool_v2 interface.
    #[inline]
    pub fn new_send_tool_added(
        &self,
    ) -> Rc<ZwpTabletToolV2> {
        let id = self.core.create_child();
        self.send_tool_added(
            &id,
        );
        id
    }

    /// Since when the pad_added message is available.
    pub const MSG__PAD_ADDED__SINCE: u32 = 1;

    /// new pad notification
    ///
    /// This event is sent whenever a new pad is known to the system. Typically,
    /// pads are physically attached to tablets and a pad_added event is
    /// sent immediately after the zwp_tablet_seat_v2.tablet_added.
    /// However, some standalone pad devices logically attach to tablets at
    /// runtime, and the client must wait for zwp_tablet_pad_v2.enter to know
    /// the tablet a pad is attached to.
    ///
    /// This event only provides the object id of the pad. All further
    /// features (buttons, strips, rings) are sent through the zwp_tablet_pad_v2
    /// interface.
    ///
    /// # Arguments
    ///
    /// - `id`: the newly added pad
    #[inline]
    pub fn try_send_pad_added(
        &self,
        id: &Rc<ZwpTabletPadV2>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateClientId("id", e)))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_tablet_seat_v2#{}.pad_added(id: zwp_tablet_pad_v2#{})\n", client_id, id, arg0);
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
            2,
            arg0_id,
        ]);
        Ok(())
    }

    /// new pad notification
    ///
    /// This event is sent whenever a new pad is known to the system. Typically,
    /// pads are physically attached to tablets and a pad_added event is
    /// sent immediately after the zwp_tablet_seat_v2.tablet_added.
    /// However, some standalone pad devices logically attach to tablets at
    /// runtime, and the client must wait for zwp_tablet_pad_v2.enter to know
    /// the tablet a pad is attached to.
    ///
    /// This event only provides the object id of the pad. All further
    /// features (buttons, strips, rings) are sent through the zwp_tablet_pad_v2
    /// interface.
    ///
    /// # Arguments
    ///
    /// - `id`: the newly added pad
    #[inline]
    pub fn send_pad_added(
        &self,
        id: &Rc<ZwpTabletPadV2>,
    ) {
        let res = self.try_send_pad_added(
            id,
        );
        if let Err(e) = res {
            log_send("zwp_tablet_seat_v2.pad_added", &e);
        }
    }

    /// new pad notification
    ///
    /// This event is sent whenever a new pad is known to the system. Typically,
    /// pads are physically attached to tablets and a pad_added event is
    /// sent immediately after the zwp_tablet_seat_v2.tablet_added.
    /// However, some standalone pad devices logically attach to tablets at
    /// runtime, and the client must wait for zwp_tablet_pad_v2.enter to know
    /// the tablet a pad is attached to.
    ///
    /// This event only provides the object id of the pad. All further
    /// features (buttons, strips, rings) are sent through the zwp_tablet_pad_v2
    /// interface.
    #[inline]
    pub fn new_try_send_pad_added(
        &self,
    ) -> Result<Rc<ZwpTabletPadV2>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_pad_added(
            &id,
        )?;
        Ok(id)
    }

    /// new pad notification
    ///
    /// This event is sent whenever a new pad is known to the system. Typically,
    /// pads are physically attached to tablets and a pad_added event is
    /// sent immediately after the zwp_tablet_seat_v2.tablet_added.
    /// However, some standalone pad devices logically attach to tablets at
    /// runtime, and the client must wait for zwp_tablet_pad_v2.enter to know
    /// the tablet a pad is attached to.
    ///
    /// This event only provides the object id of the pad. All further
    /// features (buttons, strips, rings) are sent through the zwp_tablet_pad_v2
    /// interface.
    #[inline]
    pub fn new_send_pad_added(
        &self,
    ) -> Rc<ZwpTabletPadV2> {
        let id = self.core.create_child();
        self.send_pad_added(
            &id,
        );
        id
    }
}

/// A message handler for [`ZwpTabletSeatV2`] proxies.
pub trait ZwpTabletSeatV2Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpTabletSeatV2>) {
        slf.core.delete_id();
    }

    /// release the memory for the tablet seat object
    ///
    /// Destroy the zwp_tablet_seat_v2 object. Objects created from this
    /// object are unaffected and should be destroyed separately.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwpTabletSeatV2>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_seat_v2.destroy", &e);
        }
    }

    /// new device notification
    ///
    /// This event is sent whenever a new tablet becomes available on this
    /// seat. This event only provides the object id of the tablet, any
    /// static information about the tablet (device name, vid/pid, etc.) is
    /// sent through the zwp_tablet_v2 interface.
    ///
    /// # Arguments
    ///
    /// - `id`: the newly added graphics tablet
    #[inline]
    fn handle_tablet_added(
        &mut self,
        slf: &Rc<ZwpTabletSeatV2>,
        id: &Rc<ZwpTabletV2>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_tablet_added(
            id,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_seat_v2.tablet_added", &e);
        }
    }

    /// a new tool has been used with a tablet
    ///
    /// This event is sent whenever a tool that has not previously been used
    /// with a tablet comes into use. This event only provides the object id
    /// of the tool; any static information about the tool (capabilities,
    /// type, etc.) is sent through the zwp_tablet_tool_v2 interface.
    ///
    /// # Arguments
    ///
    /// - `id`: the newly added tablet tool
    #[inline]
    fn handle_tool_added(
        &mut self,
        slf: &Rc<ZwpTabletSeatV2>,
        id: &Rc<ZwpTabletToolV2>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_tool_added(
            id,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_seat_v2.tool_added", &e);
        }
    }

    /// new pad notification
    ///
    /// This event is sent whenever a new pad is known to the system. Typically,
    /// pads are physically attached to tablets and a pad_added event is
    /// sent immediately after the zwp_tablet_seat_v2.tablet_added.
    /// However, some standalone pad devices logically attach to tablets at
    /// runtime, and the client must wait for zwp_tablet_pad_v2.enter to know
    /// the tablet a pad is attached to.
    ///
    /// This event only provides the object id of the pad. All further
    /// features (buttons, strips, rings) are sent through the zwp_tablet_pad_v2
    /// interface.
    ///
    /// # Arguments
    ///
    /// - `id`: the newly added pad
    #[inline]
    fn handle_pad_added(
        &mut self,
        slf: &Rc<ZwpTabletSeatV2>,
        id: &Rc<ZwpTabletPadV2>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_pad_added(
            id,
        );
        if let Err(e) = res {
            log_forward("zwp_tablet_seat_v2.pad_added", &e);
        }
    }
}

impl ObjectPrivate for ZwpTabletSeatV2 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpTabletSeatV2, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_tablet_seat_v2#{}.destroy()\n", client_id, id);
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
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_seat_v2#{}.tablet_added(id: zwp_tablet_v2#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = ZwpTabletV2::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetServerId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_tablet_added(&self, arg0);
                } else {
                    DefaultHandler.handle_tablet_added(&self, arg0);
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
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_seat_v2#{}.tool_added(id: zwp_tablet_tool_v2#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = ZwpTabletToolV2::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetServerId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_tool_added(&self, arg0);
                } else {
                    DefaultHandler.handle_tool_added(&self, arg0);
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
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_tablet_seat_v2#{}.pad_added(id: zwp_tablet_pad_v2#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = ZwpTabletPadV2::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetServerId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_pad_added(&self, arg0);
                } else {
                    DefaultHandler.handle_pad_added(&self, arg0);
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
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "tablet_added",
            1 => "tool_added",
            2 => "pad_added",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwpTabletSeatV2 {
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

