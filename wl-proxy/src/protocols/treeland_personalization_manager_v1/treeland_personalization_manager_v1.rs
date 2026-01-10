//! personalization manager
//!
//! This interface allows a client to customized display effects.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A treeland_personalization_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandPersonalizationManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn TreelandPersonalizationManagerV1Handler>,
}

struct DefaultHandler;

impl TreelandPersonalizationManagerV1Handler for DefaultHandler { }

impl ConcreteObject for TreelandPersonalizationManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::TreelandPersonalizationManagerV1;
    const INTERFACE_NAME: &str = "treeland_personalization_manager_v1";
}

impl TreelandPersonalizationManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl TreelandPersonalizationManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandPersonalizationManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandPersonalizationManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandPersonalizationManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandPersonalizationManagerV1 {
    /// Since when the get_window_context message is available.
    pub const MSG__GET_WINDOW_CONTEXT__SINCE: u32 = 1;

    /// get personalization window context
    ///
    /// set window background, shadow based on context
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn try_send_get_window_context(
        &self,
        id: &Rc<TreelandPersonalizationWindowContextV1>,
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
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_manager_v1#{}.get_window_context(id: treeland_personalization_window_context_v1#{}, surface: wl_surface#{})\n", id, arg0, arg1);
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

    /// get personalization window context
    ///
    /// set window background, shadow based on context
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn send_get_window_context(
        &self,
        id: &Rc<TreelandPersonalizationWindowContextV1>,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_get_window_context(
            id,
            surface,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_manager_v1.get_window_context", &e);
        }
    }

    /// get personalization window context
    ///
    /// set window background, shadow based on context
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn new_try_send_get_window_context(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Result<Rc<TreelandPersonalizationWindowContextV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_window_context(
            &id,
            surface,
        )?;
        Ok(id)
    }

    /// get personalization window context
    ///
    /// set window background, shadow based on context
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn new_send_get_window_context(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Rc<TreelandPersonalizationWindowContextV1> {
        let id = self.core.create_child();
        self.send_get_window_context(
            &id,
            surface,
        );
        id
    }

    /// Since when the get_wallpaper_context message is available.
    pub const MSG__GET_WALLPAPER_CONTEXT__SINCE: u32 = 1;

    /// custom wallpaper context
    ///
    /// custom user wallpaper
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_get_wallpaper_context(
        &self,
        id: &Rc<TreelandPersonalizationWallpaperContextV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
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
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_manager_v1#{}.get_wallpaper_context(id: treeland_personalization_wallpaper_context_v1#{})\n", id, arg0);
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

    /// custom wallpaper context
    ///
    /// custom user wallpaper
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_get_wallpaper_context(
        &self,
        id: &Rc<TreelandPersonalizationWallpaperContextV1>,
    ) {
        let res = self.try_send_get_wallpaper_context(
            id,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_manager_v1.get_wallpaper_context", &e);
        }
    }

    /// custom wallpaper context
    ///
    /// custom user wallpaper
    #[inline]
    pub fn new_try_send_get_wallpaper_context(
        &self,
    ) -> Result<Rc<TreelandPersonalizationWallpaperContextV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_wallpaper_context(
            &id,
        )?;
        Ok(id)
    }

    /// custom wallpaper context
    ///
    /// custom user wallpaper
    #[inline]
    pub fn new_send_get_wallpaper_context(
        &self,
    ) -> Rc<TreelandPersonalizationWallpaperContextV1> {
        let id = self.core.create_child();
        self.send_get_wallpaper_context(
            &id,
        );
        id
    }

    /// Since when the get_cursor_context message is available.
    pub const MSG__GET_CURSOR_CONTEXT__SINCE: u32 = 1;

    /// custom wallpaper context
    ///
    /// custom user cursor
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_get_cursor_context(
        &self,
        id: &Rc<TreelandPersonalizationCursorContextV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
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
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_manager_v1#{}.get_cursor_context(id: treeland_personalization_cursor_context_v1#{})\n", id, arg0);
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
            2,
            arg0_id,
        ]);
        Ok(())
    }

