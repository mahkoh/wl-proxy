//! an icon surface factory
//!
//! This interface allows the compositor to draw a concrete icon for a
//! subject.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A jay_icon_surface_factory_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct JayIconSurfaceFactoryV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn JayIconSurfaceFactoryV1Handler>,
}

struct DefaultHandler;

impl JayIconSurfaceFactoryV1Handler for DefaultHandler { }

impl ConcreteObject for JayIconSurfaceFactoryV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::JayIconSurfaceFactoryV1;
    const INTERFACE_NAME: &str = "jay_icon_surface_factory_v1";
}

impl JayIconSurfaceFactoryV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl JayIconSurfaceFactoryV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn JayIconSurfaceFactoryV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for JayIconSurfaceFactoryV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JayIconSurfaceFactoryV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl JayIconSurfaceFactoryV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy this object
    ///
    /// The jay_icon_surface_factory_v1.stop request must have been sent
    /// beforehand to stop the factory. Otherwise the not_stopped error is
    /// raised.
    ///
    /// Unless the client is about to close the Wayland connection, it should
    /// wait for the jay_icon_surface_factory_v1.stopped event so as not to leak
    /// jay_icon_surface_v1 objects contained in in-flight
    /// jay_icon_surface_factory_v1.surface events.
    ///
    /// This request has no effect on any existing jay_icon_surface_v1 objects.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= jay_icon_surface_factory_v1#{}.destroy()\n", id);
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

    /// destroy this object
    ///
    /// The jay_icon_surface_factory_v1.stop request must have been sent
    /// beforehand to stop the factory. Otherwise the not_stopped error is
    /// raised.
    ///
    /// Unless the client is about to close the Wayland connection, it should
    /// wait for the jay_icon_surface_factory_v1.stopped event so as not to leak
    /// jay_icon_surface_v1 objects contained in in-flight
    /// jay_icon_surface_factory_v1.surface events.
    ///
    /// This request has no effect on any existing jay_icon_surface_v1 objects.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("jay_icon_surface_factory_v1.destroy", &e);
        }
    }

    /// Since when the stop message is available.
    pub const MSG__STOP__SINCE: u32 = 1;

    /// stop the factory
    ///
    /// This request asks the compositor to stop sending
    /// jay_icon_surface_factory_v1.surface events. The compositor will reply
    /// with a jay_icon_surface_factory_v1.stopped event and will no longer send
    /// any jay_icon_surface_factory_v1.surface events after that event.
    ///
    /// The client should send jay_icon_surface_factory_v1.destroy after
    /// receiving the jay_icon_surface_factory_v1.stopped event to destroy this
    /// object.
    ///
    /// This request removes the factory from the subject so that a new factory
    /// can be created for the subject.
    ///
    /// This request transitions the jay_wl_surface_factory_v1 that was used to
    /// create this factory from the assigned to the abandoned state.
    #[inline]
    pub fn try_send_stop(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= jay_icon_surface_factory_v1#{}.stop()\n", id);
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
            1,
        ]);
        Ok(())
    }

    /// stop the factory
    ///
    /// This request asks the compositor to stop sending
    /// jay_icon_surface_factory_v1.surface events. The compositor will reply
    /// with a jay_icon_surface_factory_v1.stopped event and will no longer send
    /// any jay_icon_surface_factory_v1.surface events after that event.
    ///
    /// The client should send jay_icon_surface_factory_v1.destroy after
    /// receiving the jay_icon_surface_factory_v1.stopped event to destroy this
    /// object.
    ///
    /// This request removes the factory from the subject so that a new factory
    /// can be created for the subject.
    ///
    /// This request transitions the jay_wl_surface_factory_v1 that was used to
    /// create this factory from the assigned to the abandoned state.
    #[inline]
    pub fn send_stop(
        &self,
    ) {
        let res = self.try_send_stop(
        );
        if let Err(e) = res {
            log_send("jay_icon_surface_factory_v1.stop", &e);
        }
    }

    /// Since when the stopped message is available.
    pub const MSG__STOPPED__SINCE: u32 = 1;

    /// the reply to the stop request
    ///
    /// This event is sent in response to the jay_icon_surface_factory_v1.stop
    /// request. See the description of that request for details.
    #[inline]
    pub fn try_send_stopped(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= jay_icon_surface_factory_v1#{}.stopped()\n", client_id, id);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id);
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
        Ok(())
    }

    /// the reply to the stop request
    ///
    /// This event is sent in response to the jay_icon_surface_factory_v1.stop
    /// request. See the description of that request for details.
    #[inline]
    pub fn send_stopped(
        &self,
    ) {
        let res = self.try_send_stopped(
        );
        if let Err(e) = res {
            log_send("jay_icon_surface_factory_v1.stopped", &e);
        }
    }

    /// Since when the surface message is available.
    pub const MSG__SURFACE__SINCE: u32 = 1;

    /// a new jay_icon_surface_v1 object
    ///
    /// The compositor sends this event to ask the client to draw an icon for
    /// the subject.
    ///
    /// This event is preceded by a jay_wl_surface_factory_v1.surface event on
    /// the jay_wl_surface_factory_v1 that was used to create this factory.
    /// That jay_wl_surface_factory_v1.surface event contains the underlying
    /// wl_surface. The new jay_icon_surface_v1 object is the role object of
    /// that wl_surface and the wl_surface has the jay_icon_surface_v1 role.
    ///
    /// Some time after this event, the compositor sends a configuration
    /// sequence on the new jay_icon_surface_v1.
    ///
    /// # Arguments
    ///
    /// - `id`: the new icon surface
    #[inline]
    pub fn try_send_surface(
        &self,
        id: &Rc<JayIconSurfaceV1>,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= jay_icon_surface_factory_v1#{}.surface(id: jay_icon_surface_v1#{})\n", client_id, id, arg0);
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

    /// a new jay_icon_surface_v1 object
    ///
    /// The compositor sends this event to ask the client to draw an icon for
    /// the subject.
    ///
    /// This event is preceded by a jay_wl_surface_factory_v1.surface event on
    /// the jay_wl_surface_factory_v1 that was used to create this factory.
    /// That jay_wl_surface_factory_v1.surface event contains the underlying
    /// wl_surface. The new jay_icon_surface_v1 object is the role object of
    /// that wl_surface and the wl_surface has the jay_icon_surface_v1 role.
    ///
    /// Some time after this event, the compositor sends a configuration
    /// sequence on the new jay_icon_surface_v1.
    ///
    /// # Arguments
    ///
    /// - `id`: the new icon surface
    #[inline]
    pub fn send_surface(
        &self,
        id: &Rc<JayIconSurfaceV1>,
    ) {
        let res = self.try_send_surface(
            id,
        );
        if let Err(e) = res {
            log_send("jay_icon_surface_factory_v1.surface", &e);
        }
    }

    /// a new jay_icon_surface_v1 object
    ///
    /// The compositor sends this event to ask the client to draw an icon for
    /// the subject.
    ///
    /// This event is preceded by a jay_wl_surface_factory_v1.surface event on
    /// the jay_wl_surface_factory_v1 that was used to create this factory.
    /// That jay_wl_surface_factory_v1.surface event contains the underlying
    /// wl_surface. The new jay_icon_surface_v1 object is the role object of
    /// that wl_surface and the wl_surface has the jay_icon_surface_v1 role.
    ///
    /// Some time after this event, the compositor sends a configuration
    /// sequence on the new jay_icon_surface_v1.
    #[inline]
    pub fn new_try_send_surface(
        &self,
    ) -> Result<Rc<JayIconSurfaceV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_surface(
            &id,
        )?;
        Ok(id)
    }

    /// a new jay_icon_surface_v1 object
    ///
    /// The compositor sends this event to ask the client to draw an icon for
    /// the subject.
    ///
    /// This event is preceded by a jay_wl_surface_factory_v1.surface event on
    /// the jay_wl_surface_factory_v1 that was used to create this factory.
    /// That jay_wl_surface_factory_v1.surface event contains the underlying
    /// wl_surface. The new jay_icon_surface_v1 object is the role object of
    /// that wl_surface and the wl_surface has the jay_icon_surface_v1 role.
    ///
    /// Some time after this event, the compositor sends a configuration
    /// sequence on the new jay_icon_surface_v1.
    #[inline]
    pub fn new_send_surface(
        &self,
    ) -> Rc<JayIconSurfaceV1> {
        let id = self.core.create_child();
        self.send_surface(
            &id,
        );
        id
    }
}

