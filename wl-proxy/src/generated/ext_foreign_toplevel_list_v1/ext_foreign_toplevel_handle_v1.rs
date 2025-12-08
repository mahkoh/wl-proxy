//! a mapped toplevel
//!
//! A ext_foreign_toplevel_handle_v1 object represents a mapped toplevel
//! window. A single app may have multiple mapped toplevels.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_foreign_toplevel_handle_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaExtForeignToplevelHandleV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaExtForeignToplevelHandleV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaExtForeignToplevelHandleV1MessageHandler for DefaultMessageHandler { }

impl MetaExtForeignToplevelHandleV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaExtForeignToplevelHandleV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaExtForeignToplevelHandleV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaExtForeignToplevelHandleV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the ext_foreign_toplevel_handle_v1 object
    ///
    /// This request should be used when the client will no longer use the handle
    /// or after the closed event has been received to allow destruction of the
    /// object.
    ///
    /// When a handle is destroyed, a new handle may not be created by the server
    /// until the toplevel is unmapped and then remapped. Destroying a toplevel handle
    /// is not recommended unless the client is cleaning up child objects
    /// before destroying the ext_foreign_toplevel_list_v1 object, the toplevel
    /// was closed or the toplevel handle will not be used in the future.
    ///
    /// Other protocols which extend the ext_foreign_toplevel_handle_v1
    /// interface should require destructors for extension interfaces be
    /// called before allowing the toplevel handle to be destroyed.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
        ]);
        Ok(())
    }

    /// Since when the closed message is available.
    #[allow(dead_code)]
    pub const MSG__CLOSED__SINCE: u32 = 1;

    /// the toplevel has been closed
    ///
    /// The server will emit no further events on the ext_foreign_toplevel_handle_v1
    /// after this event. Any requests received aside from the destroy request must
    /// be ignored. Upon receiving this event, the client should destroy the handle.
    ///
    /// Other protocols which extend the ext_foreign_toplevel_handle_v1
    /// interface must also ignore requests other than destructors.
    #[inline]
    pub fn send_closed(
        &self,
    ) -> Result<(), ObjectError> {
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
        ]);
        Ok(())
    }

    /// Since when the done message is available.
    #[allow(dead_code)]
    pub const MSG__DONE__SINCE: u32 = 1;

    /// all information about the toplevel has been sent
    ///
    /// This event is sent after all changes in the toplevel state have
    /// been sent.
    ///
    /// This allows changes to the ext_foreign_toplevel_handle_v1 properties
    /// to be atomically applied. Other protocols which extend the
    /// ext_foreign_toplevel_handle_v1 interface may use this event to also
    /// atomically apply any pending state.
    ///
    /// This event must not be sent after the ext_foreign_toplevel_handle_v1.closed
    /// event.
    #[inline]
    pub fn send_done(
        &self,
    ) -> Result<(), ObjectError> {
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
        ]);
        Ok(())
    }

    /// Since when the title message is available.
    #[allow(dead_code)]
    pub const MSG__TITLE__SINCE: u32 = 1;

    /// title change
    ///
    /// The title of the toplevel has changed.
    ///
    /// The configured state must not be applied immediately. See
    /// ext_foreign_toplevel_handle_v1.done for details.
    ///
    /// # Arguments
    ///
    /// - `title`:
    #[inline]
    pub fn send_title(
        &self,
        title: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            title,
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
            2,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the app_id message is available.
    #[allow(dead_code)]
    pub const MSG__APP_ID__SINCE: u32 = 1;

    /// app_id change
    ///
    /// The app id of the toplevel has changed.
    ///
    /// The configured state must not be applied immediately. See
    /// ext_foreign_toplevel_handle_v1.done for details.
    ///
    /// # Arguments
    ///
    /// - `app_id`:
    #[inline]
    pub fn send_app_id(
        &self,
        app_id: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            app_id,
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
            3,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the identifier message is available.
    #[allow(dead_code)]
    pub const MSG__IDENTIFIER__SINCE: u32 = 1;

    /// a stable identifier for a toplevel
    ///
    /// This identifier is used to check if two or more toplevel handles belong
    /// to the same toplevel.
    ///
    /// The identifier is useful for command line tools or privileged clients
    /// which may need to reference an exact toplevel across processes or
    /// instances of the ext_foreign_toplevel_list_v1 global.
    ///
    /// The compositor must only send this event when the handle is created.
    ///
    /// The identifier must be unique per toplevel and it's handles. Two different
    /// toplevels must not have the same identifier. The identifier is only valid
    /// as long as the toplevel is mapped. If the toplevel is unmapped the identifier
    /// must not be reused. An identifier must not be reused by the compositor to
    /// ensure there are no races when sharing identifiers between processes.
    ///
    /// An identifier is a string that contains up to 32 printable ASCII bytes.
    /// An identifier must not be an empty string. It is recommended that a
    /// compositor includes an opaque generation value in identifiers. How the
    /// generation value is used when generating the identifier is implementation
    /// dependent.
    ///
    /// # Arguments
    ///
    /// - `identifier`:
    #[inline]
    pub fn send_identifier(
        &self,
        identifier: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            identifier,
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
            4,
        ]);
        fmt.string(arg0);
        Ok(())
    }
}

