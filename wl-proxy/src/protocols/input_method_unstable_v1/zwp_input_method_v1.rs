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

impl ZwpInputMethodV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::ZwpInputMethodV1;
    pub const INTERFACE_NAME: &str = "zwp_input_method_v1";
}

impl ZwpInputMethodV1 {
    pub fn set_handler(&self, handler: impl ZwpInputMethodV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

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
    #[inline]
    pub fn send_activate(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateClientId("id", e))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_v1#{}.activate(id: zwp_input_method_context_v1#{})\n", client.endpoint.id, id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
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
    pub fn send_deactivate(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError::ArgNoClientId("context", client.endpoint.id));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_v1#{}.deactivate(context: zwp_input_method_context_v1#{})\n", client.endpoint.id, id, arg0_id);
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
            1,
            arg0_id,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpInputMethodV1] proxies.
pub trait ZwpInputMethodV1Handler: Any {
    /// activate event
    ///
    /// A text input was activated. Creates an input method context object
    /// which allows communication with the text input.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn activate(
        &mut self,
        _slf: &Rc<ZwpInputMethodV1>,
        id: &Rc<ZwpInputMethodContextV1>,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        let res = _slf.send_activate(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_v1.activate message: {}", Report::new(e));
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
    fn deactivate(
        &mut self,
        _slf: &Rc<ZwpInputMethodV1>,
        context: &Rc<ZwpInputMethodContextV1>,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        if context.core().zombie.get() {
            return;
        }
        if let Some(client_id) = _slf.core.client_id.get() {
            if let Some(client_id_2) = context.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = _slf.send_deactivate(
            context,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_v1.deactivate message: {}", Report::new(e));
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

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            n => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
    }

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_v1#{}.activate(id: zwp_input_method_context_v1#{})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ZwpInputMethodContextV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetServerId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).activate(&self, arg0);
                } else {
                    DefaultHandler.activate(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_v1#{}.deactivate(context: zwp_input_method_context_v1#{})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = self.core.state.server.lookup(arg0_id) else {
                    return Err(ObjectError::NoServerObject(arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ZwpInputMethodContextV1>() else {
                    let o = self.core.state.server.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("context", o.core().interface, ObjectInterface::ZwpInputMethodContextV1));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).deactivate(&self, arg0);
                } else {
                    DefaultHandler.deactivate(&self, arg0);
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

