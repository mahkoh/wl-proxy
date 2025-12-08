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

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_text_input_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpTextInputV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpTextInputV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpTextInputV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpTextInputV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZwpTextInputV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpTextInputV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwpTextInputV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpTextInputV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpTextInputV1 {
    /// Since when the activate message is available.
    #[allow(dead_code)]
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
    pub fn send_activate(
        &self,
        seat: &Rc<MetaWlSeat>,
        surface: &Rc<MetaWlSurface>,
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
            return Err(ObjectError);
        };
        let arg0 = match arg0.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        let arg1 = match arg1.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// Since when the deactivate message is available.
    #[allow(dead_code)]
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
    pub fn send_deactivate(
        &self,
        seat: &Rc<MetaWlSeat>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            seat,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg0 = match arg0.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the show_input_panel message is available.
    #[allow(dead_code)]
    pub const MSG__SHOW_INPUT_PANEL__SINCE: u32 = 1;

    /// show input panels
    ///
    /// Requests input panels (virtual keyboard) to show.
    #[inline]
    pub fn send_show_input_panel(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the hide_input_panel message is available.
    #[allow(dead_code)]
    pub const MSG__HIDE_INPUT_PANEL__SINCE: u32 = 1;

    /// hide input panels
    ///
    /// Requests input panels (virtual keyboard) to hide.
    #[inline]
    pub fn send_hide_input_panel(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the reset message is available.
    #[allow(dead_code)]
    pub const MSG__RESET__SINCE: u32 = 1;

    /// reset
    ///
    /// Should be called by an editor widget when the input state should be
    /// reset, for example after the text was changed outside of the normal
    /// input method flow.
    #[inline]
    pub fn send_reset(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the set_surrounding_text message is available.
    #[allow(dead_code)]
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
    pub fn send_set_surrounding_text(
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
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the set_content_type message is available.
    #[allow(dead_code)]
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
    pub fn send_set_content_type(
        &self,
        hint: MetaZwpTextInputV1ContentHint,
        purpose: MetaZwpTextInputV1ContentPurpose,
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
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the set_cursor_rectangle message is available.
    #[allow(dead_code)]
    pub const MSG__SET_CURSOR_RECTANGLE__SINCE: u32 = 1;

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
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the set_preferred_language message is available.
    #[allow(dead_code)]
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
    pub fn send_set_preferred_language(
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
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the commit_state message is available.
    #[allow(dead_code)]
    pub const MSG__COMMIT_STATE__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `serial`: used to identify the known state
    #[inline]
    pub fn send_commit_state(
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
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the invoke_action message is available.
    #[allow(dead_code)]
    pub const MSG__INVOKE_ACTION__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `button`:
    /// - `index`:
    #[inline]
    pub fn send_invoke_action(
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
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the enter message is available.
    #[allow(dead_code)]
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
    pub fn send_enter(
        &self,
        surface: &Rc<MetaWlSurface>,
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
            return Err(ObjectError);
        };
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError);
        }
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
            arg0.client_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }

    /// Since when the leave message is available.
    #[allow(dead_code)]
    pub const MSG__LEAVE__SINCE: u32 = 1;

    /// leave event
    ///
    /// Notify the text_input object when it lost focus. Either in response
    /// to a deactivate request or when the assigned surface lost focus or was
    /// destroyed.
    #[inline]
    pub fn send_leave(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            1,
        ]);
        Ok(())
    }

    /// Since when the modifiers_map message is available.
    #[allow(dead_code)]
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
    pub fn send_modifiers_map(
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
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            2,
        ]);
        fmt.array(arg0);
        Ok(())
    }

    /// Since when the input_panel_state message is available.
    #[allow(dead_code)]
    pub const MSG__INPUT_PANEL_STATE__SINCE: u32 = 1;

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
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            state,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            3,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the preedit_string message is available.
    #[allow(dead_code)]
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
    pub fn send_preedit_string(
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
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            4,
            arg0,
        ]);
        fmt.string(arg1);
        fmt.string(arg2);
        Ok(())
    }

    /// Since when the preedit_styling message is available.
    #[allow(dead_code)]
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
    pub fn send_preedit_styling(
        &self,
        index: u32,
        length: u32,
        style: MetaZwpTextInputV1PreeditStyle,
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
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            5,
            arg0,
            arg1,
            arg2.0,
        ]);
        Ok(())
    }

    /// Since when the preedit_cursor message is available.
    #[allow(dead_code)]
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
    pub fn send_preedit_cursor(
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
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            6,
            arg0 as u32,
        ]);
        Ok(())
    }

    /// Since when the commit_string message is available.
    #[allow(dead_code)]
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
    pub fn send_commit_string(
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
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            7,
            arg0,
        ]);
        fmt.string(arg1);
        Ok(())
    }

    /// Since when the cursor_position message is available.
    #[allow(dead_code)]
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
    pub fn send_cursor_position(
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
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            8,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// Since when the delete_surrounding_text message is available.
    #[allow(dead_code)]
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
    pub fn send_delete_surrounding_text(
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
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            9,
            arg0 as u32,
            arg1,
        ]);
        Ok(())
    }

    /// Since when the keysym message is available.
    #[allow(dead_code)]
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
    pub fn send_keysym(
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
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            10,
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ]);
        Ok(())
    }

    /// Since when the language message is available.
    #[allow(dead_code)]
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
    pub fn send_language(
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
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            11,
            arg0,
        ]);
        fmt.string(arg1);
        Ok(())
    }

    /// Since when the text_direction message is available.
    #[allow(dead_code)]
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
    pub fn send_text_direction(
        &self,
        serial: u32,
        direction: MetaZwpTextInputV1TextDirection,
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
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            12,
            arg0,
            arg1.0,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpTextInputV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpTextInputV1MessageHandler {
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
    fn activate(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
        seat: &Rc<MetaWlSeat>,
        surface: &Rc<MetaWlSurface>,
    ) {
        let res = _slf.send_activate(
            seat,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.activate message: {}", Report::new(e));
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
    fn deactivate(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
        seat: &Rc<MetaWlSeat>,
    ) {
        let res = _slf.send_deactivate(
            seat,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.deactivate message: {}", Report::new(e));
        }
    }

    /// show input panels
    ///
    /// Requests input panels (virtual keyboard) to show.
    #[inline]
    fn show_input_panel(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
    ) {
        let res = _slf.send_show_input_panel(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.show_input_panel message: {}", Report::new(e));
        }
    }

    /// hide input panels
    ///
    /// Requests input panels (virtual keyboard) to hide.
    #[inline]
    fn hide_input_panel(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
    ) {
        let res = _slf.send_hide_input_panel(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.hide_input_panel message: {}", Report::new(e));
        }
    }

    /// reset
    ///
    /// Should be called by an editor widget when the input state should be
    /// reset, for example after the text was changed outside of the normal
    /// input method flow.
    #[inline]
    fn reset(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
    ) {
        let res = _slf.send_reset(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.reset message: {}", Report::new(e));
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
    fn set_surrounding_text(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
        text: &str,
        cursor: u32,
        anchor: u32,
    ) {
        let res = _slf.send_set_surrounding_text(
            text,
            cursor,
            anchor,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.set_surrounding_text message: {}", Report::new(e));
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
    fn set_content_type(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
        hint: MetaZwpTextInputV1ContentHint,
        purpose: MetaZwpTextInputV1ContentPurpose,
    ) {
        let res = _slf.send_set_content_type(
            hint,
            purpose,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.set_content_type message: {}", Report::new(e));
        }
    }

    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    fn set_cursor_rectangle(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = _slf.send_set_cursor_rectangle(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.set_cursor_rectangle message: {}", Report::new(e));
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
    fn set_preferred_language(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
        language: &str,
    ) {
        let res = _slf.send_set_preferred_language(
            language,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.set_preferred_language message: {}", Report::new(e));
        }
    }

    /// # Arguments
    ///
    /// - `serial`: used to identify the known state
    #[inline]
    fn commit_state(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
        serial: u32,
    ) {
        let res = _slf.send_commit_state(
            serial,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.commit_state message: {}", Report::new(e));
        }
    }

    /// # Arguments
    ///
    /// - `button`:
    /// - `index`:
    #[inline]
    fn invoke_action(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
        button: u32,
        index: u32,
    ) {
        let res = _slf.send_invoke_action(
            button,
            index,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.invoke_action message: {}", Report::new(e));
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
    fn enter(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
        surface: &Rc<MetaWlSurface>,
    ) {
        if let Some(client_id) = _slf.core.client_id.get() {
            if let Some(client_id_2) = surface.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = _slf.send_enter(
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.enter message: {}", Report::new(e));
        }
    }

    /// leave event
    ///
    /// Notify the text_input object when it lost focus. Either in response
    /// to a deactivate request or when the assigned surface lost focus or was
    /// destroyed.
    #[inline]
    fn leave(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
    ) {
        let res = _slf.send_leave(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.leave message: {}", Report::new(e));
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
    fn modifiers_map(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
        map: &[u8],
    ) {
        let res = _slf.send_modifiers_map(
            map,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.modifiers_map message: {}", Report::new(e));
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
    fn input_panel_state(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
        state: u32,
    ) {
        let res = _slf.send_input_panel_state(
            state,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.input_panel_state message: {}", Report::new(e));
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
    fn preedit_string(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
        serial: u32,
        text: &str,
        commit: &str,
    ) {
        let res = _slf.send_preedit_string(
            serial,
            text,
            commit,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.preedit_string message: {}", Report::new(e));
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
    fn preedit_styling(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
        index: u32,
        length: u32,
        style: MetaZwpTextInputV1PreeditStyle,
    ) {
        let res = _slf.send_preedit_styling(
            index,
            length,
            style,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.preedit_styling message: {}", Report::new(e));
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
    fn preedit_cursor(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
        index: i32,
    ) {
        let res = _slf.send_preedit_cursor(
            index,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.preedit_cursor message: {}", Report::new(e));
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
    fn commit_string(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
        serial: u32,
        text: &str,
    ) {
        let res = _slf.send_commit_string(
            serial,
            text,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.commit_string message: {}", Report::new(e));
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
    fn cursor_position(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
        index: i32,
        anchor: i32,
    ) {
        let res = _slf.send_cursor_position(
            index,
            anchor,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.cursor_position message: {}", Report::new(e));
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
    fn delete_surrounding_text(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
        index: i32,
        length: u32,
    ) {
        let res = _slf.send_delete_surrounding_text(
            index,
            length,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.delete_surrounding_text message: {}", Report::new(e));
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
    fn keysym(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
        serial: u32,
        time: u32,
        sym: u32,
        state: u32,
        modifiers: u32,
    ) {
        let res = _slf.send_keysym(
            serial,
            time,
            sym,
            state,
            modifiers,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.keysym message: {}", Report::new(e));
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
    fn language(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
        serial: u32,
        language: &str,
    ) {
        let res = _slf.send_language(
            serial,
            language,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.language message: {}", Report::new(e));
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
    fn text_direction(
        &mut self,
        _slf: &Rc<MetaZwpTextInputV1>,
        serial: u32,
        direction: MetaZwpTextInputV1TextDirection,
    ) {
        let res = _slf.send_text_direction(
            serial,
            direction,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_text_input_v1.text_direction message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpTextInputV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let Some(arg0) = client.endpoint.lookup(arg0) else {
                    return Err(ObjectError);
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlSeat>() else {
                    return Err(ObjectError);
                };
                let Some(arg1) = client.endpoint.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).activate(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.activate(&self, arg0, arg1);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let Some(arg0) = client.endpoint.lookup(arg0) else {
                    return Err(ObjectError);
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlSeat>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).deactivate(&self, arg0);
                } else {
                    DefaultMessageHandler.deactivate(&self, arg0);
                }
            }
            2 => {
                if let Some(handler) = handler {
                    (**handler).show_input_panel(&self);
                } else {
                    DefaultMessageHandler.show_input_panel(&self);
                }
            }
            3 => {
                if let Some(handler) = handler {
                    (**handler).hide_input_panel(&self);
                } else {
                    DefaultMessageHandler.hide_input_panel(&self);
                }
            }
            4 => {
                if let Some(handler) = handler {
                    (**handler).reset(&self);
                } else {
                    DefaultMessageHandler.reset(&self);
                }
            }
            5 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError);
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError);
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError);
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError);
                        };
                        s
                    }
                };
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError);
                };
                offset += 1;
                let Some(&arg2) = msg.get(offset) else {
                    return Err(ObjectError);
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError);
                }
                if let Some(handler) = handler {
                    (**handler).set_surrounding_text(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.set_surrounding_text(&self, arg0, arg1, arg2);
                }
            }
            6 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = MetaZwpTextInputV1ContentHint(arg0);
                let arg1 = MetaZwpTextInputV1ContentPurpose(arg1);
                if let Some(handler) = handler {
                    (**handler).set_content_type(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.set_content_type(&self, arg0, arg1);
                }
            }
            7 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                if let Some(handler) = handler {
                    (**handler).set_cursor_rectangle(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultMessageHandler.set_cursor_rectangle(&self, arg0, arg1, arg2, arg3);
                }
            }
            8 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError);
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError);
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError);
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError);
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError);
                }
                if let Some(handler) = handler {
                    (**handler).set_preferred_language(&self, arg0);
                } else {
                    DefaultMessageHandler.set_preferred_language(&self, arg0);
                }
            }
            9 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).commit_state(&self, arg0);
                } else {
                    DefaultMessageHandler.commit_state(&self, arg0);
                }
            }
            10 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).invoke_action(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.invoke_action(&self, arg0, arg1);
                }
            }
            _ => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let Some(arg0) = self.core.state.server.lookup(arg0) else {
                    return Err(ObjectError);
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).enter(&self, arg0);
                } else {
                    DefaultMessageHandler.enter(&self, arg0);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).leave(&self);
                } else {
                    DefaultMessageHandler.leave(&self);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError);
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError);
                    }
                    let start = offset;
                    offset += words;
                    &uapi::as_bytes(&msg[start..])[..len]
                };
                if offset != msg.len() {
                    return Err(ObjectError);
                }
                if let Some(handler) = handler {
                    (**handler).modifiers_map(&self, arg0);
                } else {
                    DefaultMessageHandler.modifiers_map(&self, arg0);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).input_panel_state(&self, arg0);
                } else {
                    DefaultMessageHandler.input_panel_state(&self, arg0);
                }
            }
            4 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError);
                };
                offset += 1;
                let arg1 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError);
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError);
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError);
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError);
                        };
                        s
                    }
                };
                let arg2 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError);
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError);
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError);
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError);
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError);
                }
                if let Some(handler) = handler {
                    (**handler).preedit_string(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.preedit_string(&self, arg0, arg1, arg2);
                }
            }
            5 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg2 = MetaZwpTextInputV1PreeditStyle(arg2);
                if let Some(handler) = handler {
                    (**handler).preedit_styling(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.preedit_styling(&self, arg0, arg1, arg2);
                }
            }
            6 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = arg0 as i32;
                if let Some(handler) = handler {
                    (**handler).preedit_cursor(&self, arg0);
                } else {
                    DefaultMessageHandler.preedit_cursor(&self, arg0);
                }
            }
            7 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError);
                };
                offset += 1;
                let arg1 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError);
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError);
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError);
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError);
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError);
                }
                if let Some(handler) = handler {
                    (**handler).commit_string(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.commit_string(&self, arg0, arg1);
                }
            }
            8 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                if let Some(handler) = handler {
                    (**handler).cursor_position(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.cursor_position(&self, arg0, arg1);
                }
            }
            9 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = arg0 as i32;
                if let Some(handler) = handler {
                    (**handler).delete_surrounding_text(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.delete_surrounding_text(&self, arg0, arg1);
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
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).keysym(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultMessageHandler.keysym(&self, arg0, arg1, arg2, arg3, arg4);
                }
            }
            11 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError);
                };
                offset += 1;
                let arg1 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError);
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError);
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError);
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError);
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError);
                }
                if let Some(handler) = handler {
                    (**handler).language(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.language(&self, arg0, arg1);
                }
            }
            12 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg1 = MetaZwpTextInputV1TextDirection(arg1);
                if let Some(handler) = handler {
                    (**handler).text_direction(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.text_direction(&self, arg0, arg1);
                }
            }
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }
}

