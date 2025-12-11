//! client custom window context
//!
//! This interface allows a client personalization window.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A treeland_personalization_window_context_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandPersonalizationWindowContextV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn TreelandPersonalizationWindowContextV1Handler>,
}

struct DefaultHandler;

impl TreelandPersonalizationWindowContextV1Handler for DefaultHandler { }

impl TreelandPersonalizationWindowContextV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "treeland_personalization_window_context_v1";
}

impl TreelandPersonalizationWindowContextV1 {
    pub fn set_handler(&self, handler: impl TreelandPersonalizationWindowContextV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandPersonalizationWindowContextV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandPersonalizationWindowContextV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandPersonalizationWindowContextV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandPersonalizationWindowContextV1 {
    /// Since when the set_blend_mode message is available.
    pub const MSG__SET_BLEND_MODE__SINCE: u32 = 1;

    /// set window background blend mode
    ///
    /// Set window background blend mode
    ///
    /// # Arguments
    ///
    /// - `mode`:
    #[inline]
    pub fn send_set_blend_mode(
        &self,
        mode: TreelandPersonalizationWindowContextV1BlendMode,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            mode,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_window_context_v1#{}.set_blend_mode(mode: {:?})\n", id, arg0);
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
            arg0.0 as u32,
        ]);
        Ok(())
    }

    /// Since when the set_round_corner_radius message is available.
    pub const MSG__SET_ROUND_CORNER_RADIUS__SINCE: u32 = 1;

