pub mod wayland;
#[cfg(feature = "wayland-protocols")]
pub mod alpha_modifier_v1;
#[cfg(feature = "wayland-protocols")]
pub mod color_management_v1;
#[cfg(feature = "wayland-protocols")]
pub mod color_representation_v1;
#[cfg(feature = "wayland-protocols")]
pub mod commit_timing_v1;
#[cfg(feature = "wayland-protocols")]
pub mod content_type_v1;
#[cfg(feature = "wayland-protocols")]
pub mod cursor_shape_v1;
#[cfg(feature = "wayland-protocols")]
pub mod drm_lease_v1;
#[cfg(feature = "wayland-protocols")]
pub mod ext_background_effect_v1;
#[cfg(feature = "wayland-protocols")]
pub mod ext_data_control_v1;
#[cfg(feature = "wayland-protocols")]
pub mod ext_foreign_toplevel_list_v1;
#[cfg(feature = "wayland-protocols")]
pub mod ext_idle_notify_v1;
#[cfg(feature = "wayland-protocols")]
pub mod ext_image_capture_source_v1;
#[cfg(feature = "wayland-protocols")]
pub mod ext_image_copy_capture_v1;
#[cfg(feature = "wayland-protocols")]
pub mod ext_session_lock_v1;
#[cfg(feature = "wayland-protocols")]
pub mod ext_transient_seat_v1;
#[cfg(feature = "wayland-protocols")]
pub mod ext_workspace_v1;
#[cfg(feature = "wayland-protocols")]
pub mod fifo_v1;
#[cfg(feature = "wayland-protocols")]
pub mod fractional_scale_v1;
#[cfg(feature = "wayland-protocols")]
pub mod fullscreen_shell_unstable_v1;
#[cfg(feature = "wayland-protocols")]
pub mod idle_inhibit_unstable_v1;
#[cfg(feature = "wayland-protocols")]
pub mod input_method_unstable_v1;
#[cfg(feature = "wayland-protocols")]
pub mod input_timestamps_unstable_v1;
#[cfg(feature = "wayland-protocols")]
pub mod keyboard_shortcuts_inhibit_unstable_v1;
#[cfg(feature = "wayland-protocols")]
pub mod linux_dmabuf_v1;
#[cfg(feature = "wayland-protocols")]
pub mod linux_drm_syncobj_v1;
#[cfg(feature = "wayland-protocols")]
pub mod zwp_linux_explicit_synchronization_unstable_v1;
#[cfg(feature = "wayland-protocols")]
pub mod pointer_constraints_unstable_v1;
#[cfg(feature = "wayland-protocols")]
pub mod pointer_gestures_unstable_v1;
#[cfg(feature = "wayland-protocols")]
pub mod pointer_warp_v1;
#[cfg(feature = "wayland-protocols")]
pub mod presentation_time;
#[cfg(feature = "wayland-protocols")]
pub mod wp_primary_selection_unstable_v1;
#[cfg(feature = "wayland-protocols")]
pub mod relative_pointer_unstable_v1;
#[cfg(feature = "wayland-protocols")]
pub mod security_context_v1;
#[cfg(feature = "wayland-protocols")]
pub mod single_pixel_buffer_v1;
#[cfg(feature = "wayland-protocols")]
pub mod tablet_v2;
#[cfg(feature = "wayland-protocols")]
pub mod tearing_control_v1;
#[cfg(feature = "wayland-protocols")]
pub mod text_input_unstable_v1;
#[cfg(feature = "wayland-protocols")]
pub mod text_input_unstable_v3;
#[cfg(feature = "wayland-protocols")]
pub mod viewporter;
#[cfg(feature = "wayland-protocols")]
pub mod xdg_activation_v1;
#[cfg(feature = "wayland-protocols")]
pub mod xdg_decoration_unstable_v1;
#[cfg(feature = "wayland-protocols")]
pub mod xdg_dialog_v1;
#[cfg(feature = "wayland-protocols")]
pub mod xdg_foreign_unstable_v2;
#[cfg(feature = "wayland-protocols")]
pub mod xdg_output_unstable_v1;
#[cfg(feature = "wayland-protocols")]
pub mod xdg_shell;
#[cfg(feature = "wayland-protocols")]
pub mod xdg_system_bell_v1;
#[cfg(feature = "wayland-protocols")]
pub mod xdg_toplevel_drag_v1;
#[cfg(feature = "wayland-protocols")]
pub mod xdg_toplevel_icon_v1;
#[cfg(feature = "wayland-protocols")]
pub mod xdg_toplevel_tag_v1;
#[cfg(feature = "wayland-protocols")]
pub mod xwayland_keyboard_grab_unstable_v1;
#[cfg(feature = "wayland-protocols")]
pub mod xwayland_shell_v1;
#[cfg(feature = "wlr-protocols")]
pub mod wlr_data_control_unstable_v1;
#[cfg(feature = "wlr-protocols")]
pub mod wlr_export_dmabuf_unstable_v1;
#[cfg(feature = "wlr-protocols")]
pub mod wlr_foreign_toplevel_management_unstable_v1;
#[cfg(feature = "wlr-protocols")]
pub mod wlr_gamma_control_unstable_v1;
#[cfg(feature = "wlr-protocols")]
pub mod wlr_input_inhibit_unstable_v1;
#[cfg(feature = "wlr-protocols")]
pub mod wlr_layer_shell_unstable_v1;
#[cfg(feature = "wlr-protocols")]
pub mod wlr_output_management_unstable_v1;
#[cfg(feature = "wlr-protocols")]
pub mod wlr_output_power_management_unstable_v1;
#[cfg(feature = "wlr-protocols")]
pub mod wlr_screencopy_unstable_v1;
#[cfg(feature = "wlr-protocols")]
pub mod wlr_virtual_pointer_unstable_v1;

