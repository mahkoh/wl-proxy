//! client custom wallpaper context
//!
//! This interface allows a client personalization wallpaper.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A treeland_personalization_wallpaper_context_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandPersonalizationWallpaperContextV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn TreelandPersonalizationWallpaperContextV1Handler>,
}

struct DefaultHandler;

impl TreelandPersonalizationWallpaperContextV1Handler for DefaultHandler { }

impl ConcreteObject for TreelandPersonalizationWallpaperContextV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::TreelandPersonalizationWallpaperContextV1;
    const INTERFACE_NAME: &str = "treeland_personalization_wallpaper_context_v1";
}

impl TreelandPersonalizationWallpaperContextV1 {
    pub fn set_handler(&self, handler: impl TreelandPersonalizationWallpaperContextV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandPersonalizationWallpaperContextV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandPersonalizationWallpaperContextV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandPersonalizationWallpaperContextV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandPersonalizationWallpaperContextV1 {
    /// Since when the set_fd message is available.
    pub const MSG__SET_FD__SINCE: u32 = 1;

    /// set the current user's wallpaper fd
    ///
    /// # Arguments
    ///
    /// - `fd`: wallpaper file fd
    /// - `metadata`: file related metadata information
    #[inline]
    pub fn try_send_set_fd(
        &self,
        fd: &Rc<OwnedFd>,
        metadata: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            fd,
            metadata,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_wallpaper_context_v1#{}.set_fd(fd: {}, metadata: {:?})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0.as_raw_fd(), arg1);
        }
        let endpoint = &self.core.state.server;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.fds.push_back(arg0.clone());
        fmt.words([
            id,
            0,
        ]);
        fmt.string(arg1);
        Ok(())
    }

    /// set the current user's wallpaper fd
    ///
    /// # Arguments
    ///
    /// - `fd`: wallpaper file fd
    /// - `metadata`: file related metadata information
    #[inline]
    pub fn send_set_fd(
        &self,
        fd: &Rc<OwnedFd>,
        metadata: &str,
    ) {
        let res = self.try_send_set_fd(
            fd,
            metadata,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_wallpaper_context_v1.set_fd", &e);
        }
    }

    /// Since when the set_identifier message is available.
    pub const MSG__SET_IDENTIFIER__SINCE: u32 = 1;

    /// identifier for the application window
    ///
    /// # Arguments
    ///
    /// - `identifier`: Identifier for the application window
    #[inline]
    pub fn try_send_set_identifier(
        &self,
        identifier: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            identifier,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_wallpaper_context_v1#{}.set_identifier(identifier: {:?})\n", id, arg0);
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

    /// identifier for the application window
    ///
    /// # Arguments
    ///
    /// - `identifier`: Identifier for the application window
    #[inline]
    pub fn send_set_identifier(
        &self,
        identifier: &str,
    ) {
        let res = self.try_send_set_identifier(
            identifier,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_wallpaper_context_v1.set_identifier", &e);
        }
    }

    /// Since when the set_output message is available.
    pub const MSG__SET_OUTPUT__SINCE: u32 = 1;

    /// configure xdg desktop portal options
    ///
    /// # Arguments
    ///
    /// - `output`: system output name
    #[inline]
    pub fn try_send_set_output(
        &self,
        output: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            output,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_wallpaper_context_v1#{}.set_output(output: {:?})\n", id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// configure xdg desktop portal options
    ///
    /// # Arguments
    ///
    /// - `output`: system output name
    #[inline]
    pub fn send_set_output(
        &self,
        output: &str,
    ) {
        let res = self.try_send_set_output(
            output,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_wallpaper_context_v1.set_output", &e);
        }
    }

    /// Since when the set_on message is available.
    pub const MSG__SET_ON__SINCE: u32 = 1;

    /// configure xdg desktop portal options
    ///
    /// # Arguments
    ///
    /// - `options`: xdg desktop portal options
    #[inline]
    pub fn try_send_set_on(
        &self,
        options: TreelandPersonalizationWallpaperContextV1Options,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            options,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: TreelandPersonalizationWallpaperContextV1Options) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_wallpaper_context_v1#{}.set_on(options: {:?})\n", id, arg0);
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
            3,
            arg0.0,
        ]);
        Ok(())
    }

    /// configure xdg desktop portal options
    ///
    /// # Arguments
    ///
    /// - `options`: xdg desktop portal options
    #[inline]
    pub fn send_set_on(
        &self,
        options: TreelandPersonalizationWallpaperContextV1Options,
    ) {
        let res = self.try_send_set_on(
            options,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_wallpaper_context_v1.set_on", &e);
        }
    }

    /// Since when the set_isdark message is available.
    pub const MSG__SET_ISDARK__SINCE: u32 = 1;

