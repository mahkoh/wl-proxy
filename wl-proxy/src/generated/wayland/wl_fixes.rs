//! wayland protocol fixes
//!
//! This global fixes problems with other core-protocol interfaces that
//! cannot be fixed in these interfaces themselves.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wl_fixes proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlFixes {
    core: ProxyCore,
    handler: HandlerHolder<dyn WlFixesHandler>,
}

struct DefaultHandler;

impl WlFixesHandler for DefaultHandler { }

impl WlFixes {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "wl_fixes";
}

impl WlFixes {
    pub fn set_handler(&self, handler: impl WlFixesHandler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn WlFixesHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlFixes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlFixes")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlFixes {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroys this object
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_fixes#{}.destroy()\n", id);
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
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the destroy_registry message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY_REGISTRY__SINCE: u32 = 1;

    /// destroy a wl_registry
    ///
    /// This request destroys a wl_registry object.
    ///
    /// The client should no longer use the wl_registry after making this
    /// request.
    ///
    /// The compositor will emit a wl_display.delete_id event with the object ID
    /// of the registry and will no longer emit any events on the registry. The
    /// client should re-use the object ID once it receives the
    /// wl_display.delete_id event.
    ///
    /// # Arguments
    ///
    /// - `registry`: the registry to destroy
    #[inline]
    pub fn send_destroy_registry(
        &self,
        registry: &Rc<WlRegistry>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            registry,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("registry")),
            Some(id) => id,
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_fixes#{}.destroy_registry(registry: wl_registry#{})\n", id, arg0_id);
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
            arg0_id,
        ]);
        arg0.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [WlFixes] proxies.
#[allow(dead_code)]
pub trait WlFixesHandler: Any {
    /// destroys this object
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<WlFixes>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_fixes.destroy message: {}", Report::new(e));
        }
    }

    /// destroy a wl_registry
    ///
    /// This request destroys a wl_registry object.
    ///
    /// The client should no longer use the wl_registry after making this
    /// request.
    ///
    /// The compositor will emit a wl_display.delete_id event with the object ID
    /// of the registry and will no longer emit any events on the registry. The
    /// client should re-use the object ID once it receives the
    /// wl_display.delete_id event.
    ///
    /// # Arguments
    ///
    /// - `registry`: the registry to destroy
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn destroy_registry(
        &mut self,
        _slf: &Rc<WlFixes>,
        registry: &Rc<WlRegistry>,
    ) {
        let res = _slf.send_destroy_registry(
            registry,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_fixes.destroy_registry message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for WlFixes {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::WlFixes, version),
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
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_fixes#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_fixes#{}.destroy_registry(registry: wl_registry#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlRegistry>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("registry", o.core().interface, ProxyInterface::WlRegistry));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).destroy_registry(&self, arg0);
                } else {
                    DefaultHandler.destroy_registry(&self, arg0);
                }
                arg0.core().handle_client_destroy();
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
            0 => "destroy",
            1 => "destroy_registry",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Proxy for WlFixes {
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

