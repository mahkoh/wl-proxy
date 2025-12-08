//! manager for focus grab objects
//!
//! This interface allows a client to create surface grab objects.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A hyprland_focus_grab_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct HyprlandFocusGrabManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn HyprlandFocusGrabManagerV1Handler>,
}

struct DefaultHandler;

impl HyprlandFocusGrabManagerV1Handler for DefaultHandler { }

impl ConcreteObject for HyprlandFocusGrabManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::HyprlandFocusGrabManagerV1;
    const INTERFACE_NAME: &str = "hyprland_focus_grab_manager_v1";
}

impl HyprlandFocusGrabManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl HyprlandFocusGrabManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn HyprlandFocusGrabManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for HyprlandFocusGrabManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HyprlandFocusGrabManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl HyprlandFocusGrabManagerV1 {
    /// Since when the create_grab message is available.
    pub const MSG__CREATE_GRAB__SINCE: u32 = 1;

    /// create a focus grab object
    ///
    /// Create a surface grab object.
    ///
    /// # Arguments
    ///
    /// - `grab`:
    #[inline]
    pub fn try_send_create_grab(
        &self,
        grab: &Rc<HyprlandFocusGrabV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            grab,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("grab", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_focus_grab_manager_v1#{}.create_grab(grab: hyprland_focus_grab_v1#{})\n", id, arg0);
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

    /// create a focus grab object
    ///
    /// Create a surface grab object.
    ///
    /// # Arguments
    ///
    /// - `grab`:
    #[inline]
    pub fn send_create_grab(
        &self,
        grab: &Rc<HyprlandFocusGrabV1>,
    ) {
        let res = self.try_send_create_grab(
            grab,
        );
        if let Err(e) = res {
            log_send("hyprland_focus_grab_manager_v1.create_grab", &e);
        }
    }

    /// create a focus grab object
    ///
    /// Create a surface grab object.
    #[inline]
    pub fn new_try_send_create_grab(
        &self,
    ) -> Result<Rc<HyprlandFocusGrabV1>, ObjectError> {
        let grab = self.core.create_child();
        self.try_send_create_grab(
            &grab,
        )?;
        Ok(grab)
    }

    /// create a focus grab object
    ///
    /// Create a surface grab object.
    #[inline]
    pub fn new_send_create_grab(
        &self,
    ) -> Rc<HyprlandFocusGrabV1> {
        let grab = self.core.create_child();
        self.send_create_grab(
            &grab,
        );
        grab
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the focus grab manager
    ///
    /// Destroy the focus grab manager.
    /// This doesn't destroy existing focus grab objects.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_focus_grab_manager_v1#{}.destroy()\n", id);
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
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the focus grab manager
    ///
    /// Destroy the focus grab manager.
    /// This doesn't destroy existing focus grab objects.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("hyprland_focus_grab_manager_v1.destroy", &e);
        }
    }
}

/// A message handler for [`HyprlandFocusGrabManagerV1`] proxies.
pub trait HyprlandFocusGrabManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<HyprlandFocusGrabManagerV1>) {
        slf.core.delete_id();
    }

    /// create a focus grab object
    ///
    /// Create a surface grab object.
    ///
    /// # Arguments
    ///
    /// - `grab`:
    #[inline]
    fn handle_create_grab(
        &mut self,
        slf: &Rc<HyprlandFocusGrabManagerV1>,
        grab: &Rc<HyprlandFocusGrabV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_grab(
            grab,
        );
        if let Err(e) = res {
            log_forward("hyprland_focus_grab_manager_v1.create_grab", &e);
        }
    }

    /// destroy the focus grab manager
    ///
    /// Destroy the focus grab manager.
    /// This doesn't destroy existing focus grab objects.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<HyprlandFocusGrabManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("hyprland_focus_grab_manager_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for HyprlandFocusGrabManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::HyprlandFocusGrabManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_focus_grab_manager_v1#{}.create_grab(grab: hyprland_focus_grab_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = HyprlandFocusGrabV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "grab", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_grab(&self, arg0);
                } else {
                    DefaultHandler.handle_create_grab(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_focus_grab_manager_v1#{}.destroy()\n", client_id, id);
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
            0 => "create_grab",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for HyprlandFocusGrabManagerV1 {
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

