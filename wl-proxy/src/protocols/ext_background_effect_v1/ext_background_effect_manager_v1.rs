//! background effect factory
//!
//! This protocol provides a way to improve visuals of translucent surfaces
//! by applying effects like blur to the background behind them.
//!
//! The capabilities are send when the global is bound, and every time they
//! change. Note that when the capability goes away, the corresponding effect
//! is no longer applied by the compositor, even if it was set before.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A ext_background_effect_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ExtBackgroundEffectManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ExtBackgroundEffectManagerV1Handler>,
}

struct DefaultHandler;

impl ExtBackgroundEffectManagerV1Handler for DefaultHandler { }

impl ConcreteObject for ExtBackgroundEffectManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ExtBackgroundEffectManagerV1;
    const INTERFACE_NAME: &str = "ext_background_effect_manager_v1";
}

impl ExtBackgroundEffectManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ExtBackgroundEffectManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ExtBackgroundEffectManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ExtBackgroundEffectManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtBackgroundEffectManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ExtBackgroundEffectManagerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the background effect manager
    ///
    /// Informs the server that the client will no longer be using this
    /// protocol object. Existing objects created by this object are not
    /// affected.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_background_effect_manager_v1#{}.destroy()\n", id);
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

    /// destroy the background effect manager
    ///
    /// Informs the server that the client will no longer be using this
    /// protocol object. Existing objects created by this object are not
    /// affected.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("ext_background_effect_manager_v1.destroy", &e);
        }
    }

    /// Since when the capabilities message is available.
    pub const MSG__CAPABILITIES__SINCE: u32 = 1;

    /// capabilities of the compositor
    ///
    /// # Arguments
    ///
    /// - `flags`:
    #[inline]
    pub fn try_send_capabilities(
        &self,
        flags: ExtBackgroundEffectManagerV1Capability,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            flags,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: ExtBackgroundEffectManagerV1Capability) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_background_effect_manager_v1#{}.capabilities(flags: {:?})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// capabilities of the compositor
    ///
    /// # Arguments
    ///
    /// - `flags`:
    #[inline]
    pub fn send_capabilities(
        &self,
        flags: ExtBackgroundEffectManagerV1Capability,
    ) {
        let res = self.try_send_capabilities(
            flags,
        );
        if let Err(e) = res {
            log_send("ext_background_effect_manager_v1.capabilities", &e);
        }
    }

    /// Since when the get_background_effect message is available.
    pub const MSG__GET_BACKGROUND_EFFECT__SINCE: u32 = 1;

    /// get a background effects object
    ///
    /// Instantiate an interface extension for the given wl_surface to add
    /// effects like blur for the background behind it.
    ///
    /// If the given wl_surface already has a ext_background_effect_surface_v1
    /// object associated, the background_effect_exists protocol error will be
    /// raised.
    ///
    /// # Arguments
    ///
    /// - `id`: the new ext_background_effect_surface_v1 object
    /// - `surface`: the surface
    #[inline]
    pub fn try_send_get_background_effect(
        &self,
        id: &Rc<ExtBackgroundEffectSurfaceV1>,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_background_effect_manager_v1#{}.get_background_effect(id: ext_background_effect_surface_v1#{}, surface: wl_surface#{})\n", id, arg0, arg1);
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

    /// get a background effects object
    ///
    /// Instantiate an interface extension for the given wl_surface to add
    /// effects like blur for the background behind it.
    ///
    /// If the given wl_surface already has a ext_background_effect_surface_v1
    /// object associated, the background_effect_exists protocol error will be
    /// raised.
    ///
    /// # Arguments
    ///
    /// - `id`: the new ext_background_effect_surface_v1 object
    /// - `surface`: the surface
    #[inline]
    pub fn send_get_background_effect(
        &self,
        id: &Rc<ExtBackgroundEffectSurfaceV1>,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_get_background_effect(
            id,
            surface,
        );
        if let Err(e) = res {
            log_send("ext_background_effect_manager_v1.get_background_effect", &e);
        }
    }

    /// get a background effects object
    ///
    /// Instantiate an interface extension for the given wl_surface to add
    /// effects like blur for the background behind it.
    ///
    /// If the given wl_surface already has a ext_background_effect_surface_v1
    /// object associated, the background_effect_exists protocol error will be
    /// raised.
    ///
    /// # Arguments
    ///
    /// - `surface`: the surface
    #[inline]
    pub fn new_try_send_get_background_effect(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Result<Rc<ExtBackgroundEffectSurfaceV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_background_effect(
            &id,
            surface,
        )?;
        Ok(id)
    }

    /// get a background effects object
    ///
    /// Instantiate an interface extension for the given wl_surface to add
    /// effects like blur for the background behind it.
    ///
    /// If the given wl_surface already has a ext_background_effect_surface_v1
    /// object associated, the background_effect_exists protocol error will be
    /// raised.
    ///
    /// # Arguments
    ///
    /// - `surface`: the surface
    #[inline]
    pub fn new_send_get_background_effect(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Rc<ExtBackgroundEffectSurfaceV1> {
        let id = self.core.create_child();
        self.send_get_background_effect(
            &id,
            surface,
        );
        id
    }
}

/// A message handler for [`ExtBackgroundEffectManagerV1`] proxies.
pub trait ExtBackgroundEffectManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ExtBackgroundEffectManagerV1>) {
        slf.core.delete_id();
    }

    /// destroy the background effect manager
    ///
    /// Informs the server that the client will no longer be using this
    /// protocol object. Existing objects created by this object are not
    /// affected.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ExtBackgroundEffectManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("ext_background_effect_manager_v1.destroy", &e);
        }
    }

    /// capabilities of the compositor
    ///
    /// # Arguments
    ///
    /// - `flags`:
    #[inline]
    fn handle_capabilities(
        &mut self,
        slf: &Rc<ExtBackgroundEffectManagerV1>,
        flags: ExtBackgroundEffectManagerV1Capability,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_capabilities(
            flags,
        );
        if let Err(e) = res {
            log_forward("ext_background_effect_manager_v1.capabilities", &e);
        }
    }

    /// get a background effects object
    ///
    /// Instantiate an interface extension for the given wl_surface to add
    /// effects like blur for the background behind it.
    ///
    /// If the given wl_surface already has a ext_background_effect_surface_v1
    /// object associated, the background_effect_exists protocol error will be
    /// raised.
    ///
    /// # Arguments
    ///
    /// - `id`: the new ext_background_effect_surface_v1 object
    /// - `surface`: the surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_background_effect(
        &mut self,
        slf: &Rc<ExtBackgroundEffectManagerV1>,
        id: &Rc<ExtBackgroundEffectSurfaceV1>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_background_effect(
            id,
            surface,
        );
        if let Err(e) = res {
            log_forward("ext_background_effect_manager_v1.get_background_effect", &e);
        }
    }
}

