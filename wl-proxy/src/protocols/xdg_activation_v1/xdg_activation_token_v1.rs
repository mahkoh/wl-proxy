//! an exported activation handle
//!
//! An object for setting up a token and receiving a token handle that can
//! be passed as an activation token to another client.
//!
//! The object is created using the xdg_activation_v1.get_activation_token
//! request. This object should then be populated with the app_id, surface
//! and serial information and committed. The compositor shall then issue a
//! done event with the token. In case the request's parameters are invalid,
//! the compositor will provide an invalid token.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A xdg_activation_token_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XdgActivationTokenV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn XdgActivationTokenV1Handler>,
}

struct DefaultHandler;

impl XdgActivationTokenV1Handler for DefaultHandler { }

impl ConcreteObject for XdgActivationTokenV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::XdgActivationTokenV1;
    const INTERFACE_NAME: &str = "xdg_activation_token_v1";
}

impl XdgActivationTokenV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl XdgActivationTokenV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn XdgActivationTokenV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XdgActivationTokenV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XdgActivationTokenV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XdgActivationTokenV1 {
    /// Since when the set_serial message is available.
    pub const MSG__SET_SERIAL__SINCE: u32 = 1;

    /// specifies the seat and serial of the activating event
    ///
    /// Provides information about the seat and serial event that requested the
    /// token.
    ///
    /// The serial can come from an input or focus event. For instance, if a
    /// click triggers the launch of a third-party client, the launcher client
    /// should send a set_serial request with the serial and seat from the
    /// wl_pointer.button event.
    ///
    /// Some compositors might refuse to activate toplevels when the token
    /// doesn't have a valid and recent enough event serial.
    ///
    /// Must be sent before commit. This information is optional.
    ///
    /// # Arguments
    ///
    /// - `serial`: the serial of the event that triggered the activation
    /// - `seat`: the wl_seat of the event
    #[inline]
    pub fn try_send_set_serial(
        &self,
        serial: u32,
        seat: &Rc<WlSeat>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            serial,
            seat,
        );
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("seat"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_activation_token_v1#{}.set_serial(serial: {}, seat: wl_seat#{})\n", id, arg0, arg1);
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
            0,
            arg0,
            arg1_id,
        ]);
        Ok(())
    }

    /// specifies the seat and serial of the activating event
    ///
    /// Provides information about the seat and serial event that requested the
    /// token.
    ///
    /// The serial can come from an input or focus event. For instance, if a
    /// click triggers the launch of a third-party client, the launcher client
    /// should send a set_serial request with the serial and seat from the
    /// wl_pointer.button event.
    ///
    /// Some compositors might refuse to activate toplevels when the token
    /// doesn't have a valid and recent enough event serial.
    ///
    /// Must be sent before commit. This information is optional.
    ///
    /// # Arguments
    ///
    /// - `serial`: the serial of the event that triggered the activation
    /// - `seat`: the wl_seat of the event
    #[inline]
    pub fn send_set_serial(
        &self,
        serial: u32,
        seat: &Rc<WlSeat>,
    ) {
        let res = self.try_send_set_serial(
            serial,
            seat,
        );
        if let Err(e) = res {
            log_send("xdg_activation_token_v1.set_serial", &e);
        }
    }

    /// Since when the set_app_id message is available.
    pub const MSG__SET_APP_ID__SINCE: u32 = 1;

    /// specifies the application being activated
    ///
    /// The requesting client can specify an app_id to associate the token
    /// being created with it.
    ///
    /// Must be sent before commit. This information is optional.
    ///
    /// # Arguments
    ///
    /// - `app_id`: the application id of the client being activated.
    #[inline]
    pub fn try_send_set_app_id(
        &self,
        app_id: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            app_id,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_activation_token_v1#{}.set_app_id(app_id: {:?})\n", id, arg0);
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

    /// specifies the application being activated
    ///
    /// The requesting client can specify an app_id to associate the token
    /// being created with it.
    ///
    /// Must be sent before commit. This information is optional.
    ///
    /// # Arguments
    ///
    /// - `app_id`: the application id of the client being activated.
    #[inline]
    pub fn send_set_app_id(
        &self,
        app_id: &str,
    ) {
        let res = self.try_send_set_app_id(
            app_id,
        );
        if let Err(e) = res {
            log_send("xdg_activation_token_v1.set_app_id", &e);
        }
    }

    /// Since when the set_surface message is available.
    pub const MSG__SET_SURFACE__SINCE: u32 = 1;

    /// specifies the surface requesting activation
    ///
    /// This request sets the surface requesting the activation. Note, this is
    /// different from the surface that will be activated.
    ///
    /// Some compositors might refuse to activate toplevels when the token
    /// doesn't have a requesting surface.
    ///
    /// Must be sent before commit. This information is optional.
    ///
    /// # Arguments
    ///
    /// - `surface`: the requesting surface
    #[inline]
    pub fn try_send_set_surface(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            surface,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_activation_token_v1#{}.set_surface(surface: wl_surface#{})\n", id, arg0);
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
            2,
            arg0_id,
        ]);
        Ok(())
    }

    /// specifies the surface requesting activation
    ///
    /// This request sets the surface requesting the activation. Note, this is
    /// different from the surface that will be activated.
    ///
    /// Some compositors might refuse to activate toplevels when the token
    /// doesn't have a requesting surface.
    ///
    /// Must be sent before commit. This information is optional.
    ///
    /// # Arguments
    ///
    /// - `surface`: the requesting surface
    #[inline]
    pub fn send_set_surface(
        &self,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_set_surface(
            surface,
        );
        if let Err(e) = res {
            log_send("xdg_activation_token_v1.set_surface", &e);
        }
    }

    /// Since when the commit message is available.
    pub const MSG__COMMIT__SINCE: u32 = 1;

    /// issues the token request
    ///
    /// Requests an activation token based on the different parameters that
    /// have been offered through set_serial, set_surface and set_app_id.
    #[inline]
    pub fn try_send_commit(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_activation_token_v1#{}.commit()\n", id);
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
            3,
        ]);
        Ok(())
    }

    /// issues the token request
    ///
    /// Requests an activation token based on the different parameters that
    /// have been offered through set_serial, set_surface and set_app_id.
    #[inline]
    pub fn send_commit(
        &self,
    ) {
        let res = self.try_send_commit(
        );
        if let Err(e) = res {
            log_send("xdg_activation_token_v1.commit", &e);
        }
    }

    /// Since when the done message is available.
    pub const MSG__DONE__SINCE: u32 = 1;

    /// the exported activation token
    ///
    /// The 'done' event contains the unique token of this activation request
    /// and notifies that the provider is done.
    ///
    /// # Arguments
    ///
    /// - `token`: the exported activation token
    #[inline]
    pub fn try_send_done(
        &self,
        token: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            token,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xdg_activation_token_v1#{}.done(token: {:?})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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

    /// the exported activation token
    ///
    /// The 'done' event contains the unique token of this activation request
    /// and notifies that the provider is done.
    ///
    /// # Arguments
    ///
    /// - `token`: the exported activation token
    #[inline]
    pub fn send_done(
        &self,
        token: &str,
    ) {
        let res = self.try_send_done(
            token,
        );
        if let Err(e) = res {
            log_send("xdg_activation_token_v1.done", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the xdg_activation_token_v1 object
    ///
    /// Notify the compositor that the xdg_activation_token_v1 object will no
    /// longer be used. The received token stays valid.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_activation_token_v1#{}.destroy()\n", id);
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
            4,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the xdg_activation_token_v1 object
    ///
    /// Notify the compositor that the xdg_activation_token_v1 object will no
    /// longer be used. The received token stays valid.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("xdg_activation_token_v1.destroy", &e);
        }
    }
}

/// A message handler for [`XdgActivationTokenV1`] proxies.
pub trait XdgActivationTokenV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<XdgActivationTokenV1>) {
        slf.core.delete_id();
    }

    /// specifies the seat and serial of the activating event
    ///
    /// Provides information about the seat and serial event that requested the
    /// token.
    ///
    /// The serial can come from an input or focus event. For instance, if a
    /// click triggers the launch of a third-party client, the launcher client
    /// should send a set_serial request with the serial and seat from the
    /// wl_pointer.button event.
    ///
    /// Some compositors might refuse to activate toplevels when the token
    /// doesn't have a valid and recent enough event serial.
    ///
    /// Must be sent before commit. This information is optional.
    ///
    /// # Arguments
    ///
    /// - `serial`: the serial of the event that triggered the activation
    /// - `seat`: the wl_seat of the event
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_serial(
        &mut self,
        slf: &Rc<XdgActivationTokenV1>,
        serial: u32,
        seat: &Rc<WlSeat>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_serial(
            serial,
            seat,
        );
        if let Err(e) = res {
            log_forward("xdg_activation_token_v1.set_serial", &e);
        }
    }

    /// specifies the application being activated
    ///
    /// The requesting client can specify an app_id to associate the token
    /// being created with it.
    ///
    /// Must be sent before commit. This information is optional.
    ///
    /// # Arguments
    ///
    /// - `app_id`: the application id of the client being activated.
    #[inline]
    fn handle_set_app_id(
        &mut self,
        slf: &Rc<XdgActivationTokenV1>,
        app_id: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_app_id(
            app_id,
        );
        if let Err(e) = res {
            log_forward("xdg_activation_token_v1.set_app_id", &e);
        }
    }

    /// specifies the surface requesting activation
    ///
    /// This request sets the surface requesting the activation. Note, this is
    /// different from the surface that will be activated.
    ///
    /// Some compositors might refuse to activate toplevels when the token
    /// doesn't have a requesting surface.
    ///
    /// Must be sent before commit. This information is optional.
    ///
    /// # Arguments
    ///
    /// - `surface`: the requesting surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_surface(
        &mut self,
        slf: &Rc<XdgActivationTokenV1>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_surface(
            surface,
        );
        if let Err(e) = res {
            log_forward("xdg_activation_token_v1.set_surface", &e);
        }
    }

    /// issues the token request
    ///
    /// Requests an activation token based on the different parameters that
    /// have been offered through set_serial, set_surface and set_app_id.
    #[inline]
    fn handle_commit(
        &mut self,
        slf: &Rc<XdgActivationTokenV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_commit(
        );
        if let Err(e) = res {
            log_forward("xdg_activation_token_v1.commit", &e);
        }
    }

    /// the exported activation token
    ///
    /// The 'done' event contains the unique token of this activation request
    /// and notifies that the provider is done.
    ///
    /// # Arguments
    ///
    /// - `token`: the exported activation token
    #[inline]
    fn handle_done(
        &mut self,
        slf: &Rc<XdgActivationTokenV1>,
        token: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_done(
            token,
        );
        if let Err(e) = res {
            log_forward("xdg_activation_token_v1.done", &e);
        }
    }

    /// destroy the xdg_activation_token_v1 object
    ///
    /// Notify the compositor that the xdg_activation_token_v1 object will no
    /// longer be used. The received token stays valid.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<XdgActivationTokenV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("xdg_activation_token_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for XdgActivationTokenV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::XdgActivationTokenV1, version),
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
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_activation_token_v1#{}.set_serial(serial: {}, seat: wl_seat#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("seat", o.core().interface, ObjectInterface::WlSeat)));
                };
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_set_serial(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_serial(&self, arg0, arg1);
                }
            }
            1 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "app_id")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_activation_token_v1#{}.set_app_id(app_id: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_app_id(&self, arg0);
                } else {
                    DefaultHandler.handle_set_app_id(&self, arg0);
                }
            }
            2 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_activation_token_v1#{}.set_surface(surface: wl_surface#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_surface(&self, arg0);
                } else {
                    DefaultHandler.handle_set_surface(&self, arg0);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_activation_token_v1#{}.commit()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_commit(&self);
                } else {
                    DefaultHandler.handle_commit(&self);
                }
            }
            4 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_activation_token_v1#{}.destroy()\n", client_id, id);
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
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "token")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xdg_activation_token_v1#{}.done(token: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_done(&self, arg0);
                } else {
                    DefaultHandler.handle_done(&self, arg0);
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
            0 => "set_serial",
            1 => "set_app_id",
            2 => "set_surface",
            3 => "commit",
            4 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "done",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for XdgActivationTokenV1 {
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

impl XdgActivationTokenV1 {
    /// Since when the error.already_used enum variant is available.
    pub const ENM__ERROR_ALREADY_USED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct XdgActivationTokenV1Error(pub u32);

impl XdgActivationTokenV1Error {
    /// The token has already been used previously
    pub const ALREADY_USED: Self = Self(0);
}

impl Debug for XdgActivationTokenV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ALREADY_USED => "ALREADY_USED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
