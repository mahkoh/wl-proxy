use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_fullscreen_shell_mode_feedback_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpFullscreenShellModeFeedbackV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn ZwpFullscreenShellModeFeedbackV1Handler>,
}

struct DefaultHandler;

impl ZwpFullscreenShellModeFeedbackV1Handler for DefaultHandler { }

impl ZwpFullscreenShellModeFeedbackV1 {
    pub const XML_VERSION: u32 = 1;
}

impl ZwpFullscreenShellModeFeedbackV1 {
    pub fn set_handler(&self, handler: impl ZwpFullscreenShellModeFeedbackV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpFullscreenShellModeFeedbackV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpFullscreenShellModeFeedbackV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpFullscreenShellModeFeedbackV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpFullscreenShellModeFeedbackV1 {
    /// Since when the mode_successful message is available.
    #[allow(dead_code)]
    pub const MSG__MODE_SUCCESSFUL__SINCE: u32 = 1;

    /// mode switch succeeded
    ///
    /// This event indicates that the attempted mode switch operation was
    /// successful.  A surface of the size requested in the mode switch
    /// will fill the output without scaling.
    ///
    /// Upon receiving this event, the client should destroy the
    /// wl_fullscreen_shell_mode_feedback object.
    #[inline]
    pub fn send_mode_successful(
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
            let args = format_args!("[{millis:7}.{micros:03}] client#{:<4} <= zwp_fullscreen_shell_mode_feedback_v1#{}.mode_successful()\n", client.endpoint.id, id);
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
        drop(fmt);
        drop(outgoing_ref);
        drop(client_ref);
        self.core.handle_client_destroy();
        Ok(())
    }

    /// Since when the mode_failed message is available.
    #[allow(dead_code)]
    pub const MSG__MODE_FAILED__SINCE: u32 = 1;

    /// mode switch failed
    ///
    /// This event indicates that the attempted mode switch operation
    /// failed.  This may be because the requested output mode is not
    /// possible or it may mean that the compositor does not want to allow it.
    ///
    /// Upon receiving this event, the client should destroy the
    /// wl_fullscreen_shell_mode_feedback object.
    #[inline]
    pub fn send_mode_failed(
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
            let args = format_args!("[{millis:7}.{micros:03}] client#{:<4} <= zwp_fullscreen_shell_mode_feedback_v1#{}.mode_failed()\n", client.endpoint.id, id);
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
        drop(fmt);
        drop(outgoing_ref);
        drop(client_ref);
        self.core.handle_client_destroy();
        Ok(())
    }

    /// Since when the present_cancelled message is available.
    #[allow(dead_code)]
    pub const MSG__PRESENT_CANCELLED__SINCE: u32 = 1;

    /// mode switch cancelled
    ///
    /// This event indicates that the attempted mode switch operation was
    /// cancelled.  Most likely this is because the client requested a
    /// second mode switch before the first one completed.
    ///
    /// Upon receiving this event, the client should destroy the
    /// wl_fullscreen_shell_mode_feedback object.
    #[inline]
    pub fn send_present_cancelled(
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
            let args = format_args!("[{millis:7}.{micros:03}] client#{:<4} <= zwp_fullscreen_shell_mode_feedback_v1#{}.present_cancelled()\n", client.endpoint.id, id);
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
            2,
        ]);
        drop(fmt);
        drop(outgoing_ref);
        drop(client_ref);
        self.core.handle_client_destroy();
        Ok(())
    }
}

/// A message handler for [ZwpFullscreenShellModeFeedbackV1] proxies.
#[allow(dead_code)]
pub trait ZwpFullscreenShellModeFeedbackV1Handler: Any {
    /// mode switch succeeded
    ///
    /// This event indicates that the attempted mode switch operation was
    /// successful.  A surface of the size requested in the mode switch
    /// will fill the output without scaling.
    ///
    /// Upon receiving this event, the client should destroy the
    /// wl_fullscreen_shell_mode_feedback object.
    #[inline]
    fn mode_successful(
        &mut self,
        _slf: &Rc<ZwpFullscreenShellModeFeedbackV1>,
    ) {
        let res = _slf.send_mode_successful(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_fullscreen_shell_mode_feedback_v1.mode_successful message: {}", Report::new(e));
        }
    }

    /// mode switch failed
    ///
    /// This event indicates that the attempted mode switch operation
    /// failed.  This may be because the requested output mode is not
    /// possible or it may mean that the compositor does not want to allow it.
    ///
    /// Upon receiving this event, the client should destroy the
    /// wl_fullscreen_shell_mode_feedback object.
    #[inline]
    fn mode_failed(
        &mut self,
        _slf: &Rc<ZwpFullscreenShellModeFeedbackV1>,
    ) {
        let res = _slf.send_mode_failed(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_fullscreen_shell_mode_feedback_v1.mode_failed message: {}", Report::new(e));
        }
    }

    /// mode switch cancelled
    ///
    /// This event indicates that the attempted mode switch operation was
    /// cancelled.  Most likely this is because the client requested a
    /// second mode switch before the first one completed.
    ///
    /// Upon receiving this event, the client should destroy the
    /// wl_fullscreen_shell_mode_feedback object.
    #[inline]
    fn present_cancelled(
        &mut self,
        _slf: &Rc<ZwpFullscreenShellModeFeedbackV1>,
    ) {
        let res = _slf.send_present_cancelled(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_fullscreen_shell_mode_feedback_v1.present_cancelled message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for ZwpFullscreenShellModeFeedbackV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::ZwpFullscreenShellModeFeedbackV1, version),
            handler: Default::default(),
        })
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            n => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
    }

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let args = format_args!("[{millis:7}.{micros:03}] server      -> zwp_fullscreen_shell_mode_feedback_v1#{}.mode_successful()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).mode_successful(&self);
                } else {
                    DefaultHandler.mode_successful(&self);
                }
                self.core.handle_server_destroy();
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let args = format_args!("[{millis:7}.{micros:03}] server      -> zwp_fullscreen_shell_mode_feedback_v1#{}.mode_failed()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).mode_failed(&self);
                } else {
                    DefaultHandler.mode_failed(&self);
                }
                self.core.handle_server_destroy();
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let args = format_args!("[{millis:7}.{micros:03}] server      -> zwp_fullscreen_shell_mode_feedback_v1#{}.present_cancelled()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).present_cancelled(&self);
                } else {
                    DefaultHandler.present_cancelled(&self);
                }
                self.core.handle_server_destroy();
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
        let _ = id;
        None
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "mode_successful",
            1 => "mode_failed",
            2 => "present_cancelled",
            _ => return None,
        };
        Some(name)
    }
}

impl Proxy for ZwpFullscreenShellModeFeedbackV1 {
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

