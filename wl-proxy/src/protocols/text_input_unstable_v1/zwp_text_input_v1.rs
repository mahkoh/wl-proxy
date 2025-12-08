//! text input
//!
//! An object used for text input. Adds support for text input and input
//! methods to applications. A text_input object is created from a
//! wl_text_input_manager and corresponds typically to a text entry in an
//! application.
//!
//! Requests are used to activate/deactivate the text_input object and set
//! state information like surrounding and selected text or the content type.
//! The information about entered text is sent to the text_input object via
//! the pre-edit and commit events. Using this interface removes the need
//! for applications to directly process hardware key events and compose text
//! out of them.
//!
//! Text is generally UTF-8 encoded, indices and lengths are in bytes.
//!
//! Serials are used to synchronize the state between the text input and
//! an input method. New serials are sent by the text input in the
//! commit_state request and are used by the input method to indicate
//! the known text input state in events like preedit_string, commit_string,
//! and keysym. The text input can then ignore events from the input method
//! which are based on an outdated state (for example after a reset).
//!
//! Warning! The protocol described in this file is experimental and
//! backward incompatible changes may be made. Backward compatible changes
//! may be added together with the corresponding interface version bump.
//! Backward incompatible changes are done by bumping the version number in
//! the protocol and interface names and resetting the interface version.
//! Once the protocol is to be declared stable, the 'z' prefix and the
//! version number in the protocol and interface names are removed and the
//! interface version number is reset.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_text_input_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpTextInputV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpTextInputV1Handler>,
}

struct DefaultHandler;

impl ZwpTextInputV1Handler for DefaultHandler { }

impl ConcreteObject for ZwpTextInputV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpTextInputV1;
    const INTERFACE_NAME: &str = "zwp_text_input_v1";
}

impl ZwpTextInputV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpTextInputV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpTextInputV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpTextInputV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpTextInputV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpTextInputV1 {
    /// Since when the activate message is available.
    pub const MSG__ACTIVATE__SINCE: u32 = 1;

