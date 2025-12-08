use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wl_drm object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlDrm {
    core: ObjectCore,
    handler: HandlerHolder<dyn WlDrmHandler>,
}

struct DefaultHandler;

impl WlDrmHandler for DefaultHandler { }

impl ConcreteObject for WlDrm {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::WlDrm;
    const INTERFACE_NAME: &str = "wl_drm";
}

impl WlDrm {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WlDrmHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WlDrmHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlDrm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlDrm")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlDrm {
    /// Since when the authenticate message is available.
    pub const MSG__AUTHENTICATE__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_authenticate(
        &self,
        id: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_drm#{}.authenticate(id: {})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            arg0,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_authenticate(
        &self,
        id: u32,
    ) {
        let res = self.try_send_authenticate(
            id,
        );
        if let Err(e) = res {
            log_send("wl_drm.authenticate", &e);
        }
    }

    /// Since when the create_buffer message is available.
    pub const MSG__CREATE_BUFFER__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `id`:
    /// - `name`:
    /// - `width`:
    /// - `height`:
    /// - `stride`:
    /// - `format`:
    #[inline]
    pub fn try_send_create_buffer(
        &self,
        id: &Rc<WlBuffer>,
        name: u32,
        width: i32,
        height: i32,
        stride: u32,
        format: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
        ) = (
            id,
            name,
            width,
            height,
            stride,
            format,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: i32, arg3: i32, arg4: u32, arg5: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_drm#{}.create_buffer(id: wl_buffer#{}, name: {}, width: {}, height: {}, stride: {}, format: {})\n", id, arg0, arg1, arg2, arg3, arg4, arg5);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1, arg2, arg3, arg4, arg5);
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
            arg0_id,
            arg1,
            arg2 as u32,
            arg3 as u32,
            arg4,
            arg5,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `id`:
    /// - `name`:
    /// - `width`:
    /// - `height`:
    /// - `stride`:
    /// - `format`:
    #[inline]
    pub fn send_create_buffer(
        &self,
        id: &Rc<WlBuffer>,
        name: u32,
        width: i32,
        height: i32,
        stride: u32,
        format: u32,
    ) {
        let res = self.try_send_create_buffer(
            id,
            name,
            width,
            height,
            stride,
            format,
        );
        if let Err(e) = res {
            log_send("wl_drm.create_buffer", &e);
        }
    }

    /// # Arguments
    ///
    /// - `name`:
    /// - `width`:
    /// - `height`:
    /// - `stride`:
    /// - `format`:
    #[inline]
    pub fn new_try_send_create_buffer(
        &self,
        name: u32,
        width: i32,
        height: i32,
        stride: u32,
        format: u32,
    ) -> Result<Rc<WlBuffer>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_create_buffer(
            &id,
            name,
            width,
            height,
            stride,
            format,
        )?;
        Ok(id)
    }

    /// # Arguments
    ///
    /// - `name`:
    /// - `width`:
    /// - `height`:
    /// - `stride`:
    /// - `format`:
    #[inline]
    pub fn new_send_create_buffer(
        &self,
        name: u32,
        width: i32,
        height: i32,
        stride: u32,
        format: u32,
    ) -> Rc<WlBuffer> {
        let id = self.core.create_child();
        self.send_create_buffer(
            &id,
            name,
            width,
            height,
            stride,
            format,
        );
        id
    }