/// A message handler for [ExtForeignToplevelHandleV1] proxies.
#[allow(dead_code)]
pub trait MetaExtForeignToplevelHandleV1MessageHandler {
    /// destroy the ext_foreign_toplevel_handle_v1 object
    ///
    /// This request should be used when the client will no longer use the handle
    /// or after the closed event has been received to allow destruction of the
    /// object.
    ///
    /// When a handle is destroyed, a new handle may not be created by the server
    /// until the toplevel is unmapped and then remapped. Destroying a toplevel handle
    /// is not recommended unless the client is cleaning up child objects
    /// before destroying the ext_foreign_toplevel_list_v1 object, the toplevel
    /// was closed or the toplevel handle will not be used in the future.
    ///
    /// Other protocols which extend the ext_foreign_toplevel_handle_v1
    /// interface should require destructors for extension interfaces be
    /// called before allowing the toplevel handle to be destroyed.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaExtForeignToplevelHandleV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_foreign_toplevel_handle_v1.destroy message: {}", Report::new(e));
        }
    }

    /// the toplevel has been closed
    ///
    /// The server will emit no further events on the ext_foreign_toplevel_handle_v1
    /// after this event. Any requests received aside from the destroy request must
    /// be ignored. Upon receiving this event, the client should destroy the handle.
    ///
    /// Other protocols which extend the ext_foreign_toplevel_handle_v1
    /// interface must also ignore requests other than destructors.
    #[inline]
    fn closed(
        &mut self,
        _slf: &Rc<MetaExtForeignToplevelHandleV1>,
    ) {
        let res = _slf.send_closed(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_foreign_toplevel_handle_v1.closed message: {}", Report::new(e));
        }
    }

    /// all information about the toplevel has been sent
    ///
    /// This event is sent after all changes in the toplevel state have
    /// been sent.
    ///
    /// This allows changes to the ext_foreign_toplevel_handle_v1 properties
    /// to be atomically applied. Other protocols which extend the
    /// ext_foreign_toplevel_handle_v1 interface may use this event to also
    /// atomically apply any pending state.
    ///
    /// This event must not be sent after the ext_foreign_toplevel_handle_v1.closed
    /// event.
    #[inline]
    fn done(
        &mut self,
        _slf: &Rc<MetaExtForeignToplevelHandleV1>,
    ) {
        let res = _slf.send_done(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_foreign_toplevel_handle_v1.done message: {}", Report::new(e));
        }
    }

    /// title change
    ///
    /// The title of the toplevel has changed.
    ///
    /// The configured state must not be applied immediately. See
    /// ext_foreign_toplevel_handle_v1.done for details.
    ///
    /// # Arguments
    ///
    /// - `title`:
    #[inline]
    fn title(
        &mut self,
        _slf: &Rc<MetaExtForeignToplevelHandleV1>,
        title: &str,
    ) {
        let res = _slf.send_title(
            title,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_foreign_toplevel_handle_v1.title message: {}", Report::new(e));
        }
    }

    /// app_id change
    ///
    /// The app id of the toplevel has changed.
    ///
    /// The configured state must not be applied immediately. See
    /// ext_foreign_toplevel_handle_v1.done for details.
    ///
    /// # Arguments
    ///
    /// - `app_id`:
    #[inline]
    fn app_id(
        &mut self,
        _slf: &Rc<MetaExtForeignToplevelHandleV1>,
        app_id: &str,
    ) {
        let res = _slf.send_app_id(
            app_id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_foreign_toplevel_handle_v1.app_id message: {}", Report::new(e));
        }
    }

    /// a stable identifier for a toplevel
    ///
    /// This identifier is used to check if two or more toplevel handles belong
    /// to the same toplevel.
    ///
    /// The identifier is useful for command line tools or privileged clients
    /// which may need to reference an exact toplevel across processes or
    /// instances of the ext_foreign_toplevel_list_v1 global.
    ///
    /// The compositor must only send this event when the handle is created.
    ///
    /// The identifier must be unique per toplevel and it's handles. Two different
    /// toplevels must not have the same identifier. The identifier is only valid
    /// as long as the toplevel is mapped. If the toplevel is unmapped the identifier
    /// must not be reused. An identifier must not be reused by the compositor to
    /// ensure there are no races when sharing identifiers between processes.
    ///
    /// An identifier is a string that contains up to 32 printable ASCII bytes.
    /// An identifier must not be an empty string. It is recommended that a
    /// compositor includes an opaque generation value in identifiers. How the
    /// generation value is used when generating the identifier is implementation
    /// dependent.
    ///
    /// # Arguments
    ///
    /// - `identifier`:
    #[inline]
    fn identifier(
        &mut self,
        _slf: &Rc<MetaExtForeignToplevelHandleV1>,
        identifier: &str,
    ) {
        let res = _slf.send_identifier(
            identifier,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_foreign_toplevel_handle_v1.identifier message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaExtForeignToplevelHandleV1 {
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
                if let Some(handler) = handler {
                    (**handler).closed(&self);
                } else {
                    DefaultMessageHandler.closed(&self);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).done(&self);
                } else {
                    DefaultMessageHandler.done(&self);
                }
            }
            2 => {
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
                    (**handler).title(&self, arg0);
                } else {
                    DefaultMessageHandler.title(&self, arg0);
                }
            }
            3 => {
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
                    (**handler).app_id(&self, arg0);
                } else {
                    DefaultMessageHandler.app_id(&self, arg0);
                }
            }
            4 => {
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
                    (**handler).identifier(&self, arg0);
                } else {
                    DefaultMessageHandler.identifier(&self, arg0);
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

