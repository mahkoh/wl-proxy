//! the manager singleton
//!
//! This interface is a singleton global.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A jay_icon_surface_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct JayIconSurfaceManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn JayIconSurfaceManagerV1Handler>,
}

struct DefaultHandler;

impl JayIconSurfaceManagerV1Handler for DefaultHandler { }

impl ConcreteObject for JayIconSurfaceManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::JayIconSurfaceManagerV1;
    const INTERFACE_NAME: &str = "jay_icon_surface_manager_v1";
}

impl JayIconSurfaceManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl JayIconSurfaceManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn JayIconSurfaceManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for JayIconSurfaceManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JayIconSurfaceManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl JayIconSurfaceManagerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy this manager
    ///
    /// This has no effect on any of the created factories.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= jay_icon_surface_manager_v1#{}.destroy()\n", id);
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

    /// destroy this manager
    ///
    /// This has no effect on any of the created factories.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("jay_icon_surface_manager_v1.destroy", &e);
        }
    }

    /// Since when the create_factory message is available.
    pub const MSG__CREATE_FACTORY__SINCE: u32 = 1;

    /// create a factory for a subject
    ///
    /// This request creates a new jay_icon_surface_factory_v1 for a subject.
    ///
    /// The subject must not already have a factory. Otherwise the has_factory
    /// error is raised.
    ///
    /// The jay_wl_surface_factory_v1 object will be used to send the underlying
    /// wl_surface objects of the concrete icons. This request transitions the
    /// jay_wl_surface_factory_v1 object from the initial to the assigned state.
    ///
    /// # Arguments
    ///
    /// - `id`: the new jay_icon_surface_factory_v1
    /// - `subject`: the subject to create a factory for
    /// - `surface_factory`: the wl_surface factory
    #[inline]
    pub fn try_send_create_factory(
        &self,
        id: &Rc<JayIconSurfaceFactoryV1>,
        subject: &Rc<JayIconSurfaceSubjectV1>,
        surface_factory: &Rc<JayWlSurfaceFactoryV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            id,
            subject,
            surface_factory,
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
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("subject"))),
            Some(id) => id,
        };
        let arg2_id = match arg2.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface_factory"))),
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= jay_icon_surface_manager_v1#{}.create_factory(id: jay_icon_surface_factory_v1#{}, subject: jay_icon_surface_subject_v1#{}, surface_factory: jay_wl_surface_factory_v1#{})\n", id, arg0, arg1, arg2);
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

    /// create a factory for a subject
    ///
    /// This request creates a new jay_icon_surface_factory_v1 for a subject.
    ///
    /// The subject must not already have a factory. Otherwise the has_factory
    /// error is raised.
    ///
    /// The jay_wl_surface_factory_v1 object will be used to send the underlying
    /// wl_surface objects of the concrete icons. This request transitions the
    /// jay_wl_surface_factory_v1 object from the initial to the assigned state.
    ///
    /// # Arguments
    ///
    /// - `id`: the new jay_icon_surface_factory_v1
    /// - `subject`: the subject to create a factory for
    /// - `surface_factory`: the wl_surface factory
    #[inline]
    pub fn send_create_factory(
        &self,
        id: &Rc<JayIconSurfaceFactoryV1>,
        subject: &Rc<JayIconSurfaceSubjectV1>,
        surface_factory: &Rc<JayWlSurfaceFactoryV1>,
    ) {
        let res = self.try_send_create_factory(
            id,
            subject,
            surface_factory,
        );
        if let Err(e) = res {
            log_send("jay_icon_surface_manager_v1.create_factory", &e);
        }
    }

    /// create a factory for a subject
    ///
    /// This request creates a new jay_icon_surface_factory_v1 for a subject.
    ///
    /// The subject must not already have a factory. Otherwise the has_factory
    /// error is raised.
    ///
    /// The jay_wl_surface_factory_v1 object will be used to send the underlying
    /// wl_surface objects of the concrete icons. This request transitions the
    /// jay_wl_surface_factory_v1 object from the initial to the assigned state.
    ///
    /// # Arguments
    ///
    /// - `subject`: the subject to create a factory for
    /// - `surface_factory`: the wl_surface factory
    #[inline]
    pub fn new_try_send_create_factory(
        &self,
        subject: &Rc<JayIconSurfaceSubjectV1>,
        surface_factory: &Rc<JayWlSurfaceFactoryV1>,
    ) -> Result<Rc<JayIconSurfaceFactoryV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_create_factory(
            &id,
            subject,
            surface_factory,
        )?;
        Ok(id)
    }

    /// create a factory for a subject
    ///
    /// This request creates a new jay_icon_surface_factory_v1 for a subject.
    ///
    /// The subject must not already have a factory. Otherwise the has_factory
    /// error is raised.
    ///
    /// The jay_wl_surface_factory_v1 object will be used to send the underlying
    /// wl_surface objects of the concrete icons. This request transitions the
    /// jay_wl_surface_factory_v1 object from the initial to the assigned state.
    ///
    /// # Arguments
    ///
    /// - `subject`: the subject to create a factory for
    /// - `surface_factory`: the wl_surface factory
    #[inline]
    pub fn new_send_create_factory(
        &self,
        subject: &Rc<JayIconSurfaceSubjectV1>,
        surface_factory: &Rc<JayWlSurfaceFactoryV1>,
    ) -> Rc<JayIconSurfaceFactoryV1> {
        let id = self.core.create_child();
        self.send_create_factory(
            &id,
            subject,
            surface_factory,
        );
        id
    }
}

