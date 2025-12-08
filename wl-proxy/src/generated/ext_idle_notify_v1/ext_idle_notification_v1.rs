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

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_idle_notification_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaExtIdleNotificationV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaExtIdleNotificationV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaExtIdleNotificationV1MessageHandler for DefaultMessageHandler { }

impl MetaExtIdleNotificationV1 {
    pub const XML_VERSION: u32 = 2;
}

impl MetaExtIdleNotificationV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ExtIdleNotificationV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaExtIdleNotificationV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaExtIdleNotificationV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaExtIdleNotificationV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
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
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the idled message is available.
    #[allow(dead_code)]
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
            0,
        ]);
        Ok(())
    }

    /// Since when the resumed message is available.
    #[allow(dead_code)]
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
        Ok(())
    }
}

/// A message handler for [ExtIdleNotificationV1] proxies.
#[allow(dead_code)]
pub trait MetaExtIdleNotificationV1MessageHandler {
    /// destroy the notification object
    ///
    /// Destroy the notification object.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaExtIdleNotificationV1>,
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
        _slf: &Rc<MetaExtIdleNotificationV1>,
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
        _slf: &Rc<MetaExtIdleNotificationV1>,
    ) {
        let res = _slf.send_resumed(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_idle_notification_v1.resumed message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaExtIdleNotificationV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
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
                if let Some(handler) = handler {
                    (**handler).idled(&self);
                } else {
                    DefaultMessageHandler.idled(&self);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).resumed(&self);
                } else {
                    DefaultMessageHandler.resumed(&self);
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

