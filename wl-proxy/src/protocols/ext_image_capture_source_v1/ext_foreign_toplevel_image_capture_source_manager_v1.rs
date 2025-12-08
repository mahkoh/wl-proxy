//! image capture source manager for foreign toplevels
//!
//! A manager for creating image capture source objects for
//! ext_foreign_toplevel_handle_v1 objects.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A ext_foreign_toplevel_image_capture_source_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ExtForeignToplevelImageCaptureSourceManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ExtForeignToplevelImageCaptureSourceManagerV1Handler>,
}

struct DefaultHandler;

impl ExtForeignToplevelImageCaptureSourceManagerV1Handler for DefaultHandler { }

impl ConcreteObject for ExtForeignToplevelImageCaptureSourceManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ExtForeignToplevelImageCaptureSourceManagerV1;
    const INTERFACE_NAME: &str = "ext_foreign_toplevel_image_capture_source_manager_v1";
}

impl ExtForeignToplevelImageCaptureSourceManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ExtForeignToplevelImageCaptureSourceManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ExtForeignToplevelImageCaptureSourceManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ExtForeignToplevelImageCaptureSourceManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtForeignToplevelImageCaptureSourceManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ExtForeignToplevelImageCaptureSourceManagerV1 {
    /// Since when the create_source message is available.
    pub const MSG__CREATE_SOURCE__SINCE: u32 = 1;

    /// create source object for foreign toplevel
    ///
    /// Creates a source object for a foreign toplevel handle. Images captured
    /// from this source will show the same content as the toplevel.
    ///
    /// # Arguments
    ///
    /// - `source`:
    /// - `toplevel_handle`:
    #[inline]
    pub fn try_send_create_source(
        &self,
        source: &Rc<ExtImageCaptureSourceV1>,
        toplevel_handle: &Rc<ExtForeignToplevelHandleV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            source,
            toplevel_handle,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("toplevel_handle"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("source", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_foreign_toplevel_image_capture_source_manager_v1#{}.create_source(source: ext_image_capture_source_v1#{}, toplevel_handle: ext_foreign_toplevel_handle_v1#{})\n", id, arg0, arg1);
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
            0,
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// create source object for foreign toplevel
    ///
    /// Creates a source object for a foreign toplevel handle. Images captured
    /// from this source will show the same content as the toplevel.
    ///
    /// # Arguments
    ///
    /// - `source`:
    /// - `toplevel_handle`:
    #[inline]
    pub fn send_create_source(
        &self,
        source: &Rc<ExtImageCaptureSourceV1>,
        toplevel_handle: &Rc<ExtForeignToplevelHandleV1>,
    ) {
        let res = self.try_send_create_source(
            source,
            toplevel_handle,
        );
        if let Err(e) = res {
            log_send("ext_foreign_toplevel_image_capture_source_manager_v1.create_source", &e);
        }
    }

    /// create source object for foreign toplevel
    ///
    /// Creates a source object for a foreign toplevel handle. Images captured
    /// from this source will show the same content as the toplevel.
    ///
    /// # Arguments
    ///
    /// - `toplevel_handle`:
    #[inline]
    pub fn new_try_send_create_source(
        &self,
        toplevel_handle: &Rc<ExtForeignToplevelHandleV1>,
    ) -> Result<Rc<ExtImageCaptureSourceV1>, ObjectError> {
        let source = self.core.create_child();
        self.try_send_create_source(
            &source,
            toplevel_handle,
        )?;
        Ok(source)
    }

    /// create source object for foreign toplevel
    ///
    /// Creates a source object for a foreign toplevel handle. Images captured
    /// from this source will show the same content as the toplevel.
    ///
    /// # Arguments
    ///
    /// - `toplevel_handle`:
    #[inline]
    pub fn new_send_create_source(
        &self,
        toplevel_handle: &Rc<ExtForeignToplevelHandleV1>,
    ) -> Rc<ExtImageCaptureSourceV1> {
        let source = self.core.create_child();
        self.send_create_source(
            &source,
            toplevel_handle,
        );
        source
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// delete this object
    ///
    /// Destroys the manager. This request may be sent at any time by the client
    /// and objects created by the manager will remain valid after its
    /// destruction.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_foreign_toplevel_image_capture_source_manager_v1#{}.destroy()\n", id);
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

    /// delete this object
    ///
    /// Destroys the manager. This request may be sent at any time by the client
    /// and objects created by the manager will remain valid after its
    /// destruction.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("ext_foreign_toplevel_image_capture_source_manager_v1.destroy", &e);
        }
    }
}

/// A message handler for [`ExtForeignToplevelImageCaptureSourceManagerV1`] proxies.
pub trait ExtForeignToplevelImageCaptureSourceManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ExtForeignToplevelImageCaptureSourceManagerV1>) {
        slf.core.delete_id();
    }

    /// create source object for foreign toplevel
    ///
    /// Creates a source object for a foreign toplevel handle. Images captured
    /// from this source will show the same content as the toplevel.
    ///
    /// # Arguments
    ///
    /// - `source`:
    /// - `toplevel_handle`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_create_source(
        &mut self,
        slf: &Rc<ExtForeignToplevelImageCaptureSourceManagerV1>,
        source: &Rc<ExtImageCaptureSourceV1>,
        toplevel_handle: &Rc<ExtForeignToplevelHandleV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_source(
            source,
            toplevel_handle,
        );
        if let Err(e) = res {
            log_forward("ext_foreign_toplevel_image_capture_source_manager_v1.create_source", &e);
        }
    }

    /// delete this object
    ///
    /// Destroys the manager. This request may be sent at any time by the client
    /// and objects created by the manager will remain valid after its
    /// destruction.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ExtForeignToplevelImageCaptureSourceManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("ext_foreign_toplevel_image_capture_source_manager_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for ExtForeignToplevelImageCaptureSourceManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ExtForeignToplevelImageCaptureSourceManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_foreign_toplevel_image_capture_source_manager_v1#{}.create_source(source: ext_image_capture_source_v1#{}, toplevel_handle: ext_foreign_toplevel_handle_v1#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = ExtImageCaptureSourceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "source", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<ExtForeignToplevelHandleV1>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("toplevel_handle", o.core().interface, ObjectInterface::ExtForeignToplevelHandleV1)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_create_source(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_create_source(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_foreign_toplevel_image_capture_source_manager_v1#{}.destroy()\n", client_id, id);
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
            0 => "create_source",
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

impl Object for ExtForeignToplevelImageCaptureSourceManagerV1 {
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

