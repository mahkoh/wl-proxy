use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_fullscreen_shell_mode_feedback_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpFullscreenShellModeFeedbackV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpFullscreenShellModeFeedbackV1Handler>,
}

struct DefaultHandler;

impl ZwpFullscreenShellModeFeedbackV1Handler for DefaultHandler { }

impl ConcreteObject for ZwpFullscreenShellModeFeedbackV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpFullscreenShellModeFeedbackV1;
    const INTERFACE_NAME: &str = "zwp_fullscreen_shell_mode_feedback_v1";
}

impl ZwpFullscreenShellModeFeedbackV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpFullscreenShellModeFeedbackV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
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
    pub fn try_send_mode_successful(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_fullscreen_shell_mode_feedback_v1#{}.mode_successful()\n", client_id, id);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id);
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
    ) {
        let res = self.try_send_mode_successful(
        );
        if let Err(e) = res {
            log_send("zwp_fullscreen_shell_mode_feedback_v1.mode_successful", &e);
        }
    }

    /// Since when the mode_failed message is available.
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
    pub fn try_send_mode_failed(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_fullscreen_shell_mode_feedback_v1#{}.mode_failed()\n", client_id, id);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id);
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
    ) {
        let res = self.try_send_mode_failed(
        );
        if let Err(e) = res {
            log_send("zwp_fullscreen_shell_mode_feedback_v1.mode_failed", &e);
        }
    }

    /// Since when the present_cancelled message is available.
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
    pub fn try_send_present_cancelled(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_fullscreen_shell_mode_feedback_v1#{}.present_cancelled()\n", client_id, id);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id);
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
    ) {
        let res = self.try_send_present_cancelled(
        );
        if let Err(e) = res {
            log_send("zwp_fullscreen_shell_mode_feedback_v1.present_cancelled", &e);
        }
    }
}

/// A message handler for [`ZwpFullscreenShellModeFeedbackV1`] proxies.
pub trait ZwpFullscreenShellModeFeedbackV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpFullscreenShellModeFeedbackV1>) {
        slf.core.delete_id();
    }

    /// mode switch succeeded
    ///
    /// This event indicates that the attempted mode switch operation was
    /// successful.  A surface of the size requested in the mode switch
    /// will fill the output without scaling.
    ///
    /// Upon receiving this event, the client should destroy the
    /// wl_fullscreen_shell_mode_feedback object.
    #[inline]
    fn handle_mode_successful(
        &mut self,
        slf: &Rc<ZwpFullscreenShellModeFeedbackV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_mode_successful(
        );
        if let Err(e) = res {
            log_forward("zwp_fullscreen_shell_mode_feedback_v1.mode_successful", &e);
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
    fn handle_mode_failed(
        &mut self,
        slf: &Rc<ZwpFullscreenShellModeFeedbackV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_mode_failed(
        );
        if let Err(e) = res {
            log_forward("zwp_fullscreen_shell_mode_feedback_v1.mode_failed", &e);
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
    fn handle_present_cancelled(
        &mut self,
        slf: &Rc<ZwpFullscreenShellModeFeedbackV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_present_cancelled(
        );
        if let Err(e) = res {
            log_forward("zwp_fullscreen_shell_mode_feedback_v1.present_cancelled", &e);
        }
    }
}

impl ObjectPrivate for ZwpFullscreenShellModeFeedbackV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpFullscreenShellModeFeedbackV1, version),
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
            n => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
    }

    fn handle_event(self: Rc<Self>, server: &Endpoint, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
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
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_fullscreen_shell_mode_feedback_v1#{}.mode_successful()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                self.core.handle_server_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_mode_successful(&self);
                } else {
                    DefaultHandler.handle_mode_successful(&self);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_fullscreen_shell_mode_feedback_v1#{}.mode_failed()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                self.core.handle_server_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_mode_failed(&self);
                } else {
                    DefaultHandler.handle_mode_failed(&self);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_fullscreen_shell_mode_feedback_v1#{}.present_cancelled()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                self.core.handle_server_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_present_cancelled(&self);
                } else {
                    DefaultHandler.handle_present_cancelled(&self);
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

impl Object for ZwpFullscreenShellModeFeedbackV1 {
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

