//! create dialogs related to other toplevels
//!
//! The xdg_wm_dialog_v1 interface is exposed as a global object allowing
//! to register surfaces with a xdg_toplevel role as "dialogs" relative to
//! another toplevel.
//!
//! The compositor may let this relation influence how the surface is
//! placed, displayed or interacted with.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A xdg_wm_dialog_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaXdgWmDialogV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaXdgWmDialogV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaXdgWmDialogV1MessageHandler for DefaultMessageHandler { }

impl MetaXdgWmDialogV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaXdgWmDialogV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::XdgWmDialogV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaXdgWmDialogV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaXdgWmDialogV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaXdgWmDialogV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the dialog manager object
    ///
    /// Destroys the xdg_wm_dialog_v1 object. This does not affect
    /// the xdg_dialog_v1 objects generated through it.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= xdg_wm_dialog_v1#{}.destroy()", id);
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

    /// Since when the get_xdg_dialog message is available.
    #[allow(dead_code)]
    pub const MSG__GET_XDG_DIALOG__SINCE: u32 = 1;

    /// create a dialog object
    ///
    /// Creates a xdg_dialog_v1 object for the given toplevel. See the interface
    /// description for more details.
    ///
    /// 	Compositors must raise an already_used error if clients attempt to
    /// 	create multiple xdg_dialog_v1 objects for the same xdg_toplevel.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `toplevel`:
    #[inline]
    pub fn send_get_xdg_dialog(
        &self,
        id: &Rc<MetaXdgDialogV1>,
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("toplevel")),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        eprintln!("server      <= xdg_wm_dialog_v1#{}.get_xdg_dialog(id: xdg_dialog_v1#{}, toplevel: xdg_toplevel#{})", id, arg0_id, arg1_id);
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
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }
}

/// A message handler for [XdgWmDialogV1] proxies.
#[allow(dead_code)]
pub trait MetaXdgWmDialogV1MessageHandler {
    /// destroy the dialog manager object
    ///
    /// Destroys the xdg_wm_dialog_v1 object. This does not affect
    /// the xdg_dialog_v1 objects generated through it.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaXdgWmDialogV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_wm_dialog_v1.destroy message: {}", Report::new(e));
        }
    }

    /// create a dialog object
    ///
    /// Creates a xdg_dialog_v1 object for the given toplevel. See the interface
    /// description for more details.
    ///
    /// 	Compositors must raise an already_used error if clients attempt to
    /// 	create multiple xdg_dialog_v1 objects for the same xdg_toplevel.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `toplevel`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_xdg_dialog(
        &mut self,
        _slf: &Rc<MetaXdgWmDialogV1>,
        id: &Rc<MetaXdgDialogV1>,
        toplevel: &Rc<MetaXdgToplevel>,
    ) {
        let res = _slf.send_get_xdg_dialog(
            id,
            toplevel,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_wm_dialog_v1.get_xdg_dialog message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaXdgWmDialogV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("client#{:04} -> xdg_wm_dialog_v1#{}.destroy()", client.endpoint.id, msg[0]);
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
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                eprintln!("client#{:04} -> xdg_wm_dialog_v1#{}.get_xdg_dialog(id: xdg_dialog_v1#{}, toplevel: xdg_toplevel#{})", client.endpoint.id, msg[0], arg0, arg1);
                let arg0_id = arg0;
                let arg0 = MetaXdgDialogV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaXdgToplevel>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("toplevel", o.core().interface, ProxyInterface::XdgToplevel));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_xdg_dialog(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_xdg_dialog(&self, arg0, arg1);
                }
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
            0 => "destroy",
            1 => "get_xdg_dialog",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl MetaXdgWmDialogV1 {
    /// Since when the error.already_used enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ALREADY_USED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaXdgWmDialogV1Error(pub u32);

impl MetaXdgWmDialogV1Error {
    /// the xdg_toplevel object has already been used to create a xdg_dialog_v1
    #[allow(dead_code)]
    pub const ALREADY_USED: Self = Self(0);
}

impl Debug for MetaXdgWmDialogV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ALREADY_USED => "ALREADY_USED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
