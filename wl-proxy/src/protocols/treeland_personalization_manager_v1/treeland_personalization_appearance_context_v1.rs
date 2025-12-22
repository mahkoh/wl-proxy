//! treeland window global appearance settings
//!
//! This interface allows set treeland window global appearance settings.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A treeland_personalization_appearance_context_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandPersonalizationAppearanceContextV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn TreelandPersonalizationAppearanceContextV1Handler>,
}

struct DefaultHandler;

impl TreelandPersonalizationAppearanceContextV1Handler for DefaultHandler { }

impl ConcreteObject for TreelandPersonalizationAppearanceContextV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::TreelandPersonalizationAppearanceContextV1;
    const INTERFACE_NAME: &str = "treeland_personalization_appearance_context_v1";
}

impl TreelandPersonalizationAppearanceContextV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl TreelandPersonalizationAppearanceContextV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandPersonalizationAppearanceContextV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandPersonalizationAppearanceContextV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandPersonalizationAppearanceContextV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandPersonalizationAppearanceContextV1 {
    /// Since when the round_corner_radius message is available.
    pub const MSG__ROUND_CORNER_RADIUS__SINCE: u32 = 1;

    /// round corner radius event
    ///
    /// Send this signal after setting the round corner radius.
    ///
    /// # Arguments
    ///
    /// - `radius`: round corner radius
    #[inline]
    pub fn try_send_round_corner_radius(
        &self,
        radius: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            radius,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_personalization_appearance_context_v1#{}.round_corner_radius(radius: {})\n", client_id, id, arg0);
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
            0,
            arg0 as u32,
        ]);
        Ok(())
    }

    /// round corner radius event
    ///
    /// Send this signal after setting the round corner radius.
    ///
    /// # Arguments
    ///
    /// - `radius`: round corner radius
    #[inline]
    pub fn send_round_corner_radius(
        &self,
        radius: i32,
    ) {
        let res = self.try_send_round_corner_radius(
            radius,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_appearance_context_v1.round_corner_radius", &e);
        }
    }

    /// Since when the icon_theme message is available.
    pub const MSG__ICON_THEME__SINCE: u32 = 1;

    /// icon theme event
    ///
    /// Send this signal after setting the system icon theme.
    ///
    /// # Arguments
    ///
    /// - `theme_name`: icon theme name
    #[inline]
    pub fn try_send_icon_theme(
        &self,
        theme_name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            theme_name,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_personalization_appearance_context_v1#{}.icon_theme(theme_name: {:?})\n", client_id, id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// icon theme event
    ///
    /// Send this signal after setting the system icon theme.
    ///
    /// # Arguments
    ///
    /// - `theme_name`: icon theme name
    #[inline]
    pub fn send_icon_theme(
        &self,
        theme_name: &str,
    ) {
        let res = self.try_send_icon_theme(
            theme_name,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_appearance_context_v1.icon_theme", &e);
        }
    }

    /// Since when the active_color message is available.
    pub const MSG__ACTIVE_COLOR__SINCE: u32 = 1;

    /// active color
    ///
    /// Send this signal after setting the system active color
    ///
    /// # Arguments
    ///
    /// - `active_color`: active color
    #[inline]
    pub fn try_send_active_color(
        &self,
        active_color: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            active_color,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_personalization_appearance_context_v1#{}.active_color(active_color: {:?})\n", client_id, id, arg0);
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
        fmt.string(arg0);
        Ok(())
    }

    /// active color
    ///
    /// Send this signal after setting the system active color
    ///
    /// # Arguments
    ///
    /// - `active_color`: active color
    #[inline]
    pub fn send_active_color(
        &self,
        active_color: &str,
    ) {
        let res = self.try_send_active_color(
            active_color,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_appearance_context_v1.active_color", &e);
        }
    }

    /// Since when the window_opacity message is available.
    pub const MSG__WINDOW_OPACITY__SINCE: u32 = 1;

    /// window opacity
    ///
    /// Send this signal after setting the system active color
    ///
    /// # Arguments
    ///
    /// - `opacity`: window opacity
    #[inline]
    pub fn try_send_window_opacity(
        &self,
        opacity: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            opacity,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_personalization_appearance_context_v1#{}.window_opacity(opacity: {})\n", client_id, id, arg0);
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

    /// window opacity
    ///
    /// Send this signal after setting the system active color
    ///
    /// # Arguments
    ///
    /// - `opacity`: window opacity
    #[inline]
    pub fn send_window_opacity(
        &self,
        opacity: u32,
    ) {
        let res = self.try_send_window_opacity(
            opacity,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_appearance_context_v1.window_opacity", &e);
        }
    }

    /// Since when the window_theme_type message is available.
    pub const MSG__WINDOW_THEME_TYPE__SINCE: u32 = 1;

    /// window theme
    ///
    /// Send this signal after setting the system theme
    ///
    /// # Arguments
    ///
    /// - `r#type`: window theme type
    #[inline]
    pub fn try_send_window_theme_type(
        &self,
        r#type: TreelandPersonalizationAppearanceContextV1ThemeType,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            r#type,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: TreelandPersonalizationAppearanceContextV1ThemeType) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_personalization_appearance_context_v1#{}.window_theme_type(type: {:?})\n", client_id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// window theme
    ///
    /// Send this signal after setting the system theme
    ///
    /// # Arguments
    ///
    /// - `r#type`: window theme type
    #[inline]
    pub fn send_window_theme_type(
        &self,
        r#type: TreelandPersonalizationAppearanceContextV1ThemeType,
    ) {
        let res = self.try_send_window_theme_type(
            r#type,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_appearance_context_v1.window_theme_type", &e);
        }
    }

    /// Since when the window_titlebar_height message is available.
    pub const MSG__WINDOW_TITLEBAR_HEIGHT__SINCE: u32 = 1;

    /// window titlebar height
    ///
    /// Send this signal after setting the window titlebar height
    ///
    /// # Arguments
    ///
    /// - `height`: window titlebar height
    #[inline]
    pub fn try_send_window_titlebar_height(
        &self,
        height: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            height,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_personalization_appearance_context_v1#{}.window_titlebar_height(height: {})\n", client_id, id, arg0);
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

    /// window titlebar height
    ///
    /// Send this signal after setting the window titlebar height
    ///
    /// # Arguments
    ///
    /// - `height`: window titlebar height
    #[inline]
    pub fn send_window_titlebar_height(
        &self,
        height: u32,
    ) {
        let res = self.try_send_window_titlebar_height(
            height,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_appearance_context_v1.window_titlebar_height", &e);
        }
    }

    /// Since when the set_round_corner_radius message is available.
    pub const MSG__SET_ROUND_CORNER_RADIUS__SINCE: u32 = 1;

    /// set window round corner radius
    ///
    /// Set window round corner radius
    ///
    /// # Arguments
    ///
    /// - `radius`:
    #[inline]
    pub fn try_send_set_round_corner_radius(
        &self,
        radius: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            radius,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.set_round_corner_radius(radius: {})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            arg0 as u32,
        ]);
        Ok(())
    }

    /// set window round corner radius
    ///
    /// Set window round corner radius
    ///
    /// # Arguments
    ///
    /// - `radius`:
    #[inline]
    pub fn send_set_round_corner_radius(
        &self,
        radius: i32,
    ) {
        let res = self.try_send_set_round_corner_radius(
            radius,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_appearance_context_v1.set_round_corner_radius", &e);
        }
    }

    /// Since when the get_round_corner_radius message is available.
    pub const MSG__GET_ROUND_CORNER_RADIUS__SINCE: u32 = 1;

    /// get window round corner radius
    ///
    /// Get window round corner radius
    #[inline]
    pub fn try_send_get_round_corner_radius(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.get_round_corner_radius()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
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
            1,
        ]);
        Ok(())
    }

    /// get window round corner radius
    ///
    /// Get window round corner radius
    #[inline]
    pub fn send_get_round_corner_radius(
        &self,
    ) {
        let res = self.try_send_get_round_corner_radius(
        );
        if let Err(e) = res {
            log_send("treeland_personalization_appearance_context_v1.get_round_corner_radius", &e);
        }
    }

    /// Since when the set_icon_theme message is available.
    pub const MSG__SET_ICON_THEME__SINCE: u32 = 1;

    /// set system icon theme
    ///
    /// Set the system icon theme
    ///
    /// # Arguments
    ///
    /// - `theme`: icon theme
    #[inline]
    pub fn try_send_set_icon_theme(
        &self,
        theme: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            theme,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.set_icon_theme(theme: {:?})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            2,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// set system icon theme
    ///
    /// Set the system icon theme
    ///
    /// # Arguments
    ///
    /// - `theme`: icon theme
    #[inline]
    pub fn send_set_icon_theme(
        &self,
        theme: &str,
    ) {
        let res = self.try_send_set_icon_theme(
            theme,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_appearance_context_v1.set_icon_theme", &e);
        }
    }

    /// Since when the get_icon_theme message is available.
    pub const MSG__GET_ICON_THEME__SINCE: u32 = 1;

    /// get system icon theme
    ///
    /// Get the system icon theme
    #[inline]
    pub fn try_send_get_icon_theme(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.get_icon_theme()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
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
            3,
        ]);
        Ok(())
    }

    /// get system icon theme
    ///
    /// Get the system icon theme
    #[inline]
    pub fn send_get_icon_theme(
        &self,
    ) {
        let res = self.try_send_get_icon_theme(
        );
        if let Err(e) = res {
            log_send("treeland_personalization_appearance_context_v1.get_icon_theme", &e);
        }
    }

    /// Since when the set_active_color message is available.
    pub const MSG__SET_ACTIVE_COLOR__SINCE: u32 = 1;

    /// set system active color
    ///
    /// Set the system active color
    ///
    /// # Arguments
    ///
    /// - `color`: rgb
    #[inline]
    pub fn try_send_set_active_color(
        &self,
        color: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            color,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.set_active_color(color: {:?})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            4,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// set system active color
    ///
    /// Set the system active color
    ///
    /// # Arguments
    ///
    /// - `color`: rgb
    #[inline]
    pub fn send_set_active_color(
        &self,
        color: &str,
    ) {
        let res = self.try_send_set_active_color(
            color,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_appearance_context_v1.set_active_color", &e);
        }
    }

    /// Since when the get_active_color message is available.
    pub const MSG__GET_ACTIVE_COLOR__SINCE: u32 = 1;

    /// get system active color
    ///
    /// Get the system active color
    #[inline]
    pub fn try_send_get_active_color(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.get_active_color()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
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
            5,
        ]);
        Ok(())
    }

    /// get system active color
    ///
    /// Get the system active color
    #[inline]
    pub fn send_get_active_color(
        &self,
    ) {
        let res = self.try_send_get_active_color(
        );
        if let Err(e) = res {
            log_send("treeland_personalization_appearance_context_v1.get_active_color", &e);
        }
    }

    /// Since when the set_window_opacity message is available.
    pub const MSG__SET_WINDOW_OPACITY__SINCE: u32 = 1;

    /// set window window opacity
    ///
    /// Set the window window opacity
    ///
    /// # Arguments
    ///
    /// - `opacity`: opacity
    #[inline]
    pub fn try_send_set_window_opacity(
        &self,
        opacity: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            opacity,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.set_window_opacity(opacity: {})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            6,
            arg0,
        ]);
        Ok(())
    }

    /// set window window opacity
    ///
    /// Set the window window opacity
    ///
    /// # Arguments
    ///
    /// - `opacity`: opacity
    #[inline]
    pub fn send_set_window_opacity(
        &self,
        opacity: u32,
    ) {
        let res = self.try_send_set_window_opacity(
            opacity,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_appearance_context_v1.set_window_opacity", &e);
        }
    }

    /// Since when the get_window_opacity message is available.
    pub const MSG__GET_WINDOW_OPACITY__SINCE: u32 = 1;

    /// get window window opacity
    ///
    /// Get the window window opacity
    #[inline]
    pub fn try_send_get_window_opacity(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.get_window_opacity()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
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
            7,
        ]);
        Ok(())
    }

    /// get window window opacity
    ///
    /// Get the window window opacity
    #[inline]
    pub fn send_get_window_opacity(
        &self,
    ) {
        let res = self.try_send_get_window_opacity(
        );
        if let Err(e) = res {
            log_send("treeland_personalization_appearance_context_v1.get_window_opacity", &e);
        }
    }

    /// Since when the set_window_theme_type message is available.
    pub const MSG__SET_WINDOW_THEME_TYPE__SINCE: u32 = 1;

    /// set window theme type
    ///
    /// Set the window theme.
    ///
    /// # Arguments
    ///
    /// - `r#type`: window theme type
    #[inline]
    pub fn try_send_set_window_theme_type(
        &self,
        r#type: TreelandPersonalizationAppearanceContextV1ThemeType,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            r#type,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: TreelandPersonalizationAppearanceContextV1ThemeType) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.set_window_theme_type(type: {:?})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            8,
            arg0.0,
        ]);
        Ok(())
    }

    /// set window theme type
    ///
    /// Set the window theme.
    ///
    /// # Arguments
    ///
    /// - `r#type`: window theme type
    #[inline]
    pub fn send_set_window_theme_type(
        &self,
        r#type: TreelandPersonalizationAppearanceContextV1ThemeType,
    ) {
        let res = self.try_send_set_window_theme_type(
            r#type,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_appearance_context_v1.set_window_theme_type", &e);
        }
    }

    /// Since when the get_window_theme_type message is available.
    pub const MSG__GET_WINDOW_THEME_TYPE__SINCE: u32 = 1;

    /// get window theme type
    ///
    /// Get the window theme type
    #[inline]
    pub fn try_send_get_window_theme_type(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.get_window_theme_type()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
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
            9,
        ]);
        Ok(())
    }

    /// get window theme type
    ///
    /// Get the window theme type
    #[inline]
    pub fn send_get_window_theme_type(
        &self,
    ) {
        let res = self.try_send_get_window_theme_type(
        );
        if let Err(e) = res {
            log_send("treeland_personalization_appearance_context_v1.get_window_theme_type", &e);
        }
    }

    /// Since when the set_window_titlebar_height message is available.
    pub const MSG__SET_WINDOW_TITLEBAR_HEIGHT__SINCE: u32 = 1;

    /// set window titlebar height
    ///
    /// Set the window titlebar height
    ///
    /// # Arguments
    ///
    /// - `height`: window titlebar height
    #[inline]
    pub fn try_send_set_window_titlebar_height(
        &self,
        height: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            height,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.set_window_titlebar_height(height: {})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            10,
            arg0,
        ]);
        Ok(())
    }

    /// set window titlebar height
    ///
    /// Set the window titlebar height
    ///
    /// # Arguments
    ///
    /// - `height`: window titlebar height
    #[inline]
    pub fn send_set_window_titlebar_height(
        &self,
        height: u32,
    ) {
        let res = self.try_send_set_window_titlebar_height(
            height,
        );
        if let Err(e) = res {
            log_send("treeland_personalization_appearance_context_v1.set_window_titlebar_height", &e);
        }
    }

    /// Since when the get_window_titlebar_height message is available.
    pub const MSG__GET_WINDOW_TITLEBAR_HEIGHT__SINCE: u32 = 1;

    /// get window titlebar height
    ///
    /// Get the window titlebar height
    #[inline]
    pub fn try_send_get_window_titlebar_height(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.get_window_titlebar_height()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
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
            11,
        ]);
        Ok(())
    }

    /// get window titlebar height
    ///
    /// Get the window titlebar height
    #[inline]
    pub fn send_get_window_titlebar_height(
        &self,
    ) {
        let res = self.try_send_get_window_titlebar_height(
        );
        if let Err(e) = res {
            log_send("treeland_personalization_appearance_context_v1.get_window_titlebar_height", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the context object
    ///
    /// Destroy the context object.
    #[inline]
    pub fn try_send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.destroy()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
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
            12,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the context object
    ///
    /// Destroy the context object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("treeland_personalization_appearance_context_v1.destroy", &e);
        }
    }
}

