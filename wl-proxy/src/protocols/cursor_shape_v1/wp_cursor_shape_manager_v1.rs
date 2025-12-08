//! cursor shape manager
//!
//! This global offers an alternative, optional way to set cursor images. This
//! new way uses enumerated cursors instead of a wl_surface like
//! wl_pointer.set_cursor does.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_cursor_shape_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpCursorShapeManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpCursorShapeManagerV1Handler>,
}

struct DefaultHandler;

impl WpCursorShapeManagerV1Handler for DefaultHandler { }

impl ConcreteObject for WpCursorShapeManagerV1 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::WpCursorShapeManagerV1;
    const INTERFACE_NAME: &str = "wp_cursor_shape_manager_v1";
}

impl WpCursorShapeManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WpCursorShapeManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WpCursorShapeManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpCursorShapeManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpCursorShapeManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpCursorShapeManagerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the manager
    ///
    /// Destroy the cursor shape manager.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_cursor_shape_manager_v1#{}.destroy()\n", id);
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

    /// destroy the manager
    ///
    /// Destroy the cursor shape manager.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wp_cursor_shape_manager_v1.destroy", &e);
        }
    }

    /// Since when the get_pointer message is available.
    pub const MSG__GET_POINTER__SINCE: u32 = 1;

    /// manage the cursor shape of a pointer device
    ///
    /// Obtain a wp_cursor_shape_device_v1 for a wl_pointer object.
    ///
    /// When the pointer capability is removed from the wl_seat, the
    /// wp_cursor_shape_device_v1 object becomes inert.
    ///
    /// # Arguments
    ///
    /// - `cursor_shape_device`:
    /// - `pointer`:
    #[inline]
    pub fn try_send_get_pointer(
        &self,
        cursor_shape_device: &Rc<WpCursorShapeDeviceV1>,
        pointer: &Rc<WlPointer>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            cursor_shape_device,
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
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("cursor_shape_device", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_cursor_shape_manager_v1#{}.get_pointer(cursor_shape_device: wp_cursor_shape_device_v1#{}, pointer: wl_pointer#{})\n", id, arg0, arg1);
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

    /// manage the cursor shape of a pointer device
    ///
    /// Obtain a wp_cursor_shape_device_v1 for a wl_pointer object.
    ///
    /// When the pointer capability is removed from the wl_seat, the
    /// wp_cursor_shape_device_v1 object becomes inert.
    ///
    /// # Arguments
    ///
    /// - `cursor_shape_device`:
    /// - `pointer`:
    #[inline]
    pub fn send_get_pointer(
        &self,
        cursor_shape_device: &Rc<WpCursorShapeDeviceV1>,
        pointer: &Rc<WlPointer>,
    ) {
        let res = self.try_send_get_pointer(
            cursor_shape_device,
            pointer,
        );
        if let Err(e) = res {
            log_send("wp_cursor_shape_manager_v1.get_pointer", &e);
        }
    }

    /// manage the cursor shape of a pointer device
    ///
    /// Obtain a wp_cursor_shape_device_v1 for a wl_pointer object.
    ///
    /// When the pointer capability is removed from the wl_seat, the
    /// wp_cursor_shape_device_v1 object becomes inert.
    ///
    /// # Arguments
    ///
    /// - `pointer`:
    #[inline]
    pub fn new_try_send_get_pointer(
        &self,
        pointer: &Rc<WlPointer>,
    ) -> Result<Rc<WpCursorShapeDeviceV1>, ObjectError> {
        let cursor_shape_device = self.core.create_child();
        self.try_send_get_pointer(
            &cursor_shape_device,
            pointer,
        )?;
        Ok(cursor_shape_device)
    }

    /// manage the cursor shape of a pointer device
    ///
    /// Obtain a wp_cursor_shape_device_v1 for a wl_pointer object.
    ///
    /// When the pointer capability is removed from the wl_seat, the
    /// wp_cursor_shape_device_v1 object becomes inert.
    ///
    /// # Arguments
    ///
    /// - `pointer`:
    #[inline]
    pub fn new_send_get_pointer(
        &self,
        pointer: &Rc<WlPointer>,
    ) -> Rc<WpCursorShapeDeviceV1> {
        let cursor_shape_device = self.core.create_child();
        self.send_get_pointer(
            &cursor_shape_device,
            pointer,
        );
        cursor_shape_device
    }

    /// Since when the get_tablet_tool_v2 message is available.
    pub const MSG__GET_TABLET_TOOL_V2__SINCE: u32 = 1;

    /// manage the cursor shape of a tablet tool device
    ///
    /// Obtain a wp_cursor_shape_device_v1 for a zwp_tablet_tool_v2 object.
    ///
    /// When the zwp_tablet_tool_v2 is removed, the wp_cursor_shape_device_v1
    /// object becomes inert.
    ///
    /// # Arguments
    ///
    /// - `cursor_shape_device`:
    /// - `tablet_tool`:
    #[inline]
    pub fn try_send_get_tablet_tool_v2(
        &self,
        cursor_shape_device: &Rc<WpCursorShapeDeviceV1>,
        tablet_tool: &Rc<ZwpTabletToolV2>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            cursor_shape_device,
            tablet_tool,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("tablet_tool"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("cursor_shape_device", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_cursor_shape_manager_v1#{}.get_tablet_tool_v2(cursor_shape_device: wp_cursor_shape_device_v1#{}, tablet_tool: zwp_tablet_tool_v2#{})\n", id, arg0, arg1);
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

    /// manage the cursor shape of a tablet tool device
    ///
    /// Obtain a wp_cursor_shape_device_v1 for a zwp_tablet_tool_v2 object.
    ///
    /// When the zwp_tablet_tool_v2 is removed, the wp_cursor_shape_device_v1
    /// object becomes inert.
    ///
    /// # Arguments
    ///
    /// - `cursor_shape_device`:
    /// - `tablet_tool`:
    #[inline]
    pub fn send_get_tablet_tool_v2(
        &self,
        cursor_shape_device: &Rc<WpCursorShapeDeviceV1>,
        tablet_tool: &Rc<ZwpTabletToolV2>,
    ) {
        let res = self.try_send_get_tablet_tool_v2(
            cursor_shape_device,
            tablet_tool,
        );
        if let Err(e) = res {
            log_send("wp_cursor_shape_manager_v1.get_tablet_tool_v2", &e);
        }
    }

    /// manage the cursor shape of a tablet tool device
    ///
    /// Obtain a wp_cursor_shape_device_v1 for a zwp_tablet_tool_v2 object.
    ///
    /// When the zwp_tablet_tool_v2 is removed, the wp_cursor_shape_device_v1
    /// object becomes inert.
    ///
    /// # Arguments
    ///
    /// - `tablet_tool`:
    #[inline]
    pub fn new_try_send_get_tablet_tool_v2(
        &self,
        tablet_tool: &Rc<ZwpTabletToolV2>,
    ) -> Result<Rc<WpCursorShapeDeviceV1>, ObjectError> {
        let cursor_shape_device = self.core.create_child();
        self.try_send_get_tablet_tool_v2(
            &cursor_shape_device,
            tablet_tool,
        )?;
        Ok(cursor_shape_device)
    }

    /// manage the cursor shape of a tablet tool device
    ///
    /// Obtain a wp_cursor_shape_device_v1 for a zwp_tablet_tool_v2 object.
    ///
    /// When the zwp_tablet_tool_v2 is removed, the wp_cursor_shape_device_v1
    /// object becomes inert.
    ///
    /// # Arguments
    ///
    /// - `tablet_tool`:
    #[inline]
    pub fn new_send_get_tablet_tool_v2(
        &self,
        tablet_tool: &Rc<ZwpTabletToolV2>,
    ) -> Rc<WpCursorShapeDeviceV1> {
        let cursor_shape_device = self.core.create_child();
        self.send_get_tablet_tool_v2(
            &cursor_shape_device,
            tablet_tool,
        );
        cursor_shape_device
    }
}

/// A message handler for [`WpCursorShapeManagerV1`] proxies.
pub trait WpCursorShapeManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpCursorShapeManagerV1>) {
        slf.core.delete_id();
    }

    /// destroy the manager
    ///
    /// Destroy the cursor shape manager.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WpCursorShapeManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wp_cursor_shape_manager_v1.destroy", &e);
        }
    }

    /// manage the cursor shape of a pointer device
    ///
    /// Obtain a wp_cursor_shape_device_v1 for a wl_pointer object.
    ///
    /// When the pointer capability is removed from the wl_seat, the
    /// wp_cursor_shape_device_v1 object becomes inert.
    ///
    /// # Arguments
    ///
    /// - `cursor_shape_device`:
    /// - `pointer`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_pointer(
        &mut self,
        slf: &Rc<WpCursorShapeManagerV1>,
        cursor_shape_device: &Rc<WpCursorShapeDeviceV1>,
        pointer: &Rc<WlPointer>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_pointer(
            cursor_shape_device,
            pointer,
        );
        if let Err(e) = res {
            log_forward("wp_cursor_shape_manager_v1.get_pointer", &e);
        }
    }

    /// manage the cursor shape of a tablet tool device
    ///
    /// Obtain a wp_cursor_shape_device_v1 for a zwp_tablet_tool_v2 object.
    ///
    /// When the zwp_tablet_tool_v2 is removed, the wp_cursor_shape_device_v1
    /// object becomes inert.
    ///
    /// # Arguments
    ///
    /// - `cursor_shape_device`:
    /// - `tablet_tool`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_tablet_tool_v2(
        &mut self,
        slf: &Rc<WpCursorShapeManagerV1>,
        cursor_shape_device: &Rc<WpCursorShapeDeviceV1>,
        tablet_tool: &Rc<ZwpTabletToolV2>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_tablet_tool_v2(
            cursor_shape_device,
            tablet_tool,
        );
        if let Err(e) = res {
            log_forward("wp_cursor_shape_manager_v1.get_tablet_tool_v2", &e);
        }
    }
}

impl ObjectPrivate for WpCursorShapeManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpCursorShapeManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_cursor_shape_manager_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_cursor_shape_manager_v1#{}.get_pointer(cursor_shape_device: wp_cursor_shape_device_v1#{}, pointer: wl_pointer#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = WpCursorShapeDeviceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "cursor_shape_device", e)))?;
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
                    (**handler).handle_get_pointer(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_pointer(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_cursor_shape_manager_v1#{}.get_tablet_tool_v2(cursor_shape_device: wp_cursor_shape_device_v1#{}, tablet_tool: zwp_tablet_tool_v2#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = WpCursorShapeDeviceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "cursor_shape_device", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<ZwpTabletToolV2>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("tablet_tool", o.core().interface, ObjectInterface::ZwpTabletToolV2)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_tablet_tool_v2(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_tablet_tool_v2(&self, arg0, arg1);
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
            1 => "get_pointer",
            2 => "get_tablet_tool_v2",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WpCursorShapeManagerV1 {
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

