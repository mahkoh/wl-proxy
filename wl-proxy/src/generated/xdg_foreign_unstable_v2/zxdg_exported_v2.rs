//! an exported surface handle
//!
//! An xdg_exported object represents an exported reference to a surface. The
//! exported surface may be referenced as long as the xdg_exported object not
//! destroyed. Destroying the xdg_exported invalidates any relationship the
//! importer may have established using xdg_imported.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zxdg_exported_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZxdgExportedV2 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZxdgExportedV2MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZxdgExportedV2MessageHandler for DefaultMessageHandler { }

impl MetaZxdgExportedV2 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZxdgExportedV2 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZxdgExportedV2, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZxdgExportedV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZxdgExportedV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZxdgExportedV2 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// unexport the exported surface
    ///
    /// Revoke the previously exported surface. This invalidates any
    /// relationship the importer may have set up using the xdg_imported created
    /// given the handle sent via xdg_exported.handle.
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

    /// Since when the handle message is available.
    #[allow(dead_code)]
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
    pub fn send_handle(
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
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
        ]);
        fmt.string(arg0);
        Ok(())
    }
}

/// A message handler for [ZxdgExportedV2] proxies.
#[allow(dead_code)]
pub trait MetaZxdgExportedV2MessageHandler {
    /// unexport the exported surface
    ///
    /// Revoke the previously exported surface. This invalidates any
    /// relationship the importer may have set up using the xdg_imported created
    /// given the handle sent via xdg_exported.handle.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZxdgExportedV2>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_exported_v2.destroy message: {}", Report::new(e));
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
    fn handle(
        &mut self,
        _slf: &Rc<MetaZxdgExportedV2>,
        handle: &str,
    ) {
        let res = _slf.send_handle(
            handle,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_exported_v2.handle message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZxdgExportedV2 {
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
            0 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError);
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError);
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError);
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError);
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError);
                }
                if let Some(handler) = handler {
                    (**handler).handle(&self, arg0);
                } else {
                    DefaultMessageHandler.handle(&self, arg0);
                }
            }
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }
}

