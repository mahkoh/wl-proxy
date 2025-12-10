pub mod alpha_modifier_v1;
pub mod color_management_v1;
pub mod color_representation_v1;
pub mod commit_timing_v1;
pub mod content_type_v1;
pub mod cursor_shape_v1;
pub mod drm_lease_v1;
pub mod ext_background_effect_v1;
pub mod ext_data_control_v1;
pub mod ext_foreign_toplevel_list_v1;
pub mod ext_idle_notify_v1;
pub mod ext_image_capture_source_v1;
pub mod ext_image_copy_capture_v1;
pub mod ext_session_lock_v1;
pub mod ext_transient_seat_v1;
pub mod ext_workspace_v1;
pub mod fifo_v1;
pub mod fractional_scale_v1;
pub mod fullscreen_shell_unstable_v1;
pub mod idle_inhibit_unstable_v1;
pub mod input_method_unstable_v1;
pub mod input_timestamps_unstable_v1;
pub mod keyboard_shortcuts_inhibit_unstable_v1;
pub mod linux_dmabuf_v1;
pub mod linux_drm_syncobj_v1;
pub mod zwp_linux_explicit_synchronization_unstable_v1;
pub mod pointer_constraints_unstable_v1;
pub mod pointer_gestures_unstable_v1;
pub mod pointer_warp_v1;
pub mod presentation_time;
pub mod wp_primary_selection_unstable_v1;
pub mod relative_pointer_unstable_v1;
pub mod security_context_v1;
pub mod single_pixel_buffer_v1;
pub mod tablet_v2;
pub mod tearing_control_v1;
pub mod text_input_unstable_v1;
pub mod text_input_unstable_v3;
pub mod viewporter;
pub mod wayland;
pub mod wlr_data_control_unstable_v1;
pub mod wlr_export_dmabuf_unstable_v1;
pub mod wlr_foreign_toplevel_management_unstable_v1;
pub mod wlr_gamma_control_unstable_v1;
pub mod wlr_input_inhibit_unstable_v1;
pub mod wlr_layer_shell_unstable_v1;
pub mod wlr_output_management_unstable_v1;
pub mod wlr_output_power_management_unstable_v1;
pub mod wlr_screencopy_unstable_v1;
pub mod wlr_virtual_pointer_unstable_v1;
pub mod xdg_activation_v1;
pub mod xdg_decoration_unstable_v1;
pub mod xdg_dialog_v1;
pub mod xdg_foreign_unstable_v2;
pub mod xdg_output_unstable_v1;
pub mod xdg_shell;
pub mod xdg_system_bell_v1;
pub mod xdg_toplevel_drag_v1;
pub mod xdg_toplevel_icon_v1;
pub mod xdg_toplevel_tag_v1;
pub mod xwayland_keyboard_grab_unstable_v1;
pub mod xwayland_shell_v1;

