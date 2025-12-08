//! text input
//!
//! The zwp_text_input_v3 interface represents text input and input methods
//! associated with a seat. It provides enter/leave events to follow the
//! text input focus for a seat.
//!
//! Requests are used to enable/disable the text-input object and set
//! state information like surrounding and selected text or the content type.
//! The information about the entered text is sent to the text-input object
//! via the preedit_string and commit_string events.
//!
//! Text is valid UTF-8 encoded, indices and lengths are in bytes. Indices
//! must not point to middle bytes inside a code point: they must either
//! point to the first byte of a code point or to the end of the buffer.
//! Lengths must be measured between two valid indices.
//!
//! Focus moving throughout surfaces will result in the emission of
//! zwp_text_input_v3.enter and zwp_text_input_v3.leave events. The focused
//! surface must commit zwp_text_input_v3.enable and
//! zwp_text_input_v3.disable requests as the keyboard focus moves across
//! editable and non-editable elements of the UI. Those two requests are not
//! expected to be paired with each other, the compositor must be able to
//! handle consecutive series of the same request.
//!
//! State is sent by the state requests (set_surrounding_text,
//! set_content_type and set_cursor_rectangle) and a commit request. After an
//! enter event or disable request all state information is invalidated and
//! needs to be resent by the client.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_text_input_v3 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpTextInputV3 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpTextInputV3Handler>,
}

struct DefaultHandler;

impl ZwpTextInputV3Handler for DefaultHandler { }

impl ConcreteObject for ZwpTextInputV3 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpTextInputV3;
    const INTERFACE_NAME: &str = "zwp_text_input_v3";
}

