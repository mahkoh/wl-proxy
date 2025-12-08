//! group of input devices
//!
//! A seat is a group of keyboards, pointer and touch devices. This
//! object is published as a global during start up, or when such a
//! device is hot plugged.  A seat typically has a pointer and
//! maintains a keyboard focus and a pointer focus.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wl_seat object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlSeat {
    core: ObjectCore,
    handler: HandlerHolder<dyn WlSeatHandler>,
}

struct DefaultHandler;

impl WlSeatHandler for DefaultHandler { }

impl ConcreteObject for WlSeat {
    const XML_VERSION: u32 = 10;
    const INTERFACE: ObjectInterface = ObjectInterface::WlSeat;
    const INTERFACE_NAME: &str = "wl_seat";
}

impl WlSeat {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WlSeatHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WlSeatHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlSeat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlSeat")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlSeat {
    /// Since when the capabilities message is available.
    pub const MSG__CAPABILITIES__SINCE: u32 = 1;

    /// seat capabilities changed
    ///
    /// This is sent on binding to the seat global or whenever a seat gains
    /// or loses the pointer, keyboard or touch capabilities.
    /// The argument is a capability enum containing the complete set of
    /// capabilities this seat has.
    ///
    /// When the pointer capability is added, a client may create a
    /// wl_pointer object using the wl_seat.get_pointer request. This object
    /// will receive pointer events until the capability is removed in the
    /// future.
    ///
    /// When the pointer capability is removed, a client should destroy the
    /// wl_pointer objects associated with the seat where the capability was
    /// removed, using the wl_pointer.release request. No further pointer
    /// events will be received on these objects.
    ///
    /// In some compositors, if a seat regains the pointer capability and a
    /// client has a previously obtained wl_pointer object of version 4 or
    /// less, that object may start sending pointer events again. This
    /// behavior is considered a misinterpretation of the intended behavior
    /// and must not be relied upon by the client. wl_pointer objects of
    /// version 5 or later must not send events if created before the most
    /// recent event notifying the client of an added pointer capability.
    ///
    /// The above behavior also applies to wl_keyboard and wl_touch with the
    /// keyboard and touch capabilities, respectively.
    ///
    /// # Arguments
    ///
    /// - `capabilities`: capabilities of the seat
    #[inline]
    pub fn try_send_capabilities(
        &self,
        capabilities: WlSeatCapability,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            capabilities,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: WlSeatCapability) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_seat#{}.capabilities(capabilities: {:?})\n", client_id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// seat capabilities changed
    ///
    /// This is sent on binding to the seat global or whenever a seat gains
    /// or loses the pointer, keyboard or touch capabilities.
    /// The argument is a capability enum containing the complete set of
    /// capabilities this seat has.
    ///
    /// When the pointer capability is added, a client may create a
    /// wl_pointer object using the wl_seat.get_pointer request. This object
    /// will receive pointer events until the capability is removed in the
    /// future.
    ///
    /// When the pointer capability is removed, a client should destroy the
    /// wl_pointer objects associated with the seat where the capability was
    /// removed, using the wl_pointer.release request. No further pointer
    /// events will be received on these objects.
    ///
    /// In some compositors, if a seat regains the pointer capability and a
    /// client has a previously obtained wl_pointer object of version 4 or
    /// less, that object may start sending pointer events again. This
    /// behavior is considered a misinterpretation of the intended behavior
    /// and must not be relied upon by the client. wl_pointer objects of
    /// version 5 or later must not send events if created before the most
    /// recent event notifying the client of an added pointer capability.
    ///
    /// The above behavior also applies to wl_keyboard and wl_touch with the
    /// keyboard and touch capabilities, respectively.
    ///
    /// # Arguments
    ///
    /// - `capabilities`: capabilities of the seat
    #[inline]
    pub fn send_capabilities(
        &self,
        capabilities: WlSeatCapability,
    ) {
        let res = self.try_send_capabilities(
            capabilities,
        );
        if let Err(e) = res {
            log_send("wl_seat.capabilities", &e);
        }
    }

    /// Since when the get_pointer message is available.
    pub const MSG__GET_POINTER__SINCE: u32 = 1;

