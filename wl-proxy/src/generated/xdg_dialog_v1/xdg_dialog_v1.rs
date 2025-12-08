//! dialog object
//!
//! A xdg_dialog_v1 object is an ancillary object tied to a xdg_toplevel. Its
//! purpose is hinting the compositor that the toplevel is a "dialog" (e.g. a
//! temporary window) relative to another toplevel (see
//! xdg_toplevel.set_parent). If the xdg_toplevel is destroyed, the xdg_dialog_v1
//! becomes inert.
//!
//! Through this object, the client may provide additional hints about
//! the purpose of the secondary toplevel. This interface has no effect
//! on toplevels that are not attached to a parent toplevel.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A xdg_dialog_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaXdgDialogV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaXdgDialogV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaXdgDialogV1MessageHandler for DefaultMessageHandler { }

impl MetaXdgDialogV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaXdgDialogV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::XdgDialogV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaXdgDialogV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaXdgDialogV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaXdgDialogV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the dialog object
    ///
    /// Destroys the xdg_dialog_v1 object. If this object is destroyed
    /// before the related xdg_toplevel, the compositor should unapply its
    /// effects.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= xdg_dialog_v1#{}.destroy()", id);
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

    /// Since when the set_modal message is available.
    #[allow(dead_code)]
    pub const MSG__SET_MODAL__SINCE: u32 = 1;

    /// mark dialog as modal
    ///
    /// Hints that the dialog has "modal" behavior. Modal dialogs typically
    /// require to be fully addressed by the user (i.e. closed) before resuming
    /// interaction with the parent toplevel, and may require a distinct
    /// presentation.
    ///
    /// Clients must implement the logic to filter events in the parent
    /// toplevel on their own.
    ///
    /// Compositors may choose any policy in event delivery to the parent
    /// toplevel, from delivering all events unfiltered to using them for
    /// internal consumption.
    #[inline]
    pub fn send_set_modal(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= xdg_dialog_v1#{}.set_modal()", id);
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
        Ok(())
    }

    /// Since when the unset_modal message is available.
    #[allow(dead_code)]
    pub const MSG__UNSET_MODAL__SINCE: u32 = 1;

    /// mark dialog as not modal
    ///
    /// Drops the hint that this dialog has "modal" behavior. See
    /// xdg_dialog_v1.set_modal for more details.
    #[inline]
    pub fn send_unset_modal(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= xdg_dialog_v1#{}.unset_modal()", id);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
        ]);
        Ok(())
    }
}

/// A message handler for [XdgDialogV1] proxies.
#[allow(dead_code)]
pub trait MetaXdgDialogV1MessageHandler {
    /// destroy the dialog object
    ///
    /// Destroys the xdg_dialog_v1 object. If this object is destroyed
    /// before the related xdg_toplevel, the compositor should unapply its
    /// effects.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaXdgDialogV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_dialog_v1.destroy message: {}", Report::new(e));
        }
    }

    /// mark dialog as modal
    ///
    /// Hints that the dialog has "modal" behavior. Modal dialogs typically
    /// require to be fully addressed by the user (i.e. closed) before resuming
    /// interaction with the parent toplevel, and may require a distinct
    /// presentation.
    ///
    /// Clients must implement the logic to filter events in the parent
    /// toplevel on their own.
    ///
    /// Compositors may choose any policy in event delivery to the parent
    /// toplevel, from delivering all events unfiltered to using them for
    /// internal consumption.
    #[inline]
    fn set_modal(
        &mut self,
        _slf: &Rc<MetaXdgDialogV1>,
    ) {
        let res = _slf.send_set_modal(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_dialog_v1.set_modal message: {}", Report::new(e));
        }
    }

    /// mark dialog as not modal
    ///
    /// Drops the hint that this dialog has "modal" behavior. See
    /// xdg_dialog_v1.set_modal for more details.
    #[inline]
    fn unset_modal(
        &mut self,
        _slf: &Rc<MetaXdgDialogV1>,
    ) {
        let res = _slf.send_unset_modal(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_dialog_v1.unset_modal message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaXdgDialogV1 {
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
                eprintln!("client#{:04} -> xdg_dialog_v1#{}.destroy()", client.endpoint.id, msg[0]);
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("client#{:04} -> xdg_dialog_v1#{}.set_modal()", client.endpoint.id, msg[0]);
                if let Some(handler) = handler {
                    (**handler).set_modal(&self);
                } else {
                    DefaultMessageHandler.set_modal(&self);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("client#{:04} -> xdg_dialog_v1#{}.unset_modal()", client.endpoint.id, msg[0]);
                if let Some(handler) = handler {
                    (**handler).unset_modal(&self);
                } else {
                    DefaultMessageHandler.unset_modal(&self);
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
            1 => "set_modal",
            2 => "unset_modal",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

