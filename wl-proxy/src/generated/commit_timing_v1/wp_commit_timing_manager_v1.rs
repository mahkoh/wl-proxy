//! commit timing
//!
//! When a compositor latches on to new content updates it will check for
//! any number of requirements of the available content updates (such as
//! fences of all buffers being signalled) to consider the update ready.
//!
//! This protocol provides a method for adding a time constraint to surface
//! content. This constraint indicates to the compositor that a content
//! update should be presented as closely as possible to, but not before,
//! a specified time.
//!
//! This protocol does not change the Wayland property that content
//! updates are applied in the order they are received, even when some
//! content updates contain timestamps and others do not.
//!
//! To provide timestamps, this global factory interface must be used to
//! acquire a wp_commit_timing_v1 object for a surface, which may then be
//! used to provide timestamp information for commits.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_commit_timing_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpCommitTimingManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpCommitTimingManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpCommitTimingManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaWpCommitTimingManagerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWpCommitTimingManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WpCommitTimingManagerV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpCommitTimingManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpCommitTimingManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpCommitTimingManagerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// unbind from the commit timing interface
    ///
    /// Informs the server that the client will no longer be using
    /// this protocol object. Existing objects created by this object
    /// are not affected.
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

    /// Since when the get_timer message is available.
    #[allow(dead_code)]
    pub const MSG__GET_TIMER__SINCE: u32 = 1;

    /// request commit timer interface for surface
    ///
    /// Establish a timing controller for a surface.
    ///
    /// Only one commit timer can be created for a surface, or a
    /// commit_timer_exists protocol error will be generated.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn send_get_timer(
        &self,
        id: &Rc<MetaWpCommitTimerV1>,
        surface: &Rc<MetaWlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            surface,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg1 = match arg1.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())?;
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
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
        ]);
        Ok(())
    }
}

/// A message handler for [WpCommitTimingManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaWpCommitTimingManagerV1MessageHandler {
    /// unbind from the commit timing interface
    ///
    /// Informs the server that the client will no longer be using
    /// this protocol object. Existing objects created by this object
    /// are not affected.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpCommitTimingManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_commit_timing_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// request commit timer interface for surface
    ///
    /// Establish a timing controller for a surface.
    ///
    /// Only one commit timer can be created for a surface, or a
    /// commit_timer_exists protocol error will be generated.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_timer(
        &mut self,
        _slf: &Rc<MetaWpCommitTimingManagerV1>,
        id: &Rc<MetaWpCommitTimerV1>,
        surface: &Rc<MetaWlSurface>,
    ) {
        let res = _slf.send_get_timer(
            id,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_commit_timing_manager_v1.get_timer message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpCommitTimingManagerV1 {
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
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaWpCommitTimerV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.endpoint.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_timer(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_timer(&self, arg0, arg1);
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

impl MetaWpCommitTimingManagerV1 {
    /// Since when the error.commit_timer_exists enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_COMMIT_TIMER_EXISTS__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWpCommitTimingManagerV1Error(pub u32);

impl MetaWpCommitTimingManagerV1Error {
    /// commit timer already exists for surface
    #[allow(dead_code)]
    pub const COMMIT_TIMER_EXISTS: Self = Self(0);
}

impl Debug for MetaWpCommitTimingManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::COMMIT_TIMER_EXISTS => "COMMIT_TIMER_EXISTS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
