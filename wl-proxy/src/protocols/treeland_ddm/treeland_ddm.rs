//! core global for treeland - ddm connection
//!
//! This object is primarily used for establish connection between
//! treeland and ddm.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A treeland_ddm object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandDdm {
    core: ObjectCore,
    handler: HandlerHolder<dyn TreelandDdmHandler>,
}

struct DefaultHandler;

impl TreelandDdmHandler for DefaultHandler { }

impl ConcreteObject for TreelandDdm {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::TreelandDdm;
    const INTERFACE_NAME: &str = "treeland_ddm";
}

impl TreelandDdm {
    pub fn set_handler(&self, handler: impl TreelandDdmHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandDdmHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandDdm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandDdm")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandDdm {
    /// Since when the switch_to_greeter message is available.
    pub const MSG__SWITCH_TO_GREETER__SINCE: u32 = 1;

    /// send treeland to greeter mode
    ///
    /// Send treeland to Greeter mode.
    #[inline]
    pub fn try_send_switch_to_greeter(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_ddm#{}.switch_to_greeter()\n", id);
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
        Ok(())
    }

    /// send treeland to greeter mode
    ///
    /// Send treeland to Greeter mode.
    #[inline]
    pub fn send_switch_to_greeter(
        &self,
    ) {
        let res = self.try_send_switch_to_greeter(
        );
        if let Err(e) = res {
            log_send("treeland_ddm.switch_to_greeter", &e);
        }
    }

    /// Since when the switch_to_user message is available.
    pub const MSG__SWITCH_TO_USER__SINCE: u32 = 1;

    /// call treeland to switch lockscreen user
    ///
    /// Set lockscreen user to username. Ignore when username is "ddm".
    ///
    /// # Arguments
    ///
    /// - `username`:
    #[inline]
    pub fn try_send_switch_to_user(
        &self,
        username: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            username,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_ddm#{}.switch_to_user(username: {:?})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
        fmt.string(arg0);
        Ok(())
    }

    /// call treeland to switch lockscreen user
    ///
    /// Set lockscreen user to username. Ignore when username is "ddm".
    ///
    /// # Arguments
    ///
    /// - `username`:
    #[inline]
    pub fn send_switch_to_user(
        &self,
        username: &str,
    ) {
        let res = self.try_send_switch_to_user(
            username,
        );
        if let Err(e) = res {
            log_send("treeland_ddm.switch_to_user", &e);
        }
    }

    /// Since when the activate_session message is available.
    pub const MSG__ACTIVATE_SESSION__SINCE: u32 = 1;

    /// activate wayland session
    ///
    /// Activate treeland session. This will makes treeland try to take
    /// control of screen.
    #[inline]
    pub fn try_send_activate_session(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_ddm#{}.activate_session()\n", id);
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
            2,
        ]);
        Ok(())
    }

    /// activate wayland session
    ///
    /// Activate treeland session. This will makes treeland try to take
    /// control of screen.
    #[inline]
    pub fn send_activate_session(
        &self,
    ) {
        let res = self.try_send_activate_session(
        );
        if let Err(e) = res {
            log_send("treeland_ddm.activate_session", &e);
        }
    }

    /// Since when the deactivate_session message is available.
    pub const MSG__DEACTIVATE_SESSION__SINCE: u32 = 1;

    /// deactivate wayland session
    ///
    /// Deactivate treeland session. This will release control of the
    /// screen, but not to close the current seats.
    #[inline]
    pub fn try_send_deactivate_session(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_ddm#{}.deactivate_session()\n", id);
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
            3,
        ]);
        Ok(())
    }

    /// deactivate wayland session
    ///
    /// Deactivate treeland session. This will release control of the
    /// screen, but not to close the current seats.
    #[inline]
    pub fn send_deactivate_session(
        &self,
    ) {
        let res = self.try_send_deactivate_session(
        );
        if let Err(e) = res {
            log_send("treeland_ddm.deactivate_session", &e);
        }
    }

    /// Since when the enable_render message is available.
    pub const MSG__ENABLE_RENDER__SINCE: u32 = 1;

    /// start treeland rendering
    ///
    /// Enable treeland rendering. This is primarily called after
    /// disable_render to resume treeland.
    #[inline]
    pub fn try_send_enable_render(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_ddm#{}.enable_render()\n", id);
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
            4,
        ]);
        Ok(())
    }

    /// start treeland rendering
    ///
    /// Enable treeland rendering. This is primarily called after
    /// disable_render to resume treeland.
    #[inline]
    pub fn send_enable_render(
        &self,
    ) {
        let res = self.try_send_enable_render(
        );
        if let Err(e) = res {
            log_send("treeland_ddm.enable_render", &e);
        }
    }

    /// Since when the disable_render message is available.
    pub const MSG__DISABLE_RENDER__SINCE: u32 = 1;

    /// stop treeland rendering
    ///
    /// Disable treeland rendering. This will prevent treeland from
    /// output to DRM device.
    #[inline]
    pub fn try_send_disable_render(
        &self,
        callback: &Rc<WlCallback>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            callback,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("callback", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_ddm#{}.disable_render(callback: wl_callback#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
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
            5,
            arg0_id,
        ]);
        Ok(())
    }

    /// stop treeland rendering
    ///
    /// Disable treeland rendering. This will prevent treeland from
    /// output to DRM device.
    #[inline]
    pub fn send_disable_render(
        &self,
        callback: &Rc<WlCallback>,
    ) {
        let res = self.try_send_disable_render(
            callback,
        );
        if let Err(e) = res {
            log_send("treeland_ddm.disable_render", &e);
        }
    }

    /// stop treeland rendering
    ///
    /// Disable treeland rendering. This will prevent treeland from
    /// output to DRM device.
    #[inline]
    pub fn new_try_send_disable_render(
        &self,
    ) -> Result<Rc<WlCallback>, ObjectError> {
        let callback = self.core.create_child();
        self.try_send_disable_render(
            &callback,
        )?;
        Ok(callback)
    }

    /// stop treeland rendering
    ///
    /// Disable treeland rendering. This will prevent treeland from
    /// output to DRM device.
    #[inline]
    pub fn new_send_disable_render(
        &self,
    ) -> Rc<WlCallback> {
        let callback = self.core.create_child();
        self.send_disable_render(
            &callback,
        );
        callback
    }

    /// Since when the switch_to_vt message is available.
    pub const MSG__SWITCH_TO_VT__SINCE: u32 = 1;

    /// switch to virtual terminal
    ///
    /// Call ddm to switch current virtual terminal to vtnr. ddm should
    /// take care of the switch and call ioctl respectively.
    ///
    /// # Arguments
    ///
    /// - `vtnr`:
    #[inline]
    pub fn try_send_switch_to_vt(
        &self,
        vtnr: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            vtnr,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_ddm#{}.switch_to_vt(vtnr: {})\n", client_id, id, arg0);
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
            arg0 as u32,
        ]);
        Ok(())
    }

    /// switch to virtual terminal
    ///
    /// Call ddm to switch current virtual terminal to vtnr. ddm should
    /// take care of the switch and call ioctl respectively.
    ///
    /// # Arguments
    ///
    /// - `vtnr`:
    #[inline]
    pub fn send_switch_to_vt(
        &self,
        vtnr: i32,
    ) {
        let res = self.try_send_switch_to_vt(
            vtnr,
        );
        if let Err(e) = res {
            log_send("treeland_ddm.switch_to_vt", &e);
        }
    }

    /// Since when the acquire_vt message is available.
    pub const MSG__ACQUIRE_VT__SINCE: u32 = 1;

    /// acquire control of virtual terminal
    ///
    /// Call ddm to acquire control of VT at vtnr. ddm should call
    /// VT_SETMODE respectively.
    ///
    /// # Arguments
    ///
    /// - `vtnr`:
    #[inline]
    pub fn try_send_acquire_vt(
        &self,
        vtnr: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            vtnr,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_ddm#{}.acquire_vt(vtnr: {})\n", client_id, id, arg0);
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
            1,
            arg0 as u32,
        ]);
        Ok(())
    }

    /// acquire control of virtual terminal
    ///
    /// Call ddm to acquire control of VT at vtnr. ddm should call
    /// VT_SETMODE respectively.
    ///
    /// # Arguments
    ///
    /// - `vtnr`:
    #[inline]
    pub fn send_acquire_vt(
        &self,
        vtnr: i32,
    ) {
        let res = self.try_send_acquire_vt(
            vtnr,
        );
        if let Err(e) = res {
            log_send("treeland_ddm.acquire_vt", &e);
        }
    }
}

