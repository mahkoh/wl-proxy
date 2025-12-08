//! protocol for system tray items
//!
//! This is a global that advertises a tray area. It allows clients to
//! displays items in the tray.
//!
//! The compositor can advertise multiple globals of this interface. For
//! example, if there are multiple outputs with one tray area each. In that
//! case, clients that want to display tray items should bind to and use all
//! of these globals.
//!
//! The compositor can remove this global at any time. For example, when an
//! output containing a tray area is disconnected. In that case, the items
//! created from the global will no longer be displayed and the client should
//! destroy all associated objects.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A jay_tray_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct JayTrayV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn JayTrayV1Handler>,
}

struct DefaultHandler;

impl JayTrayV1Handler for DefaultHandler { }

impl ConcreteObject for JayTrayV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::JayTrayV1;
    const INTERFACE_NAME: &str = "jay_tray_v1";
}

impl JayTrayV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl JayTrayV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn JayTrayV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for JayTrayV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JayTrayV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl JayTrayV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy this object
    ///
    /// Destroy this object.
    ///
    /// Created tray items are not affected by this.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= jay_tray_v1#{}.destroy()\n", id);
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

    /// destroy this object
    ///
    /// Destroy this object.
    ///
    /// Created tray items are not affected by this.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("jay_tray_v1.destroy", &e);
        }
    }

    /// Since when the get_tray_item message is available.
    pub const MSG__GET_TRAY_ITEM__SINCE: u32 = 1;

    /// request tray item interface for surface
    ///
    /// Create a tray item for a surface.
    ///
    /// The surface is assigned the ext-tray-item-v1 role. If the surface
    /// already has another role, the conflicting_role error is emitted.
    ///
    /// If the surface already has a role object, the already_exists error is
    /// emitted.
    ///
    /// # Arguments
    ///
    /// - `id`: the new tray item
    /// - `surface`: the underlying surface
    #[inline]
    pub fn try_send_get_tray_item(
        &self,
        id: &Rc<JayTrayItemV1>,
        surface: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            surface,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= jay_tray_v1#{}.get_tray_item(id: jay_tray_item_v1#{}, surface: wl_surface#{})\n", id, arg0, arg1);
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

    /// request tray item interface for surface
    ///
    /// Create a tray item for a surface.
    ///
    /// The surface is assigned the ext-tray-item-v1 role. If the surface
    /// already has another role, the conflicting_role error is emitted.
    ///
    /// If the surface already has a role object, the already_exists error is
    /// emitted.
    ///
    /// # Arguments
    ///
    /// - `id`: the new tray item
    /// - `surface`: the underlying surface
    #[inline]
    pub fn send_get_tray_item(
        &self,
        id: &Rc<JayTrayItemV1>,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_get_tray_item(
            id,
            surface,
        );
        if let Err(e) = res {
            log_send("jay_tray_v1.get_tray_item", &e);
        }
    }

    /// request tray item interface for surface
    ///
    /// Create a tray item for a surface.
    ///
    /// The surface is assigned the ext-tray-item-v1 role. If the surface
    /// already has another role, the conflicting_role error is emitted.
    ///
    /// If the surface already has a role object, the already_exists error is
    /// emitted.
    ///
    /// # Arguments
    ///
    /// - `surface`: the underlying surface
    #[inline]
    pub fn new_try_send_get_tray_item(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Result<Rc<JayTrayItemV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_tray_item(
            &id,
            surface,
        )?;
        Ok(id)
    }

    /// request tray item interface for surface
    ///
    /// Create a tray item for a surface.
    ///
    /// The surface is assigned the ext-tray-item-v1 role. If the surface
    /// already has another role, the conflicting_role error is emitted.
    ///
    /// If the surface already has a role object, the already_exists error is
    /// emitted.
    ///
    /// # Arguments
    ///
    /// - `surface`: the underlying surface
    #[inline]
    pub fn new_send_get_tray_item(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Rc<JayTrayItemV1> {
        let id = self.core.create_child();
        self.send_get_tray_item(
            &id,
            surface,
        );
        id
    }
}

/// A message handler for [`JayTrayV1`] proxies.
pub trait JayTrayV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<JayTrayV1>) {
        slf.core.delete_id();
    }

    /// destroy this object
    ///
    /// Destroy this object.
    ///
    /// Created tray items are not affected by this.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<JayTrayV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("jay_tray_v1.destroy", &e);
        }
    }

    /// request tray item interface for surface
    ///
    /// Create a tray item for a surface.
    ///
    /// The surface is assigned the ext-tray-item-v1 role. If the surface
    /// already has another role, the conflicting_role error is emitted.
    ///
    /// If the surface already has a role object, the already_exists error is
    /// emitted.
    ///
    /// # Arguments
    ///
    /// - `id`: the new tray item
    /// - `surface`: the underlying surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_tray_item(
        &mut self,
        slf: &Rc<JayTrayV1>,
        id: &Rc<JayTrayItemV1>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_tray_item(
            id,
            surface,
        );
        if let Err(e) = res {
            log_forward("jay_tray_v1.get_tray_item", &e);
        }
    }
}

impl ObjectPrivate for JayTrayV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::JayTrayV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> jay_tray_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> jay_tray_v1#{}.get_tray_item(id: jay_tray_item_v1#{}, surface: wl_surface#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = JayTrayItemV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_tray_item(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_tray_item(&self, arg0, arg1);
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
            1 => "get_tray_item",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for JayTrayV1 {
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

impl JayTrayV1 {
    /// Since when the error.conflicting_role enum variant is available.
    pub const ENM__ERROR_CONFLICTING_ROLE__SINCE: u32 = 1;
    /// Since when the error.already_exists enum variant is available.
    pub const ENM__ERROR_ALREADY_EXISTS__SINCE: u32 = 1;
}

/// fatal error
///
/// These fatal protocol errors may be emitted in response to
/// invalid requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct JayTrayV1Error(pub u32);

impl JayTrayV1Error {
    /// the surface already has another role
    pub const CONFLICTING_ROLE: Self = Self(0);

    /// tray item already exists for the surface
    pub const ALREADY_EXISTS: Self = Self(1);
}

impl Debug for JayTrayV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::CONFLICTING_ROLE => "CONFLICTING_ROLE",
            Self::ALREADY_EXISTS => "ALREADY_EXISTS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