#[allow(unused_imports)]
mod all_types {
    pub(super) use super::alpha_modifier_v1::wp_alpha_modifier_surface_v1::WpAlphaModifierSurfaceV1;
    pub(super) use super::alpha_modifier_v1::wp_alpha_modifier_surface_v1::WpAlphaModifierSurfaceV1Error;
    pub(super) use super::alpha_modifier_v1::wp_alpha_modifier_v1::WpAlphaModifierV1;
    pub(super) use super::alpha_modifier_v1::wp_alpha_modifier_v1::WpAlphaModifierV1Error;
    pub(super) use super::color_management_v1::wp_color_management_output_v1::WpColorManagementOutputV1;
    pub(super) use super::color_management_v1::wp_color_management_surface_feedback_v1::WpColorManagementSurfaceFeedbackV1;
    pub(super) use super::color_management_v1::wp_color_management_surface_feedback_v1::WpColorManagementSurfaceFeedbackV1Error;
    pub(super) use super::color_management_v1::wp_color_management_surface_v1::WpColorManagementSurfaceV1;
    pub(super) use super::color_management_v1::wp_color_management_surface_v1::WpColorManagementSurfaceV1Error;
    pub(super) use super::color_management_v1::wp_color_manager_v1::WpColorManagerV1;
    pub(super) use super::color_management_v1::wp_color_manager_v1::WpColorManagerV1Error;
    pub(super) use super::color_management_v1::wp_color_manager_v1::WpColorManagerV1RenderIntent;
    pub(super) use super::color_management_v1::wp_color_manager_v1::WpColorManagerV1Feature;
    pub(super) use super::color_management_v1::wp_color_manager_v1::WpColorManagerV1Primaries;
    pub(super) use super::color_management_v1::wp_color_manager_v1::WpColorManagerV1TransferFunction;
    pub(super) use super::color_management_v1::wp_image_description_creator_icc_v1::WpImageDescriptionCreatorIccV1;
    pub(super) use super::color_management_v1::wp_image_description_creator_icc_v1::WpImageDescriptionCreatorIccV1Error;
    pub(super) use super::color_management_v1::wp_image_description_creator_params_v1::WpImageDescriptionCreatorParamsV1;
    pub(super) use super::color_management_v1::wp_image_description_creator_params_v1::WpImageDescriptionCreatorParamsV1Error;
    pub(super) use super::color_management_v1::wp_image_description_info_v1::WpImageDescriptionInfoV1;
    pub(super) use super::color_management_v1::wp_image_description_v1::WpImageDescriptionV1;
    pub(super) use super::color_management_v1::wp_image_description_v1::WpImageDescriptionV1Error;
    pub(super) use super::color_management_v1::wp_image_description_v1::WpImageDescriptionV1Cause;
    pub(super) use super::color_representation_v1::wp_color_representation_manager_v1::WpColorRepresentationManagerV1;
    pub(super) use super::color_representation_v1::wp_color_representation_manager_v1::WpColorRepresentationManagerV1Error;
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::WpColorRepresentationSurfaceV1;
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::WpColorRepresentationSurfaceV1Error;
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::WpColorRepresentationSurfaceV1AlphaMode;
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::WpColorRepresentationSurfaceV1Coefficients;
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::WpColorRepresentationSurfaceV1Range;
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::WpColorRepresentationSurfaceV1ChromaLocation;
    pub(super) use super::commit_timing_v1::wp_commit_timer_v1::WpCommitTimerV1;
    pub(super) use super::commit_timing_v1::wp_commit_timer_v1::WpCommitTimerV1Error;
    pub(super) use super::commit_timing_v1::wp_commit_timing_manager_v1::WpCommitTimingManagerV1;
    pub(super) use super::commit_timing_v1::wp_commit_timing_manager_v1::WpCommitTimingManagerV1Error;
    pub(super) use super::content_type_v1::wp_content_type_manager_v1::WpContentTypeManagerV1;
    pub(super) use super::content_type_v1::wp_content_type_manager_v1::WpContentTypeManagerV1Error;
    pub(super) use super::content_type_v1::wp_content_type_v1::WpContentTypeV1;
    pub(super) use super::content_type_v1::wp_content_type_v1::WpContentTypeV1Type;
    pub(super) use super::cursor_shape_v1::wp_cursor_shape_device_v1::WpCursorShapeDeviceV1;
    pub(super) use super::cursor_shape_v1::wp_cursor_shape_device_v1::WpCursorShapeDeviceV1Shape;
    pub(super) use super::cursor_shape_v1::wp_cursor_shape_device_v1::WpCursorShapeDeviceV1Error;
    pub(super) use super::cursor_shape_v1::wp_cursor_shape_manager_v1::WpCursorShapeManagerV1;
    pub(super) use super::drm_lease_v1::wp_drm_lease_connector_v1::WpDrmLeaseConnectorV1;
    pub(super) use super::drm_lease_v1::wp_drm_lease_device_v1::WpDrmLeaseDeviceV1;
    pub(super) use super::drm_lease_v1::wp_drm_lease_request_v1::WpDrmLeaseRequestV1;
    pub(super) use super::drm_lease_v1::wp_drm_lease_request_v1::WpDrmLeaseRequestV1Error;
    pub(super) use super::drm_lease_v1::wp_drm_lease_v1::WpDrmLeaseV1;
    pub(super) use super::ext_background_effect_v1::ext_background_effect_manager_v1::ExtBackgroundEffectManagerV1;
    pub(super) use super::ext_background_effect_v1::ext_background_effect_manager_v1::ExtBackgroundEffectManagerV1Error;
    pub(super) use super::ext_background_effect_v1::ext_background_effect_manager_v1::ExtBackgroundEffectManagerV1Capability;
    pub(super) use super::ext_background_effect_v1::ext_background_effect_surface_v1::ExtBackgroundEffectSurfaceV1;
    pub(super) use super::ext_background_effect_v1::ext_background_effect_surface_v1::ExtBackgroundEffectSurfaceV1Error;
    pub(super) use super::ext_data_control_v1::ext_data_control_device_v1::ExtDataControlDeviceV1;
    pub(super) use super::ext_data_control_v1::ext_data_control_device_v1::ExtDataControlDeviceV1Error;
    pub(super) use super::ext_data_control_v1::ext_data_control_manager_v1::ExtDataControlManagerV1;
    pub(super) use super::ext_data_control_v1::ext_data_control_offer_v1::ExtDataControlOfferV1;
    pub(super) use super::ext_data_control_v1::ext_data_control_source_v1::ExtDataControlSourceV1;
    pub(super) use super::ext_data_control_v1::ext_data_control_source_v1::ExtDataControlSourceV1Error;
    pub(super) use super::ext_foreign_toplevel_list_v1::ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1;
    pub(super) use super::ext_foreign_toplevel_list_v1::ext_foreign_toplevel_list_v1::ExtForeignToplevelListV1;
    pub(super) use super::ext_idle_notify_v1::ext_idle_notification_v1::ExtIdleNotificationV1;
    pub(super) use super::ext_idle_notify_v1::ext_idle_notifier_v1::ExtIdleNotifierV1;
    pub(super) use super::ext_image_capture_source_v1::ext_foreign_toplevel_image_capture_source_manager_v1::ExtForeignToplevelImageCaptureSourceManagerV1;
    pub(super) use super::ext_image_capture_source_v1::ext_image_capture_source_v1::ExtImageCaptureSourceV1;
    pub(super) use super::ext_image_capture_source_v1::ext_output_image_capture_source_manager_v1::ExtOutputImageCaptureSourceManagerV1;
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_cursor_session_v1::ExtImageCopyCaptureCursorSessionV1;
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_cursor_session_v1::ExtImageCopyCaptureCursorSessionV1Error;
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_frame_v1::ExtImageCopyCaptureFrameV1;
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_frame_v1::ExtImageCopyCaptureFrameV1Error;
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_frame_v1::ExtImageCopyCaptureFrameV1FailureReason;
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_manager_v1::ExtImageCopyCaptureManagerV1;
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_manager_v1::ExtImageCopyCaptureManagerV1Error;
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_manager_v1::ExtImageCopyCaptureManagerV1Options;
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_session_v1::ExtImageCopyCaptureSessionV1;
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_session_v1::ExtImageCopyCaptureSessionV1Error;
    pub(super) use super::ext_session_lock_v1::ext_session_lock_manager_v1::ExtSessionLockManagerV1;
    pub(super) use super::ext_session_lock_v1::ext_session_lock_surface_v1::ExtSessionLockSurfaceV1;
    pub(super) use super::ext_session_lock_v1::ext_session_lock_surface_v1::ExtSessionLockSurfaceV1Error;
    pub(super) use super::ext_session_lock_v1::ext_session_lock_v1::ExtSessionLockV1;
    pub(super) use super::ext_session_lock_v1::ext_session_lock_v1::ExtSessionLockV1Error;
    pub(super) use super::ext_transient_seat_v1::ext_transient_seat_manager_v1::ExtTransientSeatManagerV1;
    pub(super) use super::ext_transient_seat_v1::ext_transient_seat_v1::ExtTransientSeatV1;
    pub(super) use super::ext_workspace_v1::ext_workspace_group_handle_v1::ExtWorkspaceGroupHandleV1;
    pub(super) use super::ext_workspace_v1::ext_workspace_group_handle_v1::ExtWorkspaceGroupHandleV1GroupCapabilities;
    pub(super) use super::ext_workspace_v1::ext_workspace_handle_v1::ExtWorkspaceHandleV1;
    pub(super) use super::ext_workspace_v1::ext_workspace_handle_v1::ExtWorkspaceHandleV1State;
    pub(super) use super::ext_workspace_v1::ext_workspace_handle_v1::ExtWorkspaceHandleV1WorkspaceCapabilities;
    pub(super) use super::ext_workspace_v1::ext_workspace_manager_v1::ExtWorkspaceManagerV1;
    pub(super) use super::fifo_v1::wp_fifo_manager_v1::WpFifoManagerV1;
    pub(super) use super::fifo_v1::wp_fifo_manager_v1::WpFifoManagerV1Error;
    pub(super) use super::fifo_v1::wp_fifo_v1::WpFifoV1;
    pub(super) use super::fifo_v1::wp_fifo_v1::WpFifoV1Error;
    pub(super) use super::fractional_scale_v1::wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1;
    pub(super) use super::fractional_scale_v1::wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1Error;
    pub(super) use super::fractional_scale_v1::wp_fractional_scale_v1::WpFractionalScaleV1;
    pub(super) use super::fullscreen_shell_unstable_v1::zwp_fullscreen_shell_mode_feedback_v1::ZwpFullscreenShellModeFeedbackV1;
    pub(super) use super::fullscreen_shell_unstable_v1::zwp_fullscreen_shell_v1::ZwpFullscreenShellV1;
    pub(super) use super::fullscreen_shell_unstable_v1::zwp_fullscreen_shell_v1::ZwpFullscreenShellV1Capability;
    pub(super) use super::fullscreen_shell_unstable_v1::zwp_fullscreen_shell_v1::ZwpFullscreenShellV1PresentMethod;
    pub(super) use super::fullscreen_shell_unstable_v1::zwp_fullscreen_shell_v1::ZwpFullscreenShellV1Error;
    pub(super) use super::idle_inhibit_unstable_v1::zwp_idle_inhibit_manager_v1::ZwpIdleInhibitManagerV1;
    pub(super) use super::idle_inhibit_unstable_v1::zwp_idle_inhibitor_v1::ZwpIdleInhibitorV1;
    pub(super) use super::input_method_unstable_v1::zwp_input_method_context_v1::ZwpInputMethodContextV1;
    pub(super) use super::input_method_unstable_v1::zwp_input_method_v1::ZwpInputMethodV1;
    pub(super) use super::input_method_unstable_v1::zwp_input_panel_surface_v1::ZwpInputPanelSurfaceV1;
    pub(super) use super::input_method_unstable_v1::zwp_input_panel_surface_v1::ZwpInputPanelSurfaceV1Position;
    pub(super) use super::input_method_unstable_v1::zwp_input_panel_v1::ZwpInputPanelV1;
    pub(super) use super::input_timestamps_unstable_v1::zwp_input_timestamps_manager_v1::ZwpInputTimestampsManagerV1;
    pub(super) use super::input_timestamps_unstable_v1::zwp_input_timestamps_v1::ZwpInputTimestampsV1;
    pub(super) use super::keyboard_shortcuts_inhibit_unstable_v1::zwp_keyboard_shortcuts_inhibit_manager_v1::ZwpKeyboardShortcutsInhibitManagerV1;
    pub(super) use super::keyboard_shortcuts_inhibit_unstable_v1::zwp_keyboard_shortcuts_inhibit_manager_v1::ZwpKeyboardShortcutsInhibitManagerV1Error;
    pub(super) use super::keyboard_shortcuts_inhibit_unstable_v1::zwp_keyboard_shortcuts_inhibitor_v1::ZwpKeyboardShortcutsInhibitorV1;
    pub(super) use super::linux_dmabuf_v1::zwp_linux_buffer_params_v1::ZwpLinuxBufferParamsV1;
    pub(super) use super::linux_dmabuf_v1::zwp_linux_buffer_params_v1::ZwpLinuxBufferParamsV1Error;
    pub(super) use super::linux_dmabuf_v1::zwp_linux_buffer_params_v1::ZwpLinuxBufferParamsV1Flags;
    pub(super) use super::linux_dmabuf_v1::zwp_linux_dmabuf_feedback_v1::ZwpLinuxDmabufFeedbackV1;
    pub(super) use super::linux_dmabuf_v1::zwp_linux_dmabuf_feedback_v1::ZwpLinuxDmabufFeedbackV1TrancheFlags;
    pub(super) use super::linux_dmabuf_v1::zwp_linux_dmabuf_v1::ZwpLinuxDmabufV1;
    pub(super) use super::linux_drm_syncobj_v1::wp_linux_drm_syncobj_manager_v1::WpLinuxDrmSyncobjManagerV1;
    pub(super) use super::linux_drm_syncobj_v1::wp_linux_drm_syncobj_manager_v1::WpLinuxDrmSyncobjManagerV1Error;
    pub(super) use super::linux_drm_syncobj_v1::wp_linux_drm_syncobj_surface_v1::WpLinuxDrmSyncobjSurfaceV1;
    pub(super) use super::linux_drm_syncobj_v1::wp_linux_drm_syncobj_surface_v1::WpLinuxDrmSyncobjSurfaceV1Error;
    pub(super) use super::linux_drm_syncobj_v1::wp_linux_drm_syncobj_timeline_v1::WpLinuxDrmSyncobjTimelineV1;
    pub(super) use super::zwp_linux_explicit_synchronization_unstable_v1::zwp_linux_buffer_release_v1::ZwpLinuxBufferReleaseV1;
    pub(super) use super::zwp_linux_explicit_synchronization_unstable_v1::zwp_linux_explicit_synchronization_v1::ZwpLinuxExplicitSynchronizationV1;
    pub(super) use super::zwp_linux_explicit_synchronization_unstable_v1::zwp_linux_explicit_synchronization_v1::ZwpLinuxExplicitSynchronizationV1Error;
    pub(super) use super::zwp_linux_explicit_synchronization_unstable_v1::zwp_linux_surface_synchronization_v1::ZwpLinuxSurfaceSynchronizationV1;
    pub(super) use super::zwp_linux_explicit_synchronization_unstable_v1::zwp_linux_surface_synchronization_v1::ZwpLinuxSurfaceSynchronizationV1Error;
    pub(super) use super::pointer_constraints_unstable_v1::zwp_confined_pointer_v1::ZwpConfinedPointerV1;
    pub(super) use super::pointer_constraints_unstable_v1::zwp_locked_pointer_v1::ZwpLockedPointerV1;
    pub(super) use super::pointer_constraints_unstable_v1::zwp_pointer_constraints_v1::ZwpPointerConstraintsV1;
    pub(super) use super::pointer_constraints_unstable_v1::zwp_pointer_constraints_v1::ZwpPointerConstraintsV1Error;
    pub(super) use super::pointer_constraints_unstable_v1::zwp_pointer_constraints_v1::ZwpPointerConstraintsV1Lifetime;
    pub(super) use super::pointer_gestures_unstable_v1::zwp_pointer_gesture_hold_v1::ZwpPointerGestureHoldV1;
    pub(super) use super::pointer_gestures_unstable_v1::zwp_pointer_gesture_pinch_v1::ZwpPointerGesturePinchV1;
    pub(super) use super::pointer_gestures_unstable_v1::zwp_pointer_gesture_swipe_v1::ZwpPointerGestureSwipeV1;
    pub(super) use super::pointer_gestures_unstable_v1::zwp_pointer_gestures_v1::ZwpPointerGesturesV1;
    pub(super) use super::pointer_warp_v1::wp_pointer_warp_v1::WpPointerWarpV1;
    pub(super) use super::presentation_time::wp_presentation::WpPresentation;
    pub(super) use super::presentation_time::wp_presentation::WpPresentationError;
    pub(super) use super::presentation_time::wp_presentation_feedback::WpPresentationFeedback;
    pub(super) use super::presentation_time::wp_presentation_feedback::WpPresentationFeedbackKind;
    pub(super) use super::wp_primary_selection_unstable_v1::zwp_primary_selection_device_manager_v1::ZwpPrimarySelectionDeviceManagerV1;
    pub(super) use super::wp_primary_selection_unstable_v1::zwp_primary_selection_device_v1::ZwpPrimarySelectionDeviceV1;
    pub(super) use super::wp_primary_selection_unstable_v1::zwp_primary_selection_offer_v1::ZwpPrimarySelectionOfferV1;
    pub(super) use super::wp_primary_selection_unstable_v1::zwp_primary_selection_source_v1::ZwpPrimarySelectionSourceV1;
    pub(super) use super::relative_pointer_unstable_v1::zwp_relative_pointer_manager_v1::ZwpRelativePointerManagerV1;
    pub(super) use super::relative_pointer_unstable_v1::zwp_relative_pointer_v1::ZwpRelativePointerV1;
    pub(super) use super::security_context_v1::wp_security_context_manager_v1::WpSecurityContextManagerV1;
    pub(super) use super::security_context_v1::wp_security_context_manager_v1::WpSecurityContextManagerV1Error;
    pub(super) use super::security_context_v1::wp_security_context_v1::WpSecurityContextV1;
    pub(super) use super::security_context_v1::wp_security_context_v1::WpSecurityContextV1Error;
    pub(super) use super::single_pixel_buffer_v1::wp_single_pixel_buffer_manager_v1::WpSinglePixelBufferManagerV1;
    pub(super) use super::tablet_v2::zwp_tablet_manager_v2::ZwpTabletManagerV2;
    pub(super) use super::tablet_v2::zwp_tablet_pad_dial_v2::ZwpTabletPadDialV2;
    pub(super) use super::tablet_v2::zwp_tablet_pad_group_v2::ZwpTabletPadGroupV2;
    pub(super) use super::tablet_v2::zwp_tablet_pad_ring_v2::ZwpTabletPadRingV2;
    pub(super) use super::tablet_v2::zwp_tablet_pad_ring_v2::ZwpTabletPadRingV2Source;
    pub(super) use super::tablet_v2::zwp_tablet_pad_strip_v2::ZwpTabletPadStripV2;
    pub(super) use super::tablet_v2::zwp_tablet_pad_strip_v2::ZwpTabletPadStripV2Source;
    pub(super) use super::tablet_v2::zwp_tablet_pad_v2::ZwpTabletPadV2;
    pub(super) use super::tablet_v2::zwp_tablet_pad_v2::ZwpTabletPadV2ButtonState;
    pub(super) use super::tablet_v2::zwp_tablet_seat_v2::ZwpTabletSeatV2;
    pub(super) use super::tablet_v2::zwp_tablet_tool_v2::ZwpTabletToolV2;
    pub(super) use super::tablet_v2::zwp_tablet_tool_v2::ZwpTabletToolV2Type;
    pub(super) use super::tablet_v2::zwp_tablet_tool_v2::ZwpTabletToolV2Capability;
    pub(super) use super::tablet_v2::zwp_tablet_tool_v2::ZwpTabletToolV2ButtonState;
    pub(super) use super::tablet_v2::zwp_tablet_tool_v2::ZwpTabletToolV2Error;
    pub(super) use super::tablet_v2::zwp_tablet_v2::ZwpTabletV2;
    pub(super) use super::tablet_v2::zwp_tablet_v2::ZwpTabletV2Bustype;
    pub(super) use super::tearing_control_v1::wp_tearing_control_manager_v1::WpTearingControlManagerV1;
    pub(super) use super::tearing_control_v1::wp_tearing_control_manager_v1::WpTearingControlManagerV1Error;
    pub(super) use super::tearing_control_v1::wp_tearing_control_v1::WpTearingControlV1;
    pub(super) use super::tearing_control_v1::wp_tearing_control_v1::WpTearingControlV1PresentationHint;
    pub(super) use super::text_input_unstable_v1::zwp_text_input_manager_v1::ZwpTextInputManagerV1;
    pub(super) use super::text_input_unstable_v1::zwp_text_input_v1::ZwpTextInputV1;
    pub(super) use super::text_input_unstable_v1::zwp_text_input_v1::ZwpTextInputV1ContentHint;
    pub(super) use super::text_input_unstable_v1::zwp_text_input_v1::ZwpTextInputV1ContentPurpose;
    pub(super) use super::text_input_unstable_v1::zwp_text_input_v1::ZwpTextInputV1PreeditStyle;
    pub(super) use super::text_input_unstable_v1::zwp_text_input_v1::ZwpTextInputV1TextDirection;
    pub(super) use super::text_input_unstable_v3::zwp_text_input_manager_v3::ZwpTextInputManagerV3;
    pub(super) use super::text_input_unstable_v3::zwp_text_input_v3::ZwpTextInputV3;
    pub(super) use super::text_input_unstable_v3::zwp_text_input_v3::ZwpTextInputV3ChangeCause;
    pub(super) use super::text_input_unstable_v3::zwp_text_input_v3::ZwpTextInputV3ContentHint;
    pub(super) use super::text_input_unstable_v3::zwp_text_input_v3::ZwpTextInputV3ContentPurpose;
    pub(super) use super::viewporter::wp_viewport::WpViewport;
    pub(super) use super::viewporter::wp_viewport::WpViewportError;
    pub(super) use super::viewporter::wp_viewporter::WpViewporter;
    pub(super) use super::viewporter::wp_viewporter::WpViewporterError;
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
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_device_v1::ZwlrDataControlDeviceV1;
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_device_v1::ZwlrDataControlDeviceV1Error;
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_manager_v1::ZwlrDataControlManagerV1;
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_offer_v1::ZwlrDataControlOfferV1;
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_source_v1::ZwlrDataControlSourceV1;
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_source_v1::ZwlrDataControlSourceV1Error;
    pub(super) use super::wlr_export_dmabuf_unstable_v1::zwlr_export_dmabuf_frame_v1::ZwlrExportDmabufFrameV1;
    pub(super) use super::wlr_export_dmabuf_unstable_v1::zwlr_export_dmabuf_frame_v1::ZwlrExportDmabufFrameV1Flags;
    pub(super) use super::wlr_export_dmabuf_unstable_v1::zwlr_export_dmabuf_frame_v1::ZwlrExportDmabufFrameV1CancelReason;
    pub(super) use super::wlr_export_dmabuf_unstable_v1::zwlr_export_dmabuf_manager_v1::ZwlrExportDmabufManagerV1;
    pub(super) use super::wlr_foreign_toplevel_management_unstable_v1::zwlr_foreign_toplevel_handle_v1::ZwlrForeignToplevelHandleV1;
    pub(super) use super::wlr_foreign_toplevel_management_unstable_v1::zwlr_foreign_toplevel_handle_v1::ZwlrForeignToplevelHandleV1State;
    pub(super) use super::wlr_foreign_toplevel_management_unstable_v1::zwlr_foreign_toplevel_handle_v1::ZwlrForeignToplevelHandleV1Error;
    pub(super) use super::wlr_foreign_toplevel_management_unstable_v1::zwlr_foreign_toplevel_manager_v1::ZwlrForeignToplevelManagerV1;
    pub(super) use super::wlr_gamma_control_unstable_v1::zwlr_gamma_control_manager_v1::ZwlrGammaControlManagerV1;
    pub(super) use super::wlr_gamma_control_unstable_v1::zwlr_gamma_control_v1::ZwlrGammaControlV1;
    pub(super) use super::wlr_gamma_control_unstable_v1::zwlr_gamma_control_v1::ZwlrGammaControlV1Error;
    pub(super) use super::wlr_input_inhibit_unstable_v1::zwlr_input_inhibit_manager_v1::ZwlrInputInhibitManagerV1;
    pub(super) use super::wlr_input_inhibit_unstable_v1::zwlr_input_inhibit_manager_v1::ZwlrInputInhibitManagerV1Error;
    pub(super) use super::wlr_input_inhibit_unstable_v1::zwlr_input_inhibitor_v1::ZwlrInputInhibitorV1;
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_shell_v1::ZwlrLayerShellV1;
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_shell_v1::ZwlrLayerShellV1Error;
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_shell_v1::ZwlrLayerShellV1Layer;
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_surface_v1::ZwlrLayerSurfaceV1;
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_surface_v1::ZwlrLayerSurfaceV1KeyboardInteractivity;
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_surface_v1::ZwlrLayerSurfaceV1Error;
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_surface_v1::ZwlrLayerSurfaceV1Anchor;
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_configuration_head_v1::ZwlrOutputConfigurationHeadV1;
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_configuration_head_v1::ZwlrOutputConfigurationHeadV1Error;
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_configuration_v1::ZwlrOutputConfigurationV1;
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_configuration_v1::ZwlrOutputConfigurationV1Error;
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_head_v1::ZwlrOutputHeadV1;
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_head_v1::ZwlrOutputHeadV1AdaptiveSyncState;
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_manager_v1::ZwlrOutputManagerV1;
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_mode_v1::ZwlrOutputModeV1;
    pub(super) use super::wlr_output_power_management_unstable_v1::zwlr_output_power_manager_v1::ZwlrOutputPowerManagerV1;
    pub(super) use super::wlr_output_power_management_unstable_v1::zwlr_output_power_v1::ZwlrOutputPowerV1;
    pub(super) use super::wlr_output_power_management_unstable_v1::zwlr_output_power_v1::ZwlrOutputPowerV1Mode;
    pub(super) use super::wlr_output_power_management_unstable_v1::zwlr_output_power_v1::ZwlrOutputPowerV1Error;
    pub(super) use super::wlr_screencopy_unstable_v1::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1;
    pub(super) use super::wlr_screencopy_unstable_v1::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1Error;
    pub(super) use super::wlr_screencopy_unstable_v1::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1Flags;
    pub(super) use super::wlr_screencopy_unstable_v1::zwlr_screencopy_manager_v1::ZwlrScreencopyManagerV1;
    pub(super) use super::wlr_virtual_pointer_unstable_v1::zwlr_virtual_pointer_manager_v1::ZwlrVirtualPointerManagerV1;
    pub(super) use super::wlr_virtual_pointer_unstable_v1::zwlr_virtual_pointer_v1::ZwlrVirtualPointerV1;
    pub(super) use super::wlr_virtual_pointer_unstable_v1::zwlr_virtual_pointer_v1::ZwlrVirtualPointerV1Error;
    pub(super) use super::xdg_activation_v1::xdg_activation_token_v1::XdgActivationTokenV1;
    pub(super) use super::xdg_activation_v1::xdg_activation_token_v1::XdgActivationTokenV1Error;
    pub(super) use super::xdg_activation_v1::xdg_activation_v1::XdgActivationV1;
    pub(super) use super::xdg_decoration_unstable_v1::zxdg_decoration_manager_v1::ZxdgDecorationManagerV1;
    pub(super) use super::xdg_decoration_unstable_v1::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1;
    pub(super) use super::xdg_decoration_unstable_v1::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1Error;
    pub(super) use super::xdg_decoration_unstable_v1::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1Mode;
    pub(super) use super::xdg_dialog_v1::xdg_dialog_v1::XdgDialogV1;
    pub(super) use super::xdg_dialog_v1::xdg_wm_dialog_v1::XdgWmDialogV1;
    pub(super) use super::xdg_dialog_v1::xdg_wm_dialog_v1::XdgWmDialogV1Error;
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_exported_v2::ZxdgExportedV2;
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_exporter_v2::ZxdgExporterV2;
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_exporter_v2::ZxdgExporterV2Error;
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_imported_v2::ZxdgImportedV2;
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_imported_v2::ZxdgImportedV2Error;
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_importer_v2::ZxdgImporterV2;
    pub(super) use super::xdg_output_unstable_v1::zxdg_output_manager_v1::ZxdgOutputManagerV1;
    pub(super) use super::xdg_output_unstable_v1::zxdg_output_v1::ZxdgOutputV1;
    pub(super) use super::xdg_shell::xdg_popup::XdgPopup;
    pub(super) use super::xdg_shell::xdg_popup::XdgPopupError;
    pub(super) use super::xdg_shell::xdg_positioner::XdgPositioner;
    pub(super) use super::xdg_shell::xdg_positioner::XdgPositionerError;
    pub(super) use super::xdg_shell::xdg_positioner::XdgPositionerAnchor;
    pub(super) use super::xdg_shell::xdg_positioner::XdgPositionerGravity;
    pub(super) use super::xdg_shell::xdg_positioner::XdgPositionerConstraintAdjustment;
    pub(super) use super::xdg_shell::xdg_surface::XdgSurface;
    pub(super) use super::xdg_shell::xdg_surface::XdgSurfaceError;
    pub(super) use super::xdg_shell::xdg_toplevel::XdgToplevel;
    pub(super) use super::xdg_shell::xdg_toplevel::XdgToplevelError;
    pub(super) use super::xdg_shell::xdg_toplevel::XdgToplevelResizeEdge;
    pub(super) use super::xdg_shell::xdg_toplevel::XdgToplevelState;
    pub(super) use super::xdg_shell::xdg_toplevel::XdgToplevelWmCapabilities;
    pub(super) use super::xdg_shell::xdg_wm_base::XdgWmBase;
    pub(super) use super::xdg_shell::xdg_wm_base::XdgWmBaseError;
    pub(super) use super::xdg_system_bell_v1::xdg_system_bell_v1::XdgSystemBellV1;
    pub(super) use super::xdg_toplevel_drag_v1::xdg_toplevel_drag_manager_v1::XdgToplevelDragManagerV1;
    pub(super) use super::xdg_toplevel_drag_v1::xdg_toplevel_drag_manager_v1::XdgToplevelDragManagerV1Error;
    pub(super) use super::xdg_toplevel_drag_v1::xdg_toplevel_drag_v1::XdgToplevelDragV1;
    pub(super) use super::xdg_toplevel_drag_v1::xdg_toplevel_drag_v1::XdgToplevelDragV1Error;
    pub(super) use super::xdg_toplevel_icon_v1::xdg_toplevel_icon_manager_v1::XdgToplevelIconManagerV1;
    pub(super) use super::xdg_toplevel_icon_v1::xdg_toplevel_icon_v1::XdgToplevelIconV1;
    pub(super) use super::xdg_toplevel_icon_v1::xdg_toplevel_icon_v1::XdgToplevelIconV1Error;
    pub(super) use super::xdg_toplevel_tag_v1::xdg_toplevel_tag_manager_v1::XdgToplevelTagManagerV1;
    pub(super) use super::xwayland_keyboard_grab_unstable_v1::zwp_xwayland_keyboard_grab_manager_v1::ZwpXwaylandKeyboardGrabManagerV1;
    pub(super) use super::xwayland_keyboard_grab_unstable_v1::zwp_xwayland_keyboard_grab_v1::ZwpXwaylandKeyboardGrabV1;
    pub(super) use super::xwayland_shell_v1::xwayland_shell_v1::XwaylandShellV1;
    pub(super) use super::xwayland_shell_v1::xwayland_shell_v1::XwaylandShellV1Error;
    pub(super) use super::xwayland_shell_v1::xwayland_surface_v1::XwaylandSurfaceV1;
    pub(super) use super::xwayland_shell_v1::xwayland_surface_v1::XwaylandSurfaceV1Error;

