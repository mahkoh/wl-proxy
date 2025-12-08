//! xdg_popup extension object
//!
//! This object can be used to perform additional operations on an xdg_popup.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A jay_popup_ext_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct JayPopupExtV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn JayPopupExtV1Handler>,
}

struct DefaultHandler;

impl JayPopupExtV1Handler for DefaultHandler { }

impl ConcreteObject for JayPopupExtV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::JayPopupExtV1;
    const INTERFACE_NAME: &str = "jay_popup_ext_v1";
}

impl JayPopupExtV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl JayPopupExtV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn JayPopupExtV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for JayPopupExtV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JayPopupExtV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl JayPopupExtV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroys this extension object
    ///
    /// This request has no effect on ongoing operations started through this
    /// object.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= jay_popup_ext_v1#{}.destroy()\n", id);
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

    /// destroys this extension object
    ///
    /// This request has no effect on ongoing operations started through this
    /// object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("jay_popup_ext_v1.destroy", &e);
        }
    }

    /// Since when the move message is available.
    pub const MSG__MOVE__SINCE: u32 = 1;

    /// start an interactive move
    ///
    /// Start an interactive, user-driven move of the surface.
    ///
    /// This request must be used in response to some sort of user action
    /// like a button press, key press, or touch down event. The passed
    /// serial is used to determine the type of interactive move (touch,
    /// pointer, etc).
    ///
    /// The server may ignore move requests. For example, if the passed serial
    /// is no longer valid.
    ///
    /// If triggered, the surface will lose the focus of the device
    /// (wl_pointer, wl_touch, etc) used for the move. It is up to the
    /// compositor to visually indicate that the move is taking place, such as
    /// updating a pointer cursor, during the move. There is no guarantee
    /// that the device focus will return when the move is completed.
    ///
    /// # Arguments
    ///
    /// - `seat`: the wl_seat of the user event
    /// - `serial`: the serial of the user event
    #[inline]
    pub fn try_send_move(
        &self,
        seat: &Rc<WlSeat>,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            seat,
            serial,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("seat"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= jay_popup_ext_v1#{}.move(seat: wl_seat#{}, serial: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            arg1,
        ]);
        Ok(())
    }

    /// start an interactive move
    ///
    /// Start an interactive, user-driven move of the surface.
    ///
    /// This request must be used in response to some sort of user action
    /// like a button press, key press, or touch down event. The passed
    /// serial is used to determine the type of interactive move (touch,
    /// pointer, etc).
    ///
    /// The server may ignore move requests. For example, if the passed serial
    /// is no longer valid.
    ///
    /// If triggered, the surface will lose the focus of the device
    /// (wl_pointer, wl_touch, etc) used for the move. It is up to the
    /// compositor to visually indicate that the move is taking place, such as
    /// updating a pointer cursor, during the move. There is no guarantee
    /// that the device focus will return when the move is completed.
    ///
    /// # Arguments
    ///
    /// - `seat`: the wl_seat of the user event
    /// - `serial`: the serial of the user event
    #[inline]
    pub fn send_move(
        &self,
        seat: &Rc<WlSeat>,
        serial: u32,
    ) {
        let res = self.try_send_move(
            seat,
            serial,
        );
        if let Err(e) = res {
            log_send("jay_popup_ext_v1.move", &e);
        }
    }

    /// Since when the resize message is available.
    pub const MSG__RESIZE__SINCE: u32 = 1;

    /// start an interactive resize
    ///
    /// Start a user-driven, interactive resize of the surface.
    ///
    /// This request must be used in response to some sort of user action
    /// like a button press, key press, or touch down event. The passed
    /// serial is used to determine the type of interactive resize (touch,
    /// pointer, etc).
    ///
    /// The server may ignore resize requests. For example, if the passed serial
    /// is no longer valid.
    ///
    /// If triggered, the client will receive configure events with the
    /// expected sizes. The client must also acknowledge configure events using
    /// "ack_configure".
    ///
    /// If triggered, the surface also will lose the focus of the device
    /// (wl_pointer, wl_touch, etc) used for the resize. It is up to the
    /// compositor to visually indicate that the resize is taking place,
    /// such as updating a pointer cursor, during the resize. There is no
    /// guarantee that the device focus will return when the resize is
    /// completed.
    ///
    /// The edges parameter specifies how the surface should be resized, and
    /// is one of the values of the resize_edge enum. Values not matching
    /// a variant of the enum will cause the invalid_resize_edge protocol error.
    /// The compositor may use this information to update the surface position
    /// for example when dragging the top left corner. The compositor may also
    /// use this information to adapt its behavior, e.g. choose an appropriate
    /// cursor image.
    ///
    /// # Arguments
    ///
    /// - `seat`: the wl_seat of the user event
    /// - `serial`: the serial of the user event
    /// - `edges`: which edge or corner is being dragged
    #[inline]
    pub fn try_send_resize(
        &self,
        seat: &Rc<WlSeat>,
        serial: u32,
        edges: XdgToplevelResizeEdge,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            seat,
            serial,
            edges,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("seat"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: XdgToplevelResizeEdge) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= jay_popup_ext_v1#{}.resize(seat: wl_seat#{}, serial: {}, edges: {:?})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1, arg2);
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
            arg1,
            arg2.0,
        ]);
        Ok(())
    }

    /// start an interactive resize
    ///
    /// Start a user-driven, interactive resize of the surface.
    ///
    /// This request must be used in response to some sort of user action
    /// like a button press, key press, or touch down event. The passed
    /// serial is used to determine the type of interactive resize (touch,
    /// pointer, etc).
    ///
    /// The server may ignore resize requests. For example, if the passed serial
    /// is no longer valid.
    ///
    /// If triggered, the client will receive configure events with the
    /// expected sizes. The client must also acknowledge configure events using
    /// "ack_configure".
    ///
    /// If triggered, the surface also will lose the focus of the device
    /// (wl_pointer, wl_touch, etc) used for the resize. It is up to the
    /// compositor to visually indicate that the resize is taking place,
    /// such as updating a pointer cursor, during the resize. There is no
    /// guarantee that the device focus will return when the resize is
    /// completed.
    ///
    /// The edges parameter specifies how the surface should be resized, and
    /// is one of the values of the resize_edge enum. Values not matching
    /// a variant of the enum will cause the invalid_resize_edge protocol error.
    /// The compositor may use this information to update the surface position
    /// for example when dragging the top left corner. The compositor may also
    /// use this information to adapt its behavior, e.g. choose an appropriate
    /// cursor image.
    ///
    /// # Arguments
    ///
    /// - `seat`: the wl_seat of the user event
    /// - `serial`: the serial of the user event
    /// - `edges`: which edge or corner is being dragged
    #[inline]
    pub fn send_resize(
        &self,
        seat: &Rc<WlSeat>,
        serial: u32,
        edges: XdgToplevelResizeEdge,
    ) {
        let res = self.try_send_resize(
            seat,
            serial,
            edges,
        );
        if let Err(e) = res {
            log_send("jay_popup_ext_v1.resize", &e);
        }
    }
}