    /// return pointer object
    ///
    /// The ID provided will be initialized to the wl_pointer interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the pointer
    /// capability, or has had the pointer capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the pointer capability. The missing_capability error will
    /// be sent in this case.
    ///
    /// # Arguments
    ///
    /// - `id`: seat pointer
    #[inline]
    pub fn try_send_get_pointer(
        &self,
        id: &Rc<WlPointer>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_seat#{}.get_pointer(id: wl_pointer#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// return pointer object
    ///
    /// The ID provided will be initialized to the wl_pointer interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the pointer
    /// capability, or has had the pointer capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the pointer capability. The missing_capability error will
    /// be sent in this case.
    ///
    /// # Arguments
    ///
    /// - `id`: seat pointer
    #[inline]
    pub fn send_get_pointer(
        &self,
        id: &Rc<WlPointer>,
    ) {
        let res = self.try_send_get_pointer(
            id,
        );
        if let Err(e) = res {
            log_send("wl_seat.get_pointer", &e);
        }
    }

    /// return pointer object
    ///
    /// The ID provided will be initialized to the wl_pointer interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the pointer
    /// capability, or has had the pointer capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the pointer capability. The missing_capability error will
    /// be sent in this case.
    #[inline]
    pub fn new_try_send_get_pointer(
        &self,
    ) -> Result<Rc<WlPointer>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_pointer(
            &id,
        )?;
        Ok(id)
    }

    /// return pointer object
    ///
    /// The ID provided will be initialized to the wl_pointer interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the pointer
    /// capability, or has had the pointer capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the pointer capability. The missing_capability error will
    /// be sent in this case.
    #[inline]
    pub fn new_send_get_pointer(
        &self,
    ) -> Rc<WlPointer> {
        let id = self.core.create_child();
        self.send_get_pointer(
            &id,
        );
        id
    }

    /// Since when the get_keyboard message is available.
    pub const MSG__GET_KEYBOARD__SINCE: u32 = 1;