    /// Since when the create_planar_buffer message is available.
    pub const MSG__CREATE_PLANAR_BUFFER__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `id`:
    /// - `name`:
    /// - `width`:
    /// - `height`:
    /// - `format`:
    /// - `offset0`:
    /// - `stride0`:
    /// - `offset1`:
    /// - `stride1`:
    /// - `offset2`:
    /// - `stride2`:
    #[inline]
    pub fn try_send_create_planar_buffer(
        &self,
        id: &Rc<WlBuffer>,
        name: u32,
        width: i32,
        height: i32,
        format: u32,
        offset0: i32,
        stride0: i32,
        offset1: i32,
        stride1: i32,
        offset2: i32,
        stride2: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
            arg6,
            arg7,
            arg8,
            arg9,
            arg10,
        ) = (
            id,
            name,
            width,
            height,
            format,
            offset0,
            stride0,
            offset1,
            stride1,
            offset2,
            stride2,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: i32, arg3: i32, arg4: u32, arg5: i32, arg6: i32, arg7: i32, arg8: i32, arg9: i32, arg10: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_drm#{}.create_planar_buffer(id: wl_buffer#{}, name: {}, width: {}, height: {}, format: {}, offset0: {}, stride0: {}, offset1: {}, stride1: {}, offset2: {}, stride2: {})\n", id, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10);
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
            2,
            arg0_id,
            arg1,
            arg2 as u32,
            arg3 as u32,
            arg4,
            arg5 as u32,
            arg6 as u32,
            arg7 as u32,
            arg8 as u32,
            arg9 as u32,
            arg10 as u32,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `id`:
    /// - `name`:
    /// - `width`:
    /// - `height`:
    /// - `format`:
    /// - `offset0`:
    /// - `stride0`:
    /// - `offset1`:
    /// - `stride1`:
    /// - `offset2`:
    /// - `stride2`:
    #[inline]
    pub fn send_create_planar_buffer(
        &self,
        id: &Rc<WlBuffer>,
        name: u32,
        width: i32,
        height: i32,
        format: u32,
        offset0: i32,
        stride0: i32,
        offset1: i32,
        stride1: i32,
        offset2: i32,
        stride2: i32,
    ) {
        let res = self.try_send_create_planar_buffer(
            id,
            name,
            width,
            height,
            format,
            offset0,
            stride0,
            offset1,
            stride1,
            offset2,
            stride2,
        );
        if let Err(e) = res {
            log_send("wl_drm.create_planar_buffer", &e);
        }
    }

    /// # Arguments
    ///
    /// - `name`:
    /// - `width`:
    /// - `height`:
    /// - `format`:
    /// - `offset0`:
    /// - `stride0`:
    /// - `offset1`:
    /// - `stride1`:
    /// - `offset2`:
    /// - `stride2`:
    #[inline]
    pub fn new_try_send_create_planar_buffer(
        &self,
        name: u32,
        width: i32,
        height: i32,
        format: u32,
        offset0: i32,
        stride0: i32,
        offset1: i32,
        stride1: i32,
        offset2: i32,
        stride2: i32,
    ) -> Result<Rc<WlBuffer>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_create_planar_buffer(
            &id,
            name,
            width,
            height,
            format,
            offset0,
            stride0,
            offset1,
            stride1,
            offset2,
            stride2,
        )?;
        Ok(id)
    }

    /// # Arguments
    ///
    /// - `name`:
    /// - `width`:
    /// - `height`:
    /// - `format`:
    /// - `offset0`:
    /// - `stride0`:
    /// - `offset1`:
    /// - `stride1`:
    /// - `offset2`:
    /// - `stride2`:
    #[inline]
    pub fn new_send_create_planar_buffer(
        &self,
        name: u32,
        width: i32,
        height: i32,
        format: u32,
        offset0: i32,
        stride0: i32,
        offset1: i32,
        stride1: i32,
        offset2: i32,
        stride2: i32,
    ) -> Rc<WlBuffer> {
        let id = self.core.create_child();
        self.send_create_planar_buffer(
            &id,
            name,
            width,
            height,
            format,
            offset0,
            stride0,
            offset1,
            stride1,
            offset2,
            stride2,
        );
        id
    }

    /// Since when the device message is available.
    pub const MSG__DEVICE__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    pub fn try_send_device(
        &self,
        name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            name,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_drm#{}.device(name: {:?})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    pub fn send_device(
        &self,
        name: &str,
    ) {
        let res = self.try_send_device(
            name,
        );
        if let Err(e) = res {
            log_send("wl_drm.device", &e);
        }
    }