#[allow(unused_imports)]
mod all_types {
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
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::alpha_modifier_v1::wp_alpha_modifier_surface_v1::WpAlphaModifierSurfaceV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::alpha_modifier_v1::wp_alpha_modifier_surface_v1::WpAlphaModifierSurfaceV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::alpha_modifier_v1::wp_alpha_modifier_v1::WpAlphaModifierV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::alpha_modifier_v1::wp_alpha_modifier_v1::WpAlphaModifierV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_management_v1::wp_color_management_output_v1::WpColorManagementOutputV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_management_v1::wp_color_management_surface_feedback_v1::WpColorManagementSurfaceFeedbackV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_management_v1::wp_color_management_surface_feedback_v1::WpColorManagementSurfaceFeedbackV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_management_v1::wp_color_management_surface_v1::WpColorManagementSurfaceV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_management_v1::wp_color_management_surface_v1::WpColorManagementSurfaceV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_management_v1::wp_color_manager_v1::WpColorManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_management_v1::wp_color_manager_v1::WpColorManagerV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_management_v1::wp_color_manager_v1::WpColorManagerV1RenderIntent;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_management_v1::wp_color_manager_v1::WpColorManagerV1Feature;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_management_v1::wp_color_manager_v1::WpColorManagerV1Primaries;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_management_v1::wp_color_manager_v1::WpColorManagerV1TransferFunction;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_management_v1::wp_image_description_creator_icc_v1::WpImageDescriptionCreatorIccV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_management_v1::wp_image_description_creator_icc_v1::WpImageDescriptionCreatorIccV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_management_v1::wp_image_description_creator_params_v1::WpImageDescriptionCreatorParamsV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_management_v1::wp_image_description_creator_params_v1::WpImageDescriptionCreatorParamsV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_management_v1::wp_image_description_info_v1::WpImageDescriptionInfoV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_management_v1::wp_image_description_v1::WpImageDescriptionV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_management_v1::wp_image_description_v1::WpImageDescriptionV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_management_v1::wp_image_description_v1::WpImageDescriptionV1Cause;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_representation_v1::wp_color_representation_manager_v1::WpColorRepresentationManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_representation_v1::wp_color_representation_manager_v1::WpColorRepresentationManagerV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::WpColorRepresentationSurfaceV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::WpColorRepresentationSurfaceV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::WpColorRepresentationSurfaceV1AlphaMode;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::WpColorRepresentationSurfaceV1Coefficients;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::WpColorRepresentationSurfaceV1Range;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::WpColorRepresentationSurfaceV1ChromaLocation;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::commit_timing_v1::wp_commit_timer_v1::WpCommitTimerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::commit_timing_v1::wp_commit_timer_v1::WpCommitTimerV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::commit_timing_v1::wp_commit_timing_manager_v1::WpCommitTimingManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::commit_timing_v1::wp_commit_timing_manager_v1::WpCommitTimingManagerV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::content_type_v1::wp_content_type_manager_v1::WpContentTypeManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::content_type_v1::wp_content_type_manager_v1::WpContentTypeManagerV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::content_type_v1::wp_content_type_v1::WpContentTypeV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::content_type_v1::wp_content_type_v1::WpContentTypeV1Type;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::cursor_shape_v1::wp_cursor_shape_device_v1::WpCursorShapeDeviceV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::cursor_shape_v1::wp_cursor_shape_device_v1::WpCursorShapeDeviceV1Shape;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::cursor_shape_v1::wp_cursor_shape_device_v1::WpCursorShapeDeviceV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::cursor_shape_v1::wp_cursor_shape_manager_v1::WpCursorShapeManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::drm_lease_v1::wp_drm_lease_connector_v1::WpDrmLeaseConnectorV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::drm_lease_v1::wp_drm_lease_device_v1::WpDrmLeaseDeviceV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::drm_lease_v1::wp_drm_lease_request_v1::WpDrmLeaseRequestV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::drm_lease_v1::wp_drm_lease_request_v1::WpDrmLeaseRequestV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::drm_lease_v1::wp_drm_lease_v1::WpDrmLeaseV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_background_effect_v1::ext_background_effect_manager_v1::ExtBackgroundEffectManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_background_effect_v1::ext_background_effect_manager_v1::ExtBackgroundEffectManagerV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_background_effect_v1::ext_background_effect_manager_v1::ExtBackgroundEffectManagerV1Capability;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_background_effect_v1::ext_background_effect_surface_v1::ExtBackgroundEffectSurfaceV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_background_effect_v1::ext_background_effect_surface_v1::ExtBackgroundEffectSurfaceV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_data_control_v1::ext_data_control_device_v1::ExtDataControlDeviceV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_data_control_v1::ext_data_control_device_v1::ExtDataControlDeviceV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_data_control_v1::ext_data_control_manager_v1::ExtDataControlManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_data_control_v1::ext_data_control_offer_v1::ExtDataControlOfferV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_data_control_v1::ext_data_control_source_v1::ExtDataControlSourceV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_data_control_v1::ext_data_control_source_v1::ExtDataControlSourceV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_foreign_toplevel_list_v1::ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_foreign_toplevel_list_v1::ext_foreign_toplevel_list_v1::ExtForeignToplevelListV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_idle_notify_v1::ext_idle_notification_v1::ExtIdleNotificationV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_idle_notify_v1::ext_idle_notifier_v1::ExtIdleNotifierV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_image_capture_source_v1::ext_foreign_toplevel_image_capture_source_manager_v1::ExtForeignToplevelImageCaptureSourceManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_image_capture_source_v1::ext_image_capture_source_v1::ExtImageCaptureSourceV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_image_capture_source_v1::ext_output_image_capture_source_manager_v1::ExtOutputImageCaptureSourceManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_cursor_session_v1::ExtImageCopyCaptureCursorSessionV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_cursor_session_v1::ExtImageCopyCaptureCursorSessionV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_frame_v1::ExtImageCopyCaptureFrameV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_frame_v1::ExtImageCopyCaptureFrameV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_frame_v1::ExtImageCopyCaptureFrameV1FailureReason;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_manager_v1::ExtImageCopyCaptureManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_manager_v1::ExtImageCopyCaptureManagerV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_manager_v1::ExtImageCopyCaptureManagerV1Options;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_session_v1::ExtImageCopyCaptureSessionV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_session_v1::ExtImageCopyCaptureSessionV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_session_lock_v1::ext_session_lock_manager_v1::ExtSessionLockManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_session_lock_v1::ext_session_lock_surface_v1::ExtSessionLockSurfaceV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_session_lock_v1::ext_session_lock_surface_v1::ExtSessionLockSurfaceV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_session_lock_v1::ext_session_lock_v1::ExtSessionLockV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_session_lock_v1::ext_session_lock_v1::ExtSessionLockV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_transient_seat_v1::ext_transient_seat_manager_v1::ExtTransientSeatManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_transient_seat_v1::ext_transient_seat_v1::ExtTransientSeatV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_workspace_v1::ext_workspace_group_handle_v1::ExtWorkspaceGroupHandleV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_workspace_v1::ext_workspace_group_handle_v1::ExtWorkspaceGroupHandleV1GroupCapabilities;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_workspace_v1::ext_workspace_handle_v1::ExtWorkspaceHandleV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_workspace_v1::ext_workspace_handle_v1::ExtWorkspaceHandleV1State;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_workspace_v1::ext_workspace_handle_v1::ExtWorkspaceHandleV1WorkspaceCapabilities;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::ext_workspace_v1::ext_workspace_manager_v1::ExtWorkspaceManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::fifo_v1::wp_fifo_manager_v1::WpFifoManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::fifo_v1::wp_fifo_manager_v1::WpFifoManagerV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::fifo_v1::wp_fifo_v1::WpFifoV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::fifo_v1::wp_fifo_v1::WpFifoV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::fractional_scale_v1::wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::fractional_scale_v1::wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::fractional_scale_v1::wp_fractional_scale_v1::WpFractionalScaleV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::fullscreen_shell_unstable_v1::zwp_fullscreen_shell_mode_feedback_v1::ZwpFullscreenShellModeFeedbackV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::fullscreen_shell_unstable_v1::zwp_fullscreen_shell_v1::ZwpFullscreenShellV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::fullscreen_shell_unstable_v1::zwp_fullscreen_shell_v1::ZwpFullscreenShellV1Capability;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::fullscreen_shell_unstable_v1::zwp_fullscreen_shell_v1::ZwpFullscreenShellV1PresentMethod;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::fullscreen_shell_unstable_v1::zwp_fullscreen_shell_v1::ZwpFullscreenShellV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::idle_inhibit_unstable_v1::zwp_idle_inhibit_manager_v1::ZwpIdleInhibitManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::idle_inhibit_unstable_v1::zwp_idle_inhibitor_v1::ZwpIdleInhibitorV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::input_method_unstable_v1::zwp_input_method_context_v1::ZwpInputMethodContextV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::input_method_unstable_v1::zwp_input_method_v1::ZwpInputMethodV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::input_method_unstable_v1::zwp_input_panel_surface_v1::ZwpInputPanelSurfaceV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::input_method_unstable_v1::zwp_input_panel_surface_v1::ZwpInputPanelSurfaceV1Position;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::input_method_unstable_v1::zwp_input_panel_v1::ZwpInputPanelV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::input_timestamps_unstable_v1::zwp_input_timestamps_manager_v1::ZwpInputTimestampsManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::input_timestamps_unstable_v1::zwp_input_timestamps_v1::ZwpInputTimestampsV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::keyboard_shortcuts_inhibit_unstable_v1::zwp_keyboard_shortcuts_inhibit_manager_v1::ZwpKeyboardShortcutsInhibitManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::keyboard_shortcuts_inhibit_unstable_v1::zwp_keyboard_shortcuts_inhibit_manager_v1::ZwpKeyboardShortcutsInhibitManagerV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::keyboard_shortcuts_inhibit_unstable_v1::zwp_keyboard_shortcuts_inhibitor_v1::ZwpKeyboardShortcutsInhibitorV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::linux_dmabuf_v1::zwp_linux_buffer_params_v1::ZwpLinuxBufferParamsV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::linux_dmabuf_v1::zwp_linux_buffer_params_v1::ZwpLinuxBufferParamsV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::linux_dmabuf_v1::zwp_linux_buffer_params_v1::ZwpLinuxBufferParamsV1Flags;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::linux_dmabuf_v1::zwp_linux_dmabuf_feedback_v1::ZwpLinuxDmabufFeedbackV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::linux_dmabuf_v1::zwp_linux_dmabuf_feedback_v1::ZwpLinuxDmabufFeedbackV1TrancheFlags;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::linux_dmabuf_v1::zwp_linux_dmabuf_v1::ZwpLinuxDmabufV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::linux_drm_syncobj_v1::wp_linux_drm_syncobj_manager_v1::WpLinuxDrmSyncobjManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::linux_drm_syncobj_v1::wp_linux_drm_syncobj_manager_v1::WpLinuxDrmSyncobjManagerV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::linux_drm_syncobj_v1::wp_linux_drm_syncobj_surface_v1::WpLinuxDrmSyncobjSurfaceV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::linux_drm_syncobj_v1::wp_linux_drm_syncobj_surface_v1::WpLinuxDrmSyncobjSurfaceV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::linux_drm_syncobj_v1::wp_linux_drm_syncobj_timeline_v1::WpLinuxDrmSyncobjTimelineV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::zwp_linux_explicit_synchronization_unstable_v1::zwp_linux_buffer_release_v1::ZwpLinuxBufferReleaseV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::zwp_linux_explicit_synchronization_unstable_v1::zwp_linux_explicit_synchronization_v1::ZwpLinuxExplicitSynchronizationV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::zwp_linux_explicit_synchronization_unstable_v1::zwp_linux_explicit_synchronization_v1::ZwpLinuxExplicitSynchronizationV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::zwp_linux_explicit_synchronization_unstable_v1::zwp_linux_surface_synchronization_v1::ZwpLinuxSurfaceSynchronizationV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::zwp_linux_explicit_synchronization_unstable_v1::zwp_linux_surface_synchronization_v1::ZwpLinuxSurfaceSynchronizationV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::pointer_constraints_unstable_v1::zwp_confined_pointer_v1::ZwpConfinedPointerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::pointer_constraints_unstable_v1::zwp_locked_pointer_v1::ZwpLockedPointerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::pointer_constraints_unstable_v1::zwp_pointer_constraints_v1::ZwpPointerConstraintsV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::pointer_constraints_unstable_v1::zwp_pointer_constraints_v1::ZwpPointerConstraintsV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::pointer_constraints_unstable_v1::zwp_pointer_constraints_v1::ZwpPointerConstraintsV1Lifetime;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::pointer_gestures_unstable_v1::zwp_pointer_gesture_hold_v1::ZwpPointerGestureHoldV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::pointer_gestures_unstable_v1::zwp_pointer_gesture_pinch_v1::ZwpPointerGesturePinchV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::pointer_gestures_unstable_v1::zwp_pointer_gesture_swipe_v1::ZwpPointerGestureSwipeV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::pointer_gestures_unstable_v1::zwp_pointer_gestures_v1::ZwpPointerGesturesV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::pointer_warp_v1::wp_pointer_warp_v1::WpPointerWarpV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::presentation_time::wp_presentation::WpPresentation;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::presentation_time::wp_presentation::WpPresentationError;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::presentation_time::wp_presentation_feedback::WpPresentationFeedback;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::presentation_time::wp_presentation_feedback::WpPresentationFeedbackKind;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::wp_primary_selection_unstable_v1::zwp_primary_selection_device_manager_v1::ZwpPrimarySelectionDeviceManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::wp_primary_selection_unstable_v1::zwp_primary_selection_device_v1::ZwpPrimarySelectionDeviceV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::wp_primary_selection_unstable_v1::zwp_primary_selection_offer_v1::ZwpPrimarySelectionOfferV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::wp_primary_selection_unstable_v1::zwp_primary_selection_source_v1::ZwpPrimarySelectionSourceV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::relative_pointer_unstable_v1::zwp_relative_pointer_manager_v1::ZwpRelativePointerManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::relative_pointer_unstable_v1::zwp_relative_pointer_v1::ZwpRelativePointerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::security_context_v1::wp_security_context_manager_v1::WpSecurityContextManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::security_context_v1::wp_security_context_manager_v1::WpSecurityContextManagerV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::security_context_v1::wp_security_context_v1::WpSecurityContextV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::security_context_v1::wp_security_context_v1::WpSecurityContextV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::single_pixel_buffer_v1::wp_single_pixel_buffer_manager_v1::WpSinglePixelBufferManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::tablet_v2::zwp_tablet_manager_v2::ZwpTabletManagerV2;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::tablet_v2::zwp_tablet_pad_dial_v2::ZwpTabletPadDialV2;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::tablet_v2::zwp_tablet_pad_group_v2::ZwpTabletPadGroupV2;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::tablet_v2::zwp_tablet_pad_ring_v2::ZwpTabletPadRingV2;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::tablet_v2::zwp_tablet_pad_ring_v2::ZwpTabletPadRingV2Source;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::tablet_v2::zwp_tablet_pad_strip_v2::ZwpTabletPadStripV2;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::tablet_v2::zwp_tablet_pad_strip_v2::ZwpTabletPadStripV2Source;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::tablet_v2::zwp_tablet_pad_v2::ZwpTabletPadV2;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::tablet_v2::zwp_tablet_pad_v2::ZwpTabletPadV2ButtonState;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::tablet_v2::zwp_tablet_seat_v2::ZwpTabletSeatV2;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::tablet_v2::zwp_tablet_tool_v2::ZwpTabletToolV2;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::tablet_v2::zwp_tablet_tool_v2::ZwpTabletToolV2Type;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::tablet_v2::zwp_tablet_tool_v2::ZwpTabletToolV2Capability;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::tablet_v2::zwp_tablet_tool_v2::ZwpTabletToolV2ButtonState;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::tablet_v2::zwp_tablet_tool_v2::ZwpTabletToolV2Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::tablet_v2::zwp_tablet_v2::ZwpTabletV2;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::tablet_v2::zwp_tablet_v2::ZwpTabletV2Bustype;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::tearing_control_v1::wp_tearing_control_manager_v1::WpTearingControlManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::tearing_control_v1::wp_tearing_control_manager_v1::WpTearingControlManagerV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::tearing_control_v1::wp_tearing_control_v1::WpTearingControlV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::tearing_control_v1::wp_tearing_control_v1::WpTearingControlV1PresentationHint;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::text_input_unstable_v1::zwp_text_input_manager_v1::ZwpTextInputManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::text_input_unstable_v1::zwp_text_input_v1::ZwpTextInputV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::text_input_unstable_v1::zwp_text_input_v1::ZwpTextInputV1ContentHint;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::text_input_unstable_v1::zwp_text_input_v1::ZwpTextInputV1ContentPurpose;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::text_input_unstable_v1::zwp_text_input_v1::ZwpTextInputV1PreeditStyle;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::text_input_unstable_v1::zwp_text_input_v1::ZwpTextInputV1TextDirection;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::text_input_unstable_v3::zwp_text_input_manager_v3::ZwpTextInputManagerV3;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::text_input_unstable_v3::zwp_text_input_v3::ZwpTextInputV3;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::text_input_unstable_v3::zwp_text_input_v3::ZwpTextInputV3ChangeCause;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::text_input_unstable_v3::zwp_text_input_v3::ZwpTextInputV3ContentHint;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::text_input_unstable_v3::zwp_text_input_v3::ZwpTextInputV3ContentPurpose;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::viewporter::wp_viewport::WpViewport;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::viewporter::wp_viewport::WpViewportError;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::viewporter::wp_viewporter::WpViewporter;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::viewporter::wp_viewporter::WpViewporterError;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_activation_v1::xdg_activation_token_v1::XdgActivationTokenV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_activation_v1::xdg_activation_token_v1::XdgActivationTokenV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_activation_v1::xdg_activation_v1::XdgActivationV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_decoration_unstable_v1::zxdg_decoration_manager_v1::ZxdgDecorationManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_decoration_unstable_v1::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_decoration_unstable_v1::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_decoration_unstable_v1::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1Mode;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_dialog_v1::xdg_dialog_v1::XdgDialogV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_dialog_v1::xdg_wm_dialog_v1::XdgWmDialogV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_dialog_v1::xdg_wm_dialog_v1::XdgWmDialogV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_exported_v2::ZxdgExportedV2;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_exporter_v2::ZxdgExporterV2;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_exporter_v2::ZxdgExporterV2Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_imported_v2::ZxdgImportedV2;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_imported_v2::ZxdgImportedV2Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_importer_v2::ZxdgImporterV2;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_output_unstable_v1::zxdg_output_manager_v1::ZxdgOutputManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_output_unstable_v1::zxdg_output_v1::ZxdgOutputV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_shell::xdg_popup::XdgPopup;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_shell::xdg_popup::XdgPopupError;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_shell::xdg_positioner::XdgPositioner;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_shell::xdg_positioner::XdgPositionerError;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_shell::xdg_positioner::XdgPositionerAnchor;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_shell::xdg_positioner::XdgPositionerGravity;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_shell::xdg_positioner::XdgPositionerConstraintAdjustment;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_shell::xdg_surface::XdgSurface;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_shell::xdg_surface::XdgSurfaceError;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_shell::xdg_toplevel::XdgToplevel;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_shell::xdg_toplevel::XdgToplevelError;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_shell::xdg_toplevel::XdgToplevelResizeEdge;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_shell::xdg_toplevel::XdgToplevelState;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_shell::xdg_toplevel::XdgToplevelWmCapabilities;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_shell::xdg_wm_base::XdgWmBase;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_shell::xdg_wm_base::XdgWmBaseError;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_system_bell_v1::xdg_system_bell_v1::XdgSystemBellV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_toplevel_drag_v1::xdg_toplevel_drag_manager_v1::XdgToplevelDragManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_toplevel_drag_v1::xdg_toplevel_drag_manager_v1::XdgToplevelDragManagerV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_toplevel_drag_v1::xdg_toplevel_drag_v1::XdgToplevelDragV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_toplevel_drag_v1::xdg_toplevel_drag_v1::XdgToplevelDragV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_toplevel_icon_v1::xdg_toplevel_icon_manager_v1::XdgToplevelIconManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_toplevel_icon_v1::xdg_toplevel_icon_v1::XdgToplevelIconV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_toplevel_icon_v1::xdg_toplevel_icon_v1::XdgToplevelIconV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xdg_toplevel_tag_v1::xdg_toplevel_tag_manager_v1::XdgToplevelTagManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xwayland_keyboard_grab_unstable_v1::zwp_xwayland_keyboard_grab_manager_v1::ZwpXwaylandKeyboardGrabManagerV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xwayland_keyboard_grab_unstable_v1::zwp_xwayland_keyboard_grab_v1::ZwpXwaylandKeyboardGrabV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xwayland_shell_v1::xwayland_shell_v1::XwaylandShellV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xwayland_shell_v1::xwayland_shell_v1::XwaylandShellV1Error;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xwayland_shell_v1::xwayland_surface_v1::XwaylandSurfaceV1;
    #[cfg(feature = "wayland-protocols")]
    pub(super) use super::xwayland_shell_v1::xwayland_surface_v1::XwaylandSurfaceV1Error;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_device_v1::ZwlrDataControlDeviceV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_device_v1::ZwlrDataControlDeviceV1Error;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_manager_v1::ZwlrDataControlManagerV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_offer_v1::ZwlrDataControlOfferV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_source_v1::ZwlrDataControlSourceV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_source_v1::ZwlrDataControlSourceV1Error;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_export_dmabuf_unstable_v1::zwlr_export_dmabuf_frame_v1::ZwlrExportDmabufFrameV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_export_dmabuf_unstable_v1::zwlr_export_dmabuf_frame_v1::ZwlrExportDmabufFrameV1Flags;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_export_dmabuf_unstable_v1::zwlr_export_dmabuf_frame_v1::ZwlrExportDmabufFrameV1CancelReason;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_export_dmabuf_unstable_v1::zwlr_export_dmabuf_manager_v1::ZwlrExportDmabufManagerV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_foreign_toplevel_management_unstable_v1::zwlr_foreign_toplevel_handle_v1::ZwlrForeignToplevelHandleV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_foreign_toplevel_management_unstable_v1::zwlr_foreign_toplevel_handle_v1::ZwlrForeignToplevelHandleV1State;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_foreign_toplevel_management_unstable_v1::zwlr_foreign_toplevel_handle_v1::ZwlrForeignToplevelHandleV1Error;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_foreign_toplevel_management_unstable_v1::zwlr_foreign_toplevel_manager_v1::ZwlrForeignToplevelManagerV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_gamma_control_unstable_v1::zwlr_gamma_control_manager_v1::ZwlrGammaControlManagerV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_gamma_control_unstable_v1::zwlr_gamma_control_v1::ZwlrGammaControlV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_gamma_control_unstable_v1::zwlr_gamma_control_v1::ZwlrGammaControlV1Error;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_input_inhibit_unstable_v1::zwlr_input_inhibit_manager_v1::ZwlrInputInhibitManagerV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_input_inhibit_unstable_v1::zwlr_input_inhibit_manager_v1::ZwlrInputInhibitManagerV1Error;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_input_inhibit_unstable_v1::zwlr_input_inhibitor_v1::ZwlrInputInhibitorV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_shell_v1::ZwlrLayerShellV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_shell_v1::ZwlrLayerShellV1Error;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_shell_v1::ZwlrLayerShellV1Layer;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_surface_v1::ZwlrLayerSurfaceV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_surface_v1::ZwlrLayerSurfaceV1KeyboardInteractivity;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_surface_v1::ZwlrLayerSurfaceV1Error;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_surface_v1::ZwlrLayerSurfaceV1Anchor;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_configuration_head_v1::ZwlrOutputConfigurationHeadV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_configuration_head_v1::ZwlrOutputConfigurationHeadV1Error;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_configuration_v1::ZwlrOutputConfigurationV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_configuration_v1::ZwlrOutputConfigurationV1Error;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_head_v1::ZwlrOutputHeadV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_head_v1::ZwlrOutputHeadV1AdaptiveSyncState;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_manager_v1::ZwlrOutputManagerV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_mode_v1::ZwlrOutputModeV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_output_power_management_unstable_v1::zwlr_output_power_manager_v1::ZwlrOutputPowerManagerV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_output_power_management_unstable_v1::zwlr_output_power_v1::ZwlrOutputPowerV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_output_power_management_unstable_v1::zwlr_output_power_v1::ZwlrOutputPowerV1Mode;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_output_power_management_unstable_v1::zwlr_output_power_v1::ZwlrOutputPowerV1Error;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_screencopy_unstable_v1::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_screencopy_unstable_v1::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1Error;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_screencopy_unstable_v1::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1Flags;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_screencopy_unstable_v1::zwlr_screencopy_manager_v1::ZwlrScreencopyManagerV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_virtual_pointer_unstable_v1::zwlr_virtual_pointer_manager_v1::ZwlrVirtualPointerManagerV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_virtual_pointer_unstable_v1::zwlr_virtual_pointer_v1::ZwlrVirtualPointerV1;
    #[cfg(feature = "wlr-protocols")]
    pub(super) use super::wlr_virtual_pointer_unstable_v1::zwlr_virtual_pointer_v1::ZwlrVirtualPointerV1Error;

