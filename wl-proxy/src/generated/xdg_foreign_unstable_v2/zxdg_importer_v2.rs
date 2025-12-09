//! interface for importing surfaces
//!
//! A global interface used for importing surfaces exported by xdg_exporter.
//! With this interface, a client can create a reference to a surface of
//! another client.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zxdg_importer_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZxdgImporterV2 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZxdgImporterV2MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZxdgImporterV2MessageHandler for DefaultMessageHandler { }

impl MetaZxdgImporterV2 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZxdgImporterV2 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZxdgImporterV2, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaZxdgImporterV2MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaZxdgImporterV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZxdgImporterV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZxdgImporterV2 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the xdg_importer object
    ///
    /// Notify the compositor that the xdg_importer object will no longer be
    /// used.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
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

    /// Since when the import_toplevel message is available.
    #[allow(dead_code)]
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
    pub fn send_import_toplevel(
        &self,
        id: &Rc<MetaZxdgImportedV2>,
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
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
        ]);
        fmt.string(arg1);
        Ok(())
    }
}

/// A message handler for [ZxdgImporterV2] proxies.
#[allow(dead_code)]
pub trait MetaZxdgImporterV2MessageHandler {
    /// destroy the xdg_importer object
    ///
    /// Notify the compositor that the xdg_importer object will no longer be
    /// used.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZxdgImporterV2>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_importer_v2.destroy message: {}", Report::new(e));
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
    fn import_toplevel(
        &mut self,
        _slf: &Rc<MetaZxdgImporterV2>,
        id: &Rc<MetaZxdgImportedV2>,
        handle: &str,
    ) {
        let res = _slf.send_import_toplevel(
            id,
            handle,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_importer_v2.import_toplevel message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZxdgImporterV2 {
    fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Self::new(state, version)
    }

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
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            1 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("id"));
                };
                offset += 1;
                let arg1 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("handle"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("handle"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("handle"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("handle"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                let arg0_id = arg0;
                let arg0 = MetaZxdgImportedV2::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).import_toplevel(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.import_toplevel(&self, arg0, arg1);
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

