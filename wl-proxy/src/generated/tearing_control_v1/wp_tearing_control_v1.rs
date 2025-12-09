//! per-surface tearing control interface
//!
//! An additional interface to a wl_surface object, which allows the client
//! to hint to the compositor if the content on the surface is suitable for
//! presentation with tearing.
//! The default presentation hint is vsync. See presentation_hint for more
//! details.
//!
//! If the associated wl_surface is destroyed, this object becomes inert and
//! should be destroyed.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_tearing_control_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpTearingControlV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpTearingControlV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpTearingControlV1MessageHandler for DefaultMessageHandler { }

impl MetaWpTearingControlV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWpTearingControlV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WpTearingControlV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaWpTearingControlV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaWpTearingControlV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpTearingControlV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpTearingControlV1 {
    /// Since when the set_presentation_hint message is available.
    #[allow(dead_code)]
    pub const MSG__SET_PRESENTATION_HINT__SINCE: u32 = 1;

    /// set presentation hint
    ///
    /// Set the presentation hint for the associated wl_surface. This state is
    /// double-buffered, see wl_surface.commit.
    ///
    /// The compositor is free to dynamically respect or ignore this hint based
    /// on various conditions like hardware capabilities, surface state and
    /// user preferences.
    ///
    /// # Arguments
    ///
    /// - `hint`:
    #[inline]
    pub fn send_set_presentation_hint(
        &self,
        hint: MetaWpTearingControlV1PresentationHint,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            hint,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy tearing control object
    ///
    /// Destroy this surface tearing object and revert the presentation hint to
    /// vsync. The change will be applied on the next wl_surface.commit.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
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
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [WpTearingControlV1] proxies.
#[allow(dead_code)]
pub trait MetaWpTearingControlV1MessageHandler {
    /// set presentation hint
    ///
    /// Set the presentation hint for the associated wl_surface. This state is
    /// double-buffered, see wl_surface.commit.
    ///
    /// The compositor is free to dynamically respect or ignore this hint based
    /// on various conditions like hardware capabilities, surface state and
    /// user preferences.
    ///
    /// # Arguments
    ///
    /// - `hint`:
    #[inline]
    fn set_presentation_hint(
        &mut self,
        _slf: &Rc<MetaWpTearingControlV1>,
        hint: MetaWpTearingControlV1PresentationHint,
    ) {
        let res = _slf.send_set_presentation_hint(
            hint,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_tearing_control_v1.set_presentation_hint message: {}", Report::new(e));
        }
    }

    /// destroy tearing control object
    ///
    /// Destroy this surface tearing object and revert the presentation hint to
    /// vsync. The change will be applied on the next wl_surface.commit.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpTearingControlV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_tearing_control_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpTearingControlV1 {
    fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Self::new(state, version)
    }

    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = MetaWpTearingControlV1PresentationHint(arg0);
                if let Some(handler) = handler {
                    (**handler).set_presentation_hint(&self, arg0);
                } else {
                    DefaultMessageHandler.set_presentation_hint(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
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
        let handler = &mut *self.handler.borrow();
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
            0 => "set_presentation_hint",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl MetaWpTearingControlV1 {
    /// Since when the presentation_hint.vsync enum variant is available.
    #[allow(dead_code)]
    pub const ENM__PRESENTATION_HINT_VSYNC__SINCE: u32 = 1;
    /// Since when the presentation_hint.async enum variant is available.
    #[allow(dead_code)]
    pub const ENM__PRESENTATION_HINT_ASYNC__SINCE: u32 = 1;
}

/// presentation hint values
///
/// This enum provides information for if submitted frames from the client
/// may be presented with tearing.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWpTearingControlV1PresentationHint(pub u32);

impl MetaWpTearingControlV1PresentationHint {
    /// tearing-free presentation
    ///
    /// The content of this surface is meant to be synchronized to the
    /// vertical blanking period. This should not result in visible tearing
    /// and may result in a delay before a surface commit is presented.
    #[allow(dead_code)]
    pub const VSYNC: Self = Self(0);

    /// asynchronous presentation
    ///
    /// The content of this surface is meant to be presented with minimal
    /// latency and tearing is acceptable.
    #[allow(dead_code)]
    pub const ASYNC: Self = Self(1);
}

impl Debug for MetaWpTearingControlV1PresentationHint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::VSYNC => "VSYNC",
            Self::ASYNC => "ASYNC",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
