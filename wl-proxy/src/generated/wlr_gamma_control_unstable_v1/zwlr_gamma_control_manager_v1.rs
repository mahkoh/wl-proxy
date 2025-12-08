//! manager to create per-output gamma controls
//!
//! This interface is a manager that allows creating per-output gamma
//! controls.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwlr_gamma_control_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwlrGammaControlManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwlrGammaControlManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwlrGammaControlManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaZwlrGammaControlManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwlrGammaControlManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwlrGammaControlManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwlrGammaControlManagerV1 {
    /// Since when the get_gamma_control message is available.
    #[allow(dead_code)]
    pub const MSG__GET_GAMMA_CONTROL__SINCE: u32 = 1;

    /// get a gamma control for an output
    ///
    /// Create a gamma control that can be used to adjust gamma tables for the
    /// provided output.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `output`:
    #[inline]
    pub fn send_get_gamma_control(
        &self,
        id: &Rc<MetaZwlrGammaControlV1>,
        output: &Rc<MetaWlOutput>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            output,
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
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the manager
    ///
    /// All objects created by the manager will still remain valid, until their
    /// appropriate destroy request has been called.
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

/// A message handler for [ZwlrGammaControlManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaZwlrGammaControlManagerV1MessageHandler {
    /// get a gamma control for an output
    ///
    /// Create a gamma control that can be used to adjust gamma tables for the
    /// provided output.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `output`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_gamma_control(
        &mut self,
        _slf: &Rc<MetaZwlrGammaControlManagerV1>,
        id: &Rc<MetaZwlrGammaControlV1>,
        output: &Rc<MetaWlOutput>,
    ) {
        let res = _slf.send_get_gamma_control(
            id,
            output,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_gamma_control_manager_v1.get_gamma_control message: {}", Report::new(e));
        }
    }

    /// destroy the manager
    ///
    /// All objects created by the manager will still remain valid, until their
    /// appropriate destroy request has been called.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwlrGammaControlManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_gamma_control_manager_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwlrGammaControlManagerV1 {
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
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaZwlrGammaControlV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlOutput>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_gamma_control(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_gamma_control(&self, arg0, arg1);
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
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
    }
}