    use crate::generated_helper::prelude::*;

    pub(super) fn create_proxy_for_interface(state: &Rc<State>, interface: &str, version: u32) -> Result<Rc<dyn Proxy>, ObjectError> {
        static INTERFACES: phf::Map<&'static str, fn(&Rc<State>, u32) -> Result<Rc<dyn Proxy>, ObjectError>> = phf::phf_map! {
            "wp_alpha_modifier_surface_v1" => |s, v| {
                if v > WpAlphaModifierSurfaceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpAlphaModifierSurfaceV1, v));
                }
                Ok(WpAlphaModifierSurfaceV1::new(s, v))
            },
            "wp_alpha_modifier_v1" => |s, v| {
                if v > WpAlphaModifierV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpAlphaModifierV1, v));
                }
                Ok(WpAlphaModifierV1::new(s, v))
            },
            "wp_color_management_output_v1" => |s, v| {
                if v > WpColorManagementOutputV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpColorManagementOutputV1, v));
                }
                Ok(WpColorManagementOutputV1::new(s, v))
            },
            "wp_color_management_surface_feedback_v1" => |s, v| {
                if v > WpColorManagementSurfaceFeedbackV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpColorManagementSurfaceFeedbackV1, v));
                }
                Ok(WpColorManagementSurfaceFeedbackV1::new(s, v))
            },
            "wp_color_management_surface_v1" => |s, v| {
                if v > WpColorManagementSurfaceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpColorManagementSurfaceV1, v));
                }
                Ok(WpColorManagementSurfaceV1::new(s, v))
            },
            "wp_color_manager_v1" => |s, v| {
                if v > WpColorManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpColorManagerV1, v));
                }
                Ok(WpColorManagerV1::new(s, v))
            },
            "wp_image_description_creator_icc_v1" => |s, v| {
                if v > WpImageDescriptionCreatorIccV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpImageDescriptionCreatorIccV1, v));
                }
                Ok(WpImageDescriptionCreatorIccV1::new(s, v))
            },
            "wp_image_description_creator_params_v1" => |s, v| {
                if v > WpImageDescriptionCreatorParamsV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpImageDescriptionCreatorParamsV1, v));
                }
                Ok(WpImageDescriptionCreatorParamsV1::new(s, v))
            },
            "wp_image_description_info_v1" => |s, v| {
                if v > WpImageDescriptionInfoV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpImageDescriptionInfoV1, v));
                }
                Ok(WpImageDescriptionInfoV1::new(s, v))
            },
            "wp_image_description_v1" => |s, v| {
                if v > WpImageDescriptionV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpImageDescriptionV1, v));
                }
                Ok(WpImageDescriptionV1::new(s, v))
            },
            "wp_color_representation_manager_v1" => |s, v| {
                if v > WpColorRepresentationManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpColorRepresentationManagerV1, v));
                }
                Ok(WpColorRepresentationManagerV1::new(s, v))
            },
            "wp_color_representation_surface_v1" => |s, v| {
                if v > WpColorRepresentationSurfaceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpColorRepresentationSurfaceV1, v));
                }
                Ok(WpColorRepresentationSurfaceV1::new(s, v))
            },
            "wp_commit_timer_v1" => |s, v| {
                if v > WpCommitTimerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpCommitTimerV1, v));
                }
                Ok(WpCommitTimerV1::new(s, v))
            },
            "wp_commit_timing_manager_v1" => |s, v| {
                if v > WpCommitTimingManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpCommitTimingManagerV1, v));
                }
                Ok(WpCommitTimingManagerV1::new(s, v))
            },
            "wp_content_type_manager_v1" => |s, v| {
                if v > WpContentTypeManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpContentTypeManagerV1, v));
                }
                Ok(WpContentTypeManagerV1::new(s, v))
            },
            "wp_content_type_v1" => |s, v| {
                if v > WpContentTypeV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpContentTypeV1, v));
                }
                Ok(WpContentTypeV1::new(s, v))
            },
            "wp_cursor_shape_device_v1" => |s, v| {
                if v > WpCursorShapeDeviceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpCursorShapeDeviceV1, v));
                }
                Ok(WpCursorShapeDeviceV1::new(s, v))
            },
            "wp_cursor_shape_manager_v1" => |s, v| {
                if v > WpCursorShapeManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpCursorShapeManagerV1, v));
                }
                Ok(WpCursorShapeManagerV1::new(s, v))
            },
            "wp_drm_lease_connector_v1" => |s, v| {
                if v > WpDrmLeaseConnectorV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpDrmLeaseConnectorV1, v));
                }
                Ok(WpDrmLeaseConnectorV1::new(s, v))
            },
            "wp_drm_lease_device_v1" => |s, v| {
                if v > WpDrmLeaseDeviceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpDrmLeaseDeviceV1, v));
                }
                Ok(WpDrmLeaseDeviceV1::new(s, v))
            },
            "wp_drm_lease_request_v1" => |s, v| {
                if v > WpDrmLeaseRequestV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpDrmLeaseRequestV1, v));
                }
                Ok(WpDrmLeaseRequestV1::new(s, v))
            },
            "wp_drm_lease_v1" => |s, v| {
                if v > WpDrmLeaseV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpDrmLeaseV1, v));
                }
                Ok(WpDrmLeaseV1::new(s, v))
            },
            "ext_background_effect_manager_v1" => |s, v| {
                if v > ExtBackgroundEffectManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtBackgroundEffectManagerV1, v));
                }
                Ok(ExtBackgroundEffectManagerV1::new(s, v))
            },
            "ext_background_effect_surface_v1" => |s, v| {
                if v > ExtBackgroundEffectSurfaceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtBackgroundEffectSurfaceV1, v));
                }
                Ok(ExtBackgroundEffectSurfaceV1::new(s, v))
            },
            "ext_data_control_device_v1" => |s, v| {
                if v > ExtDataControlDeviceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtDataControlDeviceV1, v));
                }
                Ok(ExtDataControlDeviceV1::new(s, v))
            },
            "ext_data_control_manager_v1" => |s, v| {
                if v > ExtDataControlManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtDataControlManagerV1, v));
                }
                Ok(ExtDataControlManagerV1::new(s, v))
            },
            "ext_data_control_offer_v1" => |s, v| {
                if v > ExtDataControlOfferV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtDataControlOfferV1, v));
                }
                Ok(ExtDataControlOfferV1::new(s, v))
            },
            "ext_data_control_source_v1" => |s, v| {
                if v > ExtDataControlSourceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtDataControlSourceV1, v));
                }
                Ok(ExtDataControlSourceV1::new(s, v))
            },
            "ext_foreign_toplevel_handle_v1" => |s, v| {
                if v > ExtForeignToplevelHandleV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtForeignToplevelHandleV1, v));
                }
                Ok(ExtForeignToplevelHandleV1::new(s, v))
            },
            "ext_foreign_toplevel_list_v1" => |s, v| {
                if v > ExtForeignToplevelListV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtForeignToplevelListV1, v));
                }
                Ok(ExtForeignToplevelListV1::new(s, v))
            },
            "ext_idle_notification_v1" => |s, v| {
                if v > ExtIdleNotificationV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtIdleNotificationV1, v));
                }
                Ok(ExtIdleNotificationV1::new(s, v))
            },
            "ext_idle_notifier_v1" => |s, v| {
                if v > ExtIdleNotifierV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtIdleNotifierV1, v));
                }
                Ok(ExtIdleNotifierV1::new(s, v))
            },
            "ext_foreign_toplevel_image_capture_source_manager_v1" => |s, v| {
                if v > ExtForeignToplevelImageCaptureSourceManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtForeignToplevelImageCaptureSourceManagerV1, v));
                }
                Ok(ExtForeignToplevelImageCaptureSourceManagerV1::new(s, v))
            },
            "ext_image_capture_source_v1" => |s, v| {
                if v > ExtImageCaptureSourceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtImageCaptureSourceV1, v));
                }
                Ok(ExtImageCaptureSourceV1::new(s, v))
            },
            "ext_output_image_capture_source_manager_v1" => |s, v| {
                if v > ExtOutputImageCaptureSourceManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtOutputImageCaptureSourceManagerV1, v));
                }
                Ok(ExtOutputImageCaptureSourceManagerV1::new(s, v))
            },
            "ext_image_copy_capture_cursor_session_v1" => |s, v| {
                if v > ExtImageCopyCaptureCursorSessionV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtImageCopyCaptureCursorSessionV1, v));
                }
                Ok(ExtImageCopyCaptureCursorSessionV1::new(s, v))
            },
            "ext_image_copy_capture_frame_v1" => |s, v| {
                if v > ExtImageCopyCaptureFrameV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtImageCopyCaptureFrameV1, v));
                }
                Ok(ExtImageCopyCaptureFrameV1::new(s, v))
            },
            "ext_image_copy_capture_manager_v1" => |s, v| {
                if v > ExtImageCopyCaptureManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtImageCopyCaptureManagerV1, v));
                }
                Ok(ExtImageCopyCaptureManagerV1::new(s, v))
            },
            "ext_image_copy_capture_session_v1" => |s, v| {
                if v > ExtImageCopyCaptureSessionV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtImageCopyCaptureSessionV1, v));
                }
                Ok(ExtImageCopyCaptureSessionV1::new(s, v))
            },
            "ext_session_lock_manager_v1" => |s, v| {
                if v > ExtSessionLockManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtSessionLockManagerV1, v));
                }
                Ok(ExtSessionLockManagerV1::new(s, v))
            },
            "ext_session_lock_surface_v1" => |s, v| {
                if v > ExtSessionLockSurfaceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtSessionLockSurfaceV1, v));
                }
                Ok(ExtSessionLockSurfaceV1::new(s, v))
            },
            "ext_session_lock_v1" => |s, v| {
                if v > ExtSessionLockV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtSessionLockV1, v));
                }
                Ok(ExtSessionLockV1::new(s, v))
            },
            "ext_transient_seat_manager_v1" => |s, v| {
                if v > ExtTransientSeatManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtTransientSeatManagerV1, v));
                }
                Ok(ExtTransientSeatManagerV1::new(s, v))
            },
            "ext_transient_seat_v1" => |s, v| {
                if v > ExtTransientSeatV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtTransientSeatV1, v));
                }
                Ok(ExtTransientSeatV1::new(s, v))
            },
            "ext_workspace_group_handle_v1" => |s, v| {
                if v > ExtWorkspaceGroupHandleV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtWorkspaceGroupHandleV1, v));
                }
                Ok(ExtWorkspaceGroupHandleV1::new(s, v))
            },
            "ext_workspace_handle_v1" => |s, v| {
                if v > ExtWorkspaceHandleV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtWorkspaceHandleV1, v));
                }
                Ok(ExtWorkspaceHandleV1::new(s, v))
            },
            "ext_workspace_manager_v1" => |s, v| {
                if v > ExtWorkspaceManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtWorkspaceManagerV1, v));
                }
                Ok(ExtWorkspaceManagerV1::new(s, v))
            },
            "wp_fifo_manager_v1" => |s, v| {
                if v > WpFifoManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpFifoManagerV1, v));
                }
                Ok(WpFifoManagerV1::new(s, v))
            },
            "wp_fifo_v1" => |s, v| {
                if v > WpFifoV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpFifoV1, v));
                }
                Ok(WpFifoV1::new(s, v))
            },
            "wp_fractional_scale_manager_v1" => |s, v| {
                if v > WpFractionalScaleManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpFractionalScaleManagerV1, v));
                }
                Ok(WpFractionalScaleManagerV1::new(s, v))
            },
            "wp_fractional_scale_v1" => |s, v| {
                if v > WpFractionalScaleV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpFractionalScaleV1, v));
                }
                Ok(WpFractionalScaleV1::new(s, v))
            },
            "zwp_fullscreen_shell_mode_feedback_v1" => |s, v| {
                if v > ZwpFullscreenShellModeFeedbackV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpFullscreenShellModeFeedbackV1, v));
                }
                Ok(ZwpFullscreenShellModeFeedbackV1::new(s, v))
            },
            "zwp_fullscreen_shell_v1" => |s, v| {
                if v > ZwpFullscreenShellV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpFullscreenShellV1, v));
                }
                Ok(ZwpFullscreenShellV1::new(s, v))
            },
            "zwp_idle_inhibit_manager_v1" => |s, v| {
                if v > ZwpIdleInhibitManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpIdleInhibitManagerV1, v));
                }
                Ok(ZwpIdleInhibitManagerV1::new(s, v))
            },
            "zwp_idle_inhibitor_v1" => |s, v| {
                if v > ZwpIdleInhibitorV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpIdleInhibitorV1, v));
                }
                Ok(ZwpIdleInhibitorV1::new(s, v))
            },
            "zwp_input_method_context_v1" => |s, v| {
                if v > ZwpInputMethodContextV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpInputMethodContextV1, v));
                }
                Ok(ZwpInputMethodContextV1::new(s, v))
            },
            "zwp_input_method_v1" => |s, v| {
                if v > ZwpInputMethodV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpInputMethodV1, v));
                }
                Ok(ZwpInputMethodV1::new(s, v))
            },
            "zwp_input_panel_surface_v1" => |s, v| {
                if v > ZwpInputPanelSurfaceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpInputPanelSurfaceV1, v));
                }
                Ok(ZwpInputPanelSurfaceV1::new(s, v))
            },
            "zwp_input_panel_v1" => |s, v| {
                if v > ZwpInputPanelV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpInputPanelV1, v));
                }
                Ok(ZwpInputPanelV1::new(s, v))
            },
            "zwp_input_timestamps_manager_v1" => |s, v| {
                if v > ZwpInputTimestampsManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpInputTimestampsManagerV1, v));
                }
                Ok(ZwpInputTimestampsManagerV1::new(s, v))
            },
            "zwp_input_timestamps_v1" => |s, v| {
                if v > ZwpInputTimestampsV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpInputTimestampsV1, v));
                }
                Ok(ZwpInputTimestampsV1::new(s, v))
            },
            "zwp_keyboard_shortcuts_inhibit_manager_v1" => |s, v| {
                if v > ZwpKeyboardShortcutsInhibitManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpKeyboardShortcutsInhibitManagerV1, v));
                }
                Ok(ZwpKeyboardShortcutsInhibitManagerV1::new(s, v))
            },
            "zwp_keyboard_shortcuts_inhibitor_v1" => |s, v| {
                if v > ZwpKeyboardShortcutsInhibitorV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpKeyboardShortcutsInhibitorV1, v));
                }
                Ok(ZwpKeyboardShortcutsInhibitorV1::new(s, v))
            },
            "zwp_linux_buffer_params_v1" => |s, v| {
                if v > ZwpLinuxBufferParamsV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpLinuxBufferParamsV1, v));
                }
                Ok(ZwpLinuxBufferParamsV1::new(s, v))
            },
            "zwp_linux_dmabuf_feedback_v1" => |s, v| {
                if v > ZwpLinuxDmabufFeedbackV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpLinuxDmabufFeedbackV1, v));
                }
                Ok(ZwpLinuxDmabufFeedbackV1::new(s, v))
            },
            "zwp_linux_dmabuf_v1" => |s, v| {
                if v > ZwpLinuxDmabufV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpLinuxDmabufV1, v));
                }
                Ok(ZwpLinuxDmabufV1::new(s, v))
            },
            "wp_linux_drm_syncobj_manager_v1" => |s, v| {
                if v > WpLinuxDrmSyncobjManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpLinuxDrmSyncobjManagerV1, v));
                }
                Ok(WpLinuxDrmSyncobjManagerV1::new(s, v))
            },
            "wp_linux_drm_syncobj_surface_v1" => |s, v| {
                if v > WpLinuxDrmSyncobjSurfaceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpLinuxDrmSyncobjSurfaceV1, v));
                }
                Ok(WpLinuxDrmSyncobjSurfaceV1::new(s, v))
            },
            "wp_linux_drm_syncobj_timeline_v1" => |s, v| {
                if v > WpLinuxDrmSyncobjTimelineV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpLinuxDrmSyncobjTimelineV1, v));
                }
                Ok(WpLinuxDrmSyncobjTimelineV1::new(s, v))
            },
            "zwp_linux_buffer_release_v1" => |s, v| {
                if v > ZwpLinuxBufferReleaseV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpLinuxBufferReleaseV1, v));
                }
                Ok(ZwpLinuxBufferReleaseV1::new(s, v))
            },
            "zwp_linux_explicit_synchronization_v1" => |s, v| {
                if v > ZwpLinuxExplicitSynchronizationV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpLinuxExplicitSynchronizationV1, v));
                }
                Ok(ZwpLinuxExplicitSynchronizationV1::new(s, v))
            },
            "zwp_linux_surface_synchronization_v1" => |s, v| {
                if v > ZwpLinuxSurfaceSynchronizationV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpLinuxSurfaceSynchronizationV1, v));
                }
                Ok(ZwpLinuxSurfaceSynchronizationV1::new(s, v))
            },
            "zwp_confined_pointer_v1" => |s, v| {
                if v > ZwpConfinedPointerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpConfinedPointerV1, v));
                }
                Ok(ZwpConfinedPointerV1::new(s, v))
            },
            "zwp_locked_pointer_v1" => |s, v| {
                if v > ZwpLockedPointerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpLockedPointerV1, v));
                }
                Ok(ZwpLockedPointerV1::new(s, v))
            },
            "zwp_pointer_constraints_v1" => |s, v| {
                if v > ZwpPointerConstraintsV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpPointerConstraintsV1, v));
                }
                Ok(ZwpPointerConstraintsV1::new(s, v))
            },
            "zwp_pointer_gesture_hold_v1" => |s, v| {
                if v > ZwpPointerGestureHoldV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpPointerGestureHoldV1, v));
                }
                Ok(ZwpPointerGestureHoldV1::new(s, v))
            },
            "zwp_pointer_gesture_pinch_v1" => |s, v| {
                if v > ZwpPointerGesturePinchV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpPointerGesturePinchV1, v));
                }
                Ok(ZwpPointerGesturePinchV1::new(s, v))
            },
            "zwp_pointer_gesture_swipe_v1" => |s, v| {
                if v > ZwpPointerGestureSwipeV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpPointerGestureSwipeV1, v));
                }
                Ok(ZwpPointerGestureSwipeV1::new(s, v))
            },
            "zwp_pointer_gestures_v1" => |s, v| {
                if v > ZwpPointerGesturesV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpPointerGesturesV1, v));
                }
                Ok(ZwpPointerGesturesV1::new(s, v))
            },
            "wp_pointer_warp_v1" => |s, v| {
                if v > WpPointerWarpV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpPointerWarpV1, v));
                }
                Ok(WpPointerWarpV1::new(s, v))
            },
            "wp_presentation" => |s, v| {
                if v > WpPresentation::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpPresentation, v));
                }
                Ok(WpPresentation::new(s, v))
            },
            "wp_presentation_feedback" => |s, v| {
                if v > WpPresentationFeedback::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpPresentationFeedback, v));
                }
                Ok(WpPresentationFeedback::new(s, v))
            },
            "zwp_primary_selection_device_manager_v1" => |s, v| {
                if v > ZwpPrimarySelectionDeviceManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpPrimarySelectionDeviceManagerV1, v));
                }
                Ok(ZwpPrimarySelectionDeviceManagerV1::new(s, v))
            },
            "zwp_primary_selection_device_v1" => |s, v| {
                if v > ZwpPrimarySelectionDeviceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpPrimarySelectionDeviceV1, v));
                }
                Ok(ZwpPrimarySelectionDeviceV1::new(s, v))
            },
            "zwp_primary_selection_offer_v1" => |s, v| {
                if v > ZwpPrimarySelectionOfferV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpPrimarySelectionOfferV1, v));
                }
                Ok(ZwpPrimarySelectionOfferV1::new(s, v))
            },
            "zwp_primary_selection_source_v1" => |s, v| {
                if v > ZwpPrimarySelectionSourceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpPrimarySelectionSourceV1, v));
                }
                Ok(ZwpPrimarySelectionSourceV1::new(s, v))
            },
            "zwp_relative_pointer_manager_v1" => |s, v| {
                if v > ZwpRelativePointerManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpRelativePointerManagerV1, v));
                }
                Ok(ZwpRelativePointerManagerV1::new(s, v))
            },
            "zwp_relative_pointer_v1" => |s, v| {
                if v > ZwpRelativePointerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpRelativePointerV1, v));
                }
                Ok(ZwpRelativePointerV1::new(s, v))
            },
            "wp_security_context_manager_v1" => |s, v| {
                if v > WpSecurityContextManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpSecurityContextManagerV1, v));
                }
                Ok(WpSecurityContextManagerV1::new(s, v))
            },
            "wp_security_context_v1" => |s, v| {
                if v > WpSecurityContextV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpSecurityContextV1, v));
                }
                Ok(WpSecurityContextV1::new(s, v))
            },
            "wp_single_pixel_buffer_manager_v1" => |s, v| {
                if v > WpSinglePixelBufferManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpSinglePixelBufferManagerV1, v));
                }
                Ok(WpSinglePixelBufferManagerV1::new(s, v))
            },
            "zwp_tablet_manager_v2" => |s, v| {
                if v > ZwpTabletManagerV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTabletManagerV2, v));
                }
                Ok(ZwpTabletManagerV2::new(s, v))
            },
            "zwp_tablet_pad_dial_v2" => |s, v| {
                if v > ZwpTabletPadDialV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTabletPadDialV2, v));
                }
                Ok(ZwpTabletPadDialV2::new(s, v))
            },
            "zwp_tablet_pad_group_v2" => |s, v| {
                if v > ZwpTabletPadGroupV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTabletPadGroupV2, v));
                }
                Ok(ZwpTabletPadGroupV2::new(s, v))
            },
            "zwp_tablet_pad_ring_v2" => |s, v| {
                if v > ZwpTabletPadRingV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTabletPadRingV2, v));
                }
                Ok(ZwpTabletPadRingV2::new(s, v))
            },
            "zwp_tablet_pad_strip_v2" => |s, v| {
                if v > ZwpTabletPadStripV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTabletPadStripV2, v));
                }
                Ok(ZwpTabletPadStripV2::new(s, v))
            },
            "zwp_tablet_pad_v2" => |s, v| {
                if v > ZwpTabletPadV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTabletPadV2, v));
                }
                Ok(ZwpTabletPadV2::new(s, v))
            },
            "zwp_tablet_seat_v2" => |s, v| {
                if v > ZwpTabletSeatV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTabletSeatV2, v));
                }
                Ok(ZwpTabletSeatV2::new(s, v))
            },
            "zwp_tablet_tool_v2" => |s, v| {
                if v > ZwpTabletToolV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTabletToolV2, v));
                }
                Ok(ZwpTabletToolV2::new(s, v))
            },
            "zwp_tablet_v2" => |s, v| {
                if v > ZwpTabletV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTabletV2, v));
                }
                Ok(ZwpTabletV2::new(s, v))
            },
            "wp_tearing_control_manager_v1" => |s, v| {
                if v > WpTearingControlManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpTearingControlManagerV1, v));
                }
                Ok(WpTearingControlManagerV1::new(s, v))
            },
            "wp_tearing_control_v1" => |s, v| {
                if v > WpTearingControlV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpTearingControlV1, v));
                }
                Ok(WpTearingControlV1::new(s, v))
            },
            "zwp_text_input_manager_v1" => |s, v| {
                if v > ZwpTextInputManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTextInputManagerV1, v));
                }
                Ok(ZwpTextInputManagerV1::new(s, v))
            },
            "zwp_text_input_v1" => |s, v| {
                if v > ZwpTextInputV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTextInputV1, v));
                }
                Ok(ZwpTextInputV1::new(s, v))
            },
            "zwp_text_input_manager_v3" => |s, v| {
                if v > ZwpTextInputManagerV3::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTextInputManagerV3, v));
                }
                Ok(ZwpTextInputManagerV3::new(s, v))
            },
            "zwp_text_input_v3" => |s, v| {
                if v > ZwpTextInputV3::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTextInputV3, v));
                }
                Ok(ZwpTextInputV3::new(s, v))
            },
            "wp_viewport" => |s, v| {
                if v > WpViewport::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpViewport, v));
                }
                Ok(WpViewport::new(s, v))
            },
            "wp_viewporter" => |s, v| {
                if v > WpViewporter::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpViewporter, v));
                }
                Ok(WpViewporter::new(s, v))
            },
            "wl_buffer" => |s, v| {
                if v > WlBuffer::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlBuffer, v));
                }
                Ok(WlBuffer::new(s, v))
            },
            "wl_callback" => |s, v| {
                if v > WlCallback::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlCallback, v));
                }
                Ok(WlCallback::new(s, v))
            },
            "wl_compositor" => |s, v| {
                if v > WlCompositor::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlCompositor, v));
                }
                Ok(WlCompositor::new(s, v))
            },
            "wl_data_device" => |s, v| {
                if v > WlDataDevice::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlDataDevice, v));
                }
                Ok(WlDataDevice::new(s, v))
            },
            "wl_data_device_manager" => |s, v| {
                if v > WlDataDeviceManager::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlDataDeviceManager, v));
                }
                Ok(WlDataDeviceManager::new(s, v))
            },
            "wl_data_offer" => |s, v| {
                if v > WlDataOffer::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlDataOffer, v));
                }
                Ok(WlDataOffer::new(s, v))
            },
            "wl_data_source" => |s, v| {
                if v > WlDataSource::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlDataSource, v));
                }
                Ok(WlDataSource::new(s, v))
            },
            "wl_display" => |s, v| {
                if v > WlDisplay::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlDisplay, v));
                }
                Ok(WlDisplay::new(s, v))
            },
            "wl_fixes" => |s, v| {
                if v > WlFixes::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlFixes, v));
                }
                Ok(WlFixes::new(s, v))
            },
            "wl_keyboard" => |s, v| {
                if v > WlKeyboard::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlKeyboard, v));
                }
                Ok(WlKeyboard::new(s, v))
            },
            "wl_output" => |s, v| {
                if v > WlOutput::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlOutput, v));
                }
                Ok(WlOutput::new(s, v))
            },
            "wl_pointer" => |s, v| {
                if v > WlPointer::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlPointer, v));
                }
                Ok(WlPointer::new(s, v))
            },
            "wl_region" => |s, v| {
                if v > WlRegion::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlRegion, v));
                }
                Ok(WlRegion::new(s, v))
            },
            "wl_registry" => |s, v| {
                if v > WlRegistry::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlRegistry, v));
                }
                Ok(WlRegistry::new(s, v))
            },
            "wl_seat" => |s, v| {
                if v > WlSeat::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlSeat, v));
                }
                Ok(WlSeat::new(s, v))
            },
            "wl_shell" => |s, v| {
                if v > WlShell::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlShell, v));
                }
                Ok(WlShell::new(s, v))
            },
            "wl_shell_surface" => |s, v| {
                if v > WlShellSurface::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlShellSurface, v));
                }
                Ok(WlShellSurface::new(s, v))
            },
            "wl_shm" => |s, v| {
                if v > WlShm::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlShm, v));
                }
                Ok(WlShm::new(s, v))
            },
            "wl_shm_pool" => |s, v| {
                if v > WlShmPool::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlShmPool, v));
                }
                Ok(WlShmPool::new(s, v))
            },
            "wl_subcompositor" => |s, v| {
                if v > WlSubcompositor::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlSubcompositor, v));
                }
                Ok(WlSubcompositor::new(s, v))
            },
            "wl_subsurface" => |s, v| {
                if v > WlSubsurface::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlSubsurface, v));
                }
                Ok(WlSubsurface::new(s, v))
            },
            "wl_surface" => |s, v| {
                if v > WlSurface::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlSurface, v));
                }
                Ok(WlSurface::new(s, v))
            },
            "wl_touch" => |s, v| {
                if v > WlTouch::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlTouch, v));
                }
                Ok(WlTouch::new(s, v))
            },
            "zwlr_data_control_device_v1" => |s, v| {
                if v > ZwlrDataControlDeviceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrDataControlDeviceV1, v));
                }
                Ok(ZwlrDataControlDeviceV1::new(s, v))
            },
            "zwlr_data_control_manager_v1" => |s, v| {
                if v > ZwlrDataControlManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrDataControlManagerV1, v));
                }
                Ok(ZwlrDataControlManagerV1::new(s, v))
            },
            "zwlr_data_control_offer_v1" => |s, v| {
                if v > ZwlrDataControlOfferV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrDataControlOfferV1, v));
                }
                Ok(ZwlrDataControlOfferV1::new(s, v))
            },
            "zwlr_data_control_source_v1" => |s, v| {
                if v > ZwlrDataControlSourceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrDataControlSourceV1, v));
                }
                Ok(ZwlrDataControlSourceV1::new(s, v))
            },
            "zwlr_export_dmabuf_frame_v1" => |s, v| {
                if v > ZwlrExportDmabufFrameV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrExportDmabufFrameV1, v));
                }
                Ok(ZwlrExportDmabufFrameV1::new(s, v))
            },
            "zwlr_export_dmabuf_manager_v1" => |s, v| {
                if v > ZwlrExportDmabufManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrExportDmabufManagerV1, v));
                }
                Ok(ZwlrExportDmabufManagerV1::new(s, v))
            },
            "zwlr_foreign_toplevel_handle_v1" => |s, v| {
                if v > ZwlrForeignToplevelHandleV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrForeignToplevelHandleV1, v));
                }
                Ok(ZwlrForeignToplevelHandleV1::new(s, v))
            },
            "zwlr_foreign_toplevel_manager_v1" => |s, v| {
                if v > ZwlrForeignToplevelManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrForeignToplevelManagerV1, v));
                }
                Ok(ZwlrForeignToplevelManagerV1::new(s, v))
            },
            "zwlr_gamma_control_manager_v1" => |s, v| {
                if v > ZwlrGammaControlManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrGammaControlManagerV1, v));
                }
                Ok(ZwlrGammaControlManagerV1::new(s, v))
            },
            "zwlr_gamma_control_v1" => |s, v| {
                if v > ZwlrGammaControlV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrGammaControlV1, v));
                }
                Ok(ZwlrGammaControlV1::new(s, v))
            },
            "zwlr_input_inhibit_manager_v1" => |s, v| {
                if v > ZwlrInputInhibitManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrInputInhibitManagerV1, v));
                }
                Ok(ZwlrInputInhibitManagerV1::new(s, v))
            },
            "zwlr_input_inhibitor_v1" => |s, v| {
                if v > ZwlrInputInhibitorV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrInputInhibitorV1, v));
                }
                Ok(ZwlrInputInhibitorV1::new(s, v))
            },
            "zwlr_layer_shell_v1" => |s, v| {
                if v > ZwlrLayerShellV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrLayerShellV1, v));
                }
                Ok(ZwlrLayerShellV1::new(s, v))
            },
            "zwlr_layer_surface_v1" => |s, v| {
                if v > ZwlrLayerSurfaceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrLayerSurfaceV1, v));
                }
                Ok(ZwlrLayerSurfaceV1::new(s, v))
            },
            "zwlr_output_configuration_head_v1" => |s, v| {
                if v > ZwlrOutputConfigurationHeadV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrOutputConfigurationHeadV1, v));
                }
                Ok(ZwlrOutputConfigurationHeadV1::new(s, v))
            },
            "zwlr_output_configuration_v1" => |s, v| {
                if v > ZwlrOutputConfigurationV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrOutputConfigurationV1, v));
                }
                Ok(ZwlrOutputConfigurationV1::new(s, v))
            },
            "zwlr_output_head_v1" => |s, v| {
                if v > ZwlrOutputHeadV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrOutputHeadV1, v));
                }
                Ok(ZwlrOutputHeadV1::new(s, v))
            },
            "zwlr_output_manager_v1" => |s, v| {
                if v > ZwlrOutputManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrOutputManagerV1, v));
                }
                Ok(ZwlrOutputManagerV1::new(s, v))
            },
            "zwlr_output_mode_v1" => |s, v| {
                if v > ZwlrOutputModeV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrOutputModeV1, v));
                }
                Ok(ZwlrOutputModeV1::new(s, v))
            },
            "zwlr_output_power_manager_v1" => |s, v| {
                if v > ZwlrOutputPowerManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrOutputPowerManagerV1, v));
                }
                Ok(ZwlrOutputPowerManagerV1::new(s, v))
            },
            "zwlr_output_power_v1" => |s, v| {
                if v > ZwlrOutputPowerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrOutputPowerV1, v));
                }
                Ok(ZwlrOutputPowerV1::new(s, v))
            },
            "zwlr_screencopy_frame_v1" => |s, v| {
                if v > ZwlrScreencopyFrameV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrScreencopyFrameV1, v));
                }
                Ok(ZwlrScreencopyFrameV1::new(s, v))
            },
            "zwlr_screencopy_manager_v1" => |s, v| {
                if v > ZwlrScreencopyManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrScreencopyManagerV1, v));
                }
                Ok(ZwlrScreencopyManagerV1::new(s, v))
            },
            "zwlr_virtual_pointer_manager_v1" => |s, v| {
                if v > ZwlrVirtualPointerManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrVirtualPointerManagerV1, v));
                }
                Ok(ZwlrVirtualPointerManagerV1::new(s, v))
            },
            "zwlr_virtual_pointer_v1" => |s, v| {
                if v > ZwlrVirtualPointerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrVirtualPointerV1, v));
                }
                Ok(ZwlrVirtualPointerV1::new(s, v))
            },
            "xdg_activation_token_v1" => |s, v| {
                if v > XdgActivationTokenV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgActivationTokenV1, v));
                }
                Ok(XdgActivationTokenV1::new(s, v))
            },
            "xdg_activation_v1" => |s, v| {
                if v > XdgActivationV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgActivationV1, v));
                }
                Ok(XdgActivationV1::new(s, v))
            },
            "zxdg_decoration_manager_v1" => |s, v| {
                if v > ZxdgDecorationManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZxdgDecorationManagerV1, v));
                }
                Ok(ZxdgDecorationManagerV1::new(s, v))
            },
            "zxdg_toplevel_decoration_v1" => |s, v| {
                if v > ZxdgToplevelDecorationV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZxdgToplevelDecorationV1, v));
                }
                Ok(ZxdgToplevelDecorationV1::new(s, v))
            },
            "xdg_dialog_v1" => |s, v| {
                if v > XdgDialogV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgDialogV1, v));
                }
                Ok(XdgDialogV1::new(s, v))
            },
            "xdg_wm_dialog_v1" => |s, v| {
                if v > XdgWmDialogV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgWmDialogV1, v));
                }
                Ok(XdgWmDialogV1::new(s, v))
            },
            "zxdg_exported_v2" => |s, v| {
                if v > ZxdgExportedV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZxdgExportedV2, v));
                }
                Ok(ZxdgExportedV2::new(s, v))
            },
            "zxdg_exporter_v2" => |s, v| {
                if v > ZxdgExporterV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZxdgExporterV2, v));
                }
                Ok(ZxdgExporterV2::new(s, v))
            },
            "zxdg_imported_v2" => |s, v| {
                if v > ZxdgImportedV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZxdgImportedV2, v));
                }
                Ok(ZxdgImportedV2::new(s, v))
            },
            "zxdg_importer_v2" => |s, v| {
                if v > ZxdgImporterV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZxdgImporterV2, v));
                }
                Ok(ZxdgImporterV2::new(s, v))
            },
            "zxdg_output_manager_v1" => |s, v| {
                if v > ZxdgOutputManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZxdgOutputManagerV1, v));
                }
                Ok(ZxdgOutputManagerV1::new(s, v))
            },
            "zxdg_output_v1" => |s, v| {
                if v > ZxdgOutputV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZxdgOutputV1, v));
                }
                Ok(ZxdgOutputV1::new(s, v))
            },
            "xdg_popup" => |s, v| {
                if v > XdgPopup::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgPopup, v));
                }
                Ok(XdgPopup::new(s, v))
            },
            "xdg_positioner" => |s, v| {
                if v > XdgPositioner::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgPositioner, v));
                }
                Ok(XdgPositioner::new(s, v))
            },
            "xdg_surface" => |s, v| {
                if v > XdgSurface::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgSurface, v));
                }
                Ok(XdgSurface::new(s, v))
            },
            "xdg_toplevel" => |s, v| {
                if v > XdgToplevel::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgToplevel, v));
                }
                Ok(XdgToplevel::new(s, v))
            },
            "xdg_wm_base" => |s, v| {
                if v > XdgWmBase::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgWmBase, v));
                }
                Ok(XdgWmBase::new(s, v))
            },
            "xdg_system_bell_v1" => |s, v| {
                if v > XdgSystemBellV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgSystemBellV1, v));
                }
                Ok(XdgSystemBellV1::new(s, v))
            },
            "xdg_toplevel_drag_manager_v1" => |s, v| {
                if v > XdgToplevelDragManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgToplevelDragManagerV1, v));
                }
                Ok(XdgToplevelDragManagerV1::new(s, v))
            },
            "xdg_toplevel_drag_v1" => |s, v| {
                if v > XdgToplevelDragV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgToplevelDragV1, v));
                }
                Ok(XdgToplevelDragV1::new(s, v))
            },
            "xdg_toplevel_icon_manager_v1" => |s, v| {
                if v > XdgToplevelIconManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgToplevelIconManagerV1, v));
                }
                Ok(XdgToplevelIconManagerV1::new(s, v))
            },
            "xdg_toplevel_icon_v1" => |s, v| {
                if v > XdgToplevelIconV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgToplevelIconV1, v));
                }
                Ok(XdgToplevelIconV1::new(s, v))
            },
            "xdg_toplevel_tag_manager_v1" => |s, v| {
                if v > XdgToplevelTagManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgToplevelTagManagerV1, v));
                }
                Ok(XdgToplevelTagManagerV1::new(s, v))
            },
            "zwp_xwayland_keyboard_grab_manager_v1" => |s, v| {
                if v > ZwpXwaylandKeyboardGrabManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpXwaylandKeyboardGrabManagerV1, v));
                }
                Ok(ZwpXwaylandKeyboardGrabManagerV1::new(s, v))
            },
            "zwp_xwayland_keyboard_grab_v1" => |s, v| {
                if v > ZwpXwaylandKeyboardGrabV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpXwaylandKeyboardGrabV1, v));
                }
                Ok(ZwpXwaylandKeyboardGrabV1::new(s, v))
            },
            "xwayland_shell_v1" => |s, v| {
                if v > XwaylandShellV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XwaylandShellV1, v));
                }
                Ok(XwaylandShellV1::new(s, v))
            },
            "xwayland_surface_v1" => |s, v| {
                if v > XwaylandSurfaceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XwaylandSurfaceV1, v));
                }
                Ok(XwaylandSurfaceV1::new(s, v))
            },
        };
        INTERFACES
            .get(interface)
            .ok_or(ObjectError::UnsupportedInterface(interface.to_string()))
            .and_then(|i| i(state, version))
    }

    pub(super) fn proxy_interface(interface: &str) -> Option<ProxyInterface> {
        static INTERFACES: phf::Map<&'static str, ProxyInterface> = phf::phf_map! {
            "wp_alpha_modifier_surface_v1" => ProxyInterface::WpAlphaModifierSurfaceV1,
            "wp_alpha_modifier_v1" => ProxyInterface::WpAlphaModifierV1,
            "wp_color_management_output_v1" => ProxyInterface::WpColorManagementOutputV1,
            "wp_color_management_surface_feedback_v1" => ProxyInterface::WpColorManagementSurfaceFeedbackV1,
            "wp_color_management_surface_v1" => ProxyInterface::WpColorManagementSurfaceV1,
            "wp_color_manager_v1" => ProxyInterface::WpColorManagerV1,
            "wp_image_description_creator_icc_v1" => ProxyInterface::WpImageDescriptionCreatorIccV1,
            "wp_image_description_creator_params_v1" => ProxyInterface::WpImageDescriptionCreatorParamsV1,
            "wp_image_description_info_v1" => ProxyInterface::WpImageDescriptionInfoV1,
            "wp_image_description_v1" => ProxyInterface::WpImageDescriptionV1,
            "wp_color_representation_manager_v1" => ProxyInterface::WpColorRepresentationManagerV1,
            "wp_color_representation_surface_v1" => ProxyInterface::WpColorRepresentationSurfaceV1,
            "wp_commit_timer_v1" => ProxyInterface::WpCommitTimerV1,
            "wp_commit_timing_manager_v1" => ProxyInterface::WpCommitTimingManagerV1,
            "wp_content_type_manager_v1" => ProxyInterface::WpContentTypeManagerV1,
            "wp_content_type_v1" => ProxyInterface::WpContentTypeV1,
            "wp_cursor_shape_device_v1" => ProxyInterface::WpCursorShapeDeviceV1,
            "wp_cursor_shape_manager_v1" => ProxyInterface::WpCursorShapeManagerV1,
            "wp_drm_lease_connector_v1" => ProxyInterface::WpDrmLeaseConnectorV1,
            "wp_drm_lease_device_v1" => ProxyInterface::WpDrmLeaseDeviceV1,
            "wp_drm_lease_request_v1" => ProxyInterface::WpDrmLeaseRequestV1,
            "wp_drm_lease_v1" => ProxyInterface::WpDrmLeaseV1,
            "ext_background_effect_manager_v1" => ProxyInterface::ExtBackgroundEffectManagerV1,
            "ext_background_effect_surface_v1" => ProxyInterface::ExtBackgroundEffectSurfaceV1,
            "ext_data_control_device_v1" => ProxyInterface::ExtDataControlDeviceV1,
            "ext_data_control_manager_v1" => ProxyInterface::ExtDataControlManagerV1,
            "ext_data_control_offer_v1" => ProxyInterface::ExtDataControlOfferV1,
            "ext_data_control_source_v1" => ProxyInterface::ExtDataControlSourceV1,
            "ext_foreign_toplevel_handle_v1" => ProxyInterface::ExtForeignToplevelHandleV1,
            "ext_foreign_toplevel_list_v1" => ProxyInterface::ExtForeignToplevelListV1,
            "ext_idle_notification_v1" => ProxyInterface::ExtIdleNotificationV1,
            "ext_idle_notifier_v1" => ProxyInterface::ExtIdleNotifierV1,
            "ext_foreign_toplevel_image_capture_source_manager_v1" => ProxyInterface::ExtForeignToplevelImageCaptureSourceManagerV1,
            "ext_image_capture_source_v1" => ProxyInterface::ExtImageCaptureSourceV1,
            "ext_output_image_capture_source_manager_v1" => ProxyInterface::ExtOutputImageCaptureSourceManagerV1,
            "ext_image_copy_capture_cursor_session_v1" => ProxyInterface::ExtImageCopyCaptureCursorSessionV1,
            "ext_image_copy_capture_frame_v1" => ProxyInterface::ExtImageCopyCaptureFrameV1,
            "ext_image_copy_capture_manager_v1" => ProxyInterface::ExtImageCopyCaptureManagerV1,
            "ext_image_copy_capture_session_v1" => ProxyInterface::ExtImageCopyCaptureSessionV1,
            "ext_session_lock_manager_v1" => ProxyInterface::ExtSessionLockManagerV1,
            "ext_session_lock_surface_v1" => ProxyInterface::ExtSessionLockSurfaceV1,
            "ext_session_lock_v1" => ProxyInterface::ExtSessionLockV1,
            "ext_transient_seat_manager_v1" => ProxyInterface::ExtTransientSeatManagerV1,
            "ext_transient_seat_v1" => ProxyInterface::ExtTransientSeatV1,
            "ext_workspace_group_handle_v1" => ProxyInterface::ExtWorkspaceGroupHandleV1,
            "ext_workspace_handle_v1" => ProxyInterface::ExtWorkspaceHandleV1,
            "ext_workspace_manager_v1" => ProxyInterface::ExtWorkspaceManagerV1,
            "wp_fifo_manager_v1" => ProxyInterface::WpFifoManagerV1,
            "wp_fifo_v1" => ProxyInterface::WpFifoV1,
            "wp_fractional_scale_manager_v1" => ProxyInterface::WpFractionalScaleManagerV1,
            "wp_fractional_scale_v1" => ProxyInterface::WpFractionalScaleV1,
            "zwp_fullscreen_shell_mode_feedback_v1" => ProxyInterface::ZwpFullscreenShellModeFeedbackV1,
            "zwp_fullscreen_shell_v1" => ProxyInterface::ZwpFullscreenShellV1,
            "zwp_idle_inhibit_manager_v1" => ProxyInterface::ZwpIdleInhibitManagerV1,
            "zwp_idle_inhibitor_v1" => ProxyInterface::ZwpIdleInhibitorV1,
            "zwp_input_method_context_v1" => ProxyInterface::ZwpInputMethodContextV1,
            "zwp_input_method_v1" => ProxyInterface::ZwpInputMethodV1,
            "zwp_input_panel_surface_v1" => ProxyInterface::ZwpInputPanelSurfaceV1,
            "zwp_input_panel_v1" => ProxyInterface::ZwpInputPanelV1,
            "zwp_input_timestamps_manager_v1" => ProxyInterface::ZwpInputTimestampsManagerV1,
            "zwp_input_timestamps_v1" => ProxyInterface::ZwpInputTimestampsV1,
            "zwp_keyboard_shortcuts_inhibit_manager_v1" => ProxyInterface::ZwpKeyboardShortcutsInhibitManagerV1,
            "zwp_keyboard_shortcuts_inhibitor_v1" => ProxyInterface::ZwpKeyboardShortcutsInhibitorV1,
            "zwp_linux_buffer_params_v1" => ProxyInterface::ZwpLinuxBufferParamsV1,
            "zwp_linux_dmabuf_feedback_v1" => ProxyInterface::ZwpLinuxDmabufFeedbackV1,
            "zwp_linux_dmabuf_v1" => ProxyInterface::ZwpLinuxDmabufV1,
            "wp_linux_drm_syncobj_manager_v1" => ProxyInterface::WpLinuxDrmSyncobjManagerV1,
            "wp_linux_drm_syncobj_surface_v1" => ProxyInterface::WpLinuxDrmSyncobjSurfaceV1,
            "wp_linux_drm_syncobj_timeline_v1" => ProxyInterface::WpLinuxDrmSyncobjTimelineV1,
            "zwp_linux_buffer_release_v1" => ProxyInterface::ZwpLinuxBufferReleaseV1,
            "zwp_linux_explicit_synchronization_v1" => ProxyInterface::ZwpLinuxExplicitSynchronizationV1,
            "zwp_linux_surface_synchronization_v1" => ProxyInterface::ZwpLinuxSurfaceSynchronizationV1,
            "zwp_confined_pointer_v1" => ProxyInterface::ZwpConfinedPointerV1,
            "zwp_locked_pointer_v1" => ProxyInterface::ZwpLockedPointerV1,
            "zwp_pointer_constraints_v1" => ProxyInterface::ZwpPointerConstraintsV1,
            "zwp_pointer_gesture_hold_v1" => ProxyInterface::ZwpPointerGestureHoldV1,
            "zwp_pointer_gesture_pinch_v1" => ProxyInterface::ZwpPointerGesturePinchV1,
            "zwp_pointer_gesture_swipe_v1" => ProxyInterface::ZwpPointerGestureSwipeV1,
            "zwp_pointer_gestures_v1" => ProxyInterface::ZwpPointerGesturesV1,
            "wp_pointer_warp_v1" => ProxyInterface::WpPointerWarpV1,
            "wp_presentation" => ProxyInterface::WpPresentation,
            "wp_presentation_feedback" => ProxyInterface::WpPresentationFeedback,
            "zwp_primary_selection_device_manager_v1" => ProxyInterface::ZwpPrimarySelectionDeviceManagerV1,
            "zwp_primary_selection_device_v1" => ProxyInterface::ZwpPrimarySelectionDeviceV1,
            "zwp_primary_selection_offer_v1" => ProxyInterface::ZwpPrimarySelectionOfferV1,
            "zwp_primary_selection_source_v1" => ProxyInterface::ZwpPrimarySelectionSourceV1,
            "zwp_relative_pointer_manager_v1" => ProxyInterface::ZwpRelativePointerManagerV1,
            "zwp_relative_pointer_v1" => ProxyInterface::ZwpRelativePointerV1,
            "wp_security_context_manager_v1" => ProxyInterface::WpSecurityContextManagerV1,
            "wp_security_context_v1" => ProxyInterface::WpSecurityContextV1,
            "wp_single_pixel_buffer_manager_v1" => ProxyInterface::WpSinglePixelBufferManagerV1,
            "zwp_tablet_manager_v2" => ProxyInterface::ZwpTabletManagerV2,
            "zwp_tablet_pad_dial_v2" => ProxyInterface::ZwpTabletPadDialV2,
            "zwp_tablet_pad_group_v2" => ProxyInterface::ZwpTabletPadGroupV2,
            "zwp_tablet_pad_ring_v2" => ProxyInterface::ZwpTabletPadRingV2,
            "zwp_tablet_pad_strip_v2" => ProxyInterface::ZwpTabletPadStripV2,
            "zwp_tablet_pad_v2" => ProxyInterface::ZwpTabletPadV2,
            "zwp_tablet_seat_v2" => ProxyInterface::ZwpTabletSeatV2,
            "zwp_tablet_tool_v2" => ProxyInterface::ZwpTabletToolV2,
            "zwp_tablet_v2" => ProxyInterface::ZwpTabletV2,
            "wp_tearing_control_manager_v1" => ProxyInterface::WpTearingControlManagerV1,
            "wp_tearing_control_v1" => ProxyInterface::WpTearingControlV1,
            "zwp_text_input_manager_v1" => ProxyInterface::ZwpTextInputManagerV1,
            "zwp_text_input_v1" => ProxyInterface::ZwpTextInputV1,
            "zwp_text_input_manager_v3" => ProxyInterface::ZwpTextInputManagerV3,
            "zwp_text_input_v3" => ProxyInterface::ZwpTextInputV3,
            "wp_viewport" => ProxyInterface::WpViewport,
            "wp_viewporter" => ProxyInterface::WpViewporter,
            "wl_buffer" => ProxyInterface::WlBuffer,
            "wl_callback" => ProxyInterface::WlCallback,
            "wl_compositor" => ProxyInterface::WlCompositor,
            "wl_data_device" => ProxyInterface::WlDataDevice,
            "wl_data_device_manager" => ProxyInterface::WlDataDeviceManager,
            "wl_data_offer" => ProxyInterface::WlDataOffer,
            "wl_data_source" => ProxyInterface::WlDataSource,
            "wl_display" => ProxyInterface::WlDisplay,
            "wl_fixes" => ProxyInterface::WlFixes,
            "wl_keyboard" => ProxyInterface::WlKeyboard,
            "wl_output" => ProxyInterface::WlOutput,
            "wl_pointer" => ProxyInterface::WlPointer,
            "wl_region" => ProxyInterface::WlRegion,
            "wl_registry" => ProxyInterface::WlRegistry,
            "wl_seat" => ProxyInterface::WlSeat,
            "wl_shell" => ProxyInterface::WlShell,
            "wl_shell_surface" => ProxyInterface::WlShellSurface,
            "wl_shm" => ProxyInterface::WlShm,
            "wl_shm_pool" => ProxyInterface::WlShmPool,
            "wl_subcompositor" => ProxyInterface::WlSubcompositor,
            "wl_subsurface" => ProxyInterface::WlSubsurface,
            "wl_surface" => ProxyInterface::WlSurface,
            "wl_touch" => ProxyInterface::WlTouch,
            "zwlr_data_control_device_v1" => ProxyInterface::ZwlrDataControlDeviceV1,
            "zwlr_data_control_manager_v1" => ProxyInterface::ZwlrDataControlManagerV1,
            "zwlr_data_control_offer_v1" => ProxyInterface::ZwlrDataControlOfferV1,
            "zwlr_data_control_source_v1" => ProxyInterface::ZwlrDataControlSourceV1,
            "zwlr_export_dmabuf_frame_v1" => ProxyInterface::ZwlrExportDmabufFrameV1,
            "zwlr_export_dmabuf_manager_v1" => ProxyInterface::ZwlrExportDmabufManagerV1,
            "zwlr_foreign_toplevel_handle_v1" => ProxyInterface::ZwlrForeignToplevelHandleV1,
            "zwlr_foreign_toplevel_manager_v1" => ProxyInterface::ZwlrForeignToplevelManagerV1,
            "zwlr_gamma_control_manager_v1" => ProxyInterface::ZwlrGammaControlManagerV1,
            "zwlr_gamma_control_v1" => ProxyInterface::ZwlrGammaControlV1,
            "zwlr_input_inhibit_manager_v1" => ProxyInterface::ZwlrInputInhibitManagerV1,
            "zwlr_input_inhibitor_v1" => ProxyInterface::ZwlrInputInhibitorV1,
            "zwlr_layer_shell_v1" => ProxyInterface::ZwlrLayerShellV1,
            "zwlr_layer_surface_v1" => ProxyInterface::ZwlrLayerSurfaceV1,
            "zwlr_output_configuration_head_v1" => ProxyInterface::ZwlrOutputConfigurationHeadV1,
            "zwlr_output_configuration_v1" => ProxyInterface::ZwlrOutputConfigurationV1,
            "zwlr_output_head_v1" => ProxyInterface::ZwlrOutputHeadV1,
            "zwlr_output_manager_v1" => ProxyInterface::ZwlrOutputManagerV1,
            "zwlr_output_mode_v1" => ProxyInterface::ZwlrOutputModeV1,
            "zwlr_output_power_manager_v1" => ProxyInterface::ZwlrOutputPowerManagerV1,
            "zwlr_output_power_v1" => ProxyInterface::ZwlrOutputPowerV1,
            "zwlr_screencopy_frame_v1" => ProxyInterface::ZwlrScreencopyFrameV1,
            "zwlr_screencopy_manager_v1" => ProxyInterface::ZwlrScreencopyManagerV1,
            "zwlr_virtual_pointer_manager_v1" => ProxyInterface::ZwlrVirtualPointerManagerV1,
            "zwlr_virtual_pointer_v1" => ProxyInterface::ZwlrVirtualPointerV1,
            "xdg_activation_token_v1" => ProxyInterface::XdgActivationTokenV1,
            "xdg_activation_v1" => ProxyInterface::XdgActivationV1,
            "zxdg_decoration_manager_v1" => ProxyInterface::ZxdgDecorationManagerV1,
            "zxdg_toplevel_decoration_v1" => ProxyInterface::ZxdgToplevelDecorationV1,
            "xdg_dialog_v1" => ProxyInterface::XdgDialogV1,
            "xdg_wm_dialog_v1" => ProxyInterface::XdgWmDialogV1,
            "zxdg_exported_v2" => ProxyInterface::ZxdgExportedV2,
            "zxdg_exporter_v2" => ProxyInterface::ZxdgExporterV2,
            "zxdg_imported_v2" => ProxyInterface::ZxdgImportedV2,
            "zxdg_importer_v2" => ProxyInterface::ZxdgImporterV2,
            "zxdg_output_manager_v1" => ProxyInterface::ZxdgOutputManagerV1,
            "zxdg_output_v1" => ProxyInterface::ZxdgOutputV1,
            "xdg_popup" => ProxyInterface::XdgPopup,
            "xdg_positioner" => ProxyInterface::XdgPositioner,
            "xdg_surface" => ProxyInterface::XdgSurface,
            "xdg_toplevel" => ProxyInterface::XdgToplevel,
            "xdg_wm_base" => ProxyInterface::XdgWmBase,
            "xdg_system_bell_v1" => ProxyInterface::XdgSystemBellV1,
            "xdg_toplevel_drag_manager_v1" => ProxyInterface::XdgToplevelDragManagerV1,
            "xdg_toplevel_drag_v1" => ProxyInterface::XdgToplevelDragV1,
            "xdg_toplevel_icon_manager_v1" => ProxyInterface::XdgToplevelIconManagerV1,
            "xdg_toplevel_icon_v1" => ProxyInterface::XdgToplevelIconV1,
            "xdg_toplevel_tag_manager_v1" => ProxyInterface::XdgToplevelTagManagerV1,
            "zwp_xwayland_keyboard_grab_manager_v1" => ProxyInterface::ZwpXwaylandKeyboardGrabManagerV1,
            "zwp_xwayland_keyboard_grab_v1" => ProxyInterface::ZwpXwaylandKeyboardGrabV1,
            "xwayland_shell_v1" => ProxyInterface::XwaylandShellV1,
            "xwayland_surface_v1" => ProxyInterface::XwaylandSurfaceV1,
        };
        INTERFACES.get(interface).copied()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, linearize::Linearize)]
