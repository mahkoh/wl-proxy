//! inhibits input events to other clients
//!
//! Clients can use this interface to prevent input events from being sent to
//! any surfaces but its own, which is useful for example in lock screen
//! software. It is assumed that access to this interface will be locked down
//! to whitelisted clients by the compositor.
//!
//! Note! This protocol is deprecated and not intended for production use.
//! For screen lockers, use the ext-session-lock-v1 protocol.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwlr_input_inhibit_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwlrInputInhibitManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwlrInputInhibitManagerV1Handler>,
}

struct DefaultHandler;

impl ZwlrInputInhibitManagerV1Handler for DefaultHandler { }

impl ZwlrInputInhibitManagerV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::ZwlrInputInhibitManagerV1;
    pub const INTERFACE_NAME: &str = "zwlr_input_inhibit_manager_v1";
}

impl ZwlrInputInhibitManagerV1 {
    pub fn set_handler(&self, handler: impl ZwlrInputInhibitManagerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZwlrInputInhibitManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwlrInputInhibitManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwlrInputInhibitManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwlrInputInhibitManagerV1 {
    /// Since when the get_inhibitor message is available.
    pub const MSG__GET_INHIBITOR__SINCE: u32 = 1;

    /// inhibit input to other clients
    ///
    /// Activates the input inhibitor. As long as the inhibitor is active, the
    /// compositor will not send input events to other clients.
    #[inline]
    pub fn send_get_inhibitor(
        &self,
        id: &Rc<ZwlrInputInhibitorV1>,
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_input_inhibit_manager_v1#{}.get_inhibitor(id: zwlr_input_inhibitor_v1#{})\n", id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwlrInputInhibitManagerV1] proxies.
pub trait ZwlrInputInhibitManagerV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwlrInputInhibitManagerV1>) {
        let _ = slf.core.delete_id();
    }

    /// inhibit input to other clients
    ///
    /// Activates the input inhibitor. As long as the inhibitor is active, the
    /// compositor will not send input events to other clients.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn handle_get_inhibitor(
        &mut self,
        _slf: &Rc<ZwlrInputInhibitManagerV1>,
        id: &Rc<ZwlrInputInhibitorV1>,
    ) {
        let res = _slf.send_get_inhibitor(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_input_inhibit_manager_v1.get_inhibitor message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for ZwlrInputInhibitManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwlrInputInhibitManagerV1, version),
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_input_inhibit_manager_v1#{}.get_inhibitor(id: zwlr_input_inhibitor_v1#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ZwlrInputInhibitorV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_get_inhibitor(&self, arg0);
                } else {
                    DefaultHandler.handle_get_inhibitor(&self, arg0);
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
            0 => "get_inhibitor",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for ZwlrInputInhibitManagerV1 {
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

impl ZwlrInputInhibitManagerV1 {
    /// Since when the error.already_inhibited enum variant is available.
    pub const ENM__ERROR_ALREADY_INHIBITED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwlrInputInhibitManagerV1Error(pub u32);

impl ZwlrInputInhibitManagerV1Error {
    /// an input inhibitor is already in use on the compositor
    pub const ALREADY_INHIBITED: Self = Self(0);
}

impl Debug for ZwlrInputInhibitManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ALREADY_INHIBITED => "ALREADY_INHIBITED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
