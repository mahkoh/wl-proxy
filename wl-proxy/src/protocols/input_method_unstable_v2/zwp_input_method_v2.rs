//! input method
//!
//! An input method object allows for clients to compose text.
//!
//! The objects connects the client to a text input in an application, and
//! lets the client to serve as an input method for a seat.
//!
//! The zwp_input_method_v2 object can occupy two distinct states: active and
//! inactive. In the active state, the object is associated to and
//! communicates with a text input. In the inactive state, there is no
//! associated text input, and the only communication is with the compositor.
//! Initially, the input method is in the inactive state.
//!
//! Requests issued in the inactive state must be accepted by the compositor.
//! Because of the serial mechanism, and the state reset on activate event,
//! they will not have any effect on the state of the next text input.
//!
//! There must be no more than one input method object per seat.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_input_method_v2 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpInputMethodV2 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpInputMethodV2Handler>,
}

struct DefaultHandler;

impl ZwpInputMethodV2Handler for DefaultHandler { }

impl ConcreteObject for ZwpInputMethodV2 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpInputMethodV2;
    const INTERFACE_NAME: &str = "zwp_input_method_v2";
}

impl ZwpInputMethodV2 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpInputMethodV2Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpInputMethodV2Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpInputMethodV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpInputMethodV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpInputMethodV2 {
    /// Since when the activate message is available.
    pub const MSG__ACTIVATE__SINCE: u32 = 1;

