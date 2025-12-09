//! idle notification manager
//!
//! This interface allows clients to monitor user idle status.
//!
//! After binding to this global, clients can create ext_idle_notification_v1
//! objects to get notified when the user is idle for a given amount of time.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_idle_notifier_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaExtIdleNotifierV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaExtIdleNotifierV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaExtIdleNotifierV1MessageHandler for DefaultMessageHandler { }

impl MetaExtIdleNotifierV1 {
    pub const XML_VERSION: u32 = 2;
}

impl MetaExtIdleNotifierV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ExtIdleNotifierV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaExtIdleNotifierV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaExtIdleNotifierV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaExtIdleNotifierV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaExtIdleNotifierV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the manager
    ///
    /// Destroy the manager object. All objects created via this interface
    /// remain valid.
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

    /// Since when the get_idle_notification message is available.
    #[allow(dead_code)]
    pub const MSG__GET_IDLE_NOTIFICATION__SINCE: u32 = 1;

    /// create a notification object
    ///
    /// Create a new idle notification object.
    ///
    /// The notification object has a minimum timeout duration and is tied to a
    /// seat. The client will be notified if the seat is inactive for at least
    /// the provided timeout. See ext_idle_notification_v1 for more details.
    ///
    /// A zero timeout is valid and means the client wants to be notified as
    /// soon as possible when the seat is inactive.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `timeout`: minimum idle timeout in msec
    /// - `seat`:
    #[inline]
    pub fn send_get_idle_notification(
        &self,
        id: &Rc<MetaExtIdleNotificationV1>,
        timeout: u32,
        seat: &Rc<MetaWlSeat>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            id,
            timeout,
            seat,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg2 = arg2.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg2_id = match arg2.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("seat")),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
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
            arg0_id,
            arg1,
            arg2_id,
        ]);
        Ok(())
    }

    /// Since when the get_input_idle_notification message is available.
    #[allow(dead_code)]
    pub const MSG__GET_INPUT_IDLE_NOTIFICATION__SINCE: u32 = 2;

    /// create a notification object
    ///
    /// Create a new idle notification object to track input from the
    /// user, such as keyboard and mouse movement. Because this object is
    /// meant to track user input alone, it ignores idle inhibitors.
    ///
    /// The notification object has a minimum timeout duration and is tied to a
    /// seat. The client will be notified if the seat is inactive for at least
    /// the provided timeout. See ext_idle_notification_v1 for more details.
    ///
    /// A zero timeout is valid and means the client wants to be notified as
    /// soon as possible when the seat is inactive.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `timeout`: minimum idle timeout in msec
    /// - `seat`:
    #[inline]
    pub fn send_get_input_idle_notification(
        &self,
        id: &Rc<MetaExtIdleNotificationV1>,
        timeout: u32,
        seat: &Rc<MetaWlSeat>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            id,
            timeout,
            seat,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg2 = arg2.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg2_id = match arg2.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("seat")),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        let endpoint = &self.core.state.server;
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
            arg1,
            arg2_id,
        ]);
        Ok(())
    }
}

/// A message handler for [ExtIdleNotifierV1] proxies.
#[allow(dead_code)]
pub trait MetaExtIdleNotifierV1MessageHandler {
    /// destroy the manager
    ///
    /// Destroy the manager object. All objects created via this interface
    /// remain valid.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaExtIdleNotifierV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_idle_notifier_v1.destroy message: {}", Report::new(e));
        }
    }

    /// create a notification object
    ///
    /// Create a new idle notification object.
    ///
    /// The notification object has a minimum timeout duration and is tied to a
    /// seat. The client will be notified if the seat is inactive for at least
    /// the provided timeout. See ext_idle_notification_v1 for more details.
    ///
    /// A zero timeout is valid and means the client wants to be notified as
    /// soon as possible when the seat is inactive.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `timeout`: minimum idle timeout in msec
    /// - `seat`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_idle_notification(
        &mut self,
        _slf: &Rc<MetaExtIdleNotifierV1>,
        id: &Rc<MetaExtIdleNotificationV1>,
        timeout: u32,
        seat: &Rc<MetaWlSeat>,
    ) {
        let res = _slf.send_get_idle_notification(
            id,
            timeout,
            seat,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_idle_notifier_v1.get_idle_notification message: {}", Report::new(e));
        }
    }

    /// create a notification object
    ///
    /// Create a new idle notification object to track input from the
    /// user, such as keyboard and mouse movement. Because this object is
    /// meant to track user input alone, it ignores idle inhibitors.
    ///
    /// The notification object has a minimum timeout duration and is tied to a
    /// seat. The client will be notified if the seat is inactive for at least
    /// the provided timeout. See ext_idle_notification_v1 for more details.
    ///
    /// A zero timeout is valid and means the client wants to be notified as
    /// soon as possible when the seat is inactive.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `timeout`: minimum idle timeout in msec
    /// - `seat`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_input_idle_notification(
        &mut self,
        _slf: &Rc<MetaExtIdleNotifierV1>,
        id: &Rc<MetaExtIdleNotificationV1>,
        timeout: u32,
        seat: &Rc<MetaWlSeat>,
    ) {
        let res = _slf.send_get_input_idle_notification(
            id,
            timeout,
            seat,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_idle_notifier_v1.get_input_idle_notification message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaExtIdleNotifierV1 {
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
            1 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                let arg0_id = arg0;
                let arg0 = MetaExtIdleNotificationV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg2_id = arg2;
                let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg2_id));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<MetaWlSeat>() else {
                    let o = client.endpoint.lookup(arg2_id).unwrap();
                    return Err(ObjectError::WrongObjectType("seat", o.core().interface, ProxyInterface::WlSeat));
                };
                let arg0 = &arg0;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).get_idle_notification(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.get_idle_notification(&self, arg0, arg1, arg2);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                let arg0_id = arg0;
                let arg0 = MetaExtIdleNotificationV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg2_id = arg2;
                let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg2_id));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<MetaWlSeat>() else {
                    let o = client.endpoint.lookup(arg2_id).unwrap();
                    return Err(ObjectError::WrongObjectType("seat", o.core().interface, ProxyInterface::WlSeat));
                };
                let arg0 = &arg0;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).get_input_idle_notification(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.get_input_idle_notification(&self, arg0, arg1, arg2);
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
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            1 => "get_idle_notification",
            2 => "get_input_idle_notification",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

