//! context object for high-resolution input timestamps
//!
//! A global interface used for requesting high-resolution timestamps
//! for input events.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_input_timestamps_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpInputTimestampsManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpInputTimestampsManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpInputTimestampsManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpInputTimestampsManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwpInputTimestampsManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpInputTimestampsManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpInputTimestampsManagerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
        ]);
        Ok(())
    }

    /// Since when the get_keyboard_timestamps message is available.
    #[allow(dead_code)]
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
        id: &Rc<MetaZwpInputTimestampsV1>,
        keyboard: &Rc<MetaWlKeyboard>,
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
            return Err(ObjectError);
        };
        let arg1 = match arg1.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
        ]);
        Ok(())
    }

    /// Since when the get_pointer_timestamps message is available.
    #[allow(dead_code)]
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
        id: &Rc<MetaZwpInputTimestampsV1>,
        pointer: &Rc<MetaWlPointer>,
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
            return Err(ObjectError);
        };
        let arg1 = match arg1.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
        ]);
        Ok(())
    }

    /// Since when the get_touch_timestamps message is available.
    #[allow(dead_code)]
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
        id: &Rc<MetaZwpInputTimestampsV1>,
        touch: &Rc<MetaWlTouch>,
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
            return Err(ObjectError);
        };
        let arg1 = match arg1.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            3,
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpInputTimestampsManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpInputTimestampsManagerV1MessageHandler {
    /// destroy the input timestamps manager object
    ///
    /// Informs the server that the client will no longer be using this
    /// protocol object. Existing objects created by this object are not
    /// affected.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpInputTimestampsManagerV1>,
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
        _slf: &Rc<MetaZwpInputTimestampsManagerV1>,
        id: &Rc<MetaZwpInputTimestampsV1>,
        keyboard: &Rc<MetaWlKeyboard>,
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
        _slf: &Rc<MetaZwpInputTimestampsManagerV1>,
        id: &Rc<MetaZwpInputTimestampsV1>,
        pointer: &Rc<MetaWlPointer>,
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
        _slf: &Rc<MetaZwpInputTimestampsManagerV1>,
        id: &Rc<MetaZwpInputTimestampsV1>,
        touch: &Rc<MetaWlTouch>,
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

impl Proxy for MetaZwpInputTimestampsManagerV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaZwpInputTimestampsV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlKeyboard>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_keyboard_timestamps(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_keyboard_timestamps(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaZwpInputTimestampsV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlPointer>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_pointer_timestamps(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_pointer_timestamps(&self, arg0, arg1);
                }
            }
            3 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaZwpInputTimestampsV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlTouch>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_touch_timestamps(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_touch_timestamps(&self, arg0, arg1);
                }
            }
            _ => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
    }
}

