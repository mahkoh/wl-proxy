//! manager to control data devices
//!
//! This interface is a manager that allows creating per-seat data device
//! controls.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwlr_data_control_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwlrDataControlManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwlrDataControlManagerV1Handler>,
}

struct DefaultHandler;

impl ZwlrDataControlManagerV1Handler for DefaultHandler { }

impl ConcreteObject for ZwlrDataControlManagerV1 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwlrDataControlManagerV1;
    const INTERFACE_NAME: &str = "zwlr_data_control_manager_v1";
}

impl ZwlrDataControlManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwlrDataControlManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwlrDataControlManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwlrDataControlManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwlrDataControlManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwlrDataControlManagerV1 {
    /// Since when the create_data_source message is available.
    pub const MSG__CREATE_DATA_SOURCE__SINCE: u32 = 1;

    /// create a new data source
    ///
    /// Create a new data source.
    ///
    /// # Arguments
    ///
    /// - `id`: data source to create
    #[inline]
    pub fn try_send_create_data_source(
        &self,
        id: &Rc<ZwlrDataControlSourceV1>,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_data_control_manager_v1#{}.create_data_source(id: zwlr_data_control_source_v1#{})\n", id, arg0);
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

    /// create a new data source
    ///
    /// Create a new data source.
    ///
    /// # Arguments
    ///
    /// - `id`: data source to create
    #[inline]
    pub fn send_create_data_source(
        &self,
        id: &Rc<ZwlrDataControlSourceV1>,
    ) {
        let res = self.try_send_create_data_source(
            id,
        );
        if let Err(e) = res {
            log_send("zwlr_data_control_manager_v1.create_data_source", &e);
        }
    }

    /// create a new data source
    ///
    /// Create a new data source.
    #[inline]
    pub fn new_try_send_create_data_source(
        &self,
    ) -> Result<Rc<ZwlrDataControlSourceV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_create_data_source(
            &id,
        )?;
        Ok(id)
    }

    /// create a new data source
    ///
    /// Create a new data source.
    #[inline]
    pub fn new_send_create_data_source(
        &self,
    ) -> Rc<ZwlrDataControlSourceV1> {
        let id = self.core.create_child();
        self.send_create_data_source(
            &id,
        );
        id
    }

    /// Since when the get_data_device message is available.
    pub const MSG__GET_DATA_DEVICE__SINCE: u32 = 1;

    /// get a data device for a seat
    ///
    /// Create a data device that can be used to manage a seat's selection.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `seat`:
    #[inline]
    pub fn try_send_get_data_device(
        &self,
        id: &Rc<ZwlrDataControlDeviceV1>,
        seat: &Rc<WlSeat>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            seat,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("seat"))),
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_data_control_manager_v1#{}.get_data_device(id: zwlr_data_control_device_v1#{}, seat: wl_seat#{})\n", id, arg0, arg1);
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

    /// get a data device for a seat
    ///
    /// Create a data device that can be used to manage a seat's selection.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `seat`:
    #[inline]
    pub fn send_get_data_device(
        &self,
        id: &Rc<ZwlrDataControlDeviceV1>,
        seat: &Rc<WlSeat>,
    ) {
        let res = self.try_send_get_data_device(
            id,
            seat,
        );
        if let Err(e) = res {
            log_send("zwlr_data_control_manager_v1.get_data_device", &e);
        }
    }

    /// get a data device for a seat
    ///
    /// Create a data device that can be used to manage a seat's selection.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    #[inline]
    pub fn new_try_send_get_data_device(
        &self,
        seat: &Rc<WlSeat>,
    ) -> Result<Rc<ZwlrDataControlDeviceV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_data_device(
            &id,
            seat,
        )?;
        Ok(id)
    }

    /// get a data device for a seat
    ///
    /// Create a data device that can be used to manage a seat's selection.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    #[inline]
    pub fn new_send_get_data_device(
        &self,
        seat: &Rc<WlSeat>,
    ) -> Rc<ZwlrDataControlDeviceV1> {
        let id = self.core.create_child();
        self.send_get_data_device(
            &id,
            seat,
        );
        id
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the manager
    ///
    /// All objects created by the manager will still remain valid, until their
    /// appropriate destroy request has been called.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_data_control_manager_v1#{}.destroy()\n", id);
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
            2,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the manager
    ///
    /// All objects created by the manager will still remain valid, until their
    /// appropriate destroy request has been called.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwlr_data_control_manager_v1.destroy", &e);
        }
    }
}

/// A message handler for [`ZwlrDataControlManagerV1`] proxies.
pub trait ZwlrDataControlManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwlrDataControlManagerV1>) {
        slf.core.delete_id();
    }

    /// create a new data source
    ///
    /// Create a new data source.
    ///
    /// # Arguments
    ///
    /// - `id`: data source to create
    #[inline]
    fn handle_create_data_source(
        &mut self,
        slf: &Rc<ZwlrDataControlManagerV1>,
        id: &Rc<ZwlrDataControlSourceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_data_source(
            id,
        );
        if let Err(e) = res {
            log_forward("zwlr_data_control_manager_v1.create_data_source", &e);
        }
    }

    /// get a data device for a seat
    ///
    /// Create a data device that can be used to manage a seat's selection.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `seat`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_data_device(
        &mut self,
        slf: &Rc<ZwlrDataControlManagerV1>,
        id: &Rc<ZwlrDataControlDeviceV1>,
        seat: &Rc<WlSeat>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_data_device(
            id,
            seat,
        );
        if let Err(e) = res {
            log_forward("zwlr_data_control_manager_v1.get_data_device", &e);
        }
    }

    /// destroy the manager
    ///
    /// All objects created by the manager will still remain valid, until their
    /// appropriate destroy request has been called.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwlrDataControlManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwlr_data_control_manager_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for ZwlrDataControlManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwlrDataControlManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_data_control_manager_v1#{}.create_data_source(id: zwlr_data_control_source_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = ZwlrDataControlSourceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_data_source(&self, arg0);
                } else {
                    DefaultHandler.handle_create_data_source(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_data_control_manager_v1#{}.get_data_device(id: zwlr_data_control_device_v1#{}, seat: wl_seat#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = ZwlrDataControlDeviceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("seat", o.core().interface, ObjectInterface::WlSeat)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_data_device(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_data_device(&self, arg0, arg1);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_data_control_manager_v1#{}.destroy()\n", client_id, id);
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
            0 => "create_data_source",
            1 => "get_data_device",
            2 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for ZwlrDataControlManagerV1 {
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

