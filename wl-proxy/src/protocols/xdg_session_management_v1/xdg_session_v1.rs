//! A session for an application
//!
//! A xdg_session_v1 object represents a session for an application. While the
//! object exists, all surfaces which have been added to the session will
//! have states stored by the compositor which can be reapplied at a later
//! time. Two sessions cannot exist for the same identifier string.
//!
//! States for surfaces added to a session are automatically updated by the
//! compositor when they are changed.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A xdg_session_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XdgSessionV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn XdgSessionV1Handler>,
}

struct DefaultHandler;

impl XdgSessionV1Handler for DefaultHandler { }

impl ConcreteObject for XdgSessionV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::XdgSessionV1;
    const INTERFACE_NAME: &str = "xdg_session_v1";
}

impl XdgSessionV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl XdgSessionV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn XdgSessionV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XdgSessionV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XdgSessionV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XdgSessionV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// Destroy the session
    ///
    /// Destroy a session object, preserving the current state but not continuing
    /// to make further updates if state changes occur. This makes the associated
    /// xdg_toplevel_session_v1 objects inert.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_session_v1#{}.destroy()\n", id);
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

    /// Destroy the session
    ///
    /// Destroy a session object, preserving the current state but not continuing
    /// to make further updates if state changes occur. This makes the associated
    /// xdg_toplevel_session_v1 objects inert.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("xdg_session_v1.destroy", &e);
        }
    }

    /// Since when the remove message is available.
    pub const MSG__REMOVE__SINCE: u32 = 1;

    /// Remove the session
    ///
    /// Remove the session, making it no longer available for restoration. A
    /// compositor should in response to this request remove the data related to
    /// this session from its storage.
    #[inline]
    pub fn try_send_remove(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_session_v1#{}.remove()\n", id);
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
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Remove the session
    ///
    /// Remove the session, making it no longer available for restoration. A
    /// compositor should in response to this request remove the data related to
    /// this session from its storage.
    #[inline]
    pub fn send_remove(
        &self,
    ) {
        let res = self.try_send_remove(
        );
        if let Err(e) = res {
            log_send("xdg_session_v1.remove", &e);
        }
    }

    /// Since when the add_toplevel message is available.
    pub const MSG__ADD_TOPLEVEL__SINCE: u32 = 1;

    /// add a new surface to the session
    ///
    /// Attempt to add a given surface to the session. The passed name is used
    /// to identify what window is being restored, and may be used to store
    /// window specific state within the session.
    ///
    /// The name given to the toplevel must not correspond to any previously
    /// existing toplevel names in the session. If the name matches an already
    /// known toplevel name in the session, a 'name_in_use' protocol error will
    /// be raised.
    ///
    /// The toplevel object must not be added more than once to any session
    /// created by the client, otherwise the 'already_added' protocol error
    /// will be raised.
    ///
    /// This request will return a xdg_toplevel_session_v1 for later
    /// manipulation. As this resource is created from an empty initial state,
    /// compositors must not emit a xdg_toplevel_session_v1.restored event for
    /// resources created through this request.
    ///
    /// The name string must be UTF-8 encoded. It is also limited by the maximum
    /// length of wayland messages (around 4KB). The 'invalid_name' protocol
    /// error will be raised if an invalid string is provided.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `toplevel`:
    /// - `name`: name identifying the toplevel
    #[inline]
    pub fn try_send_add_toplevel(
        &self,
        id: &Rc<XdgToplevelSessionV1>,
        toplevel: &Rc<XdgToplevel>,
        name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            id,
            toplevel,
            name,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("toplevel"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_session_v1#{}.add_toplevel(id: xdg_toplevel_session_v1#{}, toplevel: xdg_toplevel#{}, name: {:?})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id, arg2);
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
            arg1_id,
        ]);
        fmt.string(arg2);
        Ok(())
    }

    /// add a new surface to the session
    ///
    /// Attempt to add a given surface to the session. The passed name is used
    /// to identify what window is being restored, and may be used to store
    /// window specific state within the session.
    ///
    /// The name given to the toplevel must not correspond to any previously
    /// existing toplevel names in the session. If the name matches an already
    /// known toplevel name in the session, a 'name_in_use' protocol error will
    /// be raised.
    ///
    /// The toplevel object must not be added more than once to any session
    /// created by the client, otherwise the 'already_added' protocol error
    /// will be raised.
    ///
    /// This request will return a xdg_toplevel_session_v1 for later
    /// manipulation. As this resource is created from an empty initial state,
    /// compositors must not emit a xdg_toplevel_session_v1.restored event for
    /// resources created through this request.
    ///
    /// The name string must be UTF-8 encoded. It is also limited by the maximum
    /// length of wayland messages (around 4KB). The 'invalid_name' protocol
    /// error will be raised if an invalid string is provided.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `toplevel`:
    /// - `name`: name identifying the toplevel
    #[inline]
    pub fn send_add_toplevel(
        &self,
        id: &Rc<XdgToplevelSessionV1>,
        toplevel: &Rc<XdgToplevel>,
        name: &str,
    ) {
        let res = self.try_send_add_toplevel(
            id,
            toplevel,
            name,
        );
        if let Err(e) = res {
            log_send("xdg_session_v1.add_toplevel", &e);
        }
    }

    /// add a new surface to the session
    ///
    /// Attempt to add a given surface to the session. The passed name is used
    /// to identify what window is being restored, and may be used to store
    /// window specific state within the session.
    ///
    /// The name given to the toplevel must not correspond to any previously
    /// existing toplevel names in the session. If the name matches an already
    /// known toplevel name in the session, a 'name_in_use' protocol error will
    /// be raised.
    ///
    /// The toplevel object must not be added more than once to any session
    /// created by the client, otherwise the 'already_added' protocol error
    /// will be raised.
    ///
    /// This request will return a xdg_toplevel_session_v1 for later
    /// manipulation. As this resource is created from an empty initial state,
    /// compositors must not emit a xdg_toplevel_session_v1.restored event for
    /// resources created through this request.
    ///
    /// The name string must be UTF-8 encoded. It is also limited by the maximum
    /// length of wayland messages (around 4KB). The 'invalid_name' protocol
    /// error will be raised if an invalid string is provided.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    /// - `name`: name identifying the toplevel
    #[inline]
    pub fn new_try_send_add_toplevel(
        &self,
        toplevel: &Rc<XdgToplevel>,
        name: &str,
    ) -> Result<Rc<XdgToplevelSessionV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_add_toplevel(
            &id,
            toplevel,
            name,
        )?;
        Ok(id)
    }

    /// add a new surface to the session
    ///
    /// Attempt to add a given surface to the session. The passed name is used
    /// to identify what window is being restored, and may be used to store
    /// window specific state within the session.
    ///
    /// The name given to the toplevel must not correspond to any previously
    /// existing toplevel names in the session. If the name matches an already
    /// known toplevel name in the session, a 'name_in_use' protocol error will
    /// be raised.
    ///
    /// The toplevel object must not be added more than once to any session
    /// created by the client, otherwise the 'already_added' protocol error
    /// will be raised.
    ///
    /// This request will return a xdg_toplevel_session_v1 for later
    /// manipulation. As this resource is created from an empty initial state,
    /// compositors must not emit a xdg_toplevel_session_v1.restored event for
    /// resources created through this request.
    ///
    /// The name string must be UTF-8 encoded. It is also limited by the maximum
    /// length of wayland messages (around 4KB). The 'invalid_name' protocol
    /// error will be raised if an invalid string is provided.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    /// - `name`: name identifying the toplevel
    #[inline]
    pub fn new_send_add_toplevel(
        &self,
        toplevel: &Rc<XdgToplevel>,
        name: &str,
    ) -> Rc<XdgToplevelSessionV1> {
        let id = self.core.create_child();
        self.send_add_toplevel(
            &id,
            toplevel,
            name,
        );
        id
    }

    /// Since when the restore_toplevel message is available.
    pub const MSG__RESTORE_TOPLEVEL__SINCE: u32 = 1;

    /// restore a surface state
    ///
    /// Inform the compositor that the toplevel associated with the passed name
    /// should have its window management state restored.
    ///
    /// If the toplevel name was previously granted to another xdg_toplevel,
    /// the 'name_in_use' protocol error will be raised.
    ///
    /// The toplevel object must not be added more than once to any session
    /// created by the client, otherwise the 'already_added' protocol error
    /// will be raised.
    ///
    /// This request must be called prior to the first commit on the associated
    /// wl_surface after creating the toplevel, otherwise an 'already_mapped'
    /// error is raised.
    ///
    /// As part of the initial configure sequence, if the toplevel was
    /// successfully restored, a xdg_toplevel_session_v1.restored event is
    /// emitted. If the toplevel name was not known in the session, this request
    /// will be equivalent to the xdg_toplevel_session_v1.add_toplevel request,
    /// and no such event will be emitted. See the xdg_toplevel_session_v1.restored
    /// event for further details.
    ///
    /// The name string must be UTF-8 encoded. It is also limited by the maximum
    /// length of wayland messages (around 4KB). The 'invalid_name' protocol
    /// error will be raised if an invalid string is provided.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `toplevel`:
    /// - `name`: name identifying the toplevel
    #[inline]
    pub fn try_send_restore_toplevel(
        &self,
        id: &Rc<XdgToplevelSessionV1>,
        toplevel: &Rc<XdgToplevel>,
        name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            id,
            toplevel,
            name,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("toplevel"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_session_v1#{}.restore_toplevel(id: xdg_toplevel_session_v1#{}, toplevel: xdg_toplevel#{}, name: {:?})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id, arg2);
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
            arg0_id,
            arg1_id,
        ]);
        fmt.string(arg2);
        Ok(())
    }

    /// restore a surface state
    ///
    /// Inform the compositor that the toplevel associated with the passed name
    /// should have its window management state restored.
    ///
    /// If the toplevel name was previously granted to another xdg_toplevel,
    /// the 'name_in_use' protocol error will be raised.
    ///
    /// The toplevel object must not be added more than once to any session
    /// created by the client, otherwise the 'already_added' protocol error
    /// will be raised.
    ///
    /// This request must be called prior to the first commit on the associated
    /// wl_surface after creating the toplevel, otherwise an 'already_mapped'
    /// error is raised.
    ///
    /// As part of the initial configure sequence, if the toplevel was
    /// successfully restored, a xdg_toplevel_session_v1.restored event is
    /// emitted. If the toplevel name was not known in the session, this request
    /// will be equivalent to the xdg_toplevel_session_v1.add_toplevel request,
    /// and no such event will be emitted. See the xdg_toplevel_session_v1.restored
    /// event for further details.
    ///
    /// The name string must be UTF-8 encoded. It is also limited by the maximum
    /// length of wayland messages (around 4KB). The 'invalid_name' protocol
    /// error will be raised if an invalid string is provided.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `toplevel`:
    /// - `name`: name identifying the toplevel
    #[inline]
    pub fn send_restore_toplevel(
        &self,
        id: &Rc<XdgToplevelSessionV1>,
        toplevel: &Rc<XdgToplevel>,
        name: &str,
    ) {
        let res = self.try_send_restore_toplevel(
            id,
            toplevel,
            name,
        );
        if let Err(e) = res {
            log_send("xdg_session_v1.restore_toplevel", &e);
        }
    }

    /// restore a surface state
    ///
    /// Inform the compositor that the toplevel associated with the passed name
    /// should have its window management state restored.
    ///
    /// If the toplevel name was previously granted to another xdg_toplevel,
    /// the 'name_in_use' protocol error will be raised.
    ///
    /// The toplevel object must not be added more than once to any session
    /// created by the client, otherwise the 'already_added' protocol error
    /// will be raised.
    ///
    /// This request must be called prior to the first commit on the associated
    /// wl_surface after creating the toplevel, otherwise an 'already_mapped'
    /// error is raised.
    ///
    /// As part of the initial configure sequence, if the toplevel was
    /// successfully restored, a xdg_toplevel_session_v1.restored event is
    /// emitted. If the toplevel name was not known in the session, this request
    /// will be equivalent to the xdg_toplevel_session_v1.add_toplevel request,
    /// and no such event will be emitted. See the xdg_toplevel_session_v1.restored
    /// event for further details.
    ///
    /// The name string must be UTF-8 encoded. It is also limited by the maximum
    /// length of wayland messages (around 4KB). The 'invalid_name' protocol
    /// error will be raised if an invalid string is provided.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    /// - `name`: name identifying the toplevel
    #[inline]
    pub fn new_try_send_restore_toplevel(
        &self,
        toplevel: &Rc<XdgToplevel>,
        name: &str,
    ) -> Result<Rc<XdgToplevelSessionV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_restore_toplevel(
            &id,
            toplevel,
            name,
        )?;
        Ok(id)
    }

    /// restore a surface state
    ///
    /// Inform the compositor that the toplevel associated with the passed name
    /// should have its window management state restored.
    ///
    /// If the toplevel name was previously granted to another xdg_toplevel,
    /// the 'name_in_use' protocol error will be raised.
    ///
    /// The toplevel object must not be added more than once to any session
    /// created by the client, otherwise the 'already_added' protocol error
    /// will be raised.
    ///
    /// This request must be called prior to the first commit on the associated
    /// wl_surface after creating the toplevel, otherwise an 'already_mapped'
    /// error is raised.
    ///
    /// As part of the initial configure sequence, if the toplevel was
    /// successfully restored, a xdg_toplevel_session_v1.restored event is
    /// emitted. If the toplevel name was not known in the session, this request
    /// will be equivalent to the xdg_toplevel_session_v1.add_toplevel request,
    /// and no such event will be emitted. See the xdg_toplevel_session_v1.restored
    /// event for further details.
    ///
    /// The name string must be UTF-8 encoded. It is also limited by the maximum
    /// length of wayland messages (around 4KB). The 'invalid_name' protocol
    /// error will be raised if an invalid string is provided.
    ///
    /// # Arguments
    ///
    /// - `toplevel`:
    /// - `name`: name identifying the toplevel
    #[inline]
    pub fn new_send_restore_toplevel(
        &self,
        toplevel: &Rc<XdgToplevel>,
        name: &str,
    ) -> Rc<XdgToplevelSessionV1> {
        let id = self.core.create_child();
        self.send_restore_toplevel(
            &id,
            toplevel,
            name,
        );
        id
    }

    /// Since when the remove_toplevel message is available.
    pub const MSG__REMOVE_TOPLEVEL__SINCE: u32 = 1;

    /// remove a surface from the session
    ///
    /// Remove a specified surface from the session and render any related
    /// xdg_toplevel_session_v1 object inert. The compositor should remove any
    /// data related to the toplevel in the corresponding session from its internal
    /// storage.
    ///
    /// The window is specified by its name in the session. The name string
    /// must be encoded in UTF-8, and it is limited in size by the maximum
    /// length of wayland messages (around 4KB).
    ///
    /// # Arguments
    ///
    /// - `name`: name identifying the toplevel
    #[inline]
    pub fn try_send_remove_toplevel(
        &self,
        name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            name,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_session_v1#{}.remove_toplevel(name: {:?})\n", id, arg0);
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
            4,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// remove a surface from the session
    ///
    /// Remove a specified surface from the session and render any related
    /// xdg_toplevel_session_v1 object inert. The compositor should remove any
    /// data related to the toplevel in the corresponding session from its internal
    /// storage.
    ///
    /// The window is specified by its name in the session. The name string
    /// must be encoded in UTF-8, and it is limited in size by the maximum
    /// length of wayland messages (around 4KB).
    ///
    /// # Arguments
    ///
    /// - `name`: name identifying the toplevel
    #[inline]
    pub fn send_remove_toplevel(
        &self,
        name: &str,
    ) {
        let res = self.try_send_remove_toplevel(
            name,
        );
        if let Err(e) = res {
            log_send("xdg_session_v1.remove_toplevel", &e);
        }
    }

    /// Since when the created message is available.
    pub const MSG__CREATED__SINCE: u32 = 1;

    /// newly-created session id
    ///
    /// Emitted at most once some time after getting a new session object. It
    /// means that no previous state was restored, and a new session was created.
    /// The passed id can be persistently stored and used to restore previous
    /// sessions.
    ///
    /// # Arguments
    ///
    /// - `session_id`:
    #[inline]
    pub fn try_send_created(
        &self,
        session_id: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            session_id,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xdg_session_v1#{}.created(session_id: {:?})\n", client_id, id, arg0);
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

    /// newly-created session id
    ///
    /// Emitted at most once some time after getting a new session object. It
    /// means that no previous state was restored, and a new session was created.
    /// The passed id can be persistently stored and used to restore previous
    /// sessions.
    ///
    /// # Arguments
    ///
    /// - `session_id`:
    #[inline]
    pub fn send_created(
        &self,
        session_id: &str,
    ) {
        let res = self.try_send_created(
            session_id,
        );
        if let Err(e) = res {
            log_send("xdg_session_v1.created", &e);
        }
    }

    /// Since when the restored message is available.
    pub const MSG__RESTORED__SINCE: u32 = 1;

    /// the session has been restored
    ///
    /// Emitted at most once some time after getting a new session object. It
    /// means that previous state was at least partially restored. The same id
    /// can again be used to restore previous sessions.
    #[inline]
    pub fn try_send_restored(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xdg_session_v1#{}.restored()\n", client_id, id);
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

    /// the session has been restored
    ///
    /// Emitted at most once some time after getting a new session object. It
    /// means that previous state was at least partially restored. The same id
    /// can again be used to restore previous sessions.
    #[inline]
    pub fn send_restored(
        &self,
    ) {
        let res = self.try_send_restored(
        );
        if let Err(e) = res {
            log_send("xdg_session_v1.restored", &e);
        }
    }

    /// Since when the replaced message is available.
    pub const MSG__REPLACED__SINCE: u32 = 1;

    /// the session has been replaced
    ///
    /// Emitted at most once, if the session was taken over by some other
    /// client. When this happens, the session and all its toplevel session
    /// objects become inert, and should be destroyed.
    #[inline]
    pub fn try_send_replaced(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xdg_session_v1#{}.replaced()\n", client_id, id);
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

    /// the session has been replaced
    ///
    /// Emitted at most once, if the session was taken over by some other
    /// client. When this happens, the session and all its toplevel session
    /// objects become inert, and should be destroyed.
    #[inline]
    pub fn send_replaced(
        &self,
    ) {
        let res = self.try_send_replaced(
        );
        if let Err(e) = res {
            log_send("xdg_session_v1.replaced", &e);
        }
    }
}

/// A message handler for [`XdgSessionV1`] proxies.
pub trait XdgSessionV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<XdgSessionV1>) {
        slf.core.delete_id();
    }

    /// Destroy the session
    ///
    /// Destroy a session object, preserving the current state but not continuing
    /// to make further updates if state changes occur. This makes the associated
    /// xdg_toplevel_session_v1 objects inert.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<XdgSessionV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("xdg_session_v1.destroy", &e);
        }
    }

    /// Remove the session
    ///
    /// Remove the session, making it no longer available for restoration. A
    /// compositor should in response to this request remove the data related to
    /// this session from its storage.
    #[inline]
    fn handle_remove(
        &mut self,
        slf: &Rc<XdgSessionV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_remove(
        );
        if let Err(e) = res {
            log_forward("xdg_session_v1.remove", &e);
        }
    }

    /// add a new surface to the session
    ///
    /// Attempt to add a given surface to the session. The passed name is used
    /// to identify what window is being restored, and may be used to store
    /// window specific state within the session.
    ///
    /// The name given to the toplevel must not correspond to any previously
    /// existing toplevel names in the session. If the name matches an already
    /// known toplevel name in the session, a 'name_in_use' protocol error will
    /// be raised.
    ///
    /// The toplevel object must not be added more than once to any session
    /// created by the client, otherwise the 'already_added' protocol error
    /// will be raised.
    ///
    /// This request will return a xdg_toplevel_session_v1 for later
    /// manipulation. As this resource is created from an empty initial state,
    /// compositors must not emit a xdg_toplevel_session_v1.restored event for
    /// resources created through this request.
    ///
    /// The name string must be UTF-8 encoded. It is also limited by the maximum
    /// length of wayland messages (around 4KB). The 'invalid_name' protocol
    /// error will be raised if an invalid string is provided.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `toplevel`:
    /// - `name`: name identifying the toplevel
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_add_toplevel(
        &mut self,
        slf: &Rc<XdgSessionV1>,
        id: &Rc<XdgToplevelSessionV1>,
        toplevel: &Rc<XdgToplevel>,
        name: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_add_toplevel(
            id,
            toplevel,
            name,
        );
        if let Err(e) = res {
            log_forward("xdg_session_v1.add_toplevel", &e);
        }
    }

    /// restore a surface state
    ///
    /// Inform the compositor that the toplevel associated with the passed name
    /// should have its window management state restored.
    ///
    /// If the toplevel name was previously granted to another xdg_toplevel,
    /// the 'name_in_use' protocol error will be raised.
    ///
    /// The toplevel object must not be added more than once to any session
    /// created by the client, otherwise the 'already_added' protocol error
    /// will be raised.
    ///
    /// This request must be called prior to the first commit on the associated
    /// wl_surface after creating the toplevel, otherwise an 'already_mapped'
    /// error is raised.
    ///
    /// As part of the initial configure sequence, if the toplevel was
    /// successfully restored, a xdg_toplevel_session_v1.restored event is
    /// emitted. If the toplevel name was not known in the session, this request
    /// will be equivalent to the xdg_toplevel_session_v1.add_toplevel request,
    /// and no such event will be emitted. See the xdg_toplevel_session_v1.restored
    /// event for further details.
    ///
    /// The name string must be UTF-8 encoded. It is also limited by the maximum
    /// length of wayland messages (around 4KB). The 'invalid_name' protocol
    /// error will be raised if an invalid string is provided.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `toplevel`:
    /// - `name`: name identifying the toplevel
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_restore_toplevel(
        &mut self,
        slf: &Rc<XdgSessionV1>,
        id: &Rc<XdgToplevelSessionV1>,
        toplevel: &Rc<XdgToplevel>,
        name: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_restore_toplevel(
            id,
            toplevel,
            name,
        );
        if let Err(e) = res {
            log_forward("xdg_session_v1.restore_toplevel", &e);
        }
    }

    /// remove a surface from the session
    ///
    /// Remove a specified surface from the session and render any related
    /// xdg_toplevel_session_v1 object inert. The compositor should remove any
    /// data related to the toplevel in the corresponding session from its internal
    /// storage.
    ///
    /// The window is specified by its name in the session. The name string
    /// must be encoded in UTF-8, and it is limited in size by the maximum
    /// length of wayland messages (around 4KB).
    ///
    /// # Arguments
    ///
    /// - `name`: name identifying the toplevel
    #[inline]
    fn handle_remove_toplevel(
        &mut self,
        slf: &Rc<XdgSessionV1>,
        name: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_remove_toplevel(
            name,
        );
        if let Err(e) = res {
            log_forward("xdg_session_v1.remove_toplevel", &e);
        }
    }

    /// newly-created session id
    ///
    /// Emitted at most once some time after getting a new session object. It
    /// means that no previous state was restored, and a new session was created.
    /// The passed id can be persistently stored and used to restore previous
    /// sessions.
    ///
    /// # Arguments
    ///
    /// - `session_id`:
    #[inline]
    fn handle_created(
        &mut self,
        slf: &Rc<XdgSessionV1>,
        session_id: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_created(
            session_id,
        );
        if let Err(e) = res {
            log_forward("xdg_session_v1.created", &e);
        }
    }

    /// the session has been restored
    ///
    /// Emitted at most once some time after getting a new session object. It
    /// means that previous state was at least partially restored. The same id
    /// can again be used to restore previous sessions.
    #[inline]
    fn handle_restored(
        &mut self,
        slf: &Rc<XdgSessionV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_restored(
        );
        if let Err(e) = res {
            log_forward("xdg_session_v1.restored", &e);
        }
    }

    /// the session has been replaced
    ///
    /// Emitted at most once, if the session was taken over by some other
    /// client. When this happens, the session and all its toplevel session
    /// objects become inert, and should be destroyed.
    #[inline]
    fn handle_replaced(
        &mut self,
        slf: &Rc<XdgSessionV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_replaced(
        );
        if let Err(e) = res {
            log_forward("xdg_session_v1.replaced", &e);
        }
    }
}