impl MetaZwpTextInputV1 {
    /// Since when the content_hint.none enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_HINT_NONE__SINCE: u32 = 1;
    /// Since when the content_hint.default enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_HINT_DEFAULT__SINCE: u32 = 1;
    /// Since when the content_hint.password enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_HINT_PASSWORD__SINCE: u32 = 1;
    /// Since when the content_hint.auto_completion enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_HINT_AUTO_COMPLETION__SINCE: u32 = 1;
    /// Since when the content_hint.auto_correction enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_HINT_AUTO_CORRECTION__SINCE: u32 = 1;
    /// Since when the content_hint.auto_capitalization enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_HINT_AUTO_CAPITALIZATION__SINCE: u32 = 1;
    /// Since when the content_hint.lowercase enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_HINT_LOWERCASE__SINCE: u32 = 1;
    /// Since when the content_hint.uppercase enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_HINT_UPPERCASE__SINCE: u32 = 1;
    /// Since when the content_hint.titlecase enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_HINT_TITLECASE__SINCE: u32 = 1;
    /// Since when the content_hint.hidden_text enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_HINT_HIDDEN_TEXT__SINCE: u32 = 1;
    /// Since when the content_hint.sensitive_data enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_HINT_SENSITIVE_DATA__SINCE: u32 = 1;
    /// Since when the content_hint.latin enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_HINT_LATIN__SINCE: u32 = 1;
    /// Since when the content_hint.multiline enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_HINT_MULTILINE__SINCE: u32 = 1;

