//! handle dock preview
//!
//! This interface allows dock set windows preview.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A treeland_dock_preview_context_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandDockPreviewContextV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn TreelandDockPreviewContextV1Handler>,
}

struct DefaultHandler;

impl TreelandDockPreviewContextV1Handler for DefaultHandler { }

impl TreelandDockPreviewContextV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "treeland_dock_preview_context_v1";
}

impl TreelandDockPreviewContextV1 {
    pub fn set_handler(&self, handler: impl TreelandDockPreviewContextV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandDockPreviewContextV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandDockPreviewContextV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandDockPreviewContextV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandDockPreviewContextV1 {
    /// Since when the enter message is available.
    pub const MSG__ENTER__SINCE: u32 = 1;

    /// enter preview box
    ///
    /// This event is sent after mouse enter preview box.
    #[inline]
    pub fn send_enter(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_dock_preview_context_v1#{}.enter()\n", client.endpoint.id, id);
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
        Ok(())
    }

    /// Since when the leave message is available.
    pub const MSG__LEAVE__SINCE: u32 = 1;

    /// leave preview box
    ///
    /// This event is sent after mouse leave preview box.
    #[inline]
    pub fn send_leave(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_dock_preview_context_v1#{}.leave()\n", client.endpoint.id, id);
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
        Ok(())
    }

    /// Since when the show message is available.
    pub const MSG__SHOW__SINCE: u32 = 1;

    /// set preview surfaces
    ///
    /// X and Y are relative to the relative_surface.
    /// surfaces wl_array is identifiers.
    ///
    /// # Arguments
    ///
    /// - `surfaces`:
    /// - `x`:
    /// - `y`:
    /// - `direction`:
    #[inline]
    pub fn send_show(
        &self,
        surfaces: &[u8],
        x: i32,
        y: i32,
        direction: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            surfaces,
            x,
            y,
            direction,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_dock_preview_context_v1#{}.show(surfaces: {}, x: {}, y: {}, direction: {})\n", id, debug_array(arg0), arg1, arg2, arg3);
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
        fmt.array(arg0);
        fmt.words([
            arg1 as u32,
            arg2 as u32,
            arg3,
        ]);
        Ok(())
    }

    /// Since when the show_tooltip message is available.
    pub const MSG__SHOW_TOOLTIP__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `tooltip`:
    /// - `x`:
    /// - `y`:
    /// - `direction`:
    #[inline]
    pub fn send_show_tooltip(
        &self,
        tooltip: &str,
        x: i32,
        y: i32,
        direction: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            tooltip,
            x,
            y,
            direction,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_dock_preview_context_v1#{}.show_tooltip(tooltip: {:?}, x: {}, y: {}, direction: {})\n", id, arg0, arg1, arg2, arg3);
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
        fmt.words([
            arg1 as u32,
            arg2 as u32,
            arg3,
        ]);
        Ok(())
    }

    /// Since when the close message is available.
    pub const MSG__CLOSE__SINCE: u32 = 1;

    /// close preview box
    ///
    /// close preview box
    #[inline]
    pub fn send_close(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_dock_preview_context_v1#{}.close()\n", id);
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_dock_preview_context_v1#{}.destroy()\n", id);
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
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [TreelandDockPreviewContextV1] proxies.
pub trait TreelandDockPreviewContextV1Handler: Any {
    /// enter preview box
    ///
    /// This event is sent after mouse enter preview box.
    #[inline]
    fn enter(
        &mut self,
        _slf: &Rc<TreelandDockPreviewContextV1>,
    ) {
        let res = _slf.send_enter(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_dock_preview_context_v1.enter message: {}", Report::new(e));
        }
    }

    /// leave preview box
    ///
    /// This event is sent after mouse leave preview box.
    #[inline]
    fn leave(
        &mut self,
        _slf: &Rc<TreelandDockPreviewContextV1>,
    ) {
        let res = _slf.send_leave(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_dock_preview_context_v1.leave message: {}", Report::new(e));
        }
    }

    /// set preview surfaces
    ///
    /// X and Y are relative to the relative_surface.
    /// surfaces wl_array is identifiers.
    ///
    /// # Arguments
    ///
    /// - `surfaces`:
    /// - `x`:
    /// - `y`:
    /// - `direction`:
    #[inline]
    fn show(
        &mut self,
        _slf: &Rc<TreelandDockPreviewContextV1>,
        surfaces: &[u8],
        x: i32,
        y: i32,
        direction: u32,
    ) {
        let res = _slf.send_show(
            surfaces,
            x,
            y,
            direction,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_dock_preview_context_v1.show message: {}", Report::new(e));
        }
    }

    /// # Arguments
    ///
    /// - `tooltip`:
    /// - `x`:
    /// - `y`:
    /// - `direction`:
    #[inline]
    fn show_tooltip(
        &mut self,
        _slf: &Rc<TreelandDockPreviewContextV1>,
        tooltip: &str,
        x: i32,
        y: i32,
        direction: u32,
    ) {
        let res = _slf.send_show_tooltip(
            tooltip,
            x,
            y,
            direction,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_dock_preview_context_v1.show_tooltip message: {}", Report::new(e));
        }
    }

    /// close preview box
    ///
    /// close preview box
    #[inline]
    fn close(
        &mut self,
        _slf: &Rc<TreelandDockPreviewContextV1>,
    ) {
        let res = _slf.send_close(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_dock_preview_context_v1.close message: {}", Report::new(e));
        }
    }

    /// destroy the context object
    ///
    /// Destroy the context object.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<TreelandDockPreviewContextV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_dock_preview_context_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for TreelandDockPreviewContextV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::TreelandDockPreviewContextV1, version),
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
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("surfaces"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("surfaces"));
                    }
                    let start = offset;
                    offset += words;
                    &uapi::as_bytes(&msg[start..])[..len]
                };
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("x"));
                };
                offset += 1;
                let Some(&arg2) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("y"));
                };
                offset += 1;
                let Some(&arg3) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("direction"));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_dock_preview_context_v1#{}.show(surfaces: {}, x: {}, y: {}, direction: {})\n", client.endpoint.id, msg[0], debug_array(arg0), arg1, arg2, arg3);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).show(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.show(&self, arg0, arg1, arg2, arg3);
                }
            }
            1 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("tooltip"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("tooltip"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("tooltip"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("tooltip"));
                        };
                        s
                    }
                };
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("x"));
                };
                offset += 1;
                let Some(&arg2) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("y"));
                };
                offset += 1;
                let Some(&arg3) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("direction"));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_dock_preview_context_v1#{}.show_tooltip(tooltip: {:?}, x: {}, y: {}, direction: {})\n", client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).show_tooltip(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.show_tooltip(&self, arg0, arg1, arg2, arg3);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_dock_preview_context_v1#{}.close()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).close(&self);
                } else {
                    DefaultHandler.close(&self);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_dock_preview_context_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_dock_preview_context_v1#{}.enter()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).enter(&self);
                } else {
                    DefaultHandler.enter(&self);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_dock_preview_context_v1#{}.leave()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).leave(&self);
                } else {
                    DefaultHandler.leave(&self);
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
            0 => "show",
            1 => "show_tooltip",
            2 => "close",
            3 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "enter",
            1 => "leave",
            _ => return None,
        };
        Some(name)
    }
}

impl Proxy for TreelandDockPreviewContextV1 {
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

impl TreelandDockPreviewContextV1 {
    /// Since when the direction.top enum variant is available.
    pub const ENM__DIRECTION_TOP__SINCE: u32 = 1;
    /// Since when the direction.right enum variant is available.
    pub const ENM__DIRECTION_RIGHT__SINCE: u32 = 1;
    /// Since when the direction.bottom enum variant is available.
    pub const ENM__DIRECTION_BOTTOM__SINCE: u32 = 1;
    /// Since when the direction.left enum variant is available.
    pub const ENM__DIRECTION_LEFT__SINCE: u32 = 1;
}

/// types of direction on the dock preview
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TreelandDockPreviewContextV1Direction(pub u32);

impl TreelandDockPreviewContextV1Direction {
    /// top
    pub const TOP: Self = Self(0);

    /// right
    pub const RIGHT: Self = Self(1);

    /// bottom
    pub const BOTTOM: Self = Self(2);

    /// left
    pub const LEFT: Self = Self(3);
}

impl Debug for TreelandDockPreviewContextV1Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::TOP => "TOP",
            Self::RIGHT => "RIGHT",
            Self::BOTTOM => "BOTTOM",
            Self::LEFT => "LEFT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
