//! context object for high-resolution input timestamps
//!
//! A global interface used for requesting high-resolution timestamps
//! for input events.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_input_timestamps_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpInputTimestampsManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpInputTimestampsManagerV1Handler>,
}

struct DefaultHandler;

impl ZwpInputTimestampsManagerV1Handler for DefaultHandler { }

impl ConcreteObject for ZwpInputTimestampsManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpInputTimestampsManagerV1;
    const INTERFACE_NAME: &str = "zwp_input_timestamps_manager_v1";
}

impl ZwpInputTimestampsManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpInputTimestampsManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
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
    pub fn try_send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_timestamps_manager_v1#{}.destroy()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
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

    /// destroy the input timestamps manager object
    ///
    /// Informs the server that the client will no longer be using this
    /// protocol object. Existing objects created by this object are not
    /// affected.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwp_input_timestamps_manager_v1.destroy", &e);
        }
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
    pub fn try_send_get_keyboard_timestamps(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("keyboard"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_timestamps_manager_v1#{}.get_keyboard_timestamps(id: zwp_input_timestamps_v1#{}, keyboard: wl_keyboard#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
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
    ) {
        let res = self.try_send_get_keyboard_timestamps(
            id,
            keyboard,
        );
        if let Err(e) = res {
            log_send("zwp_input_timestamps_manager_v1.get_keyboard_timestamps", &e);
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
    /// - `keyboard`: the wl_keyboard object for which to get timestamp events
    #[inline]
    pub fn new_try_send_get_keyboard_timestamps(
        &self,
        keyboard: &Rc<WlKeyboard>,
    ) -> Result<Rc<ZwpInputTimestampsV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_keyboard_timestamps(
            &id,
            keyboard,
        )?;
        Ok(id)
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
    /// - `keyboard`: the wl_keyboard object for which to get timestamp events
    #[inline]
    pub fn new_send_get_keyboard_timestamps(
        &self,
        keyboard: &Rc<WlKeyboard>,
    ) -> Rc<ZwpInputTimestampsV1> {
        let id = self.core.create_child();
        self.send_get_keyboard_timestamps(
            &id,
            keyboard,
        );
        id
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
    pub fn try_send_get_pointer_timestamps(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("pointer"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_timestamps_manager_v1#{}.get_pointer_timestamps(id: zwp_input_timestamps_v1#{}, pointer: wl_pointer#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
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
    ) {
        let res = self.try_send_get_pointer_timestamps(
            id,
            pointer,
        );
        if let Err(e) = res {
            log_send("zwp_input_timestamps_manager_v1.get_pointer_timestamps", &e);
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
    /// - `pointer`: the wl_pointer object for which to get timestamp events
    #[inline]
    pub fn new_try_send_get_pointer_timestamps(
        &self,
        pointer: &Rc<WlPointer>,
    ) -> Result<Rc<ZwpInputTimestampsV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_pointer_timestamps(
            &id,
            pointer,
        )?;
        Ok(id)
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
    /// - `pointer`: the wl_pointer object for which to get timestamp events
    #[inline]
    pub fn new_send_get_pointer_timestamps(
        &self,
        pointer: &Rc<WlPointer>,
    ) -> Rc<ZwpInputTimestampsV1> {
        let id = self.core.create_child();
        self.send_get_pointer_timestamps(
            &id,
            pointer,
        );
        id
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
    pub fn try_send_get_touch_timestamps(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("touch"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_timestamps_manager_v1#{}.get_touch_timestamps(id: zwp_input_timestamps_v1#{}, touch: wl_touch#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
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
    ) {
        let res = self.try_send_get_touch_timestamps(
            id,
            touch,
        );
        if let Err(e) = res {
            log_send("zwp_input_timestamps_manager_v1.get_touch_timestamps", &e);
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
    /// - `touch`: the wl_touch object for which to get timestamp events
    #[inline]
    pub fn new_try_send_get_touch_timestamps(
        &self,
        touch: &Rc<WlTouch>,
    ) -> Result<Rc<ZwpInputTimestampsV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_touch_timestamps(
            &id,
            touch,
        )?;
        Ok(id)
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
    /// - `touch`: the wl_touch object for which to get timestamp events
    #[inline]
    pub fn new_send_get_touch_timestamps(
        &self,
        touch: &Rc<WlTouch>,
    ) -> Rc<ZwpInputTimestampsV1> {
        let id = self.core.create_child();
        self.send_get_touch_timestamps(
            &id,
            touch,
        );
        id
    }
}

/// A message handler for [`ZwpInputTimestampsManagerV1`] proxies.
pub trait ZwpInputTimestampsManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpInputTimestampsManagerV1>) {
        slf.core.delete_id();
    }

    /// destroy the input timestamps manager object
    ///
    /// Informs the server that the client will no longer be using this
    /// protocol object. Existing objects created by this object are not
    /// affected.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwpInputTimestampsManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwp_input_timestamps_manager_v1.destroy", &e);
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
    fn handle_get_keyboard_timestamps(
        &mut self,
        slf: &Rc<ZwpInputTimestampsManagerV1>,
        id: &Rc<ZwpInputTimestampsV1>,
        keyboard: &Rc<WlKeyboard>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_keyboard_timestamps(
            id,
            keyboard,
        );
        if let Err(e) = res {
            log_forward("zwp_input_timestamps_manager_v1.get_keyboard_timestamps", &e);
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
    fn handle_get_pointer_timestamps(
        &mut self,
        slf: &Rc<ZwpInputTimestampsManagerV1>,
        id: &Rc<ZwpInputTimestampsV1>,
        pointer: &Rc<WlPointer>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_pointer_timestamps(
            id,
            pointer,
        );
        if let Err(e) = res {
            log_forward("zwp_input_timestamps_manager_v1.get_pointer_timestamps", &e);
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
    fn handle_get_touch_timestamps(
        &mut self,
        slf: &Rc<ZwpInputTimestampsManagerV1>,
        id: &Rc<ZwpInputTimestampsV1>,
        touch: &Rc<WlTouch>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_touch_timestamps(
            id,
            touch,
        );
        if let Err(e) = res {
            log_forward("zwp_input_timestamps_manager_v1.get_touch_timestamps", &e);
        }
    }
}

impl ObjectPrivate for ZwpInputTimestampsManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpInputTimestampsManagerV1, version),
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
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_timestamps_manager_v1#{}.destroy()\n", client_id, id);
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
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_timestamps_manager_v1#{}.get_keyboard_timestamps(id: zwp_input_timestamps_v1#{}, keyboard: wl_keyboard#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = ZwpInputTimestampsV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlKeyboard>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("keyboard", o.core().interface, ObjectInterface::WlKeyboard)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_keyboard_timestamps(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_keyboard_timestamps(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_timestamps_manager_v1#{}.get_pointer_timestamps(id: zwp_input_timestamps_v1#{}, pointer: wl_pointer#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = ZwpInputTimestampsV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlPointer>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("pointer", o.core().interface, ObjectInterface::WlPointer)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_pointer_timestamps(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_pointer_timestamps(&self, arg0, arg1);
                }
            }
            3 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_timestamps_manager_v1#{}.get_touch_timestamps(id: zwp_input_timestamps_v1#{}, touch: wl_touch#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = ZwpInputTimestampsV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlTouch>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("touch", o.core().interface, ObjectInterface::WlTouch)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_touch_timestamps(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_touch_timestamps(&self, arg0, arg1);
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

    fn handle_event(self: Rc<Self>, server: &Endpoint, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
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

impl Object for ZwpInputTimestampsManagerV1 {
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

