//! idle notification manager
//!
//! This interface allows clients to monitor user idle status.
//!
//! After binding to this global, clients can create ext_idle_notification_v1
//! objects to get notified when the user is idle for a given amount of time.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A ext_idle_notifier_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ExtIdleNotifierV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ExtIdleNotifierV1Handler>,
}

struct DefaultHandler;

impl ExtIdleNotifierV1Handler for DefaultHandler { }

impl ExtIdleNotifierV1 {
    pub const XML_VERSION: u32 = 2;
    pub const INTERFACE: ObjectInterface = ObjectInterface::ExtIdleNotifierV1;
    pub const INTERFACE_NAME: &str = "ext_idle_notifier_v1";
}

impl ExtIdleNotifierV1 {
    pub fn set_handler(&self, handler: impl ExtIdleNotifierV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ExtIdleNotifierV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ExtIdleNotifierV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtIdleNotifierV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ExtIdleNotifierV1 {
    /// Since when the destroy message is available.
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_idle_notifier_v1#{}.destroy()\n", id);
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
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the get_idle_notification message is available.
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
        id: &Rc<ExtIdleNotificationV1>,
        timeout: u32,
        seat: &Rc<WlSeat>,
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_idle_notifier_v1#{}.get_idle_notification(id: ext_idle_notification_v1#{}, timeout: {}, seat: wl_seat#{})\n", id, arg0_id, arg1, arg2_id);
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
            arg0_id,
            arg1,
            arg2_id,
        ]);
        Ok(())
    }

    /// Since when the get_input_idle_notification message is available.
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
        id: &Rc<ExtIdleNotificationV1>,
        timeout: u32,
        seat: &Rc<WlSeat>,
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_idle_notifier_v1#{}.get_input_idle_notification(id: ext_idle_notification_v1#{}, timeout: {}, seat: wl_seat#{})\n", id, arg0_id, arg1, arg2_id);
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
            2,
            arg0_id,
            arg1,
            arg2_id,
        ]);
        Ok(())
    }
}

/// A message handler for [ExtIdleNotifierV1] proxies.
pub trait ExtIdleNotifierV1Handler: Any {
    /// destroy the manager
    ///
    /// Destroy the manager object. All objects created via this interface
    /// remain valid.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<ExtIdleNotifierV1>,
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
        _slf: &Rc<ExtIdleNotifierV1>,
        id: &Rc<ExtIdleNotificationV1>,
        timeout: u32,
        seat: &Rc<WlSeat>,
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
        _slf: &Rc<ExtIdleNotifierV1>,
        id: &Rc<ExtIdleNotificationV1>,
        timeout: u32,
        seat: &Rc<WlSeat>,
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

impl ObjectPrivate for ExtIdleNotifierV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ExtIdleNotifierV1, version),
            handler: Default::default(),
        })
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_idle_notifier_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
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
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_idle_notifier_v1#{}.get_idle_notification(id: ext_idle_notification_v1#{}, timeout: {}, seat: wl_seat#{})\n", client.endpoint.id, msg[0], arg0, arg1, arg2);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ExtIdleNotificationV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg2_id = arg2;
                let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg2_id));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg2_id).unwrap();
                    return Err(ObjectError::WrongObjectType("seat", o.core().interface, ObjectInterface::WlSeat));
                };
                let arg0 = &arg0;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).get_idle_notification(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.get_idle_notification(&self, arg0, arg1, arg2);
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
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_idle_notifier_v1#{}.get_input_idle_notification(id: ext_idle_notification_v1#{}, timeout: {}, seat: wl_seat#{})\n", client.endpoint.id, msg[0], arg0, arg1, arg2);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ExtIdleNotificationV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg2_id = arg2;
                let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg2_id));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg2_id).unwrap();
                    return Err(ObjectError::WrongObjectType("seat", o.core().interface, ObjectInterface::WlSeat));
                };
                let arg0 = &arg0;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).get_input_idle_notification(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.get_input_idle_notification(&self, arg0, arg1, arg2);
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

impl Object for ExtIdleNotifierV1 {
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

