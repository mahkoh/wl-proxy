//! A session for an application
//!
//! A xdg_toplevel_session_v1 resource acts as a handle for the given
//! toplevel in the session. It allows for receiving events after a
//! toplevel state was restored, and has the requests to manage them.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A xdg_toplevel_session_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XdgToplevelSessionV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn XdgToplevelSessionV1Handler>,
}

struct DefaultHandler;

impl XdgToplevelSessionV1Handler for DefaultHandler { }

impl ConcreteObject for XdgToplevelSessionV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::XdgToplevelSessionV1;
    const INTERFACE_NAME: &str = "xdg_toplevel_session_v1";
}

impl XdgToplevelSessionV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl XdgToplevelSessionV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn XdgToplevelSessionV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XdgToplevelSessionV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XdgToplevelSessionV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XdgToplevelSessionV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// Destroy the object
    ///
    /// Destroy the object. This has no effect over window management of the
    /// associated toplevel.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_toplevel_session_v1#{}.destroy()\n", id);
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

    /// Destroy the object
    ///
    /// Destroy the object. This has no effect over window management of the
    /// associated toplevel.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("xdg_toplevel_session_v1.destroy", &e);
        }
    }

    /// Since when the rename message is available.
    pub const MSG__RENAME__SINCE: u32 = 1;

    /// change the name of toplevel session
    ///
    /// Renames the toplevel session. The new name can be used in subsequent requests
    /// to identify this session object. The state associated with this toplevel
    /// session will be preserved.
    ///
    /// If the xdg_session_v1 already contains a toplevel with the specified name,
    /// the 'name_in_use' protocol error will be raised.
    ///
    /// # Arguments
    ///
    /// - `name`: new name to identify the toplevel
    #[inline]
    pub fn try_send_rename(
        &self,
        name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            name,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_toplevel_session_v1#{}.rename(name: {:?})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// change the name of toplevel session
    ///
    /// Renames the toplevel session. The new name can be used in subsequent requests
    /// to identify this session object. The state associated with this toplevel
    /// session will be preserved.
    ///
    /// If the xdg_session_v1 already contains a toplevel with the specified name,
    /// the 'name_in_use' protocol error will be raised.
    ///
    /// # Arguments
    ///
    /// - `name`: new name to identify the toplevel
    #[inline]
    pub fn send_rename(
        &self,
        name: &str,
    ) {
        let res = self.try_send_rename(
            name,
        );
        if let Err(e) = res {
            log_send("xdg_toplevel_session_v1.rename", &e);
        }
    }

    /// Since when the restored message is available.
    pub const MSG__RESTORED__SINCE: u32 = 1;

    /// a toplevel's session has been restored
    ///
    /// The "restored" event is emitted prior to the first
    /// xdg_toplevel.configure for the toplevel. It will only be emitted after
    /// xdg_session_v1.restore_toplevel, and the initial empty surface state has
    /// been applied, and it indicates that the surface's session is being
    /// restored with this configure event.
    #[inline]
    pub fn try_send_restored(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xdg_toplevel_session_v1#{}.restored()\n", client_id, id);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id);
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
        Ok(())
    }

    /// a toplevel's session has been restored
    ///
    /// The "restored" event is emitted prior to the first
    /// xdg_toplevel.configure for the toplevel. It will only be emitted after
    /// xdg_session_v1.restore_toplevel, and the initial empty surface state has
    /// been applied, and it indicates that the surface's session is being
    /// restored with this configure event.
    #[inline]
    pub fn send_restored(
        &self,
    ) {
        let res = self.try_send_restored(
        );
        if let Err(e) = res {
            log_send("xdg_toplevel_session_v1.restored", &e);
        }
    }
}

/// A message handler for [`XdgToplevelSessionV1`] proxies.
pub trait XdgToplevelSessionV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<XdgToplevelSessionV1>) {
        slf.core.delete_id();
    }

    /// Destroy the object
    ///
    /// Destroy the object. This has no effect over window management of the
    /// associated toplevel.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<XdgToplevelSessionV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("xdg_toplevel_session_v1.destroy", &e);
        }
    }

    /// change the name of toplevel session
    ///
    /// Renames the toplevel session. The new name can be used in subsequent requests
    /// to identify this session object. The state associated with this toplevel
    /// session will be preserved.
    ///
    /// If the xdg_session_v1 already contains a toplevel with the specified name,
    /// the 'name_in_use' protocol error will be raised.
    ///
    /// # Arguments
    ///
    /// - `name`: new name to identify the toplevel
    #[inline]
    fn handle_rename(
        &mut self,
        slf: &Rc<XdgToplevelSessionV1>,
        name: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_rename(
            name,
        );
        if let Err(e) = res {
            log_forward("xdg_toplevel_session_v1.rename", &e);
        }
    }

    /// a toplevel's session has been restored
    ///
    /// The "restored" event is emitted prior to the first
    /// xdg_toplevel.configure for the toplevel. It will only be emitted after
    /// xdg_session_v1.restore_toplevel, and the initial empty surface state has
    /// been applied, and it indicates that the surface's session is being
    /// restored with this configure event.
    #[inline]
    fn handle_restored(
        &mut self,
        slf: &Rc<XdgToplevelSessionV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_restored(
        );
        if let Err(e) = res {
            log_forward("xdg_toplevel_session_v1.restored", &e);
        }
    }
}

impl ObjectPrivate for XdgToplevelSessionV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::XdgToplevelSessionV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_toplevel_session_v1#{}.destroy()\n", client_id, id);
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
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "name")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_toplevel_session_v1#{}.rename(name: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_rename(&self, arg0);
                } else {
                    DefaultHandler.handle_rename(&self, arg0);
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
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xdg_toplevel_session_v1#{}.restored()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_restored(&self);
                } else {
                    DefaultHandler.handle_restored(&self);
                }
            }
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
        Ok(())
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            1 => "rename",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "restored",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for XdgToplevelSessionV1 {
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

