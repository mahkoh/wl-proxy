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

impl ConcreteObject for TreelandVirtualOutputManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::TreelandVirtualOutputManagerV1;
    const INTERFACE_NAME: &str = "treeland_virtual_output_manager_v1";
}

impl TreelandVirtualOutputManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl TreelandVirtualOutputManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
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
    pub fn try_send_create_virtual_output(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: &str, arg2: &[u8]) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_virtual_output_manager_v1#{}.create_virtual_output(id: treeland_virtual_output_v1#{}, name: {:?}, outputs: {})\n", id, arg0, arg1, debug_array(arg2));
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1, arg2);
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
        fmt.string(arg1);
        fmt.array(arg2);
        Ok(())
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
    pub fn send_create_virtual_output(
        &self,
        id: &Rc<TreelandVirtualOutputV1>,
        name: &str,
        outputs: &[u8],
    ) {
        let res = self.try_send_create_virtual_output(
            id,
            name,
            outputs,
        );
        if let Err(e) = res {
            log_send("treeland_virtual_output_manager_v1.create_virtual_output", &e);
        }
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
    /// - `name`: The name of the user readable virtual output
    /// - `outputs`: Screen name array
    #[inline]
    pub fn new_try_send_create_virtual_output(
        &self,
        name: &str,
        outputs: &[u8],
    ) -> Result<Rc<TreelandVirtualOutputV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_create_virtual_output(
            &id,
            name,
            outputs,
        )?;
        Ok(id)
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
    /// - `name`: The name of the user readable virtual output
    /// - `outputs`: Screen name array
    #[inline]
    pub fn new_send_create_virtual_output(
        &self,
        name: &str,
        outputs: &[u8],
    ) -> Rc<TreelandVirtualOutputV1> {
        let id = self.core.create_child();
        self.send_create_virtual_output(
            &id,
            name,
            outputs,
        );
        id
    }

    /// Since when the get_virtual_output_list message is available.
    pub const MSG__GET_VIRTUAL_OUTPUT_LIST__SINCE: u32 = 1;

    /// Gets a list of virtual output names
    ///
    /// Gets a list of virtual output names.
    #[inline]
    pub fn try_send_get_virtual_output_list(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_virtual_output_manager_v1#{}.get_virtual_output_list()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
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
        ]);
        Ok(())
    }

    /// Gets a list of virtual output names
    ///
    /// Gets a list of virtual output names.
    #[inline]
    pub fn send_get_virtual_output_list(
        &self,
    ) {
        let res = self.try_send_get_virtual_output_list(
        );
        if let Err(e) = res {
            log_send("treeland_virtual_output_manager_v1.get_virtual_output_list", &e);
        }
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
    pub fn try_send_virtual_output_list(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: &[u8]) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_virtual_output_manager_v1#{}.virtual_output_list(names: {})\n", client_id, id, debug_array(arg0));
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
    ) {
        let res = self.try_send_virtual_output_list(
            names,
        );
        if let Err(e) = res {
            log_send("treeland_virtual_output_manager_v1.virtual_output_list", &e);
        }
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
    pub fn try_send_get_virtual_output(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg1.generate_server_id(arg1_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg1_id = arg1.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_virtual_output_manager_v1#{}.get_virtual_output(name: {:?}, id: treeland_virtual_output_v1#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1_id);
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
            2,
        ]);
        fmt.string(arg0);
        fmt.words([
            arg1_id,
        ]);
        Ok(())
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
    pub fn send_get_virtual_output(
        &self,
        name: &str,
        id: &Rc<TreelandVirtualOutputV1>,
    ) {
        let res = self.try_send_get_virtual_output(
            name,
            id,
        );
        if let Err(e) = res {
            log_send("treeland_virtual_output_manager_v1.get_virtual_output", &e);
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
    #[inline]
    pub fn new_try_send_get_virtual_output(
        &self,
        name: &str,
    ) -> Result<Rc<TreelandVirtualOutputV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_virtual_output(
            name,
            &id,
        )?;
        Ok(id)
    }

    /// Get virtual output
    ///
    /// The client obtains the corresponding virtual_output_v1 object
    /// through the virtual output name.
    ///
    /// # Arguments
    ///
    /// - `name`: The name of the user readable virtual output
    #[inline]
    pub fn new_send_get_virtual_output(
        &self,
        name: &str,
    ) -> Rc<TreelandVirtualOutputV1> {
        let id = self.core.create_child();
        self.send_get_virtual_output(
            name,
            &id,
        );
        id
    }
}

/// A message handler for [`TreelandVirtualOutputManagerV1`] proxies.
pub trait TreelandVirtualOutputManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<TreelandVirtualOutputManagerV1>) {
        slf.core.delete_id();
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
        slf: &Rc<TreelandVirtualOutputManagerV1>,
        id: &Rc<TreelandVirtualOutputV1>,
        name: &str,
        outputs: &[u8],
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_virtual_output(
            id,
            name,
            outputs,
        );
        if let Err(e) = res {
            log_forward("treeland_virtual_output_manager_v1.create_virtual_output", &e);
        }
    }

    /// Gets a list of virtual output names
    ///
    /// Gets a list of virtual output names.
    #[inline]
    fn handle_get_virtual_output_list(
        &mut self,
        slf: &Rc<TreelandVirtualOutputManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_virtual_output_list(
        );
        if let Err(e) = res {
            log_forward("treeland_virtual_output_manager_v1.get_virtual_output_list", &e);
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
        slf: &Rc<TreelandVirtualOutputManagerV1>,
        names: &[u8],
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_virtual_output_list(
            names,
        );
        if let Err(e) = res {
            log_forward("treeland_virtual_output_manager_v1.virtual_output_list", &e);
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
        slf: &Rc<TreelandVirtualOutputManagerV1>,
        name: &str,
        id: &Rc<TreelandVirtualOutputV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_virtual_output(
            name,
            id,
        );
        if let Err(e) = res {
            log_forward("treeland_virtual_output_manager_v1.get_virtual_output", &e);
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
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("id")));
                };
                offset += 1;
                let arg1;
                (arg1, offset) = parse_string::<NonNullString>(msg, offset, "name")?;
                let arg2;
                (arg2, offset) = parse_array(msg, offset, "outputs")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: &str, arg2: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_virtual_output_manager_v1#{}.create_virtual_output(id: treeland_virtual_output_v1#{}, name: {:?}, outputs: {})\n", client_id, id, arg0, arg1, debug_array(arg2));
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let arg0 = TreelandVirtualOutputV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_virtual_output(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_create_virtual_output(&self, arg0, arg1, arg2);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_virtual_output_manager_v1#{}.get_virtual_output_list()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_get_virtual_output_list(&self);
                } else {
                    DefaultHandler.handle_get_virtual_output_list(&self);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "name")?;
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("id")));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_virtual_output_manager_v1#{}.get_virtual_output(name: {:?}, id: treeland_virtual_output_v1#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg1_id = arg1;
                let arg1 = TreelandVirtualOutputV1::new(&self.core.state, self.core.version);
                arg1.core().set_client_id(client, arg1_id, arg1.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg1_id, "id", e)))?;
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
            0 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_array(msg, offset, "names")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_virtual_output_manager_v1#{}.virtual_output_list(names: {})\n", id, debug_array(arg0));
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_virtual_output_list(&self, arg0);
                } else {
                    DefaultHandler.handle_virtual_output_list(&self, arg0);
                }
            }
            n => {
                let _ = server;
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

