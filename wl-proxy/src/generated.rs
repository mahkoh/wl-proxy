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
    pub(super) use super::alpha_modifier_v1::wp_alpha_modifier_surface_v1::MetaWpAlphaModifierSurfaceV1;
    pub(super) use super::alpha_modifier_v1::wp_alpha_modifier_surface_v1::MetaWpAlphaModifierSurfaceV1Error;
    pub(super) use super::alpha_modifier_v1::wp_alpha_modifier_v1::MetaWpAlphaModifierV1;
    pub(super) use super::alpha_modifier_v1::wp_alpha_modifier_v1::MetaWpAlphaModifierV1Error;
    pub(super) use super::color_management_v1::wp_color_management_output_v1::MetaWpColorManagementOutputV1;
    pub(super) use super::color_management_v1::wp_color_management_surface_feedback_v1::MetaWpColorManagementSurfaceFeedbackV1;
    pub(super) use super::color_management_v1::wp_color_management_surface_feedback_v1::MetaWpColorManagementSurfaceFeedbackV1Error;
    pub(super) use super::color_management_v1::wp_color_management_surface_v1::MetaWpColorManagementSurfaceV1;
    pub(super) use super::color_management_v1::wp_color_management_surface_v1::MetaWpColorManagementSurfaceV1Error;
    pub(super) use super::color_management_v1::wp_color_manager_v1::MetaWpColorManagerV1;
    pub(super) use super::color_management_v1::wp_color_manager_v1::MetaWpColorManagerV1Error;
    pub(super) use super::color_management_v1::wp_color_manager_v1::MetaWpColorManagerV1RenderIntent;
    pub(super) use super::color_management_v1::wp_color_manager_v1::MetaWpColorManagerV1Feature;
    pub(super) use super::color_management_v1::wp_color_manager_v1::MetaWpColorManagerV1Primaries;
    pub(super) use super::color_management_v1::wp_color_manager_v1::MetaWpColorManagerV1TransferFunction;
    pub(super) use super::color_management_v1::wp_image_description_creator_icc_v1::MetaWpImageDescriptionCreatorIccV1;
    pub(super) use super::color_management_v1::wp_image_description_creator_icc_v1::MetaWpImageDescriptionCreatorIccV1Error;
    pub(super) use super::color_management_v1::wp_image_description_creator_params_v1::MetaWpImageDescriptionCreatorParamsV1;
    pub(super) use super::color_management_v1::wp_image_description_creator_params_v1::MetaWpImageDescriptionCreatorParamsV1Error;
    pub(super) use super::color_management_v1::wp_image_description_info_v1::MetaWpImageDescriptionInfoV1;
    pub(super) use super::color_management_v1::wp_image_description_v1::MetaWpImageDescriptionV1;
    pub(super) use super::color_management_v1::wp_image_description_v1::MetaWpImageDescriptionV1Error;
    pub(super) use super::color_management_v1::wp_image_description_v1::MetaWpImageDescriptionV1Cause;
    pub(super) use super::color_representation_v1::wp_color_representation_manager_v1::MetaWpColorRepresentationManagerV1;
    pub(super) use super::color_representation_v1::wp_color_representation_manager_v1::MetaWpColorRepresentationManagerV1Error;
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::MetaWpColorRepresentationSurfaceV1;
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::MetaWpColorRepresentationSurfaceV1Error;
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::MetaWpColorRepresentationSurfaceV1AlphaMode;
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::MetaWpColorRepresentationSurfaceV1Coefficients;
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::MetaWpColorRepresentationSurfaceV1Range;
    pub(super) use super::color_representation_v1::wp_color_representation_surface_v1::MetaWpColorRepresentationSurfaceV1ChromaLocation;
    pub(super) use super::commit_timing_v1::wp_commit_timer_v1::MetaWpCommitTimerV1;
    pub(super) use super::commit_timing_v1::wp_commit_timer_v1::MetaWpCommitTimerV1Error;
    pub(super) use super::commit_timing_v1::wp_commit_timing_manager_v1::MetaWpCommitTimingManagerV1;
    pub(super) use super::commit_timing_v1::wp_commit_timing_manager_v1::MetaWpCommitTimingManagerV1Error;
    pub(super) use super::content_type_v1::wp_content_type_manager_v1::MetaWpContentTypeManagerV1;
    pub(super) use super::content_type_v1::wp_content_type_manager_v1::MetaWpContentTypeManagerV1Error;
    pub(super) use super::content_type_v1::wp_content_type_v1::MetaWpContentTypeV1;
    pub(super) use super::content_type_v1::wp_content_type_v1::MetaWpContentTypeV1Type;
    pub(super) use super::cursor_shape_v1::wp_cursor_shape_device_v1::MetaWpCursorShapeDeviceV1;
    pub(super) use super::cursor_shape_v1::wp_cursor_shape_device_v1::MetaWpCursorShapeDeviceV1Shape;
    pub(super) use super::cursor_shape_v1::wp_cursor_shape_device_v1::MetaWpCursorShapeDeviceV1Error;
    pub(super) use super::cursor_shape_v1::wp_cursor_shape_manager_v1::MetaWpCursorShapeManagerV1;
    pub(super) use super::drm_lease_v1::wp_drm_lease_connector_v1::MetaWpDrmLeaseConnectorV1;
    pub(super) use super::drm_lease_v1::wp_drm_lease_device_v1::MetaWpDrmLeaseDeviceV1;
    pub(super) use super::drm_lease_v1::wp_drm_lease_request_v1::MetaWpDrmLeaseRequestV1;
    pub(super) use super::drm_lease_v1::wp_drm_lease_request_v1::MetaWpDrmLeaseRequestV1Error;
    pub(super) use super::drm_lease_v1::wp_drm_lease_v1::MetaWpDrmLeaseV1;
    pub(super) use super::ext_background_effect_v1::ext_background_effect_manager_v1::MetaExtBackgroundEffectManagerV1;
    pub(super) use super::ext_background_effect_v1::ext_background_effect_manager_v1::MetaExtBackgroundEffectManagerV1Error;
    pub(super) use super::ext_background_effect_v1::ext_background_effect_manager_v1::MetaExtBackgroundEffectManagerV1Capability;
    pub(super) use super::ext_background_effect_v1::ext_background_effect_surface_v1::MetaExtBackgroundEffectSurfaceV1;
    pub(super) use super::ext_background_effect_v1::ext_background_effect_surface_v1::MetaExtBackgroundEffectSurfaceV1Error;
    pub(super) use super::ext_data_control_v1::ext_data_control_device_v1::MetaExtDataControlDeviceV1;
    pub(super) use super::ext_data_control_v1::ext_data_control_device_v1::MetaExtDataControlDeviceV1Error;
    pub(super) use super::ext_data_control_v1::ext_data_control_manager_v1::MetaExtDataControlManagerV1;
    pub(super) use super::ext_data_control_v1::ext_data_control_offer_v1::MetaExtDataControlOfferV1;
    pub(super) use super::ext_data_control_v1::ext_data_control_source_v1::MetaExtDataControlSourceV1;
    pub(super) use super::ext_data_control_v1::ext_data_control_source_v1::MetaExtDataControlSourceV1Error;
    pub(super) use super::ext_foreign_toplevel_list_v1::ext_foreign_toplevel_handle_v1::MetaExtForeignToplevelHandleV1;
    pub(super) use super::ext_foreign_toplevel_list_v1::ext_foreign_toplevel_list_v1::MetaExtForeignToplevelListV1;
    pub(super) use super::ext_idle_notify_v1::ext_idle_notification_v1::MetaExtIdleNotificationV1;
    pub(super) use super::ext_idle_notify_v1::ext_idle_notifier_v1::MetaExtIdleNotifierV1;
    pub(super) use super::ext_image_capture_source_v1::ext_foreign_toplevel_image_capture_source_manager_v1::MetaExtForeignToplevelImageCaptureSourceManagerV1;
    pub(super) use super::ext_image_capture_source_v1::ext_image_capture_source_v1::MetaExtImageCaptureSourceV1;
    pub(super) use super::ext_image_capture_source_v1::ext_output_image_capture_source_manager_v1::MetaExtOutputImageCaptureSourceManagerV1;
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_cursor_session_v1::MetaExtImageCopyCaptureCursorSessionV1;
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_cursor_session_v1::MetaExtImageCopyCaptureCursorSessionV1Error;
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_frame_v1::MetaExtImageCopyCaptureFrameV1;
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_frame_v1::MetaExtImageCopyCaptureFrameV1Error;
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_frame_v1::MetaExtImageCopyCaptureFrameV1FailureReason;
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_manager_v1::MetaExtImageCopyCaptureManagerV1;
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_manager_v1::MetaExtImageCopyCaptureManagerV1Error;
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_manager_v1::MetaExtImageCopyCaptureManagerV1Options;
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_session_v1::MetaExtImageCopyCaptureSessionV1;
    pub(super) use super::ext_image_copy_capture_v1::ext_image_copy_capture_session_v1::MetaExtImageCopyCaptureSessionV1Error;
    pub(super) use super::ext_session_lock_v1::ext_session_lock_manager_v1::MetaExtSessionLockManagerV1;
    pub(super) use super::ext_session_lock_v1::ext_session_lock_surface_v1::MetaExtSessionLockSurfaceV1;
    pub(super) use super::ext_session_lock_v1::ext_session_lock_surface_v1::MetaExtSessionLockSurfaceV1Error;
    pub(super) use super::ext_session_lock_v1::ext_session_lock_v1::MetaExtSessionLockV1;
    pub(super) use super::ext_session_lock_v1::ext_session_lock_v1::MetaExtSessionLockV1Error;
    pub(super) use super::ext_transient_seat_v1::ext_transient_seat_manager_v1::MetaExtTransientSeatManagerV1;
    pub(super) use super::ext_transient_seat_v1::ext_transient_seat_v1::MetaExtTransientSeatV1;
    pub(super) use super::ext_workspace_v1::ext_workspace_group_handle_v1::MetaExtWorkspaceGroupHandleV1;
    pub(super) use super::ext_workspace_v1::ext_workspace_group_handle_v1::MetaExtWorkspaceGroupHandleV1GroupCapabilities;
    pub(super) use super::ext_workspace_v1::ext_workspace_handle_v1::MetaExtWorkspaceHandleV1;
    pub(super) use super::ext_workspace_v1::ext_workspace_handle_v1::MetaExtWorkspaceHandleV1State;
    pub(super) use super::ext_workspace_v1::ext_workspace_handle_v1::MetaExtWorkspaceHandleV1WorkspaceCapabilities;
    pub(super) use super::ext_workspace_v1::ext_workspace_manager_v1::MetaExtWorkspaceManagerV1;
    pub(super) use super::fifo_v1::wp_fifo_manager_v1::MetaWpFifoManagerV1;
    pub(super) use super::fifo_v1::wp_fifo_manager_v1::MetaWpFifoManagerV1Error;
    pub(super) use super::fifo_v1::wp_fifo_v1::MetaWpFifoV1;
    pub(super) use super::fifo_v1::wp_fifo_v1::MetaWpFifoV1Error;
    pub(super) use super::fractional_scale_v1::wp_fractional_scale_manager_v1::MetaWpFractionalScaleManagerV1;
    pub(super) use super::fractional_scale_v1::wp_fractional_scale_manager_v1::MetaWpFractionalScaleManagerV1Error;
    pub(super) use super::fractional_scale_v1::wp_fractional_scale_v1::MetaWpFractionalScaleV1;
    pub(super) use super::fullscreen_shell_unstable_v1::zwp_fullscreen_shell_mode_feedback_v1::MetaZwpFullscreenShellModeFeedbackV1;
    pub(super) use super::fullscreen_shell_unstable_v1::zwp_fullscreen_shell_v1::MetaZwpFullscreenShellV1;
    pub(super) use super::fullscreen_shell_unstable_v1::zwp_fullscreen_shell_v1::MetaZwpFullscreenShellV1Capability;
    pub(super) use super::fullscreen_shell_unstable_v1::zwp_fullscreen_shell_v1::MetaZwpFullscreenShellV1PresentMethod;
    pub(super) use super::fullscreen_shell_unstable_v1::zwp_fullscreen_shell_v1::MetaZwpFullscreenShellV1Error;
    pub(super) use super::idle_inhibit_unstable_v1::zwp_idle_inhibit_manager_v1::MetaZwpIdleInhibitManagerV1;
    pub(super) use super::idle_inhibit_unstable_v1::zwp_idle_inhibitor_v1::MetaZwpIdleInhibitorV1;
    pub(super) use super::input_method_unstable_v1::zwp_input_method_context_v1::MetaZwpInputMethodContextV1;
    pub(super) use super::input_method_unstable_v1::zwp_input_method_v1::MetaZwpInputMethodV1;
    pub(super) use super::input_method_unstable_v1::zwp_input_panel_surface_v1::MetaZwpInputPanelSurfaceV1;
    pub(super) use super::input_method_unstable_v1::zwp_input_panel_surface_v1::MetaZwpInputPanelSurfaceV1Position;
    pub(super) use super::input_method_unstable_v1::zwp_input_panel_v1::MetaZwpInputPanelV1;
    pub(super) use super::input_timestamps_unstable_v1::zwp_input_timestamps_manager_v1::MetaZwpInputTimestampsManagerV1;
    pub(super) use super::input_timestamps_unstable_v1::zwp_input_timestamps_v1::MetaZwpInputTimestampsV1;
    pub(super) use super::keyboard_shortcuts_inhibit_unstable_v1::zwp_keyboard_shortcuts_inhibit_manager_v1::MetaZwpKeyboardShortcutsInhibitManagerV1;
    pub(super) use super::keyboard_shortcuts_inhibit_unstable_v1::zwp_keyboard_shortcuts_inhibit_manager_v1::MetaZwpKeyboardShortcutsInhibitManagerV1Error;
    pub(super) use super::keyboard_shortcuts_inhibit_unstable_v1::zwp_keyboard_shortcuts_inhibitor_v1::MetaZwpKeyboardShortcutsInhibitorV1;
    pub(super) use super::linux_dmabuf_v1::zwp_linux_buffer_params_v1::MetaZwpLinuxBufferParamsV1;
    pub(super) use super::linux_dmabuf_v1::zwp_linux_buffer_params_v1::MetaZwpLinuxBufferParamsV1Error;
    pub(super) use super::linux_dmabuf_v1::zwp_linux_buffer_params_v1::MetaZwpLinuxBufferParamsV1Flags;
    pub(super) use super::linux_dmabuf_v1::zwp_linux_dmabuf_feedback_v1::MetaZwpLinuxDmabufFeedbackV1;
    pub(super) use super::linux_dmabuf_v1::zwp_linux_dmabuf_feedback_v1::MetaZwpLinuxDmabufFeedbackV1TrancheFlags;
    pub(super) use super::linux_dmabuf_v1::zwp_linux_dmabuf_v1::MetaZwpLinuxDmabufV1;
    pub(super) use super::linux_drm_syncobj_v1::wp_linux_drm_syncobj_manager_v1::MetaWpLinuxDrmSyncobjManagerV1;
    pub(super) use super::linux_drm_syncobj_v1::wp_linux_drm_syncobj_manager_v1::MetaWpLinuxDrmSyncobjManagerV1Error;
    pub(super) use super::linux_drm_syncobj_v1::wp_linux_drm_syncobj_surface_v1::MetaWpLinuxDrmSyncobjSurfaceV1;
    pub(super) use super::linux_drm_syncobj_v1::wp_linux_drm_syncobj_surface_v1::MetaWpLinuxDrmSyncobjSurfaceV1Error;
    pub(super) use super::linux_drm_syncobj_v1::wp_linux_drm_syncobj_timeline_v1::MetaWpLinuxDrmSyncobjTimelineV1;
    pub(super) use super::zwp_linux_explicit_synchronization_unstable_v1::zwp_linux_buffer_release_v1::MetaZwpLinuxBufferReleaseV1;
    pub(super) use super::zwp_linux_explicit_synchronization_unstable_v1::zwp_linux_explicit_synchronization_v1::MetaZwpLinuxExplicitSynchronizationV1;
    pub(super) use super::zwp_linux_explicit_synchronization_unstable_v1::zwp_linux_explicit_synchronization_v1::MetaZwpLinuxExplicitSynchronizationV1Error;
    pub(super) use super::zwp_linux_explicit_synchronization_unstable_v1::zwp_linux_surface_synchronization_v1::MetaZwpLinuxSurfaceSynchronizationV1;
    pub(super) use super::zwp_linux_explicit_synchronization_unstable_v1::zwp_linux_surface_synchronization_v1::MetaZwpLinuxSurfaceSynchronizationV1Error;
    pub(super) use super::pointer_constraints_unstable_v1::zwp_confined_pointer_v1::MetaZwpConfinedPointerV1;
    pub(super) use super::pointer_constraints_unstable_v1::zwp_locked_pointer_v1::MetaZwpLockedPointerV1;
    pub(super) use super::pointer_constraints_unstable_v1::zwp_pointer_constraints_v1::MetaZwpPointerConstraintsV1;
    pub(super) use super::pointer_constraints_unstable_v1::zwp_pointer_constraints_v1::MetaZwpPointerConstraintsV1Error;
    pub(super) use super::pointer_constraints_unstable_v1::zwp_pointer_constraints_v1::MetaZwpPointerConstraintsV1Lifetime;
    pub(super) use super::pointer_gestures_unstable_v1::zwp_pointer_gesture_hold_v1::MetaZwpPointerGestureHoldV1;
    pub(super) use super::pointer_gestures_unstable_v1::zwp_pointer_gesture_pinch_v1::MetaZwpPointerGesturePinchV1;
    pub(super) use super::pointer_gestures_unstable_v1::zwp_pointer_gesture_swipe_v1::MetaZwpPointerGestureSwipeV1;
    pub(super) use super::pointer_gestures_unstable_v1::zwp_pointer_gestures_v1::MetaZwpPointerGesturesV1;
    pub(super) use super::pointer_warp_v1::wp_pointer_warp_v1::MetaWpPointerWarpV1;
    pub(super) use super::presentation_time::wp_presentation::MetaWpPresentation;
    pub(super) use super::presentation_time::wp_presentation::MetaWpPresentationError;
    pub(super) use super::presentation_time::wp_presentation_feedback::MetaWpPresentationFeedback;
    pub(super) use super::presentation_time::wp_presentation_feedback::MetaWpPresentationFeedbackKind;
    pub(super) use super::wp_primary_selection_unstable_v1::zwp_primary_selection_device_manager_v1::MetaZwpPrimarySelectionDeviceManagerV1;
    pub(super) use super::wp_primary_selection_unstable_v1::zwp_primary_selection_device_v1::MetaZwpPrimarySelectionDeviceV1;
    pub(super) use super::wp_primary_selection_unstable_v1::zwp_primary_selection_offer_v1::MetaZwpPrimarySelectionOfferV1;
    pub(super) use super::wp_primary_selection_unstable_v1::zwp_primary_selection_source_v1::MetaZwpPrimarySelectionSourceV1;
    pub(super) use super::relative_pointer_unstable_v1::zwp_relative_pointer_manager_v1::MetaZwpRelativePointerManagerV1;
    pub(super) use super::relative_pointer_unstable_v1::zwp_relative_pointer_v1::MetaZwpRelativePointerV1;
    pub(super) use super::security_context_v1::wp_security_context_manager_v1::MetaWpSecurityContextManagerV1;
    pub(super) use super::security_context_v1::wp_security_context_manager_v1::MetaWpSecurityContextManagerV1Error;
    pub(super) use super::security_context_v1::wp_security_context_v1::MetaWpSecurityContextV1;
    pub(super) use super::security_context_v1::wp_security_context_v1::MetaWpSecurityContextV1Error;
    pub(super) use super::single_pixel_buffer_v1::wp_single_pixel_buffer_manager_v1::MetaWpSinglePixelBufferManagerV1;
    pub(super) use super::tablet_v2::zwp_tablet_manager_v2::MetaZwpTabletManagerV2;
    pub(super) use super::tablet_v2::zwp_tablet_pad_dial_v2::MetaZwpTabletPadDialV2;
    pub(super) use super::tablet_v2::zwp_tablet_pad_group_v2::MetaZwpTabletPadGroupV2;
    pub(super) use super::tablet_v2::zwp_tablet_pad_ring_v2::MetaZwpTabletPadRingV2;
    pub(super) use super::tablet_v2::zwp_tablet_pad_ring_v2::MetaZwpTabletPadRingV2Source;
    pub(super) use super::tablet_v2::zwp_tablet_pad_strip_v2::MetaZwpTabletPadStripV2;
    pub(super) use super::tablet_v2::zwp_tablet_pad_strip_v2::MetaZwpTabletPadStripV2Source;
    pub(super) use super::tablet_v2::zwp_tablet_pad_v2::MetaZwpTabletPadV2;
    pub(super) use super::tablet_v2::zwp_tablet_pad_v2::MetaZwpTabletPadV2ButtonState;
    pub(super) use super::tablet_v2::zwp_tablet_seat_v2::MetaZwpTabletSeatV2;
    pub(super) use super::tablet_v2::zwp_tablet_tool_v2::MetaZwpTabletToolV2;
    pub(super) use super::tablet_v2::zwp_tablet_tool_v2::MetaZwpTabletToolV2Type;
    pub(super) use super::tablet_v2::zwp_tablet_tool_v2::MetaZwpTabletToolV2Capability;
    pub(super) use super::tablet_v2::zwp_tablet_tool_v2::MetaZwpTabletToolV2ButtonState;
    pub(super) use super::tablet_v2::zwp_tablet_tool_v2::MetaZwpTabletToolV2Error;
    pub(super) use super::tablet_v2::zwp_tablet_v2::MetaZwpTabletV2;
    pub(super) use super::tablet_v2::zwp_tablet_v2::MetaZwpTabletV2Bustype;
    pub(super) use super::tearing_control_v1::wp_tearing_control_manager_v1::MetaWpTearingControlManagerV1;
    pub(super) use super::tearing_control_v1::wp_tearing_control_manager_v1::MetaWpTearingControlManagerV1Error;
    pub(super) use super::tearing_control_v1::wp_tearing_control_v1::MetaWpTearingControlV1;
    pub(super) use super::tearing_control_v1::wp_tearing_control_v1::MetaWpTearingControlV1PresentationHint;
    pub(super) use super::text_input_unstable_v1::zwp_text_input_manager_v1::MetaZwpTextInputManagerV1;
    pub(super) use super::text_input_unstable_v1::zwp_text_input_v1::MetaZwpTextInputV1;
    pub(super) use super::text_input_unstable_v1::zwp_text_input_v1::MetaZwpTextInputV1ContentHint;
    pub(super) use super::text_input_unstable_v1::zwp_text_input_v1::MetaZwpTextInputV1ContentPurpose;
    pub(super) use super::text_input_unstable_v1::zwp_text_input_v1::MetaZwpTextInputV1PreeditStyle;
    pub(super) use super::text_input_unstable_v1::zwp_text_input_v1::MetaZwpTextInputV1TextDirection;
    pub(super) use super::text_input_unstable_v3::zwp_text_input_manager_v3::MetaZwpTextInputManagerV3;
    pub(super) use super::text_input_unstable_v3::zwp_text_input_v3::MetaZwpTextInputV3;
    pub(super) use super::text_input_unstable_v3::zwp_text_input_v3::MetaZwpTextInputV3ChangeCause;
    pub(super) use super::text_input_unstable_v3::zwp_text_input_v3::MetaZwpTextInputV3ContentHint;
    pub(super) use super::text_input_unstable_v3::zwp_text_input_v3::MetaZwpTextInputV3ContentPurpose;
    pub(super) use super::viewporter::wp_viewport::MetaWpViewport;
    pub(super) use super::viewporter::wp_viewport::MetaWpViewportError;
    pub(super) use super::viewporter::wp_viewporter::MetaWpViewporter;
    pub(super) use super::viewporter::wp_viewporter::MetaWpViewporterError;
    pub(super) use super::wayland::wl_buffer::MetaWlBuffer;
    pub(super) use super::wayland::wl_callback::MetaWlCallback;
    pub(super) use super::wayland::wl_compositor::MetaWlCompositor;
    pub(super) use super::wayland::wl_data_device::MetaWlDataDevice;
    pub(super) use super::wayland::wl_data_device::MetaWlDataDeviceError;
    pub(super) use super::wayland::wl_data_device_manager::MetaWlDataDeviceManager;
    pub(super) use super::wayland::wl_data_device_manager::MetaWlDataDeviceManagerDndAction;
    pub(super) use super::wayland::wl_data_offer::MetaWlDataOffer;
    pub(super) use super::wayland::wl_data_offer::MetaWlDataOfferError;
    pub(super) use super::wayland::wl_data_source::MetaWlDataSource;
    pub(super) use super::wayland::wl_data_source::MetaWlDataSourceError;
    pub(super) use super::wayland::wl_display::MetaWlDisplay;
    pub(super) use super::wayland::wl_display::MetaWlDisplayError;
    pub(super) use super::wayland::wl_fixes::MetaWlFixes;
    pub(super) use super::wayland::wl_keyboard::MetaWlKeyboard;
    pub(super) use super::wayland::wl_keyboard::MetaWlKeyboardKeymapFormat;
    pub(super) use super::wayland::wl_keyboard::MetaWlKeyboardKeyState;
    pub(super) use super::wayland::wl_output::MetaWlOutput;
    pub(super) use super::wayland::wl_output::MetaWlOutputSubpixel;
    pub(super) use super::wayland::wl_output::MetaWlOutputTransform;
    pub(super) use super::wayland::wl_output::MetaWlOutputMode;
    pub(super) use super::wayland::wl_pointer::MetaWlPointer;
    pub(super) use super::wayland::wl_pointer::MetaWlPointerError;
    pub(super) use super::wayland::wl_pointer::MetaWlPointerButtonState;
    pub(super) use super::wayland::wl_pointer::MetaWlPointerAxis;
    pub(super) use super::wayland::wl_pointer::MetaWlPointerAxisSource;
    pub(super) use super::wayland::wl_pointer::MetaWlPointerAxisRelativeDirection;
    pub(super) use super::wayland::wl_region::MetaWlRegion;
    pub(super) use super::wayland::wl_registry::MetaWlRegistry;
    pub(super) use super::wayland::wl_seat::MetaWlSeat;
    pub(super) use super::wayland::wl_seat::MetaWlSeatCapability;
    pub(super) use super::wayland::wl_seat::MetaWlSeatError;
    pub(super) use super::wayland::wl_shell::MetaWlShell;
    pub(super) use super::wayland::wl_shell::MetaWlShellError;
    pub(super) use super::wayland::wl_shell_surface::MetaWlShellSurface;
    pub(super) use super::wayland::wl_shell_surface::MetaWlShellSurfaceResize;
    pub(super) use super::wayland::wl_shell_surface::MetaWlShellSurfaceTransient;
    pub(super) use super::wayland::wl_shell_surface::MetaWlShellSurfaceFullscreenMethod;
    pub(super) use super::wayland::wl_shm::MetaWlShm;
    pub(super) use super::wayland::wl_shm::MetaWlShmError;
    pub(super) use super::wayland::wl_shm::MetaWlShmFormat;
    pub(super) use super::wayland::wl_shm_pool::MetaWlShmPool;
    pub(super) use super::wayland::wl_subcompositor::MetaWlSubcompositor;
    pub(super) use super::wayland::wl_subcompositor::MetaWlSubcompositorError;
    pub(super) use super::wayland::wl_subsurface::MetaWlSubsurface;
    pub(super) use super::wayland::wl_subsurface::MetaWlSubsurfaceError;
    pub(super) use super::wayland::wl_surface::MetaWlSurface;
    pub(super) use super::wayland::wl_surface::MetaWlSurfaceError;
    pub(super) use super::wayland::wl_touch::MetaWlTouch;
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_device_v1::MetaZwlrDataControlDeviceV1;
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_device_v1::MetaZwlrDataControlDeviceV1Error;
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_manager_v1::MetaZwlrDataControlManagerV1;
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_offer_v1::MetaZwlrDataControlOfferV1;
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_source_v1::MetaZwlrDataControlSourceV1;
    pub(super) use super::wlr_data_control_unstable_v1::zwlr_data_control_source_v1::MetaZwlrDataControlSourceV1Error;
    pub(super) use super::wlr_export_dmabuf_unstable_v1::zwlr_export_dmabuf_frame_v1::MetaZwlrExportDmabufFrameV1;
    pub(super) use super::wlr_export_dmabuf_unstable_v1::zwlr_export_dmabuf_frame_v1::MetaZwlrExportDmabufFrameV1Flags;
    pub(super) use super::wlr_export_dmabuf_unstable_v1::zwlr_export_dmabuf_frame_v1::MetaZwlrExportDmabufFrameV1CancelReason;
    pub(super) use super::wlr_export_dmabuf_unstable_v1::zwlr_export_dmabuf_manager_v1::MetaZwlrExportDmabufManagerV1;
    pub(super) use super::wlr_foreign_toplevel_management_unstable_v1::zwlr_foreign_toplevel_handle_v1::MetaZwlrForeignToplevelHandleV1;
    pub(super) use super::wlr_foreign_toplevel_management_unstable_v1::zwlr_foreign_toplevel_handle_v1::MetaZwlrForeignToplevelHandleV1State;
    pub(super) use super::wlr_foreign_toplevel_management_unstable_v1::zwlr_foreign_toplevel_handle_v1::MetaZwlrForeignToplevelHandleV1Error;
    pub(super) use super::wlr_foreign_toplevel_management_unstable_v1::zwlr_foreign_toplevel_manager_v1::MetaZwlrForeignToplevelManagerV1;
    pub(super) use super::wlr_gamma_control_unstable_v1::zwlr_gamma_control_manager_v1::MetaZwlrGammaControlManagerV1;
    pub(super) use super::wlr_gamma_control_unstable_v1::zwlr_gamma_control_v1::MetaZwlrGammaControlV1;
    pub(super) use super::wlr_gamma_control_unstable_v1::zwlr_gamma_control_v1::MetaZwlrGammaControlV1Error;
    pub(super) use super::wlr_input_inhibit_unstable_v1::zwlr_input_inhibit_manager_v1::MetaZwlrInputInhibitManagerV1;
    pub(super) use super::wlr_input_inhibit_unstable_v1::zwlr_input_inhibit_manager_v1::MetaZwlrInputInhibitManagerV1Error;
    pub(super) use super::wlr_input_inhibit_unstable_v1::zwlr_input_inhibitor_v1::MetaZwlrInputInhibitorV1;
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_shell_v1::MetaZwlrLayerShellV1;
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_shell_v1::MetaZwlrLayerShellV1Error;
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_shell_v1::MetaZwlrLayerShellV1Layer;
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_surface_v1::MetaZwlrLayerSurfaceV1;
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_surface_v1::MetaZwlrLayerSurfaceV1KeyboardInteractivity;
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_surface_v1::MetaZwlrLayerSurfaceV1Error;
    pub(super) use super::wlr_layer_shell_unstable_v1::zwlr_layer_surface_v1::MetaZwlrLayerSurfaceV1Anchor;
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_configuration_head_v1::MetaZwlrOutputConfigurationHeadV1;
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_configuration_head_v1::MetaZwlrOutputConfigurationHeadV1Error;
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_configuration_v1::MetaZwlrOutputConfigurationV1;
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_configuration_v1::MetaZwlrOutputConfigurationV1Error;
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_head_v1::MetaZwlrOutputHeadV1;
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_head_v1::MetaZwlrOutputHeadV1AdaptiveSyncState;
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_manager_v1::MetaZwlrOutputManagerV1;
    pub(super) use super::wlr_output_management_unstable_v1::zwlr_output_mode_v1::MetaZwlrOutputModeV1;
    pub(super) use super::wlr_output_power_management_unstable_v1::zwlr_output_power_manager_v1::MetaZwlrOutputPowerManagerV1;
    pub(super) use super::wlr_output_power_management_unstable_v1::zwlr_output_power_v1::MetaZwlrOutputPowerV1;
    pub(super) use super::wlr_output_power_management_unstable_v1::zwlr_output_power_v1::MetaZwlrOutputPowerV1Mode;
    pub(super) use super::wlr_output_power_management_unstable_v1::zwlr_output_power_v1::MetaZwlrOutputPowerV1Error;
    pub(super) use super::wlr_screencopy_unstable_v1::zwlr_screencopy_frame_v1::MetaZwlrScreencopyFrameV1;
    pub(super) use super::wlr_screencopy_unstable_v1::zwlr_screencopy_frame_v1::MetaZwlrScreencopyFrameV1Error;
    pub(super) use super::wlr_screencopy_unstable_v1::zwlr_screencopy_frame_v1::MetaZwlrScreencopyFrameV1Flags;
    pub(super) use super::wlr_screencopy_unstable_v1::zwlr_screencopy_manager_v1::MetaZwlrScreencopyManagerV1;
    pub(super) use super::wlr_virtual_pointer_unstable_v1::zwlr_virtual_pointer_manager_v1::MetaZwlrVirtualPointerManagerV1;
    pub(super) use super::wlr_virtual_pointer_unstable_v1::zwlr_virtual_pointer_v1::MetaZwlrVirtualPointerV1;
    pub(super) use super::wlr_virtual_pointer_unstable_v1::zwlr_virtual_pointer_v1::MetaZwlrVirtualPointerV1Error;
    pub(super) use super::xdg_activation_v1::xdg_activation_token_v1::MetaXdgActivationTokenV1;
    pub(super) use super::xdg_activation_v1::xdg_activation_token_v1::MetaXdgActivationTokenV1Error;
    pub(super) use super::xdg_activation_v1::xdg_activation_v1::MetaXdgActivationV1;
    pub(super) use super::xdg_decoration_unstable_v1::zxdg_decoration_manager_v1::MetaZxdgDecorationManagerV1;
    pub(super) use super::xdg_decoration_unstable_v1::zxdg_toplevel_decoration_v1::MetaZxdgToplevelDecorationV1;
    pub(super) use super::xdg_decoration_unstable_v1::zxdg_toplevel_decoration_v1::MetaZxdgToplevelDecorationV1Error;
    pub(super) use super::xdg_decoration_unstable_v1::zxdg_toplevel_decoration_v1::MetaZxdgToplevelDecorationV1Mode;
    pub(super) use super::xdg_dialog_v1::xdg_dialog_v1::MetaXdgDialogV1;
    pub(super) use super::xdg_dialog_v1::xdg_wm_dialog_v1::MetaXdgWmDialogV1;
    pub(super) use super::xdg_dialog_v1::xdg_wm_dialog_v1::MetaXdgWmDialogV1Error;
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_exported_v2::MetaZxdgExportedV2;
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_exporter_v2::MetaZxdgExporterV2;
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_exporter_v2::MetaZxdgExporterV2Error;
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_imported_v2::MetaZxdgImportedV2;
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_imported_v2::MetaZxdgImportedV2Error;
    pub(super) use super::xdg_foreign_unstable_v2::zxdg_importer_v2::MetaZxdgImporterV2;
    pub(super) use super::xdg_output_unstable_v1::zxdg_output_manager_v1::MetaZxdgOutputManagerV1;
    pub(super) use super::xdg_output_unstable_v1::zxdg_output_v1::MetaZxdgOutputV1;
    pub(super) use super::xdg_shell::xdg_popup::MetaXdgPopup;
    pub(super) use super::xdg_shell::xdg_popup::MetaXdgPopupError;
    pub(super) use super::xdg_shell::xdg_positioner::MetaXdgPositioner;
    pub(super) use super::xdg_shell::xdg_positioner::MetaXdgPositionerError;
    pub(super) use super::xdg_shell::xdg_positioner::MetaXdgPositionerAnchor;
    pub(super) use super::xdg_shell::xdg_positioner::MetaXdgPositionerGravity;
    pub(super) use super::xdg_shell::xdg_positioner::MetaXdgPositionerConstraintAdjustment;
    pub(super) use super::xdg_shell::xdg_surface::MetaXdgSurface;
    pub(super) use super::xdg_shell::xdg_surface::MetaXdgSurfaceError;
    pub(super) use super::xdg_shell::xdg_toplevel::MetaXdgToplevel;
    pub(super) use super::xdg_shell::xdg_toplevel::MetaXdgToplevelError;
    pub(super) use super::xdg_shell::xdg_toplevel::MetaXdgToplevelResizeEdge;
    pub(super) use super::xdg_shell::xdg_toplevel::MetaXdgToplevelState;
    pub(super) use super::xdg_shell::xdg_toplevel::MetaXdgToplevelWmCapabilities;
    pub(super) use super::xdg_shell::xdg_wm_base::MetaXdgWmBase;
    pub(super) use super::xdg_shell::xdg_wm_base::MetaXdgWmBaseError;
    pub(super) use super::xdg_system_bell_v1::xdg_system_bell_v1::MetaXdgSystemBellV1;
    pub(super) use super::xdg_toplevel_drag_v1::xdg_toplevel_drag_manager_v1::MetaXdgToplevelDragManagerV1;
    pub(super) use super::xdg_toplevel_drag_v1::xdg_toplevel_drag_manager_v1::MetaXdgToplevelDragManagerV1Error;
    pub(super) use super::xdg_toplevel_drag_v1::xdg_toplevel_drag_v1::MetaXdgToplevelDragV1;
    pub(super) use super::xdg_toplevel_drag_v1::xdg_toplevel_drag_v1::MetaXdgToplevelDragV1Error;
    pub(super) use super::xdg_toplevel_icon_v1::xdg_toplevel_icon_manager_v1::MetaXdgToplevelIconManagerV1;
    pub(super) use super::xdg_toplevel_icon_v1::xdg_toplevel_icon_v1::MetaXdgToplevelIconV1;
    pub(super) use super::xdg_toplevel_icon_v1::xdg_toplevel_icon_v1::MetaXdgToplevelIconV1Error;
    pub(super) use super::xdg_toplevel_tag_v1::xdg_toplevel_tag_manager_v1::MetaXdgToplevelTagManagerV1;
    pub(super) use super::xwayland_keyboard_grab_unstable_v1::zwp_xwayland_keyboard_grab_manager_v1::MetaZwpXwaylandKeyboardGrabManagerV1;
    pub(super) use super::xwayland_keyboard_grab_unstable_v1::zwp_xwayland_keyboard_grab_v1::MetaZwpXwaylandKeyboardGrabV1;
    pub(super) use super::xwayland_shell_v1::xwayland_shell_v1::MetaXwaylandShellV1;
    pub(super) use super::xwayland_shell_v1::xwayland_shell_v1::MetaXwaylandShellV1Error;
    pub(super) use super::xwayland_shell_v1::xwayland_surface_v1::MetaXwaylandSurfaceV1;
    pub(super) use super::xwayland_shell_v1::xwayland_surface_v1::MetaXwaylandSurfaceV1Error;

    use crate::generated_helper::prelude::*;

    pub(super) fn create_proxy_for_interface(state: &Rc<InnerState>, interface: &str, version: u32) -> Result<Rc<dyn Proxy>, ObjectError> {
        static INTERFACES: phf::Map<&'static str, fn(&Rc<InnerState>, u32) -> Result<Rc<dyn Proxy>, ObjectError>> = phf::phf_map! {
            "wp_alpha_modifier_surface_v1" => |s, v| {
                if v > MetaWpAlphaModifierSurfaceV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpAlphaModifierSurfaceV1::new(s, v))
            },
            "wp_alpha_modifier_v1" => |s, v| {
                if v > MetaWpAlphaModifierV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpAlphaModifierV1::new(s, v))
            },
            "wp_color_management_output_v1" => |s, v| {
                if v > MetaWpColorManagementOutputV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpColorManagementOutputV1::new(s, v))
            },
            "wp_color_management_surface_feedback_v1" => |s, v| {
                if v > MetaWpColorManagementSurfaceFeedbackV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpColorManagementSurfaceFeedbackV1::new(s, v))
            },
            "wp_color_management_surface_v1" => |s, v| {
                if v > MetaWpColorManagementSurfaceV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpColorManagementSurfaceV1::new(s, v))
            },
            "wp_color_manager_v1" => |s, v| {
                if v > MetaWpColorManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpColorManagerV1::new(s, v))
            },
            "wp_image_description_creator_icc_v1" => |s, v| {
                if v > MetaWpImageDescriptionCreatorIccV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpImageDescriptionCreatorIccV1::new(s, v))
            },
            "wp_image_description_creator_params_v1" => |s, v| {
                if v > MetaWpImageDescriptionCreatorParamsV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpImageDescriptionCreatorParamsV1::new(s, v))
            },
            "wp_image_description_info_v1" => |s, v| {
                if v > MetaWpImageDescriptionInfoV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpImageDescriptionInfoV1::new(s, v))
            },
            "wp_image_description_v1" => |s, v| {
                if v > MetaWpImageDescriptionV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpImageDescriptionV1::new(s, v))
            },
            "wp_color_representation_manager_v1" => |s, v| {
                if v > MetaWpColorRepresentationManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpColorRepresentationManagerV1::new(s, v))
            },
            "wp_color_representation_surface_v1" => |s, v| {
                if v > MetaWpColorRepresentationSurfaceV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpColorRepresentationSurfaceV1::new(s, v))
            },
            "wp_commit_timer_v1" => |s, v| {
                if v > MetaWpCommitTimerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpCommitTimerV1::new(s, v))
            },
            "wp_commit_timing_manager_v1" => |s, v| {
                if v > MetaWpCommitTimingManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpCommitTimingManagerV1::new(s, v))
            },
            "wp_content_type_manager_v1" => |s, v| {
                if v > MetaWpContentTypeManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpContentTypeManagerV1::new(s, v))
            },
            "wp_content_type_v1" => |s, v| {
                if v > MetaWpContentTypeV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpContentTypeV1::new(s, v))
            },
            "wp_cursor_shape_device_v1" => |s, v| {
                if v > MetaWpCursorShapeDeviceV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpCursorShapeDeviceV1::new(s, v))
            },
            "wp_cursor_shape_manager_v1" => |s, v| {
                if v > MetaWpCursorShapeManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpCursorShapeManagerV1::new(s, v))
            },
            "wp_drm_lease_connector_v1" => |s, v| {
                if v > MetaWpDrmLeaseConnectorV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpDrmLeaseConnectorV1::new(s, v))
            },
            "wp_drm_lease_device_v1" => |s, v| {
                if v > MetaWpDrmLeaseDeviceV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpDrmLeaseDeviceV1::new(s, v))
            },
            "wp_drm_lease_request_v1" => |s, v| {
                if v > MetaWpDrmLeaseRequestV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpDrmLeaseRequestV1::new(s, v))
            },
            "wp_drm_lease_v1" => |s, v| {
                if v > MetaWpDrmLeaseV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpDrmLeaseV1::new(s, v))
            },
            "ext_background_effect_manager_v1" => |s, v| {
                if v > MetaExtBackgroundEffectManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtBackgroundEffectManagerV1::new(s, v))
            },
            "ext_background_effect_surface_v1" => |s, v| {
                if v > MetaExtBackgroundEffectSurfaceV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtBackgroundEffectSurfaceV1::new(s, v))
            },
            "ext_data_control_device_v1" => |s, v| {
                if v > MetaExtDataControlDeviceV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtDataControlDeviceV1::new(s, v))
            },
            "ext_data_control_manager_v1" => |s, v| {
                if v > MetaExtDataControlManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtDataControlManagerV1::new(s, v))
            },
            "ext_data_control_offer_v1" => |s, v| {
                if v > MetaExtDataControlOfferV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtDataControlOfferV1::new(s, v))
            },
            "ext_data_control_source_v1" => |s, v| {
                if v > MetaExtDataControlSourceV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtDataControlSourceV1::new(s, v))
            },
            "ext_foreign_toplevel_handle_v1" => |s, v| {
                if v > MetaExtForeignToplevelHandleV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtForeignToplevelHandleV1::new(s, v))
            },
            "ext_foreign_toplevel_list_v1" => |s, v| {
                if v > MetaExtForeignToplevelListV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtForeignToplevelListV1::new(s, v))
            },
            "ext_idle_notification_v1" => |s, v| {
                if v > MetaExtIdleNotificationV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtIdleNotificationV1::new(s, v))
            },
            "ext_idle_notifier_v1" => |s, v| {
                if v > MetaExtIdleNotifierV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtIdleNotifierV1::new(s, v))
            },
            "ext_foreign_toplevel_image_capture_source_manager_v1" => |s, v| {
                if v > MetaExtForeignToplevelImageCaptureSourceManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtForeignToplevelImageCaptureSourceManagerV1::new(s, v))
            },
            "ext_image_capture_source_v1" => |s, v| {
                if v > MetaExtImageCaptureSourceV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtImageCaptureSourceV1::new(s, v))
            },
            "ext_output_image_capture_source_manager_v1" => |s, v| {
                if v > MetaExtOutputImageCaptureSourceManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtOutputImageCaptureSourceManagerV1::new(s, v))
            },
            "ext_image_copy_capture_cursor_session_v1" => |s, v| {
                if v > MetaExtImageCopyCaptureCursorSessionV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtImageCopyCaptureCursorSessionV1::new(s, v))
            },
            "ext_image_copy_capture_frame_v1" => |s, v| {
                if v > MetaExtImageCopyCaptureFrameV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtImageCopyCaptureFrameV1::new(s, v))
            },
            "ext_image_copy_capture_manager_v1" => |s, v| {
                if v > MetaExtImageCopyCaptureManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtImageCopyCaptureManagerV1::new(s, v))
            },
            "ext_image_copy_capture_session_v1" => |s, v| {
                if v > MetaExtImageCopyCaptureSessionV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtImageCopyCaptureSessionV1::new(s, v))
            },
            "ext_session_lock_manager_v1" => |s, v| {
                if v > MetaExtSessionLockManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtSessionLockManagerV1::new(s, v))
            },
            "ext_session_lock_surface_v1" => |s, v| {
                if v > MetaExtSessionLockSurfaceV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtSessionLockSurfaceV1::new(s, v))
            },
            "ext_session_lock_v1" => |s, v| {
                if v > MetaExtSessionLockV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtSessionLockV1::new(s, v))
            },
            "ext_transient_seat_manager_v1" => |s, v| {
                if v > MetaExtTransientSeatManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtTransientSeatManagerV1::new(s, v))
            },
            "ext_transient_seat_v1" => |s, v| {
                if v > MetaExtTransientSeatV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtTransientSeatV1::new(s, v))
            },
            "ext_workspace_group_handle_v1" => |s, v| {
                if v > MetaExtWorkspaceGroupHandleV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtWorkspaceGroupHandleV1::new(s, v))
            },
            "ext_workspace_handle_v1" => |s, v| {
                if v > MetaExtWorkspaceHandleV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtWorkspaceHandleV1::new(s, v))
            },
            "ext_workspace_manager_v1" => |s, v| {
                if v > MetaExtWorkspaceManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaExtWorkspaceManagerV1::new(s, v))
            },
            "wp_fifo_manager_v1" => |s, v| {
                if v > MetaWpFifoManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpFifoManagerV1::new(s, v))
            },
            "wp_fifo_v1" => |s, v| {
                if v > MetaWpFifoV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpFifoV1::new(s, v))
            },
            "wp_fractional_scale_manager_v1" => |s, v| {
                if v > MetaWpFractionalScaleManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpFractionalScaleManagerV1::new(s, v))
            },
            "wp_fractional_scale_v1" => |s, v| {
                if v > MetaWpFractionalScaleV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpFractionalScaleV1::new(s, v))
            },
            "zwp_fullscreen_shell_mode_feedback_v1" => |s, v| {
                if v > MetaZwpFullscreenShellModeFeedbackV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpFullscreenShellModeFeedbackV1::new(s, v))
            },
            "zwp_fullscreen_shell_v1" => |s, v| {
                if v > MetaZwpFullscreenShellV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpFullscreenShellV1::new(s, v))
            },
            "zwp_idle_inhibit_manager_v1" => |s, v| {
                if v > MetaZwpIdleInhibitManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpIdleInhibitManagerV1::new(s, v))
            },
            "zwp_idle_inhibitor_v1" => |s, v| {
                if v > MetaZwpIdleInhibitorV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpIdleInhibitorV1::new(s, v))
            },
            "zwp_input_method_context_v1" => |s, v| {
                if v > MetaZwpInputMethodContextV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpInputMethodContextV1::new(s, v))
            },
            "zwp_input_method_v1" => |s, v| {
                if v > MetaZwpInputMethodV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpInputMethodV1::new(s, v))
            },
            "zwp_input_panel_surface_v1" => |s, v| {
                if v > MetaZwpInputPanelSurfaceV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpInputPanelSurfaceV1::new(s, v))
            },
            "zwp_input_panel_v1" => |s, v| {
                if v > MetaZwpInputPanelV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpInputPanelV1::new(s, v))
            },
            "zwp_input_timestamps_manager_v1" => |s, v| {
                if v > MetaZwpInputTimestampsManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpInputTimestampsManagerV1::new(s, v))
            },
            "zwp_input_timestamps_v1" => |s, v| {
                if v > MetaZwpInputTimestampsV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpInputTimestampsV1::new(s, v))
            },
            "zwp_keyboard_shortcuts_inhibit_manager_v1" => |s, v| {
                if v > MetaZwpKeyboardShortcutsInhibitManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpKeyboardShortcutsInhibitManagerV1::new(s, v))
            },
            "zwp_keyboard_shortcuts_inhibitor_v1" => |s, v| {
                if v > MetaZwpKeyboardShortcutsInhibitorV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpKeyboardShortcutsInhibitorV1::new(s, v))
            },
            "zwp_linux_buffer_params_v1" => |s, v| {
                if v > MetaZwpLinuxBufferParamsV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpLinuxBufferParamsV1::new(s, v))
            },
            "zwp_linux_dmabuf_feedback_v1" => |s, v| {
                if v > MetaZwpLinuxDmabufFeedbackV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpLinuxDmabufFeedbackV1::new(s, v))
            },
            "zwp_linux_dmabuf_v1" => |s, v| {
                if v > MetaZwpLinuxDmabufV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpLinuxDmabufV1::new(s, v))
            },
            "wp_linux_drm_syncobj_manager_v1" => |s, v| {
                if v > MetaWpLinuxDrmSyncobjManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpLinuxDrmSyncobjManagerV1::new(s, v))
            },
            "wp_linux_drm_syncobj_surface_v1" => |s, v| {
                if v > MetaWpLinuxDrmSyncobjSurfaceV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpLinuxDrmSyncobjSurfaceV1::new(s, v))
            },
            "wp_linux_drm_syncobj_timeline_v1" => |s, v| {
                if v > MetaWpLinuxDrmSyncobjTimelineV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpLinuxDrmSyncobjTimelineV1::new(s, v))
            },
            "zwp_linux_buffer_release_v1" => |s, v| {
                if v > MetaZwpLinuxBufferReleaseV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpLinuxBufferReleaseV1::new(s, v))
            },
            "zwp_linux_explicit_synchronization_v1" => |s, v| {
                if v > MetaZwpLinuxExplicitSynchronizationV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpLinuxExplicitSynchronizationV1::new(s, v))
            },
            "zwp_linux_surface_synchronization_v1" => |s, v| {
                if v > MetaZwpLinuxSurfaceSynchronizationV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpLinuxSurfaceSynchronizationV1::new(s, v))
            },
            "zwp_confined_pointer_v1" => |s, v| {
                if v > MetaZwpConfinedPointerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpConfinedPointerV1::new(s, v))
            },
            "zwp_locked_pointer_v1" => |s, v| {
                if v > MetaZwpLockedPointerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpLockedPointerV1::new(s, v))
            },
            "zwp_pointer_constraints_v1" => |s, v| {
                if v > MetaZwpPointerConstraintsV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpPointerConstraintsV1::new(s, v))
            },
            "zwp_pointer_gesture_hold_v1" => |s, v| {
                if v > MetaZwpPointerGestureHoldV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpPointerGestureHoldV1::new(s, v))
            },
            "zwp_pointer_gesture_pinch_v1" => |s, v| {
                if v > MetaZwpPointerGesturePinchV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpPointerGesturePinchV1::new(s, v))
            },
            "zwp_pointer_gesture_swipe_v1" => |s, v| {
                if v > MetaZwpPointerGestureSwipeV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpPointerGestureSwipeV1::new(s, v))
            },
            "zwp_pointer_gestures_v1" => |s, v| {
                if v > MetaZwpPointerGesturesV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpPointerGesturesV1::new(s, v))
            },
            "wp_pointer_warp_v1" => |s, v| {
                if v > MetaWpPointerWarpV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpPointerWarpV1::new(s, v))
            },
            "wp_presentation" => |s, v| {
                if v > MetaWpPresentation::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpPresentation::new(s, v))
            },
            "wp_presentation_feedback" => |s, v| {
                if v > MetaWpPresentationFeedback::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpPresentationFeedback::new(s, v))
            },
            "zwp_primary_selection_device_manager_v1" => |s, v| {
                if v > MetaZwpPrimarySelectionDeviceManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpPrimarySelectionDeviceManagerV1::new(s, v))
            },
            "zwp_primary_selection_device_v1" => |s, v| {
                if v > MetaZwpPrimarySelectionDeviceV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpPrimarySelectionDeviceV1::new(s, v))
            },
            "zwp_primary_selection_offer_v1" => |s, v| {
                if v > MetaZwpPrimarySelectionOfferV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpPrimarySelectionOfferV1::new(s, v))
            },
            "zwp_primary_selection_source_v1" => |s, v| {
                if v > MetaZwpPrimarySelectionSourceV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpPrimarySelectionSourceV1::new(s, v))
            },
            "zwp_relative_pointer_manager_v1" => |s, v| {
                if v > MetaZwpRelativePointerManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpRelativePointerManagerV1::new(s, v))
            },
            "zwp_relative_pointer_v1" => |s, v| {
                if v > MetaZwpRelativePointerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpRelativePointerV1::new(s, v))
            },
            "wp_security_context_manager_v1" => |s, v| {
                if v > MetaWpSecurityContextManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpSecurityContextManagerV1::new(s, v))
            },
            "wp_security_context_v1" => |s, v| {
                if v > MetaWpSecurityContextV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpSecurityContextV1::new(s, v))
            },
            "wp_single_pixel_buffer_manager_v1" => |s, v| {
                if v > MetaWpSinglePixelBufferManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpSinglePixelBufferManagerV1::new(s, v))
            },
            "zwp_tablet_manager_v2" => |s, v| {
                if v > MetaZwpTabletManagerV2::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpTabletManagerV2::new(s, v))
            },
            "zwp_tablet_pad_dial_v2" => |s, v| {
                if v > MetaZwpTabletPadDialV2::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpTabletPadDialV2::new(s, v))
            },
            "zwp_tablet_pad_group_v2" => |s, v| {
                if v > MetaZwpTabletPadGroupV2::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpTabletPadGroupV2::new(s, v))
            },
            "zwp_tablet_pad_ring_v2" => |s, v| {
                if v > MetaZwpTabletPadRingV2::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpTabletPadRingV2::new(s, v))
            },
            "zwp_tablet_pad_strip_v2" => |s, v| {
                if v > MetaZwpTabletPadStripV2::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpTabletPadStripV2::new(s, v))
            },
            "zwp_tablet_pad_v2" => |s, v| {
                if v > MetaZwpTabletPadV2::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpTabletPadV2::new(s, v))
            },
            "zwp_tablet_seat_v2" => |s, v| {
                if v > MetaZwpTabletSeatV2::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpTabletSeatV2::new(s, v))
            },
            "zwp_tablet_tool_v2" => |s, v| {
                if v > MetaZwpTabletToolV2::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpTabletToolV2::new(s, v))
            },
            "zwp_tablet_v2" => |s, v| {
                if v > MetaZwpTabletV2::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpTabletV2::new(s, v))
            },
            "wp_tearing_control_manager_v1" => |s, v| {
                if v > MetaWpTearingControlManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpTearingControlManagerV1::new(s, v))
            },
            "wp_tearing_control_v1" => |s, v| {
                if v > MetaWpTearingControlV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpTearingControlV1::new(s, v))
            },
            "zwp_text_input_manager_v1" => |s, v| {
                if v > MetaZwpTextInputManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpTextInputManagerV1::new(s, v))
            },
            "zwp_text_input_v1" => |s, v| {
                if v > MetaZwpTextInputV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpTextInputV1::new(s, v))
            },
            "zwp_text_input_manager_v3" => |s, v| {
                if v > MetaZwpTextInputManagerV3::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpTextInputManagerV3::new(s, v))
            },
            "zwp_text_input_v3" => |s, v| {
                if v > MetaZwpTextInputV3::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpTextInputV3::new(s, v))
            },
            "wp_viewport" => |s, v| {
                if v > MetaWpViewport::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpViewport::new(s, v))
            },
            "wp_viewporter" => |s, v| {
                if v > MetaWpViewporter::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWpViewporter::new(s, v))
            },
            "wl_buffer" => |s, v| {
                if v > MetaWlBuffer::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWlBuffer::new(s, v))
            },
            "wl_callback" => |s, v| {
                if v > MetaWlCallback::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWlCallback::new(s, v))
            },
            "wl_compositor" => |s, v| {
                if v > MetaWlCompositor::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWlCompositor::new(s, v))
            },
            "wl_data_device" => |s, v| {
                if v > MetaWlDataDevice::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWlDataDevice::new(s, v))
            },
            "wl_data_device_manager" => |s, v| {
                if v > MetaWlDataDeviceManager::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWlDataDeviceManager::new(s, v))
            },
            "wl_data_offer" => |s, v| {
                if v > MetaWlDataOffer::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWlDataOffer::new(s, v))
            },
            "wl_data_source" => |s, v| {
                if v > MetaWlDataSource::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWlDataSource::new(s, v))
            },
            "wl_display" => |s, v| {
                if v > MetaWlDisplay::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWlDisplay::new(s, v))
            },
            "wl_fixes" => |s, v| {
                if v > MetaWlFixes::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWlFixes::new(s, v))
            },
            "wl_keyboard" => |s, v| {
                if v > MetaWlKeyboard::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWlKeyboard::new(s, v))
            },
            "wl_output" => |s, v| {
                if v > MetaWlOutput::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWlOutput::new(s, v))
            },
            "wl_pointer" => |s, v| {
                if v > MetaWlPointer::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWlPointer::new(s, v))
            },
            "wl_region" => |s, v| {
                if v > MetaWlRegion::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWlRegion::new(s, v))
            },
            "wl_registry" => |s, v| {
                if v > MetaWlRegistry::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWlRegistry::new(s, v))
            },
            "wl_seat" => |s, v| {
                if v > MetaWlSeat::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWlSeat::new(s, v))
            },
            "wl_shell" => |s, v| {
                if v > MetaWlShell::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWlShell::new(s, v))
            },
            "wl_shell_surface" => |s, v| {
                if v > MetaWlShellSurface::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWlShellSurface::new(s, v))
            },
            "wl_shm" => |s, v| {
                if v > MetaWlShm::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWlShm::new(s, v))
            },
            "wl_shm_pool" => |s, v| {
                if v > MetaWlShmPool::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWlShmPool::new(s, v))
            },
            "wl_subcompositor" => |s, v| {
                if v > MetaWlSubcompositor::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWlSubcompositor::new(s, v))
            },
            "wl_subsurface" => |s, v| {
                if v > MetaWlSubsurface::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWlSubsurface::new(s, v))
            },
            "wl_surface" => |s, v| {
                if v > MetaWlSurface::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWlSurface::new(s, v))
            },
            "wl_touch" => |s, v| {
                if v > MetaWlTouch::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaWlTouch::new(s, v))
            },
            "zwlr_data_control_device_v1" => |s, v| {
                if v > MetaZwlrDataControlDeviceV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrDataControlDeviceV1::new(s, v))
            },
            "zwlr_data_control_manager_v1" => |s, v| {
                if v > MetaZwlrDataControlManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrDataControlManagerV1::new(s, v))
            },
            "zwlr_data_control_offer_v1" => |s, v| {
                if v > MetaZwlrDataControlOfferV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrDataControlOfferV1::new(s, v))
            },
            "zwlr_data_control_source_v1" => |s, v| {
                if v > MetaZwlrDataControlSourceV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrDataControlSourceV1::new(s, v))
            },
            "zwlr_export_dmabuf_frame_v1" => |s, v| {
                if v > MetaZwlrExportDmabufFrameV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrExportDmabufFrameV1::new(s, v))
            },
            "zwlr_export_dmabuf_manager_v1" => |s, v| {
                if v > MetaZwlrExportDmabufManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrExportDmabufManagerV1::new(s, v))
            },
            "zwlr_foreign_toplevel_handle_v1" => |s, v| {
                if v > MetaZwlrForeignToplevelHandleV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrForeignToplevelHandleV1::new(s, v))
            },
            "zwlr_foreign_toplevel_manager_v1" => |s, v| {
                if v > MetaZwlrForeignToplevelManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrForeignToplevelManagerV1::new(s, v))
            },
            "zwlr_gamma_control_manager_v1" => |s, v| {
                if v > MetaZwlrGammaControlManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrGammaControlManagerV1::new(s, v))
            },
            "zwlr_gamma_control_v1" => |s, v| {
                if v > MetaZwlrGammaControlV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrGammaControlV1::new(s, v))
            },
            "zwlr_input_inhibit_manager_v1" => |s, v| {
                if v > MetaZwlrInputInhibitManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrInputInhibitManagerV1::new(s, v))
            },
            "zwlr_input_inhibitor_v1" => |s, v| {
                if v > MetaZwlrInputInhibitorV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrInputInhibitorV1::new(s, v))
            },
            "zwlr_layer_shell_v1" => |s, v| {
                if v > MetaZwlrLayerShellV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrLayerShellV1::new(s, v))
            },
            "zwlr_layer_surface_v1" => |s, v| {
                if v > MetaZwlrLayerSurfaceV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrLayerSurfaceV1::new(s, v))
            },
            "zwlr_output_configuration_head_v1" => |s, v| {
                if v > MetaZwlrOutputConfigurationHeadV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrOutputConfigurationHeadV1::new(s, v))
            },
            "zwlr_output_configuration_v1" => |s, v| {
                if v > MetaZwlrOutputConfigurationV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrOutputConfigurationV1::new(s, v))
            },
            "zwlr_output_head_v1" => |s, v| {
                if v > MetaZwlrOutputHeadV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrOutputHeadV1::new(s, v))
            },
            "zwlr_output_manager_v1" => |s, v| {
                if v > MetaZwlrOutputManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrOutputManagerV1::new(s, v))
            },
            "zwlr_output_mode_v1" => |s, v| {
                if v > MetaZwlrOutputModeV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrOutputModeV1::new(s, v))
            },
            "zwlr_output_power_manager_v1" => |s, v| {
                if v > MetaZwlrOutputPowerManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrOutputPowerManagerV1::new(s, v))
            },
            "zwlr_output_power_v1" => |s, v| {
                if v > MetaZwlrOutputPowerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrOutputPowerV1::new(s, v))
            },
            "zwlr_screencopy_frame_v1" => |s, v| {
                if v > MetaZwlrScreencopyFrameV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrScreencopyFrameV1::new(s, v))
            },
            "zwlr_screencopy_manager_v1" => |s, v| {
                if v > MetaZwlrScreencopyManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrScreencopyManagerV1::new(s, v))
            },
            "zwlr_virtual_pointer_manager_v1" => |s, v| {
                if v > MetaZwlrVirtualPointerManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrVirtualPointerManagerV1::new(s, v))
            },
            "zwlr_virtual_pointer_v1" => |s, v| {
                if v > MetaZwlrVirtualPointerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwlrVirtualPointerV1::new(s, v))
            },
            "xdg_activation_token_v1" => |s, v| {
                if v > MetaXdgActivationTokenV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaXdgActivationTokenV1::new(s, v))
            },
            "xdg_activation_v1" => |s, v| {
                if v > MetaXdgActivationV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaXdgActivationV1::new(s, v))
            },
            "zxdg_decoration_manager_v1" => |s, v| {
                if v > MetaZxdgDecorationManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZxdgDecorationManagerV1::new(s, v))
            },
            "zxdg_toplevel_decoration_v1" => |s, v| {
                if v > MetaZxdgToplevelDecorationV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZxdgToplevelDecorationV1::new(s, v))
            },
            "xdg_dialog_v1" => |s, v| {
                if v > MetaXdgDialogV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaXdgDialogV1::new(s, v))
            },
            "xdg_wm_dialog_v1" => |s, v| {
                if v > MetaXdgWmDialogV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaXdgWmDialogV1::new(s, v))
            },
            "zxdg_exported_v2" => |s, v| {
                if v > MetaZxdgExportedV2::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZxdgExportedV2::new(s, v))
            },
            "zxdg_exporter_v2" => |s, v| {
                if v > MetaZxdgExporterV2::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZxdgExporterV2::new(s, v))
            },
            "zxdg_imported_v2" => |s, v| {
                if v > MetaZxdgImportedV2::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZxdgImportedV2::new(s, v))
            },
            "zxdg_importer_v2" => |s, v| {
                if v > MetaZxdgImporterV2::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZxdgImporterV2::new(s, v))
            },
            "zxdg_output_manager_v1" => |s, v| {
                if v > MetaZxdgOutputManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZxdgOutputManagerV1::new(s, v))
            },
            "zxdg_output_v1" => |s, v| {
                if v > MetaZxdgOutputV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZxdgOutputV1::new(s, v))
            },
            "xdg_popup" => |s, v| {
                if v > MetaXdgPopup::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaXdgPopup::new(s, v))
            },
            "xdg_positioner" => |s, v| {
                if v > MetaXdgPositioner::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaXdgPositioner::new(s, v))
            },
            "xdg_surface" => |s, v| {
                if v > MetaXdgSurface::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaXdgSurface::new(s, v))
            },
            "xdg_toplevel" => |s, v| {
                if v > MetaXdgToplevel::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaXdgToplevel::new(s, v))
            },
            "xdg_wm_base" => |s, v| {
                if v > MetaXdgWmBase::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaXdgWmBase::new(s, v))
            },
            "xdg_system_bell_v1" => |s, v| {
                if v > MetaXdgSystemBellV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaXdgSystemBellV1::new(s, v))
            },
            "xdg_toplevel_drag_manager_v1" => |s, v| {
                if v > MetaXdgToplevelDragManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaXdgToplevelDragManagerV1::new(s, v))
            },
            "xdg_toplevel_drag_v1" => |s, v| {
                if v > MetaXdgToplevelDragV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaXdgToplevelDragV1::new(s, v))
            },
            "xdg_toplevel_icon_manager_v1" => |s, v| {
                if v > MetaXdgToplevelIconManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaXdgToplevelIconManagerV1::new(s, v))
            },
            "xdg_toplevel_icon_v1" => |s, v| {
                if v > MetaXdgToplevelIconV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaXdgToplevelIconV1::new(s, v))
            },
            "xdg_toplevel_tag_manager_v1" => |s, v| {
                if v > MetaXdgToplevelTagManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaXdgToplevelTagManagerV1::new(s, v))
            },
            "zwp_xwayland_keyboard_grab_manager_v1" => |s, v| {
                if v > MetaZwpXwaylandKeyboardGrabManagerV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpXwaylandKeyboardGrabManagerV1::new(s, v))
            },
            "zwp_xwayland_keyboard_grab_v1" => |s, v| {
                if v > MetaZwpXwaylandKeyboardGrabV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaZwpXwaylandKeyboardGrabV1::new(s, v))
            },
            "xwayland_shell_v1" => |s, v| {
                if v > MetaXwaylandShellV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaXwaylandShellV1::new(s, v))
            },
            "xwayland_surface_v1" => |s, v| {
                if v > MetaXwaylandSurfaceV1::XML_VERSION {
                    return Err(ObjectError);
                }
                Ok(MetaXwaylandSurfaceV1::new(s, v))
            },
        };
        INTERFACES.get(interface).ok_or(ObjectError).and_then(|i| i(state, version))
    }

    pub(super) fn xml_interface_version(interface: &str) -> Option<u32> {
        static INTERFACES: phf::Map<&'static str, u32> = phf::phf_map! {
            "wp_alpha_modifier_surface_v1" => 1,
            "wp_alpha_modifier_v1" => 1,
            "wp_color_management_output_v1" => 1,
            "wp_color_management_surface_feedback_v1" => 1,
            "wp_color_management_surface_v1" => 1,
            "wp_color_manager_v1" => 1,
            "wp_image_description_creator_icc_v1" => 1,
            "wp_image_description_creator_params_v1" => 1,
            "wp_image_description_info_v1" => 1,
            "wp_image_description_v1" => 1,
            "wp_color_representation_manager_v1" => 1,
            "wp_color_representation_surface_v1" => 1,
            "wp_commit_timer_v1" => 1,
            "wp_commit_timing_manager_v1" => 1,
            "wp_content_type_manager_v1" => 1,
            "wp_content_type_v1" => 1,
            "wp_cursor_shape_device_v1" => 2,
            "wp_cursor_shape_manager_v1" => 2,
            "wp_drm_lease_connector_v1" => 1,
            "wp_drm_lease_device_v1" => 1,
            "wp_drm_lease_request_v1" => 1,
            "wp_drm_lease_v1" => 1,
            "ext_background_effect_manager_v1" => 1,
            "ext_background_effect_surface_v1" => 1,
            "ext_data_control_device_v1" => 1,
            "ext_data_control_manager_v1" => 1,
            "ext_data_control_offer_v1" => 1,
            "ext_data_control_source_v1" => 1,
            "ext_foreign_toplevel_handle_v1" => 1,
            "ext_foreign_toplevel_list_v1" => 1,
            "ext_idle_notification_v1" => 2,
            "ext_idle_notifier_v1" => 2,
            "ext_foreign_toplevel_image_capture_source_manager_v1" => 1,
            "ext_image_capture_source_v1" => 1,
            "ext_output_image_capture_source_manager_v1" => 1,
            "ext_image_copy_capture_cursor_session_v1" => 1,
            "ext_image_copy_capture_frame_v1" => 1,
            "ext_image_copy_capture_manager_v1" => 1,
            "ext_image_copy_capture_session_v1" => 1,
            "ext_session_lock_manager_v1" => 1,
            "ext_session_lock_surface_v1" => 1,
            "ext_session_lock_v1" => 1,
            "ext_transient_seat_manager_v1" => 1,
            "ext_transient_seat_v1" => 1,
            "ext_workspace_group_handle_v1" => 1,
            "ext_workspace_handle_v1" => 1,
            "ext_workspace_manager_v1" => 1,
            "wp_fifo_manager_v1" => 1,
            "wp_fifo_v1" => 1,
            "wp_fractional_scale_manager_v1" => 1,
            "wp_fractional_scale_v1" => 1,
            "zwp_fullscreen_shell_mode_feedback_v1" => 1,
            "zwp_fullscreen_shell_v1" => 1,
            "zwp_idle_inhibit_manager_v1" => 1,
            "zwp_idle_inhibitor_v1" => 1,
            "zwp_input_method_context_v1" => 1,
            "zwp_input_method_v1" => 1,
            "zwp_input_panel_surface_v1" => 1,
            "zwp_input_panel_v1" => 1,
            "zwp_input_timestamps_manager_v1" => 1,
            "zwp_input_timestamps_v1" => 1,
            "zwp_keyboard_shortcuts_inhibit_manager_v1" => 1,
            "zwp_keyboard_shortcuts_inhibitor_v1" => 1,
            "zwp_linux_buffer_params_v1" => 5,
            "zwp_linux_dmabuf_feedback_v1" => 5,
            "zwp_linux_dmabuf_v1" => 5,
            "wp_linux_drm_syncobj_manager_v1" => 1,
            "wp_linux_drm_syncobj_surface_v1" => 1,
            "wp_linux_drm_syncobj_timeline_v1" => 1,
            "zwp_linux_buffer_release_v1" => 1,
            "zwp_linux_explicit_synchronization_v1" => 2,
            "zwp_linux_surface_synchronization_v1" => 2,
            "zwp_confined_pointer_v1" => 1,
            "zwp_locked_pointer_v1" => 1,
            "zwp_pointer_constraints_v1" => 1,
            "zwp_pointer_gesture_hold_v1" => 3,
            "zwp_pointer_gesture_pinch_v1" => 2,
            "zwp_pointer_gesture_swipe_v1" => 2,
            "zwp_pointer_gestures_v1" => 3,
            "wp_pointer_warp_v1" => 1,
            "wp_presentation" => 2,
            "wp_presentation_feedback" => 2,
            "zwp_primary_selection_device_manager_v1" => 1,
            "zwp_primary_selection_device_v1" => 1,
            "zwp_primary_selection_offer_v1" => 1,
            "zwp_primary_selection_source_v1" => 1,
            "zwp_relative_pointer_manager_v1" => 1,
            "zwp_relative_pointer_v1" => 1,
            "wp_security_context_manager_v1" => 1,
            "wp_security_context_v1" => 1,
            "wp_single_pixel_buffer_manager_v1" => 1,
            "zwp_tablet_manager_v2" => 2,
            "zwp_tablet_pad_dial_v2" => 2,
            "zwp_tablet_pad_group_v2" => 2,
            "zwp_tablet_pad_ring_v2" => 2,
            "zwp_tablet_pad_strip_v2" => 2,
            "zwp_tablet_pad_v2" => 2,
            "zwp_tablet_seat_v2" => 2,
            "zwp_tablet_tool_v2" => 2,
            "zwp_tablet_v2" => 2,
            "wp_tearing_control_manager_v1" => 1,
            "wp_tearing_control_v1" => 1,
            "zwp_text_input_manager_v1" => 1,
            "zwp_text_input_v1" => 1,
            "zwp_text_input_manager_v3" => 1,
            "zwp_text_input_v3" => 1,
            "wp_viewport" => 1,
            "wp_viewporter" => 1,
            "wl_buffer" => 1,
            "wl_callback" => 1,
            "wl_compositor" => 6,
            "wl_data_device" => 3,
            "wl_data_device_manager" => 3,
            "wl_data_offer" => 3,
            "wl_data_source" => 3,
            "wl_display" => 1,
            "wl_fixes" => 1,
            "wl_keyboard" => 10,
            "wl_output" => 4,
            "wl_pointer" => 10,
            "wl_region" => 1,
            "wl_registry" => 1,
            "wl_seat" => 10,
            "wl_shell" => 1,
            "wl_shell_surface" => 1,
            "wl_shm" => 2,
            "wl_shm_pool" => 2,
            "wl_subcompositor" => 1,
            "wl_subsurface" => 1,
            "wl_surface" => 6,
            "wl_touch" => 10,
            "zwlr_data_control_device_v1" => 2,
            "zwlr_data_control_manager_v1" => 2,
            "zwlr_data_control_offer_v1" => 1,
            "zwlr_data_control_source_v1" => 1,
            "zwlr_export_dmabuf_frame_v1" => 1,
            "zwlr_export_dmabuf_manager_v1" => 1,
            "zwlr_foreign_toplevel_handle_v1" => 3,
            "zwlr_foreign_toplevel_manager_v1" => 3,
            "zwlr_gamma_control_manager_v1" => 1,
            "zwlr_gamma_control_v1" => 1,
            "zwlr_input_inhibit_manager_v1" => 1,
            "zwlr_input_inhibitor_v1" => 1,
            "zwlr_layer_shell_v1" => 5,
            "zwlr_layer_surface_v1" => 5,
            "zwlr_output_configuration_head_v1" => 4,
            "zwlr_output_configuration_v1" => 4,
            "zwlr_output_head_v1" => 4,
            "zwlr_output_manager_v1" => 4,
            "zwlr_output_mode_v1" => 3,
            "zwlr_output_power_manager_v1" => 1,
            "zwlr_output_power_v1" => 1,
            "zwlr_screencopy_frame_v1" => 3,
            "zwlr_screencopy_manager_v1" => 3,
            "zwlr_virtual_pointer_manager_v1" => 2,
            "zwlr_virtual_pointer_v1" => 2,
            "xdg_activation_token_v1" => 1,
            "xdg_activation_v1" => 1,
            "zxdg_decoration_manager_v1" => 1,
            "zxdg_toplevel_decoration_v1" => 1,
            "xdg_dialog_v1" => 1,
            "xdg_wm_dialog_v1" => 1,
            "zxdg_exported_v2" => 1,
            "zxdg_exporter_v2" => 1,
            "zxdg_imported_v2" => 1,
            "zxdg_importer_v2" => 1,
            "zxdg_output_manager_v1" => 3,
            "zxdg_output_v1" => 3,
            "xdg_popup" => 7,
            "xdg_positioner" => 7,
            "xdg_surface" => 7,
            "xdg_toplevel" => 7,
            "xdg_wm_base" => 7,
            "xdg_system_bell_v1" => 1,
            "xdg_toplevel_drag_manager_v1" => 1,
            "xdg_toplevel_drag_v1" => 1,
            "xdg_toplevel_icon_manager_v1" => 1,
            "xdg_toplevel_icon_v1" => 1,
            "xdg_toplevel_tag_manager_v1" => 1,
            "zwp_xwayland_keyboard_grab_manager_v1" => 1,
            "zwp_xwayland_keyboard_grab_v1" => 1,
            "xwayland_shell_v1" => 1,
            "xwayland_surface_v1" => 1,
        };
        INTERFACES.get(interface).copied()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
}
