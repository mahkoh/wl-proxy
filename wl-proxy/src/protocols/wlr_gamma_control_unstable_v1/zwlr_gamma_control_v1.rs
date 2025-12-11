//! adjust gamma tables for an output
//!
//! This interface allows a client to adjust gamma tables for a particular
//! output.
//!
//! The client will receive the gamma size, and will then be able to set gamma
//! tables. At any time the compositor can send a failed event indicating that
//! this object is no longer valid.
//!
//! There can only be at most one gamma control object per output, which
//! has exclusive access to this particular output. When the gamma control
//! object is destroyed, the gamma table is restored to its original value.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwlr_gamma_control_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwlrGammaControlV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn ZwlrGammaControlV1Handler>,
}

struct DefaultHandler;

impl ZwlrGammaControlV1Handler for DefaultHandler { }

impl ZwlrGammaControlV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "zwlr_gamma_control_v1";
}

impl ZwlrGammaControlV1 {
    pub fn set_handler(&self, handler: impl ZwlrGammaControlV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZwlrGammaControlV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwlrGammaControlV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwlrGammaControlV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwlrGammaControlV1 {
    /// Since when the gamma_size message is available.
    pub const MSG__GAMMA_SIZE__SINCE: u32 = 1;

    /// size of gamma ramps
    ///
    /// Advertise the size of each gamma ramp.
    ///
    /// This event is sent immediately when the gamma control object is created.
    ///
    /// # Arguments
    ///
    /// - `size`: number of elements in a ramp
    #[inline]
    pub fn send_gamma_size(
        &self,
        size: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            size,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_gamma_control_v1#{}.gamma_size(size: {})\n", client.endpoint.id, id, arg0);
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
            arg0,
        ]);
        Ok(())
    }

    /// Since when the set_gamma message is available.
    pub const MSG__SET_GAMMA__SINCE: u32 = 1;

    /// set the gamma table
    ///
    /// Set the gamma table. The file descriptor can be memory-mapped to provide
    /// the raw gamma table, which contains successive gamma ramps for the red,
    /// green and blue channels. Each gamma ramp is an array of 16-byte unsigned
    /// integers which has the same length as the gamma size.
    ///
    /// The file descriptor data must have the same length as three times the
    /// gamma size.
    ///
    /// # Arguments
    ///
    /// - `fd`: gamma table file descriptor
    #[inline]
    pub fn send_set_gamma(
        &self,
        fd: &Rc<OwnedFd>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            fd,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_gamma_control_v1#{}.set_gamma(fd: {})\n", id, arg0.as_raw_fd());
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.fds.push_back(arg0.clone());
        fmt.words([
            id,
            0,
        ]);
        Ok(())
    }

    /// Since when the failed message is available.
    pub const MSG__FAILED__SINCE: u32 = 1;

    /// object no longer valid
    ///
    /// This event indicates that the gamma control is no longer valid. This
    /// can happen for a number of reasons, including:
    /// - The output doesn't support gamma tables
    /// - Setting the gamma tables failed
    /// - Another client already has exclusive gamma control for this output
    /// - The compositor has transferred gamma control to another client
    ///
    /// Upon receiving this event, the client should destroy this object.
    #[inline]
    pub fn send_failed(
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
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_gamma_control_v1#{}.failed()\n", client.endpoint.id, id);
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
        Ok(())
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy this control
    ///
    /// Destroys the gamma control object. If the object is still valid, this
    /// restores the original gamma tables.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_gamma_control_v1#{}.destroy()\n", id);
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
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [ZwlrGammaControlV1] proxies.
pub trait ZwlrGammaControlV1Handler: Any {
    /// size of gamma ramps
    ///
    /// Advertise the size of each gamma ramp.
    ///
    /// This event is sent immediately when the gamma control object is created.
    ///
    /// # Arguments
    ///
    /// - `size`: number of elements in a ramp
    #[inline]
    fn gamma_size(
        &mut self,
        _slf: &Rc<ZwlrGammaControlV1>,
        size: u32,
    ) {
        let res = _slf.send_gamma_size(
            size,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_gamma_control_v1.gamma_size message: {}", Report::new(e));
        }
    }

    /// set the gamma table
    ///
    /// Set the gamma table. The file descriptor can be memory-mapped to provide
    /// the raw gamma table, which contains successive gamma ramps for the red,
    /// green and blue channels. Each gamma ramp is an array of 16-byte unsigned
    /// integers which has the same length as the gamma size.
    ///
    /// The file descriptor data must have the same length as three times the
    /// gamma size.
    ///
    /// # Arguments
    ///
    /// - `fd`: gamma table file descriptor
    #[inline]
    fn set_gamma(
        &mut self,
        _slf: &Rc<ZwlrGammaControlV1>,
        fd: &Rc<OwnedFd>,
    ) {
        let res = _slf.send_set_gamma(
            fd,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_gamma_control_v1.set_gamma message: {}", Report::new(e));
        }
    }

    /// object no longer valid
    ///
    /// This event indicates that the gamma control is no longer valid. This
    /// can happen for a number of reasons, including:
    /// - The output doesn't support gamma tables
    /// - Setting the gamma tables failed
    /// - Another client already has exclusive gamma control for this output
    /// - The compositor has transferred gamma control to another client
    ///
    /// Upon receiving this event, the client should destroy this object.
    #[inline]
    fn failed(
        &mut self,
        _slf: &Rc<ZwlrGammaControlV1>,
    ) {
        let res = _slf.send_failed(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_gamma_control_v1.failed message: {}", Report::new(e));
        }
    }

    /// destroy this control
    ///
    /// Destroys the gamma control object. If the object is still valid, this
    /// restores the original gamma tables.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<ZwlrGammaControlV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_gamma_control_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for ZwlrGammaControlV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::ZwlrGammaControlV1, version),
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
                let Some(arg0) = fds.pop_front() else {
                    return Err(ObjectError::MissingFd("fd"));
                };
                let arg0 = &arg0;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_gamma_control_v1#{}.set_gamma(fd: {})\n", client.endpoint.id, msg[0], arg0.as_raw_fd());
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_gamma(&self, arg0);
                } else {
                    DefaultHandler.set_gamma(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_gamma_control_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_gamma_control_v1#{}.gamma_size(size: {})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).gamma_size(&self, arg0);
                } else {
                    DefaultHandler.gamma_size(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_gamma_control_v1#{}.failed()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).failed(&self);
                } else {
                    DefaultHandler.failed(&self);
                }
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
        let name = match id {
            0 => "set_gamma",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "gamma_size",
            1 => "failed",
            _ => return None,
        };
        Some(name)
    }
}

impl Proxy for ZwlrGammaControlV1 {
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

impl ZwlrGammaControlV1 {
    /// Since when the error.invalid_gamma enum variant is available.
    pub const ENM__ERROR_INVALID_GAMMA__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwlrGammaControlV1Error(pub u32);

impl ZwlrGammaControlV1Error {
    /// invalid gamma tables
    pub const INVALID_GAMMA: Self = Self(1);
}

impl Debug for ZwlrGammaControlV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_GAMMA => "INVALID_GAMMA",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