/// A message handler for [`JayIconSurfaceManagerV1`] proxies.
pub trait JayIconSurfaceManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<JayIconSurfaceManagerV1>) {
        slf.core.delete_id();
    }

    /// destroy this manager
    ///
    /// This has no effect on any of the created factories.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<JayIconSurfaceManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("jay_icon_surface_manager_v1.destroy", &e);
        }
    }

    /// create a factory for a subject
    ///
    /// This request creates a new jay_icon_surface_factory_v1 for a subject.
    ///
    /// The subject must not already have a factory. Otherwise the has_factory
    /// error is raised.
    ///
    /// The jay_wl_surface_factory_v1 object will be used to send the underlying
    /// wl_surface objects of the concrete icons. This request transitions the
    /// jay_wl_surface_factory_v1 object from the initial to the assigned state.
    ///
    /// # Arguments
    ///
    /// - `id`: the new jay_icon_surface_factory_v1
    /// - `subject`: the subject to create a factory for
    /// - `surface_factory`: the wl_surface factory
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_create_factory(
        &mut self,
        slf: &Rc<JayIconSurfaceManagerV1>,
        id: &Rc<JayIconSurfaceFactoryV1>,
        subject: &Rc<JayIconSurfaceSubjectV1>,
        surface_factory: &Rc<JayWlSurfaceFactoryV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_factory(
            id,
            subject,
            surface_factory,
        );
        if let Err(e) = res {
            log_forward("jay_icon_surface_manager_v1.create_factory", &e);
        }
    }
}

impl ObjectPrivate for JayIconSurfaceManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::JayIconSurfaceManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> jay_icon_surface_manager_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> jay_icon_surface_manager_v1#{}.create_factory(id: jay_icon_surface_factory_v1#{}, subject: jay_icon_surface_subject_v1#{}, surface_factory: jay_wl_surface_factory_v1#{})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let arg0 = JayIconSurfaceFactoryV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<JayIconSurfaceSubjectV1>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("subject", o.core().interface, ObjectInterface::JayIconSurfaceSubjectV1)));
                };
                let arg2_id = arg2;
                let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg2_id)));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<JayWlSurfaceFactoryV1>() else {
                    let o = client.endpoint.lookup(arg2_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface_factory", o.core().interface, ObjectInterface::JayWlSurfaceFactoryV1)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).handle_create_factory(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_create_factory(&self, arg0, arg1, arg2);
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
            1 => "create_factory",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }

    fn create_zombie(&self) -> Rc<dyn Object> {
        let slf = Self::new(&self.core.state, self.core.version);
        slf.core.make_zombie();
        slf
    }
}

impl Object for JayIconSurfaceManagerV1 {
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

impl JayIconSurfaceManagerV1 {
    /// Since when the error.has_factory enum variant is available.
    pub const ENM__ERROR_HAS_FACTORY__SINCE: u32 = 1;
}

/// fatal error
///
/// These fatal protocol errors may be emitted in response to
/// invalid requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct JayIconSurfaceManagerV1Error(pub u32);

impl JayIconSurfaceManagerV1Error {
    /// the subject already has a factory
    pub const HAS_FACTORY: Self = Self(0);
}

impl Debug for JayIconSurfaceManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::HAS_FACTORY => "HAS_FACTORY",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
