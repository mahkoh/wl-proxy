//! reposition the pointer to a location on a surface
//!
//! This global interface allows applications to request the pointer to be
//! moved to a position relative to a wl_surface.
//!
//! Note that if the desired behavior is to constrain the pointer to an area
//! or lock it to a position, this protocol does not provide a reliable way
//! to do that. The pointer constraint and pointer lock protocols should be
//! used for those use cases instead.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_pointer_warp_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpPointerWarpV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpPointerWarpV1Handler>,
}

struct DefaultHandler;

impl WpPointerWarpV1Handler for DefaultHandler { }

impl ConcreteObject for WpPointerWarpV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WpPointerWarpV1;
    const INTERFACE_NAME: &str = "wp_pointer_warp_v1";
}

impl WpPointerWarpV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WpPointerWarpV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WpPointerWarpV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpPointerWarpV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpPointerWarpV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpPointerWarpV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the warp manager
    ///
    /// Destroy the pointer warp manager.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_pointer_warp_v1#{}.destroy()\n", id);
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

    /// destroy the warp manager
    ///
    /// Destroy the pointer warp manager.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wp_pointer_warp_v1.destroy", &e);
        }
    }

    /// Since when the warp_pointer message is available.
    pub const MSG__WARP_POINTER__SINCE: u32 = 1;

    /// reposition the pointer
    ///
    /// Request the compositor to move the pointer to a surface-local position.
    /// Whether or not the compositor honors the request is implementation defined,
    /// but it should
    /// - honor it if the surface has pointer focus, including
    ///   when it has an implicit pointer grab
    /// - reject it if the enter serial is incorrect
    /// - reject it if the requested position is outside of the surface
    ///
    /// Note that the enter serial is valid for any surface of the client,
    /// and does not have to be from the surface the pointer is warped to.
    ///
    /// # Arguments
    ///
    /// - `surface`: surface to position the pointer on
    /// - `pointer`: the pointer that should be repositioned
    /// - `x`:
    /// - `y`:
    /// - `serial`: serial number of the enter event
    #[inline]
    pub fn try_send_warp_pointer(
        &self,
        surface: &Rc<WlSurface>,
        pointer: &Rc<WlPointer>,
        x: Fixed,
        y: Fixed,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ) = (
            surface,
            pointer,
            x,
            y,
            serial,
        );
        let arg0 = arg0.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("pointer"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: Fixed, arg3: Fixed, arg4: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_pointer_warp_v1#{}.warp_pointer(surface: wl_surface#{}, pointer: wl_pointer#{}, x: {}, y: {}, serial: {})\n", id, arg0, arg1, arg2, arg3, arg4);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id, arg2, arg3, arg4);
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
            arg2.to_wire() as u32,
            arg3.to_wire() as u32,
            arg4,
        ]);
        Ok(())
    }

    /// reposition the pointer
    ///
    /// Request the compositor to move the pointer to a surface-local position.
    /// Whether or not the compositor honors the request is implementation defined,
    /// but it should
    /// - honor it if the surface has pointer focus, including
    ///   when it has an implicit pointer grab
    /// - reject it if the enter serial is incorrect
    /// - reject it if the requested position is outside of the surface
    ///
    /// Note that the enter serial is valid for any surface of the client,
    /// and does not have to be from the surface the pointer is warped to.
    ///
    /// # Arguments
    ///
    /// - `surface`: surface to position the pointer on
    /// - `pointer`: the pointer that should be repositioned
    /// - `x`:
    /// - `y`:
    /// - `serial`: serial number of the enter event
    #[inline]
    pub fn send_warp_pointer(
        &self,
        surface: &Rc<WlSurface>,
        pointer: &Rc<WlPointer>,
        x: Fixed,
        y: Fixed,
        serial: u32,
    ) {
        let res = self.try_send_warp_pointer(
            surface,
            pointer,
            x,
            y,
            serial,
        );
        if let Err(e) = res {
            log_send("wp_pointer_warp_v1.warp_pointer", &e);
        }
    }
}

/// A message handler for [`WpPointerWarpV1`] proxies.
pub trait WpPointerWarpV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpPointerWarpV1>) {
        slf.core.delete_id();
    }

    /// destroy the warp manager
    ///
    /// Destroy the pointer warp manager.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WpPointerWarpV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wp_pointer_warp_v1.destroy", &e);
        }
    }

    /// reposition the pointer
    ///
    /// Request the compositor to move the pointer to a surface-local position.
    /// Whether or not the compositor honors the request is implementation defined,
    /// but it should
    /// - honor it if the surface has pointer focus, including
    ///   when it has an implicit pointer grab
    /// - reject it if the enter serial is incorrect
    /// - reject it if the requested position is outside of the surface
    ///
    /// Note that the enter serial is valid for any surface of the client,
    /// and does not have to be from the surface the pointer is warped to.
    ///
    /// # Arguments
    ///
    /// - `surface`: surface to position the pointer on
    /// - `pointer`: the pointer that should be repositioned
    /// - `x`:
    /// - `y`:
    /// - `serial`: serial number of the enter event
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_warp_pointer(
        &mut self,
        slf: &Rc<WpPointerWarpV1>,
        surface: &Rc<WlSurface>,
        pointer: &Rc<WlPointer>,
        x: Fixed,
        y: Fixed,
        serial: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_warp_pointer(
            surface,
            pointer,
            x,
            y,
            serial,
        );
        if let Err(e) = res {
            log_forward("wp_pointer_warp_v1.warp_pointer", &e);
        }
    }
}

impl ObjectPrivate for WpPointerWarpV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpPointerWarpV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_pointer_warp_v1#{}.destroy()\n", client_id, id);
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
                    arg4,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 28)));
                };
                let arg2 = Fixed::from_wire(arg2 as i32);
                let arg3 = Fixed::from_wire(arg3 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: Fixed, arg3: Fixed, arg4: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_pointer_warp_v1#{}.warp_pointer(surface: wl_surface#{}, pointer: wl_pointer#{}, x: {}, y: {}, serial: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlPointer>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("pointer", o.core().interface, ObjectInterface::WlPointer)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_warp_pointer(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultHandler.handle_warp_pointer(&self, arg0, arg1, arg2, arg3, arg4);
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
            1 => "warp_pointer",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WpPointerWarpV1 {
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

