//! compositor logical output region
//!
//! An xdg_output describes part of the compositor geometry.
//!
//! This typically corresponds to a monitor that displays part of the
//! compositor space.
//!
//! For objects version 3 onwards, after all xdg_output properties have been
//! sent (when the object is created and when properties are updated), a
//! wl_output.done event is sent. This allows changes to the output
//! properties to be seen as atomic, even if they happen via multiple events.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zxdg_output_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZxdgOutputV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZxdgOutputV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZxdgOutputV1MessageHandler for DefaultMessageHandler { }

impl MetaZxdgOutputV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZxdgOutputV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZxdgOutputV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZxdgOutputV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the xdg_output object
    ///
    /// Using this request a client can tell the server that it is not
    /// going to use the xdg_output object anymore.
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

    /// Since when the logical_position message is available.
    #[allow(dead_code)]
    pub const MSG__LOGICAL_POSITION__SINCE: u32 = 1;

    /// position of the output within the global compositor space
    ///
    /// The position event describes the location of the wl_output within
    /// the global compositor space.
    ///
    /// The logical_position event is sent after creating an xdg_output
    /// (see xdg_output_manager.get_xdg_output) and whenever the location
    /// of the output changes within the global compositor space.
    ///
    /// # Arguments
    ///
    /// - `x`: x position within the global compositor space
    /// - `y`: y position within the global compositor space
    #[inline]
    pub fn send_logical_position(
        &self,
        x: i32,
        y: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            x,
            y,
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
            0,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// Since when the logical_size message is available.
    #[allow(dead_code)]
    pub const MSG__LOGICAL_SIZE__SINCE: u32 = 1;

    /// size of the output in the global compositor space
    ///
    /// The logical_size event describes the size of the output in the
    /// global compositor space.
    ///
    /// Most regular Wayland clients should not pay attention to the
    /// logical size and would rather rely on xdg_shell interfaces.
    ///
    /// Some clients such as Xwayland, however, need this to configure
    /// their surfaces in the global compositor space as the compositor
    /// may apply a different scale from what is advertised by the output
    /// scaling property (to achieve fractional scaling, for example).
    ///
    /// For example, for a wl_output mode 3840×2160 and a scale factor 2:
    ///
    /// - A compositor not scaling the monitor viewport in its compositing space
    ///   will advertise a logical size of 3840×2160,
    ///
    /// - A compositor scaling the monitor viewport with scale factor 2 will
    ///   advertise a logical size of 1920×1080,
    ///
    /// - A compositor scaling the monitor viewport using a fractional scale of
    ///   1.5 will advertise a logical size of 2560×1440.
    ///
    /// For example, for a wl_output mode 1920×1080 and a 90 degree rotation,
    /// the compositor will advertise a logical size of 1080x1920.
    ///
    /// The logical_size event is sent after creating an xdg_output
    /// (see xdg_output_manager.get_xdg_output) and whenever the logical
    /// size of the output changes, either as a result of a change in the
    /// applied scale or because of a change in the corresponding output
    /// mode(see wl_output.mode) or transform (see wl_output.transform).
    ///
    /// # Arguments
    ///
    /// - `width`: width in global compositor space
    /// - `height`: height in global compositor space
    #[inline]
    pub fn send_logical_size(
        &self,
        width: i32,
        height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            width,
            height,
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
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// Since when the done message is available.
    #[allow(dead_code)]
    pub const MSG__DONE__SINCE: u32 = 1;

    /// Since when the done message is deprecated.
    #[allow(dead_code)]
    pub const MSG__DONE__DEPRECATED_SINCE: u32 = 3;

    /// all information about the output have been sent
    ///
    /// This event is sent after all other properties of an xdg_output
    /// have been sent.
    ///
    /// This allows changes to the xdg_output properties to be seen as
    /// atomic, even if they happen via multiple events.
    ///
    /// For objects version 3 onwards, this event is deprecated. Compositors
    /// are not required to send it anymore and must send wl_output.done
    /// instead.
    #[inline]
    pub fn send_done(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            2,
        ]);
        Ok(())
    }

    /// Since when the name message is available.
    #[allow(dead_code)]
    pub const MSG__NAME__SINCE: u32 = 2;

    /// name of this output
    ///
    /// Many compositors will assign names to their outputs, show them to the
    /// user, allow them to be configured by name, etc. The client may wish to
    /// know this name as well to offer the user similar behaviors.
    ///
    /// The naming convention is compositor defined, but limited to
    /// alphanumeric characters and dashes (-). Each name is unique among all
    /// wl_output globals, but if a wl_output global is destroyed the same name
    /// may be reused later. The names will also remain consistent across
    /// sessions with the same hardware and software configuration.
    ///
    /// Examples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc. However, do
    /// not assume that the name is a reflection of an underlying DRM
    /// connector, X11 connection, etc.
    ///
    /// The name event is sent after creating an xdg_output (see
    /// xdg_output_manager.get_xdg_output). This event is only sent once per
    /// xdg_output, and the name does not change over the lifetime of the
    /// wl_output global.
    ///
    ///         This event is deprecated, instead clients should use wl_output.name.
    ///         Compositors must still support this event.
    ///
    /// # Arguments
    ///
    /// - `name`: output name
    #[inline]
    pub fn send_name(
        &self,
        name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            name,
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
            3,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the description message is available.
    #[allow(dead_code)]
    pub const MSG__DESCRIPTION__SINCE: u32 = 2;

    /// human-readable description of this output
    ///
    /// Many compositors can produce human-readable descriptions of their
    /// outputs.  The client may wish to know this description as well, to
    /// communicate the user for various purposes.
    ///
    /// The description is a UTF-8 string with no convention defined for its
    /// contents. Examples might include 'Foocorp 11" Display' or 'Virtual X11
    /// output via :1'.
    ///
    /// The description event is sent after creating an xdg_output (see
    /// xdg_output_manager.get_xdg_output) and whenever the description
    /// changes. The description is optional, and may not be sent at all.
    ///
    /// For objects of version 2 and lower, this event is only sent once per
    /// xdg_output, and the description does not change over the lifetime of
    /// the wl_output global.
    ///
    /// This event is deprecated, instead clients should use
    /// wl_output.description. Compositors must still support this event.
    ///
    /// # Arguments
    ///
    /// - `description`: output description
    #[inline]
    pub fn send_description(
        &self,
        description: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            description,
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
            4,
        ]);
        fmt.string(arg0);
        Ok(())
    }
}

