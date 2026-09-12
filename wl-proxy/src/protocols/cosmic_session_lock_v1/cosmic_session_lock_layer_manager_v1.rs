use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A cosmic_session_lock_layer_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct CosmicSessionLockLayerManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn CosmicSessionLockLayerManagerV1Handler>,
}

struct DefaultHandler;

impl CosmicSessionLockLayerManagerV1Handler for DefaultHandler { }

impl ConcreteObject for CosmicSessionLockLayerManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::CosmicSessionLockLayerManagerV1;
    const INTERFACE_NAME: &str = "cosmic_session_lock_layer_manager_v1";
}

impl CosmicSessionLockLayerManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl CosmicSessionLockLayerManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn CosmicSessionLockLayerManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for CosmicSessionLockLayerManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CosmicSessionLockLayerManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl CosmicSessionLockLayerManagerV1 {
    /// Since when the set_show_on_lock message is available.
    pub const MSG__SET_SHOW_ON_LOCK__SINCE: u32 = 1;

    /// Show layer on lock.
    ///
    /// Show on lock is double buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `layer`: the layer surface
    #[inline]
    pub fn try_send_set_show_on_lock(
        &self,
        layer: &Rc<ZwlrLayerSurfaceV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            layer,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("layer"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= cosmic_session_lock_layer_manager_v1#{}.set_show_on_lock(layer: zwlr_layer_surface_v1#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// Show layer on lock.
    ///
    /// Show on lock is double buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `layer`: the layer surface
    #[inline]
    pub fn send_set_show_on_lock(
        &self,
        layer: &Rc<ZwlrLayerSurfaceV1>,
    ) {
        let res = self.try_send_set_show_on_lock(
            layer,
        );
        if let Err(e) = res {
            log_send("cosmic_session_lock_layer_manager_v1.set_show_on_lock", &e);
        }
    }

    /// Since when the unset_show_on_lock message is available.
    pub const MSG__UNSET_SHOW_ON_LOCK__SINCE: u32 = 1;

    /// Do not show layer on lock.
    ///
    /// Show on lock is double buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `layer`: the layer surface
    #[inline]
    pub fn try_send_unset_show_on_lock(
        &self,
        layer: &Rc<ZwlrLayerSurfaceV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            layer,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("layer"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= cosmic_session_lock_layer_manager_v1#{}.unset_show_on_lock(layer: zwlr_layer_surface_v1#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
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
        ]);
        Ok(())
    }

    /// Do not show layer on lock.
    ///
    /// Show on lock is double buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `layer`: the layer surface
    #[inline]
    pub fn send_unset_show_on_lock(
        &self,
        layer: &Rc<ZwlrLayerSurfaceV1>,
    ) {
        let res = self.try_send_unset_show_on_lock(
            layer,
        );
        if let Err(e) = res {
            log_send("cosmic_session_lock_layer_manager_v1.unset_show_on_lock", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the global
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= cosmic_session_lock_layer_manager_v1#{}.destroy()\n", id);
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
            2,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the global
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("cosmic_session_lock_layer_manager_v1.destroy", &e);
        }
    }
}

/// A message handler for [`CosmicSessionLockLayerManagerV1`] proxies.
pub trait CosmicSessionLockLayerManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<CosmicSessionLockLayerManagerV1>) {
        slf.core.delete_id();
    }

    /// Show layer on lock.
    ///
    /// Show on lock is double buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `layer`: the layer surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_show_on_lock(
        &mut self,
        slf: &Rc<CosmicSessionLockLayerManagerV1>,
        layer: &Rc<ZwlrLayerSurfaceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_show_on_lock(
            layer,
        );
        if let Err(e) = res {
            log_forward("cosmic_session_lock_layer_manager_v1.set_show_on_lock", &e);
        }
    }

    /// Do not show layer on lock.
    ///
    /// Show on lock is double buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `layer`: the layer surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_unset_show_on_lock(
        &mut self,
        slf: &Rc<CosmicSessionLockLayerManagerV1>,
        layer: &Rc<ZwlrLayerSurfaceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_unset_show_on_lock(
            layer,
        );
        if let Err(e) = res {
            log_forward("cosmic_session_lock_layer_manager_v1.unset_show_on_lock", &e);
        }
    }

    /// destroy the global
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<CosmicSessionLockLayerManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("cosmic_session_lock_layer_manager_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for CosmicSessionLockLayerManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::CosmicSessionLockLayerManagerV1, version),
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> cosmic_session_lock_layer_manager_v1#{}.set_show_on_lock(layer: zwlr_layer_surface_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ZwlrLayerSurfaceV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("layer", o.core().interface, ObjectInterface::ZwlrLayerSurfaceV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_show_on_lock(&self, arg0);
                } else {
                    DefaultHandler.handle_set_show_on_lock(&self, arg0);
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
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> cosmic_session_lock_layer_manager_v1#{}.unset_show_on_lock(layer: zwlr_layer_surface_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ZwlrLayerSurfaceV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("layer", o.core().interface, ObjectInterface::ZwlrLayerSurfaceV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_unset_show_on_lock(&self, arg0);
                } else {
                    DefaultHandler.handle_unset_show_on_lock(&self, arg0);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> cosmic_session_lock_layer_manager_v1#{}.destroy()\n", client_id, id);
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
            0 => "set_show_on_lock",
            1 => "unset_show_on_lock",
            2 => "destroy",
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

impl Object for CosmicSessionLockLayerManagerV1 {
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

