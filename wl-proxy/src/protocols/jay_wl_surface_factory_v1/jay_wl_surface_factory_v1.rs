//! wl_surface factory
//!
//! This interface can be used by the compositor to create wl_surface objects
//! and send them to the client.
//!
//! The created wl_surface objects will have the same version as this object,
//! which in turn has the same version as the
//! jay_wl_surface_factory_manager_v1 object. The client should bind
//! jay_wl_surface_factory_manager_v1 with the version it wants for the
//! wl_surface objects.
//!
//! Each jay_wl_surface_factory_v1 is in one of three states:
//!
//! - Initial: This is the initial state immediately after the object has been
//!   created.
//! - Assigned: The factory is assigned to a higher-level producer.
//! - Abandoned: The factory was previously in the assigned state but has been
//!   abandoned by the producer.
//!
//! The object can only transition from the initial to the assigned state and
//! from the assigned to the abandoned state. How and when these transitions
//! happen is defined by higher-level protocols building on top of this
//! protocol.
//!
//! For example, a higher-level protocol might have a constructor request
//! that accepts a jay_wl_surface_factory_v1 object as an argument and
//! transitions it from the initial to the assigned state. The created
//! higher-level object might have a stop request that transitions the
//! jay_wl_surface_factory_v1 object from the assigned to the abandoned state.
//!
//! The jay_wl_surface_factory_v1.surface event is only sent while the object
//! is in the assigned state.
//!
//! Trying to transition the jay_wl_surface_factory_v1 to the assigned state
//! while it is not in the initial state raises the already_assigned error.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A jay_wl_surface_factory_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct JayWlSurfaceFactoryV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn JayWlSurfaceFactoryV1Handler>,
}

struct DefaultHandler;

impl JayWlSurfaceFactoryV1Handler for DefaultHandler { }

impl ConcreteObject for JayWlSurfaceFactoryV1 {
    const XML_VERSION: u32 = 65535;
    const INTERFACE: ObjectInterface = ObjectInterface::JayWlSurfaceFactoryV1;
    const INTERFACE_NAME: &str = "jay_wl_surface_factory_v1";
}

impl JayWlSurfaceFactoryV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl JayWlSurfaceFactoryV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn JayWlSurfaceFactoryV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for JayWlSurfaceFactoryV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JayWlSurfaceFactoryV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl JayWlSurfaceFactoryV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy this factory
    ///
    /// This request must only be sent while the jay_wl_surface_factory_v1 is in
    /// the initial or abandoned state. Trying to send it in the assigned state
    /// raises the still_assigned error.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= jay_wl_surface_factory_v1#{}.destroy()\n", id);
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

    /// destroy this factory
    ///
    /// This request must only be sent while the jay_wl_surface_factory_v1 is in
    /// the initial or abandoned state. Trying to send it in the assigned state
    /// raises the still_assigned error.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("jay_wl_surface_factory_v1.destroy", &e);
        }
    }

    /// Since when the surface message is available.
    pub const MSG__SURFACE__SINCE: u32 = 1;

    /// a new wl_surface has been created
    ///
    /// This event is sent when the compositor creates a new wl_surface object.
    /// The meaning and use of the surface are defined by the higher-level
    /// protocol that this jay_wl_surface_factory_v1 is assigned to.
    ///
    /// The version of the wl_surface is the version of this
    /// jay_wl_surface_factory_v1 object, which is in turn the version that the
    /// client bound the jay_wl_surface_factory_manager_v1 with.
    ///
    /// # Arguments
    ///
    /// - `id`: the new surface
    #[inline]
    pub fn try_send_surface(
        &self,
        id: &Rc<WlSurface>,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= jay_wl_surface_factory_v1#{}.surface(id: wl_surface#{})\n", client_id, id, arg0);
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

    /// a new wl_surface has been created
    ///
    /// This event is sent when the compositor creates a new wl_surface object.
    /// The meaning and use of the surface are defined by the higher-level
    /// protocol that this jay_wl_surface_factory_v1 is assigned to.
    ///
    /// The version of the wl_surface is the version of this
    /// jay_wl_surface_factory_v1 object, which is in turn the version that the
    /// client bound the jay_wl_surface_factory_manager_v1 with.
    ///
    /// # Arguments
    ///
    /// - `id`: the new surface
    #[inline]
    pub fn send_surface(
        &self,
        id: &Rc<WlSurface>,
    ) {
        let res = self.try_send_surface(
            id,
        );
        if let Err(e) = res {
            log_send("jay_wl_surface_factory_v1.surface", &e);
        }
    }

    /// a new wl_surface has been created
    ///
    /// This event is sent when the compositor creates a new wl_surface object.
    /// The meaning and use of the surface are defined by the higher-level
    /// protocol that this jay_wl_surface_factory_v1 is assigned to.
    ///
    /// The version of the wl_surface is the version of this
    /// jay_wl_surface_factory_v1 object, which is in turn the version that the
    /// client bound the jay_wl_surface_factory_manager_v1 with.
    #[inline]
    pub fn new_try_send_surface(
        &self,
    ) -> Result<Rc<WlSurface>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_surface(
            &id,
        )?;
        Ok(id)
    }

    /// a new wl_surface has been created
    ///
    /// This event is sent when the compositor creates a new wl_surface object.
    /// The meaning and use of the surface are defined by the higher-level
    /// protocol that this jay_wl_surface_factory_v1 is assigned to.
    ///
    /// The version of the wl_surface is the version of this
    /// jay_wl_surface_factory_v1 object, which is in turn the version that the
    /// client bound the jay_wl_surface_factory_manager_v1 with.
    #[inline]
    pub fn new_send_surface(
        &self,
    ) -> Rc<WlSurface> {
        let id = self.core.create_child();
        self.send_surface(
            &id,
        );
        id
    }
}

