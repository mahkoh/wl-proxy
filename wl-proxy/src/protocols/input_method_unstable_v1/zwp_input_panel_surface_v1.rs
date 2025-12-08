use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_input_panel_surface_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpInputPanelSurfaceV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpInputPanelSurfaceV1Handler>,
}

struct DefaultHandler;

impl ZwpInputPanelSurfaceV1Handler for DefaultHandler { }

impl ConcreteObject for ZwpInputPanelSurfaceV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpInputPanelSurfaceV1;
    const INTERFACE_NAME: &str = "zwp_input_panel_surface_v1";
}

impl ZwpInputPanelSurfaceV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpInputPanelSurfaceV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpInputPanelSurfaceV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpInputPanelSurfaceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpInputPanelSurfaceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpInputPanelSurfaceV1 {
    /// Since when the set_toplevel message is available.
    pub const MSG__SET_TOPLEVEL__SINCE: u32 = 1;

    /// set the surface type as a keyboard
    ///
    /// Set the input_panel_surface type to keyboard.
    ///
    /// A keyboard surface is only shown when a text input is active.
    ///
    /// # Arguments
    ///
    /// - `output`:
    /// - `position`:
    #[inline]
    pub fn try_send_set_toplevel(
        &self,
        output: &Rc<WlOutput>,
        position: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            output,
            position,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("output"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_panel_surface_v1#{}.set_toplevel(output: wl_output#{}, position: {})\n", id, arg0, arg1);
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
            0,
            arg0_id,
            arg1,
        ]);
        Ok(())
    }

    /// set the surface type as a keyboard
    ///
    /// Set the input_panel_surface type to keyboard.
    ///
    /// A keyboard surface is only shown when a text input is active.
    ///
    /// # Arguments
    ///
    /// - `output`:
    /// - `position`:
    #[inline]
    pub fn send_set_toplevel(
        &self,
        output: &Rc<WlOutput>,
        position: u32,
    ) {
        let res = self.try_send_set_toplevel(
            output,
            position,
        );
        if let Err(e) = res {
            log_send("zwp_input_panel_surface_v1.set_toplevel", &e);
        }
    }

    /// Since when the set_overlay_panel message is available.
    pub const MSG__SET_OVERLAY_PANEL__SINCE: u32 = 1;

    /// set the surface type as an overlay panel
    ///
    /// Set the input_panel_surface to be an overlay panel.
    ///
    /// This is shown near the input cursor above the application window when
    /// a text input is active.
    #[inline]
    pub fn try_send_set_overlay_panel(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_panel_surface_v1#{}.set_overlay_panel()\n", id);
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
            1,
        ]);
        Ok(())
    }

    /// set the surface type as an overlay panel
    ///
    /// Set the input_panel_surface to be an overlay panel.
    ///
    /// This is shown near the input cursor above the application window when
    /// a text input is active.
    #[inline]
    pub fn send_set_overlay_panel(
        &self,
    ) {
        let res = self.try_send_set_overlay_panel(
        );
        if let Err(e) = res {
            log_send("zwp_input_panel_surface_v1.set_overlay_panel", &e);
        }
    }
}

/// A message handler for [`ZwpInputPanelSurfaceV1`] proxies.
pub trait ZwpInputPanelSurfaceV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpInputPanelSurfaceV1>) {
        slf.core.delete_id();
    }

    /// set the surface type as a keyboard
    ///
    /// Set the input_panel_surface type to keyboard.
    ///
    /// A keyboard surface is only shown when a text input is active.
    ///
    /// # Arguments
    ///
    /// - `output`:
    /// - `position`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_toplevel(
        &mut self,
        slf: &Rc<ZwpInputPanelSurfaceV1>,
        output: &Rc<WlOutput>,
        position: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_toplevel(
            output,
            position,
        );
        if let Err(e) = res {
            log_forward("zwp_input_panel_surface_v1.set_toplevel", &e);
        }
    }

    /// set the surface type as an overlay panel
    ///
    /// Set the input_panel_surface to be an overlay panel.
    ///
    /// This is shown near the input cursor above the application window when
    /// a text input is active.
    #[inline]
    fn handle_set_overlay_panel(
        &mut self,
        slf: &Rc<ZwpInputPanelSurfaceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_overlay_panel(
        );
        if let Err(e) = res {
            log_forward("zwp_input_panel_surface_v1.set_overlay_panel", &e);
        }
    }
}

impl ObjectPrivate for ZwpInputPanelSurfaceV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpInputPanelSurfaceV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_panel_surface_v1#{}.set_toplevel(output: wl_output#{}, position: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlOutput>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("output", o.core().interface, ObjectInterface::WlOutput)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_toplevel(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_toplevel(&self, arg0, arg1);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_panel_surface_v1#{}.set_overlay_panel()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_overlay_panel(&self);
                } else {
                    DefaultHandler.handle_set_overlay_panel(&self);
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
            0 => "set_toplevel",
            1 => "set_overlay_panel",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for ZwpInputPanelSurfaceV1 {
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

impl ZwpInputPanelSurfaceV1 {
    /// Since when the position.center_bottom enum variant is available.
    pub const ENM__POSITION_CENTER_BOTTOM__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwpInputPanelSurfaceV1Position(pub u32);

impl ZwpInputPanelSurfaceV1Position {
    pub const CENTER_BOTTOM: Self = Self(0);
}

impl Debug for ZwpInputPanelSurfaceV1Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::CENTER_BOTTOM => "CENTER_BOTTOM",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