impl ZwpTextInputV3 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpTextInputV3Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpTextInputV3Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpTextInputV3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpTextInputV3")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpTextInputV3 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// Destroy the wp_text_input
    ///
    /// Destroy the wp_text_input object. Also disables all surfaces enabled
    /// through this wp_text_input object.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_text_input_v3#{}.destroy()\n", id);
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

    /// Destroy the wp_text_input
    ///
    /// Destroy the wp_text_input object. Also disables all surfaces enabled
    /// through this wp_text_input object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v3.destroy", &e);
        }
    }

    /// Since when the enable message is available.
    pub const MSG__ENABLE__SINCE: u32 = 1;

    /// Request text input to be enabled
    ///
    /// Requests text input on the surface previously obtained from the enter
    /// event.
    ///
    /// This request must be issued every time the focused text input changes
    /// to a new one, including within the current surface. Use
    /// zwp_text_input_v3.disable when there is no longer any input focus on
    /// the current surface.
    ///
    /// Clients must not enable more than one text input on the single seat
    /// and should disable the current text input before enabling the new one.
    /// Requests to enable a text input when another text input is enabled
    /// on the same seat must be ignored by compositor.
    ///
    /// This request resets all state associated with previous enable, disable,
    /// set_surrounding_text, set_text_change_cause, set_content_type, and
    /// set_cursor_rectangle requests, as well as the state associated with
    /// preedit_string, commit_string, and delete_surrounding_text events.
    ///
    /// The set_surrounding_text, set_content_type and set_cursor_rectangle
    /// requests must follow if the text input supports the necessary
    /// functionality.
    ///
    /// State set with this request is double-buffered. It will get applied on
    /// the next zwp_text_input_v3.commit request, and stay valid until the
    /// next committed enable or disable request.
    ///
    /// The changes must be applied by the compositor after issuing a
    /// zwp_text_input_v3.commit request.
    #[inline]
    pub fn try_send_enable(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_text_input_v3#{}.enable()\n", id);
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
        Ok(())
    }

    /// Request text input to be enabled
    ///
    /// Requests text input on the surface previously obtained from the enter
    /// event.
    ///
    /// This request must be issued every time the focused text input changes
    /// to a new one, including within the current surface. Use
    /// zwp_text_input_v3.disable when there is no longer any input focus on
    /// the current surface.
    ///
    /// Clients must not enable more than one text input on the single seat
    /// and should disable the current text input before enabling the new one.
    /// Requests to enable a text input when another text input is enabled
    /// on the same seat must be ignored by compositor.
    ///
    /// This request resets all state associated with previous enable, disable,
    /// set_surrounding_text, set_text_change_cause, set_content_type, and
    /// set_cursor_rectangle requests, as well as the state associated with
    /// preedit_string, commit_string, and delete_surrounding_text events.
    ///
    /// The set_surrounding_text, set_content_type and set_cursor_rectangle
    /// requests must follow if the text input supports the necessary
    /// functionality.
    ///
    /// State set with this request is double-buffered. It will get applied on
    /// the next zwp_text_input_v3.commit request, and stay valid until the
    /// next committed enable or disable request.
    ///
    /// The changes must be applied by the compositor after issuing a
    /// zwp_text_input_v3.commit request.
    #[inline]
    pub fn send_enable(
        &self,
    ) {
        let res = self.try_send_enable(
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v3.enable", &e);
        }
    }

    /// Since when the disable message is available.
    pub const MSG__DISABLE__SINCE: u32 = 1;

    /// Disable text input on a surface
    ///
    /// Explicitly disable text input on the current surface (typically when
    /// there is no focus on any text entry inside the surface).
    ///
    /// State set with this request is double-buffered. It will get applied on
    /// the next zwp_text_input_v3.commit request.
    #[inline]
    pub fn try_send_disable(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_text_input_v3#{}.disable()\n", id);
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

    /// Disable text input on a surface
    ///
    /// Explicitly disable text input on the current surface (typically when
    /// there is no focus on any text entry inside the surface).
    ///
    /// State set with this request is double-buffered. It will get applied on
    /// the next zwp_text_input_v3.commit request.
    #[inline]
    pub fn send_disable(
        &self,
    ) {
        let res = self.try_send_disable(
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v3.disable", &e);
        }
    }

    /// Since when the set_surrounding_text message is available.
    pub const MSG__SET_SURROUNDING_TEXT__SINCE: u32 = 1;

    /// sets the surrounding text
    ///
    /// Sets the surrounding plain text around the input, excluding the preedit
    /// text.
    ///
    /// The client should notify the compositor of any changes in any of the
    /// values carried with this request, including changes caused by handling
    /// incoming text-input events as well as changes caused by other
    /// mechanisms like keyboard typing.
    ///
    /// If the client is unaware of the text around the cursor, it should not
    /// issue this request, to signify lack of support to the compositor.
    ///
    /// Text is UTF-8 encoded, and should include the cursor position, the
    /// complete selection and additional characters before and after them.
    /// There is a maximum length of wayland messages, so text can not be
    /// longer than 4000 bytes.
    ///
    /// Cursor is the byte offset of the cursor within text buffer.
    ///
    /// Anchor is the byte offset of the selection anchor within text buffer.
    /// If there is no selected text, anchor is the same as cursor.
    ///
    /// If any preedit text is present, it is replaced with a cursor for the
    /// purpose of this event.
    ///
    /// Values set with this request are double-buffered. They will get applied
    /// on the next zwp_text_input_v3.commit request, and stay valid until the
    /// next committed enable or disable request.
    ///
    /// The initial state for affected fields is empty, meaning that the text
    /// input does not support sending surrounding text. If the empty values
    /// get applied, subsequent attempts to change them may have no effect.
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
        cursor: i32,
        anchor: i32,
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
            fn log(state: &State, id: u32, arg0: &str, arg1: i32, arg2: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_text_input_v3#{}.set_surrounding_text(text: {:?}, cursor: {}, anchor: {})\n", id, arg0, arg1, arg2);
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
        ]);
        fmt.string(arg0);
        fmt.words([
            arg1 as u32,
            arg2 as u32,
        ]);
        Ok(())
    }

    /// sets the surrounding text
    ///
    /// Sets the surrounding plain text around the input, excluding the preedit
    /// text.
    ///
    /// The client should notify the compositor of any changes in any of the
    /// values carried with this request, including changes caused by handling
    /// incoming text-input events as well as changes caused by other
    /// mechanisms like keyboard typing.
    ///
    /// If the client is unaware of the text around the cursor, it should not
    /// issue this request, to signify lack of support to the compositor.
    ///
    /// Text is UTF-8 encoded, and should include the cursor position, the
    /// complete selection and additional characters before and after them.
    /// There is a maximum length of wayland messages, so text can not be
    /// longer than 4000 bytes.
    ///
    /// Cursor is the byte offset of the cursor within text buffer.
    ///
    /// Anchor is the byte offset of the selection anchor within text buffer.
    /// If there is no selected text, anchor is the same as cursor.
    ///
    /// If any preedit text is present, it is replaced with a cursor for the
    /// purpose of this event.
    ///
    /// Values set with this request are double-buffered. They will get applied
    /// on the next zwp_text_input_v3.commit request, and stay valid until the
    /// next committed enable or disable request.
    ///
    /// The initial state for affected fields is empty, meaning that the text
    /// input does not support sending surrounding text. If the empty values
    /// get applied, subsequent attempts to change them may have no effect.
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
        cursor: i32,
        anchor: i32,
    ) {
        let res = self.try_send_set_surrounding_text(
            text,
            cursor,
            anchor,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v3.set_surrounding_text", &e);
        }
    }

    /// Since when the set_text_change_cause message is available.
    pub const MSG__SET_TEXT_CHANGE_CAUSE__SINCE: u32 = 1;

    /// indicates the cause of surrounding text change
    ///
    /// Tells the compositor why the text surrounding the cursor changed.
    ///
    /// Whenever the client detects an external change in text, cursor, or
    /// anchor posision, it must issue this request to the compositor. This
    /// request is intended to give the input method a chance to update the
    /// preedit text in an appropriate way, e.g. by removing it when the user
    /// starts typing with a keyboard.
    ///
    /// cause describes the source of the change.
    ///
    /// The value set with this request is double-buffered. It must be applied
    /// and reset to initial at the next zwp_text_input_v3.commit request.
    ///
    /// The initial value of cause is input_method.
    ///
    /// # Arguments
    ///
    /// - `cause`:
    #[inline]
    pub fn try_send_set_text_change_cause(
        &self,
        cause: ZwpTextInputV3ChangeCause,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            cause,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: ZwpTextInputV3ChangeCause) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_text_input_v3#{}.set_text_change_cause(cause: {:?})\n", id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// indicates the cause of surrounding text change
    ///
    /// Tells the compositor why the text surrounding the cursor changed.
    ///
    /// Whenever the client detects an external change in text, cursor, or
    /// anchor posision, it must issue this request to the compositor. This
    /// request is intended to give the input method a chance to update the
    /// preedit text in an appropriate way, e.g. by removing it when the user
    /// starts typing with a keyboard.
    ///
    /// cause describes the source of the change.
    ///
    /// The value set with this request is double-buffered. It must be applied
    /// and reset to initial at the next zwp_text_input_v3.commit request.
    ///
    /// The initial value of cause is input_method.
    ///
    /// # Arguments
    ///
    /// - `cause`:
    #[inline]
    pub fn send_set_text_change_cause(
        &self,
        cause: ZwpTextInputV3ChangeCause,
    ) {
        let res = self.try_send_set_text_change_cause(
            cause,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v3.set_text_change_cause", &e);
        }
    }

    /// Since when the set_content_type message is available.
    pub const MSG__SET_CONTENT_TYPE__SINCE: u32 = 1;

    /// set content purpose and hint
    ///
    /// Sets the content purpose and content hint. While the purpose is the
    /// basic purpose of an input field, the hint flags allow to modify some of
    /// the behavior.
    ///
    /// Values set with this request are double-buffered. They will get applied
    /// on the next zwp_text_input_v3.commit request.
    /// Subsequent attempts to update them may have no effect. The values
    /// remain valid until the next committed enable or disable request.
    ///
    /// The initial value for hint is none, and the initial value for purpose
    /// is normal.
    ///
    /// # Arguments
    ///
    /// - `hint`:
    /// - `purpose`:
    #[inline]
    pub fn try_send_set_content_type(
        &self,
        hint: ZwpTextInputV3ContentHint,
        purpose: ZwpTextInputV3ContentPurpose,
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
            fn log(state: &State, id: u32, arg0: ZwpTextInputV3ContentHint, arg1: ZwpTextInputV3ContentPurpose) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_text_input_v3#{}.set_content_type(hint: {:?}, purpose: {:?})\n", id, arg0, arg1);
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
            arg0.0,
            arg1.0,
        ]);
        Ok(())
    }

    /// set content purpose and hint
    ///
    /// Sets the content purpose and content hint. While the purpose is the
    /// basic purpose of an input field, the hint flags allow to modify some of
    /// the behavior.
    ///
    /// Values set with this request are double-buffered. They will get applied
    /// on the next zwp_text_input_v3.commit request.
    /// Subsequent attempts to update them may have no effect. The values
    /// remain valid until the next committed enable or disable request.
    ///
    /// The initial value for hint is none, and the initial value for purpose
    /// is normal.
    ///
    /// # Arguments
    ///
    /// - `hint`:
    /// - `purpose`:
    #[inline]
    pub fn send_set_content_type(
        &self,
        hint: ZwpTextInputV3ContentHint,
        purpose: ZwpTextInputV3ContentPurpose,
    ) {
        let res = self.try_send_set_content_type(
            hint,
            purpose,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v3.set_content_type", &e);
        }
    }

    /// Since when the set_cursor_rectangle message is available.
    pub const MSG__SET_CURSOR_RECTANGLE__SINCE: u32 = 1;

    /// set cursor position
    ///
    /// Marks an area around the cursor as a x, y, width, height rectangle in
    /// surface local coordinates.
    ///
    /// Allows the compositor to put a window with word suggestions near the
    /// cursor, without obstructing the text being input.
    ///
    /// If the client is unaware of the position of edited text, it should not
    /// issue this request, to signify lack of support to the compositor.
    ///
    /// Values set with this request are double-buffered. They will get applied
    /// on the next zwp_text_input_v3.commit request, and stay valid until the
    /// next committed enable or disable request.
    ///
    /// The initial values describing a cursor rectangle are empty. That means
    /// the text input does not support describing the cursor area. If the
    /// empty values get applied, subsequent attempts to change them may have
    /// no effect.
    ///
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_text_input_v3#{}.set_cursor_rectangle(x: {}, y: {}, width: {}, height: {})\n", id, arg0, arg1, arg2, arg3);
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
            6,
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// set cursor position
    ///
    /// Marks an area around the cursor as a x, y, width, height rectangle in
    /// surface local coordinates.
    ///
    /// Allows the compositor to put a window with word suggestions near the
    /// cursor, without obstructing the text being input.
    ///
    /// If the client is unaware of the position of edited text, it should not
    /// issue this request, to signify lack of support to the compositor.
    ///
    /// Values set with this request are double-buffered. They will get applied
    /// on the next zwp_text_input_v3.commit request, and stay valid until the
    /// next committed enable or disable request.
    ///
    /// The initial values describing a cursor rectangle are empty. That means
    /// the text input does not support describing the cursor area. If the
    /// empty values get applied, subsequent attempts to change them may have
    /// no effect.
    ///
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
            log_send("zwp_text_input_v3.set_cursor_rectangle", &e);
        }
    }

    /// Since when the commit message is available.
    pub const MSG__COMMIT__SINCE: u32 = 1;

    /// commit state
    ///
    /// Atomically applies state changes recently sent to the compositor.
    ///
    /// The commit request establishes and updates the state of the client, and
    /// must be issued after any changes to apply them.
    ///
    /// Text input state (enabled status, content purpose, content hint,
    /// surrounding text and change cause, cursor rectangle) is conceptually
    /// double-buffered within the context of a text input, i.e. between a
    /// committed enable request and the following committed enable or disable
    /// request.
    ///
    /// Protocol requests modify the pending state, as opposed to the current
    /// state in use by the input method. A commit request atomically applies
    /// all pending state, replacing the current state. After commit, the new
    /// pending state is as documented for each related request.
    ///
    /// Requests are applied in the order of arrival.
    ///
    /// Neither current nor pending state are modified unless noted otherwise.
    ///
    /// The compositor must count the number of commit requests coming from
    /// each zwp_text_input_v3 object and use the count as the serial in done
    /// events.
    #[inline]
    pub fn try_send_commit(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_text_input_v3#{}.commit()\n", id);
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
            7,
        ]);
        Ok(())
    }

    /// commit state
    ///
    /// Atomically applies state changes recently sent to the compositor.
    ///
    /// The commit request establishes and updates the state of the client, and
    /// must be issued after any changes to apply them.
    ///
    /// Text input state (enabled status, content purpose, content hint,
    /// surrounding text and change cause, cursor rectangle) is conceptually
    /// double-buffered within the context of a text input, i.e. between a
    /// committed enable request and the following committed enable or disable
    /// request.
    ///
    /// Protocol requests modify the pending state, as opposed to the current
    /// state in use by the input method. A commit request atomically applies
    /// all pending state, replacing the current state. After commit, the new
    /// pending state is as documented for each related request.
    ///
    /// Requests are applied in the order of arrival.
    ///
    /// Neither current nor pending state are modified unless noted otherwise.
    ///
    /// The compositor must count the number of commit requests coming from
    /// each zwp_text_input_v3 object and use the count as the serial in done
    /// events.
    #[inline]
    pub fn send_commit(
        &self,
    ) {
        let res = self.try_send_commit(
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v3.commit", &e);
        }
    }

    /// Since when the enter message is available.
    pub const MSG__ENTER__SINCE: u32 = 1;

    /// enter event
    ///
    /// Notification that this seat's text-input focus is on a certain surface.
    ///
    /// If client has created multiple text input objects, compositor must send
    /// this event to all of them.
    ///
    /// When the seat has the keyboard capability the text-input focus follows
    /// the keyboard focus. This event sets the current surface for the
    /// text-input object.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_text_input_v3#{}.enter(surface: wl_surface#{})\n", client_id, id, arg0);
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
    /// Notification that this seat's text-input focus is on a certain surface.
    ///
    /// If client has created multiple text input objects, compositor must send
    /// this event to all of them.
    ///
    /// When the seat has the keyboard capability the text-input focus follows
    /// the keyboard focus. This event sets the current surface for the
    /// text-input object.
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
            log_send("zwp_text_input_v3.enter", &e);
        }
    }

    /// Since when the leave message is available.
    pub const MSG__LEAVE__SINCE: u32 = 1;

    /// leave event
    ///
    /// Notification that this seat's text-input focus is no longer on a
    /// certain surface. The client should reset any preedit string previously
    /// set.
    ///
    /// The leave notification clears the current surface. It is sent before
    /// the enter notification for the new focus. After leave event, compositor
    /// must ignore requests from any text input instances until next enter
    /// event.
    ///
    /// When the seat has the keyboard capability the text-input focus follows
    /// the keyboard focus.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn try_send_leave(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_text_input_v3#{}.leave(surface: wl_surface#{})\n", client_id, id, arg0);
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

    /// leave event
    ///
    /// Notification that this seat's text-input focus is no longer on a
    /// certain surface. The client should reset any preedit string previously
    /// set.
    ///
    /// The leave notification clears the current surface. It is sent before
    /// the enter notification for the new focus. After leave event, compositor
    /// must ignore requests from any text input instances until next enter
    /// event.
    ///
    /// When the seat has the keyboard capability the text-input focus follows
    /// the keyboard focus.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn send_leave(
        &self,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_leave(
            surface,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v3.leave", &e);
        }
    }

    /// Since when the preedit_string message is available.
    pub const MSG__PREEDIT_STRING__SINCE: u32 = 1;

    /// pre-edit
    ///
    /// Notify when a new composing text (pre-edit) should be set at the
    /// current cursor position. Any previously set composing text must be
    /// removed. Any previously existing selected text must be removed.
    ///
    /// The argument text contains the pre-edit string buffer.
    ///
    /// The parameters cursor_begin and cursor_end are counted in bytes
    /// relative to the beginning of the submitted text buffer. Cursor should
    /// be hidden when both are equal to -1.
    ///
    /// They could be represented by the client as a line if both values are
    /// the same, or as a text highlight otherwise.
    ///
    /// Values set with this event are double-buffered. They must be applied
    /// and reset to initial on the next zwp_text_input_v3.done event.
    ///
    /// The initial value of text is an empty string, and cursor_begin,
    /// cursor_end and cursor_hidden are all 0.
    ///
    /// # Arguments
    ///
    /// - `text`:
    /// - `cursor_begin`:
    /// - `cursor_end`:
    #[inline]
    pub fn try_send_preedit_string(
        &self,
        text: Option<&str>,
        cursor_begin: i32,
        cursor_end: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            text,
            cursor_begin,
            cursor_end,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: Option<&str>, arg1: i32, arg2: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_text_input_v3#{}.preedit_string(text: {:?}, cursor_begin: {}, cursor_end: {})\n", client_id, id, arg0, arg1, arg2);
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
            2,
        ]);
        if let Some(arg0) = arg0 {
            fmt.string(arg0);
        } else {
            fmt.words([0]);
        }
        fmt.words([
            arg1 as u32,
            arg2 as u32,
        ]);
        Ok(())
    }

    /// pre-edit
    ///
    /// Notify when a new composing text (pre-edit) should be set at the
    /// current cursor position. Any previously set composing text must be
    /// removed. Any previously existing selected text must be removed.
    ///
    /// The argument text contains the pre-edit string buffer.
    ///
    /// The parameters cursor_begin and cursor_end are counted in bytes
    /// relative to the beginning of the submitted text buffer. Cursor should
    /// be hidden when both are equal to -1.
    ///
    /// They could be represented by the client as a line if both values are
    /// the same, or as a text highlight otherwise.
    ///
    /// Values set with this event are double-buffered. They must be applied
    /// and reset to initial on the next zwp_text_input_v3.done event.
    ///
    /// The initial value of text is an empty string, and cursor_begin,
    /// cursor_end and cursor_hidden are all 0.
    ///
    /// # Arguments
    ///
    /// - `text`:
    /// - `cursor_begin`:
    /// - `cursor_end`:
    #[inline]
    pub fn send_preedit_string(
        &self,
        text: Option<&str>,
        cursor_begin: i32,
        cursor_end: i32,
    ) {
        let res = self.try_send_preedit_string(
            text,
            cursor_begin,
            cursor_end,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v3.preedit_string", &e);
        }
    }

    /// Since when the commit_string message is available.
    pub const MSG__COMMIT_STRING__SINCE: u32 = 1;

    /// text commit
    ///
    /// Notify when text should be inserted into the editor widget. The text to
    /// commit could be either just a single character after a key press or the
    /// result of some composing (pre-edit).
    ///
    /// Values set with this event are double-buffered. They must be applied
    /// and reset to initial on the next zwp_text_input_v3.done event.
    ///
    /// The initial value of text is an empty string.
    ///
    /// # Arguments
    ///
    /// - `text`:
    #[inline]
    pub fn try_send_commit_string(
        &self,
        text: Option<&str>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
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
            fn log(state: &State, client_id: u64, id: u32, arg0: Option<&str>) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_text_input_v3#{}.commit_string(text: {:?})\n", client_id, id, arg0);
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
        if let Some(arg0) = arg0 {
            fmt.string(arg0);
        } else {
            fmt.words([0]);
        }
        Ok(())
    }

    /// text commit
    ///
    /// Notify when text should be inserted into the editor widget. The text to
    /// commit could be either just a single character after a key press or the
    /// result of some composing (pre-edit).
    ///
    /// Values set with this event are double-buffered. They must be applied
    /// and reset to initial on the next zwp_text_input_v3.done event.
    ///
    /// The initial value of text is an empty string.
    ///
    /// # Arguments
    ///
    /// - `text`:
    #[inline]
    pub fn send_commit_string(
        &self,
        text: Option<&str>,
    ) {
        let res = self.try_send_commit_string(
            text,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v3.commit_string", &e);
        }
    }

    /// Since when the delete_surrounding_text message is available.
    pub const MSG__DELETE_SURROUNDING_TEXT__SINCE: u32 = 1;

    /// delete surrounding text
    ///
    /// Notify when the text around the current cursor position should be
    /// deleted.
    ///
    /// Before_length and after_length are the number of bytes before and after
    /// the current cursor index (excluding the selection) to delete.
    ///
    /// If a preedit text is present, in effect before_length is counted from
    /// the beginning of it, and after_length from its end (see done event
    /// sequence).
    ///
    /// Values set with this event are double-buffered. They must be applied
    /// and reset to initial on the next zwp_text_input_v3.done event.
    ///
    /// The initial values of both before_length and after_length are 0.
    ///
    /// # Arguments
    ///
    /// - `before_length`: length of text before current cursor position
    /// - `after_length`: length of text after current cursor position
    #[inline]
    pub fn try_send_delete_surrounding_text(
        &self,
        before_length: u32,
        after_length: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            before_length,
            after_length,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_text_input_v3#{}.delete_surrounding_text(before_length: {}, after_length: {})\n", client_id, id, arg0, arg1);
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
            4,
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// delete surrounding text
    ///
    /// Notify when the text around the current cursor position should be
    /// deleted.
    ///
    /// Before_length and after_length are the number of bytes before and after
    /// the current cursor index (excluding the selection) to delete.
    ///
    /// If a preedit text is present, in effect before_length is counted from
    /// the beginning of it, and after_length from its end (see done event
    /// sequence).
    ///
    /// Values set with this event are double-buffered. They must be applied
    /// and reset to initial on the next zwp_text_input_v3.done event.
    ///
    /// The initial values of both before_length and after_length are 0.
    ///
    /// # Arguments
    ///
    /// - `before_length`: length of text before current cursor position
    /// - `after_length`: length of text after current cursor position
    #[inline]
    pub fn send_delete_surrounding_text(
        &self,
        before_length: u32,
        after_length: u32,
    ) {
        let res = self.try_send_delete_surrounding_text(
            before_length,
            after_length,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v3.delete_surrounding_text", &e);
        }
    }

    /// Since when the done message is available.
    pub const MSG__DONE__SINCE: u32 = 1;

    /// apply changes
    ///
    /// Instruct the application to apply changes to state requested by the
    /// preedit_string, commit_string and delete_surrounding_text events. The
    /// state relating to these events is double-buffered, and each one
    /// modifies the pending state. This event replaces the current state with
    /// the pending state.
    ///
    /// The application must proceed by evaluating the changes in the following
    /// order:
    ///
    /// 1. Replace existing preedit string with the cursor.
    /// 2. Delete requested surrounding text.
    /// 3. Insert commit string with the cursor at its end.
    /// 4. Calculate surrounding text to send.
    /// 5. Insert new preedit text in cursor position.
    /// 6. Place cursor inside preedit text.
    ///
    /// The serial number reflects the last state of the zwp_text_input_v3
    /// object known to the compositor. The value of the serial argument must
    /// be equal to the number of commit requests already issued on that object.
    ///
    /// When the client receives a done event with a serial different than the
    /// number of past commit requests, it must proceed with evaluating and
    /// applying the changes as normal, except it should not change the current
    /// state of the zwp_text_input_v3 object. All pending state requests
    /// (set_surrounding_text, set_content_type and set_cursor_rectangle) on
    /// the zwp_text_input_v3 object should be sent and committed after
    /// receiving a zwp_text_input_v3.done event with a matching serial.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    #[inline]
    pub fn try_send_done(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_text_input_v3#{}.done(serial: {})\n", client_id, id, arg0);
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
            arg0,
        ]);
        Ok(())
    }

    /// apply changes
    ///
    /// Instruct the application to apply changes to state requested by the
    /// preedit_string, commit_string and delete_surrounding_text events. The
    /// state relating to these events is double-buffered, and each one
    /// modifies the pending state. This event replaces the current state with
    /// the pending state.
    ///
    /// The application must proceed by evaluating the changes in the following
    /// order:
    ///
    /// 1. Replace existing preedit string with the cursor.
    /// 2. Delete requested surrounding text.
    /// 3. Insert commit string with the cursor at its end.
    /// 4. Calculate surrounding text to send.
    /// 5. Insert new preedit text in cursor position.
    /// 6. Place cursor inside preedit text.
    ///
    /// The serial number reflects the last state of the zwp_text_input_v3
    /// object known to the compositor. The value of the serial argument must
    /// be equal to the number of commit requests already issued on that object.
    ///
    /// When the client receives a done event with a serial different than the
    /// number of past commit requests, it must proceed with evaluating and
    /// applying the changes as normal, except it should not change the current
    /// state of the zwp_text_input_v3 object. All pending state requests
    /// (set_surrounding_text, set_content_type and set_cursor_rectangle) on
    /// the zwp_text_input_v3 object should be sent and committed after
    /// receiving a zwp_text_input_v3.done event with a matching serial.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    #[inline]
    pub fn send_done(
        &self,
        serial: u32,
    ) {
        let res = self.try_send_done(
            serial,
        );
        if let Err(e) = res {
            log_send("zwp_text_input_v3.done", &e);
        }
    }
}

