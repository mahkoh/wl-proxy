//! Colorimetric image description
//!
//! An image description carries information about the pixel color encoding
//! and its intended display and viewing environment. The image description is
//! attached to a wl_surface via
//! wp_color_management_surface_v1.set_image_description. A compositor can use
//! this information to decode pixel values into colorimetrically meaningful
//! quantities, which allows the compositor to transform the surface contents
//! to become suitable for various displays and viewing environments.
//!
//! Note, that the wp_image_description_v1 object is not ready to be used
//! immediately after creation. The object eventually delivers either the
//! 'ready' or the 'failed' event, specified in all requests creating it. The
//! object is deemed "ready" after receiving the 'ready' event.
//!
//! An object which is not ready is illegal to use, it can only be destroyed.
//! Any other request in this interface shall result in the 'not_ready'
//! protocol error. Attempts to use an object which is not ready through other
//! interfaces shall raise protocol errors defined there.
//!
//! Once created and regardless of how it was created, a
//! wp_image_description_v1 object always refers to one fixed image
//! description. It cannot change after creation.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_image_description_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpImageDescriptionV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpImageDescriptionV1Handler>,
}

struct DefaultHandler;

impl WpImageDescriptionV1Handler for DefaultHandler { }

impl ConcreteObject for WpImageDescriptionV1 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::WpImageDescriptionV1;
    const INTERFACE_NAME: &str = "wp_image_description_v1";
}

