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

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A xdg_dialog_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XdgDialogV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn XdgDialogV1Handler>,
}

struct DefaultHandler;

impl XdgDialogV1Handler for DefaultHandler { }

impl ConcreteObject for XdgDialogV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::XdgDialogV1;
    const INTERFACE_NAME: &str = "xdg_dialog_v1";
}

impl XdgDialogV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl XdgDialogV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn XdgDialogV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XdgDialogV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XdgDialogV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XdgDialogV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the dialog object
    ///
    /// Destroys the xdg_dialog_v1 object. If this object is destroyed
    /// before the related xdg_toplevel, the compositor should unapply its
    /// effects.
    #[inline]
    pub fn try_send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_dialog_v1#{}.destroy()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
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
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the dialog object
    ///
    /// Destroys the xdg_dialog_v1 object. If this object is destroyed
    /// before the related xdg_toplevel, the compositor should unapply its
    /// effects.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("xdg_dialog_v1.destroy", &e);
        }
    }

    /// Since when the set_modal message is available.
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
    pub fn try_send_set_modal(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_dialog_v1#{}.set_modal()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
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
    ) {
        let res = self.try_send_set_modal(
        );
        if let Err(e) = res {
            log_send("xdg_dialog_v1.set_modal", &e);
        }
    }

    /// Since when the unset_modal message is available.
    pub const MSG__UNSET_MODAL__SINCE: u32 = 1;

    /// mark dialog as not modal
    ///
    /// Drops the hint that this dialog has "modal" behavior. See
    /// xdg_dialog_v1.set_modal for more details.
    #[inline]
    pub fn try_send_unset_modal(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_dialog_v1#{}.unset_modal()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
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
        Ok(())
    }

    /// mark dialog as not modal
    ///
    /// Drops the hint that this dialog has "modal" behavior. See
    /// xdg_dialog_v1.set_modal for more details.
    #[inline]
    pub fn send_unset_modal(
        &self,
    ) {
        let res = self.try_send_unset_modal(
        );
        if let Err(e) = res {
            log_send("xdg_dialog_v1.unset_modal", &e);
        }
    }
}

/// A message handler for [`XdgDialogV1`] proxies.
pub trait XdgDialogV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<XdgDialogV1>) {
        slf.core.delete_id();
    }

    /// destroy the dialog object
    ///
    /// Destroys the xdg_dialog_v1 object. If this object is destroyed
    /// before the related xdg_toplevel, the compositor should unapply its
    /// effects.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<XdgDialogV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("xdg_dialog_v1.destroy", &e);
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
    fn handle_set_modal(
        &mut self,
        slf: &Rc<XdgDialogV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_modal(
        );
        if let Err(e) = res {
            log_forward("xdg_dialog_v1.set_modal", &e);
        }
    }

    /// mark dialog as not modal
    ///
    /// Drops the hint that this dialog has "modal" behavior. See
    /// xdg_dialog_v1.set_modal for more details.
    #[inline]
    fn handle_unset_modal(
        &mut self,
        slf: &Rc<XdgDialogV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_unset_modal(
        );
        if let Err(e) = res {
            log_forward("xdg_dialog_v1.unset_modal", &e);
        }
    }
}

impl ObjectPrivate for XdgDialogV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::XdgDialogV1, version),
            handler: Default::default(),
        })
    }

    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err((ObjectError(ObjectErrorKind::HandlerBorrowed), self));
        };
        if let Some(handler) = &mut *handler {
            handler.delete_id(&self);
        } else {
            self.core.delete_id();
        }
        Ok(())
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_dialog_v1#{}.destroy()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_destroy(&self);
                } else {
                    DefaultHandler.handle_destroy(&self);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_dialog_v1#{}.set_modal()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_modal(&self);
                } else {
                    DefaultHandler.handle_set_modal(&self);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_dialog_v1#{}.unset_modal()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_unset_modal(&self);
                } else {
                    DefaultHandler.handle_unset_modal(&self);
                }
            }
            n => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, server: &Endpoint, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
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

impl Object for XdgDialogV1 {
    fn core(&self) -> &ObjectCore {
        &self.core
    }

    fn unset_handler(&self) {
        self.handler.set(None);
    }

    fn get_handler_any_ref(&self) -> Result<HandlerRef<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.try_borrow().ok_or(HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(HandlerRef::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<HandlerMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.try_borrow_mut().ok_or(HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(HandlerMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