    /// custom wallpaper context
    ///
    /// custom user cursor
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_get_cursor_context(
        &self,
        id: &Rc<TreelandPersonalizationCursorContextV1>,
    ) {
        let res = self.try_send_get_cursor_context(
            id,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_manager_v1.get_cursor_context", &e);
        }
    }

    /// custom wallpaper context
    ///
    /// custom user cursor
    #[inline]
    pub fn new_try_send_get_cursor_context(
        &self,
    ) -> Result<Rc<TreelandPersonalizationCursorContextV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_cursor_context(
            &id,
        )?;
        Ok(id)
    }

    /// custom wallpaper context
    ///
    /// custom user cursor
    #[inline]
    pub fn new_send_get_cursor_context(
        &self,
    ) -> Rc<TreelandPersonalizationCursorContextV1> {
        let id = self.core.create_child();
        self.send_get_cursor_context(
            &id,
        );
        id
    }

    /// Since when the get_font_context message is available.
    pub const MSG__GET_FONT_CONTEXT__SINCE: u32 = 1;

    /// custom treeland and window global font context
    ///
    /// custom treeland and window global font context
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_get_font_context(
        &self,
        id: &Rc<TreelandPersonalizationFontContextV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
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
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_manager_v1#{}.get_font_context(id: treeland_personalization_font_context_v1#{})\n", id, arg0);
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
            3,
            arg0_id,
        ]);
        Ok(())
    }

    /// custom treeland and window global font context
    ///
    /// custom treeland and window global font context
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_get_font_context(
        &self,
        id: &Rc<TreelandPersonalizationFontContextV1>,
    ) {
        let res = self.try_send_get_font_context(
            id,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_manager_v1.get_font_context", &e);
        }
    }

    /// custom treeland and window global font context
    ///
    /// custom treeland and window global font context
    #[inline]
    pub fn new_try_send_get_font_context(
        &self,
    ) -> Result<Rc<TreelandPersonalizationFontContextV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_font_context(
            &id,
        )?;
        Ok(id)
    }

    /// custom treeland and window global font context
    ///
    /// custom treeland and window global font context
    #[inline]
    pub fn new_send_get_font_context(
        &self,
    ) -> Rc<TreelandPersonalizationFontContextV1> {
        let id = self.core.create_child();
        self.send_get_font_context(
            &id,
        );
        id
    }

    /// Since when the get_appearance_context message is available.
    pub const MSG__GET_APPEARANCE_CONTEXT__SINCE: u32 = 1;

    /// custom treeland window global settings context
    ///
    /// custom user treeland window appearance global
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_get_appearance_context(
        &self,
        id: &Rc<TreelandPersonalizationAppearanceContextV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
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
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_manager_v1#{}.get_appearance_context(id: treeland_personalization_appearance_context_v1#{})\n", id, arg0);
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
            4,
            arg0_id,
        ]);
        Ok(())
    }

    /// custom treeland window global settings context
    ///
    /// custom user treeland window appearance global
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_get_appearance_context(
        &self,
        id: &Rc<TreelandPersonalizationAppearanceContextV1>,
    ) {
        let res = self.try_send_get_appearance_context(
            id,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_manager_v1.get_appearance_context", &e);
        }
    }

    /// custom treeland window global settings context
    ///
    /// custom user treeland window appearance global
    #[inline]
    pub fn new_try_send_get_appearance_context(
        &self,
    ) -> Result<Rc<TreelandPersonalizationAppearanceContextV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_appearance_context(
            &id,
        )?;
        Ok(id)
    }

    /// custom treeland window global settings context
    ///
    /// custom user treeland window appearance global
    #[inline]
    pub fn new_send_get_appearance_context(
        &self,
    ) -> Rc<TreelandPersonalizationAppearanceContextV1> {
        let id = self.core.create_child();
        self.send_get_appearance_context(
            &id,
        );
        id
    }
}

