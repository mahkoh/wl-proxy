//! input method
//!
//! An input method object is responsible for composing text in response to
//! input from hardware or virtual keyboards. There is one input method
//! object per seat. On activate there is a new input method context object
//! created which allows the input method to communicate with the text input.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_input_method_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpInputMethodV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpInputMethodV1Handler>,
}

struct DefaultHandler;

impl ZwpInputMethodV1Handler for DefaultHandler { }

impl ConcreteObject for ZwpInputMethodV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpInputMethodV1;
    const INTERFACE_NAME: &str = "zwp_input_method_v1";
}

impl ZwpInputMethodV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpInputMethodV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpInputMethodV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpInputMethodV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpInputMethodV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpInputMethodV1 {
    /// Since when the activate message is available.
    pub const MSG__ACTIVATE__SINCE: u32 = 1;

    /// activate event
    ///
    /// A text input was activated. Creates an input method context object
    /// which allows communication with the text input.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_activate(
        &self,
        id: &Rc<ZwpInputMethodContextV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateClientId("id", e)))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_v1#{}.activate(id: zwp_input_method_context_v1#{})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// activate event
    ///
    /// A text input was activated. Creates an input method context object
    /// which allows communication with the text input.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_activate(
        &self,
        id: &Rc<ZwpInputMethodContextV1>,
    ) {
        let res = self.try_send_activate(
            id,
        );
        if let Err(e) = res {
            log_send("zwp_input_method_v1.activate", &e);
        }
    }

    /// activate event
    ///
    /// A text input was activated. Creates an input method context object
    /// which allows communication with the text input.
    #[inline]
    pub fn new_try_send_activate(
        &self,
    ) -> Result<Rc<ZwpInputMethodContextV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_activate(
            &id,
        )?;
        Ok(id)
    }

    /// activate event
    ///
    /// A text input was activated. Creates an input method context object
    /// which allows communication with the text input.
    #[inline]
    pub fn new_send_activate(
        &self,
    ) -> Rc<ZwpInputMethodContextV1> {
        let id = self.core.create_child();
        self.send_activate(
            &id,
        );
        id
    }

    /// Since when the deactivate message is available.
    pub const MSG__DEACTIVATE__SINCE: u32 = 1;

    /// deactivate event
    ///
    /// The text input corresponding to the context argument was deactivated.
    /// The input method context should be destroyed after deactivation is
    /// handled.
    ///
    /// # Arguments
    ///
    /// - `context`:
    #[inline]
    pub fn try_send_deactivate(
        &self,
        context: &Rc<ZwpInputMethodContextV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            context,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("context", client.endpoint.id)));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_v1#{}.deactivate(context: zwp_input_method_context_v1#{})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0_id);
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
            1,
            arg0_id,
        ]);
        Ok(())
    }

    /// deactivate event
    ///
    /// The text input corresponding to the context argument was deactivated.
    /// The input method context should be destroyed after deactivation is
    /// handled.
    ///
    /// # Arguments
    ///
    /// - `context`:
    #[inline]
    pub fn send_deactivate(
        &self,
        context: &Rc<ZwpInputMethodContextV1>,
    ) {
        let res = self.try_send_deactivate(
            context,
        );
        if let Err(e) = res {
            log_send("zwp_input_method_v1.deactivate", &e);
        }
    }
}

/// A message handler for [`ZwpInputMethodV1`] proxies.
pub trait ZwpInputMethodV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpInputMethodV1>) {
        slf.core.delete_id();
    }

    /// activate event
    ///
    /// A text input was activated. Creates an input method context object
    /// which allows communication with the text input.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn handle_activate(
        &mut self,
        slf: &Rc<ZwpInputMethodV1>,
        id: &Rc<ZwpInputMethodContextV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_activate(
            id,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_v1.activate", &e);
        }
    }

    /// deactivate event
    ///
    /// The text input corresponding to the context argument was deactivated.
    /// The input method context should be destroyed after deactivation is
    /// handled.
    ///
    /// # Arguments
    ///
    /// - `context`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_deactivate(
        &mut self,
        slf: &Rc<ZwpInputMethodV1>,
        context: &Rc<ZwpInputMethodContextV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = context.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = slf.try_send_deactivate(
            context,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_v1.deactivate", &e);
        }
    }
}

impl ObjectPrivate for ZwpInputMethodV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpInputMethodV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_v1#{}.activate(id: zwp_input_method_context_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = ZwpInputMethodContextV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetServerId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_activate(&self, arg0);
                } else {
                    DefaultHandler.handle_activate(&self, arg0);
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
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_v1#{}.deactivate(context: zwp_input_method_context_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = server.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ZwpInputMethodContextV1>() else {
                    let o = server.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("context", o.core().interface, ObjectInterface::ZwpInputMethodContextV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_deactivate(&self, arg0);
                } else {
                    DefaultHandler.handle_deactivate(&self, arg0);
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
            0 => "activate",
            1 => "deactivate",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwpInputMethodV1 {
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

