//! a mapped toplevel
//!
//! A ext_foreign_toplevel_handle_v1 object represents a mapped toplevel
//! window. A single app may have multiple mapped toplevels.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A ext_foreign_toplevel_handle_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ExtForeignToplevelHandleV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ExtForeignToplevelHandleV1Handler>,
}

struct DefaultHandler;

impl ExtForeignToplevelHandleV1Handler for DefaultHandler { }

impl ConcreteObject for ExtForeignToplevelHandleV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ExtForeignToplevelHandleV1;
    const INTERFACE_NAME: &str = "ext_foreign_toplevel_handle_v1";
}

impl ExtForeignToplevelHandleV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ExtForeignToplevelHandleV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ExtForeignToplevelHandleV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ExtForeignToplevelHandleV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtForeignToplevelHandleV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ExtForeignToplevelHandleV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the ext_foreign_toplevel_handle_v1 object
    ///
    /// This request should be used when the client will no longer use the handle
    /// or after the closed event has been received to allow destruction of the
    /// object.
    ///
    /// When a handle is destroyed, a new handle may not be created by the server
    /// until the toplevel is unmapped and then remapped. Destroying a toplevel handle
    /// is not recommended unless the client is cleaning up child objects
    /// before destroying the ext_foreign_toplevel_list_v1 object, the toplevel
    /// was closed or the toplevel handle will not be used in the future.
    ///
    /// Other protocols which extend the ext_foreign_toplevel_handle_v1
    /// interface should require destructors for extension interfaces be
    /// called before allowing the toplevel handle to be destroyed.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_foreign_toplevel_handle_v1#{}.destroy()\n", id);
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

    /// destroy the ext_foreign_toplevel_handle_v1 object
    ///
    /// This request should be used when the client will no longer use the handle
    /// or after the closed event has been received to allow destruction of the
    /// object.
    ///
    /// When a handle is destroyed, a new handle may not be created by the server
    /// until the toplevel is unmapped and then remapped. Destroying a toplevel handle
    /// is not recommended unless the client is cleaning up child objects
    /// before destroying the ext_foreign_toplevel_list_v1 object, the toplevel
    /// was closed or the toplevel handle will not be used in the future.
    ///
    /// Other protocols which extend the ext_foreign_toplevel_handle_v1
    /// interface should require destructors for extension interfaces be
    /// called before allowing the toplevel handle to be destroyed.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("ext_foreign_toplevel_handle_v1.destroy", &e);
        }
    }

    /// Since when the closed message is available.
    pub const MSG__CLOSED__SINCE: u32 = 1;

    /// the toplevel has been closed
    ///
    /// The server will emit no further events on the ext_foreign_toplevel_handle_v1
    /// after this event. Any requests received aside from the destroy request must
    /// be ignored. Upon receiving this event, the client should destroy the handle.
    ///
    /// Other protocols which extend the ext_foreign_toplevel_handle_v1
    /// interface must also ignore requests other than destructors.
    #[inline]
    pub fn try_send_closed(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_foreign_toplevel_handle_v1#{}.closed()\n", client_id, id);
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

    /// the toplevel has been closed
    ///
    /// The server will emit no further events on the ext_foreign_toplevel_handle_v1
    /// after this event. Any requests received aside from the destroy request must
    /// be ignored. Upon receiving this event, the client should destroy the handle.
    ///
    /// Other protocols which extend the ext_foreign_toplevel_handle_v1
    /// interface must also ignore requests other than destructors.
    #[inline]
    pub fn send_closed(
        &self,
    ) {
        let res = self.try_send_closed(
        );
        if let Err(e) = res {
            log_send("ext_foreign_toplevel_handle_v1.closed", &e);
        }
    }

    /// Since when the done message is available.
    pub const MSG__DONE__SINCE: u32 = 1;

    /// all information about the toplevel has been sent
    ///
    /// This event is sent after all changes in the toplevel state have
    /// been sent.
    ///
    /// This allows changes to the ext_foreign_toplevel_handle_v1 properties
    /// to be atomically applied. Other protocols which extend the
    /// ext_foreign_toplevel_handle_v1 interface may use this event to also
    /// atomically apply any pending state.
    ///
    /// This event must not be sent after the ext_foreign_toplevel_handle_v1.closed
    /// event.
    #[inline]
    pub fn try_send_done(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_foreign_toplevel_handle_v1#{}.done()\n", client_id, id);
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
            1,
        ]);
        Ok(())
    }

    /// all information about the toplevel has been sent
    ///
    /// This event is sent after all changes in the toplevel state have
    /// been sent.
    ///
    /// This allows changes to the ext_foreign_toplevel_handle_v1 properties
    /// to be atomically applied. Other protocols which extend the
    /// ext_foreign_toplevel_handle_v1 interface may use this event to also
    /// atomically apply any pending state.
    ///
    /// This event must not be sent after the ext_foreign_toplevel_handle_v1.closed
    /// event.
    #[inline]
    pub fn send_done(
        &self,
    ) {
        let res = self.try_send_done(
        );
        if let Err(e) = res {
            log_send("ext_foreign_toplevel_handle_v1.done", &e);
        }
    }

    /// Since when the title message is available.
    pub const MSG__TITLE__SINCE: u32 = 1;

    /// title change
    ///
    /// The title of the toplevel has changed.
    ///
    /// The configured state must not be applied immediately. See
    /// ext_foreign_toplevel_handle_v1.done for details.
    ///
    /// # Arguments
    ///
    /// - `title`:
    #[inline]
    pub fn try_send_title(
        &self,
        title: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            title,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_foreign_toplevel_handle_v1#{}.title(title: {:?})\n", client_id, id, arg0);
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
            2,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// title change
    ///
    /// The title of the toplevel has changed.
    ///
    /// The configured state must not be applied immediately. See
    /// ext_foreign_toplevel_handle_v1.done for details.
    ///
    /// # Arguments
    ///
    /// - `title`:
    #[inline]
    pub fn send_title(
        &self,
        title: &str,
    ) {
        let res = self.try_send_title(
            title,
        );
        if let Err(e) = res {
            log_send("ext_foreign_toplevel_handle_v1.title", &e);
        }
    }

    /// Since when the app_id message is available.
    pub const MSG__APP_ID__SINCE: u32 = 1;

    /// app_id change
    ///
    /// The app id of the toplevel has changed.
    ///
    /// The configured state must not be applied immediately. See
    /// ext_foreign_toplevel_handle_v1.done for details.
    ///
    /// # Arguments
    ///
    /// - `app_id`:
    #[inline]
    pub fn try_send_app_id(
        &self,
        app_id: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            app_id,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_foreign_toplevel_handle_v1#{}.app_id(app_id: {:?})\n", client_id, id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// app_id change
    ///
    /// The app id of the toplevel has changed.
    ///
    /// The configured state must not be applied immediately. See
    /// ext_foreign_toplevel_handle_v1.done for details.
    ///
    /// # Arguments
    ///
    /// - `app_id`:
    #[inline]
    pub fn send_app_id(
        &self,
        app_id: &str,
    ) {
        let res = self.try_send_app_id(
            app_id,
        );
        if let Err(e) = res {
            log_send("ext_foreign_toplevel_handle_v1.app_id", &e);
        }
    }

    /// Since when the identifier message is available.
    pub const MSG__IDENTIFIER__SINCE: u32 = 1;

    /// a stable identifier for a toplevel
    ///
    /// This identifier is used to check if two or more toplevel handles belong
    /// to the same toplevel.
    ///
    /// The identifier is useful for command line tools or privileged clients
    /// which may need to reference an exact toplevel across processes or
    /// instances of the ext_foreign_toplevel_list_v1 global.
    ///
    /// The compositor must only send this event when the handle is created.
    ///
    /// The identifier must be unique per toplevel and it's handles. Two different
    /// toplevels must not have the same identifier. The identifier is only valid
    /// as long as the toplevel is mapped. If the toplevel is unmapped the identifier
    /// must not be reused. An identifier must not be reused by the compositor to
    /// ensure there are no races when sharing identifiers between processes.
    ///
    /// An identifier is a string that contains up to 32 printable ASCII bytes.
    /// An identifier must not be an empty string. It is recommended that a
    /// compositor includes an opaque generation value in identifiers. How the
    /// generation value is used when generating the identifier is implementation
    /// dependent.
    ///
    /// # Arguments
    ///
    /// - `identifier`:
    #[inline]
    pub fn try_send_identifier(
        &self,
        identifier: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            identifier,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_foreign_toplevel_handle_v1#{}.identifier(identifier: {:?})\n", client_id, id, arg0);
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
            4,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// a stable identifier for a toplevel
    ///
    /// This identifier is used to check if two or more toplevel handles belong
    /// to the same toplevel.
    ///
    /// The identifier is useful for command line tools or privileged clients
    /// which may need to reference an exact toplevel across processes or
    /// instances of the ext_foreign_toplevel_list_v1 global.
    ///
    /// The compositor must only send this event when the handle is created.
    ///
    /// The identifier must be unique per toplevel and it's handles. Two different
    /// toplevels must not have the same identifier. The identifier is only valid
    /// as long as the toplevel is mapped. If the toplevel is unmapped the identifier
    /// must not be reused. An identifier must not be reused by the compositor to
    /// ensure there are no races when sharing identifiers between processes.
    ///
    /// An identifier is a string that contains up to 32 printable ASCII bytes.
    /// An identifier must not be an empty string. It is recommended that a
    /// compositor includes an opaque generation value in identifiers. How the
    /// generation value is used when generating the identifier is implementation
    /// dependent.
    ///
    /// # Arguments
    ///
    /// - `identifier`:
    #[inline]
    pub fn send_identifier(
        &self,
        identifier: &str,
    ) {
        let res = self.try_send_identifier(
            identifier,
        );
        if let Err(e) = res {
            log_send("ext_foreign_toplevel_handle_v1.identifier", &e);
        }
    }
}

/// A message handler for [`ExtForeignToplevelHandleV1`] proxies.
pub trait ExtForeignToplevelHandleV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ExtForeignToplevelHandleV1>) {
        slf.core.delete_id();
    }

    /// destroy the ext_foreign_toplevel_handle_v1 object
    ///
    /// This request should be used when the client will no longer use the handle
    /// or after the closed event has been received to allow destruction of the
    /// object.
    ///
    /// When a handle is destroyed, a new handle may not be created by the server
    /// until the toplevel is unmapped and then remapped. Destroying a toplevel handle
    /// is not recommended unless the client is cleaning up child objects
    /// before destroying the ext_foreign_toplevel_list_v1 object, the toplevel
    /// was closed or the toplevel handle will not be used in the future.
    ///
    /// Other protocols which extend the ext_foreign_toplevel_handle_v1
    /// interface should require destructors for extension interfaces be
    /// called before allowing the toplevel handle to be destroyed.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ExtForeignToplevelHandleV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("ext_foreign_toplevel_handle_v1.destroy", &e);
        }
    }

    /// the toplevel has been closed
    ///
    /// The server will emit no further events on the ext_foreign_toplevel_handle_v1
    /// after this event. Any requests received aside from the destroy request must
    /// be ignored. Upon receiving this event, the client should destroy the handle.
    ///
    /// Other protocols which extend the ext_foreign_toplevel_handle_v1
    /// interface must also ignore requests other than destructors.
    #[inline]
    fn handle_closed(
        &mut self,
        slf: &Rc<ExtForeignToplevelHandleV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_closed(
        );
        if let Err(e) = res {
            log_forward("ext_foreign_toplevel_handle_v1.closed", &e);
        }
    }

    /// all information about the toplevel has been sent
    ///
    /// This event is sent after all changes in the toplevel state have
    /// been sent.
    ///
    /// This allows changes to the ext_foreign_toplevel_handle_v1 properties
    /// to be atomically applied. Other protocols which extend the
    /// ext_foreign_toplevel_handle_v1 interface may use this event to also
    /// atomically apply any pending state.
    ///
    /// This event must not be sent after the ext_foreign_toplevel_handle_v1.closed
    /// event.
    #[inline]
    fn handle_done(
        &mut self,
        slf: &Rc<ExtForeignToplevelHandleV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_done(
        );
        if let Err(e) = res {
            log_forward("ext_foreign_toplevel_handle_v1.done", &e);
        }
    }

    /// title change
    ///
    /// The title of the toplevel has changed.
    ///
    /// The configured state must not be applied immediately. See
    /// ext_foreign_toplevel_handle_v1.done for details.
    ///
    /// # Arguments
    ///
    /// - `title`:
    #[inline]
    fn handle_title(
        &mut self,
        slf: &Rc<ExtForeignToplevelHandleV1>,
        title: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_title(
            title,
        );
        if let Err(e) = res {
            log_forward("ext_foreign_toplevel_handle_v1.title", &e);
        }
    }

    /// app_id change
    ///
    /// The app id of the toplevel has changed.
    ///
    /// The configured state must not be applied immediately. See
    /// ext_foreign_toplevel_handle_v1.done for details.
    ///
    /// # Arguments
    ///
    /// - `app_id`:
    #[inline]
    fn handle_app_id(
        &mut self,
        slf: &Rc<ExtForeignToplevelHandleV1>,
        app_id: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_app_id(
            app_id,
        );
        if let Err(e) = res {
            log_forward("ext_foreign_toplevel_handle_v1.app_id", &e);
        }
    }

    /// a stable identifier for a toplevel
    ///
    /// This identifier is used to check if two or more toplevel handles belong
    /// to the same toplevel.
    ///
    /// The identifier is useful for command line tools or privileged clients
    /// which may need to reference an exact toplevel across processes or
    /// instances of the ext_foreign_toplevel_list_v1 global.
    ///
    /// The compositor must only send this event when the handle is created.
    ///
    /// The identifier must be unique per toplevel and it's handles. Two different
    /// toplevels must not have the same identifier. The identifier is only valid
    /// as long as the toplevel is mapped. If the toplevel is unmapped the identifier
    /// must not be reused. An identifier must not be reused by the compositor to
    /// ensure there are no races when sharing identifiers between processes.
    ///
    /// An identifier is a string that contains up to 32 printable ASCII bytes.
    /// An identifier must not be an empty string. It is recommended that a
    /// compositor includes an opaque generation value in identifiers. How the
    /// generation value is used when generating the identifier is implementation
    /// dependent.
    ///
    /// # Arguments
    ///
    /// - `identifier`:
    #[inline]
    fn handle_identifier(
        &mut self,
        slf: &Rc<ExtForeignToplevelHandleV1>,
        identifier: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_identifier(
            identifier,
        );
        if let Err(e) = res {
            log_forward("ext_foreign_toplevel_handle_v1.identifier", &e);
        }
    }
}

impl ObjectPrivate for ExtForeignToplevelHandleV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ExtForeignToplevelHandleV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_foreign_toplevel_handle_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_foreign_toplevel_handle_v1#{}.closed()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_closed(&self);
                } else {
                    DefaultHandler.handle_closed(&self);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_foreign_toplevel_handle_v1#{}.done()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_done(&self);
                } else {
                    DefaultHandler.handle_done(&self);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "title")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_foreign_toplevel_handle_v1#{}.title(title: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_title(&self, arg0);
                } else {
                    DefaultHandler.handle_title(&self, arg0);
                }
            }
            3 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "app_id")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_foreign_toplevel_handle_v1#{}.app_id(app_id: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_app_id(&self, arg0);
                } else {
                    DefaultHandler.handle_app_id(&self, arg0);
                }
            }
            4 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "identifier")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_foreign_toplevel_handle_v1#{}.identifier(identifier: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_identifier(&self, arg0);
                } else {
                    DefaultHandler.handle_identifier(&self, arg0);
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
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "closed",
            1 => "done",
            2 => "title",
            3 => "app_id",
            4 => "identifier",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ExtForeignToplevelHandleV1 {
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

