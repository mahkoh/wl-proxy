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

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_input_method_context_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpInputMethodContextV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpInputMethodContextV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpInputMethodContextV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpInputMethodContextV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwpInputMethodContextV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpInputMethodContextV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpInputMethodContextV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
        ]);
        Ok(())
    }

    /// Since when the commit_string message is available.
    #[allow(dead_code)]
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0,
        ]);
        fmt.string(arg1);
        Ok(())
    }

    /// Since when the preedit_string message is available.
    #[allow(dead_code)]
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
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

    /// Since when the preedit_styling message is available.
    #[allow(dead_code)]
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
    pub fn send_preedit_styling(
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
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
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

    /// Since when the preedit_cursor message is available.
    #[allow(dead_code)]
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            4,
            arg0 as u32,
        ]);
        Ok(())
    }

    /// Since when the delete_surrounding_text message is available.
    #[allow(dead_code)]
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            5,
            arg0 as u32,
            arg1,
        ]);
        Ok(())
    }

    /// Since when the cursor_position message is available.
    #[allow(dead_code)]
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            6,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// Since when the modifiers_map message is available.
    #[allow(dead_code)]
    pub const MSG__MODIFIERS_MAP__SINCE: u32 = 1;

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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            7,
        ]);
        fmt.array(arg0);
        Ok(())
    }

    /// Since when the keysym message is available.
    #[allow(dead_code)]
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
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

    /// Since when the grab_keyboard message is available.
    #[allow(dead_code)]
    pub const MSG__GRAB_KEYBOARD__SINCE: u32 = 1;

    /// grab hardware keyboard
    ///
    /// Allow an input method to receive hardware keyboard input and process
    /// key events to generate text events (with pre-edit) over the wire. This
    /// allows input methods which compose multiple key events for inputting
    /// text like it is done for CJK languages.
    #[inline]
    pub fn send_grab_keyboard(
        &self,
        keyboard: &Rc<MetaWlKeyboard>,
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
            return Err(ObjectError);
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            9,
            arg0.server_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }

    /// Since when the key message is available.
    #[allow(dead_code)]
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
    pub fn send_key(
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
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
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

    /// Since when the modifiers message is available.
    #[allow(dead_code)]
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
    pub fn send_modifiers(
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
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
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

    /// Since when the language message is available.
    #[allow(dead_code)]
    pub const MSG__LANGUAGE__SINCE: u32 = 1;

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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            12,
            arg0,
        ]);
        fmt.string(arg1);
        Ok(())
    }

    /// Since when the text_direction message is available.
    #[allow(dead_code)]
    pub const MSG__TEXT_DIRECTION__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `direction`:
    #[inline]
    pub fn send_text_direction(
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
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            13,
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// Since when the surrounding_text message is available.
    #[allow(dead_code)]
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
    pub fn send_surrounding_text(
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
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
        ]);
        fmt.string(arg0);
        fmt.words([
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// Since when the reset message is available.
    #[allow(dead_code)]
    pub const MSG__RESET__SINCE: u32 = 1;

    #[inline]
    pub fn send_reset(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            1,
        ]);
        Ok(())
    }

    /// Since when the content_type message is available.
    #[allow(dead_code)]
    pub const MSG__CONTENT_TYPE__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `hint`:
    /// - `purpose`:
    #[inline]
    pub fn send_content_type(
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
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            2,
            arg0,
            arg1,
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
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            3,
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// Since when the commit_state message is available.
    #[allow(dead_code)]
    pub const MSG__COMMIT_STATE__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `serial`: serial of text input state
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
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            4,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the preferred_language message is available.
    #[allow(dead_code)]
    pub const MSG__PREFERRED_LANGUAGE__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `language`:
    #[inline]
    pub fn send_preferred_language(
        &self,
        language: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            language,
        );
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            5,
        ]);
        fmt.string(arg0);
        Ok(())
    }
}

