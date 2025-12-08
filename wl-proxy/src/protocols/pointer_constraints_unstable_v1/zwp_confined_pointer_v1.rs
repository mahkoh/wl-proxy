//! confined pointer object
//!
//! The wp_confined_pointer interface represents a confined pointer state.
//!
//! This object will send the event 'confined' when the confinement is
//! activated. Whenever the confinement is activated, it is guaranteed that
//! the surface the pointer is confined to will already have received pointer
//! focus and that the pointer will be within the region passed to the request
//! creating this object. It is up to the compositor to decide whether this
//! requires some user interaction and if the pointer will warp to within the
//! passed region if outside.
//!
//! To unconfine the pointer, send the destroy request. This will also destroy
//! the wp_confined_pointer object.
//!
//! If the compositor decides to unconfine the pointer the unconfined event is
//! sent. The wp_confined_pointer object is at this point defunct and should
//! be destroyed.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_confined_pointer_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpConfinedPointerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpConfinedPointerV1Handler>,
}

struct DefaultHandler;

impl ZwpConfinedPointerV1Handler for DefaultHandler { }

impl ConcreteObject for ZwpConfinedPointerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpConfinedPointerV1;
    const INTERFACE_NAME: &str = "zwp_confined_pointer_v1";
}

impl ZwpConfinedPointerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpConfinedPointerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpConfinedPointerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpConfinedPointerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpConfinedPointerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpConfinedPointerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the confined pointer object
    ///
    /// Destroy the confined pointer object. If applicable, the compositor will
    /// unconfine the pointer.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_confined_pointer_v1#{}.destroy()\n", id);
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

    /// destroy the confined pointer object
    ///
    /// Destroy the confined pointer object. If applicable, the compositor will
    /// unconfine the pointer.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwp_confined_pointer_v1.destroy", &e);
        }
    }

    /// Since when the set_region message is available.
    pub const MSG__SET_REGION__SINCE: u32 = 1;

    /// set a new confine region
    ///
    /// Set a new region used to confine the pointer.
    ///
    /// The new confine region is double-buffered, see wl_surface.commit.
    ///
    /// If the confinement is active when the new confinement region is applied
    /// and the pointer ends up outside of newly applied region, the pointer may
    /// warped to a position within the new confinement region. If warped, a
    /// wl_pointer.motion event will be emitted, but no
    /// wp_relative_pointer.relative_motion event.
    ///
    /// The compositor may also, instead of using the new region, unconfine the
    /// pointer.
    ///
    /// For details about the confine region, see wp_confined_pointer.
    ///
    /// # Arguments
    ///
    /// - `region`: region of surface
    #[inline]
    pub fn try_send_set_region(
        &self,
        region: Option<&Rc<WlRegion>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            region,
        );
        let arg0 = arg0.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("region"))),
                Some(id) => id,
            },
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_confined_pointer_v1#{}.set_region(region: wl_region#{})\n", id, arg0);
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

    /// set a new confine region
    ///
    /// Set a new region used to confine the pointer.
    ///
    /// The new confine region is double-buffered, see wl_surface.commit.
    ///
    /// If the confinement is active when the new confinement region is applied
    /// and the pointer ends up outside of newly applied region, the pointer may
    /// warped to a position within the new confinement region. If warped, a
    /// wl_pointer.motion event will be emitted, but no
    /// wp_relative_pointer.relative_motion event.
    ///
    /// The compositor may also, instead of using the new region, unconfine the
    /// pointer.
    ///
    /// For details about the confine region, see wp_confined_pointer.
    ///
    /// # Arguments
    ///
    /// - `region`: region of surface
    #[inline]
    pub fn send_set_region(
        &self,
        region: Option<&Rc<WlRegion>>,
    ) {
        let res = self.try_send_set_region(
            region,
        );
        if let Err(e) = res {
            log_send("zwp_confined_pointer_v1.set_region", &e);
        }
    }

    /// Since when the confined message is available.
    pub const MSG__CONFINED__SINCE: u32 = 1;

    /// pointer confined
    ///
    /// Notification that the pointer confinement of the seat's pointer is
    /// activated.
    #[inline]
    pub fn try_send_confined(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_confined_pointer_v1#{}.confined()\n", client_id, id);
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

    /// pointer confined
    ///
    /// Notification that the pointer confinement of the seat's pointer is
    /// activated.
    #[inline]
    pub fn send_confined(
        &self,
    ) {
        let res = self.try_send_confined(
        );
        if let Err(e) = res {
            log_send("zwp_confined_pointer_v1.confined", &e);
        }
    }

    /// Since when the unconfined message is available.
    pub const MSG__UNCONFINED__SINCE: u32 = 1;

    /// pointer unconfined
    ///
    /// Notification that the pointer confinement of the seat's pointer is no
    /// longer active. If this is a oneshot pointer confinement (see
    /// wp_pointer_constraints.lifetime) this object is now defunct and should
    /// be destroyed. If this is a persistent pointer confinement (see
    /// wp_pointer_constraints.lifetime) this pointer confinement may again
    /// reactivate in the future.
    #[inline]
    pub fn try_send_unconfined(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_confined_pointer_v1#{}.unconfined()\n", client_id, id);
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
            1,
        ]);
        Ok(())
    }

    /// pointer unconfined
    ///
    /// Notification that the pointer confinement of the seat's pointer is no
    /// longer active. If this is a oneshot pointer confinement (see
    /// wp_pointer_constraints.lifetime) this object is now defunct and should
    /// be destroyed. If this is a persistent pointer confinement (see
    /// wp_pointer_constraints.lifetime) this pointer confinement may again
    /// reactivate in the future.
    #[inline]
    pub fn send_unconfined(
        &self,
    ) {
        let res = self.try_send_unconfined(
        );
        if let Err(e) = res {
            log_send("zwp_confined_pointer_v1.unconfined", &e);
        }
    }
}

