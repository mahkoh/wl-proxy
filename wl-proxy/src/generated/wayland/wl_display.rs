//! core global object
//!
//! The core global object.  This is a special singleton object.  It
//! is used for internal Wayland protocol features.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wl_display proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWlDisplay {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWlDisplayMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWlDisplayMessageHandler for DefaultMessageHandler { }

impl MetaWlDisplay {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWlDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWlDisplay")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWlDisplay {
    /// Since when the sync message is available.
    #[allow(dead_code)]
    pub const MSG__SYNC__SINCE: u32 = 1;

    /// asynchronous roundtrip
    ///
    /// The sync request asks the server to emit the 'done' event
    /// on the returned wl_callback object.  Since requests are
    /// handled in-order and events are delivered in-order, this can
    /// be used as a barrier to ensure all previous requests and the
    /// resulting events have been handled.
    ///
    /// The object returned by this request will be destroyed by the
    /// compositor after the callback is fired and as such the client must not
    /// attempt to use it after that point.
    ///
    /// The callback_data passed in the callback is undefined and should be ignored.
    #[inline]
    pub fn send_sync(
        &self,
        callback: &Rc<MetaWlCallback>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            callback,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0.server_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }

    /// Since when the get_registry message is available.
    #[allow(dead_code)]
    pub const MSG__GET_REGISTRY__SINCE: u32 = 1;

    /// get global registry object
    ///
    /// This request creates a registry object that allows the client
    /// to list and bind the global objects available from the
    /// compositor.
    ///
    /// It should be noted that the server side resources consumed in
    /// response to a get_registry request can only be released when the
    /// client disconnects, not when the client side proxy is destroyed.
    /// Therefore, clients should invoke get_registry as infrequently as
    /// possible to avoid wasting memory.
    #[inline]
    pub fn send_get_registry(
        &self,
        registry: &Rc<MetaWlRegistry>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            registry,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0.server_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }

    /// Since when the error message is available.
    #[allow(dead_code)]
    pub const MSG__ERROR__SINCE: u32 = 1;

    /// fatal error event
    ///
    /// The error event is sent out when a fatal (non-recoverable)
    /// error has occurred.  The object_id argument is the object
    /// where the error occurred, most often in response to a request
    /// to that object.  The code identifies the error and is defined
    /// by the object interface.  As such, each interface defines its
    /// own set of error codes.  The message is a brief description
    /// of the error, for (debugging) convenience.
    ///
    /// # Arguments
    ///
    /// - `object_id`: object where the error occurred
    /// - `code`: error code
    /// - `message`: error description
    #[inline]
    pub fn send_error(
        &self,
        object_id: Rc<dyn Proxy>,
        code: u32,
        message: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            object_id,
            code,
            message,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        if arg0.client_id.get() != Some(client.id) {
            return Err(ObjectError);
        }
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
            arg0.client_obj_id.get().unwrap_or(0),
            arg1,
        ]);
        fmt.string(arg2);
        Ok(())
    }

    /// Since when the delete_id message is available.
    #[allow(dead_code)]
    pub const MSG__DELETE_ID__SINCE: u32 = 1;

    /// acknowledge object ID deletion
    ///
    /// This event is used internally by the object ID management
    /// logic. When a client deletes an object that it had created,
    /// the server will send this event to acknowledge that it has
    /// seen the delete request. When the client receives this event,
    /// it will know that it can safely reuse the object ID.
    ///
    /// # Arguments
    ///
    /// - `id`: deleted object ID
    #[inline]
    pub fn send_delete_id(
        &self,
        id: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            1,
            arg0,
        ]);
        Ok(())
    }
}

/// A message handler for [WlDisplay] proxies.
#[allow(dead_code)]
pub trait MetaWlDisplayMessageHandler {
    /// asynchronous roundtrip
    ///
    /// The sync request asks the server to emit the 'done' event
    /// on the returned wl_callback object.  Since requests are
    /// handled in-order and events are delivered in-order, this can
    /// be used as a barrier to ensure all previous requests and the
    /// resulting events have been handled.
    ///
    /// The object returned by this request will be destroyed by the
    /// compositor after the callback is fired and as such the client must not
    /// attempt to use it after that point.
    ///
    /// The callback_data passed in the callback is undefined and should be ignored.
    ///
    /// # Arguments
    ///
    /// - `callback`: callback object for the sync request
    #[inline]
    fn sync(
        &mut self,
        _slf: &Rc<MetaWlDisplay>,
        callback: &Rc<MetaWlCallback>,
    ) {
        let res = _slf.send_sync(
            callback,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_display.sync message: {}", Report::new(e));
        }
    }

