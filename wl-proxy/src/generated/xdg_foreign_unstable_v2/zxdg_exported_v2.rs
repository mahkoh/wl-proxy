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
pub struct ZxdgExportedV2 {
    core: ProxyCore,
    handler: HandlerHolder<dyn ZxdgExportedV2Handler>,
}

struct DefaultHandler;

impl ZxdgExportedV2Handler for DefaultHandler { }

impl ZxdgExportedV2 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "zxdg_exported_v2";
}

impl ZxdgExportedV2 {
    pub fn set_handler(&self, handler: impl ZxdgExportedV2Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

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
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zxdg_exported_v2#{}.destroy()\n", id);
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zxdg_exported_v2#{}.handle(handle: {:?})\n", client.endpoint.id, id, arg0);
            self.core.state.log(args);
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
}

/// A message handler for [ZxdgExportedV2] proxies.
#[allow(dead_code)]
pub trait ZxdgExportedV2Handler: Any {
    /// unexport the exported surface
    ///
    /// Revoke the previously exported surface. This invalidates any
    /// relationship the importer may have set up using the xdg_imported created
    /// given the handle sent via xdg_exported.handle.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<ZxdgExportedV2>,
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
        _slf: &Rc<ZxdgExportedV2>,
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

impl ProxyPrivate for ZxdgExportedV2 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::ZxdgExportedV2, version),
            handler: Default::default(),
        })
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zxdg_exported_v2#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
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
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let mut offset = 2;
                let arg0 = {
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
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zxdg_exported_v2#{}.handle(handle: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle(&self, arg0);
                } else {
                    DefaultHandler.handle(&self, arg0);
                }
            }
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
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

impl Proxy for ZxdgExportedV2 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn unset_handler(&self) {
        self.handler.set(None);
    }

    fn get_handler_any_ref(&self) -> Result<Ref<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(Ref::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<RefMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow_mut().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(RefMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

