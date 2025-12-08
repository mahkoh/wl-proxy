//! create desktop-style surfaces
//!
//! This interface is implemented by servers that provide
//! desktop-style user interfaces.
//!
//! It allows clients to associate a wl_shell_surface with
//! a basic surface.
//!
//! Note! This protocol is deprecated and not intended for production use.
//! For desktop-style user interfaces, use xdg_shell. Compositors and clients
//! should not implement this interface.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wl_shell proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWlShell {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWlShellMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWlShellMessageHandler for DefaultMessageHandler { }

impl MetaWlShell {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWlShell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWlShell")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWlShell {
    /// Since when the get_shell_surface message is available.
    #[allow(dead_code)]
    pub const MSG__GET_SHELL_SURFACE__SINCE: u32 = 1;

    /// create a shell surface from a surface
    ///
    /// Create a shell surface for an existing surface. This gives
    /// the wl_surface the role of a shell surface. If the wl_surface
    /// already has another role, it raises a protocol error.
    ///
    /// Only one shell surface can be associated with a given surface.
    ///
    /// # Arguments
    ///
    /// - `id`: shell surface to create
    /// - `surface`: surface to be given the shell surface role
    #[inline]
    pub fn send_get_shell_surface(
        &self,
        id: &Rc<MetaWlShellSurface>,
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
}

/// A message handler for [WlShell] proxies.
#[allow(dead_code)]
pub trait MetaWlShellMessageHandler {
    /// create a shell surface from a surface
    ///
    /// Create a shell surface for an existing surface. This gives
    /// the wl_surface the role of a shell surface. If the wl_surface
    /// already has another role, it raises a protocol error.
    ///
    /// Only one shell surface can be associated with a given surface.
    ///
    /// # Arguments
    ///
    /// - `id`: shell surface to create
    /// - `surface`: surface to be given the shell surface role
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_shell_surface(
        &mut self,
        _slf: &Rc<MetaWlShell>,
        id: &Rc<MetaWlShellSurface>,
        surface: &Rc<MetaWlSurface>,
    ) {
        let res = _slf.send_get_shell_surface(
            id,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_shell.get_shell_surface message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWlShell {
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
                let arg0 = MetaWlShellSurface::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_shell_surface(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_shell_surface(&self, arg0, arg1);
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

impl MetaWlShell {
    /// Since when the error.role enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ROLE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWlShellError(pub u32);

impl MetaWlShellError {
    /// given wl_surface has another role
    #[allow(dead_code)]
    pub const ROLE: Self = Self(0);
}

impl Debug for MetaWlShellError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ROLE => "ROLE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
