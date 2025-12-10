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

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A xdg_activation_token_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XdgActivationTokenV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn XdgActivationTokenV1Handler>,
}

struct DefaultHandler;

impl XdgActivationTokenV1Handler for DefaultHandler { }

impl XdgActivationTokenV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "xdg_activation_token_v1";
}

impl XdgActivationTokenV1 {
    pub fn set_handler(&self, handler: impl XdgActivationTokenV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

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
    #[allow(dead_code)]
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
    pub fn send_set_serial(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("seat")),
            Some(id) => id,
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_activation_token_v1#{}.set_serial(serial: {}, seat: wl_seat#{})\n", id, arg0, arg1_id);
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
            arg0,
            arg1_id,
        ]);
        Ok(())
    }

    /// Since when the set_app_id message is available.
    #[allow(dead_code)]
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
    pub fn send_set_app_id(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_activation_token_v1#{}.set_app_id(app_id: {:?})\n", id, arg0);
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
            1,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the set_surface message is available.
    #[allow(dead_code)]
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
    pub fn send_set_surface(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("surface")),
            Some(id) => id,
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_activation_token_v1#{}.set_surface(surface: wl_surface#{})\n", id, arg0_id);
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
            2,
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the commit message is available.
    #[allow(dead_code)]
    pub const MSG__COMMIT__SINCE: u32 = 1;

    /// issues the token request
    ///
    /// Requests an activation token based on the different parameters that
    /// have been offered through set_serial, set_surface and set_app_id.
    #[inline]
    pub fn send_commit(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_activation_token_v1#{}.commit()\n", id);
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
            3,
        ]);
        Ok(())
    }

    /// Since when the done message is available.
    #[allow(dead_code)]
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
    pub fn send_done(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xdg_activation_token_v1#{}.done(token: {:?})\n", client.endpoint.id, id, arg0);
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

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the xdg_activation_token_v1 object
    ///
    /// Notify the compositor that the xdg_activation_token_v1 object will no
    /// longer be used. The received token stays valid.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_activation_token_v1#{}.destroy()\n", id);
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
            4,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [XdgActivationTokenV1] proxies.
#[allow(dead_code)]
pub trait XdgActivationTokenV1Handler: Any {
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
    fn set_serial(
        &mut self,
        _slf: &Rc<XdgActivationTokenV1>,
        serial: u32,
        seat: &Rc<WlSeat>,
    ) {
        let res = _slf.send_set_serial(
            serial,
            seat,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_activation_token_v1.set_serial message: {}", Report::new(e));
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
    fn set_app_id(
        &mut self,
        _slf: &Rc<XdgActivationTokenV1>,
        app_id: &str,
    ) {
        let res = _slf.send_set_app_id(
            app_id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_activation_token_v1.set_app_id message: {}", Report::new(e));
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
    fn set_surface(
        &mut self,
        _slf: &Rc<XdgActivationTokenV1>,
        surface: &Rc<WlSurface>,
    ) {
        let res = _slf.send_set_surface(
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_activation_token_v1.set_surface message: {}", Report::new(e));
        }
    }

    /// issues the token request
    ///
    /// Requests an activation token based on the different parameters that
    /// have been offered through set_serial, set_surface and set_app_id.
    #[inline]
    fn commit(
        &mut self,
        _slf: &Rc<XdgActivationTokenV1>,
    ) {
        let res = _slf.send_commit(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_activation_token_v1.commit message: {}", Report::new(e));
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
    fn done(
        &mut self,
        _slf: &Rc<XdgActivationTokenV1>,
        token: &str,
    ) {
        let res = _slf.send_done(
            token,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_activation_token_v1.done message: {}", Report::new(e));
        }
    }

    /// destroy the xdg_activation_token_v1 object
    ///
    /// Notify the compositor that the xdg_activation_token_v1 object will no
    /// longer be used. The received token stays valid.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<XdgActivationTokenV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_activation_token_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for XdgActivationTokenV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::XdgActivationTokenV1, version),
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
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_activation_token_v1#{}.set_serial(serial: {}, seat: wl_seat#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("seat", o.core().interface, ProxyInterface::WlSeat));
                };
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).set_serial(&self, arg0, arg1);
                } else {
                    DefaultHandler.set_serial(&self, arg0, arg1);
                }
            }
            1 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("app_id"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("app_id"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("app_id"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("app_id"));
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_activation_token_v1#{}.set_app_id(app_id: {:?})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_app_id(&self, arg0);
                } else {
                    DefaultHandler.set_app_id(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_activation_token_v1#{}.set_surface(surface: wl_surface#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ProxyInterface::WlSurface));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).set_surface(&self, arg0);
                } else {
                    DefaultHandler.set_surface(&self, arg0);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_activation_token_v1#{}.commit()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).commit(&self);
                } else {
                    DefaultHandler.commit(&self);
                }
            }
            4 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_activation_token_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
                        return Err(ObjectError::MissingArgument("token"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("token"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("token"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("token"));
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xdg_activation_token_v1#{}.done(token: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).done(&self, arg0);
                } else {
                    DefaultHandler.done(&self, arg0);
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

impl Proxy for XdgActivationTokenV1 {
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

impl XdgActivationTokenV1 {
    /// Since when the error.already_used enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ALREADY_USED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct XdgActivationTokenV1Error(pub u32);

impl XdgActivationTokenV1Error {
    /// The token has already been used previously
    #[allow(dead_code)]
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
