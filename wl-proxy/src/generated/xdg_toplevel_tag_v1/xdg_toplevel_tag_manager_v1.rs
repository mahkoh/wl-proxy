//! protocol for setting toplevel tags
//!
//! In order to make some window properties like position, size,
//! "always on top" or user defined rules for window behavior persistent, the
//! compositor needs some way to identify windows even after the application
//! has been restarted.
//! This protocol allows clients to make this possible by setting a tag for
//! toplevels.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A xdg_toplevel_tag_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaXdgToplevelTagManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaXdgToplevelTagManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaXdgToplevelTagManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaXdgToplevelTagManagerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaXdgToplevelTagManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::XdgToplevelTagManagerV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaXdgToplevelTagManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaXdgToplevelTagManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaXdgToplevelTagManagerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy toplevel tag object
    ///
    /// Destroy this toplevel tag manager object. This request has no other
    /// effects.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= xdg_toplevel_tag_manager_v1#{}.destroy()", id);
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

    /// Since when the set_toplevel_tag message is available.
    #[allow(dead_code)]
    pub const MSG__SET_TOPLEVEL_TAG__SINCE: u32 = 1;

    /// set tag
    ///
    /// Set a tag for a toplevel. The tag may be shown to the user in UI, so
    /// it's preferable for it to be human readable, but it must be suitable
    /// for configuration files and should not be translated.
    /// Suitable tags would for example be "main window", "settings",
    /// "e-mail composer" or similar.
    ///
    /// The tag does not need to be unique across applications, and the client
    /// may set the same tag for multiple windows, for example if the user has
    /// opened the same UI twice. How the potentially resulting conflicts are
    /// handled is compositor policy.
    ///
    /// The client should set the tag as part of the initial commit on the
    /// associated toplevel, but it may set it at any time afterwards as well,
    /// for example if the purpose of the toplevel changes.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    /// - `tag`: untranslated tag
    #[inline]
    pub fn send_set_toplevel_tag(
        &self,
        toplevel: &Rc<MetaXdgToplevel>,
        tag: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            toplevel,
            tag,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("toplevel")),
            Some(id) => id,
        };
        eprintln!("server      <= xdg_toplevel_tag_manager_v1#{}.set_toplevel_tag(toplevel: xdg_toplevel#{}, tag: {:?})", id, arg0_id, arg1);
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

    /// Since when the set_toplevel_description message is available.
    #[allow(dead_code)]
    pub const MSG__SET_TOPLEVEL_DESCRIPTION__SINCE: u32 = 1;

    /// set description
    ///
    /// Set a description for a toplevel. This description may be shown to the
    /// user in UI or read by a screen reader for accessibility purposes, and
    /// should be translated.
    /// It is recommended to make the description the translation of the tag.
    ///
    /// The client should set the description as part of the initial commit on
    /// the associated toplevel, but it may set it at any time afterwards as
    /// well, for example if the purpose of the toplevel changes.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    /// - `description`: translated description
    #[inline]
    pub fn send_set_toplevel_description(
        &self,
        toplevel: &Rc<MetaXdgToplevel>,
        description: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            toplevel,
            description,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("toplevel")),
            Some(id) => id,
        };
        eprintln!("server      <= xdg_toplevel_tag_manager_v1#{}.set_toplevel_description(toplevel: xdg_toplevel#{}, description: {:?})", id, arg0_id, arg1);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
            arg0_id,
        ]);
        fmt.string(arg1);
        Ok(())
    }
}

/// A message handler for [XdgToplevelTagManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaXdgToplevelTagManagerV1MessageHandler {
    /// destroy toplevel tag object
    ///
    /// Destroy this toplevel tag manager object. This request has no other
    /// effects.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaXdgToplevelTagManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel_tag_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// set tag
    ///
    /// Set a tag for a toplevel. The tag may be shown to the user in UI, so
    /// it's preferable for it to be human readable, but it must be suitable
    /// for configuration files and should not be translated.
    /// Suitable tags would for example be "main window", "settings",
    /// "e-mail composer" or similar.
    ///
    /// The tag does not need to be unique across applications, and the client
    /// may set the same tag for multiple windows, for example if the user has
    /// opened the same UI twice. How the potentially resulting conflicts are
    /// handled is compositor policy.
    ///
    /// The client should set the tag as part of the initial commit on the
    /// associated toplevel, but it may set it at any time afterwards as well,
    /// for example if the purpose of the toplevel changes.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    /// - `tag`: untranslated tag
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn set_toplevel_tag(
        &mut self,
        _slf: &Rc<MetaXdgToplevelTagManagerV1>,
        toplevel: &Rc<MetaXdgToplevel>,
        tag: &str,
    ) {
        let res = _slf.send_set_toplevel_tag(
            toplevel,
            tag,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel_tag_manager_v1.set_toplevel_tag message: {}", Report::new(e));
        }
    }

    /// set description
    ///
    /// Set a description for a toplevel. This description may be shown to the
    /// user in UI or read by a screen reader for accessibility purposes, and
    /// should be translated.
    /// It is recommended to make the description the translation of the tag.
    ///
    /// The client should set the description as part of the initial commit on
    /// the associated toplevel, but it may set it at any time afterwards as
    /// well, for example if the purpose of the toplevel changes.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    /// - `description`: translated description
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn set_toplevel_description(
        &mut self,
        _slf: &Rc<MetaXdgToplevelTagManagerV1>,
        toplevel: &Rc<MetaXdgToplevel>,
        description: &str,
    ) {
        let res = _slf.send_set_toplevel_description(
            toplevel,
            description,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel_tag_manager_v1.set_toplevel_description message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaXdgToplevelTagManagerV1 {
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
                eprintln!("client#{:04} -> xdg_toplevel_tag_manager_v1#{}.destroy()", client.endpoint.id, msg[0]);
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
                    return Err(ObjectError::MissingArgument("toplevel"));
                };
                offset += 1;
                let arg1 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("tag"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("tag"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("tag"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("tag"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                eprintln!("client#{:04} -> xdg_toplevel_tag_manager_v1#{}.set_toplevel_tag(toplevel: xdg_toplevel#{}, tag: {:?})", client.endpoint.id, msg[0], arg0, arg1);
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaXdgToplevel>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("toplevel", o.core().interface, ProxyInterface::XdgToplevel));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).set_toplevel_tag(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.set_toplevel_tag(&self, arg0, arg1);
                }
            }
            2 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("toplevel"));
                };
                offset += 1;
                let arg1 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("description"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("description"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("description"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("description"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                eprintln!("client#{:04} -> xdg_toplevel_tag_manager_v1#{}.set_toplevel_description(toplevel: xdg_toplevel#{}, description: {:?})", client.endpoint.id, msg[0], arg0, arg1);
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaXdgToplevel>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("toplevel", o.core().interface, ProxyInterface::XdgToplevel));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).set_toplevel_description(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.set_toplevel_description(&self, arg0, arg1);
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
            1 => "set_toplevel_tag",
            2 => "set_toplevel_description",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

