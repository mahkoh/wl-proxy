//! holder of image description ICC information
//!
//! This type of object is used for collecting all the information required
//! to create a wp_image_description_v1 object from an ICC file. A complete
//! set of required parameters consists of these properties:
//! - ICC file
//!
//! Each required property must be set exactly once if the client is to create
//! an image description. The set requests verify that a property was not
//! already set. The create request verifies that all required properties are
//! set. There may be several alternative requests for setting each property,
//! and in that case the client must choose one of them.
//!
//! Once all properties have been set, the create request must be used to
//! create the image description object, destroying the creator in the
//! process.
//!
//! The link between a pixel value (a device value in ICC) and its respective
//! colorimetry is defined by the details of the particular ICC profile.
//! Those details also determine when colorimetry becomes undefined.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_image_description_creator_icc_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpImageDescriptionCreatorIccV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpImageDescriptionCreatorIccV1Handler>,
}

struct DefaultHandler;

impl WpImageDescriptionCreatorIccV1Handler for DefaultHandler { }

impl WpImageDescriptionCreatorIccV1 {
    pub const XML_VERSION: u32 = 2;
    pub const INTERFACE: ObjectInterface = ObjectInterface::WpImageDescriptionCreatorIccV1;
    pub const INTERFACE_NAME: &str = "wp_image_description_creator_icc_v1";
}

