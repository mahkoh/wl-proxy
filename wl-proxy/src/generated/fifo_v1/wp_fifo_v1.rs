//! fifo interface
//!
//! A fifo object for a surface that may be used to add
//! display refresh constraints to content updates.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_fifo_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpFifoV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpFifoV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpFifoV1MessageHandler for DefaultMessageHandler { }

impl MetaWpFifoV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpFifoV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpFifoV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpFifoV1 {
    /// Since when the set_barrier message is available.
    #[allow(dead_code)]
    pub const MSG__SET_BARRIER__SINCE: u32 = 1;

    /// sets the start point for a fifo constraint
    ///
    /// When the content update containing the "set_barrier" is applied,
    /// it sets a "fifo_barrier" condition on the surface associated with
    /// the fifo object. The condition is cleared immediately after the
    /// following latching deadline for non-tearing presentation.
    ///
    /// The compositor may clear the condition early if it must do so to
    /// ensure client forward progress assumptions.
    ///
    /// To wait for this condition to clear, use the "wait_barrier" request.
    ///
    /// "set_barrier" is double-buffered state, see wl_surface.commit.
    ///
    /// Requesting set_barrier after the fifo object's surface is
    /// destroyed will generate a "surface_destroyed" error.
    #[inline]
    pub fn send_set_barrier(
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

    /// Since when the wait_barrier message is available.
    #[allow(dead_code)]
    pub const MSG__WAIT_BARRIER__SINCE: u32 = 1;

    /// adds a fifo constraint to a content update
    ///
    /// Indicate that this content update is not ready while a
    /// "fifo_barrier" condition is present on the surface.
    ///
    /// This means that when the content update containing "set_barrier"
    /// was made active at a latching deadline, it will be active for
    /// at least one refresh cycle. A content update which is allowed to
    /// tear might become active after a latching deadline if no content
    /// update became active at the deadline.
    ///
    /// The constraint must be ignored if the surface is a subsurface in
    /// synchronized mode. If the surface is not being updated by the
    /// compositor (off-screen, occluded) the compositor may ignore the
    /// constraint. Clients must use an additional mechanism such as
    /// frame callbacks or timestamps to ensure throttling occurs under
    /// all conditions.
    ///
    /// "wait_barrier" is double-buffered state, see wl_surface.commit.
    ///
    /// Requesting "wait_barrier" after the fifo object's surface is
    /// destroyed will generate a "surface_destroyed" error.
    #[inline]
    pub fn send_wait_barrier(
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
            1,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the fifo interface
    ///
    /// Informs the server that the client will no longer be using
    /// this protocol object.
    ///
    /// Surface state changes previously made by this protocol are
    /// unaffected by this object's destruction.
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
            2,
        ]);
        Ok(())
    }
}

/// A message handler for [WpFifoV1] proxies.
#[allow(dead_code)]
pub trait MetaWpFifoV1MessageHandler {
    /// sets the start point for a fifo constraint
    ///
    /// When the content update containing the "set_barrier" is applied,
    /// it sets a "fifo_barrier" condition on the surface associated with
    /// the fifo object. The condition is cleared immediately after the
    /// following latching deadline for non-tearing presentation.
    ///
    /// The compositor may clear the condition early if it must do so to
    /// ensure client forward progress assumptions.
    ///
    /// To wait for this condition to clear, use the "wait_barrier" request.
    ///
    /// "set_barrier" is double-buffered state, see wl_surface.commit.
    ///
    /// Requesting set_barrier after the fifo object's surface is
    /// destroyed will generate a "surface_destroyed" error.
    #[inline]
    fn set_barrier(
        &mut self,
        _slf: &Rc<MetaWpFifoV1>,
    ) {
        let res = _slf.send_set_barrier(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_fifo_v1.set_barrier message: {}", Report::new(e));
        }
    }

    /// adds a fifo constraint to a content update
    ///
    /// Indicate that this content update is not ready while a
    /// "fifo_barrier" condition is present on the surface.
    ///
    /// This means that when the content update containing "set_barrier"
    /// was made active at a latching deadline, it will be active for
    /// at least one refresh cycle. A content update which is allowed to
    /// tear might become active after a latching deadline if no content
    /// update became active at the deadline.
    ///
    /// The constraint must be ignored if the surface is a subsurface in
    /// synchronized mode. If the surface is not being updated by the
    /// compositor (off-screen, occluded) the compositor may ignore the
    /// constraint. Clients must use an additional mechanism such as
    /// frame callbacks or timestamps to ensure throttling occurs under
    /// all conditions.
    ///
    /// "wait_barrier" is double-buffered state, see wl_surface.commit.
    ///
    /// Requesting "wait_barrier" after the fifo object's surface is
    /// destroyed will generate a "surface_destroyed" error.
    #[inline]
    fn wait_barrier(
        &mut self,
        _slf: &Rc<MetaWpFifoV1>,
    ) {
        let res = _slf.send_wait_barrier(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_fifo_v1.wait_barrier message: {}", Report::new(e));
        }
    }

    /// destroy the fifo interface
    ///
    /// Informs the server that the client will no longer be using
    /// this protocol object.
    ///
    /// Surface state changes previously made by this protocol are
    /// unaffected by this object's destruction.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpFifoV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_fifo_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpFifoV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if let Some(handler) = handler {
                    (**handler).set_barrier(&self);
                } else {
                    DefaultMessageHandler.set_barrier(&self);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).wait_barrier(&self);
                } else {
                    DefaultMessageHandler.wait_barrier(&self);
                }
            }
            2 => {
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

impl MetaWpFifoV1 {
    /// Since when the error.surface_destroyed enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_SURFACE_DESTROYED__SINCE: u32 = 1;
}

/// fatal error
///
/// These fatal protocol errors may be emitted in response to
/// illegal requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWpFifoV1Error(pub u32);

impl MetaWpFifoV1Error {
    /// the associated surface no longer exists
    #[allow(dead_code)]
    pub const SURFACE_DESTROYED: Self = Self(0);
}

impl Debug for MetaWpFifoV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::SURFACE_DESTROYED => "SURFACE_DESTROYED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
