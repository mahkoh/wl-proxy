//! input method context
//!
//! Corresponds to a text input on the input method side. An input method context
//! is created on text input activation on the input method side. It allows
//! receiving information about the text input from the application via events.
//! Input method contexts do not keep state after deactivation and should be
//! destroyed after deactivation is handled.
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

/// A zwp_input_method_context_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpInputMethodContextV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpInputMethodContextV1Handler>,
}

struct DefaultHandler;

impl ZwpInputMethodContextV1Handler for DefaultHandler { }

impl ConcreteObject for ZwpInputMethodContextV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpInputMethodContextV1;
    const INTERFACE_NAME: &str = "zwp_input_method_context_v1";
}

impl ZwpInputMethodContextV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpInputMethodContextV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpInputMethodContextV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpInputMethodContextV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpInputMethodContextV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpInputMethodContextV1 {
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_context_v1#{}.destroy()\n", id);
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
            log_send("zwp_input_method_context_v1.destroy", &e);
        }
    }

    /// Since when the commit_string message is available.
    pub const MSG__COMMIT_STRING__SINCE: u32 = 1;

    /// commit string
    ///
    /// Send the commit string text for insertion to the application.
    ///
    /// The text to commit could be either just a single character after a key
    /// press or the result of some composing (pre-edit). It could be also an
    /// empty text when some text should be removed (see
    /// delete_surrounding_text) or when the input cursor should be moved (see
    /// cursor_position).
    ///
    /// Any previously set composing text will be removed.
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_context_v1#{}.commit_string(serial: {}, text: {:?})\n", id, arg0, arg1);
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
            1,
            arg0,
        ]);
        fmt.string(arg1);
        Ok(())
    }

    /// commit string
    ///
    /// Send the commit string text for insertion to the application.
    ///
    /// The text to commit could be either just a single character after a key
    /// press or the result of some composing (pre-edit). It could be also an
    /// empty text when some text should be removed (see
    /// delete_surrounding_text) or when the input cursor should be moved (see
    /// cursor_position).
    ///
    /// Any previously set composing text will be removed.
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
            log_send("zwp_input_method_context_v1.commit_string", &e);
        }
    }

    /// Since when the preedit_string message is available.
    pub const MSG__PREEDIT_STRING__SINCE: u32 = 1;

    /// pre-edit string
    ///
    /// Send the pre-edit string text to the application text input.
    ///
    /// The commit text can be used to replace the pre-edit text on reset (for
    /// example on unfocus).
    ///
    /// Previously sent preedit_style and preedit_cursor requests are also
    /// processed by the text_input.
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: &str, arg2: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_context_v1#{}.preedit_string(serial: {}, text: {:?}, commit: {:?})\n", id, arg0, arg1, arg2);
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
            2,
            arg0,
        ]);
        fmt.string(arg1);
        fmt.string(arg2);
        Ok(())
    }

    /// pre-edit string
    ///
    /// Send the pre-edit string text to the application text input.
    ///
    /// The commit text can be used to replace the pre-edit text on reset (for
    /// example on unfocus).
    ///
    /// Previously sent preedit_style and preedit_cursor requests are also
    /// processed by the text_input.
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
            log_send("zwp_input_method_context_v1.preedit_string", &e);
        }
    }

    /// Since when the preedit_styling message is available.
    pub const MSG__PREEDIT_STYLING__SINCE: u32 = 1;

    /// pre-edit styling
    ///
    /// Set the styling information on composing text. The style is applied for
    /// length in bytes from index relative to the beginning of
    /// the composing text (as byte offset). Multiple styles can
    /// be applied to a composing text.
    ///
    /// This request should be sent before sending a preedit_string request.
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
        style: u32,
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_context_v1#{}.preedit_styling(index: {}, length: {}, style: {})\n", id, arg0, arg1, arg2);
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
            3,
            arg0,
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// pre-edit styling
    ///
    /// Set the styling information on composing text. The style is applied for
    /// length in bytes from index relative to the beginning of
    /// the composing text (as byte offset). Multiple styles can
    /// be applied to a composing text.
    ///
    /// This request should be sent before sending a preedit_string request.
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
        style: u32,
    ) {
        let res = self.try_send_preedit_styling(
            index,
            length,
            style,
        );
        if let Err(e) = res {
            log_send("zwp_input_method_context_v1.preedit_styling", &e);
        }
    }

    /// Since when the preedit_cursor message is available.
    pub const MSG__PREEDIT_CURSOR__SINCE: u32 = 1;

    /// pre-edit cursor
    ///
    /// Set the cursor position inside the composing text (as byte offset)
    /// relative to the start of the composing text.
    ///
    /// When index is negative no cursor should be displayed.
    ///
    /// This request should be sent before sending a preedit_string request.
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_context_v1#{}.preedit_cursor(index: {})\n", id, arg0);
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
            arg0 as u32,
        ]);
        Ok(())
    }

    /// pre-edit cursor
    ///
    /// Set the cursor position inside the composing text (as byte offset)
    /// relative to the start of the composing text.
    ///
    /// When index is negative no cursor should be displayed.
    ///
    /// This request should be sent before sending a preedit_string request.
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
            log_send("zwp_input_method_context_v1.preedit_cursor", &e);
        }
    }

    /// Since when the delete_surrounding_text message is available.
    pub const MSG__DELETE_SURROUNDING_TEXT__SINCE: u32 = 1;

    /// delete text
    ///
    /// Remove the surrounding text.
    ///
    /// This request will be handled on the text_input side directly following
    /// a commit_string request.
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_context_v1#{}.delete_surrounding_text(index: {}, length: {})\n", id, arg0, arg1);
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
            5,
            arg0 as u32,
            arg1,
        ]);
        Ok(())
    }

    /// delete text
    ///
    /// Remove the surrounding text.
    ///
    /// This request will be handled on the text_input side directly following
    /// a commit_string request.
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
            log_send("zwp_input_method_context_v1.delete_surrounding_text", &e);
        }
    }

    /// Since when the cursor_position message is available.
    pub const MSG__CURSOR_POSITION__SINCE: u32 = 1;

    /// set cursor to a new position
    ///
    /// Set the cursor and anchor to a new position. Index is the new cursor
    /// position in bytes (when >= 0 this is relative to the end of the inserted text,
    /// otherwise it is relative to the beginning of the inserted text). Anchor is
    /// the new anchor position in bytes (when >= 0 this is relative to the end of the
    /// inserted text, otherwise it is relative to the beginning of the inserted
    /// text). When there should be no selected text, anchor should be the same
    /// as index.
    ///
    /// This request will be handled on the text_input side directly following
    /// a commit_string request.
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_context_v1#{}.cursor_position(index: {}, anchor: {})\n", id, arg0, arg1);
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
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// set cursor to a new position
    ///
    /// Set the cursor and anchor to a new position. Index is the new cursor
    /// position in bytes (when >= 0 this is relative to the end of the inserted text,
    /// otherwise it is relative to the beginning of the inserted text). Anchor is
    /// the new anchor position in bytes (when >= 0 this is relative to the end of the
    /// inserted text, otherwise it is relative to the beginning of the inserted
    /// text). When there should be no selected text, anchor should be the same
    /// as index.
    ///
    /// This request will be handled on the text_input side directly following
    /// a commit_string request.
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
            log_send("zwp_input_method_context_v1.cursor_position", &e);
        }
    }

    /// Since when the modifiers_map message is available.
    pub const MSG__MODIFIERS_MAP__SINCE: u32 = 1;

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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &[u8]) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_context_v1#{}.modifiers_map(map: {})\n", id, debug_array(arg0));
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
        fmt.array(arg0);
        Ok(())
    }

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
            log_send("zwp_input_method_context_v1.modifiers_map", &e);
        }
    }

    /// Since when the keysym message is available.
    pub const MSG__KEYSYM__SINCE: u32 = 1;

    /// keysym
    ///
    /// Notify when a key event was sent. Key events should not be used for
    /// normal text input operations, which should be done with commit_string,
    /// delete_surrounding_text, etc. The key event follows the wl_keyboard key
    /// event convention. Sym is an XKB keysym, state is a wl_keyboard key_state.
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_context_v1#{}.keysym(serial: {}, time: {}, sym: {}, state: {}, modifiers: {})\n", id, arg0, arg1, arg2, arg3, arg4);
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
            8,
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
    /// Notify when a key event was sent. Key events should not be used for
    /// normal text input operations, which should be done with commit_string,
    /// delete_surrounding_text, etc. The key event follows the wl_keyboard key
    /// event convention. Sym is an XKB keysym, state is a wl_keyboard key_state.
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
            log_send("zwp_input_method_context_v1.keysym", &e);
        }
    }

    /// Since when the grab_keyboard message is available.
    pub const MSG__GRAB_KEYBOARD__SINCE: u32 = 1;

    /// grab hardware keyboard
    ///
    /// Allow an input method to receive hardware keyboard input and process
    /// key events to generate text events (with pre-edit) over the wire. This
    /// allows input methods which compose multiple key events for inputting
    /// text like it is done for CJK languages.
    ///
    /// # Arguments
    ///
    /// - `keyboard`:
    #[inline]
    pub fn try_send_grab_keyboard(
        &self,
        keyboard: &Rc<WlKeyboard>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            keyboard,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("keyboard", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_context_v1#{}.grab_keyboard(keyboard: wl_keyboard#{})\n", id, arg0);
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

    /// grab hardware keyboard
    ///
    /// Allow an input method to receive hardware keyboard input and process
    /// key events to generate text events (with pre-edit) over the wire. This
    /// allows input methods which compose multiple key events for inputting
    /// text like it is done for CJK languages.
    ///
    /// # Arguments
    ///
    /// - `keyboard`:
    #[inline]
    pub fn send_grab_keyboard(
        &self,
        keyboard: &Rc<WlKeyboard>,
    ) {
        let res = self.try_send_grab_keyboard(
            keyboard,
        );
        if let Err(e) = res {
            log_send("zwp_input_method_context_v1.grab_keyboard", &e);
        }
    }

    /// grab hardware keyboard
    ///
    /// Allow an input method to receive hardware keyboard input and process
    /// key events to generate text events (with pre-edit) over the wire. This
    /// allows input methods which compose multiple key events for inputting
    /// text like it is done for CJK languages.
    #[inline]
    pub fn new_try_send_grab_keyboard(
        &self,
    ) -> Result<Rc<WlKeyboard>, ObjectError> {
        let keyboard = self.core.create_child();
        self.try_send_grab_keyboard(
            &keyboard,
        )?;
        Ok(keyboard)
    }

    /// grab hardware keyboard
    ///
    /// Allow an input method to receive hardware keyboard input and process
    /// key events to generate text events (with pre-edit) over the wire. This
    /// allows input methods which compose multiple key events for inputting
    /// text like it is done for CJK languages.
    #[inline]
    pub fn new_send_grab_keyboard(
        &self,
    ) -> Rc<WlKeyboard> {
        let keyboard = self.core.create_child();
        self.send_grab_keyboard(
            &keyboard,
        );
        keyboard
    }

    /// Since when the key message is available.
    pub const MSG__KEY__SINCE: u32 = 1;

    /// forward key event
    ///
    /// Forward a wl_keyboard::key event to the client that was not processed
    /// by the input method itself. Should be used when filtering key events
    /// with grab_keyboard.  The arguments should be the ones from the
    /// wl_keyboard::key event.
    ///
    /// For generating custom key events use the keysym request instead.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial from wl_keyboard::key
    /// - `time`: time from wl_keyboard::key
    /// - `key`: key from wl_keyboard::key
    /// - `state`: state from wl_keyboard::key
    #[inline]
    pub fn try_send_key(
        &self,
        serial: u32,
        time: u32,
        key: u32,
        state: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            serial,
            time,
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
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_context_v1#{}.key(serial: {}, time: {}, key: {}, state: {})\n", id, arg0, arg1, arg2, arg3);
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
            10,
            arg0,
            arg1,
            arg2,
            arg3,
        ]);
        Ok(())
    }

    /// forward key event
    ///
    /// Forward a wl_keyboard::key event to the client that was not processed
    /// by the input method itself. Should be used when filtering key events
    /// with grab_keyboard.  The arguments should be the ones from the
    /// wl_keyboard::key event.
    ///
    /// For generating custom key events use the keysym request instead.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial from wl_keyboard::key
    /// - `time`: time from wl_keyboard::key
    /// - `key`: key from wl_keyboard::key
    /// - `state`: state from wl_keyboard::key
    #[inline]
    pub fn send_key(
        &self,
        serial: u32,
        time: u32,
        key: u32,
        state: u32,
    ) {
        let res = self.try_send_key(
            serial,
            time,
            key,
            state,
        );
        if let Err(e) = res {
            log_send("zwp_input_method_context_v1.key", &e);
        }
    }

    /// Since when the modifiers message is available.
    pub const MSG__MODIFIERS__SINCE: u32 = 1;

    /// forward modifiers event
    ///
    /// Forward a wl_keyboard::modifiers event to the client that was not
    /// processed by the input method itself.  Should be used when filtering
    /// key events with grab_keyboard. The arguments should be the ones
    /// from the wl_keyboard::modifiers event.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial from wl_keyboard::modifiers
    /// - `mods_depressed`: mods_depressed from wl_keyboard::modifiers
    /// - `mods_latched`: mods_latched from wl_keyboard::modifiers
    /// - `mods_locked`: mods_locked from wl_keyboard::modifiers
    /// - `group`: group from wl_keyboard::modifiers
    #[inline]
    pub fn try_send_modifiers(
        &self,
        serial: u32,
        mods_depressed: u32,
        mods_latched: u32,
        mods_locked: u32,
        group: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ) = (
            serial,
            mods_depressed,
            mods_latched,
            mods_locked,
            group,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_context_v1#{}.modifiers(serial: {}, mods_depressed: {}, mods_latched: {}, mods_locked: {}, group: {})\n", id, arg0, arg1, arg2, arg3, arg4);
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
            11,
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ]);
        Ok(())
    }

    /// forward modifiers event
    ///
    /// Forward a wl_keyboard::modifiers event to the client that was not
    /// processed by the input method itself.  Should be used when filtering
    /// key events with grab_keyboard. The arguments should be the ones
    /// from the wl_keyboard::modifiers event.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial from wl_keyboard::modifiers
    /// - `mods_depressed`: mods_depressed from wl_keyboard::modifiers
    /// - `mods_latched`: mods_latched from wl_keyboard::modifiers
    /// - `mods_locked`: mods_locked from wl_keyboard::modifiers
    /// - `group`: group from wl_keyboard::modifiers
    #[inline]
    pub fn send_modifiers(
        &self,
        serial: u32,
        mods_depressed: u32,
        mods_latched: u32,
        mods_locked: u32,
        group: u32,
    ) {
        let res = self.try_send_modifiers(
            serial,
            mods_depressed,
            mods_latched,
            mods_locked,
            group,
        );
        if let Err(e) = res {
            log_send("zwp_input_method_context_v1.modifiers", &e);
        }
    }

    /// Since when the language message is available.
    pub const MSG__LANGUAGE__SINCE: u32 = 1;

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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_context_v1#{}.language(serial: {}, language: {:?})\n", id, arg0, arg1);
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
            12,
            arg0,
        ]);
        fmt.string(arg1);
        Ok(())
    }

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
            log_send("zwp_input_method_context_v1.language", &e);
        }
    }

    /// Since when the text_direction message is available.
    pub const MSG__TEXT_DIRECTION__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `direction`:
    #[inline]
    pub fn try_send_text_direction(
        &self,
        serial: u32,
        direction: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            serial,
            direction,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_context_v1#{}.text_direction(serial: {}, direction: {})\n", id, arg0, arg1);
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
            13,
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `direction`:
    #[inline]
    pub fn send_text_direction(
        &self,
        serial: u32,
        direction: u32,
    ) {
        let res = self.try_send_text_direction(
            serial,
            direction,
        );
        if let Err(e) = res {
            log_send("zwp_input_method_context_v1.text_direction", &e);
        }
    }

    /// Since when the surrounding_text message is available.
    pub const MSG__SURROUNDING_TEXT__SINCE: u32 = 1;

    /// surrounding text event
    ///
    /// The plain surrounding text around the input position. Cursor is the
    /// position in bytes within the surrounding text relative to the beginning
    /// of the text. Anchor is the position in bytes of the selection anchor
    /// within the surrounding text relative to the beginning of the text. If
    /// there is no selected text then anchor is the same as cursor.
    ///
    /// # Arguments
    ///
    /// - `text`:
    /// - `cursor`:
    /// - `anchor`:
    #[inline]
    pub fn try_send_surrounding_text(
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
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: &str, arg1: u32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_context_v1#{}.surrounding_text(text: {:?}, cursor: {}, anchor: {})\n", client_id, id, arg0, arg1, arg2);
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
            0,
        ]);
        fmt.string(arg0);
        fmt.words([
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// surrounding text event
    ///
    /// The plain surrounding text around the input position. Cursor is the
    /// position in bytes within the surrounding text relative to the beginning
    /// of the text. Anchor is the position in bytes of the selection anchor
    /// within the surrounding text relative to the beginning of the text. If
    /// there is no selected text then anchor is the same as cursor.
    ///
    /// # Arguments
    ///
    /// - `text`:
    /// - `cursor`:
    /// - `anchor`:
    #[inline]
    pub fn send_surrounding_text(
        &self,
        text: &str,
        cursor: u32,
        anchor: u32,
    ) {
        let res = self.try_send_surrounding_text(
            text,
            cursor,
            anchor,
        );
        if let Err(e) = res {
            log_send("zwp_input_method_context_v1.surrounding_text", &e);
        }
    }

    /// Since when the reset message is available.
    pub const MSG__RESET__SINCE: u32 = 1;

    #[inline]
    pub fn try_send_reset(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_context_v1#{}.reset()\n", client_id, id);
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

    #[inline]
    pub fn send_reset(
        &self,
    ) {
        let res = self.try_send_reset(
        );
        if let Err(e) = res {
            log_send("zwp_input_method_context_v1.reset", &e);
        }
    }

    /// Since when the content_type message is available.
    pub const MSG__CONTENT_TYPE__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `hint`:
    /// - `purpose`:
    #[inline]
    pub fn try_send_content_type(
        &self,
        hint: u32,
        purpose: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            hint,
            purpose,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_context_v1#{}.content_type(hint: {}, purpose: {})\n", client_id, id, arg0, arg1);
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

    /// # Arguments
    ///
    /// - `hint`:
    /// - `purpose`:
    #[inline]
    pub fn send_content_type(
        &self,
        hint: u32,
        purpose: u32,
    ) {
        let res = self.try_send_content_type(
            hint,
            purpose,
        );
        if let Err(e) = res {
            log_send("zwp_input_method_context_v1.content_type", &e);
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_context_v1#{}.invoke_action(button: {}, index: {})\n", client_id, id, arg0, arg1);
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
            3,
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
            log_send("zwp_input_method_context_v1.invoke_action", &e);
        }
    }

    /// Since when the commit_state message is available.
    pub const MSG__COMMIT_STATE__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `serial`: serial of text input state
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_context_v1#{}.commit_state(serial: {})\n", client_id, id, arg0);
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
            arg0,
        ]);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `serial`: serial of text input state
    #[inline]
    pub fn send_commit_state(
        &self,
        serial: u32,
    ) {
        let res = self.try_send_commit_state(
            serial,
        );
        if let Err(e) = res {
            log_send("zwp_input_method_context_v1.commit_state", &e);
        }
    }

    /// Since when the preferred_language message is available.
    pub const MSG__PREFERRED_LANGUAGE__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `language`:
    #[inline]
    pub fn try_send_preferred_language(
        &self,
        language: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_context_v1#{}.preferred_language(language: {:?})\n", client_id, id, arg0);
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
            5,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// # Arguments
    ///
    /// - `language`:
    #[inline]
    pub fn send_preferred_language(
        &self,
        language: &str,
    ) {
        let res = self.try_send_preferred_language(
            language,
        );
        if let Err(e) = res {
            log_send("zwp_input_method_context_v1.preferred_language", &e);
        }
    }
}

/// A message handler for [`ZwpInputMethodContextV1`] proxies.
pub trait ZwpInputMethodContextV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpInputMethodContextV1>) {
        slf.core.delete_id();
    }

    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwpInputMethodContextV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_context_v1.destroy", &e);
        }
    }

    /// commit string
    ///
    /// Send the commit string text for insertion to the application.
    ///
    /// The text to commit could be either just a single character after a key
    /// press or the result of some composing (pre-edit). It could be also an
    /// empty text when some text should be removed (see
    /// delete_surrounding_text) or when the input cursor should be moved (see
    /// cursor_position).
    ///
    /// Any previously set composing text will be removed.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `text`:
    #[inline]
    fn handle_commit_string(
        &mut self,
        slf: &Rc<ZwpInputMethodContextV1>,
        serial: u32,
        text: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_commit_string(
            serial,
            text,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_context_v1.commit_string", &e);
        }
    }

    /// pre-edit string
    ///
    /// Send the pre-edit string text to the application text input.
    ///
    /// The commit text can be used to replace the pre-edit text on reset (for
    /// example on unfocus).
    ///
    /// Previously sent preedit_style and preedit_cursor requests are also
    /// processed by the text_input.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `text`:
    /// - `commit`:
    #[inline]
    fn handle_preedit_string(
        &mut self,
        slf: &Rc<ZwpInputMethodContextV1>,
        serial: u32,
        text: &str,
        commit: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_preedit_string(
            serial,
            text,
            commit,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_context_v1.preedit_string", &e);
        }
    }

    /// pre-edit styling
    ///
    /// Set the styling information on composing text. The style is applied for
    /// length in bytes from index relative to the beginning of
    /// the composing text (as byte offset). Multiple styles can
    /// be applied to a composing text.
    ///
    /// This request should be sent before sending a preedit_string request.
    ///
    /// # Arguments
    ///
    /// - `index`:
    /// - `length`:
    /// - `style`:
    #[inline]
    fn handle_preedit_styling(
        &mut self,
        slf: &Rc<ZwpInputMethodContextV1>,
        index: u32,
        length: u32,
        style: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_preedit_styling(
            index,
            length,
            style,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_context_v1.preedit_styling", &e);
        }
    }

    /// pre-edit cursor
    ///
    /// Set the cursor position inside the composing text (as byte offset)
    /// relative to the start of the composing text.
    ///
    /// When index is negative no cursor should be displayed.
    ///
    /// This request should be sent before sending a preedit_string request.
    ///
    /// # Arguments
    ///
    /// - `index`:
    #[inline]
    fn handle_preedit_cursor(
        &mut self,
        slf: &Rc<ZwpInputMethodContextV1>,
        index: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_preedit_cursor(
            index,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_context_v1.preedit_cursor", &e);
        }
    }

    /// delete text
    ///
    /// Remove the surrounding text.
    ///
    /// This request will be handled on the text_input side directly following
    /// a commit_string request.
    ///
    /// # Arguments
    ///
    /// - `index`:
    /// - `length`:
    #[inline]
    fn handle_delete_surrounding_text(
        &mut self,
        slf: &Rc<ZwpInputMethodContextV1>,
        index: i32,
        length: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_delete_surrounding_text(
            index,
            length,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_context_v1.delete_surrounding_text", &e);
        }
    }

    /// set cursor to a new position
    ///
    /// Set the cursor and anchor to a new position. Index is the new cursor
    /// position in bytes (when >= 0 this is relative to the end of the inserted text,
    /// otherwise it is relative to the beginning of the inserted text). Anchor is
    /// the new anchor position in bytes (when >= 0 this is relative to the end of the
    /// inserted text, otherwise it is relative to the beginning of the inserted
    /// text). When there should be no selected text, anchor should be the same
    /// as index.
    ///
    /// This request will be handled on the text_input side directly following
    /// a commit_string request.
    ///
    /// # Arguments
    ///
    /// - `index`:
    /// - `anchor`:
    #[inline]
    fn handle_cursor_position(
        &mut self,
        slf: &Rc<ZwpInputMethodContextV1>,
        index: i32,
        anchor: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_cursor_position(
            index,
            anchor,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_context_v1.cursor_position", &e);
        }
    }

    /// # Arguments
    ///
    /// - `map`:
    #[inline]
    fn handle_modifiers_map(
        &mut self,
        slf: &Rc<ZwpInputMethodContextV1>,
        map: &[u8],
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_modifiers_map(
            map,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_context_v1.modifiers_map", &e);
        }
    }

    /// keysym
    ///
    /// Notify when a key event was sent. Key events should not be used for
    /// normal text input operations, which should be done with commit_string,
    /// delete_surrounding_text, etc. The key event follows the wl_keyboard key
    /// event convention. Sym is an XKB keysym, state is a wl_keyboard key_state.
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
        slf: &Rc<ZwpInputMethodContextV1>,
        serial: u32,
        time: u32,
        sym: u32,
        state: u32,
        modifiers: u32,
    ) {
        if !slf.core.forward_to_server.get() {
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
            log_forward("zwp_input_method_context_v1.keysym", &e);
        }
    }

    /// grab hardware keyboard
    ///
    /// Allow an input method to receive hardware keyboard input and process
    /// key events to generate text events (with pre-edit) over the wire. This
    /// allows input methods which compose multiple key events for inputting
    /// text like it is done for CJK languages.
    ///
    /// # Arguments
    ///
    /// - `keyboard`:
    #[inline]
    fn handle_grab_keyboard(
        &mut self,
        slf: &Rc<ZwpInputMethodContextV1>,
        keyboard: &Rc<WlKeyboard>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_grab_keyboard(
            keyboard,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_context_v1.grab_keyboard", &e);
        }
    }

    /// forward key event
    ///
    /// Forward a wl_keyboard::key event to the client that was not processed
    /// by the input method itself. Should be used when filtering key events
    /// with grab_keyboard.  The arguments should be the ones from the
    /// wl_keyboard::key event.
    ///
    /// For generating custom key events use the keysym request instead.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial from wl_keyboard::key
    /// - `time`: time from wl_keyboard::key
    /// - `key`: key from wl_keyboard::key
    /// - `state`: state from wl_keyboard::key
    #[inline]
    fn handle_key(
        &mut self,
        slf: &Rc<ZwpInputMethodContextV1>,
        serial: u32,
        time: u32,
        key: u32,
        state: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_key(
            serial,
            time,
            key,
            state,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_context_v1.key", &e);
        }
    }

    /// forward modifiers event
    ///
    /// Forward a wl_keyboard::modifiers event to the client that was not
    /// processed by the input method itself.  Should be used when filtering
    /// key events with grab_keyboard. The arguments should be the ones
    /// from the wl_keyboard::modifiers event.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial from wl_keyboard::modifiers
    /// - `mods_depressed`: mods_depressed from wl_keyboard::modifiers
    /// - `mods_latched`: mods_latched from wl_keyboard::modifiers
    /// - `mods_locked`: mods_locked from wl_keyboard::modifiers
    /// - `group`: group from wl_keyboard::modifiers
    #[inline]
    fn handle_modifiers(
        &mut self,
        slf: &Rc<ZwpInputMethodContextV1>,
        serial: u32,
        mods_depressed: u32,
        mods_latched: u32,
        mods_locked: u32,
        group: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_modifiers(
            serial,
            mods_depressed,
            mods_latched,
            mods_locked,
            group,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_context_v1.modifiers", &e);
        }
    }

    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `language`:
    #[inline]
    fn handle_language(
        &mut self,
        slf: &Rc<ZwpInputMethodContextV1>,
        serial: u32,
        language: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_language(
            serial,
            language,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_context_v1.language", &e);
        }
    }

    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `direction`:
    #[inline]
    fn handle_text_direction(
        &mut self,
        slf: &Rc<ZwpInputMethodContextV1>,
        serial: u32,
        direction: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_text_direction(
            serial,
            direction,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_context_v1.text_direction", &e);
        }
    }

    /// surrounding text event
    ///
    /// The plain surrounding text around the input position. Cursor is the
    /// position in bytes within the surrounding text relative to the beginning
    /// of the text. Anchor is the position in bytes of the selection anchor
    /// within the surrounding text relative to the beginning of the text. If
    /// there is no selected text then anchor is the same as cursor.
    ///
    /// # Arguments
    ///
    /// - `text`:
    /// - `cursor`:
    /// - `anchor`:
    #[inline]
    fn handle_surrounding_text(
        &mut self,
        slf: &Rc<ZwpInputMethodContextV1>,
        text: &str,
        cursor: u32,
        anchor: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_surrounding_text(
            text,
            cursor,
            anchor,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_context_v1.surrounding_text", &e);
        }
    }

    #[inline]
    fn handle_reset(
        &mut self,
        slf: &Rc<ZwpInputMethodContextV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_reset(
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_context_v1.reset", &e);
        }
    }

    /// # Arguments
    ///
    /// - `hint`:
    /// - `purpose`:
    #[inline]
    fn handle_content_type(
        &mut self,
        slf: &Rc<ZwpInputMethodContextV1>,
        hint: u32,
        purpose: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_content_type(
            hint,
            purpose,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_context_v1.content_type", &e);
        }
    }

    /// # Arguments
    ///
    /// - `button`:
    /// - `index`:
    #[inline]
    fn handle_invoke_action(
        &mut self,
        slf: &Rc<ZwpInputMethodContextV1>,
        button: u32,
        index: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_invoke_action(
            button,
            index,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_context_v1.invoke_action", &e);
        }
    }

    /// # Arguments
    ///
    /// - `serial`: serial of text input state
    #[inline]
    fn handle_commit_state(
        &mut self,
        slf: &Rc<ZwpInputMethodContextV1>,
        serial: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_commit_state(
            serial,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_context_v1.commit_state", &e);
        }
    }

    /// # Arguments
    ///
    /// - `language`:
    #[inline]
    fn handle_preferred_language(
        &mut self,
        slf: &Rc<ZwpInputMethodContextV1>,
        language: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_preferred_language(
            language,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_context_v1.preferred_language", &e);
        }
    }
}