/// A message handler for [`JayIconSurfaceFactoryV1`] proxies.
pub trait JayIconSurfaceFactoryV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<JayIconSurfaceFactoryV1>) {
        slf.core.delete_id();
    }

    /// destroy this object
    ///
    /// The jay_icon_surface_factory_v1.stop request must have been sent
    /// beforehand to stop the factory. Otherwise the not_stopped error is
    /// raised.
    ///
    /// Unless the client is about to close the Wayland connection, it should
    /// wait for the jay_icon_surface_factory_v1.stopped event so as not to leak
    /// jay_icon_surface_v1 objects contained in in-flight
    /// jay_icon_surface_factory_v1.surface events.
    ///
    /// This request has no effect on any existing jay_icon_surface_v1 objects.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<JayIconSurfaceFactoryV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("jay_icon_surface_factory_v1.destroy", &e);
        }
    }

    /// stop the factory
    ///
    /// This request asks the compositor to stop sending
    /// jay_icon_surface_factory_v1.surface events. The compositor will reply
    /// with a jay_icon_surface_factory_v1.stopped event and will no longer send
    /// any jay_icon_surface_factory_v1.surface events after that event.
    ///
    /// The client should send jay_icon_surface_factory_v1.destroy after
    /// receiving the jay_icon_surface_factory_v1.stopped event to destroy this
    /// object.
    ///
    /// This request removes the factory from the subject so that a new factory
    /// can be created for the subject.
    ///
    /// This request transitions the jay_wl_surface_factory_v1 that was used to
    /// create this factory from the assigned to the abandoned state.
    #[inline]
    fn handle_stop(
        &mut self,
        slf: &Rc<JayIconSurfaceFactoryV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_stop(
        );
        if let Err(e) = res {
            log_forward("jay_icon_surface_factory_v1.stop", &e);
        }
    }

    /// the reply to the stop request
    ///
    /// This event is sent in response to the jay_icon_surface_factory_v1.stop
    /// request. See the description of that request for details.
    #[inline]
    fn handle_stopped(
        &mut self,
        slf: &Rc<JayIconSurfaceFactoryV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_stopped(
        );
        if let Err(e) = res {
            log_forward("jay_icon_surface_factory_v1.stopped", &e);
        }
    }

    /// a new jay_icon_surface_v1 object
    ///
    /// The compositor sends this event to ask the client to draw an icon for
    /// the subject.
    ///
    /// This event is preceded by a jay_wl_surface_factory_v1.surface event on
    /// the jay_wl_surface_factory_v1 that was used to create this factory.
    /// That jay_wl_surface_factory_v1.surface event contains the underlying
    /// wl_surface. The new jay_icon_surface_v1 object is the role object of
    /// that wl_surface and the wl_surface has the jay_icon_surface_v1 role.
    ///
    /// Some time after this event, the compositor sends a configuration
    /// sequence on the new jay_icon_surface_v1.
    ///
    /// # Arguments
    ///
    /// - `id`: the new icon surface
    #[inline]
    fn handle_surface(
        &mut self,
        slf: &Rc<JayIconSurfaceFactoryV1>,
        id: &Rc<JayIconSurfaceV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_surface(
            id,
        );
        if let Err(e) = res {
            log_forward("jay_icon_surface_factory_v1.surface", &e);
        }
    }
}