    /// request activation
    ///
    /// Requests the text_input object to be activated (typically when the
    /// text entry gets focus).
    ///
    /// The seat argument is a wl_seat which maintains the focus for this
    /// activation. The surface argument is a wl_surface assigned to the
    /// text_input object and tracked for focus lost. The enter event
    /// is emitted on successful activation.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `surface`:
    #[inline]
    pub fn try_send_activate(
        &self,
        seat: &Rc<WlSeat>,
        surface: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            seat,
            surface,
        );
        let arg0 = arg0.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("seat"))),
            Some(id) => id,
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_text_input_v1#{}.activate(seat: wl_seat#{}, surface: wl_surface#{})\n", id, arg0, arg1);
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
            0,
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// request activation
    ///
    /// Requests the text_input object to be activated (typically when the
    /// text entry gets focus).
    ///
    /// The seat argument is a wl_seat which maintains the focus for this
    /// activation. The surface argument is a wl_surface assigned to the
    /// text_input object and tracked for focus lost. The enter event
    /// is emitted on successful activation.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `surface`:
    #[inline]
    pub fn send_activate(
        &self,
        seat: &Rc<WlSeat>,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_activate(
            seat,
            surface,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.activate", &e);
        }
    }

    /// Since when the deactivate message is available.
    pub const MSG__DEACTIVATE__SINCE: u32 = 1;

    /// request deactivation
    ///
    /// Requests the text_input object to be deactivated (typically when the
    /// text entry lost focus). The seat argument is a wl_seat which was used
    /// for activation.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    #[inline]
    pub fn try_send_deactivate(
        &self,
        seat: &Rc<WlSeat>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            seat,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("seat"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_text_input_v1#{}.deactivate(seat: wl_seat#{})\n", id, arg0);
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

    /// request deactivation
    ///
    /// Requests the text_input object to be deactivated (typically when the
    /// text entry lost focus). The seat argument is a wl_seat which was used
    /// for activation.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    #[inline]
    pub fn send_deactivate(
        &self,
        seat: &Rc<WlSeat>,
    ) {
        let res = self.try_send_deactivate(
            seat,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.deactivate", &e);
        }
    }

    /// Since when the show_input_panel message is available.
    pub const MSG__SHOW_INPUT_PANEL__SINCE: u32 = 1;

    /// show input panels
    ///
    /// Requests input panels (virtual keyboard) to show.
    #[inline]
    pub fn try_send_show_input_panel(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_text_input_v1#{}.show_input_panel()\n", id);
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
            2,
        ]);
        Ok(())
    }

    /// show input panels
    ///
    /// Requests input panels (virtual keyboard) to show.
    #[inline]
    pub fn send_show_input_panel(
        &self,
    ) {
        let res = self.try_send_show_input_panel(
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.show_input_panel", &e);
        }
    }

    /// Since when the hide_input_panel message is available.
    pub const MSG__HIDE_INPUT_PANEL__SINCE: u32 = 1;

    /// hide input panels
    ///
    /// Requests input panels (virtual keyboard) to hide.
    #[inline]
    pub fn try_send_hide_input_panel(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_text_input_v1#{}.hide_input_panel()\n", id);
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
            3,
        ]);
        Ok(())
    }

    /// hide input panels
    ///
    /// Requests input panels (virtual keyboard) to hide.
    #[inline]
    pub fn send_hide_input_panel(
        &self,
    ) {
        let res = self.try_send_hide_input_panel(
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.hide_input_panel", &e);
        }
    }

    /// Since when the reset message is available.
    pub const MSG__RESET__SINCE: u32 = 1;

    /// reset
    ///
    /// Should be called by an editor widget when the input state should be
    /// reset, for example after the text was changed outside of the normal
    /// input method flow.
    #[inline]
    pub fn try_send_reset(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_text_input_v1#{}.reset()\n", id);
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

    /// reset
    ///
    /// Should be called by an editor widget when the input state should be
    /// reset, for example after the text was changed outside of the normal
    /// input method flow.
    #[inline]
    pub fn send_reset(
        &self,
    ) {
        let res = self.try_send_reset(
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.reset", &e);
        }
    }

    /// Since when the set_surrounding_text message is available.
    pub const MSG__SET_SURROUNDING_TEXT__SINCE: u32 = 1;

    /// sets the surrounding text
    ///
    /// Sets the plain surrounding text around the input position. Text is
    /// UTF-8 encoded. Cursor is the byte offset within the
    /// surrounding text. Anchor is the byte offset of the
    /// selection anchor within the surrounding text. If there is no selected
    /// text anchor, then it is the same as cursor.
    ///
    /// # Arguments
    ///
    /// - `text`:
    /// - `cursor`:
    /// - `anchor`:
    #[inline]
    pub fn try_send_set_surrounding_text(
        &self,
        text: &str,
        cursor: u32,
        anchor: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            text,
            cursor,
            anchor,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str, arg1: u32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_text_input_v1#{}.set_surrounding_text(text: {:?}, cursor: {}, anchor: {})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2);
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
        ]);
        fmt.string(arg0);
        fmt.words([
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// sets the surrounding text
    ///
    /// Sets the plain surrounding text around the input position. Text is
    /// UTF-8 encoded. Cursor is the byte offset within the
    /// surrounding text. Anchor is the byte offset of the
    /// selection anchor within the surrounding text. If there is no selected
    /// text anchor, then it is the same as cursor.
    ///
    /// # Arguments
    ///
    /// - `text`:
    /// - `cursor`:
    /// - `anchor`:
    #[inline]
    pub fn send_set_surrounding_text(
        &self,
        text: &str,
        cursor: u32,
        anchor: u32,
    ) {
        let res = self.try_send_set_surrounding_text(
            text,
            cursor,
            anchor,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.set_surrounding_text", &e);
        }
    }

    /// Since when the set_content_type message is available.
    pub const MSG__SET_CONTENT_TYPE__SINCE: u32 = 1;

    /// set content purpose and hint
    ///
    /// Sets the content purpose and content hint. While the purpose is the
    /// basic purpose of an input field, the hint flags allow to modify some
    /// of the behavior.
    ///
    /// When no content type is explicitly set, a normal content purpose with
    /// default hints (auto completion, auto correction, auto capitalization)
    /// should be assumed.
    ///
    /// # Arguments
    ///
    /// - `hint`:
    /// - `purpose`:
    #[inline]
    pub fn try_send_set_content_type(
        &self,
        hint: ZwpTextInputV1ContentHint,
        purpose: ZwpTextInputV1ContentPurpose,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            hint,
            purpose,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: ZwpTextInputV1ContentHint, arg1: ZwpTextInputV1ContentPurpose) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_text_input_v1#{}.set_content_type(hint: {:?}, purpose: {:?})\n", id, arg0, arg1);
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
            6,
            arg0.0,
            arg1.0,
        ]);
        Ok(())
    }

    /// set content purpose and hint
    ///
    /// Sets the content purpose and content hint. While the purpose is the
    /// basic purpose of an input field, the hint flags allow to modify some
    /// of the behavior.
    ///
    /// When no content type is explicitly set, a normal content purpose with
    /// default hints (auto completion, auto correction, auto capitalization)
    /// should be assumed.
    ///
    /// # Arguments
    ///
    /// - `hint`:
    /// - `purpose`:
    #[inline]
    pub fn send_set_content_type(
        &self,
        hint: ZwpTextInputV1ContentHint,
        purpose: ZwpTextInputV1ContentPurpose,
    ) {
        let res = self.try_send_set_content_type(
            hint,
            purpose,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.set_content_type", &e);
        }
    }

    /// Since when the set_cursor_rectangle message is available.
    pub const MSG__SET_CURSOR_RECTANGLE__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn try_send_set_cursor_rectangle(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            x,
            y,
            width,
            height,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_text_input_v1#{}.set_cursor_rectangle(x: {}, y: {}, width: {}, height: {})\n", id, arg0, arg1, arg2, arg3);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2, arg3);
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
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn send_set_cursor_rectangle(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_set_cursor_rectangle(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.set_cursor_rectangle", &e);
        }
    }

    /// Since when the set_preferred_language message is available.
    pub const MSG__SET_PREFERRED_LANGUAGE__SINCE: u32 = 1;

    /// sets preferred language
    ///
    /// Sets a specific language. This allows for example a virtual keyboard to
    /// show a language specific layout. The "language" argument is an RFC-3066
    /// format language tag.
    ///
    /// It could be used for example in a word processor to indicate the
    /// language of the currently edited document or in an instant message
    /// application which tracks languages of contacts.
    ///
    /// # Arguments
    ///
    /// - `language`:
    #[inline]
    pub fn try_send_set_preferred_language(
        &self,
        language: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            language,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_text_input_v1#{}.set_preferred_language(language: {:?})\n", id, arg0);
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
            8,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// sets preferred language
    ///
    /// Sets a specific language. This allows for example a virtual keyboard to
    /// show a language specific layout. The "language" argument is an RFC-3066
    /// format language tag.
    ///
    /// It could be used for example in a word processor to indicate the
    /// language of the currently edited document or in an instant message
    /// application which tracks languages of contacts.
    ///
    /// # Arguments
    ///
    /// - `language`:
    #[inline]
    pub fn send_set_preferred_language(
        &self,
        language: &str,
    ) {
        let res = self.try_send_set_preferred_language(
            language,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.set_preferred_language", &e);
        }
    }

    /// Since when the commit_state message is available.
    pub const MSG__COMMIT_STATE__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `serial`: used to identify the known state
    #[inline]
    pub fn try_send_commit_state(
        &self,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            serial,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_text_input_v1#{}.commit_state(serial: {})\n", id, arg0);
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
            9,
            arg0,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `serial`: used to identify the known state
    #[inline]
    pub fn send_commit_state(
        &self,
        serial: u32,
    ) {
        let res = self.try_send_commit_state(
            serial,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.commit_state", &e);
        }
    }

    /// Since when the invoke_action message is available.
    pub const MSG__INVOKE_ACTION__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `button`:
    /// - `index`:
    #[inline]
    pub fn try_send_invoke_action(
        &self,
        button: u32,
        index: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            button,
            index,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_text_input_v1#{}.invoke_action(button: {}, index: {})\n", id, arg0, arg1);
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
            10,
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `button`:
    /// - `index`:
    #[inline]
    pub fn send_invoke_action(
        &self,
        button: u32,
        index: u32,
    ) {
        let res = self.try_send_invoke_action(
            button,
            index,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.invoke_action", &e);
        }
    }

    /// Since when the enter message is available.
    pub const MSG__ENTER__SINCE: u32 = 1;

    /// enter event
    ///
    /// Notify the text_input object when it received focus. Typically in
    /// response to an activate request.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn try_send_enter(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            surface,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("surface", client.endpoint.id)));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_text_input_v1#{}.enter(surface: wl_surface#{})\n", client_id, id, arg0);
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
            0,
            arg0_id,
        ]);
        Ok(())
    }

    /// enter event
    ///
    /// Notify the text_input object when it received focus. Typically in
    /// response to an activate request.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn send_enter(
        &self,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_enter(
            surface,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.enter", &e);
        }
    }

    /// Since when the leave message is available.
    pub const MSG__LEAVE__SINCE: u32 = 1;

    /// leave event
    ///
    /// Notify the text_input object when it lost focus. Either in response
    /// to a deactivate request or when the assigned surface lost focus or was
    /// destroyed.
    #[inline]
    pub fn try_send_leave(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_text_input_v1#{}.leave()\n", client_id, id);
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

    /// leave event
    ///
    /// Notify the text_input object when it lost focus. Either in response
    /// to a deactivate request or when the assigned surface lost focus or was
    /// destroyed.
    #[inline]
    pub fn send_leave(
        &self,
    ) {
        let res = self.try_send_leave(
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.leave", &e);
        }
    }

    /// Since when the modifiers_map message is available.
    pub const MSG__MODIFIERS_MAP__SINCE: u32 = 1;

    /// modifiers map
    ///
    /// Transfer an array of 0-terminated modifier names. The position in
    /// the array is the index of the modifier as used in the modifiers
    /// bitmask in the keysym event.
    ///
    /// # Arguments
    ///
    /// - `map`:
    #[inline]
    pub fn try_send_modifiers_map(
        &self,
        map: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            map,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: &[u8]) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_text_input_v1#{}.modifiers_map(map: {})\n", client_id, id, debug_array(arg0));
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
        fmt.array(arg0);
        Ok(())
    }

    /// modifiers map
    ///
    /// Transfer an array of 0-terminated modifier names. The position in
    /// the array is the index of the modifier as used in the modifiers
    /// bitmask in the keysym event.
    ///
    /// # Arguments
    ///
    /// - `map`:
    #[inline]
    pub fn send_modifiers_map(
        &self,
        map: &[u8],
    ) {
        let res = self.try_send_modifiers_map(
            map,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.modifiers_map", &e);
        }
    }

    /// Since when the input_panel_state message is available.
    pub const MSG__INPUT_PANEL_STATE__SINCE: u32 = 1;

    /// state of the input panel
    ///
    /// Notify when the visibility state of the input panel changed.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_input_panel_state(
        &self,
        state: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            state,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_text_input_v1#{}.input_panel_state(state: {})\n", client_id, id, arg0);
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

    /// state of the input panel
    ///
    /// Notify when the visibility state of the input panel changed.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_input_panel_state(
        &self,
        state: u32,
    ) {
        let res = self.try_send_input_panel_state(
            state,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.input_panel_state", &e);
        }
    }

    /// Since when the preedit_string message is available.
    pub const MSG__PREEDIT_STRING__SINCE: u32 = 1;

    /// pre-edit
    ///
    /// Notify when a new composing text (pre-edit) should be set around the
    /// current cursor position. Any previously set composing text should
    /// be removed.
    ///
    /// The commit text can be used to replace the preedit text on reset
    /// (for example on unfocus).
    ///
    /// The text input should also handle all preedit_style and preedit_cursor
    /// events occurring directly before preedit_string.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `text`:
    /// - `commit`:
    #[inline]
    pub fn try_send_preedit_string(
        &self,
        serial: u32,
        text: &str,
        commit: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            serial,
            text,
            commit,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: &str, arg2: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_text_input_v1#{}.preedit_string(serial: {}, text: {:?}, commit: {:?})\n", client_id, id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1, arg2);
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
            arg0,
        ]);
        fmt.string(arg1);
        fmt.string(arg2);
        Ok(())
    }

    /// pre-edit
    ///
    /// Notify when a new composing text (pre-edit) should be set around the
    /// current cursor position. Any previously set composing text should
    /// be removed.
    ///
    /// The commit text can be used to replace the preedit text on reset
    /// (for example on unfocus).
    ///
    /// The text input should also handle all preedit_style and preedit_cursor
    /// events occurring directly before preedit_string.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `text`:
    /// - `commit`:
    #[inline]
    pub fn send_preedit_string(
        &self,
        serial: u32,
        text: &str,
        commit: &str,
    ) {
        let res = self.try_send_preedit_string(
            serial,
            text,
            commit,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.preedit_string", &e);
        }
    }

    /// Since when the preedit_styling message is available.
    pub const MSG__PREEDIT_STYLING__SINCE: u32 = 1;

    /// pre-edit styling
    ///
    /// Sets styling information on composing text. The style is applied for
    /// length bytes from index relative to the beginning of the composing
    /// text (as byte offset). Multiple styles can
    /// be applied to a composing text by sending multiple preedit_styling
    /// events.
    ///
    /// This event is handled as part of a following preedit_string event.
    ///
    /// # Arguments
    ///
    /// - `index`:
    /// - `length`:
    /// - `style`:
    #[inline]
    pub fn try_send_preedit_styling(
        &self,
        index: u32,
        length: u32,
        style: ZwpTextInputV1PreeditStyle,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            index,
            length,
            style,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: ZwpTextInputV1PreeditStyle) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_text_input_v1#{}.preedit_styling(index: {}, length: {}, style: {:?})\n", client_id, id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1, arg2);
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
            5,
            arg0,
            arg1,
            arg2.0,
        ]);
        Ok(())
    }

    /// pre-edit styling
    ///
    /// Sets styling information on composing text. The style is applied for
    /// length bytes from index relative to the beginning of the composing
    /// text (as byte offset). Multiple styles can
    /// be applied to a composing text by sending multiple preedit_styling
    /// events.
    ///
    /// This event is handled as part of a following preedit_string event.
    ///
    /// # Arguments
    ///
    /// - `index`:
    /// - `length`:
    /// - `style`:
    #[inline]
    pub fn send_preedit_styling(
        &self,
        index: u32,
        length: u32,
        style: ZwpTextInputV1PreeditStyle,
    ) {
        let res = self.try_send_preedit_styling(
            index,
            length,
            style,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.preedit_styling", &e);
        }
    }

    /// Since when the preedit_cursor message is available.
    pub const MSG__PREEDIT_CURSOR__SINCE: u32 = 1;

    /// pre-edit cursor
    ///
    /// Sets the cursor position inside the composing text (as byte
    /// offset) relative to the start of the composing text. When index is a
    /// negative number no cursor is shown.
    ///
    /// This event is handled as part of a following preedit_string event.
    ///
    /// # Arguments
    ///
    /// - `index`:
    #[inline]
    pub fn try_send_preedit_cursor(
        &self,
        index: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            index,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_text_input_v1#{}.preedit_cursor(index: {})\n", client_id, id, arg0);
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
            6,
            arg0 as u32,
        ]);
        Ok(())
    }

    /// pre-edit cursor
    ///
    /// Sets the cursor position inside the composing text (as byte
    /// offset) relative to the start of the composing text. When index is a
    /// negative number no cursor is shown.
    ///
    /// This event is handled as part of a following preedit_string event.
    ///
    /// # Arguments
    ///
    /// - `index`:
    #[inline]
    pub fn send_preedit_cursor(
        &self,
        index: i32,
    ) {
        let res = self.try_send_preedit_cursor(
            index,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.preedit_cursor", &e);
        }
    }

    /// Since when the commit_string message is available.
    pub const MSG__COMMIT_STRING__SINCE: u32 = 1;

    /// commit
    ///
    /// Notify when text should be inserted into the editor widget. The text to
    /// commit could be either just a single character after a key press or the
    /// result of some composing (pre-edit). It could also be an empty text
    /// when some text should be removed (see delete_surrounding_text) or when
    /// the input cursor should be moved (see cursor_position).
    ///
    /// Any previously set composing text should be removed.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `text`:
    #[inline]
    pub fn try_send_commit_string(
        &self,
        serial: u32,
        text: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            serial,
            text,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_text_input_v1#{}.commit_string(serial: {}, text: {:?})\n", client_id, id, arg0, arg1);
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
            7,
            arg0,
        ]);
        fmt.string(arg1);
        Ok(())
    }

    /// commit
    ///
    /// Notify when text should be inserted into the editor widget. The text to
    /// commit could be either just a single character after a key press or the
    /// result of some composing (pre-edit). It could also be an empty text
    /// when some text should be removed (see delete_surrounding_text) or when
    /// the input cursor should be moved (see cursor_position).
    ///
    /// Any previously set composing text should be removed.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `text`:
    #[inline]
    pub fn send_commit_string(
        &self,
        serial: u32,
        text: &str,
    ) {
        let res = self.try_send_commit_string(
            serial,
            text,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.commit_string", &e);
        }
    }

    /// Since when the cursor_position message is available.
    pub const MSG__CURSOR_POSITION__SINCE: u32 = 1;

    /// set cursor to new position
    ///
    /// Notify when the cursor or anchor position should be modified.
    ///
    /// This event should be handled as part of a following commit_string
    /// event.
    ///
    /// # Arguments
    ///
    /// - `index`:
    /// - `anchor`:
    #[inline]
    pub fn try_send_cursor_position(
        &self,
        index: i32,
        anchor: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            index,
            anchor,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_text_input_v1#{}.cursor_position(index: {}, anchor: {})\n", client_id, id, arg0, arg1);
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
            8,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// set cursor to new position
    ///
    /// Notify when the cursor or anchor position should be modified.
    ///
    /// This event should be handled as part of a following commit_string
    /// event.
    ///
    /// # Arguments
    ///
    /// - `index`:
    /// - `anchor`:
    #[inline]
    pub fn send_cursor_position(
        &self,
        index: i32,
        anchor: i32,
    ) {
        let res = self.try_send_cursor_position(
            index,
            anchor,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.cursor_position", &e);
        }
    }

    /// Since when the delete_surrounding_text message is available.
    pub const MSG__DELETE_SURROUNDING_TEXT__SINCE: u32 = 1;

    /// delete surrounding text
    ///
    /// Notify when the text around the current cursor position should be
    /// deleted.
    ///
    /// Index is relative to the current cursor (in bytes).
    /// Length is the length of deleted text (in bytes).
    ///
    /// This event should be handled as part of a following commit_string
    /// event.
    ///
    /// # Arguments
    ///
    /// - `index`:
    /// - `length`:
    #[inline]
    pub fn try_send_delete_surrounding_text(
        &self,
        index: i32,
        length: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            index,
            length,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_text_input_v1#{}.delete_surrounding_text(index: {}, length: {})\n", client_id, id, arg0, arg1);
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
            9,
            arg0 as u32,
            arg1,
        ]);
        Ok(())
    }

    /// delete surrounding text
    ///
    /// Notify when the text around the current cursor position should be
    /// deleted.
    ///
    /// Index is relative to the current cursor (in bytes).
    /// Length is the length of deleted text (in bytes).
    ///
    /// This event should be handled as part of a following commit_string
    /// event.
    ///
    /// # Arguments
    ///
    /// - `index`:
    /// - `length`:
    #[inline]
    pub fn send_delete_surrounding_text(
        &self,
        index: i32,
        length: u32,
    ) {
        let res = self.try_send_delete_surrounding_text(
            index,
            length,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.delete_surrounding_text", &e);
        }
    }

    /// Since when the keysym message is available.
    pub const MSG__KEYSYM__SINCE: u32 = 1;

    /// keysym
    ///
    /// Notify when a key event was sent. Key events should not be used
    /// for normal text input operations, which should be done with
    /// commit_string, delete_surrounding_text, etc. The key event follows
    /// the wl_keyboard key event convention. Sym is an XKB keysym, state a
    /// wl_keyboard key_state. Modifiers are a mask for effective modifiers
    /// (where the modifier indices are set by the modifiers_map event)
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `time`:
    /// - `sym`:
    /// - `state`:
    /// - `modifiers`:
    #[inline]
    pub fn try_send_keysym(
        &self,
        serial: u32,
        time: u32,
        sym: u32,
        state: u32,
        modifiers: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ) = (
            serial,
            time,
            sym,
            state,
            modifiers,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_text_input_v1#{}.keysym(serial: {}, time: {}, sym: {}, state: {}, modifiers: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1, arg2, arg3, arg4);
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
            10,
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ]);
        Ok(())
    }

    /// keysym
    ///
    /// Notify when a key event was sent. Key events should not be used
    /// for normal text input operations, which should be done with
    /// commit_string, delete_surrounding_text, etc. The key event follows
    /// the wl_keyboard key event convention. Sym is an XKB keysym, state a
    /// wl_keyboard key_state. Modifiers are a mask for effective modifiers
    /// (where the modifier indices are set by the modifiers_map event)
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `time`:
    /// - `sym`:
    /// - `state`:
    /// - `modifiers`:
    #[inline]
    pub fn send_keysym(
        &self,
        serial: u32,
        time: u32,
        sym: u32,
        state: u32,
        modifiers: u32,
    ) {
        let res = self.try_send_keysym(
            serial,
            time,
            sym,
            state,
            modifiers,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.keysym", &e);
        }
    }

    /// Since when the language message is available.
    pub const MSG__LANGUAGE__SINCE: u32 = 1;

    /// language
    ///
    /// Sets the language of the input text. The "language" argument is an
    /// RFC-3066 format language tag.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `language`:
    #[inline]
    pub fn try_send_language(
        &self,
        serial: u32,
        language: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            serial,
            language,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_text_input_v1#{}.language(serial: {}, language: {:?})\n", client_id, id, arg0, arg1);
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
            11,
            arg0,
        ]);
        fmt.string(arg1);
        Ok(())
    }

    /// language
    ///
    /// Sets the language of the input text. The "language" argument is an
    /// RFC-3066 format language tag.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `language`:
    #[inline]
    pub fn send_language(
        &self,
        serial: u32,
        language: &str,
    ) {
        let res = self.try_send_language(
            serial,
            language,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.language", &e);
        }
    }

    /// Since when the text_direction message is available.
    pub const MSG__TEXT_DIRECTION__SINCE: u32 = 1;

    /// text direction
    ///
    /// Sets the text direction of input text.
    ///
    /// It is mainly needed for showing an input cursor on the correct side of
    /// the editor when there is no input done yet and making sure neutral
    /// direction text is laid out properly.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `direction`:
    #[inline]
    pub fn try_send_text_direction(
        &self,
        serial: u32,
        direction: ZwpTextInputV1TextDirection,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            serial,
            direction,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: ZwpTextInputV1TextDirection) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_text_input_v1#{}.text_direction(serial: {}, direction: {:?})\n", client_id, id, arg0, arg1);
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
            12,
            arg0,
            arg1.0,
        ]);
        Ok(())
    }

    /// text direction
    ///
    /// Sets the text direction of input text.
    ///
    /// It is mainly needed for showing an input cursor on the correct side of
    /// the editor when there is no input done yet and making sure neutral
    /// direction text is laid out properly.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `direction`:
    #[inline]
    pub fn send_text_direction(
        &self,
        serial: u32,
        direction: ZwpTextInputV1TextDirection,
    ) {
        let res = self.try_send_text_direction(
            serial,
            direction,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v1.text_direction", &e);
        }
    }
}