impl WpImageDescriptionV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WpImageDescriptionV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WpImageDescriptionV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpImageDescriptionV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpImageDescriptionV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpImageDescriptionV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the image description
    ///
    /// Destroy this object. It is safe to destroy an object which is not ready.
    ///
    /// Destroying a wp_image_description_v1 object has no side-effects, not
    /// even if a wp_color_management_surface_v1.set_image_description has not
    /// yet been followed by a wl_surface.commit.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_image_description_v1#{}.destroy()\n", id);
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

    /// destroy the image description
    ///
    /// Destroy this object. It is safe to destroy an object which is not ready.
    ///
    /// Destroying a wp_image_description_v1 object has no side-effects, not
    /// even if a wp_color_management_surface_v1.set_image_description has not
    /// yet been followed by a wl_surface.commit.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wp_image_description_v1.destroy", &e);
        }
    }

    /// Since when the failed message is available.
    pub const MSG__FAILED__SINCE: u32 = 1;

    /// graceful error on creating the image description
    ///
    /// If creating a wp_image_description_v1 object fails for a reason that is
    /// not defined as a protocol error, this event is sent.
    ///
    /// The requests that create image description objects define whether and
    /// when this can occur. Only such creation requests can trigger this event.
    /// This event cannot be triggered after the image description was
    /// successfully formed.
    ///
    /// Once this event has been sent, the wp_image_description_v1 object will
    /// never become ready and it can only be destroyed.
    ///
    /// # Arguments
    ///
    /// - `cause`: generic reason
    /// - `msg`: ad hoc human-readable explanation
    #[inline]
    pub fn try_send_failed(
        &self,
        cause: WpImageDescriptionV1Cause,
        msg: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            cause,
            msg,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: WpImageDescriptionV1Cause, arg1: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_image_description_v1#{}.failed(cause: {:?}, msg: {:?})\n", client_id, id, arg0, arg1);
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
            arg0.0,
        ]);
        fmt.string(arg1);
        Ok(())
    }

    /// graceful error on creating the image description
    ///
    /// If creating a wp_image_description_v1 object fails for a reason that is
    /// not defined as a protocol error, this event is sent.
    ///
    /// The requests that create image description objects define whether and
    /// when this can occur. Only such creation requests can trigger this event.
    /// This event cannot be triggered after the image description was
    /// successfully formed.
    ///
    /// Once this event has been sent, the wp_image_description_v1 object will
    /// never become ready and it can only be destroyed.
    ///
    /// # Arguments
    ///
    /// - `cause`: generic reason
    /// - `msg`: ad hoc human-readable explanation
    #[inline]
    pub fn send_failed(
        &self,
        cause: WpImageDescriptionV1Cause,
        msg: &str,
    ) {
        let res = self.try_send_failed(
            cause,
            msg,
        );
        if let Err(e) = res {
            log_send("wp_image_description_v1.failed", &e);
        }
    }

    /// Since when the ready message is available.
    pub const MSG__READY__SINCE: u32 = 1;

    /// Since when the ready message is deprecated.
    pub const MSG__READY__DEPRECATED_SINCE: u32 = 2;

    /// the object is ready to be used (32-bit)
    ///
    /// Starting from interface version 2, the 'ready2' event is sent instead
    /// of this event.
    ///
    /// For the definition of this event, see the 'ready2' event. The
    /// difference to this event is as follows.
    ///
    /// The id number is valid only as long as the protocol object is alive. If
    /// all protocol objects referring to the same image description record are
    /// destroyed, the id number may be recycled for a different image
    /// description record.
    ///
    /// # Arguments
    ///
    /// - `identity`: the 32-bit image description id number
    #[inline]
    pub fn try_send_ready(
        &self,
        identity: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            identity,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_image_description_v1#{}.ready(identity: {})\n", client_id, id, arg0);
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

    /// the object is ready to be used (32-bit)
    ///
    /// Starting from interface version 2, the 'ready2' event is sent instead
    /// of this event.
    ///
    /// For the definition of this event, see the 'ready2' event. The
    /// difference to this event is as follows.
    ///
    /// The id number is valid only as long as the protocol object is alive. If
    /// all protocol objects referring to the same image description record are
    /// destroyed, the id number may be recycled for a different image
    /// description record.
    ///
    /// # Arguments
    ///
    /// - `identity`: the 32-bit image description id number
    #[inline]
    pub fn send_ready(
        &self,
        identity: u32,
    ) {
        let res = self.try_send_ready(
            identity,
        );
        if let Err(e) = res {
            log_send("wp_image_description_v1.ready", &e);
        }
    }

    /// Since when the get_information message is available.
    pub const MSG__GET_INFORMATION__SINCE: u32 = 1;

    /// get information about the image description
    ///
    /// Creates a wp_image_description_info_v1 object which delivers the
    /// information that makes up the image description.
    ///
    /// Not all image description protocol objects allow get_information
    /// request. Whether it is allowed or not is defined by the request that
    /// created the object. If get_information is not allowed, the protocol
    /// error no_information is raised.
    ///
    /// # Arguments
    ///
    /// - `information`:
    #[inline]
    pub fn try_send_get_information(
        &self,
        information: &Rc<WpImageDescriptionInfoV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            information,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("information", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_image_description_v1#{}.get_information(information: wp_image_description_info_v1#{})\n", id, arg0);
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
            1,
            arg0_id,
        ]);
        Ok(())
    }

    /// get information about the image description
    ///
    /// Creates a wp_image_description_info_v1 object which delivers the
    /// information that makes up the image description.
    ///
    /// Not all image description protocol objects allow get_information
    /// request. Whether it is allowed or not is defined by the request that
    /// created the object. If get_information is not allowed, the protocol
    /// error no_information is raised.
    ///
    /// # Arguments
    ///
    /// - `information`:
    #[inline]
    pub fn send_get_information(
        &self,
        information: &Rc<WpImageDescriptionInfoV1>,
    ) {
        let res = self.try_send_get_information(
            information,
        );
        if let Err(e) = res {
            log_send("wp_image_description_v1.get_information", &e);
        }
    }

    /// get information about the image description
    ///
    /// Creates a wp_image_description_info_v1 object which delivers the
    /// information that makes up the image description.
    ///
    /// Not all image description protocol objects allow get_information
    /// request. Whether it is allowed or not is defined by the request that
    /// created the object. If get_information is not allowed, the protocol
    /// error no_information is raised.
    #[inline]
    pub fn new_try_send_get_information(
        &self,
    ) -> Result<Rc<WpImageDescriptionInfoV1>, ObjectError> {
        let information = self.core.create_child();
        self.try_send_get_information(
            &information,
        )?;
        Ok(information)
    }

    /// get information about the image description
    ///
    /// Creates a wp_image_description_info_v1 object which delivers the
    /// information that makes up the image description.
    ///
    /// Not all image description protocol objects allow get_information
    /// request. Whether it is allowed or not is defined by the request that
    /// created the object. If get_information is not allowed, the protocol
    /// error no_information is raised.
    #[inline]
    pub fn new_send_get_information(
        &self,
    ) -> Rc<WpImageDescriptionInfoV1> {
        let information = self.core.create_child();
        self.send_get_information(
            &information,
        );
        information
    }

    /// Since when the ready2 message is available.
    pub const MSG__READY2__SINCE: u32 = 2;

    /// the object is ready to be used
    ///
    /// Once this event has been sent, the wp_image_description_v1 object is
    /// deemed "ready". Ready objects can be used to send requests and can be
    /// used through other interfaces.
    ///
    /// Every ready wp_image_description_v1 protocol object refers to an
    /// underlying image description record in the compositor. Multiple protocol
    /// objects may end up referring to the same record. Clients may identify
    /// these "copies" by comparing their id numbers: if the numbers from two
    /// protocol objects are identical, the protocol objects refer to the same
    /// image description record. Two different image description records
    /// cannot have the same id number simultaneously. The id number does not
    /// change during the lifetime of the image description record.
    ///
    /// Image description id number is not a protocol object id. Zero is
    /// reserved as an invalid id number. It shall not be possible for a client
    /// to refer to an image description by its id number in protocol. The id
    /// numbers might not be portable between Wayland connections. A compositor
    /// shall not send an invalid id number.
    ///
    /// Compositors must not recycle image description id numbers.
    ///
    /// This identity allows clients to de-duplicate image description records
    /// and avoid get_information request if they already have the image
    /// description information.
    ///
    /// # Arguments
    ///
    /// - `identity_hi`: high 32 bits of the 64-bit image description id number
    /// - `identity_lo`: low 32 bits of the 64-bit image description id number
    #[inline]
    pub fn try_send_ready2(
        &self,
        identity_hi: u32,
        identity_lo: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            identity_hi,
            identity_lo,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_image_description_v1#{}.ready2(identity_hi: {}, identity_lo: {})\n", client_id, id, arg0, arg1);
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
            2,
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// the object is ready to be used
    ///
    /// Once this event has been sent, the wp_image_description_v1 object is
    /// deemed "ready". Ready objects can be used to send requests and can be
    /// used through other interfaces.
    ///
    /// Every ready wp_image_description_v1 protocol object refers to an
    /// underlying image description record in the compositor. Multiple protocol
    /// objects may end up referring to the same record. Clients may identify
    /// these "copies" by comparing their id numbers: if the numbers from two
    /// protocol objects are identical, the protocol objects refer to the same
    /// image description record. Two different image description records
    /// cannot have the same id number simultaneously. The id number does not
    /// change during the lifetime of the image description record.
    ///
    /// Image description id number is not a protocol object id. Zero is
    /// reserved as an invalid id number. It shall not be possible for a client
    /// to refer to an image description by its id number in protocol. The id
    /// numbers might not be portable between Wayland connections. A compositor
    /// shall not send an invalid id number.
    ///
    /// Compositors must not recycle image description id numbers.
    ///
    /// This identity allows clients to de-duplicate image description records
    /// and avoid get_information request if they already have the image
    /// description information.
    ///
    /// # Arguments
    ///
    /// - `identity_hi`: high 32 bits of the 64-bit image description id number
    /// - `identity_lo`: low 32 bits of the 64-bit image description id number
    #[inline]
    pub fn send_ready2(
        &self,
        identity_hi: u32,
        identity_lo: u32,
    ) {
        let res = self.try_send_ready2(
            identity_hi,
            identity_lo,
        );
        if let Err(e) = res {
            log_send("wp_image_description_v1.ready2", &e);
        }
    }
}

