//! receive relative pointer motion events
//!
//! The wp_locked_pointer interface represents a locked pointer state.
//!
//! While the lock of this object is active, the wl_pointer objects of the
//! associated seat will not emit any wl_pointer.motion events.
//!
//! This object will send the event 'locked' when the lock is activated.
//! Whenever the lock is activated, it is guaranteed that the locked surface
//! will already have received pointer focus and that the pointer will be
//! within the region passed to the request creating this object.
//!
//! To unlock the pointer, send the destroy request. This will also destroy
//! the wp_locked_pointer object.
//!
//! If the compositor decides to unlock the pointer the unlocked event is
//! sent. See wp_locked_pointer.unlock for details.
//!
//! When unlocking, the compositor may warp the cursor position to the set
//! cursor position hint. If it does, it will not result in any relative
//! motion events emitted via wp_relative_pointer.
//!
//! If the surface the lock was requested on is destroyed and the lock is not
//! yet activated, the wp_locked_pointer object is now defunct and must be
//! destroyed.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_locked_pointer_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpLockedPointerV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn ZwpLockedPointerV1Handler>,
}

struct DefaultHandler;

impl ZwpLockedPointerV1Handler for DefaultHandler { }

impl ZwpLockedPointerV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ProxyInterface = ProxyInterface::ZwpLockedPointerV1;
    pub const INTERFACE_NAME: &str = "zwp_locked_pointer_v1";
}

