//! idle notification
//!
//! This interface is used by the compositor to send idle notification events
//! to clients.
//!
//! Initially the notification object is not idle. The notification object
//! becomes idle when no user activity has happened for at least the timeout
//! duration, starting from the creation of the notification object. User
//! activity may include input events or a presence sensor, but is
//! compositor-specific.
//!
//! How this notification responds to idle inhibitors depends on how
//! it was constructed. If constructed from the
//! get_idle_notification request, then if an idle inhibitor is
//! active (e.g. another client has created a zwp_idle_inhibitor_v1
//! on a visible surface), the compositor must not make the
//! notification object idle. However, if constructed from the
//! get_input_idle_notification request, then idle inhibitors are
//! ignored, and only input from the user, e.g. from a keyboard or
//! mouse, counts as activity.
//!
//! When the notification object becomes idle, an idled event is sent. When
//! user activity starts again, the notification object stops being idle,
//! a resumed event is sent and the timeout is restarted.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A ext_idle_notification_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ExtIdleNotificationV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ExtIdleNotificationV1Handler>,
}

struct DefaultHandler;

impl ExtIdleNotificationV1Handler for DefaultHandler { }

impl ExtIdleNotificationV1 {
    pub const XML_VERSION: u32 = 2;
    pub const INTERFACE: ObjectInterface = ObjectInterface::ExtIdleNotificationV1;
    pub const INTERFACE_NAME: &str = "ext_idle_notification_v1";
}

impl ExtIdleNotificationV1 {
    pub fn set_handler(&self, handler: impl ExtIdleNotificationV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ExtIdleNotificationV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ExtIdleNotificationV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtIdleNotificationV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ExtIdleNotificationV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the notification object
    ///
    /// Destroy the notification object.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_idle_notification_v1#{}.destroy()\n", id);
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

    /// Since when the idled message is available.
    pub const MSG__IDLED__SINCE: u32 = 1;

    /// notification object is idle
    ///
    /// This event is sent when the notification object becomes idle.
    ///
    /// It's a compositor protocol error to send this event twice without a
    /// resumed event in-between.
    #[inline]
    pub fn send_idled(
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_idle_notification_v1#{}.idled()\n", client.endpoint.id, id);
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
        ]);
        Ok(())
    }

    /// Since when the resumed message is available.
    pub const MSG__RESUMED__SINCE: u32 = 1;

    /// notification object is no longer idle
    ///
    /// This event is sent when the notification object stops being idle.
    ///
    /// It's a compositor protocol error to send this event twice without an
    /// idled event in-between. It's a compositor protocol error to send this
    /// event prior to any idled event.
    #[inline]
    pub fn send_resumed(
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_idle_notification_v1#{}.resumed()\n", client.endpoint.id, id);
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
        Ok(())
    }
}

/// A message handler for [ExtIdleNotificationV1] proxies.
pub trait ExtIdleNotificationV1Handler: Any {
    /// destroy the notification object
    ///
    /// Destroy the notification object.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<ExtIdleNotificationV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_idle_notification_v1.destroy message: {}", Report::new(e));
        }
    }

    /// notification object is idle
    ///
    /// This event is sent when the notification object becomes idle.
    ///
    /// It's a compositor protocol error to send this event twice without a
    /// resumed event in-between.
    #[inline]
    fn idled(
        &mut self,
        _slf: &Rc<ExtIdleNotificationV1>,
    ) {
        let res = _slf.send_idled(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_idle_notification_v1.idled message: {}", Report::new(e));
        }
    }

    /// notification object is no longer idle
    ///
    /// This event is sent when the notification object stops being idle.
    ///
    /// It's a compositor protocol error to send this event twice without an
    /// idled event in-between. It's a compositor protocol error to send this
    /// event prior to any idled event.
    #[inline]
    fn resumed(
        &mut self,
        _slf: &Rc<ExtIdleNotificationV1>,
    ) {
        let res = _slf.send_resumed(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_idle_notification_v1.resumed message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for ExtIdleNotificationV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ExtIdleNotificationV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_idle_notification_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
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
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_idle_notification_v1#{}.idled()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).idled(&self);
                } else {
                    DefaultHandler.idled(&self);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_idle_notification_v1#{}.resumed()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).resumed(&self);
                } else {
                    DefaultHandler.resumed(&self);
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
            0 => "idled",
            1 => "resumed",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ExtIdleNotificationV1 {
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