    /// Set whether the current wallpaper is dark
    ///
    /// # Arguments
    ///
    /// - `isdark`: is dark
    #[inline]
    pub fn try_send_set_isdark(
        &self,
        isdark: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            isdark,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_wallpaper_context_v1#{}.set_isdark(isdark: {})\n", id, arg0);
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
            4,
            arg0,
        ]);
        Ok(())
    }

    /// Set whether the current wallpaper is dark
    ///
    /// # Arguments
    ///
    /// - `isdark`: is dark
    #[inline]
    pub fn send_set_isdark(
        &self,
        isdark: u32,
    ) {
        let res = self.try_send_set_isdark(
            isdark,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_wallpaper_context_v1.set_isdark", &e);
        }
    }

    /// Since when the commit message is available.
    pub const MSG__COMMIT__SINCE: u32 = 1;

    /// commit configuration
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_wallpaper_context_v1#{}.commit()\n", id);
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
        Ok(())
    }

    /// commit configuration
    #[inline]
    pub fn send_commit(
        &self,
    ) {
        let res = self.try_send_commit(
        );
        if let Err(e) = res {
            log_send("treeland_personalization_wallpaper_context_v1.commit", &e);
        }
    }

    /// Since when the get_metadata message is available.
    pub const MSG__GET_METADATA__SINCE: u32 = 1;

    /// get user save meta data
    ///
    /// get the current user's wallpaper
    #[inline]
    pub fn try_send_get_metadata(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_wallpaper_context_v1#{}.get_metadata()\n", id);
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
            6,
        ]);
        Ok(())
    }

    /// get user save meta data
    ///
    /// get the current user's wallpaper
    #[inline]
    pub fn send_get_metadata(
        &self,
    ) {
        let res = self.try_send_get_metadata(
        );
        if let Err(e) = res {
            log_send("treeland_personalization_wallpaper_context_v1.get_metadata", &e);
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_wallpaper_context_v1#{}.destroy()\n", id);
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
            7,
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
            log_send("treeland_personalization_wallpaper_context_v1.destroy", &e);
        }
    }

    /// Since when the metadata message is available.
    pub const MSG__METADATA__SINCE: u32 = 1;

    /// get metadata event
    ///
    /// Send this signal after getting the user's wallpaper.
    ///
    /// # Arguments
    ///
    /// - `metadata`: user meta data
    #[inline]
    pub fn try_send_metadata(
        &self,
        metadata: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            metadata,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_personalization_wallpaper_context_v1#{}.metadata(metadata: {:?})\n", client_id, id, arg0);
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
        fmt.string(arg0);
        Ok(())
    }

    /// get metadata event
    ///
    /// Send this signal after getting the user's wallpaper.
    ///
    /// # Arguments
    ///
    /// - `metadata`: user meta data
    #[inline]
    pub fn send_metadata(
        &self,
        metadata: &str,
    ) {
        let res = self.try_send_metadata(
            metadata,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_wallpaper_context_v1.metadata", &e);
        }
    }
}

/// A message handler for [TreelandPersonalizationWallpaperContextV1] proxies.
pub trait TreelandPersonalizationWallpaperContextV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<TreelandPersonalizationWallpaperContextV1>) {
        slf.core.delete_id();
    }

    /// set the current user's wallpaper fd
    ///
    /// # Arguments
    ///
    /// - `fd`: wallpaper file fd
    /// - `metadata`: file related metadata information
    #[inline]
    fn handle_set_fd(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWallpaperContextV1>,
        fd: &Rc<OwnedFd>,
        metadata: &str,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_set_fd(
            fd,
            metadata,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_wallpaper_context_v1.set_fd", &e);
        }
    }

    /// identifier for the application window
    ///
    /// # Arguments
    ///
    /// - `identifier`: Identifier for the application window
    #[inline]
    fn handle_set_identifier(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWallpaperContextV1>,
        identifier: &str,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_set_identifier(
            identifier,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_wallpaper_context_v1.set_identifier", &e);
        }
    }

    /// configure xdg desktop portal options
    ///
    /// # Arguments
    ///
    /// - `output`: system output name
    #[inline]
    fn handle_set_output(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWallpaperContextV1>,
        output: &str,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_set_output(
            output,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_wallpaper_context_v1.set_output", &e);
        }
    }

    /// configure xdg desktop portal options
    ///
    /// # Arguments
    ///
    /// - `options`: xdg desktop portal options
    #[inline]
    fn handle_set_on(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWallpaperContextV1>,
        options: TreelandPersonalizationWallpaperContextV1Options,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_set_on(
            options,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_wallpaper_context_v1.set_on", &e);
        }
    }

    /// Set whether the current wallpaper is dark
    ///
    /// # Arguments
    ///
    /// - `isdark`: is dark
    #[inline]
    fn handle_set_isdark(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWallpaperContextV1>,
        isdark: u32,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_set_isdark(
            isdark,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_wallpaper_context_v1.set_isdark", &e);
        }
    }

    /// commit configuration
    #[inline]
    fn handle_commit(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWallpaperContextV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_commit(
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_wallpaper_context_v1.commit", &e);
        }
    }

    /// get user save meta data
    ///
    /// get the current user's wallpaper
    #[inline]
    fn handle_get_metadata(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWallpaperContextV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_get_metadata(
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_wallpaper_context_v1.get_metadata", &e);
        }
    }

    /// destroy the context object
    ///
    /// Destroy the context object.
    #[inline]
    fn handle_destroy(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWallpaperContextV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_wallpaper_context_v1.destroy", &e);
        }
    }

    /// get metadata event
    ///
    /// Send this signal after getting the user's wallpaper.
    ///
    /// # Arguments
    ///
    /// - `metadata`: user meta data
    #[inline]
    fn handle_metadata(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWallpaperContextV1>,
        metadata: &str,
    ) {
        if !_slf.core.forward_to_client.get() {
            return;
        }
        let res = _slf.try_send_metadata(
            metadata,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_wallpaper_context_v1.metadata", &e);
        }
    }
}

