//! wayland protocol fixes
//!
//! This global fixes problems with other core-protocol interfaces that
//! cannot be fixed in these interfaces themselves.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wl_fixes proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWlFixes {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWlFixesMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWlFixesMessageHandler for DefaultMessageHandler { }

impl MetaWlFixes {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWlFixes {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WlFixes, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWlFixes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWlFixes")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWlFixes {
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
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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
        registry: &Rc<MetaWlRegistry>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            registry,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg0 = match arg0.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0,
        ]);
        Ok(())
    }
}

/// A message handler for [WlFixes] proxies.
#[allow(dead_code)]
pub trait MetaWlFixesMessageHandler {
    /// destroys this object
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWlFixes>,
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
        _slf: &Rc<MetaWlFixes>,
        registry: &Rc<MetaWlRegistry>,
    ) {
        let res = _slf.send_destroy_registry(
            registry,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_fixes.destroy_registry message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWlFixes {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let Some(arg0) = client.endpoint.lookup(arg0) else {
                    return Err(ObjectError);
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlRegistry>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).destroy_registry(&self, arg0);
                } else {
                    DefaultMessageHandler.destroy_registry(&self, arg0);
                }
            }
            _ => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
    }
}

