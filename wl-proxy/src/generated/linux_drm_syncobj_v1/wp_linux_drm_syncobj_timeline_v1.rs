//! synchronization object timeline
//!
//! This object represents an explicit synchronization object timeline
//! imported by the client to the compositor.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_linux_drm_syncobj_timeline_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpLinuxDrmSyncobjTimelineV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpLinuxDrmSyncobjTimelineV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpLinuxDrmSyncobjTimelineV1MessageHandler for DefaultMessageHandler { }

impl MetaWpLinuxDrmSyncobjTimelineV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpLinuxDrmSyncobjTimelineV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpLinuxDrmSyncobjTimelineV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpLinuxDrmSyncobjTimelineV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
        ]);
        Ok(())
    }
}

/// A message handler for [WpLinuxDrmSyncobjTimelineV1] proxies.
#[allow(dead_code)]
pub trait MetaWpLinuxDrmSyncobjTimelineV1MessageHandler {
    /// destroy the timeline
    ///
    /// Destroy the synchronization object timeline. Other objects are not
    /// affected by this request, in particular timeline points set by
    /// set_acquire_point and set_release_point are not unset.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpLinuxDrmSyncobjTimelineV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_linux_drm_syncobj_timeline_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpLinuxDrmSyncobjTimelineV1 {
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
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
    }
}