/// A message handler for [`TreelandPersonalizationManagerV1`] proxies.
pub trait TreelandPersonalizationManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<TreelandPersonalizationManagerV1>) {
        slf.core.delete_id();
    }

    /// get personalization window context
    ///
    /// set window background, shadow based on context
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_window_context(
        &mut self,
        slf: &Rc<TreelandPersonalizationManagerV1>,
        id: &Rc<TreelandPersonalizationWindowContextV1>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_window_context(
            id,
            surface,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_manager_v1.get_window_context", &e);
        }
    }

    /// custom wallpaper context
    ///
    /// custom user wallpaper
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn handle_get_wallpaper_context(
        &mut self,
        slf: &Rc<TreelandPersonalizationManagerV1>,
        id: &Rc<TreelandPersonalizationWallpaperContextV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_wallpaper_context(
            id,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_manager_v1.get_wallpaper_context", &e);
        }
    }

    /// custom wallpaper context
    ///
    /// custom user cursor
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn handle_get_cursor_context(
        &mut self,
        slf: &Rc<TreelandPersonalizationManagerV1>,
        id: &Rc<TreelandPersonalizationCursorContextV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_cursor_context(
            id,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_manager_v1.get_cursor_context", &e);
        }
    }

    /// custom treeland and window global font context
    ///
    /// custom treeland and window global font context
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn handle_get_font_context(
        &mut self,
        slf: &Rc<TreelandPersonalizationManagerV1>,
        id: &Rc<TreelandPersonalizationFontContextV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_font_context(
            id,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_manager_v1.get_font_context", &e);
        }
    }

    /// custom treeland window global settings context
    ///
    /// custom user treeland window appearance global
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn handle_get_appearance_context(
        &mut self,
        slf: &Rc<TreelandPersonalizationManagerV1>,
        id: &Rc<TreelandPersonalizationAppearanceContextV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_appearance_context(
            id,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_manager_v1.get_appearance_context", &e);
        }
    }
}

impl ObjectPrivate for TreelandPersonalizationManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::TreelandPersonalizationManagerV1, version),
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
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_manager_v1#{}.get_window_context(id: treeland_personalization_window_context_v1#{}, surface: wl_surface#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = TreelandPersonalizationWindowContextV1::new(&self.core.state, self.core.version);
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
                    (**handler).handle_get_window_context(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_window_context(&self, arg0, arg1);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_manager_v1#{}.get_wallpaper_context(id: treeland_personalization_wallpaper_context_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = TreelandPersonalizationWallpaperContextV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_get_wallpaper_context(&self, arg0);
                } else {
                    DefaultHandler.handle_get_wallpaper_context(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_manager_v1#{}.get_cursor_context(id: treeland_personalization_cursor_context_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = TreelandPersonalizationCursorContextV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_get_cursor_context(&self, arg0);
                } else {
                    DefaultHandler.handle_get_cursor_context(&self, arg0);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_manager_v1#{}.get_font_context(id: treeland_personalization_font_context_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = TreelandPersonalizationFontContextV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_get_font_context(&self, arg0);
                } else {
                    DefaultHandler.handle_get_font_context(&self, arg0);
                }
            }
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_manager_v1#{}.get_appearance_context(id: treeland_personalization_appearance_context_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = TreelandPersonalizationAppearanceContextV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_get_appearance_context(&self, arg0);
                } else {
                    DefaultHandler.handle_get_appearance_context(&self, arg0);
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
            0 => "get_window_context",
            1 => "get_wallpaper_context",
            2 => "get_cursor_context",
            3 => "get_font_context",
            4 => "get_appearance_context",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for TreelandPersonalizationManagerV1 {
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

