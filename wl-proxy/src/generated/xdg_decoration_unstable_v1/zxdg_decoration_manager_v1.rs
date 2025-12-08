//! window decoration manager
//!
//! This interface allows a compositor to announce support for server-side
//! decorations.
//!
//! A window decoration is a set of window controls as deemed appropriate by
//! the party managing them, such as user interface components used to move,
//! resize and change a window's state.
//!
//! A client can use this protocol to request being decorated by a supporting
//! compositor.
//!
//! If compositor and client do not negotiate the use of a server-side
//! decoration using this protocol, clients continue to self-decorate as they
//! see fit.
//!
//! Warning! The protocol described in this file is experimental and
//! backward incompatible changes may be made. Backward compatible changes
//! may be added together with the corresponding interface version bump.
//! Backward incompatible changes are done by bumping the version number in
//! the protocol and interface names and resetting the interface version.
//! Once the protocol is to be declared stable, the 'z' prefix and the
//! version number in the protocol and interface names are removed and the
//! interface version number is reset.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zxdg_decoration_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZxdgDecorationManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZxdgDecorationManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZxdgDecorationManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaZxdgDecorationManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZxdgDecorationManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZxdgDecorationManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZxdgDecorationManagerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the decoration manager object
    ///
    /// Destroy the decoration manager. This doesn't destroy objects created
    /// with the manager.
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

    /// Since when the get_toplevel_decoration message is available.
    #[allow(dead_code)]
    pub const MSG__GET_TOPLEVEL_DECORATION__SINCE: u32 = 1;

    /// create a new toplevel decoration object
    ///
    /// Create a new decoration object associated with the given toplevel.
    ///
    /// Creating an xdg_toplevel_decoration from an xdg_toplevel which has a
    /// buffer attached or committed is a client error, and any attempts by a
    /// client to attach or manipulate a buffer prior to the first
    /// xdg_toplevel_decoration.configure event must also be treated as
    /// errors.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `toplevel`:
    #[inline]
    pub fn send_get_toplevel_decoration(
        &self,
        id: &Rc<MetaZxdgToplevelDecorationV1>,
        toplevel: &Rc<MetaXdgToplevel>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            toplevel,
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

/// A message handler for [ZxdgDecorationManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaZxdgDecorationManagerV1MessageHandler {
    /// destroy the decoration manager object
    ///
    /// Destroy the decoration manager. This doesn't destroy objects created
    /// with the manager.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZxdgDecorationManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_decoration_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// create a new toplevel decoration object
    ///
    /// Create a new decoration object associated with the given toplevel.
    ///
    /// Creating an xdg_toplevel_decoration from an xdg_toplevel which has a
    /// buffer attached or committed is a client error, and any attempts by a
    /// client to attach or manipulate a buffer prior to the first
    /// xdg_toplevel_decoration.configure event must also be treated as
    /// errors.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `toplevel`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_toplevel_decoration(
        &mut self,
        _slf: &Rc<MetaZxdgDecorationManagerV1>,
        id: &Rc<MetaZxdgToplevelDecorationV1>,
        toplevel: &Rc<MetaXdgToplevel>,
    ) {
        let res = _slf.send_get_toplevel_decoration(
            id,
            toplevel,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_decoration_manager_v1.get_toplevel_decoration message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZxdgDecorationManagerV1 {
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
                let arg0 = MetaZxdgToplevelDecorationV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaXdgToplevel>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_toplevel_decoration(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_toplevel_decoration(&self, arg0, arg1);
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

