//! interface for exporting surfaces
//!
//! A global interface used for exporting surfaces that can later be imported
//! using xdg_importer.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zxdg_exporter_v2 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZxdgExporterV2 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZxdgExporterV2Handler>,
}

struct DefaultHandler;

impl ZxdgExporterV2Handler for DefaultHandler { }

impl ConcreteObject for ZxdgExporterV2 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZxdgExporterV2;
    const INTERFACE_NAME: &str = "zxdg_exporter_v2";
}

impl ZxdgExporterV2 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZxdgExporterV2Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZxdgExporterV2Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZxdgExporterV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZxdgExporterV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZxdgExporterV2 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the xdg_exporter object
    ///
    /// Notify the compositor that the xdg_exporter object will no longer be
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zxdg_exporter_v2#{}.destroy()\n", id);
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

    /// destroy the xdg_exporter object
    ///
    /// Notify the compositor that the xdg_exporter object will no longer be
    /// used.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zxdg_exporter_v2.destroy", &e);
        }
    }

    /// Since when the export_toplevel message is available.
    pub const MSG__EXPORT_TOPLEVEL__SINCE: u32 = 1;

    /// export a toplevel surface
    ///
    /// The export_toplevel request exports the passed surface so that it can later be
    /// imported via xdg_importer. When called, a new xdg_exported object will
    /// be created and xdg_exported.handle will be sent immediately. See the
    /// corresponding interface and event for details.
    ///
    /// A surface may be exported multiple times, and each exported handle may
    /// be used to create an xdg_imported multiple times. Only xdg_toplevel
    /// equivalent surfaces may be exported, otherwise an invalid_surface
    /// protocol error is sent.
    ///
    /// # Arguments
    ///
    /// - `id`: the new xdg_exported object
    /// - `surface`: the surface to export
    #[inline]
    pub fn try_send_export_toplevel(
        &self,
        id: &Rc<ZxdgExportedV2>,
        surface: &Rc<WlSurface>,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zxdg_exporter_v2#{}.export_toplevel(id: zxdg_exported_v2#{}, surface: wl_surface#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id);
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
            arg1_id,
        ]);
        Ok(())
    }

    /// export a toplevel surface
    ///
    /// The export_toplevel request exports the passed surface so that it can later be
    /// imported via xdg_importer. When called, a new xdg_exported object will
    /// be created and xdg_exported.handle will be sent immediately. See the
    /// corresponding interface and event for details.
    ///
    /// A surface may be exported multiple times, and each exported handle may
    /// be used to create an xdg_imported multiple times. Only xdg_toplevel
    /// equivalent surfaces may be exported, otherwise an invalid_surface
    /// protocol error is sent.
    ///
    /// # Arguments
    ///
    /// - `id`: the new xdg_exported object
    /// - `surface`: the surface to export
    #[inline]
    pub fn send_export_toplevel(
        &self,
        id: &Rc<ZxdgExportedV2>,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_export_toplevel(
            id,
            surface,
        );
        if let Err(e) = res {
            log_send("zxdg_exporter_v2.export_toplevel", &e);
        }
    }

    /// export a toplevel surface
    ///
    /// The export_toplevel request exports the passed surface so that it can later be
    /// imported via xdg_importer. When called, a new xdg_exported object will
    /// be created and xdg_exported.handle will be sent immediately. See the
    /// corresponding interface and event for details.
    ///
    /// A surface may be exported multiple times, and each exported handle may
    /// be used to create an xdg_imported multiple times. Only xdg_toplevel
    /// equivalent surfaces may be exported, otherwise an invalid_surface
    /// protocol error is sent.
    ///
    /// # Arguments
    ///
    /// - `surface`: the surface to export
    #[inline]
    pub fn new_try_send_export_toplevel(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Result<Rc<ZxdgExportedV2>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_export_toplevel(
            &id,
            surface,
        )?;
        Ok(id)
    }

    /// export a toplevel surface
    ///
    /// The export_toplevel request exports the passed surface so that it can later be
    /// imported via xdg_importer. When called, a new xdg_exported object will
    /// be created and xdg_exported.handle will be sent immediately. See the
    /// corresponding interface and event for details.
    ///
    /// A surface may be exported multiple times, and each exported handle may
    /// be used to create an xdg_imported multiple times. Only xdg_toplevel
    /// equivalent surfaces may be exported, otherwise an invalid_surface
    /// protocol error is sent.
    ///
    /// # Arguments
    ///
    /// - `surface`: the surface to export
    #[inline]
    pub fn new_send_export_toplevel(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Rc<ZxdgExportedV2> {
        let id = self.core.create_child();
        self.send_export_toplevel(
            &id,
            surface,
        );
        id
    }
}

/// A message handler for [`ZxdgExporterV2`] proxies.
pub trait ZxdgExporterV2Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZxdgExporterV2>) {
        slf.core.delete_id();
    }

    /// destroy the xdg_exporter object
    ///
    /// Notify the compositor that the xdg_exporter object will no longer be
    /// used.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZxdgExporterV2>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zxdg_exporter_v2.destroy", &e);
        }
    }

    /// export a toplevel surface
    ///
    /// The export_toplevel request exports the passed surface so that it can later be
    /// imported via xdg_importer. When called, a new xdg_exported object will
    /// be created and xdg_exported.handle will be sent immediately. See the
    /// corresponding interface and event for details.
    ///
    /// A surface may be exported multiple times, and each exported handle may
    /// be used to create an xdg_imported multiple times. Only xdg_toplevel
    /// equivalent surfaces may be exported, otherwise an invalid_surface
    /// protocol error is sent.
    ///
    /// # Arguments
    ///
    /// - `id`: the new xdg_exported object
    /// - `surface`: the surface to export
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_export_toplevel(
        &mut self,
        slf: &Rc<ZxdgExporterV2>,
        id: &Rc<ZxdgExportedV2>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_export_toplevel(
            id,
            surface,
        );
        if let Err(e) = res {
            log_forward("zxdg_exporter_v2.export_toplevel", &e);
        }
    }
}

impl ObjectPrivate for ZxdgExporterV2 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZxdgExporterV2, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zxdg_exporter_v2#{}.destroy()\n", client_id, id);
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
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zxdg_exporter_v2#{}.export_toplevel(id: zxdg_exported_v2#{}, surface: wl_surface#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = ZxdgExportedV2::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_export_toplevel(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_export_toplevel(&self, arg0, arg1);
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
            1 => "export_toplevel",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for ZxdgExporterV2 {
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

impl ZxdgExporterV2 {
    /// Since when the error.invalid_surface enum variant is available.
    pub const ENM__ERROR_INVALID_SURFACE__SINCE: u32 = 1;
}

/// error values
///
/// These errors can be emitted in response to invalid xdg_exporter
/// requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZxdgExporterV2Error(pub u32);

impl ZxdgExporterV2Error {
    /// surface is not an xdg_toplevel
    pub const INVALID_SURFACE: Self = Self(0);
}

impl Debug for ZxdgExporterV2Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_SURFACE => "INVALID_SURFACE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
