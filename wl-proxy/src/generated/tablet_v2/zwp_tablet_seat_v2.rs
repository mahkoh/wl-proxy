//! controller object for graphic tablet devices of a seat
//!
//! An object that provides access to the graphics tablets available on this
//! seat. After binding to this interface, the compositor sends a set of
//! zwp_tablet_seat_v2.tablet_added and zwp_tablet_seat_v2.tool_added events.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_tablet_seat_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpTabletSeatV2 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpTabletSeatV2MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpTabletSeatV2MessageHandler for DefaultMessageHandler { }

impl MetaZwpTabletSeatV2 {
    pub const XML_VERSION: u32 = 2;
}

impl MetaZwpTabletSeatV2 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpTabletSeatV2, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaZwpTabletSeatV2MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaZwpTabletSeatV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpTabletSeatV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpTabletSeatV2 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// release the memory for the tablet seat object
    ///
    /// Destroy the zwp_tablet_seat_v2 object. Objects created from this
    /// object are unaffected and should be destroyed separately.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
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
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the tablet_added message is available.
    #[allow(dead_code)]
    pub const MSG__TABLET_ADDED__SINCE: u32 = 1;

    /// new device notification
    ///
    /// This event is sent whenever a new tablet becomes available on this
    /// seat. This event only provides the object id of the tablet, any
    /// static information about the tablet (device name, vid/pid, etc.) is
    /// sent through the zwp_tablet_v2 interface.
    #[inline]
    pub fn send_tablet_added(
        &self,
        id: &Rc<MetaZwpTabletV2>,
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateClientId("id", e))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the tool_added message is available.
    #[allow(dead_code)]
    pub const MSG__TOOL_ADDED__SINCE: u32 = 1;

    /// a new tool has been used with a tablet
    ///
    /// This event is sent whenever a tool that has not previously been used
    /// with a tablet comes into use. This event only provides the object id
    /// of the tool; any static information about the tool (capabilities,
    /// type, etc.) is sent through the zwp_tablet_tool_v2 interface.
    #[inline]
    pub fn send_tool_added(
        &self,
        id: &Rc<MetaZwpTabletToolV2>,
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateClientId("id", e))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the pad_added message is available.
    #[allow(dead_code)]
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
    #[inline]
    pub fn send_pad_added(
        &self,
        id: &Rc<MetaZwpTabletPadV2>,
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateClientId("id", e))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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
}

/// A message handler for [ZwpTabletSeatV2] proxies.
#[allow(dead_code)]
pub trait MetaZwpTabletSeatV2MessageHandler {
    /// release the memory for the tablet seat object
    ///
    /// Destroy the zwp_tablet_seat_v2 object. Objects created from this
    /// object are unaffected and should be destroyed separately.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpTabletSeatV2>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_seat_v2.destroy message: {}", Report::new(e));
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
    fn tablet_added(
        &mut self,
        _slf: &Rc<MetaZwpTabletSeatV2>,
        id: &Rc<MetaZwpTabletV2>,
    ) {
        let res = _slf.send_tablet_added(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_seat_v2.tablet_added message: {}", Report::new(e));
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
    fn tool_added(
        &mut self,
        _slf: &Rc<MetaZwpTabletSeatV2>,
        id: &Rc<MetaZwpTabletToolV2>,
    ) {
        let res = _slf.send_tool_added(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_seat_v2.tool_added message: {}", Report::new(e));
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
    fn pad_added(
        &mut self,
        _slf: &Rc<MetaZwpTabletSeatV2>,
        id: &Rc<MetaZwpTabletPadV2>,
    ) {
        let res = _slf.send_pad_added(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_seat_v2.pad_added message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpTabletSeatV2 {
    fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Self::new(state, version)
    }

    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
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
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0_id = arg0;
                let arg0 = MetaZwpTabletV2::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetServerId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).tablet_added(&self, arg0);
                } else {
                    DefaultMessageHandler.tablet_added(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0_id = arg0;
                let arg0 = MetaZwpTabletToolV2::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetServerId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).tool_added(&self, arg0);
                } else {
                    DefaultMessageHandler.tool_added(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0_id = arg0;
                let arg0 = MetaZwpTabletPadV2::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetServerId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).pad_added(&self, arg0);
                } else {
                    DefaultMessageHandler.pad_added(&self, arg0);
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

