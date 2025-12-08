//! an exported surface handle
//!
//! An xdg_exported object represents an exported reference to a surface. The
//! exported surface may be referenced as long as the xdg_exported object not
//! destroyed. Destroying the xdg_exported invalidates any relationship the
//! importer may have established using xdg_imported.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zxdg_exported_v2 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZxdgExportedV2 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZxdgExportedV2Handler>,
}

struct DefaultHandler;

impl ZxdgExportedV2Handler for DefaultHandler { }

impl ConcreteObject for ZxdgExportedV2 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZxdgExportedV2;
    const INTERFACE_NAME: &str = "zxdg_exported_v2";
}

impl ZxdgExportedV2 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZxdgExportedV2Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZxdgExportedV2Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZxdgExportedV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZxdgExportedV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZxdgExportedV2 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// unexport the exported surface
    ///
    /// Revoke the previously exported surface. This invalidates any
    /// relationship the importer may have set up using the xdg_imported created
    /// given the handle sent via xdg_exported.handle.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zxdg_exported_v2#{}.destroy()\n", id);
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

    /// unexport the exported surface
    ///
    /// Revoke the previously exported surface. This invalidates any
    /// relationship the importer may have set up using the xdg_imported created
    /// given the handle sent via xdg_exported.handle.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zxdg_exported_v2.destroy", &e);
        }
    }

    /// Since when the handle message is available.
    pub const MSG__HANDLE__SINCE: u32 = 1;

    /// the exported surface handle
    ///
    /// The handle event contains the unique handle of this exported surface
    /// reference. It may be shared with any client, which then can use it to
    /// import the surface by calling xdg_importer.import_toplevel. A handle
    /// may be used to import the surface multiple times.
    ///
    /// # Arguments
    ///
    /// - `handle`: the exported surface handle
    #[inline]
    pub fn try_send_handle(
        &self,
        handle: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            handle,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zxdg_exported_v2#{}.handle(handle: {:?})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// the exported surface handle
    ///
    /// The handle event contains the unique handle of this exported surface
    /// reference. It may be shared with any client, which then can use it to
    /// import the surface by calling xdg_importer.import_toplevel. A handle
    /// may be used to import the surface multiple times.
    ///
    /// # Arguments
    ///
    /// - `handle`: the exported surface handle
    #[inline]
    pub fn send_handle(
        &self,
        handle: &str,
    ) {
        let res = self.try_send_handle(
            handle,
        );
        if let Err(e) = res {
            log_send("zxdg_exported_v2.handle", &e);
        }
    }
}

/// A message handler for [`ZxdgExportedV2`] proxies.
pub trait ZxdgExportedV2Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZxdgExportedV2>) {
        slf.core.delete_id();
    }

    /// unexport the exported surface
    ///
    /// Revoke the previously exported surface. This invalidates any
    /// relationship the importer may have set up using the xdg_imported created
    /// given the handle sent via xdg_exported.handle.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZxdgExportedV2>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zxdg_exported_v2.destroy", &e);
        }
    }

    /// the exported surface handle
    ///
    /// The handle event contains the unique handle of this exported surface
    /// reference. It may be shared with any client, which then can use it to
    /// import the surface by calling xdg_importer.import_toplevel. A handle
    /// may be used to import the surface multiple times.
    ///
    /// # Arguments
    ///
    /// - `handle`: the exported surface handle
    #[inline]
    fn handle_handle(
        &mut self,
        slf: &Rc<ZxdgExportedV2>,
        handle: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_handle(
            handle,
        );
        if let Err(e) = res {
            log_forward("zxdg_exported_v2.handle", &e);
        }
    }
}

impl ObjectPrivate for ZxdgExportedV2 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZxdgExportedV2, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zxdg_exported_v2#{}.destroy()\n", client_id, id);
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
            0 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "handle")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zxdg_exported_v2#{}.handle(handle: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_handle(&self, arg0);
                } else {
                    DefaultHandler.handle_handle(&self, arg0);
                }
            }
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
        Ok(())
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "handle",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZxdgExportedV2 {
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

