//! coordinate conversion reply

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A weston_touch_coordinate object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WestonTouchCoordinate {
    core: ObjectCore,
    handler: HandlerHolder<dyn WestonTouchCoordinateHandler>,
}

struct DefaultHandler;

impl WestonTouchCoordinateHandler for DefaultHandler { }

impl ConcreteObject for WestonTouchCoordinate {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WestonTouchCoordinate;
    const INTERFACE_NAME: &str = "weston_touch_coordinate";
}

impl WestonTouchCoordinate {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WestonTouchCoordinateHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WestonTouchCoordinateHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WestonTouchCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WestonTouchCoordinate")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WestonTouchCoordinate {
    /// Since when the result message is available.
    pub const MSG__RESULT__SINCE: u32 = 1;

    /// coordinates in raw touch space
    ///
    /// This event returns the conversion result from surface coordinates to
    /// the expected touch device coordinates.
    ///
    /// For details, see weston_touch_calibrator.convert. For the coordinate
    /// units, see weston_touch_calibrator.
    ///
    /// This event destroys the weston_touch_coordinate object.
    ///
    /// # Arguments
    ///
    /// - `x`: x coordinate in calibration units
    /// - `y`: y coordinate in calibration units
    #[inline]
    pub fn try_send_result(
        &self,
        x: u32,
        y: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            x,
            y,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_touch_coordinate#{}.result(x: {}, y: {})\n", client_id, id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1);
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
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// coordinates in raw touch space
    ///
    /// This event returns the conversion result from surface coordinates to
    /// the expected touch device coordinates.
    ///
    /// For details, see weston_touch_calibrator.convert. For the coordinate
    /// units, see weston_touch_calibrator.
    ///
    /// This event destroys the weston_touch_coordinate object.
    ///
    /// # Arguments
    ///
    /// - `x`: x coordinate in calibration units
    /// - `y`: y coordinate in calibration units
    #[inline]
    pub fn send_result(
        &self,
        x: u32,
        y: u32,
    ) {
        let res = self.try_send_result(
            x,
            y,
        );
        if let Err(e) = res {
            log_send("weston_touch_coordinate.result", &e);
        }
    }
}

/// A message handler for [`WestonTouchCoordinate`] proxies.
pub trait WestonTouchCoordinateHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WestonTouchCoordinate>) {
        slf.core.delete_id();
    }

    /// coordinates in raw touch space
    ///
    /// This event returns the conversion result from surface coordinates to
    /// the expected touch device coordinates.
    ///
    /// For details, see weston_touch_calibrator.convert. For the coordinate
    /// units, see weston_touch_calibrator.
    ///
    /// This event destroys the weston_touch_coordinate object.
    ///
    /// # Arguments
    ///
    /// - `x`: x coordinate in calibration units
    /// - `y`: y coordinate in calibration units
    #[inline]
    fn handle_result(
        &mut self,
        slf: &Rc<WestonTouchCoordinate>,
        x: u32,
        y: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_result(
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("weston_touch_coordinate.result", &e);
        }
    }
}

impl ObjectPrivate for WestonTouchCoordinate {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WestonTouchCoordinate, version),
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
            n => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
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
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_touch_coordinate#{}.result(x: {}, y: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_result(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_result(&self, arg0, arg1);
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
        let _ = id;
        None
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "result",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WestonTouchCoordinate {
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

