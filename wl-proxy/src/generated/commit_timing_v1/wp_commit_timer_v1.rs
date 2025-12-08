//! Surface commit timer
//!
//! An object to set a time constraint for a content update on a surface.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_commit_timer_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpCommitTimerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpCommitTimerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpCommitTimerV1MessageHandler for DefaultMessageHandler { }

impl MetaWpCommitTimerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWpCommitTimerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WpCommitTimerV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpCommitTimerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpCommitTimerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpCommitTimerV1 {
    /// Since when the set_timestamp message is available.
    #[allow(dead_code)]
    pub const MSG__SET_TIMESTAMP__SINCE: u32 = 1;

    /// Specify time the following commit takes effect
    ///
    /// Provide a timing constraint for a surface content update.
    ///
    /// A set_timestamp request may be made before a wl_surface.commit to
    /// tell the compositor that the content is intended to be presented
    /// as closely as possible to, but not before, the specified time.
    /// The time is in the domain of the compositor's presentation clock.
    ///
    /// An invalid_timestamp error will be generated for invalid tv_nsec.
    ///
    /// If a timestamp already exists on the surface, a timestamp_exists
    /// error is generated.
    ///
    /// Requesting set_timestamp after the commit_timer object's surface is
    /// destroyed will generate a "surface_destroyed" error.
    ///
    /// # Arguments
    ///
    /// - `tv_sec_hi`: high 32 bits of the seconds part of target time
    /// - `tv_sec_lo`: low 32 bits of the seconds part of target time
    /// - `tv_nsec`: nanoseconds part of target time
    #[inline]
    pub fn send_set_timestamp(
        &self,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= wp_commit_timer_v1#{}.set_timestamp(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {})", id, arg0, arg1, arg2);
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
            arg0,
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// Destroy the timer
    ///
    /// Informs the server that the client will no longer be using
    /// this protocol object.
    ///
    /// Existing timing constraints are not affected by the destruction.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= wp_commit_timer_v1#{}.destroy()", id);
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

/// A message handler for [WpCommitTimerV1] proxies.
#[allow(dead_code)]
pub trait MetaWpCommitTimerV1MessageHandler {
    /// Specify time the following commit takes effect
    ///
    /// Provide a timing constraint for a surface content update.
    ///
    /// A set_timestamp request may be made before a wl_surface.commit to
    /// tell the compositor that the content is intended to be presented
    /// as closely as possible to, but not before, the specified time.
    /// The time is in the domain of the compositor's presentation clock.
    ///
    /// An invalid_timestamp error will be generated for invalid tv_nsec.
    ///
    /// If a timestamp already exists on the surface, a timestamp_exists
    /// error is generated.
    ///
    /// Requesting set_timestamp after the commit_timer object's surface is
    /// destroyed will generate a "surface_destroyed" error.
    ///
    /// # Arguments
    ///
    /// - `tv_sec_hi`: high 32 bits of the seconds part of target time
    /// - `tv_sec_lo`: low 32 bits of the seconds part of target time
    /// - `tv_nsec`: nanoseconds part of target time
    #[inline]
    fn set_timestamp(
        &mut self,
        _slf: &Rc<MetaWpCommitTimerV1>,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
    ) {
        let res = _slf.send_set_timestamp(
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_commit_timer_v1.set_timestamp message: {}", Report::new(e));
        }
    }

    /// Destroy the timer
    ///
    /// Informs the server that the client will no longer be using
    /// this protocol object.
    ///
    /// Existing timing constraints are not affected by the destruction.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpCommitTimerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_commit_timer_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpCommitTimerV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                eprintln!("client#{:04} -> wp_commit_timer_v1#{}.set_timestamp(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {})", client.endpoint.id, msg[0], arg0, arg1, arg2);
                if let Some(handler) = handler {
                    (**handler).set_timestamp(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.set_timestamp(&self, arg0, arg1, arg2);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("client#{:04} -> wp_commit_timer_v1#{}.destroy()", client.endpoint.id, msg[0]);
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
            0 => "set_timestamp",
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

impl MetaWpCommitTimerV1 {
    /// Since when the error.invalid_timestamp enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_TIMESTAMP__SINCE: u32 = 1;
    /// Since when the error.timestamp_exists enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_TIMESTAMP_EXISTS__SINCE: u32 = 1;
    /// Since when the error.surface_destroyed enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_SURFACE_DESTROYED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWpCommitTimerV1Error(pub u32);

impl MetaWpCommitTimerV1Error {
    /// timestamp contains an invalid value
    #[allow(dead_code)]
    pub const INVALID_TIMESTAMP: Self = Self(0);

    /// timestamp exists
    #[allow(dead_code)]
    pub const TIMESTAMP_EXISTS: Self = Self(1);

    /// the associated surface no longer exists
    #[allow(dead_code)]
    pub const SURFACE_DESTROYED: Self = Self(2);
}

impl Debug for MetaWpCommitTimerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_TIMESTAMP => "INVALID_TIMESTAMP",
            Self::TIMESTAMP_EXISTS => "TIMESTAMP_EXISTS",
            Self::SURFACE_DESTROYED => "SURFACE_DESTROYED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
