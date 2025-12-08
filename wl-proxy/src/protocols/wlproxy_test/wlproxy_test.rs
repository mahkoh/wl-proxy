use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wlproxy_test object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlproxyTest {
    core: ObjectCore,
    handler: HandlerHolder<dyn WlproxyTestHandler>,
}

struct DefaultHandler;

impl WlproxyTestHandler for DefaultHandler { }

impl ConcreteObject for WlproxyTest {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WlproxyTest;
    const INTERFACE_NAME: &str = "wlproxy_test";
}

impl WlproxyTest {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WlproxyTestHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WlproxyTestHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlproxyTest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlproxyTest")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlproxyTest {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    #[inline]
    pub fn try_send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wlproxy_test#{}.destroy()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
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
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wlproxy_test.destroy", &e);
        }
    }

    /// Since when the recv_fd message is available.
    pub const MSG__RECV_FD__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `fd`:
    #[inline]
    pub fn try_send_recv_fd(
        &self,
        fd: &Rc<OwnedFd>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            fd,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wlproxy_test#{}.recv_fd(fd: {})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0.as_raw_fd());
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
        fmt.fds.push_back(arg0.clone());
        fmt.words([
            id,
            1,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `fd`:
    #[inline]
    pub fn send_recv_fd(
        &self,
        fd: &Rc<OwnedFd>,
    ) {
        let res = self.try_send_recv_fd(
            fd,
        );
        if let Err(e) = res {
            log_send("wlproxy_test.recv_fd", &e);
        }
    }

    /// Since when the echo_array message is available.
    pub const MSG__ECHO_ARRAY__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `echo`:
    /// - `array`:
    #[inline]
    pub fn try_send_echo_array(
        &self,
        echo: &Rc<WlproxyTestArrayEcho>,
        array: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            echo,
            array,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("echo", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: &[u8]) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wlproxy_test#{}.echo_array(echo: wlproxy_test_array_echo#{}, array: {})\n", id, arg0, debug_array(arg1));
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
        ]);
        fmt.array(arg1);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `echo`:
    /// - `array`:
    #[inline]
    pub fn send_echo_array(
        &self,
        echo: &Rc<WlproxyTestArrayEcho>,
        array: &[u8],
    ) {
        let res = self.try_send_echo_array(
            echo,
            array,
        );
        if let Err(e) = res {
            log_send("wlproxy_test.echo_array", &e);
        }
    }

    /// # Arguments
    ///
    /// - `array`:
    #[inline]
    pub fn new_try_send_echo_array(
        &self,
        array: &[u8],
    ) -> Result<Rc<WlproxyTestArrayEcho>, ObjectError> {
        let echo = self.core.create_child();
        self.try_send_echo_array(
            &echo,
            array,
        )?;
        Ok(echo)
    }

    /// # Arguments
    ///
    /// - `array`:
    #[inline]
    pub fn new_send_echo_array(
        &self,
        array: &[u8],
    ) -> Rc<WlproxyTestArrayEcho> {
        let echo = self.core.create_child();
        self.send_echo_array(
            &echo,
            array,
        );
        echo
    }

    /// Since when the echo_fd message is available.
    pub const MSG__ECHO_FD__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `echo`:
    /// - `fd1`:
    /// - `fd2`:
    #[inline]
    pub fn try_send_echo_fd(
        &self,
        echo: &Rc<WlproxyTestFdEcho>,
        fd1: &Rc<OwnedFd>,
        fd2: &Rc<OwnedFd>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            echo,
            fd1,
            fd2,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("echo", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: i32, arg2: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wlproxy_test#{}.echo_fd(echo: wlproxy_test_fd_echo#{}, fd1: {}, fd2: {})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1.as_raw_fd(), arg2.as_raw_fd());
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
        fmt.fds.push_back(arg2.clone());
        fmt.words([
            id,
            3,
            arg0_id,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `echo`:
    /// - `fd1`:
    /// - `fd2`:
    #[inline]
    pub fn send_echo_fd(
        &self,
        echo: &Rc<WlproxyTestFdEcho>,
        fd1: &Rc<OwnedFd>,
        fd2: &Rc<OwnedFd>,
    ) {
        let res = self.try_send_echo_fd(
            echo,
            fd1,
            fd2,
        );
        if let Err(e) = res {
            log_send("wlproxy_test.echo_fd", &e);
        }
    }

    /// # Arguments
    ///
    /// - `fd1`:
    /// - `fd2`:
    #[inline]
    pub fn new_try_send_echo_fd(
        &self,
        fd1: &Rc<OwnedFd>,
        fd2: &Rc<OwnedFd>,
    ) -> Result<Rc<WlproxyTestFdEcho>, ObjectError> {
        let echo = self.core.create_child();
        self.try_send_echo_fd(
            &echo,
            fd1,
            fd2,
        )?;
        Ok(echo)
    }

    /// # Arguments
    ///
    /// - `fd1`:
    /// - `fd2`:
    #[inline]
    pub fn new_send_echo_fd(
        &self,
        fd1: &Rc<OwnedFd>,
        fd2: &Rc<OwnedFd>,
    ) -> Rc<WlproxyTestFdEcho> {
        let echo = self.core.create_child();
        self.send_echo_fd(
            &echo,
            fd1,
            fd2,
        );
        echo
    }

    /// Since when the send_many_events message is available.
    pub const MSG__SEND_MANY_EVENTS__SINCE: u32 = 1;

    #[inline]
    pub fn try_send_send_many_events(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wlproxy_test#{}.send_many_events()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
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
        ]);
        Ok(())
    }

    #[inline]
    pub fn send_send_many_events(
        &self,
    ) {
        let res = self.try_send_send_many_events(
        );
        if let Err(e) = res {
            log_send("wlproxy_test.send_many_events", &e);
        }
    }

    /// Since when the many_event message is available.
    pub const MSG__MANY_EVENT__SINCE: u32 = 1;

    #[inline]
    pub fn try_send_many_event(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wlproxy_test#{}.many_event()\n", client_id, id);
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
            0,
        ]);
        Ok(())
    }

    #[inline]
    pub fn send_many_event(
        &self,
    ) {
        let res = self.try_send_many_event(
        );
        if let Err(e) = res {
            log_send("wlproxy_test.many_event", &e);
        }
    }

    /// Since when the count_hops message is available.
    pub const MSG__COUNT_HOPS__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_count_hops(
        &self,
        id: &Rc<WlproxyTestHops>,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wlproxy_test#{}.count_hops(id: wlproxy_test_hops#{})\n", id, arg0);
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
            5,
            arg0_id,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_count_hops(
        &self,
        id: &Rc<WlproxyTestHops>,
    ) {
        let res = self.try_send_count_hops(
            id,
        );
        if let Err(e) = res {
            log_send("wlproxy_test.count_hops", &e);
        }
    }

    #[inline]
    pub fn new_try_send_count_hops(
        &self,
    ) -> Result<Rc<WlproxyTestHops>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_count_hops(
            &id,
        )?;
        Ok(id)
    }

    #[inline]
    pub fn new_send_count_hops(
        &self,
    ) -> Rc<WlproxyTestHops> {
        let id = self.core.create_child();
        self.send_count_hops(
            &id,
        );
        id
    }

    /// Since when the create_dummy message is available.
    pub const MSG__CREATE_DUMMY__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_create_dummy(
        &self,
        id: &Rc<WlproxyTestDummy>,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wlproxy_test#{}.create_dummy(id: wlproxy_test_dummy#{})\n", id, arg0);
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
            6,
            arg0_id,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_create_dummy(
        &self,
        id: &Rc<WlproxyTestDummy>,
    ) {
        let res = self.try_send_create_dummy(
            id,
        );
        if let Err(e) = res {
            log_send("wlproxy_test.create_dummy", &e);
        }
    }

    #[inline]
    pub fn new_try_send_create_dummy(
        &self,
    ) -> Result<Rc<WlproxyTestDummy>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_create_dummy(
            &id,
        )?;
        Ok(id)
    }

    #[inline]
    pub fn new_send_create_dummy(
        &self,
    ) -> Rc<WlproxyTestDummy> {
        let id = self.core.create_child();
        self.send_create_dummy(
            &id,
        );
        id
    }

    /// Since when the echo_object message is available.
    pub const MSG__ECHO_OBJECT__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `echo`:
    /// - `object`:
    #[inline]
    pub fn try_send_echo_object(
        &self,
        echo: &Rc<WlproxyTestObjectEcho>,
        object: Rc<dyn Object>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            echo,
            object,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("object"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("echo", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wlproxy_test#{}.echo_object(echo: wlproxy_test_object_echo#{}, object: unknown#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id);
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
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `echo`:
    /// - `object`:
    #[inline]
    pub fn send_echo_object(
        &self,
        echo: &Rc<WlproxyTestObjectEcho>,
        object: Rc<dyn Object>,
    ) {
        let res = self.try_send_echo_object(
            echo,
            object,
        );
        if let Err(e) = res {
            log_send("wlproxy_test.echo_object", &e);
        }
    }

    /// # Arguments
    ///
    /// - `object`:
    #[inline]
    pub fn new_try_send_echo_object(
        &self,
        object: Rc<dyn Object>,
    ) -> Result<Rc<WlproxyTestObjectEcho>, ObjectError> {
        let echo = self.core.create_child();
        self.try_send_echo_object(
            &echo,
            object,
        )?;
        Ok(echo)
    }

    /// # Arguments
    ///
    /// - `object`:
    #[inline]
    pub fn new_send_echo_object(
        &self,
        object: Rc<dyn Object>,
    ) -> Rc<WlproxyTestObjectEcho> {
        let echo = self.core.create_child();
        self.send_echo_object(
            &echo,
            object,
        );
        echo
    }

    /// Since when the send_object message is available.
    pub const MSG__SEND_OBJECT__SINCE: u32 = 1;

    #[inline]
    pub fn try_send_send_object(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wlproxy_test#{}.send_object()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
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
        ]);
        Ok(())
    }

    #[inline]
    pub fn send_send_object(
        &self,
    ) {
        let res = self.try_send_send_object(
        );
        if let Err(e) = res {
            log_send("wlproxy_test.send_object", &e);
        }
    }

    /// Since when the sent_object message is available.
    pub const MSG__SENT_OBJECT__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `echo`:
    #[inline]
    pub fn try_send_sent_object(
        &self,
        echo: &Rc<WlproxyTestServerSent>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            echo,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateClientId("echo", e)))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wlproxy_test#{}.sent_object(echo: wlproxy_test_server_sent#{})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `echo`:
    #[inline]
    pub fn send_sent_object(
        &self,
        echo: &Rc<WlproxyTestServerSent>,
    ) {
        let res = self.try_send_sent_object(
            echo,
        );
        if let Err(e) = res {
            log_send("wlproxy_test.sent_object", &e);
        }
    }

    #[inline]
    pub fn new_try_send_sent_object(
        &self,
    ) -> Result<Rc<WlproxyTestServerSent>, ObjectError> {
        let echo = self.core.create_child();
        self.try_send_sent_object(
            &echo,
        )?;
        Ok(echo)
    }

    #[inline]
    pub fn new_send_sent_object(
        &self,
    ) -> Rc<WlproxyTestServerSent> {
        let echo = self.core.create_child();
        self.send_sent_object(
            &echo,
        );
        echo
    }

    /// Since when the create_non_forward message is available.
    pub const MSG__CREATE_NON_FORWARD__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_create_non_forward(
        &self,
        id: &Rc<WlproxyTestNonForward>,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wlproxy_test#{}.create_non_forward(id: wlproxy_test_non_forward#{})\n", id, arg0);
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
            9,
            arg0_id,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_create_non_forward(
        &self,
        id: &Rc<WlproxyTestNonForward>,
    ) {
        let res = self.try_send_create_non_forward(
            id,
        );
        if let Err(e) = res {
            log_send("wlproxy_test.create_non_forward", &e);
        }
    }

    #[inline]
    pub fn new_try_send_create_non_forward(
        &self,
    ) -> Result<Rc<WlproxyTestNonForward>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_create_non_forward(
            &id,
        )?;
        Ok(id)
    }

    #[inline]
    pub fn new_send_create_non_forward(
        &self,
    ) -> Rc<WlproxyTestNonForward> {
        let id = self.core.create_child();
        self.send_create_non_forward(
            &id,
        );
        id
    }
}

/// A message handler for [`WlproxyTest`] proxies.
pub trait WlproxyTestHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WlproxyTest>) {
        slf.core.delete_id();
    }

    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WlproxyTest>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wlproxy_test.destroy", &e);
        }
    }

    /// # Arguments
    ///
    /// - `fd`:
    #[inline]
    fn handle_recv_fd(
        &mut self,
        slf: &Rc<WlproxyTest>,
        fd: &Rc<OwnedFd>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_recv_fd(
            fd,
        );
        if let Err(e) = res {
            log_forward("wlproxy_test.recv_fd", &e);
        }
    }

    /// # Arguments
    ///
    /// - `echo`:
    /// - `array`:
    #[inline]
    fn handle_echo_array(
        &mut self,
        slf: &Rc<WlproxyTest>,
        echo: &Rc<WlproxyTestArrayEcho>,
        array: &[u8],
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_echo_array(
            echo,
            array,
        );
        if let Err(e) = res {
            log_forward("wlproxy_test.echo_array", &e);
        }
    }

    /// # Arguments
    ///
    /// - `echo`:
    /// - `fd1`:
    /// - `fd2`:
    #[inline]
    fn handle_echo_fd(
        &mut self,
        slf: &Rc<WlproxyTest>,
        echo: &Rc<WlproxyTestFdEcho>,
        fd1: &Rc<OwnedFd>,
        fd2: &Rc<OwnedFd>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_echo_fd(
            echo,
            fd1,
            fd2,
        );
        if let Err(e) = res {
            log_forward("wlproxy_test.echo_fd", &e);
        }
    }

    #[inline]
    fn handle_send_many_events(
        &mut self,
        slf: &Rc<WlproxyTest>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_send_many_events(
        );
        if let Err(e) = res {
            log_forward("wlproxy_test.send_many_events", &e);
        }
    }

    #[inline]
    fn handle_many_event(
        &mut self,
        slf: &Rc<WlproxyTest>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_many_event(
        );
        if let Err(e) = res {
            log_forward("wlproxy_test.many_event", &e);
        }
    }

    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn handle_count_hops(
        &mut self,
        slf: &Rc<WlproxyTest>,
        id: &Rc<WlproxyTestHops>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_count_hops(
            id,
        );
        if let Err(e) = res {
            log_forward("wlproxy_test.count_hops", &e);
        }
    }

    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn handle_create_dummy(
        &mut self,
        slf: &Rc<WlproxyTest>,
        id: &Rc<WlproxyTestDummy>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_dummy(
            id,
        );
        if let Err(e) = res {
            log_forward("wlproxy_test.create_dummy", &e);
        }
    }

    /// # Arguments
    ///
    /// - `echo`:
    /// - `object`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_echo_object(
        &mut self,
        slf: &Rc<WlproxyTest>,
        echo: &Rc<WlproxyTestObjectEcho>,
        object: Rc<dyn Object>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_echo_object(
            echo,
            object,
        );
        if let Err(e) = res {
            log_forward("wlproxy_test.echo_object", &e);
        }
    }

    #[inline]
    fn handle_send_object(
        &mut self,
        slf: &Rc<WlproxyTest>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_send_object(
        );
        if let Err(e) = res {
            log_forward("wlproxy_test.send_object", &e);
        }
    }

    /// # Arguments
    ///
    /// - `echo`:
    #[inline]
    fn handle_sent_object(
        &mut self,
        slf: &Rc<WlproxyTest>,
        echo: &Rc<WlproxyTestServerSent>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_sent_object(
            echo,
        );
        if let Err(e) = res {
            log_forward("wlproxy_test.sent_object", &e);
        }
    }

    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn handle_create_non_forward(
        &mut self,
        slf: &Rc<WlproxyTest>,
        id: &Rc<WlproxyTestNonForward>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_non_forward(
            id,
        );
        if let Err(e) = res {
            log_forward("wlproxy_test.create_non_forward", &e);
        }
    }
}