    /// Since when the content_purpose.normal enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_PURPOSE_NORMAL__SINCE: u32 = 1;
    /// Since when the content_purpose.alpha enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_PURPOSE_ALPHA__SINCE: u32 = 1;
    /// Since when the content_purpose.digits enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_PURPOSE_DIGITS__SINCE: u32 = 1;
    /// Since when the content_purpose.number enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_PURPOSE_NUMBER__SINCE: u32 = 1;
    /// Since when the content_purpose.phone enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_PURPOSE_PHONE__SINCE: u32 = 1;
    /// Since when the content_purpose.url enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_PURPOSE_URL__SINCE: u32 = 1;
    /// Since when the content_purpose.email enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_PURPOSE_EMAIL__SINCE: u32 = 1;
    /// Since when the content_purpose.name enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_PURPOSE_NAME__SINCE: u32 = 1;
    /// Since when the content_purpose.password enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_PURPOSE_PASSWORD__SINCE: u32 = 1;
    /// Since when the content_purpose.date enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_PURPOSE_DATE__SINCE: u32 = 1;
    /// Since when the content_purpose.time enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_PURPOSE_TIME__SINCE: u32 = 1;
    /// Since when the content_purpose.datetime enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_PURPOSE_DATETIME__SINCE: u32 = 1;
    /// Since when the content_purpose.terminal enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONTENT_PURPOSE_TERMINAL__SINCE: u32 = 1;