/// A message handler for [ZwpInputMethodContextV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpInputMethodContextV1MessageHandler {
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpInputMethodContextV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_context_v1.destroy message: {}", Report::new(e));
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
    fn commit_string(
        &mut self,
        _slf: &Rc<MetaZwpInputMethodContextV1>,
        serial: u32,
        text: &str,
    ) {
        let res = _slf.send_commit_string(
            serial,
            text,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_context_v1.commit_string message: {}", Report::new(e));
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
    fn preedit_string(
        &mut self,
        _slf: &Rc<MetaZwpInputMethodContextV1>,
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
            log::warn!("Could not forward a zwp_input_method_context_v1.preedit_string message: {}", Report::new(e));
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
    fn preedit_styling(
        &mut self,
        _slf: &Rc<MetaZwpInputMethodContextV1>,
        index: u32,
        length: u32,
        style: u32,
    ) {
        let res = _slf.send_preedit_styling(
            index,
            length,
            style,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_context_v1.preedit_styling message: {}", Report::new(e));
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
    fn preedit_cursor(
        &mut self,
        _slf: &Rc<MetaZwpInputMethodContextV1>,
        index: i32,
    ) {
        let res = _slf.send_preedit_cursor(
            index,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_context_v1.preedit_cursor message: {}", Report::new(e));
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
    fn delete_surrounding_text(
        &mut self,
        _slf: &Rc<MetaZwpInputMethodContextV1>,
        index: i32,
        length: u32,
    ) {
        let res = _slf.send_delete_surrounding_text(
            index,
            length,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_context_v1.delete_surrounding_text message: {}", Report::new(e));
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
    fn cursor_position(
        &mut self,
        _slf: &Rc<MetaZwpInputMethodContextV1>,
        index: i32,
        anchor: i32,
    ) {
        let res = _slf.send_cursor_position(
            index,
            anchor,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_context_v1.cursor_position message: {}", Report::new(e));
        }
    }

    /// # Arguments
    ///
    /// - `map`:
    #[inline]
    fn modifiers_map(
        &mut self,
        _slf: &Rc<MetaZwpInputMethodContextV1>,
        map: &[u8],
    ) {
        let res = _slf.send_modifiers_map(
            map,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_context_v1.modifiers_map message: {}", Report::new(e));
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
    fn keysym(
        &mut self,
        _slf: &Rc<MetaZwpInputMethodContextV1>,
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
            log::warn!("Could not forward a zwp_input_method_context_v1.keysym message: {}", Report::new(e));
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
    fn grab_keyboard(
        &mut self,
        _slf: &Rc<MetaZwpInputMethodContextV1>,
        keyboard: &Rc<MetaWlKeyboard>,
    ) {
        let res = _slf.send_grab_keyboard(
            keyboard,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_context_v1.grab_keyboard message: {}", Report::new(e));
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
    fn key(
        &mut self,
        _slf: &Rc<MetaZwpInputMethodContextV1>,
        serial: u32,
        time: u32,
        key: u32,
        state: u32,
    ) {
        let res = _slf.send_key(
            serial,
            time,
            key,
            state,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_context_v1.key message: {}", Report::new(e));
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
    fn modifiers(
        &mut self,
        _slf: &Rc<MetaZwpInputMethodContextV1>,
        serial: u32,
        mods_depressed: u32,
        mods_latched: u32,
        mods_locked: u32,
        group: u32,
    ) {
        let res = _slf.send_modifiers(
            serial,
            mods_depressed,
            mods_latched,
            mods_locked,
            group,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_context_v1.modifiers message: {}", Report::new(e));
        }
    }

    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `language`:
    #[inline]
    fn language(
        &mut self,
        _slf: &Rc<MetaZwpInputMethodContextV1>,
        serial: u32,
        language: &str,
    ) {
        let res = _slf.send_language(
            serial,
            language,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_context_v1.language message: {}", Report::new(e));
        }
    }

    /// # Arguments
    ///
    /// - `serial`: serial of the latest known text input state
    /// - `direction`:
    #[inline]
    fn text_direction(
        &mut self,
        _slf: &Rc<MetaZwpInputMethodContextV1>,
        serial: u32,
        direction: u32,
    ) {
        let res = _slf.send_text_direction(
            serial,
            direction,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_context_v1.text_direction message: {}", Report::new(e));
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
    fn surrounding_text(
        &mut self,
        _slf: &Rc<MetaZwpInputMethodContextV1>,
        text: &str,
        cursor: u32,
        anchor: u32,
    ) {
        let res = _slf.send_surrounding_text(
            text,
            cursor,
            anchor,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_context_v1.surrounding_text message: {}", Report::new(e));
        }
    }

    #[inline]
    fn reset(
        &mut self,
        _slf: &Rc<MetaZwpInputMethodContextV1>,
    ) {
        let res = _slf.send_reset(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_context_v1.reset message: {}", Report::new(e));
        }
    }

    /// # Arguments
    ///
    /// - `hint`:
    /// - `purpose`:
    #[inline]
    fn content_type(
        &mut self,
        _slf: &Rc<MetaZwpInputMethodContextV1>,
        hint: u32,
        purpose: u32,
    ) {
        let res = _slf.send_content_type(
            hint,
            purpose,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_context_v1.content_type message: {}", Report::new(e));
        }
    }

    /// # Arguments
    ///
    /// - `button`:
    /// - `index`:
    #[inline]
    fn invoke_action(
        &mut self,
        _slf: &Rc<MetaZwpInputMethodContextV1>,
        button: u32,
        index: u32,
    ) {
        let res = _slf.send_invoke_action(
            button,
            index,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_context_v1.invoke_action message: {}", Report::new(e));
        }
    }

    /// # Arguments
    ///
    /// - `serial`: serial of text input state
    #[inline]
    fn commit_state(
        &mut self,
        _slf: &Rc<MetaZwpInputMethodContextV1>,
        serial: u32,
    ) {
        let res = _slf.send_commit_state(
            serial,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_context_v1.commit_state message: {}", Report::new(e));
        }
    }

    /// # Arguments
    ///
    /// - `language`:
    #[inline]
    fn preferred_language(
        &mut self,
        _slf: &Rc<MetaZwpInputMethodContextV1>,
        language: &str,
    ) {
        let res = _slf.send_preferred_language(
            language,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_method_context_v1.preferred_language message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpInputMethodContextV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
            }
            1 => {
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
            2 => {
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
            3 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).preedit_styling(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.preedit_styling(&self, arg0, arg1, arg2);
                }
            }
            4 => {
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
            5 => {
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
            6 => {
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
            7 => {
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
            8 => {
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
            9 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaWlKeyboard::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).grab_keyboard(&self, arg0);
                } else {
                    DefaultMessageHandler.grab_keyboard(&self, arg0);
                }
            }
            10 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).key(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultMessageHandler.key(&self, arg0, arg1, arg2, arg3);
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
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).modifiers(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultMessageHandler.modifiers(&self, arg0, arg1, arg2, arg3, arg4);
                }
            }
            12 => {
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
            13 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).text_direction(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.text_direction(&self, arg0, arg1);
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
                    (**handler).surrounding_text(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.surrounding_text(&self, arg0, arg1, arg2);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).reset(&self);
                } else {
                    DefaultMessageHandler.reset(&self);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).content_type(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.content_type(&self, arg0, arg1);
                }
            }
            3 => {
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
            4 => {
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
                if offset != msg.len() {
                    return Err(ObjectError);
                }
                if let Some(handler) = handler {
                    (**handler).preferred_language(&self, arg0);
                } else {
                    DefaultMessageHandler.preferred_language(&self, arg0);
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