impl ObjectPrivate for JayIconSurfaceFactoryV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::JayIconSurfaceFactoryV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> jay_icon_surface_factory_v1#{}.destroy()\n", client_id, id);
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
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> jay_icon_surface_factory_v1#{}.stop()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_stop(&self);
                } else {
                    DefaultHandler.handle_stop(&self);
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
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> jay_icon_surface_factory_v1#{}.stopped()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_stopped(&self);
                } else {
                    DefaultHandler.handle_stopped(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> jay_icon_surface_factory_v1#{}.surface(id: jay_icon_surface_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = JayIconSurfaceV1::new(&self.core.state, self.core.version);
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
            1 => "stop",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "stopped",
            1 => "surface",
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

impl Object for JayIconSurfaceFactoryV1 {
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

impl JayIconSurfaceFactoryV1 {
    /// Since when the error.not_stopped enum variant is available.
    pub const ENM__ERROR_NOT_STOPPED__SINCE: u32 = 1;
}

/// fatal error
///
/// These fatal protocol errors may be emitted in response to
/// invalid requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct JayIconSurfaceFactoryV1Error(pub u32);

impl JayIconSurfaceFactoryV1Error {
    /// the factory has not been stopped
    pub const NOT_STOPPED: Self = Self(0);
}

impl Debug for JayIconSurfaceFactoryV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NOT_STOPPED => "NOT_STOPPED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