    /// set window round corner radius
    ///
    /// This request will set window round corner radius, invoking this request means user
    /// want to
    /// manage window round corner radius by itself. If not invoked, window round corner
    /// radius will
    /// be decided by compositor.
    ///
    /// # Arguments
    ///
    /// - `radius`:
    #[inline]
    pub fn send_set_round_corner_radius(
        &self,
        radius: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            radius,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_window_context_v1#{}.set_round_corner_radius(radius: {})\n", id, arg0);
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
            arg0 as u32,
        ]);
        Ok(())
    }

    /// Since when the set_shadow message is available.
    pub const MSG__SET_SHADOW__SINCE: u32 = 1;

    /// set window shadow
    ///
    /// Set window shadow's radius, offset and color, invoking this request indicates that
    /// client want to manage
    /// the window shadow by itself. If not invoked, window shadow will be decided by the
    /// compositor
    ///
    /// # Arguments
    ///
    /// - `radius`:
    /// - `offset_x`:
    /// - `offset_y`:
    /// - `r`:
    /// - `g`:
    /// - `b`:
    /// - `a`:
    #[inline]
    pub fn send_set_shadow(
        &self,
        radius: i32,
        offset_x: i32,
        offset_y: i32,
        r: i32,
        g: i32,
        b: i32,
        a: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
            arg6,
        ) = (
            radius,
            offset_x,
            offset_y,
            r,
            g,
            b,
            a,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_window_context_v1#{}.set_shadow(radius: {}, offset_x: {}, offset_y: {}, r: {}, g: {}, b: {}, a: {})\n", id, arg0, arg1, arg2, arg3, arg4, arg5, arg6);
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
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
            arg4 as u32,
            arg5 as u32,
            arg6 as u32,
        ]);
        Ok(())
    }

    /// Since when the set_border message is available.
    pub const MSG__SET_BORDER__SINCE: u32 = 1;

    /// set window border
    ///
    /// Set window border width and color
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `r`:
    /// - `g`:
    /// - `b`:
    /// - `a`:
    #[inline]
    pub fn send_set_border(
        &self,
        width: i32,
        r: i32,
        g: i32,
        b: i32,
        a: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ) = (
            width,
            r,
            g,
            b,
            a,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_window_context_v1#{}.set_border(width: {}, r: {}, g: {}, b: {}, a: {})\n", id, arg0, arg1, arg2, arg3, arg4);
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
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
            arg4 as u32,
        ]);
        Ok(())
    }

    /// Since when the set_titlebar message is available.
    pub const MSG__SET_TITLEBAR__SINCE: u32 = 1;

    /// set if system titlebar is enabled
    ///
    /// Set if system titlebar is enabled
    ///
    /// # Arguments
    ///
    /// - `mode`:
    #[inline]
    pub fn send_set_titlebar(
        &self,
        mode: TreelandPersonalizationWindowContextV1EnableMode,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            mode,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_window_context_v1#{}.set_titlebar(mode: {:?})\n", id, arg0);
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
            arg0.0 as u32,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_window_context_v1#{}.destroy()\n", id);
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
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [TreelandPersonalizationWindowContextV1] proxies.
pub trait TreelandPersonalizationWindowContextV1Handler: Any {
    /// set window background blend mode
    ///
    /// Set window background blend mode
    ///
    /// # Arguments
    ///
    /// - `mode`:
    #[inline]
    fn set_blend_mode(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWindowContextV1>,
        mode: TreelandPersonalizationWindowContextV1BlendMode,
    ) {
        let res = _slf.send_set_blend_mode(
            mode,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_window_context_v1.set_blend_mode message: {}", Report::new(e));
        }
    }

    /// set window round corner radius
    ///
    /// This request will set window round corner radius, invoking this request means user
    /// want to
    /// manage window round corner radius by itself. If not invoked, window round corner
    /// radius will
    /// be decided by compositor.
    ///
    /// # Arguments
    ///
    /// - `radius`:
    #[inline]
    fn set_round_corner_radius(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWindowContextV1>,
        radius: i32,
    ) {
        let res = _slf.send_set_round_corner_radius(
            radius,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_window_context_v1.set_round_corner_radius message: {}", Report::new(e));
        }
    }

    /// set window shadow
    ///
    /// Set window shadow's radius, offset and color, invoking this request indicates that
    /// client want to manage
    /// the window shadow by itself. If not invoked, window shadow will be decided by the
    /// compositor
    ///
    /// # Arguments
    ///
    /// - `radius`:
    /// - `offset_x`:
    /// - `offset_y`:
    /// - `r`:
    /// - `g`:
    /// - `b`:
    /// - `a`:
    #[inline]
    fn set_shadow(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWindowContextV1>,
        radius: i32,
        offset_x: i32,
        offset_y: i32,
        r: i32,
        g: i32,
        b: i32,
        a: i32,
    ) {
        let res = _slf.send_set_shadow(
            radius,
            offset_x,
            offset_y,
            r,
            g,
            b,
            a,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_window_context_v1.set_shadow message: {}", Report::new(e));
        }
    }

    /// set window border
    ///
    /// Set window border width and color
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `r`:
    /// - `g`:
    /// - `b`:
    /// - `a`:
    #[inline]
    fn set_border(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWindowContextV1>,
        width: i32,
        r: i32,
        g: i32,
        b: i32,
        a: i32,
    ) {
        let res = _slf.send_set_border(
            width,
            r,
            g,
            b,
            a,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_window_context_v1.set_border message: {}", Report::new(e));
        }
    }

    /// set if system titlebar is enabled
    ///
    /// Set if system titlebar is enabled
    ///
    /// # Arguments
    ///
    /// - `mode`:
    #[inline]
    fn set_titlebar(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWindowContextV1>,
        mode: TreelandPersonalizationWindowContextV1EnableMode,
    ) {
        let res = _slf.send_set_titlebar(
            mode,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_window_context_v1.set_titlebar message: {}", Report::new(e));
        }
    }

    /// destroy the context object
    ///
    /// Destroy the context object.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<TreelandPersonalizationWindowContextV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_window_context_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for TreelandPersonalizationWindowContextV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::TreelandPersonalizationWindowContextV1, version),
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
                let arg0 = TreelandPersonalizationWindowContextV1BlendMode(arg0);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_window_context_v1#{}.set_blend_mode(mode: {:?})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_blend_mode(&self, arg0);
                } else {
                    DefaultHandler.set_blend_mode(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = arg0 as i32;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_window_context_v1#{}.set_round_corner_radius(radius: {})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_round_corner_radius(&self, arg0);
                } else {
                    DefaultHandler.set_round_corner_radius(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                    arg5,
                    arg6,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 36));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                let arg4 = arg4 as i32;
                let arg5 = arg5 as i32;
                let arg6 = arg6 as i32;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_window_context_v1#{}.set_shadow(radius: {}, offset_x: {}, offset_y: {}, r: {}, g: {}, b: {}, a: {})\n", client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4, arg5, arg6);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_shadow(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6);
                } else {
                    DefaultHandler.set_shadow(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6);
                }
            }
            3 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 28));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                let arg4 = arg4 as i32;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_window_context_v1#{}.set_border(width: {}, r: {}, g: {}, b: {}, a: {})\n", client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_border(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultHandler.set_border(&self, arg0, arg1, arg2, arg3, arg4);
                }
            }
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = TreelandPersonalizationWindowContextV1EnableMode(arg0);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_window_context_v1#{}.set_titlebar(mode: {:?})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_titlebar(&self, arg0);
                } else {
                    DefaultHandler.set_titlebar(&self, arg0);
                }
            }
            5 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_window_context_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "set_blend_mode",
            1 => "set_round_corner_radius",
            2 => "set_shadow",
            3 => "set_border",
            4 => "set_titlebar",
            5 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Proxy for TreelandPersonalizationWindowContextV1 {
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

impl TreelandPersonalizationWindowContextV1 {
    /// Since when the blend_mode.transparent enum variant is available.
    pub const ENM__BLEND_MODE_TRANSPARENT__SINCE: u32 = 1;
    /// Since when the blend_mode.wallpaper enum variant is available.
    pub const ENM__BLEND_MODE_WALLPAPER__SINCE: u32 = 1;
    /// Since when the blend_mode.blur enum variant is available.
    pub const ENM__BLEND_MODE_BLUR__SINCE: u32 = 1;

    /// Since when the enable_mode.enable enum variant is available.
    pub const ENM__ENABLE_MODE_ENABLE__SINCE: u32 = 1;
    /// Since when the enable_mode.disable enum variant is available.
    pub const ENM__ENABLE_MODE_DISABLE__SINCE: u32 = 1;
}

/// window blend mode
///
/// Window blend mode defines how compositor composite window's surface over other
/// surfaces.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TreelandPersonalizationWindowContextV1BlendMode(pub u32);

impl TreelandPersonalizationWindowContextV1BlendMode {
    /// Normal blend mode, just composite over background with alpha channel
    pub const TRANSPARENT: Self = Self(0);

    /// Composite window over wallpaper
    pub const WALLPAPER: Self = Self(1);

    /// Blur the content of the window background
    pub const BLUR: Self = Self(2);
}

impl Debug for TreelandPersonalizationWindowContextV1BlendMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::TRANSPARENT => "TRANSPARENT",
            Self::WALLPAPER => "WALLPAPER",
            Self::BLUR => "BLUR",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// window enable mode
///
/// Set window enable mode
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TreelandPersonalizationWindowContextV1EnableMode(pub u32);

impl TreelandPersonalizationWindowContextV1EnableMode {
    /// force enable titlebar
    pub const ENABLE: Self = Self(0);

    /// force disable titlebar
    pub const DISABLE: Self = Self(1);
}

impl Debug for TreelandPersonalizationWindowContextV1EnableMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ENABLE => "ENABLE",
            Self::DISABLE => "DISABLE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
