//! get relative pointer objects
//!
//! A global interface used for getting the relative pointer object for a
//! given pointer.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_relative_pointer_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpRelativePointerManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpRelativePointerManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpRelativePointerManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpRelativePointerManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwpRelativePointerManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpRelativePointerManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpRelativePointerManagerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the relative pointer manager object
    ///
    /// Used by the client to notify the server that it will no longer use this
    /// relative pointer manager object.
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

    /// Since when the get_relative_pointer message is available.
    #[allow(dead_code)]
    pub const MSG__GET_RELATIVE_POINTER__SINCE: u32 = 1;

    /// get a relative pointer object
    ///
    /// Create a relative pointer interface given a wl_pointer object. See the
    /// wp_relative_pointer interface for more details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `pointer`:
    #[inline]
    pub fn send_get_relative_pointer(
        &self,
        id: &Rc<MetaZwpRelativePointerV1>,
        pointer: &Rc<MetaWlPointer>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            pointer,
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
            1,
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpRelativePointerManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpRelativePointerManagerV1MessageHandler {
    /// destroy the relative pointer manager object
    ///
    /// Used by the client to notify the server that it will no longer use this
    /// relative pointer manager object.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpRelativePointerManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_relative_pointer_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// get a relative pointer object
    ///
    /// Create a relative pointer interface given a wl_pointer object. See the
    /// wp_relative_pointer interface for more details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `pointer`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_relative_pointer(
        &mut self,
        _slf: &Rc<MetaZwpRelativePointerManagerV1>,
        id: &Rc<MetaZwpRelativePointerV1>,
        pointer: &Rc<MetaWlPointer>,
    ) {
        let res = _slf.send_get_relative_pointer(
            id,
            pointer,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_relative_pointer_manager_v1.get_relative_pointer message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpRelativePointerManagerV1 {
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
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaZwpRelativePointerV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlPointer>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_relative_pointer(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_relative_pointer(&self, arg0, arg1);
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