/// A message handler for [`WpImageDescriptionV1`] proxies.
pub trait WpImageDescriptionV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpImageDescriptionV1>) {
        slf.core.delete_id();
    }

    /// destroy the image description
    ///
    /// Destroy this object. It is safe to destroy an object which is not ready.
    ///
    /// Destroying a wp_image_description_v1 object has no side-effects, not
    /// even if a wp_color_management_surface_v1.set_image_description has not
    /// yet been followed by a wl_surface.commit.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WpImageDescriptionV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wp_image_description_v1.destroy", &e);
        }
    }

    /// graceful error on creating the image description
    ///
    /// If creating a wp_image_description_v1 object fails for a reason that is
    /// not defined as a protocol error, this event is sent.
    ///
    /// The requests that create image description objects define whether and
    /// when this can occur. Only such creation requests can trigger this event.
    /// This event cannot be triggered after the image description was
    /// successfully formed.
    ///
    /// Once this event has been sent, the wp_image_description_v1 object will
    /// never become ready and it can only be destroyed.
    ///
    /// # Arguments
    ///
    /// - `cause`: generic reason
    /// - `msg`: ad hoc human-readable explanation
    #[inline]
    fn handle_failed(
        &mut self,
        slf: &Rc<WpImageDescriptionV1>,
        cause: WpImageDescriptionV1Cause,
        msg: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_failed(
            cause,
            msg,
        );
        if let Err(e) = res {
            log_forward("wp_image_description_v1.failed", &e);
        }
    }

    /// the object is ready to be used (32-bit)
    ///
    /// Starting from interface version 2, the 'ready2' event is sent instead
    /// of this event.
    ///
    /// For the definition of this event, see the 'ready2' event. The
    /// difference to this event is as follows.
    ///
    /// The id number is valid only as long as the protocol object is alive. If
    /// all protocol objects referring to the same image description record are
    /// destroyed, the id number may be recycled for a different image
    /// description record.
    ///
    /// # Arguments
    ///
    /// - `identity`: the 32-bit image description id number
    #[inline]
    fn handle_ready(
        &mut self,
        slf: &Rc<WpImageDescriptionV1>,
        identity: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_ready(
            identity,
        );
        if let Err(e) = res {
            log_forward("wp_image_description_v1.ready", &e);
        }
    }

    /// get information about the image description
    ///
    /// Creates a wp_image_description_info_v1 object which delivers the
    /// information that makes up the image description.
    ///
    /// Not all image description protocol objects allow get_information
    /// request. Whether it is allowed or not is defined by the request that
    /// created the object. If get_information is not allowed, the protocol
    /// error no_information is raised.
    ///
    /// # Arguments
    ///
    /// - `information`:
    #[inline]
    fn handle_get_information(
        &mut self,
        slf: &Rc<WpImageDescriptionV1>,
        information: &Rc<WpImageDescriptionInfoV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_information(
            information,
        );
        if let Err(e) = res {
            log_forward("wp_image_description_v1.get_information", &e);
        }
    }

    /// the object is ready to be used
    ///
    /// Once this event has been sent, the wp_image_description_v1 object is
    /// deemed "ready". Ready objects can be used to send requests and can be
    /// used through other interfaces.
    ///
    /// Every ready wp_image_description_v1 protocol object refers to an
    /// underlying image description record in the compositor. Multiple protocol
    /// objects may end up referring to the same record. Clients may identify
    /// these "copies" by comparing their id numbers: if the numbers from two
    /// protocol objects are identical, the protocol objects refer to the same
    /// image description record. Two different image description records
    /// cannot have the same id number simultaneously. The id number does not
    /// change during the lifetime of the image description record.
    ///
    /// Image description id number is not a protocol object id. Zero is
    /// reserved as an invalid id number. It shall not be possible for a client
    /// to refer to an image description by its id number in protocol. The id
    /// numbers might not be portable between Wayland connections. A compositor
    /// shall not send an invalid id number.
    ///
    /// Compositors must not recycle image description id numbers.
    ///
    /// This identity allows clients to de-duplicate image description records
    /// and avoid get_information request if they already have the image
    /// description information.
    ///
    /// # Arguments
    ///
    /// - `identity_hi`: high 32 bits of the 64-bit image description id number
    /// - `identity_lo`: low 32 bits of the 64-bit image description id number
    #[inline]
    fn handle_ready2(
        &mut self,
        slf: &Rc<WpImageDescriptionV1>,
        identity_hi: u32,
        identity_lo: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_ready2(
            identity_hi,
            identity_lo,
        );
        if let Err(e) = res {
            log_forward("wp_image_description_v1.ready2", &e);
        }
    }
}