impl ObjectPrivate for ExtBackgroundEffectManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ExtBackgroundEffectManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_background_effect_manager_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_background_effect_manager_v1#{}.get_background_effect(id: ext_background_effect_surface_v1#{}, surface: wl_surface#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = ExtBackgroundEffectSurfaceV1::new(&self.core.state, self.core.version);
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
                    (**handler).handle_get_background_effect(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_background_effect(&self, arg0, arg1);
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
                let arg0 = ExtBackgroundEffectManagerV1Capability(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: ExtBackgroundEffectManagerV1Capability) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_background_effect_manager_v1#{}.capabilities(flags: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_capabilities(&self, arg0);
                } else {
                    DefaultHandler.handle_capabilities(&self, arg0);
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
            1 => "get_background_effect",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "capabilities",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ExtBackgroundEffectManagerV1 {
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

impl ExtBackgroundEffectManagerV1 {
    /// Since when the error.background_effect_exists enum variant is available.
    pub const ENM__ERROR_BACKGROUND_EFFECT_EXISTS__SINCE: u32 = 1;

    /// Since when the capability.blur enum variant is available.
    pub const ENM__CAPABILITY_BLUR__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtBackgroundEffectManagerV1Error(pub u32);

impl ExtBackgroundEffectManagerV1Error {
    /// the surface already has a background effect object
    pub const BACKGROUND_EFFECT_EXISTS: Self = Self(0);
}

impl Debug for ExtBackgroundEffectManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::BACKGROUND_EFFECT_EXISTS => "BACKGROUND_EFFECT_EXISTS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct ExtBackgroundEffectManagerV1Capability(pub u32);

/// An iterator over the set bits in a [`ExtBackgroundEffectManagerV1Capability`].
///
/// You can construct this with the `IntoIterator` implementation of `ExtBackgroundEffectManagerV1Capability`.
#[derive(Clone, Debug)]
pub struct ExtBackgroundEffectManagerV1CapabilityIter(pub u32);

impl ExtBackgroundEffectManagerV1Capability {
    /// the compositor supports applying blur
    pub const BLUR: Self = Self(1);
}

impl ExtBackgroundEffectManagerV1Capability {
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    #[inline]
    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    #[inline]
    pub const fn insert(&mut self, other: Self) {
        *self = self.union(other);
    }

    #[inline]
    pub const fn remove(&mut self, other: Self) {
        *self = self.difference(other);
    }

    #[inline]
    pub const fn toggle(&mut self, other: Self) {
        *self = self.symmetric_difference(other);
    }

    #[inline]
    pub const fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }

    #[inline]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline]
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    #[inline]
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(!self.0)
    }

    #[inline]
    #[must_use]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    #[inline]
    pub const fn all_known() -> Self {
        #[allow(clippy::eq_op, clippy::identity_op)]
        Self(0 | 1)
    }
}

impl Iterator for ExtBackgroundEffectManagerV1CapabilityIter {
    type Item = ExtBackgroundEffectManagerV1Capability;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(ExtBackgroundEffectManagerV1Capability(bit))
    }
}

impl IntoIterator for ExtBackgroundEffectManagerV1Capability {
    type Item = ExtBackgroundEffectManagerV1Capability;
    type IntoIter = ExtBackgroundEffectManagerV1CapabilityIter;

    fn into_iter(self) -> Self::IntoIter {
        ExtBackgroundEffectManagerV1CapabilityIter(self.0)
    }
}

impl BitAnd for ExtBackgroundEffectManagerV1Capability {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for ExtBackgroundEffectManagerV1Capability {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for ExtBackgroundEffectManagerV1Capability {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for ExtBackgroundEffectManagerV1Capability {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for ExtBackgroundEffectManagerV1Capability {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for ExtBackgroundEffectManagerV1Capability {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for ExtBackgroundEffectManagerV1Capability {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for ExtBackgroundEffectManagerV1Capability {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for ExtBackgroundEffectManagerV1Capability {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for ExtBackgroundEffectManagerV1Capability {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = self.0;
        let mut first = true;
        if v & 1 == 1 {
            v &= !1;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("BLUR")?;
        }
        if v != 0 {
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            write!(f, "0x{v:032x}")?;
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}
