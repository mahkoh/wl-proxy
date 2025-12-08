//! manager for xdg_popup extensions
//!
//! This global allows clients to create jay_popup_ext_v1 extension objects
//! for xdg_popups.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A jay_popup_ext_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct JayPopupExtManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn JayPopupExtManagerV1Handler>,
}

struct DefaultHandler;

impl JayPopupExtManagerV1Handler for DefaultHandler { }

impl ConcreteObject for JayPopupExtManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::JayPopupExtManagerV1;
    const INTERFACE_NAME: &str = "jay_popup_ext_manager_v1";
}

impl JayPopupExtManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl JayPopupExtManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn JayPopupExtManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for JayPopupExtManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JayPopupExtManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl JayPopupExtManagerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroys this manager
    ///
    /// This request has no effect on any created extension objects.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= jay_popup_ext_manager_v1#{}.destroy()\n", id);
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

    /// destroys this manager
    ///
    /// This request has no effect on any created extension objects.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("jay_popup_ext_manager_v1.destroy", &e);
        }
    }

    /// Since when the get_ext message is available.
    pub const MSG__GET_EXT__SINCE: u32 = 1;

    /// creates a jay_popup_ext_v1 extension object
    ///
    /// Each xdg_popup can have at most one jay_popup_ext_v1 extension object.
    /// Sending this request while another extension object exists causes the
    /// already_exists error to be emitted.
    ///
    /// The extension object must be destroyed before the xdg_popup. Otherwise
    /// the has_extension error is emitted.
    ///
    /// # Arguments
    ///
    /// - `id`: the new jay_popup_ext_v1
    /// - `popup`: the xdg_popup
    #[inline]
    pub fn try_send_get_ext(
        &self,
        id: &Rc<JayPopupExtV1>,
        popup: &Rc<XdgPopup>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            popup,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("popup"))),
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= jay_popup_ext_manager_v1#{}.get_ext(id: jay_popup_ext_v1#{}, popup: xdg_popup#{})\n", id, arg0, arg1);
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

    /// creates a jay_popup_ext_v1 extension object
    ///
    /// Each xdg_popup can have at most one jay_popup_ext_v1 extension object.
    /// Sending this request while another extension object exists causes the
    /// already_exists error to be emitted.
    ///
    /// The extension object must be destroyed before the xdg_popup. Otherwise
    /// the has_extension error is emitted.
    ///
    /// # Arguments
    ///
    /// - `id`: the new jay_popup_ext_v1
    /// - `popup`: the xdg_popup
    #[inline]
    pub fn send_get_ext(
        &self,
        id: &Rc<JayPopupExtV1>,
        popup: &Rc<XdgPopup>,
    ) {
        let res = self.try_send_get_ext(
            id,
            popup,
        );
        if let Err(e) = res {
            log_send("jay_popup_ext_manager_v1.get_ext", &e);
        }
    }

    /// creates a jay_popup_ext_v1 extension object
    ///
    /// Each xdg_popup can have at most one jay_popup_ext_v1 extension object.
    /// Sending this request while another extension object exists causes the
    /// already_exists error to be emitted.
    ///
    /// The extension object must be destroyed before the xdg_popup. Otherwise
    /// the has_extension error is emitted.
    ///
    /// # Arguments
    ///
    /// - `popup`: the xdg_popup
    #[inline]
    pub fn new_try_send_get_ext(
        &self,
        popup: &Rc<XdgPopup>,
    ) -> Result<Rc<JayPopupExtV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_ext(
            &id,
            popup,
        )?;
        Ok(id)
    }

    /// creates a jay_popup_ext_v1 extension object
    ///
    /// Each xdg_popup can have at most one jay_popup_ext_v1 extension object.
    /// Sending this request while another extension object exists causes the
    /// already_exists error to be emitted.
    ///
    /// The extension object must be destroyed before the xdg_popup. Otherwise
    /// the has_extension error is emitted.
    ///
    /// # Arguments
    ///
    /// - `popup`: the xdg_popup
    #[inline]
    pub fn new_send_get_ext(
        &self,
        popup: &Rc<XdgPopup>,
    ) -> Rc<JayPopupExtV1> {
        let id = self.core.create_child();
        self.send_get_ext(
            &id,
            popup,
        );
        id
    }
}

/// A message handler for [`JayPopupExtManagerV1`] proxies.
pub trait JayPopupExtManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<JayPopupExtManagerV1>) {
        slf.core.delete_id();
    }

    /// destroys this manager
    ///
    /// This request has no effect on any created extension objects.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<JayPopupExtManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("jay_popup_ext_manager_v1.destroy", &e);
        }
    }

    /// creates a jay_popup_ext_v1 extension object
    ///
    /// Each xdg_popup can have at most one jay_popup_ext_v1 extension object.
    /// Sending this request while another extension object exists causes the
    /// already_exists error to be emitted.
    ///
    /// The extension object must be destroyed before the xdg_popup. Otherwise
    /// the has_extension error is emitted.
    ///
    /// # Arguments
    ///
    /// - `id`: the new jay_popup_ext_v1
    /// - `popup`: the xdg_popup
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_ext(
        &mut self,
        slf: &Rc<JayPopupExtManagerV1>,
        id: &Rc<JayPopupExtV1>,
        popup: &Rc<XdgPopup>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_ext(
            id,
            popup,
        );
        if let Err(e) = res {
            log_forward("jay_popup_ext_manager_v1.get_ext", &e);
        }
    }
}

impl ObjectPrivate for JayPopupExtManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::JayPopupExtManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> jay_popup_ext_manager_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> jay_popup_ext_manager_v1#{}.get_ext(id: jay_popup_ext_v1#{}, popup: xdg_popup#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = JayPopupExtV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<XdgPopup>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("popup", o.core().interface, ObjectInterface::XdgPopup)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_ext(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_ext(&self, arg0, arg1);
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
            1 => "get_ext",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for JayPopupExtManagerV1 {
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

impl JayPopupExtManagerV1 {
    /// Since when the error.already_exists enum variant is available.
    pub const ENM__ERROR_ALREADY_EXISTS__SINCE: u32 = 1;
}

/// fatal error
///
/// These fatal protocol errors may be emitted in response to
/// invalid requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct JayPopupExtManagerV1Error(pub u32);

impl JayPopupExtManagerV1Error {
    /// extension object already exists for the xdg_popup
    pub const ALREADY_EXISTS: Self = Self(1);
}

impl Debug for JayPopupExtManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ALREADY_EXISTS => "ALREADY_EXISTS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
