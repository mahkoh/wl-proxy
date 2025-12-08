//! the compositor singleton
//!
//! A compositor.  This object is a singleton global.  The
//! compositor is in charge of combining the contents of multiple
//! surfaces into one displayable output.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wl_compositor object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlCompositor {
    core: ObjectCore,
    handler: HandlerHolder<dyn WlCompositorHandler>,
}

struct DefaultHandler;

impl WlCompositorHandler for DefaultHandler { }

impl ConcreteObject for WlCompositor {
    const XML_VERSION: u32 = 6;
    const INTERFACE: ObjectInterface = ObjectInterface::WlCompositor;
    const INTERFACE_NAME: &str = "wl_compositor";
}

impl WlCompositor {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WlCompositorHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WlCompositorHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlCompositor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlCompositor")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlCompositor {
    /// Since when the create_surface message is available.
    pub const MSG__CREATE_SURFACE__SINCE: u32 = 1;

    /// create new surface
    ///
    /// Ask the compositor to create a new surface.
    ///
    /// # Arguments
    ///
    /// - `id`: the new surface
    #[inline]
    pub fn try_send_create_surface(
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_compositor#{}.create_surface(id: wl_surface#{})\n", id, arg0);
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

    /// create new surface
    ///
    /// Ask the compositor to create a new surface.
    ///
    /// # Arguments
    ///
    /// - `id`: the new surface
    #[inline]
    pub fn send_create_surface(
        &self,
        id: &Rc<WlSurface>,
    ) {
        let res = self.try_send_create_surface(
            id,
        );
        if let Err(e) = res {
            log_send("wl_compositor.create_surface", &e);
        }
    }

    /// create new surface
    ///
    /// Ask the compositor to create a new surface.
    #[inline]
    pub fn new_try_send_create_surface(
        &self,
    ) -> Result<Rc<WlSurface>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_create_surface(
            &id,
        )?;
        Ok(id)
    }

    /// create new surface
    ///
    /// Ask the compositor to create a new surface.
    #[inline]
    pub fn new_send_create_surface(
        &self,
    ) -> Rc<WlSurface> {
        let id = self.core.create_child();
        self.send_create_surface(
            &id,
        );
        id
    }

    /// Since when the create_region message is available.
    pub const MSG__CREATE_REGION__SINCE: u32 = 1;

    /// create new region
    ///
    /// Ask the compositor to create a new region.
    ///
    /// # Arguments
    ///
    /// - `id`: the new region
    #[inline]
    pub fn try_send_create_region(
        &self,
        id: &Rc<WlRegion>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_compositor#{}.create_region(id: wl_region#{})\n", id, arg0);
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

    /// create new region
    ///
    /// Ask the compositor to create a new region.
    ///
    /// # Arguments
    ///
    /// - `id`: the new region
    #[inline]
    pub fn send_create_region(
        &self,
        id: &Rc<WlRegion>,
    ) {
        let res = self.try_send_create_region(
            id,
        );
        if let Err(e) = res {
            log_send("wl_compositor.create_region", &e);
        }
    }

    /// create new region
    ///
    /// Ask the compositor to create a new region.
    #[inline]
    pub fn new_try_send_create_region(
        &self,
    ) -> Result<Rc<WlRegion>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_create_region(
            &id,
        )?;
        Ok(id)
    }

    /// create new region
    ///
    /// Ask the compositor to create a new region.
    #[inline]
    pub fn new_send_create_region(
        &self,
    ) -> Rc<WlRegion> {
        let id = self.core.create_child();
        self.send_create_region(
            &id,
        );
        id
    }
}

/// A message handler for [`WlCompositor`] proxies.
pub trait WlCompositorHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WlCompositor>) {
        slf.core.delete_id();
    }

    /// create new surface
    ///
    /// Ask the compositor to create a new surface.
    ///
    /// # Arguments
    ///
    /// - `id`: the new surface
    #[inline]
    fn handle_create_surface(
        &mut self,
        slf: &Rc<WlCompositor>,
        id: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_surface(
            id,
        );
        if let Err(e) = res {
            log_forward("wl_compositor.create_surface", &e);
        }
    }

    /// create new region
    ///
    /// Ask the compositor to create a new region.
    ///
    /// # Arguments
    ///
    /// - `id`: the new region
    #[inline]
    fn handle_create_region(
        &mut self,
        slf: &Rc<WlCompositor>,
        id: &Rc<WlRegion>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_region(
            id,
        );
        if let Err(e) = res {
            log_forward("wl_compositor.create_region", &e);
        }
    }
}

impl ObjectPrivate for WlCompositor {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WlCompositor, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_compositor#{}.create_surface(id: wl_surface#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WlSurface::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_surface(&self, arg0);
                } else {
                    DefaultHandler.handle_create_surface(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_compositor#{}.create_region(id: wl_region#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WlRegion::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_region(&self, arg0);
                } else {
                    DefaultHandler.handle_create_region(&self, arg0);
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
            0 => "create_surface",
            1 => "create_region",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WlCompositor {
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

