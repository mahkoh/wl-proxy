//! interface for activating surfaces
//!
//! A global interface used for informing the compositor about applications
//! being activated or started, or for applications to request to be
//! activated.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A xdg_activation_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XdgActivationV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn XdgActivationV1Handler>,
}

struct DefaultHandler;

impl XdgActivationV1Handler for DefaultHandler { }

impl ConcreteObject for XdgActivationV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::XdgActivationV1;
    const INTERFACE_NAME: &str = "xdg_activation_v1";
}

impl XdgActivationV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl XdgActivationV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn XdgActivationV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XdgActivationV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XdgActivationV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XdgActivationV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the xdg_activation object
    ///
    /// Notify the compositor that the xdg_activation object will no longer be
    /// used.
    ///
    /// The child objects created via this interface are unaffected and should
    /// be destroyed separately.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_activation_v1#{}.destroy()\n", id);
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

    /// destroy the xdg_activation object
    ///
    /// Notify the compositor that the xdg_activation object will no longer be
    /// used.
    ///
    /// The child objects created via this interface are unaffected and should
    /// be destroyed separately.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("xdg_activation_v1.destroy", &e);
        }
    }

    /// Since when the get_activation_token message is available.
    pub const MSG__GET_ACTIVATION_TOKEN__SINCE: u32 = 1;

    /// requests a token
    ///
    /// Creates an xdg_activation_token_v1 object that will provide
    /// the initiating client with a unique token for this activation. This
    /// token should be offered to the clients to be activated.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_get_activation_token(
        &self,
        id: &Rc<XdgActivationTokenV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_activation_v1#{}.get_activation_token(id: xdg_activation_token_v1#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
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
        Ok(())
    }

    /// requests a token
    ///
    /// Creates an xdg_activation_token_v1 object that will provide
    /// the initiating client with a unique token for this activation. This
    /// token should be offered to the clients to be activated.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_get_activation_token(
        &self,
        id: &Rc<XdgActivationTokenV1>,
    ) {
        let res = self.try_send_get_activation_token(
            id,
        );
        if let Err(e) = res {
            log_send("xdg_activation_v1.get_activation_token", &e);
        }
    }

    /// requests a token
    ///
    /// Creates an xdg_activation_token_v1 object that will provide
    /// the initiating client with a unique token for this activation. This
    /// token should be offered to the clients to be activated.
    #[inline]
    pub fn new_try_send_get_activation_token(
        &self,
    ) -> Result<Rc<XdgActivationTokenV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_activation_token(
            &id,
        )?;
        Ok(id)
    }

    /// requests a token
    ///
    /// Creates an xdg_activation_token_v1 object that will provide
    /// the initiating client with a unique token for this activation. This
    /// token should be offered to the clients to be activated.
    #[inline]
    pub fn new_send_get_activation_token(
        &self,
    ) -> Rc<XdgActivationTokenV1> {
        let id = self.core.create_child();
        self.send_get_activation_token(
            &id,
        );
        id
    }

    /// Since when the activate message is available.
    pub const MSG__ACTIVATE__SINCE: u32 = 1;

    /// notify new interaction being available
    ///
    /// Requests surface activation. It's up to the compositor to display
    /// this information as desired, for example by placing the surface above
    /// the rest.
    ///
    /// The compositor may know who requested this by checking the activation
    /// token and might decide not to follow through with the activation if it's
    /// considered unwanted.
    ///
    /// Compositors can ignore unknown activation tokens when an invalid
    /// token is passed.
    ///
    /// # Arguments
    ///
    /// - `token`: the activation token of the initiating client
    /// - `surface`: the wl_surface to activate
    #[inline]
    pub fn try_send_activate(
        &self,
        token: &str,
        surface: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            token,
            surface,
        );
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_activation_v1#{}.activate(token: {:?}, surface: wl_surface#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1_id);
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
        ]);
        fmt.string(arg0);
        fmt.words([
            arg1_id,
        ]);
        Ok(())
    }

    /// notify new interaction being available
    ///
    /// Requests surface activation. It's up to the compositor to display
    /// this information as desired, for example by placing the surface above
    /// the rest.
    ///
    /// The compositor may know who requested this by checking the activation
    /// token and might decide not to follow through with the activation if it's
    /// considered unwanted.
    ///
    /// Compositors can ignore unknown activation tokens when an invalid
    /// token is passed.
    ///
    /// # Arguments
    ///
    /// - `token`: the activation token of the initiating client
    /// - `surface`: the wl_surface to activate
    #[inline]
    pub fn send_activate(
        &self,
        token: &str,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_activate(
            token,
            surface,
        );
        if let Err(e) = res {
            log_send("xdg_activation_v1.activate", &e);
        }
    }
}

/// A message handler for [`XdgActivationV1`] proxies.
pub trait XdgActivationV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<XdgActivationV1>) {
        slf.core.delete_id();
    }

    /// destroy the xdg_activation object
    ///
    /// Notify the compositor that the xdg_activation object will no longer be
    /// used.
    ///
    /// The child objects created via this interface are unaffected and should
    /// be destroyed separately.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<XdgActivationV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("xdg_activation_v1.destroy", &e);
        }
    }

    /// requests a token
    ///
    /// Creates an xdg_activation_token_v1 object that will provide
    /// the initiating client with a unique token for this activation. This
    /// token should be offered to the clients to be activated.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn handle_get_activation_token(
        &mut self,
        slf: &Rc<XdgActivationV1>,
        id: &Rc<XdgActivationTokenV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_activation_token(
            id,
        );
        if let Err(e) = res {
            log_forward("xdg_activation_v1.get_activation_token", &e);
        }
    }

    /// notify new interaction being available
    ///
    /// Requests surface activation. It's up to the compositor to display
    /// this information as desired, for example by placing the surface above
    /// the rest.
    ///
    /// The compositor may know who requested this by checking the activation
    /// token and might decide not to follow through with the activation if it's
    /// considered unwanted.
    ///
    /// Compositors can ignore unknown activation tokens when an invalid
    /// token is passed.
    ///
    /// # Arguments
    ///
    /// - `token`: the activation token of the initiating client
    /// - `surface`: the wl_surface to activate
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_activate(
        &mut self,
        slf: &Rc<XdgActivationV1>,
        token: &str,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_activate(
            token,
            surface,
        );
        if let Err(e) = res {
            log_forward("xdg_activation_v1.activate", &e);
        }
    }
}

impl ObjectPrivate for XdgActivationV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::XdgActivationV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_activation_v1#{}.destroy()\n", client_id, id);
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_activation_v1#{}.get_activation_token(id: xdg_activation_token_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = XdgActivationTokenV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_get_activation_token(&self, arg0);
                } else {
                    DefaultHandler.handle_get_activation_token(&self, arg0);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "token")?;
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("surface")));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_activation_v1#{}.activate(token: {:?}, surface: wl_surface#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_activate(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_activate(&self, arg0, arg1);
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
            1 => "get_activation_token",
            2 => "activate",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for XdgActivationV1 {
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