    /// input method has been requested
    ///
    /// Notification that a text input focused on this seat requested the input
    /// method to be activated.
    ///
    /// This event serves the purpose of providing the compositor with an
    /// active input method.
    ///
    /// This event resets all state associated with previous enable, disable,
    /// surrounding_text, text_change_cause, and content_type events, as well
    /// as the state associated with set_preedit_string, commit_string, and
    /// delete_surrounding_text requests. In addition, it marks the
    /// zwp_input_method_v2 object as active, and makes any existing
    /// zwp_input_popup_surface_v2 objects visible.
    ///
    /// The surrounding_text, and content_type events must follow before the
    /// next done event if the text input supports the respective
    /// functionality.
    ///
    /// State set with this event is double-buffered. It will get applied on
    /// the next zwp_input_method_v2.done event, and stay valid until changed.
    #[inline]
    pub fn try_send_activate(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_v2#{}.activate()\n", client_id, id);
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

    /// input method has been requested
    ///
    /// Notification that a text input focused on this seat requested the input
    /// method to be activated.
    ///
    /// This event serves the purpose of providing the compositor with an
    /// active input method.
    ///
    /// This event resets all state associated with previous enable, disable,
    /// surrounding_text, text_change_cause, and content_type events, as well
    /// as the state associated with set_preedit_string, commit_string, and
    /// delete_surrounding_text requests. In addition, it marks the
    /// zwp_input_method_v2 object as active, and makes any existing
    /// zwp_input_popup_surface_v2 objects visible.
    ///
    /// The surrounding_text, and content_type events must follow before the
    /// next done event if the text input supports the respective
    /// functionality.
    ///
    /// State set with this event is double-buffered. It will get applied on
    /// the next zwp_input_method_v2.done event, and stay valid until changed.
    #[inline]
    pub fn send_activate(
        &self,
    ) {
        let res = self.try_send_activate(
        );
        if let Err(e) = res {
            log_send("zwp_input_method_v2.activate", &e);
        }
    }

    /// Since when the deactivate message is available.
    pub const MSG__DEACTIVATE__SINCE: u32 = 1;

    /// deactivate event
    ///
    /// Notification that no focused text input currently needs an active 
    /// input method on this seat.
    ///
    /// This event marks the zwp_input_method_v2 object as inactive. The
    /// compositor must make all existing zwp_input_popup_surface_v2 objects
    /// invisible until the next activate event.
    ///
    /// State set with this event is double-buffered. It will get applied on
    /// the next zwp_input_method_v2.done event, and stay valid until changed.
    #[inline]
    pub fn try_send_deactivate(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_v2#{}.deactivate()\n", client_id, id);
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

    /// deactivate event
    ///
    /// Notification that no focused text input currently needs an active 
    /// input method on this seat.
    ///
    /// This event marks the zwp_input_method_v2 object as inactive. The
    /// compositor must make all existing zwp_input_popup_surface_v2 objects
    /// invisible until the next activate event.
    ///
    /// State set with this event is double-buffered. It will get applied on
    /// the next zwp_input_method_v2.done event, and stay valid until changed.
    #[inline]
    pub fn send_deactivate(
        &self,
    ) {
        let res = self.try_send_deactivate(
        );
        if let Err(e) = res {
            log_send("zwp_input_method_v2.deactivate", &e);
        }
    }

    /// Since when the surrounding_text message is available.
    pub const MSG__SURROUNDING_TEXT__SINCE: u32 = 1;

    /// surrounding text event
    ///
    /// Updates the surrounding plain text around the cursor, excluding the
    /// preedit text.
    ///
    /// If any preedit text is present, it is replaced with the cursor for the
    /// purpose of this event.
    ///
    /// The argument text is a buffer containing the preedit string, and must
    /// include the cursor position, and the complete selection. It should
    /// contain additional characters before and after these. There is a
    /// maximum length of wayland messages, so text can not be longer than 4000
    /// bytes.
    ///
    /// cursor is the byte offset of the cursor within the text buffer.
    ///
    /// anchor is the byte offset of the selection anchor within the text
    /// buffer. If there is no selected text, anchor must be the same as
    /// cursor.
    ///
    /// If this event does not arrive before the first done event, the input
    /// method may assume that the text input does not support this
    /// functionality and ignore following surrounding_text events.
    ///
    /// Values set with this event are double-buffered. They will get applied
    /// and set to initial values on the next zwp_input_method_v2.done
    /// event.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_v2#{}.surrounding_text(text: {:?}, cursor: {}, anchor: {})\n", client_id, id, arg0, arg1, arg2);
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
        fmt.string(arg0);
        fmt.words([
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// surrounding text event
    ///
    /// Updates the surrounding plain text around the cursor, excluding the
    /// preedit text.
    ///
    /// If any preedit text is present, it is replaced with the cursor for the
    /// purpose of this event.
    ///
    /// The argument text is a buffer containing the preedit string, and must
    /// include the cursor position, and the complete selection. It should
    /// contain additional characters before and after these. There is a
    /// maximum length of wayland messages, so text can not be longer than 4000
    /// bytes.
    ///
    /// cursor is the byte offset of the cursor within the text buffer.
    ///
    /// anchor is the byte offset of the selection anchor within the text
    /// buffer. If there is no selected text, anchor must be the same as
    /// cursor.
    ///
    /// If this event does not arrive before the first done event, the input
    /// method may assume that the text input does not support this
    /// functionality and ignore following surrounding_text events.
    ///
    /// Values set with this event are double-buffered. They will get applied
    /// and set to initial values on the next zwp_input_method_v2.done
    /// event.
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
            log_send("zwp_input_method_v2.surrounding_text", &e);
        }
    }

    /// Since when the text_change_cause message is available.
    pub const MSG__TEXT_CHANGE_CAUSE__SINCE: u32 = 1;

    /// indicates the cause of surrounding text change
    ///
    /// Tells the input method why the text surrounding the cursor changed.
    ///
    /// Whenever the client detects an external change in text, cursor, or
    /// anchor position, it must issue this request to the compositor. This
    /// request is intended to give the input method a chance to update the
    /// preedit text in an appropriate way, e.g. by removing it when the user
    /// starts typing with a keyboard.
    ///
    /// cause describes the source of the change.
    ///
    /// The value set with this event is double-buffered. It will get applied
    /// and set to its initial value on the next zwp_input_method_v2.done
    /// event.
    ///
    /// The initial value of cause is input_method.
    ///
    /// # Arguments
    ///
    /// - `cause`:
    #[inline]
    pub fn try_send_text_change_cause(
        &self,
        cause: ZwpTextInputV3ChangeCause,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            cause,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: ZwpTextInputV3ChangeCause) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_v2#{}.text_change_cause(cause: {:?})\n", client_id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// indicates the cause of surrounding text change
    ///
    /// Tells the input method why the text surrounding the cursor changed.
    ///
    /// Whenever the client detects an external change in text, cursor, or
    /// anchor position, it must issue this request to the compositor. This
    /// request is intended to give the input method a chance to update the
    /// preedit text in an appropriate way, e.g. by removing it when the user
    /// starts typing with a keyboard.
    ///
    /// cause describes the source of the change.
    ///
    /// The value set with this event is double-buffered. It will get applied
    /// and set to its initial value on the next zwp_input_method_v2.done
    /// event.
    ///
    /// The initial value of cause is input_method.
    ///
    /// # Arguments
    ///
    /// - `cause`:
    #[inline]
    pub fn send_text_change_cause(
        &self,
        cause: ZwpTextInputV3ChangeCause,
    ) {
        let res = self.try_send_text_change_cause(
            cause,
        );
        if let Err(e) = res {
            log_send("zwp_input_method_v2.text_change_cause", &e);
        }
    }

    /// Since when the content_type message is available.
    pub const MSG__CONTENT_TYPE__SINCE: u32 = 1;

    /// content purpose and hint
    ///
    /// Indicates the content type and hint for the current
    /// zwp_input_method_v2 instance.
    ///
    /// Values set with this event are double-buffered. They will get applied
    /// on the next zwp_input_method_v2.done event.
    ///
    /// The initial value for hint is none, and the initial value for purpose
    /// is normal.
    ///
    /// # Arguments
    ///
    /// - `hint`:
    /// - `purpose`:
    #[inline]
    pub fn try_send_content_type(
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
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: ZwpTextInputV3ContentHint, arg1: ZwpTextInputV3ContentPurpose) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_v2#{}.content_type(hint: {:?}, purpose: {:?})\n", client_id, id, arg0, arg1);
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
            arg0.0,
            arg1.0,
        ]);
        Ok(())
    }

    /// content purpose and hint
    ///
    /// Indicates the content type and hint for the current
    /// zwp_input_method_v2 instance.
    ///
    /// Values set with this event are double-buffered. They will get applied
    /// on the next zwp_input_method_v2.done event.
    ///
    /// The initial value for hint is none, and the initial value for purpose
    /// is normal.
    ///
    /// # Arguments
    ///
    /// - `hint`:
    /// - `purpose`:
    #[inline]
    pub fn send_content_type(
        &self,
        hint: ZwpTextInputV3ContentHint,
        purpose: ZwpTextInputV3ContentPurpose,
    ) {
        let res = self.try_send_content_type(
            hint,
            purpose,
        );
        if let Err(e) = res {
            log_send("zwp_input_method_v2.content_type", &e);
        }
    }

    /// Since when the done message is available.
    pub const MSG__DONE__SINCE: u32 = 1;

    /// apply state
    ///
    /// Atomically applies state changes recently sent to the client.
    ///
    /// The done event establishes and updates the state of the client, and
    /// must be issued after any changes to apply them.
    ///
    /// Text input state (content purpose, content hint, surrounding text, and
    /// change cause) is conceptually double-buffered within an input method
    /// context.
    ///
    /// Events modify the pending state, as opposed to the current state in use
    /// by the input method. A done event atomically applies all pending state,
    /// replacing the current state. After done, the new pending state is as
    /// documented for each related request.
    ///
    /// Events must be applied in the order of arrival.
    ///
    /// Neither current nor pending state are modified unless noted otherwise.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_v2#{}.done()\n", client_id, id);
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
            5,
        ]);
        Ok(())
    }

    /// apply state
    ///
    /// Atomically applies state changes recently sent to the client.
    ///
    /// The done event establishes and updates the state of the client, and
    /// must be issued after any changes to apply them.
    ///
    /// Text input state (content purpose, content hint, surrounding text, and
    /// change cause) is conceptually double-buffered within an input method
    /// context.
    ///
    /// Events modify the pending state, as opposed to the current state in use
    /// by the input method. A done event atomically applies all pending state,
    /// replacing the current state. After done, the new pending state is as
    /// documented for each related request.
    ///
    /// Events must be applied in the order of arrival.
    ///
    /// Neither current nor pending state are modified unless noted otherwise.
    #[inline]
    pub fn send_done(
        &self,
    ) {
        let res = self.try_send_done(
        );
        if let Err(e) = res {
            log_send("zwp_input_method_v2.done", &e);
        }
    }

    /// Since when the commit_string message is available.
    pub const MSG__COMMIT_STRING__SINCE: u32 = 1;

    /// commit string
    ///
    /// Send the commit string text for insertion to the application.
    ///
    /// Inserts a string at current cursor position (see commit event
    /// sequence). The string to commit could be either just a single character
    /// after a key press or the result of some composing.
    ///
    /// The argument text is a buffer containing the string to insert. There is
    /// a maximum length of wayland messages, so text can not be longer than
    /// 4000 bytes.
    ///
    /// Values set with this event are double-buffered. They must be applied
    /// and reset to initial on the next zwp_text_input_v3.commit request.
    ///
    /// The initial value of text is an empty string.
    ///
    /// # Arguments
    ///
    /// - `text`:
    #[inline]
    pub fn try_send_commit_string(
        &self,
        text: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            text,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_v2#{}.commit_string(text: {:?})\n", id, arg0);
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
            0,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// commit string
    ///
    /// Send the commit string text for insertion to the application.
    ///
    /// Inserts a string at current cursor position (see commit event
    /// sequence). The string to commit could be either just a single character
    /// after a key press or the result of some composing.
    ///
    /// The argument text is a buffer containing the string to insert. There is
    /// a maximum length of wayland messages, so text can not be longer than
    /// 4000 bytes.
    ///
    /// Values set with this event are double-buffered. They must be applied
    /// and reset to initial on the next zwp_text_input_v3.commit request.
    ///
    /// The initial value of text is an empty string.
    ///
    /// # Arguments
    ///
    /// - `text`:
    #[inline]
    pub fn send_commit_string(
        &self,
        text: &str,
    ) {
        let res = self.try_send_commit_string(
            text,
        );
        if let Err(e) = res {
            log_send("zwp_input_method_v2.commit_string", &e);
        }
    }

    /// Since when the set_preedit_string message is available.
    pub const MSG__SET_PREEDIT_STRING__SINCE: u32 = 1;

    /// pre-edit string
    ///
    /// Send the pre-edit string text to the application text input.
    ///
    /// Place a new composing text (pre-edit) at the current cursor position.
    /// Any previously set composing text must be removed. Any previously
    /// existing selected text must be removed. The cursor is moved to a new
    /// position within the preedit string.
    ///
    /// The argument text is a buffer containing the preedit string. There is
    /// a maximum length of wayland messages, so text can not be longer than
    /// 4000 bytes.
    ///
    /// The arguments cursor_begin and cursor_end are counted in bytes relative
    /// to the beginning of the submitted string buffer. Cursor should be
    /// hidden by the text input when both are equal to -1.
    ///
    /// cursor_begin indicates the beginning of the cursor. cursor_end
    /// indicates the end of the cursor. It may be equal or different than
    /// cursor_begin.
    ///
    /// Values set with this event are double-buffered. They must be applied on
    /// the next zwp_input_method_v2.commit event.
    ///
    /// The initial value of text is an empty string. The initial value of
    /// cursor_begin, and cursor_end are both 0.
    ///
    /// # Arguments
    ///
    /// - `text`:
    /// - `cursor_begin`:
    /// - `cursor_end`:
    #[inline]
    pub fn try_send_set_preedit_string(
        &self,
        text: &str,
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str, arg1: i32, arg2: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_v2#{}.set_preedit_string(text: {:?}, cursor_begin: {}, cursor_end: {})\n", id, arg0, arg1, arg2);
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
            1,
        ]);
        fmt.string(arg0);
        fmt.words([
            arg1 as u32,
            arg2 as u32,
        ]);
        Ok(())
    }

    /// pre-edit string
    ///
    /// Send the pre-edit string text to the application text input.
    ///
    /// Place a new composing text (pre-edit) at the current cursor position.
    /// Any previously set composing text must be removed. Any previously
    /// existing selected text must be removed. The cursor is moved to a new
    /// position within the preedit string.
    ///
    /// The argument text is a buffer containing the preedit string. There is
    /// a maximum length of wayland messages, so text can not be longer than
    /// 4000 bytes.
    ///
    /// The arguments cursor_begin and cursor_end are counted in bytes relative
    /// to the beginning of the submitted string buffer. Cursor should be
    /// hidden by the text input when both are equal to -1.
    ///
    /// cursor_begin indicates the beginning of the cursor. cursor_end
    /// indicates the end of the cursor. It may be equal or different than
    /// cursor_begin.
    ///
    /// Values set with this event are double-buffered. They must be applied on
    /// the next zwp_input_method_v2.commit event.
    ///
    /// The initial value of text is an empty string. The initial value of
    /// cursor_begin, and cursor_end are both 0.
    ///
    /// # Arguments
    ///
    /// - `text`:
    /// - `cursor_begin`:
    /// - `cursor_end`:
    #[inline]
    pub fn send_set_preedit_string(
        &self,
        text: &str,
        cursor_begin: i32,
        cursor_end: i32,
    ) {
        let res = self.try_send_set_preedit_string(
            text,
            cursor_begin,
            cursor_end,
        );
        if let Err(e) = res {
            log_send("zwp_input_method_v2.set_preedit_string", &e);
        }
    }

    /// Since when the delete_surrounding_text message is available.
    pub const MSG__DELETE_SURROUNDING_TEXT__SINCE: u32 = 1;

    /// delete text
    ///
    /// Remove the surrounding text.
    ///
    /// before_length and after_length are the number of bytes before and after
    /// the current cursor index (excluding the preedit text) to delete.
    ///
    /// If any preedit text is present, it is replaced with the cursor for the
    /// purpose of this event. In effect before_length is counted from the
    /// beginning of preedit text, and after_length from its end (see commit
    /// event sequence).
    ///
    /// Values set with this event are double-buffered. They must be applied
    /// and reset to initial on the next zwp_input_method_v2.commit request.
    ///
    /// The initial values of both before_length and after_length are 0.
    ///
    /// # Arguments
    ///
    /// - `before_length`:
    /// - `after_length`:
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_v2#{}.delete_surrounding_text(before_length: {}, after_length: {})\n", id, arg0, arg1);
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
            2,
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// delete text
    ///
    /// Remove the surrounding text.
    ///
    /// before_length and after_length are the number of bytes before and after
    /// the current cursor index (excluding the preedit text) to delete.
    ///
    /// If any preedit text is present, it is replaced with the cursor for the
    /// purpose of this event. In effect before_length is counted from the
    /// beginning of preedit text, and after_length from its end (see commit
    /// event sequence).
    ///
    /// Values set with this event are double-buffered. They must be applied
    /// and reset to initial on the next zwp_input_method_v2.commit request.
    ///
    /// The initial values of both before_length and after_length are 0.
    ///
    /// # Arguments
    ///
    /// - `before_length`:
    /// - `after_length`:
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
            log_send("zwp_input_method_v2.delete_surrounding_text", &e);
        }
    }

    /// Since when the commit message is available.
    pub const MSG__COMMIT__SINCE: u32 = 1;

    /// apply state
    ///
    /// Apply state changes from commit_string, set_preedit_string and
    /// delete_surrounding_text requests.
    ///
    /// The state relating to these events is double-buffered, and each one
    /// modifies the pending state. This request replaces the current state
    /// with the pending state.
    ///
    /// The connected text input is expected to proceed by evaluating the
    /// changes in the following order:
    ///
    /// 1. Replace existing preedit string with the cursor.
    /// 2. Delete requested surrounding text.
    /// 3. Insert commit string with the cursor at its end.
    /// 4. Calculate surrounding text to send.
    /// 5. Insert new preedit text in cursor position.
    /// 6. Place cursor inside preedit text.
    ///
    /// The serial number reflects the last state of the zwp_input_method_v2
    /// object known to the client. The value of the serial argument must be
    /// equal to the number of done events already issued by that object. When
    /// the compositor receives a commit request with a serial different than
    /// the number of past done events, it must proceed as normal, except it
    /// should not change the current state of the zwp_input_method_v2 object.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    #[inline]
    pub fn try_send_commit(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_v2#{}.commit(serial: {})\n", id, arg0);
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
            3,
            arg0,
        ]);
        Ok(())
    }

    /// apply state
    ///
    /// Apply state changes from commit_string, set_preedit_string and
    /// delete_surrounding_text requests.
    ///
    /// The state relating to these events is double-buffered, and each one
    /// modifies the pending state. This request replaces the current state
    /// with the pending state.
    ///
    /// The connected text input is expected to proceed by evaluating the
    /// changes in the following order:
    ///
    /// 1. Replace existing preedit string with the cursor.
    /// 2. Delete requested surrounding text.
    /// 3. Insert commit string with the cursor at its end.
    /// 4. Calculate surrounding text to send.
    /// 5. Insert new preedit text in cursor position.
    /// 6. Place cursor inside preedit text.
    ///
    /// The serial number reflects the last state of the zwp_input_method_v2
    /// object known to the client. The value of the serial argument must be
    /// equal to the number of done events already issued by that object. When
    /// the compositor receives a commit request with a serial different than
    /// the number of past done events, it must proceed as normal, except it
    /// should not change the current state of the zwp_input_method_v2 object.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    #[inline]
    pub fn send_commit(
        &self,
        serial: u32,
    ) {
        let res = self.try_send_commit(
            serial,
        );
        if let Err(e) = res {
            log_send("zwp_input_method_v2.commit", &e);
        }
    }

    /// Since when the get_input_popup_surface message is available.
    pub const MSG__GET_INPUT_POPUP_SURFACE__SINCE: u32 = 1;

    /// create popup surface
    ///
    /// Creates a new zwp_input_popup_surface_v2 object wrapping a given
    /// surface.
    ///
    /// The surface gets assigned the "input_popup" role. If the surface
    /// already has an assigned role, the compositor must issue a protocol
    /// error.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn try_send_get_input_popup_surface(
        &self,
        id: &Rc<ZwpInputPopupSurfaceV2>,
        surface: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            surface,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_v2#{}.get_input_popup_surface(id: zwp_input_popup_surface_v2#{}, surface: wl_surface#{})\n", id, arg0, arg1);
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
            4,
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// create popup surface
    ///
    /// Creates a new zwp_input_popup_surface_v2 object wrapping a given
    /// surface.
    ///
    /// The surface gets assigned the "input_popup" role. If the surface
    /// already has an assigned role, the compositor must issue a protocol
    /// error.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn send_get_input_popup_surface(
        &self,
        id: &Rc<ZwpInputPopupSurfaceV2>,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_get_input_popup_surface(
            id,
            surface,
        );
        if let Err(e) = res {
            log_send("zwp_input_method_v2.get_input_popup_surface", &e);
        }
    }

    /// create popup surface
    ///
    /// Creates a new zwp_input_popup_surface_v2 object wrapping a given
    /// surface.
    ///
    /// The surface gets assigned the "input_popup" role. If the surface
    /// already has an assigned role, the compositor must issue a protocol
    /// error.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn new_try_send_get_input_popup_surface(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Result<Rc<ZwpInputPopupSurfaceV2>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_input_popup_surface(
            &id,
            surface,
        )?;
        Ok(id)
    }

    /// create popup surface
    ///
    /// Creates a new zwp_input_popup_surface_v2 object wrapping a given
    /// surface.
    ///
    /// The surface gets assigned the "input_popup" role. If the surface
    /// already has an assigned role, the compositor must issue a protocol
    /// error.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn new_send_get_input_popup_surface(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Rc<ZwpInputPopupSurfaceV2> {
        let id = self.core.create_child();
        self.send_get_input_popup_surface(
            &id,
            surface,
        );
        id
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
    /// The compositor should send all keyboard events on the seat to the grab
    /// holder via the returned wl_keyboard object. Nevertheless, the
    /// compositor may decide not to forward any particular event. The
    /// compositor must not further process any event after it has been
    /// forwarded to the grab holder.
    ///
    /// Releasing the resulting wl_keyboard object releases the grab.
    ///
    /// # Arguments
    ///
    /// - `keyboard`:
    #[inline]
    pub fn try_send_grab_keyboard(
        &self,
        keyboard: &Rc<ZwpInputMethodKeyboardGrabV2>,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_v2#{}.grab_keyboard(keyboard: zwp_input_method_keyboard_grab_v2#{})\n", id, arg0);
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

    /// grab hardware keyboard
    ///
    /// Allow an input method to receive hardware keyboard input and process
    /// key events to generate text events (with pre-edit) over the wire. This
    /// allows input methods which compose multiple key events for inputting
    /// text like it is done for CJK languages.
    ///
    /// The compositor should send all keyboard events on the seat to the grab
    /// holder via the returned wl_keyboard object. Nevertheless, the
    /// compositor may decide not to forward any particular event. The
    /// compositor must not further process any event after it has been
    /// forwarded to the grab holder.
    ///
    /// Releasing the resulting wl_keyboard object releases the grab.
    ///
    /// # Arguments
    ///
    /// - `keyboard`:
    #[inline]
    pub fn send_grab_keyboard(
        &self,
        keyboard: &Rc<ZwpInputMethodKeyboardGrabV2>,
    ) {
        let res = self.try_send_grab_keyboard(
            keyboard,
        );
        if let Err(e) = res {
            log_send("zwp_input_method_v2.grab_keyboard", &e);
        }
    }

    /// grab hardware keyboard
    ///
    /// Allow an input method to receive hardware keyboard input and process
    /// key events to generate text events (with pre-edit) over the wire. This
    /// allows input methods which compose multiple key events for inputting
    /// text like it is done for CJK languages.
    ///
    /// The compositor should send all keyboard events on the seat to the grab
    /// holder via the returned wl_keyboard object. Nevertheless, the
    /// compositor may decide not to forward any particular event. The
    /// compositor must not further process any event after it has been
    /// forwarded to the grab holder.
    ///
    /// Releasing the resulting wl_keyboard object releases the grab.
    #[inline]
    pub fn new_try_send_grab_keyboard(
        &self,
    ) -> Result<Rc<ZwpInputMethodKeyboardGrabV2>, ObjectError> {
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
    ///
    /// The compositor should send all keyboard events on the seat to the grab
    /// holder via the returned wl_keyboard object. Nevertheless, the
    /// compositor may decide not to forward any particular event. The
    /// compositor must not further process any event after it has been
    /// forwarded to the grab holder.
    ///
    /// Releasing the resulting wl_keyboard object releases the grab.
    #[inline]
    pub fn new_send_grab_keyboard(
        &self,
    ) -> Rc<ZwpInputMethodKeyboardGrabV2> {
        let keyboard = self.core.create_child();
        self.send_grab_keyboard(
            &keyboard,
        );
        keyboard
    }

    /// Since when the unavailable message is available.
    pub const MSG__UNAVAILABLE__SINCE: u32 = 1;

    /// input method unavailable
    ///
    /// The input method ceased to be available.
    ///
    /// The compositor must issue this event as the only event on the object if
    /// there was another input_method object associated with the same seat at
    /// the time of its creation.
    ///
    /// The compositor must issue this request when the object is no longer
    /// usable, e.g. due to seat removal.
    ///
    /// The input method context becomes inert and should be destroyed after
    /// deactivation is handled. Any further requests and events except for the
    /// destroy request must be ignored.
    #[inline]
    pub fn try_send_unavailable(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_input_method_v2#{}.unavailable()\n", client_id, id);
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
            6,
        ]);
        Ok(())
    }

    /// input method unavailable
    ///
    /// The input method ceased to be available.
    ///
    /// The compositor must issue this event as the only event on the object if
    /// there was another input_method object associated with the same seat at
    /// the time of its creation.
    ///
    /// The compositor must issue this request when the object is no longer
    /// usable, e.g. due to seat removal.
    ///
    /// The input method context becomes inert and should be destroyed after
    /// deactivation is handled. Any further requests and events except for the
    /// destroy request must be ignored.
    #[inline]
    pub fn send_unavailable(
        &self,
    ) {
        let res = self.try_send_unavailable(
        );
        if let Err(e) = res {
            log_send("zwp_input_method_v2.unavailable", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the text input
    ///
    /// Destroys the zwp_text_input_v2 object and any associated child
    /// objects, i.e. zwp_input_popup_surface_v2 and
    /// zwp_input_method_keyboard_grab_v2.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_input_method_v2#{}.destroy()\n", id);
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
            6,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the text input
    ///
    /// Destroys the zwp_text_input_v2 object and any associated child
    /// objects, i.e. zwp_input_popup_surface_v2 and
    /// zwp_input_method_keyboard_grab_v2.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwp_input_method_v2.destroy", &e);
        }
    }
}