/// A message handler for [`ZwpTextInputV1`] proxies.
pub trait ZwpTextInputV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpTextInputV1>) {
        slf.core.delete_id();
    }

    /// request activation
    ///
    /// Requests the text_input object to be activated (typically when the
    /// text entry gets focus).
    ///
    /// The seat argument is a wl_seat which maintains the focus for this
    /// activation. The surface argument is a wl_surface assigned to the
    /// text_input object and tracked for focus lost. The enter event
    /// is emitted on successful activation.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_activate(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
        seat: &Rc<WlSeat>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_activate(
            seat,
            surface,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.activate", &e);
        }
    }

    /// request deactivation
    ///
    /// Requests the text_input object to be deactivated (typically when the
    /// text entry lost focus). The seat argument is a wl_seat which was used
    /// for activation.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_deactivate(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
        seat: &Rc<WlSeat>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_deactivate(
            seat,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.deactivate", &e);
        }
    }

    /// show input panels
    ///
    /// Requests input panels (virtual keyboard) to show.
    #[inline]
    fn handle_show_input_panel(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_show_input_panel(
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.show_input_panel", &e);
        }
    }

    /// hide input panels
    ///
    /// Requests input panels (virtual keyboard) to hide.
    #[inline]
    fn handle_hide_input_panel(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_hide_input_panel(
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.hide_input_panel", &e);
        }
    }

    /// reset
    ///
    /// Should be called by an editor widget when the input state should be
    /// reset, for example after the text was changed outside of the normal
    /// input method flow.
    #[inline]
    fn handle_reset(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_reset(
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.reset", &e);
        }
    }

    /// sets the surrounding text
    ///
    /// Sets the plain surrounding text around the input position. Text is
    /// UTF-8 encoded. Cursor is the byte offset within the
    /// surrounding text. Anchor is the byte offset of the
    /// selection anchor within the surrounding text. If there is no selected
    /// text anchor, then it is the same as cursor.
    ///
    /// # Arguments
    ///
    /// - `text`:
    /// - `cursor`:
    /// - `anchor`:
    #[inline]
    fn handle_set_surrounding_text(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
        text: &str,
        cursor: u32,
        anchor: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_surrounding_text(
            text,
            cursor,
            anchor,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.set_surrounding_text", &e);
        }
    }

    /// set content purpose and hint
    ///
    /// Sets the content purpose and content hint. While the purpose is the
    /// basic purpose of an input field, the hint flags allow to modify some
    /// of the behavior.
    ///
    /// When no content type is explicitly set, a normal content purpose with
    /// default hints (auto completion, auto correction, auto capitalization)
    /// should be assumed.
    ///
    /// # Arguments
    ///
    /// - `hint`:
    /// - `purpose`:
    #[inline]
    fn handle_set_content_type(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
        hint: ZwpTextInputV1ContentHint,
        purpose: ZwpTextInputV1ContentPurpose,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_content_type(
            hint,
            purpose,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.set_content_type", &e);
        }
    }

    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    fn handle_set_cursor_rectangle(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_cursor_rectangle(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.set_cursor_rectangle", &e);
        }
    }

    /// sets preferred language
    ///
    /// Sets a specific language. This allows for example a virtual keyboard to
    /// show a language specific layout. The "language" argument is an RFC-3066
    /// format language tag.
    ///
    /// It could be used for example in a word processor to indicate the
    /// language of the currently edited document or in an instant message
    /// application which tracks languages of contacts.
    ///
    /// # Arguments
    ///
    /// - `language`:
    #[inline]
    fn handle_set_preferred_language(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
        language: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_preferred_language(
            language,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.set_preferred_language", &e);
        }
    }

    /// # Arguments
    ///
    /// - `serial`: used to identify the known state
    #[inline]
    fn handle_commit_state(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
        serial: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_commit_state(
            serial,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.commit_state", &e);
        }
    }

    /// # Arguments
    ///
    /// - `button`:
    /// - `index`:
    #[inline]
    fn handle_invoke_action(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
        button: u32,
        index: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_invoke_action(
            button,
            index,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.invoke_action", &e);
        }
    }

    /// enter event
    ///
    /// Notify the text_input object when it received focus. Typically in
    /// response to an activate request.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_enter(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = surface.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = slf.try_send_enter(
            surface,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.enter", &e);
        }
    }

    /// leave event
    ///
    /// Notify the text_input object when it lost focus. Either in response
    /// to a deactivate request or when the assigned surface lost focus or was
    /// destroyed.
    #[inline]
    fn handle_leave(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_leave(
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.leave", &e);
        }
    }

    /// modifiers map
    ///
    /// Transfer an array of 0-terminated modifier names. The position in
    /// the array is the index of the modifier as used in the modifiers
    /// bitmask in the keysym event.
    ///
    /// # Arguments
    ///
    /// - `map`:
    #[inline]
    fn handle_modifiers_map(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
        map: &[u8],
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_modifiers_map(
            map,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.modifiers_map", &e);
        }
    }

    /// state of the input panel
    ///
    /// Notify when the visibility state of the input panel changed.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_input_panel_state(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
        state: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_input_panel_state(
            state,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.input_panel_state", &e);
        }
    }

    /// pre-edit
    ///
    /// Notify when a new composing text (pre-edit) should be set around the
    /// current cursor position. Any previously set composing text should
    /// be removed.
    ///
    /// The commit text can be used to replace the preedit text on reset
    /// (for example on unfocus).
    ///
    /// The text input should also handle all preedit_style and preedit_cursor
    /// events occurring directly before preedit_string.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `text`:
    /// - `commit`:
    #[inline]
    fn handle_preedit_string(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
        serial: u32,
        text: &str,
        commit: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_preedit_string(
            serial,
            text,
            commit,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.preedit_string", &e);
        }
    }

    /// pre-edit styling
    ///
    /// Sets styling information on composing text. The style is applied for
    /// length bytes from index relative to the beginning of the composing
    /// text (as byte offset). Multiple styles can
    /// be applied to a composing text by sending multiple preedit_styling
    /// events.
    ///
    /// This event is handled as part of a following preedit_string event.
    ///
    /// # Arguments
    ///
    /// - `index`:
    /// - `length`:
    /// - `style`:
    #[inline]
    fn handle_preedit_styling(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
        index: u32,
        length: u32,
        style: ZwpTextInputV1PreeditStyle,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_preedit_styling(
            index,
            length,
            style,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.preedit_styling", &e);
        }
    }

    /// pre-edit cursor
    ///
    /// Sets the cursor position inside the composing text (as byte
    /// offset) relative to the start of the composing text. When index is a
    /// negative number no cursor is shown.
    ///
    /// This event is handled as part of a following preedit_string event.
    ///
    /// # Arguments
    ///
    /// - `index`:
    #[inline]
    fn handle_preedit_cursor(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
        index: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_preedit_cursor(
            index,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.preedit_cursor", &e);
        }
    }

    /// commit
    ///
    /// Notify when text should be inserted into the editor widget. The text to
    /// commit could be either just a single character after a key press or the
    /// result of some composing (pre-edit). It could also be an empty text
    /// when some text should be removed (see delete_surrounding_text) or when
    /// the input cursor should be moved (see cursor_position).
    ///
    /// Any previously set composing text should be removed.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `text`:
    #[inline]
    fn handle_commit_string(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
        serial: u32,
        text: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_commit_string(
            serial,
            text,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.commit_string", &e);
        }
    }

    /// set cursor to new position
    ///
    /// Notify when the cursor or anchor position should be modified.
    ///
    /// This event should be handled as part of a following commit_string
    /// event.
    ///
    /// # Arguments
    ///
    /// - `index`:
    /// - `anchor`:
    #[inline]
    fn handle_cursor_position(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
        index: i32,
        anchor: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_cursor_position(
            index,
            anchor,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.cursor_position", &e);
        }
    }

    /// delete surrounding text
    ///
    /// Notify when the text around the current cursor position should be
    /// deleted.
    ///
    /// Index is relative to the current cursor (in bytes).
    /// Length is the length of deleted text (in bytes).
    ///
    /// This event should be handled as part of a following commit_string
    /// event.
    ///
    /// # Arguments
    ///
    /// - `index`:
    /// - `length`:
    #[inline]
    fn handle_delete_surrounding_text(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
        index: i32,
        length: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_delete_surrounding_text(
            index,
            length,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.delete_surrounding_text", &e);
        }
    }

    /// keysym
    ///
    /// Notify when a key event was sent. Key events should not be used
    /// for normal text input operations, which should be done with
    /// commit_string, delete_surrounding_text, etc. The key event follows
    /// the wl_keyboard key event convention. Sym is an XKB keysym, state a
    /// wl_keyboard key_state. Modifiers are a mask for effective modifiers
    /// (where the modifier indices are set by the modifiers_map event)
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `time`:
    /// - `sym`:
    /// - `state`:
    /// - `modifiers`:
    #[inline]
    fn handle_keysym(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
        serial: u32,
        time: u32,
        sym: u32,
        state: u32,
        modifiers: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_keysym(
            serial,
            time,
            sym,
            state,
            modifiers,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.keysym", &e);
        }
    }

    /// language
    ///
    /// Sets the language of the input text. The "language" argument is an
    /// RFC-3066 format language tag.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `language`:
    #[inline]
    fn handle_language(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
        serial: u32,
        language: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_language(
            serial,
            language,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.language", &e);
        }
    }

    /// text direction
    ///
    /// Sets the text direction of input text.
    ///
    /// It is mainly needed for showing an input cursor on the correct side of
    /// the editor when there is no input done yet and making sure neutral
    /// direction text is laid out properly.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `direction`:
    #[inline]
    fn handle_text_direction(
        &mut self,
        slf: &Rc<ZwpTextInputV1>,
        serial: u32,
        direction: ZwpTextInputV1TextDirection,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_text_direction(
            serial,
            direction,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v1.text_direction", &e);
        }
    }
}