    /// return keyboard object
    ///
    /// The ID provided will be initialized to the wl_keyboard interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the keyboard
    /// capability, or has had the keyboard capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the keyboard capability. The missing_capability error will
    /// be sent in this case.
    ///
    /// # Arguments
    ///
    /// - `id`: seat keyboard
    #[inline]
    pub fn try_send_get_keyboard(
        &self,
        id: &Rc<WlKeyboard>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_seat#{}.get_keyboard(id: wl_keyboard#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
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
        ]);
        Ok(())
    }

    /// return keyboard object
    ///
    /// The ID provided will be initialized to the wl_keyboard interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the keyboard
    /// capability, or has had the keyboard capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the keyboard capability. The missing_capability error will
    /// be sent in this case.
    ///
    /// # Arguments
    ///
    /// - `id`: seat keyboard
    #[inline]
    pub fn send_get_keyboard(
        &self,
        id: &Rc<WlKeyboard>,
    ) {
        let res = self.try_send_get_keyboard(
            id,
        );
        if let Err(e) = res {
            log_send("wl_seat.get_keyboard", &e);
        }
    }

    /// return keyboard object
    ///
    /// The ID provided will be initialized to the wl_keyboard interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the keyboard
    /// capability, or has had the keyboard capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the keyboard capability. The missing_capability error will
    /// be sent in this case.
    #[inline]
    pub fn new_try_send_get_keyboard(
        &self,
    ) -> Result<Rc<WlKeyboard>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_keyboard(
            &id,
        )?;
        Ok(id)
    }

    /// return keyboard object
    ///
    /// The ID provided will be initialized to the wl_keyboard interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the keyboard
    /// capability, or has had the keyboard capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the keyboard capability. The missing_capability error will
    /// be sent in this case.
    #[inline]
    pub fn new_send_get_keyboard(
        &self,
    ) -> Rc<WlKeyboard> {
        let id = self.core.create_child();
        self.send_get_keyboard(
            &id,
        );
        id
    }

    /// Since when the get_touch message is available.
    pub const MSG__GET_TOUCH__SINCE: u32 = 1;

    /// return touch object
    ///
    /// The ID provided will be initialized to the wl_touch interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the touch
    /// capability, or has had the touch capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the touch capability. The missing_capability error will
    /// be sent in this case.
    ///
    /// # Arguments
    ///
    /// - `id`: seat touch interface
    #[inline]
    pub fn try_send_get_touch(
        &self,
        id: &Rc<WlTouch>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_seat#{}.get_touch(id: wl_touch#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
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
        ]);
        Ok(())
    }

    /// return touch object
    ///
    /// The ID provided will be initialized to the wl_touch interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the touch
    /// capability, or has had the touch capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the touch capability. The missing_capability error will
    /// be sent in this case.
    ///
    /// # Arguments
    ///
    /// - `id`: seat touch interface
    #[inline]
    pub fn send_get_touch(
        &self,
        id: &Rc<WlTouch>,
    ) {
        let res = self.try_send_get_touch(
            id,
        );
        if let Err(e) = res {
            log_send("wl_seat.get_touch", &e);
        }
    }

    /// return touch object
    ///
    /// The ID provided will be initialized to the wl_touch interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the touch
    /// capability, or has had the touch capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the touch capability. The missing_capability error will
    /// be sent in this case.
    #[inline]
    pub fn new_try_send_get_touch(
        &self,
    ) -> Result<Rc<WlTouch>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_touch(
            &id,
        )?;
        Ok(id)
    }

    /// return touch object
    ///
    /// The ID provided will be initialized to the wl_touch interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the touch
    /// capability, or has had the touch capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the touch capability. The missing_capability error will
    /// be sent in this case.
    #[inline]
    pub fn new_send_get_touch(
        &self,
    ) -> Rc<WlTouch> {
        let id = self.core.create_child();
        self.send_get_touch(
            &id,
        );
        id
    }

    /// Since when the name message is available.
    pub const MSG__NAME__SINCE: u32 = 2;

    /// unique identifier for this seat
    ///
    /// In a multi-seat configuration the seat name can be used by clients to
    /// help identify which physical devices the seat represents.
    ///
    /// The seat name is a UTF-8 string with no convention defined for its
    /// contents. Each name is unique among all wl_seat globals. The name is
    /// only guaranteed to be unique for the current compositor instance.
    ///
    /// The same seat names are used for all clients. Thus, the name can be
    /// shared across processes to refer to a specific wl_seat global.
    ///
    /// The name event is sent after binding to the seat global, and should be sent
    /// before announcing capabilities. This event only sent once per seat object,
    /// and the name does not change over the lifetime of the wl_seat global.
    ///
    /// Compositors may re-use the same seat name if the wl_seat global is
    /// destroyed and re-created later.
    ///
    /// # Arguments
    ///
    /// - `name`: seat identifier
    #[inline]
    pub fn try_send_name(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_seat#{}.name(name: {:?})\n", client_id, id, arg0);
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

    /// unique identifier for this seat
    ///
    /// In a multi-seat configuration the seat name can be used by clients to
    /// help identify which physical devices the seat represents.
    ///
    /// The seat name is a UTF-8 string with no convention defined for its
    /// contents. Each name is unique among all wl_seat globals. The name is
    /// only guaranteed to be unique for the current compositor instance.
    ///
    /// The same seat names are used for all clients. Thus, the name can be
    /// shared across processes to refer to a specific wl_seat global.
    ///
    /// The name event is sent after binding to the seat global, and should be sent
    /// before announcing capabilities. This event only sent once per seat object,
    /// and the name does not change over the lifetime of the wl_seat global.
    ///
    /// Compositors may re-use the same seat name if the wl_seat global is
    /// destroyed and re-created later.
    ///
    /// # Arguments
    ///
    /// - `name`: seat identifier
    #[inline]
    pub fn send_name(
        &self,
        name: &str,
    ) {
        let res = self.try_send_name(
            name,
        );
        if let Err(e) = res {
            log_send("wl_seat.name", &e);
        }
    }

    /// Since when the release message is available.
    pub const MSG__RELEASE__SINCE: u32 = 5;

    /// release the seat object
    ///
    /// Using this request a client can tell the server that it is not going to
    /// use the seat object anymore.
    #[inline]
    pub fn try_send_release(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_seat#{}.release()\n", id);
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
            3,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// release the seat object
    ///
    /// Using this request a client can tell the server that it is not going to
    /// use the seat object anymore.
    #[inline]
    pub fn send_release(
        &self,
    ) {
        let res = self.try_send_release(
        );
        if let Err(e) = res {
            log_send("wl_seat.release", &e);
        }
    }
}

/// A message handler for [`WlSeat`] proxies.
pub trait WlSeatHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WlSeat>) {
        slf.core.delete_id();
    }

    /// seat capabilities changed
    ///
    /// This is sent on binding to the seat global or whenever a seat gains
    /// or loses the pointer, keyboard or touch capabilities.
    /// The argument is a capability enum containing the complete set of
    /// capabilities this seat has.
    ///
    /// When the pointer capability is added, a client may create a
    /// wl_pointer object using the wl_seat.get_pointer request. This object
    /// will receive pointer events until the capability is removed in the
    /// future.
    ///
    /// When the pointer capability is removed, a client should destroy the
    /// wl_pointer objects associated with the seat where the capability was
    /// removed, using the wl_pointer.release request. No further pointer
    /// events will be received on these objects.
    ///
    /// In some compositors, if a seat regains the pointer capability and a
    /// client has a previously obtained wl_pointer object of version 4 or
    /// less, that object may start sending pointer events again. This
    /// behavior is considered a misinterpretation of the intended behavior
    /// and must not be relied upon by the client. wl_pointer objects of
    /// version 5 or later must not send events if created before the most
    /// recent event notifying the client of an added pointer capability.
    ///
    /// The above behavior also applies to wl_keyboard and wl_touch with the
    /// keyboard and touch capabilities, respectively.
    ///
    /// # Arguments
    ///
    /// - `capabilities`: capabilities of the seat
    #[inline]
    fn handle_capabilities(
        &mut self,
        slf: &Rc<WlSeat>,
        capabilities: WlSeatCapability,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_capabilities(
            capabilities,
        );
        if let Err(e) = res {
            log_forward("wl_seat.capabilities", &e);
        }
    }

    /// return pointer object
    ///
    /// The ID provided will be initialized to the wl_pointer interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the pointer
    /// capability, or has had the pointer capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the pointer capability. The missing_capability error will
    /// be sent in this case.
    ///
    /// # Arguments
    ///
    /// - `id`: seat pointer
    #[inline]
    fn handle_get_pointer(
        &mut self,
        slf: &Rc<WlSeat>,
        id: &Rc<WlPointer>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_pointer(
            id,
        );
        if let Err(e) = res {
            log_forward("wl_seat.get_pointer", &e);
        }
    }

    /// return keyboard object
    ///
    /// The ID provided will be initialized to the wl_keyboard interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the keyboard
    /// capability, or has had the keyboard capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the keyboard capability. The missing_capability error will
    /// be sent in this case.
    ///
    /// # Arguments
    ///
    /// - `id`: seat keyboard
    #[inline]
    fn handle_get_keyboard(
        &mut self,
        slf: &Rc<WlSeat>,
        id: &Rc<WlKeyboard>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_keyboard(
            id,
        );
        if let Err(e) = res {
            log_forward("wl_seat.get_keyboard", &e);
        }
    }

    /// return touch object
    ///
    /// The ID provided will be initialized to the wl_touch interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the touch
    /// capability, or has had the touch capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the touch capability. The missing_capability error will
    /// be sent in this case.
    ///
    /// # Arguments
    ///
    /// - `id`: seat touch interface
    #[inline]
    fn handle_get_touch(
        &mut self,
        slf: &Rc<WlSeat>,
        id: &Rc<WlTouch>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_touch(
            id,
        );
        if let Err(e) = res {
            log_forward("wl_seat.get_touch", &e);
        }
    }

    /// unique identifier for this seat
    ///
    /// In a multi-seat configuration the seat name can be used by clients to
    /// help identify which physical devices the seat represents.
    ///
    /// The seat name is a UTF-8 string with no convention defined for its
    /// contents. Each name is unique among all wl_seat globals. The name is
    /// only guaranteed to be unique for the current compositor instance.
    ///
    /// The same seat names are used for all clients. Thus, the name can be
    /// shared across processes to refer to a specific wl_seat global.
    ///
    /// The name event is sent after binding to the seat global, and should be sent
    /// before announcing capabilities. This event only sent once per seat object,
    /// and the name does not change over the lifetime of the wl_seat global.
    ///
    /// Compositors may re-use the same seat name if the wl_seat global is
    /// destroyed and re-created later.
    ///
    /// # Arguments
    ///
    /// - `name`: seat identifier
    #[inline]
    fn handle_name(
        &mut self,
        slf: &Rc<WlSeat>,
        name: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_name(
            name,
        );
        if let Err(e) = res {
            log_forward("wl_seat.name", &e);
        }
    }

    /// release the seat object
    ///
    /// Using this request a client can tell the server that it is not going to
    /// use the seat object anymore.
    #[inline]
    fn handle_release(
        &mut self,
        slf: &Rc<WlSeat>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_release(
        );
        if let Err(e) = res {
            log_forward("wl_seat.release", &e);
        }
    }
}