/// A message handler for [`JayWlSurfaceFactoryV1`] proxies.
pub trait JayWlSurfaceFactoryV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<JayWlSurfaceFactoryV1>) {
        slf.core.delete_id();
    }

    /// destroy this factory
    ///
    /// This request must only be sent while the jay_wl_surface_factory_v1 is in
    /// the initial or abandoned state. Trying to send it in the assigned state
    /// raises the still_assigned error.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<JayWlSurfaceFactoryV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("jay_wl_surface_factory_v1.destroy", &e);
        }
    }

    /// a new wl_surface has been created
    ///
    /// This event is sent when the compositor creates a new wl_surface object.
    /// The meaning and use of the surface are defined by the higher-level
    /// protocol that this jay_wl_surface_factory_v1 is assigned to.
    ///
    /// The version of the wl_surface is the version of this
    /// jay_wl_surface_factory_v1 object, which is in turn the version that the
    /// client bound the jay_wl_surface_factory_manager_v1 with.
    ///
    /// # Arguments
    ///
    /// - `id`: the new surface
    #[inline]
    fn handle_surface(
        &mut self,
        slf: &Rc<JayWlSurfaceFactoryV1>,
        id: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_surface(
            id,
        );
        if let Err(e) = res {
            log_forward("jay_wl_surface_factory_v1.surface", &e);
        }
    }
}

impl ObjectPrivate for JayWlSurfaceFactoryV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::JayWlSurfaceFactoryV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> jay_wl_surface_factory_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> jay_wl_surface_factory_v1#{}.surface(id: wl_surface#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WlSurface::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetServerId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_surface(&self, arg0);
                } else {
                    DefaultHandler.handle_surface(&self, arg0);
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
            0 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "surface",
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

impl Object for JayWlSurfaceFactoryV1 {
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

impl JayWlSurfaceFactoryV1 {
    /// Since when the error.already_assigned enum variant is available.
    pub const ENM__ERROR_ALREADY_ASSIGNED__SINCE: u32 = 1;
    /// Since when the error.still_assigned enum variant is available.
    pub const ENM__ERROR_STILL_ASSIGNED__SINCE: u32 = 1;
}

/// fatal error
///
/// These fatal protocol errors may be emitted in response to
/// invalid requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct JayWlSurfaceFactoryV1Error(pub u32);

impl JayWlSurfaceFactoryV1Error {
    /// the factory was already assigned to a producer
    pub const ALREADY_ASSIGNED: Self = Self(0);

    /// the factory is still assigned to a producer
    pub const STILL_ASSIGNED: Self = Self(1);
}

impl Debug for JayWlSurfaceFactoryV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ALREADY_ASSIGNED => "ALREADY_ASSIGNED",
            Self::STILL_ASSIGNED => "STILL_ASSIGNED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