impl ObjectPrivate for ZwpTextInputV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpTextInputV1, version),
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_text_input_v1#{}.activate(seat: wl_seat#{}, surface: wl_surface#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("seat", o.core().interface, ObjectInterface::WlSeat)));
                };
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_activate(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_activate(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_text_input_v1#{}.deactivate(seat: wl_seat#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("seat", o.core().interface, ObjectInterface::WlSeat)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_deactivate(&self, arg0);
                } else {
                    DefaultHandler.handle_deactivate(&self, arg0);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_text_input_v1#{}.show_input_panel()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_show_input_panel(&self);
                } else {
                    DefaultHandler.handle_show_input_panel(&self);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_text_input_v1#{}.hide_input_panel()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_hide_input_panel(&self);
                } else {
                    DefaultHandler.handle_hide_input_panel(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_text_input_v1#{}.reset()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_reset(&self);
                } else {
                    DefaultHandler.handle_reset(&self);
                }
            }
            5 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "text")?;
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("cursor")));
                };
                offset += 1;
                let Some(&arg2) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("anchor")));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str, arg1: u32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_text_input_v1#{}.set_surrounding_text(text: {:?}, cursor: {}, anchor: {})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_surrounding_text(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_set_surrounding_text(&self, arg0, arg1, arg2);
                }
            }
            6 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = ZwpTextInputV1ContentHint(arg0);
                let arg1 = ZwpTextInputV1ContentPurpose(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: ZwpTextInputV1ContentHint, arg1: ZwpTextInputV1ContentPurpose) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_text_input_v1#{}.set_content_type(hint: {:?}, purpose: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_content_type(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_content_type(&self, arg0, arg1);
                }
            }
            7 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 24)));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_text_input_v1#{}.set_cursor_rectangle(x: {}, y: {}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_cursor_rectangle(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_set_cursor_rectangle(&self, arg0, arg1, arg2, arg3);
                }
            }
            8 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "language")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_text_input_v1#{}.set_preferred_language(language: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_preferred_language(&self, arg0);
                } else {
                    DefaultHandler.handle_set_preferred_language(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_text_input_v1#{}.commit_state(serial: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_commit_state(&self, arg0);
                } else {
                    DefaultHandler.handle_commit_state(&self, arg0);
                }
            }
            10 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_text_input_v1#{}.invoke_action(button: {}, index: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_invoke_action(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_invoke_action(&self, arg0, arg1);
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_text_input_v1#{}.enter(surface: wl_surface#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = server.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = server.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_enter(&self, arg0);
                } else {
                    DefaultHandler.handle_enter(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_text_input_v1#{}.leave()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_leave(&self);
                } else {
                    DefaultHandler.handle_leave(&self);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_array(msg, offset, "map")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_text_input_v1#{}.modifiers_map(map: {})\n", id, debug_array(arg0));
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_modifiers_map(&self, arg0);
                } else {
                    DefaultHandler.handle_modifiers_map(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_text_input_v1#{}.input_panel_state(state: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_input_panel_state(&self, arg0);
                } else {
                    DefaultHandler.handle_input_panel_state(&self, arg0);
                }
            }
            4 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("serial")));
                };
                offset += 1;
                let arg1;
                (arg1, offset) = parse_string::<NonNullString>(msg, offset, "text")?;
                let arg2;
                (arg2, offset) = parse_string::<NonNullString>(msg, offset, "commit")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: &str, arg2: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_text_input_v1#{}.preedit_string(serial: {}, text: {:?}, commit: {:?})\n", id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_preedit_string(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_preedit_string(&self, arg0, arg1, arg2);
                }
            }
            5 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                let arg2 = ZwpTextInputV1PreeditStyle(arg2);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: ZwpTextInputV1PreeditStyle) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_text_input_v1#{}.preedit_styling(index: {}, length: {}, style: {:?})\n", id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_preedit_styling(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_preedit_styling(&self, arg0, arg1, arg2);
                }
            }
            6 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = arg0 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_text_input_v1#{}.preedit_cursor(index: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_preedit_cursor(&self, arg0);
                } else {
                    DefaultHandler.handle_preedit_cursor(&self, arg0);
                }
            }
            7 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("serial")));
                };
                offset += 1;
                let arg1;
                (arg1, offset) = parse_string::<NonNullString>(msg, offset, "text")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_text_input_v1#{}.commit_string(serial: {}, text: {:?})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_commit_string(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_commit_string(&self, arg0, arg1);
                }
            }
            8 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_text_input_v1#{}.cursor_position(index: {}, anchor: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_cursor_position(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_cursor_position(&self, arg0, arg1);
                }
            }
            9 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = arg0 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_text_input_v1#{}.delete_surrounding_text(index: {}, length: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_delete_surrounding_text(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_delete_surrounding_text(&self, arg0, arg1);
                }
            }
            10 => {
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
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_text_input_v1#{}.keysym(serial: {}, time: {}, sym: {}, state: {}, modifiers: {})\n", id, arg0, arg1, arg2, arg3, arg4);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2, arg3, arg4);
                }
                if let Some(handler) = handler {
                    (**handler).handle_keysym(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultHandler.handle_keysym(&self, arg0, arg1, arg2, arg3, arg4);
                }
            }
            11 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("serial")));
                };
                offset += 1;
                let arg1;
                (arg1, offset) = parse_string::<NonNullString>(msg, offset, "language")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_text_input_v1#{}.language(serial: {}, language: {:?})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_language(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_language(&self, arg0, arg1);
                }
            }
            12 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = ZwpTextInputV1TextDirection(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: ZwpTextInputV1TextDirection) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_text_input_v1#{}.text_direction(serial: {}, direction: {:?})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_text_direction(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_text_direction(&self, arg0, arg1);
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
            0 => "activate",
            1 => "deactivate",
            2 => "show_input_panel",
            3 => "hide_input_panel",
            4 => "reset",
            5 => "set_surrounding_text",
            6 => "set_content_type",
            7 => "set_cursor_rectangle",
            8 => "set_preferred_language",
            9 => "commit_state",
            10 => "invoke_action",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "enter",
            1 => "leave",
            2 => "modifiers_map",
            3 => "input_panel_state",
            4 => "preedit_string",
            5 => "preedit_styling",
            6 => "preedit_cursor",
            7 => "commit_string",
            8 => "cursor_position",
            9 => "delete_surrounding_text",
            10 => "keysym",
            11 => "language",
            12 => "text_direction",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwpTextInputV1 {
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

impl ZwpTextInputV1 {
    /// Since when the content_hint.none enum variant is available.
    pub const ENM__CONTENT_HINT_NONE__SINCE: u32 = 1;
    /// Since when the content_hint.default enum variant is available.
    pub const ENM__CONTENT_HINT_DEFAULT__SINCE: u32 = 1;
    /// Since when the content_hint.password enum variant is available.
    pub const ENM__CONTENT_HINT_PASSWORD__SINCE: u32 = 1;
    /// Since when the content_hint.auto_completion enum variant is available.
    pub const ENM__CONTENT_HINT_AUTO_COMPLETION__SINCE: u32 = 1;
    /// Since when the content_hint.auto_correction enum variant is available.
    pub const ENM__CONTENT_HINT_AUTO_CORRECTION__SINCE: u32 = 1;
    /// Since when the content_hint.auto_capitalization enum variant is available.
    pub const ENM__CONTENT_HINT_AUTO_CAPITALIZATION__SINCE: u32 = 1;
    /// Since when the content_hint.lowercase enum variant is available.
    pub const ENM__CONTENT_HINT_LOWERCASE__SINCE: u32 = 1;
    /// Since when the content_hint.uppercase enum variant is available.
    pub const ENM__CONTENT_HINT_UPPERCASE__SINCE: u32 = 1;
    /// Since when the content_hint.titlecase enum variant is available.
    pub const ENM__CONTENT_HINT_TITLECASE__SINCE: u32 = 1;
    /// Since when the content_hint.hidden_text enum variant is available.
    pub const ENM__CONTENT_HINT_HIDDEN_TEXT__SINCE: u32 = 1;
    /// Since when the content_hint.sensitive_data enum variant is available.
    pub const ENM__CONTENT_HINT_SENSITIVE_DATA__SINCE: u32 = 1;
    /// Since when the content_hint.latin enum variant is available.
    pub const ENM__CONTENT_HINT_LATIN__SINCE: u32 = 1;
    /// Since when the content_hint.multiline enum variant is available.
    pub const ENM__CONTENT_HINT_MULTILINE__SINCE: u32 = 1;

    /// Since when the content_purpose.normal enum variant is available.
    pub const ENM__CONTENT_PURPOSE_NORMAL__SINCE: u32 = 1;
    /// Since when the content_purpose.alpha enum variant is available.
    pub const ENM__CONTENT_PURPOSE_ALPHA__SINCE: u32 = 1;
    /// Since when the content_purpose.digits enum variant is available.
    pub const ENM__CONTENT_PURPOSE_DIGITS__SINCE: u32 = 1;
    /// Since when the content_purpose.number enum variant is available.
    pub const ENM__CONTENT_PURPOSE_NUMBER__SINCE: u32 = 1;
    /// Since when the content_purpose.phone enum variant is available.
    pub const ENM__CONTENT_PURPOSE_PHONE__SINCE: u32 = 1;
    /// Since when the content_purpose.url enum variant is available.
    pub const ENM__CONTENT_PURPOSE_URL__SINCE: u32 = 1;
    /// Since when the content_purpose.email enum variant is available.
    pub const ENM__CONTENT_PURPOSE_EMAIL__SINCE: u32 = 1;
    /// Since when the content_purpose.name enum variant is available.
    pub const ENM__CONTENT_PURPOSE_NAME__SINCE: u32 = 1;
    /// Since when the content_purpose.password enum variant is available.
    pub const ENM__CONTENT_PURPOSE_PASSWORD__SINCE: u32 = 1;
    /// Since when the content_purpose.date enum variant is available.
    pub const ENM__CONTENT_PURPOSE_DATE__SINCE: u32 = 1;
    /// Since when the content_purpose.time enum variant is available.
    pub const ENM__CONTENT_PURPOSE_TIME__SINCE: u32 = 1;
    /// Since when the content_purpose.datetime enum variant is available.
    pub const ENM__CONTENT_PURPOSE_DATETIME__SINCE: u32 = 1;
    /// Since when the content_purpose.terminal enum variant is available.
    pub const ENM__CONTENT_PURPOSE_TERMINAL__SINCE: u32 = 1;

    /// Since when the preedit_style.default enum variant is available.
    pub const ENM__PREEDIT_STYLE_DEFAULT__SINCE: u32 = 1;
    /// Since when the preedit_style.none enum variant is available.
    pub const ENM__PREEDIT_STYLE_NONE__SINCE: u32 = 1;
    /// Since when the preedit_style.active enum variant is available.
    pub const ENM__PREEDIT_STYLE_ACTIVE__SINCE: u32 = 1;
    /// Since when the preedit_style.inactive enum variant is available.
    pub const ENM__PREEDIT_STYLE_INACTIVE__SINCE: u32 = 1;
    /// Since when the preedit_style.highlight enum variant is available.
    pub const ENM__PREEDIT_STYLE_HIGHLIGHT__SINCE: u32 = 1;
    /// Since when the preedit_style.underline enum variant is available.
    pub const ENM__PREEDIT_STYLE_UNDERLINE__SINCE: u32 = 1;
    /// Since when the preedit_style.selection enum variant is available.
    pub const ENM__PREEDIT_STYLE_SELECTION__SINCE: u32 = 1;
    /// Since when the preedit_style.incorrect enum variant is available.
    pub const ENM__PREEDIT_STYLE_INCORRECT__SINCE: u32 = 1;

    /// Since when the text_direction.auto enum variant is available.
    pub const ENM__TEXT_DIRECTION_AUTO__SINCE: u32 = 1;
    /// Since when the text_direction.ltr enum variant is available.
    pub const ENM__TEXT_DIRECTION_LTR__SINCE: u32 = 1;
    /// Since when the text_direction.rtl enum variant is available.
    pub const ENM__TEXT_DIRECTION_RTL__SINCE: u32 = 1;
}

/// content hint
///
/// Content hint is a bitmask to allow to modify the behavior of the text
/// input.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct ZwpTextInputV1ContentHint(pub u32);

/// An iterator over the set bits in a [`ZwpTextInputV1ContentHint`].
///
/// You can construct this with the `IntoIterator` implementation of `ZwpTextInputV1ContentHint`.
#[derive(Clone, Debug)]
pub struct ZwpTextInputV1ContentHintIter(pub u32);

impl ZwpTextInputV1ContentHint {
    /// no special behaviour
    pub const NONE: Self = Self(0x0);

    /// auto completion, correction and capitalization
    pub const DEFAULT: Self = Self(0x7);

    /// hidden and sensitive text
    pub const PASSWORD: Self = Self(0xc0);

    /// suggest word completions
    pub const AUTO_COMPLETION: Self = Self(0x1);

    /// suggest word corrections
    pub const AUTO_CORRECTION: Self = Self(0x2);

    /// switch to uppercase letters at the start of a sentence
    pub const AUTO_CAPITALIZATION: Self = Self(0x4);

    /// prefer lowercase letters
    pub const LOWERCASE: Self = Self(0x8);

    /// prefer uppercase letters
    pub const UPPERCASE: Self = Self(0x10);

    /// prefer casing for titles and headings (can be language dependent)
    pub const TITLECASE: Self = Self(0x20);

    /// characters should be hidden
    pub const HIDDEN_TEXT: Self = Self(0x40);

    /// typed text should not be stored
    pub const SENSITIVE_DATA: Self = Self(0x80);

    /// just latin characters should be entered
    pub const LATIN: Self = Self(0x100);

    /// the text input is multiline
    pub const MULTILINE: Self = Self(0x200);
}

impl ZwpTextInputV1ContentHint {
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    #[inline]
    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    #[inline]
    pub const fn insert(&mut self, other: Self) {
        *self = self.union(other);
    }

    #[inline]
    pub const fn remove(&mut self, other: Self) {
        *self = self.difference(other);
    }

    #[inline]
    pub const fn toggle(&mut self, other: Self) {
        *self = self.symmetric_difference(other);
    }

    #[inline]
    pub const fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }

    #[inline]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline]
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    #[inline]
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(!self.0)
    }

    #[inline]
    #[must_use]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    #[inline]
    pub const fn all_known() -> Self {
        #[allow(clippy::eq_op, clippy::identity_op)]
        Self(0 | 0x0 | 0x7 | 0xc0 | 0x1 | 0x2 | 0x4 | 0x8 | 0x10 | 0x20 | 0x40 | 0x80 | 0x100 | 0x200)
    }
}

