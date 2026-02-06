//! manage zones for clients
//!
//! The 'xx_zone_manager' interface defines base requests for obtaining and
//! managing zones for a client.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A xx_zone_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XxZoneManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn XxZoneManagerV1Handler>,
}

struct DefaultHandler;

impl XxZoneManagerV1Handler for DefaultHandler { }

impl ConcreteObject for XxZoneManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::XxZoneManagerV1;
    const INTERFACE_NAME: &str = "xx_zone_manager_v1";
}

impl XxZoneManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl XxZoneManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn XxZoneManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XxZoneManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XxZoneManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XxZoneManagerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// Destroy this object
    ///
    /// This has no effect other than to destroy the xx_zone_manager object.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xx_zone_manager_v1#{}.destroy()\n", id);
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

    /// Destroy this object
    ///
    /// This has no effect other than to destroy the xx_zone_manager object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("xx_zone_manager_v1.destroy", &e);
        }
    }

    /// Since when the get_zone_item message is available.
    pub const MSG__GET_ZONE_ITEM__SINCE: u32 = 1;

    /// create a positionable item representing a toplevel
    ///
    /// Create a new positionable zone item from an 'xdg_toplevel'.
    /// The resulting wrapper object can then be used to position the
    /// toplevel window in a zone.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `toplevel`: the toplevel window
    #[inline]
    pub fn try_send_get_zone_item(
        &self,
        id: &Rc<XxZoneItemV1>,
        toplevel: &Rc<XdgToplevel>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            toplevel,
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
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xx_zone_manager_v1#{}.get_zone_item(id: xx_zone_item_v1#{}, toplevel: xdg_toplevel#{})\n", id, arg0, arg1);
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
            1,
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// create a positionable item representing a toplevel
    ///
    /// Create a new positionable zone item from an 'xdg_toplevel'.
    /// The resulting wrapper object can then be used to position the
    /// toplevel window in a zone.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `toplevel`: the toplevel window
    #[inline]
    pub fn send_get_zone_item(
        &self,
        id: &Rc<XxZoneItemV1>,
        toplevel: &Rc<XdgToplevel>,
    ) {
        let res = self.try_send_get_zone_item(
            id,
            toplevel,
        );
        if let Err(e) = res {
            log_send("xx_zone_manager_v1.get_zone_item", &e);
        }
    }

    /// create a positionable item representing a toplevel
    ///
    /// Create a new positionable zone item from an 'xdg_toplevel'.
    /// The resulting wrapper object can then be used to position the
    /// toplevel window in a zone.
    ///
    /// # Arguments
    ///
    /// - `toplevel`: the toplevel window
    #[inline]
    pub fn new_try_send_get_zone_item(
        &self,
        toplevel: &Rc<XdgToplevel>,
    ) -> Result<Rc<XxZoneItemV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_zone_item(
            &id,
            toplevel,
        )?;
        Ok(id)
    }

    /// create a positionable item representing a toplevel
    ///
    /// Create a new positionable zone item from an 'xdg_toplevel'.
    /// The resulting wrapper object can then be used to position the
    /// toplevel window in a zone.
    ///
    /// # Arguments
    ///
    /// - `toplevel`: the toplevel window
    #[inline]
    pub fn new_send_get_zone_item(
        &self,
        toplevel: &Rc<XdgToplevel>,
    ) -> Rc<XxZoneItemV1> {
        let id = self.core.create_child();
        self.send_get_zone_item(
            &id,
            toplevel,
        );
        id
    }

    /// Since when the get_zone message is available.
    pub const MSG__GET_ZONE__SINCE: u32 = 1;

    /// join a zone or request a new one
    ///
    /// Create a new zone. While the zone object exists, the compositor
    /// must consider it "used" and keep track of it.
    ///
    /// A zone is represented by a string 'handle'.
    ///
    /// The compositor must keep zone handles valid while any client is
    /// referencing the corresponding zone.
    /// The compositor may always give a client the same zone for a given
    /// output, and remember its position and size for the client, but
    /// clients should not rely on this behavior.
    ///
    /// A client can request a zone to be placed on a specific
    /// output by passing a wl_output as 'output'. If a valid output
    /// is set, the compositor should place the zone on that output.
    /// If NULL is passed, the compositor decides the output.
    ///
    /// The compositor should provide the biggest reasonable zone space
    /// for the client, governed by its own policy.
    ///
    /// If the compositor wants to deny zone creation (e.g. on a specific
    /// output), the returned zone must be "invalid". A zone is invalid
    /// if it has a negative size, in which case the client is forbidden
    /// to place items in it.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `output`: the preferred output to place the zone on, or NULL
    #[inline]
    pub fn try_send_get_zone(
        &self,
        id: &Rc<XxZoneV1>,
        output: Option<&Rc<WlOutput>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            output,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1 {
            None => 0,
            Some(arg1) => match arg1.server_obj_id.get() {
                None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("output"))),
                Some(id) => id,
            },
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xx_zone_manager_v1#{}.get_zone(id: xx_zone_v1#{}, output: wl_output#{})\n", id, arg0, arg1);
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
            2,
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// join a zone or request a new one
    ///
    /// Create a new zone. While the zone object exists, the compositor
    /// must consider it "used" and keep track of it.
    ///
    /// A zone is represented by a string 'handle'.
    ///
    /// The compositor must keep zone handles valid while any client is
    /// referencing the corresponding zone.
    /// The compositor may always give a client the same zone for a given
    /// output, and remember its position and size for the client, but
    /// clients should not rely on this behavior.
    ///
    /// A client can request a zone to be placed on a specific
    /// output by passing a wl_output as 'output'. If a valid output
    /// is set, the compositor should place the zone on that output.
    /// If NULL is passed, the compositor decides the output.
    ///
    /// The compositor should provide the biggest reasonable zone space
    /// for the client, governed by its own policy.
    ///
    /// If the compositor wants to deny zone creation (e.g. on a specific
    /// output), the returned zone must be "invalid". A zone is invalid
    /// if it has a negative size, in which case the client is forbidden
    /// to place items in it.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `output`: the preferred output to place the zone on, or NULL
    #[inline]
    pub fn send_get_zone(
        &self,
        id: &Rc<XxZoneV1>,
        output: Option<&Rc<WlOutput>>,
    ) {
        let res = self.try_send_get_zone(
            id,
            output,
        );
        if let Err(e) = res {
            log_send("xx_zone_manager_v1.get_zone", &e);
        }
    }

    /// join a zone or request a new one
    ///
    /// Create a new zone. While the zone object exists, the compositor
    /// must consider it "used" and keep track of it.
    ///
    /// A zone is represented by a string 'handle'.
    ///
    /// The compositor must keep zone handles valid while any client is
    /// referencing the corresponding zone.
    /// The compositor may always give a client the same zone for a given
    /// output, and remember its position and size for the client, but
    /// clients should not rely on this behavior.
    ///
    /// A client can request a zone to be placed on a specific
    /// output by passing a wl_output as 'output'. If a valid output
    /// is set, the compositor should place the zone on that output.
    /// If NULL is passed, the compositor decides the output.
    ///
    /// The compositor should provide the biggest reasonable zone space
    /// for the client, governed by its own policy.
    ///
    /// If the compositor wants to deny zone creation (e.g. on a specific
    /// output), the returned zone must be "invalid". A zone is invalid
    /// if it has a negative size, in which case the client is forbidden
    /// to place items in it.
    ///
    /// # Arguments
    ///
    /// - `output`: the preferred output to place the zone on, or NULL
    #[inline]
    pub fn new_try_send_get_zone(
        &self,
        output: Option<&Rc<WlOutput>>,
    ) -> Result<Rc<XxZoneV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_zone(
            &id,
            output,
        )?;
        Ok(id)
    }

    /// join a zone or request a new one
    ///
    /// Create a new zone. While the zone object exists, the compositor
    /// must consider it "used" and keep track of it.
    ///
    /// A zone is represented by a string 'handle'.
    ///
    /// The compositor must keep zone handles valid while any client is
    /// referencing the corresponding zone.
    /// The compositor may always give a client the same zone for a given
    /// output, and remember its position and size for the client, but
    /// clients should not rely on this behavior.
    ///
    /// A client can request a zone to be placed on a specific
    /// output by passing a wl_output as 'output'. If a valid output
    /// is set, the compositor should place the zone on that output.
    /// If NULL is passed, the compositor decides the output.
    ///
    /// The compositor should provide the biggest reasonable zone space
    /// for the client, governed by its own policy.
    ///
    /// If the compositor wants to deny zone creation (e.g. on a specific
    /// output), the returned zone must be "invalid". A zone is invalid
    /// if it has a negative size, in which case the client is forbidden
    /// to place items in it.
    ///
    /// # Arguments
    ///
    /// - `output`: the preferred output to place the zone on, or NULL
    #[inline]
    pub fn new_send_get_zone(
        &self,
        output: Option<&Rc<WlOutput>>,
    ) -> Rc<XxZoneV1> {
        let id = self.core.create_child();
        self.send_get_zone(
            &id,
            output,
        );
        id
    }

    /// Since when the get_zone_from_handle message is available.
    pub const MSG__GET_ZONE_FROM_HANDLE__SINCE: u32 = 1;

    /// join a zone via its handle
    ///
    /// Create a new zone object using the zone's handle.
    /// For the returned zone, the same rules as described in
    /// 'get_zone' apply.
    ///
    /// This request returns a reference to an existing or remembered zone
    /// that is represented by 'handle'.
    /// The zone may potentially have been created by a different client.
    ///
    /// This allows cooperating clients to share the same coordinate space.
    ///
    /// If the zone handle was invalid or unknown, a new zone must
    /// be created and returned instead, following the rules outlined
    /// in 'get_zone' and assuming no output preference.
    ///
    /// Every new zone object created by this request emits its initial event
    /// sequence, including the 'handle' event, which must return a different
    /// handle from the one passed to this request in case the existing zone
    /// could not be joined.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `handle`: the handle of a zone
    #[inline]
    pub fn try_send_get_zone_from_handle(
        &self,
        id: &Rc<XxZoneV1>,
        handle: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            handle,
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
            fn log(state: &State, id: u32, arg0: u32, arg1: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xx_zone_manager_v1#{}.get_zone_from_handle(id: xx_zone_v1#{}, handle: {:?})\n", id, arg0, arg1);
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
            3,
            arg0_id,
        ]);
        fmt.string(arg1);
        Ok(())
    }

    /// join a zone via its handle
    ///
    /// Create a new zone object using the zone's handle.
    /// For the returned zone, the same rules as described in
    /// 'get_zone' apply.
    ///
    /// This request returns a reference to an existing or remembered zone
    /// that is represented by 'handle'.
    /// The zone may potentially have been created by a different client.
    ///
    /// This allows cooperating clients to share the same coordinate space.
    ///
    /// If the zone handle was invalid or unknown, a new zone must
    /// be created and returned instead, following the rules outlined
    /// in 'get_zone' and assuming no output preference.
    ///
    /// Every new zone object created by this request emits its initial event
    /// sequence, including the 'handle' event, which must return a different
    /// handle from the one passed to this request in case the existing zone
    /// could not be joined.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `handle`: the handle of a zone
    #[inline]
    pub fn send_get_zone_from_handle(
        &self,
        id: &Rc<XxZoneV1>,
        handle: &str,
    ) {
        let res = self.try_send_get_zone_from_handle(
            id,
            handle,
        );
        if let Err(e) = res {
            log_send("xx_zone_manager_v1.get_zone_from_handle", &e);
        }
    }

    /// join a zone via its handle
    ///
    /// Create a new zone object using the zone's handle.
    /// For the returned zone, the same rules as described in
    /// 'get_zone' apply.
    ///
    /// This request returns a reference to an existing or remembered zone
    /// that is represented by 'handle'.
    /// The zone may potentially have been created by a different client.
    ///
    /// This allows cooperating clients to share the same coordinate space.
    ///
    /// If the zone handle was invalid or unknown, a new zone must
    /// be created and returned instead, following the rules outlined
    /// in 'get_zone' and assuming no output preference.
    ///
    /// Every new zone object created by this request emits its initial event
    /// sequence, including the 'handle' event, which must return a different
    /// handle from the one passed to this request in case the existing zone
    /// could not be joined.
    ///
    /// # Arguments
    ///
    /// - `handle`: the handle of a zone
    #[inline]
    pub fn new_try_send_get_zone_from_handle(
        &self,
        handle: &str,
    ) -> Result<Rc<XxZoneV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_zone_from_handle(
            &id,
            handle,
        )?;
        Ok(id)
    }

    /// join a zone via its handle
    ///
    /// Create a new zone object using the zone's handle.
    /// For the returned zone, the same rules as described in
    /// 'get_zone' apply.
    ///
    /// This request returns a reference to an existing or remembered zone
    /// that is represented by 'handle'.
    /// The zone may potentially have been created by a different client.
    ///
    /// This allows cooperating clients to share the same coordinate space.
    ///
    /// If the zone handle was invalid or unknown, a new zone must
    /// be created and returned instead, following the rules outlined
    /// in 'get_zone' and assuming no output preference.
    ///
    /// Every new zone object created by this request emits its initial event
    /// sequence, including the 'handle' event, which must return a different
    /// handle from the one passed to this request in case the existing zone
    /// could not be joined.
    ///
    /// # Arguments
    ///
    /// - `handle`: the handle of a zone
    #[inline]
    pub fn new_send_get_zone_from_handle(
        &self,
        handle: &str,
    ) -> Rc<XxZoneV1> {
        let id = self.core.create_child();
        self.send_get_zone_from_handle(
            &id,
            handle,
        );
        id
    }
}

