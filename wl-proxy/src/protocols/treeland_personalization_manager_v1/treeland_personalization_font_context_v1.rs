//! treeland window global font settings
//!
//! This interface allows set treeland window global font settings.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A treeland_personalization_font_context_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandPersonalizationFontContextV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn TreelandPersonalizationFontContextV1Handler>,
}

struct DefaultHandler;

impl TreelandPersonalizationFontContextV1Handler for DefaultHandler { }

impl TreelandPersonalizationFontContextV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "treeland_personalization_font_context_v1";
}

impl TreelandPersonalizationFontContextV1 {
    pub fn set_handler(&self, handler: impl TreelandPersonalizationFontContextV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandPersonalizationFontContextV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandPersonalizationFontContextV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandPersonalizationFontContextV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandPersonalizationFontContextV1 {
    /// Since when the font message is available.
    pub const MSG__FONT__SINCE: u32 = 1;

    /// font event
    ///
    /// Send this signal after setting the system font.
    ///
    /// # Arguments
    ///
    /// - `font_name`: font name
    #[inline]
    pub fn send_font(
        &self,
        font_name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            font_name,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_personalization_font_context_v1#{}.font(font_name: {:?})\n", client.endpoint.id, id, arg0);
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

    /// Since when the monospace_font message is available.
    pub const MSG__MONOSPACE_FONT__SINCE: u32 = 1;

    /// monospace font event
    ///
    /// Send this signal after setting the system monospace font.
    ///
    /// # Arguments
    ///
    /// - `font_name`: monospace font name
    #[inline]
    pub fn send_monospace_font(
        &self,
        font_name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            font_name,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_personalization_font_context_v1#{}.monospace_font(font_name: {:?})\n", client.endpoint.id, id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the font_size message is available.
    pub const MSG__FONT_SIZE__SINCE: u32 = 1;

    /// font size
    ///
    /// Send this signal after setting the system font size.
    ///
    /// # Arguments
    ///
    /// - `font_size`: font size
    #[inline]
    pub fn send_font_size(
        &self,
        font_size: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            font_size,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_personalization_font_context_v1#{}.font_size(font_size: {})\n", client.endpoint.id, id, arg0);
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
            2,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the set_font_size message is available.
    pub const MSG__SET_FONT_SIZE__SINCE: u32 = 1;

    /// set system font size
    ///
    /// Set the system font size
    ///
    /// # Arguments
    ///
    /// - `size`: font size
    #[inline]
    pub fn send_set_font_size(
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
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_font_context_v1#{}.set_font_size(size: {})\n", id, arg0);
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
            arg0,
        ]);
        Ok(())
    }

    /// Since when the get_font_size message is available.
    pub const MSG__GET_FONT_SIZE__SINCE: u32 = 1;

    /// get system font size
    ///
    /// Get the system font size
    #[inline]
    pub fn send_get_font_size(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_font_context_v1#{}.get_font_size()\n", id);
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

    /// Since when the set_font message is available.
    pub const MSG__SET_FONT__SINCE: u32 = 1;

    /// set system font
    ///
    /// Set the system font
    ///
    /// # Arguments
    ///
    /// - `font_name`: font name
    #[inline]
    pub fn send_set_font(
        &self,
        font_name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            font_name,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_font_context_v1#{}.set_font(font_name: {:?})\n", id, arg0);
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

    /// Since when the get_font message is available.
    pub const MSG__GET_FONT__SINCE: u32 = 1;

    /// get system font
    ///
    /// Get the system font
    #[inline]
    pub fn send_get_font(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_font_context_v1#{}.get_font()\n", id);
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
        ]);
        Ok(())
    }

    /// Since when the set_monospace_font message is available.
    pub const MSG__SET_MONOSPACE_FONT__SINCE: u32 = 1;

    /// set system monospace font
    ///
    /// Set the system monospace font
    ///
    /// # Arguments
    ///
    /// - `font_name`: monospace font name
    #[inline]
    pub fn send_set_monospace_font(
        &self,
        font_name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            font_name,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_font_context_v1#{}.set_monospace_font(font_name: {:?})\n", id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the get_monospace_font message is available.
    pub const MSG__GET_MONOSPACE_FONT__SINCE: u32 = 1;

    /// get system monospace font
    ///
    /// Get the system monospace font
    #[inline]
    pub fn send_get_monospace_font(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_font_context_v1#{}.get_monospace_font()\n", id);
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_font_context_v1#{}.destroy()\n", id);
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
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [TreelandPersonalizationFontContextV1] proxies.
pub trait TreelandPersonalizationFontContextV1Handler: Any {
    /// font event
    ///
    /// Send this signal after setting the system font.
    ///
    /// # Arguments
    ///
    /// - `font_name`: font name
    #[inline]
    fn font(
        &mut self,
        _slf: &Rc<TreelandPersonalizationFontContextV1>,
        font_name: &str,
    ) {
        let res = _slf.send_font(
            font_name,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_font_context_v1.font message: {}", Report::new(e));
        }
    }

    /// monospace font event
    ///
    /// Send this signal after setting the system monospace font.
    ///
    /// # Arguments
    ///
    /// - `font_name`: monospace font name
    #[inline]
    fn monospace_font(
        &mut self,
        _slf: &Rc<TreelandPersonalizationFontContextV1>,
        font_name: &str,
    ) {
        let res = _slf.send_monospace_font(
            font_name,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_font_context_v1.monospace_font message: {}", Report::new(e));
        }
    }

    /// font size
    ///
    /// Send this signal after setting the system font size.
    ///
    /// # Arguments
    ///
    /// - `font_size`: font size
    #[inline]
    fn font_size(
        &mut self,
        _slf: &Rc<TreelandPersonalizationFontContextV1>,
        font_size: u32,
    ) {
        let res = _slf.send_font_size(
            font_size,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_font_context_v1.font_size message: {}", Report::new(e));
        }
    }

    /// set system font size
    ///
    /// Set the system font size
    ///
    /// # Arguments
    ///
    /// - `size`: font size
    #[inline]
    fn set_font_size(
        &mut self,
        _slf: &Rc<TreelandPersonalizationFontContextV1>,
        size: u32,
    ) {
        let res = _slf.send_set_font_size(
            size,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_font_context_v1.set_font_size message: {}", Report::new(e));
        }
    }

    /// get system font size
    ///
    /// Get the system font size
    #[inline]
    fn get_font_size(
        &mut self,
        _slf: &Rc<TreelandPersonalizationFontContextV1>,
    ) {
        let res = _slf.send_get_font_size(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_font_context_v1.get_font_size message: {}", Report::new(e));
        }
    }

    /// set system font
    ///
    /// Set the system font
    ///
    /// # Arguments
    ///
    /// - `font_name`: font name
    #[inline]
    fn set_font(
        &mut self,
        _slf: &Rc<TreelandPersonalizationFontContextV1>,
        font_name: &str,
    ) {
        let res = _slf.send_set_font(
            font_name,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_font_context_v1.set_font message: {}", Report::new(e));
        }
    }

    /// get system font
    ///
    /// Get the system font
    #[inline]
    fn get_font(
        &mut self,
        _slf: &Rc<TreelandPersonalizationFontContextV1>,
    ) {
        let res = _slf.send_get_font(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_font_context_v1.get_font message: {}", Report::new(e));
        }
    }

    /// set system monospace font
    ///
    /// Set the system monospace font
    ///
    /// # Arguments
    ///
    /// - `font_name`: monospace font name
    #[inline]
    fn set_monospace_font(
        &mut self,
        _slf: &Rc<TreelandPersonalizationFontContextV1>,
        font_name: &str,
    ) {
        let res = _slf.send_set_monospace_font(
            font_name,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_font_context_v1.set_monospace_font message: {}", Report::new(e));
        }
    }

    /// get system monospace font
    ///
    /// Get the system monospace font
    #[inline]
    fn get_monospace_font(
        &mut self,
        _slf: &Rc<TreelandPersonalizationFontContextV1>,
    ) {
        let res = _slf.send_get_monospace_font(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_font_context_v1.get_monospace_font message: {}", Report::new(e));
        }
    }

    /// destroy the context object
    ///
    /// Destroy the context object.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<TreelandPersonalizationFontContextV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_font_context_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for TreelandPersonalizationFontContextV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::TreelandPersonalizationFontContextV1, version),
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_font_context_v1#{}.set_font_size(size: {})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_font_size(&self, arg0);
                } else {
                    DefaultHandler.set_font_size(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_font_context_v1#{}.get_font_size()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).get_font_size(&self);
                } else {
                    DefaultHandler.get_font_size(&self);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("font_name"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("font_name"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("font_name"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("font_name"));
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_font_context_v1#{}.set_font(font_name: {:?})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_font(&self, arg0);
                } else {
                    DefaultHandler.set_font(&self, arg0);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_font_context_v1#{}.get_font()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).get_font(&self);
                } else {
                    DefaultHandler.get_font(&self);
                }
            }
            4 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("font_name"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("font_name"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("font_name"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("font_name"));
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_font_context_v1#{}.set_monospace_font(font_name: {:?})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_monospace_font(&self, arg0);
                } else {
                    DefaultHandler.set_monospace_font(&self, arg0);
                }
            }
            5 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_font_context_v1#{}.get_monospace_font()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).get_monospace_font(&self);
                } else {
                    DefaultHandler.get_monospace_font(&self);
                }
            }
            6 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_font_context_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
                        return Err(ObjectError::MissingArgument("font_name"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("font_name"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("font_name"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("font_name"));
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_personalization_font_context_v1#{}.font(font_name: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).font(&self, arg0);
                } else {
                    DefaultHandler.font(&self, arg0);
                }
            }
            1 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("font_name"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("font_name"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("font_name"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("font_name"));
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_personalization_font_context_v1#{}.monospace_font(font_name: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).monospace_font(&self, arg0);
                } else {
                    DefaultHandler.monospace_font(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_personalization_font_context_v1#{}.font_size(font_size: {})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).font_size(&self, arg0);
                } else {
                    DefaultHandler.font_size(&self, arg0);
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
            0 => "set_font_size",
            1 => "get_font_size",
            2 => "set_font",
            3 => "get_font",
            4 => "set_monospace_font",
            5 => "get_monospace_font",
            6 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "font",
            1 => "monospace_font",
            2 => "font_size",
            _ => return None,
        };
        Some(name)
    }
}

impl Proxy for TreelandPersonalizationFontContextV1 {
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