impl Iterator for ZwpTextInputV1ContentHintIter {
    type Item = ZwpTextInputV1ContentHint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(ZwpTextInputV1ContentHint(bit))
    }
}

impl IntoIterator for ZwpTextInputV1ContentHint {
    type Item = ZwpTextInputV1ContentHint;
    type IntoIter = ZwpTextInputV1ContentHintIter;

    fn into_iter(self) -> Self::IntoIter {
        ZwpTextInputV1ContentHintIter(self.0)
    }
}

impl BitAnd for ZwpTextInputV1ContentHint {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for ZwpTextInputV1ContentHint {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for ZwpTextInputV1ContentHint {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for ZwpTextInputV1ContentHint {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for ZwpTextInputV1ContentHint {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for ZwpTextInputV1ContentHint {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for ZwpTextInputV1ContentHint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for ZwpTextInputV1ContentHint {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for ZwpTextInputV1ContentHint {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for ZwpTextInputV1ContentHint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = self.0;
        let mut first = true;
        if v & 0x7 == 0x7 {
            v &= !0x7;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("DEFAULT")?;
        }
        if v & 0xc0 == 0xc0 {
            v &= !0xc0;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("PASSWORD")?;
        }
        if v & 0x1 == 0x1 {
            v &= !0x1;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("AUTO_COMPLETION")?;
        }
        if v & 0x2 == 0x2 {
            v &= !0x2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("AUTO_CORRECTION")?;
        }
        if v & 0x4 == 0x4 {
            v &= !0x4;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("AUTO_CAPITALIZATION")?;
        }
        if v & 0x8 == 0x8 {
            v &= !0x8;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("LOWERCASE")?;
        }
        if v & 0x10 == 0x10 {
            v &= !0x10;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("UPPERCASE")?;
        }
        if v & 0x20 == 0x20 {
            v &= !0x20;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("TITLECASE")?;
        }
        if v & 0x40 == 0x40 {
            v &= !0x40;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("HIDDEN_TEXT")?;
        }
        if v & 0x80 == 0x80 {
            v &= !0x80;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("SENSITIVE_DATA")?;
        }
        if v & 0x100 == 0x100 {
            v &= !0x100;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("LATIN")?;
        }
        if v & 0x200 == 0x200 {
            v &= !0x200;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("MULTILINE")?;
        }
        if v != 0 {
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            write!(f, "0x{v:032x}")?;
        }
        if first {
            f.write_str("NONE")?;
        }
        Ok(())
    }
}

/// content purpose
///
/// The content purpose allows to specify the primary purpose of a text
/// input.
///
/// This allows an input method to show special purpose input panels with
/// extra characters or to disallow some characters.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwpTextInputV1ContentPurpose(pub u32);

impl ZwpTextInputV1ContentPurpose {
    /// default input, allowing all characters
    pub const NORMAL: Self = Self(0);