/// A message handler for [`ZwpTextInputV3`] proxies.
pub trait ZwpTextInputV3Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpTextInputV3>) {
        slf.core.delete_id();
    }

    /// Destroy the wp_text_input
    ///
    /// Destroy the wp_text_input object. Also disables all surfaces enabled
    /// through this wp_text_input object.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwpTextInputV3>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v3.destroy", &e);
        }
    }

    /// Request text input to be enabled
    ///
    /// Requests text input on the surface previously obtained from the enter
    /// event.
    ///
    /// This request must be issued every time the focused text input changes
    /// to a new one, including within the current surface. Use
    /// zwp_text_input_v3.disable when there is no longer any input focus on
    /// the current surface.
    ///
    /// Clients must not enable more than one text input on the single seat
    /// and should disable the current text input before enabling the new one.
    /// Requests to enable a text input when another text input is enabled
    /// on the same seat must be ignored by compositor.
    ///
    /// This request resets all state associated with previous enable, disable,
    /// set_surrounding_text, set_text_change_cause, set_content_type, and
    /// set_cursor_rectangle requests, as well as the state associated with
    /// preedit_string, commit_string, and delete_surrounding_text events.
    ///
    /// The set_surrounding_text, set_content_type and set_cursor_rectangle
    /// requests must follow if the text input supports the necessary
    /// functionality.
    ///
    /// State set with this request is double-buffered. It will get applied on
    /// the next zwp_text_input_v3.commit request, and stay valid until the
    /// next committed enable or disable request.
    ///
    /// The changes must be applied by the compositor after issuing a
    /// zwp_text_input_v3.commit request.
    #[inline]
    fn handle_enable(
        &mut self,
        slf: &Rc<ZwpTextInputV3>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_enable(
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v3.enable", &e);
        }
    }

    /// Disable text input on a surface
    ///
    /// Explicitly disable text input on the current surface (typically when
    /// there is no focus on any text entry inside the surface).
    ///
    /// State set with this request is double-buffered. It will get applied on
    /// the next zwp_text_input_v3.commit request.
    #[inline]
    fn handle_disable(
        &mut self,
        slf: &Rc<ZwpTextInputV3>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_disable(
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v3.disable", &e);
        }
    }

    /// sets the surrounding text
    ///
    /// Sets the surrounding plain text around the input, excluding the preedit
    /// text.
    ///
    /// The client should notify the compositor of any changes in any of the
    /// values carried with this request, including changes caused by handling
    /// incoming text-input events as well as changes caused by other
    /// mechanisms like keyboard typing.
    ///
    /// If the client is unaware of the text around the cursor, it should not
    /// issue this request, to signify lack of support to the compositor.
    ///
    /// Text is UTF-8 encoded, and should include the cursor position, the
    /// complete selection and additional characters before and after them.
    /// There is a maximum length of wayland messages, so text can not be
    /// longer than 4000 bytes.
    ///
    /// Cursor is the byte offset of the cursor within text buffer.
    ///
    /// Anchor is the byte offset of the selection anchor within text buffer.
    /// If there is no selected text, anchor is the same as cursor.
    ///
    /// If any preedit text is present, it is replaced with a cursor for the
    /// purpose of this event.
    ///
    /// Values set with this request are double-buffered. They will get applied
    /// on the next zwp_text_input_v3.commit request, and stay valid until the
    /// next committed enable or disable request.
    ///
    /// The initial state for affected fields is empty, meaning that the text
    /// input does not support sending surrounding text. If the empty values
    /// get applied, subsequent attempts to change them may have no effect.
    ///
    /// # Arguments
    ///
    /// - `text`:
    /// - `cursor`:
    /// - `anchor`:
    #[inline]
    fn handle_set_surrounding_text(
        &mut self,
        slf: &Rc<ZwpTextInputV3>,
        text: &str,
        cursor: i32,
        anchor: i32,
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
            log_forward("zwp_text_input_v3.set_surrounding_text", &e);
        }
    }

    /// indicates the cause of surrounding text change
    ///
    /// Tells the compositor why the text surrounding the cursor changed.
    ///
    /// Whenever the client detects an external change in text, cursor, or
    /// anchor posision, it must issue this request to the compositor. This
    /// request is intended to give the input method a chance to update the
    /// preedit text in an appropriate way, e.g. by removing it when the user
    /// starts typing with a keyboard.
    ///
    /// cause describes the source of the change.
    ///
    /// The value set with this request is double-buffered. It must be applied
    /// and reset to initial at the next zwp_text_input_v3.commit request.
    ///
    /// The initial value of cause is input_method.
    ///
    /// # Arguments
    ///
    /// - `cause`:
    #[inline]
    fn handle_set_text_change_cause(
        &mut self,
        slf: &Rc<ZwpTextInputV3>,
        cause: ZwpTextInputV3ChangeCause,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_text_change_cause(
            cause,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v3.set_text_change_cause", &e);
        }
    }

    /// set content purpose and hint
    ///
    /// Sets the content purpose and content hint. While the purpose is the
    /// basic purpose of an input field, the hint flags allow to modify some of
    /// the behavior.
    ///
    /// Values set with this request are double-buffered. They will get applied
    /// on the next zwp_text_input_v3.commit request.
    /// Subsequent attempts to update them may have no effect. The values
    /// remain valid until the next committed enable or disable request.
    ///
    /// The initial value for hint is none, and the initial value for purpose
    /// is normal.
    ///
    /// # Arguments
    ///
    /// - `hint`:
    /// - `purpose`:
    #[inline]
    fn handle_set_content_type(
        &mut self,
        slf: &Rc<ZwpTextInputV3>,
        hint: ZwpTextInputV3ContentHint,
        purpose: ZwpTextInputV3ContentPurpose,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_content_type(
            hint,
            purpose,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v3.set_content_type", &e);
        }
    }

    /// set cursor position
    ///
    /// Marks an area around the cursor as a x, y, width, height rectangle in
    /// surface local coordinates.
    ///
    /// Allows the compositor to put a window with word suggestions near the
    /// cursor, without obstructing the text being input.
    ///
    /// If the client is unaware of the position of edited text, it should not
    /// issue this request, to signify lack of support to the compositor.
    ///
    /// Values set with this request are double-buffered. They will get applied
    /// on the next zwp_text_input_v3.commit request, and stay valid until the
    /// next committed enable or disable request.
    ///
    /// The initial values describing a cursor rectangle are empty. That means
    /// the text input does not support describing the cursor area. If the
    /// empty values get applied, subsequent attempts to change them may have
    /// no effect.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    fn handle_set_cursor_rectangle(
        &mut self,
        slf: &Rc<ZwpTextInputV3>,
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
            log_forward("zwp_text_input_v3.set_cursor_rectangle", &e);
        }
    }

    /// commit state
    ///
    /// Atomically applies state changes recently sent to the compositor.
    ///
    /// The commit request establishes and updates the state of the client, and
    /// must be issued after any changes to apply them.
    ///
    /// Text input state (enabled status, content purpose, content hint,
    /// surrounding text and change cause, cursor rectangle) is conceptually
    /// double-buffered within the context of a text input, i.e. between a
    /// committed enable request and the following committed enable or disable
    /// request.
    ///
    /// Protocol requests modify the pending state, as opposed to the current
    /// state in use by the input method. A commit request atomically applies
    /// all pending state, replacing the current state. After commit, the new
    /// pending state is as documented for each related request.
    ///
    /// Requests are applied in the order of arrival.
    ///
    /// Neither current nor pending state are modified unless noted otherwise.
    ///
    /// The compositor must count the number of commit requests coming from
    /// each zwp_text_input_v3 object and use the count as the serial in done
    /// events.
    #[inline]
    fn handle_commit(
        &mut self,
        slf: &Rc<ZwpTextInputV3>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_commit(
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v3.commit", &e);
        }
    }

    /// enter event
    ///
    /// Notification that this seat's text-input focus is on a certain surface.
    ///
    /// If client has created multiple text input objects, compositor must send
    /// this event to all of them.
    ///
    /// When the seat has the keyboard capability the text-input focus follows
    /// the keyboard focus. This event sets the current surface for the
    /// text-input object.
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
        slf: &Rc<ZwpTextInputV3>,
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
            log_forward("zwp_text_input_v3.enter", &e);
        }
    }

    /// leave event
    ///
    /// Notification that this seat's text-input focus is no longer on a
    /// certain surface. The client should reset any preedit string previously
    /// set.
    ///
    /// The leave notification clears the current surface. It is sent before
    /// the enter notification for the new focus. After leave event, compositor
    /// must ignore requests from any text input instances until next enter
    /// event.
    ///
    /// When the seat has the keyboard capability the text-input focus follows
    /// the keyboard focus.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_leave(
        &mut self,
        slf: &Rc<ZwpTextInputV3>,
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
        let res = slf.try_send_leave(
            surface,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v3.leave", &e);
        }
    }

    /// pre-edit
    ///
    /// Notify when a new composing text (pre-edit) should be set at the
    /// current cursor position. Any previously set composing text must be
    /// removed. Any previously existing selected text must be removed.
    ///
    /// The argument text contains the pre-edit string buffer.
    ///
    /// The parameters cursor_begin and cursor_end are counted in bytes
    /// relative to the beginning of the submitted text buffer. Cursor should
    /// be hidden when both are equal to -1.
    ///
    /// They could be represented by the client as a line if both values are
    /// the same, or as a text highlight otherwise.
    ///
    /// Values set with this event are double-buffered. They must be applied
    /// and reset to initial on the next zwp_text_input_v3.done event.
    ///
    /// The initial value of text is an empty string, and cursor_begin,
    /// cursor_end and cursor_hidden are all 0.
    ///
    /// # Arguments
    ///
    /// - `text`:
    /// - `cursor_begin`:
    /// - `cursor_end`:
    #[inline]
    fn handle_preedit_string(
        &mut self,
        slf: &Rc<ZwpTextInputV3>,
        text: Option<&str>,
        cursor_begin: i32,
        cursor_end: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_preedit_string(
            text,
            cursor_begin,
            cursor_end,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v3.preedit_string", &e);
        }
    }

    /// text commit
    ///
    /// Notify when text should be inserted into the editor widget. The text to
    /// commit could be either just a single character after a key press or the
    /// result of some composing (pre-edit).
    ///
    /// Values set with this event are double-buffered. They must be applied
    /// and reset to initial on the next zwp_text_input_v3.done event.
    ///
    /// The initial value of text is an empty string.
    ///
    /// # Arguments
    ///
    /// - `text`:
    #[inline]
    fn handle_commit_string(
        &mut self,
        slf: &Rc<ZwpTextInputV3>,
        text: Option<&str>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_commit_string(
            text,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v3.commit_string", &e);
        }
    }

    /// delete surrounding text
    ///
    /// Notify when the text around the current cursor position should be
    /// deleted.
    ///
    /// Before_length and after_length are the number of bytes before and after
    /// the current cursor index (excluding the selection) to delete.
    ///
    /// If a preedit text is present, in effect before_length is counted from
    /// the beginning of it, and after_length from its end (see done event
    /// sequence).
    ///
    /// Values set with this event are double-buffered. They must be applied
    /// and reset to initial on the next zwp_text_input_v3.done event.
    ///
    /// The initial values of both before_length and after_length are 0.
    ///
    /// # Arguments
    ///
    /// - `before_length`: length of text before current cursor position
    /// - `after_length`: length of text after current cursor position
    #[inline]
    fn handle_delete_surrounding_text(
        &mut self,
        slf: &Rc<ZwpTextInputV3>,
        before_length: u32,
        after_length: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_delete_surrounding_text(
            before_length,
            after_length,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v3.delete_surrounding_text", &e);
        }
    }

    /// apply changes
    ///
    /// Instruct the application to apply changes to state requested by the
    /// preedit_string, commit_string and delete_surrounding_text events. The
    /// state relating to these events is double-buffered, and each one
    /// modifies the pending state. This event replaces the current state with
    /// the pending state.
    ///
    /// The application must proceed by evaluating the changes in the following
    /// order:
    ///
    /// 1. Replace existing preedit string with the cursor.
    /// 2. Delete requested surrounding text.
    /// 3. Insert commit string with the cursor at its end.
    /// 4. Calculate surrounding text to send.
    /// 5. Insert new preedit text in cursor position.
    /// 6. Place cursor inside preedit text.
    ///
    /// The serial number reflects the last state of the zwp_text_input_v3
    /// object known to the compositor. The value of the serial argument must
    /// be equal to the number of commit requests already issued on that object.
    ///
    /// When the client receives a done event with a serial different than the
    /// number of past commit requests, it must proceed with evaluating and
    /// applying the changes as normal, except it should not change the current
    /// state of the zwp_text_input_v3 object. All pending state requests
    /// (set_surrounding_text, set_content_type and set_cursor_rectangle) on
    /// the zwp_text_input_v3 object should be sent and committed after
    /// receiving a zwp_text_input_v3.done event with a matching serial.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    #[inline]
    fn handle_done(
        &mut self,
        slf: &Rc<ZwpTextInputV3>,
        serial: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_done(
            serial,
        );
        if let Err(e) = res {
            log_forward("zwp_text_input_v3.done", &e);
        }
    }
}

