//! screen output object
//!
//! A treeland_virtual_output_v1 represents a set virtual screen output object.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A treeland_virtual_output_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandVirtualOutputV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn TreelandVirtualOutputV1Handler>,
}

struct DefaultHandler;

impl TreelandVirtualOutputV1Handler for DefaultHandler { }

impl TreelandVirtualOutputV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ProxyInterface = ProxyInterface::TreelandVirtualOutputV1;
    pub const INTERFACE_NAME: &str = "treeland_virtual_output_v1";
}

impl TreelandVirtualOutputV1 {
    pub fn set_handler(&self, handler: impl TreelandVirtualOutputV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandVirtualOutputV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandVirtualOutputV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandVirtualOutputV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandVirtualOutputV1 {
    /// Since when the outputs message is available.
    pub const MSG__OUTPUTS__SINCE: u32 = 1;

    /// screen output changes
    ///
    /// This event is sent to the client when any screen in the array changes.
    ///
    /// The element of the array is the name of the screen.
    ///
    /// The first element of the array outputs is the screen to be copied, and
    /// the subsequent elements are the screens to be mirrored.
    ///
    /// When the primary screen (the screen being copied) is removed, a successor
    /// is selected from the queue as the primary screen, and the queue information
    /// is updated.
    ///
    /// # Arguments
    ///
    /// - `name`: The name of the user readable virtual output
    /// - `outputs`: Screen name array
    #[inline]
    pub fn send_outputs(
        &self,
        name: &str,
        outputs: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            name,
            outputs,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_virtual_output_v1#{}.outputs(name: {:?}, outputs: {})\n", client.endpoint.id, id, arg0, debug_array(arg1));
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
        fmt.string(arg0);
        fmt.array(arg1);
        Ok(())
    }

    /// Since when the error message is available.
    pub const MSG__ERROR__SINCE: u32 = 1;

    /// Screen copy mode error event
    ///
    /// When an error occurs, an error event is emitted, terminating the replication
    /// mode request issued by the client.
    ///
    /// # Arguments
    ///
    /// - `code`: error code
    /// - `message`: error description
    #[inline]
    pub fn send_error(
        &self,
        code: u32,
        message: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            code,
            message,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_virtual_output_v1#{}.error(code: {}, message: {:?})\n", client.endpoint.id, id, arg0, arg1);
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
            arg0,
        ]);
        fmt.string(arg1);
        Ok(())
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the output
    ///
    /// Destroy the output.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_virtual_output_v1#{}.destroy()\n", id);
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
}

/// A message handler for [TreelandVirtualOutputV1] proxies.
pub trait TreelandVirtualOutputV1Handler: Any {
    /// screen output changes
    ///
    /// This event is sent to the client when any screen in the array changes.
    ///
    /// The element of the array is the name of the screen.
    ///
    /// The first element of the array outputs is the screen to be copied, and
    /// the subsequent elements are the screens to be mirrored.
    ///
    /// When the primary screen (the screen being copied) is removed, a successor
    /// is selected from the queue as the primary screen, and the queue information
    /// is updated.
    ///
    /// # Arguments
    ///
    /// - `name`: The name of the user readable virtual output
    /// - `outputs`: Screen name array
    #[inline]
    fn outputs(
        &mut self,
        _slf: &Rc<TreelandVirtualOutputV1>,
        name: &str,
        outputs: &[u8],
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        let res = _slf.send_outputs(
            name,
            outputs,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_virtual_output_v1.outputs message: {}", Report::new(e));
        }
    }

    /// Screen copy mode error event
    ///
    /// When an error occurs, an error event is emitted, terminating the replication
    /// mode request issued by the client.
    ///
    /// # Arguments
    ///
    /// - `code`: error code
    /// - `message`: error description
    #[inline]
    fn error(
        &mut self,
        _slf: &Rc<TreelandVirtualOutputV1>,
        code: u32,
        message: &str,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        let res = _slf.send_error(
            code,
            message,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_virtual_output_v1.error message: {}", Report::new(e));
        }
    }

    /// destroy the output
    ///
    /// Destroy the output.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<TreelandVirtualOutputV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_virtual_output_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for TreelandVirtualOutputV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::TreelandVirtualOutputV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_virtual_output_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
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
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("name"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("name"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("name"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("name"));
                        };
                        s
                    }
                };
                let arg1 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("outputs"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("outputs"));
                    }
                    let start = offset;
                    offset += words;
                    &uapi::as_bytes(&msg[start..])[..len]
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_virtual_output_v1#{}.outputs(name: {:?}, outputs: {})\n", msg[0], arg0, debug_array(arg1));
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).outputs(&self, arg0, arg1);
                } else {
                    DefaultHandler.outputs(&self, arg0, arg1);
                }
            }
            1 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("code"));
                };
                offset += 1;
                let arg1 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("message"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("message"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("message"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("message"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_virtual_output_v1#{}.error(code: {}, message: {:?})\n", msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).error(&self, arg0, arg1);
                } else {
                    DefaultHandler.error(&self, arg0, arg1);
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
            0 => "outputs",
            1 => "error",
            _ => return None,
        };
        Some(name)
    }
}

impl Proxy for TreelandVirtualOutputV1 {
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

impl TreelandVirtualOutputV1 {
    /// Since when the error.invalid_group_name enum variant is available.
    pub const ENM__ERROR_INVALID_GROUP_NAME__SINCE: u32 = 1;
    /// Since when the error.invalid_screen_number enum variant is available.
    pub const ENM__ERROR_INVALID_SCREEN_NUMBER__SINCE: u32 = 1;
    /// Since when the error.invalid_output enum variant is available.
    pub const ENM__ERROR_INVALID_OUTPUT__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TreelandVirtualOutputV1Error(pub u32);

impl TreelandVirtualOutputV1Error {
    /// Group name is empty
    pub const INVALID_GROUP_NAME: Self = Self(0);

    /// The number of screens applying for copy mode is less than 2
    pub const INVALID_SCREEN_NUMBER: Self = Self(1);

    /// Output does not exist
    pub const INVALID_OUTPUT: Self = Self(2);
}

impl Debug for TreelandVirtualOutputV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_GROUP_NAME => "INVALID_GROUP_NAME",
            Self::INVALID_SCREEN_NUMBER => "INVALID_SCREEN_NUMBER",
            Self::INVALID_OUTPUT => "INVALID_OUTPUT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