    /// allow only alphabetic characters
    pub const ALPHA: Self = Self(1);

    /// allow only digits
    pub const DIGITS: Self = Self(2);

    /// input a number (including decimal separator and sign)
    pub const NUMBER: Self = Self(3);

    /// input a phone number
    pub const PHONE: Self = Self(4);

    /// input an URL
    pub const URL: Self = Self(5);

    /// input an email address
    pub const EMAIL: Self = Self(6);

    /// input a name of a person
    pub const NAME: Self = Self(7);

    /// input a password (combine with password or sensitive_data hint)
    pub const PASSWORD: Self = Self(8);

    /// input a date
    pub const DATE: Self = Self(9);

    /// input a time
    pub const TIME: Self = Self(10);

    /// input a date and time
    pub const DATETIME: Self = Self(11);

    /// input for a terminal
    pub const TERMINAL: Self = Self(12);
}

impl Debug for ZwpTextInputV1ContentPurpose {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NORMAL => "NORMAL",
            Self::ALPHA => "ALPHA",
            Self::DIGITS => "DIGITS",
            Self::NUMBER => "NUMBER",
            Self::PHONE => "PHONE",
            Self::URL => "URL",
            Self::EMAIL => "EMAIL",
            Self::NAME => "NAME",
            Self::PASSWORD => "PASSWORD",
            Self::DATE => "DATE",
            Self::TIME => "TIME",
            Self::DATETIME => "DATETIME",
            Self::TERMINAL => "TERMINAL",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwpTextInputV1PreeditStyle(pub u32);

impl ZwpTextInputV1PreeditStyle {
    /// default style for composing text
    pub const DEFAULT: Self = Self(0);