/// A message handler for [`XxZoneManagerV1`] proxies.
pub trait XxZoneManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<XxZoneManagerV1>) {
        slf.core.delete_id();
    }

    /// Destroy this object
    ///
    /// This has no effect other than to destroy the xx_zone_manager object.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<XxZoneManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("xx_zone_manager_v1.destroy", &e);
        }
    }

    /// create a positionable item representing a toplevel
    ///
    /// Create a new positionable zone item from an 'xdg_toplevel'.
    /// The resulting wrapper object can then be used to position the
    /// toplevel window in a zone.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `toplevel`: the toplevel window
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_zone_item(
        &mut self,
        slf: &Rc<XxZoneManagerV1>,
        id: &Rc<XxZoneItemV1>,
        toplevel: &Rc<XdgToplevel>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_zone_item(
            id,
            toplevel,
        );
        if let Err(e) = res {
            log_forward("xx_zone_manager_v1.get_zone_item", &e);
        }
    }

    /// join a zone or request a new one
    ///
    /// Create a new zone. While the zone object exists, the compositor
    /// must consider it "used" and keep track of it.
    ///
    /// A zone is represented by a string 'handle'.
    ///
    /// The compositor must keep zone handles valid while any client is
    /// referencing the corresponding zone.
    /// The compositor may always give a client the same zone for a given
    /// output, and remember its position and size for the client, but
    /// clients should not rely on this behavior.
    ///
    /// A client can request a zone to be placed on a specific
    /// output by passing a wl_output as 'output'. If a valid output
    /// is set, the compositor should place the zone on that output.
    /// If NULL is passed, the compositor decides the output.
    ///
    /// The compositor should provide the biggest reasonable zone space
    /// for the client, governed by its own policy.
    ///
    /// If the compositor wants to deny zone creation (e.g. on a specific
    /// output), the returned zone must be "invalid". A zone is invalid
    /// if it has a negative size, in which case the client is forbidden
    /// to place items in it.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `output`: the preferred output to place the zone on, or NULL
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_zone(
        &mut self,
        slf: &Rc<XxZoneManagerV1>,
        id: &Rc<XxZoneV1>,
        output: Option<&Rc<WlOutput>>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_zone(
            id,
            output,
        );
        if let Err(e) = res {
            log_forward("xx_zone_manager_v1.get_zone", &e);
        }
    }

    /// join a zone via its handle
    ///
    /// Create a new zone object using the zone's handle.
    /// For the returned zone, the same rules as described in
    /// 'get_zone' apply.
    ///
    /// This request returns a reference to an existing or remembered zone
    /// that is represented by 'handle'.
    /// The zone may potentially have been created by a different client.
    ///
    /// This allows cooperating clients to share the same coordinate space.
    ///
    /// If the zone handle was invalid or unknown, a new zone must
    /// be created and returned instead, following the rules outlined
    /// in 'get_zone' and assuming no output preference.
    ///
    /// Every new zone object created by this request emits its initial event
    /// sequence, including the 'handle' event, which must return a different
    /// handle from the one passed to this request in case the existing zone
    /// could not be joined.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `handle`: the handle of a zone
    #[inline]
    fn handle_get_zone_from_handle(
        &mut self,
        slf: &Rc<XxZoneManagerV1>,
        id: &Rc<XxZoneV1>,
        handle: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_zone_from_handle(
            id,
            handle,
        );
        if let Err(e) = res {
            log_forward("xx_zone_manager_v1.get_zone_from_handle", &e);
        }
    }
}

