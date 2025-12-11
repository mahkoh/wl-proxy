//! context object for high-resolution input timestamps
//!
//! A global interface used for requesting high-resolution timestamps
//! for input events.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_input_timestamps_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpInputTimestampsManagerV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn ZwpInputTimestampsManagerV1Handler>,
}

struct DefaultHandler;

impl ZwpInputTimestampsManagerV1Handler for DefaultHandler { }

impl ZwpInputTimestampsManagerV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "zwp_input_timestamps_manager_v1";
}

impl ZwpInputTimestampsManagerV1 {
    pub fn set_handler(&self, handler: impl ZwpInputTimestampsManagerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpInputTimestampsManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpInputTimestampsManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpInputTimestampsManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpInputTimestampsManagerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the input timestamps manager object
    ///
    /// Informs the server that the client will no longer be using this
    /// protocol object. Existing objects created by this object are not
    /// affected.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_timestamps_manager_v1#{}.destroy()\n", id);
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
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the get_keyboard_timestamps message is available.
    pub const MSG__GET_KEYBOARD_TIMESTAMPS__SINCE: u32 = 1;

    /// subscribe to high-resolution keyboard timestamp events
    ///
    /// Creates a new input timestamps object that represents a subscription
    /// to high-resolution timestamp events for all wl_keyboard events that
    /// carry a timestamp.
    ///
    /// If the associated wl_keyboard object is invalidated, either through
    /// client action (e.g. release) or server-side changes, the input
    /// timestamps object becomes inert and the client should destroy it
    /// by calling zwp_input_timestamps_v1.destroy.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `keyboard`: the wl_keyboard object for which to get timestamp events
    #[inline]
    pub fn send_get_keyboard_timestamps(
        &self,
        id: &Rc<ZwpInputTimestampsV1>,
        keyboard: &Rc<WlKeyboard>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            keyboard,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("keyboard")),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_timestamps_manager_v1#{}.get_keyboard_timestamps(id: zwp_input_timestamps_v1#{}, keyboard: wl_keyboard#{})\n", id, arg0_id, arg1_id);
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
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// Since when the get_pointer_timestamps message is available.
    pub const MSG__GET_POINTER_TIMESTAMPS__SINCE: u32 = 1;

    /// subscribe to high-resolution pointer timestamp events
    ///
    /// Creates a new input timestamps object that represents a subscription
    /// to high-resolution timestamp events for all wl_pointer events that
    /// carry a timestamp.
    ///
    /// If the associated wl_pointer object is invalidated, either through
    /// client action (e.g. release) or server-side changes, the input
    /// timestamps object becomes inert and the client should destroy it
    /// by calling zwp_input_timestamps_v1.destroy.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `pointer`: the wl_pointer object for which to get timestamp events
    #[inline]
    pub fn send_get_pointer_timestamps(
        &self,
        id: &Rc<ZwpInputTimestampsV1>,
        pointer: &Rc<WlPointer>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            pointer,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("pointer")),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_timestamps_manager_v1#{}.get_pointer_timestamps(id: zwp_input_timestamps_v1#{}, pointer: wl_pointer#{})\n", id, arg0_id, arg1_id);
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
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// Since when the get_touch_timestamps message is available.
    pub const MSG__GET_TOUCH_TIMESTAMPS__SINCE: u32 = 1;

    /// subscribe to high-resolution touch timestamp events
    ///
    /// Creates a new input timestamps object that represents a subscription
    /// to high-resolution timestamp events for all wl_touch events that
    /// carry a timestamp.
    ///
    /// If the associated wl_touch object becomes invalid, either through
    /// client action (e.g. release) or server-side changes, the input
    /// timestamps object becomes inert and the client should destroy it
    /// by calling zwp_input_timestamps_v1.destroy.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `touch`: the wl_touch object for which to get timestamp events
    #[inline]
    pub fn send_get_touch_timestamps(
        &self,
        id: &Rc<ZwpInputTimestampsV1>,
        touch: &Rc<WlTouch>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            touch,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("touch")),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_timestamps_manager_v1#{}.get_touch_timestamps(id: zwp_input_timestamps_v1#{}, touch: wl_touch#{})\n", id, arg0_id, arg1_id);
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
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpInputTimestampsManagerV1] proxies.
pub trait ZwpInputTimestampsManagerV1Handler: Any {
    /// destroy the input timestamps manager object
    ///
    /// Informs the server that the client will no longer be using this
    /// protocol object. Existing objects created by this object are not
    /// affected.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<ZwpInputTimestampsManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_timestamps_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// subscribe to high-resolution keyboard timestamp events
    ///
    /// Creates a new input timestamps object that represents a subscription
    /// to high-resolution timestamp events for all wl_keyboard events that
    /// carry a timestamp.
    ///
    /// If the associated wl_keyboard object is invalidated, either through
    /// client action (e.g. release) or server-side changes, the input
    /// timestamps object becomes inert and the client should destroy it
    /// by calling zwp_input_timestamps_v1.destroy.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `keyboard`: the wl_keyboard object for which to get timestamp events
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_keyboard_timestamps(
        &mut self,
        _slf: &Rc<ZwpInputTimestampsManagerV1>,
        id: &Rc<ZwpInputTimestampsV1>,
        keyboard: &Rc<WlKeyboard>,
    ) {
        let res = _slf.send_get_keyboard_timestamps(
            id,
            keyboard,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_timestamps_manager_v1.get_keyboard_timestamps message: {}", Report::new(e));
        }
    }