    /// Since when the preedit_style.default enum variant is available.
    #[allow(dead_code)]
    pub const ENM__PREEDIT_STYLE_DEFAULT__SINCE: u32 = 1;
    /// Since when the preedit_style.none enum variant is available.
    #[allow(dead_code)]
    pub const ENM__PREEDIT_STYLE_NONE__SINCE: u32 = 1;
    /// Since when the preedit_style.active enum variant is available.
    #[allow(dead_code)]
    pub const ENM__PREEDIT_STYLE_ACTIVE__SINCE: u32 = 1;
    /// Since when the preedit_style.inactive enum variant is available.
    #[allow(dead_code)]
    pub const ENM__PREEDIT_STYLE_INACTIVE__SINCE: u32 = 1;
    /// Since when the preedit_style.highlight enum variant is available.
    #[allow(dead_code)]
    pub const ENM__PREEDIT_STYLE_HIGHLIGHT__SINCE: u32 = 1;
    /// Since when the preedit_style.underline enum variant is available.
    #[allow(dead_code)]
    pub const ENM__PREEDIT_STYLE_UNDERLINE__SINCE: u32 = 1;
    /// Since when the preedit_style.selection enum variant is available.
    #[allow(dead_code)]
    pub const ENM__PREEDIT_STYLE_SELECTION__SINCE: u32 = 1;
    /// Since when the preedit_style.incorrect enum variant is available.
    #[allow(dead_code)]
    pub const ENM__PREEDIT_STYLE_INCORRECT__SINCE: u32 = 1;

