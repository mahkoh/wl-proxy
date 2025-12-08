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

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwlr_gamma_control_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwlrGammaControlV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwlrGammaControlV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwlrGammaControlV1MessageHandler for DefaultMessageHandler { }

impl MetaZwlrGammaControlV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwlrGammaControlV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwlrGammaControlV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwlrGammaControlV1 {
    /// Since when the gamma_size message is available.
    #[allow(dead_code)]
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
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the set_gamma message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.fds.push_back(arg0.clone());
        fmt.words([
            id,
            0,
        ]);
        Ok(())
    }

    /// Since when the failed message is available.
    #[allow(dead_code)]
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

    /// Since when the destroy message is available.
    #[allow(dead_code)]
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
}

/// A message handler for [ZwlrGammaControlV1] proxies.
#[allow(dead_code)]
pub trait MetaZwlrGammaControlV1MessageHandler {
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
        _slf: &Rc<MetaZwlrGammaControlV1>,
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
        _slf: &Rc<MetaZwlrGammaControlV1>,
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
        _slf: &Rc<MetaZwlrGammaControlV1>,
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
        _slf: &Rc<MetaZwlrGammaControlV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_gamma_control_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwlrGammaControlV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let Some(arg0) = fds.pop_front() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).set_gamma(&self, arg0);
                } else {
                    DefaultMessageHandler.set_gamma(&self, arg0);
                }
            }
            1 => {
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
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).gamma_size(&self, arg0);
                } else {
                    DefaultMessageHandler.gamma_size(&self, arg0);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).failed(&self);
                } else {
                    DefaultMessageHandler.failed(&self);
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

impl MetaZwlrGammaControlV1 {
    /// Since when the error.invalid_gamma enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_GAMMA__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaZwlrGammaControlV1Error(pub u32);

impl MetaZwlrGammaControlV1Error {
    /// invalid gamma tables
    #[allow(dead_code)]
    pub const INVALID_GAMMA: Self = Self(1);
}

impl Debug for MetaZwlrGammaControlV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_GAMMA => "INVALID_GAMMA",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