    use crate::generated_helper::prelude::*;

    pub(super) fn create_proxy_for_interface(state: &Rc<State>, interface: &str, version: u32) -> Result<Rc<dyn Proxy>, ObjectError> {
        proxy_interface(interface)
            .ok_or(ObjectError::UnsupportedInterface(interface.to_string()))
            .and_then(|i| i.create_proxy(state, version))
    }

    pub(super) fn proxy_interface(interface: &str) -> Option<ProxyInterface> {
        static INTERFACES: phf::Map<&'static str, Option<ProxyInterface>> = phf::phf_map! {
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
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpAlphaModifierSurfaceV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_alpha_modifier_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpAlphaModifierV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_color_management_output_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpColorManagementOutputV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_color_management_surface_feedback_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpColorManagementSurfaceFeedbackV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_color_management_surface_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpColorManagementSurfaceV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_color_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpColorManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_image_description_creator_icc_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpImageDescriptionCreatorIccV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_image_description_creator_params_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpImageDescriptionCreatorParamsV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_image_description_info_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpImageDescriptionInfoV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_image_description_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpImageDescriptionV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_color_representation_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpColorRepresentationManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_color_representation_surface_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpColorRepresentationSurfaceV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_commit_timer_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpCommitTimerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_commit_timing_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpCommitTimingManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_content_type_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpContentTypeManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_content_type_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpContentTypeV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_cursor_shape_device_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpCursorShapeDeviceV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_cursor_shape_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpCursorShapeManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_drm_lease_connector_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpDrmLeaseConnectorV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_drm_lease_device_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpDrmLeaseDeviceV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_drm_lease_request_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpDrmLeaseRequestV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_drm_lease_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpDrmLeaseV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_background_effect_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtBackgroundEffectManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_background_effect_surface_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtBackgroundEffectSurfaceV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_data_control_device_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtDataControlDeviceV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_data_control_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtDataControlManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_data_control_offer_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtDataControlOfferV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_data_control_source_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtDataControlSourceV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_foreign_toplevel_handle_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtForeignToplevelHandleV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_foreign_toplevel_list_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtForeignToplevelListV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_idle_notification_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtIdleNotificationV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_idle_notifier_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtIdleNotifierV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_foreign_toplevel_image_capture_source_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtForeignToplevelImageCaptureSourceManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_image_capture_source_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtImageCaptureSourceV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_output_image_capture_source_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtOutputImageCaptureSourceManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_image_copy_capture_cursor_session_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtImageCopyCaptureCursorSessionV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_image_copy_capture_frame_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtImageCopyCaptureFrameV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_image_copy_capture_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtImageCopyCaptureManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_image_copy_capture_session_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtImageCopyCaptureSessionV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_session_lock_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtSessionLockManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_session_lock_surface_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtSessionLockSurfaceV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_session_lock_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtSessionLockV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_transient_seat_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtTransientSeatManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_transient_seat_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtTransientSeatV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_workspace_group_handle_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtWorkspaceGroupHandleV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_workspace_handle_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtWorkspaceHandleV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "ext_workspace_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ExtWorkspaceManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_fifo_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpFifoManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_fifo_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpFifoV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_fractional_scale_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpFractionalScaleManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_fractional_scale_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpFractionalScaleV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_fullscreen_shell_mode_feedback_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpFullscreenShellModeFeedbackV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_fullscreen_shell_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpFullscreenShellV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_idle_inhibit_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpIdleInhibitManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_idle_inhibitor_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpIdleInhibitorV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_input_method_context_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpInputMethodContextV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_input_method_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpInputMethodV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_input_panel_surface_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpInputPanelSurfaceV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_input_panel_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpInputPanelV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_input_timestamps_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpInputTimestampsManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_input_timestamps_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpInputTimestampsV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_keyboard_shortcuts_inhibit_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpKeyboardShortcutsInhibitManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_keyboard_shortcuts_inhibitor_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpKeyboardShortcutsInhibitorV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_linux_buffer_params_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpLinuxBufferParamsV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_linux_dmabuf_feedback_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpLinuxDmabufFeedbackV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_linux_dmabuf_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpLinuxDmabufV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_linux_drm_syncobj_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpLinuxDrmSyncobjManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_linux_drm_syncobj_surface_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpLinuxDrmSyncobjSurfaceV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_linux_drm_syncobj_timeline_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpLinuxDrmSyncobjTimelineV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_linux_buffer_release_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpLinuxBufferReleaseV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_linux_explicit_synchronization_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpLinuxExplicitSynchronizationV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_linux_surface_synchronization_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpLinuxSurfaceSynchronizationV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_confined_pointer_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpConfinedPointerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_locked_pointer_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpLockedPointerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_pointer_constraints_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpPointerConstraintsV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_pointer_gesture_hold_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpPointerGestureHoldV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_pointer_gesture_pinch_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpPointerGesturePinchV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_pointer_gesture_swipe_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpPointerGestureSwipeV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_pointer_gestures_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpPointerGesturesV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_pointer_warp_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpPointerWarpV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_presentation" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpPresentation) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_presentation_feedback" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpPresentationFeedback) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_primary_selection_device_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpPrimarySelectionDeviceManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_primary_selection_device_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpPrimarySelectionDeviceV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_primary_selection_offer_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpPrimarySelectionOfferV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_primary_selection_source_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpPrimarySelectionSourceV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_relative_pointer_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpRelativePointerManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_relative_pointer_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpRelativePointerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_security_context_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpSecurityContextManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_security_context_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpSecurityContextV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_single_pixel_buffer_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpSinglePixelBufferManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_tablet_manager_v2" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpTabletManagerV2) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_tablet_pad_dial_v2" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpTabletPadDialV2) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_tablet_pad_group_v2" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpTabletPadGroupV2) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_tablet_pad_ring_v2" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpTabletPadRingV2) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_tablet_pad_strip_v2" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpTabletPadStripV2) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_tablet_pad_v2" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpTabletPadV2) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_tablet_seat_v2" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpTabletSeatV2) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_tablet_tool_v2" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpTabletToolV2) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_tablet_v2" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpTabletV2) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_tearing_control_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpTearingControlManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_tearing_control_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpTearingControlV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_text_input_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpTextInputManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_text_input_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpTextInputV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_text_input_manager_v3" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpTextInputManagerV3) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_text_input_v3" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpTextInputV3) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_viewport" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpViewport) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "wp_viewporter" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::WpViewporter) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "xdg_activation_token_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::XdgActivationTokenV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "xdg_activation_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::XdgActivationV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zxdg_decoration_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZxdgDecorationManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zxdg_toplevel_decoration_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZxdgToplevelDecorationV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "xdg_dialog_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::XdgDialogV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "xdg_wm_dialog_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::XdgWmDialogV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zxdg_exported_v2" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZxdgExportedV2) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zxdg_exporter_v2" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZxdgExporterV2) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zxdg_imported_v2" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZxdgImportedV2) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zxdg_importer_v2" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZxdgImporterV2) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zxdg_output_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZxdgOutputManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zxdg_output_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZxdgOutputV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "xdg_popup" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::XdgPopup) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "xdg_positioner" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::XdgPositioner) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "xdg_surface" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::XdgSurface) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "xdg_toplevel" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::XdgToplevel) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "xdg_wm_base" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::XdgWmBase) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "xdg_system_bell_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::XdgSystemBellV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "xdg_toplevel_drag_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::XdgToplevelDragManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "xdg_toplevel_drag_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::XdgToplevelDragV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "xdg_toplevel_icon_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::XdgToplevelIconManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "xdg_toplevel_icon_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::XdgToplevelIconV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "xdg_toplevel_tag_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::XdgToplevelTagManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_xwayland_keyboard_grab_manager_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpXwaylandKeyboardGrabManagerV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwp_xwayland_keyboard_grab_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::ZwpXwaylandKeyboardGrabV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "xwayland_shell_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::XwaylandShellV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "xwayland_surface_v1" => {
                #[cfg(feature = "wayland-protocols")] { Some(ProxyInterface::XwaylandSurfaceV1) }
                #[cfg(not(feature = "wayland-protocols"))] { None }
            },
            "zwlr_data_control_device_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrDataControlDeviceV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_data_control_manager_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrDataControlManagerV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_data_control_offer_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrDataControlOfferV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_data_control_source_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrDataControlSourceV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_export_dmabuf_frame_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrExportDmabufFrameV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_export_dmabuf_manager_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrExportDmabufManagerV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_foreign_toplevel_handle_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrForeignToplevelHandleV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_foreign_toplevel_manager_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrForeignToplevelManagerV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_gamma_control_manager_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrGammaControlManagerV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_gamma_control_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrGammaControlV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_input_inhibit_manager_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrInputInhibitManagerV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_input_inhibitor_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrInputInhibitorV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_layer_shell_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrLayerShellV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_layer_surface_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrLayerSurfaceV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_output_configuration_head_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrOutputConfigurationHeadV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_output_configuration_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrOutputConfigurationV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_output_head_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrOutputHeadV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_output_manager_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrOutputManagerV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_output_mode_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrOutputModeV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_output_power_manager_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrOutputPowerManagerV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_output_power_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrOutputPowerV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_screencopy_frame_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrScreencopyFrameV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_screencopy_manager_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrScreencopyManagerV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_virtual_pointer_manager_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrVirtualPointerManagerV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
            "zwlr_virtual_pointer_v1" => {
                #[cfg(feature = "wlr-protocols")] { Some(ProxyInterface::ZwlrVirtualPointerV1) }
                #[cfg(not(feature = "wlr-protocols"))] { None }
            },
        };
        INTERFACES.get(interface).copied().flatten()
    }

    impl ProxyInterface {
        fn create_proxy(self, state: &Rc<State>, version: u32) -> Result<Rc<dyn Proxy>, ObjectError> {
            match self {
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
                #[cfg(feature = "wayland-protocols")]
                Self::WpAlphaModifierSurfaceV1 => {
                    if version > WpAlphaModifierSurfaceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpAlphaModifierSurfaceV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpAlphaModifierV1 => {
                    if version > WpAlphaModifierV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpAlphaModifierV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpColorManagementOutputV1 => {
                    if version > WpColorManagementOutputV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpColorManagementOutputV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpColorManagementSurfaceFeedbackV1 => {
                    if version > WpColorManagementSurfaceFeedbackV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpColorManagementSurfaceFeedbackV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpColorManagementSurfaceV1 => {
                    if version > WpColorManagementSurfaceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpColorManagementSurfaceV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpColorManagerV1 => {
                    if version > WpColorManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpColorManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpImageDescriptionCreatorIccV1 => {
                    if version > WpImageDescriptionCreatorIccV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpImageDescriptionCreatorIccV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpImageDescriptionCreatorParamsV1 => {
                    if version > WpImageDescriptionCreatorParamsV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpImageDescriptionCreatorParamsV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpImageDescriptionInfoV1 => {
                    if version > WpImageDescriptionInfoV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpImageDescriptionInfoV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpImageDescriptionV1 => {
                    if version > WpImageDescriptionV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpImageDescriptionV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpColorRepresentationManagerV1 => {
                    if version > WpColorRepresentationManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpColorRepresentationManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpColorRepresentationSurfaceV1 => {
                    if version > WpColorRepresentationSurfaceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpColorRepresentationSurfaceV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpCommitTimerV1 => {
                    if version > WpCommitTimerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpCommitTimerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpCommitTimingManagerV1 => {
                    if version > WpCommitTimingManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpCommitTimingManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpContentTypeManagerV1 => {
                    if version > WpContentTypeManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpContentTypeManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpContentTypeV1 => {
                    if version > WpContentTypeV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpContentTypeV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpCursorShapeDeviceV1 => {
                    if version > WpCursorShapeDeviceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpCursorShapeDeviceV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpCursorShapeManagerV1 => {
                    if version > WpCursorShapeManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpCursorShapeManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpDrmLeaseConnectorV1 => {
                    if version > WpDrmLeaseConnectorV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpDrmLeaseConnectorV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpDrmLeaseDeviceV1 => {
                    if version > WpDrmLeaseDeviceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpDrmLeaseDeviceV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpDrmLeaseRequestV1 => {
                    if version > WpDrmLeaseRequestV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpDrmLeaseRequestV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpDrmLeaseV1 => {
                    if version > WpDrmLeaseV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpDrmLeaseV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtBackgroundEffectManagerV1 => {
                    if version > ExtBackgroundEffectManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtBackgroundEffectManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtBackgroundEffectSurfaceV1 => {
                    if version > ExtBackgroundEffectSurfaceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtBackgroundEffectSurfaceV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtDataControlDeviceV1 => {
                    if version > ExtDataControlDeviceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtDataControlDeviceV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtDataControlManagerV1 => {
                    if version > ExtDataControlManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtDataControlManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtDataControlOfferV1 => {
                    if version > ExtDataControlOfferV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtDataControlOfferV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtDataControlSourceV1 => {
                    if version > ExtDataControlSourceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtDataControlSourceV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtForeignToplevelHandleV1 => {
                    if version > ExtForeignToplevelHandleV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtForeignToplevelHandleV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtForeignToplevelListV1 => {
                    if version > ExtForeignToplevelListV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtForeignToplevelListV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtIdleNotificationV1 => {
                    if version > ExtIdleNotificationV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtIdleNotificationV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtIdleNotifierV1 => {
                    if version > ExtIdleNotifierV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtIdleNotifierV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtForeignToplevelImageCaptureSourceManagerV1 => {
                    if version > ExtForeignToplevelImageCaptureSourceManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtForeignToplevelImageCaptureSourceManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtImageCaptureSourceV1 => {
                    if version > ExtImageCaptureSourceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtImageCaptureSourceV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtOutputImageCaptureSourceManagerV1 => {
                    if version > ExtOutputImageCaptureSourceManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtOutputImageCaptureSourceManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtImageCopyCaptureCursorSessionV1 => {
                    if version > ExtImageCopyCaptureCursorSessionV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtImageCopyCaptureCursorSessionV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtImageCopyCaptureFrameV1 => {
                    if version > ExtImageCopyCaptureFrameV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtImageCopyCaptureFrameV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtImageCopyCaptureManagerV1 => {
                    if version > ExtImageCopyCaptureManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtImageCopyCaptureManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtImageCopyCaptureSessionV1 => {
                    if version > ExtImageCopyCaptureSessionV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtImageCopyCaptureSessionV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtSessionLockManagerV1 => {
                    if version > ExtSessionLockManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtSessionLockManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtSessionLockSurfaceV1 => {
                    if version > ExtSessionLockSurfaceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtSessionLockSurfaceV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtSessionLockV1 => {
                    if version > ExtSessionLockV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtSessionLockV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtTransientSeatManagerV1 => {
                    if version > ExtTransientSeatManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtTransientSeatManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtTransientSeatV1 => {
                    if version > ExtTransientSeatV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtTransientSeatV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtWorkspaceGroupHandleV1 => {
                    if version > ExtWorkspaceGroupHandleV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtWorkspaceGroupHandleV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtWorkspaceHandleV1 => {
                    if version > ExtWorkspaceHandleV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtWorkspaceHandleV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ExtWorkspaceManagerV1 => {
                    if version > ExtWorkspaceManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ExtWorkspaceManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpFifoManagerV1 => {
                    if version > WpFifoManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpFifoManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpFifoV1 => {
                    if version > WpFifoV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpFifoV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpFractionalScaleManagerV1 => {
                    if version > WpFractionalScaleManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpFractionalScaleManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpFractionalScaleV1 => {
                    if version > WpFractionalScaleV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpFractionalScaleV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpFullscreenShellModeFeedbackV1 => {
                    if version > ZwpFullscreenShellModeFeedbackV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpFullscreenShellModeFeedbackV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpFullscreenShellV1 => {
                    if version > ZwpFullscreenShellV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpFullscreenShellV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpIdleInhibitManagerV1 => {
                    if version > ZwpIdleInhibitManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpIdleInhibitManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpIdleInhibitorV1 => {
                    if version > ZwpIdleInhibitorV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpIdleInhibitorV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpInputMethodContextV1 => {
                    if version > ZwpInputMethodContextV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpInputMethodContextV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpInputMethodV1 => {
                    if version > ZwpInputMethodV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpInputMethodV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpInputPanelSurfaceV1 => {
                    if version > ZwpInputPanelSurfaceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpInputPanelSurfaceV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpInputPanelV1 => {
                    if version > ZwpInputPanelV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpInputPanelV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpInputTimestampsManagerV1 => {
                    if version > ZwpInputTimestampsManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpInputTimestampsManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpInputTimestampsV1 => {
                    if version > ZwpInputTimestampsV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpInputTimestampsV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpKeyboardShortcutsInhibitManagerV1 => {
                    if version > ZwpKeyboardShortcutsInhibitManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpKeyboardShortcutsInhibitManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpKeyboardShortcutsInhibitorV1 => {
                    if version > ZwpKeyboardShortcutsInhibitorV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpKeyboardShortcutsInhibitorV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpLinuxBufferParamsV1 => {
                    if version > ZwpLinuxBufferParamsV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpLinuxBufferParamsV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpLinuxDmabufFeedbackV1 => {
                    if version > ZwpLinuxDmabufFeedbackV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpLinuxDmabufFeedbackV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpLinuxDmabufV1 => {
                    if version > ZwpLinuxDmabufV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpLinuxDmabufV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpLinuxDrmSyncobjManagerV1 => {
                    if version > WpLinuxDrmSyncobjManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpLinuxDrmSyncobjManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpLinuxDrmSyncobjSurfaceV1 => {
                    if version > WpLinuxDrmSyncobjSurfaceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpLinuxDrmSyncobjSurfaceV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpLinuxDrmSyncobjTimelineV1 => {
                    if version > WpLinuxDrmSyncobjTimelineV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpLinuxDrmSyncobjTimelineV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpLinuxBufferReleaseV1 => {
                    if version > ZwpLinuxBufferReleaseV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpLinuxBufferReleaseV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpLinuxExplicitSynchronizationV1 => {
                    if version > ZwpLinuxExplicitSynchronizationV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpLinuxExplicitSynchronizationV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpLinuxSurfaceSynchronizationV1 => {
                    if version > ZwpLinuxSurfaceSynchronizationV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpLinuxSurfaceSynchronizationV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpConfinedPointerV1 => {
                    if version > ZwpConfinedPointerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpConfinedPointerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpLockedPointerV1 => {
                    if version > ZwpLockedPointerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpLockedPointerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpPointerConstraintsV1 => {
                    if version > ZwpPointerConstraintsV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpPointerConstraintsV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpPointerGestureHoldV1 => {
                    if version > ZwpPointerGestureHoldV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpPointerGestureHoldV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpPointerGesturePinchV1 => {
                    if version > ZwpPointerGesturePinchV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpPointerGesturePinchV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpPointerGestureSwipeV1 => {
                    if version > ZwpPointerGestureSwipeV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpPointerGestureSwipeV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpPointerGesturesV1 => {
                    if version > ZwpPointerGesturesV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpPointerGesturesV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpPointerWarpV1 => {
                    if version > WpPointerWarpV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpPointerWarpV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpPresentation => {
                    if version > WpPresentation::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpPresentation::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpPresentationFeedback => {
                    if version > WpPresentationFeedback::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpPresentationFeedback::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpPrimarySelectionDeviceManagerV1 => {
                    if version > ZwpPrimarySelectionDeviceManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpPrimarySelectionDeviceManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpPrimarySelectionDeviceV1 => {
                    if version > ZwpPrimarySelectionDeviceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpPrimarySelectionDeviceV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpPrimarySelectionOfferV1 => {
                    if version > ZwpPrimarySelectionOfferV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpPrimarySelectionOfferV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpPrimarySelectionSourceV1 => {
                    if version > ZwpPrimarySelectionSourceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpPrimarySelectionSourceV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpRelativePointerManagerV1 => {
                    if version > ZwpRelativePointerManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpRelativePointerManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpRelativePointerV1 => {
                    if version > ZwpRelativePointerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpRelativePointerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpSecurityContextManagerV1 => {
                    if version > WpSecurityContextManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpSecurityContextManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpSecurityContextV1 => {
                    if version > WpSecurityContextV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpSecurityContextV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpSinglePixelBufferManagerV1 => {
                    if version > WpSinglePixelBufferManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpSinglePixelBufferManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpTabletManagerV2 => {
                    if version > ZwpTabletManagerV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTabletManagerV2::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpTabletPadDialV2 => {
                    if version > ZwpTabletPadDialV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTabletPadDialV2::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpTabletPadGroupV2 => {
                    if version > ZwpTabletPadGroupV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTabletPadGroupV2::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpTabletPadRingV2 => {
                    if version > ZwpTabletPadRingV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTabletPadRingV2::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpTabletPadStripV2 => {
                    if version > ZwpTabletPadStripV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTabletPadStripV2::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpTabletPadV2 => {
                    if version > ZwpTabletPadV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTabletPadV2::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpTabletSeatV2 => {
                    if version > ZwpTabletSeatV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTabletSeatV2::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpTabletToolV2 => {
                    if version > ZwpTabletToolV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTabletToolV2::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpTabletV2 => {
                    if version > ZwpTabletV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTabletV2::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpTearingControlManagerV1 => {
                    if version > WpTearingControlManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpTearingControlManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpTearingControlV1 => {
                    if version > WpTearingControlV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpTearingControlV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpTextInputManagerV1 => {
                    if version > ZwpTextInputManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTextInputManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpTextInputV1 => {
                    if version > ZwpTextInputV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTextInputV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpTextInputManagerV3 => {
                    if version > ZwpTextInputManagerV3::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTextInputManagerV3::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpTextInputV3 => {
                    if version > ZwpTextInputV3::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpTextInputV3::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpViewport => {
                    if version > WpViewport::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpViewport::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::WpViewporter => {
                    if version > WpViewporter::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(WpViewporter::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::XdgActivationTokenV1 => {
                    if version > XdgActivationTokenV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgActivationTokenV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::XdgActivationV1 => {
                    if version > XdgActivationV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgActivationV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZxdgDecorationManagerV1 => {
                    if version > ZxdgDecorationManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZxdgDecorationManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZxdgToplevelDecorationV1 => {
                    if version > ZxdgToplevelDecorationV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZxdgToplevelDecorationV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::XdgDialogV1 => {
                    if version > XdgDialogV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgDialogV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::XdgWmDialogV1 => {
                    if version > XdgWmDialogV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgWmDialogV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZxdgExportedV2 => {
                    if version > ZxdgExportedV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZxdgExportedV2::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZxdgExporterV2 => {
                    if version > ZxdgExporterV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZxdgExporterV2::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZxdgImportedV2 => {
                    if version > ZxdgImportedV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZxdgImportedV2::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZxdgImporterV2 => {
                    if version > ZxdgImporterV2::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZxdgImporterV2::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZxdgOutputManagerV1 => {
                    if version > ZxdgOutputManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZxdgOutputManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZxdgOutputV1 => {
                    if version > ZxdgOutputV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZxdgOutputV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::XdgPopup => {
                    if version > XdgPopup::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgPopup::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::XdgPositioner => {
                    if version > XdgPositioner::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgPositioner::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::XdgSurface => {
                    if version > XdgSurface::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgSurface::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::XdgToplevel => {
                    if version > XdgToplevel::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgToplevel::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::XdgWmBase => {
                    if version > XdgWmBase::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgWmBase::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::XdgSystemBellV1 => {
                    if version > XdgSystemBellV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgSystemBellV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::XdgToplevelDragManagerV1 => {
                    if version > XdgToplevelDragManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgToplevelDragManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::XdgToplevelDragV1 => {
                    if version > XdgToplevelDragV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgToplevelDragV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::XdgToplevelIconManagerV1 => {
                    if version > XdgToplevelIconManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgToplevelIconManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::XdgToplevelIconV1 => {
                    if version > XdgToplevelIconV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgToplevelIconV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::XdgToplevelTagManagerV1 => {
                    if version > XdgToplevelTagManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XdgToplevelTagManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpXwaylandKeyboardGrabManagerV1 => {
                    if version > ZwpXwaylandKeyboardGrabManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpXwaylandKeyboardGrabManagerV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::ZwpXwaylandKeyboardGrabV1 => {
                    if version > ZwpXwaylandKeyboardGrabV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwpXwaylandKeyboardGrabV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::XwaylandShellV1 => {
                    if version > XwaylandShellV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XwaylandShellV1::new(state, version))
                }
                #[cfg(feature = "wayland-protocols")]
                Self::XwaylandSurfaceV1 => {
                    if version > XwaylandSurfaceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(XwaylandSurfaceV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrDataControlDeviceV1 => {
                    if version > ZwlrDataControlDeviceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrDataControlDeviceV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrDataControlManagerV1 => {
                    if version > ZwlrDataControlManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrDataControlManagerV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrDataControlOfferV1 => {
                    if version > ZwlrDataControlOfferV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrDataControlOfferV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrDataControlSourceV1 => {
                    if version > ZwlrDataControlSourceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrDataControlSourceV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrExportDmabufFrameV1 => {
                    if version > ZwlrExportDmabufFrameV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrExportDmabufFrameV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrExportDmabufManagerV1 => {
                    if version > ZwlrExportDmabufManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrExportDmabufManagerV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrForeignToplevelHandleV1 => {
                    if version > ZwlrForeignToplevelHandleV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrForeignToplevelHandleV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrForeignToplevelManagerV1 => {
                    if version > ZwlrForeignToplevelManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrForeignToplevelManagerV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrGammaControlManagerV1 => {
                    if version > ZwlrGammaControlManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrGammaControlManagerV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrGammaControlV1 => {
                    if version > ZwlrGammaControlV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrGammaControlV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrInputInhibitManagerV1 => {
                    if version > ZwlrInputInhibitManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrInputInhibitManagerV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrInputInhibitorV1 => {
                    if version > ZwlrInputInhibitorV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrInputInhibitorV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrLayerShellV1 => {
                    if version > ZwlrLayerShellV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrLayerShellV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrLayerSurfaceV1 => {
                    if version > ZwlrLayerSurfaceV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrLayerSurfaceV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrOutputConfigurationHeadV1 => {
                    if version > ZwlrOutputConfigurationHeadV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrOutputConfigurationHeadV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrOutputConfigurationV1 => {
                    if version > ZwlrOutputConfigurationV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrOutputConfigurationV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrOutputHeadV1 => {
                    if version > ZwlrOutputHeadV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrOutputHeadV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrOutputManagerV1 => {
                    if version > ZwlrOutputManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrOutputManagerV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrOutputModeV1 => {
                    if version > ZwlrOutputModeV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrOutputModeV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrOutputPowerManagerV1 => {
                    if version > ZwlrOutputPowerManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrOutputPowerManagerV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrOutputPowerV1 => {
                    if version > ZwlrOutputPowerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrOutputPowerV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrScreencopyFrameV1 => {
                    if version > ZwlrScreencopyFrameV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrScreencopyFrameV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrScreencopyManagerV1 => {
                    if version > ZwlrScreencopyManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrScreencopyManagerV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
                Self::ZwlrVirtualPointerManagerV1 => {
                    if version > ZwlrVirtualPointerManagerV1::XML_VERSION {
                        return Err(ObjectError::MaxVersion(self, version));
                    }
                    Ok(ZwlrVirtualPointerManagerV1::new(state, version))
                }
                #[cfg(feature = "wlr-protocols")]
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ProxyInterface {
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
    #[cfg(feature = "wayland-protocols")]
    WpAlphaModifierSurfaceV1,
    #[cfg(feature = "wayland-protocols")]
    WpAlphaModifierV1,
    #[cfg(feature = "wayland-protocols")]
    WpColorManagementOutputV1,
    #[cfg(feature = "wayland-protocols")]
    WpColorManagementSurfaceFeedbackV1,
    #[cfg(feature = "wayland-protocols")]
    WpColorManagementSurfaceV1,
    #[cfg(feature = "wayland-protocols")]
    WpColorManagerV1,
    #[cfg(feature = "wayland-protocols")]
    WpImageDescriptionCreatorIccV1,
    #[cfg(feature = "wayland-protocols")]
    WpImageDescriptionCreatorParamsV1,
    #[cfg(feature = "wayland-protocols")]
    WpImageDescriptionInfoV1,
    #[cfg(feature = "wayland-protocols")]
    WpImageDescriptionV1,
    #[cfg(feature = "wayland-protocols")]
    WpColorRepresentationManagerV1,
    #[cfg(feature = "wayland-protocols")]
    WpColorRepresentationSurfaceV1,
    #[cfg(feature = "wayland-protocols")]
    WpCommitTimerV1,
    #[cfg(feature = "wayland-protocols")]
    WpCommitTimingManagerV1,
    #[cfg(feature = "wayland-protocols")]
    WpContentTypeManagerV1,
    #[cfg(feature = "wayland-protocols")]
    WpContentTypeV1,
    #[cfg(feature = "wayland-protocols")]
    WpCursorShapeDeviceV1,
    #[cfg(feature = "wayland-protocols")]
    WpCursorShapeManagerV1,
    #[cfg(feature = "wayland-protocols")]
    WpDrmLeaseConnectorV1,
    #[cfg(feature = "wayland-protocols")]
    WpDrmLeaseDeviceV1,
    #[cfg(feature = "wayland-protocols")]
    WpDrmLeaseRequestV1,
    #[cfg(feature = "wayland-protocols")]
    WpDrmLeaseV1,
    #[cfg(feature = "wayland-protocols")]
    ExtBackgroundEffectManagerV1,
    #[cfg(feature = "wayland-protocols")]
    ExtBackgroundEffectSurfaceV1,
    #[cfg(feature = "wayland-protocols")]
    ExtDataControlDeviceV1,
    #[cfg(feature = "wayland-protocols")]
    ExtDataControlManagerV1,
    #[cfg(feature = "wayland-protocols")]
    ExtDataControlOfferV1,
    #[cfg(feature = "wayland-protocols")]
    ExtDataControlSourceV1,
    #[cfg(feature = "wayland-protocols")]
    ExtForeignToplevelHandleV1,
    #[cfg(feature = "wayland-protocols")]
    ExtForeignToplevelListV1,
    #[cfg(feature = "wayland-protocols")]
    ExtIdleNotificationV1,
    #[cfg(feature = "wayland-protocols")]
    ExtIdleNotifierV1,
    #[cfg(feature = "wayland-protocols")]
    ExtForeignToplevelImageCaptureSourceManagerV1,
    #[cfg(feature = "wayland-protocols")]
    ExtImageCaptureSourceV1,
    #[cfg(feature = "wayland-protocols")]
    ExtOutputImageCaptureSourceManagerV1,
    #[cfg(feature = "wayland-protocols")]
    ExtImageCopyCaptureCursorSessionV1,
    #[cfg(feature = "wayland-protocols")]
    ExtImageCopyCaptureFrameV1,
    #[cfg(feature = "wayland-protocols")]
    ExtImageCopyCaptureManagerV1,
    #[cfg(feature = "wayland-protocols")]
    ExtImageCopyCaptureSessionV1,
    #[cfg(feature = "wayland-protocols")]
    ExtSessionLockManagerV1,
    #[cfg(feature = "wayland-protocols")]
    ExtSessionLockSurfaceV1,
    #[cfg(feature = "wayland-protocols")]
    ExtSessionLockV1,
    #[cfg(feature = "wayland-protocols")]
    ExtTransientSeatManagerV1,
    #[cfg(feature = "wayland-protocols")]
    ExtTransientSeatV1,
    #[cfg(feature = "wayland-protocols")]
    ExtWorkspaceGroupHandleV1,
    #[cfg(feature = "wayland-protocols")]
    ExtWorkspaceHandleV1,
    #[cfg(feature = "wayland-protocols")]
    ExtWorkspaceManagerV1,
    #[cfg(feature = "wayland-protocols")]
    WpFifoManagerV1,
    #[cfg(feature = "wayland-protocols")]
    WpFifoV1,
    #[cfg(feature = "wayland-protocols")]
    WpFractionalScaleManagerV1,
    #[cfg(feature = "wayland-protocols")]
    WpFractionalScaleV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpFullscreenShellModeFeedbackV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpFullscreenShellV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpIdleInhibitManagerV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpIdleInhibitorV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpInputMethodContextV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpInputMethodV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpInputPanelSurfaceV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpInputPanelV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpInputTimestampsManagerV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpInputTimestampsV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpKeyboardShortcutsInhibitManagerV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpKeyboardShortcutsInhibitorV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpLinuxBufferParamsV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpLinuxDmabufFeedbackV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpLinuxDmabufV1,
    #[cfg(feature = "wayland-protocols")]
    WpLinuxDrmSyncobjManagerV1,
    #[cfg(feature = "wayland-protocols")]
    WpLinuxDrmSyncobjSurfaceV1,
    #[cfg(feature = "wayland-protocols")]
    WpLinuxDrmSyncobjTimelineV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpLinuxBufferReleaseV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpLinuxExplicitSynchronizationV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpLinuxSurfaceSynchronizationV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpConfinedPointerV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpLockedPointerV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpPointerConstraintsV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpPointerGestureHoldV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpPointerGesturePinchV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpPointerGestureSwipeV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpPointerGesturesV1,
    #[cfg(feature = "wayland-protocols")]
    WpPointerWarpV1,
    #[cfg(feature = "wayland-protocols")]
    WpPresentation,
    #[cfg(feature = "wayland-protocols")]
    WpPresentationFeedback,
    #[cfg(feature = "wayland-protocols")]
    ZwpPrimarySelectionDeviceManagerV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpPrimarySelectionDeviceV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpPrimarySelectionOfferV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpPrimarySelectionSourceV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpRelativePointerManagerV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpRelativePointerV1,
    #[cfg(feature = "wayland-protocols")]
    WpSecurityContextManagerV1,
    #[cfg(feature = "wayland-protocols")]
    WpSecurityContextV1,
    #[cfg(feature = "wayland-protocols")]
    WpSinglePixelBufferManagerV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpTabletManagerV2,
    #[cfg(feature = "wayland-protocols")]
    ZwpTabletPadDialV2,
    #[cfg(feature = "wayland-protocols")]
    ZwpTabletPadGroupV2,
    #[cfg(feature = "wayland-protocols")]
    ZwpTabletPadRingV2,
    #[cfg(feature = "wayland-protocols")]
    ZwpTabletPadStripV2,
    #[cfg(feature = "wayland-protocols")]
    ZwpTabletPadV2,
    #[cfg(feature = "wayland-protocols")]
    ZwpTabletSeatV2,
    #[cfg(feature = "wayland-protocols")]
    ZwpTabletToolV2,
    #[cfg(feature = "wayland-protocols")]
    ZwpTabletV2,
    #[cfg(feature = "wayland-protocols")]
    WpTearingControlManagerV1,
    #[cfg(feature = "wayland-protocols")]
    WpTearingControlV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpTextInputManagerV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpTextInputV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpTextInputManagerV3,
    #[cfg(feature = "wayland-protocols")]
    ZwpTextInputV3,
    #[cfg(feature = "wayland-protocols")]
    WpViewport,
    #[cfg(feature = "wayland-protocols")]
    WpViewporter,
    #[cfg(feature = "wayland-protocols")]
    XdgActivationTokenV1,
    #[cfg(feature = "wayland-protocols")]
    XdgActivationV1,
    #[cfg(feature = "wayland-protocols")]
    ZxdgDecorationManagerV1,
    #[cfg(feature = "wayland-protocols")]
    ZxdgToplevelDecorationV1,
    #[cfg(feature = "wayland-protocols")]
    XdgDialogV1,
    #[cfg(feature = "wayland-protocols")]
    XdgWmDialogV1,
    #[cfg(feature = "wayland-protocols")]
    ZxdgExportedV2,
    #[cfg(feature = "wayland-protocols")]
    ZxdgExporterV2,
    #[cfg(feature = "wayland-protocols")]
    ZxdgImportedV2,
    #[cfg(feature = "wayland-protocols")]
    ZxdgImporterV2,
    #[cfg(feature = "wayland-protocols")]
    ZxdgOutputManagerV1,
    #[cfg(feature = "wayland-protocols")]
    ZxdgOutputV1,
    #[cfg(feature = "wayland-protocols")]
    XdgPopup,
    #[cfg(feature = "wayland-protocols")]
    XdgPositioner,
    #[cfg(feature = "wayland-protocols")]
    XdgSurface,
    #[cfg(feature = "wayland-protocols")]
    XdgToplevel,
    #[cfg(feature = "wayland-protocols")]
    XdgWmBase,
    #[cfg(feature = "wayland-protocols")]
    XdgSystemBellV1,
    #[cfg(feature = "wayland-protocols")]
    XdgToplevelDragManagerV1,
    #[cfg(feature = "wayland-protocols")]
    XdgToplevelDragV1,
    #[cfg(feature = "wayland-protocols")]
    XdgToplevelIconManagerV1,
    #[cfg(feature = "wayland-protocols")]
    XdgToplevelIconV1,
    #[cfg(feature = "wayland-protocols")]
    XdgToplevelTagManagerV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpXwaylandKeyboardGrabManagerV1,
    #[cfg(feature = "wayland-protocols")]
    ZwpXwaylandKeyboardGrabV1,
    #[cfg(feature = "wayland-protocols")]
    XwaylandShellV1,
    #[cfg(feature = "wayland-protocols")]
    XwaylandSurfaceV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrDataControlDeviceV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrDataControlManagerV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrDataControlOfferV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrDataControlSourceV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrExportDmabufFrameV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrExportDmabufManagerV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrForeignToplevelHandleV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrForeignToplevelManagerV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrGammaControlManagerV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrGammaControlV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrInputInhibitManagerV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrInputInhibitorV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrLayerShellV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrLayerSurfaceV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrOutputConfigurationHeadV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrOutputConfigurationV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrOutputHeadV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrOutputManagerV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrOutputModeV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrOutputPowerManagerV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrOutputPowerV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrScreencopyFrameV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrScreencopyManagerV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrVirtualPointerManagerV1,
    #[cfg(feature = "wlr-protocols")]
    ZwlrVirtualPointerV1,
}

impl ProxyInterface {
    pub fn name(self) -> &'static str {
        match self {
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
            #[cfg(feature = "wayland-protocols")]
            Self::WpAlphaModifierSurfaceV1 => "wp_alpha_modifier_surface_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpAlphaModifierV1 => "wp_alpha_modifier_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpColorManagementOutputV1 => "wp_color_management_output_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpColorManagementSurfaceFeedbackV1 => "wp_color_management_surface_feedback_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpColorManagementSurfaceV1 => "wp_color_management_surface_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpColorManagerV1 => "wp_color_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpImageDescriptionCreatorIccV1 => "wp_image_description_creator_icc_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpImageDescriptionCreatorParamsV1 => "wp_image_description_creator_params_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpImageDescriptionInfoV1 => "wp_image_description_info_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpImageDescriptionV1 => "wp_image_description_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpColorRepresentationManagerV1 => "wp_color_representation_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpColorRepresentationSurfaceV1 => "wp_color_representation_surface_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpCommitTimerV1 => "wp_commit_timer_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpCommitTimingManagerV1 => "wp_commit_timing_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpContentTypeManagerV1 => "wp_content_type_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpContentTypeV1 => "wp_content_type_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpCursorShapeDeviceV1 => "wp_cursor_shape_device_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpCursorShapeManagerV1 => "wp_cursor_shape_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpDrmLeaseConnectorV1 => "wp_drm_lease_connector_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpDrmLeaseDeviceV1 => "wp_drm_lease_device_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpDrmLeaseRequestV1 => "wp_drm_lease_request_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpDrmLeaseV1 => "wp_drm_lease_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtBackgroundEffectManagerV1 => "ext_background_effect_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtBackgroundEffectSurfaceV1 => "ext_background_effect_surface_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtDataControlDeviceV1 => "ext_data_control_device_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtDataControlManagerV1 => "ext_data_control_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtDataControlOfferV1 => "ext_data_control_offer_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtDataControlSourceV1 => "ext_data_control_source_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtForeignToplevelHandleV1 => "ext_foreign_toplevel_handle_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtForeignToplevelListV1 => "ext_foreign_toplevel_list_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtIdleNotificationV1 => "ext_idle_notification_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtIdleNotifierV1 => "ext_idle_notifier_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtForeignToplevelImageCaptureSourceManagerV1 => "ext_foreign_toplevel_image_capture_source_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtImageCaptureSourceV1 => "ext_image_capture_source_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtOutputImageCaptureSourceManagerV1 => "ext_output_image_capture_source_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtImageCopyCaptureCursorSessionV1 => "ext_image_copy_capture_cursor_session_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtImageCopyCaptureFrameV1 => "ext_image_copy_capture_frame_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtImageCopyCaptureManagerV1 => "ext_image_copy_capture_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtImageCopyCaptureSessionV1 => "ext_image_copy_capture_session_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtSessionLockManagerV1 => "ext_session_lock_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtSessionLockSurfaceV1 => "ext_session_lock_surface_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtSessionLockV1 => "ext_session_lock_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtTransientSeatManagerV1 => "ext_transient_seat_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtTransientSeatV1 => "ext_transient_seat_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtWorkspaceGroupHandleV1 => "ext_workspace_group_handle_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtWorkspaceHandleV1 => "ext_workspace_handle_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ExtWorkspaceManagerV1 => "ext_workspace_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpFifoManagerV1 => "wp_fifo_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpFifoV1 => "wp_fifo_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpFractionalScaleManagerV1 => "wp_fractional_scale_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpFractionalScaleV1 => "wp_fractional_scale_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpFullscreenShellModeFeedbackV1 => "zwp_fullscreen_shell_mode_feedback_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpFullscreenShellV1 => "zwp_fullscreen_shell_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpIdleInhibitManagerV1 => "zwp_idle_inhibit_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpIdleInhibitorV1 => "zwp_idle_inhibitor_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpInputMethodContextV1 => "zwp_input_method_context_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpInputMethodV1 => "zwp_input_method_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpInputPanelSurfaceV1 => "zwp_input_panel_surface_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpInputPanelV1 => "zwp_input_panel_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpInputTimestampsManagerV1 => "zwp_input_timestamps_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpInputTimestampsV1 => "zwp_input_timestamps_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpKeyboardShortcutsInhibitManagerV1 => "zwp_keyboard_shortcuts_inhibit_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpKeyboardShortcutsInhibitorV1 => "zwp_keyboard_shortcuts_inhibitor_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpLinuxBufferParamsV1 => "zwp_linux_buffer_params_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpLinuxDmabufFeedbackV1 => "zwp_linux_dmabuf_feedback_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpLinuxDmabufV1 => "zwp_linux_dmabuf_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpLinuxDrmSyncobjManagerV1 => "wp_linux_drm_syncobj_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpLinuxDrmSyncobjSurfaceV1 => "wp_linux_drm_syncobj_surface_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpLinuxDrmSyncobjTimelineV1 => "wp_linux_drm_syncobj_timeline_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpLinuxBufferReleaseV1 => "zwp_linux_buffer_release_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpLinuxExplicitSynchronizationV1 => "zwp_linux_explicit_synchronization_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpLinuxSurfaceSynchronizationV1 => "zwp_linux_surface_synchronization_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpConfinedPointerV1 => "zwp_confined_pointer_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpLockedPointerV1 => "zwp_locked_pointer_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpPointerConstraintsV1 => "zwp_pointer_constraints_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpPointerGestureHoldV1 => "zwp_pointer_gesture_hold_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpPointerGesturePinchV1 => "zwp_pointer_gesture_pinch_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpPointerGestureSwipeV1 => "zwp_pointer_gesture_swipe_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpPointerGesturesV1 => "zwp_pointer_gestures_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpPointerWarpV1 => "wp_pointer_warp_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpPresentation => "wp_presentation",
            #[cfg(feature = "wayland-protocols")]
            Self::WpPresentationFeedback => "wp_presentation_feedback",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpPrimarySelectionDeviceManagerV1 => "zwp_primary_selection_device_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpPrimarySelectionDeviceV1 => "zwp_primary_selection_device_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpPrimarySelectionOfferV1 => "zwp_primary_selection_offer_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpPrimarySelectionSourceV1 => "zwp_primary_selection_source_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpRelativePointerManagerV1 => "zwp_relative_pointer_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpRelativePointerV1 => "zwp_relative_pointer_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpSecurityContextManagerV1 => "wp_security_context_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpSecurityContextV1 => "wp_security_context_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpSinglePixelBufferManagerV1 => "wp_single_pixel_buffer_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTabletManagerV2 => "zwp_tablet_manager_v2",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTabletPadDialV2 => "zwp_tablet_pad_dial_v2",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTabletPadGroupV2 => "zwp_tablet_pad_group_v2",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTabletPadRingV2 => "zwp_tablet_pad_ring_v2",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTabletPadStripV2 => "zwp_tablet_pad_strip_v2",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTabletPadV2 => "zwp_tablet_pad_v2",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTabletSeatV2 => "zwp_tablet_seat_v2",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTabletToolV2 => "zwp_tablet_tool_v2",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTabletV2 => "zwp_tablet_v2",
            #[cfg(feature = "wayland-protocols")]
            Self::WpTearingControlManagerV1 => "wp_tearing_control_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::WpTearingControlV1 => "wp_tearing_control_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTextInputManagerV1 => "zwp_text_input_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTextInputV1 => "zwp_text_input_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTextInputManagerV3 => "zwp_text_input_manager_v3",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTextInputV3 => "zwp_text_input_v3",
            #[cfg(feature = "wayland-protocols")]
            Self::WpViewport => "wp_viewport",
            #[cfg(feature = "wayland-protocols")]
            Self::WpViewporter => "wp_viewporter",
            #[cfg(feature = "wayland-protocols")]
            Self::XdgActivationTokenV1 => "xdg_activation_token_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::XdgActivationV1 => "xdg_activation_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZxdgDecorationManagerV1 => "zxdg_decoration_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZxdgToplevelDecorationV1 => "zxdg_toplevel_decoration_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::XdgDialogV1 => "xdg_dialog_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::XdgWmDialogV1 => "xdg_wm_dialog_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZxdgExportedV2 => "zxdg_exported_v2",
            #[cfg(feature = "wayland-protocols")]
            Self::ZxdgExporterV2 => "zxdg_exporter_v2",
            #[cfg(feature = "wayland-protocols")]
            Self::ZxdgImportedV2 => "zxdg_imported_v2",
            #[cfg(feature = "wayland-protocols")]
            Self::ZxdgImporterV2 => "zxdg_importer_v2",
            #[cfg(feature = "wayland-protocols")]
            Self::ZxdgOutputManagerV1 => "zxdg_output_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZxdgOutputV1 => "zxdg_output_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::XdgPopup => "xdg_popup",
            #[cfg(feature = "wayland-protocols")]
            Self::XdgPositioner => "xdg_positioner",
            #[cfg(feature = "wayland-protocols")]
            Self::XdgSurface => "xdg_surface",
            #[cfg(feature = "wayland-protocols")]
            Self::XdgToplevel => "xdg_toplevel",
            #[cfg(feature = "wayland-protocols")]
            Self::XdgWmBase => "xdg_wm_base",
            #[cfg(feature = "wayland-protocols")]
            Self::XdgSystemBellV1 => "xdg_system_bell_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::XdgToplevelDragManagerV1 => "xdg_toplevel_drag_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::XdgToplevelDragV1 => "xdg_toplevel_drag_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::XdgToplevelIconManagerV1 => "xdg_toplevel_icon_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::XdgToplevelIconV1 => "xdg_toplevel_icon_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::XdgToplevelTagManagerV1 => "xdg_toplevel_tag_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpXwaylandKeyboardGrabManagerV1 => "zwp_xwayland_keyboard_grab_manager_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpXwaylandKeyboardGrabV1 => "zwp_xwayland_keyboard_grab_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::XwaylandShellV1 => "xwayland_shell_v1",
            #[cfg(feature = "wayland-protocols")]
            Self::XwaylandSurfaceV1 => "xwayland_surface_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrDataControlDeviceV1 => "zwlr_data_control_device_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrDataControlManagerV1 => "zwlr_data_control_manager_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrDataControlOfferV1 => "zwlr_data_control_offer_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrDataControlSourceV1 => "zwlr_data_control_source_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrExportDmabufFrameV1 => "zwlr_export_dmabuf_frame_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrExportDmabufManagerV1 => "zwlr_export_dmabuf_manager_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrForeignToplevelHandleV1 => "zwlr_foreign_toplevel_handle_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrForeignToplevelManagerV1 => "zwlr_foreign_toplevel_manager_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrGammaControlManagerV1 => "zwlr_gamma_control_manager_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrGammaControlV1 => "zwlr_gamma_control_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrInputInhibitManagerV1 => "zwlr_input_inhibit_manager_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrInputInhibitorV1 => "zwlr_input_inhibitor_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrLayerShellV1 => "zwlr_layer_shell_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrLayerSurfaceV1 => "zwlr_layer_surface_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrOutputConfigurationHeadV1 => "zwlr_output_configuration_head_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrOutputConfigurationV1 => "zwlr_output_configuration_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrOutputHeadV1 => "zwlr_output_head_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrOutputManagerV1 => "zwlr_output_manager_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrOutputModeV1 => "zwlr_output_mode_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrOutputPowerManagerV1 => "zwlr_output_power_manager_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrOutputPowerV1 => "zwlr_output_power_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrScreencopyFrameV1 => "zwlr_screencopy_frame_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrScreencopyManagerV1 => "zwlr_screencopy_manager_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrVirtualPointerManagerV1 => "zwlr_virtual_pointer_manager_v1",
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrVirtualPointerV1 => "zwlr_virtual_pointer_v1",
        }
    }

    pub fn xml_version(self) -> u32 {
        match self {
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
            #[cfg(feature = "wayland-protocols")]
            Self::WpAlphaModifierSurfaceV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpAlphaModifierV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpColorManagementOutputV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpColorManagementSurfaceFeedbackV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpColorManagementSurfaceV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpColorManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpImageDescriptionCreatorIccV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpImageDescriptionCreatorParamsV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpImageDescriptionInfoV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpImageDescriptionV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpColorRepresentationManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpColorRepresentationSurfaceV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpCommitTimerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpCommitTimingManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpContentTypeManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpContentTypeV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpCursorShapeDeviceV1 => 2,
            #[cfg(feature = "wayland-protocols")]
            Self::WpCursorShapeManagerV1 => 2,
            #[cfg(feature = "wayland-protocols")]
            Self::WpDrmLeaseConnectorV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpDrmLeaseDeviceV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpDrmLeaseRequestV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpDrmLeaseV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtBackgroundEffectManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtBackgroundEffectSurfaceV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtDataControlDeviceV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtDataControlManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtDataControlOfferV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtDataControlSourceV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtForeignToplevelHandleV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtForeignToplevelListV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtIdleNotificationV1 => 2,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtIdleNotifierV1 => 2,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtForeignToplevelImageCaptureSourceManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtImageCaptureSourceV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtOutputImageCaptureSourceManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtImageCopyCaptureCursorSessionV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtImageCopyCaptureFrameV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtImageCopyCaptureManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtImageCopyCaptureSessionV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtSessionLockManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtSessionLockSurfaceV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtSessionLockV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtTransientSeatManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtTransientSeatV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtWorkspaceGroupHandleV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtWorkspaceHandleV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ExtWorkspaceManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpFifoManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpFifoV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpFractionalScaleManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpFractionalScaleV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpFullscreenShellModeFeedbackV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpFullscreenShellV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpIdleInhibitManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpIdleInhibitorV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpInputMethodContextV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpInputMethodV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpInputPanelSurfaceV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpInputPanelV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpInputTimestampsManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpInputTimestampsV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpKeyboardShortcutsInhibitManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpKeyboardShortcutsInhibitorV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpLinuxBufferParamsV1 => 5,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpLinuxDmabufFeedbackV1 => 5,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpLinuxDmabufV1 => 5,
            #[cfg(feature = "wayland-protocols")]
            Self::WpLinuxDrmSyncobjManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpLinuxDrmSyncobjSurfaceV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpLinuxDrmSyncobjTimelineV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpLinuxBufferReleaseV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpLinuxExplicitSynchronizationV1 => 2,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpLinuxSurfaceSynchronizationV1 => 2,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpConfinedPointerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpLockedPointerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpPointerConstraintsV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpPointerGestureHoldV1 => 3,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpPointerGesturePinchV1 => 2,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpPointerGestureSwipeV1 => 2,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpPointerGesturesV1 => 3,
            #[cfg(feature = "wayland-protocols")]
            Self::WpPointerWarpV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpPresentation => 2,
            #[cfg(feature = "wayland-protocols")]
            Self::WpPresentationFeedback => 2,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpPrimarySelectionDeviceManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpPrimarySelectionDeviceV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpPrimarySelectionOfferV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpPrimarySelectionSourceV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpRelativePointerManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpRelativePointerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpSecurityContextManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpSecurityContextV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpSinglePixelBufferManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTabletManagerV2 => 2,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTabletPadDialV2 => 2,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTabletPadGroupV2 => 2,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTabletPadRingV2 => 2,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTabletPadStripV2 => 2,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTabletPadV2 => 2,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTabletSeatV2 => 2,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTabletToolV2 => 2,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTabletV2 => 2,
            #[cfg(feature = "wayland-protocols")]
            Self::WpTearingControlManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpTearingControlV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTextInputManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTextInputV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTextInputManagerV3 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpTextInputV3 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpViewport => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::WpViewporter => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::XdgActivationTokenV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::XdgActivationV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZxdgDecorationManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZxdgToplevelDecorationV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::XdgDialogV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::XdgWmDialogV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZxdgExportedV2 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZxdgExporterV2 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZxdgImportedV2 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZxdgImporterV2 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZxdgOutputManagerV1 => 3,
            #[cfg(feature = "wayland-protocols")]
            Self::ZxdgOutputV1 => 3,
            #[cfg(feature = "wayland-protocols")]
            Self::XdgPopup => 7,
            #[cfg(feature = "wayland-protocols")]
            Self::XdgPositioner => 7,
            #[cfg(feature = "wayland-protocols")]
            Self::XdgSurface => 7,
            #[cfg(feature = "wayland-protocols")]
            Self::XdgToplevel => 7,
            #[cfg(feature = "wayland-protocols")]
            Self::XdgWmBase => 7,
            #[cfg(feature = "wayland-protocols")]
            Self::XdgSystemBellV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::XdgToplevelDragManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::XdgToplevelDragV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::XdgToplevelIconManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::XdgToplevelIconV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::XdgToplevelTagManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpXwaylandKeyboardGrabManagerV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::ZwpXwaylandKeyboardGrabV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::XwaylandShellV1 => 1,
            #[cfg(feature = "wayland-protocols")]
            Self::XwaylandSurfaceV1 => 1,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrDataControlDeviceV1 => 2,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrDataControlManagerV1 => 2,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrDataControlOfferV1 => 1,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrDataControlSourceV1 => 1,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrExportDmabufFrameV1 => 1,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrExportDmabufManagerV1 => 1,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrForeignToplevelHandleV1 => 3,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrForeignToplevelManagerV1 => 3,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrGammaControlManagerV1 => 1,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrGammaControlV1 => 1,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrInputInhibitManagerV1 => 1,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrInputInhibitorV1 => 1,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrLayerShellV1 => 5,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrLayerSurfaceV1 => 5,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrOutputConfigurationHeadV1 => 4,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrOutputConfigurationV1 => 4,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrOutputHeadV1 => 4,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrOutputManagerV1 => 4,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrOutputModeV1 => 3,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrOutputPowerManagerV1 => 1,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrOutputPowerV1 => 1,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrScreencopyFrameV1 => 3,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrScreencopyManagerV1 => 3,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrVirtualPointerManagerV1 => 2,
            #[cfg(feature = "wlr-protocols")]
            Self::ZwlrVirtualPointerV1 => 2,
        }
    }
}
