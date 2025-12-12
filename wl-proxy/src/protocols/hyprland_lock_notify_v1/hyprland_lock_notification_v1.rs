//! lock notification
//!
//! This interface is used by the compositor to send lock notification events
//! to clients.
//!
//! Typically the "locked" and "unlocked" events are emitted when a client
//! locks/unlocks the session via ext-session-lock, but the compositor may
//! choose to send notifications for any other locking mechanisms.
//!
//! The compositor must notfiy after possible transition periods
//! between locked and unlocked states of the session.
//! In the context of ext-session-lock, that means the "locked" event is
//! expected to be sent after the session-lock client has presented
//! a lock screen frame on every output, which corresponds to the "locked"
//! event of ext-session-lock.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A hyprland_lock_notification_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct HyprlandLockNotificationV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn HyprlandLockNotificationV1Handler>,
}

struct DefaultHandler;

impl HyprlandLockNotificationV1Handler for DefaultHandler { }

impl HyprlandLockNotificationV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::HyprlandLockNotificationV1;
    pub const INTERFACE_NAME: &str = "hyprland_lock_notification_v1";
}

impl HyprlandLockNotificationV1 {
    pub fn set_handler(&self, handler: impl HyprlandLockNotificationV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn HyprlandLockNotificationV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for HyprlandLockNotificationV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HyprlandLockNotificationV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl HyprlandLockNotificationV1 {
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_lock_notification_v1#{}.destroy()\n", id);
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

    /// Since when the locked message is available.
    pub const MSG__LOCKED__SINCE: u32 = 1;

    /// session is locked
    ///
    /// This event is sent when the wayland session is locked.
    ///
    /// It's a compositor protocol error to send this event twice without an
    /// unlock event in-between.
    #[inline]
    pub fn send_locked(
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= hyprland_lock_notification_v1#{}.locked()\n", client.endpoint.id, id);
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

    /// Since when the unlocked message is available.
    pub const MSG__UNLOCKED__SINCE: u32 = 1;

    /// session is no longer locked
    ///
    /// This event is sent when the wayland session is unlocked.
    ///
    /// It's a compositor protocol error to send this event twice without an
    /// locked event in-between. It's a compositor protocol error to send this
    /// event prior to any locked event.
    #[inline]
    pub fn send_unlocked(
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= hyprland_lock_notification_v1#{}.unlocked()\n", client.endpoint.id, id);
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

/// A message handler for [HyprlandLockNotificationV1] proxies.
pub trait HyprlandLockNotificationV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<HyprlandLockNotificationV1>) {
        let _ = slf.core.delete_id();
    }

    /// destroy the notification object
    ///
    /// Destroy the notification object.
    #[inline]
    fn handle_destroy(
        &mut self,
        _slf: &Rc<HyprlandLockNotificationV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a hyprland_lock_notification_v1.destroy message: {}", Report::new(e));
        }
    }

    /// session is locked
    ///
    /// This event is sent when the wayland session is locked.
    ///
    /// It's a compositor protocol error to send this event twice without an
    /// unlock event in-between.
    #[inline]
    fn handle_locked(
        &mut self,
        _slf: &Rc<HyprlandLockNotificationV1>,
    ) {
        let res = _slf.send_locked(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a hyprland_lock_notification_v1.locked message: {}", Report::new(e));
        }
    }

    /// session is no longer locked
    ///
    /// This event is sent when the wayland session is unlocked.
    ///
    /// It's a compositor protocol error to send this event twice without an
    /// locked event in-between. It's a compositor protocol error to send this
    /// event prior to any locked event.
    #[inline]
    fn handle_unlocked(
        &mut self,
        _slf: &Rc<HyprlandLockNotificationV1>,
    ) {
        let res = _slf.send_unlocked(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a hyprland_lock_notification_v1.unlocked message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for HyprlandLockNotificationV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::HyprlandLockNotificationV1, version),
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
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_lock_notification_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> hyprland_lock_notification_v1#{}.locked()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_locked(&self);
                } else {
                    DefaultHandler.handle_locked(&self);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> hyprland_lock_notification_v1#{}.unlocked()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_unlocked(&self);
                } else {
                    DefaultHandler.handle_unlocked(&self);
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
            0 => "locked",
            1 => "unlocked",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for HyprlandLockNotificationV1 {
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