/// A message handler for [`JayPopupExtV1`] proxies.
pub trait JayPopupExtV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<JayPopupExtV1>) {
        slf.core.delete_id();
    }

    /// destroys this extension object
    ///
    /// This request has no effect on ongoing operations started through this
    /// object.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<JayPopupExtV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("jay_popup_ext_v1.destroy", &e);
        }
    }

    /// start an interactive move
    ///
    /// Start an interactive, user-driven move of the surface.
    ///
    /// This request must be used in response to some sort of user action
    /// like a button press, key press, or touch down event. The passed
    /// serial is used to determine the type of interactive move (touch,
    /// pointer, etc).
    ///
    /// The server may ignore move requests. For example, if the passed serial
    /// is no longer valid.
    ///
    /// If triggered, the surface will lose the focus of the device
    /// (wl_pointer, wl_touch, etc) used for the move. It is up to the
    /// compositor to visually indicate that the move is taking place, such as
    /// updating a pointer cursor, during the move. There is no guarantee
    /// that the device focus will return when the move is completed.
    ///
    /// # Arguments
    ///
    /// - `seat`: the wl_seat of the user event
    /// - `serial`: the serial of the user event
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_move(
        &mut self,
        slf: &Rc<JayPopupExtV1>,
        seat: &Rc<WlSeat>,
        serial: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_move(
            seat,
            serial,
        );
        if let Err(e) = res {
            log_forward("jay_popup_ext_v1.move", &e);
        }
    }

    /// start an interactive resize
    ///
    /// Start a user-driven, interactive resize of the surface.
    ///
    /// This request must be used in response to some sort of user action
    /// like a button press, key press, or touch down event. The passed
    /// serial is used to determine the type of interactive resize (touch,
    /// pointer, etc).
    ///
    /// The server may ignore resize requests. For example, if the passed serial
    /// is no longer valid.
    ///
    /// If triggered, the client will receive configure events with the
    /// expected sizes. The client must also acknowledge configure events using
    /// "ack_configure".
    ///
    /// If triggered, the surface also will lose the focus of the device
    /// (wl_pointer, wl_touch, etc) used for the resize. It is up to the
    /// compositor to visually indicate that the resize is taking place,
    /// such as updating a pointer cursor, during the resize. There is no
    /// guarantee that the device focus will return when the resize is
    /// completed.
    ///
    /// The edges parameter specifies how the surface should be resized, and
    /// is one of the values of the resize_edge enum. Values not matching
    /// a variant of the enum will cause the invalid_resize_edge protocol error.
    /// The compositor may use this information to update the surface position
    /// for example when dragging the top left corner. The compositor may also
    /// use this information to adapt its behavior, e.g. choose an appropriate
    /// cursor image.
    ///
    /// # Arguments
    ///
    /// - `seat`: the wl_seat of the user event
    /// - `serial`: the serial of the user event
    /// - `edges`: which edge or corner is being dragged
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_resize(
        &mut self,
        slf: &Rc<JayPopupExtV1>,
        seat: &Rc<WlSeat>,
        serial: u32,
        edges: XdgToplevelResizeEdge,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_resize(
            seat,
            serial,
            edges,
        );
        if let Err(e) = res {
            log_forward("jay_popup_ext_v1.resize", &e);
        }
    }
}

