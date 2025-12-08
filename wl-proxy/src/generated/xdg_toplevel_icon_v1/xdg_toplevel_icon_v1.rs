//! a toplevel window icon
//!
//! This interface defines a toplevel icon.
//! An icon can have a name, and multiple buffers.
//! In order to be applied, the icon must have either a name, or at least
//! one buffer assigned. Applying an empty icon (with no buffer or name) to
//! a toplevel should reset its icon to the default icon.
//!
//! It is up to compositor policy whether to prefer using a buffer or loading
//! an icon via its name. See 'set_name' and 'add_buffer' for details.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A xdg_toplevel_icon_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaXdgToplevelIconV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaXdgToplevelIconV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaXdgToplevelIconV1MessageHandler for DefaultMessageHandler { }

impl MetaXdgToplevelIconV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaXdgToplevelIconV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaXdgToplevelIconV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaXdgToplevelIconV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the icon object
    ///
    /// Destroys the 'xdg_toplevel_icon_v1' object.
    /// The icon must still remain set on every toplevel it was assigned to,
    /// until the toplevel icon is reset explicitly.
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

    /// Since when the set_name message is available.
    #[allow(dead_code)]
    pub const MSG__SET_NAME__SINCE: u32 = 1;

    /// set an icon name
    ///
    /// This request assigns an icon name to this icon.
    /// Any previously set name is overridden.
    ///
    /// The compositor must resolve 'icon_name' according to the lookup rules
    /// described in the XDG icon theme specification[1] using the
    /// environment's current icon theme.
    ///
    /// If the compositor does not support icon names or cannot resolve
    /// 'icon_name' according to the XDG icon theme specification it must
    /// fall back to using pixel buffer data instead.
    ///
    /// If this request is made after the icon has been assigned to a toplevel
    /// via 'set_icon', a 'immutable' error must be raised.
    ///
    /// [1]: https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html
    ///
    /// # Arguments
    ///
    /// - `icon_name`:
    #[inline]
    pub fn send_set_name(
        &self,
        icon_name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            icon_name,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the add_buffer message is available.
    #[allow(dead_code)]
    pub const MSG__ADD_BUFFER__SINCE: u32 = 1;

    /// add icon data from a pixel buffer
    ///
    /// This request adds pixel data supplied as wl_buffer to the icon.
    ///
    /// The client should add pixel data for all icon sizes and scales that
    /// it can provide, or which are explicitly requested by the compositor
    /// via 'icon_size' events on xdg_toplevel_icon_manager_v1.
    ///
    /// The wl_buffer supplying pixel data as 'buffer' must be backed by wl_shm
    /// and must be a square (width and height being equal).
    /// If any of these buffer requirements are not fulfilled, a 'invalid_buffer'
    /// error must be raised.
    ///
    /// If this icon instance already has a buffer of the same size and scale
    /// from a previous 'add_buffer' request, data from the last request
    /// overrides the preexisting pixel data.
    ///
    /// The wl_buffer must be kept alive for as long as the xdg_toplevel_icon
    /// it is associated with is not destroyed, otherwise a 'no_buffer' error
    /// is raised. The buffer contents must not be modified after it was
    /// assigned to the icon. As a result, the region of the wl_shm_pool's
    /// backing storage used for the wl_buffer must not be modified after this
    /// request is sent. The wl_buffer.release event is unused.
    ///
    /// If this request is made after the icon has been assigned to a toplevel
    /// via 'set_icon', a 'immutable' error must be raised.
    ///
    /// # Arguments
    ///
    /// - `buffer`:
    /// - `scale`: the scaling factor of the icon, e.g. 1
    #[inline]
    pub fn send_add_buffer(
        &self,
        buffer: &Rc<MetaWlBuffer>,
        scale: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            buffer,
            scale,
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
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
            arg0,
            arg1 as u32,
        ]);
        Ok(())
    }
}