/// A message handler for [ZxdgOutputV1] proxies.
#[allow(dead_code)]
pub trait MetaZxdgOutputV1MessageHandler {
    /// destroy the xdg_output object
    ///
    /// Using this request a client can tell the server that it is not
    /// going to use the xdg_output object anymore.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZxdgOutputV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_output_v1.destroy message: {}", Report::new(e));
        }
    }

    /// position of the output within the global compositor space
    ///
    /// The position event describes the location of the wl_output within
    /// the global compositor space.
    ///
    /// The logical_position event is sent after creating an xdg_output
    /// (see xdg_output_manager.get_xdg_output) and whenever the location
    /// of the output changes within the global compositor space.
    ///
    /// # Arguments
    ///
    /// - `x`: x position within the global compositor space
    /// - `y`: y position within the global compositor space
    #[inline]
    fn logical_position(
        &mut self,
        _slf: &Rc<MetaZxdgOutputV1>,
        x: i32,
        y: i32,
    ) {
        let res = _slf.send_logical_position(
            x,
            y,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_output_v1.logical_position message: {}", Report::new(e));
        }
    }

    /// size of the output in the global compositor space
    ///
    /// The logical_size event describes the size of the output in the
    /// global compositor space.
    ///
    /// Most regular Wayland clients should not pay attention to the
    /// logical size and would rather rely on xdg_shell interfaces.
    ///
    /// Some clients such as Xwayland, however, need this to configure
    /// their surfaces in the global compositor space as the compositor
    /// may apply a different scale from what is advertised by the output
    /// scaling property (to achieve fractional scaling, for example).
    ///
    /// For example, for a wl_output mode 3840×2160 and a scale factor 2:
    ///
    /// - A compositor not scaling the monitor viewport in its compositing space
    ///   will advertise a logical size of 3840×2160,
    ///
    /// - A compositor scaling the monitor viewport with scale factor 2 will
    ///   advertise a logical size of 1920×1080,
    ///
    /// - A compositor scaling the monitor viewport using a fractional scale of
    ///   1.5 will advertise a logical size of 2560×1440.
    ///
    /// For example, for a wl_output mode 1920×1080 and a 90 degree rotation,
    /// the compositor will advertise a logical size of 1080x1920.
    ///
    /// The logical_size event is sent after creating an xdg_output
    /// (see xdg_output_manager.get_xdg_output) and whenever the logical
    /// size of the output changes, either as a result of a change in the
    /// applied scale or because of a change in the corresponding output
    /// mode(see wl_output.mode) or transform (see wl_output.transform).
    ///
    /// # Arguments
    ///
    /// - `width`: width in global compositor space
    /// - `height`: height in global compositor space
    #[inline]
    fn logical_size(
        &mut self,
        _slf: &Rc<MetaZxdgOutputV1>,
        width: i32,
        height: i32,
    ) {
        let res = _slf.send_logical_size(
            width,
            height,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_output_v1.logical_size message: {}", Report::new(e));
        }
    }

    /// all information about the output have been sent
    ///
    /// This event is sent after all other properties of an xdg_output
    /// have been sent.
    ///
    /// This allows changes to the xdg_output properties to be seen as
    /// atomic, even if they happen via multiple events.
    ///
    /// For objects version 3 onwards, this event is deprecated. Compositors
    /// are not required to send it anymore and must send wl_output.done
    /// instead.
    #[inline]
    fn done(
        &mut self,
        _slf: &Rc<MetaZxdgOutputV1>,
    ) {
        let res = _slf.send_done(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_output_v1.done message: {}", Report::new(e));
        }
    }

    /// name of this output
    ///
    /// Many compositors will assign names to their outputs, show them to the
    /// user, allow them to be configured by name, etc. The client may wish to
    /// know this name as well to offer the user similar behaviors.
    ///
    /// The naming convention is compositor defined, but limited to
    /// alphanumeric characters and dashes (-). Each name is unique among all
    /// wl_output globals, but if a wl_output global is destroyed the same name
    /// may be reused later. The names will also remain consistent across
    /// sessions with the same hardware and software configuration.
    ///
    /// Examples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc. However, do
    /// not assume that the name is a reflection of an underlying DRM
    /// connector, X11 connection, etc.
    ///
    /// The name event is sent after creating an xdg_output (see
    /// xdg_output_manager.get_xdg_output). This event is only sent once per
    /// xdg_output, and the name does not change over the lifetime of the
    /// wl_output global.
    ///
    ///         This event is deprecated, instead clients should use wl_output.name.
    ///         Compositors must still support this event.
    ///
    /// # Arguments
    ///
    /// - `name`: output name
    #[inline]
    fn name(
        &mut self,
        _slf: &Rc<MetaZxdgOutputV1>,
        name: &str,
    ) {
        let res = _slf.send_name(
            name,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_output_v1.name message: {}", Report::new(e));
        }
    }

    /// human-readable description of this output
    ///
    /// Many compositors can produce human-readable descriptions of their
    /// outputs.  The client may wish to know this description as well, to
    /// communicate the user for various purposes.
    ///
    /// The description is a UTF-8 string with no convention defined for its
    /// contents. Examples might include 'Foocorp 11" Display' or 'Virtual X11
    /// output via :1'.
    ///
    /// The description event is sent after creating an xdg_output (see
    /// xdg_output_manager.get_xdg_output) and whenever the description
    /// changes. The description is optional, and may not be sent at all.
    ///
    /// For objects of version 2 and lower, this event is only sent once per
    /// xdg_output, and the description does not change over the lifetime of
    /// the wl_output global.
    ///
    /// This event is deprecated, instead clients should use
    /// wl_output.description. Compositors must still support this event.
    ///
    /// # Arguments
    ///
    /// - `description`: output description
    #[inline]
    fn description(
        &mut self,
        _slf: &Rc<MetaZxdgOutputV1>,
        description: &str,
    ) {
        let res = _slf.send_description(
            description,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_output_v1.description message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZxdgOutputV1 {
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
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                if let Some(handler) = handler {
                    (**handler).logical_position(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.logical_position(&self, arg0, arg1);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                if let Some(handler) = handler {
                    (**handler).logical_size(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.logical_size(&self, arg0, arg1);
                }
            }
            2 => {
                if let Some(handler) = handler {
                    (**handler).done(&self);
                } else {
                    DefaultMessageHandler.done(&self);
                }
            }
            3 => {
                let mut offset = 2;
                let arg0 = {
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
                if let Some(handler) = handler {
                    (**handler).name(&self, arg0);
                } else {
                    DefaultMessageHandler.name(&self, arg0);
                }
            }
            4 => {
                let mut offset = 2;
                let arg0 = {
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
                if let Some(handler) = handler {
                    (**handler).description(&self, arg0);
                } else {
                    DefaultMessageHandler.description(&self, arg0);
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

