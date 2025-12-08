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

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A xdg_toplevel_tag_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XdgToplevelTagManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn XdgToplevelTagManagerV1Handler>,
}

struct DefaultHandler;

impl XdgToplevelTagManagerV1Handler for DefaultHandler { }

impl ConcreteObject for XdgToplevelTagManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::XdgToplevelTagManagerV1;
    const INTERFACE_NAME: &str = "xdg_toplevel_tag_manager_v1";
}

impl XdgToplevelTagManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl XdgToplevelTagManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn XdgToplevelTagManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XdgToplevelTagManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XdgToplevelTagManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XdgToplevelTagManagerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy toplevel tag object
    ///
    /// Destroy this toplevel tag manager object. This request has no other
    /// effects.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_toplevel_tag_manager_v1#{}.destroy()\n", id);
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

    /// destroy toplevel tag object
    ///
    /// Destroy this toplevel tag manager object. This request has no other
    /// effects.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("xdg_toplevel_tag_manager_v1.destroy", &e);
        }
    }

    /// Since when the set_toplevel_tag message is available.
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
    pub fn try_send_set_toplevel_tag(
        &self,
        toplevel: &Rc<XdgToplevel>,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("toplevel"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_toplevel_tag_manager_v1#{}.set_toplevel_tag(toplevel: xdg_toplevel#{}, tag: {:?})\n", id, arg0, arg1);
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
        toplevel: &Rc<XdgToplevel>,
        tag: &str,
    ) {
        let res = self.try_send_set_toplevel_tag(
            toplevel,
            tag,
        );
        if let Err(e) = res {
            log_send("xdg_toplevel_tag_manager_v1.set_toplevel_tag", &e);
        }
    }

    /// Since when the set_toplevel_description message is available.
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
    pub fn try_send_set_toplevel_description(
        &self,
        toplevel: &Rc<XdgToplevel>,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("toplevel"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_toplevel_tag_manager_v1#{}.set_toplevel_description(toplevel: xdg_toplevel#{}, description: {:?})\n", id, arg0, arg1);
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
            2,
            arg0_id,
        ]);
        fmt.string(arg1);
        Ok(())
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
    #[inline]
    pub fn send_set_toplevel_description(
        &self,
        toplevel: &Rc<XdgToplevel>,
        description: &str,
    ) {
        let res = self.try_send_set_toplevel_description(
            toplevel,
            description,
        );
        if let Err(e) = res {
            log_send("xdg_toplevel_tag_manager_v1.set_toplevel_description", &e);
        }
    }
}

/// A message handler for [`XdgToplevelTagManagerV1`] proxies.
pub trait XdgToplevelTagManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<XdgToplevelTagManagerV1>) {
        slf.core.delete_id();
    }

    /// destroy toplevel tag object
    ///
    /// Destroy this toplevel tag manager object. This request has no other
    /// effects.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<XdgToplevelTagManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("xdg_toplevel_tag_manager_v1.destroy", &e);
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
    fn handle_set_toplevel_tag(
        &mut self,
        slf: &Rc<XdgToplevelTagManagerV1>,
        toplevel: &Rc<XdgToplevel>,
        tag: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_toplevel_tag(
            toplevel,
            tag,
        );
        if let Err(e) = res {
            log_forward("xdg_toplevel_tag_manager_v1.set_toplevel_tag", &e);
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
    fn handle_set_toplevel_description(
        &mut self,
        slf: &Rc<XdgToplevelTagManagerV1>,
        toplevel: &Rc<XdgToplevel>,
        description: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_toplevel_description(
            toplevel,
            description,
        );
        if let Err(e) = res {
            log_forward("xdg_toplevel_tag_manager_v1.set_toplevel_description", &e);
        }
    }
}

impl ObjectPrivate for XdgToplevelTagManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::XdgToplevelTagManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_toplevel_tag_manager_v1#{}.destroy()\n", client_id, id);
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
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("toplevel")));
                };
                offset += 1;
                let arg1;
                (arg1, offset) = parse_string::<NonNullString>(msg, offset, "tag")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_toplevel_tag_manager_v1#{}.set_toplevel_tag(toplevel: xdg_toplevel#{}, tag: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<XdgToplevel>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("toplevel", o.core().interface, ObjectInterface::XdgToplevel)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_toplevel_tag(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_toplevel_tag(&self, arg0, arg1);
                }
            }
            2 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("toplevel")));
                };
                offset += 1;
                let arg1;
                (arg1, offset) = parse_string::<NonNullString>(msg, offset, "description")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_toplevel_tag_manager_v1#{}.set_toplevel_description(toplevel: xdg_toplevel#{}, description: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<XdgToplevel>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("toplevel", o.core().interface, ObjectInterface::XdgToplevel)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_toplevel_description(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_toplevel_description(&self, arg0, arg1);
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

impl Object for XdgToplevelTagManagerV1 {
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

