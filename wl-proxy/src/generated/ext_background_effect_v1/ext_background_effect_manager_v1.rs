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

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_background_effect_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaExtBackgroundEffectManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaExtBackgroundEffectManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaExtBackgroundEffectManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaExtBackgroundEffectManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaExtBackgroundEffectManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaExtBackgroundEffectManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaExtBackgroundEffectManagerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the background effect manager
    ///
    /// Informs the server that the client will no longer be using this
    /// protocol object. Existing objects created by this object are not
    /// affected.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
        ]);
        Ok(())
    }

    /// Since when the capabilities message is available.
    #[allow(dead_code)]
    pub const MSG__CAPABILITIES__SINCE: u32 = 1;

    /// capabilities of the compositor
    ///
    /// # Arguments
    ///
    /// - `flags`:
    #[inline]
    pub fn send_capabilities(
        &self,
        flags: MetaExtBackgroundEffectManagerV1Capability,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            flags,
        );
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the get_background_effect message is available.
    #[allow(dead_code)]
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
    pub fn send_get_background_effect(
        &self,
        id: &Rc<MetaExtBackgroundEffectSurfaceV1>,
        surface: &Rc<MetaWlSurface>,
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
            return Err(ObjectError);
        };
        let arg1 = match arg1.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
        ]);
        Ok(())
    }
}

/// A message handler for [ExtBackgroundEffectManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaExtBackgroundEffectManagerV1MessageHandler {
    /// destroy the background effect manager
    ///
    /// Informs the server that the client will no longer be using this
    /// protocol object. Existing objects created by this object are not
    /// affected.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaExtBackgroundEffectManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_background_effect_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// capabilities of the compositor
    ///
    /// # Arguments
    ///
    /// - `flags`:
    #[inline]
    fn capabilities(
        &mut self,
        _slf: &Rc<MetaExtBackgroundEffectManagerV1>,
        flags: MetaExtBackgroundEffectManagerV1Capability,
    ) {
        let res = _slf.send_capabilities(
            flags,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_background_effect_manager_v1.capabilities message: {}", Report::new(e));
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
    fn get_background_effect(
        &mut self,
        _slf: &Rc<MetaExtBackgroundEffectManagerV1>,
        id: &Rc<MetaExtBackgroundEffectSurfaceV1>,
        surface: &Rc<MetaWlSurface>,
    ) {
        let res = _slf.send_get_background_effect(
            id,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_background_effect_manager_v1.get_background_effect message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaExtBackgroundEffectManagerV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaExtBackgroundEffectSurfaceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_background_effect(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_background_effect(&self, arg0, arg1);
                }
            }
            _ => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = MetaExtBackgroundEffectManagerV1Capability(arg0);
                if let Some(handler) = handler {
                    (**handler).capabilities(&self, arg0);
                } else {
                    DefaultMessageHandler.capabilities(&self, arg0);
                }
            }
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }
}

impl MetaExtBackgroundEffectManagerV1 {
    /// Since when the error.background_effect_exists enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_BACKGROUND_EFFECT_EXISTS__SINCE: u32 = 1;

    /// Since when the capability.blur enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CAPABILITY_BLUR__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaExtBackgroundEffectManagerV1Error(pub u32);

impl MetaExtBackgroundEffectManagerV1Error {
    /// the surface already has a background effect object
    #[allow(dead_code)]
    pub const BACKGROUND_EFFECT_EXISTS: Self = Self(0);
}

impl Debug for MetaExtBackgroundEffectManagerV1Error {
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
#[allow(dead_code)]
pub struct MetaExtBackgroundEffectManagerV1Capability(pub u32);

/// An iterator over the set bits in a [MetaExtBackgroundEffectManagerV1Capability].
///
/// You can construct this with the `IntoIterator` implementation of `MetaExtBackgroundEffectManagerV1Capability`.
#[derive(Clone, Debug)]
pub struct MetaExtBackgroundEffectManagerV1CapabilityIter(pub u32);

impl MetaExtBackgroundEffectManagerV1Capability {
    /// the compositor supports applying blur
    #[allow(dead_code)]
    pub const BLUR: Self = Self(1);
}

#[allow(dead_code)]
impl MetaExtBackgroundEffectManagerV1Capability {
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

impl Iterator for MetaExtBackgroundEffectManagerV1CapabilityIter {
    type Item = MetaExtBackgroundEffectManagerV1Capability;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(MetaExtBackgroundEffectManagerV1Capability(bit))
    }
}

impl IntoIterator for MetaExtBackgroundEffectManagerV1Capability {
    type Item = MetaExtBackgroundEffectManagerV1Capability;
    type IntoIter = MetaExtBackgroundEffectManagerV1CapabilityIter;

    fn into_iter(self) -> Self::IntoIter {
        MetaExtBackgroundEffectManagerV1CapabilityIter(self.0)
    }
}

impl BitAnd for MetaExtBackgroundEffectManagerV1Capability {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for MetaExtBackgroundEffectManagerV1Capability {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for MetaExtBackgroundEffectManagerV1Capability {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for MetaExtBackgroundEffectManagerV1Capability {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for MetaExtBackgroundEffectManagerV1Capability {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for MetaExtBackgroundEffectManagerV1Capability {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for MetaExtBackgroundEffectManagerV1Capability {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for MetaExtBackgroundEffectManagerV1Capability {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for MetaExtBackgroundEffectManagerV1Capability {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for MetaExtBackgroundEffectManagerV1Capability {
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