    /// style should be the same as in non-composing text
    pub const NONE: Self = Self(1);

    pub const ACTIVE: Self = Self(2);

    pub const INACTIVE: Self = Self(3);

    pub const HIGHLIGHT: Self = Self(4);

    pub const UNDERLINE: Self = Self(5);

    pub const SELECTION: Self = Self(6);

    pub const INCORRECT: Self = Self(7);
}

impl Debug for ZwpTextInputV1PreeditStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DEFAULT => "DEFAULT",
            Self::NONE => "NONE",
            Self::ACTIVE => "ACTIVE",
            Self::INACTIVE => "INACTIVE",
            Self::HIGHLIGHT => "HIGHLIGHT",
            Self::UNDERLINE => "UNDERLINE",
            Self::SELECTION => "SELECTION",
            Self::INCORRECT => "INCORRECT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwpTextInputV1TextDirection(pub u32);

impl ZwpTextInputV1TextDirection {
    /// automatic text direction based on text and language
    pub const AUTO: Self = Self(0);

    /// left-to-right
    pub const LTR: Self = Self(1);

    /// right-to-left
    pub const RTL: Self = Self(2);
}

impl Debug for ZwpTextInputV1TextDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::AUTO => "AUTO",
            Self::LTR => "LTR",
            Self::RTL => "RTL",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
