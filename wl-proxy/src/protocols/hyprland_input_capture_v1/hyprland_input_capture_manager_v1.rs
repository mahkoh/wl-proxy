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

impl HyprlandInputCaptureManagerV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::HyprlandInputCaptureManagerV1;
    pub const INTERFACE_NAME: &str = "hyprland_input_capture_manager_v1";
}

impl HyprlandInputCaptureManagerV1 {
    pub fn set_handler(&self, handler: impl HyprlandInputCaptureManagerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

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
    pub fn send_create_session(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("session", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_input_capture_manager_v1#{}.create_session(session: hyprland_input_capture_v1#{}, handle: {:?})\n", id, arg0_id, arg1);
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
            arg0_id,
        ]);
        fmt.string(arg1);
        Ok(())
    }
}

/// A message handler for [HyprlandInputCaptureManagerV1] proxies.
pub trait HyprlandInputCaptureManagerV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<HyprlandInputCaptureManagerV1>) {
        let _ = slf.core.delete_id();
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
        _slf: &Rc<HyprlandInputCaptureManagerV1>,
        session: &Rc<HyprlandInputCaptureV1>,
        handle: &str,
    ) {
        let res = _slf.send_create_session(
            session,
            handle,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a hyprland_input_capture_manager_v1.create_session message: {}", Report::new(e));
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
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err((ObjectError::HandlerBorrowed, self));
        };
        if let Some(handler) = &mut *handler {
            handler.delete_id(&self);
        } else {
            let _ = self.core.delete_id();
        }
        Ok(())
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("session"));
                };
                offset += 1;
                let arg1 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("handle"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("handle"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("handle"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("handle"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_input_capture_manager_v1#{}.create_session(session: hyprland_input_capture_v1#{}, handle: {:?})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = HyprlandInputCaptureV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "session", e))?;
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

