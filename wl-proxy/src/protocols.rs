#[cfg(feature = "protocol-hyprland_ctm_control_v1")]
pub mod hyprland_ctm_control_v1;
#[cfg(feature = "protocol-hyprland_focus_grab_v1")]
pub mod hyprland_focus_grab_v1;
#[cfg(feature = "protocol-hyprland_global_shortcuts_v1")]
pub mod hyprland_global_shortcuts_v1;
#[cfg(feature = "protocol-hyprland_input_capture_v1")]
pub mod hyprland_input_capture_v1;
#[cfg(feature = "protocol-hyprland_lock_notify_v1")]
pub mod hyprland_lock_notify_v1;
#[cfg(feature = "protocol-hyprland_surface_v1")]
pub mod hyprland_surface_v1;
#[cfg(feature = "protocol-hyprland_toplevel_export_v1")]
pub mod hyprland_toplevel_export_v1;
#[cfg(feature = "protocol-hyprland_toplevel_mapping_v1")]
pub mod hyprland_toplevel_mapping_v1;
#[cfg(feature = "protocol-jay_tray_v1")]
pub mod jay_tray_v1;
#[cfg(feature = "protocol-drm")]
pub mod drm;
#[cfg(feature = "protocol-input_method_unstable_v2")]
pub mod input_method_unstable_v2;
#[cfg(feature = "protocol-org_kde_kwin_server_decoration_v1")]
pub mod org_kde_kwin_server_decoration_v1;
#[cfg(feature = "protocol-virtual_keyboard_unstable_v1")]
pub mod virtual_keyboard_unstable_v1;
#[cfg(feature = "protocol-treeland_app_id_resolver_v1")]
pub mod treeland_app_id_resolver_v1;
#[cfg(feature = "protocol-treeland_dde_shell_v1")]
pub mod treeland_dde_shell_v1;
#[cfg(feature = "protocol-treeland_ddm")]
pub mod treeland_ddm;
#[cfg(feature = "protocol-treeland_foreign_toplevel_manager_v1")]
pub mod treeland_foreign_toplevel_manager_v1;
#[cfg(feature = "protocol-treeland_output_manager_v1")]
pub mod treeland_output_manager_v1;
#[cfg(feature = "protocol-treeland_personalization_manager_v1")]
pub mod treeland_personalization_manager_v1;
#[cfg(feature = "protocol-treeland_prelaunch_splash_v1")]
pub mod treeland_prelaunch_splash_v1;
#[cfg(feature = "protocol-treeland_screensaver")]
pub mod treeland_screensaver;
#[cfg(feature = "protocol-treeland_shortcut_manager_v1")]
pub mod treeland_shortcut_manager_v1;
#[cfg(feature = "protocol-treeland_shortcut_manager_v2")]
pub mod treeland_shortcut_manager_v2;
#[cfg(feature = "protocol-treeland_virtual_output_manager_v1")]
pub mod treeland_virtual_output_manager_v1;
#[cfg(feature = "protocol-treeland_wallpaper_color_v1")]
pub mod treeland_wallpaper_color_v1;
#[cfg(feature = "protocol-treeland_window_management_v1")]
pub mod treeland_window_management_v1;
pub mod wayland;
#[cfg(feature = "protocol-alpha_modifier_v1")]
pub mod alpha_modifier_v1;
#[cfg(feature = "protocol-color_management_v1")]
pub mod color_management_v1;
#[cfg(feature = "protocol-color_representation_v1")]
pub mod color_representation_v1;
#[cfg(feature = "protocol-commit_timing_v1")]
pub mod commit_timing_v1;
#[cfg(feature = "protocol-content_type_v1")]
pub mod content_type_v1;
#[cfg(feature = "protocol-cursor_shape_v1")]
pub mod cursor_shape_v1;
#[cfg(feature = "protocol-drm_lease_v1")]
pub mod drm_lease_v1;
#[cfg(feature = "protocol-ext_background_effect_v1")]
pub mod ext_background_effect_v1;
#[cfg(feature = "protocol-ext_data_control_v1")]
pub mod ext_data_control_v1;
#[cfg(feature = "protocol-ext_foreign_toplevel_list_v1")]
pub mod ext_foreign_toplevel_list_v1;
#[cfg(feature = "protocol-ext_idle_notify_v1")]
pub mod ext_idle_notify_v1;
#[cfg(feature = "protocol-ext_image_capture_source_v1")]
pub mod ext_image_capture_source_v1;
#[cfg(feature = "protocol-ext_image_copy_capture_v1")]
pub mod ext_image_copy_capture_v1;
#[cfg(feature = "protocol-ext_session_lock_v1")]
pub mod ext_session_lock_v1;
#[cfg(feature = "protocol-ext_transient_seat_v1")]
pub mod ext_transient_seat_v1;
#[cfg(feature = "protocol-ext_workspace_v1")]
pub mod ext_workspace_v1;
#[cfg(feature = "protocol-fifo_v1")]
pub mod fifo_v1;
#[cfg(feature = "protocol-fractional_scale_v1")]
pub mod fractional_scale_v1;
#[cfg(feature = "protocol-fullscreen_shell_unstable_v1")]
pub mod fullscreen_shell_unstable_v1;
#[cfg(feature = "protocol-idle_inhibit_unstable_v1")]
pub mod idle_inhibit_unstable_v1;
#[cfg(feature = "protocol-input_method_unstable_v1")]
pub mod input_method_unstable_v1;
#[cfg(feature = "protocol-input_timestamps_unstable_v1")]
pub mod input_timestamps_unstable_v1;
#[cfg(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1")]
pub mod keyboard_shortcuts_inhibit_unstable_v1;
#[cfg(feature = "protocol-linux_dmabuf_v1")]
pub mod linux_dmabuf_v1;
#[cfg(feature = "protocol-linux_drm_syncobj_v1")]
pub mod linux_drm_syncobj_v1;
#[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
pub mod pointer_constraints_unstable_v1;
#[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
pub mod pointer_gestures_unstable_v1;
#[cfg(feature = "protocol-pointer_warp_v1")]
pub mod pointer_warp_v1;
#[cfg(feature = "protocol-presentation_time")]
pub mod presentation_time;
#[cfg(feature = "protocol-relative_pointer_unstable_v1")]
pub mod relative_pointer_unstable_v1;
#[cfg(feature = "protocol-security_context_v1")]
pub mod security_context_v1;
#[cfg(feature = "protocol-single_pixel_buffer_v1")]
pub mod single_pixel_buffer_v1;
#[cfg(feature = "protocol-tablet_v2")]
pub mod tablet_v2;
#[cfg(feature = "protocol-tearing_control_v1")]
pub mod tearing_control_v1;
#[cfg(feature = "protocol-text_input_unstable_v1")]
pub mod text_input_unstable_v1;
#[cfg(feature = "protocol-text_input_unstable_v3")]
pub mod text_input_unstable_v3;
#[cfg(feature = "protocol-viewporter")]
pub mod viewporter;
#[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
pub mod wp_primary_selection_unstable_v1;
#[cfg(feature = "protocol-xdg_activation_v1")]
pub mod xdg_activation_v1;
#[cfg(feature = "protocol-xdg_decoration_unstable_v1")]
pub mod xdg_decoration_unstable_v1;
#[cfg(feature = "protocol-xdg_dialog_v1")]
pub mod xdg_dialog_v1;
#[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
pub mod xdg_foreign_unstable_v2;
#[cfg(feature = "protocol-xdg_output_unstable_v1")]
pub mod xdg_output_unstable_v1;
#[cfg(feature = "protocol-xdg_shell")]
pub mod xdg_shell;
#[cfg(feature = "protocol-xdg_system_bell_v1")]
pub mod xdg_system_bell_v1;
#[cfg(feature = "protocol-xdg_toplevel_drag_v1")]
pub mod xdg_toplevel_drag_v1;
#[cfg(feature = "protocol-xdg_toplevel_icon_v1")]
pub mod xdg_toplevel_icon_v1;
#[cfg(feature = "protocol-xdg_toplevel_tag_v1")]
pub mod xdg_toplevel_tag_v1;
#[cfg(feature = "protocol-xwayland_keyboard_grab_unstable_v1")]
pub mod xwayland_keyboard_grab_unstable_v1;
#[cfg(feature = "protocol-xwayland_shell_v1")]
pub mod xwayland_shell_v1;
#[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
pub mod zwp_linux_explicit_synchronization_unstable_v1;
#[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
pub mod wlr_data_control_unstable_v1;
#[cfg(feature = "protocol-wlr_export_dmabuf_unstable_v1")]
pub mod wlr_export_dmabuf_unstable_v1;
#[cfg(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1")]
pub mod wlr_foreign_toplevel_management_unstable_v1;
#[cfg(feature = "protocol-wlr_gamma_control_unstable_v1")]
pub mod wlr_gamma_control_unstable_v1;
#[cfg(feature = "protocol-wlr_input_inhibit_unstable_v1")]
pub mod wlr_input_inhibit_unstable_v1;
#[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")]
pub mod wlr_layer_shell_unstable_v1;
#[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
pub mod wlr_output_management_unstable_v1;
#[cfg(feature = "protocol-wlr_output_power_management_unstable_v1")]
pub mod wlr_output_power_management_unstable_v1;
#[cfg(feature = "protocol-wlr_screencopy_unstable_v1")]
pub mod wlr_screencopy_unstable_v1;
#[cfg(feature = "protocol-wlr_virtual_pointer_unstable_v1")]
pub mod wlr_virtual_pointer_unstable_v1;