impl ObjectPrivate for WlSeat {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WlSeat, version),
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_seat#{}.get_pointer(id: wl_pointer#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WlPointer::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_get_pointer(&self, arg0);
                } else {
                    DefaultHandler.handle_get_pointer(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_seat#{}.get_keyboard(id: wl_keyboard#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WlKeyboard::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_get_keyboard(&self, arg0);
                } else {
                    DefaultHandler.handle_get_keyboard(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_seat#{}.get_touch(id: wl_touch#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WlTouch::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_get_touch(&self, arg0);
                } else {
                    DefaultHandler.handle_get_touch(&self, arg0);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_seat#{}.release()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_release(&self);
                } else {
                    DefaultHandler.handle_release(&self);
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
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = WlSeatCapability(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: WlSeatCapability) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_seat#{}.capabilities(capabilities: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_capabilities(&self, arg0);
                } else {
                    DefaultHandler.handle_capabilities(&self, arg0);
                }
            }
            1 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "name")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_seat#{}.name(name: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_name(&self, arg0);
                } else {
                    DefaultHandler.handle_name(&self, arg0);
                }
            }
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
        Ok(())
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "get_pointer",
            1 => "get_keyboard",
            2 => "get_touch",
            3 => "release",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "capabilities",
            1 => "name",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WlSeat {
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

impl WlSeat {
    /// Since when the capability.pointer enum variant is available.
    pub const ENM__CAPABILITY_POINTER__SINCE: u32 = 1;
    /// Since when the capability.keyboard enum variant is available.
    pub const ENM__CAPABILITY_KEYBOARD__SINCE: u32 = 1;
    /// Since when the capability.touch enum variant is available.
    pub const ENM__CAPABILITY_TOUCH__SINCE: u32 = 1;

    /// Since when the error.missing_capability enum variant is available.
    pub const ENM__ERROR_MISSING_CAPABILITY__SINCE: u32 = 1;
}

/// seat capability bitmask
///
/// This is a bitmask of capabilities this seat has; if a member is
/// set, then it is present on the seat.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct WlSeatCapability(pub u32);

/// An iterator over the set bits in a [`WlSeatCapability`].
///
/// You can construct this with the `IntoIterator` implementation of `WlSeatCapability`.
#[derive(Clone, Debug)]
pub struct WlSeatCapabilityIter(pub u32);

impl WlSeatCapability {
    /// the seat has pointer devices
    pub const POINTER: Self = Self(1);

    /// the seat has one or more keyboards
    pub const KEYBOARD: Self = Self(2);

    /// the seat has touch devices
    pub const TOUCH: Self = Self(4);
}

impl WlSeatCapability {
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    #[inline]
    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    #[inline]
    pub const fn insert(&mut self, other: Self) {
        *self = self.union(other);
    }

    #[inline]
    pub const fn remove(&mut self, other: Self) {
        *self = self.difference(other);
    }

    #[inline]
    pub const fn toggle(&mut self, other: Self) {
        *self = self.symmetric_difference(other);
    }

    #[inline]
    pub const fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }

    #[inline]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline]
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    #[inline]
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(!self.0)
    }

    #[inline]
    #[must_use]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    #[inline]
    pub const fn all_known() -> Self {
        #[allow(clippy::eq_op, clippy::identity_op)]
        Self(0 | 1 | 2 | 4)
    }
}

