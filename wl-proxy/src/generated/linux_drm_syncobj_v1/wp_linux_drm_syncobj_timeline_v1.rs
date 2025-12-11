//! synchronization object timeline
//!
//! This object represents an explicit synchronization object timeline
//! imported by the client to the compositor.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_linux_drm_syncobj_timeline_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpLinuxDrmSyncobjTimelineV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn WpLinuxDrmSyncobjTimelineV1Handler>,
}

struct DefaultHandler;

impl WpLinuxDrmSyncobjTimelineV1Handler for DefaultHandler { }

impl WpLinuxDrmSyncobjTimelineV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "wp_linux_drm_syncobj_timeline_v1";
}

impl WpLinuxDrmSyncobjTimelineV1 {
    pub fn set_handler(&self, handler: impl WpLinuxDrmSyncobjTimelineV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn WpLinuxDrmSyncobjTimelineV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpLinuxDrmSyncobjTimelineV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpLinuxDrmSyncobjTimelineV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpLinuxDrmSyncobjTimelineV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the timeline
    ///
    /// Destroy the synchronization object timeline. Other objects are not
    /// affected by this request, in particular timeline points set by
    /// set_acquire_point and set_release_point are not unset.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_linux_drm_syncobj_timeline_v1#{}.destroy()\n", id);
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
}

/// A message handler for [WpLinuxDrmSyncobjTimelineV1] proxies.
pub trait WpLinuxDrmSyncobjTimelineV1Handler: Any {
    /// destroy the timeline
    ///
    /// Destroy the synchronization object timeline. Other objects are not
    /// affected by this request, in particular timeline points set by
    /// set_acquire_point and set_release_point are not unset.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<WpLinuxDrmSyncobjTimelineV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_linux_drm_syncobj_timeline_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for WpLinuxDrmSyncobjTimelineV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::WpLinuxDrmSyncobjTimelineV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_linux_drm_syncobj_timeline_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
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
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Proxy for WpLinuxDrmSyncobjTimelineV1 {
    fn core(&self) -> &ProxyCore {
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

