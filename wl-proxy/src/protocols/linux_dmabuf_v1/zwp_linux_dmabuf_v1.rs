//! factory for creating dmabuf-based wl_buffers
//!
//! This interface offers ways to create generic dmabuf-based wl_buffers.
//!
//! For more information about dmabuf, see:
//! https://www.kernel.org/doc/html/next/userspace-api/dma-buf-alloc-exchange.html
//!
//! Clients can use the get_surface_feedback request to get dmabuf feedback
//! for a particular surface. If the client wants to retrieve feedback not
//! tied to a surface, they can use the get_default_feedback request.
//!
//! The following are required from clients:
//!
//! - Clients must ensure that either all data in the dma-buf is
//!   coherent for all subsequent read access or that coherency is
//!   correctly handled by the underlying kernel-side dma-buf
//!   implementation.
//!
//! - Don't make any more attachments after sending the buffer to the
//!   compositor. Making more attachments later increases the risk of
//!   the compositor not being able to use (re-import) an existing
//!   dmabuf-based wl_buffer.
//!
//! The underlying graphics stack must ensure the following:
//!
//! - The dmabuf file descriptors relayed to the server will stay valid
//!   for the whole lifetime of the wl_buffer. This means the server may
//!   at any time use those fds to import the dmabuf into any kernel
//!   sub-system that might accept it.
//!
//! However, when the underlying graphics stack fails to deliver the
//! promise, because of e.g. a device hot-unplug which raises internal
//! errors, after the wl_buffer has been successfully created the
//! compositor must not raise protocol errors to the client when dmabuf
//! import later fails.
//!
//! To create a wl_buffer from one or more dmabufs, a client creates a
//! zwp_linux_dmabuf_params_v1 object with a zwp_linux_dmabuf_v1.create_params
//! request. All planes required by the intended format are added with
//! the 'add' request. Finally, a 'create' or 'create_immed' request is
//! issued, which has the following outcome depending on the import success.
//!
//! The 'create' request,
//! - on success, triggers a 'created' event which provides the final
//!   wl_buffer to the client.
//! - on failure, triggers a 'failed' event to convey that the server
//!   cannot use the dmabufs received from the client.
//!
//! For the 'create_immed' request,
//! - on success, the server immediately imports the added dmabufs to
//!   create a wl_buffer. No event is sent from the server in this case.
//! - on failure, the server can choose to either:
//!   - terminate the client by raising a fatal error.
//!   - mark the wl_buffer as failed, and send a 'failed' event to the
//!     client. If the client uses a failed wl_buffer as an argument to any
//!     request, the behaviour is compositor implementation-defined.
//!
//! For all DRM formats and unless specified in another protocol extension,
//! pre-multiplied alpha is used for pixel values.
//!
//! Unless specified otherwise in another protocol extension, implicit
//! synchronization is used. In other words, compositors and clients must
//! wait and signal fences implicitly passed via the DMA-BUF's reservation
//! mechanism.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_linux_dmabuf_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpLinuxDmabufV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn ZwpLinuxDmabufV1Handler>,
}

struct DefaultHandler;

impl ZwpLinuxDmabufV1Handler for DefaultHandler { }

impl ZwpLinuxDmabufV1 {
    pub const XML_VERSION: u32 = 5;
    pub const INTERFACE: &str = "zwp_linux_dmabuf_v1";
}