#[allow(unused_imports)]
mod all_types {
    #[cfg(feature = "protocol-hyprland_ctm_control_v1")]
    pub(super) use super::hyprland_ctm_control_v1::hyprland_ctm_control_manager_v1::HyprlandCtmControlManagerV1;
    #[cfg(feature = "protocol-hyprland_ctm_control_v1")]
    pub(super) use super::hyprland_ctm_control_v1::hyprland_ctm_control_manager_v1::HyprlandCtmControlManagerV1Error;
    #[cfg(feature = "protocol-hyprland_focus_grab_v1")]
    pub(super) use super::hyprland_focus_grab_v1::hyprland_focus_grab_manager_v1::HyprlandFocusGrabManagerV1;
    #[cfg(feature = "protocol-hyprland_focus_grab_v1")]
    pub(super) use super::hyprland_focus_grab_v1::hyprland_focus_grab_v1::HyprlandFocusGrabV1;
    #[cfg(feature = "protocol-hyprland_global_shortcuts_v1")]
    pub(super) use super::hyprland_global_shortcuts_v1::hyprland_global_shortcut_v1::HyprlandGlobalShortcutV1;
    #[cfg(feature = "protocol-hyprland_global_shortcuts_v1")]
    pub(super) use super::hyprland_global_shortcuts_v1::hyprland_global_shortcuts_manager_v1::HyprlandGlobalShortcutsManagerV1;
    #[cfg(feature = "protocol-hyprland_global_shortcuts_v1")]
    pub(super) use super::hyprland_global_shortcuts_v1::hyprland_global_shortcuts_manager_v1::HyprlandGlobalShortcutsManagerV1Error;
    #[cfg(feature = "protocol-hyprland_input_capture_v1")]
    pub(super) use super::hyprland_input_capture_v1::hyprland_input_capture_manager_v1::HyprlandInputCaptureManagerV1;
    #[cfg(feature = "protocol-hyprland_input_capture_v1")]
    pub(super) use super::hyprland_input_capture_v1::hyprland_input_capture_v1::HyprlandInputCaptureV1;
    #[cfg(feature = "protocol-hyprland_input_capture_v1")]
    pub(super) use super::hyprland_input_capture_v1::hyprland_input_capture_v1::HyprlandInputCaptureV1Error;
    #[cfg(feature = "protocol-hyprland_lock_notify_v1")]
    pub(super) use super::hyprland_lock_notify_v1::hyprland_lock_notification_v1::HyprlandLockNotificationV1;
    #[cfg(feature = "protocol-hyprland_lock_notify_v1")]
    pub(super) use super::hyprland_lock_notify_v1::hyprland_lock_notifier_v1::HyprlandLockNotifierV1;
    #[cfg(feature = "protocol-hyprland_surface_v1")]
    pub(super) use super::hyprland_surface_v1::hyprland_surface_manager_v1::HyprlandSurfaceManagerV1;
    #[cfg(feature = "protocol-hyprland_surface_v1")]
    pub(super) use super::hyprland_surface_v1::hyprland_surface_manager_v1::HyprlandSurfaceManagerV1Error;
    #[cfg(feature = "protocol-hyprland_surface_v1")]
    pub(super) use super::hyprland_surface_v1::hyprland_surface_v1::HyprlandSurfaceV1;
    #[cfg(feature = "protocol-hyprland_surface_v1")]
    pub(super) use super::hyprland_surface_v1::hyprland_surface_v1::HyprlandSurfaceV1Error;
    #[cfg(feature = "protocol-hyprland_toplevel_export_v1")]
    pub(super) use super::hyprland_toplevel_export_v1::hyprland_toplevel_export_frame_v1::HyprlandToplevelExportFrameV1;
    #[cfg(feature = "protocol-hyprland_toplevel_export_v1")]
    pub(super) use super::hyprland_toplevel_export_v1::hyprland_toplevel_export_frame_v1::HyprlandToplevelExportFrameV1Error;
    #[cfg(feature = "protocol-hyprland_toplevel_export_v1")]
    pub(super) use super::hyprland_toplevel_export_v1::hyprland_toplevel_export_frame_v1::HyprlandToplevelExportFrameV1Flags;
    #[cfg(feature = "protocol-hyprland_toplevel_export_v1")]
    pub(super) use super::hyprland_toplevel_export_v1::hyprland_toplevel_export_manager_v1::HyprlandToplevelExportManagerV1;
    #[cfg(feature = "protocol-hyprland_toplevel_mapping_v1")]
    pub(super) use super::hyprland_toplevel_mapping_v1::hyprland_toplevel_mapping_manager_v1::HyprlandToplevelMappingManagerV1;
    #[cfg(feature = "protocol-hyprland_toplevel_mapping_v1")]
    pub(super) use super::hyprland_toplevel_mapping_v1::hyprland_toplevel_window_mapping_handle_v1::HyprlandToplevelWindowMappingHandleV1;
    #[cfg(feature = "protocol-jay_tray_v1")]
    pub(super) use super::jay_tray_v1::jay_tray_item_v1::JayTrayItemV1;
    #[cfg(feature = "protocol-jay_tray_v1")]
    pub(super) use super::jay_tray_v1::jay_tray_item_v1::JayTrayItemV1Error;
    #[cfg(feature = "protocol-jay_tray_v1")]
    pub(super) use super::jay_tray_v1::jay_tray_item_v1::JayTrayItemV1KeyboardFocusHint;
    #[cfg(feature = "protocol-jay_tray_v1")]
    pub(super) use super::jay_tray_v1::jay_tray_v1::JayTrayV1;
    #[cfg(feature = "protocol-jay_tray_v1")]
    pub(super) use super::jay_tray_v1::jay_tray_v1::JayTrayV1Error;
    #[cfg(feature = "protocol-drm")]
    pub(super) use super::drm::wl_drm::WlDrm;
    #[cfg(feature = "protocol-drm")]
    pub(super) use super::drm::wl_drm::WlDrmError;
    #[cfg(feature = "protocol-drm")]
    pub(super) use super::drm::wl_drm::WlDrmFormat;
    #[cfg(feature = "protocol-drm")]
    pub(super) use super::drm::wl_drm::WlDrmCapability;
    #[cfg(feature = "protocol-input_method_unstable_v2")]
    pub(super) use super::input_method_unstable_v2::zwp_input_method_keyboard_grab_v2::ZwpInputMethodKeyboardGrabV2;
    #[cfg(feature = "protocol-input_method_unstable_v2")]
    pub(super) use super::input_method_unstable_v2::zwp_input_method_manager_v2::ZwpInputMethodManagerV2;
    #[cfg(feature = "protocol-input_method_unstable_v2")]
    pub(super) use super::input_method_unstable_v2::zwp_input_method_v2::ZwpInputMethodV2;
    #[cfg(feature = "protocol-input_method_unstable_v2")]
    pub(super) use super::input_method_unstable_v2::zwp_input_method_v2::ZwpInputMethodV2Error;
    #[cfg(feature = "protocol-input_method_unstable_v2")]
    pub(super) use super::input_method_unstable_v2::zwp_input_popup_surface_v2::ZwpInputPopupSurfaceV2;
    #[cfg(feature = "protocol-org_kde_kwin_server_decoration_v1")]
    pub(super) use super::org_kde_kwin_server_decoration_v1::org_kde_kwin_server_decoration::OrgKdeKwinServerDecoration;
    #[cfg(feature = "protocol-org_kde_kwin_server_decoration_v1")]
    pub(super) use super::org_kde_kwin_server_decoration_v1::org_kde_kwin_server_decoration::OrgKdeKwinServerDecorationMode;
    #[cfg(feature = "protocol-org_kde_kwin_server_decoration_v1")]
    pub(super) use super::org_kde_kwin_server_decoration_v1::org_kde_kwin_server_decoration_manager::OrgKdeKwinServerDecorationManager;
    #[cfg(feature = "protocol-org_kde_kwin_server_decoration_v1")]
    pub(super) use super::org_kde_kwin_server_decoration_v1::org_kde_kwin_server_decoration_manager::OrgKdeKwinServerDecorationManagerMode;
    #[cfg(feature = "protocol-virtual_keyboard_unstable_v1")]
    pub(super) use super::virtual_keyboard_unstable_v1::zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1;
    #[cfg(feature = "protocol-virtual_keyboard_unstable_v1")]
    pub(super) use super::virtual_keyboard_unstable_v1::zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1Error;
    #[cfg(feature = "protocol-virtual_keyboard_unstable_v1")]
    pub(super) use super::virtual_keyboard_unstable_v1::zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1;
    #[cfg(feature = "protocol-virtual_keyboard_unstable_v1")]
    pub(super) use super::virtual_keyboard_unstable_v1::zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1Error;
    #[cfg(feature = "protocol-treeland_app_id_resolver_v1")]
    pub(super) use super::treeland_app_id_resolver_v1::treeland_app_id_resolver_manager_v1::TreelandAppIdResolverManagerV1;
    #[cfg(feature = "protocol-treeland_app_id_resolver_v1")]
    pub(super) use super::treeland_app_id_resolver_v1::treeland_app_id_resolver_manager_v1::TreelandAppIdResolverManagerV1Error;
    #[cfg(feature = "protocol-treeland_app_id_resolver_v1")]
    pub(super) use super::treeland_app_id_resolver_v1::treeland_app_id_resolver_v1::TreelandAppIdResolverV1;
    #[cfg(feature = "protocol-treeland_dde_shell_v1")]
    pub(super) use super::treeland_dde_shell_v1::treeland_dde_active_v1::TreelandDdeActiveV1;
    #[cfg(feature = "protocol-treeland_dde_shell_v1")]
    pub(super) use super::treeland_dde_shell_v1::treeland_dde_active_v1::TreelandDdeActiveV1Reason;
    #[cfg(feature = "protocol-treeland_dde_shell_v1")]
    pub(super) use super::treeland_dde_shell_v1::treeland_dde_shell_manager_v1::TreelandDdeShellManagerV1;
    #[cfg(feature = "protocol-treeland_dde_shell_v1")]
    pub(super) use super::treeland_dde_shell_v1::treeland_dde_shell_surface_v1::TreelandDdeShellSurfaceV1;
    #[cfg(feature = "protocol-treeland_dde_shell_v1")]
    pub(super) use super::treeland_dde_shell_v1::treeland_dde_shell_surface_v1::TreelandDdeShellSurfaceV1Role;
    #[cfg(feature = "protocol-treeland_dde_shell_v1")]
    pub(super) use super::treeland_dde_shell_v1::treeland_lockscreen_v1::TreelandLockscreenV1;
    #[cfg(feature = "protocol-treeland_dde_shell_v1")]
    pub(super) use super::treeland_dde_shell_v1::treeland_multitaskview_v1::TreelandMultitaskviewV1;
    #[cfg(feature = "protocol-treeland_dde_shell_v1")]
    pub(super) use super::treeland_dde_shell_v1::treeland_window_overlap_checker::TreelandWindowOverlapChecker;
    #[cfg(feature = "protocol-treeland_dde_shell_v1")]
    pub(super) use super::treeland_dde_shell_v1::treeland_window_overlap_checker::TreelandWindowOverlapCheckerAnchor;
    #[cfg(feature = "protocol-treeland_dde_shell_v1")]
    pub(super) use super::treeland_dde_shell_v1::treeland_window_picker_v1::TreelandWindowPickerV1;
    #[cfg(feature = "protocol-treeland_ddm")]
    pub(super) use super::treeland_ddm::treeland_ddm::TreelandDdm;
    #[cfg(feature = "protocol-treeland_foreign_toplevel_manager_v1")]
    pub(super) use super::treeland_foreign_toplevel_manager_v1::treeland_dock_preview_context_v1::TreelandDockPreviewContextV1;
    #[cfg(feature = "protocol-treeland_foreign_toplevel_manager_v1")]
    pub(super) use super::treeland_foreign_toplevel_manager_v1::treeland_dock_preview_context_v1::TreelandDockPreviewContextV1Direction;
    #[cfg(feature = "protocol-treeland_foreign_toplevel_manager_v1")]
    pub(super) use super::treeland_foreign_toplevel_manager_v1::treeland_foreign_toplevel_handle_v1::TreelandForeignToplevelHandleV1;
    #[cfg(feature = "protocol-treeland_foreign_toplevel_manager_v1")]
    pub(super) use super::treeland_foreign_toplevel_manager_v1::treeland_foreign_toplevel_handle_v1::TreelandForeignToplevelHandleV1State;
    #[cfg(feature = "protocol-treeland_foreign_toplevel_manager_v1")]
    pub(super) use super::treeland_foreign_toplevel_manager_v1::treeland_foreign_toplevel_handle_v1::TreelandForeignToplevelHandleV1Error;
    #[cfg(feature = "protocol-treeland_foreign_toplevel_manager_v1")]
    pub(super) use super::treeland_foreign_toplevel_manager_v1::treeland_foreign_toplevel_manager_v1::TreelandForeignToplevelManagerV1;
    #[cfg(feature = "protocol-treeland_output_manager_v1")]
    pub(super) use super::treeland_output_manager_v1::treeland_output_color_control_v1::TreelandOutputColorControlV1;
    #[cfg(feature = "protocol-treeland_output_manager_v1")]
    pub(super) use super::treeland_output_manager_v1::treeland_output_color_control_v1::TreelandOutputColorControlV1Error;
    #[cfg(feature = "protocol-treeland_output_manager_v1")]
    pub(super) use super::treeland_output_manager_v1::treeland_output_manager_v1::TreelandOutputManagerV1;
    #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
    pub(super) use super::treeland_personalization_manager_v1::treeland_personalization_appearance_context_v1::TreelandPersonalizationAppearanceContextV1;
    #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
    pub(super) use super::treeland_personalization_manager_v1::treeland_personalization_appearance_context_v1::TreelandPersonalizationAppearanceContextV1ThemeType;
    #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
    pub(super) use super::treeland_personalization_manager_v1::treeland_personalization_appearance_context_v1::TreelandPersonalizationAppearanceContextV1Error;
    #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
    pub(super) use super::treeland_personalization_manager_v1::treeland_personalization_cursor_context_v1::TreelandPersonalizationCursorContextV1;
    #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
    pub(super) use super::treeland_personalization_manager_v1::treeland_personalization_font_context_v1::TreelandPersonalizationFontContextV1;
    #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
    pub(super) use super::treeland_personalization_manager_v1::treeland_personalization_manager_v1::TreelandPersonalizationManagerV1;
    #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
    pub(super) use super::treeland_personalization_manager_v1::treeland_personalization_wallpaper_context_v1::TreelandPersonalizationWallpaperContextV1;
    #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
    pub(super) use super::treeland_personalization_manager_v1::treeland_personalization_wallpaper_context_v1::TreelandPersonalizationWallpaperContextV1Options;
    #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
    pub(super) use super::treeland_personalization_manager_v1::treeland_personalization_window_context_v1::TreelandPersonalizationWindowContextV1;
    #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
    pub(super) use super::treeland_personalization_manager_v1::treeland_personalization_window_context_v1::TreelandPersonalizationWindowContextV1BlendMode;
    #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
    pub(super) use super::treeland_personalization_manager_v1::treeland_personalization_window_context_v1::TreelandPersonalizationWindowContextV1EnableMode;
    #[cfg(feature = "protocol-treeland_prelaunch_splash_v1")]
    pub(super) use super::treeland_prelaunch_splash_v1::treeland_prelaunch_splash_manager_v1::TreelandPrelaunchSplashManagerV1;
    #[cfg(feature = "protocol-treeland_screensaver")]
    pub(super) use super::treeland_screensaver::treeland_screensaver::TreelandScreensaver;
    #[cfg(feature = "protocol-treeland_screensaver")]
    pub(super) use super::treeland_screensaver::treeland_screensaver::TreelandScreensaverError;
    #[cfg(feature = "protocol-treeland_shortcut_manager_v1")]
    pub(super) use super::treeland_shortcut_manager_v1::treeland_shortcut_context_v1::TreelandShortcutContextV1;
    #[cfg(feature = "protocol-treeland_shortcut_manager_v1")]
    pub(super) use super::treeland_shortcut_manager_v1::treeland_shortcut_context_v1::TreelandShortcutContextV1Error;
    #[cfg(feature = "protocol-treeland_shortcut_manager_v1")]
    pub(super) use super::treeland_shortcut_manager_v1::treeland_shortcut_manager_v1::TreelandShortcutManagerV1;
    #[cfg(feature = "protocol-treeland_shortcut_manager_v2")]
    pub(super) use super::treeland_shortcut_manager_v2::treeland_shortcut_manager_v2::TreelandShortcutManagerV2;
    #[cfg(feature = "protocol-treeland_shortcut_manager_v2")]
    pub(super) use super::treeland_shortcut_manager_v2::treeland_shortcut_manager_v2::TreelandShortcutManagerV2Direction;
    #[cfg(feature = "protocol-treeland_shortcut_manager_v2")]
    pub(super) use super::treeland_shortcut_manager_v2::treeland_shortcut_manager_v2::TreelandShortcutManagerV2Action;
    #[cfg(feature = "protocol-treeland_shortcut_manager_v2")]
    pub(super) use super::treeland_shortcut_manager_v2::treeland_shortcut_manager_v2::TreelandShortcutManagerV2KeybindMode;
    #[cfg(feature = "protocol-treeland_shortcut_manager_v2")]
    pub(super) use super::treeland_shortcut_manager_v2::treeland_shortcut_manager_v2::TreelandShortcutManagerV2BindError;
    #[cfg(feature = "protocol-treeland_shortcut_manager_v2")]
    pub(super) use super::treeland_shortcut_manager_v2::treeland_shortcut_manager_v2::TreelandShortcutManagerV2Error;
    #[cfg(feature = "protocol-treeland_virtual_output_manager_v1")]
    pub(super) use super::treeland_virtual_output_manager_v1::treeland_virtual_output_manager_v1::TreelandVirtualOutputManagerV1;
    #[cfg(feature = "protocol-treeland_virtual_output_manager_v1")]
    pub(super) use super::treeland_virtual_output_manager_v1::treeland_virtual_output_v1::TreelandVirtualOutputV1;
    #[cfg(feature = "protocol-treeland_virtual_output_manager_v1")]
    pub(super) use super::treeland_virtual_output_manager_v1::treeland_virtual_output_v1::TreelandVirtualOutputV1Error;
    #[cfg(feature = "protocol-treeland_wallpaper_color_v1")]
    pub(super) use super::treeland_wallpaper_color_v1::treeland_wallpaper_color_manager_v1::TreelandWallpaperColorManagerV1;
    #[cfg(feature = "protocol-treeland_window_management_v1")]
    pub(super) use super::treeland_window_management_v1::treeland_window_management_v1::TreelandWindowManagementV1;
    #[cfg(feature = "protocol-treeland_window_management_v1")]
    pub(super) use super::treeland_window_management_v1::treeland_window_management_v1::TreelandWindowManagementV1DesktopState;
    pub(super) use super::wayland::wl_buffer::WlBuffer;
    pub(super) use super::wayland::wl_callback::WlCallback;
    pub(super) use super::wayland::wl_compositor::WlCompositor;
    pub(super) use super::wayland::wl_data_device::WlDataDevice;
    pub(super) use super::wayland::wl_data_device::WlDataDeviceError;
    pub(super) use super::wayland::wl_data_device_manager::WlDataDeviceManager;
    pub(super) use super::wayland::wl_data_device_manager::WlDataDeviceManagerDndAction;
    pub(super) use super::wayland::wl_data_offer::WlDataOffer;
    pub(super) use super::wayland::wl_data_offer::WlDataOfferError;
    pub(super) use super::wayland::wl_data_source::WlDataSource;
    pub(super) use super::wayland::wl_data_source::WlDataSourceError;
    pub(super) use super::wayland::wl_display::WlDisplay;
    pub(super) use super::wayland::wl_display::WlDisplayError;
    pub(super) use super::wayland::wl_fixes::WlFixes;
    pub(super) use super::wayland::wl_keyboard::WlKeyboard;
    pub(super) use super::wayland::wl_keyboard::WlKeyboardKeymapFormat;
    pub(super) use super::wayland::wl_keyboard::WlKeyboardKeyState;
    pub(super) use super::wayland::wl_output::WlOutput;
    pub(super) use super::wayland::wl_output::WlOutputSubpixel;
    pub(super) use super::wayland::wl_output::WlOutputTransform;
    pub(super) use super::wayland::wl_output::WlOutputMode;
    pub(super) use super::wayland::wl_pointer::WlPointer;
    pub(super) use super::wayland::wl_pointer::WlPointerError;
    pub(super) use super::wayland::wl_pointer::WlPointerButtonState;
    pub(super) use super::wayland::wl_pointer::WlPointerAxis;
    pub(super) use super::wayland::wl_pointer::WlPointerAxisSource;
    pub(super) use super::wayland::wl_pointer::WlPointerAxisRelativeDirection;
    pub(super) use super::wayland::wl_region::WlRegion;
    pub(super) use super::wayland::wl_registry::WlRegistry;
    pub(super) use super::wayland::wl_seat::WlSeat;
    pub(super) use super::wayland::wl_seat::WlSeatCapability;
    pub(super) use super::wayland::wl_seat::WlSeatError;
    pub(super) use super::wayland::wl_shell::WlShell;
    pub(super) use super::wayland::wl_shell::WlShellError;
    pub(super) use super::wayland::wl_shell_surface::WlShellSurface;
    pub(super) use super::wayland::wl_shell_surface::WlShellSurfaceResize;
    pub(super) use super::wayland::wl_shell_surface::WlShellSurfaceTransient;
    pub(super) use super::wayland::wl_shell_surface::WlShellSurfaceFullscreenMethod;
    pub(super) use super::wayland::wl_shm::WlShm;
    pub(super) use super::wayland::wl_shm::WlShmError;
    pub(super) use super::wayland::wl_shm::WlShmFormat;
    pub(super) use super::wayland::wl_shm_pool::WlShmPool;
    pub(super) use super::wayland::wl_subcompositor::WlSubcompositor;
    pub(super) use super::wayland::wl_subcompositor::WlSubcompositorError;
    pub(super) use super::wayland::wl_subsurface::WlSubsurface;
    pub(super) use super::wayland::wl_subsurface::WlSubsurfaceError;
    pub(super) use super::wayland::wl_surface::WlSurface;
    pub(super) use super::wayland::wl_surface::WlSurfaceError;
    pub(super) use super::wayland::wl_touch::WlTouch;
    #[cfg(feature = "protocol-alpha_modifier_v1")]
    pub(super) use super::alpha_modifier_v1::wp_alpha_modifier_surface_v1::WpAlphaModifierSurfaceV1;
    #[cfg(feature = "protocol-alpha_modifier_v1")]
    pub(super) use super::alpha_modifier_v1::wp_alpha_modifier_surface_v1::WpAlphaModifierSurfaceV1Error;
    #[cfg(feature = "protocol-alpha_modifier_v1")]
    pub(super) use super::alpha_modifier_v1::wp_alpha_modifier_v1::WpAlphaModifierV1;
    #[cfg(feature = "protocol-alpha_modifier_v1")]
    pub(super) use super::alpha_modifier_v1::wp_alpha_modifier_v1::WpAlphaModifierV1Error;
    #[cfg(feature = "protocol-color_management_v1")]
    pub(super) use super::color_management_v1::wp_color_management_output_v1::WpColorManagementOutputV1;
    #[cfg(feature = "protocol-color_management_v1")]
    pub(super) use super::color_management_v1::wp_color_management_surface_feedback_v1::WpColorManagementSurfaceFeedbackV1;
    #[cfg(feature = "protocol-color_management_v1")]
    pub(super) use super::color_management_v1::wp_color_management_surface_feedback_v1::WpColorManagementSurfaceFeedbackV1Error;
    #[cfg(feature = "protocol-color_management_v1")]
    pub(super) use super::color_management_v1::wp_color_management_surface_v1::WpColorManagementSurfaceV1;
    #[cfg(feature = "protocol-color_management_v1")]
    pub(super) use super::color_management_v1::wp_color_management_surface_v1::WpColorManagementSurfaceV1Error;
    #[cfg(feature = "protocol-color_management_v1")]
    pub(super) use super::color_management_v1::wp_color_manager_v1::WpColorManagerV1;
    #[cfg(feature = "protocol-color_management_v1")]
    pub(super) use super::color_management_v1::wp_color_manager_v1::WpColorManagerV1Error;
    #[cfg(feature = "protocol-color_management_v1")]
    pub(super) use super::color_management_v1::wp_color_manager_v1::WpColorManagerV1RenderIntent;
    #[cfg(feature = "protocol-color_management_v1")]
    pub(super) use super::color_management_v1::wp_color_manager_v1::WpColorManagerV1Feature;
    #[cfg(feature = "protocol-color_management_v1")]
    pub(super) use super::color_management_v1::wp_color_manager_v1::WpColorManagerV1Primaries;
    #[cfg(feature = "protocol-color_management_v1")]
    pub(super) use super::color_management_v1::wp_color_manager_v1::WpColorManagerV1TransferFunction;
    #[cfg(feature = "protocol-color_management_v1")]
    pub(super) use super::color_management_v1::wp_image_description_creator_icc_v1::WpImageDescriptionCreatorIccV1;
    #[cfg(feature = "protocol-color_management_v1")]
    pub(super) use super::color_management_v1::wp_image_description_creator_icc_v1::WpImageDescriptionCreatorIccV1Error;
    #[cfg(feature = "protocol-color_management_v1")]
    pub(super) use super::color_management_v1::wp_image_description_creator_params_v1::WpImageDescriptionCreatorParamsV1;
    #[cfg(feature = "protocol-color_management_v1")]
    pub(super) use super::color_management_v1::wp_image_description_creator_params_v1::WpImageDescriptionCreatorParamsV1Error;
    #[cfg(feature = "protocol-color_management_v1")]
    pub(super) use super::color_management_v1::wp_image_description_info_v1::WpImageDescriptionInfoV1;
    #[cfg(feature = "protocol-color_management_v1")]
    pub(super) use super::color_management_v1::wp_image_description_v1::WpImageDescriptionV1;
    #[cfg(feature = "protocol-color_management_v1")]
    pub(super) use super::color_management_v1::wp_image_description_v1::WpImageDescriptionV1Error;
    #[cfg(feature = "protocol-color_management_v1")]
    pub(super) use super::color_management_v1::wp_image_description_v1::WpImageDescriptionV1Cause;
    #[cfg(feature = "protocol-color_representation_v1")]
    pub(super) use super::color_representation_v1::wp_color_representation_manager_v1::WpColorRepresentationManagerV1;
    #[cfg(feature = "protocol-color_representation_v1")]
    pub(super) use super::color_representation_v1::wp_color_representation_manager_v1::WpColorRepresentationManagerV1Error;
    #[cfg(feature = "protocol-color_representation_v1")]
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::WpColorRepresentationSurfaceV1;
    #[cfg(feature = "protocol-color_representation_v1")]
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::WpColorRepresentationSurfaceV1Error;
    #[cfg(feature = "protocol-color_representation_v1")]
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::WpColorRepresentationSurfaceV1AlphaMode;
    #[cfg(feature = "protocol-color_representation_v1")]
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::WpColorRepresentationSurfaceV1Coefficients;
    #[cfg(feature = "protocol-color_representation_v1")]
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::WpColorRepresentationSurfaceV1Range;
    #[cfg(feature = "protocol-color_representation_v1")]
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::WpColorRepresentationSurfaceV1ChromaLocation;
    #[cfg(feature = "protocol-commit_timing_v1")]
    pub(super) use super::commit_timing_v1::wp_commit_timer_v1::WpCommitTimerV1;
    #[cfg(feature = "protocol-commit_timing_v1")]
    pub(super) use super::commit_timing_v1::wp_commit_timer_v1::WpCommitTimerV1Error;
    #[cfg(feature = "protocol-commit_timing_v1")]
    pub(super) use super::commit_timing_v1::wp_commit_timing_manager_v1::WpCommitTimingManagerV1;
    #[cfg(feature = "protocol-commit_timing_v1")]
    pub(super) use super::commit_timing_v1::wp_commit_timing_manager_v1::WpCommitTimingManagerV1Error;
    #[cfg(feature = "protocol-content_type_v1")]
    pub(super) use super::content_type_v1::wp_content_type_manager_v1::WpContentTypeManagerV1;
    #[cfg(feature = "protocol-content_type_v1")]
    pub(super) use super::content_type_v1::wp_content_type_manager_v1::WpContentTypeManagerV1Error;
    #[cfg(feature = "protocol-content_type_v1")]
    pub(super) use super::content_type_v1::wp_content_type_v1::WpContentTypeV1;
    #[cfg(feature = "protocol-content_type_v1")]
    pub(super) use super::content_type_v1::wp_content_type_v1::WpContentTypeV1Type;
    #[cfg(feature = "protocol-cursor_shape_v1")]
    pub(super) use super::cursor_shape_v1::wp_cursor_shape_device_v1::WpCursorShapeDeviceV1;
    #[cfg(feature = "protocol-cursor_shape_v1")]
    pub(super) use super::cursor_shape_v1::wp_cursor_shape_device_v1::WpCursorShapeDeviceV1Shape;
    #[cfg(feature = "protocol-cursor_shape_v1")]
    pub(super) use super::cursor_shape_v1::wp_cursor_shape_device_v1::WpCursorShapeDeviceV1Error;
    #[cfg(feature = "protocol-cursor_shape_v1")]
    pub(super) use super::cursor_shape_v1::wp_cursor_shape_manager_v1::WpCursorShapeManagerV1;
    #[cfg(feature = "protocol-drm_lease_v1")]
    pub(super) use super::drm_lease_v1::wp_drm_lease_connector_v1::WpDrmLeaseConnectorV1;
    #[cfg(feature = "protocol-drm_lease_v1")]
    pub(super) use super::drm_lease_v1::wp_drm_lease_device_v1::WpDrmLeaseDeviceV1;
    #[cfg(feature = "protocol-drm_lease_v1")]
    pub(super) use super::drm_lease_v1::wp_drm_lease_request_v1::WpDrmLeaseRequestV1;
    #[cfg(feature = "protocol-drm_lease_v1")]
    pub(super) use super::drm_lease_v1::wp_drm_lease_request_v1::WpDrmLeaseRequestV1Error;
    #[cfg(feature = "protocol-drm_lease_v1")]
    pub(super) use super::drm_lease_v1::wp_drm_lease_v1::WpDrmLeaseV1;
    #[cfg(feature = "protocol-ext_background_effect_v1")]
    pub(super) use super::ext_background_effect_v1::ext_background_effect_manager_v1::ExtBackgroundEffectManagerV1;
    #[cfg(feature = "protocol-ext_background_effect_v1")]
    pub(super) use super::ext_background_effect_v1::ext_background_effect_manager_v1::ExtBackgroundEffectManagerV1Error;
    #[cfg(feature = "protocol-ext_background_effect_v1")]
    pub(super) use super::ext_background_effect_v1::ext_background_effect_manager_v1::ExtBackgroundEffectManagerV1Capability;
    #[cfg(feature = "protocol-ext_background_effect_v1")]
    pub(super) use super::ext_background_effect_v1::ext_background_effect_surface_v1::ExtBackgroundEffectSurfaceV1;
    #[cfg(feature = "protocol-ext_background_effect_v1")]
    pub(super) use super::ext_background_effect_v1::ext_background_effect_surface_v1::ExtBackgroundEffectSurfaceV1Error;
    #[cfg(feature = "protocol-ext_data_control_v1")]
    pub(super) use super::ext_data_control_v1::ext_data_control_device_v1::ExtDataControlDeviceV1;
    #[cfg(feature = "protocol-ext_data_control_v1")]
    pub(super) use super::ext_data_control_v1::ext_data_control_device_v1::ExtDataControlDeviceV1Error;
    #[cfg(feature = "protocol-ext_data_control_v1")]
    pub(super) use super::ext_data_control_v1::ext_data_control_manager_v1::ExtDataControlManagerV1;
    #[cfg(feature = "protocol-ext_data_control_v1")]
    pub(super) use super::ext_data_control_v1::ext_data_control_offer_v1::ExtDataControlOfferV1;
    #[cfg(feature = "protocol-ext_data_control_v1")]
    pub(super) use super::ext_data_control_v1::ext_data_control_source_v1::ExtDataControlSourceV1;
    #[cfg(feature = "protocol-ext_data_control_v1")]
    pub(super) use super::ext_data_control_v1::ext_data_control_source_v1::ExtDataControlSourceV1Error;
    #[cfg(feature = "protocol-ext_foreign_toplevel_list_v1")]
    pub(super) use super::ext_foreign_toplevel_list_v1::ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1;
    #[cfg(feature = "protocol-ext_foreign_toplevel_list_v1")]
    pub(super) use super::ext_foreign_toplevel_list_v1::ext_foreign_toplevel_list_v1::ExtForeignToplevelListV1;
    #[cfg(feature = "protocol-ext_idle_notify_v1")]
    pub(super) use super::ext_idle_notify_v1::ext_idle_notification_v1::ExtIdleNotificationV1;
    #[cfg(feature = "protocol-ext_idle_notify_v1")]
    pub(super) use super::ext_idle_notify_v1::ext_idle_notifier_v1::ExtIdleNotifierV1;
    #[cfg(feature = "protocol-ext_image_capture_source_v1")]
    pub(super) use super::ext_image_capture_source_v1::ext_foreign_toplevel_image_capture_source_manager_v1::ExtForeignToplevelImageCaptureSourceManagerV1;
    #[cfg(feature = "protocol-ext_image_capture_source_v1")]
    pub(super) use super::ext_image_capture_source_v1::ext_image_capture_source_v1::ExtImageCaptureSourceV1;
    #[cfg(feature = "protocol-ext_image_capture_source_v1")]
    pub(super) use super::ext_image_capture_source_v1::ext_output_image_capture_source_manager_v1::ExtOutputImageCaptureSourceManagerV1;
    #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_cursor_session_v1::ExtImageCopyCaptureCursorSessionV1;
    #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_cursor_session_v1::ExtImageCopyCaptureCursorSessionV1Error;
    #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_frame_v1::ExtImageCopyCaptureFrameV1;
    #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_frame_v1::ExtImageCopyCaptureFrameV1Error;
    #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_frame_v1::ExtImageCopyCaptureFrameV1FailureReason;
    #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_manager_v1::ExtImageCopyCaptureManagerV1;
    #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_manager_v1::ExtImageCopyCaptureManagerV1Error;
    #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_manager_v1::ExtImageCopyCaptureManagerV1Options;
    #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_session_v1::ExtImageCopyCaptureSessionV1;
    #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_session_v1::ExtImageCopyCaptureSessionV1Error;
    #[cfg(feature = "protocol-ext_session_lock_v1")]
    pub(super) use super::ext_session_lock_v1::ext_session_lock_manager_v1::ExtSessionLockManagerV1;
    #[cfg(feature = "protocol-ext_session_lock_v1")]
    pub(super) use super::ext_session_lock_v1::ext_session_lock_surface_v1::ExtSessionLockSurfaceV1;
    #[cfg(feature = "protocol-ext_session_lock_v1")]
    pub(super) use super::ext_session_lock_v1::ext_session_lock_surface_v1::ExtSessionLockSurfaceV1Error;
    #[cfg(feature = "protocol-ext_session_lock_v1")]
    pub(super) use super::ext_session_lock_v1::ext_session_lock_v1::ExtSessionLockV1;
    #[cfg(feature = "protocol-ext_session_lock_v1")]
    pub(super) use super::ext_session_lock_v1::ext_session_lock_v1::ExtSessionLockV1Error;
    #[cfg(feature = "protocol-ext_transient_seat_v1")]
    pub(super) use super::ext_transient_seat_v1::ext_transient_seat_manager_v1::ExtTransientSeatManagerV1;
    #[cfg(feature = "protocol-ext_transient_seat_v1")]
    pub(super) use super::ext_transient_seat_v1::ext_transient_seat_v1::ExtTransientSeatV1;
    #[cfg(feature = "protocol-ext_workspace_v1")]
    pub(super) use super::ext_workspace_v1::ext_workspace_group_handle_v1::ExtWorkspaceGroupHandleV1;
    #[cfg(feature = "protocol-ext_workspace_v1")]
    pub(super) use super::ext_workspace_v1::ext_workspace_group_handle_v1::ExtWorkspaceGroupHandleV1GroupCapabilities;
    #[cfg(feature = "protocol-ext_workspace_v1")]
    pub(super) use super::ext_workspace_v1::ext_workspace_handle_v1::ExtWorkspaceHandleV1;
    #[cfg(feature = "protocol-ext_workspace_v1")]
    pub(super) use super::ext_workspace_v1::ext_workspace_handle_v1::ExtWorkspaceHandleV1State;
    #[cfg(feature = "protocol-ext_workspace_v1")]
    pub(super) use super::ext_workspace_v1::ext_workspace_handle_v1::ExtWorkspaceHandleV1WorkspaceCapabilities;
    #[cfg(feature = "protocol-ext_workspace_v1")]
    pub(super) use super::ext_workspace_v1::ext_workspace_manager_v1::ExtWorkspaceManagerV1;
    #[cfg(feature = "protocol-fifo_v1")]
    pub(super) use super::fifo_v1::wp_fifo_manager_v1::WpFifoManagerV1;
    #[cfg(feature = "protocol-fifo_v1")]
    pub(super) use super::fifo_v1::wp_fifo_manager_v1::WpFifoManagerV1Error;
    #[cfg(feature = "protocol-fifo_v1")]
    pub(super) use super::fifo_v1::wp_fifo_v1::WpFifoV1;
    #[cfg(feature = "protocol-fifo_v1")]
    pub(super) use super::fifo_v1::wp_fifo_v1::WpFifoV1Error;
    #[cfg(feature = "protocol-fractional_scale_v1")]
    pub(super) use super::fractional_scale_v1::wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1;
    #[cfg(feature = "protocol-fractional_scale_v1")]
    pub(super) use super::fractional_scale_v1::wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1Error;
    #[cfg(feature = "protocol-fractional_scale_v1")]
    pub(super) use super::fractional_scale_v1::wp_fractional_scale_v1::WpFractionalScaleV1;
    #[cfg(feature = "protocol-fullscreen_shell_unstable_v1")]
    pub(super) use super::fullscreen_shell_unstable_v1::zwp_fullscreen_shell_mode_feedback_v1::ZwpFullscreenShellModeFeedbackV1;
    #[cfg(feature = "protocol-fullscreen_shell_unstable_v1")]
    pub(super) use super::fullscreen_shell_unstable_v1::zwp_fullscreen_shell_v1::ZwpFullscreenShellV1;
    #[cfg(feature = "protocol-fullscreen_shell_unstable_v1")]
    pub(super) use super::fullscreen_shell_unstable_v1::zwp_fullscreen_shell_v1::ZwpFullscreenShellV1Capability;
    #[cfg(feature = "protocol-fullscreen_shell_unstable_v1")]
    pub(super) use super::fullscreen_shell_unstable_v1::zwp_fullscreen_shell_v1::ZwpFullscreenShellV1PresentMethod;
    #[cfg(feature = "protocol-fullscreen_shell_unstable_v1")]
    pub(super) use super::fullscreen_shell_unstable_v1::zwp_fullscreen_shell_v1::ZwpFullscreenShellV1Error;
    #[cfg(feature = "protocol-idle_inhibit_unstable_v1")]
    pub(super) use super::idle_inhibit_unstable_v1::zwp_idle_inhibit_manager_v1::ZwpIdleInhibitManagerV1;
    #[cfg(feature = "protocol-idle_inhibit_unstable_v1")]
    pub(super) use super::idle_inhibit_unstable_v1::zwp_idle_inhibitor_v1::ZwpIdleInhibitorV1;
    #[cfg(feature = "protocol-input_method_unstable_v1")]
    pub(super) use super::input_method_unstable_v1::zwp_input_method_context_v1::ZwpInputMethodContextV1;
    #[cfg(feature = "protocol-input_method_unstable_v1")]
    pub(super) use super::input_method_unstable_v1::zwp_input_method_v1::ZwpInputMethodV1;
    #[cfg(feature = "protocol-input_method_unstable_v1")]
    pub(super) use super::input_method_unstable_v1::zwp_input_panel_surface_v1::ZwpInputPanelSurfaceV1;
    #[cfg(feature = "protocol-input_method_unstable_v1")]
    pub(super) use super::input_method_unstable_v1::zwp_input_panel_surface_v1::ZwpInputPanelSurfaceV1Position;
    #[cfg(feature = "protocol-input_method_unstable_v1")]
    pub(super) use super::input_method_unstable_v1::zwp_input_panel_v1::ZwpInputPanelV1;
    #[cfg(feature = "protocol-input_timestamps_unstable_v1")]
    pub(super) use super::input_timestamps_unstable_v1::zwp_input_timestamps_manager_v1::ZwpInputTimestampsManagerV1;
    #[cfg(feature = "protocol-input_timestamps_unstable_v1")]
    pub(super) use super::input_timestamps_unstable_v1::zwp_input_timestamps_v1::ZwpInputTimestampsV1;
    #[cfg(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1")]
    pub(super) use super::keyboard_shortcuts_inhibit_unstable_v1::zwp_keyboard_shortcuts_inhibit_manager_v1::ZwpKeyboardShortcutsInhibitManagerV1;
    #[cfg(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1")]
    pub(super) use super::keyboard_shortcuts_inhibit_unstable_v1::zwp_keyboard_shortcuts_inhibit_manager_v1::ZwpKeyboardShortcutsInhibitManagerV1Error;
    #[cfg(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1")]
    pub(super) use super::keyboard_shortcuts_inhibit_unstable_v1::zwp_keyboard_shortcuts_inhibitor_v1::ZwpKeyboardShortcutsInhibitorV1;
    #[cfg(feature = "protocol-linux_dmabuf_v1")]
    pub(super) use super::linux_dmabuf_v1::zwp_linux_buffer_params_v1::ZwpLinuxBufferParamsV1;
    #[cfg(feature = "protocol-linux_dmabuf_v1")]
    pub(super) use super::linux_dmabuf_v1::zwp_linux_buffer_params_v1::ZwpLinuxBufferParamsV1Error;
    #[cfg(feature = "protocol-linux_dmabuf_v1")]
    pub(super) use super::linux_dmabuf_v1::zwp_linux_buffer_params_v1::ZwpLinuxBufferParamsV1Flags;
    #[cfg(feature = "protocol-linux_dmabuf_v1")]
    pub(super) use super::linux_dmabuf_v1::zwp_linux_dmabuf_feedback_v1::ZwpLinuxDmabufFeedbackV1;
    #[cfg(feature = "protocol-linux_dmabuf_v1")]
    pub(super) use super::linux_dmabuf_v1::zwp_linux_dmabuf_feedback_v1::ZwpLinuxDmabufFeedbackV1TrancheFlags;
    #[cfg(feature = "protocol-linux_dmabuf_v1")]
    pub(super) use super::linux_dmabuf_v1::zwp_linux_dmabuf_v1::ZwpLinuxDmabufV1;
    #[cfg(feature = "protocol-linux_drm_syncobj_v1")]
    pub(super) use super::linux_drm_syncobj_v1::wp_linux_drm_syncobj_manager_v1::WpLinuxDrmSyncobjManagerV1;
    #[cfg(feature = "protocol-linux_drm_syncobj_v1")]
    pub(super) use super::linux_drm_syncobj_v1::wp_linux_drm_syncobj_manager_v1::WpLinuxDrmSyncobjManagerV1Error;
    #[cfg(feature = "protocol-linux_drm_syncobj_v1")]
    pub(super) use super::linux_drm_syncobj_v1::wp_linux_drm_syncobj_surface_v1::WpLinuxDrmSyncobjSurfaceV1;
    #[cfg(feature = "protocol-linux_drm_syncobj_v1")]
    pub(super) use super::linux_drm_syncobj_v1::wp_linux_drm_syncobj_surface_v1::WpLinuxDrmSyncobjSurfaceV1Error;
    #[cfg(feature = "protocol-linux_drm_syncobj_v1")]
    pub(super) use super::linux_drm_syncobj_v1::wp_linux_drm_syncobj_timeline_v1::WpLinuxDrmSyncobjTimelineV1;
    #[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
    pub(super) use super::pointer_constraints_unstable_v1::zwp_confined_pointer_v1::ZwpConfinedPointerV1;
    #[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
    pub(super) use super::pointer_constraints_unstable_v1::zwp_locked_pointer_v1::ZwpLockedPointerV1;
    #[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
    pub(super) use super::pointer_constraints_unstable_v1::zwp_pointer_constraints_v1::ZwpPointerConstraintsV1;
    #[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
    pub(super) use super::pointer_constraints_unstable_v1::zwp_pointer_constraints_v1::ZwpPointerConstraintsV1Error;
    #[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
    pub(super) use super::pointer_constraints_unstable_v1::zwp_pointer_constraints_v1::ZwpPointerConstraintsV1Lifetime;
    #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
    pub(super) use super::pointer_gestures_unstable_v1::zwp_pointer_gesture_hold_v1::ZwpPointerGestureHoldV1;
    #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
    pub(super) use super::pointer_gestures_unstable_v1::zwp_pointer_gesture_pinch_v1::ZwpPointerGesturePinchV1;
    #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
    pub(super) use super::pointer_gestures_unstable_v1::zwp_pointer_gesture_swipe_v1::ZwpPointerGestureSwipeV1;
    #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
    pub(super) use super::pointer_gestures_unstable_v1::zwp_pointer_gestures_v1::ZwpPointerGesturesV1;
    #[cfg(feature = "protocol-pointer_warp_v1")]
    pub(super) use super::pointer_warp_v1::wp_pointer_warp_v1::WpPointerWarpV1;
    #[cfg(feature = "protocol-presentation_time")]
    pub(super) use super::presentation_time::wp_presentation::WpPresentation;
    #[cfg(feature = "protocol-presentation_time")]
    pub(super) use super::presentation_time::wp_presentation::WpPresentationError;
    #[cfg(feature = "protocol-presentation_time")]
    pub(super) use super::presentation_time::wp_presentation_feedback::WpPresentationFeedback;
    #[cfg(feature = "protocol-presentation_time")]
    pub(super) use super::presentation_time::wp_presentation_feedback::WpPresentationFeedbackKind;
    #[cfg(feature = "protocol-relative_pointer_unstable_v1")]
    pub(super) use super::relative_pointer_unstable_v1::zwp_relative_pointer_manager_v1::ZwpRelativePointerManagerV1;
    #[cfg(feature = "protocol-relative_pointer_unstable_v1")]
    pub(super) use super::relative_pointer_unstable_v1::zwp_relative_pointer_v1::ZwpRelativePointerV1;
    #[cfg(feature = "protocol-security_context_v1")]
    pub(super) use super::security_context_v1::wp_security_context_manager_v1::WpSecurityContextManagerV1;
    #[cfg(feature = "protocol-security_context_v1")]
    pub(super) use super::security_context_v1::wp_security_context_manager_v1::WpSecurityContextManagerV1Error;
    #[cfg(feature = "protocol-security_context_v1")]
    pub(super) use super::security_context_v1::wp_security_context_v1::WpSecurityContextV1;
    #[cfg(feature = "protocol-security_context_v1")]
    pub(super) use super::security_context_v1::wp_security_context_v1::WpSecurityContextV1Error;
    #[cfg(feature = "protocol-single_pixel_buffer_v1")]
    pub(super) use super::single_pixel_buffer_v1::wp_single_pixel_buffer_manager_v1::WpSinglePixelBufferManagerV1;
    #[cfg(feature = "protocol-tablet_v2")]
    pub(super) use super::tablet_v2::zwp_tablet_manager_v2::ZwpTabletManagerV2;
    #[cfg(feature = "protocol-tablet_v2")]
    pub(super) use super::tablet_v2::zwp_tablet_pad_dial_v2::ZwpTabletPadDialV2;
    #[cfg(feature = "protocol-tablet_v2")]
    pub(super) use super::tablet_v2::zwp_tablet_pad_group_v2::ZwpTabletPadGroupV2;
    #[cfg(feature = "protocol-tablet_v2")]
    pub(super) use super::tablet_v2::zwp_tablet_pad_ring_v2::ZwpTabletPadRingV2;
    #[cfg(feature = "protocol-tablet_v2")]
    pub(super) use super::tablet_v2::zwp_tablet_pad_ring_v2::ZwpTabletPadRingV2Source;
    #[cfg(feature = "protocol-tablet_v2")]
    pub(super) use super::tablet_v2::zwp_tablet_pad_strip_v2::ZwpTabletPadStripV2;
    #[cfg(feature = "protocol-tablet_v2")]
    pub(super) use super::tablet_v2::zwp_tablet_pad_strip_v2::ZwpTabletPadStripV2Source;
    #[cfg(feature = "protocol-tablet_v2")]
    pub(super) use super::tablet_v2::zwp_tablet_pad_v2::ZwpTabletPadV2;
    #[cfg(feature = "protocol-tablet_v2")]
    pub(super) use super::tablet_v2::zwp_tablet_pad_v2::ZwpTabletPadV2ButtonState;
    #[cfg(feature = "protocol-tablet_v2")]
    pub(super) use super::tablet_v2::zwp_tablet_seat_v2::ZwpTabletSeatV2;
    #[cfg(feature = "protocol-tablet_v2")]
    pub(super) use super::tablet_v2::zwp_tablet_tool_v2::ZwpTabletToolV2;
    #[cfg(feature = "protocol-tablet_v2")]
    pub(super) use super::tablet_v2::zwp_tablet_tool_v2::ZwpTabletToolV2Type;
    #[cfg(feature = "protocol-tablet_v2")]
    pub(super) use super::tablet_v2::zwp_tablet_tool_v2::ZwpTabletToolV2Capability;
    #[cfg(feature = "protocol-tablet_v2")]
    pub(super) use super::tablet_v2::zwp_tablet_tool_v2::ZwpTabletToolV2ButtonState;
    #[cfg(feature = "protocol-tablet_v2")]
    pub(super) use super::tablet_v2::zwp_tablet_tool_v2::ZwpTabletToolV2Error;
    #[cfg(feature = "protocol-tablet_v2")]
    pub(super) use super::tablet_v2::zwp_tablet_v2::ZwpTabletV2;
    #[cfg(feature = "protocol-tablet_v2")]
    pub(super) use super::tablet_v2::zwp_tablet_v2::ZwpTabletV2Bustype;
    #[cfg(feature = "protocol-tearing_control_v1")]
    pub(super) use super::tearing_control_v1::wp_tearing_control_manager_v1::WpTearingControlManagerV1;
    #[cfg(feature = "protocol-tearing_control_v1")]
    pub(super) use super::tearing_control_v1::wp_tearing_control_manager_v1::WpTearingControlManagerV1Error;
    #[cfg(feature = "protocol-tearing_control_v1")]
    pub(super) use super::tearing_control_v1::wp_tearing_control_v1::WpTearingControlV1;
    #[cfg(feature = "protocol-tearing_control_v1")]
    pub(super) use super::tearing_control_v1::wp_tearing_control_v1::WpTearingControlV1PresentationHint;
    #[cfg(feature = "protocol-text_input_unstable_v1")]
    pub(super) use super::text_input_unstable_v1::zwp_text_input_manager_v1::ZwpTextInputManagerV1;
    #[cfg(feature = "protocol-text_input_unstable_v1")]
    pub(super) use super::text_input_unstable_v1::zwp_text_input_v1::ZwpTextInputV1;
    #[cfg(feature = "protocol-text_input_unstable_v1")]
    pub(super) use super::text_input_unstable_v1::zwp_text_input_v1::ZwpTextInputV1ContentHint;
    #[cfg(feature = "protocol-text_input_unstable_v1")]
    pub(super) use super::text_input_unstable_v1::zwp_text_input_v1::ZwpTextInputV1ContentPurpose;
    #[cfg(feature = "protocol-text_input_unstable_v1")]
    pub(super) use super::text_input_unstable_v1::zwp_text_input_v1::ZwpTextInputV1PreeditStyle;
    #[cfg(feature = "protocol-text_input_unstable_v1")]
    pub(super) use super::text_input_unstable_v1::zwp_text_input_v1::ZwpTextInputV1TextDirection;
    #[cfg(feature = "protocol-text_input_unstable_v3")]
    pub(super) use super::text_input_unstable_v3::zwp_text_input_manager_v3::ZwpTextInputManagerV3;
    #[cfg(feature = "protocol-text_input_unstable_v3")]
    pub(super) use super::text_input_unstable_v3::zwp_text_input_v3::ZwpTextInputV3;
    #[cfg(feature = "protocol-text_input_unstable_v3")]
    pub(super) use super::text_input_unstable_v3::zwp_text_input_v3::ZwpTextInputV3ChangeCause;
    #[cfg(feature = "protocol-text_input_unstable_v3")]
    pub(super) use super::text_input_unstable_v3::zwp_text_input_v3::ZwpTextInputV3ContentHint;
    #[cfg(feature = "protocol-text_input_unstable_v3")]
    pub(super) use super::text_input_unstable_v3::zwp_text_input_v3::ZwpTextInputV3ContentPurpose;
    #[cfg(feature = "protocol-viewporter")]
    pub(super) use super::viewporter::wp_viewport::WpViewport;
    #[cfg(feature = "protocol-viewporter")]
    pub(super) use super::viewporter::wp_viewport::WpViewportError;
    #[cfg(feature = "protocol-viewporter")]
    pub(super) use super::viewporter::wp_viewporter::WpViewporter;
    #[cfg(feature = "protocol-viewporter")]
    pub(super) use super::viewporter::wp_viewporter::WpViewporterError;
    #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
    pub(super) use super::wp_primary_selection_unstable_v1::zwp_primary_selection_device_manager_v1::ZwpPrimarySelectionDeviceManagerV1;
    #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
    pub(super) use super::wp_primary_selection_unstable_v1::zwp_primary_selection_device_v1::ZwpPrimarySelectionDeviceV1;
    #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
    pub(super) use super::wp_primary_selection_unstable_v1::zwp_primary_selection_offer_v1::ZwpPrimarySelectionOfferV1;
    #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
    pub(super) use super::wp_primary_selection_unstable_v1::zwp_primary_selection_source_v1::ZwpPrimarySelectionSourceV1;
    #[cfg(feature = "protocol-xdg_activation_v1")]
    pub(super) use super::xdg_activation_v1::xdg_activation_token_v1::XdgActivationTokenV1;
    #[cfg(feature = "protocol-xdg_activation_v1")]
    pub(super) use super::xdg_activation_v1::xdg_activation_token_v1::XdgActivationTokenV1Error;
    #[cfg(feature = "protocol-xdg_activation_v1")]
    pub(super) use super::xdg_activation_v1::xdg_activation_v1::XdgActivationV1;
    #[cfg(feature = "protocol-xdg_decoration_unstable_v1")]
    pub(super) use super::xdg_decoration_unstable_v1::zxdg_decoration_manager_v1::ZxdgDecorationManagerV1;
    #[cfg(feature = "protocol-xdg_decoration_unstable_v1")]
    pub(super) use super::xdg_decoration_unstable_v1::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1;
    #[cfg(feature = "protocol-xdg_decoration_unstable_v1")]
    pub(super) use super::xdg_decoration_unstable_v1::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1Error;
    #[cfg(feature = "protocol-xdg_decoration_unstable_v1")]
    pub(super) use super::xdg_decoration_unstable_v1::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1Mode;
    #[cfg(feature = "protocol-xdg_dialog_v1")]
    pub(super) use super::xdg_dialog_v1::xdg_dialog_v1::XdgDialogV1;
    #[cfg(feature = "protocol-xdg_dialog_v1")]
    pub(super) use super::xdg_dialog_v1::xdg_wm_dialog_v1::XdgWmDialogV1;
    #[cfg(feature = "protocol-xdg_dialog_v1")]
    pub(super) use super::xdg_dialog_v1::xdg_wm_dialog_v1::XdgWmDialogV1Error;
    #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_exported_v2::ZxdgExportedV2;
    #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_exporter_v2::ZxdgExporterV2;
    #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_exporter_v2::ZxdgExporterV2Error;
    #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_imported_v2::ZxdgImportedV2;
    #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_imported_v2::ZxdgImportedV2Error;
    #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_importer_v2::ZxdgImporterV2;
    #[cfg(feature = "protocol-xdg_output_unstable_v1")]
    pub(super) use super::xdg_output_unstable_v1::zxdg_output_manager_v1::ZxdgOutputManagerV1;
    #[cfg(feature = "protocol-xdg_output_unstable_v1")]
    pub(super) use super::xdg_output_unstable_v1::zxdg_output_v1::ZxdgOutputV1;
    #[cfg(feature = "protocol-xdg_shell")]
    pub(super) use super::xdg_shell::xdg_popup::XdgPopup;
    #[cfg(feature = "protocol-xdg_shell")]
    pub(super) use super::xdg_shell::xdg_popup::XdgPopupError;
    #[cfg(feature = "protocol-xdg_shell")]
    pub(super) use super::xdg_shell::xdg_positioner::XdgPositioner;
    #[cfg(feature = "protocol-xdg_shell")]
    pub(super) use super::xdg_shell::xdg_positioner::XdgPositionerError;
    #[cfg(feature = "protocol-xdg_shell")]
    pub(super) use super::xdg_shell::xdg_positioner::XdgPositionerAnchor;
    #[cfg(feature = "protocol-xdg_shell")]
    pub(super) use super::xdg_shell::xdg_positioner::XdgPositionerGravity;
    #[cfg(feature = "protocol-xdg_shell")]
    pub(super) use super::xdg_shell::xdg_positioner::XdgPositionerConstraintAdjustment;
    #[cfg(feature = "protocol-xdg_shell")]
    pub(super) use super::xdg_shell::xdg_surface::XdgSurface;
    #[cfg(feature = "protocol-xdg_shell")]
    pub(super) use super::xdg_shell::xdg_surface::XdgSurfaceError;
    #[cfg(feature = "protocol-xdg_shell")]
    pub(super) use super::xdg_shell::xdg_toplevel::XdgToplevel;
    #[cfg(feature = "protocol-xdg_shell")]
    pub(super) use super::xdg_shell::xdg_toplevel::XdgToplevelError;
    #[cfg(feature = "protocol-xdg_shell")]
    pub(super) use super::xdg_shell::xdg_toplevel::XdgToplevelResizeEdge;
    #[cfg(feature = "protocol-xdg_shell")]
    pub(super) use super::xdg_shell::xdg_toplevel::XdgToplevelState;
    #[cfg(feature = "protocol-xdg_shell")]
    pub(super) use super::xdg_shell::xdg_toplevel::XdgToplevelWmCapabilities;
    #[cfg(feature = "protocol-xdg_shell")]
    pub(super) use super::xdg_shell::xdg_wm_base::XdgWmBase;
    #[cfg(feature = "protocol-xdg_shell")]
    pub(super) use super::xdg_shell::xdg_wm_base::XdgWmBaseError;
    #[cfg(feature = "protocol-xdg_system_bell_v1")]
    pub(super) use super::xdg_system_bell_v1::xdg_system_bell_v1::XdgSystemBellV1;
    #[cfg(feature = "protocol-xdg_toplevel_drag_v1")]
    pub(super) use super::xdg_toplevel_drag_v1::xdg_toplevel_drag_manager_v1::XdgToplevelDragManagerV1;
    #[cfg(feature = "protocol-xdg_toplevel_drag_v1")]
    pub(super) use super::xdg_toplevel_drag_v1::xdg_toplevel_drag_manager_v1::XdgToplevelDragManagerV1Error;
    #[cfg(feature = "protocol-xdg_toplevel_drag_v1")]
    pub(super) use super::xdg_toplevel_drag_v1::xdg_toplevel_drag_v1::XdgToplevelDragV1;
    #[cfg(feature = "protocol-xdg_toplevel_drag_v1")]
    pub(super) use super::xdg_toplevel_drag_v1::xdg_toplevel_drag_v1::XdgToplevelDragV1Error;
    #[cfg(feature = "protocol-xdg_toplevel_icon_v1")]
    pub(super) use super::xdg_toplevel_icon_v1::xdg_toplevel_icon_manager_v1::XdgToplevelIconManagerV1;
    #[cfg(feature = "protocol-xdg_toplevel_icon_v1")]
    pub(super) use super::xdg_toplevel_icon_v1::xdg_toplevel_icon_v1::XdgToplevelIconV1;
    #[cfg(feature = "protocol-xdg_toplevel_icon_v1")]
    pub(super) use super::xdg_toplevel_icon_v1::xdg_toplevel_icon_v1::XdgToplevelIconV1Error;
    #[cfg(feature = "protocol-xdg_toplevel_tag_v1")]
    pub(super) use super::xdg_toplevel_tag_v1::xdg_toplevel_tag_manager_v1::XdgToplevelTagManagerV1;
    #[cfg(feature = "protocol-xwayland_keyboard_grab_unstable_v1")]
    pub(super) use super::xwayland_keyboard_grab_unstable_v1::zwp_xwayland_keyboard_grab_manager_v1::ZwpXwaylandKeyboardGrabManagerV1;
    #[cfg(feature = "protocol-xwayland_keyboard_grab_unstable_v1")]
    pub(super) use super::xwayland_keyboard_grab_unstable_v1::zwp_xwayland_keyboard_grab_v1::ZwpXwaylandKeyboardGrabV1;
    #[cfg(feature = "protocol-xwayland_shell_v1")]
    pub(super) use super::xwayland_shell_v1::xwayland_shell_v1::XwaylandShellV1;
    #[cfg(feature = "protocol-xwayland_shell_v1")]
    pub(super) use super::xwayland_shell_v1::xwayland_shell_v1::XwaylandShellV1Error;
    #[cfg(feature = "protocol-xwayland_shell_v1")]
    pub(super) use super::xwayland_shell_v1::xwayland_surface_v1::XwaylandSurfaceV1;
    #[cfg(feature = "protocol-xwayland_shell_v1")]
    pub(super) use super::xwayland_shell_v1::xwayland_surface_v1::XwaylandSurfaceV1Error;
    #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
    pub(super) use super::zwp_linux_explicit_synchronization_unstable_v1::zwp_linux_buffer_release_v1::ZwpLinuxBufferReleaseV1;
    #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
    pub(super) use super::zwp_linux_explicit_synchronization_unstable_v1::zwp_linux_explicit_synchronization_v1::ZwpLinuxExplicitSynchronizationV1;
    #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
    pub(super) use super::zwp_linux_explicit_synchronization_unstable_v1::zwp_linux_explicit_synchronization_v1::ZwpLinuxExplicitSynchronizationV1Error;
    #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
    pub(super) use super::zwp_linux_explicit_synchronization_unstable_v1::zwp_linux_surface_synchronization_v1::ZwpLinuxSurfaceSynchronizationV1;
    #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
    pub(super) use super::zwp_linux_explicit_synchronization_unstable_v1::zwp_linux_surface_synchronization_v1::ZwpLinuxSurfaceSynchronizationV1Error;
    #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_device_v1::ZwlrDataControlDeviceV1;
    #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_device_v1::ZwlrDataControlDeviceV1Error;
    #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_manager_v1::ZwlrDataControlManagerV1;
    #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_offer_v1::ZwlrDataControlOfferV1;
    #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_source_v1::ZwlrDataControlSourceV1;
    #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_source_v1::ZwlrDataControlSourceV1Error;
    #[cfg(feature = "protocol-wlr_export_dmabuf_unstable_v1")]
    pub(super) use super::wlr_export_dmabuf_unstable_v1::zwlr_export_dmabuf_frame_v1::ZwlrExportDmabufFrameV1;
    #[cfg(feature = "protocol-wlr_export_dmabuf_unstable_v1")]
    pub(super) use super::wlr_export_dmabuf_unstable_v1::zwlr_export_dmabuf_frame_v1::ZwlrExportDmabufFrameV1Flags;
    #[cfg(feature = "protocol-wlr_export_dmabuf_unstable_v1")]
    pub(super) use super::wlr_export_dmabuf_unstable_v1::zwlr_export_dmabuf_frame_v1::ZwlrExportDmabufFrameV1CancelReason;
    #[cfg(feature = "protocol-wlr_export_dmabuf_unstable_v1")]
    pub(super) use super::wlr_export_dmabuf_unstable_v1::zwlr_export_dmabuf_manager_v1::ZwlrExportDmabufManagerV1;
    #[cfg(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1")]
    pub(super) use super::wlr_foreign_toplevel_management_unstable_v1::zwlr_foreign_toplevel_handle_v1::ZwlrForeignToplevelHandleV1;
    #[cfg(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1")]
    pub(super) use super::wlr_foreign_toplevel_management_unstable_v1::zwlr_foreign_toplevel_handle_v1::ZwlrForeignToplevelHandleV1State;
    #[cfg(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1")]
    pub(super) use super::wlr_foreign_toplevel_management_unstable_v1::zwlr_foreign_toplevel_handle_v1::ZwlrForeignToplevelHandleV1Error;
    #[cfg(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1")]
    pub(super) use super::wlr_foreign_toplevel_management_unstable_v1::zwlr_foreign_toplevel_manager_v1::ZwlrForeignToplevelManagerV1;
    #[cfg(feature = "protocol-wlr_gamma_control_unstable_v1")]
    pub(super) use super::wlr_gamma_control_unstable_v1::zwlr_gamma_control_manager_v1::ZwlrGammaControlManagerV1;
    #[cfg(feature = "protocol-wlr_gamma_control_unstable_v1")]
    pub(super) use super::wlr_gamma_control_unstable_v1::zwlr_gamma_control_v1::ZwlrGammaControlV1;
    #[cfg(feature = "protocol-wlr_gamma_control_unstable_v1")]
    pub(super) use super::wlr_gamma_control_unstable_v1::zwlr_gamma_control_v1::ZwlrGammaControlV1Error;
    #[cfg(feature = "protocol-wlr_input_inhibit_unstable_v1")]
    pub(super) use super::wlr_input_inhibit_unstable_v1::zwlr_input_inhibit_manager_v1::ZwlrInputInhibitManagerV1;
    #[cfg(feature = "protocol-wlr_input_inhibit_unstable_v1")]
    pub(super) use super::wlr_input_inhibit_unstable_v1::zwlr_input_inhibit_manager_v1::ZwlrInputInhibitManagerV1Error;
    #[cfg(feature = "protocol-wlr_input_inhibit_unstable_v1")]
    pub(super) use super::wlr_input_inhibit_unstable_v1::zwlr_input_inhibitor_v1::ZwlrInputInhibitorV1;
    #[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")]
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_shell_v1::ZwlrLayerShellV1;
    #[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")]
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_shell_v1::ZwlrLayerShellV1Error;
    #[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")]
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_shell_v1::ZwlrLayerShellV1Layer;
    #[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")]
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_surface_v1::ZwlrLayerSurfaceV1;
    #[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")]
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_surface_v1::ZwlrLayerSurfaceV1KeyboardInteractivity;
    #[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")]
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_surface_v1::ZwlrLayerSurfaceV1Error;
    #[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")]
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_surface_v1::ZwlrLayerSurfaceV1Anchor;
    #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_configuration_head_v1::ZwlrOutputConfigurationHeadV1;
    #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_configuration_head_v1::ZwlrOutputConfigurationHeadV1Error;
    #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_configuration_v1::ZwlrOutputConfigurationV1;
    #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_configuration_v1::ZwlrOutputConfigurationV1Error;
    #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_head_v1::ZwlrOutputHeadV1;
    #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_head_v1::ZwlrOutputHeadV1AdaptiveSyncState;
    #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_manager_v1::ZwlrOutputManagerV1;
    #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_mode_v1::ZwlrOutputModeV1;
    #[cfg(feature = "protocol-wlr_output_power_management_unstable_v1")]
    pub(super) use super::wlr_output_power_management_unstable_v1::zwlr_output_power_manager_v1::ZwlrOutputPowerManagerV1;
    #[cfg(feature = "protocol-wlr_output_power_management_unstable_v1")]
    pub(super) use super::wlr_output_power_management_unstable_v1::zwlr_output_power_v1::ZwlrOutputPowerV1;
    #[cfg(feature = "protocol-wlr_output_power_management_unstable_v1")]
    pub(super) use super::wlr_output_power_management_unstable_v1::zwlr_output_power_v1::ZwlrOutputPowerV1Mode;
    #[cfg(feature = "protocol-wlr_output_power_management_unstable_v1")]
    pub(super) use super::wlr_output_power_management_unstable_v1::zwlr_output_power_v1::ZwlrOutputPowerV1Error;
    #[cfg(feature = "protocol-wlr_screencopy_unstable_v1")]
    pub(super) use super::wlr_screencopy_unstable_v1::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1;
    #[cfg(feature = "protocol-wlr_screencopy_unstable_v1")]
    pub(super) use super::wlr_screencopy_unstable_v1::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1Error;
    #[cfg(feature = "protocol-wlr_screencopy_unstable_v1")]
    pub(super) use super::wlr_screencopy_unstable_v1::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1Flags;
    #[cfg(feature = "protocol-wlr_screencopy_unstable_v1")]
    pub(super) use super::wlr_screencopy_unstable_v1::zwlr_screencopy_manager_v1::ZwlrScreencopyManagerV1;
    #[cfg(feature = "protocol-wlr_virtual_pointer_unstable_v1")]
    pub(super) use super::wlr_virtual_pointer_unstable_v1::zwlr_virtual_pointer_manager_v1::ZwlrVirtualPointerManagerV1;
    #[cfg(feature = "protocol-wlr_virtual_pointer_unstable_v1")]
    pub(super) use super::wlr_virtual_pointer_unstable_v1::zwlr_virtual_pointer_v1::ZwlrVirtualPointerV1;
    #[cfg(feature = "protocol-wlr_virtual_pointer_unstable_v1")]
    pub(super) use super::wlr_virtual_pointer_unstable_v1::zwlr_virtual_pointer_v1::ZwlrVirtualPointerV1Error;

