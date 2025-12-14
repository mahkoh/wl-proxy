//! Manager creates virtual output
//!
//! This interface is a manager that allows the creation of copied output.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A treeland_virtual_output_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandVirtualOutputManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn TreelandVirtualOutputManagerV1Handler>,
}

struct DefaultHandler;

impl TreelandVirtualOutputManagerV1Handler for DefaultHandler { }

impl TreelandVirtualOutputManagerV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::TreelandVirtualOutputManagerV1;
    pub const INTERFACE_NAME: &str = "treeland_virtual_output_manager_v1";
}

impl TreelandVirtualOutputManagerV1 {
    pub fn set_handler(&self, handler: impl TreelandVirtualOutputManagerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandVirtualOutputManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandVirtualOutputManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandVirtualOutputManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandVirtualOutputManagerV1 {
    /// Since when the create_virtual_output message is available.
    pub const MSG__CREATE_VIRTUAL_OUTPUT__SINCE: u32 = 1;

    /// Create a virtual output
    ///
    /// Create virtual output that can be used when setting screen copy mode for use
    /// on multiple screens. Virtual outputs are created to mirror multiple wl_output
    /// outputs.
    ///
    /// The element of the array is the name of the screen.
    ///
    /// The first element of the array outputs is the screen to be copied, and
    /// the subsequent elements are the screens to be mirrored.
    ///
    /// The client calling this interface will not generate an additional wl_output
    /// object on the client.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `name`: The name of the user readable virtual output
    /// - `outputs`: Screen name array
    #[inline]
    pub fn send_create_virtual_output(
        &self,
        id: &Rc<TreelandVirtualOutputV1>,
        name: &str,
        outputs: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            id,
            name,
            outputs,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_virtual_output_manager_v1#{}.create_virtual_output(id: treeland_virtual_output_v1#{}, name: {:?}, outputs: {})\n", id, arg0_id, arg1, debug_array(arg2));
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
        fmt.string(arg1);
        fmt.array(arg2);
        Ok(())
    }

    /// Since when the get_virtual_output_list message is available.
    pub const MSG__GET_VIRTUAL_OUTPUT_LIST__SINCE: u32 = 1;

    /// Gets a list of virtual output names
    ///
    /// Gets a list of virtual output names.
    #[inline]
    pub fn send_get_virtual_output_list(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_virtual_output_manager_v1#{}.get_virtual_output_list()\n", id);
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
            1,
        ]);
        Ok(())
    }

    /// Since when the virtual_output_list message is available.
    pub const MSG__VIRTUAL_OUTPUT_LIST__SINCE: u32 = 1;

    /// Send a list of virtual output names
    ///
    /// Sends a list of virtual output names to the client.
    ///
    /// # Arguments
    ///
    /// - `names`: List of virtual output names
    #[inline]
    pub fn send_virtual_output_list(
        &self,
        names: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            names,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_virtual_output_manager_v1#{}.virtual_output_list(names: {})\n", client.endpoint.id, id, debug_array(arg0));
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
        fmt.array(arg0);
        Ok(())
    }

    /// Since when the get_virtual_output message is available.
    pub const MSG__GET_VIRTUAL_OUTPUT__SINCE: u32 = 1;

    /// Get virtual output
    ///
    /// The client obtains the corresponding virtual_output_v1 object
    /// through the virtual output name.
    ///
    /// # Arguments
    ///
    /// - `name`: The name of the user readable virtual output
    /// - `id`:
    #[inline]
    pub fn send_get_virtual_output(
        &self,
        name: &str,
        id: &Rc<TreelandVirtualOutputV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            name,
            id,
        );
        let arg1_obj = arg1;
        let arg1 = arg1_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg1.generate_server_id(arg1_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg1_id = arg1.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_virtual_output_manager_v1#{}.get_virtual_output(name: {:?}, id: treeland_virtual_output_v1#{})\n", id, arg0, arg1_id);
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
            2,
        ]);
        fmt.string(arg0);
        fmt.words([
            arg1_id,
        ]);
        Ok(())
    }
}