impl ObjectPrivate for JayPopupExtV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::JayPopupExtV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> jay_popup_ext_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> jay_popup_ext_v1#{}.move(seat: wl_seat#{}, serial: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("seat", o.core().interface, ObjectInterface::WlSeat)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_move(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_move(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                let arg2 = XdgToplevelResizeEdge(arg2);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: XdgToplevelResizeEdge) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> jay_popup_ext_v1#{}.resize(seat: wl_seat#{}, serial: {}, edges: {:?})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("seat", o.core().interface, ObjectInterface::WlSeat)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_resize(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_resize(&self, arg0, arg1, arg2);
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
            1 => "move",
            2 => "resize",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for JayPopupExtV1 {
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

impl JayPopupExtV1 {
    /// Since when the error.invalid_resize_edge enum variant is available.
    pub const ENM__ERROR_INVALID_RESIZE_EDGE__SINCE: u32 = 1;
    /// Since when the error.has_extension enum variant is available.
    pub const ENM__ERROR_HAS_EXTENSION__SINCE: u32 = 1;
}

/// fatal error
///
/// These fatal protocol errors may be emitted in response to
/// invalid requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct JayPopupExtV1Error(pub u32);

impl JayPopupExtV1Error {
    /// provided value is not a valid variant of the resize_edge enum
    pub const INVALID_RESIZE_EDGE: Self = Self(0);

    /// the xdg_popup still has an extension object
    pub const HAS_EXTENSION: Self = Self(1);
}

impl Debug for JayPopupExtV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_RESIZE_EDGE => "INVALID_RESIZE_EDGE",
            Self::HAS_EXTENSION => "HAS_EXTENSION",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