impl ObjectPrivate for WpImageDescriptionV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpImageDescriptionV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_image_description_v1#{}.destroy()\n", client_id, id);
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_image_description_v1#{}.get_information(information: wp_image_description_info_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WpImageDescriptionInfoV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "information", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_get_information(&self, arg0);
                } else {
                    DefaultHandler.handle_get_information(&self, arg0);
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
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("cause")));
                };
                offset += 1;
                let arg1;
                (arg1, offset) = parse_string::<NonNullString>(msg, offset, "msg")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                let arg0 = WpImageDescriptionV1Cause(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: WpImageDescriptionV1Cause, arg1: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_image_description_v1#{}.failed(cause: {:?}, msg: {:?})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_failed(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_failed(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_image_description_v1#{}.ready(identity: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_ready(&self, arg0);
                } else {
                    DefaultHandler.handle_ready(&self, arg0);
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
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_image_description_v1#{}.ready2(identity_hi: {}, identity_lo: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_ready2(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_ready2(&self, arg0, arg1);
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
            1 => "get_information",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "failed",
            1 => "ready",
            2 => "ready2",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WpImageDescriptionV1 {
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

impl WpImageDescriptionV1 {
    /// Since when the error.not_ready enum variant is available.
    pub const ENM__ERROR_NOT_READY__SINCE: u32 = 1;
    /// Since when the error.no_information enum variant is available.
    pub const ENM__ERROR_NO_INFORMATION__SINCE: u32 = 1;

    /// Since when the cause.low_version enum variant is available.
    pub const ENM__CAUSE_LOW_VERSION__SINCE: u32 = 1;
    /// Since when the cause.unsupported enum variant is available.
    pub const ENM__CAUSE_UNSUPPORTED__SINCE: u32 = 1;
    /// Since when the cause.operating_system enum variant is available.
    pub const ENM__CAUSE_OPERATING_SYSTEM__SINCE: u32 = 1;
    /// Since when the cause.no_output enum variant is available.
    pub const ENM__CAUSE_NO_OUTPUT__SINCE: u32 = 1;
}

/// protocol errors
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpImageDescriptionV1Error(pub u32);

impl WpImageDescriptionV1Error {
    /// attempted to use an object which is not ready
    pub const NOT_READY: Self = Self(0);

    /// get_information not allowed
    pub const NO_INFORMATION: Self = Self(1);
}

impl Debug for WpImageDescriptionV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NOT_READY => "NOT_READY",
            Self::NO_INFORMATION => "NO_INFORMATION",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// generic reason for failure
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpImageDescriptionV1Cause(pub u32);

impl WpImageDescriptionV1Cause {
    /// interface version too low
    pub const LOW_VERSION: Self = Self(0);

    /// unsupported image description data
    pub const UNSUPPORTED: Self = Self(1);

    /// error independent of the client
    pub const OPERATING_SYSTEM: Self = Self(2);

    /// the relevant output no longer exists
    pub const NO_OUTPUT: Self = Self(3);
}

impl Debug for WpImageDescriptionV1Cause {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::LOW_VERSION => "LOW_VERSION",
            Self::UNSUPPORTED => "UNSUPPORTED",
            Self::OPERATING_SYSTEM => "OPERATING_SYSTEM",
            Self::NO_OUTPUT => "NO_OUTPUT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
