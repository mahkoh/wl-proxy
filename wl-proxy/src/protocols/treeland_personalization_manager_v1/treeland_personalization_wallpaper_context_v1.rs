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

/// A treeland_personalization_wallpaper_context_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandPersonalizationWallpaperContextV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn TreelandPersonalizationWallpaperContextV1Handler>,
}

struct DefaultHandler;

impl TreelandPersonalizationWallpaperContextV1Handler for DefaultHandler { }

impl TreelandPersonalizationWallpaperContextV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "treeland_personalization_wallpaper_context_v1";
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
    pub fn send_set_fd(
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
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_wallpaper_context_v1#{}.set_fd(fd: {}, metadata: {:?})\n", id, arg0.as_raw_fd(), arg1);
            self.core.state.log(args);
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

    /// Since when the set_identifier message is available.
    pub const MSG__SET_IDENTIFIER__SINCE: u32 = 1;

    /// identifier for the application window
    ///
    /// # Arguments
    ///
    /// - `identifier`: Identifier for the application window
    #[inline]
    pub fn send_set_identifier(
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
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_wallpaper_context_v1#{}.set_identifier(identifier: {:?})\n", id, arg0);
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
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the set_output message is available.
    pub const MSG__SET_OUTPUT__SINCE: u32 = 1;

    /// configure xdg desktop portal options
    ///
    /// # Arguments
    ///
    /// - `output`: system output name
    #[inline]
    pub fn send_set_output(
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
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_wallpaper_context_v1#{}.set_output(output: {:?})\n", id, arg0);
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
        Ok(())
    }

    /// Since when the set_on message is available.
    pub const MSG__SET_ON__SINCE: u32 = 1;

    /// configure xdg desktop portal options
    ///
    /// # Arguments
    ///
    /// - `options`: xdg desktop portal options
    #[inline]
    pub fn send_set_on(
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
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_wallpaper_context_v1#{}.set_on(options: {:?})\n", id, arg0);
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
            3,
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the set_isdark message is available.
    pub const MSG__SET_ISDARK__SINCE: u32 = 1;

    /// Set whether the current wallpaper is dark
    ///
    /// # Arguments
    ///
    /// - `isdark`: is dark
    #[inline]
    pub fn send_set_isdark(
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
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_wallpaper_context_v1#{}.set_isdark(isdark: {})\n", id, arg0);
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
            4,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the commit message is available.
    pub const MSG__COMMIT__SINCE: u32 = 1;

    /// commit configuration
    #[inline]
    pub fn send_commit(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_wallpaper_context_v1#{}.commit()\n", id);
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
            5,
        ]);
        Ok(())
    }

    /// Since when the get_metadata message is available.
    pub const MSG__GET_METADATA__SINCE: u32 = 1;

    /// get user save meta data
    ///
    /// get the current user's wallpaper
    #[inline]
    pub fn send_get_metadata(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_wallpaper_context_v1#{}.get_metadata()\n", id);
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
            6,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the context object
    ///
    /// Destroy the context object.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_wallpaper_context_v1#{}.destroy()\n", id);
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
            7,
        ]);
        self.core.handle_server_destroy();
        Ok(())
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
    pub fn send_metadata(
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
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_personalization_wallpaper_context_v1#{}.metadata(metadata: {:?})\n", client.endpoint.id, id, arg0);
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
        Ok(())
    }
}

/// A message handler for [TreelandPersonalizationWallpaperContextV1] proxies.
pub trait TreelandPersonalizationWallpaperContextV1Handler: Any {
    /// set the current user's wallpaper fd
    ///
    /// # Arguments
    ///
    /// - `fd`: wallpaper file fd
    /// - `metadata`: file related metadata information
    #[inline]
    fn set_fd(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWallpaperContextV1>,
        fd: &Rc<OwnedFd>,
        metadata: &str,
    ) {
        let res = _slf.send_set_fd(
            fd,
            metadata,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_wallpaper_context_v1.set_fd message: {}", Report::new(e));
        }
    }

