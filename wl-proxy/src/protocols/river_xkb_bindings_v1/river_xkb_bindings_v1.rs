//! xkbcommon bindings global interface
//!
//! This global interface should only be advertised to the client if the
//! river_window_manager_v1 global is also advertised.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A river_xkb_bindings_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct RiverXkbBindingsV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn RiverXkbBindingsV1Handler>,
}

struct DefaultHandler;

impl RiverXkbBindingsV1Handler for DefaultHandler { }

impl ConcreteObject for RiverXkbBindingsV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::RiverXkbBindingsV1;
    const INTERFACE_NAME: &str = "river_xkb_bindings_v1";
}

impl RiverXkbBindingsV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl RiverXkbBindingsV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn RiverXkbBindingsV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for RiverXkbBindingsV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RiverXkbBindingsV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl RiverXkbBindingsV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the river_xkb_bindings_v1 object
    ///
    /// This request indicates that the client will no longer use the
    /// river_xkb_bindings_v1 object.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_xkb_bindings_v1#{}.destroy()\n", id);
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

    /// destroy the river_xkb_bindings_v1 object
    ///
    /// This request indicates that the client will no longer use the
    /// river_xkb_bindings_v1 object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("river_xkb_bindings_v1.destroy", &e);
        }
    }

    /// Since when the get_xkb_binding message is available.
    pub const MSG__GET_XKB_BINDING__SINCE: u32 = 1;

    /// define a new xkbcommon key binding
    ///
    /// Define a key binding for the given seat in terms of an xkbcommon keysym
    /// and other configurable properties.
    ///
    /// The new key binding is not enabled until initial configuration is
    /// completed and the enable request is made during a manage sequence.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `id`:
    /// - `keysym`: an xkbcommon keysym
    /// - `modifiers`:
    #[inline]
    pub fn try_send_get_xkb_binding(
        &self,
        seat: &Rc<RiverSeatV1>,
        id: &Rc<RiverXkbBindingV1>,
        keysym: u32,
        modifiers: RiverSeatV1Modifiers,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            seat,
            id,
            keysym,
            modifiers,
        );
        let arg0 = arg0.core();
        let arg1_obj = arg1;
        let arg1 = arg1_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("seat"))),
            Some(id) => id,
        };
        arg1.generate_server_id(arg1_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg1_id = arg1.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: RiverSeatV1Modifiers) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_xkb_bindings_v1#{}.get_xkb_binding(seat: river_seat_v1#{}, id: river_xkb_binding_v1#{}, keysym: {}, modifiers: {:?})\n", id, arg0, arg1, arg2, arg3);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id, arg2, arg3);
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
            arg1_id,
            arg2,
            arg3.0,
        ]);
        Ok(())
    }

    /// define a new xkbcommon key binding
    ///
    /// Define a key binding for the given seat in terms of an xkbcommon keysym
    /// and other configurable properties.
    ///
    /// The new key binding is not enabled until initial configuration is
    /// completed and the enable request is made during a manage sequence.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `id`:
    /// - `keysym`: an xkbcommon keysym
    /// - `modifiers`:
    #[inline]
    pub fn send_get_xkb_binding(
        &self,
        seat: &Rc<RiverSeatV1>,
        id: &Rc<RiverXkbBindingV1>,
        keysym: u32,
        modifiers: RiverSeatV1Modifiers,
    ) {
        let res = self.try_send_get_xkb_binding(
            seat,
            id,
            keysym,
            modifiers,
        );
        if let Err(e) = res {
            log_send("river_xkb_bindings_v1.get_xkb_binding", &e);
        }
    }

    /// define a new xkbcommon key binding
    ///
    /// Define a key binding for the given seat in terms of an xkbcommon keysym
    /// and other configurable properties.
    ///
    /// The new key binding is not enabled until initial configuration is
    /// completed and the enable request is made during a manage sequence.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `keysym`: an xkbcommon keysym
    /// - `modifiers`:
    #[inline]
    pub fn new_try_send_get_xkb_binding(
        &self,
        seat: &Rc<RiverSeatV1>,
        keysym: u32,
        modifiers: RiverSeatV1Modifiers,
    ) -> Result<Rc<RiverXkbBindingV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_xkb_binding(
            seat,
            &id,
            keysym,
            modifiers,
        )?;
        Ok(id)
    }

    /// define a new xkbcommon key binding
    ///
    /// Define a key binding for the given seat in terms of an xkbcommon keysym
    /// and other configurable properties.
    ///
    /// The new key binding is not enabled until initial configuration is
    /// completed and the enable request is made during a manage sequence.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `keysym`: an xkbcommon keysym
    /// - `modifiers`:
    #[inline]
    pub fn new_send_get_xkb_binding(
        &self,
        seat: &Rc<RiverSeatV1>,
        keysym: u32,
        modifiers: RiverSeatV1Modifiers,
    ) -> Rc<RiverXkbBindingV1> {
        let id = self.core.create_child();
        self.send_get_xkb_binding(
            seat,
            &id,
            keysym,
            modifiers,
        );
        id
    }
}

/// A message handler for [`RiverXkbBindingsV1`] proxies.
pub trait RiverXkbBindingsV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<RiverXkbBindingsV1>) {
        slf.core.delete_id();
    }

    /// destroy the river_xkb_bindings_v1 object
    ///
    /// This request indicates that the client will no longer use the
    /// river_xkb_bindings_v1 object.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<RiverXkbBindingsV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("river_xkb_bindings_v1.destroy", &e);
        }
    }

    /// define a new xkbcommon key binding
    ///
    /// Define a key binding for the given seat in terms of an xkbcommon keysym
    /// and other configurable properties.
    ///
    /// The new key binding is not enabled until initial configuration is
    /// completed and the enable request is made during a manage sequence.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `id`:
    /// - `keysym`: an xkbcommon keysym
    /// - `modifiers`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_xkb_binding(
        &mut self,
        slf: &Rc<RiverXkbBindingsV1>,
        seat: &Rc<RiverSeatV1>,
        id: &Rc<RiverXkbBindingV1>,
        keysym: u32,
        modifiers: RiverSeatV1Modifiers,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_xkb_binding(
            seat,
            id,
            keysym,
            modifiers,
        );
        if let Err(e) = res {
            log_forward("river_xkb_bindings_v1.get_xkb_binding", &e);
        }
    }
}

impl ObjectPrivate for RiverXkbBindingsV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::RiverXkbBindingsV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_xkb_bindings_v1#{}.destroy()\n", client_id, id);
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
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 24)));
                };
                let arg3 = RiverSeatV1Modifiers(arg3);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: RiverSeatV1Modifiers) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_xkb_bindings_v1#{}.get_xkb_binding(seat: river_seat_v1#{}, id: river_xkb_binding_v1#{}, keysym: {}, modifiers: {:?})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<RiverSeatV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("seat", o.core().interface, ObjectInterface::RiverSeatV1)));
                };
                let arg1_id = arg1;
                let arg1 = RiverXkbBindingV1::new(&self.core.state, self.core.version);
                arg1.core().set_client_id(client, arg1_id, arg1.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg1_id, "id", e)))?;
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_xkb_binding(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_get_xkb_binding(&self, arg0, arg1, arg2, arg3);
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
            1 => "get_xkb_binding",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for RiverXkbBindingsV1 {
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

