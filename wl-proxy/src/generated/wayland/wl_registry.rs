//! global registry object
//!
//! The singleton global registry object.  The server has a number of
//! global objects that are available to all clients.  These objects
//! typically represent an actual object in the server (for example,
//! an input device) or they are singleton objects that provide
//! extension functionality.
//!
//! When a client creates a registry object, the registry object
//! will emit a global event for each global currently in the
//! registry.  Globals come and go as a result of device or
//! monitor hotplugs, reconfiguration or other events, and the
//! registry will send out global and global_remove events to
//! keep the client up to date with the changes.  To mark the end
//! of the initial burst of events, the client can use the
//! wl_display.sync request immediately after calling
//! wl_display.get_registry.
//!
//! A client can bind to a global object by using the bind
//! request.  This creates a client-side handle that lets the object
//! emit events to the client and lets the client invoke requests on
//! the object.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wl_registry proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWlRegistry {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWlRegistryMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWlRegistryMessageHandler for DefaultMessageHandler { }

impl MetaWlRegistry {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWlRegistry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWlRegistry")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWlRegistry {
    /// Since when the bind message is available.
    #[allow(dead_code)]
    pub const MSG__BIND__SINCE: u32 = 1;

    /// bind an object to the display
    ///
    /// Binds a new, client-created object to the server using the
    /// specified name as the identifier.
    ///
    /// # Arguments
    ///
    /// - `name`: unique numeric name of the object
    /// - `id`: bounded object
    #[inline]
    pub fn send_bind(
        &self,
        name: u32,
        id: Rc<dyn Proxy>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            name,
            id,
        );
        let arg1_obj = arg1;
        let arg1 = arg1_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        arg1.generate_server_id(arg1_obj.clone())?;
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0,
            arg1.server_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }

    /// Since when the global message is available.
    #[allow(dead_code)]
    pub const MSG__GLOBAL__SINCE: u32 = 1;

    /// announce global object
    ///
    /// Notify the client of global objects.
    ///
    /// The event notifies the client that a global object with
    /// the given name is now available, and it implements the
    /// given version of the given interface.
    ///
    /// # Arguments
    ///
    /// - `name`: numeric name of the global object
    /// - `interface`: interface implemented by the object
    /// - `version`: interface version
    #[inline]
    pub fn send_global(
        &self,
        name: u32,
        interface: &str,
        version: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            name,
            interface,
            version,
        );
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
            arg0,
        ]);
        fmt.string(arg1);
        fmt.words([
            arg2,
        ]);
        Ok(())
    }

    /// Since when the global_remove message is available.
    #[allow(dead_code)]
    pub const MSG__GLOBAL_REMOVE__SINCE: u32 = 1;

    /// announce removal of global object
    ///
    /// Notify the client of removed global objects.
    ///
    /// This event notifies the client that the global identified
    /// by name is no longer available.  If the client bound to
    /// the global using the bind request, the client should now
    /// destroy that object.
    ///
    /// The object remains valid and requests to the object will be
    /// ignored until the client destroys it, to avoid races between
    /// the global going away and a client sending a request to it.
    ///
    /// # Arguments
    ///
    /// - `name`: numeric name of the global object
    #[inline]
    pub fn send_global_remove(
        &self,
        name: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            name,
        );
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            1,
            arg0,
        ]);
        Ok(())
    }
}

/// A message handler for [WlRegistry] proxies.
#[allow(dead_code)]
pub trait MetaWlRegistryMessageHandler {
    /// bind an object to the display
    ///
    /// Binds a new, client-created object to the server using the
    /// specified name as the identifier.
    ///
    /// # Arguments
    ///
    /// - `name`: unique numeric name of the object
    /// - `id`: bounded object
    #[inline]
    fn bind(
        &mut self,
        _slf: &Rc<MetaWlRegistry>,
        name: u32,
        id: Rc<dyn Proxy>,
    ) {
        let res = _slf.send_bind(
            name,
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_registry.bind message: {}", Report::new(e));
        }
    }

    /// announce global object
    ///
    /// Notify the client of global objects.
    ///
    /// The event notifies the client that a global object with
    /// the given name is now available, and it implements the
    /// given version of the given interface.
    ///
    /// # Arguments
    ///
    /// - `name`: numeric name of the global object
    /// - `interface`: interface implemented by the object
    /// - `version`: interface version
    #[inline]
    fn global(
        &mut self,
        _slf: &Rc<MetaWlRegistry>,
        name: u32,
        interface: &str,
        version: u32,
    ) {
        let res = _slf.send_global(
            name,
            interface,
            version,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_registry.global message: {}", Report::new(e));
        }
    }

    /// announce removal of global object
    ///
    /// Notify the client of removed global objects.
    ///
    /// This event notifies the client that the global identified
    /// by name is no longer available.  If the client bound to
    /// the global using the bind request, the client should now
    /// destroy that object.
    ///
    /// The object remains valid and requests to the object will be
    /// ignored until the client destroys it, to avoid races between
    /// the global going away and a client sending a request to it.
    ///
    /// # Arguments
    ///
    /// - `name`: numeric name of the global object
    #[inline]
    fn global_remove(
        &mut self,
        _slf: &Rc<MetaWlRegistry>,
        name: u32,
    ) {
        let res = _slf.send_global_remove(
            name,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_registry.global_remove message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWlRegistry {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError);
                };
                offset += 1;
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError);
                };
                offset += 1;
                let arg1_interface = {
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
                let Some(&arg1_version) = msg.get(offset) else {
                    return Err(ObjectError);
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError);
                }
                let arg1_id = arg1;
                let arg1 = create_proxy_for_interface(&self.core.state, arg1_interface, arg1_version)?;
                arg1.core().set_client_id(client, arg1_id, arg1.clone())?;
                if let Some(handler) = handler {
                    (**handler).bind(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.bind(&self, arg0, arg1);
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
            0 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError);
                };
                offset += 1;
                let arg1 = {
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
                let Some(&arg2) = msg.get(offset) else {
                    return Err(ObjectError);
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError);
                }
                if let Some(handler) = handler {
                    (**handler).global(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.global(&self, arg0, arg1, arg2);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).global_remove(&self, arg0);
                } else {
                    DefaultMessageHandler.global_remove(&self, arg0);
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

