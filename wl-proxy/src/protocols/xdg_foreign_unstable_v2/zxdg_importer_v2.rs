//! interface for importing surfaces
//!
//! A global interface used for importing surfaces exported by xdg_exporter.
//! With this interface, a client can create a reference to a surface of
//! another client.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zxdg_importer_v2 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZxdgImporterV2 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZxdgImporterV2Handler>,
}

struct DefaultHandler;

impl ZxdgImporterV2Handler for DefaultHandler { }

impl ConcreteObject for ZxdgImporterV2 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZxdgImporterV2;
    const INTERFACE_NAME: &str = "zxdg_importer_v2";
}

impl ZxdgImporterV2 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZxdgImporterV2Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZxdgImporterV2Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZxdgImporterV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZxdgImporterV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZxdgImporterV2 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the xdg_importer object
    ///
    /// Notify the compositor that the xdg_importer object will no longer be
    /// used.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zxdg_importer_v2#{}.destroy()\n", id);
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

    /// destroy the xdg_importer object
    ///
    /// Notify the compositor that the xdg_importer object will no longer be
    /// used.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zxdg_importer_v2.destroy", &e);
        }
    }

    /// Since when the import_toplevel message is available.
    pub const MSG__IMPORT_TOPLEVEL__SINCE: u32 = 1;

    /// import a toplevel surface
    ///
    /// The import_toplevel request imports a surface from any client given a handle
    /// retrieved by exporting said surface using xdg_exporter.export_toplevel.
    /// When called, a new xdg_imported object will be created. This new object
    /// represents the imported surface, and the importing client can
    /// manipulate its relationship using it. See xdg_imported for details.
    ///
    /// # Arguments
    ///
    /// - `id`: the new xdg_imported object
    /// - `handle`: the exported surface handle
    #[inline]
    pub fn try_send_import_toplevel(
        &self,
        id: &Rc<ZxdgImportedV2>,
        handle: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            handle,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zxdg_importer_v2#{}.import_toplevel(id: zxdg_imported_v2#{}, handle: {:?})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            arg0_id,
        ]);
        fmt.string(arg1);
        Ok(())
    }

    /// import a toplevel surface
    ///
    /// The import_toplevel request imports a surface from any client given a handle
    /// retrieved by exporting said surface using xdg_exporter.export_toplevel.
    /// When called, a new xdg_imported object will be created. This new object
    /// represents the imported surface, and the importing client can
    /// manipulate its relationship using it. See xdg_imported for details.
    ///
    /// # Arguments
    ///
    /// - `id`: the new xdg_imported object
    /// - `handle`: the exported surface handle
    #[inline]
    pub fn send_import_toplevel(
        &self,
        id: &Rc<ZxdgImportedV2>,
        handle: &str,
    ) {
        let res = self.try_send_import_toplevel(
            id,
            handle,
        );
        if let Err(e) = res {
            log_send("zxdg_importer_v2.import_toplevel", &e);
        }
    }

    /// import a toplevel surface
    ///
    /// The import_toplevel request imports a surface from any client given a handle
    /// retrieved by exporting said surface using xdg_exporter.export_toplevel.
    /// When called, a new xdg_imported object will be created. This new object
    /// represents the imported surface, and the importing client can
    /// manipulate its relationship using it. See xdg_imported for details.
    ///
    /// # Arguments
    ///
    /// - `handle`: the exported surface handle
    #[inline]
    pub fn new_try_send_import_toplevel(
        &self,
        handle: &str,
    ) -> Result<Rc<ZxdgImportedV2>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_import_toplevel(
            &id,
            handle,
        )?;
        Ok(id)
    }

    /// import a toplevel surface
    ///
    /// The import_toplevel request imports a surface from any client given a handle
    /// retrieved by exporting said surface using xdg_exporter.export_toplevel.
    /// When called, a new xdg_imported object will be created. This new object
    /// represents the imported surface, and the importing client can
    /// manipulate its relationship using it. See xdg_imported for details.
    ///
    /// # Arguments
    ///
    /// - `handle`: the exported surface handle
    #[inline]
    pub fn new_send_import_toplevel(
        &self,
        handle: &str,
    ) -> Rc<ZxdgImportedV2> {
        let id = self.core.create_child();
        self.send_import_toplevel(
            &id,
            handle,
        );
        id
    }
}

/// A message handler for [`ZxdgImporterV2`] proxies.
pub trait ZxdgImporterV2Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZxdgImporterV2>) {
        slf.core.delete_id();
    }

    /// destroy the xdg_importer object
    ///
    /// Notify the compositor that the xdg_importer object will no longer be
    /// used.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZxdgImporterV2>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zxdg_importer_v2.destroy", &e);
        }
    }

    /// import a toplevel surface
    ///
    /// The import_toplevel request imports a surface from any client given a handle
    /// retrieved by exporting said surface using xdg_exporter.export_toplevel.
    /// When called, a new xdg_imported object will be created. This new object
    /// represents the imported surface, and the importing client can
    /// manipulate its relationship using it. See xdg_imported for details.
    ///
    /// # Arguments
    ///
    /// - `id`: the new xdg_imported object
    /// - `handle`: the exported surface handle
    #[inline]
    fn handle_import_toplevel(
        &mut self,
        slf: &Rc<ZxdgImporterV2>,
        id: &Rc<ZxdgImportedV2>,
        handle: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_import_toplevel(
            id,
            handle,
        );
        if let Err(e) = res {
            log_forward("zxdg_importer_v2.import_toplevel", &e);
        }
    }
}

impl ObjectPrivate for ZxdgImporterV2 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZxdgImporterV2, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zxdg_importer_v2#{}.destroy()\n", client_id, id);
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
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("id")));
                };
                offset += 1;
                let arg1;
                (arg1, offset) = parse_string::<NonNullString>(msg, offset, "handle")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zxdg_importer_v2#{}.import_toplevel(id: zxdg_imported_v2#{}, handle: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = ZxdgImportedV2::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_import_toplevel(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_import_toplevel(&self, arg0, arg1);
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
            1 => "import_toplevel",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for ZxdgImporterV2 {
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