    /// Since when the text_direction.auto enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TEXT_DIRECTION_AUTO__SINCE: u32 = 1;
    /// Since when the text_direction.ltr enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TEXT_DIRECTION_LTR__SINCE: u32 = 1;
    /// Since when the text_direction.rtl enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TEXT_DIRECTION_RTL__SINCE: u32 = 1;
}

/// content hint
///
/// Content hint is a bitmask to allow to modify the behavior of the text
/// input.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
#[allow(dead_code)]
pub struct MetaZwpTextInputV1ContentHint(pub u32);

/// An iterator over the set bits in a [MetaZwpTextInputV1ContentHint].
///
/// You can construct this with the `IntoIterator` implementation of `MetaZwpTextInputV1ContentHint`.
#[derive(Clone, Debug)]
pub struct MetaZwpTextInputV1ContentHintIter(pub u32);

impl MetaZwpTextInputV1ContentHint {
    /// no special behaviour
    #[allow(dead_code)]
    pub const NONE: Self = Self(0x0);

    /// auto completion, correction and capitalization
    #[allow(dead_code)]
    pub const DEFAULT: Self = Self(0x7);

    /// hidden and sensitive text
    #[allow(dead_code)]
    pub const PASSWORD: Self = Self(0xc0);

    /// suggest word completions
    #[allow(dead_code)]
    pub const AUTO_COMPLETION: Self = Self(0x1);