impl ObjectPrivate for ZwpTextInputV3 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpTextInputV3, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_text_input_v3#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_text_input_v3#{}.enable()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_enable(&self);
                } else {
                    DefaultHandler.handle_enable(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_text_input_v3#{}.disable()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_disable(&self);
                } else {
                    DefaultHandler.handle_disable(&self);
                }
            }
            3 => {
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
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str, arg1: i32, arg2: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_text_input_v3#{}.set_surrounding_text(text: {:?}, cursor: {}, anchor: {})\n", client_id, id, arg0, arg1, arg2);
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
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = ZwpTextInputV3ChangeCause(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: ZwpTextInputV3ChangeCause) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_text_input_v3#{}.set_text_change_cause(cause: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_text_change_cause(&self, arg0);
                } else {
                    DefaultHandler.handle_set_text_change_cause(&self, arg0);
                }
            }
            5 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = ZwpTextInputV3ContentHint(arg0);
                let arg1 = ZwpTextInputV3ContentPurpose(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: ZwpTextInputV3ContentHint, arg1: ZwpTextInputV3ContentPurpose) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_text_input_v3#{}.set_content_type(hint: {:?}, purpose: {:?})\n", client_id, id, arg0, arg1);
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
            6 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_text_input_v3#{}.set_cursor_rectangle(x: {}, y: {}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2, arg3);
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
            7 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_text_input_v3#{}.commit()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_commit(&self);
                } else {
                    DefaultHandler.handle_commit(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_text_input_v3#{}.enter(surface: wl_surface#{})\n", id, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_text_input_v3#{}.leave(surface: wl_surface#{})\n", id, arg0);
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
                    (**handler).handle_leave(&self, arg0);
                } else {
                    DefaultHandler.handle_leave(&self, arg0);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NullableString>(msg, offset, "text")?;
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("cursor_begin")));
                };
                offset += 1;
                let Some(&arg2) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("cursor_end")));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: Option<&str>, arg1: i32, arg2: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_text_input_v3#{}.preedit_string(text: {:?}, cursor_begin: {}, cursor_end: {})\n", id, arg0, arg1, arg2);
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
            3 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NullableString>(msg, offset, "text")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: Option<&str>) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_text_input_v3#{}.commit_string(text: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_commit_string(&self, arg0);
                } else {
                    DefaultHandler.handle_commit_string(&self, arg0);
                }
            }
            4 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_text_input_v3#{}.delete_surrounding_text(before_length: {}, after_length: {})\n", id, arg0, arg1);
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
            5 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_text_input_v3#{}.done(serial: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_done(&self, arg0);
                } else {
                    DefaultHandler.handle_done(&self, arg0);
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
            1 => "enable",
            2 => "disable",
            3 => "set_surrounding_text",
            4 => "set_text_change_cause",
            5 => "set_content_type",
            6 => "set_cursor_rectangle",
            7 => "commit",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "enter",
            1 => "leave",
            2 => "preedit_string",
            3 => "commit_string",
            4 => "delete_surrounding_text",
            5 => "done",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwpTextInputV3 {
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

impl ZwpTextInputV3 {
    /// Since when the change_cause.input_method enum variant is available.
    pub const ENM__CHANGE_CAUSE_INPUT_METHOD__SINCE: u32 = 1;
    /// Since when the change_cause.other enum variant is available.
    pub const ENM__CHANGE_CAUSE_OTHER__SINCE: u32 = 1;

    /// Since when the content_hint.none enum variant is available.
    pub const ENM__CONTENT_HINT_NONE__SINCE: u32 = 1;
    /// Since when the content_hint.completion enum variant is available.
    pub const ENM__CONTENT_HINT_COMPLETION__SINCE: u32 = 1;
    /// Since when the content_hint.spellcheck enum variant is available.
    pub const ENM__CONTENT_HINT_SPELLCHECK__SINCE: u32 = 1;
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
    /// Since when the content_purpose.pin enum variant is available.
    pub const ENM__CONTENT_PURPOSE_PIN__SINCE: u32 = 1;
    /// Since when the content_purpose.date enum variant is available.
    pub const ENM__CONTENT_PURPOSE_DATE__SINCE: u32 = 1;
    /// Since when the content_purpose.time enum variant is available.
    pub const ENM__CONTENT_PURPOSE_TIME__SINCE: u32 = 1;
    /// Since when the content_purpose.datetime enum variant is available.
    pub const ENM__CONTENT_PURPOSE_DATETIME__SINCE: u32 = 1;
    /// Since when the content_purpose.terminal enum variant is available.
    pub const ENM__CONTENT_PURPOSE_TERMINAL__SINCE: u32 = 1;
}

/// text change reason
///
/// Reason for the change of surrounding text or cursor posision.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwpTextInputV3ChangeCause(pub u32);

impl ZwpTextInputV3ChangeCause {
    /// input method caused the change
    pub const INPUT_METHOD: Self = Self(0);

    /// something else than the input method caused the change
    pub const OTHER: Self = Self(1);
}

impl Debug for ZwpTextInputV3ChangeCause {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INPUT_METHOD => "INPUT_METHOD",
            Self::OTHER => "OTHER",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// content hint
///
/// Content hint is a bitmask to allow to modify the behavior of the text
/// input.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct ZwpTextInputV3ContentHint(pub u32);

/// An iterator over the set bits in a [`ZwpTextInputV3ContentHint`].
///
/// You can construct this with the `IntoIterator` implementation of `ZwpTextInputV3ContentHint`.
#[derive(Clone, Debug)]
pub struct ZwpTextInputV3ContentHintIter(pub u32);

impl ZwpTextInputV3ContentHint {
    /// no special behavior
    pub const NONE: Self = Self(0x0);

    /// suggest word completions
    pub const COMPLETION: Self = Self(0x1);

    /// suggest word corrections
    pub const SPELLCHECK: Self = Self(0x2);

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

    /// just Latin characters should be entered
    pub const LATIN: Self = Self(0x100);

    /// the text input is multiline
    pub const MULTILINE: Self = Self(0x200);
}

impl ZwpTextInputV3ContentHint {
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
        Self(0 | 0x0 | 0x1 | 0x2 | 0x4 | 0x8 | 0x10 | 0x20 | 0x40 | 0x80 | 0x100 | 0x200)
    }
}

impl Iterator for ZwpTextInputV3ContentHintIter {
    type Item = ZwpTextInputV3ContentHint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(ZwpTextInputV3ContentHint(bit))
    }
}

impl IntoIterator for ZwpTextInputV3ContentHint {
    type Item = ZwpTextInputV3ContentHint;
    type IntoIter = ZwpTextInputV3ContentHintIter;

    fn into_iter(self) -> Self::IntoIter {
        ZwpTextInputV3ContentHintIter(self.0)
    }
}

impl BitAnd for ZwpTextInputV3ContentHint {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for ZwpTextInputV3ContentHint {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for ZwpTextInputV3ContentHint {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for ZwpTextInputV3ContentHint {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for ZwpTextInputV3ContentHint {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for ZwpTextInputV3ContentHint {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for ZwpTextInputV3ContentHint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for ZwpTextInputV3ContentHint {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for ZwpTextInputV3ContentHint {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for ZwpTextInputV3ContentHint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = self.0;
        let mut first = true;
        if v & 0x1 == 0x1 {
            v &= !0x1;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("COMPLETION")?;
        }
        if v & 0x2 == 0x2 {
            v &= !0x2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("SPELLCHECK")?;
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
pub struct ZwpTextInputV3ContentPurpose(pub u32);

impl ZwpTextInputV3ContentPurpose {
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

    /// input a password (combine with sensitive_data hint)
    pub const PASSWORD: Self = Self(8);

    /// input is a numeric password (combine with sensitive_data hint)
    pub const PIN: Self = Self(9);

    /// input a date
    pub const DATE: Self = Self(10);

    /// input a time
    pub const TIME: Self = Self(11);

    /// input a date and time
    pub const DATETIME: Self = Self(12);

    /// input for a terminal
    pub const TERMINAL: Self = Self(13);
}

impl Debug for ZwpTextInputV3ContentPurpose {
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
            Self::PIN => "PIN",
            Self::DATE => "DATE",
            Self::TIME => "TIME",
            Self::DATETIME => "DATETIME",
            Self::TERMINAL => "TERMINAL",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
