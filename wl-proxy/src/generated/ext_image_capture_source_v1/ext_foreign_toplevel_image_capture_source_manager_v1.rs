//! image capture source manager for foreign toplevels
//!
//! A manager for creating image capture source objects for
//! ext_foreign_toplevel_handle_v1 objects.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_foreign_toplevel_image_capture_source_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ExtForeignToplevelImageCaptureSourceManagerV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn ExtForeignToplevelImageCaptureSourceManagerV1Handler>,
}

struct DefaultHandler;

impl ExtForeignToplevelImageCaptureSourceManagerV1Handler for DefaultHandler { }

impl ExtForeignToplevelImageCaptureSourceManagerV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "ext_foreign_toplevel_image_capture_source_manager_v1";
}

impl ExtForeignToplevelImageCaptureSourceManagerV1 {
    pub fn set_handler(&self, handler: impl ExtForeignToplevelImageCaptureSourceManagerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

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
    #[allow(dead_code)]
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
    pub fn send_create_source(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("toplevel_handle")),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("source", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_foreign_toplevel_image_capture_source_manager_v1#{}.create_source(source: ext_image_capture_source_v1#{}, toplevel_handle: ext_foreign_toplevel_handle_v1#{})\n", id, arg0_id, arg1_id);
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
            arg1_id,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// delete this object
    ///
    /// Destroys the manager. This request may be sent at any time by the client
    /// and objects created by the manager will remain valid after its
    /// destruction.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_foreign_toplevel_image_capture_source_manager_v1#{}.destroy()\n", id);
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
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [ExtForeignToplevelImageCaptureSourceManagerV1] proxies.
#[allow(dead_code)]
pub trait ExtForeignToplevelImageCaptureSourceManagerV1Handler: Any {
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
    fn create_source(
        &mut self,
        _slf: &Rc<ExtForeignToplevelImageCaptureSourceManagerV1>,
        source: &Rc<ExtImageCaptureSourceV1>,
        toplevel_handle: &Rc<ExtForeignToplevelHandleV1>,
    ) {
        let res = _slf.send_create_source(
            source,
            toplevel_handle,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_foreign_toplevel_image_capture_source_manager_v1.create_source message: {}", Report::new(e));
        }
    }

    /// delete this object
    ///
    /// Destroys the manager. This request may be sent at any time by the client
    /// and objects created by the manager will remain valid after its
    /// destruction.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<ExtForeignToplevelImageCaptureSourceManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_foreign_toplevel_image_capture_source_manager_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for ExtForeignToplevelImageCaptureSourceManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::ExtForeignToplevelImageCaptureSourceManagerV1, version),
            handler: Default::default(),
        })
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_foreign_toplevel_image_capture_source_manager_v1#{}.create_source(source: ext_image_capture_source_v1#{}, toplevel_handle: ext_foreign_toplevel_handle_v1#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ExtImageCaptureSourceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "source", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<ExtForeignToplevelHandleV1>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("toplevel_handle", o.core().interface, ProxyInterface::ExtForeignToplevelHandleV1));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).create_source(&self, arg0, arg1);
                } else {
                    DefaultHandler.create_source(&self, arg0, arg1);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_foreign_toplevel_image_capture_source_manager_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
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

impl Proxy for ExtForeignToplevelImageCaptureSourceManagerV1 {
    fn core(&self) -> &ProxyCore {
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