/// A message handler for [XdgToplevelIconV1] proxies.
#[allow(dead_code)]
pub trait MetaXdgToplevelIconV1MessageHandler {
    /// destroy the icon object
    ///
    /// Destroys the 'xdg_toplevel_icon_v1' object.
    /// The icon must still remain set on every toplevel it was assigned to,
    /// until the toplevel icon is reset explicitly.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaXdgToplevelIconV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel_icon_v1.destroy message: {}", Report::new(e));
        }
    }

    /// set an icon name
    ///
    /// This request assigns an icon name to this icon.
    /// Any previously set name is overridden.
    ///
    /// The compositor must resolve 'icon_name' according to the lookup rules
    /// described in the XDG icon theme specification[1] using the
    /// environment's current icon theme.
    ///
    /// If the compositor does not support icon names or cannot resolve
    /// 'icon_name' according to the XDG icon theme specification it must
    /// fall back to using pixel buffer data instead.
    ///
    /// If this request is made after the icon has been assigned to a toplevel
    /// via 'set_icon', a 'immutable' error must be raised.
    ///
    /// [1]: https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html
    ///
    /// # Arguments
    ///
    /// - `icon_name`:
    #[inline]
    fn set_name(
        &mut self,
        _slf: &Rc<MetaXdgToplevelIconV1>,
        icon_name: &str,
    ) {
        let res = _slf.send_set_name(
            icon_name,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel_icon_v1.set_name message: {}", Report::new(e));
        }
    }

    /// add icon data from a pixel buffer
    ///
    /// This request adds pixel data supplied as wl_buffer to the icon.
    ///
    /// The client should add pixel data for all icon sizes and scales that
    /// it can provide, or which are explicitly requested by the compositor
    /// via 'icon_size' events on xdg_toplevel_icon_manager_v1.
    ///
    /// The wl_buffer supplying pixel data as 'buffer' must be backed by wl_shm
    /// and must be a square (width and height being equal).
    /// If any of these buffer requirements are not fulfilled, a 'invalid_buffer'
    /// error must be raised.
    ///
    /// If this icon instance already has a buffer of the same size and scale
    /// from a previous 'add_buffer' request, data from the last request
    /// overrides the preexisting pixel data.
    ///
    /// The wl_buffer must be kept alive for as long as the xdg_toplevel_icon
    /// it is associated with is not destroyed, otherwise a 'no_buffer' error
    /// is raised. The buffer contents must not be modified after it was
    /// assigned to the icon. As a result, the region of the wl_shm_pool's
    /// backing storage used for the wl_buffer must not be modified after this
    /// request is sent. The wl_buffer.release event is unused.
    ///
    /// If this request is made after the icon has been assigned to a toplevel
    /// via 'set_icon', a 'immutable' error must be raised.
    ///
    /// # Arguments
    ///
    /// - `buffer`:
    /// - `scale`: the scaling factor of the icon, e.g. 1
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn add_buffer(
        &mut self,
        _slf: &Rc<MetaXdgToplevelIconV1>,
        buffer: &Rc<MetaWlBuffer>,
        scale: i32,
    ) {
        let res = _slf.send_add_buffer(
            buffer,
            scale,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_toplevel_icon_v1.add_buffer message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaXdgToplevelIconV1 {
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
                    (**handler).set_name(&self, arg0);
                } else {
                    DefaultMessageHandler.set_name(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let Some(arg0) = client.lookup(arg0) else {
                    return Err(ObjectError);
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlBuffer>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = arg1 as i32;
                if let Some(handler) = handler {
                    (**handler).add_buffer(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.add_buffer(&self, arg0, arg1);
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

impl MetaXdgToplevelIconV1 {
    /// Since when the error.invalid_buffer enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_BUFFER__SINCE: u32 = 1;
    /// Since when the error.immutable enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_IMMUTABLE__SINCE: u32 = 1;
    /// Since when the error.no_buffer enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_NO_BUFFER__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaXdgToplevelIconV1Error(pub u32);

impl MetaXdgToplevelIconV1Error {
    /// the provided buffer does not satisfy requirements
    #[allow(dead_code)]
    pub const INVALID_BUFFER: Self = Self(1);

    /// the icon has already been assigned to a toplevel and must not be changed
    #[allow(dead_code)]
    pub const IMMUTABLE: Self = Self(2);

    /// the provided buffer has been destroyed before the toplevel icon
    #[allow(dead_code)]
    pub const NO_BUFFER: Self = Self(3);
}

impl Debug for MetaXdgToplevelIconV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_BUFFER => "INVALID_BUFFER",
            Self::IMMUTABLE => "IMMUTABLE",
            Self::NO_BUFFER => "NO_BUFFER",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
