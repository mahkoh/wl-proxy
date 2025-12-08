//! fractional scale interface to a wl_surface
//!
//! An additional interface to a wl_surface object which allows the compositor
//! to inform the client of the preferred scale.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_fractional_scale_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpFractionalScaleV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpFractionalScaleV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpFractionalScaleV1MessageHandler for DefaultMessageHandler { }

impl MetaWpFractionalScaleV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWpFractionalScaleV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WpFractionalScaleV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpFractionalScaleV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpFractionalScaleV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpFractionalScaleV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// remove surface scale information for surface
    ///
    /// Destroy the fractional scale object. When this object is destroyed,
    /// preferred_scale events will no longer be sent.
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

    /// Since when the preferred_scale message is available.
    #[allow(dead_code)]
    pub const MSG__PREFERRED_SCALE__SINCE: u32 = 1;

    /// notify of new preferred scale
    ///
    /// Notification of a new preferred scale for this surface that the
    /// compositor suggests that the client should use.
    ///
    /// The sent scale is the numerator of a fraction with a denominator of 120.
    ///
    /// # Arguments
    ///
    /// - `scale`: the new preferred scale
    #[inline]
    pub fn send_preferred_scale(
        &self,
        scale: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            scale,
        );
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
            arg0,
        ]);
        Ok(())
    }
}

/// A message handler for [WpFractionalScaleV1] proxies.
#[allow(dead_code)]
pub trait MetaWpFractionalScaleV1MessageHandler {
    /// remove surface scale information for surface
    ///
    /// Destroy the fractional scale object. When this object is destroyed,
    /// preferred_scale events will no longer be sent.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpFractionalScaleV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_fractional_scale_v1.destroy message: {}", Report::new(e));
        }
    }

    /// notify of new preferred scale
    ///
    /// Notification of a new preferred scale for this surface that the
    /// compositor suggests that the client should use.
    ///
    /// The sent scale is the numerator of a fraction with a denominator of 120.
    ///
    /// # Arguments
    ///
    /// - `scale`: the new preferred scale
    #[inline]
    fn preferred_scale(
        &mut self,
        _slf: &Rc<MetaWpFractionalScaleV1>,
        scale: u32,
    ) {
        let res = _slf.send_preferred_scale(
            scale,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_fractional_scale_v1.preferred_scale message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpFractionalScaleV1 {
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).preferred_scale(&self, arg0);
                } else {
                    DefaultMessageHandler.preferred_scale(&self, arg0);
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