    /// get global registry object
    ///
    /// This request creates a registry object that allows the client
    /// to list and bind the global objects available from the
    /// compositor.
    ///
    /// It should be noted that the server side resources consumed in
    /// response to a get_registry request can only be released when the
    /// client disconnects, not when the client side proxy is destroyed.
    /// Therefore, clients should invoke get_registry as infrequently as
    /// possible to avoid wasting memory.
    ///
    /// # Arguments
    ///
    /// - `registry`: global registry object
    #[inline]
    fn get_registry(
        &mut self,
        _slf: &Rc<MetaWlDisplay>,
        registry: &Rc<MetaWlRegistry>,
    ) {
        let res = _slf.send_get_registry(
            registry,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_display.get_registry message: {}", Report::new(e));
        }
    }

    /// fatal error event
    ///
    /// The error event is sent out when a fatal (non-recoverable)
    /// error has occurred.  The object_id argument is the object
    /// where the error occurred, most often in response to a request
    /// to that object.  The code identifies the error and is defined
    /// by the object interface.  As such, each interface defines its
    /// own set of error codes.  The message is a brief description
    /// of the error, for (debugging) convenience.
    ///
    /// # Arguments
    ///
    /// - `object_id`: object where the error occurred
    /// - `code`: error code
    /// - `message`: error description
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn error(
        &mut self,
        _slf: &Rc<MetaWlDisplay>,
        object_id: Rc<dyn Proxy>,
        code: u32,
        message: &str,
    ) {
        if let Some(client_id) = _slf.core.client_id.get() {
            if let Some(client_id_2) = object_id.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = _slf.send_error(
            object_id,
            code,
            message,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_display.error message: {}", Report::new(e));
        }
    }

    /// acknowledge object ID deletion
    ///
    /// This event is used internally by the object ID management
    /// logic. When a client deletes an object that it had created,
    /// the server will send this event to acknowledge that it has
    /// seen the delete request. When the client receives this event,
    /// it will know that it can safely reuse the object ID.
    ///
    /// # Arguments
    ///
    /// - `id`: deleted object ID
    #[inline]
    fn delete_id(
        &mut self,
        _slf: &Rc<MetaWlDisplay>,
        id: u32,
    ) {
        let res = _slf.send_delete_id(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_display.delete_id message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWlDisplay {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaWlCallback::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).sync(&self, arg0);
                } else {
                    DefaultMessageHandler.sync(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaWlRegistry::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).get_registry(&self, arg0);
                } else {
                    DefaultMessageHandler.get_registry(&self, arg0);
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
            0 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError);
                };
                offset += 1;
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError);
                };
                offset += 1;
                let arg2 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError);
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError);
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError);
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError);
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError);
                }
                let Some(arg0) = self.core.state.lookup(arg0) else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).error(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.error(&self, arg0, arg1, arg2);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).delete_id(&self, arg0);
                } else {
                    DefaultMessageHandler.delete_id(&self, arg0);
                }
            }
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }
}

impl MetaWlDisplay {
    /// Since when the error.invalid_object enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_OBJECT__SINCE: u32 = 1;
    /// Since when the error.invalid_method enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_METHOD__SINCE: u32 = 1;
    /// Since when the error.no_memory enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_NO_MEMORY__SINCE: u32 = 1;
    /// Since when the error.implementation enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_IMPLEMENTATION__SINCE: u32 = 1;
}

/// global error values
///
/// These errors are global and can be emitted in response to any
/// server request.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWlDisplayError(pub u32);

impl MetaWlDisplayError {
    /// server couldn't find object
    #[allow(dead_code)]
    pub const INVALID_OBJECT: Self = Self(0);

    /// method doesn't exist on the specified interface or malformed request
    #[allow(dead_code)]
    pub const INVALID_METHOD: Self = Self(1);

    /// server is out of memory
    #[allow(dead_code)]
    pub const NO_MEMORY: Self = Self(2);

    /// implementation error in compositor
    #[allow(dead_code)]
    pub const IMPLEMENTATION: Self = Self(3);
}

impl Debug for MetaWlDisplayError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_OBJECT => "INVALID_OBJECT",
            Self::INVALID_METHOD => "INVALID_METHOD",
            Self::NO_MEMORY => "NO_MEMORY",
            Self::IMPLEMENTATION => "IMPLEMENTATION",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
