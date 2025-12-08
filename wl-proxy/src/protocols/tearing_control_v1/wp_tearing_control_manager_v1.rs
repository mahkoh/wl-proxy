//! protocol for tearing control
//!
//! For some use cases like games or drawing tablets it can make sense to
//! reduce latency by accepting tearing with the use of asynchronous page
//! flips. This global is a factory interface, allowing clients to inform
//! which type of presentation the content of their surfaces is suitable for.
//!
//! Graphics APIs like EGL or Vulkan, that manage the buffer queue and commits
//! of a wl_surface themselves, are likely to be using this extension
//! internally. If a client is using such an API for a wl_surface, it should
//! not directly use this extension on that surface, to avoid raising a
//! tearing_control_exists protocol error.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_tearing_control_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpTearingControlManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpTearingControlManagerV1Handler>,
}

struct DefaultHandler;

impl WpTearingControlManagerV1Handler for DefaultHandler { }

impl ConcreteObject for WpTearingControlManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WpTearingControlManagerV1;
    const INTERFACE_NAME: &str = "wp_tearing_control_manager_v1";
}

impl WpTearingControlManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WpTearingControlManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WpTearingControlManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpTearingControlManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpTearingControlManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpTearingControlManagerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy tearing control factory object
    ///
    /// Destroy this tearing control factory object. Other objects, including
    /// wp_tearing_control_v1 objects created by this factory, are not affected
    /// by this request.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_tearing_control_manager_v1#{}.destroy()\n", id);
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

    /// destroy tearing control factory object
    ///
    /// Destroy this tearing control factory object. Other objects, including
    /// wp_tearing_control_v1 objects created by this factory, are not affected
    /// by this request.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wp_tearing_control_manager_v1.destroy", &e);
        }
    }

    /// Since when the get_tearing_control message is available.
    pub const MSG__GET_TEARING_CONTROL__SINCE: u32 = 1;

    /// extend surface interface for tearing control
    ///
    /// Instantiate an interface extension for the given wl_surface to request
    /// asynchronous page flips for presentation.
    ///
    /// If the given wl_surface already has a wp_tearing_control_v1 object
    /// associated, the tearing_control_exists protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn try_send_get_tearing_control(
        &self,
        id: &Rc<WpTearingControlV1>,
        surface: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            surface,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_tearing_control_manager_v1#{}.get_tearing_control(id: wp_tearing_control_v1#{}, surface: wl_surface#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id);
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
        ]);
        Ok(())
    }

    /// extend surface interface for tearing control
    ///
    /// Instantiate an interface extension for the given wl_surface to request
    /// asynchronous page flips for presentation.
    ///
    /// If the given wl_surface already has a wp_tearing_control_v1 object
    /// associated, the tearing_control_exists protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn send_get_tearing_control(
        &self,
        id: &Rc<WpTearingControlV1>,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_get_tearing_control(
            id,
            surface,
        );
        if let Err(e) = res {
            log_send("wp_tearing_control_manager_v1.get_tearing_control", &e);
        }
    }

    /// extend surface interface for tearing control
    ///
    /// Instantiate an interface extension for the given wl_surface to request
    /// asynchronous page flips for presentation.
    ///
    /// If the given wl_surface already has a wp_tearing_control_v1 object
    /// associated, the tearing_control_exists protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn new_try_send_get_tearing_control(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Result<Rc<WpTearingControlV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_tearing_control(
            &id,
            surface,
        )?;
        Ok(id)
    }

    /// extend surface interface for tearing control
    ///
    /// Instantiate an interface extension for the given wl_surface to request
    /// asynchronous page flips for presentation.
    ///
    /// If the given wl_surface already has a wp_tearing_control_v1 object
    /// associated, the tearing_control_exists protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn new_send_get_tearing_control(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Rc<WpTearingControlV1> {
        let id = self.core.create_child();
        self.send_get_tearing_control(
            &id,
            surface,
        );
        id
    }
}

/// A message handler for [`WpTearingControlManagerV1`] proxies.
pub trait WpTearingControlManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpTearingControlManagerV1>) {
        slf.core.delete_id();
    }

    /// destroy tearing control factory object
    ///
    /// Destroy this tearing control factory object. Other objects, including
    /// wp_tearing_control_v1 objects created by this factory, are not affected
    /// by this request.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WpTearingControlManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wp_tearing_control_manager_v1.destroy", &e);
        }
    }

    /// extend surface interface for tearing control
    ///
    /// Instantiate an interface extension for the given wl_surface to request
    /// asynchronous page flips for presentation.
    ///
    /// If the given wl_surface already has a wp_tearing_control_v1 object
    /// associated, the tearing_control_exists protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_tearing_control(
        &mut self,
        slf: &Rc<WpTearingControlManagerV1>,
        id: &Rc<WpTearingControlV1>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_tearing_control(
            id,
            surface,
        );
        if let Err(e) = res {
            log_forward("wp_tearing_control_manager_v1.get_tearing_control", &e);
        }
    }
}

impl ObjectPrivate for WpTearingControlManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpTearingControlManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_tearing_control_manager_v1#{}.destroy()\n", client_id, id);
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_tearing_control_manager_v1#{}.get_tearing_control(id: wp_tearing_control_v1#{}, surface: wl_surface#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = WpTearingControlV1::new(&self.core.state, self.core.version);
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
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_tearing_control(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_tearing_control(&self, arg0, arg1);
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
            1 => "get_tearing_control",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WpTearingControlManagerV1 {
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

impl WpTearingControlManagerV1 {
    /// Since when the error.tearing_control_exists enum variant is available.
    pub const ENM__ERROR_TEARING_CONTROL_EXISTS__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpTearingControlManagerV1Error(pub u32);

impl WpTearingControlManagerV1Error {
    /// the surface already has a tearing object associated
    pub const TEARING_CONTROL_EXISTS: Self = Self(0);
}

impl Debug for WpTearingControlManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::TEARING_CONTROL_EXISTS => "TEARING_CONTROL_EXISTS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