impl ZwpLockedPointerV1 {
    pub fn set_handler(&self, handler: impl ZwpLockedPointerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpLockedPointerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpLockedPointerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpLockedPointerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpLockedPointerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the locked pointer object
    ///
    /// Destroy the locked pointer object. If applicable, the compositor will
    /// unlock the pointer.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_locked_pointer_v1#{}.destroy()\n", id);
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

    /// Since when the set_cursor_position_hint message is available.
    pub const MSG__SET_CURSOR_POSITION_HINT__SINCE: u32 = 1;

    /// set the pointer cursor position hint
    ///
    /// Set the cursor position hint relative to the top left corner of the
    /// surface.
    ///
    /// If the client is drawing its own cursor, it should update the position
    /// hint to the position of its own cursor. A compositor may use this
    /// information to warp the pointer upon unlock in order to avoid pointer
    /// jumps.
    ///
    /// The cursor position hint is double-buffered state, see
    /// wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `surface_x`: surface-local x coordinate
    /// - `surface_y`: surface-local y coordinate
    #[inline]
    pub fn send_set_cursor_position_hint(
        &self,
        surface_x: Fixed,
        surface_y: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            surface_x,
            surface_y,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_locked_pointer_v1#{}.set_cursor_position_hint(surface_x: {}, surface_y: {})\n", id, arg0, arg1);
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
            arg0.to_wire() as u32,
            arg1.to_wire() as u32,
        ]);
        Ok(())
    }

    /// Since when the set_region message is available.
    pub const MSG__SET_REGION__SINCE: u32 = 1;

    /// set a new lock region
    ///
    /// Set a new region used to lock the pointer.
    ///
    /// The new lock region is double-buffered, see wl_surface.commit.
    ///
    /// For details about the lock region, see wp_locked_pointer.
    ///
    /// # Arguments
    ///
    /// - `region`: region of surface
    #[inline]
    pub fn send_set_region(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError::ArgNoServerId("region")),
                Some(id) => id,
            },
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_locked_pointer_v1#{}.set_region(region: wl_region#{})\n", id, arg0_id);
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
            2,
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the locked message is available.
    pub const MSG__LOCKED__SINCE: u32 = 1;

    /// lock activation event
    ///
    /// Notification that the pointer lock of the seat's pointer is activated.
    #[inline]
    pub fn send_locked(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_locked_pointer_v1#{}.locked()\n", client.endpoint.id, id);
            self.core.state.log(args);
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

    /// Since when the unlocked message is available.
    pub const MSG__UNLOCKED__SINCE: u32 = 1;

    /// lock deactivation event
    ///
    /// Notification that the pointer lock of the seat's pointer is no longer
    /// active. If this is a oneshot pointer lock (see
    /// wp_pointer_constraints.lifetime) this object is now defunct and should
    /// be destroyed. If this is a persistent pointer lock (see
    /// wp_pointer_constraints.lifetime) this pointer lock may again
    /// reactivate in the future.
    #[inline]
    pub fn send_unlocked(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_locked_pointer_v1#{}.unlocked()\n", client.endpoint.id, id);
            self.core.state.log(args);
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
}

/// A message handler for [ZwpLockedPointerV1] proxies.
pub trait ZwpLockedPointerV1Handler: Any {
    /// destroy the locked pointer object
    ///
    /// Destroy the locked pointer object. If applicable, the compositor will
    /// unlock the pointer.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<ZwpLockedPointerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_locked_pointer_v1.destroy message: {}", Report::new(e));
        }
    }

    /// set the pointer cursor position hint
    ///
    /// Set the cursor position hint relative to the top left corner of the
    /// surface.
    ///
    /// If the client is drawing its own cursor, it should update the position
    /// hint to the position of its own cursor. A compositor may use this
    /// information to warp the pointer upon unlock in order to avoid pointer
    /// jumps.
    ///
    /// The cursor position hint is double-buffered state, see
    /// wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `surface_x`: surface-local x coordinate
    /// - `surface_y`: surface-local y coordinate
    #[inline]
    fn set_cursor_position_hint(
        &mut self,
        _slf: &Rc<ZwpLockedPointerV1>,
        surface_x: Fixed,
        surface_y: Fixed,
    ) {
        let res = _slf.send_set_cursor_position_hint(
            surface_x,
            surface_y,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_locked_pointer_v1.set_cursor_position_hint message: {}", Report::new(e));
        }
    }

    /// set a new lock region
    ///
    /// Set a new region used to lock the pointer.
    ///
    /// The new lock region is double-buffered, see wl_surface.commit.
    ///
    /// For details about the lock region, see wp_locked_pointer.
    ///
    /// # Arguments
    ///
    /// - `region`: region of surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn set_region(
        &mut self,
        _slf: &Rc<ZwpLockedPointerV1>,
        region: Option<&Rc<WlRegion>>,
    ) {
        let res = _slf.send_set_region(
            region,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_locked_pointer_v1.set_region message: {}", Report::new(e));
        }
    }

    /// lock activation event
    ///
    /// Notification that the pointer lock of the seat's pointer is activated.
    #[inline]
    fn locked(
        &mut self,
        _slf: &Rc<ZwpLockedPointerV1>,
    ) {
        let res = _slf.send_locked(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_locked_pointer_v1.locked message: {}", Report::new(e));
        }
    }

    /// lock deactivation event
    ///
    /// Notification that the pointer lock of the seat's pointer is no longer
    /// active. If this is a oneshot pointer lock (see
    /// wp_pointer_constraints.lifetime) this object is now defunct and should
    /// be destroyed. If this is a persistent pointer lock (see
    /// wp_pointer_constraints.lifetime) this pointer lock may again
    /// reactivate in the future.
    #[inline]
    fn unlocked(
        &mut self,
        _slf: &Rc<ZwpLockedPointerV1>,
    ) {
        let res = _slf.send_unlocked(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_locked_pointer_v1.unlocked message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for ZwpLockedPointerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::ZwpLockedPointerV1, version),
            handler: Default::default(),
        })
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_locked_pointer_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg0 = Fixed::from_wire(arg0 as i32);
                let arg1 = Fixed::from_wire(arg1 as i32);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_locked_pointer_v1#{}.set_cursor_position_hint(surface_x: {}, surface_y: {})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_cursor_position_hint(&self, arg0, arg1);
                } else {
                    DefaultHandler.set_cursor_position_hint(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_locked_pointer_v1#{}.set_region(region: wl_region#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlRegion>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError::WrongObjectType("region", o.core().interface, ProxyInterface::WlRegion));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).set_region(&self, arg0);
                } else {
                    DefaultHandler.set_region(&self, arg0);
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
        let Some(mut handler) = self.handler.try_borrow() else {
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_locked_pointer_v1#{}.locked()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).locked(&self);
                } else {
                    DefaultHandler.locked(&self);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_locked_pointer_v1#{}.unlocked()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).unlocked(&self);
                } else {
                    DefaultHandler.unlocked(&self);
                }
            }
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
        Ok(())
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            1 => "set_cursor_position_hint",
            2 => "set_region",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "locked",
            1 => "unlocked",
            _ => return None,
        };
        Some(name)
    }
}

impl Proxy for ZwpLockedPointerV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn unset_handler(&self) {
        self.handler.set(None);
    }

    fn get_handler_any_ref(&self) -> Result<Ref<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(Ref::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<RefMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow_mut().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(RefMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

