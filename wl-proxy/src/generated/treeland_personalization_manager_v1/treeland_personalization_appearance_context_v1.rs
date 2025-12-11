//! treeland window global appearance settings
//!
//! This interface allows set treeland window global appearance settings.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A treeland_personalization_appearance_context_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandPersonalizationAppearanceContextV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn TreelandPersonalizationAppearanceContextV1Handler>,
}

struct DefaultHandler;

impl TreelandPersonalizationAppearanceContextV1Handler for DefaultHandler { }

impl TreelandPersonalizationAppearanceContextV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "treeland_personalization_appearance_context_v1";
}

impl TreelandPersonalizationAppearanceContextV1 {
    pub fn set_handler(&self, handler: impl TreelandPersonalizationAppearanceContextV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

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
    pub fn send_round_corner_radius(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_personalization_appearance_context_v1#{}.round_corner_radius(radius: {})\n", client.endpoint.id, id, arg0);
            self.core.state.log(args);
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
    pub fn send_icon_theme(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_personalization_appearance_context_v1#{}.icon_theme(theme_name: {:?})\n", client.endpoint.id, id, arg0);
            self.core.state.log(args);
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
    pub fn send_active_color(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_personalization_appearance_context_v1#{}.active_color(active_color: {:?})\n", client.endpoint.id, id, arg0);
            self.core.state.log(args);
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
    pub fn send_window_opacity(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_personalization_appearance_context_v1#{}.window_opacity(opacity: {})\n", client.endpoint.id, id, arg0);
            self.core.state.log(args);
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
    pub fn send_window_theme_type(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_personalization_appearance_context_v1#{}.window_theme_type(type: {:?})\n", client.endpoint.id, id, arg0);
            self.core.state.log(args);
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
    pub fn send_window_titlebar_height(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_personalization_appearance_context_v1#{}.window_titlebar_height(height: {})\n", client.endpoint.id, id, arg0);
            self.core.state.log(args);
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
    pub fn send_set_round_corner_radius(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.set_round_corner_radius(radius: {})\n", id, arg0);
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
            arg0 as u32,
        ]);
        Ok(())
    }

    /// Since when the get_round_corner_radius message is available.
    pub const MSG__GET_ROUND_CORNER_RADIUS__SINCE: u32 = 1;

    /// get window round corner radius
    ///
    /// Get window round corner radius
    #[inline]
    pub fn send_get_round_corner_radius(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.get_round_corner_radius()\n", id);
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
            1,
        ]);
        Ok(())
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
    pub fn send_set_icon_theme(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.set_icon_theme(theme: {:?})\n", id, arg0);
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
            2,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the get_icon_theme message is available.
    pub const MSG__GET_ICON_THEME__SINCE: u32 = 1;

    /// get system icon theme
    ///
    /// Get the system icon theme
    #[inline]
    pub fn send_get_icon_theme(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.get_icon_theme()\n", id);
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
            3,
        ]);
        Ok(())
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
    pub fn send_set_active_color(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.set_active_color(color: {:?})\n", id, arg0);
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
            4,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the get_active_color message is available.
    pub const MSG__GET_ACTIVE_COLOR__SINCE: u32 = 1;

    /// get system active color
    ///
    /// Get the system active color
    #[inline]
    pub fn send_get_active_color(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.get_active_color()\n", id);
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
            5,
        ]);
        Ok(())
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
    pub fn send_set_window_opacity(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.set_window_opacity(opacity: {})\n", id, arg0);
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
            6,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the get_window_opacity message is available.
    pub const MSG__GET_WINDOW_OPACITY__SINCE: u32 = 1;

    /// get window window opacity
    ///
    /// Get the window window opacity
    #[inline]
    pub fn send_get_window_opacity(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.get_window_opacity()\n", id);
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
            7,
        ]);
        Ok(())
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
    pub fn send_set_window_theme_type(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.set_window_theme_type(type: {:?})\n", id, arg0);
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
            8,
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the get_window_theme_type message is available.
    pub const MSG__GET_WINDOW_THEME_TYPE__SINCE: u32 = 1;

    /// get window theme type
    ///
    /// Get the window theme type
    #[inline]
    pub fn send_get_window_theme_type(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.get_window_theme_type()\n", id);
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
            9,
        ]);
        Ok(())
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
    pub fn send_set_window_titlebar_height(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.set_window_titlebar_height(height: {})\n", id, arg0);
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
            10,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the get_window_titlebar_height message is available.
    pub const MSG__GET_WINDOW_TITLEBAR_HEIGHT__SINCE: u32 = 1;

    /// get window titlebar height
    ///
    /// Get the window titlebar height
    #[inline]
    pub fn send_get_window_titlebar_height(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.get_window_titlebar_height()\n", id);
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
            11,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the context object
    ///
    /// Destroy the context object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_personalization_appearance_context_v1#{}.destroy()\n", id);
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
            12,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [TreelandPersonalizationAppearanceContextV1] proxies.
pub trait TreelandPersonalizationAppearanceContextV1Handler: Any {
    /// round corner radius event
    ///
    /// Send this signal after setting the round corner radius.
    ///
    /// # Arguments
    ///
    /// - `radius`: round corner radius
    #[inline]
    fn round_corner_radius(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        radius: i32,
    ) {
        let res = _slf.send_round_corner_radius(
            radius,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_appearance_context_v1.round_corner_radius message: {}", Report::new(e));
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
    fn icon_theme(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        theme_name: &str,
    ) {
        let res = _slf.send_icon_theme(
            theme_name,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_appearance_context_v1.icon_theme message: {}", Report::new(e));
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
    fn active_color(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        active_color: &str,
    ) {
        let res = _slf.send_active_color(
            active_color,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_appearance_context_v1.active_color message: {}", Report::new(e));
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
    fn window_opacity(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        opacity: u32,
    ) {
        let res = _slf.send_window_opacity(
            opacity,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_appearance_context_v1.window_opacity message: {}", Report::new(e));
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
    fn window_theme_type(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        r#type: TreelandPersonalizationAppearanceContextV1ThemeType,
    ) {
        let res = _slf.send_window_theme_type(
            r#type,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_appearance_context_v1.window_theme_type message: {}", Report::new(e));
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
    fn window_titlebar_height(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        height: u32,
    ) {
        let res = _slf.send_window_titlebar_height(
            height,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_appearance_context_v1.window_titlebar_height message: {}", Report::new(e));
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
    fn set_round_corner_radius(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        radius: i32,
    ) {
        let res = _slf.send_set_round_corner_radius(
            radius,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_appearance_context_v1.set_round_corner_radius message: {}", Report::new(e));
        }
    }

    /// get window round corner radius
    ///
    /// Get window round corner radius
    #[inline]
    fn get_round_corner_radius(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
    ) {
        let res = _slf.send_get_round_corner_radius(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_appearance_context_v1.get_round_corner_radius message: {}", Report::new(e));
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
    fn set_icon_theme(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        theme: &str,
    ) {
        let res = _slf.send_set_icon_theme(
            theme,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_appearance_context_v1.set_icon_theme message: {}", Report::new(e));
        }
    }

    /// get system icon theme
    ///
    /// Get the system icon theme
    #[inline]
    fn get_icon_theme(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
    ) {
        let res = _slf.send_get_icon_theme(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_appearance_context_v1.get_icon_theme message: {}", Report::new(e));
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
    fn set_active_color(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        color: &str,
    ) {
        let res = _slf.send_set_active_color(
            color,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_appearance_context_v1.set_active_color message: {}", Report::new(e));
        }
    }

    /// get system active color
    ///
    /// Get the system active color
    #[inline]
    fn get_active_color(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
    ) {
        let res = _slf.send_get_active_color(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_appearance_context_v1.get_active_color message: {}", Report::new(e));
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
    fn set_window_opacity(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        opacity: u32,
    ) {
        let res = _slf.send_set_window_opacity(
            opacity,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_appearance_context_v1.set_window_opacity message: {}", Report::new(e));
        }
    }

    /// get window window opacity
    ///
    /// Get the window window opacity
    #[inline]
    fn get_window_opacity(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
    ) {
        let res = _slf.send_get_window_opacity(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_appearance_context_v1.get_window_opacity message: {}", Report::new(e));
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
    fn set_window_theme_type(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        r#type: TreelandPersonalizationAppearanceContextV1ThemeType,
    ) {
        let res = _slf.send_set_window_theme_type(
            r#type,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_appearance_context_v1.set_window_theme_type message: {}", Report::new(e));
        }
    }

    /// get window theme type
    ///
    /// Get the window theme type
    #[inline]
    fn get_window_theme_type(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
    ) {
        let res = _slf.send_get_window_theme_type(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_appearance_context_v1.get_window_theme_type message: {}", Report::new(e));
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
    fn set_window_titlebar_height(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
        height: u32,
    ) {
        let res = _slf.send_set_window_titlebar_height(
            height,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_appearance_context_v1.set_window_titlebar_height message: {}", Report::new(e));
        }
    }

    /// get window titlebar height
    ///
    /// Get the window titlebar height
    #[inline]
    fn get_window_titlebar_height(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
    ) {
        let res = _slf.send_get_window_titlebar_height(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_appearance_context_v1.get_window_titlebar_height message: {}", Report::new(e));
        }
    }

    /// destroy the context object
    ///
    /// Destroy the context object.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<TreelandPersonalizationAppearanceContextV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_personalization_appearance_context_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for TreelandPersonalizationAppearanceContextV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::TreelandPersonalizationAppearanceContextV1, version),
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
                let arg0 = arg0 as i32;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.set_round_corner_radius(radius: {})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_round_corner_radius(&self, arg0);
                } else {
                    DefaultHandler.set_round_corner_radius(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.get_round_corner_radius()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).get_round_corner_radius(&self);
                } else {
                    DefaultHandler.get_round_corner_radius(&self);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("theme"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("theme"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("theme"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("theme"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.set_icon_theme(theme: {:?})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_icon_theme(&self, arg0);
                } else {
                    DefaultHandler.set_icon_theme(&self, arg0);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.get_icon_theme()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).get_icon_theme(&self);
                } else {
                    DefaultHandler.get_icon_theme(&self);
                }
            }
            4 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("color"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("color"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("color"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("color"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.set_active_color(color: {:?})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_active_color(&self, arg0);
                } else {
                    DefaultHandler.set_active_color(&self, arg0);
                }
            }
            5 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.get_active_color()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).get_active_color(&self);
                } else {
                    DefaultHandler.get_active_color(&self);
                }
            }
            6 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.set_window_opacity(opacity: {})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_window_opacity(&self, arg0);
                } else {
                    DefaultHandler.set_window_opacity(&self, arg0);
                }
            }
            7 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.get_window_opacity()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).get_window_opacity(&self);
                } else {
                    DefaultHandler.get_window_opacity(&self);
                }
            }
            8 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = TreelandPersonalizationAppearanceContextV1ThemeType(arg0);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.set_window_theme_type(type: {:?})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_window_theme_type(&self, arg0);
                } else {
                    DefaultHandler.set_window_theme_type(&self, arg0);
                }
            }
            9 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.get_window_theme_type()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).get_window_theme_type(&self);
                } else {
                    DefaultHandler.get_window_theme_type(&self);
                }
            }
            10 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.set_window_titlebar_height(height: {})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_window_titlebar_height(&self, arg0);
                } else {
                    DefaultHandler.set_window_titlebar_height(&self, arg0);
                }
            }
            11 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.get_window_titlebar_height()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).get_window_titlebar_height(&self);
                } else {
                    DefaultHandler.get_window_titlebar_height(&self);
                }
            }
            12 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_personalization_appearance_context_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
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
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = arg0 as i32;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_personalization_appearance_context_v1#{}.round_corner_radius(radius: {})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).round_corner_radius(&self, arg0);
                } else {
                    DefaultHandler.round_corner_radius(&self, arg0);
                }
            }
            1 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("theme_name"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("theme_name"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("theme_name"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("theme_name"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_personalization_appearance_context_v1#{}.icon_theme(theme_name: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).icon_theme(&self, arg0);
                } else {
                    DefaultHandler.icon_theme(&self, arg0);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("active_color"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("active_color"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("active_color"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("active_color"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_personalization_appearance_context_v1#{}.active_color(active_color: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).active_color(&self, arg0);
                } else {
                    DefaultHandler.active_color(&self, arg0);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_personalization_appearance_context_v1#{}.window_opacity(opacity: {})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).window_opacity(&self, arg0);
                } else {
                    DefaultHandler.window_opacity(&self, arg0);
                }
            }
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = TreelandPersonalizationAppearanceContextV1ThemeType(arg0);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_personalization_appearance_context_v1#{}.window_theme_type(type: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).window_theme_type(&self, arg0);
                } else {
                    DefaultHandler.window_theme_type(&self, arg0);
                }
            }
            5 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_personalization_appearance_context_v1#{}.window_titlebar_height(height: {})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).window_titlebar_height(&self, arg0);
                } else {
                    DefaultHandler.window_titlebar_height(&self, arg0);
                }
            }
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
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

impl Proxy for TreelandPersonalizationAppearanceContextV1 {
    fn core(&self) -> &ProxyCore {
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
