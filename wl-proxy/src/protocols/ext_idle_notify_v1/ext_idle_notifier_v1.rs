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

impl ConcreteObject for ExtIdleNotifierV1 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::ExtIdleNotifierV1;
    const INTERFACE_NAME: &str = "ext_idle_notifier_v1";
}

impl ExtIdleNotifierV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ExtIdleNotifierV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_idle_notifier_v1#{}.destroy()\n", id);
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

    /// destroy the manager
    ///
    /// Destroy the manager object. All objects created via this interface
    /// remain valid.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("ext_idle_notifier_v1.destroy", &e);
        }
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
    pub fn try_send_get_idle_notification(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg2_id = match arg2.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("seat"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_idle_notifier_v1#{}.get_idle_notification(id: ext_idle_notification_v1#{}, timeout: {}, seat: wl_seat#{})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1, arg2_id);
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
            arg1,
            arg2_id,
        ]);
        Ok(())
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
    #[inline]
    pub fn send_get_idle_notification(
        &self,
        id: &Rc<ExtIdleNotificationV1>,
        timeout: u32,
        seat: &Rc<WlSeat>,
    ) {
        let res = self.try_send_get_idle_notification(
            id,
            timeout,
            seat,
        );
        if let Err(e) = res {
            log_send("ext_idle_notifier_v1.get_idle_notification", &e);
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
    /// - `timeout`: minimum idle timeout in msec
    /// - `seat`:
    #[inline]
    pub fn new_try_send_get_idle_notification(
        &self,
        timeout: u32,
        seat: &Rc<WlSeat>,
    ) -> Result<Rc<ExtIdleNotificationV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_idle_notification(
            &id,
            timeout,
            seat,
        )?;
        Ok(id)
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
    /// - `timeout`: minimum idle timeout in msec
    /// - `seat`:
    #[inline]
    pub fn new_send_get_idle_notification(
        &self,
        timeout: u32,
        seat: &Rc<WlSeat>,
    ) -> Rc<ExtIdleNotificationV1> {
        let id = self.core.create_child();
        self.send_get_idle_notification(
            &id,
            timeout,
            seat,
        );
        id
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
    pub fn try_send_get_input_idle_notification(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg2_id = match arg2.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("seat"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_idle_notifier_v1#{}.get_input_idle_notification(id: ext_idle_notification_v1#{}, timeout: {}, seat: wl_seat#{})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1, arg2_id);
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
            arg1,
            arg2_id,
        ]);
        Ok(())
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
    #[inline]
    pub fn send_get_input_idle_notification(
        &self,
        id: &Rc<ExtIdleNotificationV1>,
        timeout: u32,
        seat: &Rc<WlSeat>,
    ) {
        let res = self.try_send_get_input_idle_notification(
            id,
            timeout,
            seat,
        );
        if let Err(e) = res {
            log_send("ext_idle_notifier_v1.get_input_idle_notification", &e);
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
    /// - `timeout`: minimum idle timeout in msec
    /// - `seat`:
    #[inline]
    pub fn new_try_send_get_input_idle_notification(
        &self,
        timeout: u32,
        seat: &Rc<WlSeat>,
    ) -> Result<Rc<ExtIdleNotificationV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_input_idle_notification(
            &id,
            timeout,
            seat,
        )?;
        Ok(id)
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
    /// - `timeout`: minimum idle timeout in msec
    /// - `seat`:
    #[inline]
    pub fn new_send_get_input_idle_notification(
        &self,
        timeout: u32,
        seat: &Rc<WlSeat>,
    ) -> Rc<ExtIdleNotificationV1> {
        let id = self.core.create_child();
        self.send_get_input_idle_notification(
            &id,
            timeout,
            seat,
        );
        id
    }
}

/// A message handler for [`ExtIdleNotifierV1`] proxies.
pub trait ExtIdleNotifierV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ExtIdleNotifierV1>) {
        slf.core.delete_id();
    }

    /// destroy the manager
    ///
    /// Destroy the manager object. All objects created via this interface
    /// remain valid.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ExtIdleNotifierV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("ext_idle_notifier_v1.destroy", &e);
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
    fn handle_get_idle_notification(
        &mut self,
        slf: &Rc<ExtIdleNotifierV1>,
        id: &Rc<ExtIdleNotificationV1>,
        timeout: u32,
        seat: &Rc<WlSeat>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_idle_notification(
            id,
            timeout,
            seat,
        );
        if let Err(e) = res {
            log_forward("ext_idle_notifier_v1.get_idle_notification", &e);
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
    fn handle_get_input_idle_notification(
        &mut self,
        slf: &Rc<ExtIdleNotifierV1>,
        id: &Rc<ExtIdleNotificationV1>,
        timeout: u32,
        seat: &Rc<WlSeat>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_input_idle_notification(
            id,
            timeout,
            seat,
        );
        if let Err(e) = res {
            log_forward("ext_idle_notifier_v1.get_input_idle_notification", &e);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_idle_notifier_v1#{}.destroy()\n", client_id, id);
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
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_idle_notifier_v1#{}.get_idle_notification(id: ext_idle_notification_v1#{}, timeout: {}, seat: wl_seat#{})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let arg0 = ExtIdleNotificationV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg2_id = arg2;
                let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg2_id)));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg2_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("seat", o.core().interface, ObjectInterface::WlSeat)));
                };
                let arg0 = &arg0;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).handle_get_idle_notification(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_get_idle_notification(&self, arg0, arg1, arg2);
                }
            }
            2 => {
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
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_idle_notifier_v1#{}.get_input_idle_notification(id: ext_idle_notification_v1#{}, timeout: {}, seat: wl_seat#{})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let arg0 = ExtIdleNotificationV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg2_id = arg2;
                let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg2_id)));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg2_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("seat", o.core().interface, ObjectInterface::WlSeat)));
                };
                let arg0 = &arg0;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).handle_get_input_idle_notification(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_get_input_idle_notification(&self, arg0, arg1, arg2);
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