pub enum ProxyInterface {
    WpAlphaModifierSurfaceV1,
    WpAlphaModifierV1,
    WpColorManagementOutputV1,
    WpColorManagementSurfaceFeedbackV1,
    WpColorManagementSurfaceV1,
    WpColorManagerV1,
    WpImageDescriptionCreatorIccV1,
    WpImageDescriptionCreatorParamsV1,
    WpImageDescriptionInfoV1,
    WpImageDescriptionV1,
    WpColorRepresentationManagerV1,
    WpColorRepresentationSurfaceV1,
    WpCommitTimerV1,
    WpCommitTimingManagerV1,
    WpContentTypeManagerV1,
    WpContentTypeV1,
    WpCursorShapeDeviceV1,
    WpCursorShapeManagerV1,
    WpDrmLeaseConnectorV1,
    WpDrmLeaseDeviceV1,
    WpDrmLeaseRequestV1,
    WpDrmLeaseV1,
    ExtBackgroundEffectManagerV1,
    ExtBackgroundEffectSurfaceV1,
    ExtDataControlDeviceV1,
    ExtDataControlManagerV1,
    ExtDataControlOfferV1,
    ExtDataControlSourceV1,
    ExtForeignToplevelHandleV1,
    ExtForeignToplevelListV1,
    ExtIdleNotificationV1,
    ExtIdleNotifierV1,
    ExtForeignToplevelImageCaptureSourceManagerV1,
    ExtImageCaptureSourceV1,
    ExtOutputImageCaptureSourceManagerV1,
    ExtImageCopyCaptureCursorSessionV1,
    ExtImageCopyCaptureFrameV1,
    ExtImageCopyCaptureManagerV1,
    ExtImageCopyCaptureSessionV1,
    ExtSessionLockManagerV1,
    ExtSessionLockSurfaceV1,
    ExtSessionLockV1,
    ExtTransientSeatManagerV1,
    ExtTransientSeatV1,
    ExtWorkspaceGroupHandleV1,
    ExtWorkspaceHandleV1,
    ExtWorkspaceManagerV1,
    WpFifoManagerV1,
    WpFifoV1,
    WpFractionalScaleManagerV1,
    WpFractionalScaleV1,
    ZwpFullscreenShellModeFeedbackV1,
    ZwpFullscreenShellV1,
    ZwpIdleInhibitManagerV1,
    ZwpIdleInhibitorV1,
    ZwpInputMethodContextV1,
    ZwpInputMethodV1,
    ZwpInputPanelSurfaceV1,
    ZwpInputPanelV1,
    ZwpInputTimestampsManagerV1,
    ZwpInputTimestampsV1,
    ZwpKeyboardShortcutsInhibitManagerV1,
    ZwpKeyboardShortcutsInhibitorV1,
    ZwpLinuxBufferParamsV1,
    ZwpLinuxDmabufFeedbackV1,
    ZwpLinuxDmabufV1,
    WpLinuxDrmSyncobjManagerV1,
    WpLinuxDrmSyncobjSurfaceV1,
    WpLinuxDrmSyncobjTimelineV1,
    ZwpLinuxBufferReleaseV1,
    ZwpLinuxExplicitSynchronizationV1,
    ZwpLinuxSurfaceSynchronizationV1,
    ZwpConfinedPointerV1,
    ZwpLockedPointerV1,
    ZwpPointerConstraintsV1,
    ZwpPointerGestureHoldV1,
    ZwpPointerGesturePinchV1,
    ZwpPointerGestureSwipeV1,
    ZwpPointerGesturesV1,
    WpPointerWarpV1,
    WpPresentation,
    WpPresentationFeedback,
    ZwpPrimarySelectionDeviceManagerV1,
    ZwpPrimarySelectionDeviceV1,
    ZwpPrimarySelectionOfferV1,
    ZwpPrimarySelectionSourceV1,
    ZwpRelativePointerManagerV1,
    ZwpRelativePointerV1,
    WpSecurityContextManagerV1,
    WpSecurityContextV1,
    WpSinglePixelBufferManagerV1,
    ZwpTabletManagerV2,
    ZwpTabletPadDialV2,
    ZwpTabletPadGroupV2,
    ZwpTabletPadRingV2,
    ZwpTabletPadStripV2,
    ZwpTabletPadV2,
    ZwpTabletSeatV2,
    ZwpTabletToolV2,
    ZwpTabletV2,
    WpTearingControlManagerV1,
    WpTearingControlV1,
    ZwpTextInputManagerV1,
    ZwpTextInputV1,
    ZwpTextInputManagerV3,
    ZwpTextInputV3,
    WpViewport,
    WpViewporter,
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
    ZwlrDataControlDeviceV1,
    ZwlrDataControlManagerV1,
    ZwlrDataControlOfferV1,
    ZwlrDataControlSourceV1,
    ZwlrExportDmabufFrameV1,
    ZwlrExportDmabufManagerV1,
    ZwlrForeignToplevelHandleV1,
    ZwlrForeignToplevelManagerV1,
    ZwlrGammaControlManagerV1,
    ZwlrGammaControlV1,
    ZwlrInputInhibitManagerV1,
    ZwlrInputInhibitorV1,
    ZwlrLayerShellV1,
    ZwlrLayerSurfaceV1,
    ZwlrOutputConfigurationHeadV1,
    ZwlrOutputConfigurationV1,
    ZwlrOutputHeadV1,
    ZwlrOutputManagerV1,
    ZwlrOutputModeV1,
    ZwlrOutputPowerManagerV1,
    ZwlrOutputPowerV1,
    ZwlrScreencopyFrameV1,
    ZwlrScreencopyManagerV1,
    ZwlrVirtualPointerManagerV1,
    ZwlrVirtualPointerV1,
    XdgActivationTokenV1,
    XdgActivationV1,
    ZxdgDecorationManagerV1,
    ZxdgToplevelDecorationV1,
    XdgDialogV1,
    XdgWmDialogV1,
    ZxdgExportedV2,
    ZxdgExporterV2,
    ZxdgImportedV2,
    ZxdgImporterV2,
    ZxdgOutputManagerV1,
    ZxdgOutputV1,
    XdgPopup,
    XdgPositioner,
    XdgSurface,
    XdgToplevel,
    XdgWmBase,
    XdgSystemBellV1,
    XdgToplevelDragManagerV1,
    XdgToplevelDragV1,
    XdgToplevelIconManagerV1,
    XdgToplevelIconV1,
    XdgToplevelTagManagerV1,
    ZwpXwaylandKeyboardGrabManagerV1,
    ZwpXwaylandKeyboardGrabV1,
    XwaylandShellV1,
    XwaylandSurfaceV1,
}