/// A message handler for [`ZwpConfinedPointerV1`] proxies.
pub trait ZwpConfinedPointerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpConfinedPointerV1>) {
        slf.core.delete_id();
    }

    /// destroy the confined pointer object
    ///
    /// Destroy the confined pointer object. If applicable, the compositor will
    /// unconfine the pointer.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwpConfinedPointerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwp_confined_pointer_v1.destroy", &e);
        }
    }

    /// set a new confine region
    ///
    /// Set a new region used to confine the pointer.
    ///
    /// The new confine region is double-buffered, see wl_surface.commit.
    ///
    /// If the confinement is active when the new confinement region is applied
    /// and the pointer ends up outside of newly applied region, the pointer may
    /// warped to a position within the new confinement region. If warped, a
    /// wl_pointer.motion event will be emitted, but no
    /// wp_relative_pointer.relative_motion event.
    ///
    /// The compositor may also, instead of using the new region, unconfine the
    /// pointer.
    ///
    /// For details about the confine region, see wp_confined_pointer.
    ///
    /// # Arguments
    ///
    /// - `region`: region of surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_region(
        &mut self,
        slf: &Rc<ZwpConfinedPointerV1>,
        region: Option<&Rc<WlRegion>>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_region(
            region,
        );
        if let Err(e) = res {
            log_forward("zwp_confined_pointer_v1.set_region", &e);
        }
    }

    /// pointer confined
    ///
    /// Notification that the pointer confinement of the seat's pointer is
    /// activated.
    #[inline]
    fn handle_confined(
        &mut self,
        slf: &Rc<ZwpConfinedPointerV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_confined(
        );
        if let Err(e) = res {
            log_forward("zwp_confined_pointer_v1.confined", &e);
        }
    }

    /// pointer unconfined
    ///
    /// Notification that the pointer confinement of the seat's pointer is no
    /// longer active. If this is a oneshot pointer confinement (see
    /// wp_pointer_constraints.lifetime) this object is now defunct and should
    /// be destroyed. If this is a persistent pointer confinement (see
    /// wp_pointer_constraints.lifetime) this pointer confinement may again
    /// reactivate in the future.
    #[inline]
    fn handle_unconfined(
        &mut self,
        slf: &Rc<ZwpConfinedPointerV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_unconfined(
        );
        if let Err(e) = res {
            log_forward("zwp_confined_pointer_v1.unconfined", &e);
        }
    }
}

impl ObjectPrivate for ZwpConfinedPointerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpConfinedPointerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_confined_pointer_v1#{}.destroy()\n", client_id, id);
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_confined_pointer_v1#{}.set_region(region: wl_region#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlRegion>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("region", o.core().interface, ObjectInterface::WlRegion)));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_set_region(&self, arg0);
                } else {
                    DefaultHandler.handle_set_region(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_confined_pointer_v1#{}.confined()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_confined(&self);
                } else {
                    DefaultHandler.handle_confined(&self);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_confined_pointer_v1#{}.unconfined()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_unconfined(&self);
                } else {
                    DefaultHandler.handle_unconfined(&self);
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
            1 => "set_region",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "confined",
            1 => "unconfined",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwpConfinedPointerV1 {
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