impl ObjectPrivate for TreelandPersonalizationWallpaperContextV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::TreelandPersonalizationWallpaperContextV1, version),
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
                let arg1 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("metadata"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("metadata"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("metadata"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("metadata"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                let Some(arg0) = fds.pop_front() else {
                    return Err(ObjectError::MissingFd("fd"));
                };
                let arg0 = &arg0;
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_wallpaper_context_v1#{}.set_fd(fd: {}, metadata: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0.as_raw_fd(), arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_fd(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_fd(&self, arg0, arg1);
                }
            }
            1 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("identifier"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("identifier"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("identifier"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("identifier"));
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_wallpaper_context_v1#{}.set_identifier(identifier: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_identifier(&self, arg0);
                } else {
                    DefaultHandler.handle_set_identifier(&self, arg0);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("output"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("output"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("output"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("output"));
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_wallpaper_context_v1#{}.set_output(output: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_output(&self, arg0);
                } else {
                    DefaultHandler.handle_set_output(&self, arg0);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = TreelandPersonalizationWallpaperContextV1Options(arg0);
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: TreelandPersonalizationWallpaperContextV1Options) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_wallpaper_context_v1#{}.set_on(options: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_on(&self, arg0);
                } else {
                    DefaultHandler.handle_set_on(&self, arg0);
                }
            }
            4 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_wallpaper_context_v1#{}.set_isdark(isdark: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_isdark(&self, arg0);
                } else {
                    DefaultHandler.handle_set_isdark(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_wallpaper_context_v1#{}.commit()\n", client_id, id);
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
            6 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_wallpaper_context_v1#{}.get_metadata()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_get_metadata(&self);
                } else {
                    DefaultHandler.handle_get_metadata(&self);
                }
            }
            7 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_wallpaper_context_v1#{}.destroy()\n", client_id, id);
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
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("metadata"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("metadata"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("metadata"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("metadata"));
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_personalization_wallpaper_context_v1#{}.metadata(metadata: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_metadata(&self, arg0);
                } else {
                    DefaultHandler.handle_metadata(&self, arg0);
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
            0 => "set_fd",
            1 => "set_identifier",
            2 => "set_output",
            3 => "set_on",
            4 => "set_isdark",
            5 => "commit",
            6 => "get_metadata",
            7 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "metadata",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for TreelandPersonalizationWallpaperContextV1 {
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

impl TreelandPersonalizationWallpaperContextV1 {
    /// Since when the options.preview enum variant is available.
    pub const ENM__OPTIONS_PREVIEW__SINCE: u32 = 1;
    /// Since when the options.background enum variant is available.
    pub const ENM__OPTIONS_BACKGROUND__SINCE: u32 = 1;
    /// Since when the options.lockscreen enum variant is available.
    pub const ENM__OPTIONS_LOCKSCREEN__SINCE: u32 = 1;
}

/// xdg desktop portal supported keys
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TreelandPersonalizationWallpaperContextV1Options(pub u32);

impl TreelandPersonalizationWallpaperContextV1Options {
    /// whether to show a preview of the picture
    pub const PREVIEW: Self = Self(1);

    /// configure screen background
    pub const BACKGROUND: Self = Self(2);

    /// configure screen wallpaper
    pub const LOCKSCREEN: Self = Self(4);
}

impl Debug for TreelandPersonalizationWallpaperContextV1Options {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::PREVIEW => "PREVIEW",
            Self::BACKGROUND => "BACKGROUND",
            Self::LOCKSCREEN => "LOCKSCREEN",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