impl ObjectPrivate for ZwpInputMethodContextV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpInputMethodContextV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_context_v1#{}.destroy()\n", client_id, id);
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
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_context_v1#{}.commit_string(serial: {}, text: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_commit_string(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_commit_string(&self, arg0, arg1);
                }
            }
            2 => {
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
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: &str, arg2: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_context_v1#{}.preedit_string(serial: {}, text: {:?}, commit: {:?})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_preedit_string(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_preedit_string(&self, arg0, arg1, arg2);
                }
            }
            3 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_context_v1#{}.preedit_styling(index: {}, length: {}, style: {})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_preedit_styling(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_preedit_styling(&self, arg0, arg1, arg2);
                }
            }
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = arg0 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_context_v1#{}.preedit_cursor(index: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_preedit_cursor(&self, arg0);
                } else {
                    DefaultHandler.handle_preedit_cursor(&self, arg0);
                }
            }
            5 => {
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
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_context_v1#{}.delete_surrounding_text(index: {}, length: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_delete_surrounding_text(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_delete_surrounding_text(&self, arg0, arg1);
                }
            }
            6 => {
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
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_context_v1#{}.cursor_position(index: {}, anchor: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_cursor_position(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_cursor_position(&self, arg0, arg1);
                }
            }
            7 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_array(msg, offset, "map")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_context_v1#{}.modifiers_map(map: {})\n", client_id, id, debug_array(arg0));
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_modifiers_map(&self, arg0);
                } else {
                    DefaultHandler.handle_modifiers_map(&self, arg0);
                }
            }
            8 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_context_v1#{}.keysym(serial: {}, time: {}, sym: {}, state: {}, modifiers: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4);
                }
                if let Some(handler) = handler {
                    (**handler).handle_keysym(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultHandler.handle_keysym(&self, arg0, arg1, arg2, arg3, arg4);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_context_v1#{}.grab_keyboard(keyboard: wl_keyboard#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WlKeyboard::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "keyboard", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_grab_keyboard(&self, arg0);
                } else {
                    DefaultHandler.handle_grab_keyboard(&self, arg0);
                }
            }
            10 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 24)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_context_v1#{}.key(serial: {}, time: {}, key: {}, state: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_key(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_key(&self, arg0, arg1, arg2, arg3);
                }
            }
            11 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_context_v1#{}.modifiers(serial: {}, mods_depressed: {}, mods_latched: {}, mods_locked: {}, group: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4);
                }
                if let Some(handler) = handler {
                    (**handler).handle_modifiers(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultHandler.handle_modifiers(&self, arg0, arg1, arg2, arg3, arg4);
                }
            }
            12 => {
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
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_context_v1#{}.language(serial: {}, language: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_language(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_language(&self, arg0, arg1);
                }
            }
            13 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_context_v1#{}.text_direction(serial: {}, direction: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_text_direction(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_text_direction(&self, arg0, arg1);
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
                    fn log(state: &State, id: u32, arg0: &str, arg1: u32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_context_v1#{}.surrounding_text(text: {:?}, cursor: {}, anchor: {})\n", id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_surrounding_text(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_surrounding_text(&self, arg0, arg1, arg2);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_context_v1#{}.reset()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_reset(&self);
                } else {
                    DefaultHandler.handle_reset(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_context_v1#{}.content_type(hint: {}, purpose: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_content_type(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_content_type(&self, arg0, arg1);
                }
            }
            3 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_context_v1#{}.invoke_action(button: {}, index: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_invoke_action(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_invoke_action(&self, arg0, arg1);
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
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_context_v1#{}.commit_state(serial: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_commit_state(&self, arg0);
                } else {
                    DefaultHandler.handle_commit_state(&self, arg0);
                }
            }
            5 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "language")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_context_v1#{}.preferred_language(language: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_preferred_language(&self, arg0);
                } else {
                    DefaultHandler.handle_preferred_language(&self, arg0);
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
            1 => "commit_string",
            2 => "preedit_string",
            3 => "preedit_styling",
            4 => "preedit_cursor",
            5 => "delete_surrounding_text",
            6 => "cursor_position",
            7 => "modifiers_map",
            8 => "keysym",
            9 => "grab_keyboard",
            10 => "key",
            11 => "modifiers",
            12 => "language",
            13 => "text_direction",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "surrounding_text",
            1 => "reset",
            2 => "content_type",
            3 => "invoke_action",
            4 => "commit_state",
            5 => "preferred_language",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwpInputMethodContextV1 {
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