/// A message handler for [`ZwpInputMethodV2`] proxies.
pub trait ZwpInputMethodV2Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpInputMethodV2>) {
        slf.core.delete_id();
    }

    /// input method has been requested
    ///
    /// Notification that a text input focused on this seat requested the input
    /// method to be activated.
    ///
    /// This event serves the purpose of providing the compositor with an
    /// active input method.
    ///
    /// This event resets all state associated with previous enable, disable,
    /// surrounding_text, text_change_cause, and content_type events, as well
    /// as the state associated with set_preedit_string, commit_string, and
    /// delete_surrounding_text requests. In addition, it marks the
    /// zwp_input_method_v2 object as active, and makes any existing
    /// zwp_input_popup_surface_v2 objects visible.
    ///
    /// The surrounding_text, and content_type events must follow before the
    /// next done event if the text input supports the respective
    /// functionality.
    ///
    /// State set with this event is double-buffered. It will get applied on
    /// the next zwp_input_method_v2.done event, and stay valid until changed.
    #[inline]
    fn handle_activate(
        &mut self,
        slf: &Rc<ZwpInputMethodV2>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_activate(
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_v2.activate", &e);
        }
    }

    /// deactivate event
    ///
    /// Notification that no focused text input currently needs an active 
    /// input method on this seat.
    ///
    /// This event marks the zwp_input_method_v2 object as inactive. The
    /// compositor must make all existing zwp_input_popup_surface_v2 objects
    /// invisible until the next activate event.
    ///
    /// State set with this event is double-buffered. It will get applied on
    /// the next zwp_input_method_v2.done event, and stay valid until changed.
    #[inline]
    fn handle_deactivate(
        &mut self,
        slf: &Rc<ZwpInputMethodV2>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_deactivate(
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_v2.deactivate", &e);
        }
    }

    /// surrounding text event
    ///
    /// Updates the surrounding plain text around the cursor, excluding the
    /// preedit text.
    ///
    /// If any preedit text is present, it is replaced with the cursor for the
    /// purpose of this event.
    ///
    /// The argument text is a buffer containing the preedit string, and must
    /// include the cursor position, and the complete selection. It should
    /// contain additional characters before and after these. There is a
    /// maximum length of wayland messages, so text can not be longer than 4000
    /// bytes.
    ///
    /// cursor is the byte offset of the cursor within the text buffer.
    ///
    /// anchor is the byte offset of the selection anchor within the text
    /// buffer. If there is no selected text, anchor must be the same as
    /// cursor.
    ///
    /// If this event does not arrive before the first done event, the input
    /// method may assume that the text input does not support this
    /// functionality and ignore following surrounding_text events.
    ///
    /// Values set with this event are double-buffered. They will get applied
    /// and set to initial values on the next zwp_input_method_v2.done
    /// event.
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
    fn handle_surrounding_text(
        &mut self,
        slf: &Rc<ZwpInputMethodV2>,
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
            log_forward("zwp_input_method_v2.surrounding_text", &e);
        }
    }

    /// indicates the cause of surrounding text change
    ///
    /// Tells the input method why the text surrounding the cursor changed.
    ///
    /// Whenever the client detects an external change in text, cursor, or
    /// anchor position, it must issue this request to the compositor. This
    /// request is intended to give the input method a chance to update the
    /// preedit text in an appropriate way, e.g. by removing it when the user
    /// starts typing with a keyboard.
    ///
    /// cause describes the source of the change.
    ///
    /// The value set with this event is double-buffered. It will get applied
    /// and set to its initial value on the next zwp_input_method_v2.done
    /// event.
    ///
    /// The initial value of cause is input_method.
    ///
    /// # Arguments
    ///
    /// - `cause`:
    #[inline]
    fn handle_text_change_cause(
        &mut self,
        slf: &Rc<ZwpInputMethodV2>,
        cause: ZwpTextInputV3ChangeCause,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_text_change_cause(
            cause,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_v2.text_change_cause", &e);
        }
    }

    /// content purpose and hint
    ///
    /// Indicates the content type and hint for the current
    /// zwp_input_method_v2 instance.
    ///
    /// Values set with this event are double-buffered. They will get applied
    /// on the next zwp_input_method_v2.done event.
    ///
    /// The initial value for hint is none, and the initial value for purpose
    /// is normal.
    ///
    /// # Arguments
    ///
    /// - `hint`:
    /// - `purpose`:
    #[inline]
    fn handle_content_type(
        &mut self,
        slf: &Rc<ZwpInputMethodV2>,
        hint: ZwpTextInputV3ContentHint,
        purpose: ZwpTextInputV3ContentPurpose,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_content_type(
            hint,
            purpose,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_v2.content_type", &e);
        }
    }

    /// apply state
    ///
    /// Atomically applies state changes recently sent to the client.
    ///
    /// The done event establishes and updates the state of the client, and
    /// must be issued after any changes to apply them.
    ///
    /// Text input state (content purpose, content hint, surrounding text, and
    /// change cause) is conceptually double-buffered within an input method
    /// context.
    ///
    /// Events modify the pending state, as opposed to the current state in use
    /// by the input method. A done event atomically applies all pending state,
    /// replacing the current state. After done, the new pending state is as
    /// documented for each related request.
    ///
    /// Events must be applied in the order of arrival.
    ///
    /// Neither current nor pending state are modified unless noted otherwise.
    #[inline]
    fn handle_done(
        &mut self,
        slf: &Rc<ZwpInputMethodV2>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_done(
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_v2.done", &e);
        }
    }

    /// commit string
    ///
    /// Send the commit string text for insertion to the application.
    ///
    /// Inserts a string at current cursor position (see commit event
    /// sequence). The string to commit could be either just a single character
    /// after a key press or the result of some composing.
    ///
    /// The argument text is a buffer containing the string to insert. There is
    /// a maximum length of wayland messages, so text can not be longer than
    /// 4000 bytes.
    ///
    /// Values set with this event are double-buffered. They must be applied
    /// and reset to initial on the next zwp_text_input_v3.commit request.
    ///
    /// The initial value of text is an empty string.
    ///
    /// # Arguments
    ///
    /// - `text`:
    #[inline]
    fn handle_commit_string(
        &mut self,
        slf: &Rc<ZwpInputMethodV2>,
        text: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_commit_string(
            text,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_v2.commit_string", &e);
        }
    }

    /// pre-edit string
    ///
    /// Send the pre-edit string text to the application text input.
    ///
    /// Place a new composing text (pre-edit) at the current cursor position.
    /// Any previously set composing text must be removed. Any previously
    /// existing selected text must be removed. The cursor is moved to a new
    /// position within the preedit string.
    ///
    /// The argument text is a buffer containing the preedit string. There is
    /// a maximum length of wayland messages, so text can not be longer than
    /// 4000 bytes.
    ///
    /// The arguments cursor_begin and cursor_end are counted in bytes relative
    /// to the beginning of the submitted string buffer. Cursor should be
    /// hidden by the text input when both are equal to -1.
    ///
    /// cursor_begin indicates the beginning of the cursor. cursor_end
    /// indicates the end of the cursor. It may be equal or different than
    /// cursor_begin.
    ///
    /// Values set with this event are double-buffered. They must be applied on
    /// the next zwp_input_method_v2.commit event.
    ///
    /// The initial value of text is an empty string. The initial value of
    /// cursor_begin, and cursor_end are both 0.
    ///
    /// # Arguments
    ///
    /// - `text`:
    /// - `cursor_begin`:
    /// - `cursor_end`:
    #[inline]
    fn handle_set_preedit_string(
        &mut self,
        slf: &Rc<ZwpInputMethodV2>,
        text: &str,
        cursor_begin: i32,
        cursor_end: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_preedit_string(
            text,
            cursor_begin,
            cursor_end,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_v2.set_preedit_string", &e);
        }
    }

    /// delete text
    ///
    /// Remove the surrounding text.
    ///
    /// before_length and after_length are the number of bytes before and after
    /// the current cursor index (excluding the preedit text) to delete.
    ///
    /// If any preedit text is present, it is replaced with the cursor for the
    /// purpose of this event. In effect before_length is counted from the
    /// beginning of preedit text, and after_length from its end (see commit
    /// event sequence).
    ///
    /// Values set with this event are double-buffered. They must be applied
    /// and reset to initial on the next zwp_input_method_v2.commit request.
    ///
    /// The initial values of both before_length and after_length are 0.
    ///
    /// # Arguments
    ///
    /// - `before_length`:
    /// - `after_length`:
    #[inline]
    fn handle_delete_surrounding_text(
        &mut self,
        slf: &Rc<ZwpInputMethodV2>,
        before_length: u32,
        after_length: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_delete_surrounding_text(
            before_length,
            after_length,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_v2.delete_surrounding_text", &e);
        }
    }

    /// apply state
    ///
    /// Apply state changes from commit_string, set_preedit_string and
    /// delete_surrounding_text requests.
    ///
    /// The state relating to these events is double-buffered, and each one
    /// modifies the pending state. This request replaces the current state
    /// with the pending state.
    ///
    /// The connected text input is expected to proceed by evaluating the
    /// changes in the following order:
    ///
    /// 1. Replace existing preedit string with the cursor.
    /// 2. Delete requested surrounding text.
    /// 3. Insert commit string with the cursor at its end.
    /// 4. Calculate surrounding text to send.
    /// 5. Insert new preedit text in cursor position.
    /// 6. Place cursor inside preedit text.
    ///
    /// The serial number reflects the last state of the zwp_input_method_v2
    /// object known to the client. The value of the serial argument must be
    /// equal to the number of done events already issued by that object. When
    /// the compositor receives a commit request with a serial different than
    /// the number of past done events, it must proceed as normal, except it
    /// should not change the current state of the zwp_input_method_v2 object.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    #[inline]
    fn handle_commit(
        &mut self,
        slf: &Rc<ZwpInputMethodV2>,
        serial: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_commit(
            serial,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_v2.commit", &e);
        }
    }

    /// create popup surface
    ///
    /// Creates a new zwp_input_popup_surface_v2 object wrapping a given
    /// surface.
    ///
    /// The surface gets assigned the "input_popup" role. If the surface
    /// already has an assigned role, the compositor must issue a protocol
    /// error.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_input_popup_surface(
        &mut self,
        slf: &Rc<ZwpInputMethodV2>,
        id: &Rc<ZwpInputPopupSurfaceV2>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_input_popup_surface(
            id,
            surface,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_v2.get_input_popup_surface", &e);
        }
    }

    /// grab hardware keyboard
    ///
    /// Allow an input method to receive hardware keyboard input and process
    /// key events to generate text events (with pre-edit) over the wire. This
    /// allows input methods which compose multiple key events for inputting
    /// text like it is done for CJK languages.
    ///
    /// The compositor should send all keyboard events on the seat to the grab
    /// holder via the returned wl_keyboard object. Nevertheless, the
    /// compositor may decide not to forward any particular event. The
    /// compositor must not further process any event after it has been
    /// forwarded to the grab holder.
    ///
    /// Releasing the resulting wl_keyboard object releases the grab.
    ///
    /// # Arguments
    ///
    /// - `keyboard`:
    #[inline]
    fn handle_grab_keyboard(
        &mut self,
        slf: &Rc<ZwpInputMethodV2>,
        keyboard: &Rc<ZwpInputMethodKeyboardGrabV2>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_grab_keyboard(
            keyboard,
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_v2.grab_keyboard", &e);
        }
    }

    /// input method unavailable
    ///
    /// The input method ceased to be available.
    ///
    /// The compositor must issue this event as the only event on the object if
    /// there was another input_method object associated with the same seat at
    /// the time of its creation.
    ///
    /// The compositor must issue this request when the object is no longer
    /// usable, e.g. due to seat removal.
    ///
    /// The input method context becomes inert and should be destroyed after
    /// deactivation is handled. Any further requests and events except for the
    /// destroy request must be ignored.
    #[inline]
    fn handle_unavailable(
        &mut self,
        slf: &Rc<ZwpInputMethodV2>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_unavailable(
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_v2.unavailable", &e);
        }
    }

    /// destroy the text input
    ///
    /// Destroys the zwp_text_input_v2 object and any associated child
    /// objects, i.e. zwp_input_popup_surface_v2 and
    /// zwp_input_method_keyboard_grab_v2.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwpInputMethodV2>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwp_input_method_v2.destroy", &e);
        }
    }
}