impl ObjectPrivate for WlproxyTest {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WlproxyTest, version),
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
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wlproxy_test#{}.destroy()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_destroy(&self);
                } else {
                    DefaultHandler.handle_destroy(&self);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                let Some(arg0) = fds.pop_front() else {
                    return Err(ObjectError(ObjectErrorKind::MissingFd("fd")));
                };
                let arg0 = &arg0;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wlproxy_test#{}.recv_fd(fd: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0.as_raw_fd());
                }
                if let Some(handler) = handler {
                    (**handler).handle_recv_fd(&self, arg0);
                } else {
                    DefaultHandler.handle_recv_fd(&self, arg0);
                }
            }
            2 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("echo")));
                };
                offset += 1;
                let arg1;
                (arg1, offset) = parse_array(msg, offset, "array")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wlproxy_test#{}.echo_array(echo: wlproxy_test_array_echo#{}, array: {})\n", client_id, id, arg0, debug_array(arg1));
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = WlproxyTestArrayEcho::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "echo", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_echo_array(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_echo_array(&self, arg0, arg1);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let Some(arg1) = fds.pop_front() else {
                    return Err(ObjectError(ObjectErrorKind::MissingFd("fd1")));
                };
                let Some(arg2) = fds.pop_front() else {
                    return Err(ObjectError(ObjectErrorKind::MissingFd("fd2")));
                };
                let arg1 = &arg1;
                let arg2 = &arg2;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: i32, arg2: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wlproxy_test#{}.echo_fd(echo: wlproxy_test_fd_echo#{}, fd1: {}, fd2: {})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1.as_raw_fd(), arg2.as_raw_fd());
                }
                let arg0_id = arg0;
                let arg0 = WlproxyTestFdEcho::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "echo", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_echo_fd(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_echo_fd(&self, arg0, arg1, arg2);
                }
            }
            4 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wlproxy_test#{}.send_many_events()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_send_many_events(&self);
                } else {
                    DefaultHandler.handle_send_many_events(&self);
                }
            }
            5 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wlproxy_test#{}.count_hops(id: wlproxy_test_hops#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WlproxyTestHops::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_count_hops(&self, arg0);
                } else {
                    DefaultHandler.handle_count_hops(&self, arg0);
                }
            }
            6 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wlproxy_test#{}.create_dummy(id: wlproxy_test_dummy#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WlproxyTestDummy::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_dummy(&self, arg0);
                } else {
                    DefaultHandler.handle_create_dummy(&self, arg0);
                }
            }
            7 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wlproxy_test#{}.echo_object(echo: wlproxy_test_object_echo#{}, object: unknown#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = WlproxyTestObjectEcho::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "echo", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_echo_object(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_echo_object(&self, arg0, arg1);
                }
            }
            8 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wlproxy_test#{}.send_object()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_send_object(&self);
                } else {
                    DefaultHandler.handle_send_object(&self);
                }
            }
            9 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wlproxy_test#{}.create_non_forward(id: wlproxy_test_non_forward#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WlproxyTestNonForward::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_non_forward(&self, arg0);
                } else {
                    DefaultHandler.handle_create_non_forward(&self, arg0);
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
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wlproxy_test#{}.many_event()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_many_event(&self);
                } else {
                    DefaultHandler.handle_many_event(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wlproxy_test#{}.sent_object(echo: wlproxy_test_server_sent#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WlproxyTestServerSent::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetServerId(arg0_id, "echo", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_sent_object(&self, arg0);
                } else {
                    DefaultHandler.handle_sent_object(&self, arg0);
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
            0 => "destroy",
            1 => "recv_fd",
            2 => "echo_array",
            3 => "echo_fd",
            4 => "send_many_events",
            5 => "count_hops",
            6 => "create_dummy",
            7 => "echo_object",
            8 => "send_object",
            9 => "create_non_forward",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "many_event",
            1 => "sent_object",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WlproxyTest {
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