impl ObjectPrivate for XxZoneManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::XxZoneManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xx_zone_manager_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xx_zone_manager_v1#{}.get_zone_item(id: xx_zone_item_v1#{}, toplevel: xdg_toplevel#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = XxZoneItemV1::new(&self.core.state, self.core.version);
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
                    (**handler).handle_get_zone_item(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_zone_item(&self, arg0, arg1);
                }
            }
            2 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xx_zone_manager_v1#{}.get_zone(id: xx_zone_v1#{}, output: wl_output#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = XxZoneV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1 = if arg1 == 0 {
                    None
                } else {
                    let arg1_id = arg1;
                    let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                    };
                    let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlOutput>() else {
                        let o = client.endpoint.lookup(arg1_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("output", o.core().interface, ObjectInterface::WlOutput)));
                    };
                    Some(arg1)
                };
                let arg0 = &arg0;
                let arg1 = arg1.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_get_zone(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_zone(&self, arg0, arg1);
                }
            }
            3 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("id")));
                };
                offset += 1;
                let arg1;
                (arg1, offset) = parse_string::<NonNullString>(msg, offset, "handle")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xx_zone_manager_v1#{}.get_zone_from_handle(id: xx_zone_v1#{}, handle: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = XxZoneV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_get_zone_from_handle(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_zone_from_handle(&self, arg0, arg1);
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
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            1 => "get_zone_item",
            2 => "get_zone",
            3 => "get_zone_from_handle",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for XxZoneManagerV1 {
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

