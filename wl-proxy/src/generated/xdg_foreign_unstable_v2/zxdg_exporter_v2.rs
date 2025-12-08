//! interface for exporting surfaces
//!
//! A global interface used for exporting surfaces that can later be imported
//! using xdg_importer.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zxdg_exporter_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZxdgExporterV2 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZxdgExporterV2MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZxdgExporterV2MessageHandler for DefaultMessageHandler { }

impl MetaZxdgExporterV2 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZxdgExporterV2 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZxdgExporterV2, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZxdgExporterV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZxdgExporterV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZxdgExporterV2 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the xdg_exporter object
    ///
    /// Notify the compositor that the xdg_exporter object will no longer be
    /// used.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
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

    /// Since when the export_toplevel message is available.
    #[allow(dead_code)]
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
    ///         equivalent surfaces may be exported, otherwise an invalid_surface
    ///         protocol error is sent.
    ///
    /// # Arguments
    ///
    /// - `id`: the new xdg_exported object
    /// - `surface`: the surface to export
    #[inline]
    pub fn send_export_toplevel(
        &self,
        id: &Rc<MetaZxdgExportedV2>,
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
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
        ]);
        Ok(())
    }
}

/// A message handler for [ZxdgExporterV2] proxies.
#[allow(dead_code)]
pub trait MetaZxdgExporterV2MessageHandler {
    /// destroy the xdg_exporter object
    ///
    /// Notify the compositor that the xdg_exporter object will no longer be
    /// used.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZxdgExporterV2>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_exporter_v2.destroy message: {}", Report::new(e));
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
    ///         equivalent surfaces may be exported, otherwise an invalid_surface
    ///         protocol error is sent.
    ///
    /// # Arguments
    ///
    /// - `id`: the new xdg_exported object
    /// - `surface`: the surface to export
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn export_toplevel(
        &mut self,
        _slf: &Rc<MetaZxdgExporterV2>,
        id: &Rc<MetaZxdgExportedV2>,
        surface: &Rc<MetaWlSurface>,
    ) {
        let res = _slf.send_export_toplevel(
            id,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_exporter_v2.export_toplevel message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZxdgExporterV2 {
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
                self.core.handle_client_destroy();
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaZxdgExportedV2::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.endpoint.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).export_toplevel(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.export_toplevel(&self, arg0, arg1);
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

impl MetaZxdgExporterV2 {
    /// Since when the error.invalid_surface enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_SURFACE__SINCE: u32 = 1;
}

/// error values
///
/// These errors can be emitted in response to invalid xdg_exporter
/// requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaZxdgExporterV2Error(pub u32);

impl MetaZxdgExporterV2Error {
    /// surface is not an xdg_toplevel
    #[allow(dead_code)]
    pub const INVALID_SURFACE: Self = Self(0);
}

impl Debug for MetaZxdgExporterV2Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_SURFACE => "INVALID_SURFACE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