    /// subscribe to high-resolution pointer timestamp events
    ///
    /// Creates a new input timestamps object that represents a subscription
    /// to high-resolution timestamp events for all wl_pointer events that
    /// carry a timestamp.
    ///
    /// If the associated wl_pointer object is invalidated, either through
    /// client action (e.g. release) or server-side changes, the input
    /// timestamps object becomes inert and the client should destroy it
    /// by calling zwp_input_timestamps_v1.destroy.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `pointer`: the wl_pointer object for which to get timestamp events
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_pointer_timestamps(
        &mut self,
        _slf: &Rc<ZwpInputTimestampsManagerV1>,
        id: &Rc<ZwpInputTimestampsV1>,
        pointer: &Rc<WlPointer>,
    ) {
        let res = _slf.send_get_pointer_timestamps(
            id,
            pointer,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_timestamps_manager_v1.get_pointer_timestamps message: {}", Report::new(e));
        }
    }

    /// subscribe to high-resolution touch timestamp events
    ///
    /// Creates a new input timestamps object that represents a subscription
    /// to high-resolution timestamp events for all wl_touch events that
    /// carry a timestamp.
    ///
    /// If the associated wl_touch object becomes invalid, either through
    /// client action (e.g. release) or server-side changes, the input
    /// timestamps object becomes inert and the client should destroy it
    /// by calling zwp_input_timestamps_v1.destroy.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `touch`: the wl_touch object for which to get timestamp events
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_touch_timestamps(
        &mut self,
        _slf: &Rc<ZwpInputTimestampsManagerV1>,
        id: &Rc<ZwpInputTimestampsV1>,
        touch: &Rc<WlTouch>,
    ) {
        let res = _slf.send_get_touch_timestamps(
            id,
            touch,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_timestamps_manager_v1.get_touch_timestamps message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for ZwpInputTimestampsManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::ZwpInputTimestampsManagerV1, version),
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
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_timestamps_manager_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_timestamps_manager_v1#{}.get_keyboard_timestamps(id: zwp_input_timestamps_v1#{}, keyboard: wl_keyboard#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ZwpInputTimestampsV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlKeyboard>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("keyboard", o.core().interface, ProxyInterface::WlKeyboard));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_keyboard_timestamps(&self, arg0, arg1);
                } else {
                    DefaultHandler.get_keyboard_timestamps(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_timestamps_manager_v1#{}.get_pointer_timestamps(id: zwp_input_timestamps_v1#{}, pointer: wl_pointer#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ZwpInputTimestampsV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlPointer>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("pointer", o.core().interface, ProxyInterface::WlPointer));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_pointer_timestamps(&self, arg0, arg1);
                } else {
                    DefaultHandler.get_pointer_timestamps(&self, arg0, arg1);
                }
            }
            3 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_timestamps_manager_v1#{}.get_touch_timestamps(id: zwp_input_timestamps_v1#{}, touch: wl_touch#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ZwpInputTimestampsV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlTouch>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("touch", o.core().interface, ProxyInterface::WlTouch));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_touch_timestamps(&self, arg0, arg1);
                } else {
                    DefaultHandler.get_touch_timestamps(&self, arg0, arg1);
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
            0 => "destroy",
            1 => "get_keyboard_timestamps",
            2 => "get_pointer_timestamps",
            3 => "get_touch_timestamps",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Proxy for ZwpInputTimestampsManagerV1 {
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

