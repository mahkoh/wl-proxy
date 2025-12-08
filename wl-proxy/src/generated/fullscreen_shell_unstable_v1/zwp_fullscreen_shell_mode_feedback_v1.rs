use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_fullscreen_shell_mode_feedback_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpFullscreenShellModeFeedbackV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpFullscreenShellModeFeedbackV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpFullscreenShellModeFeedbackV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpFullscreenShellModeFeedbackV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwpFullscreenShellModeFeedbackV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpFullscreenShellModeFeedbackV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpFullscreenShellModeFeedbackV1 {
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
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
        ]);
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
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            1,
        ]);
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
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            2,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpFullscreenShellModeFeedbackV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpFullscreenShellModeFeedbackV1MessageHandler {
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
        _slf: &Rc<MetaZwpFullscreenShellModeFeedbackV1>,
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
        _slf: &Rc<MetaZwpFullscreenShellModeFeedbackV1>,
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
        _slf: &Rc<MetaZwpFullscreenShellModeFeedbackV1>,
    ) {
        let res = _slf.send_present_cancelled(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_fullscreen_shell_mode_feedback_v1.present_cancelled message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpFullscreenShellModeFeedbackV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            _ => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
    }

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if let Some(handler) = handler {
                    (**handler).mode_successful(&self);
                } else {
                    DefaultMessageHandler.mode_successful(&self);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).mode_failed(&self);
                } else {
                    DefaultMessageHandler.mode_failed(&self);
                }
            }
            2 => {
                if let Some(handler) = handler {
                    (**handler).present_cancelled(&self);
                } else {
                    DefaultMessageHandler.present_cancelled(&self);
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

