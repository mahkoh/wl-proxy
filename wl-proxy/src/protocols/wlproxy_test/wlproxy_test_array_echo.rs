use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wlproxy_test_array_echo object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlproxyTestArrayEcho {
    core: ObjectCore,
    handler: HandlerHolder<dyn WlproxyTestArrayEchoHandler>,
}

struct DefaultHandler;

impl WlproxyTestArrayEchoHandler for DefaultHandler { }

impl ConcreteObject for WlproxyTestArrayEcho {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WlproxyTestArrayEcho;
    const INTERFACE_NAME: &str = "wlproxy_test_array_echo";
}

impl WlproxyTestArrayEcho {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WlproxyTestArrayEchoHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WlproxyTestArrayEchoHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlproxyTestArrayEcho {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlproxyTestArrayEcho")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlproxyTestArrayEcho {
    /// Since when the array message is available.
    pub const MSG__ARRAY__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `array`:
    #[inline]
    pub fn try_send_array(
        &self,
        array: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            array,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: &[u8]) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wlproxy_test_array_echo#{}.array(array: {})\n", client_id, id, debug_array(arg0));
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
        fmt.array(arg0);
        drop(fmt);
        drop(outgoing_ref);
        drop(client_ref);
        self.core.handle_client_destroy();
        Ok(())
    }

    /// # Arguments
    ///
    /// - `array`:
    #[inline]
    pub fn send_array(
        &self,
        array: &[u8],
    ) {
        let res = self.try_send_array(
            array,
        );
        if let Err(e) = res {
            log_send("wlproxy_test_array_echo.array", &e);
        }
    }
}

/// A message handler for [`WlproxyTestArrayEcho`] proxies.
pub trait WlproxyTestArrayEchoHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WlproxyTestArrayEcho>) {
        slf.core.delete_id();
    }

    /// # Arguments
    ///
    /// - `array`:
    #[inline]
    fn handle_array(
        &mut self,
        slf: &Rc<WlproxyTestArrayEcho>,
        array: &[u8],
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_array(
            array,
        );
        if let Err(e) = res {
            log_forward("wlproxy_test_array_echo.array", &e);
        }
    }
}

impl ObjectPrivate for WlproxyTestArrayEcho {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WlproxyTestArrayEcho, version),
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
            n => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
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
                (arg0, offset) = parse_array(msg, offset, "array")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wlproxy_test_array_echo#{}.array(array: {})\n", id, debug_array(arg0));
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                self.core.handle_server_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_array(&self, arg0);
                } else {
                    DefaultHandler.handle_array(&self, arg0);
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
        let _ = id;
        None
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "array",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WlproxyTestArrayEcho {
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

