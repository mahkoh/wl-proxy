//!
//! Object to get and set `group` for a particular `wl_keyboard`.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zcosmic_keyboard_layout_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZcosmicKeyboardLayoutV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZcosmicKeyboardLayoutV1Handler>,
}

struct DefaultHandler;

impl ZcosmicKeyboardLayoutV1Handler for DefaultHandler { }

impl ConcreteObject for ZcosmicKeyboardLayoutV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZcosmicKeyboardLayoutV1;
    const INTERFACE_NAME: &str = "zcosmic_keyboard_layout_v1";
}

impl ZcosmicKeyboardLayoutV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZcosmicKeyboardLayoutV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZcosmicKeyboardLayoutV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZcosmicKeyboardLayoutV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZcosmicKeyboardLayoutV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZcosmicKeyboardLayoutV1 {
    /// Since when the group message is available.
    pub const MSG__GROUP__SINCE: u32 = 1;

    /// layout group changed
    ///
    /// Unlike `wl_keyboard::modifiers`, this event is received even when the client has no focused window.
    ///
    /// # Arguments
    ///
    /// - `group`:
    #[inline]
    pub fn try_send_group(
        &self,
        group: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            group,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zcosmic_keyboard_layout_v1#{}.group(group: {})\n", client_id, id, arg0);
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
            arg0,
        ]);
        Ok(())
    }

    /// layout group changed
    ///
    /// Unlike `wl_keyboard::modifiers`, this event is received even when the client has no focused window.
    ///
    /// # Arguments
    ///
    /// - `group`:
    #[inline]
    pub fn send_group(
        &self,
        group: u32,
    ) {
        let res = self.try_send_group(
            group,
        );
        if let Err(e) = res {
            log_send("zcosmic_keyboard_layout_v1.group", &e);
        }
    }

    /// Since when the set_group message is available.
    pub const MSG__SET_GROUP__SINCE: u32 = 1;

    /// set layout group
    ///
    /// On success, the `group` event will be generated for all clients, and the focused client will
    /// receive `wl_keyboard::modifiers` with the new group.
    ///
    /// The request will be ignored if `group` is out of bounds.
    ///
    /// # Arguments
    ///
    /// - `group`:
    #[inline]
    pub fn try_send_set_group(
        &self,
        group: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            group,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_keyboard_layout_v1#{}.set_group(group: {})\n", id, arg0);
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
            0,
            arg0,
        ]);
        Ok(())
    }

    /// set layout group
    ///
    /// On success, the `group` event will be generated for all clients, and the focused client will
    /// receive `wl_keyboard::modifiers` with the new group.
    ///
    /// The request will be ignored if `group` is out of bounds.
    ///
    /// # Arguments
    ///
    /// - `group`:
    #[inline]
    pub fn send_set_group(
        &self,
        group: u32,
    ) {
        let res = self.try_send_set_group(
            group,
        );
        if let Err(e) = res {
            log_send("zcosmic_keyboard_layout_v1.set_group", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// delete this object
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_keyboard_layout_v1#{}.destroy()\n", id);
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
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// delete this object
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zcosmic_keyboard_layout_v1.destroy", &e);
        }
    }
}

/// A message handler for [`ZcosmicKeyboardLayoutV1`] proxies.
pub trait ZcosmicKeyboardLayoutV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZcosmicKeyboardLayoutV1>) {
        slf.core.delete_id();
    }

    /// layout group changed
    ///
    /// Unlike `wl_keyboard::modifiers`, this event is received even when the client has no focused window.
    ///
    /// # Arguments
    ///
    /// - `group`:
    #[inline]
    fn handle_group(
        &mut self,
        slf: &Rc<ZcosmicKeyboardLayoutV1>,
        group: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_group(
            group,
        );
        if let Err(e) = res {
            log_forward("zcosmic_keyboard_layout_v1.group", &e);
        }
    }

    /// set layout group
    ///
    /// On success, the `group` event will be generated for all clients, and the focused client will
    /// receive `wl_keyboard::modifiers` with the new group.
    ///
    /// The request will be ignored if `group` is out of bounds.
    ///
    /// # Arguments
    ///
    /// - `group`:
    #[inline]
    fn handle_set_group(
        &mut self,
        slf: &Rc<ZcosmicKeyboardLayoutV1>,
        group: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_group(
            group,
        );
        if let Err(e) = res {
            log_forward("zcosmic_keyboard_layout_v1.set_group", &e);
        }
    }

    /// delete this object
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZcosmicKeyboardLayoutV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zcosmic_keyboard_layout_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for ZcosmicKeyboardLayoutV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZcosmicKeyboardLayoutV1, version),
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_keyboard_layout_v1#{}.set_group(group: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_group(&self, arg0);
                } else {
                    DefaultHandler.handle_set_group(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_keyboard_layout_v1#{}.destroy()\n", client_id, id);
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zcosmic_keyboard_layout_v1#{}.group(group: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_group(&self, arg0);
                } else {
                    DefaultHandler.handle_group(&self, arg0);
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
            0 => "set_group",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "group",
            _ => return None,
        };
        Some(name)
    }

    fn create_zombie(&self) -> Rc<dyn Object> {
        let slf = Self::new(&self.core.state, self.core.version);
        slf.core.make_zombie();
        slf
    }
}

impl Object for ZcosmicKeyboardLayoutV1 {
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