impl ProxyInterface {
    pub fn name(self) -> &'static str {
        match self {
            Self::WpAlphaModifierSurfaceV1 => "wp_alpha_modifier_surface_v1",
            Self::WpAlphaModifierV1 => "wp_alpha_modifier_v1",
            Self::WpColorManagementOutputV1 => "wp_color_management_output_v1",
            Self::WpColorManagementSurfaceFeedbackV1 => "wp_color_management_surface_feedback_v1",
            Self::WpColorManagementSurfaceV1 => "wp_color_management_surface_v1",
            Self::WpColorManagerV1 => "wp_color_manager_v1",
            Self::WpImageDescriptionCreatorIccV1 => "wp_image_description_creator_icc_v1",
            Self::WpImageDescriptionCreatorParamsV1 => "wp_image_description_creator_params_v1",
            Self::WpImageDescriptionInfoV1 => "wp_image_description_info_v1",
            Self::WpImageDescriptionV1 => "wp_image_description_v1",
            Self::WpColorRepresentationManagerV1 => "wp_color_representation_manager_v1",
            Self::WpColorRepresentationSurfaceV1 => "wp_color_representation_surface_v1",
            Self::WpCommitTimerV1 => "wp_commit_timer_v1",
            Self::WpCommitTimingManagerV1 => "wp_commit_timing_manager_v1",
            Self::WpContentTypeManagerV1 => "wp_content_type_manager_v1",
            Self::WpContentTypeV1 => "wp_content_type_v1",
            Self::WpCursorShapeDeviceV1 => "wp_cursor_shape_device_v1",
            Self::WpCursorShapeManagerV1 => "wp_cursor_shape_manager_v1",
            Self::WpDrmLeaseConnectorV1 => "wp_drm_lease_connector_v1",
            Self::WpDrmLeaseDeviceV1 => "wp_drm_lease_device_v1",
            Self::WpDrmLeaseRequestV1 => "wp_drm_lease_request_v1",
            Self::WpDrmLeaseV1 => "wp_drm_lease_v1",
            Self::ExtBackgroundEffectManagerV1 => "ext_background_effect_manager_v1",
            Self::ExtBackgroundEffectSurfaceV1 => "ext_background_effect_surface_v1",
            Self::ExtDataControlDeviceV1 => "ext_data_control_device_v1",
            Self::ExtDataControlManagerV1 => "ext_data_control_manager_v1",
            Self::ExtDataControlOfferV1 => "ext_data_control_offer_v1",
            Self::ExtDataControlSourceV1 => "ext_data_control_source_v1",
            Self::ExtForeignToplevelHandleV1 => "ext_foreign_toplevel_handle_v1",
            Self::ExtForeignToplevelListV1 => "ext_foreign_toplevel_list_v1",
            Self::ExtIdleNotificationV1 => "ext_idle_notification_v1",
            Self::ExtIdleNotifierV1 => "ext_idle_notifier_v1",
            Self::ExtForeignToplevelImageCaptureSourceManagerV1 => "ext_foreign_toplevel_image_capture_source_manager_v1",
            Self::ExtImageCaptureSourceV1 => "ext_image_capture_source_v1",
            Self::ExtOutputImageCaptureSourceManagerV1 => "ext_output_image_capture_source_manager_v1",
            Self::ExtImageCopyCaptureCursorSessionV1 => "ext_image_copy_capture_cursor_session_v1",
            Self::ExtImageCopyCaptureFrameV1 => "ext_image_copy_capture_frame_v1",
            Self::ExtImageCopyCaptureManagerV1 => "ext_image_copy_capture_manager_v1",
            Self::ExtImageCopyCaptureSessionV1 => "ext_image_copy_capture_session_v1",
            Self::ExtSessionLockManagerV1 => "ext_session_lock_manager_v1",
            Self::ExtSessionLockSurfaceV1 => "ext_session_lock_surface_v1",
            Self::ExtSessionLockV1 => "ext_session_lock_v1",
            Self::ExtTransientSeatManagerV1 => "ext_transient_seat_manager_v1",
            Self::ExtTransientSeatV1 => "ext_transient_seat_v1",
            Self::ExtWorkspaceGroupHandleV1 => "ext_workspace_group_handle_v1",
            Self::ExtWorkspaceHandleV1 => "ext_workspace_handle_v1",
            Self::ExtWorkspaceManagerV1 => "ext_workspace_manager_v1",
            Self::WpFifoManagerV1 => "wp_fifo_manager_v1",
            Self::WpFifoV1 => "wp_fifo_v1",
            Self::WpFractionalScaleManagerV1 => "wp_fractional_scale_manager_v1",
            Self::WpFractionalScaleV1 => "wp_fractional_scale_v1",
            Self::ZwpFullscreenShellModeFeedbackV1 => "zwp_fullscreen_shell_mode_feedback_v1",
            Self::ZwpFullscreenShellV1 => "zwp_fullscreen_shell_v1",
            Self::ZwpIdleInhibitManagerV1 => "zwp_idle_inhibit_manager_v1",
            Self::ZwpIdleInhibitorV1 => "zwp_idle_inhibitor_v1",
            Self::ZwpInputMethodContextV1 => "zwp_input_method_context_v1",
            Self::ZwpInputMethodV1 => "zwp_input_method_v1",
            Self::ZwpInputPanelSurfaceV1 => "zwp_input_panel_surface_v1",
            Self::ZwpInputPanelV1 => "zwp_input_panel_v1",
            Self::ZwpInputTimestampsManagerV1 => "zwp_input_timestamps_manager_v1",
            Self::ZwpInputTimestampsV1 => "zwp_input_timestamps_v1",
            Self::ZwpKeyboardShortcutsInhibitManagerV1 => "zwp_keyboard_shortcuts_inhibit_manager_v1",
            Self::ZwpKeyboardShortcutsInhibitorV1 => "zwp_keyboard_shortcuts_inhibitor_v1",
            Self::ZwpLinuxBufferParamsV1 => "zwp_linux_buffer_params_v1",
            Self::ZwpLinuxDmabufFeedbackV1 => "zwp_linux_dmabuf_feedback_v1",
            Self::ZwpLinuxDmabufV1 => "zwp_linux_dmabuf_v1",
            Self::WpLinuxDrmSyncobjManagerV1 => "wp_linux_drm_syncobj_manager_v1",
            Self::WpLinuxDrmSyncobjSurfaceV1 => "wp_linux_drm_syncobj_surface_v1",
            Self::WpLinuxDrmSyncobjTimelineV1 => "wp_linux_drm_syncobj_timeline_v1",
            Self::ZwpLinuxBufferReleaseV1 => "zwp_linux_buffer_release_v1",
            Self::ZwpLinuxExplicitSynchronizationV1 => "zwp_linux_explicit_synchronization_v1",
            Self::ZwpLinuxSurfaceSynchronizationV1 => "zwp_linux_surface_synchronization_v1",
            Self::ZwpConfinedPointerV1 => "zwp_confined_pointer_v1",
            Self::ZwpLockedPointerV1 => "zwp_locked_pointer_v1",
            Self::ZwpPointerConstraintsV1 => "zwp_pointer_constraints_v1",
            Self::ZwpPointerGestureHoldV1 => "zwp_pointer_gesture_hold_v1",
            Self::ZwpPointerGesturePinchV1 => "zwp_pointer_gesture_pinch_v1",
            Self::ZwpPointerGestureSwipeV1 => "zwp_pointer_gesture_swipe_v1",
            Self::ZwpPointerGesturesV1 => "zwp_pointer_gestures_v1",
            Self::WpPointerWarpV1 => "wp_pointer_warp_v1",
            Self::WpPresentation => "wp_presentation",
            Self::WpPresentationFeedback => "wp_presentation_feedback",
            Self::ZwpPrimarySelectionDeviceManagerV1 => "zwp_primary_selection_device_manager_v1",
            Self::ZwpPrimarySelectionDeviceV1 => "zwp_primary_selection_device_v1",
            Self::ZwpPrimarySelectionOfferV1 => "zwp_primary_selection_offer_v1",
            Self::ZwpPrimarySelectionSourceV1 => "zwp_primary_selection_source_v1",
            Self::ZwpRelativePointerManagerV1 => "zwp_relative_pointer_manager_v1",
            Self::ZwpRelativePointerV1 => "zwp_relative_pointer_v1",
            Self::WpSecurityContextManagerV1 => "wp_security_context_manager_v1",
            Self::WpSecurityContextV1 => "wp_security_context_v1",
            Self::WpSinglePixelBufferManagerV1 => "wp_single_pixel_buffer_manager_v1",
            Self::ZwpTabletManagerV2 => "zwp_tablet_manager_v2",
            Self::ZwpTabletPadDialV2 => "zwp_tablet_pad_dial_v2",
            Self::ZwpTabletPadGroupV2 => "zwp_tablet_pad_group_v2",
            Self::ZwpTabletPadRingV2 => "zwp_tablet_pad_ring_v2",
            Self::ZwpTabletPadStripV2 => "zwp_tablet_pad_strip_v2",
            Self::ZwpTabletPadV2 => "zwp_tablet_pad_v2",
            Self::ZwpTabletSeatV2 => "zwp_tablet_seat_v2",
            Self::ZwpTabletToolV2 => "zwp_tablet_tool_v2",
            Self::ZwpTabletV2 => "zwp_tablet_v2",
            Self::WpTearingControlManagerV1 => "wp_tearing_control_manager_v1",
            Self::WpTearingControlV1 => "wp_tearing_control_v1",
            Self::ZwpTextInputManagerV1 => "zwp_text_input_manager_v1",
            Self::ZwpTextInputV1 => "zwp_text_input_v1",
            Self::ZwpTextInputManagerV3 => "zwp_text_input_manager_v3",
            Self::ZwpTextInputV3 => "zwp_text_input_v3",
            Self::WpViewport => "wp_viewport",
            Self::WpViewporter => "wp_viewporter",
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
            Self::ZwlrDataControlDeviceV1 => "zwlr_data_control_device_v1",
            Self::ZwlrDataControlManagerV1 => "zwlr_data_control_manager_v1",
            Self::ZwlrDataControlOfferV1 => "zwlr_data_control_offer_v1",
            Self::ZwlrDataControlSourceV1 => "zwlr_data_control_source_v1",
            Self::ZwlrExportDmabufFrameV1 => "zwlr_export_dmabuf_frame_v1",
            Self::ZwlrExportDmabufManagerV1 => "zwlr_export_dmabuf_manager_v1",
            Self::ZwlrForeignToplevelHandleV1 => "zwlr_foreign_toplevel_handle_v1",
            Self::ZwlrForeignToplevelManagerV1 => "zwlr_foreign_toplevel_manager_v1",
            Self::ZwlrGammaControlManagerV1 => "zwlr_gamma_control_manager_v1",
            Self::ZwlrGammaControlV1 => "zwlr_gamma_control_v1",
            Self::ZwlrInputInhibitManagerV1 => "zwlr_input_inhibit_manager_v1",
            Self::ZwlrInputInhibitorV1 => "zwlr_input_inhibitor_v1",
            Self::ZwlrLayerShellV1 => "zwlr_layer_shell_v1",
            Self::ZwlrLayerSurfaceV1 => "zwlr_layer_surface_v1",
            Self::ZwlrOutputConfigurationHeadV1 => "zwlr_output_configuration_head_v1",
            Self::ZwlrOutputConfigurationV1 => "zwlr_output_configuration_v1",
            Self::ZwlrOutputHeadV1 => "zwlr_output_head_v1",
            Self::ZwlrOutputManagerV1 => "zwlr_output_manager_v1",
            Self::ZwlrOutputModeV1 => "zwlr_output_mode_v1",
            Self::ZwlrOutputPowerManagerV1 => "zwlr_output_power_manager_v1",
            Self::ZwlrOutputPowerV1 => "zwlr_output_power_v1",
            Self::ZwlrScreencopyFrameV1 => "zwlr_screencopy_frame_v1",
            Self::ZwlrScreencopyManagerV1 => "zwlr_screencopy_manager_v1",
            Self::ZwlrVirtualPointerManagerV1 => "zwlr_virtual_pointer_manager_v1",
            Self::ZwlrVirtualPointerV1 => "zwlr_virtual_pointer_v1",
            Self::XdgActivationTokenV1 => "xdg_activation_token_v1",
            Self::XdgActivationV1 => "xdg_activation_v1",
            Self::ZxdgDecorationManagerV1 => "zxdg_decoration_manager_v1",
            Self::ZxdgToplevelDecorationV1 => "zxdg_toplevel_decoration_v1",
            Self::XdgDialogV1 => "xdg_dialog_v1",
            Self::XdgWmDialogV1 => "xdg_wm_dialog_v1",
            Self::ZxdgExportedV2 => "zxdg_exported_v2",
            Self::ZxdgExporterV2 => "zxdg_exporter_v2",
            Self::ZxdgImportedV2 => "zxdg_imported_v2",
            Self::ZxdgImporterV2 => "zxdg_importer_v2",
            Self::ZxdgOutputManagerV1 => "zxdg_output_manager_v1",
            Self::ZxdgOutputV1 => "zxdg_output_v1",
            Self::XdgPopup => "xdg_popup",
            Self::XdgPositioner => "xdg_positioner",
            Self::XdgSurface => "xdg_surface",
            Self::XdgToplevel => "xdg_toplevel",
            Self::XdgWmBase => "xdg_wm_base",
            Self::XdgSystemBellV1 => "xdg_system_bell_v1",
            Self::XdgToplevelDragManagerV1 => "xdg_toplevel_drag_manager_v1",
            Self::XdgToplevelDragV1 => "xdg_toplevel_drag_v1",
            Self::XdgToplevelIconManagerV1 => "xdg_toplevel_icon_manager_v1",
            Self::XdgToplevelIconV1 => "xdg_toplevel_icon_v1",
            Self::XdgToplevelTagManagerV1 => "xdg_toplevel_tag_manager_v1",
            Self::ZwpXwaylandKeyboardGrabManagerV1 => "zwp_xwayland_keyboard_grab_manager_v1",
            Self::ZwpXwaylandKeyboardGrabV1 => "zwp_xwayland_keyboard_grab_v1",
            Self::XwaylandShellV1 => "xwayland_shell_v1",
            Self::XwaylandSurfaceV1 => "xwayland_surface_v1",
        }
    }

    pub fn xml_version(self) -> u32 {
        match self {
            Self::WpAlphaModifierSurfaceV1 => 1,
            Self::WpAlphaModifierV1 => 1,
            Self::WpColorManagementOutputV1 => 1,
            Self::WpColorManagementSurfaceFeedbackV1 => 1,
            Self::WpColorManagementSurfaceV1 => 1,
            Self::WpColorManagerV1 => 1,
            Self::WpImageDescriptionCreatorIccV1 => 1,
            Self::WpImageDescriptionCreatorParamsV1 => 1,
            Self::WpImageDescriptionInfoV1 => 1,
            Self::WpImageDescriptionV1 => 1,
            Self::WpColorRepresentationManagerV1 => 1,
            Self::WpColorRepresentationSurfaceV1 => 1,
            Self::WpCommitTimerV1 => 1,
            Self::WpCommitTimingManagerV1 => 1,
            Self::WpContentTypeManagerV1 => 1,
            Self::WpContentTypeV1 => 1,
            Self::WpCursorShapeDeviceV1 => 2,
            Self::WpCursorShapeManagerV1 => 2,
            Self::WpDrmLeaseConnectorV1 => 1,
            Self::WpDrmLeaseDeviceV1 => 1,
            Self::WpDrmLeaseRequestV1 => 1,
            Self::WpDrmLeaseV1 => 1,
            Self::ExtBackgroundEffectManagerV1 => 1,
            Self::ExtBackgroundEffectSurfaceV1 => 1,
            Self::ExtDataControlDeviceV1 => 1,
            Self::ExtDataControlManagerV1 => 1,
            Self::ExtDataControlOfferV1 => 1,
            Self::ExtDataControlSourceV1 => 1,
            Self::ExtForeignToplevelHandleV1 => 1,
            Self::ExtForeignToplevelListV1 => 1,
            Self::ExtIdleNotificationV1 => 2,
            Self::ExtIdleNotifierV1 => 2,
            Self::ExtForeignToplevelImageCaptureSourceManagerV1 => 1,
            Self::ExtImageCaptureSourceV1 => 1,
            Self::ExtOutputImageCaptureSourceManagerV1 => 1,
            Self::ExtImageCopyCaptureCursorSessionV1 => 1,
            Self::ExtImageCopyCaptureFrameV1 => 1,
            Self::ExtImageCopyCaptureManagerV1 => 1,
            Self::ExtImageCopyCaptureSessionV1 => 1,
            Self::ExtSessionLockManagerV1 => 1,
            Self::ExtSessionLockSurfaceV1 => 1,
            Self::ExtSessionLockV1 => 1,
            Self::ExtTransientSeatManagerV1 => 1,
            Self::ExtTransientSeatV1 => 1,
            Self::ExtWorkspaceGroupHandleV1 => 1,
            Self::ExtWorkspaceHandleV1 => 1,
            Self::ExtWorkspaceManagerV1 => 1,
            Self::WpFifoManagerV1 => 1,
            Self::WpFifoV1 => 1,
            Self::WpFractionalScaleManagerV1 => 1,
            Self::WpFractionalScaleV1 => 1,
            Self::ZwpFullscreenShellModeFeedbackV1 => 1,
            Self::ZwpFullscreenShellV1 => 1,
            Self::ZwpIdleInhibitManagerV1 => 1,
            Self::ZwpIdleInhibitorV1 => 1,
            Self::ZwpInputMethodContextV1 => 1,
            Self::ZwpInputMethodV1 => 1,
            Self::ZwpInputPanelSurfaceV1 => 1,
            Self::ZwpInputPanelV1 => 1,
            Self::ZwpInputTimestampsManagerV1 => 1,
            Self::ZwpInputTimestampsV1 => 1,
            Self::ZwpKeyboardShortcutsInhibitManagerV1 => 1,
            Self::ZwpKeyboardShortcutsInhibitorV1 => 1,
            Self::ZwpLinuxBufferParamsV1 => 5,
            Self::ZwpLinuxDmabufFeedbackV1 => 5,
            Self::ZwpLinuxDmabufV1 => 5,
            Self::WpLinuxDrmSyncobjManagerV1 => 1,
            Self::WpLinuxDrmSyncobjSurfaceV1 => 1,
            Self::WpLinuxDrmSyncobjTimelineV1 => 1,
            Self::ZwpLinuxBufferReleaseV1 => 1,
            Self::ZwpLinuxExplicitSynchronizationV1 => 2,
            Self::ZwpLinuxSurfaceSynchronizationV1 => 2,
            Self::ZwpConfinedPointerV1 => 1,
            Self::ZwpLockedPointerV1 => 1,
            Self::ZwpPointerConstraintsV1 => 1,
            Self::ZwpPointerGestureHoldV1 => 3,
            Self::ZwpPointerGesturePinchV1 => 2,
            Self::ZwpPointerGestureSwipeV1 => 2,
            Self::ZwpPointerGesturesV1 => 3,
            Self::WpPointerWarpV1 => 1,
            Self::WpPresentation => 2,
            Self::WpPresentationFeedback => 2,
            Self::ZwpPrimarySelectionDeviceManagerV1 => 1,
            Self::ZwpPrimarySelectionDeviceV1 => 1,
            Self::ZwpPrimarySelectionOfferV1 => 1,
            Self::ZwpPrimarySelectionSourceV1 => 1,
            Self::ZwpRelativePointerManagerV1 => 1,
            Self::ZwpRelativePointerV1 => 1,
            Self::WpSecurityContextManagerV1 => 1,
            Self::WpSecurityContextV1 => 1,
            Self::WpSinglePixelBufferManagerV1 => 1,
            Self::ZwpTabletManagerV2 => 2,
            Self::ZwpTabletPadDialV2 => 2,
            Self::ZwpTabletPadGroupV2 => 2,
            Self::ZwpTabletPadRingV2 => 2,
            Self::ZwpTabletPadStripV2 => 2,
            Self::ZwpTabletPadV2 => 2,
            Self::ZwpTabletSeatV2 => 2,
            Self::ZwpTabletToolV2 => 2,
            Self::ZwpTabletV2 => 2,
            Self::WpTearingControlManagerV1 => 1,
            Self::WpTearingControlV1 => 1,
            Self::ZwpTextInputManagerV1 => 1,
            Self::ZwpTextInputV1 => 1,
            Self::ZwpTextInputManagerV3 => 1,
            Self::ZwpTextInputV3 => 1,
            Self::WpViewport => 1,
            Self::WpViewporter => 1,
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
            Self::ZwlrDataControlDeviceV1 => 2,
            Self::ZwlrDataControlManagerV1 => 2,
            Self::ZwlrDataControlOfferV1 => 1,
            Self::ZwlrDataControlSourceV1 => 1,
            Self::ZwlrExportDmabufFrameV1 => 1,
            Self::ZwlrExportDmabufManagerV1 => 1,
            Self::ZwlrForeignToplevelHandleV1 => 3,
            Self::ZwlrForeignToplevelManagerV1 => 3,
            Self::ZwlrGammaControlManagerV1 => 1,
            Self::ZwlrGammaControlV1 => 1,
            Self::ZwlrInputInhibitManagerV1 => 1,
            Self::ZwlrInputInhibitorV1 => 1,
            Self::ZwlrLayerShellV1 => 5,
            Self::ZwlrLayerSurfaceV1 => 5,
            Self::ZwlrOutputConfigurationHeadV1 => 4,
            Self::ZwlrOutputConfigurationV1 => 4,
            Self::ZwlrOutputHeadV1 => 4,
            Self::ZwlrOutputManagerV1 => 4,
            Self::ZwlrOutputModeV1 => 3,
            Self::ZwlrOutputPowerManagerV1 => 1,
            Self::ZwlrOutputPowerV1 => 1,
            Self::ZwlrScreencopyFrameV1 => 3,
            Self::ZwlrScreencopyManagerV1 => 3,
            Self::ZwlrVirtualPointerManagerV1 => 2,
            Self::ZwlrVirtualPointerV1 => 2,
            Self::XdgActivationTokenV1 => 1,
            Self::XdgActivationV1 => 1,
            Self::ZxdgDecorationManagerV1 => 1,
            Self::ZxdgToplevelDecorationV1 => 1,
            Self::XdgDialogV1 => 1,
            Self::XdgWmDialogV1 => 1,
            Self::ZxdgExportedV2 => 1,
            Self::ZxdgExporterV2 => 1,
            Self::ZxdgImportedV2 => 1,
            Self::ZxdgImporterV2 => 1,
            Self::ZxdgOutputManagerV1 => 3,
            Self::ZxdgOutputV1 => 3,
            Self::XdgPopup => 7,
            Self::XdgPositioner => 7,
            Self::XdgSurface => 7,
            Self::XdgToplevel => 7,
            Self::XdgWmBase => 7,
            Self::XdgSystemBellV1 => 1,
            Self::XdgToplevelDragManagerV1 => 1,
            Self::XdgToplevelDragV1 => 1,
            Self::XdgToplevelIconManagerV1 => 1,
            Self::XdgToplevelIconV1 => 1,
            Self::XdgToplevelTagManagerV1 => 1,
            Self::ZwpXwaylandKeyboardGrabManagerV1 => 1,
            Self::ZwpXwaylandKeyboardGrabV1 => 1,
            Self::XwaylandShellV1 => 1,
            Self::XwaylandSurfaceV1 => 1,
        }
    }
}
