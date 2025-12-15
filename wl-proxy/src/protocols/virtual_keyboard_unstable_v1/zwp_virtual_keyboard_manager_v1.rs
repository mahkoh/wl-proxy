//! virtual keyboard manager
//!
//! A virtual keyboard manager allows an application to provide keyboard
//! input events as if they came from a physical keyboard.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_virtual_keyboard_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpVirtualKeyboardManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpVirtualKeyboardManagerV1Handler>,
}

struct DefaultHandler;

impl ZwpVirtualKeyboardManagerV1Handler for DefaultHandler { }

impl ConcreteObject for ZwpVirtualKeyboardManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpVirtualKeyboardManagerV1;
    const INTERFACE_NAME: &str = "zwp_virtual_keyboard_manager_v1";
}

impl ZwpVirtualKeyboardManagerV1 {
    pub fn set_handler(&self, handler: impl ZwpVirtualKeyboardManagerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpVirtualKeyboardManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpVirtualKeyboardManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpVirtualKeyboardManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpVirtualKeyboardManagerV1 {
    /// Since when the create_virtual_keyboard message is available.
    pub const MSG__CREATE_VIRTUAL_KEYBOARD__SINCE: u32 = 1;

    /// Create a new virtual keyboard
    ///
    /// Creates a new virtual keyboard associated to a seat.
    ///
    /// If the compositor enables a keyboard to perform arbitrary actions, it
    /// should present an error when an untrusted client requests a new
    /// keyboard.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `id`:
    #[inline]
    pub fn try_send_create_virtual_keyboard(
        &self,
        seat: &Rc<WlSeat>,
        id: &Rc<ZwpVirtualKeyboardV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            seat,
            id,
        );
        let arg0 = arg0.core();
        let arg1_obj = arg1;
        let arg1 = arg1_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("seat")),
            Some(id) => id,
        };
        arg1.generate_server_id(arg1_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg1_id = arg1.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_virtual_keyboard_manager_v1#{}.create_virtual_keyboard(seat: wl_seat#{}, id: zwp_virtual_keyboard_v1#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id);
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
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// Create a new virtual keyboard
    ///
    /// Creates a new virtual keyboard associated to a seat.
    ///
    /// If the compositor enables a keyboard to perform arbitrary actions, it
    /// should present an error when an untrusted client requests a new
    /// keyboard.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `id`:
    #[inline]
    pub fn send_create_virtual_keyboard(
        &self,
        seat: &Rc<WlSeat>,
        id: &Rc<ZwpVirtualKeyboardV1>,
    ) {
        let res = self.try_send_create_virtual_keyboard(
            seat,
            id,
        );
        if let Err(e) = res {
            log_send("zwp_virtual_keyboard_manager_v1.create_virtual_keyboard", &e);
        }
    }
}

/// A message handler for [ZwpVirtualKeyboardManagerV1] proxies.
pub trait ZwpVirtualKeyboardManagerV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpVirtualKeyboardManagerV1>) {
        slf.core.delete_id();
    }

    /// Create a new virtual keyboard
    ///
    /// Creates a new virtual keyboard associated to a seat.
    ///
    /// If the compositor enables a keyboard to perform arbitrary actions, it
    /// should present an error when an untrusted client requests a new
    /// keyboard.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `id`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_create_virtual_keyboard(
        &mut self,
        _slf: &Rc<ZwpVirtualKeyboardManagerV1>,
        seat: &Rc<WlSeat>,
        id: &Rc<ZwpVirtualKeyboardV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_create_virtual_keyboard(
            seat,
            id,
        );
        if let Err(e) = res {
            log_forward("zwp_virtual_keyboard_manager_v1.create_virtual_keyboard", &e);
        }
    }
}

impl ObjectPrivate for ZwpVirtualKeyboardManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpVirtualKeyboardManagerV1, version),
            handler: Default::default(),
        })
    }

    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err((ObjectError::HandlerBorrowed, self));
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
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_virtual_keyboard_manager_v1#{}.create_virtual_keyboard(seat: wl_seat#{}, id: zwp_virtual_keyboard_v1#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("seat", o.core().interface, ObjectInterface::WlSeat));
                };
                let arg1_id = arg1;
                let arg1 = ZwpVirtualKeyboardV1::new(&self.core.state, self.core.version);
                arg1.core().set_client_id(client, arg1_id, arg1.clone())
                    .map_err(|e| ObjectError::SetClientId(arg1_id, "id", e))?;
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_create_virtual_keyboard(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_create_virtual_keyboard(&self, arg0, arg1);
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
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
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
            0 => "create_virtual_keyboard",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for ZwpVirtualKeyboardManagerV1 {
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

impl ZwpVirtualKeyboardManagerV1 {
    /// Since when the error.unauthorized enum variant is available.
    pub const ENM__ERROR_UNAUTHORIZED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwpVirtualKeyboardManagerV1Error(pub u32);

impl ZwpVirtualKeyboardManagerV1Error {
    /// client not authorized to use the interface
    pub const UNAUTHORIZED: Self = Self(0);
}

impl Debug for ZwpVirtualKeyboardManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::UNAUTHORIZED => "UNAUTHORIZED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
