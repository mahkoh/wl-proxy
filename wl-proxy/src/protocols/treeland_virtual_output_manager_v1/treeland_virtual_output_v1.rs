//! screen output object
//!
//! A treeland_virtual_output_v1 represents a set virtual screen output object.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A treeland_virtual_output_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandVirtualOutputV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn TreelandVirtualOutputV1Handler>,
}

struct DefaultHandler;

impl TreelandVirtualOutputV1Handler for DefaultHandler { }

impl ConcreteObject for TreelandVirtualOutputV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::TreelandVirtualOutputV1;
    const INTERFACE_NAME: &str = "treeland_virtual_output_v1";
}

impl TreelandVirtualOutputV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl TreelandVirtualOutputV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
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
    pub fn try_send_outputs(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: &str, arg1: &[u8]) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_virtual_output_v1#{}.outputs(name: {:?}, outputs: {})\n", client_id, id, arg0, debug_array(arg1));
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1);
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
    ) {
        let res = self.try_send_outputs(
            name,
            outputs,
        );
        if let Err(e) = res {
            log_send("treeland_virtual_output_v1.outputs", &e);
        }
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
    pub fn try_send_error(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_virtual_output_v1#{}.error(code: {}, message: {:?})\n", client_id, id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1);
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
    ) {
        let res = self.try_send_error(
            code,
            message,
        );
        if let Err(e) = res {
            log_send("treeland_virtual_output_v1.error", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the output
    ///
    /// Destroy the output.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_virtual_output_v1#{}.destroy()\n", id);
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
            0,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the output
    ///
    /// Destroy the output.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("treeland_virtual_output_v1.destroy", &e);
        }
    }
}

/// A message handler for [`TreelandVirtualOutputV1`] proxies.
pub trait TreelandVirtualOutputV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<TreelandVirtualOutputV1>) {
        slf.core.delete_id();
    }

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
    fn handle_outputs(
        &mut self,
        slf: &Rc<TreelandVirtualOutputV1>,
        name: &str,
        outputs: &[u8],
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_outputs(
            name,
            outputs,
        );
        if let Err(e) = res {
            log_forward("treeland_virtual_output_v1.outputs", &e);
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
    fn handle_error(
        &mut self,
        slf: &Rc<TreelandVirtualOutputV1>,
        code: u32,
        message: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_error(
            code,
            message,
        );
        if let Err(e) = res {
            log_forward("treeland_virtual_output_v1.error", &e);
        }
    }

    /// destroy the output
    ///
    /// Destroy the output.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<TreelandVirtualOutputV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("treeland_virtual_output_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for TreelandVirtualOutputV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::TreelandVirtualOutputV1, version),
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
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_virtual_output_v1#{}.destroy()\n", client_id, id);
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
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "name")?;
                let arg1;
                (arg1, offset) = parse_array(msg, offset, "outputs")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str, arg1: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_virtual_output_v1#{}.outputs(name: {:?}, outputs: {})\n", id, arg0, debug_array(arg1));
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_outputs(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_outputs(&self, arg0, arg1);
                }
            }
            1 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("code")));
                };
                offset += 1;
                let arg1;
                (arg1, offset) = parse_string::<NonNullString>(msg, offset, "message")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_virtual_output_v1#{}.error(code: {}, message: {:?})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_error(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_error(&self, arg0, arg1);
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

impl Object for TreelandVirtualOutputV1 {
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