impl ZwpLinuxDmabufV1 {
    pub fn set_handler(&self, handler: impl ZwpLinuxDmabufV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpLinuxDmabufV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpLinuxDmabufV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpLinuxDmabufV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpLinuxDmabufV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// unbind the factory
    ///
    /// Objects created through this interface, especially wl_buffers, will
    /// remain valid.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_linux_dmabuf_v1#{}.destroy()\n", id);
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

    /// Since when the create_params message is available.
    pub const MSG__CREATE_PARAMS__SINCE: u32 = 1;

    /// create a temporary object for buffer parameters
    ///
    /// This temporary object is used to collect multiple dmabuf handles into
    /// a single batch to create a wl_buffer. It can only be used once and
    /// should be destroyed after a 'created' or 'failed' event has been
    /// received.
    #[inline]
    pub fn send_create_params(
        &self,
        params_id: &Rc<ZwpLinuxBufferParamsV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            params_id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("params_id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_linux_dmabuf_v1#{}.create_params(params_id: zwp_linux_buffer_params_v1#{})\n", id, arg0_id);
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
        Ok(())
    }

    /// Since when the format message is available.
    pub const MSG__FORMAT__SINCE: u32 = 1;

    /// Since when the format message is deprecated.
    pub const MSG__FORMAT__DEPRECATED_SINCE: u32 = 4;

    /// supported buffer format
    ///
    /// This event advertises one buffer format that the server supports.
    /// All the supported formats are advertised once when the client
    /// binds to this interface. A roundtrip after binding guarantees
    /// that the client has received all supported formats.
    ///
    /// For the definition of the format codes, see the
    /// zwp_linux_buffer_params_v1::create request.
    ///
    /// Starting version 4, the format event is deprecated and must not be
    /// sent by compositors. Instead, use get_default_feedback or
    /// get_surface_feedback.
    ///
    /// # Arguments
    ///
    /// - `format`: DRM_FORMAT code
    #[inline]
    pub fn send_format(
        &self,
        format: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            format,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_linux_dmabuf_v1#{}.format(format: {})\n", client.endpoint.id, id, arg0);
            self.core.state.log(args);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the modifier message is available.
    pub const MSG__MODIFIER__SINCE: u32 = 3;

    /// Since when the modifier message is deprecated.
    pub const MSG__MODIFIER__DEPRECATED_SINCE: u32 = 4;

    /// supported buffer format modifier
    ///
    /// This event advertises the formats that the server supports, along with
    /// the modifiers supported for each format. All the supported modifiers
    /// for all the supported formats are advertised once when the client
    /// binds to this interface. A roundtrip after binding guarantees that
    /// the client has received all supported format-modifier pairs.
    ///
    /// For legacy support, DRM_FORMAT_MOD_INVALID (that is, modifier_hi ==
    /// 0x00ffffff and modifier_lo == 0xffffffff) is allowed in this event.
    /// It indicates that the server can support the format with an implicit
    /// modifier. When a plane has DRM_FORMAT_MOD_INVALID as its modifier, it
    /// is as if no explicit modifier is specified. The effective modifier
    /// will be derived from the dmabuf.
    ///
    /// A compositor that sends valid modifiers and DRM_FORMAT_MOD_INVALID for
    /// a given format supports both explicit modifiers and implicit modifiers.
    ///
    /// For the definition of the format and modifier codes, see the
    /// zwp_linux_buffer_params_v1::create and zwp_linux_buffer_params_v1::add
    /// requests.
    ///
    /// Starting version 4, the modifier event is deprecated and must not be
    /// sent by compositors. Instead, use get_default_feedback or
    /// get_surface_feedback.
    ///
    /// # Arguments
    ///
    /// - `format`: DRM_FORMAT code
    /// - `modifier_hi`: high 32 bits of layout modifier
    /// - `modifier_lo`: low 32 bits of layout modifier
    #[inline]
    pub fn send_modifier(
        &self,
        format: u32,
        modifier_hi: u32,
        modifier_lo: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            format,
            modifier_hi,
            modifier_lo,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_linux_dmabuf_v1#{}.modifier(format: {}, modifier_hi: {}, modifier_lo: {})\n", client.endpoint.id, id, arg0, arg1, arg2);
            self.core.state.log(args);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0,
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// Since when the get_default_feedback message is available.
    pub const MSG__GET_DEFAULT_FEEDBACK__SINCE: u32 = 4;

    /// get default feedback
    ///
    /// This request creates a new wp_linux_dmabuf_feedback object not bound
    /// to a particular surface. This object will deliver feedback about dmabuf
    /// parameters to use if the client doesn't support per-surface feedback
    /// (see get_surface_feedback).
    #[inline]
    pub fn send_get_default_feedback(
        &self,
        id: &Rc<ZwpLinuxDmabufFeedbackV1>,
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_linux_dmabuf_v1#{}.get_default_feedback(id: zwp_linux_dmabuf_feedback_v1#{})\n", id, arg0_id);
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
            2,
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the get_surface_feedback message is available.
    pub const MSG__GET_SURFACE_FEEDBACK__SINCE: u32 = 4;

    /// get feedback for a surface
    ///
    /// This request creates a new wp_linux_dmabuf_feedback object for the
    /// specified wl_surface. This object will deliver feedback about dmabuf
    /// parameters to use for buffers attached to this surface.
    ///
    /// If the surface is destroyed before the wp_linux_dmabuf_feedback object,
    /// the feedback object becomes inert.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn send_get_surface_feedback(
        &self,
        id: &Rc<ZwpLinuxDmabufFeedbackV1>,
        surface: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            surface,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("surface")),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_linux_dmabuf_v1#{}.get_surface_feedback(id: zwp_linux_dmabuf_feedback_v1#{}, surface: wl_surface#{})\n", id, arg0_id, arg1_id);
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
            3,
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpLinuxDmabufV1] proxies.
pub trait ZwpLinuxDmabufV1Handler: Any {
    /// unbind the factory
    ///
    /// Objects created through this interface, especially wl_buffers, will
    /// remain valid.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<ZwpLinuxDmabufV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_linux_dmabuf_v1.destroy message: {}", Report::new(e));
        }
    }

    /// create a temporary object for buffer parameters
    ///
    /// This temporary object is used to collect multiple dmabuf handles into
    /// a single batch to create a wl_buffer. It can only be used once and
    /// should be destroyed after a 'created' or 'failed' event has been
    /// received.
    ///
    /// # Arguments
    ///
    /// - `params_id`: the new temporary
    #[inline]
    fn create_params(
        &mut self,
        _slf: &Rc<ZwpLinuxDmabufV1>,
        params_id: &Rc<ZwpLinuxBufferParamsV1>,
    ) {
        let res = _slf.send_create_params(
            params_id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_linux_dmabuf_v1.create_params message: {}", Report::new(e));
        }
    }

    /// supported buffer format
    ///
    /// This event advertises one buffer format that the server supports.
    /// All the supported formats are advertised once when the client
    /// binds to this interface. A roundtrip after binding guarantees
    /// that the client has received all supported formats.
    ///
    /// For the definition of the format codes, see the
    /// zwp_linux_buffer_params_v1::create request.
    ///
    /// Starting version 4, the format event is deprecated and must not be
    /// sent by compositors. Instead, use get_default_feedback or
    /// get_surface_feedback.
    ///
    /// # Arguments
    ///
    /// - `format`: DRM_FORMAT code
    #[inline]
    fn format(
        &mut self,
        _slf: &Rc<ZwpLinuxDmabufV1>,
        format: u32,
    ) {
        let res = _slf.send_format(
            format,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_linux_dmabuf_v1.format message: {}", Report::new(e));
        }
    }

    /// supported buffer format modifier
    ///
    /// This event advertises the formats that the server supports, along with
    /// the modifiers supported for each format. All the supported modifiers
    /// for all the supported formats are advertised once when the client
    /// binds to this interface. A roundtrip after binding guarantees that
    /// the client has received all supported format-modifier pairs.
    ///
    /// For legacy support, DRM_FORMAT_MOD_INVALID (that is, modifier_hi ==
    /// 0x00ffffff and modifier_lo == 0xffffffff) is allowed in this event.
    /// It indicates that the server can support the format with an implicit
    /// modifier. When a plane has DRM_FORMAT_MOD_INVALID as its modifier, it
    /// is as if no explicit modifier is specified. The effective modifier
    /// will be derived from the dmabuf.
    ///
    /// A compositor that sends valid modifiers and DRM_FORMAT_MOD_INVALID for
    /// a given format supports both explicit modifiers and implicit modifiers.
    ///
    /// For the definition of the format and modifier codes, see the
    /// zwp_linux_buffer_params_v1::create and zwp_linux_buffer_params_v1::add
    /// requests.
    ///
    /// Starting version 4, the modifier event is deprecated and must not be
    /// sent by compositors. Instead, use get_default_feedback or
    /// get_surface_feedback.
    ///
    /// # Arguments
    ///
    /// - `format`: DRM_FORMAT code
    /// - `modifier_hi`: high 32 bits of layout modifier
    /// - `modifier_lo`: low 32 bits of layout modifier
    #[inline]
    fn modifier(
        &mut self,
        _slf: &Rc<ZwpLinuxDmabufV1>,
        format: u32,
        modifier_hi: u32,
        modifier_lo: u32,
    ) {
        let res = _slf.send_modifier(
            format,
            modifier_hi,
            modifier_lo,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_linux_dmabuf_v1.modifier message: {}", Report::new(e));
        }
    }

    /// get default feedback
    ///
    /// This request creates a new wp_linux_dmabuf_feedback object not bound
    /// to a particular surface. This object will deliver feedback about dmabuf
    /// parameters to use if the client doesn't support per-surface feedback
    /// (see get_surface_feedback).
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn get_default_feedback(
        &mut self,
        _slf: &Rc<ZwpLinuxDmabufV1>,
        id: &Rc<ZwpLinuxDmabufFeedbackV1>,
    ) {
        let res = _slf.send_get_default_feedback(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_linux_dmabuf_v1.get_default_feedback message: {}", Report::new(e));
        }
    }

    /// get feedback for a surface
    ///
    /// This request creates a new wp_linux_dmabuf_feedback object for the
    /// specified wl_surface. This object will deliver feedback about dmabuf
    /// parameters to use for buffers attached to this surface.
    ///
    /// If the surface is destroyed before the wp_linux_dmabuf_feedback object,
    /// the feedback object becomes inert.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_surface_feedback(
        &mut self,
        _slf: &Rc<ZwpLinuxDmabufV1>,
        id: &Rc<ZwpLinuxDmabufFeedbackV1>,
        surface: &Rc<WlSurface>,
    ) {
        let res = _slf.send_get_surface_feedback(
            id,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_linux_dmabuf_v1.get_surface_feedback message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for ZwpLinuxDmabufV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::ZwpLinuxDmabufV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_linux_dmabuf_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_linux_dmabuf_v1#{}.create_params(params_id: zwp_linux_buffer_params_v1#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ZwpLinuxBufferParamsV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "params_id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).create_params(&self, arg0);
                } else {
                    DefaultHandler.create_params(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_linux_dmabuf_v1#{}.get_default_feedback(id: zwp_linux_dmabuf_feedback_v1#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ZwpLinuxDmabufFeedbackV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).get_default_feedback(&self, arg0);
                } else {
                    DefaultHandler.get_default_feedback(&self, arg0);
                }
            }
            3 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_linux_dmabuf_v1#{}.get_surface_feedback(id: zwp_linux_dmabuf_feedback_v1#{}, surface: wl_surface#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ZwpLinuxDmabufFeedbackV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ProxyInterface::WlSurface));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_surface_feedback(&self, arg0, arg1);
                } else {
                    DefaultHandler.get_surface_feedback(&self, arg0, arg1);
                }
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
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_linux_dmabuf_v1#{}.format(format: {})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).format(&self, arg0);
                } else {
                    DefaultHandler.format(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_linux_dmabuf_v1#{}.modifier(format: {}, modifier_hi: {}, modifier_lo: {})\n", msg[0], arg0, arg1, arg2);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).modifier(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.modifier(&self, arg0, arg1, arg2);
                }
            }
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
        Ok(())
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            1 => "create_params",
            2 => "get_default_feedback",
            3 => "get_surface_feedback",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "format",
            1 => "modifier",
            _ => return None,
        };
        Some(name)
    }
}

impl Proxy for ZwpLinuxDmabufV1 {
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

