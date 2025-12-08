//! weston internal testing
//!
//! Internal testing facilities for the weston compositor.
//!
//! It can't be stressed enough that these should never ever be used
//! outside of running weston's tests.  The weston-test.so module should
//! never be installed.
//!
//! These requests may allow clients to do very bad things.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A weston_test object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WestonTest {
    core: ObjectCore,
    handler: HandlerHolder<dyn WestonTestHandler>,
}

struct DefaultHandler;

impl WestonTestHandler for DefaultHandler { }

impl ConcreteObject for WestonTest {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WestonTest;
    const INTERFACE_NAME: &str = "weston_test";
}

impl WestonTest {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WestonTestHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WestonTestHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WestonTest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WestonTest")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WestonTest {
    /// Since when the move_surface message is available.
    pub const MSG__MOVE_SURFACE__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `surface`:
    /// - `x`:
    /// - `y`:
    #[inline]
    pub fn try_send_move_surface(
        &self,
        surface: &Rc<WlSurface>,
        x: i32,
        y: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            surface,
            x,
            y,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: i32, arg2: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_test#{}.move_surface(surface: wl_surface#{}, x: {}, y: {})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1, arg2);
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
            arg0_id,
            arg1 as u32,
            arg2 as u32,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `surface`:
    /// - `x`:
    /// - `y`:
    #[inline]
    pub fn send_move_surface(
        &self,
        surface: &Rc<WlSurface>,
        x: i32,
        y: i32,
    ) {
        let res = self.try_send_move_surface(
            surface,
            x,
            y,
        );
        if let Err(e) = res {
            log_send("weston_test.move_surface", &e);
        }
    }

    /// Since when the move_pointer message is available.
    pub const MSG__MOVE_POINTER__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `tv_sec_hi`:
    /// - `tv_sec_lo`:
    /// - `tv_nsec`:
    /// - `x`:
    /// - `y`:
    #[inline]
    pub fn try_send_move_pointer(
        &self,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
        x: i32,
        y: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ) = (
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
            x,
            y,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: i32, arg4: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_test#{}.move_pointer(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {}, x: {}, y: {})\n", id, arg0, arg1, arg2, arg3, arg4);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2, arg3, arg4);
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
            arg0,
            arg1,
            arg2,
            arg3 as u32,
            arg4 as u32,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `tv_sec_hi`:
    /// - `tv_sec_lo`:
    /// - `tv_nsec`:
    /// - `x`:
    /// - `y`:
    #[inline]
    pub fn send_move_pointer(
        &self,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
        x: i32,
        y: i32,
    ) {
        let res = self.try_send_move_pointer(
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
            x,
            y,
        );
        if let Err(e) = res {
            log_send("weston_test.move_pointer", &e);
        }
    }

    /// Since when the send_button message is available.
    pub const MSG__SEND_BUTTON__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `tv_sec_hi`:
    /// - `tv_sec_lo`:
    /// - `tv_nsec`:
    /// - `button`:
    /// - `state`:
    #[inline]
    pub fn try_send_send_button(
        &self,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
        button: i32,
        state: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ) = (
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
            button,
            state,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: i32, arg4: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_test#{}.send_button(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {}, button: {}, state: {})\n", id, arg0, arg1, arg2, arg3, arg4);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2, arg3, arg4);
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
            arg0,
            arg1,
            arg2,
            arg3 as u32,
            arg4,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `tv_sec_hi`:
    /// - `tv_sec_lo`:
    /// - `tv_nsec`:
    /// - `button`:
    /// - `state`:
    #[inline]
    pub fn send_send_button(
        &self,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
        button: i32,
        state: u32,
    ) {
        let res = self.try_send_send_button(
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
            button,
            state,
        );
        if let Err(e) = res {
            log_send("weston_test.send_button", &e);
        }
    }

    /// Since when the send_axis message is available.
    pub const MSG__SEND_AXIS__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `tv_sec_hi`:
    /// - `tv_sec_lo`:
    /// - `tv_nsec`:
    /// - `axis`:
    /// - `value`:
    #[inline]
    pub fn try_send_send_axis(
        &self,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
        axis: u32,
        value: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ) = (
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
            axis,
            value,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: Fixed) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_test#{}.send_axis(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {}, axis: {}, value: {})\n", id, arg0, arg1, arg2, arg3, arg4);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2, arg3, arg4);
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
            3,
            arg0,
            arg1,
            arg2,
            arg3,
            arg4.to_wire() as u32,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `tv_sec_hi`:
    /// - `tv_sec_lo`:
    /// - `tv_nsec`:
    /// - `axis`:
    /// - `value`:
    #[inline]
    pub fn send_send_axis(
        &self,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
        axis: u32,
        value: Fixed,
    ) {
        let res = self.try_send_send_axis(
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
            axis,
            value,
        );
        if let Err(e) = res {
            log_send("weston_test.send_axis", &e);
        }
    }

    /// Since when the activate_surface message is available.
    pub const MSG__ACTIVATE_SURFACE__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn try_send_activate_surface(
        &self,
        surface: Option<&Rc<WlSurface>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            surface,
        );
        let arg0 = arg0.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
                Some(id) => id,
            },
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_test#{}.activate_surface(surface: wl_surface#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
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
            4,
            arg0_id,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn send_activate_surface(
        &self,
        surface: Option<&Rc<WlSurface>>,
    ) {
        let res = self.try_send_activate_surface(
            surface,
        );
        if let Err(e) = res {
            log_send("weston_test.activate_surface", &e);
        }
    }

    /// Since when the send_key message is available.
    pub const MSG__SEND_KEY__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `tv_sec_hi`:
    /// - `tv_sec_lo`:
    /// - `tv_nsec`:
    /// - `key`:
    /// - `state`:
    #[inline]
    pub fn try_send_send_key(
        &self,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
        key: u32,
        state: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ) = (
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
            key,
            state,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_test#{}.send_key(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {}, key: {}, state: {})\n", id, arg0, arg1, arg2, arg3, arg4);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2, arg3, arg4);
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
            5,
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `tv_sec_hi`:
    /// - `tv_sec_lo`:
    /// - `tv_nsec`:
    /// - `key`:
    /// - `state`:
    #[inline]
    pub fn send_send_key(
        &self,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
        key: u32,
        state: u32,
    ) {
        let res = self.try_send_send_key(
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
            key,
            state,
        );
        if let Err(e) = res {
            log_send("weston_test.send_key", &e);
        }
    }

    /// Since when the device_release message is available.
    pub const MSG__DEVICE_RELEASE__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `device`:
    #[inline]
    pub fn try_send_device_release(
        &self,
        device: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            device,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_test#{}.device_release(device: {:?})\n", id, arg0);
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
            6,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `device`:
    #[inline]
    pub fn send_device_release(
        &self,
        device: &str,
    ) {
        let res = self.try_send_device_release(
            device,
        );
        if let Err(e) = res {
            log_send("weston_test.device_release", &e);
        }
    }

    /// Since when the device_add message is available.
    pub const MSG__DEVICE_ADD__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `device`:
    #[inline]
    pub fn try_send_device_add(
        &self,
        device: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            device,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_test#{}.device_add(device: {:?})\n", id, arg0);
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
            7,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `device`:
    #[inline]
    pub fn send_device_add(
        &self,
        device: &str,
    ) {
        let res = self.try_send_device_add(
            device,
        );
        if let Err(e) = res {
            log_send("weston_test.device_add", &e);
        }
    }

    /// Since when the pointer_position message is available.
    pub const MSG__POINTER_POSITION__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    #[inline]
    pub fn try_send_pointer_position(
        &self,
        x: Fixed,
        y: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            x,
            y,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: Fixed, arg1: Fixed) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_test#{}.pointer_position(x: {}, y: {})\n", client_id, id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1);
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
            arg0.to_wire() as u32,
            arg1.to_wire() as u32,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    #[inline]
    pub fn send_pointer_position(
        &self,
        x: Fixed,
        y: Fixed,
    ) {
        let res = self.try_send_pointer_position(
            x,
            y,
        );
        if let Err(e) = res {
            log_send("weston_test.pointer_position", &e);
        }
    }

    /// Since when the send_touch message is available.
    pub const MSG__SEND_TOUCH__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `tv_sec_hi`:
    /// - `tv_sec_lo`:
    /// - `tv_nsec`:
    /// - `touch_id`:
    /// - `x`:
    /// - `y`:
    /// - `touch_type`:
    #[inline]
    pub fn try_send_send_touch(
        &self,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
        touch_id: i32,
        x: Fixed,
        y: Fixed,
        touch_type: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
            arg6,
        ) = (
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
            touch_id,
            x,
            y,
            touch_type,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: i32, arg4: Fixed, arg5: Fixed, arg6: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_test#{}.send_touch(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {}, touch_id: {}, x: {}, y: {}, touch_type: {})\n", id, arg0, arg1, arg2, arg3, arg4, arg5, arg6);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2, arg3, arg4, arg5, arg6);
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
            8,
            arg0,
            arg1,
            arg2,
            arg3 as u32,
            arg4.to_wire() as u32,
            arg5.to_wire() as u32,
            arg6,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `tv_sec_hi`:
    /// - `tv_sec_lo`:
    /// - `tv_nsec`:
    /// - `touch_id`:
    /// - `x`:
    /// - `y`:
    /// - `touch_type`:
    #[inline]
    pub fn send_send_touch(
        &self,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
        touch_id: i32,
        x: Fixed,
        y: Fixed,
        touch_type: u32,
    ) {
        let res = self.try_send_send_touch(
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
            touch_id,
            x,
            y,
            touch_type,
        );
        if let Err(e) = res {
            log_send("weston_test.send_touch", &e);
        }
    }

    /// Since when the client_break message is available.
    pub const MSG__CLIENT_BREAK__SINCE: u32 = 1;

    /// request compositor pause at a certain point
    ///
    /// Request that the compositor pauses execution at a certain point. When
    /// execution is paused, the compositor will signal the shared semaphore
    /// to the client.
    ///
    /// # Arguments
    ///
    /// - `breakpoint`: event type to wait for
    /// - `resource_id`: optional Wayland resource ID to filter for (type-specific)
    #[inline]
    pub fn try_send_client_break(
        &self,
        breakpoint: WestonTestBreakpoint,
        resource_id: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            breakpoint,
            resource_id,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: WestonTestBreakpoint, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_test#{}.client_break(breakpoint: {:?}, resource_id: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1);
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
            9,
            arg0.0,
            arg1,
        ]);
        Ok(())
    }

    /// request compositor pause at a certain point
    ///
    /// Request that the compositor pauses execution at a certain point. When
    /// execution is paused, the compositor will signal the shared semaphore
    /// to the client.
    ///
    /// # Arguments
    ///
    /// - `breakpoint`: event type to wait for
    /// - `resource_id`: optional Wayland resource ID to filter for (type-specific)
    #[inline]
    pub fn send_client_break(
        &self,
        breakpoint: WestonTestBreakpoint,
        resource_id: u32,
    ) {
        let res = self.try_send_client_break(
            breakpoint,
            resource_id,
        );
        if let Err(e) = res {
            log_send("weston_test.client_break", &e);
        }
    }
}

/// A message handler for [`WestonTest`] proxies.
pub trait WestonTestHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WestonTest>) {
        slf.core.delete_id();
    }

    /// # Arguments
    ///
    /// - `surface`:
    /// - `x`:
    /// - `y`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_move_surface(
        &mut self,
        slf: &Rc<WestonTest>,
        surface: &Rc<WlSurface>,
        x: i32,
        y: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_move_surface(
            surface,
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("weston_test.move_surface", &e);
        }
    }

    /// # Arguments
    ///
    /// - `tv_sec_hi`:
    /// - `tv_sec_lo`:
    /// - `tv_nsec`:
    /// - `x`:
    /// - `y`:
    #[inline]
    fn handle_move_pointer(
        &mut self,
        slf: &Rc<WestonTest>,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
        x: i32,
        y: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_move_pointer(
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("weston_test.move_pointer", &e);
        }
    }

    /// # Arguments
    ///
    /// - `tv_sec_hi`:
    /// - `tv_sec_lo`:
    /// - `tv_nsec`:
    /// - `button`:
    /// - `state`:
    #[inline]
    fn handle_send_button(
        &mut self,
        slf: &Rc<WestonTest>,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
        button: i32,
        state: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_send_button(
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
            button,
            state,
        );
        if let Err(e) = res {
            log_forward("weston_test.send_button", &e);
        }
    }

    /// # Arguments
    ///
    /// - `tv_sec_hi`:
    /// - `tv_sec_lo`:
    /// - `tv_nsec`:
    /// - `axis`:
    /// - `value`:
    #[inline]
    fn handle_send_axis(
        &mut self,
        slf: &Rc<WestonTest>,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
        axis: u32,
        value: Fixed,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_send_axis(
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
            axis,
            value,
        );
        if let Err(e) = res {
            log_forward("weston_test.send_axis", &e);
        }
    }

    /// # Arguments
    ///
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_activate_surface(
        &mut self,
        slf: &Rc<WestonTest>,
        surface: Option<&Rc<WlSurface>>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_activate_surface(
            surface,
        );
        if let Err(e) = res {
            log_forward("weston_test.activate_surface", &e);
        }
    }

    /// # Arguments
    ///
    /// - `tv_sec_hi`:
    /// - `tv_sec_lo`:
    /// - `tv_nsec`:
    /// - `key`:
    /// - `state`:
    #[inline]
    fn handle_send_key(
        &mut self,
        slf: &Rc<WestonTest>,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
        key: u32,
        state: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_send_key(
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
            key,
            state,
        );
        if let Err(e) = res {
            log_forward("weston_test.send_key", &e);
        }
    }

    /// # Arguments
    ///
    /// - `device`:
    #[inline]
    fn handle_device_release(
        &mut self,
        slf: &Rc<WestonTest>,
        device: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_device_release(
            device,
        );
        if let Err(e) = res {
            log_forward("weston_test.device_release", &e);
        }
    }

    /// # Arguments
    ///
    /// - `device`:
    #[inline]
    fn handle_device_add(
        &mut self,
        slf: &Rc<WestonTest>,
        device: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_device_add(
            device,
        );
        if let Err(e) = res {
            log_forward("weston_test.device_add", &e);
        }
    }

    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    #[inline]
    fn handle_pointer_position(
        &mut self,
        slf: &Rc<WestonTest>,
        x: Fixed,
        y: Fixed,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_pointer_position(
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("weston_test.pointer_position", &e);
        }
    }

    /// # Arguments
    ///
    /// - `tv_sec_hi`:
    /// - `tv_sec_lo`:
    /// - `tv_nsec`:
    /// - `touch_id`:
    /// - `x`:
    /// - `y`:
    /// - `touch_type`:
    #[inline]
    fn handle_send_touch(
        &mut self,
        slf: &Rc<WestonTest>,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
        touch_id: i32,
        x: Fixed,
        y: Fixed,
        touch_type: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_send_touch(
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
            touch_id,
            x,
            y,
            touch_type,
        );
        if let Err(e) = res {
            log_forward("weston_test.send_touch", &e);
        }
    }

    /// request compositor pause at a certain point
    ///
    /// Request that the compositor pauses execution at a certain point. When
    /// execution is paused, the compositor will signal the shared semaphore
    /// to the client.
    ///
    /// # Arguments
    ///
    /// - `breakpoint`: event type to wait for
    /// - `resource_id`: optional Wayland resource ID to filter for (type-specific)
    #[inline]
    fn handle_client_break(
        &mut self,
        slf: &Rc<WestonTest>,
        breakpoint: WestonTestBreakpoint,
        resource_id: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_client_break(
            breakpoint,
            resource_id,
        );
        if let Err(e) = res {
            log_forward("weston_test.client_break", &e);
        }
    }
}

impl ObjectPrivate for WestonTest {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WestonTest, version),
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
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: i32, arg2: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_test#{}.move_surface(surface: wl_surface#{}, x: {}, y: {})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_move_surface(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_move_surface(&self, arg0, arg1, arg2);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 28)));
                };
                let arg3 = arg3 as i32;
                let arg4 = arg4 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: i32, arg4: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_test#{}.move_pointer(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {}, x: {}, y: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4);
                }
                if let Some(handler) = handler {
                    (**handler).handle_move_pointer(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultHandler.handle_move_pointer(&self, arg0, arg1, arg2, arg3, arg4);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 28)));
                };
                let arg3 = arg3 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: i32, arg4: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_test#{}.send_button(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {}, button: {}, state: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4);
                }
                if let Some(handler) = handler {
                    (**handler).handle_send_button(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultHandler.handle_send_button(&self, arg0, arg1, arg2, arg3, arg4);
                }
            }
            3 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 28)));
                };
                let arg4 = Fixed::from_wire(arg4 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: Fixed) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_test#{}.send_axis(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {}, axis: {}, value: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4);
                }
                if let Some(handler) = handler {
                    (**handler).handle_send_axis(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultHandler.handle_send_axis(&self, arg0, arg1, arg2, arg3, arg4);
                }
            }
            4 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_test#{}.activate_surface(surface: wl_surface#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_activate_surface(&self, arg0);
                } else {
                    DefaultHandler.handle_activate_surface(&self, arg0);
                }
            }
            5 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 28)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_test#{}.send_key(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {}, key: {}, state: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4);
                }
                if let Some(handler) = handler {
                    (**handler).handle_send_key(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultHandler.handle_send_key(&self, arg0, arg1, arg2, arg3, arg4);
                }
            }
            6 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "device")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_test#{}.device_release(device: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_device_release(&self, arg0);
                } else {
                    DefaultHandler.handle_device_release(&self, arg0);
                }
            }
            7 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "device")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_test#{}.device_add(device: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_device_add(&self, arg0);
                } else {
                    DefaultHandler.handle_device_add(&self, arg0);
                }
            }
            8 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                    arg5,
                    arg6,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 36)));
                };
                let arg3 = arg3 as i32;
                let arg4 = Fixed::from_wire(arg4 as i32);
                let arg5 = Fixed::from_wire(arg5 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: i32, arg4: Fixed, arg5: Fixed, arg6: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_test#{}.send_touch(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {}, touch_id: {}, x: {}, y: {}, touch_type: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4, arg5, arg6);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4, arg5, arg6);
                }
                if let Some(handler) = handler {
                    (**handler).handle_send_touch(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6);
                } else {
                    DefaultHandler.handle_send_touch(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6);
                }
            }
            9 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = WestonTestBreakpoint(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: WestonTestBreakpoint, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_test#{}.client_break(breakpoint: {:?}, resource_id: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_client_break(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_client_break(&self, arg0, arg1);
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
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = Fixed::from_wire(arg0 as i32);
                let arg1 = Fixed::from_wire(arg1 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: Fixed, arg1: Fixed) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_test#{}.pointer_position(x: {}, y: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_pointer_position(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_pointer_position(&self, arg0, arg1);
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
            0 => "move_surface",
            1 => "move_pointer",
            2 => "send_button",
            3 => "send_axis",
            4 => "activate_surface",
            5 => "send_key",
            6 => "device_release",
            7 => "device_add",
            8 => "send_touch",
            9 => "client_break",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "pointer_position",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WestonTest {
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

impl WestonTest {
    /// Since when the error.touch_up_with_coordinate enum variant is available.
    pub const ENM__ERROR_TOUCH_UP_WITH_COORDINATE__SINCE: u32 = 1;

    /// Since when the breakpoint.post_repaint enum variant is available.
    pub const ENM__BREAKPOINT_POST_REPAINT__SINCE: u32 = 1;
    /// Since when the breakpoint.post_latch enum variant is available.
    pub const ENM__BREAKPOINT_POST_LATCH__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WestonTestError(pub u32);

impl WestonTestError {
    /// invalid coordinate
    pub const TOUCH_UP_WITH_COORDINATE: Self = Self(0);
}

impl Debug for WestonTestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::TOUCH_UP_WITH_COORDINATE => "TOUCH_UP_WITH_COORDINATE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WestonTestBreakpoint(pub u32);

impl WestonTestBreakpoint {
    /// after output repaint (filter type: wl_output)
    pub const POST_REPAINT: Self = Self(0);

    /// after output latch (filter type: wl_output)
    pub const POST_LATCH: Self = Self(1);
}

impl Debug for WestonTestBreakpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::POST_REPAINT => "POST_REPAINT",
            Self::POST_LATCH => "POST_LATCH",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
