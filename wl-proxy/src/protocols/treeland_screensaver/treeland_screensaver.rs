//! Simple idle inhibit protocol
//!
//! This object implements a simple idle inhibit protocol.
//!
//! Call inhibit to prevent treeland from entering idle state.
//! Call uninhibit or disconnect from the global to release
//! the inhibit.
//!
//! If the client disconnects from the compositor, the inhibit
//! associated with that client is automatically released.
//!
//! There can be only one inhibit per client per time. Calling
//! inhibit multiple times will raise an error. Call uninhibit
//! before inhibit to update application_name and reason
//! recorded.
//!
//! Warning! The protocol described in this file is currently
//! in the testing phase. Backward compatible changes may be
//! added together with the corresponding interface version
//! bump. Backward incompatible changes can only be done by
//! creating a new major version of the extension.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A treeland_screensaver object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandScreensaver {
    core: ObjectCore,
    handler: HandlerHolder<dyn TreelandScreensaverHandler>,
}

struct DefaultHandler;

impl TreelandScreensaverHandler for DefaultHandler { }

impl ConcreteObject for TreelandScreensaver {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::TreelandScreensaver;
    const INTERFACE_NAME: &str = "treeland_screensaver";
}

impl TreelandScreensaver {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl TreelandScreensaverHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandScreensaverHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandScreensaver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandScreensaver")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandScreensaver {
    /// Since when the inhibit message is available.
    pub const MSG__INHIBIT__SINCE: u32 = 1;

    /// Inhibit idleness
    ///
    /// Inhibit idleness with given application_name and reason_for_inhibit.
    ///
    /// # Arguments
    ///
    /// - `application_name`:
    /// - `reason_for_inhibit`:
    #[inline]
    pub fn try_send_inhibit(
        &self,
        application_name: &str,
        reason_for_inhibit: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            application_name,
            reason_for_inhibit,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str, arg1: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_screensaver#{}.inhibit(application_name: {:?}, reason_for_inhibit: {:?})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1);
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
        fmt.string(arg0);
        fmt.string(arg1);
        Ok(())
    }

    /// Inhibit idleness
    ///
    /// Inhibit idleness with given application_name and reason_for_inhibit.
    ///
    /// # Arguments
    ///
    /// - `application_name`:
    /// - `reason_for_inhibit`:
    #[inline]
    pub fn send_inhibit(
        &self,
        application_name: &str,
        reason_for_inhibit: &str,
    ) {
        let res = self.try_send_inhibit(
            application_name,
            reason_for_inhibit,
        );
        if let Err(e) = res {
            log_send("treeland_screensaver.inhibit", &e);
        }
    }

    /// Since when the uninhibit message is available.
    pub const MSG__UNINHIBIT__SINCE: u32 = 1;

    /// Uninhibit idleness
    ///
    /// Uninhibit idleness previously inhibited by inhibit request.
    #[inline]
    pub fn try_send_uninhibit(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_screensaver#{}.uninhibit()\n", id);
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
        Ok(())
    }

    /// Uninhibit idleness
    ///
    /// Uninhibit idleness previously inhibited by inhibit request.
    #[inline]
    pub fn send_uninhibit(
        &self,
    ) {
        let res = self.try_send_uninhibit(
        );
        if let Err(e) = res {
            log_send("treeland_screensaver.uninhibit", &e);
        }
    }
}

/// A message handler for [`TreelandScreensaver`] proxies.
pub trait TreelandScreensaverHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<TreelandScreensaver>) {
        slf.core.delete_id();
    }

    /// Inhibit idleness
    ///
    /// Inhibit idleness with given application_name and reason_for_inhibit.
    ///
    /// # Arguments
    ///
    /// - `application_name`:
    /// - `reason_for_inhibit`:
    #[inline]
    fn handle_inhibit(
        &mut self,
        _slf: &Rc<TreelandScreensaver>,
        application_name: &str,
        reason_for_inhibit: &str,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_inhibit(
            application_name,
            reason_for_inhibit,
        );
        if let Err(e) = res {
            log_forward("treeland_screensaver.inhibit", &e);
        }
    }

    /// Uninhibit idleness
    ///
    /// Uninhibit idleness previously inhibited by inhibit request.
    #[inline]
    fn handle_uninhibit(
        &mut self,
        _slf: &Rc<TreelandScreensaver>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_uninhibit(
        );
        if let Err(e) = res {
            log_forward("treeland_screensaver.uninhibit", &e);
        }
    }
}

impl ObjectPrivate for TreelandScreensaver {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::TreelandScreensaver, version),
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
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError(ObjectErrorKind::MissingArgument("application_name")));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError(ObjectErrorKind::MissingArgument("application_name")));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError(ObjectErrorKind::NullString("application_name")));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError(ObjectErrorKind::NonUtf8("application_name")));
                        };
                        s
                    }
                };
                let arg1 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError(ObjectErrorKind::MissingArgument("reason_for_inhibit")));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError(ObjectErrorKind::MissingArgument("reason_for_inhibit")));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError(ObjectErrorKind::NullString("reason_for_inhibit")));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError(ObjectErrorKind::NonUtf8("reason_for_inhibit")));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str, arg1: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_screensaver#{}.inhibit(application_name: {:?}, reason_for_inhibit: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_inhibit(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_inhibit(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_screensaver#{}.uninhibit()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_uninhibit(&self);
                } else {
                    DefaultHandler.handle_uninhibit(&self);
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
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "inhibit",
            1 => "uninhibit",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for TreelandScreensaver {
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

impl TreelandScreensaver {
    /// Since when the error.not_yet_inhibited enum variant is available.
    pub const ENM__ERROR_NOT_YET_INHIBITED__SINCE: u32 = 1;
    /// Since when the error.already_inhibited enum variant is available.
    pub const ENM__ERROR_ALREADY_INHIBITED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TreelandScreensaverError(pub u32);

impl TreelandScreensaverError {
    /// Trying to uninhibit but no active inhibit existed
    pub const NOT_YET_INHIBITED: Self = Self(0);

    /// Trying to inhibit with an active inhibit existed
    pub const ALREADY_INHIBITED: Self = Self(1);
}

impl Debug for TreelandScreensaverError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NOT_YET_INHIBITED => "NOT_YET_INHIBITED",
            Self::ALREADY_INHIBITED => "ALREADY_INHIBITED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