    /// Since when the format message is available.
    pub const MSG__FORMAT__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `format`:
    #[inline]
    pub fn try_send_format(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_drm#{}.format(format: {})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `format`:
    #[inline]
    pub fn send_format(
        &self,
        format: u32,
    ) {
        let res = self.try_send_format(
            format,
        );
        if let Err(e) = res {
            log_send("wl_drm.format", &e);
        }
    }

    /// Since when the authenticated message is available.
    pub const MSG__AUTHENTICATED__SINCE: u32 = 1;

    #[inline]
    pub fn try_send_authenticated(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_drm#{}.authenticated()\n", client_id, id);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id);
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
            2,
        ]);
        Ok(())
    }

    #[inline]
    pub fn send_authenticated(
        &self,
    ) {
        let res = self.try_send_authenticated(
        );
        if let Err(e) = res {
            log_send("wl_drm.authenticated", &e);
        }
    }

    /// Since when the capabilities message is available.
    pub const MSG__CAPABILITIES__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `value`:
    #[inline]
    pub fn try_send_capabilities(
        &self,
        value: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            value,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_drm#{}.capabilities(value: {})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
            3,
            arg0,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `value`:
    #[inline]
    pub fn send_capabilities(
        &self,
        value: u32,
    ) {
        let res = self.try_send_capabilities(
            value,
        );
        if let Err(e) = res {
            log_send("wl_drm.capabilities", &e);
        }
    }

    /// Since when the create_prime_buffer message is available.
    pub const MSG__CREATE_PRIME_BUFFER__SINCE: u32 = 2;

    /// # Arguments
    ///
    /// - `id`:
    /// - `name`:
    /// - `width`:
    /// - `height`:
    /// - `format`:
    /// - `offset0`:
    /// - `stride0`:
    /// - `offset1`:
    /// - `stride1`:
    /// - `offset2`:
    /// - `stride2`:
    #[inline]
    pub fn try_send_create_prime_buffer(
        &self,
        id: &Rc<WlBuffer>,
        name: &Rc<OwnedFd>,
        width: i32,
        height: i32,
        format: u32,
        offset0: i32,
        stride0: i32,
        offset1: i32,
        stride1: i32,
        offset2: i32,
        stride2: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
            arg6,
            arg7,
            arg8,
            arg9,
            arg10,
        ) = (
            id,
            name,
            width,
            height,
            format,
            offset0,
            stride0,
            offset1,
            stride1,
            offset2,
            stride2,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: i32, arg2: i32, arg3: i32, arg4: u32, arg5: i32, arg6: i32, arg7: i32, arg8: i32, arg9: i32, arg10: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_drm#{}.create_prime_buffer(id: wl_buffer#{}, name: {}, width: {}, height: {}, format: {}, offset0: {}, stride0: {}, offset1: {}, stride1: {}, offset2: {}, stride2: {})\n", id, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1.as_raw_fd(), arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10);
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
        fmt.fds.push_back(arg1.clone());
        fmt.words([
            id,
            3,
            arg0_id,
            arg2 as u32,
            arg3 as u32,
            arg4,
            arg5 as u32,
            arg6 as u32,
            arg7 as u32,
            arg8 as u32,
            arg9 as u32,
            arg10 as u32,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `id`:
    /// - `name`:
    /// - `width`:
    /// - `height`:
    /// - `format`:
    /// - `offset0`:
    /// - `stride0`:
    /// - `offset1`:
    /// - `stride1`:
    /// - `offset2`:
    /// - `stride2`:
    #[inline]
    pub fn send_create_prime_buffer(
        &self,
        id: &Rc<WlBuffer>,
        name: &Rc<OwnedFd>,
        width: i32,
        height: i32,
        format: u32,
        offset0: i32,
        stride0: i32,
        offset1: i32,
        stride1: i32,
        offset2: i32,
        stride2: i32,
    ) {
        let res = self.try_send_create_prime_buffer(
            id,
            name,
            width,
            height,
            format,
            offset0,
            stride0,
            offset1,
            stride1,
            offset2,
            stride2,
        );
        if let Err(e) = res {
            log_send("wl_drm.create_prime_buffer", &e);
        }
    }

    /// # Arguments
    ///
    /// - `name`:
    /// - `width`:
    /// - `height`:
    /// - `format`:
    /// - `offset0`:
    /// - `stride0`:
    /// - `offset1`:
    /// - `stride1`:
    /// - `offset2`:
    /// - `stride2`:
    #[inline]
    pub fn new_try_send_create_prime_buffer(
        &self,
        name: &Rc<OwnedFd>,
        width: i32,
        height: i32,
        format: u32,
        offset0: i32,
        stride0: i32,
        offset1: i32,
        stride1: i32,
        offset2: i32,
        stride2: i32,
    ) -> Result<Rc<WlBuffer>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_create_prime_buffer(
            &id,
            name,
            width,
            height,
            format,
            offset0,
            stride0,
            offset1,
            stride1,
            offset2,
            stride2,
        )?;
        Ok(id)
    }

    /// # Arguments
    ///
    /// - `name`:
    /// - `width`:
    /// - `height`:
    /// - `format`:
    /// - `offset0`:
    /// - `stride0`:
    /// - `offset1`:
    /// - `stride1`:
    /// - `offset2`:
    /// - `stride2`:
    #[inline]
    pub fn new_send_create_prime_buffer(
        &self,
        name: &Rc<OwnedFd>,
        width: i32,
        height: i32,
        format: u32,
        offset0: i32,
        stride0: i32,
        offset1: i32,
        stride1: i32,
        offset2: i32,
        stride2: i32,
    ) -> Rc<WlBuffer> {
        let id = self.core.create_child();
        self.send_create_prime_buffer(
            &id,
            name,
            width,
            height,
            format,
            offset0,
            stride0,
            offset1,
            stride1,
            offset2,
            stride2,
        );
        id
    }
}

/// A message handler for [`WlDrm`] proxies.
pub trait WlDrmHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WlDrm>) {
        slf.core.delete_id();
    }

    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn handle_authenticate(
        &mut self,
        slf: &Rc<WlDrm>,
        id: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_authenticate(
            id,
        );
        if let Err(e) = res {
            log_forward("wl_drm.authenticate", &e);
        }
    }

    /// # Arguments
    ///
    /// - `id`:
    /// - `name`:
    /// - `width`:
    /// - `height`:
    /// - `stride`:
    /// - `format`:
    #[inline]
    fn handle_create_buffer(
        &mut self,
        slf: &Rc<WlDrm>,
        id: &Rc<WlBuffer>,
        name: u32,
        width: i32,
        height: i32,
        stride: u32,
        format: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_buffer(
            id,
            name,
            width,
            height,
            stride,
            format,
        );
        if let Err(e) = res {
            log_forward("wl_drm.create_buffer", &e);
        }
    }

    /// # Arguments
    ///
    /// - `id`:
    /// - `name`:
    /// - `width`:
    /// - `height`:
    /// - `format`:
    /// - `offset0`:
    /// - `stride0`:
    /// - `offset1`:
    /// - `stride1`:
    /// - `offset2`:
    /// - `stride2`:
    #[inline]
    fn handle_create_planar_buffer(
        &mut self,
        slf: &Rc<WlDrm>,
        id: &Rc<WlBuffer>,
        name: u32,
        width: i32,
        height: i32,
        format: u32,
        offset0: i32,
        stride0: i32,
        offset1: i32,
        stride1: i32,
        offset2: i32,
        stride2: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_planar_buffer(
            id,
            name,
            width,
            height,
            format,
            offset0,
            stride0,
            offset1,
            stride1,
            offset2,
            stride2,
        );
        if let Err(e) = res {
            log_forward("wl_drm.create_planar_buffer", &e);
        }
    }

    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    fn handle_device(
        &mut self,
        slf: &Rc<WlDrm>,
        name: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_device(
            name,
        );
        if let Err(e) = res {
            log_forward("wl_drm.device", &e);
        }
    }

    /// # Arguments
    ///
    /// - `format`:
    #[inline]
    fn handle_format(
        &mut self,
        slf: &Rc<WlDrm>,
        format: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_format(
            format,
        );
        if let Err(e) = res {
            log_forward("wl_drm.format", &e);
        }
    }

    #[inline]
    fn handle_authenticated(
        &mut self,
        slf: &Rc<WlDrm>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_authenticated(
        );
        if let Err(e) = res {
            log_forward("wl_drm.authenticated", &e);
        }
    }

    /// # Arguments
    ///
    /// - `value`:
    #[inline]
    fn handle_capabilities(
        &mut self,
        slf: &Rc<WlDrm>,
        value: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_capabilities(
            value,
        );
        if let Err(e) = res {
            log_forward("wl_drm.capabilities", &e);
        }
    }

    /// # Arguments
    ///
    /// - `id`:
    /// - `name`:
    /// - `width`:
    /// - `height`:
    /// - `format`:
    /// - `offset0`:
    /// - `stride0`:
    /// - `offset1`:
    /// - `stride1`:
    /// - `offset2`:
    /// - `stride2`:
    #[inline]
    fn handle_create_prime_buffer(
        &mut self,
        slf: &Rc<WlDrm>,
        id: &Rc<WlBuffer>,
        name: &Rc<OwnedFd>,
        width: i32,
        height: i32,
        format: u32,
        offset0: i32,
        stride0: i32,
        offset1: i32,
        stride1: i32,
        offset2: i32,
        stride2: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_prime_buffer(
            id,
            name,
            width,
            height,
            format,
            offset0,
            stride0,
            offset1,
            stride1,
            offset2,
            stride2,
        );
        if let Err(e) = res {
            log_forward("wl_drm.create_prime_buffer", &e);
        }
    }
}

