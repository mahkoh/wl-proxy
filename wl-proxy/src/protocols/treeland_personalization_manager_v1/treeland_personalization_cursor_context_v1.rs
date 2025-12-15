//! client custom cursor context
//!
//! This interface allows a client personalization cursor.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A treeland_personalization_cursor_context_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandPersonalizationCursorContextV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn TreelandPersonalizationCursorContextV1Handler>,
}

struct DefaultHandler;

impl TreelandPersonalizationCursorContextV1Handler for DefaultHandler { }

impl TreelandPersonalizationCursorContextV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::TreelandPersonalizationCursorContextV1;
    pub const INTERFACE_NAME: &str = "treeland_personalization_cursor_context_v1";
}

impl TreelandPersonalizationCursorContextV1 {
    pub fn set_handler(&self, handler: impl TreelandPersonalizationCursorContextV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandPersonalizationCursorContextV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandPersonalizationCursorContextV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandPersonalizationCursorContextV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandPersonalizationCursorContextV1 {
    /// Since when the set_theme message is available.
    pub const MSG__SET_THEME__SINCE: u32 = 1;

    /// set system cursor theme
    ///
    /// # Arguments
    ///
    /// - `name`: cursor theme name
    #[inline]
    pub fn try_send_set_theme(
        &self,
        name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            name,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_cursor_context_v1#{}.set_theme(name: {:?})\n", id, arg0);
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
            0,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// set system cursor theme
    ///
    /// # Arguments
    ///
    /// - `name`: cursor theme name
    #[inline]
    pub fn send_set_theme(
        &self,
        name: &str,
    ) {
        let res = self.try_send_set_theme(
            name,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_cursor_context_v1.set_theme", &e);
        }
    }

    /// Since when the get_theme message is available.
    pub const MSG__GET_THEME__SINCE: u32 = 1;

    /// get system cursor theme
    #[inline]
    pub fn try_send_get_theme(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_cursor_context_v1#{}.get_theme()\n", id);
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

    /// get system cursor theme
    #[inline]
    pub fn send_get_theme(
        &self,
    ) {
        let res = self.try_send_get_theme(
        );
        if let Err(e) = res {
            log_send("treeland_personalization_cursor_context_v1.get_theme", &e);
        }
    }

    /// Since when the set_size message is available.
    pub const MSG__SET_SIZE__SINCE: u32 = 1;

    /// set system cursor size
    ///
    /// # Arguments
    ///
    /// - `size`: cursor size
    #[inline]
    pub fn try_send_set_size(
        &self,
        size: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            size,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_cursor_context_v1#{}.set_size(size: {})\n", id, arg0);
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
            2,
            arg0,
        ]);
        Ok(())
    }

    /// set system cursor size
    ///
    /// # Arguments
    ///
    /// - `size`: cursor size
    #[inline]
    pub fn send_set_size(
        &self,
        size: u32,
    ) {
        let res = self.try_send_set_size(
            size,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_cursor_context_v1.set_size", &e);
        }
    }

    /// Since when the get_size message is available.
    pub const MSG__GET_SIZE__SINCE: u32 = 1;

    /// get system cursor size
    #[inline]
    pub fn try_send_get_size(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_cursor_context_v1#{}.get_size()\n", id);
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

    /// get system cursor size
    #[inline]
    pub fn send_get_size(
        &self,
    ) {
        let res = self.try_send_get_size(
        );
        if let Err(e) = res {
            log_send("treeland_personalization_cursor_context_v1.get_size", &e);
        }
    }

    /// Since when the commit message is available.
    pub const MSG__COMMIT__SINCE: u32 = 1;

    /// commit configure
    ///
    /// if only one commit fails validation, the commit will fail
    #[inline]
    pub fn try_send_commit(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_cursor_context_v1#{}.commit()\n", id);
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

    /// commit configure
    ///
    /// if only one commit fails validation, the commit will fail
    #[inline]
    pub fn send_commit(
        &self,
    ) {
        let res = self.try_send_commit(
        );
        if let Err(e) = res {
            log_send("treeland_personalization_cursor_context_v1.commit", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the context object
    ///
    /// Destroy the context object.
    #[inline]
    pub fn try_send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_cursor_context_v1#{}.destroy()\n", id);
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
            5,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the context object
    ///
    /// Destroy the context object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("treeland_personalization_cursor_context_v1.destroy", &e);
        }
    }

    /// Since when the verfity message is available.
    pub const MSG__VERFITY__SINCE: u32 = 1;

    /// verfity event
    ///
    /// Send this signal after commit cursor configure.
    ///
    /// # Arguments
    ///
    /// - `success`: check whether the configuration is successful
    #[inline]
    pub fn try_send_verfity(
        &self,
        success: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            success,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_personalization_cursor_context_v1#{}.verfity(success: {})\n", client_id, id, arg0);
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

    /// verfity event
    ///
    /// Send this signal after commit cursor configure.
    ///
    /// # Arguments
    ///
    /// - `success`: check whether the configuration is successful
    #[inline]
    pub fn send_verfity(
        &self,
        success: i32,
    ) {
        let res = self.try_send_verfity(
            success,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_cursor_context_v1.verfity", &e);
        }
    }

    /// Since when the theme message is available.
    pub const MSG__THEME__SINCE: u32 = 1;

    /// cursor theme changed event
    ///
    /// Send this signal after system cursor theme changed.
    ///
    /// # Arguments
    ///
    /// - `name`: cursor theme name
    #[inline]
    pub fn try_send_theme(
        &self,
        name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            name,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_personalization_cursor_context_v1#{}.theme(name: {:?})\n", client_id, id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// cursor theme changed event
    ///
    /// Send this signal after system cursor theme changed.
    ///
    /// # Arguments
    ///
    /// - `name`: cursor theme name
    #[inline]
    pub fn send_theme(
        &self,
        name: &str,
    ) {
        let res = self.try_send_theme(
            name,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_cursor_context_v1.theme", &e);
        }
    }

    /// Since when the size message is available.
    pub const MSG__SIZE__SINCE: u32 = 1;

    /// cursor size changed event
    ///
    /// Send this signal after system cursor size changed.
    ///
    /// # Arguments
    ///
    /// - `size`: cursor size
    #[inline]
    pub fn try_send_size(
        &self,
        size: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            size,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_personalization_cursor_context_v1#{}.size(size: {})\n", client_id, id, arg0);
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
            2,
            arg0,
        ]);
        Ok(())
    }

    /// cursor size changed event
    ///
    /// Send this signal after system cursor size changed.
    ///
    /// # Arguments
    ///
    /// - `size`: cursor size
    #[inline]
    pub fn send_size(
        &self,
        size: u32,
    ) {
        let res = self.try_send_size(
            size,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_cursor_context_v1.size", &e);
        }
    }
}

/// A message handler for [TreelandPersonalizationCursorContextV1] proxies.
pub trait TreelandPersonalizationCursorContextV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<TreelandPersonalizationCursorContextV1>) {
        slf.core.delete_id();
    }

    /// set system cursor theme
    ///
    /// # Arguments
    ///
    /// - `name`: cursor theme name
    #[inline]
    fn handle_set_theme(
        &mut self,
        _slf: &Rc<TreelandPersonalizationCursorContextV1>,
        name: &str,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_set_theme(
            name,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_cursor_context_v1.set_theme", &e);
        }
    }

    /// get system cursor theme
    #[inline]
    fn handle_get_theme(
        &mut self,
        _slf: &Rc<TreelandPersonalizationCursorContextV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_get_theme(
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_cursor_context_v1.get_theme", &e);
        }
    }

    /// set system cursor size
    ///
    /// # Arguments
    ///
    /// - `size`: cursor size
    #[inline]
    fn handle_set_size(
        &mut self,
        _slf: &Rc<TreelandPersonalizationCursorContextV1>,
        size: u32,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_set_size(
            size,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_cursor_context_v1.set_size", &e);
        }
    }

    /// get system cursor size
    #[inline]
    fn handle_get_size(
        &mut self,
        _slf: &Rc<TreelandPersonalizationCursorContextV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_get_size(
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_cursor_context_v1.get_size", &e);
        }
    }

    /// commit configure
    ///
    /// if only one commit fails validation, the commit will fail
    #[inline]
    fn handle_commit(
        &mut self,
        _slf: &Rc<TreelandPersonalizationCursorContextV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_commit(
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_cursor_context_v1.commit", &e);
        }
    }

    /// destroy the context object
    ///
    /// Destroy the context object.
    #[inline]
    fn handle_destroy(
        &mut self,
        _slf: &Rc<TreelandPersonalizationCursorContextV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_cursor_context_v1.destroy", &e);
        }
    }

    /// verfity event
    ///
    /// Send this signal after commit cursor configure.
    ///
    /// # Arguments
    ///
    /// - `success`: check whether the configuration is successful
    #[inline]
    fn handle_verfity(
        &mut self,
        _slf: &Rc<TreelandPersonalizationCursorContextV1>,
        success: i32,
    ) {
        if !_slf.core.forward_to_client.get() {
            return;
        }
        let res = _slf.try_send_verfity(
            success,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_cursor_context_v1.verfity", &e);
        }
    }

    /// cursor theme changed event
    ///
    /// Send this signal after system cursor theme changed.
    ///
    /// # Arguments
    ///
    /// - `name`: cursor theme name
    #[inline]
    fn handle_theme(
        &mut self,
        _slf: &Rc<TreelandPersonalizationCursorContextV1>,
        name: &str,
    ) {
        if !_slf.core.forward_to_client.get() {
            return;
        }
        let res = _slf.try_send_theme(
            name,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_cursor_context_v1.theme", &e);
        }
    }

    /// cursor size changed event
    ///
    /// Send this signal after system cursor size changed.
    ///
    /// # Arguments
    ///
    /// - `size`: cursor size
    #[inline]
    fn handle_size(
        &mut self,
        _slf: &Rc<TreelandPersonalizationCursorContextV1>,
        size: u32,
    ) {
        if !_slf.core.forward_to_client.get() {
            return;
        }
        let res = _slf.try_send_size(
            size,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_cursor_context_v1.size", &e);
        }
    }
}

impl ObjectPrivate for TreelandPersonalizationCursorContextV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::TreelandPersonalizationCursorContextV1, version),
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
            self.core.delete_id();
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
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_cursor_context_v1#{}.set_theme(name: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_theme(&self, arg0);
                } else {
                    DefaultHandler.handle_set_theme(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_cursor_context_v1#{}.get_theme()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_get_theme(&self);
                } else {
                    DefaultHandler.handle_get_theme(&self);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_cursor_context_v1#{}.set_size(size: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_size(&self, arg0);
                } else {
                    DefaultHandler.handle_set_size(&self, arg0);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_cursor_context_v1#{}.get_size()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_get_size(&self);
                } else {
                    DefaultHandler.handle_get_size(&self);
                }
            }
            4 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_cursor_context_v1#{}.commit()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_commit(&self);
                } else {
                    DefaultHandler.handle_commit(&self);
                }
            }
            5 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_cursor_context_v1#{}.destroy()\n", client_id, id);
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
                let arg0 = arg0 as i32;
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_personalization_cursor_context_v1#{}.verfity(success: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_verfity(&self, arg0);
                } else {
                    DefaultHandler.handle_verfity(&self, arg0);
                }
            }
            1 => {
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
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_personalization_cursor_context_v1#{}.theme(name: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_theme(&self, arg0);
                } else {
                    DefaultHandler.handle_theme(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_personalization_cursor_context_v1#{}.size(size: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_size(&self, arg0);
                } else {
                    DefaultHandler.handle_size(&self, arg0);
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
            0 => "set_theme",
            1 => "get_theme",
            2 => "set_size",
            3 => "get_size",
            4 => "commit",
            5 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "verfity",
            1 => "theme",
            2 => "size",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for TreelandPersonalizationCursorContextV1 {
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

