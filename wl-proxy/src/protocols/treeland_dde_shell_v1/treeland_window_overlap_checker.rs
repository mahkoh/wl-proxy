//! An opened toplevel
//!
//! A treeland_dde_shell_handle_v1 object represents an opened toplevel window. Each
//! app may have multiple opened toplevels.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A treeland_window_overlap_checker object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandWindowOverlapChecker {
    core: ObjectCore,
    handler: HandlerHolder<dyn TreelandWindowOverlapCheckerHandler>,
}

struct DefaultHandler;

impl TreelandWindowOverlapCheckerHandler for DefaultHandler { }

impl ConcreteObject for TreelandWindowOverlapChecker {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::TreelandWindowOverlapChecker;
    const INTERFACE_NAME: &str = "treeland_window_overlap_checker";
}

impl TreelandWindowOverlapChecker {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl TreelandWindowOverlapCheckerHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandWindowOverlapCheckerHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandWindowOverlapChecker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandWindowOverlapChecker")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandWindowOverlapChecker {
    /// Since when the enter message is available.
    pub const MSG__ENTER__SINCE: u32 = 1;

    /// Window has overlapped
    ///
    /// This event is sent when windows overlapped.
    /// This event is sent only once.
    #[inline]
    pub fn try_send_enter(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_window_overlap_checker#{}.enter()\n", client_id, id);
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

    /// Window has overlapped
    ///
    /// This event is sent when windows overlapped.
    /// This event is sent only once.
    #[inline]
    pub fn send_enter(
        &self,
    ) {
        let res = self.try_send_enter(
        );
        if let Err(e) = res {
            log_send("treeland_window_overlap_checker.enter", &e);
        }
    }

    /// Since when the leave message is available.
    pub const MSG__LEAVE__SINCE: u32 = 1;

    /// Window not has overlapped
    ///
    /// This event is sent when windows not overlapped.
    /// This event is sent only once.
    #[inline]
    pub fn try_send_leave(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_window_overlap_checker#{}.leave()\n", client_id, id);
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

    /// Window not has overlapped
    ///
    /// This event is sent when windows not overlapped.
    /// This event is sent only once.
    #[inline]
    pub fn send_leave(
        &self,
    ) {
        let res = self.try_send_leave(
        );
        if let Err(e) = res {
            log_send("treeland_window_overlap_checker.leave", &e);
        }
    }

    /// Since when the update message is available.
    pub const MSG__UPDATE__SINCE: u32 = 1;

    /// Register the detected surface
    ///
    /// This interface is used to receive the detected surface.
    /// When the xdgshell window in the workspace overlaps with the detected window,
    /// an event will be sent to notify the client to process it.
    /// The window position will only be recorded when this interface is called.
    /// If the window moves, this interface needs to be called again.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    /// - `anchor`:
    /// - `output`:
    #[inline]
    pub fn try_send_update(
        &self,
        width: i32,
        height: i32,
        anchor: TreelandWindowOverlapCheckerAnchor,
        output: &Rc<WlOutput>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            width,
            height,
            anchor,
            output,
        );
        let arg3 = arg3.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg3_id = match arg3.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("output"))),
            Some(id) => id,
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: i32, arg2: TreelandWindowOverlapCheckerAnchor, arg3: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_window_overlap_checker#{}.update(width: {}, height: {}, anchor: {:?}, output: wl_output#{})\n", id, arg0, arg1, arg2, arg3);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2, arg3_id);
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
            arg0 as u32,
            arg1 as u32,
            arg2.0,
            arg3_id,
        ]);
        Ok(())
    }

    /// Register the detected surface
    ///
    /// This interface is used to receive the detected surface.
    /// When the xdgshell window in the workspace overlaps with the detected window,
    /// an event will be sent to notify the client to process it.
    /// The window position will only be recorded when this interface is called.
    /// If the window moves, this interface needs to be called again.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    /// - `anchor`:
    /// - `output`:
    #[inline]
    pub fn send_update(
        &self,
        width: i32,
        height: i32,
        anchor: TreelandWindowOverlapCheckerAnchor,
        output: &Rc<WlOutput>,
    ) {
        let res = self.try_send_update(
            width,
            height,
            anchor,
            output,
        );
        if let Err(e) = res {
            log_send("treeland_window_overlap_checker.update", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the treeland_window_overlap_checker object
    ///
    /// Destroys the treeland_window_overlap_checker object.
    ///
    /// This request should be called either when the client does not want to
    /// use the toplevel anymore or after the closed event to finalize the
    /// destruction of the object.
    #[inline]
    pub fn try_send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_window_overlap_checker#{}.destroy()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
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
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the treeland_window_overlap_checker object
    ///
    /// Destroys the treeland_window_overlap_checker object.
    ///
    /// This request should be called either when the client does not want to
    /// use the toplevel anymore or after the closed event to finalize the
    /// destruction of the object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("treeland_window_overlap_checker.destroy", &e);
        }
    }
}