/// A message handler for [`TreelandPersonalizationAppearanceContextV1`] proxies.
pub trait TreelandPersonalizationAppearanceContextV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<TreelandPersonalizationAppearanceContextV1>) {
        slf.core.delete_id();
    }

    /// round corner radius event
    ///
    /// Send this signal after setting the round corner radius.
    ///
    /// # Arguments
    ///
    /// - `radius`: round corner radius
    #[inline]
    fn handle_round_corner_radius(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        radius: i32,
    ) {
        if !_slf.core.forward_to_client.get() {
            return;
        }
        let res = _slf.try_send_round_corner_radius(
            radius,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_appearance_context_v1.round_corner_radius", &e);
        }
    }

    /// icon theme event
    ///
    /// Send this signal after setting the system icon theme.
    ///
    /// # Arguments
    ///
    /// - `theme_name`: icon theme name
    #[inline]
    fn handle_icon_theme(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        theme_name: &str,
    ) {
        if !_slf.core.forward_to_client.get() {
            return;
        }
        let res = _slf.try_send_icon_theme(
            theme_name,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_appearance_context_v1.icon_theme", &e);
        }
    }

    /// active color
    ///
    /// Send this signal after setting the system active color
    ///
    /// # Arguments
    ///
    /// - `active_color`: active color
    #[inline]
    fn handle_active_color(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        active_color: &str,
    ) {
        if !_slf.core.forward_to_client.get() {
            return;
        }
        let res = _slf.try_send_active_color(
            active_color,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_appearance_context_v1.active_color", &e);
        }
    }

    /// window opacity
    ///
    /// Send this signal after setting the system active color
    ///
    /// # Arguments
    ///
    /// - `opacity`: window opacity
    #[inline]
    fn handle_window_opacity(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        opacity: u32,
    ) {
        if !_slf.core.forward_to_client.get() {
            return;
        }
        let res = _slf.try_send_window_opacity(
            opacity,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_appearance_context_v1.window_opacity", &e);
        }
    }

    /// window theme
    ///
    /// Send this signal after setting the system theme
    ///
    /// # Arguments
    ///
    /// - `r#type`: window theme type
    #[inline]
    fn handle_window_theme_type(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        r#type: TreelandPersonalizationAppearanceContextV1ThemeType,
    ) {
        if !_slf.core.forward_to_client.get() {
            return;
        }
        let res = _slf.try_send_window_theme_type(
            r#type,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_appearance_context_v1.window_theme_type", &e);
        }
    }

    /// window titlebar height
    ///
    /// Send this signal after setting the window titlebar height
    ///
    /// # Arguments
    ///
    /// - `height`: window titlebar height
    #[inline]
    fn handle_window_titlebar_height(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        height: u32,
    ) {
        if !_slf.core.forward_to_client.get() {
            return;
        }
        let res = _slf.try_send_window_titlebar_height(
            height,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_appearance_context_v1.window_titlebar_height", &e);
        }
    }

    /// set window round corner radius
    ///
    /// Set window round corner radius
    ///
    /// # Arguments
    ///
    /// - `radius`:
    #[inline]
    fn handle_set_round_corner_radius(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        radius: i32,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_set_round_corner_radius(
            radius,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_appearance_context_v1.set_round_corner_radius", &e);
        }
    }

    /// get window round corner radius
    ///
    /// Get window round corner radius
    #[inline]
    fn handle_get_round_corner_radius(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_get_round_corner_radius(
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_appearance_context_v1.get_round_corner_radius", &e);
        }
    }

    /// set system icon theme
    ///
    /// Set the system icon theme
    ///
    /// # Arguments
    ///
    /// - `theme`: icon theme
    #[inline]
    fn handle_set_icon_theme(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        theme: &str,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_set_icon_theme(
            theme,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_appearance_context_v1.set_icon_theme", &e);
        }
    }

    /// get system icon theme
    ///
    /// Get the system icon theme
    #[inline]
    fn handle_get_icon_theme(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_get_icon_theme(
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_appearance_context_v1.get_icon_theme", &e);
        }
    }

    /// set system active color
    ///
    /// Set the system active color
    ///
    /// # Arguments
    ///
    /// - `color`: rgb
    #[inline]
    fn handle_set_active_color(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        color: &str,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_set_active_color(
            color,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_appearance_context_v1.set_active_color", &e);
        }
    }

    /// get system active color
    ///
    /// Get the system active color
    #[inline]
    fn handle_get_active_color(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_get_active_color(
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_appearance_context_v1.get_active_color", &e);
        }
    }

    /// set window window opacity
    ///
    /// Set the window window opacity
    ///
    /// # Arguments
    ///
    /// - `opacity`: opacity
    #[inline]
    fn handle_set_window_opacity(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        opacity: u32,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_set_window_opacity(
            opacity,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_appearance_context_v1.set_window_opacity", &e);
        }
    }

    /// get window window opacity
    ///
    /// Get the window window opacity
    #[inline]
    fn handle_get_window_opacity(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_get_window_opacity(
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_appearance_context_v1.get_window_opacity", &e);
        }
    }

    /// set window theme type
    ///
    /// Set the window theme.
    ///
    /// # Arguments
    ///
    /// - `r#type`: window theme type
    #[inline]
    fn handle_set_window_theme_type(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        r#type: TreelandPersonalizationAppearanceContextV1ThemeType,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_set_window_theme_type(
            r#type,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_appearance_context_v1.set_window_theme_type", &e);
        }
    }

    /// get window theme type
    ///
    /// Get the window theme type
    #[inline]
    fn handle_get_window_theme_type(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_get_window_theme_type(
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_appearance_context_v1.get_window_theme_type", &e);
        }
    }

    /// set window titlebar height
    ///
    /// Set the window titlebar height
    ///
    /// # Arguments
    ///
    /// - `height`: window titlebar height
    #[inline]
    fn handle_set_window_titlebar_height(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        height: u32,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_set_window_titlebar_height(
            height,
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_appearance_context_v1.set_window_titlebar_height", &e);
        }
    }

    /// get window titlebar height
    ///
    /// Get the window titlebar height
    #[inline]
    fn handle_get_window_titlebar_height(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_get_window_titlebar_height(
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_appearance_context_v1.get_window_titlebar_height", &e);
        }
    }

    /// destroy the context object
    ///
    /// Destroy the context object.
    #[inline]
    fn handle_destroy(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("treeland_personalization_appearance_context_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for TreelandPersonalizationAppearanceContextV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::TreelandPersonalizationAppearanceContextV1, version),
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = arg0 as i32;
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.set_round_corner_radius(radius: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_round_corner_radius(&self, arg0);
                } else {
                    DefaultHandler.handle_set_round_corner_radius(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.get_round_corner_radius()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_get_round_corner_radius(&self);
                } else {
                    DefaultHandler.handle_get_round_corner_radius(&self);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError(ObjectErrorKind::MissingArgument("theme")));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError(ObjectErrorKind::MissingArgument("theme")));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError(ObjectErrorKind::NullString("theme")));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError(ObjectErrorKind::NonUtf8("theme")));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.set_icon_theme(theme: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_icon_theme(&self, arg0);
                } else {
                    DefaultHandler.handle_set_icon_theme(&self, arg0);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.get_icon_theme()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_get_icon_theme(&self);
                } else {
                    DefaultHandler.handle_get_icon_theme(&self);
                }
            }
            4 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError(ObjectErrorKind::MissingArgument("color")));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError(ObjectErrorKind::MissingArgument("color")));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError(ObjectErrorKind::NullString("color")));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError(ObjectErrorKind::NonUtf8("color")));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.set_active_color(color: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_active_color(&self, arg0);
                } else {
                    DefaultHandler.handle_set_active_color(&self, arg0);
                }
            }
            5 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.get_active_color()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_get_active_color(&self);
                } else {
                    DefaultHandler.handle_get_active_color(&self);
                }
            }
            6 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.set_window_opacity(opacity: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_window_opacity(&self, arg0);
                } else {
                    DefaultHandler.handle_set_window_opacity(&self, arg0);
                }
            }
            7 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.get_window_opacity()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_get_window_opacity(&self);
                } else {
                    DefaultHandler.handle_get_window_opacity(&self);
                }
            }
            8 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = TreelandPersonalizationAppearanceContextV1ThemeType(arg0);
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: TreelandPersonalizationAppearanceContextV1ThemeType) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.set_window_theme_type(type: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_window_theme_type(&self, arg0);
                } else {
                    DefaultHandler.handle_set_window_theme_type(&self, arg0);
                }
            }
            9 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.get_window_theme_type()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_get_window_theme_type(&self);
                } else {
                    DefaultHandler.handle_get_window_theme_type(&self);
                }
            }
            10 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.set_window_titlebar_height(height: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_window_titlebar_height(&self, arg0);
                } else {
                    DefaultHandler.handle_set_window_titlebar_height(&self, arg0);
                }
            }
            11 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.get_window_titlebar_height()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_get_window_titlebar_height(&self);
                } else {
                    DefaultHandler.handle_get_window_titlebar_height(&self);
                }
            }
            12 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.destroy()\n", client_id, id);
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

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
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
                let arg0 = arg0 as i32;
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_personalization_appearance_context_v1#{}.round_corner_radius(radius: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_round_corner_radius(&self, arg0);
                } else {
                    DefaultHandler.handle_round_corner_radius(&self, arg0);
                }
            }
            1 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError(ObjectErrorKind::MissingArgument("theme_name")));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError(ObjectErrorKind::MissingArgument("theme_name")));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError(ObjectErrorKind::NullString("theme_name")));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError(ObjectErrorKind::NonUtf8("theme_name")));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_personalization_appearance_context_v1#{}.icon_theme(theme_name: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_icon_theme(&self, arg0);
                } else {
                    DefaultHandler.handle_icon_theme(&self, arg0);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError(ObjectErrorKind::MissingArgument("active_color")));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError(ObjectErrorKind::MissingArgument("active_color")));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError(ObjectErrorKind::NullString("active_color")));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError(ObjectErrorKind::NonUtf8("active_color")));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_personalization_appearance_context_v1#{}.active_color(active_color: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_active_color(&self, arg0);
                } else {
                    DefaultHandler.handle_active_color(&self, arg0);
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
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_personalization_appearance_context_v1#{}.window_opacity(opacity: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_window_opacity(&self, arg0);
                } else {
                    DefaultHandler.handle_window_opacity(&self, arg0);
                }
            }
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = TreelandPersonalizationAppearanceContextV1ThemeType(arg0);
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: TreelandPersonalizationAppearanceContextV1ThemeType) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_personalization_appearance_context_v1#{}.window_theme_type(type: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_window_theme_type(&self, arg0);
                } else {
                    DefaultHandler.handle_window_theme_type(&self, arg0);
                }
            }
            5 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_personalization_appearance_context_v1#{}.window_titlebar_height(height: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_window_titlebar_height(&self, arg0);
                } else {
                    DefaultHandler.handle_window_titlebar_height(&self, arg0);
                }
            }
            n => {
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
            0 => "set_round_corner_radius",
            1 => "get_round_corner_radius",
            2 => "set_icon_theme",
            3 => "get_icon_theme",
            4 => "set_active_color",
            5 => "get_active_color",
            6 => "set_window_opacity",
            7 => "get_window_opacity",
            8 => "set_window_theme_type",
            9 => "get_window_theme_type",
            10 => "set_window_titlebar_height",
            11 => "get_window_titlebar_height",
            12 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "round_corner_radius",
            1 => "icon_theme",
            2 => "active_color",
            3 => "window_opacity",
            4 => "window_theme_type",
            5 => "window_titlebar_height",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for TreelandPersonalizationAppearanceContextV1 {
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

impl TreelandPersonalizationAppearanceContextV1 {
    /// Since when the theme_type.auto enum variant is available.
    pub const ENM__THEME_TYPE_AUTO__SINCE: u32 = 1;
    /// Since when the theme_type.light enum variant is available.
    pub const ENM__THEME_TYPE_LIGHT__SINCE: u32 = 1;
    /// Since when the theme_type.dark enum variant is available.
    pub const ENM__THEME_TYPE_DARK__SINCE: u32 = 1;

    /// Since when the error.invalid_round_corner_radius enum variant is available.
    pub const ENM__ERROR_INVALID_ROUND_CORNER_RADIUS__SINCE: u32 = 1;
    /// Since when the error.invalid_icon_theme enum variant is available.
    pub const ENM__ERROR_INVALID_ICON_THEME__SINCE: u32 = 1;
    /// Since when the error.invalid_active_color enum variant is available.
    pub const ENM__ERROR_INVALID_ACTIVE_COLOR__SINCE: u32 = 1;
    /// Since when the error.invalid_window_opacity enum variant is available.
    pub const ENM__ERROR_INVALID_WINDOW_OPACITY__SINCE: u32 = 1;
    /// Since when the error.invalid_window_theme_type enum variant is available.
    pub const ENM__ERROR_INVALID_WINDOW_THEME_TYPE__SINCE: u32 = 1;
    /// Since when the error.invalid_window_titlebar_height enum variant is available.
    pub const ENM__ERROR_INVALID_WINDOW_TITLEBAR_HEIGHT__SINCE: u32 = 1;
}

/// window theme type
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TreelandPersonalizationAppearanceContextV1ThemeType(pub u32);

impl TreelandPersonalizationAppearanceContextV1ThemeType {
    /// window auto theme
    pub const AUTO: Self = Self(1);

    /// window light theme
    pub const LIGHT: Self = Self(2);

    /// window dark theme
    pub const DARK: Self = Self(4);
}

impl Debug for TreelandPersonalizationAppearanceContextV1ThemeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::AUTO => "AUTO",
            Self::LIGHT => "LIGHT",
            Self::DARK => "DARK",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TreelandPersonalizationAppearanceContextV1Error(pub u32);

impl TreelandPersonalizationAppearanceContextV1Error {
    /// Wrong round corner radius
    pub const INVALID_ROUND_CORNER_RADIUS: Self = Self(0);

    /// Wrong icon theme
    pub const INVALID_ICON_THEME: Self = Self(1);

    /// Wrong active color
    pub const INVALID_ACTIVE_COLOR: Self = Self(2);

    /// Wrong window opacity
    pub const INVALID_WINDOW_OPACITY: Self = Self(4);

    /// Wrong theme type
    pub const INVALID_WINDOW_THEME_TYPE: Self = Self(8);

    /// Wrong window titlebar height
    pub const INVALID_WINDOW_TITLEBAR_HEIGHT: Self = Self(16);
}

impl Debug for TreelandPersonalizationAppearanceContextV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_ROUND_CORNER_RADIUS => "INVALID_ROUND_CORNER_RADIUS",
            Self::INVALID_ICON_THEME => "INVALID_ICON_THEME",
            Self::INVALID_ACTIVE_COLOR => "INVALID_ACTIVE_COLOR",
            Self::INVALID_WINDOW_OPACITY => "INVALID_WINDOW_OPACITY",
            Self::INVALID_WINDOW_THEME_TYPE => "INVALID_WINDOW_THEME_TYPE",
            Self::INVALID_WINDOW_TITLEBAR_HEIGHT => "INVALID_WINDOW_TITLEBAR_HEIGHT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