    /// suggest word corrections
    #[allow(dead_code)]
    pub const AUTO_CORRECTION: Self = Self(0x2);

    /// switch to uppercase letters at the start of a sentence
    #[allow(dead_code)]
    pub const AUTO_CAPITALIZATION: Self = Self(0x4);

    /// prefer lowercase letters
    #[allow(dead_code)]
    pub const LOWERCASE: Self = Self(0x8);

    /// prefer uppercase letters
    #[allow(dead_code)]
    pub const UPPERCASE: Self = Self(0x10);

    /// prefer casing for titles and headings (can be language dependent)
    #[allow(dead_code)]
    pub const TITLECASE: Self = Self(0x20);

    /// characters should be hidden
    #[allow(dead_code)]
    pub const HIDDEN_TEXT: Self = Self(0x40);

    /// typed text should not be stored
    #[allow(dead_code)]
    pub const SENSITIVE_DATA: Self = Self(0x80);

    /// just latin characters should be entered
    #[allow(dead_code)]
    pub const LATIN: Self = Self(0x100);

    /// the text input is multiline
    #[allow(dead_code)]
    pub const MULTILINE: Self = Self(0x200);
}

#[allow(dead_code)]
impl MetaZwpTextInputV1ContentHint {
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

impl Iterator for MetaZwpTextInputV1ContentHintIter {
    type Item = MetaZwpTextInputV1ContentHint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(MetaZwpTextInputV1ContentHint(bit))
    }
}