impl ObjectPrivate for XdgSessionV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::XdgSessionV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_session_v1#{}.destroy()\n", client_id, id);
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
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_session_v1#{}.remove()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_remove(&self);
                } else {
                    DefaultHandler.handle_remove(&self);
                }
            }
            2 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("id")));
                };
                offset += 1;
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("toplevel")));
                };
                offset += 1;
                let arg2;
                (arg2, offset) = parse_string::<NonNullString>(msg, offset, "name")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_session_v1#{}.add_toplevel(id: xdg_toplevel_session_v1#{}, toplevel: xdg_toplevel#{}, name: {:?})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let arg0 = XdgToplevelSessionV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<XdgToplevel>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("toplevel", o.core().interface, ObjectInterface::XdgToplevel)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_add_toplevel(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_add_toplevel(&self, arg0, arg1, arg2);
                }
            }
            3 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("id")));
                };
                offset += 1;
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("toplevel")));
                };
                offset += 1;
                let arg2;
                (arg2, offset) = parse_string::<NonNullString>(msg, offset, "name")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_session_v1#{}.restore_toplevel(id: xdg_toplevel_session_v1#{}, toplevel: xdg_toplevel#{}, name: {:?})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let arg0 = XdgToplevelSessionV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<XdgToplevel>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("toplevel", o.core().interface, ObjectInterface::XdgToplevel)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_restore_toplevel(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_restore_toplevel(&self, arg0, arg1, arg2);
                }
            }
            4 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "name")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_session_v1#{}.remove_toplevel(name: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_remove_toplevel(&self, arg0);
                } else {
                    DefaultHandler.handle_remove_toplevel(&self, arg0);
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
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "session_id")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xdg_session_v1#{}.created(session_id: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_created(&self, arg0);
                } else {
                    DefaultHandler.handle_created(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xdg_session_v1#{}.restored()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_restored(&self);
                } else {
                    DefaultHandler.handle_restored(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xdg_session_v1#{}.replaced()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_replaced(&self);
                } else {
                    DefaultHandler.handle_replaced(&self);
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
            1 => "remove",
            2 => "add_toplevel",
            3 => "restore_toplevel",
            4 => "remove_toplevel",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "created",
            1 => "restored",
            2 => "replaced",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for XdgSessionV1 {
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

impl XdgSessionV1 {
    /// Since when the error.name_in_use enum variant is available.
    pub const ENM__ERROR_NAME_IN_USE__SINCE: u32 = 1;
    /// Since when the error.already_mapped enum variant is available.
    pub const ENM__ERROR_ALREADY_MAPPED__SINCE: u32 = 1;
    /// Since when the error.invalid_name enum variant is available.
    pub const ENM__ERROR_INVALID_NAME__SINCE: u32 = 1;
    /// Since when the error.already_added enum variant is available.
    pub const ENM__ERROR_ALREADY_ADDED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct XdgSessionV1Error(pub u32);

impl XdgSessionV1Error {
    /// toplevel name is already in use
    pub const NAME_IN_USE: Self = Self(1);

    /// toplevel was already mapped when restored
    pub const ALREADY_MAPPED: Self = Self(2);

    /// provided toplevel name is invalid
    pub const INVALID_NAME: Self = Self(3);

    /// toplevel already added
    pub const ALREADY_ADDED: Self = Self(4);
}

impl Debug for XdgSessionV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NAME_IN_USE => "NAME_IN_USE",
            Self::ALREADY_MAPPED => "ALREADY_MAPPED",
            Self::INVALID_NAME => "INVALID_NAME",
            Self::ALREADY_ADDED => "ALREADY_ADDED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