    use crate::protocol_helpers::prelude::*;

    pub(super) fn create_proxy_for_interface(state: &Rc<State>, interface: &str, version: u32) -> Result<Rc<dyn Proxy>, ObjectError> {
        proxy_interface(interface)
            .ok_or(ObjectError::UnsupportedInterface(interface.to_string()))
            .and_then(|i| i.create_proxy(state, version))
    }

    pub(super) fn proxy_interface(interface: &str) -> Option<ProxyInterface> {
        static INTERFACES: phf::Map<&'static str, Option<ProxyInterface>> = phf::phf_map! {
            "hyprland_ctm_control_manager_v1" => {
                #[cfg(feature = "protocol-hyprland_ctm_control_v1")] { Some(ProxyInterface::HyprlandCtmControlManagerV1) }
                #[cfg(not(feature = "protocol-hyprland_ctm_control_v1"))] { None }
            },
            "hyprland_focus_grab_manager_v1" => {
                #[cfg(feature = "protocol-hyprland_focus_grab_v1")] { Some(ProxyInterface::HyprlandFocusGrabManagerV1) }
                #[cfg(not(feature = "protocol-hyprland_focus_grab_v1"))] { None }
            },
            "hyprland_focus_grab_v1" => {
                #[cfg(feature = "protocol-hyprland_focus_grab_v1")] { Some(ProxyInterface::HyprlandFocusGrabV1) }
                #[cfg(not(feature = "protocol-hyprland_focus_grab_v1"))] { None }
            },
            "hyprland_global_shortcut_v1" => {
                #[cfg(feature = "protocol-hyprland_global_shortcuts_v1")] { Some(ProxyInterface::HyprlandGlobalShortcutV1) }
                #[cfg(not(feature = "protocol-hyprland_global_shortcuts_v1"))] { None }
            },
            "hyprland_global_shortcuts_manager_v1" => {
                #[cfg(feature = "protocol-hyprland_global_shortcuts_v1")] { Some(ProxyInterface::HyprlandGlobalShortcutsManagerV1) }
                #[cfg(not(feature = "protocol-hyprland_global_shortcuts_v1"))] { None }
            },
            "hyprland_input_capture_manager_v1" => {
                #[cfg(feature = "protocol-hyprland_input_capture_v1")] { Some(ProxyInterface::HyprlandInputCaptureManagerV1) }
                #[cfg(not(feature = "protocol-hyprland_input_capture_v1"))] { None }
            },
            "hyprland_input_capture_v1" => {
                #[cfg(feature = "protocol-hyprland_input_capture_v1")] { Some(ProxyInterface::HyprlandInputCaptureV1) }
                #[cfg(not(feature = "protocol-hyprland_input_capture_v1"))] { None }
            },
            "hyprland_lock_notification_v1" => {
                #[cfg(feature = "protocol-hyprland_lock_notify_v1")] { Some(ProxyInterface::HyprlandLockNotificationV1) }
                #[cfg(not(feature = "protocol-hyprland_lock_notify_v1"))] { None }
            },
            "hyprland_lock_notifier_v1" => {
                #[cfg(feature = "protocol-hyprland_lock_notify_v1")] { Some(ProxyInterface::HyprlandLockNotifierV1) }
                #[cfg(not(feature = "protocol-hyprland_lock_notify_v1"))] { None }
            },
            "hyprland_surface_manager_v1" => {
                #[cfg(feature = "protocol-hyprland_surface_v1")] { Some(ProxyInterface::HyprlandSurfaceManagerV1) }
                #[cfg(not(feature = "protocol-hyprland_surface_v1"))] { None }
            },
            "hyprland_surface_v1" => {
                #[cfg(feature = "protocol-hyprland_surface_v1")] { Some(ProxyInterface::HyprlandSurfaceV1) }
                #[cfg(not(feature = "protocol-hyprland_surface_v1"))] { None }
            },
            "hyprland_toplevel_export_frame_v1" => {
                #[cfg(feature = "protocol-hyprland_toplevel_export_v1")] { Some(ProxyInterface::HyprlandToplevelExportFrameV1) }
                #[cfg(not(feature = "protocol-hyprland_toplevel_export_v1"))] { None }
            },
            "hyprland_toplevel_export_manager_v1" => {
                #[cfg(feature = "protocol-hyprland_toplevel_export_v1")] { Some(ProxyInterface::HyprlandToplevelExportManagerV1) }
                #[cfg(not(feature = "protocol-hyprland_toplevel_export_v1"))] { None }
            },
            "hyprland_toplevel_mapping_manager_v1" => {
                #[cfg(feature = "protocol-hyprland_toplevel_mapping_v1")] { Some(ProxyInterface::HyprlandToplevelMappingManagerV1) }
                #[cfg(not(feature = "protocol-hyprland_toplevel_mapping_v1"))] { None }
            },
            "hyprland_toplevel_window_mapping_handle_v1" => {
                #[cfg(feature = "protocol-hyprland_toplevel_mapping_v1")] { Some(ProxyInterface::HyprlandToplevelWindowMappingHandleV1) }
                #[cfg(not(feature = "protocol-hyprland_toplevel_mapping_v1"))] { None }
            },
            "jay_tray_item_v1" => {
                #[cfg(feature = "protocol-jay_tray_v1")] { Some(ProxyInterface::JayTrayItemV1) }
                #[cfg(not(feature = "protocol-jay_tray_v1"))] { None }
            },
            "jay_tray_v1" => {
                #[cfg(feature = "protocol-jay_tray_v1")] { Some(ProxyInterface::JayTrayV1) }
                #[cfg(not(feature = "protocol-jay_tray_v1"))] { None }
            },
            "wl_drm" => {
                #[cfg(feature = "protocol-drm")] { Some(ProxyInterface::WlDrm) }
                #[cfg(not(feature = "protocol-drm"))] { None }
            },
            "zwp_input_method_keyboard_grab_v2" => {
                #[cfg(feature = "protocol-input_method_unstable_v2")] { Some(ProxyInterface::ZwpInputMethodKeyboardGrabV2) }
                #[cfg(not(feature = "protocol-input_method_unstable_v2"))] { None }
            },
            "zwp_input_method_manager_v2" => {
                #[cfg(feature = "protocol-input_method_unstable_v2")] { Some(ProxyInterface::ZwpInputMethodManagerV2) }
                #[cfg(not(feature = "protocol-input_method_unstable_v2"))] { None }
            },
            "zwp_input_method_v2" => {
                #[cfg(feature = "protocol-input_method_unstable_v2")] { Some(ProxyInterface::ZwpInputMethodV2) }
                #[cfg(not(feature = "protocol-input_method_unstable_v2"))] { None }
            },
            "zwp_input_popup_surface_v2" => {
                #[cfg(feature = "protocol-input_method_unstable_v2")] { Some(ProxyInterface::ZwpInputPopupSurfaceV2) }
                #[cfg(not(feature = "protocol-input_method_unstable_v2"))] { None }
            },
            "org_kde_kwin_server_decoration" => {
                #[cfg(feature = "protocol-org_kde_kwin_server_decoration_v1")] { Some(ProxyInterface::OrgKdeKwinServerDecoration) }
                #[cfg(not(feature = "protocol-org_kde_kwin_server_decoration_v1"))] { None }
            },
            "org_kde_kwin_server_decoration_manager" => {
                #[cfg(feature = "protocol-org_kde_kwin_server_decoration_v1")] { Some(ProxyInterface::OrgKdeKwinServerDecorationManager) }
                #[cfg(not(feature = "protocol-org_kde_kwin_server_decoration_v1"))] { None }
            },
            "zwp_virtual_keyboard_manager_v1" => {
                #[cfg(feature = "protocol-virtual_keyboard_unstable_v1")] { Some(ProxyInterface::ZwpVirtualKeyboardManagerV1) }
                #[cfg(not(feature = "protocol-virtual_keyboard_unstable_v1"))] { None }
            },
            "zwp_virtual_keyboard_v1" => {
                #[cfg(feature = "protocol-virtual_keyboard_unstable_v1")] { Some(ProxyInterface::ZwpVirtualKeyboardV1) }
                #[cfg(not(feature = "protocol-virtual_keyboard_unstable_v1"))] { None }
            },
            "treeland_app_id_resolver_manager_v1" => {
                #[cfg(feature = "protocol-treeland_app_id_resolver_v1")] { Some(ProxyInterface::TreelandAppIdResolverManagerV1) }
                #[cfg(not(feature = "protocol-treeland_app_id_resolver_v1"))] { None }
            },
            "treeland_app_id_resolver_v1" => {
                #[cfg(feature = "protocol-treeland_app_id_resolver_v1")] { Some(ProxyInterface::TreelandAppIdResolverV1) }
                #[cfg(not(feature = "protocol-treeland_app_id_resolver_v1"))] { None }
            },
            "treeland_dde_active_v1" => {
                #[cfg(feature = "protocol-treeland_dde_shell_v1")] { Some(ProxyInterface::TreelandDdeActiveV1) }
                #[cfg(not(feature = "protocol-treeland_dde_shell_v1"))] { None }
            },
            "treeland_dde_shell_manager_v1" => {
                #[cfg(feature = "protocol-treeland_dde_shell_v1")] { Some(ProxyInterface::TreelandDdeShellManagerV1) }
                #[cfg(not(feature = "protocol-treeland_dde_shell_v1"))] { None }
            },
            "treeland_dde_shell_surface_v1" => {
                #[cfg(feature = "protocol-treeland_dde_shell_v1")] { Some(ProxyInterface::TreelandDdeShellSurfaceV1) }
                #[cfg(not(feature = "protocol-treeland_dde_shell_v1"))] { None }
            },
            "treeland_lockscreen_v1" => {
                #[cfg(feature = "protocol-treeland_dde_shell_v1")] { Some(ProxyInterface::TreelandLockscreenV1) }
                #[cfg(not(feature = "protocol-treeland_dde_shell_v1"))] { None }
            },
            "treeland_multitaskview_v1" => {
                #[cfg(feature = "protocol-treeland_dde_shell_v1")] { Some(ProxyInterface::TreelandMultitaskviewV1) }
                #[cfg(not(feature = "protocol-treeland_dde_shell_v1"))] { None }
            },
            "treeland_window_overlap_checker" => {
                #[cfg(feature = "protocol-treeland_dde_shell_v1")] { Some(ProxyInterface::TreelandWindowOverlapChecker) }
                #[cfg(not(feature = "protocol-treeland_dde_shell_v1"))] { None }
            },
            "treeland_window_picker_v1" => {
                #[cfg(feature = "protocol-treeland_dde_shell_v1")] { Some(ProxyInterface::TreelandWindowPickerV1) }
                #[cfg(not(feature = "protocol-treeland_dde_shell_v1"))] { None }
            },
            "treeland_ddm" => {
                #[cfg(feature = "protocol-treeland_ddm")] { Some(ProxyInterface::TreelandDdm) }
                #[cfg(not(feature = "protocol-treeland_ddm"))] { None }
            },
            "treeland_dock_preview_context_v1" => {
                #[cfg(feature = "protocol-treeland_foreign_toplevel_manager_v1")] { Some(ProxyInterface::TreelandDockPreviewContextV1) }
                #[cfg(not(feature = "protocol-treeland_foreign_toplevel_manager_v1"))] { None }
            },
            "treeland_foreign_toplevel_handle_v1" => {
                #[cfg(feature = "protocol-treeland_foreign_toplevel_manager_v1")] { Some(ProxyInterface::TreelandForeignToplevelHandleV1) }
                #[cfg(not(feature = "protocol-treeland_foreign_toplevel_manager_v1"))] { None }
            },
            "treeland_foreign_toplevel_manager_v1" => {
                #[cfg(feature = "protocol-treeland_foreign_toplevel_manager_v1")] { Some(ProxyInterface::TreelandForeignToplevelManagerV1) }
                #[cfg(not(feature = "protocol-treeland_foreign_toplevel_manager_v1"))] { None }
            },
            "treeland_output_color_control_v1" => {
                #[cfg(feature = "protocol-treeland_output_manager_v1")] { Some(ProxyInterface::TreelandOutputColorControlV1) }
                #[cfg(not(feature = "protocol-treeland_output_manager_v1"))] { None }
            },
            "treeland_output_manager_v1" => {
                #[cfg(feature = "protocol-treeland_output_manager_v1")] { Some(ProxyInterface::TreelandOutputManagerV1) }
                #[cfg(not(feature = "protocol-treeland_output_manager_v1"))] { None }
            },
            "treeland_personalization_appearance_context_v1" => {
                #[cfg(feature = "protocol-treeland_personalization_manager_v1")] { Some(ProxyInterface::TreelandPersonalizationAppearanceContextV1) }
                #[cfg(not(feature = "protocol-treeland_personalization_manager_v1"))] { None }
            },
            "treeland_personalization_cursor_context_v1" => {
                #[cfg(feature = "protocol-treeland_personalization_manager_v1")] { Some(ProxyInterface::TreelandPersonalizationCursorContextV1) }
                #[cfg(not(feature = "protocol-treeland_personalization_manager_v1"))] { None }
            },
            "treeland_personalization_font_context_v1" => {
                #[cfg(feature = "protocol-treeland_personalization_manager_v1")] { Some(ProxyInterface::TreelandPersonalizationFontContextV1) }
                #[cfg(not(feature = "protocol-treeland_personalization_manager_v1"))] { None }
            },
            "treeland_personalization_manager_v1" => {
                #[cfg(feature = "protocol-treeland_personalization_manager_v1")] { Some(ProxyInterface::TreelandPersonalizationManagerV1) }
                #[cfg(not(feature = "protocol-treeland_personalization_manager_v1"))] { None }
            },
            "treeland_personalization_wallpaper_context_v1" => {
                #[cfg(feature = "protocol-treeland_personalization_manager_v1")] { Some(ProxyInterface::TreelandPersonalizationWallpaperContextV1) }
                #[cfg(not(feature = "protocol-treeland_personalization_manager_v1"))] { None }
            },
            "treeland_personalization_window_context_v1" => {
                #[cfg(feature = "protocol-treeland_personalization_manager_v1")] { Some(ProxyInterface::TreelandPersonalizationWindowContextV1) }
                #[cfg(not(feature = "protocol-treeland_personalization_manager_v1"))] { None }
            },
            "treeland_prelaunch_splash_manager_v1" => {
                #[cfg(feature = "protocol-treeland_prelaunch_splash_v1")] { Some(ProxyInterface::TreelandPrelaunchSplashManagerV1) }
                #[cfg(not(feature = "protocol-treeland_prelaunch_splash_v1"))] { None }
            },
            "treeland_screensaver" => {
                #[cfg(feature = "protocol-treeland_screensaver")] { Some(ProxyInterface::TreelandScreensaver) }
                #[cfg(not(feature = "protocol-treeland_screensaver"))] { None }
            },
            "treeland_shortcut_context_v1" => {
                #[cfg(feature = "protocol-treeland_shortcut_manager_v1")] { Some(ProxyInterface::TreelandShortcutContextV1) }
                #[cfg(not(feature = "protocol-treeland_shortcut_manager_v1"))] { None }
            },
            "treeland_shortcut_manager_v1" => {
                #[cfg(feature = "protocol-treeland_shortcut_manager_v1")] { Some(ProxyInterface::TreelandShortcutManagerV1) }
                #[cfg(not(feature = "protocol-treeland_shortcut_manager_v1"))] { None }
            },
            "treeland_shortcut_manager_v2" => {
                #[cfg(feature = "protocol-treeland_shortcut_manager_v2")] { Some(ProxyInterface::TreelandShortcutManagerV2) }
                #[cfg(not(feature = "protocol-treeland_shortcut_manager_v2"))] { None }
            },
            "treeland_virtual_output_manager_v1" => {
                #[cfg(feature = "protocol-treeland_virtual_output_manager_v1")] { Some(ProxyInterface::TreelandVirtualOutputManagerV1) }
                #[cfg(not(feature = "protocol-treeland_virtual_output_manager_v1"))] { None }
            },
            "treeland_virtual_output_v1" => {
                #[cfg(feature = "protocol-treeland_virtual_output_manager_v1")] { Some(ProxyInterface::TreelandVirtualOutputV1) }
                #[cfg(not(feature = "protocol-treeland_virtual_output_manager_v1"))] { None }
            },
            "treeland_wallpaper_color_manager_v1" => {
                #[cfg(feature = "protocol-treeland_wallpaper_color_v1")] { Some(ProxyInterface::TreelandWallpaperColorManagerV1) }
                #[cfg(not(feature = "protocol-treeland_wallpaper_color_v1"))] { None }
            },
            "treeland_window_management_v1" => {
                #[cfg(feature = "protocol-treeland_window_management_v1")] { Some(ProxyInterface::TreelandWindowManagementV1) }
                #[cfg(not(feature = "protocol-treeland_window_management_v1"))] { None }
            },
            "wl_buffer" => Some(ProxyInterface::WlBuffer),
            "wl_callback" => Some(ProxyInterface::WlCallback),
            "wl_compositor" => Some(ProxyInterface::WlCompositor),
            "wl_data_device" => Some(ProxyInterface::WlDataDevice),
            "wl_data_device_manager" => Some(ProxyInterface::WlDataDeviceManager),
            "wl_data_offer" => Some(ProxyInterface::WlDataOffer),
            "wl_data_source" => Some(ProxyInterface::WlDataSource),
            "wl_display" => Some(ProxyInterface::WlDisplay),
            "wl_fixes" => Some(ProxyInterface::WlFixes),
            "wl_keyboard" => Some(ProxyInterface::WlKeyboard),
            "wl_output" => Some(ProxyInterface::WlOutput),
            "wl_pointer" => Some(ProxyInterface::WlPointer),
            "wl_region" => Some(ProxyInterface::WlRegion),
            "wl_registry" => Some(ProxyInterface::WlRegistry),
            "wl_seat" => Some(ProxyInterface::WlSeat),
            "wl_shell" => Some(ProxyInterface::WlShell),
            "wl_shell_surface" => Some(ProxyInterface::WlShellSurface),
            "wl_shm" => Some(ProxyInterface::WlShm),
            "wl_shm_pool" => Some(ProxyInterface::WlShmPool),
            "wl_subcompositor" => Some(ProxyInterface::WlSubcompositor),
            "wl_subsurface" => Some(ProxyInterface::WlSubsurface),
            "wl_surface" => Some(ProxyInterface::WlSurface),
            "wl_touch" => Some(ProxyInterface::WlTouch),
            "wp_alpha_modifier_surface_v1" => {
                #[cfg(feature = "protocol-alpha_modifier_v1")] { Some(ProxyInterface::WpAlphaModifierSurfaceV1) }
                #[cfg(not(feature = "protocol-alpha_modifier_v1"))] { None }
            },
            "wp_alpha_modifier_v1" => {
                #[cfg(feature = "protocol-alpha_modifier_v1")] { Some(ProxyInterface::WpAlphaModifierV1) }
                #[cfg(not(feature = "protocol-alpha_modifier_v1"))] { None }
            },
            "wp_color_management_output_v1" => {
                #[cfg(feature = "protocol-color_management_v1")] { Some(ProxyInterface::WpColorManagementOutputV1) }
                #[cfg(not(feature = "protocol-color_management_v1"))] { None }
            },
            "wp_color_management_surface_feedback_v1" => {
                #[cfg(feature = "protocol-color_management_v1")] { Some(ProxyInterface::WpColorManagementSurfaceFeedbackV1) }
                #[cfg(not(feature = "protocol-color_management_v1"))] { None }
            },
            "wp_color_management_surface_v1" => {
                #[cfg(feature = "protocol-color_management_v1")] { Some(ProxyInterface::WpColorManagementSurfaceV1) }
                #[cfg(not(feature = "protocol-color_management_v1"))] { None }
            },
            "wp_color_manager_v1" => {
                #[cfg(feature = "protocol-color_management_v1")] { Some(ProxyInterface::WpColorManagerV1) }
                #[cfg(not(feature = "protocol-color_management_v1"))] { None }
            },
            "wp_image_description_creator_icc_v1" => {
                #[cfg(feature = "protocol-color_management_v1")] { Some(ProxyInterface::WpImageDescriptionCreatorIccV1) }
                #[cfg(not(feature = "protocol-color_management_v1"))] { None }
            },
            "wp_image_description_creator_params_v1" => {
                #[cfg(feature = "protocol-color_management_v1")] { Some(ProxyInterface::WpImageDescriptionCreatorParamsV1) }
                #[cfg(not(feature = "protocol-color_management_v1"))] { None }
            },
            "wp_image_description_info_v1" => {
                #[cfg(feature = "protocol-color_management_v1")] { Some(ProxyInterface::WpImageDescriptionInfoV1) }
                #[cfg(not(feature = "protocol-color_management_v1"))] { None }
            },
            "wp_image_description_v1" => {
                #[cfg(feature = "protocol-color_management_v1")] { Some(ProxyInterface::WpImageDescriptionV1) }
                #[cfg(not(feature = "protocol-color_management_v1"))] { None }
            },
            "wp_color_representation_manager_v1" => {
                #[cfg(feature = "protocol-color_representation_v1")] { Some(ProxyInterface::WpColorRepresentationManagerV1) }
                #[cfg(not(feature = "protocol-color_representation_v1"))] { None }
            },
            "wp_color_representation_surface_v1" => {
                #[cfg(feature = "protocol-color_representation_v1")] { Some(ProxyInterface::WpColorRepresentationSurfaceV1) }
                #[cfg(not(feature = "protocol-color_representation_v1"))] { None }
            },
            "wp_commit_timer_v1" => {
                #[cfg(feature = "protocol-commit_timing_v1")] { Some(ProxyInterface::WpCommitTimerV1) }
                #[cfg(not(feature = "protocol-commit_timing_v1"))] { None }
            },
            "wp_commit_timing_manager_v1" => {
                #[cfg(feature = "protocol-commit_timing_v1")] { Some(ProxyInterface::WpCommitTimingManagerV1) }
                #[cfg(not(feature = "protocol-commit_timing_v1"))] { None }
            },
            "wp_content_type_manager_v1" => {
                #[cfg(feature = "protocol-content_type_v1")] { Some(ProxyInterface::WpContentTypeManagerV1) }
                #[cfg(not(feature = "protocol-content_type_v1"))] { None }
            },
            "wp_content_type_v1" => {
                #[cfg(feature = "protocol-content_type_v1")] { Some(ProxyInterface::WpContentTypeV1) }
                #[cfg(not(feature = "protocol-content_type_v1"))] { None }
            },
            "wp_cursor_shape_device_v1" => {
                #[cfg(feature = "protocol-cursor_shape_v1")] { Some(ProxyInterface::WpCursorShapeDeviceV1) }
                #[cfg(not(feature = "protocol-cursor_shape_v1"))] { None }
            },
            "wp_cursor_shape_manager_v1" => {
                #[cfg(feature = "protocol-cursor_shape_v1")] { Some(ProxyInterface::WpCursorShapeManagerV1) }
                #[cfg(not(feature = "protocol-cursor_shape_v1"))] { None }
            },
            "wp_drm_lease_connector_v1" => {
                #[cfg(feature = "protocol-drm_lease_v1")] { Some(ProxyInterface::WpDrmLeaseConnectorV1) }
                #[cfg(not(feature = "protocol-drm_lease_v1"))] { None }
            },
            "wp_drm_lease_device_v1" => {
                #[cfg(feature = "protocol-drm_lease_v1")] { Some(ProxyInterface::WpDrmLeaseDeviceV1) }
                #[cfg(not(feature = "protocol-drm_lease_v1"))] { None }
            },
            "wp_drm_lease_request_v1" => {
                #[cfg(feature = "protocol-drm_lease_v1")] { Some(ProxyInterface::WpDrmLeaseRequestV1) }
                #[cfg(not(feature = "protocol-drm_lease_v1"))] { None }
            },
            "wp_drm_lease_v1" => {
                #[cfg(feature = "protocol-drm_lease_v1")] { Some(ProxyInterface::WpDrmLeaseV1) }
                #[cfg(not(feature = "protocol-drm_lease_v1"))] { None }
            },
            "ext_background_effect_manager_v1" => {
                #[cfg(feature = "protocol-ext_background_effect_v1")] { Some(ProxyInterface::ExtBackgroundEffectManagerV1) }
                #[cfg(not(feature = "protocol-ext_background_effect_v1"))] { None }
            },
            "ext_background_effect_surface_v1" => {
                #[cfg(feature = "protocol-ext_background_effect_v1")] { Some(ProxyInterface::ExtBackgroundEffectSurfaceV1) }
                #[cfg(not(feature = "protocol-ext_background_effect_v1"))] { None }
            },
            "ext_data_control_device_v1" => {
                #[cfg(feature = "protocol-ext_data_control_v1")] { Some(ProxyInterface::ExtDataControlDeviceV1) }
                #[cfg(not(feature = "protocol-ext_data_control_v1"))] { None }
            },
            "ext_data_control_manager_v1" => {
                #[cfg(feature = "protocol-ext_data_control_v1")] { Some(ProxyInterface::ExtDataControlManagerV1) }
                #[cfg(not(feature = "protocol-ext_data_control_v1"))] { None }
            },
            "ext_data_control_offer_v1" => {
                #[cfg(feature = "protocol-ext_data_control_v1")] { Some(ProxyInterface::ExtDataControlOfferV1) }
                #[cfg(not(feature = "protocol-ext_data_control_v1"))] { None }
            },
            "ext_data_control_source_v1" => {
                #[cfg(feature = "protocol-ext_data_control_v1")] { Some(ProxyInterface::ExtDataControlSourceV1) }
                #[cfg(not(feature = "protocol-ext_data_control_v1"))] { None }
            },
            "ext_foreign_toplevel_handle_v1" => {
                #[cfg(feature = "protocol-ext_foreign_toplevel_list_v1")] { Some(ProxyInterface::ExtForeignToplevelHandleV1) }
                #[cfg(not(feature = "protocol-ext_foreign_toplevel_list_v1"))] { None }
            },
            "ext_foreign_toplevel_list_v1" => {
                #[cfg(feature = "protocol-ext_foreign_toplevel_list_v1")] { Some(ProxyInterface::ExtForeignToplevelListV1) }
                #[cfg(not(feature = "protocol-ext_foreign_toplevel_list_v1"))] { None }
            },
            "ext_idle_notification_v1" => {
                #[cfg(feature = "protocol-ext_idle_notify_v1")] { Some(ProxyInterface::ExtIdleNotificationV1) }
                #[cfg(not(feature = "protocol-ext_idle_notify_v1"))] { None }
            },
            "ext_idle_notifier_v1" => {
                #[cfg(feature = "protocol-ext_idle_notify_v1")] { Some(ProxyInterface::ExtIdleNotifierV1) }
                #[cfg(not(feature = "protocol-ext_idle_notify_v1"))] { None }
            },
            "ext_foreign_toplevel_image_capture_source_manager_v1" => {
                #[cfg(feature = "protocol-ext_image_capture_source_v1")] { Some(ProxyInterface::ExtForeignToplevelImageCaptureSourceManagerV1) }
                #[cfg(not(feature = "protocol-ext_image_capture_source_v1"))] { None }
            },
            "ext_image_capture_source_v1" => {
                #[cfg(feature = "protocol-ext_image_capture_source_v1")] { Some(ProxyInterface::ExtImageCaptureSourceV1) }
                #[cfg(not(feature = "protocol-ext_image_capture_source_v1"))] { None }
            },
            "ext_output_image_capture_source_manager_v1" => {
                #[cfg(feature = "protocol-ext_image_capture_source_v1")] { Some(ProxyInterface::ExtOutputImageCaptureSourceManagerV1) }
                #[cfg(not(feature = "protocol-ext_image_capture_source_v1"))] { None }
            },
            "ext_image_copy_capture_cursor_session_v1" => {
                #[cfg(feature = "protocol-ext_image_copy_capture_v1")] { Some(ProxyInterface::ExtImageCopyCaptureCursorSessionV1) }
                #[cfg(not(feature = "protocol-ext_image_copy_capture_v1"))] { None }
            },
            "ext_image_copy_capture_frame_v1" => {
                #[cfg(feature = "protocol-ext_image_copy_capture_v1")] { Some(ProxyInterface::ExtImageCopyCaptureFrameV1) }
                #[cfg(not(feature = "protocol-ext_image_copy_capture_v1"))] { None }
            },
            "ext_image_copy_capture_manager_v1" => {
                #[cfg(feature = "protocol-ext_image_copy_capture_v1")] { Some(ProxyInterface::ExtImageCopyCaptureManagerV1) }
                #[cfg(not(feature = "protocol-ext_image_copy_capture_v1"))] { None }
            },
            "ext_image_copy_capture_session_v1" => {
                #[cfg(feature = "protocol-ext_image_copy_capture_v1")] { Some(ProxyInterface::ExtImageCopyCaptureSessionV1) }
                #[cfg(not(feature = "protocol-ext_image_copy_capture_v1"))] { None }
            },
            "ext_session_lock_manager_v1" => {
                #[cfg(feature = "protocol-ext_session_lock_v1")] { Some(ProxyInterface::ExtSessionLockManagerV1) }
                #[cfg(not(feature = "protocol-ext_session_lock_v1"))] { None }
            },
            "ext_session_lock_surface_v1" => {
                #[cfg(feature = "protocol-ext_session_lock_v1")] { Some(ProxyInterface::ExtSessionLockSurfaceV1) }
                #[cfg(not(feature = "protocol-ext_session_lock_v1"))] { None }
            },
            "ext_session_lock_v1" => {
                #[cfg(feature = "protocol-ext_session_lock_v1")] { Some(ProxyInterface::ExtSessionLockV1) }
                #[cfg(not(feature = "protocol-ext_session_lock_v1"))] { None }
            },
            "ext_transient_seat_manager_v1" => {
                #[cfg(feature = "protocol-ext_transient_seat_v1")] { Some(ProxyInterface::ExtTransientSeatManagerV1) }
                #[cfg(not(feature = "protocol-ext_transient_seat_v1"))] { None }
            },
            "ext_transient_seat_v1" => {
                #[cfg(feature = "protocol-ext_transient_seat_v1")] { Some(ProxyInterface::ExtTransientSeatV1) }
                #[cfg(not(feature = "protocol-ext_transient_seat_v1"))] { None }
            },
            "ext_workspace_group_handle_v1" => {
                #[cfg(feature = "protocol-ext_workspace_v1")] { Some(ProxyInterface::ExtWorkspaceGroupHandleV1) }
                #[cfg(not(feature = "protocol-ext_workspace_v1"))] { None }
            },
            "ext_workspace_handle_v1" => {
                #[cfg(feature = "protocol-ext_workspace_v1")] { Some(ProxyInterface::ExtWorkspaceHandleV1) }
                #[cfg(not(feature = "protocol-ext_workspace_v1"))] { None }
            },
            "ext_workspace_manager_v1" => {
                #[cfg(feature = "protocol-ext_workspace_v1")] { Some(ProxyInterface::ExtWorkspaceManagerV1) }
                #[cfg(not(feature = "protocol-ext_workspace_v1"))] { None }
            },
            "wp_fifo_manager_v1" => {
                #[cfg(feature = "protocol-fifo_v1")] { Some(ProxyInterface::WpFifoManagerV1) }
                #[cfg(not(feature = "protocol-fifo_v1"))] { None }
            },
            "wp_fifo_v1" => {
                #[cfg(feature = "protocol-fifo_v1")] { Some(ProxyInterface::WpFifoV1) }
                #[cfg(not(feature = "protocol-fifo_v1"))] { None }
            },
            "wp_fractional_scale_manager_v1" => {
                #[cfg(feature = "protocol-fractional_scale_v1")] { Some(ProxyInterface::WpFractionalScaleManagerV1) }
                #[cfg(not(feature = "protocol-fractional_scale_v1"))] { None }
            },
            "wp_fractional_scale_v1" => {
                #[cfg(feature = "protocol-fractional_scale_v1")] { Some(ProxyInterface::WpFractionalScaleV1) }
                #[cfg(not(feature = "protocol-fractional_scale_v1"))] { None }
            },
            "zwp_fullscreen_shell_mode_feedback_v1" => {
                #[cfg(feature = "protocol-fullscreen_shell_unstable_v1")] { Some(ProxyInterface::ZwpFullscreenShellModeFeedbackV1) }
                #[cfg(not(feature = "protocol-fullscreen_shell_unstable_v1"))] { None }
            },
            "zwp_fullscreen_shell_v1" => {
                #[cfg(feature = "protocol-fullscreen_shell_unstable_v1")] { Some(ProxyInterface::ZwpFullscreenShellV1) }
                #[cfg(not(feature = "protocol-fullscreen_shell_unstable_v1"))] { None }
            },
            "zwp_idle_inhibit_manager_v1" => {
                #[cfg(feature = "protocol-idle_inhibit_unstable_v1")] { Some(ProxyInterface::ZwpIdleInhibitManagerV1) }
                #[cfg(not(feature = "protocol-idle_inhibit_unstable_v1"))] { None }
            },
            "zwp_idle_inhibitor_v1" => {
                #[cfg(feature = "protocol-idle_inhibit_unstable_v1")] { Some(ProxyInterface::ZwpIdleInhibitorV1) }
                #[cfg(not(feature = "protocol-idle_inhibit_unstable_v1"))] { None }
            },
            "zwp_input_method_context_v1" => {
                #[cfg(feature = "protocol-input_method_unstable_v1")] { Some(ProxyInterface::ZwpInputMethodContextV1) }
                #[cfg(not(feature = "protocol-input_method_unstable_v1"))] { None }
            },
            "zwp_input_method_v1" => {
                #[cfg(feature = "protocol-input_method_unstable_v1")] { Some(ProxyInterface::ZwpInputMethodV1) }
                #[cfg(not(feature = "protocol-input_method_unstable_v1"))] { None }
            },
            "zwp_input_panel_surface_v1" => {
                #[cfg(feature = "protocol-input_method_unstable_v1")] { Some(ProxyInterface::ZwpInputPanelSurfaceV1) }
                #[cfg(not(feature = "protocol-input_method_unstable_v1"))] { None }
            },
            "zwp_input_panel_v1" => {
                #[cfg(feature = "protocol-input_method_unstable_v1")] { Some(ProxyInterface::ZwpInputPanelV1) }
                #[cfg(not(feature = "protocol-input_method_unstable_v1"))] { None }
            },
            "zwp_input_timestamps_manager_v1" => {
                #[cfg(feature = "protocol-input_timestamps_unstable_v1")] { Some(ProxyInterface::ZwpInputTimestampsManagerV1) }
                #[cfg(not(feature = "protocol-input_timestamps_unstable_v1"))] { None }
            },
            "zwp_input_timestamps_v1" => {
                #[cfg(feature = "protocol-input_timestamps_unstable_v1")] { Some(ProxyInterface::ZwpInputTimestampsV1) }
                #[cfg(not(feature = "protocol-input_timestamps_unstable_v1"))] { None }
            },
            "zwp_keyboard_shortcuts_inhibit_manager_v1" => {
                #[cfg(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1")] { Some(ProxyInterface::ZwpKeyboardShortcutsInhibitManagerV1) }
                #[cfg(not(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1"))] { None }
            },
            "zwp_keyboard_shortcuts_inhibitor_v1" => {
                #[cfg(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1")] { Some(ProxyInterface::ZwpKeyboardShortcutsInhibitorV1) }
                #[cfg(not(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1"))] { None }
            },
            "zwp_linux_buffer_params_v1" => {
                #[cfg(feature = "protocol-linux_dmabuf_v1")] { Some(ProxyInterface::ZwpLinuxBufferParamsV1) }
                #[cfg(not(feature = "protocol-linux_dmabuf_v1"))] { None }
            },
            "zwp_linux_dmabuf_feedback_v1" => {
                #[cfg(feature = "protocol-linux_dmabuf_v1")] { Some(ProxyInterface::ZwpLinuxDmabufFeedbackV1) }
                #[cfg(not(feature = "protocol-linux_dmabuf_v1"))] { None }
            },
            "zwp_linux_dmabuf_v1" => {
                #[cfg(feature = "protocol-linux_dmabuf_v1")] { Some(ProxyInterface::ZwpLinuxDmabufV1) }
                #[cfg(not(feature = "protocol-linux_dmabuf_v1"))] { None }
            },
            "wp_linux_drm_syncobj_manager_v1" => {
                #[cfg(feature = "protocol-linux_drm_syncobj_v1")] { Some(ProxyInterface::WpLinuxDrmSyncobjManagerV1) }
                #[cfg(not(feature = "protocol-linux_drm_syncobj_v1"))] { None }
            },
            "wp_linux_drm_syncobj_surface_v1" => {
                #[cfg(feature = "protocol-linux_drm_syncobj_v1")] { Some(ProxyInterface::WpLinuxDrmSyncobjSurfaceV1) }
                #[cfg(not(feature = "protocol-linux_drm_syncobj_v1"))] { None }
            },
            "wp_linux_drm_syncobj_timeline_v1" => {
                #[cfg(feature = "protocol-linux_drm_syncobj_v1")] { Some(ProxyInterface::WpLinuxDrmSyncobjTimelineV1) }
                #[cfg(not(feature = "protocol-linux_drm_syncobj_v1"))] { None }
            },
            "zwp_confined_pointer_v1" => {
                #[cfg(feature = "protocol-pointer_constraints_unstable_v1")] { Some(ProxyInterface::ZwpConfinedPointerV1) }
                #[cfg(not(feature = "protocol-pointer_constraints_unstable_v1"))] { None }
            },
            "zwp_locked_pointer_v1" => {
                #[cfg(feature = "protocol-pointer_constraints_unstable_v1")] { Some(ProxyInterface::ZwpLockedPointerV1) }
                #[cfg(not(feature = "protocol-pointer_constraints_unstable_v1"))] { None }
            },
            "zwp_pointer_constraints_v1" => {
                #[cfg(feature = "protocol-pointer_constraints_unstable_v1")] { Some(ProxyInterface::ZwpPointerConstraintsV1) }
                #[cfg(not(feature = "protocol-pointer_constraints_unstable_v1"))] { None }
            },
            "zwp_pointer_gesture_hold_v1" => {
                #[cfg(feature = "protocol-pointer_gestures_unstable_v1")] { Some(ProxyInterface::ZwpPointerGestureHoldV1) }
                #[cfg(not(feature = "protocol-pointer_gestures_unstable_v1"))] { None }
            },
            "zwp_pointer_gesture_pinch_v1" => {
                #[cfg(feature = "protocol-pointer_gestures_unstable_v1")] { Some(ProxyInterface::ZwpPointerGesturePinchV1) }
                #[cfg(not(feature = "protocol-pointer_gestures_unstable_v1"))] { None }
            },
            "zwp_pointer_gesture_swipe_v1" => {
                #[cfg(feature = "protocol-pointer_gestures_unstable_v1")] { Some(ProxyInterface::ZwpPointerGestureSwipeV1) }
                #[cfg(not(feature = "protocol-pointer_gestures_unstable_v1"))] { None }
            },
            "zwp_pointer_gestures_v1" => {
                #[cfg(feature = "protocol-pointer_gestures_unstable_v1")] { Some(ProxyInterface::ZwpPointerGesturesV1) }
                #[cfg(not(feature = "protocol-pointer_gestures_unstable_v1"))] { None }
            },
            "wp_pointer_warp_v1" => {
                #[cfg(feature = "protocol-pointer_warp_v1")] { Some(ProxyInterface::WpPointerWarpV1) }
                #[cfg(not(feature = "protocol-pointer_warp_v1"))] { None }
            },
            "wp_presentation" => {
                #[cfg(feature = "protocol-presentation_time")] { Some(ProxyInterface::WpPresentation) }
                #[cfg(not(feature = "protocol-presentation_time"))] { None }
            },
            "wp_presentation_feedback" => {
                #[cfg(feature = "protocol-presentation_time")] { Some(ProxyInterface::WpPresentationFeedback) }
                #[cfg(not(feature = "protocol-presentation_time"))] { None }
            },
            "zwp_relative_pointer_manager_v1" => {
                #[cfg(feature = "protocol-relative_pointer_unstable_v1")] { Some(ProxyInterface::ZwpRelativePointerManagerV1) }
                #[cfg(not(feature = "protocol-relative_pointer_unstable_v1"))] { None }
            },
            "zwp_relative_pointer_v1" => {
                #[cfg(feature = "protocol-relative_pointer_unstable_v1")] { Some(ProxyInterface::ZwpRelativePointerV1) }
                #[cfg(not(feature = "protocol-relative_pointer_unstable_v1"))] { None }
            },
            "wp_security_context_manager_v1" => {
                #[cfg(feature = "protocol-security_context_v1")] { Some(ProxyInterface::WpSecurityContextManagerV1) }
                #[cfg(not(feature = "protocol-security_context_v1"))] { None }
            },
            "wp_security_context_v1" => {
                #[cfg(feature = "protocol-security_context_v1")] { Some(ProxyInterface::WpSecurityContextV1) }
                #[cfg(not(feature = "protocol-security_context_v1"))] { None }
            },
            "wp_single_pixel_buffer_manager_v1" => {
                #[cfg(feature = "protocol-single_pixel_buffer_v1")] { Some(ProxyInterface::WpSinglePixelBufferManagerV1) }
                #[cfg(not(feature = "protocol-single_pixel_buffer_v1"))] { None }
            },
            "zwp_tablet_manager_v2" => {
                #[cfg(feature = "protocol-tablet_v2")] { Some(ProxyInterface::ZwpTabletManagerV2) }
                #[cfg(not(feature = "protocol-tablet_v2"))] { None }
            },
            "zwp_tablet_pad_dial_v2" => {
                #[cfg(feature = "protocol-tablet_v2")] { Some(ProxyInterface::ZwpTabletPadDialV2) }
                #[cfg(not(feature = "protocol-tablet_v2"))] { None }
            },
            "zwp_tablet_pad_group_v2" => {
                #[cfg(feature = "protocol-tablet_v2")] { Some(ProxyInterface::ZwpTabletPadGroupV2) }
                #[cfg(not(feature = "protocol-tablet_v2"))] { None }
            },
            "zwp_tablet_pad_ring_v2" => {
                #[cfg(feature = "protocol-tablet_v2")] { Some(ProxyInterface::ZwpTabletPadRingV2) }
                #[cfg(not(feature = "protocol-tablet_v2"))] { None }
            },
            "zwp_tablet_pad_strip_v2" => {
                #[cfg(feature = "protocol-tablet_v2")] { Some(ProxyInterface::ZwpTabletPadStripV2) }
                #[cfg(not(feature = "protocol-tablet_v2"))] { None }
            },
            "zwp_tablet_pad_v2" => {
                #[cfg(feature = "protocol-tablet_v2")] { Some(ProxyInterface::ZwpTabletPadV2) }
                #[cfg(not(feature = "protocol-tablet_v2"))] { None }
            },
            "zwp_tablet_seat_v2" => {
                #[cfg(feature = "protocol-tablet_v2")] { Some(ProxyInterface::ZwpTabletSeatV2) }
                #[cfg(not(feature = "protocol-tablet_v2"))] { None }
            },
            "zwp_tablet_tool_v2" => {
                #[cfg(feature = "protocol-tablet_v2")] { Some(ProxyInterface::ZwpTabletToolV2) }
                #[cfg(not(feature = "protocol-tablet_v2"))] { None }
            },
            "zwp_tablet_v2" => {
                #[cfg(feature = "protocol-tablet_v2")] { Some(ProxyInterface::ZwpTabletV2) }
                #[cfg(not(feature = "protocol-tablet_v2"))] { None }
            },
            "wp_tearing_control_manager_v1" => {
                #[cfg(feature = "protocol-tearing_control_v1")] { Some(ProxyInterface::WpTearingControlManagerV1) }
                #[cfg(not(feature = "protocol-tearing_control_v1"))] { None }
            },
            "wp_tearing_control_v1" => {
                #[cfg(feature = "protocol-tearing_control_v1")] { Some(ProxyInterface::WpTearingControlV1) }
                #[cfg(not(feature = "protocol-tearing_control_v1"))] { None }
            },
            "zwp_text_input_manager_v1" => {
                #[cfg(feature = "protocol-text_input_unstable_v1")] { Some(ProxyInterface::ZwpTextInputManagerV1) }
                #[cfg(not(feature = "protocol-text_input_unstable_v1"))] { None }
            },
            "zwp_text_input_v1" => {
                #[cfg(feature = "protocol-text_input_unstable_v1")] { Some(ProxyInterface::ZwpTextInputV1) }
                #[cfg(not(feature = "protocol-text_input_unstable_v1"))] { None }
            },
            "zwp_text_input_manager_v3" => {
                #[cfg(feature = "protocol-text_input_unstable_v3")] { Some(ProxyInterface::ZwpTextInputManagerV3) }
                #[cfg(not(feature = "protocol-text_input_unstable_v3"))] { None }
            },
            "zwp_text_input_v3" => {
                #[cfg(feature = "protocol-text_input_unstable_v3")] { Some(ProxyInterface::ZwpTextInputV3) }
                #[cfg(not(feature = "protocol-text_input_unstable_v3"))] { None }
            },
            "wp_viewport" => {
                #[cfg(feature = "protocol-viewporter")] { Some(ProxyInterface::WpViewport) }
                #[cfg(not(feature = "protocol-viewporter"))] { None }
            },
            "wp_viewporter" => {
                #[cfg(feature = "protocol-viewporter")] { Some(ProxyInterface::WpViewporter) }
                #[cfg(not(feature = "protocol-viewporter"))] { None }
            },
            "zwp_primary_selection_device_manager_v1" => {
                #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")] { Some(ProxyInterface::ZwpPrimarySelectionDeviceManagerV1) }
                #[cfg(not(feature = "protocol-wp_primary_selection_unstable_v1"))] { None }
            },
            "zwp_primary_selection_device_v1" => {
                #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")] { Some(ProxyInterface::ZwpPrimarySelectionDeviceV1) }
                #[cfg(not(feature = "protocol-wp_primary_selection_unstable_v1"))] { None }
            },
            "zwp_primary_selection_offer_v1" => {
                #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")] { Some(ProxyInterface::ZwpPrimarySelectionOfferV1) }
                #[cfg(not(feature = "protocol-wp_primary_selection_unstable_v1"))] { None }
            },
            "zwp_primary_selection_source_v1" => {
                #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")] { Some(ProxyInterface::ZwpPrimarySelectionSourceV1) }
                #[cfg(not(feature = "protocol-wp_primary_selection_unstable_v1"))] { None }
            },
            "xdg_activation_token_v1" => {
                #[cfg(feature = "protocol-xdg_activation_v1")] { Some(ProxyInterface::XdgActivationTokenV1) }
                #[cfg(not(feature = "protocol-xdg_activation_v1"))] { None }
            },
            "xdg_activation_v1" => {
                #[cfg(feature = "protocol-xdg_activation_v1")] { Some(ProxyInterface::XdgActivationV1) }
                #[cfg(not(feature = "protocol-xdg_activation_v1"))] { None }
            },
            "zxdg_decoration_manager_v1" => {
                #[cfg(feature = "protocol-xdg_decoration_unstable_v1")] { Some(ProxyInterface::ZxdgDecorationManagerV1) }
                #[cfg(not(feature = "protocol-xdg_decoration_unstable_v1"))] { None }
            },
            "zxdg_toplevel_decoration_v1" => {
                #[cfg(feature = "protocol-xdg_decoration_unstable_v1")] { Some(ProxyInterface::ZxdgToplevelDecorationV1) }
                #[cfg(not(feature = "protocol-xdg_decoration_unstable_v1"))] { None }
            },
            "xdg_dialog_v1" => {
                #[cfg(feature = "protocol-xdg_dialog_v1")] { Some(ProxyInterface::XdgDialogV1) }
                #[cfg(not(feature = "protocol-xdg_dialog_v1"))] { None }
            },
            "xdg_wm_dialog_v1" => {
                #[cfg(feature = "protocol-xdg_dialog_v1")] { Some(ProxyInterface::XdgWmDialogV1) }
                #[cfg(not(feature = "protocol-xdg_dialog_v1"))] { None }
            },
            "zxdg_exported_v2" => {
                #[cfg(feature = "protocol-xdg_foreign_unstable_v2")] { Some(ProxyInterface::ZxdgExportedV2) }
                #[cfg(not(feature = "protocol-xdg_foreign_unstable_v2"))] { None }
            },
            "zxdg_exporter_v2" => {
                #[cfg(feature = "protocol-xdg_foreign_unstable_v2")] { Some(ProxyInterface::ZxdgExporterV2) }
                #[cfg(not(feature = "protocol-xdg_foreign_unstable_v2"))] { None }
            },
            "zxdg_imported_v2" => {
                #[cfg(feature = "protocol-xdg_foreign_unstable_v2")] { Some(ProxyInterface::ZxdgImportedV2) }
                #[cfg(not(feature = "protocol-xdg_foreign_unstable_v2"))] { None }
            },
            "zxdg_importer_v2" => {
                #[cfg(feature = "protocol-xdg_foreign_unstable_v2")] { Some(ProxyInterface::ZxdgImporterV2) }
                #[cfg(not(feature = "protocol-xdg_foreign_unstable_v2"))] { None }
            },
            "zxdg_output_manager_v1" => {
                #[cfg(feature = "protocol-xdg_output_unstable_v1")] { Some(ProxyInterface::ZxdgOutputManagerV1) }
                #[cfg(not(feature = "protocol-xdg_output_unstable_v1"))] { None }
            },
            "zxdg_output_v1" => {
                #[cfg(feature = "protocol-xdg_output_unstable_v1")] { Some(ProxyInterface::ZxdgOutputV1) }
                #[cfg(not(feature = "protocol-xdg_output_unstable_v1"))] { None }
            },
            "xdg_popup" => {
                #[cfg(feature = "protocol-xdg_shell")] { Some(ProxyInterface::XdgPopup) }
                #[cfg(not(feature = "protocol-xdg_shell"))] { None }
            },
            "xdg_positioner" => {
                #[cfg(feature = "protocol-xdg_shell")] { Some(ProxyInterface::XdgPositioner) }
                #[cfg(not(feature = "protocol-xdg_shell"))] { None }
            },
            "xdg_surface" => {
                #[cfg(feature = "protocol-xdg_shell")] { Some(ProxyInterface::XdgSurface) }
                #[cfg(not(feature = "protocol-xdg_shell"))] { None }
            },
            "xdg_toplevel" => {
                #[cfg(feature = "protocol-xdg_shell")] { Some(ProxyInterface::XdgToplevel) }
                #[cfg(not(feature = "protocol-xdg_shell"))] { None }
            },
            "xdg_wm_base" => {
                #[cfg(feature = "protocol-xdg_shell")] { Some(ProxyInterface::XdgWmBase) }
                #[cfg(not(feature = "protocol-xdg_shell"))] { None }
            },
            "xdg_system_bell_v1" => {
                #[cfg(feature = "protocol-xdg_system_bell_v1")] { Some(ProxyInterface::XdgSystemBellV1) }
                #[cfg(not(feature = "protocol-xdg_system_bell_v1"))] { None }
            },
            "xdg_toplevel_drag_manager_v1" => {
                #[cfg(feature = "protocol-xdg_toplevel_drag_v1")] { Some(ProxyInterface::XdgToplevelDragManagerV1) }
                #[cfg(not(feature = "protocol-xdg_toplevel_drag_v1"))] { None }
            },
            "xdg_toplevel_drag_v1" => {
                #[cfg(feature = "protocol-xdg_toplevel_drag_v1")] { Some(ProxyInterface::XdgToplevelDragV1) }
                #[cfg(not(feature = "protocol-xdg_toplevel_drag_v1"))] { None }
            },
            "xdg_toplevel_icon_manager_v1" => {
                #[cfg(feature = "protocol-xdg_toplevel_icon_v1")] { Some(ProxyInterface::XdgToplevelIconManagerV1) }
                #[cfg(not(feature = "protocol-xdg_toplevel_icon_v1"))] { None }
            },
            "xdg_toplevel_icon_v1" => {
                #[cfg(feature = "protocol-xdg_toplevel_icon_v1")] { Some(ProxyInterface::XdgToplevelIconV1) }
                #[cfg(not(feature = "protocol-xdg_toplevel_icon_v1"))] { None }
            },
            "xdg_toplevel_tag_manager_v1" => {
                #[cfg(feature = "protocol-xdg_toplevel_tag_v1")] { Some(ProxyInterface::XdgToplevelTagManagerV1) }
                #[cfg(not(feature = "protocol-xdg_toplevel_tag_v1"))] { None }
            },
            "zwp_xwayland_keyboard_grab_manager_v1" => {
                #[cfg(feature = "protocol-xwayland_keyboard_grab_unstable_v1")] { Some(ProxyInterface::ZwpXwaylandKeyboardGrabManagerV1) }
                #[cfg(not(feature = "protocol-xwayland_keyboard_grab_unstable_v1"))] { None }
            },
            "zwp_xwayland_keyboard_grab_v1" => {
                #[cfg(feature = "protocol-xwayland_keyboard_grab_unstable_v1")] { Some(ProxyInterface::ZwpXwaylandKeyboardGrabV1) }
                #[cfg(not(feature = "protocol-xwayland_keyboard_grab_unstable_v1"))] { None }
            },
            "xwayland_shell_v1" => {
                #[cfg(feature = "protocol-xwayland_shell_v1")] { Some(ProxyInterface::XwaylandShellV1) }
                #[cfg(not(feature = "protocol-xwayland_shell_v1"))] { None }
            },
            "xwayland_surface_v1" => {
                #[cfg(feature = "protocol-xwayland_shell_v1")] { Some(ProxyInterface::XwaylandSurfaceV1) }
                #[cfg(not(feature = "protocol-xwayland_shell_v1"))] { None }
            },
            "zwp_linux_buffer_release_v1" => {
                #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")] { Some(ProxyInterface::ZwpLinuxBufferReleaseV1) }
                #[cfg(not(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1"))] { None }
            },
            "zwp_linux_explicit_synchronization_v1" => {
                #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")] { Some(ProxyInterface::ZwpLinuxExplicitSynchronizationV1) }
                #[cfg(not(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1"))] { None }
            },
            "zwp_linux_surface_synchronization_v1" => {
                #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")] { Some(ProxyInterface::ZwpLinuxSurfaceSynchronizationV1) }
                #[cfg(not(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1"))] { None }
            },
            "zwlr_data_control_device_v1" => {
                #[cfg(feature = "protocol-wlr_data_control_unstable_v1")] { Some(ProxyInterface::ZwlrDataControlDeviceV1) }
                #[cfg(not(feature = "protocol-wlr_data_control_unstable_v1"))] { None }
            },
            "zwlr_data_control_manager_v1" => {
                #[cfg(feature = "protocol-wlr_data_control_unstable_v1")] { Some(ProxyInterface::ZwlrDataControlManagerV1) }
                #[cfg(not(feature = "protocol-wlr_data_control_unstable_v1"))] { None }
            },
            "zwlr_data_control_offer_v1" => {
                #[cfg(feature = "protocol-wlr_data_control_unstable_v1")] { Some(ProxyInterface::ZwlrDataControlOfferV1) }
                #[cfg(not(feature = "protocol-wlr_data_control_unstable_v1"))] { None }
            },
            "zwlr_data_control_source_v1" => {
                #[cfg(feature = "protocol-wlr_data_control_unstable_v1")] { Some(ProxyInterface::ZwlrDataControlSourceV1) }
                #[cfg(not(feature = "protocol-wlr_data_control_unstable_v1"))] { None }
            },
            "zwlr_export_dmabuf_frame_v1" => {
                #[cfg(feature = "protocol-wlr_export_dmabuf_unstable_v1")] { Some(ProxyInterface::ZwlrExportDmabufFrameV1) }
                #[cfg(not(feature = "protocol-wlr_export_dmabuf_unstable_v1"))] { None }
            },
            "zwlr_export_dmabuf_manager_v1" => {
                #[cfg(feature = "protocol-wlr_export_dmabuf_unstable_v1")] { Some(ProxyInterface::ZwlrExportDmabufManagerV1) }
                #[cfg(not(feature = "protocol-wlr_export_dmabuf_unstable_v1"))] { None }
            },
            "zwlr_foreign_toplevel_handle_v1" => {
                #[cfg(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1")] { Some(ProxyInterface::ZwlrForeignToplevelHandleV1) }
                #[cfg(not(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1"))] { None }
            },
            "zwlr_foreign_toplevel_manager_v1" => {
                #[cfg(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1")] { Some(ProxyInterface::ZwlrForeignToplevelManagerV1) }
                #[cfg(not(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1"))] { None }
            },
            "zwlr_gamma_control_manager_v1" => {
                #[cfg(feature = "protocol-wlr_gamma_control_unstable_v1")] { Some(ProxyInterface::ZwlrGammaControlManagerV1) }
                #[cfg(not(feature = "protocol-wlr_gamma_control_unstable_v1"))] { None }
            },
            "zwlr_gamma_control_v1" => {
                #[cfg(feature = "protocol-wlr_gamma_control_unstable_v1")] { Some(ProxyInterface::ZwlrGammaControlV1) }
                #[cfg(not(feature = "protocol-wlr_gamma_control_unstable_v1"))] { None }
            },
            "zwlr_input_inhibit_manager_v1" => {
                #[cfg(feature = "protocol-wlr_input_inhibit_unstable_v1")] { Some(ProxyInterface::ZwlrInputInhibitManagerV1) }
                #[cfg(not(feature = "protocol-wlr_input_inhibit_unstable_v1"))] { None }
            },
            "zwlr_input_inhibitor_v1" => {
                #[cfg(feature = "protocol-wlr_input_inhibit_unstable_v1")] { Some(ProxyInterface::ZwlrInputInhibitorV1) }
                #[cfg(not(feature = "protocol-wlr_input_inhibit_unstable_v1"))] { None }
            },
            "zwlr_layer_shell_v1" => {
                #[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")] { Some(ProxyInterface::ZwlrLayerShellV1) }
                #[cfg(not(feature = "protocol-wlr_layer_shell_unstable_v1"))] { None }
            },
            "zwlr_layer_surface_v1" => {
                #[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")] { Some(ProxyInterface::ZwlrLayerSurfaceV1) }
                #[cfg(not(feature = "protocol-wlr_layer_shell_unstable_v1"))] { None }
            },
            "zwlr_output_configuration_head_v1" => {
                #[cfg(feature = "protocol-wlr_output_management_unstable_v1")] { Some(ProxyInterface::ZwlrOutputConfigurationHeadV1) }
                #[cfg(not(feature = "protocol-wlr_output_management_unstable_v1"))] { None }
            },
            "zwlr_output_configuration_v1" => {
                #[cfg(feature = "protocol-wlr_output_management_unstable_v1")] { Some(ProxyInterface::ZwlrOutputConfigurationV1) }
                #[cfg(not(feature = "protocol-wlr_output_management_unstable_v1"))] { None }
            },
            "zwlr_output_head_v1" => {
                #[cfg(feature = "protocol-wlr_output_management_unstable_v1")] { Some(ProxyInterface::ZwlrOutputHeadV1) }
                #[cfg(not(feature = "protocol-wlr_output_management_unstable_v1"))] { None }
            },
            "zwlr_output_manager_v1" => {
                #[cfg(feature = "protocol-wlr_output_management_unstable_v1")] { Some(ProxyInterface::ZwlrOutputManagerV1) }
                #[cfg(not(feature = "protocol-wlr_output_management_unstable_v1"))] { None }
            },
            "zwlr_output_mode_v1" => {
                #[cfg(feature = "protocol-wlr_output_management_unstable_v1")] { Some(ProxyInterface::ZwlrOutputModeV1) }
                #[cfg(not(feature = "protocol-wlr_output_management_unstable_v1"))] { None }
            },
            "zwlr_output_power_manager_v1" => {
                #[cfg(feature = "protocol-wlr_output_power_management_unstable_v1")] { Some(ProxyInterface::ZwlrOutputPowerManagerV1) }
                #[cfg(not(feature = "protocol-wlr_output_power_management_unstable_v1"))] { None }
            },
            "zwlr_output_power_v1" => {
                #[cfg(feature = "protocol-wlr_output_power_management_unstable_v1")] { Some(ProxyInterface::ZwlrOutputPowerV1) }
                #[cfg(not(feature = "protocol-wlr_output_power_management_unstable_v1"))] { None }
            },
            "zwlr_screencopy_frame_v1" => {
                #[cfg(feature = "protocol-wlr_screencopy_unstable_v1")] { Some(ProxyInterface::ZwlrScreencopyFrameV1) }
                #[cfg(not(feature = "protocol-wlr_screencopy_unstable_v1"))] { None }
            },
            "zwlr_screencopy_manager_v1" => {
                #[cfg(feature = "protocol-wlr_screencopy_unstable_v1")] { Some(ProxyInterface::ZwlrScreencopyManagerV1) }
                #[cfg(not(feature = "protocol-wlr_screencopy_unstable_v1"))] { None }
            },
            "zwlr_virtual_pointer_manager_v1" => {
                #[cfg(feature = "protocol-wlr_virtual_pointer_unstable_v1")] { Some(ProxyInterface::ZwlrVirtualPointerManagerV1) }
                #[cfg(not(feature = "protocol-wlr_virtual_pointer_unstable_v1"))] { None }
            },
            "zwlr_virtual_pointer_v1" => {
                #[cfg(feature = "protocol-wlr_virtual_pointer_unstable_v1")] { Some(ProxyInterface::ZwlrVirtualPointerV1) }
                #[cfg(not(feature = "protocol-wlr_virtual_pointer_unstable_v1"))] { None }
            },
        };
        INTERFACES.get(interface).copied().flatten()
    }

    impl ProxyInterface {
        fn create_proxy(self, state: &Rc<State>, version: u32) -> Result<Rc<dyn Proxy>, ObjectError> {
            match self {
                #[cfg(feature = "protocol-hyprland_ctm_control_v1")]
                Self::HyprlandCtmControlManagerV1 => {
                    if version > HyprlandCtmControlManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(HyprlandCtmControlManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_focus_grab_v1")]
                Self::HyprlandFocusGrabManagerV1 => {
                    if version > HyprlandFocusGrabManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(HyprlandFocusGrabManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_focus_grab_v1")]
                Self::HyprlandFocusGrabV1 => {
                    if version > HyprlandFocusGrabV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(HyprlandFocusGrabV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_global_shortcuts_v1")]
                Self::HyprlandGlobalShortcutV1 => {
                    if version > HyprlandGlobalShortcutV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(HyprlandGlobalShortcutV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_global_shortcuts_v1")]
                Self::HyprlandGlobalShortcutsManagerV1 => {
                    if version > HyprlandGlobalShortcutsManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(HyprlandGlobalShortcutsManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_input_capture_v1")]
                Self::HyprlandInputCaptureManagerV1 => {
                    if version > HyprlandInputCaptureManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(HyprlandInputCaptureManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_input_capture_v1")]
                Self::HyprlandInputCaptureV1 => {
                    if version > HyprlandInputCaptureV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(HyprlandInputCaptureV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_lock_notify_v1")]
                Self::HyprlandLockNotificationV1 => {
                    if version > HyprlandLockNotificationV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(HyprlandLockNotificationV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_lock_notify_v1")]
                Self::HyprlandLockNotifierV1 => {
                    if version > HyprlandLockNotifierV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(HyprlandLockNotifierV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_surface_v1")]
                Self::HyprlandSurfaceManagerV1 => {
                    if version > HyprlandSurfaceManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(HyprlandSurfaceManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_surface_v1")]
                Self::HyprlandSurfaceV1 => {
                    if version > HyprlandSurfaceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(HyprlandSurfaceV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_toplevel_export_v1")]
                Self::HyprlandToplevelExportFrameV1 => {
                    if version > HyprlandToplevelExportFrameV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(HyprlandToplevelExportFrameV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_toplevel_export_v1")]
                Self::HyprlandToplevelExportManagerV1 => {
                    if version > HyprlandToplevelExportManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(HyprlandToplevelExportManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_toplevel_mapping_v1")]
                Self::HyprlandToplevelMappingManagerV1 => {
                    if version > HyprlandToplevelMappingManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(HyprlandToplevelMappingManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_toplevel_mapping_v1")]
                Self::HyprlandToplevelWindowMappingHandleV1 => {
                    if version > HyprlandToplevelWindowMappingHandleV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(HyprlandToplevelWindowMappingHandleV1::new(state, version))
                }
                #[cfg(feature = "protocol-jay_tray_v1")]
                Self::JayTrayItemV1 => {
                    if version > JayTrayItemV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(JayTrayItemV1::new(state, version))
                }
                #[cfg(feature = "protocol-jay_tray_v1")]
                Self::JayTrayV1 => {
                    if version > JayTrayV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(JayTrayV1::new(state, version))
                }
                #[cfg(feature = "protocol-drm")]
                Self::WlDrm => {
                    if version > WlDrm::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlDrm::new(state, version))
                }
                #[cfg(feature = "protocol-input_method_unstable_v2")]
                Self::ZwpInputMethodKeyboardGrabV2 => {
                    if version > ZwpInputMethodKeyboardGrabV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpInputMethodKeyboardGrabV2::new(state, version))
                }
                #[cfg(feature = "protocol-input_method_unstable_v2")]
                Self::ZwpInputMethodManagerV2 => {
                    if version > ZwpInputMethodManagerV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpInputMethodManagerV2::new(state, version))
                }
                #[cfg(feature = "protocol-input_method_unstable_v2")]
                Self::ZwpInputMethodV2 => {
                    if version > ZwpInputMethodV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpInputMethodV2::new(state, version))
                }
                #[cfg(feature = "protocol-input_method_unstable_v2")]
                Self::ZwpInputPopupSurfaceV2 => {
                    if version > ZwpInputPopupSurfaceV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpInputPopupSurfaceV2::new(state, version))
                }
                #[cfg(feature = "protocol-org_kde_kwin_server_decoration_v1")]
                Self::OrgKdeKwinServerDecoration => {
                    if version > OrgKdeKwinServerDecoration::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(OrgKdeKwinServerDecoration::new(state, version))
                }
                #[cfg(feature = "protocol-org_kde_kwin_server_decoration_v1")]
                Self::OrgKdeKwinServerDecorationManager => {
                    if version > OrgKdeKwinServerDecorationManager::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(OrgKdeKwinServerDecorationManager::new(state, version))
                }
                #[cfg(feature = "protocol-virtual_keyboard_unstable_v1")]
                Self::ZwpVirtualKeyboardManagerV1 => {
                    if version > ZwpVirtualKeyboardManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpVirtualKeyboardManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-virtual_keyboard_unstable_v1")]
                Self::ZwpVirtualKeyboardV1 => {
                    if version > ZwpVirtualKeyboardV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpVirtualKeyboardV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_app_id_resolver_v1")]
                Self::TreelandAppIdResolverManagerV1 => {
                    if version > TreelandAppIdResolverManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandAppIdResolverManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_app_id_resolver_v1")]
                Self::TreelandAppIdResolverV1 => {
                    if version > TreelandAppIdResolverV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandAppIdResolverV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_dde_shell_v1")]
                Self::TreelandDdeActiveV1 => {
                    if version > TreelandDdeActiveV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandDdeActiveV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_dde_shell_v1")]
                Self::TreelandDdeShellManagerV1 => {
                    if version > TreelandDdeShellManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandDdeShellManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_dde_shell_v1")]
                Self::TreelandDdeShellSurfaceV1 => {
                    if version > TreelandDdeShellSurfaceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandDdeShellSurfaceV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_dde_shell_v1")]
                Self::TreelandLockscreenV1 => {
                    if version > TreelandLockscreenV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandLockscreenV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_dde_shell_v1")]
                Self::TreelandMultitaskviewV1 => {
                    if version > TreelandMultitaskviewV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandMultitaskviewV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_dde_shell_v1")]
                Self::TreelandWindowOverlapChecker => {
                    if version > TreelandWindowOverlapChecker::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandWindowOverlapChecker::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_dde_shell_v1")]
                Self::TreelandWindowPickerV1 => {
                    if version > TreelandWindowPickerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandWindowPickerV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_ddm")]
                Self::TreelandDdm => {
                    if version > TreelandDdm::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandDdm::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_foreign_toplevel_manager_v1")]
                Self::TreelandDockPreviewContextV1 => {
                    if version > TreelandDockPreviewContextV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandDockPreviewContextV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_foreign_toplevel_manager_v1")]
                Self::TreelandForeignToplevelHandleV1 => {
                    if version > TreelandForeignToplevelHandleV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandForeignToplevelHandleV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_foreign_toplevel_manager_v1")]
                Self::TreelandForeignToplevelManagerV1 => {
                    if version > TreelandForeignToplevelManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandForeignToplevelManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_output_manager_v1")]
                Self::TreelandOutputColorControlV1 => {
                    if version > TreelandOutputColorControlV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandOutputColorControlV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_output_manager_v1")]
                Self::TreelandOutputManagerV1 => {
                    if version > TreelandOutputManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandOutputManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
                Self::TreelandPersonalizationAppearanceContextV1 => {
                    if version > TreelandPersonalizationAppearanceContextV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandPersonalizationAppearanceContextV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
                Self::TreelandPersonalizationCursorContextV1 => {
                    if version > TreelandPersonalizationCursorContextV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandPersonalizationCursorContextV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
                Self::TreelandPersonalizationFontContextV1 => {
                    if version > TreelandPersonalizationFontContextV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandPersonalizationFontContextV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
                Self::TreelandPersonalizationManagerV1 => {
                    if version > TreelandPersonalizationManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandPersonalizationManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
                Self::TreelandPersonalizationWallpaperContextV1 => {
                    if version > TreelandPersonalizationWallpaperContextV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandPersonalizationWallpaperContextV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
                Self::TreelandPersonalizationWindowContextV1 => {
                    if version > TreelandPersonalizationWindowContextV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandPersonalizationWindowContextV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_prelaunch_splash_v1")]
                Self::TreelandPrelaunchSplashManagerV1 => {
                    if version > TreelandPrelaunchSplashManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandPrelaunchSplashManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_screensaver")]
                Self::TreelandScreensaver => {
                    if version > TreelandScreensaver::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandScreensaver::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_shortcut_manager_v1")]
                Self::TreelandShortcutContextV1 => {
                    if version > TreelandShortcutContextV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandShortcutContextV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_shortcut_manager_v1")]
                Self::TreelandShortcutManagerV1 => {
                    if version > TreelandShortcutManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandShortcutManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_shortcut_manager_v2")]
                Self::TreelandShortcutManagerV2 => {
                    if version > TreelandShortcutManagerV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandShortcutManagerV2::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_virtual_output_manager_v1")]
                Self::TreelandVirtualOutputManagerV1 => {
                    if version > TreelandVirtualOutputManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandVirtualOutputManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_virtual_output_manager_v1")]
                Self::TreelandVirtualOutputV1 => {
                    if version > TreelandVirtualOutputV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandVirtualOutputV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_wallpaper_color_v1")]
                Self::TreelandWallpaperColorManagerV1 => {
                    if version > TreelandWallpaperColorManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandWallpaperColorManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-treeland_window_management_v1")]
                Self::TreelandWindowManagementV1 => {
                    if version > TreelandWindowManagementV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(TreelandWindowManagementV1::new(state, version))
                }
                Self::WlBuffer => {
                    if version > WlBuffer::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlBuffer::new(state, version))
                }
                Self::WlCallback => {
                    if version > WlCallback::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlCallback::new(state, version))
                }
                Self::WlCompositor => {
                    if version > WlCompositor::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlCompositor::new(state, version))
                }
                Self::WlDataDevice => {
                    if version > WlDataDevice::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlDataDevice::new(state, version))
                }
                Self::WlDataDeviceManager => {
                    if version > WlDataDeviceManager::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlDataDeviceManager::new(state, version))
                }
                Self::WlDataOffer => {
                    if version > WlDataOffer::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlDataOffer::new(state, version))
                }
                Self::WlDataSource => {
                    if version > WlDataSource::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlDataSource::new(state, version))
                }
                Self::WlDisplay => {
                    if version > WlDisplay::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlDisplay::new(state, version))
                }
                Self::WlFixes => {
                    if version > WlFixes::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlFixes::new(state, version))
                }
                Self::WlKeyboard => {
                    if version > WlKeyboard::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlKeyboard::new(state, version))
                }
                Self::WlOutput => {
                    if version > WlOutput::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlOutput::new(state, version))
                }
                Self::WlPointer => {
                    if version > WlPointer::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlPointer::new(state, version))
                }
                Self::WlRegion => {
                    if version > WlRegion::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlRegion::new(state, version))
                }
                Self::WlRegistry => {
                    if version > WlRegistry::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlRegistry::new(state, version))
                }
                Self::WlSeat => {
                    if version > WlSeat::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlSeat::new(state, version))
                }
                Self::WlShell => {
                    if version > WlShell::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlShell::new(state, version))
                }
                Self::WlShellSurface => {
                    if version > WlShellSurface::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlShellSurface::new(state, version))
                }
                Self::WlShm => {
                    if version > WlShm::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlShm::new(state, version))
                }
                Self::WlShmPool => {
                    if version > WlShmPool::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlShmPool::new(state, version))
                }
                Self::WlSubcompositor => {
                    if version > WlSubcompositor::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlSubcompositor::new(state, version))
                }
                Self::WlSubsurface => {
                    if version > WlSubsurface::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlSubsurface::new(state, version))
                }
                Self::WlSurface => {
                    if version > WlSurface::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlSurface::new(state, version))
                }
                Self::WlTouch => {
                    if version > WlTouch::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WlTouch::new(state, version))
                }
                #[cfg(feature = "protocol-alpha_modifier_v1")]
                Self::WpAlphaModifierSurfaceV1 => {
                    if version > WpAlphaModifierSurfaceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpAlphaModifierSurfaceV1::new(state, version))
                }
                #[cfg(feature = "protocol-alpha_modifier_v1")]
                Self::WpAlphaModifierV1 => {
                    if version > WpAlphaModifierV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpAlphaModifierV1::new(state, version))
                }
                #[cfg(feature = "protocol-color_management_v1")]
                Self::WpColorManagementOutputV1 => {
                    if version > WpColorManagementOutputV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpColorManagementOutputV1::new(state, version))
                }
                #[cfg(feature = "protocol-color_management_v1")]
                Self::WpColorManagementSurfaceFeedbackV1 => {
                    if version > WpColorManagementSurfaceFeedbackV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpColorManagementSurfaceFeedbackV1::new(state, version))
                }
                #[cfg(feature = "protocol-color_management_v1")]
                Self::WpColorManagementSurfaceV1 => {
                    if version > WpColorManagementSurfaceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpColorManagementSurfaceV1::new(state, version))
                }
                #[cfg(feature = "protocol-color_management_v1")]
                Self::WpColorManagerV1 => {
                    if version > WpColorManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpColorManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-color_management_v1")]
                Self::WpImageDescriptionCreatorIccV1 => {
                    if version > WpImageDescriptionCreatorIccV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpImageDescriptionCreatorIccV1::new(state, version))
                }
                #[cfg(feature = "protocol-color_management_v1")]
                Self::WpImageDescriptionCreatorParamsV1 => {
                    if version > WpImageDescriptionCreatorParamsV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpImageDescriptionCreatorParamsV1::new(state, version))
                }
                #[cfg(feature = "protocol-color_management_v1")]
                Self::WpImageDescriptionInfoV1 => {
                    if version > WpImageDescriptionInfoV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpImageDescriptionInfoV1::new(state, version))
                }
                #[cfg(feature = "protocol-color_management_v1")]
                Self::WpImageDescriptionV1 => {
                    if version > WpImageDescriptionV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpImageDescriptionV1::new(state, version))
                }
                #[cfg(feature = "protocol-color_representation_v1")]
                Self::WpColorRepresentationManagerV1 => {
                    if version > WpColorRepresentationManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpColorRepresentationManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-color_representation_v1")]
                Self::WpColorRepresentationSurfaceV1 => {
                    if version > WpColorRepresentationSurfaceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpColorRepresentationSurfaceV1::new(state, version))
                }
                #[cfg(feature = "protocol-commit_timing_v1")]
                Self::WpCommitTimerV1 => {
                    if version > WpCommitTimerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpCommitTimerV1::new(state, version))
                }
                #[cfg(feature = "protocol-commit_timing_v1")]
                Self::WpCommitTimingManagerV1 => {
                    if version > WpCommitTimingManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpCommitTimingManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-content_type_v1")]
                Self::WpContentTypeManagerV1 => {
                    if version > WpContentTypeManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpContentTypeManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-content_type_v1")]
                Self::WpContentTypeV1 => {
                    if version > WpContentTypeV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpContentTypeV1::new(state, version))
                }
                #[cfg(feature = "protocol-cursor_shape_v1")]
                Self::WpCursorShapeDeviceV1 => {
                    if version > WpCursorShapeDeviceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpCursorShapeDeviceV1::new(state, version))
                }
                #[cfg(feature = "protocol-cursor_shape_v1")]
                Self::WpCursorShapeManagerV1 => {
                    if version > WpCursorShapeManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpCursorShapeManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-drm_lease_v1")]
                Self::WpDrmLeaseConnectorV1 => {
                    if version > WpDrmLeaseConnectorV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpDrmLeaseConnectorV1::new(state, version))
                }
                #[cfg(feature = "protocol-drm_lease_v1")]
                Self::WpDrmLeaseDeviceV1 => {
                    if version > WpDrmLeaseDeviceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpDrmLeaseDeviceV1::new(state, version))
                }
                #[cfg(feature = "protocol-drm_lease_v1")]
                Self::WpDrmLeaseRequestV1 => {
                    if version > WpDrmLeaseRequestV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpDrmLeaseRequestV1::new(state, version))
                }
                #[cfg(feature = "protocol-drm_lease_v1")]
                Self::WpDrmLeaseV1 => {
                    if version > WpDrmLeaseV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpDrmLeaseV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_background_effect_v1")]
                Self::ExtBackgroundEffectManagerV1 => {
                    if version > ExtBackgroundEffectManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtBackgroundEffectManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_background_effect_v1")]
                Self::ExtBackgroundEffectSurfaceV1 => {
                    if version > ExtBackgroundEffectSurfaceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtBackgroundEffectSurfaceV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_data_control_v1")]
                Self::ExtDataControlDeviceV1 => {
                    if version > ExtDataControlDeviceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtDataControlDeviceV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_data_control_v1")]
                Self::ExtDataControlManagerV1 => {
                    if version > ExtDataControlManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtDataControlManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_data_control_v1")]
                Self::ExtDataControlOfferV1 => {
                    if version > ExtDataControlOfferV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtDataControlOfferV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_data_control_v1")]
                Self::ExtDataControlSourceV1 => {
                    if version > ExtDataControlSourceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtDataControlSourceV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_foreign_toplevel_list_v1")]
                Self::ExtForeignToplevelHandleV1 => {
                    if version > ExtForeignToplevelHandleV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtForeignToplevelHandleV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_foreign_toplevel_list_v1")]
                Self::ExtForeignToplevelListV1 => {
                    if version > ExtForeignToplevelListV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtForeignToplevelListV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_idle_notify_v1")]
                Self::ExtIdleNotificationV1 => {
                    if version > ExtIdleNotificationV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtIdleNotificationV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_idle_notify_v1")]
                Self::ExtIdleNotifierV1 => {
                    if version > ExtIdleNotifierV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtIdleNotifierV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_image_capture_source_v1")]
                Self::ExtForeignToplevelImageCaptureSourceManagerV1 => {
                    if version > ExtForeignToplevelImageCaptureSourceManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtForeignToplevelImageCaptureSourceManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_image_capture_source_v1")]
                Self::ExtImageCaptureSourceV1 => {
                    if version > ExtImageCaptureSourceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtImageCaptureSourceV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_image_capture_source_v1")]
                Self::ExtOutputImageCaptureSourceManagerV1 => {
                    if version > ExtOutputImageCaptureSourceManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtOutputImageCaptureSourceManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
                Self::ExtImageCopyCaptureCursorSessionV1 => {
                    if version > ExtImageCopyCaptureCursorSessionV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtImageCopyCaptureCursorSessionV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
                Self::ExtImageCopyCaptureFrameV1 => {
                    if version > ExtImageCopyCaptureFrameV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtImageCopyCaptureFrameV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
                Self::ExtImageCopyCaptureManagerV1 => {
                    if version > ExtImageCopyCaptureManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtImageCopyCaptureManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
                Self::ExtImageCopyCaptureSessionV1 => {
                    if version > ExtImageCopyCaptureSessionV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtImageCopyCaptureSessionV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_session_lock_v1")]
                Self::ExtSessionLockManagerV1 => {
                    if version > ExtSessionLockManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtSessionLockManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_session_lock_v1")]
                Self::ExtSessionLockSurfaceV1 => {
                    if version > ExtSessionLockSurfaceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtSessionLockSurfaceV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_session_lock_v1")]
                Self::ExtSessionLockV1 => {
                    if version > ExtSessionLockV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtSessionLockV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_transient_seat_v1")]
                Self::ExtTransientSeatManagerV1 => {
                    if version > ExtTransientSeatManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtTransientSeatManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_transient_seat_v1")]
                Self::ExtTransientSeatV1 => {
                    if version > ExtTransientSeatV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtTransientSeatV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_workspace_v1")]
                Self::ExtWorkspaceGroupHandleV1 => {
                    if version > ExtWorkspaceGroupHandleV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtWorkspaceGroupHandleV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_workspace_v1")]
                Self::ExtWorkspaceHandleV1 => {
                    if version > ExtWorkspaceHandleV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtWorkspaceHandleV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_workspace_v1")]
                Self::ExtWorkspaceManagerV1 => {
                    if version > ExtWorkspaceManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtWorkspaceManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-fifo_v1")]
                Self::WpFifoManagerV1 => {
                    if version > WpFifoManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpFifoManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-fifo_v1")]
                Self::WpFifoV1 => {
                    if version > WpFifoV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpFifoV1::new(state, version))
                }
                #[cfg(feature = "protocol-fractional_scale_v1")]
                Self::WpFractionalScaleManagerV1 => {
                    if version > WpFractionalScaleManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpFractionalScaleManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-fractional_scale_v1")]
                Self::WpFractionalScaleV1 => {
                    if version > WpFractionalScaleV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpFractionalScaleV1::new(state, version))
                }
                #[cfg(feature = "protocol-fullscreen_shell_unstable_v1")]
                Self::ZwpFullscreenShellModeFeedbackV1 => {
                    if version > ZwpFullscreenShellModeFeedbackV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpFullscreenShellModeFeedbackV1::new(state, version))
                }
                #[cfg(feature = "protocol-fullscreen_shell_unstable_v1")]
                Self::ZwpFullscreenShellV1 => {
                    if version > ZwpFullscreenShellV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpFullscreenShellV1::new(state, version))
                }
                #[cfg(feature = "protocol-idle_inhibit_unstable_v1")]
                Self::ZwpIdleInhibitManagerV1 => {
                    if version > ZwpIdleInhibitManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpIdleInhibitManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-idle_inhibit_unstable_v1")]
                Self::ZwpIdleInhibitorV1 => {
                    if version > ZwpIdleInhibitorV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpIdleInhibitorV1::new(state, version))
                }
                #[cfg(feature = "protocol-input_method_unstable_v1")]
                Self::ZwpInputMethodContextV1 => {
                    if version > ZwpInputMethodContextV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpInputMethodContextV1::new(state, version))
                }
                #[cfg(feature = "protocol-input_method_unstable_v1")]
                Self::ZwpInputMethodV1 => {
                    if version > ZwpInputMethodV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpInputMethodV1::new(state, version))
                }
                #[cfg(feature = "protocol-input_method_unstable_v1")]
                Self::ZwpInputPanelSurfaceV1 => {
                    if version > ZwpInputPanelSurfaceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpInputPanelSurfaceV1::new(state, version))
                }
                #[cfg(feature = "protocol-input_method_unstable_v1")]
                Self::ZwpInputPanelV1 => {
                    if version > ZwpInputPanelV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpInputPanelV1::new(state, version))
                }
                #[cfg(feature = "protocol-input_timestamps_unstable_v1")]
                Self::ZwpInputTimestampsManagerV1 => {
                    if version > ZwpInputTimestampsManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpInputTimestampsManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-input_timestamps_unstable_v1")]
                Self::ZwpInputTimestampsV1 => {
                    if version > ZwpInputTimestampsV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpInputTimestampsV1::new(state, version))
                }
                #[cfg(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1")]
                Self::ZwpKeyboardShortcutsInhibitManagerV1 => {
                    if version > ZwpKeyboardShortcutsInhibitManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpKeyboardShortcutsInhibitManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1")]
                Self::ZwpKeyboardShortcutsInhibitorV1 => {
                    if version > ZwpKeyboardShortcutsInhibitorV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpKeyboardShortcutsInhibitorV1::new(state, version))
                }
                #[cfg(feature = "protocol-linux_dmabuf_v1")]
                Self::ZwpLinuxBufferParamsV1 => {
                    if version > ZwpLinuxBufferParamsV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpLinuxBufferParamsV1::new(state, version))
                }
                #[cfg(feature = "protocol-linux_dmabuf_v1")]
                Self::ZwpLinuxDmabufFeedbackV1 => {
                    if version > ZwpLinuxDmabufFeedbackV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpLinuxDmabufFeedbackV1::new(state, version))
                }
                #[cfg(feature = "protocol-linux_dmabuf_v1")]
                Self::ZwpLinuxDmabufV1 => {
                    if version > ZwpLinuxDmabufV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpLinuxDmabufV1::new(state, version))
                }
                #[cfg(feature = "protocol-linux_drm_syncobj_v1")]
                Self::WpLinuxDrmSyncobjManagerV1 => {
                    if version > WpLinuxDrmSyncobjManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpLinuxDrmSyncobjManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-linux_drm_syncobj_v1")]
                Self::WpLinuxDrmSyncobjSurfaceV1 => {
                    if version > WpLinuxDrmSyncobjSurfaceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpLinuxDrmSyncobjSurfaceV1::new(state, version))
                }
                #[cfg(feature = "protocol-linux_drm_syncobj_v1")]
                Self::WpLinuxDrmSyncobjTimelineV1 => {
                    if version > WpLinuxDrmSyncobjTimelineV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpLinuxDrmSyncobjTimelineV1::new(state, version))
                }
                #[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
                Self::ZwpConfinedPointerV1 => {
                    if version > ZwpConfinedPointerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpConfinedPointerV1::new(state, version))
                }
                #[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
                Self::ZwpLockedPointerV1 => {
                    if version > ZwpLockedPointerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpLockedPointerV1::new(state, version))
                }
                #[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
                Self::ZwpPointerConstraintsV1 => {
                    if version > ZwpPointerConstraintsV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpPointerConstraintsV1::new(state, version))
                }
                #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
                Self::ZwpPointerGestureHoldV1 => {
                    if version > ZwpPointerGestureHoldV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpPointerGestureHoldV1::new(state, version))
                }
                #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
                Self::ZwpPointerGesturePinchV1 => {
                    if version > ZwpPointerGesturePinchV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpPointerGesturePinchV1::new(state, version))
                }
                #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
                Self::ZwpPointerGestureSwipeV1 => {
                    if version > ZwpPointerGestureSwipeV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpPointerGestureSwipeV1::new(state, version))
                }
                #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
                Self::ZwpPointerGesturesV1 => {
                    if version > ZwpPointerGesturesV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpPointerGesturesV1::new(state, version))
                }
                #[cfg(feature = "protocol-pointer_warp_v1")]
                Self::WpPointerWarpV1 => {
                    if version > WpPointerWarpV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpPointerWarpV1::new(state, version))
                }
                #[cfg(feature = "protocol-presentation_time")]
                Self::WpPresentation => {
                    if version > WpPresentation::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpPresentation::new(state, version))
                }
                #[cfg(feature = "protocol-presentation_time")]
                Self::WpPresentationFeedback => {
                    if version > WpPresentationFeedback::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpPresentationFeedback::new(state, version))
                }
                #[cfg(feature = "protocol-relative_pointer_unstable_v1")]
                Self::ZwpRelativePointerManagerV1 => {
                    if version > ZwpRelativePointerManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpRelativePointerManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-relative_pointer_unstable_v1")]
                Self::ZwpRelativePointerV1 => {
                    if version > ZwpRelativePointerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpRelativePointerV1::new(state, version))
                }
                #[cfg(feature = "protocol-security_context_v1")]
                Self::WpSecurityContextManagerV1 => {
                    if version > WpSecurityContextManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpSecurityContextManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-security_context_v1")]
                Self::WpSecurityContextV1 => {
                    if version > WpSecurityContextV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpSecurityContextV1::new(state, version))
                }
                #[cfg(feature = "protocol-single_pixel_buffer_v1")]
                Self::WpSinglePixelBufferManagerV1 => {
                    if version > WpSinglePixelBufferManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpSinglePixelBufferManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-tablet_v2")]
                Self::ZwpTabletManagerV2 => {
                    if version > ZwpTabletManagerV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTabletManagerV2::new(state, version))
                }
                #[cfg(feature = "protocol-tablet_v2")]
                Self::ZwpTabletPadDialV2 => {
                    if version > ZwpTabletPadDialV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTabletPadDialV2::new(state, version))
                }
                #[cfg(feature = "protocol-tablet_v2")]
                Self::ZwpTabletPadGroupV2 => {
                    if version > ZwpTabletPadGroupV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTabletPadGroupV2::new(state, version))
                }
                #[cfg(feature = "protocol-tablet_v2")]
                Self::ZwpTabletPadRingV2 => {
                    if version > ZwpTabletPadRingV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTabletPadRingV2::new(state, version))
                }
                #[cfg(feature = "protocol-tablet_v2")]
                Self::ZwpTabletPadStripV2 => {
                    if version > ZwpTabletPadStripV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTabletPadStripV2::new(state, version))
                }
                #[cfg(feature = "protocol-tablet_v2")]
                Self::ZwpTabletPadV2 => {
                    if version > ZwpTabletPadV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTabletPadV2::new(state, version))
                }
                #[cfg(feature = "protocol-tablet_v2")]
                Self::ZwpTabletSeatV2 => {
                    if version > ZwpTabletSeatV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTabletSeatV2::new(state, version))
                }
                #[cfg(feature = "protocol-tablet_v2")]
                Self::ZwpTabletToolV2 => {
                    if version > ZwpTabletToolV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTabletToolV2::new(state, version))
                }
                #[cfg(feature = "protocol-tablet_v2")]
                Self::ZwpTabletV2 => {
                    if version > ZwpTabletV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTabletV2::new(state, version))
                }
                #[cfg(feature = "protocol-tearing_control_v1")]
                Self::WpTearingControlManagerV1 => {
                    if version > WpTearingControlManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpTearingControlManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-tearing_control_v1")]
                Self::WpTearingControlV1 => {
                    if version > WpTearingControlV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpTearingControlV1::new(state, version))
                }
                #[cfg(feature = "protocol-text_input_unstable_v1")]
                Self::ZwpTextInputManagerV1 => {
                    if version > ZwpTextInputManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTextInputManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-text_input_unstable_v1")]
                Self::ZwpTextInputV1 => {
                    if version > ZwpTextInputV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTextInputV1::new(state, version))
                }
                #[cfg(feature = "protocol-text_input_unstable_v3")]
                Self::ZwpTextInputManagerV3 => {
                    if version > ZwpTextInputManagerV3::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTextInputManagerV3::new(state, version))
                }
                #[cfg(feature = "protocol-text_input_unstable_v3")]
                Self::ZwpTextInputV3 => {
                    if version > ZwpTextInputV3::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTextInputV3::new(state, version))
                }
                #[cfg(feature = "protocol-viewporter")]
                Self::WpViewport => {
                    if version > WpViewport::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpViewport::new(state, version))
                }
                #[cfg(feature = "protocol-viewporter")]
                Self::WpViewporter => {
                    if version > WpViewporter::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpViewporter::new(state, version))
                }
                #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
                Self::ZwpPrimarySelectionDeviceManagerV1 => {
                    if version > ZwpPrimarySelectionDeviceManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpPrimarySelectionDeviceManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
                Self::ZwpPrimarySelectionDeviceV1 => {
                    if version > ZwpPrimarySelectionDeviceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpPrimarySelectionDeviceV1::new(state, version))
                }
                #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
                Self::ZwpPrimarySelectionOfferV1 => {
                    if version > ZwpPrimarySelectionOfferV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpPrimarySelectionOfferV1::new(state, version))
                }
                #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
                Self::ZwpPrimarySelectionSourceV1 => {
                    if version > ZwpPrimarySelectionSourceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpPrimarySelectionSourceV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_activation_v1")]
                Self::XdgActivationTokenV1 => {
                    if version > XdgActivationTokenV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgActivationTokenV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_activation_v1")]
                Self::XdgActivationV1 => {
                    if version > XdgActivationV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgActivationV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_decoration_unstable_v1")]
                Self::ZxdgDecorationManagerV1 => {
                    if version > ZxdgDecorationManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZxdgDecorationManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_decoration_unstable_v1")]
                Self::ZxdgToplevelDecorationV1 => {
                    if version > ZxdgToplevelDecorationV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZxdgToplevelDecorationV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_dialog_v1")]
                Self::XdgDialogV1 => {
                    if version > XdgDialogV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgDialogV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_dialog_v1")]
                Self::XdgWmDialogV1 => {
                    if version > XdgWmDialogV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgWmDialogV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
                Self::ZxdgExportedV2 => {
                    if version > ZxdgExportedV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZxdgExportedV2::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
                Self::ZxdgExporterV2 => {
                    if version > ZxdgExporterV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZxdgExporterV2::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
                Self::ZxdgImportedV2 => {
                    if version > ZxdgImportedV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZxdgImportedV2::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
                Self::ZxdgImporterV2 => {
                    if version > ZxdgImporterV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZxdgImporterV2::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_output_unstable_v1")]
                Self::ZxdgOutputManagerV1 => {
                    if version > ZxdgOutputManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZxdgOutputManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_output_unstable_v1")]
                Self::ZxdgOutputV1 => {
                    if version > ZxdgOutputV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZxdgOutputV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_shell")]
                Self::XdgPopup => {
                    if version > XdgPopup::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgPopup::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_shell")]
                Self::XdgPositioner => {
                    if version > XdgPositioner::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgPositioner::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_shell")]
                Self::XdgSurface => {
                    if version > XdgSurface::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgSurface::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_shell")]
                Self::XdgToplevel => {
                    if version > XdgToplevel::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgToplevel::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_shell")]
                Self::XdgWmBase => {
                    if version > XdgWmBase::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgWmBase::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_system_bell_v1")]
                Self::XdgSystemBellV1 => {
                    if version > XdgSystemBellV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgSystemBellV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_toplevel_drag_v1")]
                Self::XdgToplevelDragManagerV1 => {
                    if version > XdgToplevelDragManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgToplevelDragManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_toplevel_drag_v1")]
                Self::XdgToplevelDragV1 => {
                    if version > XdgToplevelDragV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgToplevelDragV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_toplevel_icon_v1")]
                Self::XdgToplevelIconManagerV1 => {
                    if version > XdgToplevelIconManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgToplevelIconManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_toplevel_icon_v1")]
                Self::XdgToplevelIconV1 => {
                    if version > XdgToplevelIconV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgToplevelIconV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_toplevel_tag_v1")]
                Self::XdgToplevelTagManagerV1 => {
                    if version > XdgToplevelTagManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgToplevelTagManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-xwayland_keyboard_grab_unstable_v1")]
                Self::ZwpXwaylandKeyboardGrabManagerV1 => {
                    if version > ZwpXwaylandKeyboardGrabManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpXwaylandKeyboardGrabManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-xwayland_keyboard_grab_unstable_v1")]
                Self::ZwpXwaylandKeyboardGrabV1 => {
                    if version > ZwpXwaylandKeyboardGrabV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpXwaylandKeyboardGrabV1::new(state, version))
                }
                #[cfg(feature = "protocol-xwayland_shell_v1")]
                Self::XwaylandShellV1 => {
                    if version > XwaylandShellV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XwaylandShellV1::new(state, version))
                }
                #[cfg(feature = "protocol-xwayland_shell_v1")]
                Self::XwaylandSurfaceV1 => {
                    if version > XwaylandSurfaceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XwaylandSurfaceV1::new(state, version))
                }
                #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
                Self::ZwpLinuxBufferReleaseV1 => {
                    if version > ZwpLinuxBufferReleaseV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpLinuxBufferReleaseV1::new(state, version))
                }
                #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
                Self::ZwpLinuxExplicitSynchronizationV1 => {
                    if version > ZwpLinuxExplicitSynchronizationV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpLinuxExplicitSynchronizationV1::new(state, version))
                }
                #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
                Self::ZwpLinuxSurfaceSynchronizationV1 => {
                    if version > ZwpLinuxSurfaceSynchronizationV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpLinuxSurfaceSynchronizationV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
                Self::ZwlrDataControlDeviceV1 => {
                    if version > ZwlrDataControlDeviceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrDataControlDeviceV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
                Self::ZwlrDataControlManagerV1 => {
                    if version > ZwlrDataControlManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrDataControlManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
                Self::ZwlrDataControlOfferV1 => {
                    if version > ZwlrDataControlOfferV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrDataControlOfferV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
                Self::ZwlrDataControlSourceV1 => {
                    if version > ZwlrDataControlSourceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrDataControlSourceV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_export_dmabuf_unstable_v1")]
                Self::ZwlrExportDmabufFrameV1 => {
                    if version > ZwlrExportDmabufFrameV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrExportDmabufFrameV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_export_dmabuf_unstable_v1")]
                Self::ZwlrExportDmabufManagerV1 => {
                    if version > ZwlrExportDmabufManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrExportDmabufManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1")]
                Self::ZwlrForeignToplevelHandleV1 => {
                    if version > ZwlrForeignToplevelHandleV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrForeignToplevelHandleV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1")]
                Self::ZwlrForeignToplevelManagerV1 => {
                    if version > ZwlrForeignToplevelManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrForeignToplevelManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_gamma_control_unstable_v1")]
                Self::ZwlrGammaControlManagerV1 => {
                    if version > ZwlrGammaControlManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrGammaControlManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_gamma_control_unstable_v1")]
                Self::ZwlrGammaControlV1 => {
                    if version > ZwlrGammaControlV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrGammaControlV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_input_inhibit_unstable_v1")]
                Self::ZwlrInputInhibitManagerV1 => {
                    if version > ZwlrInputInhibitManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrInputInhibitManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_input_inhibit_unstable_v1")]
                Self::ZwlrInputInhibitorV1 => {
                    if version > ZwlrInputInhibitorV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrInputInhibitorV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")]
                Self::ZwlrLayerShellV1 => {
                    if version > ZwlrLayerShellV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrLayerShellV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")]
                Self::ZwlrLayerSurfaceV1 => {
                    if version > ZwlrLayerSurfaceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrLayerSurfaceV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
                Self::ZwlrOutputConfigurationHeadV1 => {
                    if version > ZwlrOutputConfigurationHeadV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrOutputConfigurationHeadV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
                Self::ZwlrOutputConfigurationV1 => {
                    if version > ZwlrOutputConfigurationV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrOutputConfigurationV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
                Self::ZwlrOutputHeadV1 => {
                    if version > ZwlrOutputHeadV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrOutputHeadV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
                Self::ZwlrOutputManagerV1 => {
                    if version > ZwlrOutputManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrOutputManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
                Self::ZwlrOutputModeV1 => {
                    if version > ZwlrOutputModeV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrOutputModeV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_output_power_management_unstable_v1")]
                Self::ZwlrOutputPowerManagerV1 => {
                    if version > ZwlrOutputPowerManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrOutputPowerManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_output_power_management_unstable_v1")]
                Self::ZwlrOutputPowerV1 => {
                    if version > ZwlrOutputPowerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrOutputPowerV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_screencopy_unstable_v1")]
                Self::ZwlrScreencopyFrameV1 => {
                    if version > ZwlrScreencopyFrameV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrScreencopyFrameV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_screencopy_unstable_v1")]
                Self::ZwlrScreencopyManagerV1 => {
                    if version > ZwlrScreencopyManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrScreencopyManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_virtual_pointer_unstable_v1")]
                Self::ZwlrVirtualPointerManagerV1 => {
                    if version > ZwlrVirtualPointerManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrVirtualPointerManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_virtual_pointer_unstable_v1")]
                Self::ZwlrVirtualPointerV1 => {
                    if version > ZwlrVirtualPointerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrVirtualPointerV1::new(state, version))
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, linearize::Linearize)]
#[linearize(const)]
pub enum ProxyInterface {
    #[cfg(feature = "protocol-hyprland_ctm_control_v1")]
    HyprlandCtmControlManagerV1,
    #[cfg(feature = "protocol-hyprland_focus_grab_v1")]
    HyprlandFocusGrabManagerV1,
    #[cfg(feature = "protocol-hyprland_focus_grab_v1")]
    HyprlandFocusGrabV1,
    #[cfg(feature = "protocol-hyprland_global_shortcuts_v1")]
    HyprlandGlobalShortcutV1,
    #[cfg(feature = "protocol-hyprland_global_shortcuts_v1")]
    HyprlandGlobalShortcutsManagerV1,
    #[cfg(feature = "protocol-hyprland_input_capture_v1")]
    HyprlandInputCaptureManagerV1,
    #[cfg(feature = "protocol-hyprland_input_capture_v1")]
    HyprlandInputCaptureV1,
    #[cfg(feature = "protocol-hyprland_lock_notify_v1")]
    HyprlandLockNotificationV1,
    #[cfg(feature = "protocol-hyprland_lock_notify_v1")]
    HyprlandLockNotifierV1,
    #[cfg(feature = "protocol-hyprland_surface_v1")]
    HyprlandSurfaceManagerV1,
    #[cfg(feature = "protocol-hyprland_surface_v1")]
    HyprlandSurfaceV1,
    #[cfg(feature = "protocol-hyprland_toplevel_export_v1")]
    HyprlandToplevelExportFrameV1,
    #[cfg(feature = "protocol-hyprland_toplevel_export_v1")]
    HyprlandToplevelExportManagerV1,
    #[cfg(feature = "protocol-hyprland_toplevel_mapping_v1")]
    HyprlandToplevelMappingManagerV1,
    #[cfg(feature = "protocol-hyprland_toplevel_mapping_v1")]
    HyprlandToplevelWindowMappingHandleV1,
    #[cfg(feature = "protocol-jay_tray_v1")]
    JayTrayItemV1,
    #[cfg(feature = "protocol-jay_tray_v1")]
    JayTrayV1,
    #[cfg(feature = "protocol-drm")]
    WlDrm,
    #[cfg(feature = "protocol-input_method_unstable_v2")]
    ZwpInputMethodKeyboardGrabV2,
    #[cfg(feature = "protocol-input_method_unstable_v2")]
    ZwpInputMethodManagerV2,
    #[cfg(feature = "protocol-input_method_unstable_v2")]
    ZwpInputMethodV2,
    #[cfg(feature = "protocol-input_method_unstable_v2")]
    ZwpInputPopupSurfaceV2,
    #[cfg(feature = "protocol-org_kde_kwin_server_decoration_v1")]
    OrgKdeKwinServerDecoration,
    #[cfg(feature = "protocol-org_kde_kwin_server_decoration_v1")]
    OrgKdeKwinServerDecorationManager,
    #[cfg(feature = "protocol-virtual_keyboard_unstable_v1")]
    ZwpVirtualKeyboardManagerV1,
    #[cfg(feature = "protocol-virtual_keyboard_unstable_v1")]
    ZwpVirtualKeyboardV1,
    #[cfg(feature = "protocol-treeland_app_id_resolver_v1")]
    TreelandAppIdResolverManagerV1,
    #[cfg(feature = "protocol-treeland_app_id_resolver_v1")]
    TreelandAppIdResolverV1,
    #[cfg(feature = "protocol-treeland_dde_shell_v1")]
    TreelandDdeActiveV1,
    #[cfg(feature = "protocol-treeland_dde_shell_v1")]
    TreelandDdeShellManagerV1,
    #[cfg(feature = "protocol-treeland_dde_shell_v1")]
    TreelandDdeShellSurfaceV1,
    #[cfg(feature = "protocol-treeland_dde_shell_v1")]
    TreelandLockscreenV1,
    #[cfg(feature = "protocol-treeland_dde_shell_v1")]
    TreelandMultitaskviewV1,
    #[cfg(feature = "protocol-treeland_dde_shell_v1")]
    TreelandWindowOverlapChecker,
    #[cfg(feature = "protocol-treeland_dde_shell_v1")]
    TreelandWindowPickerV1,
    #[cfg(feature = "protocol-treeland_ddm")]
    TreelandDdm,
    #[cfg(feature = "protocol-treeland_foreign_toplevel_manager_v1")]
    TreelandDockPreviewContextV1,
    #[cfg(feature = "protocol-treeland_foreign_toplevel_manager_v1")]
    TreelandForeignToplevelHandleV1,
    #[cfg(feature = "protocol-treeland_foreign_toplevel_manager_v1")]
    TreelandForeignToplevelManagerV1,
    #[cfg(feature = "protocol-treeland_output_manager_v1")]
    TreelandOutputColorControlV1,
    #[cfg(feature = "protocol-treeland_output_manager_v1")]
    TreelandOutputManagerV1,
    #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
    TreelandPersonalizationAppearanceContextV1,
    #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
    TreelandPersonalizationCursorContextV1,
    #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
    TreelandPersonalizationFontContextV1,
    #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
    TreelandPersonalizationManagerV1,
    #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
    TreelandPersonalizationWallpaperContextV1,
    #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
    TreelandPersonalizationWindowContextV1,
    #[cfg(feature = "protocol-treeland_prelaunch_splash_v1")]
    TreelandPrelaunchSplashManagerV1,
    #[cfg(feature = "protocol-treeland_screensaver")]
    TreelandScreensaver,
    #[cfg(feature = "protocol-treeland_shortcut_manager_v1")]
    TreelandShortcutContextV1,
    #[cfg(feature = "protocol-treeland_shortcut_manager_v1")]
    TreelandShortcutManagerV1,
    #[cfg(feature = "protocol-treeland_shortcut_manager_v2")]
    TreelandShortcutManagerV2,
    #[cfg(feature = "protocol-treeland_virtual_output_manager_v1")]
    TreelandVirtualOutputManagerV1,
    #[cfg(feature = "protocol-treeland_virtual_output_manager_v1")]
    TreelandVirtualOutputV1,
    #[cfg(feature = "protocol-treeland_wallpaper_color_v1")]
    TreelandWallpaperColorManagerV1,
    #[cfg(feature = "protocol-treeland_window_management_v1")]
    TreelandWindowManagementV1,
    WlBuffer,
    WlCallback,
    WlCompositor,
    WlDataDevice,
    WlDataDeviceManager,
    WlDataOffer,
    WlDataSource,
    WlDisplay,
    WlFixes,
    WlKeyboard,
    WlOutput,
    WlPointer,
    WlRegion,
    WlRegistry,
    WlSeat,
    WlShell,
    WlShellSurface,
    WlShm,
    WlShmPool,
    WlSubcompositor,
    WlSubsurface,
    WlSurface,
    WlTouch,
    #[cfg(feature = "protocol-alpha_modifier_v1")]
    WpAlphaModifierSurfaceV1,
    #[cfg(feature = "protocol-alpha_modifier_v1")]
    WpAlphaModifierV1,
    #[cfg(feature = "protocol-color_management_v1")]
    WpColorManagementOutputV1,
    #[cfg(feature = "protocol-color_management_v1")]
    WpColorManagementSurfaceFeedbackV1,
    #[cfg(feature = "protocol-color_management_v1")]
    WpColorManagementSurfaceV1,
    #[cfg(feature = "protocol-color_management_v1")]
    WpColorManagerV1,
    #[cfg(feature = "protocol-color_management_v1")]
    WpImageDescriptionCreatorIccV1,
    #[cfg(feature = "protocol-color_management_v1")]
    WpImageDescriptionCreatorParamsV1,
    #[cfg(feature = "protocol-color_management_v1")]
    WpImageDescriptionInfoV1,
    #[cfg(feature = "protocol-color_management_v1")]
    WpImageDescriptionV1,
    #[cfg(feature = "protocol-color_representation_v1")]
    WpColorRepresentationManagerV1,
    #[cfg(feature = "protocol-color_representation_v1")]
    WpColorRepresentationSurfaceV1,
    #[cfg(feature = "protocol-commit_timing_v1")]
    WpCommitTimerV1,
    #[cfg(feature = "protocol-commit_timing_v1")]
    WpCommitTimingManagerV1,
    #[cfg(feature = "protocol-content_type_v1")]
    WpContentTypeManagerV1,
    #[cfg(feature = "protocol-content_type_v1")]
    WpContentTypeV1,
    #[cfg(feature = "protocol-cursor_shape_v1")]
    WpCursorShapeDeviceV1,
    #[cfg(feature = "protocol-cursor_shape_v1")]
    WpCursorShapeManagerV1,
    #[cfg(feature = "protocol-drm_lease_v1")]
    WpDrmLeaseConnectorV1,
    #[cfg(feature = "protocol-drm_lease_v1")]
    WpDrmLeaseDeviceV1,
    #[cfg(feature = "protocol-drm_lease_v1")]
    WpDrmLeaseRequestV1,
    #[cfg(feature = "protocol-drm_lease_v1")]
    WpDrmLeaseV1,
    #[cfg(feature = "protocol-ext_background_effect_v1")]
    ExtBackgroundEffectManagerV1,
    #[cfg(feature = "protocol-ext_background_effect_v1")]
    ExtBackgroundEffectSurfaceV1,
    #[cfg(feature = "protocol-ext_data_control_v1")]
    ExtDataControlDeviceV1,
    #[cfg(feature = "protocol-ext_data_control_v1")]
    ExtDataControlManagerV1,
    #[cfg(feature = "protocol-ext_data_control_v1")]
    ExtDataControlOfferV1,
    #[cfg(feature = "protocol-ext_data_control_v1")]
    ExtDataControlSourceV1,
    #[cfg(feature = "protocol-ext_foreign_toplevel_list_v1")]
    ExtForeignToplevelHandleV1,
    #[cfg(feature = "protocol-ext_foreign_toplevel_list_v1")]
    ExtForeignToplevelListV1,
    #[cfg(feature = "protocol-ext_idle_notify_v1")]
    ExtIdleNotificationV1,
    #[cfg(feature = "protocol-ext_idle_notify_v1")]
    ExtIdleNotifierV1,
    #[cfg(feature = "protocol-ext_image_capture_source_v1")]
    ExtForeignToplevelImageCaptureSourceManagerV1,
    #[cfg(feature = "protocol-ext_image_capture_source_v1")]
    ExtImageCaptureSourceV1,
    #[cfg(feature = "protocol-ext_image_capture_source_v1")]
    ExtOutputImageCaptureSourceManagerV1,
    #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
    ExtImageCopyCaptureCursorSessionV1,
    #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
    ExtImageCopyCaptureFrameV1,
    #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
    ExtImageCopyCaptureManagerV1,
    #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
    ExtImageCopyCaptureSessionV1,
    #[cfg(feature = "protocol-ext_session_lock_v1")]
    ExtSessionLockManagerV1,
    #[cfg(feature = "protocol-ext_session_lock_v1")]
    ExtSessionLockSurfaceV1,
    #[cfg(feature = "protocol-ext_session_lock_v1")]
    ExtSessionLockV1,
    #[cfg(feature = "protocol-ext_transient_seat_v1")]
    ExtTransientSeatManagerV1,
    #[cfg(feature = "protocol-ext_transient_seat_v1")]
    ExtTransientSeatV1,
    #[cfg(feature = "protocol-ext_workspace_v1")]
    ExtWorkspaceGroupHandleV1,
    #[cfg(feature = "protocol-ext_workspace_v1")]
    ExtWorkspaceHandleV1,
    #[cfg(feature = "protocol-ext_workspace_v1")]
    ExtWorkspaceManagerV1,
    #[cfg(feature = "protocol-fifo_v1")]
    WpFifoManagerV1,
    #[cfg(feature = "protocol-fifo_v1")]
    WpFifoV1,
    #[cfg(feature = "protocol-fractional_scale_v1")]
    WpFractionalScaleManagerV1,
    #[cfg(feature = "protocol-fractional_scale_v1")]
    WpFractionalScaleV1,
    #[cfg(feature = "protocol-fullscreen_shell_unstable_v1")]
    ZwpFullscreenShellModeFeedbackV1,
    #[cfg(feature = "protocol-fullscreen_shell_unstable_v1")]
    ZwpFullscreenShellV1,
    #[cfg(feature = "protocol-idle_inhibit_unstable_v1")]
    ZwpIdleInhibitManagerV1,
    #[cfg(feature = "protocol-idle_inhibit_unstable_v1")]
    ZwpIdleInhibitorV1,
    #[cfg(feature = "protocol-input_method_unstable_v1")]
    ZwpInputMethodContextV1,
    #[cfg(feature = "protocol-input_method_unstable_v1")]
    ZwpInputMethodV1,
    #[cfg(feature = "protocol-input_method_unstable_v1")]
    ZwpInputPanelSurfaceV1,
    #[cfg(feature = "protocol-input_method_unstable_v1")]
    ZwpInputPanelV1,
    #[cfg(feature = "protocol-input_timestamps_unstable_v1")]
    ZwpInputTimestampsManagerV1,
    #[cfg(feature = "protocol-input_timestamps_unstable_v1")]
    ZwpInputTimestampsV1,
    #[cfg(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1")]
    ZwpKeyboardShortcutsInhibitManagerV1,
    #[cfg(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1")]
    ZwpKeyboardShortcutsInhibitorV1,
    #[cfg(feature = "protocol-linux_dmabuf_v1")]
    ZwpLinuxBufferParamsV1,
    #[cfg(feature = "protocol-linux_dmabuf_v1")]
    ZwpLinuxDmabufFeedbackV1,
    #[cfg(feature = "protocol-linux_dmabuf_v1")]
    ZwpLinuxDmabufV1,
    #[cfg(feature = "protocol-linux_drm_syncobj_v1")]
    WpLinuxDrmSyncobjManagerV1,
    #[cfg(feature = "protocol-linux_drm_syncobj_v1")]
    WpLinuxDrmSyncobjSurfaceV1,
    #[cfg(feature = "protocol-linux_drm_syncobj_v1")]
    WpLinuxDrmSyncobjTimelineV1,
    #[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
    ZwpConfinedPointerV1,
    #[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
    ZwpLockedPointerV1,
    #[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
    ZwpPointerConstraintsV1,
    #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
    ZwpPointerGestureHoldV1,
    #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
    ZwpPointerGesturePinchV1,
    #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
    ZwpPointerGestureSwipeV1,
    #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
    ZwpPointerGesturesV1,
    #[cfg(feature = "protocol-pointer_warp_v1")]
    WpPointerWarpV1,
    #[cfg(feature = "protocol-presentation_time")]
    WpPresentation,
    #[cfg(feature = "protocol-presentation_time")]
    WpPresentationFeedback,
    #[cfg(feature = "protocol-relative_pointer_unstable_v1")]
    ZwpRelativePointerManagerV1,
    #[cfg(feature = "protocol-relative_pointer_unstable_v1")]
    ZwpRelativePointerV1,
    #[cfg(feature = "protocol-security_context_v1")]
    WpSecurityContextManagerV1,
    #[cfg(feature = "protocol-security_context_v1")]
    WpSecurityContextV1,
    #[cfg(feature = "protocol-single_pixel_buffer_v1")]
    WpSinglePixelBufferManagerV1,
    #[cfg(feature = "protocol-tablet_v2")]
    ZwpTabletManagerV2,
    #[cfg(feature = "protocol-tablet_v2")]
    ZwpTabletPadDialV2,
    #[cfg(feature = "protocol-tablet_v2")]
    ZwpTabletPadGroupV2,
    #[cfg(feature = "protocol-tablet_v2")]
    ZwpTabletPadRingV2,
    #[cfg(feature = "protocol-tablet_v2")]
    ZwpTabletPadStripV2,
    #[cfg(feature = "protocol-tablet_v2")]
    ZwpTabletPadV2,
    #[cfg(feature = "protocol-tablet_v2")]
    ZwpTabletSeatV2,
    #[cfg(feature = "protocol-tablet_v2")]
    ZwpTabletToolV2,
    #[cfg(feature = "protocol-tablet_v2")]
    ZwpTabletV2,
    #[cfg(feature = "protocol-tearing_control_v1")]
    WpTearingControlManagerV1,
    #[cfg(feature = "protocol-tearing_control_v1")]
    WpTearingControlV1,
    #[cfg(feature = "protocol-text_input_unstable_v1")]
    ZwpTextInputManagerV1,
    #[cfg(feature = "protocol-text_input_unstable_v1")]
    ZwpTextInputV1,
    #[cfg(feature = "protocol-text_input_unstable_v3")]
    ZwpTextInputManagerV3,
    #[cfg(feature = "protocol-text_input_unstable_v3")]
    ZwpTextInputV3,
    #[cfg(feature = "protocol-viewporter")]
    WpViewport,
    #[cfg(feature = "protocol-viewporter")]
    WpViewporter,
    #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
    ZwpPrimarySelectionDeviceManagerV1,
    #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
    ZwpPrimarySelectionDeviceV1,
    #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
    ZwpPrimarySelectionOfferV1,
    #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
    ZwpPrimarySelectionSourceV1,
    #[cfg(feature = "protocol-xdg_activation_v1")]
    XdgActivationTokenV1,
    #[cfg(feature = "protocol-xdg_activation_v1")]
    XdgActivationV1,
    #[cfg(feature = "protocol-xdg_decoration_unstable_v1")]
    ZxdgDecorationManagerV1,
    #[cfg(feature = "protocol-xdg_decoration_unstable_v1")]
    ZxdgToplevelDecorationV1,
    #[cfg(feature = "protocol-xdg_dialog_v1")]
    XdgDialogV1,
    #[cfg(feature = "protocol-xdg_dialog_v1")]
    XdgWmDialogV1,
    #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
    ZxdgExportedV2,
    #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
    ZxdgExporterV2,
    #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
    ZxdgImportedV2,
    #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
    ZxdgImporterV2,
    #[cfg(feature = "protocol-xdg_output_unstable_v1")]
    ZxdgOutputManagerV1,
    #[cfg(feature = "protocol-xdg_output_unstable_v1")]
    ZxdgOutputV1,
    #[cfg(feature = "protocol-xdg_shell")]
    XdgPopup,
    #[cfg(feature = "protocol-xdg_shell")]
    XdgPositioner,
    #[cfg(feature = "protocol-xdg_shell")]
    XdgSurface,
    #[cfg(feature = "protocol-xdg_shell")]
    XdgToplevel,
    #[cfg(feature = "protocol-xdg_shell")]
    XdgWmBase,
    #[cfg(feature = "protocol-xdg_system_bell_v1")]
    XdgSystemBellV1,
    #[cfg(feature = "protocol-xdg_toplevel_drag_v1")]
    XdgToplevelDragManagerV1,
    #[cfg(feature = "protocol-xdg_toplevel_drag_v1")]
    XdgToplevelDragV1,
    #[cfg(feature = "protocol-xdg_toplevel_icon_v1")]
    XdgToplevelIconManagerV1,
    #[cfg(feature = "protocol-xdg_toplevel_icon_v1")]
    XdgToplevelIconV1,
    #[cfg(feature = "protocol-xdg_toplevel_tag_v1")]
    XdgToplevelTagManagerV1,
    #[cfg(feature = "protocol-xwayland_keyboard_grab_unstable_v1")]
    ZwpXwaylandKeyboardGrabManagerV1,
    #[cfg(feature = "protocol-xwayland_keyboard_grab_unstable_v1")]
    ZwpXwaylandKeyboardGrabV1,
    #[cfg(feature = "protocol-xwayland_shell_v1")]
    XwaylandShellV1,
    #[cfg(feature = "protocol-xwayland_shell_v1")]
    XwaylandSurfaceV1,
    #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
    ZwpLinuxBufferReleaseV1,
    #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
    ZwpLinuxExplicitSynchronizationV1,
    #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
    ZwpLinuxSurfaceSynchronizationV1,
    #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
    ZwlrDataControlDeviceV1,
    #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
    ZwlrDataControlManagerV1,
    #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
    ZwlrDataControlOfferV1,
    #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
    ZwlrDataControlSourceV1,
    #[cfg(feature = "protocol-wlr_export_dmabuf_unstable_v1")]
    ZwlrExportDmabufFrameV1,
    #[cfg(feature = "protocol-wlr_export_dmabuf_unstable_v1")]
    ZwlrExportDmabufManagerV1,
    #[cfg(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1")]
    ZwlrForeignToplevelHandleV1,
    #[cfg(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1")]
    ZwlrForeignToplevelManagerV1,
    #[cfg(feature = "protocol-wlr_gamma_control_unstable_v1")]
    ZwlrGammaControlManagerV1,
    #[cfg(feature = "protocol-wlr_gamma_control_unstable_v1")]
    ZwlrGammaControlV1,
    #[cfg(feature = "protocol-wlr_input_inhibit_unstable_v1")]
    ZwlrInputInhibitManagerV1,
    #[cfg(feature = "protocol-wlr_input_inhibit_unstable_v1")]
    ZwlrInputInhibitorV1,
    #[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")]
    ZwlrLayerShellV1,
    #[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")]
    ZwlrLayerSurfaceV1,
    #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
    ZwlrOutputConfigurationHeadV1,
    #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
    ZwlrOutputConfigurationV1,
    #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
    ZwlrOutputHeadV1,
    #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
    ZwlrOutputManagerV1,
    #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
    ZwlrOutputModeV1,
    #[cfg(feature = "protocol-wlr_output_power_management_unstable_v1")]
    ZwlrOutputPowerManagerV1,
    #[cfg(feature = "protocol-wlr_output_power_management_unstable_v1")]
    ZwlrOutputPowerV1,
    #[cfg(feature = "protocol-wlr_screencopy_unstable_v1")]
    ZwlrScreencopyFrameV1,
    #[cfg(feature = "protocol-wlr_screencopy_unstable_v1")]
    ZwlrScreencopyManagerV1,
    #[cfg(feature = "protocol-wlr_virtual_pointer_unstable_v1")]
    ZwlrVirtualPointerManagerV1,
    #[cfg(feature = "protocol-wlr_virtual_pointer_unstable_v1")]
    ZwlrVirtualPointerV1,
}

impl ProxyInterface {
    pub fn name(self) -> &'static str {
        match self {
            #[cfg(feature = "protocol-hyprland_ctm_control_v1")]
            Self::HyprlandCtmControlManagerV1 => "hyprland_ctm_control_manager_v1",
            #[cfg(feature = "protocol-hyprland_focus_grab_v1")]
            Self::HyprlandFocusGrabManagerV1 => "hyprland_focus_grab_manager_v1",
            #[cfg(feature = "protocol-hyprland_focus_grab_v1")]
            Self::HyprlandFocusGrabV1 => "hyprland_focus_grab_v1",
            #[cfg(feature = "protocol-hyprland_global_shortcuts_v1")]
            Self::HyprlandGlobalShortcutV1 => "hyprland_global_shortcut_v1",
            #[cfg(feature = "protocol-hyprland_global_shortcuts_v1")]
            Self::HyprlandGlobalShortcutsManagerV1 => "hyprland_global_shortcuts_manager_v1",
            #[cfg(feature = "protocol-hyprland_input_capture_v1")]
            Self::HyprlandInputCaptureManagerV1 => "hyprland_input_capture_manager_v1",
            #[cfg(feature = "protocol-hyprland_input_capture_v1")]
            Self::HyprlandInputCaptureV1 => "hyprland_input_capture_v1",
            #[cfg(feature = "protocol-hyprland_lock_notify_v1")]
            Self::HyprlandLockNotificationV1 => "hyprland_lock_notification_v1",
            #[cfg(feature = "protocol-hyprland_lock_notify_v1")]
            Self::HyprlandLockNotifierV1 => "hyprland_lock_notifier_v1",
            #[cfg(feature = "protocol-hyprland_surface_v1")]
            Self::HyprlandSurfaceManagerV1 => "hyprland_surface_manager_v1",
            #[cfg(feature = "protocol-hyprland_surface_v1")]
            Self::HyprlandSurfaceV1 => "hyprland_surface_v1",
            #[cfg(feature = "protocol-hyprland_toplevel_export_v1")]
            Self::HyprlandToplevelExportFrameV1 => "hyprland_toplevel_export_frame_v1",
            #[cfg(feature = "protocol-hyprland_toplevel_export_v1")]
            Self::HyprlandToplevelExportManagerV1 => "hyprland_toplevel_export_manager_v1",
            #[cfg(feature = "protocol-hyprland_toplevel_mapping_v1")]
            Self::HyprlandToplevelMappingManagerV1 => "hyprland_toplevel_mapping_manager_v1",
            #[cfg(feature = "protocol-hyprland_toplevel_mapping_v1")]
            Self::HyprlandToplevelWindowMappingHandleV1 => "hyprland_toplevel_window_mapping_handle_v1",
            #[cfg(feature = "protocol-jay_tray_v1")]
            Self::JayTrayItemV1 => "jay_tray_item_v1",
            #[cfg(feature = "protocol-jay_tray_v1")]
            Self::JayTrayV1 => "jay_tray_v1",
            #[cfg(feature = "protocol-drm")]
            Self::WlDrm => "wl_drm",
            #[cfg(feature = "protocol-input_method_unstable_v2")]
            Self::ZwpInputMethodKeyboardGrabV2 => "zwp_input_method_keyboard_grab_v2",
            #[cfg(feature = "protocol-input_method_unstable_v2")]
            Self::ZwpInputMethodManagerV2 => "zwp_input_method_manager_v2",
            #[cfg(feature = "protocol-input_method_unstable_v2")]
            Self::ZwpInputMethodV2 => "zwp_input_method_v2",
            #[cfg(feature = "protocol-input_method_unstable_v2")]
            Self::ZwpInputPopupSurfaceV2 => "zwp_input_popup_surface_v2",
            #[cfg(feature = "protocol-org_kde_kwin_server_decoration_v1")]
            Self::OrgKdeKwinServerDecoration => "org_kde_kwin_server_decoration",
            #[cfg(feature = "protocol-org_kde_kwin_server_decoration_v1")]
            Self::OrgKdeKwinServerDecorationManager => "org_kde_kwin_server_decoration_manager",
            #[cfg(feature = "protocol-virtual_keyboard_unstable_v1")]
            Self::ZwpVirtualKeyboardManagerV1 => "zwp_virtual_keyboard_manager_v1",
            #[cfg(feature = "protocol-virtual_keyboard_unstable_v1")]
            Self::ZwpVirtualKeyboardV1 => "zwp_virtual_keyboard_v1",
            #[cfg(feature = "protocol-treeland_app_id_resolver_v1")]
            Self::TreelandAppIdResolverManagerV1 => "treeland_app_id_resolver_manager_v1",
            #[cfg(feature = "protocol-treeland_app_id_resolver_v1")]
            Self::TreelandAppIdResolverV1 => "treeland_app_id_resolver_v1",
            #[cfg(feature = "protocol-treeland_dde_shell_v1")]
            Self::TreelandDdeActiveV1 => "treeland_dde_active_v1",
            #[cfg(feature = "protocol-treeland_dde_shell_v1")]
            Self::TreelandDdeShellManagerV1 => "treeland_dde_shell_manager_v1",
            #[cfg(feature = "protocol-treeland_dde_shell_v1")]
            Self::TreelandDdeShellSurfaceV1 => "treeland_dde_shell_surface_v1",
            #[cfg(feature = "protocol-treeland_dde_shell_v1")]
            Self::TreelandLockscreenV1 => "treeland_lockscreen_v1",
            #[cfg(feature = "protocol-treeland_dde_shell_v1")]
            Self::TreelandMultitaskviewV1 => "treeland_multitaskview_v1",
            #[cfg(feature = "protocol-treeland_dde_shell_v1")]
            Self::TreelandWindowOverlapChecker => "treeland_window_overlap_checker",
            #[cfg(feature = "protocol-treeland_dde_shell_v1")]
            Self::TreelandWindowPickerV1 => "treeland_window_picker_v1",
            #[cfg(feature = "protocol-treeland_ddm")]
            Self::TreelandDdm => "treeland_ddm",
            #[cfg(feature = "protocol-treeland_foreign_toplevel_manager_v1")]
            Self::TreelandDockPreviewContextV1 => "treeland_dock_preview_context_v1",
            #[cfg(feature = "protocol-treeland_foreign_toplevel_manager_v1")]
            Self::TreelandForeignToplevelHandleV1 => "treeland_foreign_toplevel_handle_v1",
            #[cfg(feature = "protocol-treeland_foreign_toplevel_manager_v1")]
            Self::TreelandForeignToplevelManagerV1 => "treeland_foreign_toplevel_manager_v1",
            #[cfg(feature = "protocol-treeland_output_manager_v1")]
            Self::TreelandOutputColorControlV1 => "treeland_output_color_control_v1",
            #[cfg(feature = "protocol-treeland_output_manager_v1")]
            Self::TreelandOutputManagerV1 => "treeland_output_manager_v1",
            #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
            Self::TreelandPersonalizationAppearanceContextV1 => "treeland_personalization_appearance_context_v1",
            #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
            Self::TreelandPersonalizationCursorContextV1 => "treeland_personalization_cursor_context_v1",
            #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
            Self::TreelandPersonalizationFontContextV1 => "treeland_personalization_font_context_v1",
            #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
            Self::TreelandPersonalizationManagerV1 => "treeland_personalization_manager_v1",
            #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
            Self::TreelandPersonalizationWallpaperContextV1 => "treeland_personalization_wallpaper_context_v1",
            #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
            Self::TreelandPersonalizationWindowContextV1 => "treeland_personalization_window_context_v1",
            #[cfg(feature = "protocol-treeland_prelaunch_splash_v1")]
            Self::TreelandPrelaunchSplashManagerV1 => "treeland_prelaunch_splash_manager_v1",
            #[cfg(feature = "protocol-treeland_screensaver")]
            Self::TreelandScreensaver => "treeland_screensaver",
            #[cfg(feature = "protocol-treeland_shortcut_manager_v1")]
            Self::TreelandShortcutContextV1 => "treeland_shortcut_context_v1",
            #[cfg(feature = "protocol-treeland_shortcut_manager_v1")]
            Self::TreelandShortcutManagerV1 => "treeland_shortcut_manager_v1",
            #[cfg(feature = "protocol-treeland_shortcut_manager_v2")]
            Self::TreelandShortcutManagerV2 => "treeland_shortcut_manager_v2",
            #[cfg(feature = "protocol-treeland_virtual_output_manager_v1")]
            Self::TreelandVirtualOutputManagerV1 => "treeland_virtual_output_manager_v1",
            #[cfg(feature = "protocol-treeland_virtual_output_manager_v1")]
            Self::TreelandVirtualOutputV1 => "treeland_virtual_output_v1",
            #[cfg(feature = "protocol-treeland_wallpaper_color_v1")]
            Self::TreelandWallpaperColorManagerV1 => "treeland_wallpaper_color_manager_v1",
            #[cfg(feature = "protocol-treeland_window_management_v1")]
            Self::TreelandWindowManagementV1 => "treeland_window_management_v1",
            Self::WlBuffer => "wl_buffer",
            Self::WlCallback => "wl_callback",
            Self::WlCompositor => "wl_compositor",
            Self::WlDataDevice => "wl_data_device",
            Self::WlDataDeviceManager => "wl_data_device_manager",
            Self::WlDataOffer => "wl_data_offer",
            Self::WlDataSource => "wl_data_source",
            Self::WlDisplay => "wl_display",
            Self::WlFixes => "wl_fixes",
            Self::WlKeyboard => "wl_keyboard",
            Self::WlOutput => "wl_output",
            Self::WlPointer => "wl_pointer",
            Self::WlRegion => "wl_region",
            Self::WlRegistry => "wl_registry",
            Self::WlSeat => "wl_seat",
            Self::WlShell => "wl_shell",
            Self::WlShellSurface => "wl_shell_surface",
            Self::WlShm => "wl_shm",
            Self::WlShmPool => "wl_shm_pool",
            Self::WlSubcompositor => "wl_subcompositor",
            Self::WlSubsurface => "wl_subsurface",
            Self::WlSurface => "wl_surface",
            Self::WlTouch => "wl_touch",
            #[cfg(feature = "protocol-alpha_modifier_v1")]
            Self::WpAlphaModifierSurfaceV1 => "wp_alpha_modifier_surface_v1",
            #[cfg(feature = "protocol-alpha_modifier_v1")]
            Self::WpAlphaModifierV1 => "wp_alpha_modifier_v1",
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpColorManagementOutputV1 => "wp_color_management_output_v1",
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpColorManagementSurfaceFeedbackV1 => "wp_color_management_surface_feedback_v1",
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpColorManagementSurfaceV1 => "wp_color_management_surface_v1",
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpColorManagerV1 => "wp_color_manager_v1",
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpImageDescriptionCreatorIccV1 => "wp_image_description_creator_icc_v1",
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpImageDescriptionCreatorParamsV1 => "wp_image_description_creator_params_v1",
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpImageDescriptionInfoV1 => "wp_image_description_info_v1",
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpImageDescriptionV1 => "wp_image_description_v1",
            #[cfg(feature = "protocol-color_representation_v1")]
            Self::WpColorRepresentationManagerV1 => "wp_color_representation_manager_v1",
            #[cfg(feature = "protocol-color_representation_v1")]
            Self::WpColorRepresentationSurfaceV1 => "wp_color_representation_surface_v1",
            #[cfg(feature = "protocol-commit_timing_v1")]
            Self::WpCommitTimerV1 => "wp_commit_timer_v1",
            #[cfg(feature = "protocol-commit_timing_v1")]
            Self::WpCommitTimingManagerV1 => "wp_commit_timing_manager_v1",
            #[cfg(feature = "protocol-content_type_v1")]
            Self::WpContentTypeManagerV1 => "wp_content_type_manager_v1",
            #[cfg(feature = "protocol-content_type_v1")]
            Self::WpContentTypeV1 => "wp_content_type_v1",
            #[cfg(feature = "protocol-cursor_shape_v1")]
            Self::WpCursorShapeDeviceV1 => "wp_cursor_shape_device_v1",
            #[cfg(feature = "protocol-cursor_shape_v1")]
            Self::WpCursorShapeManagerV1 => "wp_cursor_shape_manager_v1",
            #[cfg(feature = "protocol-drm_lease_v1")]
            Self::WpDrmLeaseConnectorV1 => "wp_drm_lease_connector_v1",
            #[cfg(feature = "protocol-drm_lease_v1")]
            Self::WpDrmLeaseDeviceV1 => "wp_drm_lease_device_v1",
            #[cfg(feature = "protocol-drm_lease_v1")]
            Self::WpDrmLeaseRequestV1 => "wp_drm_lease_request_v1",
            #[cfg(feature = "protocol-drm_lease_v1")]
            Self::WpDrmLeaseV1 => "wp_drm_lease_v1",
            #[cfg(feature = "protocol-ext_background_effect_v1")]
            Self::ExtBackgroundEffectManagerV1 => "ext_background_effect_manager_v1",
            #[cfg(feature = "protocol-ext_background_effect_v1")]
            Self::ExtBackgroundEffectSurfaceV1 => "ext_background_effect_surface_v1",
            #[cfg(feature = "protocol-ext_data_control_v1")]
            Self::ExtDataControlDeviceV1 => "ext_data_control_device_v1",
            #[cfg(feature = "protocol-ext_data_control_v1")]
            Self::ExtDataControlManagerV1 => "ext_data_control_manager_v1",
            #[cfg(feature = "protocol-ext_data_control_v1")]
            Self::ExtDataControlOfferV1 => "ext_data_control_offer_v1",
            #[cfg(feature = "protocol-ext_data_control_v1")]
            Self::ExtDataControlSourceV1 => "ext_data_control_source_v1",
            #[cfg(feature = "protocol-ext_foreign_toplevel_list_v1")]
            Self::ExtForeignToplevelHandleV1 => "ext_foreign_toplevel_handle_v1",
            #[cfg(feature = "protocol-ext_foreign_toplevel_list_v1")]
            Self::ExtForeignToplevelListV1 => "ext_foreign_toplevel_list_v1",
            #[cfg(feature = "protocol-ext_idle_notify_v1")]
            Self::ExtIdleNotificationV1 => "ext_idle_notification_v1",
            #[cfg(feature = "protocol-ext_idle_notify_v1")]
            Self::ExtIdleNotifierV1 => "ext_idle_notifier_v1",
            #[cfg(feature = "protocol-ext_image_capture_source_v1")]
            Self::ExtForeignToplevelImageCaptureSourceManagerV1 => "ext_foreign_toplevel_image_capture_source_manager_v1",
            #[cfg(feature = "protocol-ext_image_capture_source_v1")]
            Self::ExtImageCaptureSourceV1 => "ext_image_capture_source_v1",
            #[cfg(feature = "protocol-ext_image_capture_source_v1")]
            Self::ExtOutputImageCaptureSourceManagerV1 => "ext_output_image_capture_source_manager_v1",
            #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
            Self::ExtImageCopyCaptureCursorSessionV1 => "ext_image_copy_capture_cursor_session_v1",
            #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
            Self::ExtImageCopyCaptureFrameV1 => "ext_image_copy_capture_frame_v1",
            #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
            Self::ExtImageCopyCaptureManagerV1 => "ext_image_copy_capture_manager_v1",
            #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
            Self::ExtImageCopyCaptureSessionV1 => "ext_image_copy_capture_session_v1",
            #[cfg(feature = "protocol-ext_session_lock_v1")]
            Self::ExtSessionLockManagerV1 => "ext_session_lock_manager_v1",
            #[cfg(feature = "protocol-ext_session_lock_v1")]
            Self::ExtSessionLockSurfaceV1 => "ext_session_lock_surface_v1",
            #[cfg(feature = "protocol-ext_session_lock_v1")]
            Self::ExtSessionLockV1 => "ext_session_lock_v1",
            #[cfg(feature = "protocol-ext_transient_seat_v1")]
            Self::ExtTransientSeatManagerV1 => "ext_transient_seat_manager_v1",
            #[cfg(feature = "protocol-ext_transient_seat_v1")]
            Self::ExtTransientSeatV1 => "ext_transient_seat_v1",
            #[cfg(feature = "protocol-ext_workspace_v1")]
            Self::ExtWorkspaceGroupHandleV1 => "ext_workspace_group_handle_v1",
            #[cfg(feature = "protocol-ext_workspace_v1")]
            Self::ExtWorkspaceHandleV1 => "ext_workspace_handle_v1",
            #[cfg(feature = "protocol-ext_workspace_v1")]
            Self::ExtWorkspaceManagerV1 => "ext_workspace_manager_v1",
            #[cfg(feature = "protocol-fifo_v1")]
            Self::WpFifoManagerV1 => "wp_fifo_manager_v1",
            #[cfg(feature = "protocol-fifo_v1")]
            Self::WpFifoV1 => "wp_fifo_v1",
            #[cfg(feature = "protocol-fractional_scale_v1")]
            Self::WpFractionalScaleManagerV1 => "wp_fractional_scale_manager_v1",
            #[cfg(feature = "protocol-fractional_scale_v1")]
            Self::WpFractionalScaleV1 => "wp_fractional_scale_v1",
            #[cfg(feature = "protocol-fullscreen_shell_unstable_v1")]
            Self::ZwpFullscreenShellModeFeedbackV1 => "zwp_fullscreen_shell_mode_feedback_v1",
            #[cfg(feature = "protocol-fullscreen_shell_unstable_v1")]
            Self::ZwpFullscreenShellV1 => "zwp_fullscreen_shell_v1",
            #[cfg(feature = "protocol-idle_inhibit_unstable_v1")]
            Self::ZwpIdleInhibitManagerV1 => "zwp_idle_inhibit_manager_v1",
            #[cfg(feature = "protocol-idle_inhibit_unstable_v1")]
            Self::ZwpIdleInhibitorV1 => "zwp_idle_inhibitor_v1",
            #[cfg(feature = "protocol-input_method_unstable_v1")]
            Self::ZwpInputMethodContextV1 => "zwp_input_method_context_v1",
            #[cfg(feature = "protocol-input_method_unstable_v1")]
            Self::ZwpInputMethodV1 => "zwp_input_method_v1",
            #[cfg(feature = "protocol-input_method_unstable_v1")]
            Self::ZwpInputPanelSurfaceV1 => "zwp_input_panel_surface_v1",
            #[cfg(feature = "protocol-input_method_unstable_v1")]
            Self::ZwpInputPanelV1 => "zwp_input_panel_v1",
            #[cfg(feature = "protocol-input_timestamps_unstable_v1")]
            Self::ZwpInputTimestampsManagerV1 => "zwp_input_timestamps_manager_v1",
            #[cfg(feature = "protocol-input_timestamps_unstable_v1")]
            Self::ZwpInputTimestampsV1 => "zwp_input_timestamps_v1",
            #[cfg(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1")]
            Self::ZwpKeyboardShortcutsInhibitManagerV1 => "zwp_keyboard_shortcuts_inhibit_manager_v1",
            #[cfg(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1")]
            Self::ZwpKeyboardShortcutsInhibitorV1 => "zwp_keyboard_shortcuts_inhibitor_v1",
            #[cfg(feature = "protocol-linux_dmabuf_v1")]
            Self::ZwpLinuxBufferParamsV1 => "zwp_linux_buffer_params_v1",
            #[cfg(feature = "protocol-linux_dmabuf_v1")]
            Self::ZwpLinuxDmabufFeedbackV1 => "zwp_linux_dmabuf_feedback_v1",
            #[cfg(feature = "protocol-linux_dmabuf_v1")]
            Self::ZwpLinuxDmabufV1 => "zwp_linux_dmabuf_v1",
            #[cfg(feature = "protocol-linux_drm_syncobj_v1")]
            Self::WpLinuxDrmSyncobjManagerV1 => "wp_linux_drm_syncobj_manager_v1",
            #[cfg(feature = "protocol-linux_drm_syncobj_v1")]
            Self::WpLinuxDrmSyncobjSurfaceV1 => "wp_linux_drm_syncobj_surface_v1",
            #[cfg(feature = "protocol-linux_drm_syncobj_v1")]
            Self::WpLinuxDrmSyncobjTimelineV1 => "wp_linux_drm_syncobj_timeline_v1",
            #[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
            Self::ZwpConfinedPointerV1 => "zwp_confined_pointer_v1",
            #[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
            Self::ZwpLockedPointerV1 => "zwp_locked_pointer_v1",
            #[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
            Self::ZwpPointerConstraintsV1 => "zwp_pointer_constraints_v1",
            #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
            Self::ZwpPointerGestureHoldV1 => "zwp_pointer_gesture_hold_v1",
            #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
            Self::ZwpPointerGesturePinchV1 => "zwp_pointer_gesture_pinch_v1",
            #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
            Self::ZwpPointerGestureSwipeV1 => "zwp_pointer_gesture_swipe_v1",
            #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
            Self::ZwpPointerGesturesV1 => "zwp_pointer_gestures_v1",
            #[cfg(feature = "protocol-pointer_warp_v1")]
            Self::WpPointerWarpV1 => "wp_pointer_warp_v1",
            #[cfg(feature = "protocol-presentation_time")]
            Self::WpPresentation => "wp_presentation",
            #[cfg(feature = "protocol-presentation_time")]
            Self::WpPresentationFeedback => "wp_presentation_feedback",
            #[cfg(feature = "protocol-relative_pointer_unstable_v1")]
            Self::ZwpRelativePointerManagerV1 => "zwp_relative_pointer_manager_v1",
            #[cfg(feature = "protocol-relative_pointer_unstable_v1")]
            Self::ZwpRelativePointerV1 => "zwp_relative_pointer_v1",
            #[cfg(feature = "protocol-security_context_v1")]
            Self::WpSecurityContextManagerV1 => "wp_security_context_manager_v1",
            #[cfg(feature = "protocol-security_context_v1")]
            Self::WpSecurityContextV1 => "wp_security_context_v1",
            #[cfg(feature = "protocol-single_pixel_buffer_v1")]
            Self::WpSinglePixelBufferManagerV1 => "wp_single_pixel_buffer_manager_v1",
            #[cfg(feature = "protocol-tablet_v2")]
            Self::ZwpTabletManagerV2 => "zwp_tablet_manager_v2",
            #[cfg(feature = "protocol-tablet_v2")]
            Self::ZwpTabletPadDialV2 => "zwp_tablet_pad_dial_v2",
            #[cfg(feature = "protocol-tablet_v2")]
            Self::ZwpTabletPadGroupV2 => "zwp_tablet_pad_group_v2",
            #[cfg(feature = "protocol-tablet_v2")]
            Self::ZwpTabletPadRingV2 => "zwp_tablet_pad_ring_v2",
            #[cfg(feature = "protocol-tablet_v2")]
            Self::ZwpTabletPadStripV2 => "zwp_tablet_pad_strip_v2",
            #[cfg(feature = "protocol-tablet_v2")]
            Self::ZwpTabletPadV2 => "zwp_tablet_pad_v2",
            #[cfg(feature = "protocol-tablet_v2")]
            Self::ZwpTabletSeatV2 => "zwp_tablet_seat_v2",
            #[cfg(feature = "protocol-tablet_v2")]
            Self::ZwpTabletToolV2 => "zwp_tablet_tool_v2",
            #[cfg(feature = "protocol-tablet_v2")]
            Self::ZwpTabletV2 => "zwp_tablet_v2",
            #[cfg(feature = "protocol-tearing_control_v1")]
            Self::WpTearingControlManagerV1 => "wp_tearing_control_manager_v1",
            #[cfg(feature = "protocol-tearing_control_v1")]
            Self::WpTearingControlV1 => "wp_tearing_control_v1",
            #[cfg(feature = "protocol-text_input_unstable_v1")]
            Self::ZwpTextInputManagerV1 => "zwp_text_input_manager_v1",
            #[cfg(feature = "protocol-text_input_unstable_v1")]
            Self::ZwpTextInputV1 => "zwp_text_input_v1",
            #[cfg(feature = "protocol-text_input_unstable_v3")]
            Self::ZwpTextInputManagerV3 => "zwp_text_input_manager_v3",
            #[cfg(feature = "protocol-text_input_unstable_v3")]
            Self::ZwpTextInputV3 => "zwp_text_input_v3",
            #[cfg(feature = "protocol-viewporter")]
            Self::WpViewport => "wp_viewport",
            #[cfg(feature = "protocol-viewporter")]
            Self::WpViewporter => "wp_viewporter",
            #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
            Self::ZwpPrimarySelectionDeviceManagerV1 => "zwp_primary_selection_device_manager_v1",
            #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
            Self::ZwpPrimarySelectionDeviceV1 => "zwp_primary_selection_device_v1",
            #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
            Self::ZwpPrimarySelectionOfferV1 => "zwp_primary_selection_offer_v1",
            #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
            Self::ZwpPrimarySelectionSourceV1 => "zwp_primary_selection_source_v1",
            #[cfg(feature = "protocol-xdg_activation_v1")]
            Self::XdgActivationTokenV1 => "xdg_activation_token_v1",
            #[cfg(feature = "protocol-xdg_activation_v1")]
            Self::XdgActivationV1 => "xdg_activation_v1",
            #[cfg(feature = "protocol-xdg_decoration_unstable_v1")]
            Self::ZxdgDecorationManagerV1 => "zxdg_decoration_manager_v1",
            #[cfg(feature = "protocol-xdg_decoration_unstable_v1")]
            Self::ZxdgToplevelDecorationV1 => "zxdg_toplevel_decoration_v1",
            #[cfg(feature = "protocol-xdg_dialog_v1")]
            Self::XdgDialogV1 => "xdg_dialog_v1",
            #[cfg(feature = "protocol-xdg_dialog_v1")]
            Self::XdgWmDialogV1 => "xdg_wm_dialog_v1",
            #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
            Self::ZxdgExportedV2 => "zxdg_exported_v2",
            #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
            Self::ZxdgExporterV2 => "zxdg_exporter_v2",
            #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
            Self::ZxdgImportedV2 => "zxdg_imported_v2",
            #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
            Self::ZxdgImporterV2 => "zxdg_importer_v2",
            #[cfg(feature = "protocol-xdg_output_unstable_v1")]
            Self::ZxdgOutputManagerV1 => "zxdg_output_manager_v1",
            #[cfg(feature = "protocol-xdg_output_unstable_v1")]
            Self::ZxdgOutputV1 => "zxdg_output_v1",
            #[cfg(feature = "protocol-xdg_shell")]
            Self::XdgPopup => "xdg_popup",
            #[cfg(feature = "protocol-xdg_shell")]
            Self::XdgPositioner => "xdg_positioner",
            #[cfg(feature = "protocol-xdg_shell")]
            Self::XdgSurface => "xdg_surface",
            #[cfg(feature = "protocol-xdg_shell")]
            Self::XdgToplevel => "xdg_toplevel",
            #[cfg(feature = "protocol-xdg_shell")]
            Self::XdgWmBase => "xdg_wm_base",
            #[cfg(feature = "protocol-xdg_system_bell_v1")]
            Self::XdgSystemBellV1 => "xdg_system_bell_v1",
            #[cfg(feature = "protocol-xdg_toplevel_drag_v1")]
            Self::XdgToplevelDragManagerV1 => "xdg_toplevel_drag_manager_v1",
            #[cfg(feature = "protocol-xdg_toplevel_drag_v1")]
            Self::XdgToplevelDragV1 => "xdg_toplevel_drag_v1",
            #[cfg(feature = "protocol-xdg_toplevel_icon_v1")]
            Self::XdgToplevelIconManagerV1 => "xdg_toplevel_icon_manager_v1",
            #[cfg(feature = "protocol-xdg_toplevel_icon_v1")]
            Self::XdgToplevelIconV1 => "xdg_toplevel_icon_v1",
            #[cfg(feature = "protocol-xdg_toplevel_tag_v1")]
            Self::XdgToplevelTagManagerV1 => "xdg_toplevel_tag_manager_v1",
            #[cfg(feature = "protocol-xwayland_keyboard_grab_unstable_v1")]
            Self::ZwpXwaylandKeyboardGrabManagerV1 => "zwp_xwayland_keyboard_grab_manager_v1",
            #[cfg(feature = "protocol-xwayland_keyboard_grab_unstable_v1")]
            Self::ZwpXwaylandKeyboardGrabV1 => "zwp_xwayland_keyboard_grab_v1",
            #[cfg(feature = "protocol-xwayland_shell_v1")]
            Self::XwaylandShellV1 => "xwayland_shell_v1",
            #[cfg(feature = "protocol-xwayland_shell_v1")]
            Self::XwaylandSurfaceV1 => "xwayland_surface_v1",
            #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
            Self::ZwpLinuxBufferReleaseV1 => "zwp_linux_buffer_release_v1",
            #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
            Self::ZwpLinuxExplicitSynchronizationV1 => "zwp_linux_explicit_synchronization_v1",
            #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
            Self::ZwpLinuxSurfaceSynchronizationV1 => "zwp_linux_surface_synchronization_v1",
            #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
            Self::ZwlrDataControlDeviceV1 => "zwlr_data_control_device_v1",
            #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
            Self::ZwlrDataControlManagerV1 => "zwlr_data_control_manager_v1",
            #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
            Self::ZwlrDataControlOfferV1 => "zwlr_data_control_offer_v1",
            #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
            Self::ZwlrDataControlSourceV1 => "zwlr_data_control_source_v1",
            #[cfg(feature = "protocol-wlr_export_dmabuf_unstable_v1")]
            Self::ZwlrExportDmabufFrameV1 => "zwlr_export_dmabuf_frame_v1",
            #[cfg(feature = "protocol-wlr_export_dmabuf_unstable_v1")]
            Self::ZwlrExportDmabufManagerV1 => "zwlr_export_dmabuf_manager_v1",
            #[cfg(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1")]
            Self::ZwlrForeignToplevelHandleV1 => "zwlr_foreign_toplevel_handle_v1",
            #[cfg(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1")]
            Self::ZwlrForeignToplevelManagerV1 => "zwlr_foreign_toplevel_manager_v1",
            #[cfg(feature = "protocol-wlr_gamma_control_unstable_v1")]
            Self::ZwlrGammaControlManagerV1 => "zwlr_gamma_control_manager_v1",
            #[cfg(feature = "protocol-wlr_gamma_control_unstable_v1")]
            Self::ZwlrGammaControlV1 => "zwlr_gamma_control_v1",
            #[cfg(feature = "protocol-wlr_input_inhibit_unstable_v1")]
            Self::ZwlrInputInhibitManagerV1 => "zwlr_input_inhibit_manager_v1",
            #[cfg(feature = "protocol-wlr_input_inhibit_unstable_v1")]
            Self::ZwlrInputInhibitorV1 => "zwlr_input_inhibitor_v1",
            #[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")]
            Self::ZwlrLayerShellV1 => "zwlr_layer_shell_v1",
            #[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")]
            Self::ZwlrLayerSurfaceV1 => "zwlr_layer_surface_v1",
            #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
            Self::ZwlrOutputConfigurationHeadV1 => "zwlr_output_configuration_head_v1",
            #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
            Self::ZwlrOutputConfigurationV1 => "zwlr_output_configuration_v1",
            #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
            Self::ZwlrOutputHeadV1 => "zwlr_output_head_v1",
            #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
            Self::ZwlrOutputManagerV1 => "zwlr_output_manager_v1",
            #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
            Self::ZwlrOutputModeV1 => "zwlr_output_mode_v1",
            #[cfg(feature = "protocol-wlr_output_power_management_unstable_v1")]
            Self::ZwlrOutputPowerManagerV1 => "zwlr_output_power_manager_v1",
            #[cfg(feature = "protocol-wlr_output_power_management_unstable_v1")]
            Self::ZwlrOutputPowerV1 => "zwlr_output_power_v1",
            #[cfg(feature = "protocol-wlr_screencopy_unstable_v1")]
            Self::ZwlrScreencopyFrameV1 => "zwlr_screencopy_frame_v1",
            #[cfg(feature = "protocol-wlr_screencopy_unstable_v1")]
            Self::ZwlrScreencopyManagerV1 => "zwlr_screencopy_manager_v1",
            #[cfg(feature = "protocol-wlr_virtual_pointer_unstable_v1")]
            Self::ZwlrVirtualPointerManagerV1 => "zwlr_virtual_pointer_manager_v1",
            #[cfg(feature = "protocol-wlr_virtual_pointer_unstable_v1")]
            Self::ZwlrVirtualPointerV1 => "zwlr_virtual_pointer_v1",
        }
    }

    pub fn xml_version(self) -> u32 {
        match self {
            #[cfg(feature = "protocol-hyprland_ctm_control_v1")]
            Self::HyprlandCtmControlManagerV1 => 2,
            #[cfg(feature = "protocol-hyprland_focus_grab_v1")]
            Self::HyprlandFocusGrabManagerV1 => 1,
            #[cfg(feature = "protocol-hyprland_focus_grab_v1")]
            Self::HyprlandFocusGrabV1 => 1,
            #[cfg(feature = "protocol-hyprland_global_shortcuts_v1")]
            Self::HyprlandGlobalShortcutV1 => 1,
            #[cfg(feature = "protocol-hyprland_global_shortcuts_v1")]
            Self::HyprlandGlobalShortcutsManagerV1 => 1,
            #[cfg(feature = "protocol-hyprland_input_capture_v1")]
            Self::HyprlandInputCaptureManagerV1 => 1,
            #[cfg(feature = "protocol-hyprland_input_capture_v1")]
            Self::HyprlandInputCaptureV1 => 1,
            #[cfg(feature = "protocol-hyprland_lock_notify_v1")]
            Self::HyprlandLockNotificationV1 => 1,
            #[cfg(feature = "protocol-hyprland_lock_notify_v1")]
            Self::HyprlandLockNotifierV1 => 1,
            #[cfg(feature = "protocol-hyprland_surface_v1")]
            Self::HyprlandSurfaceManagerV1 => 2,
            #[cfg(feature = "protocol-hyprland_surface_v1")]
            Self::HyprlandSurfaceV1 => 2,
            #[cfg(feature = "protocol-hyprland_toplevel_export_v1")]
            Self::HyprlandToplevelExportFrameV1 => 2,
            #[cfg(feature = "protocol-hyprland_toplevel_export_v1")]
            Self::HyprlandToplevelExportManagerV1 => 2,
            #[cfg(feature = "protocol-hyprland_toplevel_mapping_v1")]
            Self::HyprlandToplevelMappingManagerV1 => 1,
            #[cfg(feature = "protocol-hyprland_toplevel_mapping_v1")]
            Self::HyprlandToplevelWindowMappingHandleV1 => 1,
            #[cfg(feature = "protocol-jay_tray_v1")]
            Self::JayTrayItemV1 => 1,
            #[cfg(feature = "protocol-jay_tray_v1")]
            Self::JayTrayV1 => 1,
            #[cfg(feature = "protocol-drm")]
            Self::WlDrm => 2,
            #[cfg(feature = "protocol-input_method_unstable_v2")]
            Self::ZwpInputMethodKeyboardGrabV2 => 1,
            #[cfg(feature = "protocol-input_method_unstable_v2")]
            Self::ZwpInputMethodManagerV2 => 1,
            #[cfg(feature = "protocol-input_method_unstable_v2")]
            Self::ZwpInputMethodV2 => 1,
            #[cfg(feature = "protocol-input_method_unstable_v2")]
            Self::ZwpInputPopupSurfaceV2 => 1,
            #[cfg(feature = "protocol-org_kde_kwin_server_decoration_v1")]
            Self::OrgKdeKwinServerDecoration => 1,
            #[cfg(feature = "protocol-org_kde_kwin_server_decoration_v1")]
            Self::OrgKdeKwinServerDecorationManager => 1,
            #[cfg(feature = "protocol-virtual_keyboard_unstable_v1")]
            Self::ZwpVirtualKeyboardManagerV1 => 1,
            #[cfg(feature = "protocol-virtual_keyboard_unstable_v1")]
            Self::ZwpVirtualKeyboardV1 => 1,
            #[cfg(feature = "protocol-treeland_app_id_resolver_v1")]
            Self::TreelandAppIdResolverManagerV1 => 1,
            #[cfg(feature = "protocol-treeland_app_id_resolver_v1")]
            Self::TreelandAppIdResolverV1 => 1,
            #[cfg(feature = "protocol-treeland_dde_shell_v1")]
            Self::TreelandDdeActiveV1 => 1,
            #[cfg(feature = "protocol-treeland_dde_shell_v1")]
            Self::TreelandDdeShellManagerV1 => 1,
            #[cfg(feature = "protocol-treeland_dde_shell_v1")]
            Self::TreelandDdeShellSurfaceV1 => 1,
            #[cfg(feature = "protocol-treeland_dde_shell_v1")]
            Self::TreelandLockscreenV1 => 1,
            #[cfg(feature = "protocol-treeland_dde_shell_v1")]
            Self::TreelandMultitaskviewV1 => 1,
            #[cfg(feature = "protocol-treeland_dde_shell_v1")]
            Self::TreelandWindowOverlapChecker => 1,
            #[cfg(feature = "protocol-treeland_dde_shell_v1")]
            Self::TreelandWindowPickerV1 => 1,
            #[cfg(feature = "protocol-treeland_ddm")]
            Self::TreelandDdm => 1,
            #[cfg(feature = "protocol-treeland_foreign_toplevel_manager_v1")]
            Self::TreelandDockPreviewContextV1 => 1,
            #[cfg(feature = "protocol-treeland_foreign_toplevel_manager_v1")]
            Self::TreelandForeignToplevelHandleV1 => 1,
            #[cfg(feature = "protocol-treeland_foreign_toplevel_manager_v1")]
            Self::TreelandForeignToplevelManagerV1 => 1,
            #[cfg(feature = "protocol-treeland_output_manager_v1")]
            Self::TreelandOutputColorControlV1 => 1,
            #[cfg(feature = "protocol-treeland_output_manager_v1")]
            Self::TreelandOutputManagerV1 => 2,
            #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
            Self::TreelandPersonalizationAppearanceContextV1 => 1,
            #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
            Self::TreelandPersonalizationCursorContextV1 => 1,
            #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
            Self::TreelandPersonalizationFontContextV1 => 1,
            #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
            Self::TreelandPersonalizationManagerV1 => 1,
            #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
            Self::TreelandPersonalizationWallpaperContextV1 => 1,
            #[cfg(feature = "protocol-treeland_personalization_manager_v1")]
            Self::TreelandPersonalizationWindowContextV1 => 1,
            #[cfg(feature = "protocol-treeland_prelaunch_splash_v1")]
            Self::TreelandPrelaunchSplashManagerV1 => 1,
            #[cfg(feature = "protocol-treeland_screensaver")]
            Self::TreelandScreensaver => 1,
            #[cfg(feature = "protocol-treeland_shortcut_manager_v1")]
            Self::TreelandShortcutContextV1 => 1,
            #[cfg(feature = "protocol-treeland_shortcut_manager_v1")]
            Self::TreelandShortcutManagerV1 => 1,
            #[cfg(feature = "protocol-treeland_shortcut_manager_v2")]
            Self::TreelandShortcutManagerV2 => 1,
            #[cfg(feature = "protocol-treeland_virtual_output_manager_v1")]
            Self::TreelandVirtualOutputManagerV1 => 1,
            #[cfg(feature = "protocol-treeland_virtual_output_manager_v1")]
            Self::TreelandVirtualOutputV1 => 1,
            #[cfg(feature = "protocol-treeland_wallpaper_color_v1")]
            Self::TreelandWallpaperColorManagerV1 => 1,
            #[cfg(feature = "protocol-treeland_window_management_v1")]
            Self::TreelandWindowManagementV1 => 1,
            Self::WlBuffer => 1,
            Self::WlCallback => 1,
            Self::WlCompositor => 6,
            Self::WlDataDevice => 3,
            Self::WlDataDeviceManager => 3,
            Self::WlDataOffer => 3,
            Self::WlDataSource => 3,
            Self::WlDisplay => 1,
            Self::WlFixes => 1,
            Self::WlKeyboard => 10,
            Self::WlOutput => 4,
            Self::WlPointer => 10,
            Self::WlRegion => 1,
            Self::WlRegistry => 1,
            Self::WlSeat => 10,
            Self::WlShell => 1,
            Self::WlShellSurface => 1,
            Self::WlShm => 2,
            Self::WlShmPool => 2,
            Self::WlSubcompositor => 1,
            Self::WlSubsurface => 1,
            Self::WlSurface => 6,
            Self::WlTouch => 10,
            #[cfg(feature = "protocol-alpha_modifier_v1")]
            Self::WpAlphaModifierSurfaceV1 => 1,
            #[cfg(feature = "protocol-alpha_modifier_v1")]
            Self::WpAlphaModifierV1 => 1,
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpColorManagementOutputV1 => 1,
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpColorManagementSurfaceFeedbackV1 => 1,
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpColorManagementSurfaceV1 => 1,
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpColorManagerV1 => 1,
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpImageDescriptionCreatorIccV1 => 1,
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpImageDescriptionCreatorParamsV1 => 1,
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpImageDescriptionInfoV1 => 1,
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpImageDescriptionV1 => 1,
            #[cfg(feature = "protocol-color_representation_v1")]
            Self::WpColorRepresentationManagerV1 => 1,
            #[cfg(feature = "protocol-color_representation_v1")]
            Self::WpColorRepresentationSurfaceV1 => 1,
            #[cfg(feature = "protocol-commit_timing_v1")]
            Self::WpCommitTimerV1 => 1,
            #[cfg(feature = "protocol-commit_timing_v1")]
            Self::WpCommitTimingManagerV1 => 1,
            #[cfg(feature = "protocol-content_type_v1")]
            Self::WpContentTypeManagerV1 => 1,
            #[cfg(feature = "protocol-content_type_v1")]
            Self::WpContentTypeV1 => 1,
            #[cfg(feature = "protocol-cursor_shape_v1")]
            Self::WpCursorShapeDeviceV1 => 2,
            #[cfg(feature = "protocol-cursor_shape_v1")]
            Self::WpCursorShapeManagerV1 => 2,
            #[cfg(feature = "protocol-drm_lease_v1")]
            Self::WpDrmLeaseConnectorV1 => 1,
            #[cfg(feature = "protocol-drm_lease_v1")]
            Self::WpDrmLeaseDeviceV1 => 1,
            #[cfg(feature = "protocol-drm_lease_v1")]
            Self::WpDrmLeaseRequestV1 => 1,
            #[cfg(feature = "protocol-drm_lease_v1")]
            Self::WpDrmLeaseV1 => 1,
            #[cfg(feature = "protocol-ext_background_effect_v1")]
            Self::ExtBackgroundEffectManagerV1 => 1,
            #[cfg(feature = "protocol-ext_background_effect_v1")]
            Self::ExtBackgroundEffectSurfaceV1 => 1,
            #[cfg(feature = "protocol-ext_data_control_v1")]
            Self::ExtDataControlDeviceV1 => 1,
            #[cfg(feature = "protocol-ext_data_control_v1")]
            Self::ExtDataControlManagerV1 => 1,
            #[cfg(feature = "protocol-ext_data_control_v1")]
            Self::ExtDataControlOfferV1 => 1,
            #[cfg(feature = "protocol-ext_data_control_v1")]
            Self::ExtDataControlSourceV1 => 1,
            #[cfg(feature = "protocol-ext_foreign_toplevel_list_v1")]
            Self::ExtForeignToplevelHandleV1 => 1,
            #[cfg(feature = "protocol-ext_foreign_toplevel_list_v1")]
            Self::ExtForeignToplevelListV1 => 1,
            #[cfg(feature = "protocol-ext_idle_notify_v1")]
            Self::ExtIdleNotificationV1 => 2,
            #[cfg(feature = "protocol-ext_idle_notify_v1")]
            Self::ExtIdleNotifierV1 => 2,
            #[cfg(feature = "protocol-ext_image_capture_source_v1")]
            Self::ExtForeignToplevelImageCaptureSourceManagerV1 => 1,
            #[cfg(feature = "protocol-ext_image_capture_source_v1")]
            Self::ExtImageCaptureSourceV1 => 1,
            #[cfg(feature = "protocol-ext_image_capture_source_v1")]
            Self::ExtOutputImageCaptureSourceManagerV1 => 1,
            #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
            Self::ExtImageCopyCaptureCursorSessionV1 => 1,
            #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
            Self::ExtImageCopyCaptureFrameV1 => 1,
            #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
            Self::ExtImageCopyCaptureManagerV1 => 1,
            #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
            Self::ExtImageCopyCaptureSessionV1 => 1,
            #[cfg(feature = "protocol-ext_session_lock_v1")]
            Self::ExtSessionLockManagerV1 => 1,
            #[cfg(feature = "protocol-ext_session_lock_v1")]
            Self::ExtSessionLockSurfaceV1 => 1,
            #[cfg(feature = "protocol-ext_session_lock_v1")]
            Self::ExtSessionLockV1 => 1,
            #[cfg(feature = "protocol-ext_transient_seat_v1")]
            Self::ExtTransientSeatManagerV1 => 1,
            #[cfg(feature = "protocol-ext_transient_seat_v1")]
            Self::ExtTransientSeatV1 => 1,
            #[cfg(feature = "protocol-ext_workspace_v1")]
            Self::ExtWorkspaceGroupHandleV1 => 1,
            #[cfg(feature = "protocol-ext_workspace_v1")]
            Self::ExtWorkspaceHandleV1 => 1,
            #[cfg(feature = "protocol-ext_workspace_v1")]
            Self::ExtWorkspaceManagerV1 => 1,
            #[cfg(feature = "protocol-fifo_v1")]
            Self::WpFifoManagerV1 => 1,
            #[cfg(feature = "protocol-fifo_v1")]
            Self::WpFifoV1 => 1,
            #[cfg(feature = "protocol-fractional_scale_v1")]
            Self::WpFractionalScaleManagerV1 => 1,
            #[cfg(feature = "protocol-fractional_scale_v1")]
            Self::WpFractionalScaleV1 => 1,
            #[cfg(feature = "protocol-fullscreen_shell_unstable_v1")]
            Self::ZwpFullscreenShellModeFeedbackV1 => 1,
            #[cfg(feature = "protocol-fullscreen_shell_unstable_v1")]
            Self::ZwpFullscreenShellV1 => 1,
            #[cfg(feature = "protocol-idle_inhibit_unstable_v1")]
            Self::ZwpIdleInhibitManagerV1 => 1,
            #[cfg(feature = "protocol-idle_inhibit_unstable_v1")]
            Self::ZwpIdleInhibitorV1 => 1,
            #[cfg(feature = "protocol-input_method_unstable_v1")]
            Self::ZwpInputMethodContextV1 => 1,
            #[cfg(feature = "protocol-input_method_unstable_v1")]
            Self::ZwpInputMethodV1 => 1,
            #[cfg(feature = "protocol-input_method_unstable_v1")]
            Self::ZwpInputPanelSurfaceV1 => 1,
            #[cfg(feature = "protocol-input_method_unstable_v1")]
            Self::ZwpInputPanelV1 => 1,
            #[cfg(feature = "protocol-input_timestamps_unstable_v1")]
            Self::ZwpInputTimestampsManagerV1 => 1,
            #[cfg(feature = "protocol-input_timestamps_unstable_v1")]
            Self::ZwpInputTimestampsV1 => 1,
            #[cfg(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1")]
            Self::ZwpKeyboardShortcutsInhibitManagerV1 => 1,
            #[cfg(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1")]
            Self::ZwpKeyboardShortcutsInhibitorV1 => 1,
            #[cfg(feature = "protocol-linux_dmabuf_v1")]
            Self::ZwpLinuxBufferParamsV1 => 5,
            #[cfg(feature = "protocol-linux_dmabuf_v1")]
            Self::ZwpLinuxDmabufFeedbackV1 => 5,
            #[cfg(feature = "protocol-linux_dmabuf_v1")]
            Self::ZwpLinuxDmabufV1 => 5,
            #[cfg(feature = "protocol-linux_drm_syncobj_v1")]
            Self::WpLinuxDrmSyncobjManagerV1 => 1,
            #[cfg(feature = "protocol-linux_drm_syncobj_v1")]
            Self::WpLinuxDrmSyncobjSurfaceV1 => 1,
            #[cfg(feature = "protocol-linux_drm_syncobj_v1")]
            Self::WpLinuxDrmSyncobjTimelineV1 => 1,
            #[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
            Self::ZwpConfinedPointerV1 => 1,
            #[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
            Self::ZwpLockedPointerV1 => 1,
            #[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
            Self::ZwpPointerConstraintsV1 => 1,
            #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
            Self::ZwpPointerGestureHoldV1 => 3,
            #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
            Self::ZwpPointerGesturePinchV1 => 2,
            #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
            Self::ZwpPointerGestureSwipeV1 => 2,
            #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
            Self::ZwpPointerGesturesV1 => 3,
            #[cfg(feature = "protocol-pointer_warp_v1")]
            Self::WpPointerWarpV1 => 1,
            #[cfg(feature = "protocol-presentation_time")]
            Self::WpPresentation => 2,
            #[cfg(feature = "protocol-presentation_time")]
            Self::WpPresentationFeedback => 2,
            #[cfg(feature = "protocol-relative_pointer_unstable_v1")]
            Self::ZwpRelativePointerManagerV1 => 1,
            #[cfg(feature = "protocol-relative_pointer_unstable_v1")]
            Self::ZwpRelativePointerV1 => 1,
            #[cfg(feature = "protocol-security_context_v1")]
            Self::WpSecurityContextManagerV1 => 1,
            #[cfg(feature = "protocol-security_context_v1")]
            Self::WpSecurityContextV1 => 1,
            #[cfg(feature = "protocol-single_pixel_buffer_v1")]
            Self::WpSinglePixelBufferManagerV1 => 1,
            #[cfg(feature = "protocol-tablet_v2")]
            Self::ZwpTabletManagerV2 => 2,
            #[cfg(feature = "protocol-tablet_v2")]
            Self::ZwpTabletPadDialV2 => 2,
            #[cfg(feature = "protocol-tablet_v2")]
            Self::ZwpTabletPadGroupV2 => 2,
            #[cfg(feature = "protocol-tablet_v2")]
            Self::ZwpTabletPadRingV2 => 2,
            #[cfg(feature = "protocol-tablet_v2")]
            Self::ZwpTabletPadStripV2 => 2,
            #[cfg(feature = "protocol-tablet_v2")]
            Self::ZwpTabletPadV2 => 2,
            #[cfg(feature = "protocol-tablet_v2")]
            Self::ZwpTabletSeatV2 => 2,
            #[cfg(feature = "protocol-tablet_v2")]
            Self::ZwpTabletToolV2 => 2,
            #[cfg(feature = "protocol-tablet_v2")]
            Self::ZwpTabletV2 => 2,
            #[cfg(feature = "protocol-tearing_control_v1")]
            Self::WpTearingControlManagerV1 => 1,
            #[cfg(feature = "protocol-tearing_control_v1")]
            Self::WpTearingControlV1 => 1,
            #[cfg(feature = "protocol-text_input_unstable_v1")]
            Self::ZwpTextInputManagerV1 => 1,
            #[cfg(feature = "protocol-text_input_unstable_v1")]
            Self::ZwpTextInputV1 => 1,
            #[cfg(feature = "protocol-text_input_unstable_v3")]
            Self::ZwpTextInputManagerV3 => 1,
            #[cfg(feature = "protocol-text_input_unstable_v3")]
            Self::ZwpTextInputV3 => 1,
            #[cfg(feature = "protocol-viewporter")]
            Self::WpViewport => 1,
            #[cfg(feature = "protocol-viewporter")]
            Self::WpViewporter => 1,
            #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
            Self::ZwpPrimarySelectionDeviceManagerV1 => 1,
            #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
            Self::ZwpPrimarySelectionDeviceV1 => 1,
            #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
            Self::ZwpPrimarySelectionOfferV1 => 1,
            #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
            Self::ZwpPrimarySelectionSourceV1 => 1,
            #[cfg(feature = "protocol-xdg_activation_v1")]
            Self::XdgActivationTokenV1 => 1,
            #[cfg(feature = "protocol-xdg_activation_v1")]
            Self::XdgActivationV1 => 1,
            #[cfg(feature = "protocol-xdg_decoration_unstable_v1")]
            Self::ZxdgDecorationManagerV1 => 1,
            #[cfg(feature = "protocol-xdg_decoration_unstable_v1")]
            Self::ZxdgToplevelDecorationV1 => 1,
            #[cfg(feature = "protocol-xdg_dialog_v1")]
            Self::XdgDialogV1 => 1,
            #[cfg(feature = "protocol-xdg_dialog_v1")]
            Self::XdgWmDialogV1 => 1,
            #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
            Self::ZxdgExportedV2 => 1,
            #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
            Self::ZxdgExporterV2 => 1,
            #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
            Self::ZxdgImportedV2 => 1,
            #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
            Self::ZxdgImporterV2 => 1,
            #[cfg(feature = "protocol-xdg_output_unstable_v1")]
            Self::ZxdgOutputManagerV1 => 3,
            #[cfg(feature = "protocol-xdg_output_unstable_v1")]
            Self::ZxdgOutputV1 => 3,
            #[cfg(feature = "protocol-xdg_shell")]
            Self::XdgPopup => 7,
            #[cfg(feature = "protocol-xdg_shell")]
            Self::XdgPositioner => 7,
            #[cfg(feature = "protocol-xdg_shell")]
            Self::XdgSurface => 7,
            #[cfg(feature = "protocol-xdg_shell")]
            Self::XdgToplevel => 7,
            #[cfg(feature = "protocol-xdg_shell")]
            Self::XdgWmBase => 7,
            #[cfg(feature = "protocol-xdg_system_bell_v1")]
            Self::XdgSystemBellV1 => 1,
            #[cfg(feature = "protocol-xdg_toplevel_drag_v1")]
            Self::XdgToplevelDragManagerV1 => 1,
            #[cfg(feature = "protocol-xdg_toplevel_drag_v1")]
            Self::XdgToplevelDragV1 => 1,
            #[cfg(feature = "protocol-xdg_toplevel_icon_v1")]
            Self::XdgToplevelIconManagerV1 => 1,
            #[cfg(feature = "protocol-xdg_toplevel_icon_v1")]
            Self::XdgToplevelIconV1 => 1,
            #[cfg(feature = "protocol-xdg_toplevel_tag_v1")]
            Self::XdgToplevelTagManagerV1 => 1,
            #[cfg(feature = "protocol-xwayland_keyboard_grab_unstable_v1")]
            Self::ZwpXwaylandKeyboardGrabManagerV1 => 1,
            #[cfg(feature = "protocol-xwayland_keyboard_grab_unstable_v1")]
            Self::ZwpXwaylandKeyboardGrabV1 => 1,
            #[cfg(feature = "protocol-xwayland_shell_v1")]
            Self::XwaylandShellV1 => 1,
            #[cfg(feature = "protocol-xwayland_shell_v1")]
            Self::XwaylandSurfaceV1 => 1,
            #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
            Self::ZwpLinuxBufferReleaseV1 => 1,
            #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
            Self::ZwpLinuxExplicitSynchronizationV1 => 2,
            #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
            Self::ZwpLinuxSurfaceSynchronizationV1 => 2,
            #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
            Self::ZwlrDataControlDeviceV1 => 2,
            #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
            Self::ZwlrDataControlManagerV1 => 2,
            #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
            Self::ZwlrDataControlOfferV1 => 1,
            #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
            Self::ZwlrDataControlSourceV1 => 1,
            #[cfg(feature = "protocol-wlr_export_dmabuf_unstable_v1")]
            Self::ZwlrExportDmabufFrameV1 => 1,
            #[cfg(feature = "protocol-wlr_export_dmabuf_unstable_v1")]
            Self::ZwlrExportDmabufManagerV1 => 1,
            #[cfg(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1")]
            Self::ZwlrForeignToplevelHandleV1 => 3,
            #[cfg(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1")]
            Self::ZwlrForeignToplevelManagerV1 => 3,
            #[cfg(feature = "protocol-wlr_gamma_control_unstable_v1")]
            Self::ZwlrGammaControlManagerV1 => 1,
            #[cfg(feature = "protocol-wlr_gamma_control_unstable_v1")]
            Self::ZwlrGammaControlV1 => 1,
            #[cfg(feature = "protocol-wlr_input_inhibit_unstable_v1")]
            Self::ZwlrInputInhibitManagerV1 => 1,
            #[cfg(feature = "protocol-wlr_input_inhibit_unstable_v1")]
            Self::ZwlrInputInhibitorV1 => 1,
            #[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")]
            Self::ZwlrLayerShellV1 => 5,
            #[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")]
            Self::ZwlrLayerSurfaceV1 => 5,
            #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
            Self::ZwlrOutputConfigurationHeadV1 => 4,
            #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
            Self::ZwlrOutputConfigurationV1 => 4,
            #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
            Self::ZwlrOutputHeadV1 => 4,
            #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
            Self::ZwlrOutputManagerV1 => 4,
            #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
            Self::ZwlrOutputModeV1 => 3,
            #[cfg(feature = "protocol-wlr_output_power_management_unstable_v1")]
            Self::ZwlrOutputPowerManagerV1 => 1,
            #[cfg(feature = "protocol-wlr_output_power_management_unstable_v1")]
            Self::ZwlrOutputPowerV1 => 1,
            #[cfg(feature = "protocol-wlr_screencopy_unstable_v1")]
            Self::ZwlrScreencopyFrameV1 => 3,
            #[cfg(feature = "protocol-wlr_screencopy_unstable_v1")]
            Self::ZwlrScreencopyManagerV1 => 3,
            #[cfg(feature = "protocol-wlr_virtual_pointer_unstable_v1")]
            Self::ZwlrVirtualPointerManagerV1 => 2,
            #[cfg(feature = "protocol-wlr_virtual_pointer_unstable_v1")]
            Self::ZwlrVirtualPointerV1 => 2,
        }
    }
}
