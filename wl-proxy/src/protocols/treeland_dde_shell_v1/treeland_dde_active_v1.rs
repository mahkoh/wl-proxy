//! metadata interface
//!
//! An interface used to monitor special events.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A treeland_dde_active_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandDdeActiveV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn TreelandDdeActiveV1Handler>,
}

struct DefaultHandler;

impl TreelandDdeActiveV1Handler for DefaultHandler { }

impl TreelandDdeActiveV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::TreelandDdeActiveV1;
    pub const INTERFACE_NAME: &str = "treeland_dde_active_v1";
}

impl TreelandDdeActiveV1 {
    pub fn set_handler(&self, handler: impl TreelandDdeActiveV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandDdeActiveV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandDdeActiveV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandDdeActiveV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandDdeActiveV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the treeland_dde_active_v1 object
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_dde_active_v1#{}.destroy()\n", id);
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

    /// Since when the active_in message is available.
    pub const MSG__ACTIVE_IN__SINCE: u32 = 1;

    /// activeIn event
    ///
    /// # Arguments
    ///
    /// - `reason`:
    #[inline]
    pub fn send_active_in(
        &self,
        reason: TreelandDdeActiveV1Reason,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            reason,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_dde_active_v1#{}.active_in(reason: {:?})\n", client.endpoint.id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the active_out message is available.
    pub const MSG__ACTIVE_OUT__SINCE: u32 = 1;

    /// activeOut event
    ///
    /// # Arguments
    ///
    /// - `reason`:
    #[inline]
    pub fn send_active_out(
        &self,
        reason: TreelandDdeActiveV1Reason,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            reason,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_dde_active_v1#{}.active_out(reason: {:?})\n", client.endpoint.id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the start_drag message is available.
    pub const MSG__START_DRAG__SINCE: u32 = 1;

    /// sent when the compositor starts a drag operation
    #[inline]
    pub fn send_start_drag(
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_dde_active_v1#{}.start_drag()\n", client.endpoint.id, id);
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
            2,
        ]);
        Ok(())
    }

    /// Since when the drop message is available.
    pub const MSG__DROP__SINCE: u32 = 1;

    /// sent when the compositor drop operation
    #[inline]
    pub fn send_drop(
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_dde_active_v1#{}.drop()\n", client.endpoint.id, id);
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
            3,
        ]);
        Ok(())
    }
}

/// A message handler for [TreelandDdeActiveV1] proxies.
pub trait TreelandDdeActiveV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<TreelandDdeActiveV1>) {
        let _ = slf.core.delete_id();
    }

    /// destroy the treeland_dde_active_v1 object
    #[inline]
    fn handle_destroy(
        &mut self,
        _slf: &Rc<TreelandDdeActiveV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_dde_active_v1.destroy message: {}", Report::new(e));
        }
    }

    /// activeIn event
    ///
    /// # Arguments
    ///
    /// - `reason`:
    #[inline]
    fn handle_active_in(
        &mut self,
        _slf: &Rc<TreelandDdeActiveV1>,
        reason: TreelandDdeActiveV1Reason,
    ) {
        if !_slf.core.forward_to_client.get() {
            return;
        }
        let res = _slf.send_active_in(
            reason,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_dde_active_v1.active_in message: {}", Report::new(e));
        }
    }

    /// activeOut event
    ///
    /// # Arguments
    ///
    /// - `reason`:
    #[inline]
    fn handle_active_out(
        &mut self,
        _slf: &Rc<TreelandDdeActiveV1>,
        reason: TreelandDdeActiveV1Reason,
    ) {
        if !_slf.core.forward_to_client.get() {
            return;
        }
        let res = _slf.send_active_out(
            reason,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_dde_active_v1.active_out message: {}", Report::new(e));
        }
    }

    /// sent when the compositor starts a drag operation
    #[inline]
    fn handle_start_drag(
        &mut self,
        _slf: &Rc<TreelandDdeActiveV1>,
    ) {
        if !_slf.core.forward_to_client.get() {
            return;
        }
        let res = _slf.send_start_drag(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_dde_active_v1.start_drag message: {}", Report::new(e));
        }
    }

    /// sent when the compositor drop operation
    #[inline]
    fn handle_drop(
        &mut self,
        _slf: &Rc<TreelandDdeActiveV1>,
    ) {
        if !_slf.core.forward_to_client.get() {
            return;
        }
        let res = _slf.send_drop(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_dde_active_v1.drop message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for TreelandDdeActiveV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::TreelandDdeActiveV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_dde_active_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
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
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = TreelandDdeActiveV1Reason(arg0);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_dde_active_v1#{}.active_in(reason: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_active_in(&self, arg0);
                } else {
                    DefaultHandler.handle_active_in(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = TreelandDdeActiveV1Reason(arg0);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_dde_active_v1#{}.active_out(reason: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_active_out(&self, arg0);
                } else {
                    DefaultHandler.handle_active_out(&self, arg0);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_dde_active_v1#{}.start_drag()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_start_drag(&self);
                } else {
                    DefaultHandler.handle_start_drag(&self);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_dde_active_v1#{}.drop()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_drop(&self);
                } else {
                    DefaultHandler.handle_drop(&self);
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
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "active_in",
            1 => "active_out",
            2 => "start_drag",
            3 => "drop",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for TreelandDdeActiveV1 {
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

impl TreelandDdeActiveV1 {
    /// Since when the reason.mouse enum variant is available.
    pub const ENM__REASON_MOUSE__SINCE: u32 = 1;
    /// Since when the reason.wheel enum variant is available.
    pub const ENM__REASON_WHEEL__SINCE: u32 = 1;
}

/// event reason
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TreelandDdeActiveV1Reason(pub u32);

impl TreelandDdeActiveV1Reason {
    pub const MOUSE: Self = Self(0);

    pub const WHEEL: Self = Self(1);
}

impl Debug for TreelandDdeActiveV1Reason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::MOUSE => "MOUSE",
            Self::WHEEL => "WHEEL",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
