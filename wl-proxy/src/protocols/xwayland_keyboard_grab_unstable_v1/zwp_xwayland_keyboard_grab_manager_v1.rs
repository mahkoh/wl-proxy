//! context object for keyboard grab manager
//!
//! A global interface used for grabbing the keyboard.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_xwayland_keyboard_grab_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpXwaylandKeyboardGrabManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpXwaylandKeyboardGrabManagerV1Handler>,
}

struct DefaultHandler;

impl ZwpXwaylandKeyboardGrabManagerV1Handler for DefaultHandler { }

impl ConcreteObject for ZwpXwaylandKeyboardGrabManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpXwaylandKeyboardGrabManagerV1;
    const INTERFACE_NAME: &str = "zwp_xwayland_keyboard_grab_manager_v1";
}

impl ZwpXwaylandKeyboardGrabManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpXwaylandKeyboardGrabManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpXwaylandKeyboardGrabManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpXwaylandKeyboardGrabManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpXwaylandKeyboardGrabManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpXwaylandKeyboardGrabManagerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the keyboard grab manager
    ///
    /// Destroy the keyboard grab manager.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_xwayland_keyboard_grab_manager_v1#{}.destroy()\n", id);
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

    /// destroy the keyboard grab manager
    ///
    /// Destroy the keyboard grab manager.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwp_xwayland_keyboard_grab_manager_v1.destroy", &e);
        }
    }

    /// Since when the grab_keyboard message is available.
    pub const MSG__GRAB_KEYBOARD__SINCE: u32 = 1;

    /// grab the keyboard to a surface
    ///
    /// The grab_keyboard request asks for a grab of the keyboard, forcing
    /// the keyboard focus for the given seat upon the given surface.
    ///
    /// The protocol provides no guarantee that the grab is ever satisfied,
    /// and does not require the compositor to send an error if the grab
    /// cannot ever be satisfied. It is thus possible to request a keyboard
    /// grab that will never be effective.
    ///
    /// The protocol:
    ///
    /// * does not guarantee that the grab itself is applied for a surface,
    ///   the grab request may be silently ignored by the compositor,
    /// * does not guarantee that any events are sent to this client even
    ///   if the grab is applied to a surface,
    /// * does not guarantee that events sent to this client are exhaustive,
    ///   a compositor may filter some events for its own consumption,
    /// * does not guarantee that events sent to this client are continuous,
    ///   a compositor may change and reroute keyboard events while the grab
    ///   is nominally active.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`: surface to report keyboard events to
    /// - `seat`: the seat for which the keyboard should be grabbed
    #[inline]
    pub fn try_send_grab_keyboard(
        &self,
        id: &Rc<ZwpXwaylandKeyboardGrabV1>,
        surface: &Rc<WlSurface>,
        seat: &Rc<WlSeat>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            id,
            surface,
            seat,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let arg2 = arg2.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        let arg2_id = match arg2.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("seat"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_xwayland_keyboard_grab_manager_v1#{}.grab_keyboard(id: zwp_xwayland_keyboard_grab_v1#{}, surface: wl_surface#{}, seat: wl_seat#{})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id, arg2_id);
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
            arg2_id,
        ]);
        Ok(())
    }

    /// grab the keyboard to a surface
    ///
    /// The grab_keyboard request asks for a grab of the keyboard, forcing
    /// the keyboard focus for the given seat upon the given surface.
    ///
    /// The protocol provides no guarantee that the grab is ever satisfied,
    /// and does not require the compositor to send an error if the grab
    /// cannot ever be satisfied. It is thus possible to request a keyboard
    /// grab that will never be effective.
    ///
    /// The protocol:
    ///
    /// * does not guarantee that the grab itself is applied for a surface,
    ///   the grab request may be silently ignored by the compositor,
    /// * does not guarantee that any events are sent to this client even
    ///   if the grab is applied to a surface,
    /// * does not guarantee that events sent to this client are exhaustive,
    ///   a compositor may filter some events for its own consumption,
    /// * does not guarantee that events sent to this client are continuous,
    ///   a compositor may change and reroute keyboard events while the grab
    ///   is nominally active.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`: surface to report keyboard events to
    /// - `seat`: the seat for which the keyboard should be grabbed
    #[inline]
    pub fn send_grab_keyboard(
        &self,
        id: &Rc<ZwpXwaylandKeyboardGrabV1>,
        surface: &Rc<WlSurface>,
        seat: &Rc<WlSeat>,
    ) {
        let res = self.try_send_grab_keyboard(
            id,
            surface,
            seat,
        );
        if let Err(e) = res {
            log_send("zwp_xwayland_keyboard_grab_manager_v1.grab_keyboard", &e);
        }
    }

    /// grab the keyboard to a surface
    ///
    /// The grab_keyboard request asks for a grab of the keyboard, forcing
    /// the keyboard focus for the given seat upon the given surface.
    ///
    /// The protocol provides no guarantee that the grab is ever satisfied,
    /// and does not require the compositor to send an error if the grab
    /// cannot ever be satisfied. It is thus possible to request a keyboard
    /// grab that will never be effective.
    ///
    /// The protocol:
    ///
    /// * does not guarantee that the grab itself is applied for a surface,
    ///   the grab request may be silently ignored by the compositor,
    /// * does not guarantee that any events are sent to this client even
    ///   if the grab is applied to a surface,
    /// * does not guarantee that events sent to this client are exhaustive,
    ///   a compositor may filter some events for its own consumption,
    /// * does not guarantee that events sent to this client are continuous,
    ///   a compositor may change and reroute keyboard events while the grab
    ///   is nominally active.
    ///
    /// # Arguments
    ///
    /// - `surface`: surface to report keyboard events to
    /// - `seat`: the seat for which the keyboard should be grabbed
    #[inline]
    pub fn new_try_send_grab_keyboard(
        &self,
        surface: &Rc<WlSurface>,
        seat: &Rc<WlSeat>,
    ) -> Result<Rc<ZwpXwaylandKeyboardGrabV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_grab_keyboard(
            &id,
            surface,
            seat,
        )?;
        Ok(id)
    }

    /// grab the keyboard to a surface
    ///
    /// The grab_keyboard request asks for a grab of the keyboard, forcing
    /// the keyboard focus for the given seat upon the given surface.
    ///
    /// The protocol provides no guarantee that the grab is ever satisfied,
    /// and does not require the compositor to send an error if the grab
    /// cannot ever be satisfied. It is thus possible to request a keyboard
    /// grab that will never be effective.
    ///
    /// The protocol:
    ///
    /// * does not guarantee that the grab itself is applied for a surface,
    ///   the grab request may be silently ignored by the compositor,
    /// * does not guarantee that any events are sent to this client even
    ///   if the grab is applied to a surface,
    /// * does not guarantee that events sent to this client are exhaustive,
    ///   a compositor may filter some events for its own consumption,
    /// * does not guarantee that events sent to this client are continuous,
    ///   a compositor may change and reroute keyboard events while the grab
    ///   is nominally active.
    ///
    /// # Arguments
    ///
    /// - `surface`: surface to report keyboard events to
    /// - `seat`: the seat for which the keyboard should be grabbed
    #[inline]
    pub fn new_send_grab_keyboard(
        &self,
        surface: &Rc<WlSurface>,
        seat: &Rc<WlSeat>,
    ) -> Rc<ZwpXwaylandKeyboardGrabV1> {
        let id = self.core.create_child();
        self.send_grab_keyboard(
            &id,
            surface,
            seat,
        );
        id
    }
}

