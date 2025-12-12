use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A treeland_wallpaper_color_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandWallpaperColorManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn TreelandWallpaperColorManagerV1Handler>,
}

struct DefaultHandler;

impl TreelandWallpaperColorManagerV1Handler for DefaultHandler { }

impl TreelandWallpaperColorManagerV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::TreelandWallpaperColorManagerV1;
    pub const INTERFACE_NAME: &str = "treeland_wallpaper_color_manager_v1";
}

impl TreelandWallpaperColorManagerV1 {
    pub fn set_handler(&self, handler: impl TreelandWallpaperColorManagerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandWallpaperColorManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandWallpaperColorManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandWallpaperColorManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandWallpaperColorManagerV1 {
    /// Since when the watch message is available.
    pub const MSG__WATCH__SINCE: u32 = 1;

    /// watch wallpaper color
    ///
    /// Monitor the wallpaper color of a given screen.
    ///
    /// # Arguments
    ///
    /// - `output`: system output name
    #[inline]
    pub fn send_watch(
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_wallpaper_color_manager_v1#{}.watch(output: {:?})\n", id, arg0);
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
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the unwatch message is available.
    pub const MSG__UNWATCH__SINCE: u32 = 1;

    /// unwatch wallpaper color
    ///
    /// Stop monitor the wallpaper color for the given screen.
    ///
    /// # Arguments
    ///
    /// - `output`: system output name
    #[inline]
    pub fn send_unwatch(
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_wallpaper_color_manager_v1#{}.unwatch(output: {:?})\n", id, arg0);
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

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the context object
    ///
    /// The client no longer cares about wallpaper_color.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_wallpaper_color_manager_v1#{}.destroy()\n", id);
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
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the output_color message is available.
    pub const MSG__OUTPUT_COLOR__SINCE: u32 = 1;

    /// output color changed
    ///
    /// Tell the client that the wallpaper color of the screen it is monitoring has changed.
    /// This event will also be sent immediately when the client requests a watch.
    ///
    /// # Arguments
    ///
    /// - `output`: system output name
    /// - `isdark`:
    #[inline]
    pub fn send_output_color(
        &self,
        output: &str,
        isdark: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            output,
            isdark,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_wallpaper_color_manager_v1#{}.output_color(output: {:?}, isdark: {})\n", client.endpoint.id, id, arg0, arg1);
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
        fmt.words([
            arg1,
        ]);
        Ok(())
    }
}

/// A message handler for [TreelandWallpaperColorManagerV1] proxies.
pub trait TreelandWallpaperColorManagerV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<TreelandWallpaperColorManagerV1>) {
        let _ = slf.core.delete_id();
    }

    /// watch wallpaper color
    ///
    /// Monitor the wallpaper color of a given screen.
    ///
    /// # Arguments
    ///
    /// - `output`: system output name
    #[inline]
    fn handle_watch(
        &mut self,
        _slf: &Rc<TreelandWallpaperColorManagerV1>,
        output: &str,
    ) {
        let res = _slf.send_watch(
            output,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_wallpaper_color_manager_v1.watch message: {}", Report::new(e));
        }
    }

    /// unwatch wallpaper color
    ///
    /// Stop monitor the wallpaper color for the given screen.
    ///
    /// # Arguments
    ///
    /// - `output`: system output name
    #[inline]
    fn handle_unwatch(
        &mut self,
        _slf: &Rc<TreelandWallpaperColorManagerV1>,
        output: &str,
    ) {
        let res = _slf.send_unwatch(
            output,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_wallpaper_color_manager_v1.unwatch message: {}", Report::new(e));
        }
    }

    /// destroy the context object
    ///
    /// The client no longer cares about wallpaper_color.
    #[inline]
    fn handle_destroy(
        &mut self,
        _slf: &Rc<TreelandWallpaperColorManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_wallpaper_color_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// output color changed
    ///
    /// Tell the client that the wallpaper color of the screen it is monitoring has changed.
    /// This event will also be sent immediately when the client requests a watch.
    ///
    /// # Arguments
    ///
    /// - `output`: system output name
    /// - `isdark`:
    #[inline]
    fn handle_output_color(
        &mut self,
        _slf: &Rc<TreelandWallpaperColorManagerV1>,
        output: &str,
        isdark: u32,
    ) {
        let res = _slf.send_output_color(
            output,
            isdark,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_wallpaper_color_manager_v1.output_color message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for TreelandWallpaperColorManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::TreelandWallpaperColorManagerV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_wallpaper_color_manager_v1#{}.watch(output: {:?})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_watch(&self, arg0);
                } else {
                    DefaultHandler.handle_watch(&self, arg0);
                }
            }
            1 => {
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_wallpaper_color_manager_v1#{}.unwatch(output: {:?})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_unwatch(&self, arg0);
                } else {
                    DefaultHandler.handle_unwatch(&self, arg0);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_wallpaper_color_manager_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
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
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("isdark"));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_wallpaper_color_manager_v1#{}.output_color(output: {:?}, isdark: {})\n", msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_output_color(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_output_color(&self, arg0, arg1);
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
            0 => "watch",
            1 => "unwatch",
            2 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "output_color",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for TreelandWallpaperColorManagerV1 {
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