/// A message handler for [TreelandDdm] proxies.
pub trait TreelandDdmHandler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<TreelandDdm>) {
        slf.core.delete_id();
    }

    /// send treeland to greeter mode
    ///
    /// Send treeland to Greeter mode.
    #[inline]
    fn handle_switch_to_greeter(
        &mut self,
        _slf: &Rc<TreelandDdm>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_switch_to_greeter(
        );
        if let Err(e) = res {
            log_forward("treeland_ddm.switch_to_greeter", &e);
        }
    }

    /// call treeland to switch lockscreen user
    ///
    /// Set lockscreen user to username. Ignore when username is "ddm".
    ///
    /// # Arguments
    ///
    /// - `username`:
    #[inline]
    fn handle_switch_to_user(
        &mut self,
        _slf: &Rc<TreelandDdm>,
        username: &str,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_switch_to_user(
            username,
        );
        if let Err(e) = res {
            log_forward("treeland_ddm.switch_to_user", &e);
        }
    }

    /// activate wayland session
    ///
    /// Activate treeland session. This will makes treeland try to take
    /// control of screen.
    #[inline]
    fn handle_activate_session(
        &mut self,
        _slf: &Rc<TreelandDdm>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_activate_session(
        );
        if let Err(e) = res {
            log_forward("treeland_ddm.activate_session", &e);
        }
    }

    /// deactivate wayland session
    ///
    /// Deactivate treeland session. This will release control of the
    /// screen, but not to close the current seats.
    #[inline]
    fn handle_deactivate_session(
        &mut self,
        _slf: &Rc<TreelandDdm>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_deactivate_session(
        );
        if let Err(e) = res {
            log_forward("treeland_ddm.deactivate_session", &e);
        }
    }

    /// start treeland rendering
    ///
    /// Enable treeland rendering. This is primarily called after
    /// disable_render to resume treeland.
    #[inline]
    fn handle_enable_render(
        &mut self,
        _slf: &Rc<TreelandDdm>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_enable_render(
        );
        if let Err(e) = res {
            log_forward("treeland_ddm.enable_render", &e);
        }
    }

    /// stop treeland rendering
    ///
    /// Disable treeland rendering. This will prevent treeland from
    /// output to DRM device.
    ///
    /// # Arguments
    ///
    /// - `callback`:
    #[inline]
    fn handle_disable_render(
        &mut self,
        _slf: &Rc<TreelandDdm>,
        callback: &Rc<WlCallback>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_disable_render(
            callback,
        );
        if let Err(e) = res {
            log_forward("treeland_ddm.disable_render", &e);
        }
    }

    /// switch to virtual terminal
    ///
    /// Call ddm to switch current virtual terminal to vtnr. ddm should
    /// take care of the switch and call ioctl respectively.
    ///
    /// # Arguments
    ///
    /// - `vtnr`:
    #[inline]
    fn handle_switch_to_vt(
        &mut self,
        _slf: &Rc<TreelandDdm>,
        vtnr: i32,
    ) {
        if !_slf.core.forward_to_client.get() {
            return;
        }
        let res = _slf.try_send_switch_to_vt(
            vtnr,
        );
        if let Err(e) = res {
            log_forward("treeland_ddm.switch_to_vt", &e);
        }
    }

    /// acquire control of virtual terminal
    ///
    /// Call ddm to acquire control of VT at vtnr. ddm should call
    /// VT_SETMODE respectively.
    ///
    /// # Arguments
    ///
    /// - `vtnr`:
    #[inline]
    fn handle_acquire_vt(
        &mut self,
        _slf: &Rc<TreelandDdm>,
        vtnr: i32,
    ) {
        if !_slf.core.forward_to_client.get() {
            return;
        }
        let res = _slf.try_send_acquire_vt(
            vtnr,
        );
        if let Err(e) = res {
            log_forward("treeland_ddm.acquire_vt", &e);
        }
    }
}

