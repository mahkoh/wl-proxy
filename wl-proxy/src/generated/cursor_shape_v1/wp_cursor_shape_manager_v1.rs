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

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_cursor_shape_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpCursorShapeManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpCursorShapeManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpCursorShapeManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaWpCursorShapeManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpCursorShapeManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpCursorShapeManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpCursorShapeManagerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the manager
    ///
    /// Destroy the cursor shape manager.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
        ]);
        Ok(())
    }

    /// Since when the get_pointer message is available.
    #[allow(dead_code)]
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
    pub fn send_get_pointer(
        &self,
        cursor_shape_device: &Rc<MetaWpCursorShapeDeviceV1>,
        pointer: &Rc<MetaWlPointer>,
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
            return Err(ObjectError);
        };
        let arg1 = match arg1.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
        ]);
        Ok(())
    }

    /// Since when the get_tablet_tool_v2 message is available.
    #[allow(dead_code)]
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
    pub fn send_get_tablet_tool_v2(
        &self,
        cursor_shape_device: &Rc<MetaWpCursorShapeDeviceV1>,
        tablet_tool: &Rc<MetaZwpTabletToolV2>,
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
            return Err(ObjectError);
        };
        let arg1 = match arg1.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
        ]);
        Ok(())
    }
}

/// A message handler for [WpCursorShapeManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaWpCursorShapeManagerV1MessageHandler {
    /// destroy the manager
    ///
    /// Destroy the cursor shape manager.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpCursorShapeManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_cursor_shape_manager_v1.destroy message: {}", Report::new(e));
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
    fn get_pointer(
        &mut self,
        _slf: &Rc<MetaWpCursorShapeManagerV1>,
        cursor_shape_device: &Rc<MetaWpCursorShapeDeviceV1>,
        pointer: &Rc<MetaWlPointer>,
    ) {
        let res = _slf.send_get_pointer(
            cursor_shape_device,
            pointer,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_cursor_shape_manager_v1.get_pointer message: {}", Report::new(e));
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
    fn get_tablet_tool_v2(
        &mut self,
        _slf: &Rc<MetaWpCursorShapeManagerV1>,
        cursor_shape_device: &Rc<MetaWpCursorShapeDeviceV1>,
        tablet_tool: &Rc<MetaZwpTabletToolV2>,
    ) {
        let res = _slf.send_get_tablet_tool_v2(
            cursor_shape_device,
            tablet_tool,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_cursor_shape_manager_v1.get_tablet_tool_v2 message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpCursorShapeManagerV1 {
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
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaWpCursorShapeDeviceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlPointer>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_pointer(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_pointer(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaWpCursorShapeDeviceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaZwpTabletToolV2>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_tablet_tool_v2(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_tablet_tool_v2(&self, arg0, arg1);
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