/// A message handler for [TreelandVirtualOutputManagerV1] proxies.
pub trait TreelandVirtualOutputManagerV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<TreelandVirtualOutputManagerV1>) {
        let _ = slf.core.delete_id();
    }

    /// Create a virtual output
    ///
    /// Create virtual output that can be used when setting screen copy mode for use
    /// on multiple screens. Virtual outputs are created to mirror multiple wl_output
    /// outputs.
    ///
    /// The element of the array is the name of the screen.
    ///
    /// The first element of the array outputs is the screen to be copied, and
    /// the subsequent elements are the screens to be mirrored.
    ///
    /// The client calling this interface will not generate an additional wl_output
    /// object on the client.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `name`: The name of the user readable virtual output
    /// - `outputs`: Screen name array
    #[inline]
    fn handle_create_virtual_output(
        &mut self,
        _slf: &Rc<TreelandVirtualOutputManagerV1>,
        id: &Rc<TreelandVirtualOutputV1>,
        name: &str,
        outputs: &[u8],
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.send_create_virtual_output(
            id,
            name,
            outputs,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_virtual_output_manager_v1.create_virtual_output message: {}", Report::new(e));
        }
    }

    /// Gets a list of virtual output names
    ///
    /// Gets a list of virtual output names.
    #[inline]
    fn handle_get_virtual_output_list(
        &mut self,
        _slf: &Rc<TreelandVirtualOutputManagerV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.send_get_virtual_output_list(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_virtual_output_manager_v1.get_virtual_output_list message: {}", Report::new(e));
        }
    }

    /// Send a list of virtual output names
    ///
    /// Sends a list of virtual output names to the client.
    ///
    /// # Arguments
    ///
    /// - `names`: List of virtual output names
    #[inline]
    fn handle_virtual_output_list(
        &mut self,
        _slf: &Rc<TreelandVirtualOutputManagerV1>,
        names: &[u8],
    ) {
        if !_slf.core.forward_to_client.get() {
            return;
        }
        let res = _slf.send_virtual_output_list(
            names,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_virtual_output_manager_v1.virtual_output_list message: {}", Report::new(e));
        }
    }

    /// Get virtual output
    ///
    /// The client obtains the corresponding virtual_output_v1 object
    /// through the virtual output name.
    ///
    /// # Arguments
    ///
    /// - `name`: The name of the user readable virtual output
    /// - `id`:
    #[inline]
    fn handle_get_virtual_output(
        &mut self,
        _slf: &Rc<TreelandVirtualOutputManagerV1>,
        name: &str,
        id: &Rc<TreelandVirtualOutputV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.send_get_virtual_output(
            name,
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_virtual_output_manager_v1.get_virtual_output message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for TreelandVirtualOutputManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::TreelandVirtualOutputManagerV1, version),
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
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("id"));
                };
                offset += 1;
                let arg1 = {
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
                let arg2 = {
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_virtual_output_manager_v1#{}.create_virtual_output(id: treeland_virtual_output_v1#{}, name: {:?}, outputs: {})\n", client.endpoint.id, msg[0], arg0, arg1, debug_array(arg2));
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = TreelandVirtualOutputV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_virtual_output(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_create_virtual_output(&self, arg0, arg1, arg2);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_virtual_output_manager_v1#{}.get_virtual_output_list()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_get_virtual_output_list(&self);
                } else {
                    DefaultHandler.handle_get_virtual_output_list(&self);
                }
            }
            2 => {
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
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("id"));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_virtual_output_manager_v1#{}.get_virtual_output(name: {:?}, id: treeland_virtual_output_v1#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg1_id = arg1;
                let arg1 = TreelandVirtualOutputV1::new(&self.core.state, self.core.version);
                arg1.core().set_client_id(client, arg1_id, arg1.clone())
                    .map_err(|e| ObjectError::SetClientId(arg1_id, "id", e))?;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_virtual_output(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_virtual_output(&self, arg0, arg1);
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
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("names"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("names"));
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_virtual_output_manager_v1#{}.virtual_output_list(names: {})\n", msg[0], debug_array(arg0));
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_virtual_output_list(&self, arg0);
                } else {
                    DefaultHandler.handle_virtual_output_list(&self, arg0);
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
            0 => "create_virtual_output",
            1 => "get_virtual_output_list",
            2 => "get_virtual_output",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "virtual_output_list",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for TreelandVirtualOutputManagerV1 {
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