impl ObjectPrivate for ZwpInputMethodV2 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpInputMethodV2, version),
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
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "text")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_v2#{}.commit_string(text: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_commit_string(&self, arg0);
                } else {
                    DefaultHandler.handle_commit_string(&self, arg0);
                }
            }
            1 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "text")?;
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
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str, arg1: i32, arg2: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_v2#{}.set_preedit_string(text: {:?}, cursor_begin: {}, cursor_end: {})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_preedit_string(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_set_preedit_string(&self, arg0, arg1, arg2);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_v2#{}.delete_surrounding_text(before_length: {}, after_length: {})\n", client_id, id, arg0, arg1);
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
            3 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_v2#{}.commit(serial: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_commit(&self, arg0);
                } else {
                    DefaultHandler.handle_commit(&self, arg0);
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
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_v2#{}.get_input_popup_surface(id: zwp_input_popup_surface_v2#{}, surface: wl_surface#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = ZwpInputPopupSurfaceV2::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
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
                    (**handler).handle_get_input_popup_surface(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_input_popup_surface(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_v2#{}.grab_keyboard(keyboard: zwp_input_method_keyboard_grab_v2#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = ZwpInputMethodKeyboardGrabV2::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "keyboard", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_grab_keyboard(&self, arg0);
                } else {
                    DefaultHandler.handle_grab_keyboard(&self, arg0);
                }
            }
            6 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_input_method_v2#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_v2#{}.activate()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_activate(&self);
                } else {
                    DefaultHandler.handle_activate(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_v2#{}.deactivate()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_deactivate(&self);
                } else {
                    DefaultHandler.handle_deactivate(&self);
                }
            }
            2 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_v2#{}.surrounding_text(text: {:?}, cursor: {}, anchor: {})\n", id, arg0, arg1, arg2);
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
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = ZwpTextInputV3ChangeCause(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: ZwpTextInputV3ChangeCause) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_v2#{}.text_change_cause(cause: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_text_change_cause(&self, arg0);
                } else {
                    DefaultHandler.handle_text_change_cause(&self, arg0);
                }
            }
            4 => {
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
                    fn log(state: &State, id: u32, arg0: ZwpTextInputV3ContentHint, arg1: ZwpTextInputV3ContentPurpose) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_v2#{}.content_type(hint: {:?}, purpose: {:?})\n", id, arg0, arg1);
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
            5 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_v2#{}.done()\n", id);
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
            6 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_input_method_v2#{}.unavailable()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_unavailable(&self);
                } else {
                    DefaultHandler.handle_unavailable(&self);
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
            0 => "commit_string",
            1 => "set_preedit_string",
            2 => "delete_surrounding_text",
            3 => "commit",
            4 => "get_input_popup_surface",
            5 => "grab_keyboard",
            6 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "activate",
            1 => "deactivate",
            2 => "surrounding_text",
            3 => "text_change_cause",
            4 => "content_type",
            5 => "done",
            6 => "unavailable",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwpInputMethodV2 {
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

impl ZwpInputMethodV2 {
    /// Since when the error.role enum variant is available.
    pub const ENM__ERROR_ROLE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwpInputMethodV2Error(pub u32);

impl ZwpInputMethodV2Error {
    /// wl_surface has another role
    pub const ROLE: Self = Self(0);
}

impl Debug for ZwpInputMethodV2Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ROLE => "ROLE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
