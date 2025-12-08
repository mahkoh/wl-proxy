//! manage input capture sessions
//!
//! This interface allows to create an input capture session.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A hyprland_input_capture_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct HyprlandInputCaptureManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn HyprlandInputCaptureManagerV1Handler>,
}

struct DefaultHandler;

impl HyprlandInputCaptureManagerV1Handler for DefaultHandler { }

impl ConcreteObject for HyprlandInputCaptureManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::HyprlandInputCaptureManagerV1;
    const INTERFACE_NAME: &str = "hyprland_input_capture_manager_v1";
}

impl HyprlandInputCaptureManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl HyprlandInputCaptureManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn HyprlandInputCaptureManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for HyprlandInputCaptureManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HyprlandInputCaptureManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl HyprlandInputCaptureManagerV1 {
    /// Since when the create_session message is available.
    pub const MSG__CREATE_SESSION__SINCE: u32 = 1;

    /// create a input capture session
    ///
    /// Create a input capture session.
    ///
    /// # Arguments
    ///
    /// - `session`:
    /// - `handle`:
    #[inline]
    pub fn try_send_create_session(
        &self,
        session: &Rc<HyprlandInputCaptureV1>,
        handle: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            session,
            handle,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("session", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_input_capture_manager_v1#{}.create_session(session: hyprland_input_capture_v1#{}, handle: {:?})\n", id, arg0, arg1);
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
        ]);
        fmt.string(arg1);
        Ok(())
    }

    /// create a input capture session
    ///
    /// Create a input capture session.
    ///
    /// # Arguments
    ///
    /// - `session`:
    /// - `handle`:
    #[inline]
    pub fn send_create_session(
        &self,
        session: &Rc<HyprlandInputCaptureV1>,
        handle: &str,
    ) {
        let res = self.try_send_create_session(
            session,
            handle,
        );
        if let Err(e) = res {
            log_send("hyprland_input_capture_manager_v1.create_session", &e);
        }
    }

    /// create a input capture session
    ///
    /// Create a input capture session.
    ///
    /// # Arguments
    ///
    /// - `handle`:
    #[inline]
    pub fn new_try_send_create_session(
        &self,
        handle: &str,
    ) -> Result<Rc<HyprlandInputCaptureV1>, ObjectError> {
        let session = self.core.create_child();
        self.try_send_create_session(
            &session,
            handle,
        )?;
        Ok(session)
    }

    /// create a input capture session
    ///
    /// Create a input capture session.
    ///
    /// # Arguments
    ///
    /// - `handle`:
    #[inline]
    pub fn new_send_create_session(
        &self,
        handle: &str,
    ) -> Rc<HyprlandInputCaptureV1> {
        let session = self.core.create_child();
        self.send_create_session(
            &session,
            handle,
        );
        session
    }
}

/// A message handler for [`HyprlandInputCaptureManagerV1`] proxies.
pub trait HyprlandInputCaptureManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<HyprlandInputCaptureManagerV1>) {
        slf.core.delete_id();
    }

    /// create a input capture session
    ///
    /// Create a input capture session.
    ///
    /// # Arguments
    ///
    /// - `session`:
    /// - `handle`:
    #[inline]
    fn handle_create_session(
        &mut self,
        slf: &Rc<HyprlandInputCaptureManagerV1>,
        session: &Rc<HyprlandInputCaptureV1>,
        handle: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_session(
            session,
            handle,
        );
        if let Err(e) = res {
            log_forward("hyprland_input_capture_manager_v1.create_session", &e);
        }
    }
}

impl ObjectPrivate for HyprlandInputCaptureManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::HyprlandInputCaptureManagerV1, version),
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
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("session")));
                };
                offset += 1;
                let arg1;
                (arg1, offset) = parse_string::<NonNullString>(msg, offset, "handle")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_input_capture_manager_v1#{}.create_session(session: hyprland_input_capture_v1#{}, handle: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = HyprlandInputCaptureV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "session", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_session(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_create_session(&self, arg0, arg1);
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
            0 => "create_session",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for HyprlandInputCaptureManagerV1 {
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