impl WpImageDescriptionCreatorIccV1 {
    pub fn set_handler(&self, handler: impl WpImageDescriptionCreatorIccV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn WpImageDescriptionCreatorIccV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpImageDescriptionCreatorIccV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpImageDescriptionCreatorIccV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpImageDescriptionCreatorIccV1 {
    /// Since when the create message is available.
    pub const MSG__CREATE__SINCE: u32 = 1;

    /// Create the image description object from ICC data
    ///
    /// Create an image description object based on the ICC information
    /// previously set on this object. A compositor must parse the ICC data in
    /// some undefined but finite amount of time.
    ///
    /// The completeness of the parameter set is verified. If the set is not
    /// complete, the protocol error incomplete_set is raised. For the
    /// definition of a complete set, see the description of this interface.
    ///
    /// If the particular combination of the information is not supported
    /// by the compositor, the resulting image description object shall
    /// immediately deliver the wp_image_description_v1.failed event with the
    /// 'unsupported' cause. If a valid image description was created from the
    /// information, the wp_image_description_v1.ready event will eventually
    /// be sent instead.
    ///
    /// This request destroys the wp_image_description_creator_icc_v1 object.
    ///
    /// The resulting image description object does not allow get_information
    /// request.
    #[inline]
    pub fn send_create(
        &self,
        image_description: &Rc<WpImageDescriptionV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            image_description,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("image_description", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_image_description_creator_icc_v1#{}.create(image_description: wp_image_description_v1#{})\n", id, arg0_id);
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
            arg0_id,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the set_icc_file message is available.
    pub const MSG__SET_ICC_FILE__SINCE: u32 = 1;

    /// set the ICC profile file
    ///
    /// Sets the ICC profile file to be used as the basis of the image
    /// description.
    ///
    /// The data shall be found through the given fd at the given offset, having
    /// the given length. The fd must be seekable and readable. Violating these
    /// requirements raises the bad_fd protocol error.
    ///
    /// If reading the data fails due to an error independent of the client, the
    /// compositor shall send the wp_image_description_v1.failed event on the
    /// created wp_image_description_v1 with the 'operating_system' cause.
    ///
    /// The maximum size of the ICC profile is 32 MB. If length is greater than
    /// that or zero, the protocol error bad_size is raised. If offset + length
    /// exceeds the file size, the protocol error out_of_file is raised.
    ///
    /// A compositor may read the file at any time starting from this request
    /// and only until whichever happens first:
    /// - If create request was issued, the wp_image_description_v1 object
    ///   delivers either failed or ready event; or
    /// - if create request was not issued, this
    ///   wp_image_description_creator_icc_v1 object is destroyed.
    ///
    /// A compositor shall not modify the contents of the file, and the fd may
    /// be sealed for writes and size changes. The client must ensure to its
    /// best ability that the data does not change while the compositor is
    /// reading it.
    ///
    /// The data must represent a valid ICC profile. The ICC profile version
    /// must be 2 or 4, it must be a 3 channel profile and the class must be
    /// Display or ColorSpace. Violating these requirements will not result in a
    /// protocol error, but will eventually send the
    /// wp_image_description_v1.failed event on the created
    /// wp_image_description_v1 with the 'unsupported' cause.
    ///
    /// See the International Color Consortium specification ICC.1:2022 for more
    /// details about ICC profiles.
    ///
    /// If ICC file has already been set on this object, the protocol error
    /// already_set is raised.
    ///
    /// # Arguments
    ///
    /// - `icc_profile`: ICC profile
    /// - `offset`: byte offset in fd to start of ICC data
    /// - `length`: length of ICC data in bytes
    #[inline]
    pub fn send_set_icc_file(
        &self,
        icc_profile: &Rc<OwnedFd>,
        offset: u32,
        length: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            icc_profile,
            offset,
            length,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_image_description_creator_icc_v1#{}.set_icc_file(icc_profile: {}, offset: {}, length: {})\n", id, arg0.as_raw_fd(), arg1, arg2);
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
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
            arg1,
            arg2,
        ]);
        Ok(())
    }
}

/// A message handler for [WpImageDescriptionCreatorIccV1] proxies.
pub trait WpImageDescriptionCreatorIccV1Handler: Any {
    /// Create the image description object from ICC data
    ///
    /// Create an image description object based on the ICC information
    /// previously set on this object. A compositor must parse the ICC data in
    /// some undefined but finite amount of time.
    ///
    /// The completeness of the parameter set is verified. If the set is not
    /// complete, the protocol error incomplete_set is raised. For the
    /// definition of a complete set, see the description of this interface.
    ///
    /// If the particular combination of the information is not supported
    /// by the compositor, the resulting image description object shall
    /// immediately deliver the wp_image_description_v1.failed event with the
    /// 'unsupported' cause. If a valid image description was created from the
    /// information, the wp_image_description_v1.ready event will eventually
    /// be sent instead.
    ///
    /// This request destroys the wp_image_description_creator_icc_v1 object.
    ///
    /// The resulting image description object does not allow get_information
    /// request.
    ///
    /// # Arguments
    ///
    /// - `image_description`:
    #[inline]
    fn create(
        &mut self,
        _slf: &Rc<WpImageDescriptionCreatorIccV1>,
        image_description: &Rc<WpImageDescriptionV1>,
    ) {
        let res = _slf.send_create(
            image_description,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_image_description_creator_icc_v1.create message: {}", Report::new(e));
        }
    }

    /// set the ICC profile file
    ///
    /// Sets the ICC profile file to be used as the basis of the image
    /// description.
    ///
    /// The data shall be found through the given fd at the given offset, having
    /// the given length. The fd must be seekable and readable. Violating these
    /// requirements raises the bad_fd protocol error.
    ///
    /// If reading the data fails due to an error independent of the client, the
    /// compositor shall send the wp_image_description_v1.failed event on the
    /// created wp_image_description_v1 with the 'operating_system' cause.
    ///
    /// The maximum size of the ICC profile is 32 MB. If length is greater than
    /// that or zero, the protocol error bad_size is raised. If offset + length
    /// exceeds the file size, the protocol error out_of_file is raised.
    ///
    /// A compositor may read the file at any time starting from this request
    /// and only until whichever happens first:
    /// - If create request was issued, the wp_image_description_v1 object
    ///   delivers either failed or ready event; or
    /// - if create request was not issued, this
    ///   wp_image_description_creator_icc_v1 object is destroyed.
    ///
    /// A compositor shall not modify the contents of the file, and the fd may
    /// be sealed for writes and size changes. The client must ensure to its
    /// best ability that the data does not change while the compositor is
    /// reading it.
    ///
    /// The data must represent a valid ICC profile. The ICC profile version
    /// must be 2 or 4, it must be a 3 channel profile and the class must be
    /// Display or ColorSpace. Violating these requirements will not result in a
    /// protocol error, but will eventually send the
    /// wp_image_description_v1.failed event on the created
    /// wp_image_description_v1 with the 'unsupported' cause.
    ///
    /// See the International Color Consortium specification ICC.1:2022 for more
    /// details about ICC profiles.
    ///
    /// If ICC file has already been set on this object, the protocol error
    /// already_set is raised.
    ///
    /// # Arguments
    ///
    /// - `icc_profile`: ICC profile
    /// - `offset`: byte offset in fd to start of ICC data
    /// - `length`: length of ICC data in bytes
    #[inline]
    fn set_icc_file(
        &mut self,
        _slf: &Rc<WpImageDescriptionCreatorIccV1>,
        icc_profile: &Rc<OwnedFd>,
        offset: u32,
        length: u32,
    ) {
        let res = _slf.send_set_icc_file(
            icc_profile,
            offset,
            length,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_image_description_creator_icc_v1.set_icc_file message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for WpImageDescriptionCreatorIccV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpImageDescriptionCreatorIccV1, version),
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_image_description_creator_icc_v1#{}.create(image_description: wp_image_description_v1#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = WpImageDescriptionV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "image_description", e))?;
                let arg0 = &arg0;
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).create(&self, arg0);
                } else {
                    DefaultHandler.create(&self, arg0);
                }
            }
            1 => {
                let [
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let Some(arg0) = fds.pop_front() else {
                    return Err(ObjectError::MissingFd("icc_profile"));
                };
                let arg0 = &arg0;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_image_description_creator_icc_v1#{}.set_icc_file(icc_profile: {}, offset: {}, length: {})\n", client.endpoint.id, msg[0], arg0.as_raw_fd(), arg1, arg2);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_icc_file(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.set_icc_file(&self, arg0, arg1, arg2);
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
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "create",
            1 => "set_icc_file",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WpImageDescriptionCreatorIccV1 {
    fn core(&self) -> &ObjectCore {
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

impl WpImageDescriptionCreatorIccV1 {
    /// Since when the error.incomplete_set enum variant is available.
    pub const ENM__ERROR_INCOMPLETE_SET__SINCE: u32 = 1;
    /// Since when the error.already_set enum variant is available.
    pub const ENM__ERROR_ALREADY_SET__SINCE: u32 = 1;
    /// Since when the error.bad_fd enum variant is available.
    pub const ENM__ERROR_BAD_FD__SINCE: u32 = 1;
    /// Since when the error.bad_size enum variant is available.
    pub const ENM__ERROR_BAD_SIZE__SINCE: u32 = 1;
    /// Since when the error.out_of_file enum variant is available.
    pub const ENM__ERROR_OUT_OF_FILE__SINCE: u32 = 1;
}

/// protocol errors
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpImageDescriptionCreatorIccV1Error(pub u32);

impl WpImageDescriptionCreatorIccV1Error {
    /// incomplete parameter set
    pub const INCOMPLETE_SET: Self = Self(0);

    /// property already set
    pub const ALREADY_SET: Self = Self(1);

    /// fd not seekable and readable
    pub const BAD_FD: Self = Self(2);

    /// no or too much data
    pub const BAD_SIZE: Self = Self(3);

    /// offset + length exceeds file size
    pub const OUT_OF_FILE: Self = Self(4);
}

impl Debug for WpImageDescriptionCreatorIccV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INCOMPLETE_SET => "INCOMPLETE_SET",
            Self::ALREADY_SET => "ALREADY_SET",
            Self::BAD_FD => "BAD_FD",
            Self::BAD_SIZE => "BAD_SIZE",
            Self::OUT_OF_FILE => "OUT_OF_FILE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