/// A message handler for [`TreelandWindowOverlapChecker`] proxies.
pub trait TreelandWindowOverlapCheckerHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<TreelandWindowOverlapChecker>) {
        slf.core.delete_id();
    }

    /// Window has overlapped
    ///
    /// This event is sent when windows overlapped.
    /// This event is sent only once.
    #[inline]
    fn handle_enter(
        &mut self,
        _slf: &Rc<TreelandWindowOverlapChecker>,
    ) {
        if !_slf.core.forward_to_client.get() {
            return;
        }
        let res = _slf.try_send_enter(
        );
        if let Err(e) = res {
            log_forward("treeland_window_overlap_checker.enter", &e);
        }
    }

    /// Window not has overlapped
    ///
    /// This event is sent when windows not overlapped.
    /// This event is sent only once.
    #[inline]
    fn handle_leave(
        &mut self,
        _slf: &Rc<TreelandWindowOverlapChecker>,
    ) {
        if !_slf.core.forward_to_client.get() {
            return;
        }
        let res = _slf.try_send_leave(
        );
        if let Err(e) = res {
            log_forward("treeland_window_overlap_checker.leave", &e);
        }
    }

    /// Register the detected surface
    ///
    /// This interface is used to receive the detected surface.
    /// When the xdgshell window in the workspace overlaps with the detected window,
    /// an event will be sent to notify the client to process it.
    /// The window position will only be recorded when this interface is called.
    /// If the window moves, this interface needs to be called again.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    /// - `anchor`:
    /// - `output`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_update(
        &mut self,
        _slf: &Rc<TreelandWindowOverlapChecker>,
        width: i32,
        height: i32,
        anchor: TreelandWindowOverlapCheckerAnchor,
        output: &Rc<WlOutput>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_update(
            width,
            height,
            anchor,
            output,
        );
        if let Err(e) = res {
            log_forward("treeland_window_overlap_checker.update", &e);
        }
    }

    /// destroy the treeland_window_overlap_checker object
    ///
    /// Destroys the treeland_window_overlap_checker object.
    ///
    /// This request should be called either when the client does not want to
    /// use the toplevel anymore or after the closed event to finalize the
    /// destruction of the object.
    #[inline]
    fn handle_destroy(
        &mut self,
        _slf: &Rc<TreelandWindowOverlapChecker>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("treeland_window_overlap_checker.destroy", &e);
        }
    }
}

