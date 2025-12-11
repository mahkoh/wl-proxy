//! fifo interface
//!
//! A fifo object for a surface that may be used to add
//! display refresh constraints to content updates.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_fifo_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpFifoV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn WpFifoV1Handler>,
}

struct DefaultHandler;

impl WpFifoV1Handler for DefaultHandler { }

impl WpFifoV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "wp_fifo_v1";
}

impl WpFifoV1 {
    pub fn set_handler(&self, handler: impl WpFifoV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn WpFifoV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpFifoV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpFifoV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpFifoV1 {
    /// Since when the set_barrier message is available.
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_fifo_v1#{}.set_barrier()\n", id);
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
        Ok(())
    }

    /// Since when the wait_barrier message is available.
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_fifo_v1#{}.wait_barrier()\n", id);
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
            1,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_fifo_v1#{}.destroy()\n", id);
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
            2,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [WpFifoV1] proxies.
pub trait WpFifoV1Handler: Any {
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
        _slf: &Rc<WpFifoV1>,
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
        _slf: &Rc<WpFifoV1>,
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
        _slf: &Rc<WpFifoV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_fifo_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for WpFifoV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::WpFifoV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_fifo_v1#{}.set_barrier()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_barrier(&self);
                } else {
                    DefaultHandler.set_barrier(&self);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_fifo_v1#{}.wait_barrier()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).wait_barrier(&self);
                } else {
                    DefaultHandler.wait_barrier(&self);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_fifo_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
            0 => "set_barrier",
            1 => "wait_barrier",
            2 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Proxy for WpFifoV1 {
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

impl WpFifoV1 {
    /// Since when the error.surface_destroyed enum variant is available.
    pub const ENM__ERROR_SURFACE_DESTROYED__SINCE: u32 = 1;
}

/// fatal error
///
/// These fatal protocol errors may be emitted in response to
/// illegal requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpFifoV1Error(pub u32);

impl WpFifoV1Error {
    /// the associated surface no longer exists
    pub const SURFACE_DESTROYED: Self = Self(0);
}

impl Debug for WpFifoV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::SURFACE_DESTROYED => "SURFACE_DESTROYED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