impl Iterator for WlSeatCapabilityIter {
    type Item = WlSeatCapability;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(WlSeatCapability(bit))
    }
}

impl IntoIterator for WlSeatCapability {
    type Item = WlSeatCapability;
    type IntoIter = WlSeatCapabilityIter;

    fn into_iter(self) -> Self::IntoIter {
        WlSeatCapabilityIter(self.0)
    }
}

impl BitAnd for WlSeatCapability {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for WlSeatCapability {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for WlSeatCapability {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for WlSeatCapability {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for WlSeatCapability {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for WlSeatCapability {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for WlSeatCapability {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for WlSeatCapability {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for WlSeatCapability {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for WlSeatCapability {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = self.0;
        let mut first = true;
        if v & 1 == 1 {
            v &= !1;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("POINTER")?;
        }
        if v & 2 == 2 {
            v &= !2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("KEYBOARD")?;
        }
        if v & 4 == 4 {
            v &= !4;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("TOUCH")?;
        }
        if v != 0 {
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            write!(f, "0x{v:032x}")?;
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

/// wl_seat error values
///
/// These errors can be emitted in response to wl_seat requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WlSeatError(pub u32);

impl WlSeatError {
    /// get_pointer, get_keyboard or get_touch called on seat without the matching capability
    pub const MISSING_CAPABILITY: Self = Self(0);
}

impl Debug for WlSeatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::MISSING_CAPABILITY => "MISSING_CAPABILITY",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