impl ObjectPrivate for TreelandWindowOverlapChecker {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::TreelandWindowOverlapChecker, version),
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
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 24)));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = TreelandWindowOverlapCheckerAnchor(arg2);
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32, arg2: TreelandWindowOverlapCheckerAnchor, arg3: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_window_overlap_checker#{}.update(width: {}, height: {}, anchor: {:?}, output: wl_output#{})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                let arg3_id = arg3;
                let Some(arg3) = client.endpoint.lookup(arg3_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg3_id)));
                };
                let Ok(arg3) = (arg3 as Rc<dyn Any>).downcast::<WlOutput>() else {
                    let o = client.endpoint.lookup(arg3_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("output", o.core().interface, ObjectInterface::WlOutput)));
                };
                let arg3 = &arg3;
                if let Some(handler) = handler {
                    (**handler).handle_update(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_update(&self, arg0, arg1, arg2, arg3);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_window_overlap_checker#{}.destroy()\n", client_id, id);
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

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_window_overlap_checker#{}.enter()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_enter(&self);
                } else {
                    DefaultHandler.handle_enter(&self);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_window_overlap_checker#{}.leave()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_leave(&self);
                } else {
                    DefaultHandler.handle_leave(&self);
                }
            }
            n => {
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
            0 => "update",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "enter",
            1 => "leave",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for TreelandWindowOverlapChecker {
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

impl TreelandWindowOverlapChecker {
    /// Since when the anchor.top enum variant is available.
    pub const ENM__ANCHOR_TOP__SINCE: u32 = 1;
    /// Since when the anchor.bottom enum variant is available.
    pub const ENM__ANCHOR_BOTTOM__SINCE: u32 = 1;
    /// Since when the anchor.left enum variant is available.
    pub const ENM__ANCHOR_LEFT__SINCE: u32 = 1;
    /// Since when the anchor.right enum variant is available.
    pub const ENM__ANCHOR_RIGHT__SINCE: u32 = 1;
}

/// same layershell
///
/// same layershell
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct TreelandWindowOverlapCheckerAnchor(pub u32);

/// An iterator over the set bits in a [`TreelandWindowOverlapCheckerAnchor`].
///
/// You can construct this with the `IntoIterator` implementation of `TreelandWindowOverlapCheckerAnchor`.
#[derive(Clone, Debug)]
pub struct TreelandWindowOverlapCheckerAnchorIter(pub u32);

impl TreelandWindowOverlapCheckerAnchor {
    /// the top edge of the anchor rectangle
    pub const TOP: Self = Self(1);

    /// the bottom edge of the anchor rectangle
    pub const BOTTOM: Self = Self(2);

    /// the left edge of the anchor rectangle
    pub const LEFT: Self = Self(4);

    /// the right edge of the anchor rectangle
    pub const RIGHT: Self = Self(8);
}

impl TreelandWindowOverlapCheckerAnchor {
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
        Self(0 | 1 | 2 | 4 | 8)
    }
}

impl Iterator for TreelandWindowOverlapCheckerAnchorIter {
    type Item = TreelandWindowOverlapCheckerAnchor;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(TreelandWindowOverlapCheckerAnchor(bit))
    }
}

impl IntoIterator for TreelandWindowOverlapCheckerAnchor {
    type Item = TreelandWindowOverlapCheckerAnchor;
    type IntoIter = TreelandWindowOverlapCheckerAnchorIter;

    fn into_iter(self) -> Self::IntoIter {
        TreelandWindowOverlapCheckerAnchorIter(self.0)
    }
}

impl BitAnd for TreelandWindowOverlapCheckerAnchor {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for TreelandWindowOverlapCheckerAnchor {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for TreelandWindowOverlapCheckerAnchor {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for TreelandWindowOverlapCheckerAnchor {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for TreelandWindowOverlapCheckerAnchor {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for TreelandWindowOverlapCheckerAnchor {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for TreelandWindowOverlapCheckerAnchor {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for TreelandWindowOverlapCheckerAnchor {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for TreelandWindowOverlapCheckerAnchor {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for TreelandWindowOverlapCheckerAnchor {
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
            f.write_str("TOP")?;
        }
        if v & 2 == 2 {
            v &= !2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("BOTTOM")?;
        }
        if v & 4 == 4 {
            v &= !4;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("LEFT")?;
        }
        if v & 8 == 8 {
            v &= !8;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("RIGHT")?;
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