impl ObjectPrivate for TreelandDdm {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::TreelandDdm, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_ddm#{}.switch_to_greeter()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_switch_to_greeter(&self);
                } else {
                    DefaultHandler.handle_switch_to_greeter(&self);
                }
            }
            1 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError(ObjectErrorKind::MissingArgument("username")));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError(ObjectErrorKind::MissingArgument("username")));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError(ObjectErrorKind::NullString("username")));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError(ObjectErrorKind::NonUtf8("username")));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_ddm#{}.switch_to_user(username: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_switch_to_user(&self, arg0);
                } else {
                    DefaultHandler.handle_switch_to_user(&self, arg0);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_ddm#{}.activate_session()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_activate_session(&self);
                } else {
                    DefaultHandler.handle_activate_session(&self);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_ddm#{}.deactivate_session()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_deactivate_session(&self);
                } else {
                    DefaultHandler.handle_deactivate_session(&self);
                }
            }
            4 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_ddm#{}.enable_render()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_enable_render(&self);
                } else {
                    DefaultHandler.handle_enable_render(&self);
                }
            }
            5 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_ddm#{}.disable_render(callback: wl_callback#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WlCallback::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "callback", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_disable_render(&self, arg0);
                } else {
                    DefaultHandler.handle_disable_render(&self, arg0);
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = arg0 as i32;
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_ddm#{}.switch_to_vt(vtnr: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_switch_to_vt(&self, arg0);
                } else {
                    DefaultHandler.handle_switch_to_vt(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = arg0 as i32;
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_ddm#{}.acquire_vt(vtnr: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_acquire_vt(&self, arg0);
                } else {
                    DefaultHandler.handle_acquire_vt(&self, arg0);
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
            0 => "switch_to_greeter",
            1 => "switch_to_user",
            2 => "activate_session",
            3 => "deactivate_session",
            4 => "enable_render",
            5 => "disable_render",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "switch_to_vt",
            1 => "acquire_vt",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for TreelandDdm {
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