    /// identifier for the application window
    ///
    /// # Arguments
    ///
    /// - `identifier`: Identifier for the application window
    #[inline]
    fn set_identifier(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWallpaperContextV1>,
        identifier: &str,
    ) {
        let res = _slf.send_set_identifier(
            identifier,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_wallpaper_context_v1.set_identifier message: {}", Report::new(e));
        }
    }

    /// configure xdg desktop portal options
    ///
    /// # Arguments
    ///
    /// - `output`: system output name
    #[inline]
    fn set_output(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWallpaperContextV1>,
        output: &str,
    ) {
        let res = _slf.send_set_output(
            output,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_wallpaper_context_v1.set_output message: {}", Report::new(e));
        }
    }

    /// configure xdg desktop portal options
    ///
    /// # Arguments
    ///
    /// - `options`: xdg desktop portal options
    #[inline]
    fn set_on(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWallpaperContextV1>,
        options: TreelandPersonalizationWallpaperContextV1Options,
    ) {
        let res = _slf.send_set_on(
            options,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_wallpaper_context_v1.set_on message: {}", Report::new(e));
        }
    }

    /// Set whether the current wallpaper is dark
    ///
    /// # Arguments
    ///
    /// - `isdark`: is dark
    #[inline]
    fn set_isdark(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWallpaperContextV1>,
        isdark: u32,
    ) {
        let res = _slf.send_set_isdark(
            isdark,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_wallpaper_context_v1.set_isdark message: {}", Report::new(e));
        }
    }

    /// commit configuration
    #[inline]
    fn commit(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWallpaperContextV1>,
    ) {
        let res = _slf.send_commit(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_wallpaper_context_v1.commit message: {}", Report::new(e));
        }
    }

    /// get user save meta data
    ///
    /// get the current user's wallpaper
    #[inline]
    fn get_metadata(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWallpaperContextV1>,
    ) {
        let res = _slf.send_get_metadata(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_wallpaper_context_v1.get_metadata message: {}", Report::new(e));
        }
    }

    /// destroy the context object
    ///
    /// Destroy the context object.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWallpaperContextV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_wallpaper_context_v1.destroy message: {}", Report::new(e));
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
    fn metadata(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWallpaperContextV1>,
        metadata: &str,
    ) {
        let res = _slf.send_metadata(
            metadata,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_wallpaper_context_v1.metadata message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for TreelandPersonalizationWallpaperContextV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::TreelandPersonalizationWallpaperContextV1, version),
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
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_wallpaper_context_v1#{}.set_fd(fd: {}, metadata: {:?})\n", client.endpoint.id, msg[0], arg0.as_raw_fd(), arg1);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_fd(&self, arg0, arg1);
                } else {
                    DefaultHandler.set_fd(&self, arg0, arg1);
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
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_wallpaper_context_v1#{}.set_identifier(identifier: {:?})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_identifier(&self, arg0);
                } else {
                    DefaultHandler.set_identifier(&self, arg0);
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
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_wallpaper_context_v1#{}.set_output(output: {:?})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_output(&self, arg0);
                } else {
                    DefaultHandler.set_output(&self, arg0);
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
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_wallpaper_context_v1#{}.set_on(options: {:?})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_on(&self, arg0);
                } else {
                    DefaultHandler.set_on(&self, arg0);
                }
            }
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_wallpaper_context_v1#{}.set_isdark(isdark: {})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_isdark(&self, arg0);
                } else {
                    DefaultHandler.set_isdark(&self, arg0);
                }
            }
            5 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_wallpaper_context_v1#{}.commit()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).commit(&self);
                } else {
                    DefaultHandler.commit(&self);
                }
            }
            6 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_wallpaper_context_v1#{}.get_metadata()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).get_metadata(&self);
                } else {
                    DefaultHandler.get_metadata(&self);
                }
            }
            7 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_wallpaper_context_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_personalization_wallpaper_context_v1#{}.metadata(metadata: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).metadata(&self, arg0);
                } else {
                    DefaultHandler.metadata(&self, arg0);
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

impl Proxy for TreelandPersonalizationWallpaperContextV1 {
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
