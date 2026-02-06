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
#[cfg(feature = "protocol-jay_popup_ext_v1")]
pub mod jay_popup_ext_v1;
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
#[cfg(feature = "protocol-xx_zones_v1")]
pub mod xx_zones_v1;
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
#[cfg(feature = "protocol-wlproxy_sync_v1")]
pub mod wlproxy_sync_v1;
#[cfg(test)]
pub mod wlproxy_test;
#[cfg(feature = "protocol-river_input_management_v1")]
pub mod river_input_management_v1;
#[cfg(feature = "protocol-river_layer_shell_v1")]
pub mod river_layer_shell_v1;
#[cfg(feature = "protocol-river_libinput_config_v1")]
pub mod river_libinput_config_v1;
#[cfg(feature = "protocol-river_window_management_v1")]
pub mod river_window_management_v1;
#[cfg(feature = "protocol-river_xkb_bindings_v1")]
pub mod river_xkb_bindings_v1;
#[cfg(feature = "protocol-river_xkb_config_v1")]
pub mod river_xkb_config_v1;
#[cfg(feature = "protocol-ivi_application")]
pub mod ivi_application;
#[cfg(feature = "protocol-ivi_hmi_controller")]
pub mod ivi_hmi_controller;
#[cfg(feature = "protocol-weston_content_protection")]
pub mod weston_content_protection;
#[cfg(feature = "protocol-weston_debug")]
pub mod weston_debug;
#[cfg(feature = "protocol-weston_desktop")]
pub mod weston_desktop;
#[cfg(feature = "protocol-weston_direct_display")]
pub mod weston_direct_display;
#[cfg(feature = "protocol-weston_output_capture")]
pub mod weston_output_capture;
#[cfg(feature = "protocol-weston_test")]
pub mod weston_test;
#[cfg(feature = "protocol-weston_touch_calibration")]
pub mod weston_touch_calibration;
#[cfg(feature = "protocol-cosmic_a11y_v1")]
pub mod cosmic_a11y_v1;
#[cfg(feature = "protocol-cosmic_corner_radius_v1")]
pub mod cosmic_corner_radius_v1;
#[cfg(feature = "protocol-cosmic_image_source_unstable_v1")]
pub mod cosmic_image_source_unstable_v1;
#[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
pub mod cosmic_output_management_unstable_v1;
#[cfg(feature = "protocol-cosmic_overlap_notify_unstable_v1")]
pub mod cosmic_overlap_notify_unstable_v1;
#[cfg(feature = "protocol-cosmic_workspace_unstable_v2")]
pub mod cosmic_workspace_unstable_v2;

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
    #[cfg(feature = "protocol-jay_popup_ext_v1")]
    pub(super) use super::jay_popup_ext_v1::jay_popup_ext_manager_v1::JayPopupExtManagerV1;
    #[cfg(feature = "protocol-jay_popup_ext_v1")]
    pub(super) use super::jay_popup_ext_v1::jay_popup_ext_manager_v1::JayPopupExtManagerV1Error;
    #[cfg(feature = "protocol-jay_popup_ext_v1")]
    pub(super) use super::jay_popup_ext_v1::jay_popup_ext_v1::JayPopupExtV1;
    #[cfg(feature = "protocol-jay_popup_ext_v1")]
    pub(super) use super::jay_popup_ext_v1::jay_popup_ext_v1::JayPopupExtV1Error;
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
    pub(super) use super::color_management_v1::wp_image_description_reference_v1::WpImageDescriptionReferenceV1;
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
    #[cfg(feature = "protocol-xx_zones_v1")]
    pub(super) use super::xx_zones_v1::xx_zone_item_v1::XxZoneItemV1;
    #[cfg(feature = "protocol-xx_zones_v1")]
    pub(super) use super::xx_zones_v1::xx_zone_manager_v1::XxZoneManagerV1;
    #[cfg(feature = "protocol-xx_zones_v1")]
    pub(super) use super::xx_zones_v1::xx_zone_v1::XxZoneV1;
    #[cfg(feature = "protocol-xx_zones_v1")]
    pub(super) use super::xx_zones_v1::xx_zone_v1::XxZoneV1Error;
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
    #[cfg(feature = "protocol-wlproxy_sync_v1")]
    pub(super) use super::wlproxy_sync_v1::wlproxy_sync_v1::WlproxySyncV1;
    #[cfg(test)]
    pub(super) use super::wlproxy_test::wlproxy_test::WlproxyTest;
    #[cfg(test)]
    pub(super) use super::wlproxy_test::wlproxy_test_array_echo::WlproxyTestArrayEcho;
    #[cfg(test)]
    pub(super) use super::wlproxy_test::wlproxy_test_dummy::WlproxyTestDummy;
    #[cfg(test)]
    pub(super) use super::wlproxy_test::wlproxy_test_fd_echo::WlproxyTestFdEcho;
    #[cfg(test)]
    pub(super) use super::wlproxy_test::wlproxy_test_hops::WlproxyTestHops;
    #[cfg(test)]
    pub(super) use super::wlproxy_test::wlproxy_test_non_forward::WlproxyTestNonForward;
    #[cfg(test)]
    pub(super) use super::wlproxy_test::wlproxy_test_object_echo::WlproxyTestObjectEcho;
    #[cfg(test)]
    pub(super) use super::wlproxy_test::wlproxy_test_server_sent::WlproxyTestServerSent;
    #[cfg(feature = "protocol-river_input_management_v1")]
    pub(super) use super::river_input_management_v1::river_input_device_v1::RiverInputDeviceV1;
    #[cfg(feature = "protocol-river_input_management_v1")]
    pub(super) use super::river_input_management_v1::river_input_device_v1::RiverInputDeviceV1Error;
    #[cfg(feature = "protocol-river_input_management_v1")]
    pub(super) use super::river_input_management_v1::river_input_device_v1::RiverInputDeviceV1Type;
    #[cfg(feature = "protocol-river_input_management_v1")]
    pub(super) use super::river_input_management_v1::river_input_manager_v1::RiverInputManagerV1;
    #[cfg(feature = "protocol-river_input_management_v1")]
    pub(super) use super::river_input_management_v1::river_input_manager_v1::RiverInputManagerV1Error;
    #[cfg(feature = "protocol-river_layer_shell_v1")]
    pub(super) use super::river_layer_shell_v1::river_layer_shell_output_v1::RiverLayerShellOutputV1;
    #[cfg(feature = "protocol-river_layer_shell_v1")]
    pub(super) use super::river_layer_shell_v1::river_layer_shell_seat_v1::RiverLayerShellSeatV1;
    #[cfg(feature = "protocol-river_layer_shell_v1")]
    pub(super) use super::river_layer_shell_v1::river_layer_shell_v1::RiverLayerShellV1;
    #[cfg(feature = "protocol-river_layer_shell_v1")]
    pub(super) use super::river_layer_shell_v1::river_layer_shell_v1::RiverLayerShellV1Error;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_accel_config_v1::RiverLibinputAccelConfigV1;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_accel_config_v1::RiverLibinputAccelConfigV1Error;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_accel_config_v1::RiverLibinputAccelConfigV1AccelType;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_config_v1::RiverLibinputConfigV1;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_config_v1::RiverLibinputConfigV1Error;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_device_v1::RiverLibinputDeviceV1;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_device_v1::RiverLibinputDeviceV1Error;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_device_v1::RiverLibinputDeviceV1SendEventsModes;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_device_v1::RiverLibinputDeviceV1TapState;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_device_v1::RiverLibinputDeviceV1TapButtonMap;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_device_v1::RiverLibinputDeviceV1DragState;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_device_v1::RiverLibinputDeviceV1DragLockState;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_device_v1::RiverLibinputDeviceV1ThreeFingerDragState;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_device_v1::RiverLibinputDeviceV1AccelProfile;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_device_v1::RiverLibinputDeviceV1AccelProfiles;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_device_v1::RiverLibinputDeviceV1NaturalScrollState;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_device_v1::RiverLibinputDeviceV1LeftHandedState;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_device_v1::RiverLibinputDeviceV1ClickMethod;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_device_v1::RiverLibinputDeviceV1ClickMethods;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_device_v1::RiverLibinputDeviceV1ClickfingerButtonMap;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_device_v1::RiverLibinputDeviceV1MiddleEmulationState;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_device_v1::RiverLibinputDeviceV1ScrollMethod;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_device_v1::RiverLibinputDeviceV1ScrollMethods;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_device_v1::RiverLibinputDeviceV1ScrollButtonLockState;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_device_v1::RiverLibinputDeviceV1DwtState;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_device_v1::RiverLibinputDeviceV1DwtpState;
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    pub(super) use super::river_libinput_config_v1::river_libinput_result_v1::RiverLibinputResultV1;
    #[cfg(feature = "protocol-river_window_management_v1")]
    pub(super) use super::river_window_management_v1::river_decoration_v1::RiverDecorationV1;
    #[cfg(feature = "protocol-river_window_management_v1")]
    pub(super) use super::river_window_management_v1::river_decoration_v1::RiverDecorationV1Error;
    #[cfg(feature = "protocol-river_window_management_v1")]
    pub(super) use super::river_window_management_v1::river_node_v1::RiverNodeV1;
    #[cfg(feature = "protocol-river_window_management_v1")]
    pub(super) use super::river_window_management_v1::river_output_v1::RiverOutputV1;
    #[cfg(feature = "protocol-river_window_management_v1")]
    pub(super) use super::river_window_management_v1::river_pointer_binding_v1::RiverPointerBindingV1;
    #[cfg(feature = "protocol-river_window_management_v1")]
    pub(super) use super::river_window_management_v1::river_seat_v1::RiverSeatV1;
    #[cfg(feature = "protocol-river_window_management_v1")]
    pub(super) use super::river_window_management_v1::river_seat_v1::RiverSeatV1Modifiers;
    #[cfg(feature = "protocol-river_window_management_v1")]
    pub(super) use super::river_window_management_v1::river_shell_surface_v1::RiverShellSurfaceV1;
    #[cfg(feature = "protocol-river_window_management_v1")]
    pub(super) use super::river_window_management_v1::river_shell_surface_v1::RiverShellSurfaceV1Error;
    #[cfg(feature = "protocol-river_window_management_v1")]
    pub(super) use super::river_window_management_v1::river_window_manager_v1::RiverWindowManagerV1;
    #[cfg(feature = "protocol-river_window_management_v1")]
    pub(super) use super::river_window_management_v1::river_window_manager_v1::RiverWindowManagerV1Error;
    #[cfg(feature = "protocol-river_window_management_v1")]
    pub(super) use super::river_window_management_v1::river_window_v1::RiverWindowV1;
    #[cfg(feature = "protocol-river_window_management_v1")]
    pub(super) use super::river_window_management_v1::river_window_v1::RiverWindowV1Error;
    #[cfg(feature = "protocol-river_window_management_v1")]
    pub(super) use super::river_window_management_v1::river_window_v1::RiverWindowV1DecorationHint;
    #[cfg(feature = "protocol-river_window_management_v1")]
    pub(super) use super::river_window_management_v1::river_window_v1::RiverWindowV1Edges;
    #[cfg(feature = "protocol-river_window_management_v1")]
    pub(super) use super::river_window_management_v1::river_window_v1::RiverWindowV1Capabilities;
    #[cfg(feature = "protocol-river_xkb_bindings_v1")]
    pub(super) use super::river_xkb_bindings_v1::river_xkb_binding_v1::RiverXkbBindingV1;
    #[cfg(feature = "protocol-river_xkb_bindings_v1")]
    pub(super) use super::river_xkb_bindings_v1::river_xkb_bindings_seat_v1::RiverXkbBindingsSeatV1;
    #[cfg(feature = "protocol-river_xkb_bindings_v1")]
    pub(super) use super::river_xkb_bindings_v1::river_xkb_bindings_v1::RiverXkbBindingsV1;
    #[cfg(feature = "protocol-river_xkb_bindings_v1")]
    pub(super) use super::river_xkb_bindings_v1::river_xkb_bindings_v1::RiverXkbBindingsV1Error;
    #[cfg(feature = "protocol-river_xkb_config_v1")]
    pub(super) use super::river_xkb_config_v1::river_xkb_config_v1::RiverXkbConfigV1;
    #[cfg(feature = "protocol-river_xkb_config_v1")]
    pub(super) use super::river_xkb_config_v1::river_xkb_config_v1::RiverXkbConfigV1Error;
    #[cfg(feature = "protocol-river_xkb_config_v1")]
    pub(super) use super::river_xkb_config_v1::river_xkb_config_v1::RiverXkbConfigV1KeymapFormat;
    #[cfg(feature = "protocol-river_xkb_config_v1")]
    pub(super) use super::river_xkb_config_v1::river_xkb_keyboard_v1::RiverXkbKeyboardV1;
    #[cfg(feature = "protocol-river_xkb_config_v1")]
    pub(super) use super::river_xkb_config_v1::river_xkb_keyboard_v1::RiverXkbKeyboardV1Error;
    #[cfg(feature = "protocol-river_xkb_config_v1")]
    pub(super) use super::river_xkb_config_v1::river_xkb_keymap_v1::RiverXkbKeymapV1;
    #[cfg(feature = "protocol-ivi_application")]
    pub(super) use super::ivi_application::ivi_application::IviApplication;
    #[cfg(feature = "protocol-ivi_application")]
    pub(super) use super::ivi_application::ivi_application::IviApplicationError;
    #[cfg(feature = "protocol-ivi_application")]
    pub(super) use super::ivi_application::ivi_surface::IviSurface;
    #[cfg(feature = "protocol-ivi_hmi_controller")]
    pub(super) use super::ivi_hmi_controller::ivi_hmi_controller::IviHmiController;
    #[cfg(feature = "protocol-ivi_hmi_controller")]
    pub(super) use super::ivi_hmi_controller::ivi_hmi_controller::IviHmiControllerLayoutMode;
    #[cfg(feature = "protocol-ivi_hmi_controller")]
    pub(super) use super::ivi_hmi_controller::ivi_hmi_controller::IviHmiControllerHome;
    #[cfg(feature = "protocol-weston_content_protection")]
    pub(super) use super::weston_content_protection::weston_content_protection::WestonContentProtection;
    #[cfg(feature = "protocol-weston_content_protection")]
    pub(super) use super::weston_content_protection::weston_content_protection::WestonContentProtectionError;
    #[cfg(feature = "protocol-weston_content_protection")]
    pub(super) use super::weston_content_protection::weston_protected_surface::WestonProtectedSurface;
    #[cfg(feature = "protocol-weston_content_protection")]
    pub(super) use super::weston_content_protection::weston_protected_surface::WestonProtectedSurfaceError;
    #[cfg(feature = "protocol-weston_content_protection")]
    pub(super) use super::weston_content_protection::weston_protected_surface::WestonProtectedSurfaceType;
    #[cfg(feature = "protocol-weston_debug")]
    pub(super) use super::weston_debug::weston_debug_stream_v1::WestonDebugStreamV1;
    #[cfg(feature = "protocol-weston_debug")]
    pub(super) use super::weston_debug::weston_debug_v1::WestonDebugV1;
    #[cfg(feature = "protocol-weston_desktop")]
    pub(super) use super::weston_desktop::weston_desktop_shell::WestonDesktopShell;
    #[cfg(feature = "protocol-weston_desktop")]
    pub(super) use super::weston_desktop::weston_desktop_shell::WestonDesktopShellCursor;
    #[cfg(feature = "protocol-weston_desktop")]
    pub(super) use super::weston_desktop::weston_desktop_shell::WestonDesktopShellPanelPosition;
    #[cfg(feature = "protocol-weston_desktop")]
    pub(super) use super::weston_desktop::weston_desktop_shell::WestonDesktopShellError;
    #[cfg(feature = "protocol-weston_desktop")]
    pub(super) use super::weston_desktop::weston_screensaver::WestonScreensaver;
    #[cfg(feature = "protocol-weston_direct_display")]
    pub(super) use super::weston_direct_display::weston_direct_display_v1::WestonDirectDisplayV1;
    #[cfg(feature = "protocol-weston_output_capture")]
    pub(super) use super::weston_output_capture::weston_capture_source_v1::WestonCaptureSourceV1;
    #[cfg(feature = "protocol-weston_output_capture")]
    pub(super) use super::weston_output_capture::weston_capture_source_v1::WestonCaptureSourceV1Error;
    #[cfg(feature = "protocol-weston_output_capture")]
    pub(super) use super::weston_output_capture::weston_capture_v1::WestonCaptureV1;
    #[cfg(feature = "protocol-weston_output_capture")]
    pub(super) use super::weston_output_capture::weston_capture_v1::WestonCaptureV1Error;
    #[cfg(feature = "protocol-weston_output_capture")]
    pub(super) use super::weston_output_capture::weston_capture_v1::WestonCaptureV1Source;
    #[cfg(feature = "protocol-weston_test")]
    pub(super) use super::weston_test::weston_test::WestonTest;
    #[cfg(feature = "protocol-weston_test")]
    pub(super) use super::weston_test::weston_test::WestonTestError;
    #[cfg(feature = "protocol-weston_test")]
    pub(super) use super::weston_test::weston_test::WestonTestBreakpoint;
    #[cfg(feature = "protocol-weston_test")]
    pub(super) use super::weston_test::weston_test_runner::WestonTestRunner;
    #[cfg(feature = "protocol-weston_test")]
    pub(super) use super::weston_test::weston_test_runner::WestonTestRunnerError;
    #[cfg(feature = "protocol-weston_touch_calibration")]
    pub(super) use super::weston_touch_calibration::weston_touch_calibration::WestonTouchCalibration;
    #[cfg(feature = "protocol-weston_touch_calibration")]
    pub(super) use super::weston_touch_calibration::weston_touch_calibration::WestonTouchCalibrationError;
    #[cfg(feature = "protocol-weston_touch_calibration")]
    pub(super) use super::weston_touch_calibration::weston_touch_calibrator::WestonTouchCalibrator;
    #[cfg(feature = "protocol-weston_touch_calibration")]
    pub(super) use super::weston_touch_calibration::weston_touch_calibrator::WestonTouchCalibratorError;
    #[cfg(feature = "protocol-weston_touch_calibration")]
    pub(super) use super::weston_touch_calibration::weston_touch_coordinate::WestonTouchCoordinate;
    #[cfg(feature = "protocol-cosmic_a11y_v1")]
    pub(super) use super::cosmic_a11y_v1::cosmic_a11y_manager_v1::CosmicA11yManagerV1;
    #[cfg(feature = "protocol-cosmic_a11y_v1")]
    pub(super) use super::cosmic_a11y_v1::cosmic_a11y_manager_v1::CosmicA11yManagerV1ActiveState;
    #[cfg(feature = "protocol-cosmic_a11y_v1")]
    pub(super) use super::cosmic_a11y_v1::cosmic_a11y_manager_v1::CosmicA11yManagerV1Filter;
    #[cfg(feature = "protocol-cosmic_a11y_v1")]
    pub(super) use super::cosmic_a11y_v1::cosmic_a11y_manager_v1::CosmicA11yManagerV1Error;
    #[cfg(feature = "protocol-cosmic_corner_radius_v1")]
    pub(super) use super::cosmic_corner_radius_v1::cosmic_corner_radius_manager_v1::CosmicCornerRadiusManagerV1;
    #[cfg(feature = "protocol-cosmic_corner_radius_v1")]
    pub(super) use super::cosmic_corner_radius_v1::cosmic_corner_radius_manager_v1::CosmicCornerRadiusManagerV1Error;
    #[cfg(feature = "protocol-cosmic_corner_radius_v1")]
    pub(super) use super::cosmic_corner_radius_v1::cosmic_corner_radius_toplevel_v1::CosmicCornerRadiusToplevelV1;
    #[cfg(feature = "protocol-cosmic_corner_radius_v1")]
    pub(super) use super::cosmic_corner_radius_v1::cosmic_corner_radius_toplevel_v1::CosmicCornerRadiusToplevelV1Error;
    #[cfg(feature = "protocol-cosmic_image_source_unstable_v1")]
    pub(super) use super::cosmic_image_source_unstable_v1::zcosmic_workspace_image_capture_source_manager_v1::ZcosmicWorkspaceImageCaptureSourceManagerV1;
    #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
    pub(super) use super::cosmic_output_management_unstable_v1::zcosmic_output_configuration_head_v1::ZcosmicOutputConfigurationHeadV1;
    #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
    pub(super) use super::cosmic_output_management_unstable_v1::zcosmic_output_configuration_v1::ZcosmicOutputConfigurationV1;
    #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
    pub(super) use super::cosmic_output_management_unstable_v1::zcosmic_output_configuration_v1::ZcosmicOutputConfigurationV1Error;
    #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
    pub(super) use super::cosmic_output_management_unstable_v1::zcosmic_output_head_v1::ZcosmicOutputHeadV1;
    #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
    pub(super) use super::cosmic_output_management_unstable_v1::zcosmic_output_head_v1::ZcosmicOutputHeadV1AdaptiveSyncAvailability;
    #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
    pub(super) use super::cosmic_output_management_unstable_v1::zcosmic_output_head_v1::ZcosmicOutputHeadV1AdaptiveSyncStateExt;
    #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
    pub(super) use super::cosmic_output_management_unstable_v1::zcosmic_output_manager_v1::ZcosmicOutputManagerV1;
    #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
    pub(super) use super::cosmic_output_management_unstable_v1::zcosmic_output_manager_v1::ZcosmicOutputManagerV1Error;
    #[cfg(feature = "protocol-cosmic_overlap_notify_unstable_v1")]
    pub(super) use super::cosmic_overlap_notify_unstable_v1::zcosmic_overlap_notification_v1::ZcosmicOverlapNotificationV1;
    #[cfg(feature = "protocol-cosmic_overlap_notify_unstable_v1")]
    pub(super) use super::cosmic_overlap_notify_unstable_v1::zcosmic_overlap_notify_v1::ZcosmicOverlapNotifyV1;
    #[cfg(feature = "protocol-cosmic_workspace_unstable_v2")]
    pub(super) use super::cosmic_workspace_unstable_v2::zcosmic_workspace_handle_v2::ZcosmicWorkspaceHandleV2;
    #[cfg(feature = "protocol-cosmic_workspace_unstable_v2")]
    pub(super) use super::cosmic_workspace_unstable_v2::zcosmic_workspace_handle_v2::ZcosmicWorkspaceHandleV2WorkspaceCapabilities;
    #[cfg(feature = "protocol-cosmic_workspace_unstable_v2")]
    pub(super) use super::cosmic_workspace_unstable_v2::zcosmic_workspace_handle_v2::ZcosmicWorkspaceHandleV2TilingState;
    #[cfg(feature = "protocol-cosmic_workspace_unstable_v2")]
    pub(super) use super::cosmic_workspace_unstable_v2::zcosmic_workspace_handle_v2::ZcosmicWorkspaceHandleV2State;
    #[cfg(feature = "protocol-cosmic_workspace_unstable_v2")]
    pub(super) use super::cosmic_workspace_unstable_v2::zcosmic_workspace_manager_v2::ZcosmicWorkspaceManagerV2;
    #[cfg(feature = "protocol-cosmic_workspace_unstable_v2")]
    pub(super) use super::cosmic_workspace_unstable_v2::zcosmic_workspace_manager_v2::ZcosmicWorkspaceManagerV2Error;

    use crate::protocol_helpers::prelude::*;

    pub(super) fn create_object_for_interface(state: &Rc<State>, interface: &str, version: u32) -> Result<Rc<dyn Object>, ObjectError> {
        ObjectInterface::from_str(interface)
            .ok_or(ObjectError(ObjectErrorKind::UnsupportedInterface(interface.to_string())))
            .and_then(|i| i.create_object(state, version))
    }

    impl ObjectInterface {
        #[expect(clippy::should_implement_trait)]
        pub fn from_str(interface: &str) -> Option<ObjectInterface> {
            static INTERFACES: phf::Map<&'static str, Option<ObjectInterface>> = phf::phf_map! {
                "hyprland_ctm_control_manager_v1" => {
                    #[cfg(feature = "protocol-hyprland_ctm_control_v1")] { Some(ObjectInterface::HyprlandCtmControlManagerV1) }
                    #[cfg(not(feature = "protocol-hyprland_ctm_control_v1"))] { None }
                },
                "hyprland_focus_grab_manager_v1" => {
                    #[cfg(feature = "protocol-hyprland_focus_grab_v1")] { Some(ObjectInterface::HyprlandFocusGrabManagerV1) }
                    #[cfg(not(feature = "protocol-hyprland_focus_grab_v1"))] { None }
                },
                "hyprland_focus_grab_v1" => {
                    #[cfg(feature = "protocol-hyprland_focus_grab_v1")] { Some(ObjectInterface::HyprlandFocusGrabV1) }
                    #[cfg(not(feature = "protocol-hyprland_focus_grab_v1"))] { None }
                },
                "hyprland_global_shortcut_v1" => {
                    #[cfg(feature = "protocol-hyprland_global_shortcuts_v1")] { Some(ObjectInterface::HyprlandGlobalShortcutV1) }
                    #[cfg(not(feature = "protocol-hyprland_global_shortcuts_v1"))] { None }
                },
                "hyprland_global_shortcuts_manager_v1" => {
                    #[cfg(feature = "protocol-hyprland_global_shortcuts_v1")] { Some(ObjectInterface::HyprlandGlobalShortcutsManagerV1) }
                    #[cfg(not(feature = "protocol-hyprland_global_shortcuts_v1"))] { None }
                },
                "hyprland_input_capture_manager_v1" => {
                    #[cfg(feature = "protocol-hyprland_input_capture_v1")] { Some(ObjectInterface::HyprlandInputCaptureManagerV1) }
                    #[cfg(not(feature = "protocol-hyprland_input_capture_v1"))] { None }
                },
                "hyprland_input_capture_v1" => {
                    #[cfg(feature = "protocol-hyprland_input_capture_v1")] { Some(ObjectInterface::HyprlandInputCaptureV1) }
                    #[cfg(not(feature = "protocol-hyprland_input_capture_v1"))] { None }
                },
                "hyprland_lock_notification_v1" => {
                    #[cfg(feature = "protocol-hyprland_lock_notify_v1")] { Some(ObjectInterface::HyprlandLockNotificationV1) }
                    #[cfg(not(feature = "protocol-hyprland_lock_notify_v1"))] { None }
                },
                "hyprland_lock_notifier_v1" => {
                    #[cfg(feature = "protocol-hyprland_lock_notify_v1")] { Some(ObjectInterface::HyprlandLockNotifierV1) }
                    #[cfg(not(feature = "protocol-hyprland_lock_notify_v1"))] { None }
                },
                "hyprland_surface_manager_v1" => {
                    #[cfg(feature = "protocol-hyprland_surface_v1")] { Some(ObjectInterface::HyprlandSurfaceManagerV1) }
                    #[cfg(not(feature = "protocol-hyprland_surface_v1"))] { None }
                },
                "hyprland_surface_v1" => {
                    #[cfg(feature = "protocol-hyprland_surface_v1")] { Some(ObjectInterface::HyprlandSurfaceV1) }
                    #[cfg(not(feature = "protocol-hyprland_surface_v1"))] { None }
                },
                "hyprland_toplevel_export_frame_v1" => {
                    #[cfg(feature = "protocol-hyprland_toplevel_export_v1")] { Some(ObjectInterface::HyprlandToplevelExportFrameV1) }
                    #[cfg(not(feature = "protocol-hyprland_toplevel_export_v1"))] { None }
                },
                "hyprland_toplevel_export_manager_v1" => {
                    #[cfg(feature = "protocol-hyprland_toplevel_export_v1")] { Some(ObjectInterface::HyprlandToplevelExportManagerV1) }
                    #[cfg(not(feature = "protocol-hyprland_toplevel_export_v1"))] { None }
                },
                "hyprland_toplevel_mapping_manager_v1" => {
                    #[cfg(feature = "protocol-hyprland_toplevel_mapping_v1")] { Some(ObjectInterface::HyprlandToplevelMappingManagerV1) }
                    #[cfg(not(feature = "protocol-hyprland_toplevel_mapping_v1"))] { None }
                },
                "hyprland_toplevel_window_mapping_handle_v1" => {
                    #[cfg(feature = "protocol-hyprland_toplevel_mapping_v1")] { Some(ObjectInterface::HyprlandToplevelWindowMappingHandleV1) }
                    #[cfg(not(feature = "protocol-hyprland_toplevel_mapping_v1"))] { None }
                },
                "jay_popup_ext_manager_v1" => {
                    #[cfg(feature = "protocol-jay_popup_ext_v1")] { Some(ObjectInterface::JayPopupExtManagerV1) }
                    #[cfg(not(feature = "protocol-jay_popup_ext_v1"))] { None }
                },
                "jay_popup_ext_v1" => {
                    #[cfg(feature = "protocol-jay_popup_ext_v1")] { Some(ObjectInterface::JayPopupExtV1) }
                    #[cfg(not(feature = "protocol-jay_popup_ext_v1"))] { None }
                },
                "jay_tray_item_v1" => {
                    #[cfg(feature = "protocol-jay_tray_v1")] { Some(ObjectInterface::JayTrayItemV1) }
                    #[cfg(not(feature = "protocol-jay_tray_v1"))] { None }
                },
                "jay_tray_v1" => {
                    #[cfg(feature = "protocol-jay_tray_v1")] { Some(ObjectInterface::JayTrayV1) }
                    #[cfg(not(feature = "protocol-jay_tray_v1"))] { None }
                },
                "wl_drm" => {
                    #[cfg(feature = "protocol-drm")] { Some(ObjectInterface::WlDrm) }
                    #[cfg(not(feature = "protocol-drm"))] { None }
                },
                "zwp_input_method_keyboard_grab_v2" => {
                    #[cfg(feature = "protocol-input_method_unstable_v2")] { Some(ObjectInterface::ZwpInputMethodKeyboardGrabV2) }
                    #[cfg(not(feature = "protocol-input_method_unstable_v2"))] { None }
                },
                "zwp_input_method_manager_v2" => {
                    #[cfg(feature = "protocol-input_method_unstable_v2")] { Some(ObjectInterface::ZwpInputMethodManagerV2) }
                    #[cfg(not(feature = "protocol-input_method_unstable_v2"))] { None }
                },
                "zwp_input_method_v2" => {
                    #[cfg(feature = "protocol-input_method_unstable_v2")] { Some(ObjectInterface::ZwpInputMethodV2) }
                    #[cfg(not(feature = "protocol-input_method_unstable_v2"))] { None }
                },
                "zwp_input_popup_surface_v2" => {
                    #[cfg(feature = "protocol-input_method_unstable_v2")] { Some(ObjectInterface::ZwpInputPopupSurfaceV2) }
                    #[cfg(not(feature = "protocol-input_method_unstable_v2"))] { None }
                },
                "org_kde_kwin_server_decoration" => {
                    #[cfg(feature = "protocol-org_kde_kwin_server_decoration_v1")] { Some(ObjectInterface::OrgKdeKwinServerDecoration) }
                    #[cfg(not(feature = "protocol-org_kde_kwin_server_decoration_v1"))] { None }
                },
                "org_kde_kwin_server_decoration_manager" => {
                    #[cfg(feature = "protocol-org_kde_kwin_server_decoration_v1")] { Some(ObjectInterface::OrgKdeKwinServerDecorationManager) }
                    #[cfg(not(feature = "protocol-org_kde_kwin_server_decoration_v1"))] { None }
                },
                "zwp_virtual_keyboard_manager_v1" => {
                    #[cfg(feature = "protocol-virtual_keyboard_unstable_v1")] { Some(ObjectInterface::ZwpVirtualKeyboardManagerV1) }
                    #[cfg(not(feature = "protocol-virtual_keyboard_unstable_v1"))] { None }
                },
                "zwp_virtual_keyboard_v1" => {
                    #[cfg(feature = "protocol-virtual_keyboard_unstable_v1")] { Some(ObjectInterface::ZwpVirtualKeyboardV1) }
                    #[cfg(not(feature = "protocol-virtual_keyboard_unstable_v1"))] { None }
                },
                "wl_buffer" => Some(ObjectInterface::WlBuffer),
                "wl_callback" => Some(ObjectInterface::WlCallback),
                "wl_compositor" => Some(ObjectInterface::WlCompositor),
                "wl_data_device" => Some(ObjectInterface::WlDataDevice),
                "wl_data_device_manager" => Some(ObjectInterface::WlDataDeviceManager),
                "wl_data_offer" => Some(ObjectInterface::WlDataOffer),
                "wl_data_source" => Some(ObjectInterface::WlDataSource),
                "wl_display" => Some(ObjectInterface::WlDisplay),
                "wl_fixes" => Some(ObjectInterface::WlFixes),
                "wl_keyboard" => Some(ObjectInterface::WlKeyboard),
                "wl_output" => Some(ObjectInterface::WlOutput),
                "wl_pointer" => Some(ObjectInterface::WlPointer),
                "wl_region" => Some(ObjectInterface::WlRegion),
                "wl_registry" => Some(ObjectInterface::WlRegistry),
                "wl_seat" => Some(ObjectInterface::WlSeat),
                "wl_shell" => Some(ObjectInterface::WlShell),
                "wl_shell_surface" => Some(ObjectInterface::WlShellSurface),
                "wl_shm" => Some(ObjectInterface::WlShm),
                "wl_shm_pool" => Some(ObjectInterface::WlShmPool),
                "wl_subcompositor" => Some(ObjectInterface::WlSubcompositor),
                "wl_subsurface" => Some(ObjectInterface::WlSubsurface),
                "wl_surface" => Some(ObjectInterface::WlSurface),
                "wl_touch" => Some(ObjectInterface::WlTouch),
                "wp_alpha_modifier_surface_v1" => {
                    #[cfg(feature = "protocol-alpha_modifier_v1")] { Some(ObjectInterface::WpAlphaModifierSurfaceV1) }
                    #[cfg(not(feature = "protocol-alpha_modifier_v1"))] { None }
                },
                "wp_alpha_modifier_v1" => {
                    #[cfg(feature = "protocol-alpha_modifier_v1")] { Some(ObjectInterface::WpAlphaModifierV1) }
                    #[cfg(not(feature = "protocol-alpha_modifier_v1"))] { None }
                },
                "wp_color_management_output_v1" => {
                    #[cfg(feature = "protocol-color_management_v1")] { Some(ObjectInterface::WpColorManagementOutputV1) }
                    #[cfg(not(feature = "protocol-color_management_v1"))] { None }
                },
                "wp_color_management_surface_feedback_v1" => {
                    #[cfg(feature = "protocol-color_management_v1")] { Some(ObjectInterface::WpColorManagementSurfaceFeedbackV1) }
                    #[cfg(not(feature = "protocol-color_management_v1"))] { None }
                },
                "wp_color_management_surface_v1" => {
                    #[cfg(feature = "protocol-color_management_v1")] { Some(ObjectInterface::WpColorManagementSurfaceV1) }
                    #[cfg(not(feature = "protocol-color_management_v1"))] { None }
                },
                "wp_color_manager_v1" => {
                    #[cfg(feature = "protocol-color_management_v1")] { Some(ObjectInterface::WpColorManagerV1) }
                    #[cfg(not(feature = "protocol-color_management_v1"))] { None }
                },
                "wp_image_description_creator_icc_v1" => {
                    #[cfg(feature = "protocol-color_management_v1")] { Some(ObjectInterface::WpImageDescriptionCreatorIccV1) }
                    #[cfg(not(feature = "protocol-color_management_v1"))] { None }
                },
                "wp_image_description_creator_params_v1" => {
                    #[cfg(feature = "protocol-color_management_v1")] { Some(ObjectInterface::WpImageDescriptionCreatorParamsV1) }
                    #[cfg(not(feature = "protocol-color_management_v1"))] { None }
                },
                "wp_image_description_info_v1" => {
                    #[cfg(feature = "protocol-color_management_v1")] { Some(ObjectInterface::WpImageDescriptionInfoV1) }
                    #[cfg(not(feature = "protocol-color_management_v1"))] { None }
                },
                "wp_image_description_reference_v1" => {
                    #[cfg(feature = "protocol-color_management_v1")] { Some(ObjectInterface::WpImageDescriptionReferenceV1) }
                    #[cfg(not(feature = "protocol-color_management_v1"))] { None }
                },
                "wp_image_description_v1" => {
                    #[cfg(feature = "protocol-color_management_v1")] { Some(ObjectInterface::WpImageDescriptionV1) }
                    #[cfg(not(feature = "protocol-color_management_v1"))] { None }
                },
                "wp_color_representation_manager_v1" => {
                    #[cfg(feature = "protocol-color_representation_v1")] { Some(ObjectInterface::WpColorRepresentationManagerV1) }
                    #[cfg(not(feature = "protocol-color_representation_v1"))] { None }
                },
                "wp_color_representation_surface_v1" => {
                    #[cfg(feature = "protocol-color_representation_v1")] { Some(ObjectInterface::WpColorRepresentationSurfaceV1) }
                    #[cfg(not(feature = "protocol-color_representation_v1"))] { None }
                },
                "wp_commit_timer_v1" => {
                    #[cfg(feature = "protocol-commit_timing_v1")] { Some(ObjectInterface::WpCommitTimerV1) }
                    #[cfg(not(feature = "protocol-commit_timing_v1"))] { None }
                },
                "wp_commit_timing_manager_v1" => {
                    #[cfg(feature = "protocol-commit_timing_v1")] { Some(ObjectInterface::WpCommitTimingManagerV1) }
                    #[cfg(not(feature = "protocol-commit_timing_v1"))] { None }
                },
                "wp_content_type_manager_v1" => {
                    #[cfg(feature = "protocol-content_type_v1")] { Some(ObjectInterface::WpContentTypeManagerV1) }
                    #[cfg(not(feature = "protocol-content_type_v1"))] { None }
                },
                "wp_content_type_v1" => {
                    #[cfg(feature = "protocol-content_type_v1")] { Some(ObjectInterface::WpContentTypeV1) }
                    #[cfg(not(feature = "protocol-content_type_v1"))] { None }
                },
                "wp_cursor_shape_device_v1" => {
                    #[cfg(feature = "protocol-cursor_shape_v1")] { Some(ObjectInterface::WpCursorShapeDeviceV1) }
                    #[cfg(not(feature = "protocol-cursor_shape_v1"))] { None }
                },
                "wp_cursor_shape_manager_v1" => {
                    #[cfg(feature = "protocol-cursor_shape_v1")] { Some(ObjectInterface::WpCursorShapeManagerV1) }
                    #[cfg(not(feature = "protocol-cursor_shape_v1"))] { None }
                },
                "wp_drm_lease_connector_v1" => {
                    #[cfg(feature = "protocol-drm_lease_v1")] { Some(ObjectInterface::WpDrmLeaseConnectorV1) }
                    #[cfg(not(feature = "protocol-drm_lease_v1"))] { None }
                },
                "wp_drm_lease_device_v1" => {
                    #[cfg(feature = "protocol-drm_lease_v1")] { Some(ObjectInterface::WpDrmLeaseDeviceV1) }
                    #[cfg(not(feature = "protocol-drm_lease_v1"))] { None }
                },
                "wp_drm_lease_request_v1" => {
                    #[cfg(feature = "protocol-drm_lease_v1")] { Some(ObjectInterface::WpDrmLeaseRequestV1) }
                    #[cfg(not(feature = "protocol-drm_lease_v1"))] { None }
                },
                "wp_drm_lease_v1" => {
                    #[cfg(feature = "protocol-drm_lease_v1")] { Some(ObjectInterface::WpDrmLeaseV1) }
                    #[cfg(not(feature = "protocol-drm_lease_v1"))] { None }
                },
                "ext_background_effect_manager_v1" => {
                    #[cfg(feature = "protocol-ext_background_effect_v1")] { Some(ObjectInterface::ExtBackgroundEffectManagerV1) }
                    #[cfg(not(feature = "protocol-ext_background_effect_v1"))] { None }
                },
                "ext_background_effect_surface_v1" => {
                    #[cfg(feature = "protocol-ext_background_effect_v1")] { Some(ObjectInterface::ExtBackgroundEffectSurfaceV1) }
                    #[cfg(not(feature = "protocol-ext_background_effect_v1"))] { None }
                },
                "ext_data_control_device_v1" => {
                    #[cfg(feature = "protocol-ext_data_control_v1")] { Some(ObjectInterface::ExtDataControlDeviceV1) }
                    #[cfg(not(feature = "protocol-ext_data_control_v1"))] { None }
                },
                "ext_data_control_manager_v1" => {
                    #[cfg(feature = "protocol-ext_data_control_v1")] { Some(ObjectInterface::ExtDataControlManagerV1) }
                    #[cfg(not(feature = "protocol-ext_data_control_v1"))] { None }
                },
                "ext_data_control_offer_v1" => {
                    #[cfg(feature = "protocol-ext_data_control_v1")] { Some(ObjectInterface::ExtDataControlOfferV1) }
                    #[cfg(not(feature = "protocol-ext_data_control_v1"))] { None }
                },
                "ext_data_control_source_v1" => {
                    #[cfg(feature = "protocol-ext_data_control_v1")] { Some(ObjectInterface::ExtDataControlSourceV1) }
                    #[cfg(not(feature = "protocol-ext_data_control_v1"))] { None }
                },
                "ext_foreign_toplevel_handle_v1" => {
                    #[cfg(feature = "protocol-ext_foreign_toplevel_list_v1")] { Some(ObjectInterface::ExtForeignToplevelHandleV1) }
                    #[cfg(not(feature = "protocol-ext_foreign_toplevel_list_v1"))] { None }
                },
                "ext_foreign_toplevel_list_v1" => {
                    #[cfg(feature = "protocol-ext_foreign_toplevel_list_v1")] { Some(ObjectInterface::ExtForeignToplevelListV1) }
                    #[cfg(not(feature = "protocol-ext_foreign_toplevel_list_v1"))] { None }
                },
                "ext_idle_notification_v1" => {
                    #[cfg(feature = "protocol-ext_idle_notify_v1")] { Some(ObjectInterface::ExtIdleNotificationV1) }
                    #[cfg(not(feature = "protocol-ext_idle_notify_v1"))] { None }
                },
                "ext_idle_notifier_v1" => {
                    #[cfg(feature = "protocol-ext_idle_notify_v1")] { Some(ObjectInterface::ExtIdleNotifierV1) }
                    #[cfg(not(feature = "protocol-ext_idle_notify_v1"))] { None }
                },
                "ext_foreign_toplevel_image_capture_source_manager_v1" => {
                    #[cfg(feature = "protocol-ext_image_capture_source_v1")] { Some(ObjectInterface::ExtForeignToplevelImageCaptureSourceManagerV1) }
                    #[cfg(not(feature = "protocol-ext_image_capture_source_v1"))] { None }
                },
                "ext_image_capture_source_v1" => {
                    #[cfg(feature = "protocol-ext_image_capture_source_v1")] { Some(ObjectInterface::ExtImageCaptureSourceV1) }
                    #[cfg(not(feature = "protocol-ext_image_capture_source_v1"))] { None }
                },
                "ext_output_image_capture_source_manager_v1" => {
                    #[cfg(feature = "protocol-ext_image_capture_source_v1")] { Some(ObjectInterface::ExtOutputImageCaptureSourceManagerV1) }
                    #[cfg(not(feature = "protocol-ext_image_capture_source_v1"))] { None }
                },
                "ext_image_copy_capture_cursor_session_v1" => {
                    #[cfg(feature = "protocol-ext_image_copy_capture_v1")] { Some(ObjectInterface::ExtImageCopyCaptureCursorSessionV1) }
                    #[cfg(not(feature = "protocol-ext_image_copy_capture_v1"))] { None }
                },
                "ext_image_copy_capture_frame_v1" => {
                    #[cfg(feature = "protocol-ext_image_copy_capture_v1")] { Some(ObjectInterface::ExtImageCopyCaptureFrameV1) }
                    #[cfg(not(feature = "protocol-ext_image_copy_capture_v1"))] { None }
                },
                "ext_image_copy_capture_manager_v1" => {
                    #[cfg(feature = "protocol-ext_image_copy_capture_v1")] { Some(ObjectInterface::ExtImageCopyCaptureManagerV1) }
                    #[cfg(not(feature = "protocol-ext_image_copy_capture_v1"))] { None }
                },
                "ext_image_copy_capture_session_v1" => {
                    #[cfg(feature = "protocol-ext_image_copy_capture_v1")] { Some(ObjectInterface::ExtImageCopyCaptureSessionV1) }
                    #[cfg(not(feature = "protocol-ext_image_copy_capture_v1"))] { None }
                },
                "ext_session_lock_manager_v1" => {
                    #[cfg(feature = "protocol-ext_session_lock_v1")] { Some(ObjectInterface::ExtSessionLockManagerV1) }
                    #[cfg(not(feature = "protocol-ext_session_lock_v1"))] { None }
                },
                "ext_session_lock_surface_v1" => {
                    #[cfg(feature = "protocol-ext_session_lock_v1")] { Some(ObjectInterface::ExtSessionLockSurfaceV1) }
                    #[cfg(not(feature = "protocol-ext_session_lock_v1"))] { None }
                },
                "ext_session_lock_v1" => {
                    #[cfg(feature = "protocol-ext_session_lock_v1")] { Some(ObjectInterface::ExtSessionLockV1) }
                    #[cfg(not(feature = "protocol-ext_session_lock_v1"))] { None }
                },
                "ext_transient_seat_manager_v1" => {
                    #[cfg(feature = "protocol-ext_transient_seat_v1")] { Some(ObjectInterface::ExtTransientSeatManagerV1) }
                    #[cfg(not(feature = "protocol-ext_transient_seat_v1"))] { None }
                },
                "ext_transient_seat_v1" => {
                    #[cfg(feature = "protocol-ext_transient_seat_v1")] { Some(ObjectInterface::ExtTransientSeatV1) }
                    #[cfg(not(feature = "protocol-ext_transient_seat_v1"))] { None }
                },
                "ext_workspace_group_handle_v1" => {
                    #[cfg(feature = "protocol-ext_workspace_v1")] { Some(ObjectInterface::ExtWorkspaceGroupHandleV1) }
                    #[cfg(not(feature = "protocol-ext_workspace_v1"))] { None }
                },
                "ext_workspace_handle_v1" => {
                    #[cfg(feature = "protocol-ext_workspace_v1")] { Some(ObjectInterface::ExtWorkspaceHandleV1) }
                    #[cfg(not(feature = "protocol-ext_workspace_v1"))] { None }
                },
                "ext_workspace_manager_v1" => {
                    #[cfg(feature = "protocol-ext_workspace_v1")] { Some(ObjectInterface::ExtWorkspaceManagerV1) }
                    #[cfg(not(feature = "protocol-ext_workspace_v1"))] { None }
                },
                "wp_fifo_manager_v1" => {
                    #[cfg(feature = "protocol-fifo_v1")] { Some(ObjectInterface::WpFifoManagerV1) }
                    #[cfg(not(feature = "protocol-fifo_v1"))] { None }
                },
                "wp_fifo_v1" => {
                    #[cfg(feature = "protocol-fifo_v1")] { Some(ObjectInterface::WpFifoV1) }
                    #[cfg(not(feature = "protocol-fifo_v1"))] { None }
                },
                "wp_fractional_scale_manager_v1" => {
                    #[cfg(feature = "protocol-fractional_scale_v1")] { Some(ObjectInterface::WpFractionalScaleManagerV1) }
                    #[cfg(not(feature = "protocol-fractional_scale_v1"))] { None }
                },
                "wp_fractional_scale_v1" => {
                    #[cfg(feature = "protocol-fractional_scale_v1")] { Some(ObjectInterface::WpFractionalScaleV1) }
                    #[cfg(not(feature = "protocol-fractional_scale_v1"))] { None }
                },
                "zwp_fullscreen_shell_mode_feedback_v1" => {
                    #[cfg(feature = "protocol-fullscreen_shell_unstable_v1")] { Some(ObjectInterface::ZwpFullscreenShellModeFeedbackV1) }
                    #[cfg(not(feature = "protocol-fullscreen_shell_unstable_v1"))] { None }
                },
                "zwp_fullscreen_shell_v1" => {
                    #[cfg(feature = "protocol-fullscreen_shell_unstable_v1")] { Some(ObjectInterface::ZwpFullscreenShellV1) }
                    #[cfg(not(feature = "protocol-fullscreen_shell_unstable_v1"))] { None }
                },
                "zwp_idle_inhibit_manager_v1" => {
                    #[cfg(feature = "protocol-idle_inhibit_unstable_v1")] { Some(ObjectInterface::ZwpIdleInhibitManagerV1) }
                    #[cfg(not(feature = "protocol-idle_inhibit_unstable_v1"))] { None }
                },
                "zwp_idle_inhibitor_v1" => {
                    #[cfg(feature = "protocol-idle_inhibit_unstable_v1")] { Some(ObjectInterface::ZwpIdleInhibitorV1) }
                    #[cfg(not(feature = "protocol-idle_inhibit_unstable_v1"))] { None }
                },
                "zwp_input_method_context_v1" => {
                    #[cfg(feature = "protocol-input_method_unstable_v1")] { Some(ObjectInterface::ZwpInputMethodContextV1) }
                    #[cfg(not(feature = "protocol-input_method_unstable_v1"))] { None }
                },
                "zwp_input_method_v1" => {
                    #[cfg(feature = "protocol-input_method_unstable_v1")] { Some(ObjectInterface::ZwpInputMethodV1) }
                    #[cfg(not(feature = "protocol-input_method_unstable_v1"))] { None }
                },
                "zwp_input_panel_surface_v1" => {
                    #[cfg(feature = "protocol-input_method_unstable_v1")] { Some(ObjectInterface::ZwpInputPanelSurfaceV1) }
                    #[cfg(not(feature = "protocol-input_method_unstable_v1"))] { None }
                },
                "zwp_input_panel_v1" => {
                    #[cfg(feature = "protocol-input_method_unstable_v1")] { Some(ObjectInterface::ZwpInputPanelV1) }
                    #[cfg(not(feature = "protocol-input_method_unstable_v1"))] { None }
                },
                "zwp_input_timestamps_manager_v1" => {
                    #[cfg(feature = "protocol-input_timestamps_unstable_v1")] { Some(ObjectInterface::ZwpInputTimestampsManagerV1) }
                    #[cfg(not(feature = "protocol-input_timestamps_unstable_v1"))] { None }
                },
                "zwp_input_timestamps_v1" => {
                    #[cfg(feature = "protocol-input_timestamps_unstable_v1")] { Some(ObjectInterface::ZwpInputTimestampsV1) }
                    #[cfg(not(feature = "protocol-input_timestamps_unstable_v1"))] { None }
                },
                "zwp_keyboard_shortcuts_inhibit_manager_v1" => {
                    #[cfg(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1")] { Some(ObjectInterface::ZwpKeyboardShortcutsInhibitManagerV1) }
                    #[cfg(not(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1"))] { None }
                },
                "zwp_keyboard_shortcuts_inhibitor_v1" => {
                    #[cfg(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1")] { Some(ObjectInterface::ZwpKeyboardShortcutsInhibitorV1) }
                    #[cfg(not(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1"))] { None }
                },
                "zwp_linux_buffer_params_v1" => {
                    #[cfg(feature = "protocol-linux_dmabuf_v1")] { Some(ObjectInterface::ZwpLinuxBufferParamsV1) }
                    #[cfg(not(feature = "protocol-linux_dmabuf_v1"))] { None }
                },
                "zwp_linux_dmabuf_feedback_v1" => {
                    #[cfg(feature = "protocol-linux_dmabuf_v1")] { Some(ObjectInterface::ZwpLinuxDmabufFeedbackV1) }
                    #[cfg(not(feature = "protocol-linux_dmabuf_v1"))] { None }
                },
                "zwp_linux_dmabuf_v1" => {
                    #[cfg(feature = "protocol-linux_dmabuf_v1")] { Some(ObjectInterface::ZwpLinuxDmabufV1) }
                    #[cfg(not(feature = "protocol-linux_dmabuf_v1"))] { None }
                },
                "wp_linux_drm_syncobj_manager_v1" => {
                    #[cfg(feature = "protocol-linux_drm_syncobj_v1")] { Some(ObjectInterface::WpLinuxDrmSyncobjManagerV1) }
                    #[cfg(not(feature = "protocol-linux_drm_syncobj_v1"))] { None }
                },
                "wp_linux_drm_syncobj_surface_v1" => {
                    #[cfg(feature = "protocol-linux_drm_syncobj_v1")] { Some(ObjectInterface::WpLinuxDrmSyncobjSurfaceV1) }
                    #[cfg(not(feature = "protocol-linux_drm_syncobj_v1"))] { None }
                },
                "wp_linux_drm_syncobj_timeline_v1" => {
                    #[cfg(feature = "protocol-linux_drm_syncobj_v1")] { Some(ObjectInterface::WpLinuxDrmSyncobjTimelineV1) }
                    #[cfg(not(feature = "protocol-linux_drm_syncobj_v1"))] { None }
                },
                "zwp_confined_pointer_v1" => {
                    #[cfg(feature = "protocol-pointer_constraints_unstable_v1")] { Some(ObjectInterface::ZwpConfinedPointerV1) }
                    #[cfg(not(feature = "protocol-pointer_constraints_unstable_v1"))] { None }
                },
                "zwp_locked_pointer_v1" => {
                    #[cfg(feature = "protocol-pointer_constraints_unstable_v1")] { Some(ObjectInterface::ZwpLockedPointerV1) }
                    #[cfg(not(feature = "protocol-pointer_constraints_unstable_v1"))] { None }
                },
                "zwp_pointer_constraints_v1" => {
                    #[cfg(feature = "protocol-pointer_constraints_unstable_v1")] { Some(ObjectInterface::ZwpPointerConstraintsV1) }
                    #[cfg(not(feature = "protocol-pointer_constraints_unstable_v1"))] { None }
                },
                "zwp_pointer_gesture_hold_v1" => {
                    #[cfg(feature = "protocol-pointer_gestures_unstable_v1")] { Some(ObjectInterface::ZwpPointerGestureHoldV1) }
                    #[cfg(not(feature = "protocol-pointer_gestures_unstable_v1"))] { None }
                },
                "zwp_pointer_gesture_pinch_v1" => {
                    #[cfg(feature = "protocol-pointer_gestures_unstable_v1")] { Some(ObjectInterface::ZwpPointerGesturePinchV1) }
                    #[cfg(not(feature = "protocol-pointer_gestures_unstable_v1"))] { None }
                },
                "zwp_pointer_gesture_swipe_v1" => {
                    #[cfg(feature = "protocol-pointer_gestures_unstable_v1")] { Some(ObjectInterface::ZwpPointerGestureSwipeV1) }
                    #[cfg(not(feature = "protocol-pointer_gestures_unstable_v1"))] { None }
                },
                "zwp_pointer_gestures_v1" => {
                    #[cfg(feature = "protocol-pointer_gestures_unstable_v1")] { Some(ObjectInterface::ZwpPointerGesturesV1) }
                    #[cfg(not(feature = "protocol-pointer_gestures_unstable_v1"))] { None }
                },
                "wp_pointer_warp_v1" => {
                    #[cfg(feature = "protocol-pointer_warp_v1")] { Some(ObjectInterface::WpPointerWarpV1) }
                    #[cfg(not(feature = "protocol-pointer_warp_v1"))] { None }
                },
                "wp_presentation" => {
                    #[cfg(feature = "protocol-presentation_time")] { Some(ObjectInterface::WpPresentation) }
                    #[cfg(not(feature = "protocol-presentation_time"))] { None }
                },
                "wp_presentation_feedback" => {
                    #[cfg(feature = "protocol-presentation_time")] { Some(ObjectInterface::WpPresentationFeedback) }
                    #[cfg(not(feature = "protocol-presentation_time"))] { None }
                },
                "zwp_relative_pointer_manager_v1" => {
                    #[cfg(feature = "protocol-relative_pointer_unstable_v1")] { Some(ObjectInterface::ZwpRelativePointerManagerV1) }
                    #[cfg(not(feature = "protocol-relative_pointer_unstable_v1"))] { None }
                },
                "zwp_relative_pointer_v1" => {
                    #[cfg(feature = "protocol-relative_pointer_unstable_v1")] { Some(ObjectInterface::ZwpRelativePointerV1) }
                    #[cfg(not(feature = "protocol-relative_pointer_unstable_v1"))] { None }
                },
                "wp_security_context_manager_v1" => {
                    #[cfg(feature = "protocol-security_context_v1")] { Some(ObjectInterface::WpSecurityContextManagerV1) }
                    #[cfg(not(feature = "protocol-security_context_v1"))] { None }
                },
                "wp_security_context_v1" => {
                    #[cfg(feature = "protocol-security_context_v1")] { Some(ObjectInterface::WpSecurityContextV1) }
                    #[cfg(not(feature = "protocol-security_context_v1"))] { None }
                },
                "wp_single_pixel_buffer_manager_v1" => {
                    #[cfg(feature = "protocol-single_pixel_buffer_v1")] { Some(ObjectInterface::WpSinglePixelBufferManagerV1) }
                    #[cfg(not(feature = "protocol-single_pixel_buffer_v1"))] { None }
                },
                "zwp_tablet_manager_v2" => {
                    #[cfg(feature = "protocol-tablet_v2")] { Some(ObjectInterface::ZwpTabletManagerV2) }
                    #[cfg(not(feature = "protocol-tablet_v2"))] { None }
                },
                "zwp_tablet_pad_dial_v2" => {
                    #[cfg(feature = "protocol-tablet_v2")] { Some(ObjectInterface::ZwpTabletPadDialV2) }
                    #[cfg(not(feature = "protocol-tablet_v2"))] { None }
                },
                "zwp_tablet_pad_group_v2" => {
                    #[cfg(feature = "protocol-tablet_v2")] { Some(ObjectInterface::ZwpTabletPadGroupV2) }
                    #[cfg(not(feature = "protocol-tablet_v2"))] { None }
                },
                "zwp_tablet_pad_ring_v2" => {
                    #[cfg(feature = "protocol-tablet_v2")] { Some(ObjectInterface::ZwpTabletPadRingV2) }
                    #[cfg(not(feature = "protocol-tablet_v2"))] { None }
                },
                "zwp_tablet_pad_strip_v2" => {
                    #[cfg(feature = "protocol-tablet_v2")] { Some(ObjectInterface::ZwpTabletPadStripV2) }
                    #[cfg(not(feature = "protocol-tablet_v2"))] { None }
                },
                "zwp_tablet_pad_v2" => {
                    #[cfg(feature = "protocol-tablet_v2")] { Some(ObjectInterface::ZwpTabletPadV2) }
                    #[cfg(not(feature = "protocol-tablet_v2"))] { None }
                },
                "zwp_tablet_seat_v2" => {
                    #[cfg(feature = "protocol-tablet_v2")] { Some(ObjectInterface::ZwpTabletSeatV2) }
                    #[cfg(not(feature = "protocol-tablet_v2"))] { None }
                },
                "zwp_tablet_tool_v2" => {
                    #[cfg(feature = "protocol-tablet_v2")] { Some(ObjectInterface::ZwpTabletToolV2) }
                    #[cfg(not(feature = "protocol-tablet_v2"))] { None }
                },
                "zwp_tablet_v2" => {
                    #[cfg(feature = "protocol-tablet_v2")] { Some(ObjectInterface::ZwpTabletV2) }
                    #[cfg(not(feature = "protocol-tablet_v2"))] { None }
                },
                "wp_tearing_control_manager_v1" => {
                    #[cfg(feature = "protocol-tearing_control_v1")] { Some(ObjectInterface::WpTearingControlManagerV1) }
                    #[cfg(not(feature = "protocol-tearing_control_v1"))] { None }
                },
                "wp_tearing_control_v1" => {
                    #[cfg(feature = "protocol-tearing_control_v1")] { Some(ObjectInterface::WpTearingControlV1) }
                    #[cfg(not(feature = "protocol-tearing_control_v1"))] { None }
                },
                "zwp_text_input_manager_v1" => {
                    #[cfg(feature = "protocol-text_input_unstable_v1")] { Some(ObjectInterface::ZwpTextInputManagerV1) }
                    #[cfg(not(feature = "protocol-text_input_unstable_v1"))] { None }
                },
                "zwp_text_input_v1" => {
                    #[cfg(feature = "protocol-text_input_unstable_v1")] { Some(ObjectInterface::ZwpTextInputV1) }
                    #[cfg(not(feature = "protocol-text_input_unstable_v1"))] { None }
                },
                "zwp_text_input_manager_v3" => {
                    #[cfg(feature = "protocol-text_input_unstable_v3")] { Some(ObjectInterface::ZwpTextInputManagerV3) }
                    #[cfg(not(feature = "protocol-text_input_unstable_v3"))] { None }
                },
                "zwp_text_input_v3" => {
                    #[cfg(feature = "protocol-text_input_unstable_v3")] { Some(ObjectInterface::ZwpTextInputV3) }
                    #[cfg(not(feature = "protocol-text_input_unstable_v3"))] { None }
                },
                "wp_viewport" => {
                    #[cfg(feature = "protocol-viewporter")] { Some(ObjectInterface::WpViewport) }
                    #[cfg(not(feature = "protocol-viewporter"))] { None }
                },
                "wp_viewporter" => {
                    #[cfg(feature = "protocol-viewporter")] { Some(ObjectInterface::WpViewporter) }
                    #[cfg(not(feature = "protocol-viewporter"))] { None }
                },
                "zwp_primary_selection_device_manager_v1" => {
                    #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")] { Some(ObjectInterface::ZwpPrimarySelectionDeviceManagerV1) }
                    #[cfg(not(feature = "protocol-wp_primary_selection_unstable_v1"))] { None }
                },
                "zwp_primary_selection_device_v1" => {
                    #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")] { Some(ObjectInterface::ZwpPrimarySelectionDeviceV1) }
                    #[cfg(not(feature = "protocol-wp_primary_selection_unstable_v1"))] { None }
                },
                "zwp_primary_selection_offer_v1" => {
                    #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")] { Some(ObjectInterface::ZwpPrimarySelectionOfferV1) }
                    #[cfg(not(feature = "protocol-wp_primary_selection_unstable_v1"))] { None }
                },
                "zwp_primary_selection_source_v1" => {
                    #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")] { Some(ObjectInterface::ZwpPrimarySelectionSourceV1) }
                    #[cfg(not(feature = "protocol-wp_primary_selection_unstable_v1"))] { None }
                },
                "xdg_activation_token_v1" => {
                    #[cfg(feature = "protocol-xdg_activation_v1")] { Some(ObjectInterface::XdgActivationTokenV1) }
                    #[cfg(not(feature = "protocol-xdg_activation_v1"))] { None }
                },
                "xdg_activation_v1" => {
                    #[cfg(feature = "protocol-xdg_activation_v1")] { Some(ObjectInterface::XdgActivationV1) }
                    #[cfg(not(feature = "protocol-xdg_activation_v1"))] { None }
                },
                "zxdg_decoration_manager_v1" => {
                    #[cfg(feature = "protocol-xdg_decoration_unstable_v1")] { Some(ObjectInterface::ZxdgDecorationManagerV1) }
                    #[cfg(not(feature = "protocol-xdg_decoration_unstable_v1"))] { None }
                },
                "zxdg_toplevel_decoration_v1" => {
                    #[cfg(feature = "protocol-xdg_decoration_unstable_v1")] { Some(ObjectInterface::ZxdgToplevelDecorationV1) }
                    #[cfg(not(feature = "protocol-xdg_decoration_unstable_v1"))] { None }
                },
                "xdg_dialog_v1" => {
                    #[cfg(feature = "protocol-xdg_dialog_v1")] { Some(ObjectInterface::XdgDialogV1) }
                    #[cfg(not(feature = "protocol-xdg_dialog_v1"))] { None }
                },
                "xdg_wm_dialog_v1" => {
                    #[cfg(feature = "protocol-xdg_dialog_v1")] { Some(ObjectInterface::XdgWmDialogV1) }
                    #[cfg(not(feature = "protocol-xdg_dialog_v1"))] { None }
                },
                "zxdg_exported_v2" => {
                    #[cfg(feature = "protocol-xdg_foreign_unstable_v2")] { Some(ObjectInterface::ZxdgExportedV2) }
                    #[cfg(not(feature = "protocol-xdg_foreign_unstable_v2"))] { None }
                },
                "zxdg_exporter_v2" => {
                    #[cfg(feature = "protocol-xdg_foreign_unstable_v2")] { Some(ObjectInterface::ZxdgExporterV2) }
                    #[cfg(not(feature = "protocol-xdg_foreign_unstable_v2"))] { None }
                },
                "zxdg_imported_v2" => {
                    #[cfg(feature = "protocol-xdg_foreign_unstable_v2")] { Some(ObjectInterface::ZxdgImportedV2) }
                    #[cfg(not(feature = "protocol-xdg_foreign_unstable_v2"))] { None }
                },
                "zxdg_importer_v2" => {
                    #[cfg(feature = "protocol-xdg_foreign_unstable_v2")] { Some(ObjectInterface::ZxdgImporterV2) }
                    #[cfg(not(feature = "protocol-xdg_foreign_unstable_v2"))] { None }
                },
                "zxdg_output_manager_v1" => {
                    #[cfg(feature = "protocol-xdg_output_unstable_v1")] { Some(ObjectInterface::ZxdgOutputManagerV1) }
                    #[cfg(not(feature = "protocol-xdg_output_unstable_v1"))] { None }
                },
                "zxdg_output_v1" => {
                    #[cfg(feature = "protocol-xdg_output_unstable_v1")] { Some(ObjectInterface::ZxdgOutputV1) }
                    #[cfg(not(feature = "protocol-xdg_output_unstable_v1"))] { None }
                },
                "xdg_popup" => {
                    #[cfg(feature = "protocol-xdg_shell")] { Some(ObjectInterface::XdgPopup) }
                    #[cfg(not(feature = "protocol-xdg_shell"))] { None }
                },
                "xdg_positioner" => {
                    #[cfg(feature = "protocol-xdg_shell")] { Some(ObjectInterface::XdgPositioner) }
                    #[cfg(not(feature = "protocol-xdg_shell"))] { None }
                },
                "xdg_surface" => {
                    #[cfg(feature = "protocol-xdg_shell")] { Some(ObjectInterface::XdgSurface) }
                    #[cfg(not(feature = "protocol-xdg_shell"))] { None }
                },
                "xdg_toplevel" => {
                    #[cfg(feature = "protocol-xdg_shell")] { Some(ObjectInterface::XdgToplevel) }
                    #[cfg(not(feature = "protocol-xdg_shell"))] { None }
                },
                "xdg_wm_base" => {
                    #[cfg(feature = "protocol-xdg_shell")] { Some(ObjectInterface::XdgWmBase) }
                    #[cfg(not(feature = "protocol-xdg_shell"))] { None }
                },
                "xdg_system_bell_v1" => {
                    #[cfg(feature = "protocol-xdg_system_bell_v1")] { Some(ObjectInterface::XdgSystemBellV1) }
                    #[cfg(not(feature = "protocol-xdg_system_bell_v1"))] { None }
                },
                "xdg_toplevel_drag_manager_v1" => {
                    #[cfg(feature = "protocol-xdg_toplevel_drag_v1")] { Some(ObjectInterface::XdgToplevelDragManagerV1) }
                    #[cfg(not(feature = "protocol-xdg_toplevel_drag_v1"))] { None }
                },
                "xdg_toplevel_drag_v1" => {
                    #[cfg(feature = "protocol-xdg_toplevel_drag_v1")] { Some(ObjectInterface::XdgToplevelDragV1) }
                    #[cfg(not(feature = "protocol-xdg_toplevel_drag_v1"))] { None }
                },
                "xdg_toplevel_icon_manager_v1" => {
                    #[cfg(feature = "protocol-xdg_toplevel_icon_v1")] { Some(ObjectInterface::XdgToplevelIconManagerV1) }
                    #[cfg(not(feature = "protocol-xdg_toplevel_icon_v1"))] { None }
                },
                "xdg_toplevel_icon_v1" => {
                    #[cfg(feature = "protocol-xdg_toplevel_icon_v1")] { Some(ObjectInterface::XdgToplevelIconV1) }
                    #[cfg(not(feature = "protocol-xdg_toplevel_icon_v1"))] { None }
                },
                "xdg_toplevel_tag_manager_v1" => {
                    #[cfg(feature = "protocol-xdg_toplevel_tag_v1")] { Some(ObjectInterface::XdgToplevelTagManagerV1) }
                    #[cfg(not(feature = "protocol-xdg_toplevel_tag_v1"))] { None }
                },
                "zwp_xwayland_keyboard_grab_manager_v1" => {
                    #[cfg(feature = "protocol-xwayland_keyboard_grab_unstable_v1")] { Some(ObjectInterface::ZwpXwaylandKeyboardGrabManagerV1) }
                    #[cfg(not(feature = "protocol-xwayland_keyboard_grab_unstable_v1"))] { None }
                },
                "zwp_xwayland_keyboard_grab_v1" => {
                    #[cfg(feature = "protocol-xwayland_keyboard_grab_unstable_v1")] { Some(ObjectInterface::ZwpXwaylandKeyboardGrabV1) }
                    #[cfg(not(feature = "protocol-xwayland_keyboard_grab_unstable_v1"))] { None }
                },
                "xwayland_shell_v1" => {
                    #[cfg(feature = "protocol-xwayland_shell_v1")] { Some(ObjectInterface::XwaylandShellV1) }
                    #[cfg(not(feature = "protocol-xwayland_shell_v1"))] { None }
                },
                "xwayland_surface_v1" => {
                    #[cfg(feature = "protocol-xwayland_shell_v1")] { Some(ObjectInterface::XwaylandSurfaceV1) }
                    #[cfg(not(feature = "protocol-xwayland_shell_v1"))] { None }
                },
                "xx_zone_item_v1" => {
                    #[cfg(feature = "protocol-xx_zones_v1")] { Some(ObjectInterface::XxZoneItemV1) }
                    #[cfg(not(feature = "protocol-xx_zones_v1"))] { None }
                },
                "xx_zone_manager_v1" => {
                    #[cfg(feature = "protocol-xx_zones_v1")] { Some(ObjectInterface::XxZoneManagerV1) }
                    #[cfg(not(feature = "protocol-xx_zones_v1"))] { None }
                },
                "xx_zone_v1" => {
                    #[cfg(feature = "protocol-xx_zones_v1")] { Some(ObjectInterface::XxZoneV1) }
                    #[cfg(not(feature = "protocol-xx_zones_v1"))] { None }
                },
                "zwp_linux_buffer_release_v1" => {
                    #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")] { Some(ObjectInterface::ZwpLinuxBufferReleaseV1) }
                    #[cfg(not(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1"))] { None }
                },
                "zwp_linux_explicit_synchronization_v1" => {
                    #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")] { Some(ObjectInterface::ZwpLinuxExplicitSynchronizationV1) }
                    #[cfg(not(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1"))] { None }
                },
                "zwp_linux_surface_synchronization_v1" => {
                    #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")] { Some(ObjectInterface::ZwpLinuxSurfaceSynchronizationV1) }
                    #[cfg(not(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1"))] { None }
                },
                "zwlr_data_control_device_v1" => {
                    #[cfg(feature = "protocol-wlr_data_control_unstable_v1")] { Some(ObjectInterface::ZwlrDataControlDeviceV1) }
                    #[cfg(not(feature = "protocol-wlr_data_control_unstable_v1"))] { None }
                },
                "zwlr_data_control_manager_v1" => {
                    #[cfg(feature = "protocol-wlr_data_control_unstable_v1")] { Some(ObjectInterface::ZwlrDataControlManagerV1) }
                    #[cfg(not(feature = "protocol-wlr_data_control_unstable_v1"))] { None }
                },
                "zwlr_data_control_offer_v1" => {
                    #[cfg(feature = "protocol-wlr_data_control_unstable_v1")] { Some(ObjectInterface::ZwlrDataControlOfferV1) }
                    #[cfg(not(feature = "protocol-wlr_data_control_unstable_v1"))] { None }
                },
                "zwlr_data_control_source_v1" => {
                    #[cfg(feature = "protocol-wlr_data_control_unstable_v1")] { Some(ObjectInterface::ZwlrDataControlSourceV1) }
                    #[cfg(not(feature = "protocol-wlr_data_control_unstable_v1"))] { None }
                },
                "zwlr_export_dmabuf_frame_v1" => {
                    #[cfg(feature = "protocol-wlr_export_dmabuf_unstable_v1")] { Some(ObjectInterface::ZwlrExportDmabufFrameV1) }
                    #[cfg(not(feature = "protocol-wlr_export_dmabuf_unstable_v1"))] { None }
                },
                "zwlr_export_dmabuf_manager_v1" => {
                    #[cfg(feature = "protocol-wlr_export_dmabuf_unstable_v1")] { Some(ObjectInterface::ZwlrExportDmabufManagerV1) }
                    #[cfg(not(feature = "protocol-wlr_export_dmabuf_unstable_v1"))] { None }
                },
                "zwlr_foreign_toplevel_handle_v1" => {
                    #[cfg(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1")] { Some(ObjectInterface::ZwlrForeignToplevelHandleV1) }
                    #[cfg(not(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1"))] { None }
                },
                "zwlr_foreign_toplevel_manager_v1" => {
                    #[cfg(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1")] { Some(ObjectInterface::ZwlrForeignToplevelManagerV1) }
                    #[cfg(not(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1"))] { None }
                },
                "zwlr_gamma_control_manager_v1" => {
                    #[cfg(feature = "protocol-wlr_gamma_control_unstable_v1")] { Some(ObjectInterface::ZwlrGammaControlManagerV1) }
                    #[cfg(not(feature = "protocol-wlr_gamma_control_unstable_v1"))] { None }
                },
                "zwlr_gamma_control_v1" => {
                    #[cfg(feature = "protocol-wlr_gamma_control_unstable_v1")] { Some(ObjectInterface::ZwlrGammaControlV1) }
                    #[cfg(not(feature = "protocol-wlr_gamma_control_unstable_v1"))] { None }
                },
                "zwlr_input_inhibit_manager_v1" => {
                    #[cfg(feature = "protocol-wlr_input_inhibit_unstable_v1")] { Some(ObjectInterface::ZwlrInputInhibitManagerV1) }
                    #[cfg(not(feature = "protocol-wlr_input_inhibit_unstable_v1"))] { None }
                },
                "zwlr_input_inhibitor_v1" => {
                    #[cfg(feature = "protocol-wlr_input_inhibit_unstable_v1")] { Some(ObjectInterface::ZwlrInputInhibitorV1) }
                    #[cfg(not(feature = "protocol-wlr_input_inhibit_unstable_v1"))] { None }
                },
                "zwlr_layer_shell_v1" => {
                    #[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")] { Some(ObjectInterface::ZwlrLayerShellV1) }
                    #[cfg(not(feature = "protocol-wlr_layer_shell_unstable_v1"))] { None }
                },
                "zwlr_layer_surface_v1" => {
                    #[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")] { Some(ObjectInterface::ZwlrLayerSurfaceV1) }
                    #[cfg(not(feature = "protocol-wlr_layer_shell_unstable_v1"))] { None }
                },
                "zwlr_output_configuration_head_v1" => {
                    #[cfg(feature = "protocol-wlr_output_management_unstable_v1")] { Some(ObjectInterface::ZwlrOutputConfigurationHeadV1) }
                    #[cfg(not(feature = "protocol-wlr_output_management_unstable_v1"))] { None }
                },
                "zwlr_output_configuration_v1" => {
                    #[cfg(feature = "protocol-wlr_output_management_unstable_v1")] { Some(ObjectInterface::ZwlrOutputConfigurationV1) }
                    #[cfg(not(feature = "protocol-wlr_output_management_unstable_v1"))] { None }
                },
                "zwlr_output_head_v1" => {
                    #[cfg(feature = "protocol-wlr_output_management_unstable_v1")] { Some(ObjectInterface::ZwlrOutputHeadV1) }
                    #[cfg(not(feature = "protocol-wlr_output_management_unstable_v1"))] { None }
                },
                "zwlr_output_manager_v1" => {
                    #[cfg(feature = "protocol-wlr_output_management_unstable_v1")] { Some(ObjectInterface::ZwlrOutputManagerV1) }
                    #[cfg(not(feature = "protocol-wlr_output_management_unstable_v1"))] { None }
                },
                "zwlr_output_mode_v1" => {
                    #[cfg(feature = "protocol-wlr_output_management_unstable_v1")] { Some(ObjectInterface::ZwlrOutputModeV1) }
                    #[cfg(not(feature = "protocol-wlr_output_management_unstable_v1"))] { None }
                },
                "zwlr_output_power_manager_v1" => {
                    #[cfg(feature = "protocol-wlr_output_power_management_unstable_v1")] { Some(ObjectInterface::ZwlrOutputPowerManagerV1) }
                    #[cfg(not(feature = "protocol-wlr_output_power_management_unstable_v1"))] { None }
                },
                "zwlr_output_power_v1" => {
                    #[cfg(feature = "protocol-wlr_output_power_management_unstable_v1")] { Some(ObjectInterface::ZwlrOutputPowerV1) }
                    #[cfg(not(feature = "protocol-wlr_output_power_management_unstable_v1"))] { None }
                },
                "zwlr_screencopy_frame_v1" => {
                    #[cfg(feature = "protocol-wlr_screencopy_unstable_v1")] { Some(ObjectInterface::ZwlrScreencopyFrameV1) }
                    #[cfg(not(feature = "protocol-wlr_screencopy_unstable_v1"))] { None }
                },
                "zwlr_screencopy_manager_v1" => {
                    #[cfg(feature = "protocol-wlr_screencopy_unstable_v1")] { Some(ObjectInterface::ZwlrScreencopyManagerV1) }
                    #[cfg(not(feature = "protocol-wlr_screencopy_unstable_v1"))] { None }
                },
                "zwlr_virtual_pointer_manager_v1" => {
                    #[cfg(feature = "protocol-wlr_virtual_pointer_unstable_v1")] { Some(ObjectInterface::ZwlrVirtualPointerManagerV1) }
                    #[cfg(not(feature = "protocol-wlr_virtual_pointer_unstable_v1"))] { None }
                },
                "zwlr_virtual_pointer_v1" => {
                    #[cfg(feature = "protocol-wlr_virtual_pointer_unstable_v1")] { Some(ObjectInterface::ZwlrVirtualPointerV1) }
                    #[cfg(not(feature = "protocol-wlr_virtual_pointer_unstable_v1"))] { None }
                },
                "wlproxy_sync_v1" => {
                    #[cfg(feature = "protocol-wlproxy_sync_v1")] { Some(ObjectInterface::WlproxySyncV1) }
                    #[cfg(not(feature = "protocol-wlproxy_sync_v1"))] { None }
                },
                "wlproxy_test" => {
                    #[cfg(test)] { Some(ObjectInterface::WlproxyTest) }
                    #[cfg(not(test))] { None }
                },
                "wlproxy_test_array_echo" => {
                    #[cfg(test)] { Some(ObjectInterface::WlproxyTestArrayEcho) }
                    #[cfg(not(test))] { None }
                },
                "wlproxy_test_dummy" => {
                    #[cfg(test)] { Some(ObjectInterface::WlproxyTestDummy) }
                    #[cfg(not(test))] { None }
                },
                "wlproxy_test_fd_echo" => {
                    #[cfg(test)] { Some(ObjectInterface::WlproxyTestFdEcho) }
                    #[cfg(not(test))] { None }
                },
                "wlproxy_test_hops" => {
                    #[cfg(test)] { Some(ObjectInterface::WlproxyTestHops) }
                    #[cfg(not(test))] { None }
                },
                "wlproxy_test_non_forward" => {
                    #[cfg(test)] { Some(ObjectInterface::WlproxyTestNonForward) }
                    #[cfg(not(test))] { None }
                },
                "wlproxy_test_object_echo" => {
                    #[cfg(test)] { Some(ObjectInterface::WlproxyTestObjectEcho) }
                    #[cfg(not(test))] { None }
                },
                "wlproxy_test_server_sent" => {
                    #[cfg(test)] { Some(ObjectInterface::WlproxyTestServerSent) }
                    #[cfg(not(test))] { None }
                },
                "river_input_device_v1" => {
                    #[cfg(feature = "protocol-river_input_management_v1")] { Some(ObjectInterface::RiverInputDeviceV1) }
                    #[cfg(not(feature = "protocol-river_input_management_v1"))] { None }
                },
                "river_input_manager_v1" => {
                    #[cfg(feature = "protocol-river_input_management_v1")] { Some(ObjectInterface::RiverInputManagerV1) }
                    #[cfg(not(feature = "protocol-river_input_management_v1"))] { None }
                },
                "river_layer_shell_output_v1" => {
                    #[cfg(feature = "protocol-river_layer_shell_v1")] { Some(ObjectInterface::RiverLayerShellOutputV1) }
                    #[cfg(not(feature = "protocol-river_layer_shell_v1"))] { None }
                },
                "river_layer_shell_seat_v1" => {
                    #[cfg(feature = "protocol-river_layer_shell_v1")] { Some(ObjectInterface::RiverLayerShellSeatV1) }
                    #[cfg(not(feature = "protocol-river_layer_shell_v1"))] { None }
                },
                "river_layer_shell_v1" => {
                    #[cfg(feature = "protocol-river_layer_shell_v1")] { Some(ObjectInterface::RiverLayerShellV1) }
                    #[cfg(not(feature = "protocol-river_layer_shell_v1"))] { None }
                },
                "river_libinput_accel_config_v1" => {
                    #[cfg(feature = "protocol-river_libinput_config_v1")] { Some(ObjectInterface::RiverLibinputAccelConfigV1) }
                    #[cfg(not(feature = "protocol-river_libinput_config_v1"))] { None }
                },
                "river_libinput_config_v1" => {
                    #[cfg(feature = "protocol-river_libinput_config_v1")] { Some(ObjectInterface::RiverLibinputConfigV1) }
                    #[cfg(not(feature = "protocol-river_libinput_config_v1"))] { None }
                },
                "river_libinput_device_v1" => {
                    #[cfg(feature = "protocol-river_libinput_config_v1")] { Some(ObjectInterface::RiverLibinputDeviceV1) }
                    #[cfg(not(feature = "protocol-river_libinput_config_v1"))] { None }
                },
                "river_libinput_result_v1" => {
                    #[cfg(feature = "protocol-river_libinput_config_v1")] { Some(ObjectInterface::RiverLibinputResultV1) }
                    #[cfg(not(feature = "protocol-river_libinput_config_v1"))] { None }
                },
                "river_decoration_v1" => {
                    #[cfg(feature = "protocol-river_window_management_v1")] { Some(ObjectInterface::RiverDecorationV1) }
                    #[cfg(not(feature = "protocol-river_window_management_v1"))] { None }
                },
                "river_node_v1" => {
                    #[cfg(feature = "protocol-river_window_management_v1")] { Some(ObjectInterface::RiverNodeV1) }
                    #[cfg(not(feature = "protocol-river_window_management_v1"))] { None }
                },
                "river_output_v1" => {
                    #[cfg(feature = "protocol-river_window_management_v1")] { Some(ObjectInterface::RiverOutputV1) }
                    #[cfg(not(feature = "protocol-river_window_management_v1"))] { None }
                },
                "river_pointer_binding_v1" => {
                    #[cfg(feature = "protocol-river_window_management_v1")] { Some(ObjectInterface::RiverPointerBindingV1) }
                    #[cfg(not(feature = "protocol-river_window_management_v1"))] { None }
                },
                "river_seat_v1" => {
                    #[cfg(feature = "protocol-river_window_management_v1")] { Some(ObjectInterface::RiverSeatV1) }
                    #[cfg(not(feature = "protocol-river_window_management_v1"))] { None }
                },
                "river_shell_surface_v1" => {
                    #[cfg(feature = "protocol-river_window_management_v1")] { Some(ObjectInterface::RiverShellSurfaceV1) }
                    #[cfg(not(feature = "protocol-river_window_management_v1"))] { None }
                },
                "river_window_manager_v1" => {
                    #[cfg(feature = "protocol-river_window_management_v1")] { Some(ObjectInterface::RiverWindowManagerV1) }
                    #[cfg(not(feature = "protocol-river_window_management_v1"))] { None }
                },
                "river_window_v1" => {
                    #[cfg(feature = "protocol-river_window_management_v1")] { Some(ObjectInterface::RiverWindowV1) }
                    #[cfg(not(feature = "protocol-river_window_management_v1"))] { None }
                },
                "river_xkb_binding_v1" => {
                    #[cfg(feature = "protocol-river_xkb_bindings_v1")] { Some(ObjectInterface::RiverXkbBindingV1) }
                    #[cfg(not(feature = "protocol-river_xkb_bindings_v1"))] { None }
                },
                "river_xkb_bindings_seat_v1" => {
                    #[cfg(feature = "protocol-river_xkb_bindings_v1")] { Some(ObjectInterface::RiverXkbBindingsSeatV1) }
                    #[cfg(not(feature = "protocol-river_xkb_bindings_v1"))] { None }
                },
                "river_xkb_bindings_v1" => {
                    #[cfg(feature = "protocol-river_xkb_bindings_v1")] { Some(ObjectInterface::RiverXkbBindingsV1) }
                    #[cfg(not(feature = "protocol-river_xkb_bindings_v1"))] { None }
                },
                "river_xkb_config_v1" => {
                    #[cfg(feature = "protocol-river_xkb_config_v1")] { Some(ObjectInterface::RiverXkbConfigV1) }
                    #[cfg(not(feature = "protocol-river_xkb_config_v1"))] { None }
                },
                "river_xkb_keyboard_v1" => {
                    #[cfg(feature = "protocol-river_xkb_config_v1")] { Some(ObjectInterface::RiverXkbKeyboardV1) }
                    #[cfg(not(feature = "protocol-river_xkb_config_v1"))] { None }
                },
                "river_xkb_keymap_v1" => {
                    #[cfg(feature = "protocol-river_xkb_config_v1")] { Some(ObjectInterface::RiverXkbKeymapV1) }
                    #[cfg(not(feature = "protocol-river_xkb_config_v1"))] { None }
                },
                "ivi_application" => {
                    #[cfg(feature = "protocol-ivi_application")] { Some(ObjectInterface::IviApplication) }
                    #[cfg(not(feature = "protocol-ivi_application"))] { None }
                },
                "ivi_surface" => {
                    #[cfg(feature = "protocol-ivi_application")] { Some(ObjectInterface::IviSurface) }
                    #[cfg(not(feature = "protocol-ivi_application"))] { None }
                },
                "ivi_hmi_controller" => {
                    #[cfg(feature = "protocol-ivi_hmi_controller")] { Some(ObjectInterface::IviHmiController) }
                    #[cfg(not(feature = "protocol-ivi_hmi_controller"))] { None }
                },
                "weston_content_protection" => {
                    #[cfg(feature = "protocol-weston_content_protection")] { Some(ObjectInterface::WestonContentProtection) }
                    #[cfg(not(feature = "protocol-weston_content_protection"))] { None }
                },
                "weston_protected_surface" => {
                    #[cfg(feature = "protocol-weston_content_protection")] { Some(ObjectInterface::WestonProtectedSurface) }
                    #[cfg(not(feature = "protocol-weston_content_protection"))] { None }
                },
                "weston_debug_stream_v1" => {
                    #[cfg(feature = "protocol-weston_debug")] { Some(ObjectInterface::WestonDebugStreamV1) }
                    #[cfg(not(feature = "protocol-weston_debug"))] { None }
                },
                "weston_debug_v1" => {
                    #[cfg(feature = "protocol-weston_debug")] { Some(ObjectInterface::WestonDebugV1) }
                    #[cfg(not(feature = "protocol-weston_debug"))] { None }
                },
                "weston_desktop_shell" => {
                    #[cfg(feature = "protocol-weston_desktop")] { Some(ObjectInterface::WestonDesktopShell) }
                    #[cfg(not(feature = "protocol-weston_desktop"))] { None }
                },
                "weston_screensaver" => {
                    #[cfg(feature = "protocol-weston_desktop")] { Some(ObjectInterface::WestonScreensaver) }
                    #[cfg(not(feature = "protocol-weston_desktop"))] { None }
                },
                "weston_direct_display_v1" => {
                    #[cfg(feature = "protocol-weston_direct_display")] { Some(ObjectInterface::WestonDirectDisplayV1) }
                    #[cfg(not(feature = "protocol-weston_direct_display"))] { None }
                },
                "weston_capture_source_v1" => {
                    #[cfg(feature = "protocol-weston_output_capture")] { Some(ObjectInterface::WestonCaptureSourceV1) }
                    #[cfg(not(feature = "protocol-weston_output_capture"))] { None }
                },
                "weston_capture_v1" => {
                    #[cfg(feature = "protocol-weston_output_capture")] { Some(ObjectInterface::WestonCaptureV1) }
                    #[cfg(not(feature = "protocol-weston_output_capture"))] { None }
                },
                "weston_test" => {
                    #[cfg(feature = "protocol-weston_test")] { Some(ObjectInterface::WestonTest) }
                    #[cfg(not(feature = "protocol-weston_test"))] { None }
                },
                "weston_test_runner" => {
                    #[cfg(feature = "protocol-weston_test")] { Some(ObjectInterface::WestonTestRunner) }
                    #[cfg(not(feature = "protocol-weston_test"))] { None }
                },
                "weston_touch_calibration" => {
                    #[cfg(feature = "protocol-weston_touch_calibration")] { Some(ObjectInterface::WestonTouchCalibration) }
                    #[cfg(not(feature = "protocol-weston_touch_calibration"))] { None }
                },
                "weston_touch_calibrator" => {
                    #[cfg(feature = "protocol-weston_touch_calibration")] { Some(ObjectInterface::WestonTouchCalibrator) }
                    #[cfg(not(feature = "protocol-weston_touch_calibration"))] { None }
                },
                "weston_touch_coordinate" => {
                    #[cfg(feature = "protocol-weston_touch_calibration")] { Some(ObjectInterface::WestonTouchCoordinate) }
                    #[cfg(not(feature = "protocol-weston_touch_calibration"))] { None }
                },
                "cosmic_a11y_manager_v1" => {
                    #[cfg(feature = "protocol-cosmic_a11y_v1")] { Some(ObjectInterface::CosmicA11yManagerV1) }
                    #[cfg(not(feature = "protocol-cosmic_a11y_v1"))] { None }
                },
                "cosmic_corner_radius_manager_v1" => {
                    #[cfg(feature = "protocol-cosmic_corner_radius_v1")] { Some(ObjectInterface::CosmicCornerRadiusManagerV1) }
                    #[cfg(not(feature = "protocol-cosmic_corner_radius_v1"))] { None }
                },
                "cosmic_corner_radius_toplevel_v1" => {
                    #[cfg(feature = "protocol-cosmic_corner_radius_v1")] { Some(ObjectInterface::CosmicCornerRadiusToplevelV1) }
                    #[cfg(not(feature = "protocol-cosmic_corner_radius_v1"))] { None }
                },
                "zcosmic_workspace_image_capture_source_manager_v1" => {
                    #[cfg(feature = "protocol-cosmic_image_source_unstable_v1")] { Some(ObjectInterface::ZcosmicWorkspaceImageCaptureSourceManagerV1) }
                    #[cfg(not(feature = "protocol-cosmic_image_source_unstable_v1"))] { None }
                },
                "zcosmic_output_configuration_head_v1" => {
                    #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")] { Some(ObjectInterface::ZcosmicOutputConfigurationHeadV1) }
                    #[cfg(not(feature = "protocol-cosmic_output_management_unstable_v1"))] { None }
                },
                "zcosmic_output_configuration_v1" => {
                    #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")] { Some(ObjectInterface::ZcosmicOutputConfigurationV1) }
                    #[cfg(not(feature = "protocol-cosmic_output_management_unstable_v1"))] { None }
                },
                "zcosmic_output_head_v1" => {
                    #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")] { Some(ObjectInterface::ZcosmicOutputHeadV1) }
                    #[cfg(not(feature = "protocol-cosmic_output_management_unstable_v1"))] { None }
                },
                "zcosmic_output_manager_v1" => {
                    #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")] { Some(ObjectInterface::ZcosmicOutputManagerV1) }
                    #[cfg(not(feature = "protocol-cosmic_output_management_unstable_v1"))] { None }
                },
                "zcosmic_overlap_notification_v1" => {
                    #[cfg(feature = "protocol-cosmic_overlap_notify_unstable_v1")] { Some(ObjectInterface::ZcosmicOverlapNotificationV1) }
                    #[cfg(not(feature = "protocol-cosmic_overlap_notify_unstable_v1"))] { None }
                },
                "zcosmic_overlap_notify_v1" => {
                    #[cfg(feature = "protocol-cosmic_overlap_notify_unstable_v1")] { Some(ObjectInterface::ZcosmicOverlapNotifyV1) }
                    #[cfg(not(feature = "protocol-cosmic_overlap_notify_unstable_v1"))] { None }
                },
                "zcosmic_workspace_handle_v2" => {
                    #[cfg(feature = "protocol-cosmic_workspace_unstable_v2")] { Some(ObjectInterface::ZcosmicWorkspaceHandleV2) }
                    #[cfg(not(feature = "protocol-cosmic_workspace_unstable_v2"))] { None }
                },
                "zcosmic_workspace_manager_v2" => {
                    #[cfg(feature = "protocol-cosmic_workspace_unstable_v2")] { Some(ObjectInterface::ZcosmicWorkspaceManagerV2) }
                    #[cfg(not(feature = "protocol-cosmic_workspace_unstable_v2"))] { None }
                },
            };
            INTERFACES.get(interface).copied().flatten()
        }

        fn create_object(self, state: &Rc<State>, version: u32) -> Result<Rc<dyn Object>, ObjectError> {
            match self {
                #[cfg(feature = "protocol-hyprland_ctm_control_v1")]
                Self::HyprlandCtmControlManagerV1 => {
                    if version > HyprlandCtmControlManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(HyprlandCtmControlManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_focus_grab_v1")]
                Self::HyprlandFocusGrabManagerV1 => {
                    if version > HyprlandFocusGrabManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(HyprlandFocusGrabManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_focus_grab_v1")]
                Self::HyprlandFocusGrabV1 => {
                    if version > HyprlandFocusGrabV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(HyprlandFocusGrabV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_global_shortcuts_v1")]
                Self::HyprlandGlobalShortcutV1 => {
                    if version > HyprlandGlobalShortcutV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(HyprlandGlobalShortcutV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_global_shortcuts_v1")]
                Self::HyprlandGlobalShortcutsManagerV1 => {
                    if version > HyprlandGlobalShortcutsManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(HyprlandGlobalShortcutsManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_input_capture_v1")]
                Self::HyprlandInputCaptureManagerV1 => {
                    if version > HyprlandInputCaptureManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(HyprlandInputCaptureManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_input_capture_v1")]
                Self::HyprlandInputCaptureV1 => {
                    if version > HyprlandInputCaptureV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(HyprlandInputCaptureV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_lock_notify_v1")]
                Self::HyprlandLockNotificationV1 => {
                    if version > HyprlandLockNotificationV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(HyprlandLockNotificationV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_lock_notify_v1")]
                Self::HyprlandLockNotifierV1 => {
                    if version > HyprlandLockNotifierV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(HyprlandLockNotifierV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_surface_v1")]
                Self::HyprlandSurfaceManagerV1 => {
                    if version > HyprlandSurfaceManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(HyprlandSurfaceManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_surface_v1")]
                Self::HyprlandSurfaceV1 => {
                    if version > HyprlandSurfaceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(HyprlandSurfaceV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_toplevel_export_v1")]
                Self::HyprlandToplevelExportFrameV1 => {
                    if version > HyprlandToplevelExportFrameV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(HyprlandToplevelExportFrameV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_toplevel_export_v1")]
                Self::HyprlandToplevelExportManagerV1 => {
                    if version > HyprlandToplevelExportManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(HyprlandToplevelExportManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_toplevel_mapping_v1")]
                Self::HyprlandToplevelMappingManagerV1 => {
                    if version > HyprlandToplevelMappingManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(HyprlandToplevelMappingManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-hyprland_toplevel_mapping_v1")]
                Self::HyprlandToplevelWindowMappingHandleV1 => {
                    if version > HyprlandToplevelWindowMappingHandleV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(HyprlandToplevelWindowMappingHandleV1::new(state, version))
                }
                #[cfg(feature = "protocol-jay_popup_ext_v1")]
                Self::JayPopupExtManagerV1 => {
                    if version > JayPopupExtManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(JayPopupExtManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-jay_popup_ext_v1")]
                Self::JayPopupExtV1 => {
                    if version > JayPopupExtV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(JayPopupExtV1::new(state, version))
                }
                #[cfg(feature = "protocol-jay_tray_v1")]
                Self::JayTrayItemV1 => {
                    if version > JayTrayItemV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(JayTrayItemV1::new(state, version))
                }
                #[cfg(feature = "protocol-jay_tray_v1")]
                Self::JayTrayV1 => {
                    if version > JayTrayV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(JayTrayV1::new(state, version))
                }
                #[cfg(feature = "protocol-drm")]
                Self::WlDrm => {
                    if version > WlDrm::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlDrm::new(state, version))
                }
                #[cfg(feature = "protocol-input_method_unstable_v2")]
                Self::ZwpInputMethodKeyboardGrabV2 => {
                    if version > ZwpInputMethodKeyboardGrabV2::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpInputMethodKeyboardGrabV2::new(state, version))
                }
                #[cfg(feature = "protocol-input_method_unstable_v2")]
                Self::ZwpInputMethodManagerV2 => {
                    if version > ZwpInputMethodManagerV2::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpInputMethodManagerV2::new(state, version))
                }
                #[cfg(feature = "protocol-input_method_unstable_v2")]
                Self::ZwpInputMethodV2 => {
                    if version > ZwpInputMethodV2::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpInputMethodV2::new(state, version))
                }
                #[cfg(feature = "protocol-input_method_unstable_v2")]
                Self::ZwpInputPopupSurfaceV2 => {
                    if version > ZwpInputPopupSurfaceV2::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpInputPopupSurfaceV2::new(state, version))
                }
                #[cfg(feature = "protocol-org_kde_kwin_server_decoration_v1")]
                Self::OrgKdeKwinServerDecoration => {
                    if version > OrgKdeKwinServerDecoration::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(OrgKdeKwinServerDecoration::new(state, version))
                }
                #[cfg(feature = "protocol-org_kde_kwin_server_decoration_v1")]
                Self::OrgKdeKwinServerDecorationManager => {
                    if version > OrgKdeKwinServerDecorationManager::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(OrgKdeKwinServerDecorationManager::new(state, version))
                }
                #[cfg(feature = "protocol-virtual_keyboard_unstable_v1")]
                Self::ZwpVirtualKeyboardManagerV1 => {
                    if version > ZwpVirtualKeyboardManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpVirtualKeyboardManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-virtual_keyboard_unstable_v1")]
                Self::ZwpVirtualKeyboardV1 => {
                    if version > ZwpVirtualKeyboardV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpVirtualKeyboardV1::new(state, version))
                }
                Self::WlBuffer => {
                    if version > WlBuffer::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlBuffer::new(state, version))
                }
                Self::WlCallback => {
                    if version > WlCallback::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlCallback::new(state, version))
                }
                Self::WlCompositor => {
                    if version > WlCompositor::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlCompositor::new(state, version))
                }
                Self::WlDataDevice => {
                    if version > WlDataDevice::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlDataDevice::new(state, version))
                }
                Self::WlDataDeviceManager => {
                    if version > WlDataDeviceManager::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlDataDeviceManager::new(state, version))
                }
                Self::WlDataOffer => {
                    if version > WlDataOffer::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlDataOffer::new(state, version))
                }
                Self::WlDataSource => {
                    if version > WlDataSource::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlDataSource::new(state, version))
                }
                Self::WlDisplay => {
                    if version > WlDisplay::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlDisplay::new(state, version))
                }
                Self::WlFixes => {
                    if version > WlFixes::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlFixes::new(state, version))
                }
                Self::WlKeyboard => {
                    if version > WlKeyboard::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlKeyboard::new(state, version))
                }
                Self::WlOutput => {
                    if version > WlOutput::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlOutput::new(state, version))
                }
                Self::WlPointer => {
                    if version > WlPointer::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlPointer::new(state, version))
                }
                Self::WlRegion => {
                    if version > WlRegion::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlRegion::new(state, version))
                }
                Self::WlRegistry => {
                    if version > WlRegistry::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlRegistry::new(state, version))
                }
                Self::WlSeat => {
                    if version > WlSeat::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlSeat::new(state, version))
                }
                Self::WlShell => {
                    if version > WlShell::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlShell::new(state, version))
                }
                Self::WlShellSurface => {
                    if version > WlShellSurface::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlShellSurface::new(state, version))
                }
                Self::WlShm => {
                    if version > WlShm::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlShm::new(state, version))
                }
                Self::WlShmPool => {
                    if version > WlShmPool::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlShmPool::new(state, version))
                }
                Self::WlSubcompositor => {
                    if version > WlSubcompositor::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlSubcompositor::new(state, version))
                }
                Self::WlSubsurface => {
                    if version > WlSubsurface::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlSubsurface::new(state, version))
                }
                Self::WlSurface => {
                    if version > WlSurface::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlSurface::new(state, version))
                }
                Self::WlTouch => {
                    if version > WlTouch::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlTouch::new(state, version))
                }
                #[cfg(feature = "protocol-alpha_modifier_v1")]
                Self::WpAlphaModifierSurfaceV1 => {
                    if version > WpAlphaModifierSurfaceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpAlphaModifierSurfaceV1::new(state, version))
                }
                #[cfg(feature = "protocol-alpha_modifier_v1")]
                Self::WpAlphaModifierV1 => {
                    if version > WpAlphaModifierV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpAlphaModifierV1::new(state, version))
                }
                #[cfg(feature = "protocol-color_management_v1")]
                Self::WpColorManagementOutputV1 => {
                    if version > WpColorManagementOutputV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpColorManagementOutputV1::new(state, version))
                }
                #[cfg(feature = "protocol-color_management_v1")]
                Self::WpColorManagementSurfaceFeedbackV1 => {
                    if version > WpColorManagementSurfaceFeedbackV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpColorManagementSurfaceFeedbackV1::new(state, version))
                }
                #[cfg(feature = "protocol-color_management_v1")]
                Self::WpColorManagementSurfaceV1 => {
                    if version > WpColorManagementSurfaceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpColorManagementSurfaceV1::new(state, version))
                }
                #[cfg(feature = "protocol-color_management_v1")]
                Self::WpColorManagerV1 => {
                    if version > WpColorManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpColorManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-color_management_v1")]
                Self::WpImageDescriptionCreatorIccV1 => {
                    if version > WpImageDescriptionCreatorIccV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpImageDescriptionCreatorIccV1::new(state, version))
                }
                #[cfg(feature = "protocol-color_management_v1")]
                Self::WpImageDescriptionCreatorParamsV1 => {
                    if version > WpImageDescriptionCreatorParamsV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpImageDescriptionCreatorParamsV1::new(state, version))
                }
                #[cfg(feature = "protocol-color_management_v1")]
                Self::WpImageDescriptionInfoV1 => {
                    if version > WpImageDescriptionInfoV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpImageDescriptionInfoV1::new(state, version))
                }
                #[cfg(feature = "protocol-color_management_v1")]
                Self::WpImageDescriptionReferenceV1 => {
                    if version > WpImageDescriptionReferenceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpImageDescriptionReferenceV1::new(state, version))
                }
                #[cfg(feature = "protocol-color_management_v1")]
                Self::WpImageDescriptionV1 => {
                    if version > WpImageDescriptionV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpImageDescriptionV1::new(state, version))
                }
                #[cfg(feature = "protocol-color_representation_v1")]
                Self::WpColorRepresentationManagerV1 => {
                    if version > WpColorRepresentationManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpColorRepresentationManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-color_representation_v1")]
                Self::WpColorRepresentationSurfaceV1 => {
                    if version > WpColorRepresentationSurfaceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpColorRepresentationSurfaceV1::new(state, version))
                }
                #[cfg(feature = "protocol-commit_timing_v1")]
                Self::WpCommitTimerV1 => {
                    if version > WpCommitTimerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpCommitTimerV1::new(state, version))
                }
                #[cfg(feature = "protocol-commit_timing_v1")]
                Self::WpCommitTimingManagerV1 => {
                    if version > WpCommitTimingManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpCommitTimingManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-content_type_v1")]
                Self::WpContentTypeManagerV1 => {
                    if version > WpContentTypeManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpContentTypeManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-content_type_v1")]
                Self::WpContentTypeV1 => {
                    if version > WpContentTypeV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpContentTypeV1::new(state, version))
                }
                #[cfg(feature = "protocol-cursor_shape_v1")]
                Self::WpCursorShapeDeviceV1 => {
                    if version > WpCursorShapeDeviceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpCursorShapeDeviceV1::new(state, version))
                }
                #[cfg(feature = "protocol-cursor_shape_v1")]
                Self::WpCursorShapeManagerV1 => {
                    if version > WpCursorShapeManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpCursorShapeManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-drm_lease_v1")]
                Self::WpDrmLeaseConnectorV1 => {
                    if version > WpDrmLeaseConnectorV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpDrmLeaseConnectorV1::new(state, version))
                }
                #[cfg(feature = "protocol-drm_lease_v1")]
                Self::WpDrmLeaseDeviceV1 => {
                    if version > WpDrmLeaseDeviceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpDrmLeaseDeviceV1::new(state, version))
                }
                #[cfg(feature = "protocol-drm_lease_v1")]
                Self::WpDrmLeaseRequestV1 => {
                    if version > WpDrmLeaseRequestV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpDrmLeaseRequestV1::new(state, version))
                }
                #[cfg(feature = "protocol-drm_lease_v1")]
                Self::WpDrmLeaseV1 => {
                    if version > WpDrmLeaseV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpDrmLeaseV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_background_effect_v1")]
                Self::ExtBackgroundEffectManagerV1 => {
                    if version > ExtBackgroundEffectManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtBackgroundEffectManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_background_effect_v1")]
                Self::ExtBackgroundEffectSurfaceV1 => {
                    if version > ExtBackgroundEffectSurfaceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtBackgroundEffectSurfaceV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_data_control_v1")]
                Self::ExtDataControlDeviceV1 => {
                    if version > ExtDataControlDeviceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtDataControlDeviceV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_data_control_v1")]
                Self::ExtDataControlManagerV1 => {
                    if version > ExtDataControlManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtDataControlManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_data_control_v1")]
                Self::ExtDataControlOfferV1 => {
                    if version > ExtDataControlOfferV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtDataControlOfferV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_data_control_v1")]
                Self::ExtDataControlSourceV1 => {
                    if version > ExtDataControlSourceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtDataControlSourceV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_foreign_toplevel_list_v1")]
                Self::ExtForeignToplevelHandleV1 => {
                    if version > ExtForeignToplevelHandleV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtForeignToplevelHandleV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_foreign_toplevel_list_v1")]
                Self::ExtForeignToplevelListV1 => {
                    if version > ExtForeignToplevelListV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtForeignToplevelListV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_idle_notify_v1")]
                Self::ExtIdleNotificationV1 => {
                    if version > ExtIdleNotificationV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtIdleNotificationV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_idle_notify_v1")]
                Self::ExtIdleNotifierV1 => {
                    if version > ExtIdleNotifierV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtIdleNotifierV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_image_capture_source_v1")]
                Self::ExtForeignToplevelImageCaptureSourceManagerV1 => {
                    if version > ExtForeignToplevelImageCaptureSourceManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtForeignToplevelImageCaptureSourceManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_image_capture_source_v1")]
                Self::ExtImageCaptureSourceV1 => {
                    if version > ExtImageCaptureSourceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtImageCaptureSourceV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_image_capture_source_v1")]
                Self::ExtOutputImageCaptureSourceManagerV1 => {
                    if version > ExtOutputImageCaptureSourceManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtOutputImageCaptureSourceManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
                Self::ExtImageCopyCaptureCursorSessionV1 => {
                    if version > ExtImageCopyCaptureCursorSessionV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtImageCopyCaptureCursorSessionV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
                Self::ExtImageCopyCaptureFrameV1 => {
                    if version > ExtImageCopyCaptureFrameV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtImageCopyCaptureFrameV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
                Self::ExtImageCopyCaptureManagerV1 => {
                    if version > ExtImageCopyCaptureManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtImageCopyCaptureManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
                Self::ExtImageCopyCaptureSessionV1 => {
                    if version > ExtImageCopyCaptureSessionV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtImageCopyCaptureSessionV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_session_lock_v1")]
                Self::ExtSessionLockManagerV1 => {
                    if version > ExtSessionLockManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtSessionLockManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_session_lock_v1")]
                Self::ExtSessionLockSurfaceV1 => {
                    if version > ExtSessionLockSurfaceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtSessionLockSurfaceV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_session_lock_v1")]
                Self::ExtSessionLockV1 => {
                    if version > ExtSessionLockV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtSessionLockV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_transient_seat_v1")]
                Self::ExtTransientSeatManagerV1 => {
                    if version > ExtTransientSeatManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtTransientSeatManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_transient_seat_v1")]
                Self::ExtTransientSeatV1 => {
                    if version > ExtTransientSeatV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtTransientSeatV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_workspace_v1")]
                Self::ExtWorkspaceGroupHandleV1 => {
                    if version > ExtWorkspaceGroupHandleV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtWorkspaceGroupHandleV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_workspace_v1")]
                Self::ExtWorkspaceHandleV1 => {
                    if version > ExtWorkspaceHandleV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtWorkspaceHandleV1::new(state, version))
                }
                #[cfg(feature = "protocol-ext_workspace_v1")]
                Self::ExtWorkspaceManagerV1 => {
                    if version > ExtWorkspaceManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ExtWorkspaceManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-fifo_v1")]
                Self::WpFifoManagerV1 => {
                    if version > WpFifoManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpFifoManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-fifo_v1")]
                Self::WpFifoV1 => {
                    if version > WpFifoV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpFifoV1::new(state, version))
                }
                #[cfg(feature = "protocol-fractional_scale_v1")]
                Self::WpFractionalScaleManagerV1 => {
                    if version > WpFractionalScaleManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpFractionalScaleManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-fractional_scale_v1")]
                Self::WpFractionalScaleV1 => {
                    if version > WpFractionalScaleV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpFractionalScaleV1::new(state, version))
                }
                #[cfg(feature = "protocol-fullscreen_shell_unstable_v1")]
                Self::ZwpFullscreenShellModeFeedbackV1 => {
                    if version > ZwpFullscreenShellModeFeedbackV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpFullscreenShellModeFeedbackV1::new(state, version))
                }
                #[cfg(feature = "protocol-fullscreen_shell_unstable_v1")]
                Self::ZwpFullscreenShellV1 => {
                    if version > ZwpFullscreenShellV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpFullscreenShellV1::new(state, version))
                }
                #[cfg(feature = "protocol-idle_inhibit_unstable_v1")]
                Self::ZwpIdleInhibitManagerV1 => {
                    if version > ZwpIdleInhibitManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpIdleInhibitManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-idle_inhibit_unstable_v1")]
                Self::ZwpIdleInhibitorV1 => {
                    if version > ZwpIdleInhibitorV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpIdleInhibitorV1::new(state, version))
                }
                #[cfg(feature = "protocol-input_method_unstable_v1")]
                Self::ZwpInputMethodContextV1 => {
                    if version > ZwpInputMethodContextV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpInputMethodContextV1::new(state, version))
                }
                #[cfg(feature = "protocol-input_method_unstable_v1")]
                Self::ZwpInputMethodV1 => {
                    if version > ZwpInputMethodV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpInputMethodV1::new(state, version))
                }
                #[cfg(feature = "protocol-input_method_unstable_v1")]
                Self::ZwpInputPanelSurfaceV1 => {
                    if version > ZwpInputPanelSurfaceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpInputPanelSurfaceV1::new(state, version))
                }
                #[cfg(feature = "protocol-input_method_unstable_v1")]
                Self::ZwpInputPanelV1 => {
                    if version > ZwpInputPanelV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpInputPanelV1::new(state, version))
                }
                #[cfg(feature = "protocol-input_timestamps_unstable_v1")]
                Self::ZwpInputTimestampsManagerV1 => {
                    if version > ZwpInputTimestampsManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpInputTimestampsManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-input_timestamps_unstable_v1")]
                Self::ZwpInputTimestampsV1 => {
                    if version > ZwpInputTimestampsV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpInputTimestampsV1::new(state, version))
                }
                #[cfg(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1")]
                Self::ZwpKeyboardShortcutsInhibitManagerV1 => {
                    if version > ZwpKeyboardShortcutsInhibitManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpKeyboardShortcutsInhibitManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1")]
                Self::ZwpKeyboardShortcutsInhibitorV1 => {
                    if version > ZwpKeyboardShortcutsInhibitorV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpKeyboardShortcutsInhibitorV1::new(state, version))
                }
                #[cfg(feature = "protocol-linux_dmabuf_v1")]
                Self::ZwpLinuxBufferParamsV1 => {
                    if version > ZwpLinuxBufferParamsV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpLinuxBufferParamsV1::new(state, version))
                }
                #[cfg(feature = "protocol-linux_dmabuf_v1")]
                Self::ZwpLinuxDmabufFeedbackV1 => {
                    if version > ZwpLinuxDmabufFeedbackV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpLinuxDmabufFeedbackV1::new(state, version))
                }
                #[cfg(feature = "protocol-linux_dmabuf_v1")]
                Self::ZwpLinuxDmabufV1 => {
                    if version > ZwpLinuxDmabufV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpLinuxDmabufV1::new(state, version))
                }
                #[cfg(feature = "protocol-linux_drm_syncobj_v1")]
                Self::WpLinuxDrmSyncobjManagerV1 => {
                    if version > WpLinuxDrmSyncobjManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpLinuxDrmSyncobjManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-linux_drm_syncobj_v1")]
                Self::WpLinuxDrmSyncobjSurfaceV1 => {
                    if version > WpLinuxDrmSyncobjSurfaceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpLinuxDrmSyncobjSurfaceV1::new(state, version))
                }
                #[cfg(feature = "protocol-linux_drm_syncobj_v1")]
                Self::WpLinuxDrmSyncobjTimelineV1 => {
                    if version > WpLinuxDrmSyncobjTimelineV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpLinuxDrmSyncobjTimelineV1::new(state, version))
                }
                #[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
                Self::ZwpConfinedPointerV1 => {
                    if version > ZwpConfinedPointerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpConfinedPointerV1::new(state, version))
                }
                #[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
                Self::ZwpLockedPointerV1 => {
                    if version > ZwpLockedPointerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpLockedPointerV1::new(state, version))
                }
                #[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
                Self::ZwpPointerConstraintsV1 => {
                    if version > ZwpPointerConstraintsV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpPointerConstraintsV1::new(state, version))
                }
                #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
                Self::ZwpPointerGestureHoldV1 => {
                    if version > ZwpPointerGestureHoldV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpPointerGestureHoldV1::new(state, version))
                }
                #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
                Self::ZwpPointerGesturePinchV1 => {
                    if version > ZwpPointerGesturePinchV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpPointerGesturePinchV1::new(state, version))
                }
                #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
                Self::ZwpPointerGestureSwipeV1 => {
                    if version > ZwpPointerGestureSwipeV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpPointerGestureSwipeV1::new(state, version))
                }
                #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
                Self::ZwpPointerGesturesV1 => {
                    if version > ZwpPointerGesturesV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpPointerGesturesV1::new(state, version))
                }
                #[cfg(feature = "protocol-pointer_warp_v1")]
                Self::WpPointerWarpV1 => {
                    if version > WpPointerWarpV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpPointerWarpV1::new(state, version))
                }
                #[cfg(feature = "protocol-presentation_time")]
                Self::WpPresentation => {
                    if version > WpPresentation::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpPresentation::new(state, version))
                }
                #[cfg(feature = "protocol-presentation_time")]
                Self::WpPresentationFeedback => {
                    if version > WpPresentationFeedback::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpPresentationFeedback::new(state, version))
                }
                #[cfg(feature = "protocol-relative_pointer_unstable_v1")]
                Self::ZwpRelativePointerManagerV1 => {
                    if version > ZwpRelativePointerManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpRelativePointerManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-relative_pointer_unstable_v1")]
                Self::ZwpRelativePointerV1 => {
                    if version > ZwpRelativePointerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpRelativePointerV1::new(state, version))
                }
                #[cfg(feature = "protocol-security_context_v1")]
                Self::WpSecurityContextManagerV1 => {
                    if version > WpSecurityContextManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpSecurityContextManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-security_context_v1")]
                Self::WpSecurityContextV1 => {
                    if version > WpSecurityContextV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpSecurityContextV1::new(state, version))
                }
                #[cfg(feature = "protocol-single_pixel_buffer_v1")]
                Self::WpSinglePixelBufferManagerV1 => {
                    if version > WpSinglePixelBufferManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpSinglePixelBufferManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-tablet_v2")]
                Self::ZwpTabletManagerV2 => {
                    if version > ZwpTabletManagerV2::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpTabletManagerV2::new(state, version))
                }
                #[cfg(feature = "protocol-tablet_v2")]
                Self::ZwpTabletPadDialV2 => {
                    if version > ZwpTabletPadDialV2::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpTabletPadDialV2::new(state, version))
                }
                #[cfg(feature = "protocol-tablet_v2")]
                Self::ZwpTabletPadGroupV2 => {
                    if version > ZwpTabletPadGroupV2::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpTabletPadGroupV2::new(state, version))
                }
                #[cfg(feature = "protocol-tablet_v2")]
                Self::ZwpTabletPadRingV2 => {
                    if version > ZwpTabletPadRingV2::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpTabletPadRingV2::new(state, version))
                }
                #[cfg(feature = "protocol-tablet_v2")]
                Self::ZwpTabletPadStripV2 => {
                    if version > ZwpTabletPadStripV2::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpTabletPadStripV2::new(state, version))
                }
                #[cfg(feature = "protocol-tablet_v2")]
                Self::ZwpTabletPadV2 => {
                    if version > ZwpTabletPadV2::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpTabletPadV2::new(state, version))
                }
                #[cfg(feature = "protocol-tablet_v2")]
                Self::ZwpTabletSeatV2 => {
                    if version > ZwpTabletSeatV2::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpTabletSeatV2::new(state, version))
                }
                #[cfg(feature = "protocol-tablet_v2")]
                Self::ZwpTabletToolV2 => {
                    if version > ZwpTabletToolV2::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpTabletToolV2::new(state, version))
                }
                #[cfg(feature = "protocol-tablet_v2")]
                Self::ZwpTabletV2 => {
                    if version > ZwpTabletV2::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpTabletV2::new(state, version))
                }
                #[cfg(feature = "protocol-tearing_control_v1")]
                Self::WpTearingControlManagerV1 => {
                    if version > WpTearingControlManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpTearingControlManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-tearing_control_v1")]
                Self::WpTearingControlV1 => {
                    if version > WpTearingControlV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpTearingControlV1::new(state, version))
                }
                #[cfg(feature = "protocol-text_input_unstable_v1")]
                Self::ZwpTextInputManagerV1 => {
                    if version > ZwpTextInputManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpTextInputManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-text_input_unstable_v1")]
                Self::ZwpTextInputV1 => {
                    if version > ZwpTextInputV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpTextInputV1::new(state, version))
                }
                #[cfg(feature = "protocol-text_input_unstable_v3")]
                Self::ZwpTextInputManagerV3 => {
                    if version > ZwpTextInputManagerV3::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpTextInputManagerV3::new(state, version))
                }
                #[cfg(feature = "protocol-text_input_unstable_v3")]
                Self::ZwpTextInputV3 => {
                    if version > ZwpTextInputV3::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpTextInputV3::new(state, version))
                }
                #[cfg(feature = "protocol-viewporter")]
                Self::WpViewport => {
                    if version > WpViewport::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpViewport::new(state, version))
                }
                #[cfg(feature = "protocol-viewporter")]
                Self::WpViewporter => {
                    if version > WpViewporter::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WpViewporter::new(state, version))
                }
                #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
                Self::ZwpPrimarySelectionDeviceManagerV1 => {
                    if version > ZwpPrimarySelectionDeviceManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpPrimarySelectionDeviceManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
                Self::ZwpPrimarySelectionDeviceV1 => {
                    if version > ZwpPrimarySelectionDeviceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpPrimarySelectionDeviceV1::new(state, version))
                }
                #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
                Self::ZwpPrimarySelectionOfferV1 => {
                    if version > ZwpPrimarySelectionOfferV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpPrimarySelectionOfferV1::new(state, version))
                }
                #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
                Self::ZwpPrimarySelectionSourceV1 => {
                    if version > ZwpPrimarySelectionSourceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpPrimarySelectionSourceV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_activation_v1")]
                Self::XdgActivationTokenV1 => {
                    if version > XdgActivationTokenV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(XdgActivationTokenV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_activation_v1")]
                Self::XdgActivationV1 => {
                    if version > XdgActivationV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(XdgActivationV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_decoration_unstable_v1")]
                Self::ZxdgDecorationManagerV1 => {
                    if version > ZxdgDecorationManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZxdgDecorationManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_decoration_unstable_v1")]
                Self::ZxdgToplevelDecorationV1 => {
                    if version > ZxdgToplevelDecorationV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZxdgToplevelDecorationV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_dialog_v1")]
                Self::XdgDialogV1 => {
                    if version > XdgDialogV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(XdgDialogV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_dialog_v1")]
                Self::XdgWmDialogV1 => {
                    if version > XdgWmDialogV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(XdgWmDialogV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
                Self::ZxdgExportedV2 => {
                    if version > ZxdgExportedV2::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZxdgExportedV2::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
                Self::ZxdgExporterV2 => {
                    if version > ZxdgExporterV2::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZxdgExporterV2::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
                Self::ZxdgImportedV2 => {
                    if version > ZxdgImportedV2::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZxdgImportedV2::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
                Self::ZxdgImporterV2 => {
                    if version > ZxdgImporterV2::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZxdgImporterV2::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_output_unstable_v1")]
                Self::ZxdgOutputManagerV1 => {
                    if version > ZxdgOutputManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZxdgOutputManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_output_unstable_v1")]
                Self::ZxdgOutputV1 => {
                    if version > ZxdgOutputV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZxdgOutputV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_shell")]
                Self::XdgPopup => {
                    if version > XdgPopup::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(XdgPopup::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_shell")]
                Self::XdgPositioner => {
                    if version > XdgPositioner::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(XdgPositioner::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_shell")]
                Self::XdgSurface => {
                    if version > XdgSurface::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(XdgSurface::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_shell")]
                Self::XdgToplevel => {
                    if version > XdgToplevel::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(XdgToplevel::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_shell")]
                Self::XdgWmBase => {
                    if version > XdgWmBase::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(XdgWmBase::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_system_bell_v1")]
                Self::XdgSystemBellV1 => {
                    if version > XdgSystemBellV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(XdgSystemBellV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_toplevel_drag_v1")]
                Self::XdgToplevelDragManagerV1 => {
                    if version > XdgToplevelDragManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(XdgToplevelDragManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_toplevel_drag_v1")]
                Self::XdgToplevelDragV1 => {
                    if version > XdgToplevelDragV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(XdgToplevelDragV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_toplevel_icon_v1")]
                Self::XdgToplevelIconManagerV1 => {
                    if version > XdgToplevelIconManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(XdgToplevelIconManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_toplevel_icon_v1")]
                Self::XdgToplevelIconV1 => {
                    if version > XdgToplevelIconV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(XdgToplevelIconV1::new(state, version))
                }
                #[cfg(feature = "protocol-xdg_toplevel_tag_v1")]
                Self::XdgToplevelTagManagerV1 => {
                    if version > XdgToplevelTagManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(XdgToplevelTagManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-xwayland_keyboard_grab_unstable_v1")]
                Self::ZwpXwaylandKeyboardGrabManagerV1 => {
                    if version > ZwpXwaylandKeyboardGrabManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpXwaylandKeyboardGrabManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-xwayland_keyboard_grab_unstable_v1")]
                Self::ZwpXwaylandKeyboardGrabV1 => {
                    if version > ZwpXwaylandKeyboardGrabV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpXwaylandKeyboardGrabV1::new(state, version))
                }
                #[cfg(feature = "protocol-xwayland_shell_v1")]
                Self::XwaylandShellV1 => {
                    if version > XwaylandShellV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(XwaylandShellV1::new(state, version))
                }
                #[cfg(feature = "protocol-xwayland_shell_v1")]
                Self::XwaylandSurfaceV1 => {
                    if version > XwaylandSurfaceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(XwaylandSurfaceV1::new(state, version))
                }
                #[cfg(feature = "protocol-xx_zones_v1")]
                Self::XxZoneItemV1 => {
                    if version > XxZoneItemV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(XxZoneItemV1::new(state, version))
                }
                #[cfg(feature = "protocol-xx_zones_v1")]
                Self::XxZoneManagerV1 => {
                    if version > XxZoneManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(XxZoneManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-xx_zones_v1")]
                Self::XxZoneV1 => {
                    if version > XxZoneV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(XxZoneV1::new(state, version))
                }
                #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
                Self::ZwpLinuxBufferReleaseV1 => {
                    if version > ZwpLinuxBufferReleaseV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpLinuxBufferReleaseV1::new(state, version))
                }
                #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
                Self::ZwpLinuxExplicitSynchronizationV1 => {
                    if version > ZwpLinuxExplicitSynchronizationV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpLinuxExplicitSynchronizationV1::new(state, version))
                }
                #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
                Self::ZwpLinuxSurfaceSynchronizationV1 => {
                    if version > ZwpLinuxSurfaceSynchronizationV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwpLinuxSurfaceSynchronizationV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
                Self::ZwlrDataControlDeviceV1 => {
                    if version > ZwlrDataControlDeviceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrDataControlDeviceV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
                Self::ZwlrDataControlManagerV1 => {
                    if version > ZwlrDataControlManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrDataControlManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
                Self::ZwlrDataControlOfferV1 => {
                    if version > ZwlrDataControlOfferV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrDataControlOfferV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
                Self::ZwlrDataControlSourceV1 => {
                    if version > ZwlrDataControlSourceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrDataControlSourceV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_export_dmabuf_unstable_v1")]
                Self::ZwlrExportDmabufFrameV1 => {
                    if version > ZwlrExportDmabufFrameV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrExportDmabufFrameV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_export_dmabuf_unstable_v1")]
                Self::ZwlrExportDmabufManagerV1 => {
                    if version > ZwlrExportDmabufManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrExportDmabufManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1")]
                Self::ZwlrForeignToplevelHandleV1 => {
                    if version > ZwlrForeignToplevelHandleV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrForeignToplevelHandleV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1")]
                Self::ZwlrForeignToplevelManagerV1 => {
                    if version > ZwlrForeignToplevelManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrForeignToplevelManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_gamma_control_unstable_v1")]
                Self::ZwlrGammaControlManagerV1 => {
                    if version > ZwlrGammaControlManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrGammaControlManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_gamma_control_unstable_v1")]
                Self::ZwlrGammaControlV1 => {
                    if version > ZwlrGammaControlV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrGammaControlV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_input_inhibit_unstable_v1")]
                Self::ZwlrInputInhibitManagerV1 => {
                    if version > ZwlrInputInhibitManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrInputInhibitManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_input_inhibit_unstable_v1")]
                Self::ZwlrInputInhibitorV1 => {
                    if version > ZwlrInputInhibitorV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrInputInhibitorV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")]
                Self::ZwlrLayerShellV1 => {
                    if version > ZwlrLayerShellV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrLayerShellV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")]
                Self::ZwlrLayerSurfaceV1 => {
                    if version > ZwlrLayerSurfaceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrLayerSurfaceV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
                Self::ZwlrOutputConfigurationHeadV1 => {
                    if version > ZwlrOutputConfigurationHeadV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrOutputConfigurationHeadV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
                Self::ZwlrOutputConfigurationV1 => {
                    if version > ZwlrOutputConfigurationV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrOutputConfigurationV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
                Self::ZwlrOutputHeadV1 => {
                    if version > ZwlrOutputHeadV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrOutputHeadV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
                Self::ZwlrOutputManagerV1 => {
                    if version > ZwlrOutputManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrOutputManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
                Self::ZwlrOutputModeV1 => {
                    if version > ZwlrOutputModeV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrOutputModeV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_output_power_management_unstable_v1")]
                Self::ZwlrOutputPowerManagerV1 => {
                    if version > ZwlrOutputPowerManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrOutputPowerManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_output_power_management_unstable_v1")]
                Self::ZwlrOutputPowerV1 => {
                    if version > ZwlrOutputPowerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrOutputPowerV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_screencopy_unstable_v1")]
                Self::ZwlrScreencopyFrameV1 => {
                    if version > ZwlrScreencopyFrameV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrScreencopyFrameV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_screencopy_unstable_v1")]
                Self::ZwlrScreencopyManagerV1 => {
                    if version > ZwlrScreencopyManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrScreencopyManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_virtual_pointer_unstable_v1")]
                Self::ZwlrVirtualPointerManagerV1 => {
                    if version > ZwlrVirtualPointerManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrVirtualPointerManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlr_virtual_pointer_unstable_v1")]
                Self::ZwlrVirtualPointerV1 => {
                    if version > ZwlrVirtualPointerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZwlrVirtualPointerV1::new(state, version))
                }
                #[cfg(feature = "protocol-wlproxy_sync_v1")]
                Self::WlproxySyncV1 => {
                    if version > WlproxySyncV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlproxySyncV1::new(state, version))
                }
                #[cfg(test)]
                Self::WlproxyTest => {
                    if version > WlproxyTest::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlproxyTest::new(state, version))
                }
                #[cfg(test)]
                Self::WlproxyTestArrayEcho => {
                    if version > WlproxyTestArrayEcho::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlproxyTestArrayEcho::new(state, version))
                }
                #[cfg(test)]
                Self::WlproxyTestDummy => {
                    if version > WlproxyTestDummy::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlproxyTestDummy::new(state, version))
                }
                #[cfg(test)]
                Self::WlproxyTestFdEcho => {
                    if version > WlproxyTestFdEcho::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlproxyTestFdEcho::new(state, version))
                }
                #[cfg(test)]
                Self::WlproxyTestHops => {
                    if version > WlproxyTestHops::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlproxyTestHops::new(state, version))
                }
                #[cfg(test)]
                Self::WlproxyTestNonForward => {
                    if version > WlproxyTestNonForward::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlproxyTestNonForward::new(state, version))
                }
                #[cfg(test)]
                Self::WlproxyTestObjectEcho => {
                    if version > WlproxyTestObjectEcho::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlproxyTestObjectEcho::new(state, version))
                }
                #[cfg(test)]
                Self::WlproxyTestServerSent => {
                    if version > WlproxyTestServerSent::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WlproxyTestServerSent::new(state, version))
                }
                #[cfg(feature = "protocol-river_input_management_v1")]
                Self::RiverInputDeviceV1 => {
                    if version > RiverInputDeviceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(RiverInputDeviceV1::new(state, version))
                }
                #[cfg(feature = "protocol-river_input_management_v1")]
                Self::RiverInputManagerV1 => {
                    if version > RiverInputManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(RiverInputManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-river_layer_shell_v1")]
                Self::RiverLayerShellOutputV1 => {
                    if version > RiverLayerShellOutputV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(RiverLayerShellOutputV1::new(state, version))
                }
                #[cfg(feature = "protocol-river_layer_shell_v1")]
                Self::RiverLayerShellSeatV1 => {
                    if version > RiverLayerShellSeatV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(RiverLayerShellSeatV1::new(state, version))
                }
                #[cfg(feature = "protocol-river_layer_shell_v1")]
                Self::RiverLayerShellV1 => {
                    if version > RiverLayerShellV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(RiverLayerShellV1::new(state, version))
                }
                #[cfg(feature = "protocol-river_libinput_config_v1")]
                Self::RiverLibinputAccelConfigV1 => {
                    if version > RiverLibinputAccelConfigV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(RiverLibinputAccelConfigV1::new(state, version))
                }
                #[cfg(feature = "protocol-river_libinput_config_v1")]
                Self::RiverLibinputConfigV1 => {
                    if version > RiverLibinputConfigV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(RiverLibinputConfigV1::new(state, version))
                }
                #[cfg(feature = "protocol-river_libinput_config_v1")]
                Self::RiverLibinputDeviceV1 => {
                    if version > RiverLibinputDeviceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(RiverLibinputDeviceV1::new(state, version))
                }
                #[cfg(feature = "protocol-river_libinput_config_v1")]
                Self::RiverLibinputResultV1 => {
                    if version > RiverLibinputResultV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(RiverLibinputResultV1::new(state, version))
                }
                #[cfg(feature = "protocol-river_window_management_v1")]
                Self::RiverDecorationV1 => {
                    if version > RiverDecorationV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(RiverDecorationV1::new(state, version))
                }
                #[cfg(feature = "protocol-river_window_management_v1")]
                Self::RiverNodeV1 => {
                    if version > RiverNodeV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(RiverNodeV1::new(state, version))
                }
                #[cfg(feature = "protocol-river_window_management_v1")]
                Self::RiverOutputV1 => {
                    if version > RiverOutputV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(RiverOutputV1::new(state, version))
                }
                #[cfg(feature = "protocol-river_window_management_v1")]
                Self::RiverPointerBindingV1 => {
                    if version > RiverPointerBindingV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(RiverPointerBindingV1::new(state, version))
                }
                #[cfg(feature = "protocol-river_window_management_v1")]
                Self::RiverSeatV1 => {
                    if version > RiverSeatV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(RiverSeatV1::new(state, version))
                }
                #[cfg(feature = "protocol-river_window_management_v1")]
                Self::RiverShellSurfaceV1 => {
                    if version > RiverShellSurfaceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(RiverShellSurfaceV1::new(state, version))
                }
                #[cfg(feature = "protocol-river_window_management_v1")]
                Self::RiverWindowManagerV1 => {
                    if version > RiverWindowManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(RiverWindowManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-river_window_management_v1")]
                Self::RiverWindowV1 => {
                    if version > RiverWindowV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(RiverWindowV1::new(state, version))
                }
                #[cfg(feature = "protocol-river_xkb_bindings_v1")]
                Self::RiverXkbBindingV1 => {
                    if version > RiverXkbBindingV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(RiverXkbBindingV1::new(state, version))
                }
                #[cfg(feature = "protocol-river_xkb_bindings_v1")]
                Self::RiverXkbBindingsSeatV1 => {
                    if version > RiverXkbBindingsSeatV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(RiverXkbBindingsSeatV1::new(state, version))
                }
                #[cfg(feature = "protocol-river_xkb_bindings_v1")]
                Self::RiverXkbBindingsV1 => {
                    if version > RiverXkbBindingsV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(RiverXkbBindingsV1::new(state, version))
                }
                #[cfg(feature = "protocol-river_xkb_config_v1")]
                Self::RiverXkbConfigV1 => {
                    if version > RiverXkbConfigV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(RiverXkbConfigV1::new(state, version))
                }
                #[cfg(feature = "protocol-river_xkb_config_v1")]
                Self::RiverXkbKeyboardV1 => {
                    if version > RiverXkbKeyboardV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(RiverXkbKeyboardV1::new(state, version))
                }
                #[cfg(feature = "protocol-river_xkb_config_v1")]
                Self::RiverXkbKeymapV1 => {
                    if version > RiverXkbKeymapV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(RiverXkbKeymapV1::new(state, version))
                }
                #[cfg(feature = "protocol-ivi_application")]
                Self::IviApplication => {
                    if version > IviApplication::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(IviApplication::new(state, version))
                }
                #[cfg(feature = "protocol-ivi_application")]
                Self::IviSurface => {
                    if version > IviSurface::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(IviSurface::new(state, version))
                }
                #[cfg(feature = "protocol-ivi_hmi_controller")]
                Self::IviHmiController => {
                    if version > IviHmiController::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(IviHmiController::new(state, version))
                }
                #[cfg(feature = "protocol-weston_content_protection")]
                Self::WestonContentProtection => {
                    if version > WestonContentProtection::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WestonContentProtection::new(state, version))
                }
                #[cfg(feature = "protocol-weston_content_protection")]
                Self::WestonProtectedSurface => {
                    if version > WestonProtectedSurface::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WestonProtectedSurface::new(state, version))
                }
                #[cfg(feature = "protocol-weston_debug")]
                Self::WestonDebugStreamV1 => {
                    if version > WestonDebugStreamV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WestonDebugStreamV1::new(state, version))
                }
                #[cfg(feature = "protocol-weston_debug")]
                Self::WestonDebugV1 => {
                    if version > WestonDebugV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WestonDebugV1::new(state, version))
                }
                #[cfg(feature = "protocol-weston_desktop")]
                Self::WestonDesktopShell => {
                    if version > WestonDesktopShell::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WestonDesktopShell::new(state, version))
                }
                #[cfg(feature = "protocol-weston_desktop")]
                Self::WestonScreensaver => {
                    if version > WestonScreensaver::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WestonScreensaver::new(state, version))
                }
                #[cfg(feature = "protocol-weston_direct_display")]
                Self::WestonDirectDisplayV1 => {
                    if version > WestonDirectDisplayV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WestonDirectDisplayV1::new(state, version))
                }
                #[cfg(feature = "protocol-weston_output_capture")]
                Self::WestonCaptureSourceV1 => {
                    if version > WestonCaptureSourceV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WestonCaptureSourceV1::new(state, version))
                }
                #[cfg(feature = "protocol-weston_output_capture")]
                Self::WestonCaptureV1 => {
                    if version > WestonCaptureV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WestonCaptureV1::new(state, version))
                }
                #[cfg(feature = "protocol-weston_test")]
                Self::WestonTest => {
                    if version > WestonTest::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WestonTest::new(state, version))
                }
                #[cfg(feature = "protocol-weston_test")]
                Self::WestonTestRunner => {
                    if version > WestonTestRunner::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WestonTestRunner::new(state, version))
                }
                #[cfg(feature = "protocol-weston_touch_calibration")]
                Self::WestonTouchCalibration => {
                    if version > WestonTouchCalibration::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WestonTouchCalibration::new(state, version))
                }
                #[cfg(feature = "protocol-weston_touch_calibration")]
                Self::WestonTouchCalibrator => {
                    if version > WestonTouchCalibrator::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WestonTouchCalibrator::new(state, version))
                }
                #[cfg(feature = "protocol-weston_touch_calibration")]
                Self::WestonTouchCoordinate => {
                    if version > WestonTouchCoordinate::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(WestonTouchCoordinate::new(state, version))
                }
                #[cfg(feature = "protocol-cosmic_a11y_v1")]
                Self::CosmicA11yManagerV1 => {
                    if version > CosmicA11yManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(CosmicA11yManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-cosmic_corner_radius_v1")]
                Self::CosmicCornerRadiusManagerV1 => {
                    if version > CosmicCornerRadiusManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(CosmicCornerRadiusManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-cosmic_corner_radius_v1")]
                Self::CosmicCornerRadiusToplevelV1 => {
                    if version > CosmicCornerRadiusToplevelV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(CosmicCornerRadiusToplevelV1::new(state, version))
                }
                #[cfg(feature = "protocol-cosmic_image_source_unstable_v1")]
                Self::ZcosmicWorkspaceImageCaptureSourceManagerV1 => {
                    if version > ZcosmicWorkspaceImageCaptureSourceManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZcosmicWorkspaceImageCaptureSourceManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
                Self::ZcosmicOutputConfigurationHeadV1 => {
                    if version > ZcosmicOutputConfigurationHeadV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZcosmicOutputConfigurationHeadV1::new(state, version))
                }
                #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
                Self::ZcosmicOutputConfigurationV1 => {
                    if version > ZcosmicOutputConfigurationV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZcosmicOutputConfigurationV1::new(state, version))
                }
                #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
                Self::ZcosmicOutputHeadV1 => {
                    if version > ZcosmicOutputHeadV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZcosmicOutputHeadV1::new(state, version))
                }
                #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
                Self::ZcosmicOutputManagerV1 => {
                    if version > ZcosmicOutputManagerV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZcosmicOutputManagerV1::new(state, version))
                }
                #[cfg(feature = "protocol-cosmic_overlap_notify_unstable_v1")]
                Self::ZcosmicOverlapNotificationV1 => {
                    if version > ZcosmicOverlapNotificationV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZcosmicOverlapNotificationV1::new(state, version))
                }
                #[cfg(feature = "protocol-cosmic_overlap_notify_unstable_v1")]
                Self::ZcosmicOverlapNotifyV1 => {
                    if version > ZcosmicOverlapNotifyV1::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZcosmicOverlapNotifyV1::new(state, version))
                }
                #[cfg(feature = "protocol-cosmic_workspace_unstable_v2")]
                Self::ZcosmicWorkspaceHandleV2 => {
                    if version > ZcosmicWorkspaceHandleV2::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZcosmicWorkspaceHandleV2::new(state, version))
                }
                #[cfg(feature = "protocol-cosmic_workspace_unstable_v2")]
                Self::ZcosmicWorkspaceManagerV2 => {
                    if version > ZcosmicWorkspaceManagerV2::XML_VERSION {
                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));
                    }
                    Ok(ZcosmicWorkspaceManagerV2::new(state, version))
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, linearize::Linearize)]
#[linearize(const)]
pub enum ObjectInterface {
    /// hyprland_ctm_control_manager_v1
    #[cfg(feature = "protocol-hyprland_ctm_control_v1")]
    HyprlandCtmControlManagerV1,
    /// hyprland_focus_grab_manager_v1
    #[cfg(feature = "protocol-hyprland_focus_grab_v1")]
    HyprlandFocusGrabManagerV1,
    /// hyprland_focus_grab_v1
    #[cfg(feature = "protocol-hyprland_focus_grab_v1")]
    HyprlandFocusGrabV1,
    /// hyprland_global_shortcut_v1
    #[cfg(feature = "protocol-hyprland_global_shortcuts_v1")]
    HyprlandGlobalShortcutV1,
    /// hyprland_global_shortcuts_manager_v1
    #[cfg(feature = "protocol-hyprland_global_shortcuts_v1")]
    HyprlandGlobalShortcutsManagerV1,
    /// hyprland_input_capture_manager_v1
    #[cfg(feature = "protocol-hyprland_input_capture_v1")]
    HyprlandInputCaptureManagerV1,
    /// hyprland_input_capture_v1
    #[cfg(feature = "protocol-hyprland_input_capture_v1")]
    HyprlandInputCaptureV1,
    /// hyprland_lock_notification_v1
    #[cfg(feature = "protocol-hyprland_lock_notify_v1")]
    HyprlandLockNotificationV1,
    /// hyprland_lock_notifier_v1
    #[cfg(feature = "protocol-hyprland_lock_notify_v1")]
    HyprlandLockNotifierV1,
    /// hyprland_surface_manager_v1
    #[cfg(feature = "protocol-hyprland_surface_v1")]
    HyprlandSurfaceManagerV1,
    /// hyprland_surface_v1
    #[cfg(feature = "protocol-hyprland_surface_v1")]
    HyprlandSurfaceV1,
    /// hyprland_toplevel_export_frame_v1
    #[cfg(feature = "protocol-hyprland_toplevel_export_v1")]
    HyprlandToplevelExportFrameV1,
    /// hyprland_toplevel_export_manager_v1
    #[cfg(feature = "protocol-hyprland_toplevel_export_v1")]
    HyprlandToplevelExportManagerV1,
    /// hyprland_toplevel_mapping_manager_v1
    #[cfg(feature = "protocol-hyprland_toplevel_mapping_v1")]
    HyprlandToplevelMappingManagerV1,
    /// hyprland_toplevel_window_mapping_handle_v1
    #[cfg(feature = "protocol-hyprland_toplevel_mapping_v1")]
    HyprlandToplevelWindowMappingHandleV1,
    /// jay_popup_ext_manager_v1
    #[cfg(feature = "protocol-jay_popup_ext_v1")]
    JayPopupExtManagerV1,
    /// jay_popup_ext_v1
    #[cfg(feature = "protocol-jay_popup_ext_v1")]
    JayPopupExtV1,
    /// jay_tray_item_v1
    #[cfg(feature = "protocol-jay_tray_v1")]
    JayTrayItemV1,
    /// jay_tray_v1
    #[cfg(feature = "protocol-jay_tray_v1")]
    JayTrayV1,
    /// wl_drm
    #[cfg(feature = "protocol-drm")]
    WlDrm,
    /// zwp_input_method_keyboard_grab_v2
    #[cfg(feature = "protocol-input_method_unstable_v2")]
    ZwpInputMethodKeyboardGrabV2,
    /// zwp_input_method_manager_v2
    #[cfg(feature = "protocol-input_method_unstable_v2")]
    ZwpInputMethodManagerV2,
    /// zwp_input_method_v2
    #[cfg(feature = "protocol-input_method_unstable_v2")]
    ZwpInputMethodV2,
    /// zwp_input_popup_surface_v2
    #[cfg(feature = "protocol-input_method_unstable_v2")]
    ZwpInputPopupSurfaceV2,
    /// org_kde_kwin_server_decoration
    #[cfg(feature = "protocol-org_kde_kwin_server_decoration_v1")]
    OrgKdeKwinServerDecoration,
    /// org_kde_kwin_server_decoration_manager
    #[cfg(feature = "protocol-org_kde_kwin_server_decoration_v1")]
    OrgKdeKwinServerDecorationManager,
    /// zwp_virtual_keyboard_manager_v1
    #[cfg(feature = "protocol-virtual_keyboard_unstable_v1")]
    ZwpVirtualKeyboardManagerV1,
    /// zwp_virtual_keyboard_v1
    #[cfg(feature = "protocol-virtual_keyboard_unstable_v1")]
    ZwpVirtualKeyboardV1,
    /// wl_buffer
    WlBuffer,
    /// wl_callback
    WlCallback,
    /// wl_compositor
    WlCompositor,
    /// wl_data_device
    WlDataDevice,
    /// wl_data_device_manager
    WlDataDeviceManager,
    /// wl_data_offer
    WlDataOffer,
    /// wl_data_source
    WlDataSource,
    /// wl_display
    WlDisplay,
    /// wl_fixes
    WlFixes,
    /// wl_keyboard
    WlKeyboard,
    /// wl_output
    WlOutput,
    /// wl_pointer
    WlPointer,
    /// wl_region
    WlRegion,
    /// wl_registry
    WlRegistry,
    /// wl_seat
    WlSeat,
    /// wl_shell
    WlShell,
    /// wl_shell_surface
    WlShellSurface,
    /// wl_shm
    WlShm,
    /// wl_shm_pool
    WlShmPool,
    /// wl_subcompositor
    WlSubcompositor,
    /// wl_subsurface
    WlSubsurface,
    /// wl_surface
    WlSurface,
    /// wl_touch
    WlTouch,
    /// wp_alpha_modifier_surface_v1
    #[cfg(feature = "protocol-alpha_modifier_v1")]
    WpAlphaModifierSurfaceV1,
    /// wp_alpha_modifier_v1
    #[cfg(feature = "protocol-alpha_modifier_v1")]
    WpAlphaModifierV1,
    /// wp_color_management_output_v1
    #[cfg(feature = "protocol-color_management_v1")]
    WpColorManagementOutputV1,
    /// wp_color_management_surface_feedback_v1
    #[cfg(feature = "protocol-color_management_v1")]
    WpColorManagementSurfaceFeedbackV1,
    /// wp_color_management_surface_v1
    #[cfg(feature = "protocol-color_management_v1")]
    WpColorManagementSurfaceV1,
    /// wp_color_manager_v1
    #[cfg(feature = "protocol-color_management_v1")]
    WpColorManagerV1,
    /// wp_image_description_creator_icc_v1
    #[cfg(feature = "protocol-color_management_v1")]
    WpImageDescriptionCreatorIccV1,
    /// wp_image_description_creator_params_v1
    #[cfg(feature = "protocol-color_management_v1")]
    WpImageDescriptionCreatorParamsV1,
    /// wp_image_description_info_v1
    #[cfg(feature = "protocol-color_management_v1")]
    WpImageDescriptionInfoV1,
    /// wp_image_description_reference_v1
    #[cfg(feature = "protocol-color_management_v1")]
    WpImageDescriptionReferenceV1,
    /// wp_image_description_v1
    #[cfg(feature = "protocol-color_management_v1")]
    WpImageDescriptionV1,
    /// wp_color_representation_manager_v1
    #[cfg(feature = "protocol-color_representation_v1")]
    WpColorRepresentationManagerV1,
    /// wp_color_representation_surface_v1
    #[cfg(feature = "protocol-color_representation_v1")]
    WpColorRepresentationSurfaceV1,
    /// wp_commit_timer_v1
    #[cfg(feature = "protocol-commit_timing_v1")]
    WpCommitTimerV1,
    /// wp_commit_timing_manager_v1
    #[cfg(feature = "protocol-commit_timing_v1")]
    WpCommitTimingManagerV1,
    /// wp_content_type_manager_v1
    #[cfg(feature = "protocol-content_type_v1")]
    WpContentTypeManagerV1,
    /// wp_content_type_v1
    #[cfg(feature = "protocol-content_type_v1")]
    WpContentTypeV1,
    /// wp_cursor_shape_device_v1
    #[cfg(feature = "protocol-cursor_shape_v1")]
    WpCursorShapeDeviceV1,
    /// wp_cursor_shape_manager_v1
    #[cfg(feature = "protocol-cursor_shape_v1")]
    WpCursorShapeManagerV1,
    /// wp_drm_lease_connector_v1
    #[cfg(feature = "protocol-drm_lease_v1")]
    WpDrmLeaseConnectorV1,
    /// wp_drm_lease_device_v1
    #[cfg(feature = "protocol-drm_lease_v1")]
    WpDrmLeaseDeviceV1,
    /// wp_drm_lease_request_v1
    #[cfg(feature = "protocol-drm_lease_v1")]
    WpDrmLeaseRequestV1,
    /// wp_drm_lease_v1
    #[cfg(feature = "protocol-drm_lease_v1")]
    WpDrmLeaseV1,
    /// ext_background_effect_manager_v1
    #[cfg(feature = "protocol-ext_background_effect_v1")]
    ExtBackgroundEffectManagerV1,
    /// ext_background_effect_surface_v1
    #[cfg(feature = "protocol-ext_background_effect_v1")]
    ExtBackgroundEffectSurfaceV1,
    /// ext_data_control_device_v1
    #[cfg(feature = "protocol-ext_data_control_v1")]
    ExtDataControlDeviceV1,
    /// ext_data_control_manager_v1
    #[cfg(feature = "protocol-ext_data_control_v1")]
    ExtDataControlManagerV1,
    /// ext_data_control_offer_v1
    #[cfg(feature = "protocol-ext_data_control_v1")]
    ExtDataControlOfferV1,
    /// ext_data_control_source_v1
    #[cfg(feature = "protocol-ext_data_control_v1")]
    ExtDataControlSourceV1,
    /// ext_foreign_toplevel_handle_v1
    #[cfg(feature = "protocol-ext_foreign_toplevel_list_v1")]
    ExtForeignToplevelHandleV1,
    /// ext_foreign_toplevel_list_v1
    #[cfg(feature = "protocol-ext_foreign_toplevel_list_v1")]
    ExtForeignToplevelListV1,
    /// ext_idle_notification_v1
    #[cfg(feature = "protocol-ext_idle_notify_v1")]
    ExtIdleNotificationV1,
    /// ext_idle_notifier_v1
    #[cfg(feature = "protocol-ext_idle_notify_v1")]
    ExtIdleNotifierV1,
    /// ext_foreign_toplevel_image_capture_source_manager_v1
    #[cfg(feature = "protocol-ext_image_capture_source_v1")]
    ExtForeignToplevelImageCaptureSourceManagerV1,
    /// ext_image_capture_source_v1
    #[cfg(feature = "protocol-ext_image_capture_source_v1")]
    ExtImageCaptureSourceV1,
    /// ext_output_image_capture_source_manager_v1
    #[cfg(feature = "protocol-ext_image_capture_source_v1")]
    ExtOutputImageCaptureSourceManagerV1,
    /// ext_image_copy_capture_cursor_session_v1
    #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
    ExtImageCopyCaptureCursorSessionV1,
    /// ext_image_copy_capture_frame_v1
    #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
    ExtImageCopyCaptureFrameV1,
    /// ext_image_copy_capture_manager_v1
    #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
    ExtImageCopyCaptureManagerV1,
    /// ext_image_copy_capture_session_v1
    #[cfg(feature = "protocol-ext_image_copy_capture_v1")]
    ExtImageCopyCaptureSessionV1,
    /// ext_session_lock_manager_v1
    #[cfg(feature = "protocol-ext_session_lock_v1")]
    ExtSessionLockManagerV1,
    /// ext_session_lock_surface_v1
    #[cfg(feature = "protocol-ext_session_lock_v1")]
    ExtSessionLockSurfaceV1,
    /// ext_session_lock_v1
    #[cfg(feature = "protocol-ext_session_lock_v1")]
    ExtSessionLockV1,
    /// ext_transient_seat_manager_v1
    #[cfg(feature = "protocol-ext_transient_seat_v1")]
    ExtTransientSeatManagerV1,
    /// ext_transient_seat_v1
    #[cfg(feature = "protocol-ext_transient_seat_v1")]
    ExtTransientSeatV1,
    /// ext_workspace_group_handle_v1
    #[cfg(feature = "protocol-ext_workspace_v1")]
    ExtWorkspaceGroupHandleV1,
    /// ext_workspace_handle_v1
    #[cfg(feature = "protocol-ext_workspace_v1")]
    ExtWorkspaceHandleV1,
    /// ext_workspace_manager_v1
    #[cfg(feature = "protocol-ext_workspace_v1")]
    ExtWorkspaceManagerV1,
    /// wp_fifo_manager_v1
    #[cfg(feature = "protocol-fifo_v1")]
    WpFifoManagerV1,
    /// wp_fifo_v1
    #[cfg(feature = "protocol-fifo_v1")]
    WpFifoV1,
    /// wp_fractional_scale_manager_v1
    #[cfg(feature = "protocol-fractional_scale_v1")]
    WpFractionalScaleManagerV1,
    /// wp_fractional_scale_v1
    #[cfg(feature = "protocol-fractional_scale_v1")]
    WpFractionalScaleV1,
    /// zwp_fullscreen_shell_mode_feedback_v1
    #[cfg(feature = "protocol-fullscreen_shell_unstable_v1")]
    ZwpFullscreenShellModeFeedbackV1,
    /// zwp_fullscreen_shell_v1
    #[cfg(feature = "protocol-fullscreen_shell_unstable_v1")]
    ZwpFullscreenShellV1,
    /// zwp_idle_inhibit_manager_v1
    #[cfg(feature = "protocol-idle_inhibit_unstable_v1")]
    ZwpIdleInhibitManagerV1,
    /// zwp_idle_inhibitor_v1
    #[cfg(feature = "protocol-idle_inhibit_unstable_v1")]
    ZwpIdleInhibitorV1,
    /// zwp_input_method_context_v1
    #[cfg(feature = "protocol-input_method_unstable_v1")]
    ZwpInputMethodContextV1,
    /// zwp_input_method_v1
    #[cfg(feature = "protocol-input_method_unstable_v1")]
    ZwpInputMethodV1,
    /// zwp_input_panel_surface_v1
    #[cfg(feature = "protocol-input_method_unstable_v1")]
    ZwpInputPanelSurfaceV1,
    /// zwp_input_panel_v1
    #[cfg(feature = "protocol-input_method_unstable_v1")]
    ZwpInputPanelV1,
    /// zwp_input_timestamps_manager_v1
    #[cfg(feature = "protocol-input_timestamps_unstable_v1")]
    ZwpInputTimestampsManagerV1,
    /// zwp_input_timestamps_v1
    #[cfg(feature = "protocol-input_timestamps_unstable_v1")]
    ZwpInputTimestampsV1,
    /// zwp_keyboard_shortcuts_inhibit_manager_v1
    #[cfg(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1")]
    ZwpKeyboardShortcutsInhibitManagerV1,
    /// zwp_keyboard_shortcuts_inhibitor_v1
    #[cfg(feature = "protocol-keyboard_shortcuts_inhibit_unstable_v1")]
    ZwpKeyboardShortcutsInhibitorV1,
    /// zwp_linux_buffer_params_v1
    #[cfg(feature = "protocol-linux_dmabuf_v1")]
    ZwpLinuxBufferParamsV1,
    /// zwp_linux_dmabuf_feedback_v1
    #[cfg(feature = "protocol-linux_dmabuf_v1")]
    ZwpLinuxDmabufFeedbackV1,
    /// zwp_linux_dmabuf_v1
    #[cfg(feature = "protocol-linux_dmabuf_v1")]
    ZwpLinuxDmabufV1,
    /// wp_linux_drm_syncobj_manager_v1
    #[cfg(feature = "protocol-linux_drm_syncobj_v1")]
    WpLinuxDrmSyncobjManagerV1,
    /// wp_linux_drm_syncobj_surface_v1
    #[cfg(feature = "protocol-linux_drm_syncobj_v1")]
    WpLinuxDrmSyncobjSurfaceV1,
    /// wp_linux_drm_syncobj_timeline_v1
    #[cfg(feature = "protocol-linux_drm_syncobj_v1")]
    WpLinuxDrmSyncobjTimelineV1,
    /// zwp_confined_pointer_v1
    #[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
    ZwpConfinedPointerV1,
    /// zwp_locked_pointer_v1
    #[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
    ZwpLockedPointerV1,
    /// zwp_pointer_constraints_v1
    #[cfg(feature = "protocol-pointer_constraints_unstable_v1")]
    ZwpPointerConstraintsV1,
    /// zwp_pointer_gesture_hold_v1
    #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
    ZwpPointerGestureHoldV1,
    /// zwp_pointer_gesture_pinch_v1
    #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
    ZwpPointerGesturePinchV1,
    /// zwp_pointer_gesture_swipe_v1
    #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
    ZwpPointerGestureSwipeV1,
    /// zwp_pointer_gestures_v1
    #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
    ZwpPointerGesturesV1,
    /// wp_pointer_warp_v1
    #[cfg(feature = "protocol-pointer_warp_v1")]
    WpPointerWarpV1,
    /// wp_presentation
    #[cfg(feature = "protocol-presentation_time")]
    WpPresentation,
    /// wp_presentation_feedback
    #[cfg(feature = "protocol-presentation_time")]
    WpPresentationFeedback,
    /// zwp_relative_pointer_manager_v1
    #[cfg(feature = "protocol-relative_pointer_unstable_v1")]
    ZwpRelativePointerManagerV1,
    /// zwp_relative_pointer_v1
    #[cfg(feature = "protocol-relative_pointer_unstable_v1")]
    ZwpRelativePointerV1,
    /// wp_security_context_manager_v1
    #[cfg(feature = "protocol-security_context_v1")]
    WpSecurityContextManagerV1,
    /// wp_security_context_v1
    #[cfg(feature = "protocol-security_context_v1")]
    WpSecurityContextV1,
    /// wp_single_pixel_buffer_manager_v1
    #[cfg(feature = "protocol-single_pixel_buffer_v1")]
    WpSinglePixelBufferManagerV1,
    /// zwp_tablet_manager_v2
    #[cfg(feature = "protocol-tablet_v2")]
    ZwpTabletManagerV2,
    /// zwp_tablet_pad_dial_v2
    #[cfg(feature = "protocol-tablet_v2")]
    ZwpTabletPadDialV2,
    /// zwp_tablet_pad_group_v2
    #[cfg(feature = "protocol-tablet_v2")]
    ZwpTabletPadGroupV2,
    /// zwp_tablet_pad_ring_v2
    #[cfg(feature = "protocol-tablet_v2")]
    ZwpTabletPadRingV2,
    /// zwp_tablet_pad_strip_v2
    #[cfg(feature = "protocol-tablet_v2")]
    ZwpTabletPadStripV2,
    /// zwp_tablet_pad_v2
    #[cfg(feature = "protocol-tablet_v2")]
    ZwpTabletPadV2,
    /// zwp_tablet_seat_v2
    #[cfg(feature = "protocol-tablet_v2")]
    ZwpTabletSeatV2,
    /// zwp_tablet_tool_v2
    #[cfg(feature = "protocol-tablet_v2")]
    ZwpTabletToolV2,
    /// zwp_tablet_v2
    #[cfg(feature = "protocol-tablet_v2")]
    ZwpTabletV2,
    /// wp_tearing_control_manager_v1
    #[cfg(feature = "protocol-tearing_control_v1")]
    WpTearingControlManagerV1,
    /// wp_tearing_control_v1
    #[cfg(feature = "protocol-tearing_control_v1")]
    WpTearingControlV1,
    /// zwp_text_input_manager_v1
    #[cfg(feature = "protocol-text_input_unstable_v1")]
    ZwpTextInputManagerV1,
    /// zwp_text_input_v1
    #[cfg(feature = "protocol-text_input_unstable_v1")]
    ZwpTextInputV1,
    /// zwp_text_input_manager_v3
    #[cfg(feature = "protocol-text_input_unstable_v3")]
    ZwpTextInputManagerV3,
    /// zwp_text_input_v3
    #[cfg(feature = "protocol-text_input_unstable_v3")]
    ZwpTextInputV3,
    /// wp_viewport
    #[cfg(feature = "protocol-viewporter")]
    WpViewport,
    /// wp_viewporter
    #[cfg(feature = "protocol-viewporter")]
    WpViewporter,
    /// zwp_primary_selection_device_manager_v1
    #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
    ZwpPrimarySelectionDeviceManagerV1,
    /// zwp_primary_selection_device_v1
    #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
    ZwpPrimarySelectionDeviceV1,
    /// zwp_primary_selection_offer_v1
    #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
    ZwpPrimarySelectionOfferV1,
    /// zwp_primary_selection_source_v1
    #[cfg(feature = "protocol-wp_primary_selection_unstable_v1")]
    ZwpPrimarySelectionSourceV1,
    /// xdg_activation_token_v1
    #[cfg(feature = "protocol-xdg_activation_v1")]
    XdgActivationTokenV1,
    /// xdg_activation_v1
    #[cfg(feature = "protocol-xdg_activation_v1")]
    XdgActivationV1,
    /// zxdg_decoration_manager_v1
    #[cfg(feature = "protocol-xdg_decoration_unstable_v1")]
    ZxdgDecorationManagerV1,
    /// zxdg_toplevel_decoration_v1
    #[cfg(feature = "protocol-xdg_decoration_unstable_v1")]
    ZxdgToplevelDecorationV1,
    /// xdg_dialog_v1
    #[cfg(feature = "protocol-xdg_dialog_v1")]
    XdgDialogV1,
    /// xdg_wm_dialog_v1
    #[cfg(feature = "protocol-xdg_dialog_v1")]
    XdgWmDialogV1,
    /// zxdg_exported_v2
    #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
    ZxdgExportedV2,
    /// zxdg_exporter_v2
    #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
    ZxdgExporterV2,
    /// zxdg_imported_v2
    #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
    ZxdgImportedV2,
    /// zxdg_importer_v2
    #[cfg(feature = "protocol-xdg_foreign_unstable_v2")]
    ZxdgImporterV2,
    /// zxdg_output_manager_v1
    #[cfg(feature = "protocol-xdg_output_unstable_v1")]
    ZxdgOutputManagerV1,
    /// zxdg_output_v1
    #[cfg(feature = "protocol-xdg_output_unstable_v1")]
    ZxdgOutputV1,
    /// xdg_popup
    #[cfg(feature = "protocol-xdg_shell")]
    XdgPopup,
    /// xdg_positioner
    #[cfg(feature = "protocol-xdg_shell")]
    XdgPositioner,
    /// xdg_surface
    #[cfg(feature = "protocol-xdg_shell")]
    XdgSurface,
    /// xdg_toplevel
    #[cfg(feature = "protocol-xdg_shell")]
    XdgToplevel,
    /// xdg_wm_base
    #[cfg(feature = "protocol-xdg_shell")]
    XdgWmBase,
    /// xdg_system_bell_v1
    #[cfg(feature = "protocol-xdg_system_bell_v1")]
    XdgSystemBellV1,
    /// xdg_toplevel_drag_manager_v1
    #[cfg(feature = "protocol-xdg_toplevel_drag_v1")]
    XdgToplevelDragManagerV1,
    /// xdg_toplevel_drag_v1
    #[cfg(feature = "protocol-xdg_toplevel_drag_v1")]
    XdgToplevelDragV1,
    /// xdg_toplevel_icon_manager_v1
    #[cfg(feature = "protocol-xdg_toplevel_icon_v1")]
    XdgToplevelIconManagerV1,
    /// xdg_toplevel_icon_v1
    #[cfg(feature = "protocol-xdg_toplevel_icon_v1")]
    XdgToplevelIconV1,
    /// xdg_toplevel_tag_manager_v1
    #[cfg(feature = "protocol-xdg_toplevel_tag_v1")]
    XdgToplevelTagManagerV1,
    /// zwp_xwayland_keyboard_grab_manager_v1
    #[cfg(feature = "protocol-xwayland_keyboard_grab_unstable_v1")]
    ZwpXwaylandKeyboardGrabManagerV1,
    /// zwp_xwayland_keyboard_grab_v1
    #[cfg(feature = "protocol-xwayland_keyboard_grab_unstable_v1")]
    ZwpXwaylandKeyboardGrabV1,
    /// xwayland_shell_v1
    #[cfg(feature = "protocol-xwayland_shell_v1")]
    XwaylandShellV1,
    /// xwayland_surface_v1
    #[cfg(feature = "protocol-xwayland_shell_v1")]
    XwaylandSurfaceV1,
    /// xx_zone_item_v1
    #[cfg(feature = "protocol-xx_zones_v1")]
    XxZoneItemV1,
    /// xx_zone_manager_v1
    #[cfg(feature = "protocol-xx_zones_v1")]
    XxZoneManagerV1,
    /// xx_zone_v1
    #[cfg(feature = "protocol-xx_zones_v1")]
    XxZoneV1,
    /// zwp_linux_buffer_release_v1
    #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
    ZwpLinuxBufferReleaseV1,
    /// zwp_linux_explicit_synchronization_v1
    #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
    ZwpLinuxExplicitSynchronizationV1,
    /// zwp_linux_surface_synchronization_v1
    #[cfg(feature = "protocol-zwp_linux_explicit_synchronization_unstable_v1")]
    ZwpLinuxSurfaceSynchronizationV1,
    /// zwlr_data_control_device_v1
    #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
    ZwlrDataControlDeviceV1,
    /// zwlr_data_control_manager_v1
    #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
    ZwlrDataControlManagerV1,
    /// zwlr_data_control_offer_v1
    #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
    ZwlrDataControlOfferV1,
    /// zwlr_data_control_source_v1
    #[cfg(feature = "protocol-wlr_data_control_unstable_v1")]
    ZwlrDataControlSourceV1,
    /// zwlr_export_dmabuf_frame_v1
    #[cfg(feature = "protocol-wlr_export_dmabuf_unstable_v1")]
    ZwlrExportDmabufFrameV1,
    /// zwlr_export_dmabuf_manager_v1
    #[cfg(feature = "protocol-wlr_export_dmabuf_unstable_v1")]
    ZwlrExportDmabufManagerV1,
    /// zwlr_foreign_toplevel_handle_v1
    #[cfg(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1")]
    ZwlrForeignToplevelHandleV1,
    /// zwlr_foreign_toplevel_manager_v1
    #[cfg(feature = "protocol-wlr_foreign_toplevel_management_unstable_v1")]
    ZwlrForeignToplevelManagerV1,
    /// zwlr_gamma_control_manager_v1
    #[cfg(feature = "protocol-wlr_gamma_control_unstable_v1")]
    ZwlrGammaControlManagerV1,
    /// zwlr_gamma_control_v1
    #[cfg(feature = "protocol-wlr_gamma_control_unstable_v1")]
    ZwlrGammaControlV1,
    /// zwlr_input_inhibit_manager_v1
    #[cfg(feature = "protocol-wlr_input_inhibit_unstable_v1")]
    ZwlrInputInhibitManagerV1,
    /// zwlr_input_inhibitor_v1
    #[cfg(feature = "protocol-wlr_input_inhibit_unstable_v1")]
    ZwlrInputInhibitorV1,
    /// zwlr_layer_shell_v1
    #[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")]
    ZwlrLayerShellV1,
    /// zwlr_layer_surface_v1
    #[cfg(feature = "protocol-wlr_layer_shell_unstable_v1")]
    ZwlrLayerSurfaceV1,
    /// zwlr_output_configuration_head_v1
    #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
    ZwlrOutputConfigurationHeadV1,
    /// zwlr_output_configuration_v1
    #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
    ZwlrOutputConfigurationV1,
    /// zwlr_output_head_v1
    #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
    ZwlrOutputHeadV1,
    /// zwlr_output_manager_v1
    #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
    ZwlrOutputManagerV1,
    /// zwlr_output_mode_v1
    #[cfg(feature = "protocol-wlr_output_management_unstable_v1")]
    ZwlrOutputModeV1,
    /// zwlr_output_power_manager_v1
    #[cfg(feature = "protocol-wlr_output_power_management_unstable_v1")]
    ZwlrOutputPowerManagerV1,
    /// zwlr_output_power_v1
    #[cfg(feature = "protocol-wlr_output_power_management_unstable_v1")]
    ZwlrOutputPowerV1,
    /// zwlr_screencopy_frame_v1
    #[cfg(feature = "protocol-wlr_screencopy_unstable_v1")]
    ZwlrScreencopyFrameV1,
    /// zwlr_screencopy_manager_v1
    #[cfg(feature = "protocol-wlr_screencopy_unstable_v1")]
    ZwlrScreencopyManagerV1,
    /// zwlr_virtual_pointer_manager_v1
    #[cfg(feature = "protocol-wlr_virtual_pointer_unstable_v1")]
    ZwlrVirtualPointerManagerV1,
    /// zwlr_virtual_pointer_v1
    #[cfg(feature = "protocol-wlr_virtual_pointer_unstable_v1")]
    ZwlrVirtualPointerV1,
    /// wlproxy_sync_v1
    #[cfg(feature = "protocol-wlproxy_sync_v1")]
    WlproxySyncV1,
    /// wlproxy_test
    #[cfg(test)]
    WlproxyTest,
    /// wlproxy_test_array_echo
    #[cfg(test)]
    WlproxyTestArrayEcho,
    /// wlproxy_test_dummy
    #[cfg(test)]
    WlproxyTestDummy,
    /// wlproxy_test_fd_echo
    #[cfg(test)]
    WlproxyTestFdEcho,
    /// wlproxy_test_hops
    #[cfg(test)]
    WlproxyTestHops,
    /// wlproxy_test_non_forward
    #[cfg(test)]
    WlproxyTestNonForward,
    /// wlproxy_test_object_echo
    #[cfg(test)]
    WlproxyTestObjectEcho,
    /// wlproxy_test_server_sent
    #[cfg(test)]
    WlproxyTestServerSent,
    /// river_input_device_v1
    #[cfg(feature = "protocol-river_input_management_v1")]
    RiverInputDeviceV1,
    /// river_input_manager_v1
    #[cfg(feature = "protocol-river_input_management_v1")]
    RiverInputManagerV1,
    /// river_layer_shell_output_v1
    #[cfg(feature = "protocol-river_layer_shell_v1")]
    RiverLayerShellOutputV1,
    /// river_layer_shell_seat_v1
    #[cfg(feature = "protocol-river_layer_shell_v1")]
    RiverLayerShellSeatV1,
    /// river_layer_shell_v1
    #[cfg(feature = "protocol-river_layer_shell_v1")]
    RiverLayerShellV1,
    /// river_libinput_accel_config_v1
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    RiverLibinputAccelConfigV1,
    /// river_libinput_config_v1
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    RiverLibinputConfigV1,
    /// river_libinput_device_v1
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    RiverLibinputDeviceV1,
    /// river_libinput_result_v1
    #[cfg(feature = "protocol-river_libinput_config_v1")]
    RiverLibinputResultV1,
    /// river_decoration_v1
    #[cfg(feature = "protocol-river_window_management_v1")]
    RiverDecorationV1,
    /// river_node_v1
    #[cfg(feature = "protocol-river_window_management_v1")]
    RiverNodeV1,
    /// river_output_v1
    #[cfg(feature = "protocol-river_window_management_v1")]
    RiverOutputV1,
    /// river_pointer_binding_v1
    #[cfg(feature = "protocol-river_window_management_v1")]
    RiverPointerBindingV1,
    /// river_seat_v1
    #[cfg(feature = "protocol-river_window_management_v1")]
    RiverSeatV1,
    /// river_shell_surface_v1
    #[cfg(feature = "protocol-river_window_management_v1")]
    RiverShellSurfaceV1,
    /// river_window_manager_v1
    #[cfg(feature = "protocol-river_window_management_v1")]
    RiverWindowManagerV1,
    /// river_window_v1
    #[cfg(feature = "protocol-river_window_management_v1")]
    RiverWindowV1,
    /// river_xkb_binding_v1
    #[cfg(feature = "protocol-river_xkb_bindings_v1")]
    RiverXkbBindingV1,
    /// river_xkb_bindings_seat_v1
    #[cfg(feature = "protocol-river_xkb_bindings_v1")]
    RiverXkbBindingsSeatV1,
    /// river_xkb_bindings_v1
    #[cfg(feature = "protocol-river_xkb_bindings_v1")]
    RiverXkbBindingsV1,
    /// river_xkb_config_v1
    #[cfg(feature = "protocol-river_xkb_config_v1")]
    RiverXkbConfigV1,
    /// river_xkb_keyboard_v1
    #[cfg(feature = "protocol-river_xkb_config_v1")]
    RiverXkbKeyboardV1,
    /// river_xkb_keymap_v1
    #[cfg(feature = "protocol-river_xkb_config_v1")]
    RiverXkbKeymapV1,
    /// ivi_application
    #[cfg(feature = "protocol-ivi_application")]
    IviApplication,
    /// ivi_surface
    #[cfg(feature = "protocol-ivi_application")]
    IviSurface,
    /// ivi_hmi_controller
    #[cfg(feature = "protocol-ivi_hmi_controller")]
    IviHmiController,
    /// weston_content_protection
    #[cfg(feature = "protocol-weston_content_protection")]
    WestonContentProtection,
    /// weston_protected_surface
    #[cfg(feature = "protocol-weston_content_protection")]
    WestonProtectedSurface,
    /// weston_debug_stream_v1
    #[cfg(feature = "protocol-weston_debug")]
    WestonDebugStreamV1,
    /// weston_debug_v1
    #[cfg(feature = "protocol-weston_debug")]
    WestonDebugV1,
    /// weston_desktop_shell
    #[cfg(feature = "protocol-weston_desktop")]
    WestonDesktopShell,
    /// weston_screensaver
    #[cfg(feature = "protocol-weston_desktop")]
    WestonScreensaver,
    /// weston_direct_display_v1
    #[cfg(feature = "protocol-weston_direct_display")]
    WestonDirectDisplayV1,
    /// weston_capture_source_v1
    #[cfg(feature = "protocol-weston_output_capture")]
    WestonCaptureSourceV1,
    /// weston_capture_v1
    #[cfg(feature = "protocol-weston_output_capture")]
    WestonCaptureV1,
    /// weston_test
    #[cfg(feature = "protocol-weston_test")]
    WestonTest,
    /// weston_test_runner
    #[cfg(feature = "protocol-weston_test")]
    WestonTestRunner,
    /// weston_touch_calibration
    #[cfg(feature = "protocol-weston_touch_calibration")]
    WestonTouchCalibration,
    /// weston_touch_calibrator
    #[cfg(feature = "protocol-weston_touch_calibration")]
    WestonTouchCalibrator,
    /// weston_touch_coordinate
    #[cfg(feature = "protocol-weston_touch_calibration")]
    WestonTouchCoordinate,
    /// cosmic_a11y_manager_v1
    #[cfg(feature = "protocol-cosmic_a11y_v1")]
    CosmicA11yManagerV1,
    /// cosmic_corner_radius_manager_v1
    #[cfg(feature = "protocol-cosmic_corner_radius_v1")]
    CosmicCornerRadiusManagerV1,
    /// cosmic_corner_radius_toplevel_v1
    #[cfg(feature = "protocol-cosmic_corner_radius_v1")]
    CosmicCornerRadiusToplevelV1,
    /// zcosmic_workspace_image_capture_source_manager_v1
    #[cfg(feature = "protocol-cosmic_image_source_unstable_v1")]
    ZcosmicWorkspaceImageCaptureSourceManagerV1,
    /// zcosmic_output_configuration_head_v1
    #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
    ZcosmicOutputConfigurationHeadV1,
    /// zcosmic_output_configuration_v1
    #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
    ZcosmicOutputConfigurationV1,
    /// zcosmic_output_head_v1
    #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
    ZcosmicOutputHeadV1,
    /// zcosmic_output_manager_v1
    #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
    ZcosmicOutputManagerV1,
    /// zcosmic_overlap_notification_v1
    #[cfg(feature = "protocol-cosmic_overlap_notify_unstable_v1")]
    ZcosmicOverlapNotificationV1,
    /// zcosmic_overlap_notify_v1
    #[cfg(feature = "protocol-cosmic_overlap_notify_unstable_v1")]
    ZcosmicOverlapNotifyV1,
    /// zcosmic_workspace_handle_v2
    #[cfg(feature = "protocol-cosmic_workspace_unstable_v2")]
    ZcosmicWorkspaceHandleV2,
    /// zcosmic_workspace_manager_v2
    #[cfg(feature = "protocol-cosmic_workspace_unstable_v2")]
    ZcosmicWorkspaceManagerV2,
}

impl ObjectInterface {
    pub const fn name(self) -> &'static str {
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
            #[cfg(feature = "protocol-jay_popup_ext_v1")]
            Self::JayPopupExtManagerV1 => "jay_popup_ext_manager_v1",
            #[cfg(feature = "protocol-jay_popup_ext_v1")]
            Self::JayPopupExtV1 => "jay_popup_ext_v1",
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
            Self::WpImageDescriptionReferenceV1 => "wp_image_description_reference_v1",
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
            #[cfg(feature = "protocol-xx_zones_v1")]
            Self::XxZoneItemV1 => "xx_zone_item_v1",
            #[cfg(feature = "protocol-xx_zones_v1")]
            Self::XxZoneManagerV1 => "xx_zone_manager_v1",
            #[cfg(feature = "protocol-xx_zones_v1")]
            Self::XxZoneV1 => "xx_zone_v1",
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
            #[cfg(feature = "protocol-wlproxy_sync_v1")]
            Self::WlproxySyncV1 => "wlproxy_sync_v1",
            #[cfg(test)]
            Self::WlproxyTest => "wlproxy_test",
            #[cfg(test)]
            Self::WlproxyTestArrayEcho => "wlproxy_test_array_echo",
            #[cfg(test)]
            Self::WlproxyTestDummy => "wlproxy_test_dummy",
            #[cfg(test)]
            Self::WlproxyTestFdEcho => "wlproxy_test_fd_echo",
            #[cfg(test)]
            Self::WlproxyTestHops => "wlproxy_test_hops",
            #[cfg(test)]
            Self::WlproxyTestNonForward => "wlproxy_test_non_forward",
            #[cfg(test)]
            Self::WlproxyTestObjectEcho => "wlproxy_test_object_echo",
            #[cfg(test)]
            Self::WlproxyTestServerSent => "wlproxy_test_server_sent",
            #[cfg(feature = "protocol-river_input_management_v1")]
            Self::RiverInputDeviceV1 => "river_input_device_v1",
            #[cfg(feature = "protocol-river_input_management_v1")]
            Self::RiverInputManagerV1 => "river_input_manager_v1",
            #[cfg(feature = "protocol-river_layer_shell_v1")]
            Self::RiverLayerShellOutputV1 => "river_layer_shell_output_v1",
            #[cfg(feature = "protocol-river_layer_shell_v1")]
            Self::RiverLayerShellSeatV1 => "river_layer_shell_seat_v1",
            #[cfg(feature = "protocol-river_layer_shell_v1")]
            Self::RiverLayerShellV1 => "river_layer_shell_v1",
            #[cfg(feature = "protocol-river_libinput_config_v1")]
            Self::RiverLibinputAccelConfigV1 => "river_libinput_accel_config_v1",
            #[cfg(feature = "protocol-river_libinput_config_v1")]
            Self::RiverLibinputConfigV1 => "river_libinput_config_v1",
            #[cfg(feature = "protocol-river_libinput_config_v1")]
            Self::RiverLibinputDeviceV1 => "river_libinput_device_v1",
            #[cfg(feature = "protocol-river_libinput_config_v1")]
            Self::RiverLibinputResultV1 => "river_libinput_result_v1",
            #[cfg(feature = "protocol-river_window_management_v1")]
            Self::RiverDecorationV1 => "river_decoration_v1",
            #[cfg(feature = "protocol-river_window_management_v1")]
            Self::RiverNodeV1 => "river_node_v1",
            #[cfg(feature = "protocol-river_window_management_v1")]
            Self::RiverOutputV1 => "river_output_v1",
            #[cfg(feature = "protocol-river_window_management_v1")]
            Self::RiverPointerBindingV1 => "river_pointer_binding_v1",
            #[cfg(feature = "protocol-river_window_management_v1")]
            Self::RiverSeatV1 => "river_seat_v1",
            #[cfg(feature = "protocol-river_window_management_v1")]
            Self::RiverShellSurfaceV1 => "river_shell_surface_v1",
            #[cfg(feature = "protocol-river_window_management_v1")]
            Self::RiverWindowManagerV1 => "river_window_manager_v1",
            #[cfg(feature = "protocol-river_window_management_v1")]
            Self::RiverWindowV1 => "river_window_v1",
            #[cfg(feature = "protocol-river_xkb_bindings_v1")]
            Self::RiverXkbBindingV1 => "river_xkb_binding_v1",
            #[cfg(feature = "protocol-river_xkb_bindings_v1")]
            Self::RiverXkbBindingsSeatV1 => "river_xkb_bindings_seat_v1",
            #[cfg(feature = "protocol-river_xkb_bindings_v1")]
            Self::RiverXkbBindingsV1 => "river_xkb_bindings_v1",
            #[cfg(feature = "protocol-river_xkb_config_v1")]
            Self::RiverXkbConfigV1 => "river_xkb_config_v1",
            #[cfg(feature = "protocol-river_xkb_config_v1")]
            Self::RiverXkbKeyboardV1 => "river_xkb_keyboard_v1",
            #[cfg(feature = "protocol-river_xkb_config_v1")]
            Self::RiverXkbKeymapV1 => "river_xkb_keymap_v1",
            #[cfg(feature = "protocol-ivi_application")]
            Self::IviApplication => "ivi_application",
            #[cfg(feature = "protocol-ivi_application")]
            Self::IviSurface => "ivi_surface",
            #[cfg(feature = "protocol-ivi_hmi_controller")]
            Self::IviHmiController => "ivi_hmi_controller",
            #[cfg(feature = "protocol-weston_content_protection")]
            Self::WestonContentProtection => "weston_content_protection",
            #[cfg(feature = "protocol-weston_content_protection")]
            Self::WestonProtectedSurface => "weston_protected_surface",
            #[cfg(feature = "protocol-weston_debug")]
            Self::WestonDebugStreamV1 => "weston_debug_stream_v1",
            #[cfg(feature = "protocol-weston_debug")]
            Self::WestonDebugV1 => "weston_debug_v1",
            #[cfg(feature = "protocol-weston_desktop")]
            Self::WestonDesktopShell => "weston_desktop_shell",
            #[cfg(feature = "protocol-weston_desktop")]
            Self::WestonScreensaver => "weston_screensaver",
            #[cfg(feature = "protocol-weston_direct_display")]
            Self::WestonDirectDisplayV1 => "weston_direct_display_v1",
            #[cfg(feature = "protocol-weston_output_capture")]
            Self::WestonCaptureSourceV1 => "weston_capture_source_v1",
            #[cfg(feature = "protocol-weston_output_capture")]
            Self::WestonCaptureV1 => "weston_capture_v1",
            #[cfg(feature = "protocol-weston_test")]
            Self::WestonTest => "weston_test",
            #[cfg(feature = "protocol-weston_test")]
            Self::WestonTestRunner => "weston_test_runner",
            #[cfg(feature = "protocol-weston_touch_calibration")]
            Self::WestonTouchCalibration => "weston_touch_calibration",
            #[cfg(feature = "protocol-weston_touch_calibration")]
            Self::WestonTouchCalibrator => "weston_touch_calibrator",
            #[cfg(feature = "protocol-weston_touch_calibration")]
            Self::WestonTouchCoordinate => "weston_touch_coordinate",
            #[cfg(feature = "protocol-cosmic_a11y_v1")]
            Self::CosmicA11yManagerV1 => "cosmic_a11y_manager_v1",
            #[cfg(feature = "protocol-cosmic_corner_radius_v1")]
            Self::CosmicCornerRadiusManagerV1 => "cosmic_corner_radius_manager_v1",
            #[cfg(feature = "protocol-cosmic_corner_radius_v1")]
            Self::CosmicCornerRadiusToplevelV1 => "cosmic_corner_radius_toplevel_v1",
            #[cfg(feature = "protocol-cosmic_image_source_unstable_v1")]
            Self::ZcosmicWorkspaceImageCaptureSourceManagerV1 => "zcosmic_workspace_image_capture_source_manager_v1",
            #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
            Self::ZcosmicOutputConfigurationHeadV1 => "zcosmic_output_configuration_head_v1",
            #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
            Self::ZcosmicOutputConfigurationV1 => "zcosmic_output_configuration_v1",
            #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
            Self::ZcosmicOutputHeadV1 => "zcosmic_output_head_v1",
            #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
            Self::ZcosmicOutputManagerV1 => "zcosmic_output_manager_v1",
            #[cfg(feature = "protocol-cosmic_overlap_notify_unstable_v1")]
            Self::ZcosmicOverlapNotificationV1 => "zcosmic_overlap_notification_v1",
            #[cfg(feature = "protocol-cosmic_overlap_notify_unstable_v1")]
            Self::ZcosmicOverlapNotifyV1 => "zcosmic_overlap_notify_v1",
            #[cfg(feature = "protocol-cosmic_workspace_unstable_v2")]
            Self::ZcosmicWorkspaceHandleV2 => "zcosmic_workspace_handle_v2",
            #[cfg(feature = "protocol-cosmic_workspace_unstable_v2")]
            Self::ZcosmicWorkspaceManagerV2 => "zcosmic_workspace_manager_v2",
        }
    }

    pub const fn xml_version(self) -> u32 {
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
            #[cfg(feature = "protocol-jay_popup_ext_v1")]
            Self::JayPopupExtManagerV1 => 1,
            #[cfg(feature = "protocol-jay_popup_ext_v1")]
            Self::JayPopupExtV1 => 1,
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
            Self::WpColorManagementOutputV1 => 2,
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpColorManagementSurfaceFeedbackV1 => 2,
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpColorManagementSurfaceV1 => 2,
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpColorManagerV1 => 2,
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpImageDescriptionCreatorIccV1 => 2,
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpImageDescriptionCreatorParamsV1 => 2,
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpImageDescriptionInfoV1 => 2,
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpImageDescriptionReferenceV1 => 1,
            #[cfg(feature = "protocol-color_management_v1")]
            Self::WpImageDescriptionV1 => 2,
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
            Self::ZwpPointerGesturePinchV1 => 3,
            #[cfg(feature = "protocol-pointer_gestures_unstable_v1")]
            Self::ZwpPointerGestureSwipeV1 => 3,
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
            Self::ZxdgDecorationManagerV1 => 2,
            #[cfg(feature = "protocol-xdg_decoration_unstable_v1")]
            Self::ZxdgToplevelDecorationV1 => 2,
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
            #[cfg(feature = "protocol-xx_zones_v1")]
            Self::XxZoneItemV1 => 1,
            #[cfg(feature = "protocol-xx_zones_v1")]
            Self::XxZoneManagerV1 => 1,
            #[cfg(feature = "protocol-xx_zones_v1")]
            Self::XxZoneV1 => 1,
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
            #[cfg(feature = "protocol-wlproxy_sync_v1")]
            Self::WlproxySyncV1 => 1,
            #[cfg(test)]
            Self::WlproxyTest => 1,
            #[cfg(test)]
            Self::WlproxyTestArrayEcho => 1,
            #[cfg(test)]
            Self::WlproxyTestDummy => 1,
            #[cfg(test)]
            Self::WlproxyTestFdEcho => 1,
            #[cfg(test)]
            Self::WlproxyTestHops => 1,
            #[cfg(test)]
            Self::WlproxyTestNonForward => 1,
            #[cfg(test)]
            Self::WlproxyTestObjectEcho => 1,
            #[cfg(test)]
            Self::WlproxyTestServerSent => 1,
            #[cfg(feature = "protocol-river_input_management_v1")]
            Self::RiverInputDeviceV1 => 1,
            #[cfg(feature = "protocol-river_input_management_v1")]
            Self::RiverInputManagerV1 => 1,
            #[cfg(feature = "protocol-river_layer_shell_v1")]
            Self::RiverLayerShellOutputV1 => 1,
            #[cfg(feature = "protocol-river_layer_shell_v1")]
            Self::RiverLayerShellSeatV1 => 1,
            #[cfg(feature = "protocol-river_layer_shell_v1")]
            Self::RiverLayerShellV1 => 1,
            #[cfg(feature = "protocol-river_libinput_config_v1")]
            Self::RiverLibinputAccelConfigV1 => 1,
            #[cfg(feature = "protocol-river_libinput_config_v1")]
            Self::RiverLibinputConfigV1 => 1,
            #[cfg(feature = "protocol-river_libinput_config_v1")]
            Self::RiverLibinputDeviceV1 => 1,
            #[cfg(feature = "protocol-river_libinput_config_v1")]
            Self::RiverLibinputResultV1 => 1,
            #[cfg(feature = "protocol-river_window_management_v1")]
            Self::RiverDecorationV1 => 3,
            #[cfg(feature = "protocol-river_window_management_v1")]
            Self::RiverNodeV1 => 3,
            #[cfg(feature = "protocol-river_window_management_v1")]
            Self::RiverOutputV1 => 3,
            #[cfg(feature = "protocol-river_window_management_v1")]
            Self::RiverPointerBindingV1 => 3,
            #[cfg(feature = "protocol-river_window_management_v1")]
            Self::RiverSeatV1 => 3,
            #[cfg(feature = "protocol-river_window_management_v1")]
            Self::RiverShellSurfaceV1 => 3,
            #[cfg(feature = "protocol-river_window_management_v1")]
            Self::RiverWindowManagerV1 => 3,
            #[cfg(feature = "protocol-river_window_management_v1")]
            Self::RiverWindowV1 => 3,
            #[cfg(feature = "protocol-river_xkb_bindings_v1")]
            Self::RiverXkbBindingV1 => 2,
            #[cfg(feature = "protocol-river_xkb_bindings_v1")]
            Self::RiverXkbBindingsSeatV1 => 2,
            #[cfg(feature = "protocol-river_xkb_bindings_v1")]
            Self::RiverXkbBindingsV1 => 2,
            #[cfg(feature = "protocol-river_xkb_config_v1")]
            Self::RiverXkbConfigV1 => 1,
            #[cfg(feature = "protocol-river_xkb_config_v1")]
            Self::RiverXkbKeyboardV1 => 1,
            #[cfg(feature = "protocol-river_xkb_config_v1")]
            Self::RiverXkbKeymapV1 => 1,
            #[cfg(feature = "protocol-ivi_application")]
            Self::IviApplication => 1,
            #[cfg(feature = "protocol-ivi_application")]
            Self::IviSurface => 1,
            #[cfg(feature = "protocol-ivi_hmi_controller")]
            Self::IviHmiController => 1,
            #[cfg(feature = "protocol-weston_content_protection")]
            Self::WestonContentProtection => 1,
            #[cfg(feature = "protocol-weston_content_protection")]
            Self::WestonProtectedSurface => 1,
            #[cfg(feature = "protocol-weston_debug")]
            Self::WestonDebugStreamV1 => 1,
            #[cfg(feature = "protocol-weston_debug")]
            Self::WestonDebugV1 => 1,
            #[cfg(feature = "protocol-weston_desktop")]
            Self::WestonDesktopShell => 1,
            #[cfg(feature = "protocol-weston_desktop")]
            Self::WestonScreensaver => 1,
            #[cfg(feature = "protocol-weston_direct_display")]
            Self::WestonDirectDisplayV1 => 1,
            #[cfg(feature = "protocol-weston_output_capture")]
            Self::WestonCaptureSourceV1 => 2,
            #[cfg(feature = "protocol-weston_output_capture")]
            Self::WestonCaptureV1 => 2,
            #[cfg(feature = "protocol-weston_test")]
            Self::WestonTest => 1,
            #[cfg(feature = "protocol-weston_test")]
            Self::WestonTestRunner => 1,
            #[cfg(feature = "protocol-weston_touch_calibration")]
            Self::WestonTouchCalibration => 1,
            #[cfg(feature = "protocol-weston_touch_calibration")]
            Self::WestonTouchCalibrator => 1,
            #[cfg(feature = "protocol-weston_touch_calibration")]
            Self::WestonTouchCoordinate => 1,
            #[cfg(feature = "protocol-cosmic_a11y_v1")]
            Self::CosmicA11yManagerV1 => 3,
            #[cfg(feature = "protocol-cosmic_corner_radius_v1")]
            Self::CosmicCornerRadiusManagerV1 => 1,
            #[cfg(feature = "protocol-cosmic_corner_radius_v1")]
            Self::CosmicCornerRadiusToplevelV1 => 1,
            #[cfg(feature = "protocol-cosmic_image_source_unstable_v1")]
            Self::ZcosmicWorkspaceImageCaptureSourceManagerV1 => 1,
            #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
            Self::ZcosmicOutputConfigurationHeadV1 => 2,
            #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
            Self::ZcosmicOutputConfigurationV1 => 1,
            #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
            Self::ZcosmicOutputHeadV1 => 3,
            #[cfg(feature = "protocol-cosmic_output_management_unstable_v1")]
            Self::ZcosmicOutputManagerV1 => 3,
            #[cfg(feature = "protocol-cosmic_overlap_notify_unstable_v1")]
            Self::ZcosmicOverlapNotificationV1 => 1,
            #[cfg(feature = "protocol-cosmic_overlap_notify_unstable_v1")]
            Self::ZcosmicOverlapNotifyV1 => 1,
            #[cfg(feature = "protocol-cosmic_workspace_unstable_v2")]
            Self::ZcosmicWorkspaceHandleV2 => 2,
            #[cfg(feature = "protocol-cosmic_workspace_unstable_v2")]
            Self::ZcosmicWorkspaceManagerV2 => 2,
        }
    }
}