/// A message handler for [`ZwpXwaylandKeyboardGrabManagerV1`] proxies.
pub trait ZwpXwaylandKeyboardGrabManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpXwaylandKeyboardGrabManagerV1>) {
        slf.core.delete_id();
    }

    /// destroy the keyboard grab manager
    ///
    /// Destroy the keyboard grab manager.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwpXwaylandKeyboardGrabManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwp_xwayland_keyboard_grab_manager_v1.destroy", &e);
        }
    }

    /// grab the keyboard to a surface
    ///
    /// The grab_keyboard request asks for a grab of the keyboard, forcing
    /// the keyboard focus for the given seat upon the given surface.
    ///
    /// The protocol provides no guarantee that the grab is ever satisfied,
    /// and does not require the compositor to send an error if the grab
    /// cannot ever be satisfied. It is thus possible to request a keyboard
    /// grab that will never be effective.
    ///
    /// The protocol:
    ///
    /// * does not guarantee that the grab itself is applied for a surface,
    ///   the grab request may be silently ignored by the compositor,
    /// * does not guarantee that any events are sent to this client even
    ///   if the grab is applied to a surface,
    /// * does not guarantee that events sent to this client are exhaustive,
    ///   a compositor may filter some events for its own consumption,
    /// * does not guarantee that events sent to this client are continuous,
    ///   a compositor may change and reroute keyboard events while the grab
    ///   is nominally active.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`: surface to report keyboard events to
    /// - `seat`: the seat for which the keyboard should be grabbed
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_grab_keyboard(
        &mut self,
        slf: &Rc<ZwpXwaylandKeyboardGrabManagerV1>,
        id: &Rc<ZwpXwaylandKeyboardGrabV1>,
        surface: &Rc<WlSurface>,
        seat: &Rc<WlSeat>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_grab_keyboard(
            id,
            surface,
            seat,
        );
        if let Err(e) = res {
            log_forward("zwp_xwayland_keyboard_grab_manager_v1.grab_keyboard", &e);
        }
    }
}

impl ObjectPrivate for ZwpXwaylandKeyboardGrabManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpXwaylandKeyboardGrabManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_xwayland_keyboard_grab_manager_v1#{}.destroy()\n", client_id, id);
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_xwayland_keyboard_grab_manager_v1#{}.grab_keyboard(id: zwp_xwayland_keyboard_grab_v1#{}, surface: wl_surface#{}, seat: wl_seat#{})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let arg0 = ZwpXwaylandKeyboardGrabV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg2_id = arg2;
                let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg2_id)));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg2_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("seat", o.core().interface, ObjectInterface::WlSeat)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).handle_grab_keyboard(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_grab_keyboard(&self, arg0, arg1, arg2);
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
            1 => "grab_keyboard",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for ZwpXwaylandKeyboardGrabManagerV1 {
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