impl ObjectPrivate for WlDrm {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WlDrm, version),
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_drm#{}.authenticate(id: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_authenticate(&self, arg0);
                } else {
                    DefaultHandler.handle_authenticate(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                    arg5,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 32)));
                };
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: i32, arg3: i32, arg4: u32, arg5: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_drm#{}.create_buffer(id: wl_buffer#{}, name: {}, width: {}, height: {}, stride: {}, format: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4, arg5);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4, arg5);
                }
                let arg0_id = arg0;
                let arg0 = WlBuffer::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_buffer(&self, arg0, arg1, arg2, arg3, arg4, arg5);
                } else {
                    DefaultHandler.handle_create_buffer(&self, arg0, arg1, arg2, arg3, arg4, arg5);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                    arg5,
                    arg6,
                    arg7,
                    arg8,
                    arg9,
                    arg10,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 52)));
                };
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                let arg5 = arg5 as i32;
                let arg6 = arg6 as i32;
                let arg7 = arg7 as i32;
                let arg8 = arg8 as i32;
                let arg9 = arg9 as i32;
                let arg10 = arg10 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: i32, arg3: i32, arg4: u32, arg5: i32, arg6: i32, arg7: i32, arg8: i32, arg9: i32, arg10: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_drm#{}.create_planar_buffer(id: wl_buffer#{}, name: {}, width: {}, height: {}, format: {}, offset0: {}, stride0: {}, offset1: {}, stride1: {}, offset2: {}, stride2: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10);
                }
                let arg0_id = arg0;
                let arg0 = WlBuffer::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_planar_buffer(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10);
                } else {
                    DefaultHandler.handle_create_planar_buffer(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10);
                }
            }
            3 => {
                let [
                    arg0,
                    arg2,
                    arg3,
                    arg4,
                    arg5,
                    arg6,
                    arg7,
                    arg8,
                    arg9,
                    arg10,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 48)));
                };
                let Some(arg1) = fds.pop_front() else {
                    return Err(ObjectError(ObjectErrorKind::MissingFd("name")));
                };
                let arg1 = &arg1;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                let arg5 = arg5 as i32;
                let arg6 = arg6 as i32;
                let arg7 = arg7 as i32;
                let arg8 = arg8 as i32;
                let arg9 = arg9 as i32;
                let arg10 = arg10 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: i32, arg2: i32, arg3: i32, arg4: u32, arg5: i32, arg6: i32, arg7: i32, arg8: i32, arg9: i32, arg10: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_drm#{}.create_prime_buffer(id: wl_buffer#{}, name: {}, width: {}, height: {}, format: {}, offset0: {}, stride0: {}, offset1: {}, stride1: {}, offset2: {}, stride2: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1.as_raw_fd(), arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10);
                }
                let arg0_id = arg0;
                let arg0 = WlBuffer::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_prime_buffer(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10);
                } else {
                    DefaultHandler.handle_create_prime_buffer(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10);
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
            0 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "name")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_drm#{}.device(name: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_device(&self, arg0);
                } else {
                    DefaultHandler.handle_device(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_drm#{}.format(format: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_format(&self, arg0);
                } else {
                    DefaultHandler.handle_format(&self, arg0);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_drm#{}.authenticated()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_authenticated(&self);
                } else {
                    DefaultHandler.handle_authenticated(&self);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_drm#{}.capabilities(value: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_capabilities(&self, arg0);
                } else {
                    DefaultHandler.handle_capabilities(&self, arg0);
                }
            }
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
        Ok(())
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "authenticate",
            1 => "create_buffer",
            2 => "create_planar_buffer",
            3 => "create_prime_buffer",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "device",
            1 => "format",
            2 => "authenticated",
            3 => "capabilities",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WlDrm {
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

impl WlDrm {
    /// Since when the error.authenticate_fail enum variant is available.
    pub const ENM__ERROR_AUTHENTICATE_FAIL__SINCE: u32 = 1;
    /// Since when the error.invalid_format enum variant is available.
    pub const ENM__ERROR_INVALID_FORMAT__SINCE: u32 = 1;
    /// Since when the error.invalid_name enum variant is available.
    pub const ENM__ERROR_INVALID_NAME__SINCE: u32 = 1;

    /// Since when the format.c8 enum variant is available.
    pub const ENM__FORMAT_C8__SINCE: u32 = 1;
    /// Since when the format.rgb332 enum variant is available.
    pub const ENM__FORMAT_RGB332__SINCE: u32 = 1;
    /// Since when the format.bgr233 enum variant is available.
    pub const ENM__FORMAT_BGR233__SINCE: u32 = 1;
    /// Since when the format.xrgb4444 enum variant is available.
    pub const ENM__FORMAT_XRGB4444__SINCE: u32 = 1;
    /// Since when the format.xbgr4444 enum variant is available.
    pub const ENM__FORMAT_XBGR4444__SINCE: u32 = 1;
    /// Since when the format.rgbx4444 enum variant is available.
    pub const ENM__FORMAT_RGBX4444__SINCE: u32 = 1;
    /// Since when the format.bgrx4444 enum variant is available.
    pub const ENM__FORMAT_BGRX4444__SINCE: u32 = 1;
    /// Since when the format.argb4444 enum variant is available.
    pub const ENM__FORMAT_ARGB4444__SINCE: u32 = 1;
    /// Since when the format.abgr4444 enum variant is available.
    pub const ENM__FORMAT_ABGR4444__SINCE: u32 = 1;
    /// Since when the format.rgba4444 enum variant is available.
    pub const ENM__FORMAT_RGBA4444__SINCE: u32 = 1;
    /// Since when the format.bgra4444 enum variant is available.
    pub const ENM__FORMAT_BGRA4444__SINCE: u32 = 1;
    /// Since when the format.xrgb1555 enum variant is available.
    pub const ENM__FORMAT_XRGB1555__SINCE: u32 = 1;
    /// Since when the format.xbgr1555 enum variant is available.
    pub const ENM__FORMAT_XBGR1555__SINCE: u32 = 1;
    /// Since when the format.rgbx5551 enum variant is available.
    pub const ENM__FORMAT_RGBX5551__SINCE: u32 = 1;
    /// Since when the format.bgrx5551 enum variant is available.
    pub const ENM__FORMAT_BGRX5551__SINCE: u32 = 1;
    /// Since when the format.argb1555 enum variant is available.
    pub const ENM__FORMAT_ARGB1555__SINCE: u32 = 1;
    /// Since when the format.abgr1555 enum variant is available.
    pub const ENM__FORMAT_ABGR1555__SINCE: u32 = 1;
    /// Since when the format.rgba5551 enum variant is available.
    pub const ENM__FORMAT_RGBA5551__SINCE: u32 = 1;
    /// Since when the format.bgra5551 enum variant is available.
    pub const ENM__FORMAT_BGRA5551__SINCE: u32 = 1;
    /// Since when the format.rgb565 enum variant is available.
    pub const ENM__FORMAT_RGB565__SINCE: u32 = 1;
    /// Since when the format.bgr565 enum variant is available.
    pub const ENM__FORMAT_BGR565__SINCE: u32 = 1;
    /// Since when the format.rgb888 enum variant is available.
    pub const ENM__FORMAT_RGB888__SINCE: u32 = 1;
    /// Since when the format.bgr888 enum variant is available.
    pub const ENM__FORMAT_BGR888__SINCE: u32 = 1;
    /// Since when the format.xrgb8888 enum variant is available.
    pub const ENM__FORMAT_XRGB8888__SINCE: u32 = 1;
    /// Since when the format.xbgr8888 enum variant is available.
    pub const ENM__FORMAT_XBGR8888__SINCE: u32 = 1;
    /// Since when the format.rgbx8888 enum variant is available.
    pub const ENM__FORMAT_RGBX8888__SINCE: u32 = 1;
    /// Since when the format.bgrx8888 enum variant is available.
    pub const ENM__FORMAT_BGRX8888__SINCE: u32 = 1;
    /// Since when the format.argb8888 enum variant is available.
    pub const ENM__FORMAT_ARGB8888__SINCE: u32 = 1;
    /// Since when the format.abgr8888 enum variant is available.
    pub const ENM__FORMAT_ABGR8888__SINCE: u32 = 1;
    /// Since when the format.rgba8888 enum variant is available.
    pub const ENM__FORMAT_RGBA8888__SINCE: u32 = 1;
    /// Since when the format.bgra8888 enum variant is available.
    pub const ENM__FORMAT_BGRA8888__SINCE: u32 = 1;
    /// Since when the format.xrgb2101010 enum variant is available.
    pub const ENM__FORMAT_XRGB2101010__SINCE: u32 = 1;
    /// Since when the format.xbgr2101010 enum variant is available.
    pub const ENM__FORMAT_XBGR2101010__SINCE: u32 = 1;
    /// Since when the format.rgbx1010102 enum variant is available.
    pub const ENM__FORMAT_RGBX1010102__SINCE: u32 = 1;
    /// Since when the format.bgrx1010102 enum variant is available.
    pub const ENM__FORMAT_BGRX1010102__SINCE: u32 = 1;
    /// Since when the format.argb2101010 enum variant is available.
    pub const ENM__FORMAT_ARGB2101010__SINCE: u32 = 1;
    /// Since when the format.abgr2101010 enum variant is available.
    pub const ENM__FORMAT_ABGR2101010__SINCE: u32 = 1;
    /// Since when the format.rgba1010102 enum variant is available.
    pub const ENM__FORMAT_RGBA1010102__SINCE: u32 = 1;
    /// Since when the format.bgra1010102 enum variant is available.
    pub const ENM__FORMAT_BGRA1010102__SINCE: u32 = 1;
    /// Since when the format.yuyv enum variant is available.
    pub const ENM__FORMAT_YUYV__SINCE: u32 = 1;
    /// Since when the format.yvyu enum variant is available.
    pub const ENM__FORMAT_YVYU__SINCE: u32 = 1;
    /// Since when the format.uyvy enum variant is available.
    pub const ENM__FORMAT_UYVY__SINCE: u32 = 1;
    /// Since when the format.vyuy enum variant is available.
    pub const ENM__FORMAT_VYUY__SINCE: u32 = 1;
    /// Since when the format.ayuv enum variant is available.
    pub const ENM__FORMAT_AYUV__SINCE: u32 = 1;
    /// Since when the format.xyuv8888 enum variant is available.
    pub const ENM__FORMAT_XYUV8888__SINCE: u32 = 1;
    /// Since when the format.nv12 enum variant is available.
    pub const ENM__FORMAT_NV12__SINCE: u32 = 1;
    /// Since when the format.nv21 enum variant is available.
    pub const ENM__FORMAT_NV21__SINCE: u32 = 1;
    /// Since when the format.nv16 enum variant is available.
    pub const ENM__FORMAT_NV16__SINCE: u32 = 1;
    /// Since when the format.nv61 enum variant is available.
    pub const ENM__FORMAT_NV61__SINCE: u32 = 1;
    /// Since when the format.yuv410 enum variant is available.
    pub const ENM__FORMAT_YUV410__SINCE: u32 = 1;
    /// Since when the format.yvu410 enum variant is available.
    pub const ENM__FORMAT_YVU410__SINCE: u32 = 1;
    /// Since when the format.yuv411 enum variant is available.
    pub const ENM__FORMAT_YUV411__SINCE: u32 = 1;
    /// Since when the format.yvu411 enum variant is available.
    pub const ENM__FORMAT_YVU411__SINCE: u32 = 1;
    /// Since when the format.yuv420 enum variant is available.
    pub const ENM__FORMAT_YUV420__SINCE: u32 = 1;
    /// Since when the format.yvu420 enum variant is available.
    pub const ENM__FORMAT_YVU420__SINCE: u32 = 1;
    /// Since when the format.yuv422 enum variant is available.
    pub const ENM__FORMAT_YUV422__SINCE: u32 = 1;
    /// Since when the format.yvu422 enum variant is available.
    pub const ENM__FORMAT_YVU422__SINCE: u32 = 1;
    /// Since when the format.yuv444 enum variant is available.
    pub const ENM__FORMAT_YUV444__SINCE: u32 = 1;
    /// Since when the format.yvu444 enum variant is available.
    pub const ENM__FORMAT_YVU444__SINCE: u32 = 1;
    /// Since when the format.abgr16f enum variant is available.
    pub const ENM__FORMAT_ABGR16F__SINCE: u32 = 1;
    /// Since when the format.xbgr16f enum variant is available.
    pub const ENM__FORMAT_XBGR16F__SINCE: u32 = 1;

    /// Since when the capability.prime enum variant is available.
    pub const ENM__CAPABILITY_PRIME__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WlDrmError(pub u32);

impl WlDrmError {
    pub const AUTHENTICATE_FAIL: Self = Self(0);

    pub const INVALID_FORMAT: Self = Self(1);

    pub const INVALID_NAME: Self = Self(2);
}

impl Debug for WlDrmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::AUTHENTICATE_FAIL => "AUTHENTICATE_FAIL",
            Self::INVALID_FORMAT => "INVALID_FORMAT",
            Self::INVALID_NAME => "INVALID_NAME",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WlDrmFormat(pub u32);

impl WlDrmFormat {
    pub const C8: Self = Self(0x20203843);

    pub const RGB332: Self = Self(0x38424752);

    pub const BGR233: Self = Self(0x38524742);

    pub const XRGB4444: Self = Self(0x32315258);

    pub const XBGR4444: Self = Self(0x32314258);

    pub const RGBX4444: Self = Self(0x32315852);

    pub const BGRX4444: Self = Self(0x32315842);

    pub const ARGB4444: Self = Self(0x32315241);

    pub const ABGR4444: Self = Self(0x32314241);

    pub const RGBA4444: Self = Self(0x32314152);

    pub const BGRA4444: Self = Self(0x32314142);

    pub const XRGB1555: Self = Self(0x35315258);

    pub const XBGR1555: Self = Self(0x35314258);

    pub const RGBX5551: Self = Self(0x35315852);

    pub const BGRX5551: Self = Self(0x35315842);

    pub const ARGB1555: Self = Self(0x35315241);

    pub const ABGR1555: Self = Self(0x35314241);

    pub const RGBA5551: Self = Self(0x35314152);

    pub const BGRA5551: Self = Self(0x35314142);

    pub const RGB565: Self = Self(0x36314752);

    pub const BGR565: Self = Self(0x36314742);

    pub const RGB888: Self = Self(0x34324752);

    pub const BGR888: Self = Self(0x34324742);

    pub const XRGB8888: Self = Self(0x34325258);

    pub const XBGR8888: Self = Self(0x34324258);

    pub const RGBX8888: Self = Self(0x34325852);

    pub const BGRX8888: Self = Self(0x34325842);

    pub const ARGB8888: Self = Self(0x34325241);

    pub const ABGR8888: Self = Self(0x34324241);

    pub const RGBA8888: Self = Self(0x34324152);

    pub const BGRA8888: Self = Self(0x34324142);

    pub const XRGB2101010: Self = Self(0x30335258);

    pub const XBGR2101010: Self = Self(0x30334258);

    pub const RGBX1010102: Self = Self(0x30335852);

    pub const BGRX1010102: Self = Self(0x30335842);

    pub const ARGB2101010: Self = Self(0x30335241);

    pub const ABGR2101010: Self = Self(0x30334241);

    pub const RGBA1010102: Self = Self(0x30334152);

    pub const BGRA1010102: Self = Self(0x30334142);

    pub const YUYV: Self = Self(0x56595559);

    pub const YVYU: Self = Self(0x55595659);

    pub const UYVY: Self = Self(0x59565955);

    pub const VYUY: Self = Self(0x59555956);

    pub const AYUV: Self = Self(0x56555941);

    pub const XYUV8888: Self = Self(0x56555958);

    pub const NV12: Self = Self(0x3231564e);

    pub const NV21: Self = Self(0x3132564e);

    pub const NV16: Self = Self(0x3631564e);

    pub const NV61: Self = Self(0x3136564e);

    pub const YUV410: Self = Self(0x39565559);

    pub const YVU410: Self = Self(0x39555659);

    pub const YUV411: Self = Self(0x31315559);

    pub const YVU411: Self = Self(0x31315659);

    pub const YUV420: Self = Self(0x32315559);

    pub const YVU420: Self = Self(0x32315659);

    pub const YUV422: Self = Self(0x36315559);

    pub const YVU422: Self = Self(0x36315659);

    pub const YUV444: Self = Self(0x34325559);

    pub const YVU444: Self = Self(0x34325659);

    pub const ABGR16F: Self = Self(0x48344241);

    pub const XBGR16F: Self = Self(0x48344258);
}

impl Debug for WlDrmFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::C8 => "C8",
            Self::RGB332 => "RGB332",
            Self::BGR233 => "BGR233",
            Self::XRGB4444 => "XRGB4444",
            Self::XBGR4444 => "XBGR4444",
            Self::RGBX4444 => "RGBX4444",
            Self::BGRX4444 => "BGRX4444",
            Self::ARGB4444 => "ARGB4444",
            Self::ABGR4444 => "ABGR4444",
            Self::RGBA4444 => "RGBA4444",
            Self::BGRA4444 => "BGRA4444",
            Self::XRGB1555 => "XRGB1555",
            Self::XBGR1555 => "XBGR1555",
            Self::RGBX5551 => "RGBX5551",
            Self::BGRX5551 => "BGRX5551",
            Self::ARGB1555 => "ARGB1555",
            Self::ABGR1555 => "ABGR1555",
            Self::RGBA5551 => "RGBA5551",
            Self::BGRA5551 => "BGRA5551",
            Self::RGB565 => "RGB565",
            Self::BGR565 => "BGR565",
            Self::RGB888 => "RGB888",
            Self::BGR888 => "BGR888",
            Self::XRGB8888 => "XRGB8888",
            Self::XBGR8888 => "XBGR8888",
            Self::RGBX8888 => "RGBX8888",
            Self::BGRX8888 => "BGRX8888",
            Self::ARGB8888 => "ARGB8888",
            Self::ABGR8888 => "ABGR8888",
            Self::RGBA8888 => "RGBA8888",
            Self::BGRA8888 => "BGRA8888",
            Self::XRGB2101010 => "XRGB2101010",
            Self::XBGR2101010 => "XBGR2101010",
            Self::RGBX1010102 => "RGBX1010102",
            Self::BGRX1010102 => "BGRX1010102",
            Self::ARGB2101010 => "ARGB2101010",
            Self::ABGR2101010 => "ABGR2101010",
            Self::RGBA1010102 => "RGBA1010102",
            Self::BGRA1010102 => "BGRA1010102",
            Self::YUYV => "YUYV",
            Self::YVYU => "YVYU",
            Self::UYVY => "UYVY",
            Self::VYUY => "VYUY",
            Self::AYUV => "AYUV",
            Self::XYUV8888 => "XYUV8888",
            Self::NV12 => "NV12",
            Self::NV21 => "NV21",
            Self::NV16 => "NV16",
            Self::NV61 => "NV61",
            Self::YUV410 => "YUV410",
            Self::YVU410 => "YVU410",
            Self::YUV411 => "YUV411",
            Self::YVU411 => "YVU411",
            Self::YUV420 => "YUV420",
            Self::YVU420 => "YVU420",
            Self::YUV422 => "YUV422",
            Self::YVU422 => "YVU422",
            Self::YUV444 => "YUV444",
            Self::YVU444 => "YVU444",
            Self::ABGR16F => "ABGR16F",
            Self::XBGR16F => "XBGR16F",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// wl_drm capability bitmask
///
/// Bitmask of capabilities.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WlDrmCapability(pub u32);

impl WlDrmCapability {
    /// wl_drm prime available
    pub const PRIME: Self = Self(1);
}

impl Debug for WlDrmCapability {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::PRIME => "PRIME",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
