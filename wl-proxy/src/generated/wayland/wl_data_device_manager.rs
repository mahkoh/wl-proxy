//! data transfer interface
//!
//! The wl_data_device_manager is a singleton global object that
//! provides access to inter-client data transfer mechanisms such as
//! copy-and-paste and drag-and-drop.  These mechanisms are tied to
//! a wl_seat and this interface lets a client get a wl_data_device
//! corresponding to a wl_seat.
//!
//! Depending on the version bound, the objects created from the bound
//! wl_data_device_manager object will have different requirements for
//! functioning properly. See wl_data_source.set_actions,
//! wl_data_offer.accept and wl_data_offer.finish for details.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wl_data_device_manager proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWlDataDeviceManager {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWlDataDeviceManagerMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWlDataDeviceManagerMessageHandler for DefaultMessageHandler { }

impl MetaWlDataDeviceManager {
    pub const XML_VERSION: u32 = 3;
}

impl MetaWlDataDeviceManager {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WlDataDeviceManager, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWlDataDeviceManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWlDataDeviceManager")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWlDataDeviceManager {
    /// Since when the create_data_source message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE_DATA_SOURCE__SINCE: u32 = 1;

    /// create a new data source
    ///
    /// Create a new data source.
    #[inline]
    pub fn send_create_data_source(
        &self,
        id: &Rc<MetaWlDataSource>,
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
            return Err(ObjectError);
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0.server_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }

    /// Since when the get_data_device message is available.
    #[allow(dead_code)]
    pub const MSG__GET_DATA_DEVICE__SINCE: u32 = 1;

    /// create a new data device
    ///
    /// Create a new data device for a given seat.
    ///
    /// # Arguments
    ///
    /// - `id`: data device to create
    /// - `seat`: seat associated with the data device
    #[inline]
    pub fn send_get_data_device(
        &self,
        id: &Rc<MetaWlDataDevice>,
        seat: &Rc<MetaWlSeat>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            seat,
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
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
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

/// A message handler for [WlDataDeviceManager] proxies.
#[allow(dead_code)]
pub trait MetaWlDataDeviceManagerMessageHandler {
    /// create a new data source
    ///
    /// Create a new data source.
    ///
    /// # Arguments
    ///
    /// - `id`: data source to create
    #[inline]
    fn create_data_source(
        &mut self,
        _slf: &Rc<MetaWlDataDeviceManager>,
        id: &Rc<MetaWlDataSource>,
    ) {
        let res = _slf.send_create_data_source(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_device_manager.create_data_source message: {}", Report::new(e));
        }
    }

    /// create a new data device
    ///
    /// Create a new data device for a given seat.
    ///
    /// # Arguments
    ///
    /// - `id`: data device to create
    /// - `seat`: seat associated with the data device
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_data_device(
        &mut self,
        _slf: &Rc<MetaWlDataDeviceManager>,
        id: &Rc<MetaWlDataDevice>,
        seat: &Rc<MetaWlSeat>,
    ) {
        let res = _slf.send_get_data_device(
            id,
            seat,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_device_manager.get_data_device message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWlDataDeviceManager {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaWlDataSource::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).create_data_source(&self, arg0);
                } else {
                    DefaultMessageHandler.create_data_source(&self, arg0);
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
                let arg0 = MetaWlDataDevice::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.endpoint.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSeat>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_data_device(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_data_device(&self, arg0, arg1);
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
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
    }
}

impl MetaWlDataDeviceManager {
    /// Since when the dnd_action.none enum variant is available.
    #[allow(dead_code)]
    pub const ENM__DND_ACTION_NONE__SINCE: u32 = 1;
    /// Since when the dnd_action.copy enum variant is available.
    #[allow(dead_code)]
    pub const ENM__DND_ACTION_COPY__SINCE: u32 = 1;
    /// Since when the dnd_action.move enum variant is available.
    #[allow(dead_code)]
    pub const ENM__DND_ACTION_MOVE__SINCE: u32 = 1;
    /// Since when the dnd_action.ask enum variant is available.
    #[allow(dead_code)]
    pub const ENM__DND_ACTION_ASK__SINCE: u32 = 1;
}

/// drag and drop actions
///
/// This is a bitmask of the available/preferred actions in a
/// drag-and-drop operation.
///
/// In the compositor, the selected action is a result of matching the
/// actions offered by the source and destination sides.  "action" events
/// with a "none" action will be sent to both source and destination if
/// there is no match. All further checks will effectively happen on
/// (source actions ∩ destination actions).
///
/// In addition, compositors may also pick different actions in
/// reaction to key modifiers being pressed. One common design that
/// is used in major toolkits (and the behavior recommended for
/// compositors) is:
///
/// - If no modifiers are pressed, the first match (in bit order)
///   will be used.
/// - Pressing Shift selects "move", if enabled in the mask.
/// - Pressing Control selects "copy", if enabled in the mask.
///
/// Behavior beyond that is considered implementation-dependent.
/// Compositors may for example bind other modifiers (like Alt/Meta)
/// or drags initiated with other buttons than BTN_LEFT to specific
/// actions (e.g. "ask").
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
#[allow(dead_code)]
pub struct MetaWlDataDeviceManagerDndAction(pub u32);

/// An iterator over the set bits in a [MetaWlDataDeviceManagerDndAction].
///
/// You can construct this with the `IntoIterator` implementation of `MetaWlDataDeviceManagerDndAction`.
#[derive(Clone, Debug)]
pub struct MetaWlDataDeviceManagerDndActionIter(pub u32);

impl MetaWlDataDeviceManagerDndAction {
    /// no action
    #[allow(dead_code)]
    pub const NONE: Self = Self(0);

    /// copy action
    #[allow(dead_code)]
    pub const COPY: Self = Self(1);

    /// move action
    #[allow(dead_code)]
    pub const MOVE: Self = Self(2);

    /// ask action
    #[allow(dead_code)]
    pub const ASK: Self = Self(4);
}

#[allow(dead_code)]
impl MetaWlDataDeviceManagerDndAction {
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
        Self(0 | 0 | 1 | 2 | 4)
    }
}

impl Iterator for MetaWlDataDeviceManagerDndActionIter {
    type Item = MetaWlDataDeviceManagerDndAction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(MetaWlDataDeviceManagerDndAction(bit))
    }
}

impl IntoIterator for MetaWlDataDeviceManagerDndAction {
    type Item = MetaWlDataDeviceManagerDndAction;
    type IntoIter = MetaWlDataDeviceManagerDndActionIter;

    fn into_iter(self) -> Self::IntoIter {
        MetaWlDataDeviceManagerDndActionIter(self.0)
    }
}

impl BitAnd for MetaWlDataDeviceManagerDndAction {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for MetaWlDataDeviceManagerDndAction {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for MetaWlDataDeviceManagerDndAction {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for MetaWlDataDeviceManagerDndAction {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for MetaWlDataDeviceManagerDndAction {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for MetaWlDataDeviceManagerDndAction {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for MetaWlDataDeviceManagerDndAction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for MetaWlDataDeviceManagerDndAction {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for MetaWlDataDeviceManagerDndAction {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for MetaWlDataDeviceManagerDndAction {
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
            f.write_str("COPY")?;
        }
        if v & 2 == 2 {
            v &= !2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("MOVE")?;
        }
        if v & 4 == 4 {
            v &= !4;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("ASK")?;
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
            f.write_str("NONE")?;
        }
        Ok(())
    }
}
