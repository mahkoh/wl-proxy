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

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wl_data_device_manager object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlDataDeviceManager {
    core: ObjectCore,
    handler: HandlerHolder<dyn WlDataDeviceManagerHandler>,
}

struct DefaultHandler;

impl WlDataDeviceManagerHandler for DefaultHandler { }

impl ConcreteObject for WlDataDeviceManager {
    const XML_VERSION: u32 = 3;
    const INTERFACE: ObjectInterface = ObjectInterface::WlDataDeviceManager;
    const INTERFACE_NAME: &str = "wl_data_device_manager";
}

impl WlDataDeviceManager {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WlDataDeviceManagerHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WlDataDeviceManagerHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlDataDeviceManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlDataDeviceManager")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlDataDeviceManager {
    /// Since when the create_data_source message is available.
    pub const MSG__CREATE_DATA_SOURCE__SINCE: u32 = 1;

    /// create a new data source
    ///
    /// Create a new data source.
    ///
    /// # Arguments
    ///
    /// - `id`: data source to create
    #[inline]
    pub fn try_send_create_data_source(
        &self,
        id: &Rc<WlDataSource>,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_data_device_manager#{}.create_data_source(id: wl_data_source#{})\n", id, arg0);
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

    /// create a new data source
    ///
    /// Create a new data source.
    ///
    /// # Arguments
    ///
    /// - `id`: data source to create
    #[inline]
    pub fn send_create_data_source(
        &self,
        id: &Rc<WlDataSource>,
    ) {
        let res = self.try_send_create_data_source(
            id,
        );
        if let Err(e) = res {
            log_send("wl_data_device_manager.create_data_source", &e);
        }
    }

    /// create a new data source
    ///
    /// Create a new data source.
    #[inline]
    pub fn new_try_send_create_data_source(
        &self,
    ) -> Result<Rc<WlDataSource>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_create_data_source(
            &id,
        )?;
        Ok(id)
    }

    /// create a new data source
    ///
    /// Create a new data source.
    #[inline]
    pub fn new_send_create_data_source(
        &self,
    ) -> Rc<WlDataSource> {
        let id = self.core.create_child();
        self.send_create_data_source(
            &id,
        );
        id
    }

    /// Since when the get_data_device message is available.
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
    pub fn try_send_get_data_device(
        &self,
        id: &Rc<WlDataDevice>,
        seat: &Rc<WlSeat>,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("seat"))),
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_data_device_manager#{}.get_data_device(id: wl_data_device#{}, seat: wl_seat#{})\n", id, arg0, arg1);
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
        id: &Rc<WlDataDevice>,
        seat: &Rc<WlSeat>,
    ) {
        let res = self.try_send_get_data_device(
            id,
            seat,
        );
        if let Err(e) = res {
            log_send("wl_data_device_manager.get_data_device", &e);
        }
    }

    /// create a new data device
    ///
    /// Create a new data device for a given seat.
    ///
    /// # Arguments
    ///
    /// - `seat`: seat associated with the data device
    #[inline]
    pub fn new_try_send_get_data_device(
        &self,
        seat: &Rc<WlSeat>,
    ) -> Result<Rc<WlDataDevice>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_data_device(
            &id,
            seat,
        )?;
        Ok(id)
    }

    /// create a new data device
    ///
    /// Create a new data device for a given seat.
    ///
    /// # Arguments
    ///
    /// - `seat`: seat associated with the data device
    #[inline]
    pub fn new_send_get_data_device(
        &self,
        seat: &Rc<WlSeat>,
    ) -> Rc<WlDataDevice> {
        let id = self.core.create_child();
        self.send_get_data_device(
            &id,
            seat,
        );
        id
    }
}

/// A message handler for [`WlDataDeviceManager`] proxies.
pub trait WlDataDeviceManagerHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WlDataDeviceManager>) {
        slf.core.delete_id();
    }

    /// create a new data source
    ///
    /// Create a new data source.
    ///
    /// # Arguments
    ///
    /// - `id`: data source to create
    #[inline]
    fn handle_create_data_source(
        &mut self,
        slf: &Rc<WlDataDeviceManager>,
        id: &Rc<WlDataSource>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_data_source(
            id,
        );
        if let Err(e) = res {
            log_forward("wl_data_device_manager.create_data_source", &e);
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
    fn handle_get_data_device(
        &mut self,
        slf: &Rc<WlDataDeviceManager>,
        id: &Rc<WlDataDevice>,
        seat: &Rc<WlSeat>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_data_device(
            id,
            seat,
        );
        if let Err(e) = res {
            log_forward("wl_data_device_manager.get_data_device", &e);
        }
    }
}

impl ObjectPrivate for WlDataDeviceManager {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WlDataDeviceManager, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_data_device_manager#{}.create_data_source(id: wl_data_source#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WlDataSource::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_data_source(&self, arg0);
                } else {
                    DefaultHandler.handle_create_data_source(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_data_device_manager#{}.get_data_device(id: wl_data_device#{}, seat: wl_seat#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = WlDataDevice::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("seat", o.core().interface, ObjectInterface::WlSeat)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_data_device(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_data_device(&self, arg0, arg1);
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
            0 => "create_data_source",
            1 => "get_data_device",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WlDataDeviceManager {
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

impl WlDataDeviceManager {
    /// Since when the dnd_action.none enum variant is available.
    pub const ENM__DND_ACTION_NONE__SINCE: u32 = 1;
    /// Since when the dnd_action.copy enum variant is available.
    pub const ENM__DND_ACTION_COPY__SINCE: u32 = 1;
    /// Since when the dnd_action.move enum variant is available.
    pub const ENM__DND_ACTION_MOVE__SINCE: u32 = 1;
    /// Since when the dnd_action.ask enum variant is available.
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
pub struct WlDataDeviceManagerDndAction(pub u32);

/// An iterator over the set bits in a [`WlDataDeviceManagerDndAction`].
///
/// You can construct this with the `IntoIterator` implementation of `WlDataDeviceManagerDndAction`.
#[derive(Clone, Debug)]
pub struct WlDataDeviceManagerDndActionIter(pub u32);

impl WlDataDeviceManagerDndAction {
    /// no action
    pub const NONE: Self = Self(0);

    /// copy action
    pub const COPY: Self = Self(1);

    /// move action
    pub const MOVE: Self = Self(2);

    /// ask action
    pub const ASK: Self = Self(4);
}

impl WlDataDeviceManagerDndAction {
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

impl Iterator for WlDataDeviceManagerDndActionIter {
    type Item = WlDataDeviceManagerDndAction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(WlDataDeviceManagerDndAction(bit))
    }
}

impl IntoIterator for WlDataDeviceManagerDndAction {
    type Item = WlDataDeviceManagerDndAction;
    type IntoIter = WlDataDeviceManagerDndActionIter;

    fn into_iter(self) -> Self::IntoIter {
        WlDataDeviceManagerDndActionIter(self.0)
    }
}

impl BitAnd for WlDataDeviceManagerDndAction {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for WlDataDeviceManagerDndAction {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for WlDataDeviceManagerDndAction {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for WlDataDeviceManagerDndAction {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for WlDataDeviceManagerDndAction {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for WlDataDeviceManagerDndAction {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for WlDataDeviceManagerDndAction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for WlDataDeviceManagerDndAction {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for WlDataDeviceManagerDndAction {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for WlDataDeviceManagerDndAction {
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