impl IntoIterator for MetaZwpTextInputV1ContentHint {
    type Item = MetaZwpTextInputV1ContentHint;
    type IntoIter = MetaZwpTextInputV1ContentHintIter;

    fn into_iter(self) -> Self::IntoIter {
        MetaZwpTextInputV1ContentHintIter(self.0)
    }
}

impl BitAnd for MetaZwpTextInputV1ContentHint {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for MetaZwpTextInputV1ContentHint {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for MetaZwpTextInputV1ContentHint {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for MetaZwpTextInputV1ContentHint {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for MetaZwpTextInputV1ContentHint {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for MetaZwpTextInputV1ContentHint {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for MetaZwpTextInputV1ContentHint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for MetaZwpTextInputV1ContentHint {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for MetaZwpTextInputV1ContentHint {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for MetaZwpTextInputV1ContentHint {
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
#[allow(dead_code)]
pub struct MetaZwpTextInputV1ContentPurpose(pub u32);

impl MetaZwpTextInputV1ContentPurpose {
    /// default input, allowing all characters
    #[allow(dead_code)]
    pub const NORMAL: Self = Self(0);

    /// allow only alphabetic characters
    #[allow(dead_code)]
    pub const ALPHA: Self = Self(1);

    /// allow only digits
    #[allow(dead_code)]
    pub const DIGITS: Self = Self(2);

    /// input a number (including decimal separator and sign)
    #[allow(dead_code)]
    pub const NUMBER: Self = Self(3);

    /// input a phone number
    #[allow(dead_code)]
    pub const PHONE: Self = Self(4);

    /// input an URL
    #[allow(dead_code)]
    pub const URL: Self = Self(5);

    /// input an email address
    #[allow(dead_code)]
    pub const EMAIL: Self = Self(6);

    /// input a name of a person
    #[allow(dead_code)]
    pub const NAME: Self = Self(7);

    /// input a password (combine with password or sensitive_data hint)
    #[allow(dead_code)]
    pub const PASSWORD: Self = Self(8);

    /// input a date
    #[allow(dead_code)]
    pub const DATE: Self = Self(9);

    /// input a time
    #[allow(dead_code)]
    pub const TIME: Self = Self(10);

    /// input a date and time
    #[allow(dead_code)]
    pub const DATETIME: Self = Self(11);

    /// input for a terminal
    #[allow(dead_code)]
    pub const TERMINAL: Self = Self(12);
}

impl Debug for MetaZwpTextInputV1ContentPurpose {
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
#[allow(dead_code)]
pub struct MetaZwpTextInputV1PreeditStyle(pub u32);

impl MetaZwpTextInputV1PreeditStyle {
    /// default style for composing text
    #[allow(dead_code)]
    pub const DEFAULT: Self = Self(0);

    /// style should be the same as in non-composing text
    #[allow(dead_code)]
    pub const NONE: Self = Self(1);

    #[allow(dead_code)]
    pub const ACTIVE: Self = Self(2);

    #[allow(dead_code)]
    pub const INACTIVE: Self = Self(3);

    #[allow(dead_code)]
    pub const HIGHLIGHT: Self = Self(4);

    #[allow(dead_code)]
    pub const UNDERLINE: Self = Self(5);

    #[allow(dead_code)]
    pub const SELECTION: Self = Self(6);

    #[allow(dead_code)]
    pub const INCORRECT: Self = Self(7);
}

impl Debug for MetaZwpTextInputV1PreeditStyle {
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
#[allow(dead_code)]
pub struct MetaZwpTextInputV1TextDirection(pub u32);

impl MetaZwpTextInputV1TextDirection {
    /// automatic text direction based on text and language
    #[allow(dead_code)]
    pub const AUTO: Self = Self(0);

    /// left-to-right
    #[allow(dead_code)]
    pub const LTR: Self = Self(1);

    /// right-to-left
    #[allow(dead_code)]
    pub const RTL: Self = Self(2);
}

impl Debug for MetaZwpTextInputV1TextDirection {
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
