//! protocol for fifo constraints
//!
//! When a Wayland compositor considers applying a content update,
//! it must ensure all the update's readiness constraints (fences, etc)
//! are met.
//!
//! This protocol provides a way to use the completion of a display refresh
//! cycle as an additional readiness constraint.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_fifo_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpFifoManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpFifoManagerV1Handler>,
}

struct DefaultHandler;

impl WpFifoManagerV1Handler for DefaultHandler { }

impl WpFifoManagerV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::WpFifoManagerV1;
    pub const INTERFACE_NAME: &str = "wp_fifo_manager_v1";
}

impl WpFifoManagerV1 {
    pub fn set_handler(&self, handler: impl WpFifoManagerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn WpFifoManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpFifoManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpFifoManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpFifoManagerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// unbind from the manager interface
    ///
    /// Informs the server that the client will no longer be using
    /// this protocol object. Existing objects created by this object
    /// are not affected.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_fifo_manager_v1#{}.destroy()\n", id);
            self.core.state.log(args);
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
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the get_fifo message is available.
    pub const MSG__GET_FIFO__SINCE: u32 = 1;

    /// request fifo interface for surface
    ///
    /// Establish a fifo object for a surface that may be used to add
    /// display refresh constraints to content updates.
    ///
    /// Only one such object may exist for a surface and attempting
    /// to create more than one will result in an already_exists
    /// protocol error. If a surface is acted on by multiple software
    /// components, general best practice is that only the component
    /// performing wl_surface.attach operations should use this protocol.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn send_get_fifo(
        &self,
        id: &Rc<WpFifoV1>,
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("surface")),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_fifo_manager_v1#{}.get_fifo(id: wp_fifo_v1#{}, surface: wl_surface#{})\n", id, arg0_id, arg1_id);
            self.core.state.log(args);
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
            1,
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }
}

/// A message handler for [WpFifoManagerV1] proxies.
pub trait WpFifoManagerV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpFifoManagerV1>) {
        let _ = slf.core.delete_id();
    }

    /// unbind from the manager interface
    ///
    /// Informs the server that the client will no longer be using
    /// this protocol object. Existing objects created by this object
    /// are not affected.
    #[inline]
    fn handle_destroy(
        &mut self,
        _slf: &Rc<WpFifoManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_fifo_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// request fifo interface for surface
    ///
    /// Establish a fifo object for a surface that may be used to add
    /// display refresh constraints to content updates.
    ///
    /// Only one such object may exist for a surface and attempting
    /// to create more than one will result in an already_exists
    /// protocol error. If a surface is acted on by multiple software
    /// components, general best practice is that only the component
    /// performing wl_surface.attach operations should use this protocol.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_fifo(
        &mut self,
        _slf: &Rc<WpFifoManagerV1>,
        id: &Rc<WpFifoV1>,
        surface: &Rc<WlSurface>,
    ) {
        let res = _slf.send_get_fifo(
            id,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_fifo_manager_v1.get_fifo message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for WpFifoManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpFifoManagerV1, version),
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
            let _ = self.core.delete_id();
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
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_fifo_manager_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
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
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_fifo_manager_v1#{}.get_fifo(id: wp_fifo_v1#{}, surface: wl_surface#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = WpFifoV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_fifo(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_fifo(&self, arg0, arg1);
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
            0 => "destroy",
            1 => "get_fifo",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WpFifoManagerV1 {
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

impl WpFifoManagerV1 {
    /// Since when the error.already_exists enum variant is available.
    pub const ENM__ERROR_ALREADY_EXISTS__SINCE: u32 = 1;
}

/// fatal presentation error
///
/// These fatal protocol errors may be emitted in response to
/// illegal requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpFifoManagerV1Error(pub u32);

impl WpFifoManagerV1Error {
    /// fifo manager already exists for surface
    pub const ALREADY_EXISTS: Self = Self(0);
}

impl Debug for WpFifoManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ALREADY_EXISTS => "ALREADY_EXISTS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
