//! Object representing a toplevel move during a drag

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A xdg_toplevel_drag_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaXdgToplevelDragV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaXdgToplevelDragV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaXdgToplevelDragV1MessageHandler for DefaultMessageHandler { }

impl MetaXdgToplevelDragV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaXdgToplevelDragV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::XdgToplevelDragV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaXdgToplevelDragV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaXdgToplevelDragV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaXdgToplevelDragV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy an xdg_toplevel_drag_v1 object
    ///
    /// Destroy this xdg_toplevel_drag_v1 object. This request must only be
    /// called after the underlying wl_data_source drag has ended, as indicated
    /// by the dnd_drop_performed or cancelled events. In any other case an
    /// ongoing_drag error is raised.
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

    /// Since when the attach message is available.
    #[allow(dead_code)]
    pub const MSG__ATTACH__SINCE: u32 = 1;

    /// Move a toplevel with the drag operation
    ///
    /// Request that the window will be moved with the cursor during the drag
    /// operation. The offset is a hint to the compositor how the toplevel
    /// should be positioned relative to the cursor hotspot in surface local
    /// coordinates and relative to the geometry of the toplevel being attached.
    /// See xdg_surface.set_window_geometry. For example it might only
    /// be used when an unmapped window is attached. The attached window
    /// does not participate in the selection of the drag target.
    ///
    /// If the toplevel is unmapped while it is attached, it is automatically
    /// detached from the drag. In this case this request has to be called again
    /// if the window should be attached after it is remapped.
    ///
    /// This request can be called multiple times but issuing it while a
    /// toplevel with an active role is attached raises a toplevel_attached
    /// error.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    /// - `x_offset`: dragged surface x offset
    /// - `y_offset`: dragged surface y offset
    #[inline]
    pub fn send_attach(
        &self,
        toplevel: &Rc<MetaXdgToplevel>,
        x_offset: i32,
        y_offset: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            toplevel,
            x_offset,
            y_offset,
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
            arg1 as u32,
            arg2 as u32,
        ]);
        Ok(())
    }
}

/// A message handler for [XdgToplevelDragV1] proxies.
#[allow(dead_code)]
pub trait MetaXdgToplevelDragV1MessageHandler {
    /// destroy an xdg_toplevel_drag_v1 object
    ///
    /// Destroy this xdg_toplevel_drag_v1 object. This request must only be
    /// called after the underlying wl_data_source drag has ended, as indicated
    /// by the dnd_drop_performed or cancelled events. In any other case an
    /// ongoing_drag error is raised.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaXdgToplevelDragV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel_drag_v1.destroy message: {}", Report::new(e));
        }
    }

    /// Move a toplevel with the drag operation
    ///
    /// Request that the window will be moved with the cursor during the drag
    /// operation. The offset is a hint to the compositor how the toplevel
    /// should be positioned relative to the cursor hotspot in surface local
    /// coordinates and relative to the geometry of the toplevel being attached.
    /// See xdg_surface.set_window_geometry. For example it might only
    /// be used when an unmapped window is attached. The attached window
    /// does not participate in the selection of the drag target.
    ///
    /// If the toplevel is unmapped while it is attached, it is automatically
    /// detached from the drag. In this case this request has to be called again
    /// if the window should be attached after it is remapped.
    ///
    /// This request can be called multiple times but issuing it while a
    /// toplevel with an active role is attached raises a toplevel_attached
    /// error.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    /// - `x_offset`: dragged surface x offset
    /// - `y_offset`: dragged surface y offset
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn attach(
        &mut self,
        _slf: &Rc<MetaXdgToplevelDragV1>,
        toplevel: &Rc<MetaXdgToplevel>,
        x_offset: i32,
        y_offset: i32,
    ) {
        let res = _slf.send_attach(
            toplevel,
            x_offset,
            y_offset,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel_drag_v1.attach message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaXdgToplevelDragV1 {
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
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let Some(arg0) = client.endpoint.lookup(arg0) else {
                    return Err(ObjectError);
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaXdgToplevel>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                if let Some(handler) = handler {
                    (**handler).attach(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.attach(&self, arg0, arg1, arg2);
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

impl MetaXdgToplevelDragV1 {
    /// Since when the error.toplevel_attached enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_TOPLEVEL_ATTACHED__SINCE: u32 = 1;
    /// Since when the error.ongoing_drag enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ONGOING_DRAG__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaXdgToplevelDragV1Error(pub u32);

impl MetaXdgToplevelDragV1Error {
    /// valid toplevel already attached
    #[allow(dead_code)]
    pub const TOPLEVEL_ATTACHED: Self = Self(0);

    /// drag has not ended
    #[allow(dead_code)]
    pub const ONGOING_DRAG: Self = Self(1);
}

impl Debug for MetaXdgToplevelDragV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::TOPLEVEL_ATTACHED => "TOPLEVEL_ATTACHED",
            Self::ONGOING_DRAG => "ONGOING_DRAG",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
