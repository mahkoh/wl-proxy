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
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpAlphaModifierSurfaceV1, v));
                }
                Ok(MetaWpAlphaModifierSurfaceV1::new(s, v))
            },
            "wp_alpha_modifier_v1" => |s, v| {
                if v > MetaWpAlphaModifierV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpAlphaModifierV1, v));
                }
                Ok(MetaWpAlphaModifierV1::new(s, v))
            },
            "wp_color_management_output_v1" => |s, v| {
                if v > MetaWpColorManagementOutputV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpColorManagementOutputV1, v));
                }
                Ok(MetaWpColorManagementOutputV1::new(s, v))
            },
            "wp_color_management_surface_feedback_v1" => |s, v| {
                if v > MetaWpColorManagementSurfaceFeedbackV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpColorManagementSurfaceFeedbackV1, v));
                }
                Ok(MetaWpColorManagementSurfaceFeedbackV1::new(s, v))
            },
            "wp_color_management_surface_v1" => |s, v| {
                if v > MetaWpColorManagementSurfaceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpColorManagementSurfaceV1, v));
                }
                Ok(MetaWpColorManagementSurfaceV1::new(s, v))
            },
            "wp_color_manager_v1" => |s, v| {
                if v > MetaWpColorManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpColorManagerV1, v));
                }
                Ok(MetaWpColorManagerV1::new(s, v))
            },
            "wp_image_description_creator_icc_v1" => |s, v| {
                if v > MetaWpImageDescriptionCreatorIccV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpImageDescriptionCreatorIccV1, v));
                }
                Ok(MetaWpImageDescriptionCreatorIccV1::new(s, v))
            },
            "wp_image_description_creator_params_v1" => |s, v| {
                if v > MetaWpImageDescriptionCreatorParamsV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpImageDescriptionCreatorParamsV1, v));
                }
                Ok(MetaWpImageDescriptionCreatorParamsV1::new(s, v))
            },
            "wp_image_description_info_v1" => |s, v| {
                if v > MetaWpImageDescriptionInfoV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpImageDescriptionInfoV1, v));
                }
                Ok(MetaWpImageDescriptionInfoV1::new(s, v))
            },
            "wp_image_description_v1" => |s, v| {
                if v > MetaWpImageDescriptionV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpImageDescriptionV1, v));
                }
                Ok(MetaWpImageDescriptionV1::new(s, v))
            },
            "wp_color_representation_manager_v1" => |s, v| {
                if v > MetaWpColorRepresentationManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpColorRepresentationManagerV1, v));
                }
                Ok(MetaWpColorRepresentationManagerV1::new(s, v))
            },
            "wp_color_representation_surface_v1" => |s, v| {
                if v > MetaWpColorRepresentationSurfaceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpColorRepresentationSurfaceV1, v));
                }
                Ok(MetaWpColorRepresentationSurfaceV1::new(s, v))
            },
            "wp_commit_timer_v1" => |s, v| {
                if v > MetaWpCommitTimerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpCommitTimerV1, v));
                }
                Ok(MetaWpCommitTimerV1::new(s, v))
            },
            "wp_commit_timing_manager_v1" => |s, v| {
                if v > MetaWpCommitTimingManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpCommitTimingManagerV1, v));
                }
                Ok(MetaWpCommitTimingManagerV1::new(s, v))
            },
            "wp_content_type_manager_v1" => |s, v| {
                if v > MetaWpContentTypeManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpContentTypeManagerV1, v));
                }
                Ok(MetaWpContentTypeManagerV1::new(s, v))
            },
            "wp_content_type_v1" => |s, v| {
                if v > MetaWpContentTypeV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpContentTypeV1, v));
                }
                Ok(MetaWpContentTypeV1::new(s, v))
            },
            "wp_cursor_shape_device_v1" => |s, v| {
                if v > MetaWpCursorShapeDeviceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpCursorShapeDeviceV1, v));
                }
                Ok(MetaWpCursorShapeDeviceV1::new(s, v))
            },
            "wp_cursor_shape_manager_v1" => |s, v| {
                if v > MetaWpCursorShapeManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpCursorShapeManagerV1, v));
                }
                Ok(MetaWpCursorShapeManagerV1::new(s, v))
            },
            "wp_drm_lease_connector_v1" => |s, v| {
                if v > MetaWpDrmLeaseConnectorV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpDrmLeaseConnectorV1, v));
                }
                Ok(MetaWpDrmLeaseConnectorV1::new(s, v))
            },
            "wp_drm_lease_device_v1" => |s, v| {
                if v > MetaWpDrmLeaseDeviceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpDrmLeaseDeviceV1, v));
                }
                Ok(MetaWpDrmLeaseDeviceV1::new(s, v))
            },
            "wp_drm_lease_request_v1" => |s, v| {
                if v > MetaWpDrmLeaseRequestV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpDrmLeaseRequestV1, v));
                }
                Ok(MetaWpDrmLeaseRequestV1::new(s, v))
            },
            "wp_drm_lease_v1" => |s, v| {
                if v > MetaWpDrmLeaseV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpDrmLeaseV1, v));
                }
                Ok(MetaWpDrmLeaseV1::new(s, v))
            },
            "ext_background_effect_manager_v1" => |s, v| {
                if v > MetaExtBackgroundEffectManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtBackgroundEffectManagerV1, v));
                }
                Ok(MetaExtBackgroundEffectManagerV1::new(s, v))
            },
            "ext_background_effect_surface_v1" => |s, v| {
                if v > MetaExtBackgroundEffectSurfaceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtBackgroundEffectSurfaceV1, v));
                }
                Ok(MetaExtBackgroundEffectSurfaceV1::new(s, v))
            },
            "ext_data_control_device_v1" => |s, v| {
                if v > MetaExtDataControlDeviceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtDataControlDeviceV1, v));
                }
                Ok(MetaExtDataControlDeviceV1::new(s, v))
            },
            "ext_data_control_manager_v1" => |s, v| {
                if v > MetaExtDataControlManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtDataControlManagerV1, v));
                }
                Ok(MetaExtDataControlManagerV1::new(s, v))
            },
            "ext_data_control_offer_v1" => |s, v| {
                if v > MetaExtDataControlOfferV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtDataControlOfferV1, v));
                }
                Ok(MetaExtDataControlOfferV1::new(s, v))
            },
            "ext_data_control_source_v1" => |s, v| {
                if v > MetaExtDataControlSourceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtDataControlSourceV1, v));
                }
                Ok(MetaExtDataControlSourceV1::new(s, v))
            },
            "ext_foreign_toplevel_handle_v1" => |s, v| {
                if v > MetaExtForeignToplevelHandleV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtForeignToplevelHandleV1, v));
                }
                Ok(MetaExtForeignToplevelHandleV1::new(s, v))
            },
            "ext_foreign_toplevel_list_v1" => |s, v| {
                if v > MetaExtForeignToplevelListV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtForeignToplevelListV1, v));
                }
                Ok(MetaExtForeignToplevelListV1::new(s, v))
            },
            "ext_idle_notification_v1" => |s, v| {
                if v > MetaExtIdleNotificationV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtIdleNotificationV1, v));
                }
                Ok(MetaExtIdleNotificationV1::new(s, v))
            },
            "ext_idle_notifier_v1" => |s, v| {
                if v > MetaExtIdleNotifierV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtIdleNotifierV1, v));
                }
                Ok(MetaExtIdleNotifierV1::new(s, v))
            },
            "ext_foreign_toplevel_image_capture_source_manager_v1" => |s, v| {
                if v > MetaExtForeignToplevelImageCaptureSourceManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtForeignToplevelImageCaptureSourceManagerV1, v));
                }
                Ok(MetaExtForeignToplevelImageCaptureSourceManagerV1::new(s, v))
            },
            "ext_image_capture_source_v1" => |s, v| {
                if v > MetaExtImageCaptureSourceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtImageCaptureSourceV1, v));
                }
                Ok(MetaExtImageCaptureSourceV1::new(s, v))
            },
            "ext_output_image_capture_source_manager_v1" => |s, v| {
                if v > MetaExtOutputImageCaptureSourceManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtOutputImageCaptureSourceManagerV1, v));
                }
                Ok(MetaExtOutputImageCaptureSourceManagerV1::new(s, v))
            },
            "ext_image_copy_capture_cursor_session_v1" => |s, v| {
                if v > MetaExtImageCopyCaptureCursorSessionV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtImageCopyCaptureCursorSessionV1, v));
                }
                Ok(MetaExtImageCopyCaptureCursorSessionV1::new(s, v))
            },
            "ext_image_copy_capture_frame_v1" => |s, v| {
                if v > MetaExtImageCopyCaptureFrameV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtImageCopyCaptureFrameV1, v));
                }
                Ok(MetaExtImageCopyCaptureFrameV1::new(s, v))
            },
            "ext_image_copy_capture_manager_v1" => |s, v| {
                if v > MetaExtImageCopyCaptureManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtImageCopyCaptureManagerV1, v));
                }
                Ok(MetaExtImageCopyCaptureManagerV1::new(s, v))
            },
            "ext_image_copy_capture_session_v1" => |s, v| {
                if v > MetaExtImageCopyCaptureSessionV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtImageCopyCaptureSessionV1, v));
                }
                Ok(MetaExtImageCopyCaptureSessionV1::new(s, v))
            },
            "ext_session_lock_manager_v1" => |s, v| {
                if v > MetaExtSessionLockManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtSessionLockManagerV1, v));
                }
                Ok(MetaExtSessionLockManagerV1::new(s, v))
            },
            "ext_session_lock_surface_v1" => |s, v| {
                if v > MetaExtSessionLockSurfaceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtSessionLockSurfaceV1, v));
                }
                Ok(MetaExtSessionLockSurfaceV1::new(s, v))
            },
            "ext_session_lock_v1" => |s, v| {
                if v > MetaExtSessionLockV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtSessionLockV1, v));
                }
                Ok(MetaExtSessionLockV1::new(s, v))
            },
            "ext_transient_seat_manager_v1" => |s, v| {
                if v > MetaExtTransientSeatManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtTransientSeatManagerV1, v));
                }
                Ok(MetaExtTransientSeatManagerV1::new(s, v))
            },
            "ext_transient_seat_v1" => |s, v| {
                if v > MetaExtTransientSeatV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtTransientSeatV1, v));
                }
                Ok(MetaExtTransientSeatV1::new(s, v))
            },
            "ext_workspace_group_handle_v1" => |s, v| {
                if v > MetaExtWorkspaceGroupHandleV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtWorkspaceGroupHandleV1, v));
                }
                Ok(MetaExtWorkspaceGroupHandleV1::new(s, v))
            },
            "ext_workspace_handle_v1" => |s, v| {
                if v > MetaExtWorkspaceHandleV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtWorkspaceHandleV1, v));
                }
                Ok(MetaExtWorkspaceHandleV1::new(s, v))
            },
            "ext_workspace_manager_v1" => |s, v| {
                if v > MetaExtWorkspaceManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ExtWorkspaceManagerV1, v));
                }
                Ok(MetaExtWorkspaceManagerV1::new(s, v))
            },
            "wp_fifo_manager_v1" => |s, v| {
                if v > MetaWpFifoManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpFifoManagerV1, v));
                }
                Ok(MetaWpFifoManagerV1::new(s, v))
            },
            "wp_fifo_v1" => |s, v| {
                if v > MetaWpFifoV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpFifoV1, v));
                }
                Ok(MetaWpFifoV1::new(s, v))
            },
            "wp_fractional_scale_manager_v1" => |s, v| {
                if v > MetaWpFractionalScaleManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpFractionalScaleManagerV1, v));
                }
                Ok(MetaWpFractionalScaleManagerV1::new(s, v))
            },
            "wp_fractional_scale_v1" => |s, v| {
                if v > MetaWpFractionalScaleV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpFractionalScaleV1, v));
                }
                Ok(MetaWpFractionalScaleV1::new(s, v))
            },
            "zwp_fullscreen_shell_mode_feedback_v1" => |s, v| {
                if v > MetaZwpFullscreenShellModeFeedbackV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpFullscreenShellModeFeedbackV1, v));
                }
                Ok(MetaZwpFullscreenShellModeFeedbackV1::new(s, v))
            },
            "zwp_fullscreen_shell_v1" => |s, v| {
                if v > MetaZwpFullscreenShellV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpFullscreenShellV1, v));
                }
                Ok(MetaZwpFullscreenShellV1::new(s, v))
            },
            "zwp_idle_inhibit_manager_v1" => |s, v| {
                if v > MetaZwpIdleInhibitManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpIdleInhibitManagerV1, v));
                }
                Ok(MetaZwpIdleInhibitManagerV1::new(s, v))
            },
            "zwp_idle_inhibitor_v1" => |s, v| {
                if v > MetaZwpIdleInhibitorV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpIdleInhibitorV1, v));
                }
                Ok(MetaZwpIdleInhibitorV1::new(s, v))
            },
            "zwp_input_method_context_v1" => |s, v| {
                if v > MetaZwpInputMethodContextV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpInputMethodContextV1, v));
                }
                Ok(MetaZwpInputMethodContextV1::new(s, v))
            },
            "zwp_input_method_v1" => |s, v| {
                if v > MetaZwpInputMethodV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpInputMethodV1, v));
                }
                Ok(MetaZwpInputMethodV1::new(s, v))
            },
            "zwp_input_panel_surface_v1" => |s, v| {
                if v > MetaZwpInputPanelSurfaceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpInputPanelSurfaceV1, v));
                }
                Ok(MetaZwpInputPanelSurfaceV1::new(s, v))
            },
            "zwp_input_panel_v1" => |s, v| {
                if v > MetaZwpInputPanelV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpInputPanelV1, v));
                }
                Ok(MetaZwpInputPanelV1::new(s, v))
            },
            "zwp_input_timestamps_manager_v1" => |s, v| {
                if v > MetaZwpInputTimestampsManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpInputTimestampsManagerV1, v));
                }
                Ok(MetaZwpInputTimestampsManagerV1::new(s, v))
            },
            "zwp_input_timestamps_v1" => |s, v| {
                if v > MetaZwpInputTimestampsV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpInputTimestampsV1, v));
                }
                Ok(MetaZwpInputTimestampsV1::new(s, v))
            },
            "zwp_keyboard_shortcuts_inhibit_manager_v1" => |s, v| {
                if v > MetaZwpKeyboardShortcutsInhibitManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpKeyboardShortcutsInhibitManagerV1, v));
                }
                Ok(MetaZwpKeyboardShortcutsInhibitManagerV1::new(s, v))
            },
            "zwp_keyboard_shortcuts_inhibitor_v1" => |s, v| {
                if v > MetaZwpKeyboardShortcutsInhibitorV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpKeyboardShortcutsInhibitorV1, v));
                }
                Ok(MetaZwpKeyboardShortcutsInhibitorV1::new(s, v))
            },
            "zwp_linux_buffer_params_v1" => |s, v| {
                if v > MetaZwpLinuxBufferParamsV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpLinuxBufferParamsV1, v));
                }
                Ok(MetaZwpLinuxBufferParamsV1::new(s, v))
            },
            "zwp_linux_dmabuf_feedback_v1" => |s, v| {
                if v > MetaZwpLinuxDmabufFeedbackV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpLinuxDmabufFeedbackV1, v));
                }
                Ok(MetaZwpLinuxDmabufFeedbackV1::new(s, v))
            },
            "zwp_linux_dmabuf_v1" => |s, v| {
                if v > MetaZwpLinuxDmabufV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpLinuxDmabufV1, v));
                }
                Ok(MetaZwpLinuxDmabufV1::new(s, v))
            },
            "wp_linux_drm_syncobj_manager_v1" => |s, v| {
                if v > MetaWpLinuxDrmSyncobjManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpLinuxDrmSyncobjManagerV1, v));
                }
                Ok(MetaWpLinuxDrmSyncobjManagerV1::new(s, v))
            },
            "wp_linux_drm_syncobj_surface_v1" => |s, v| {
                if v > MetaWpLinuxDrmSyncobjSurfaceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpLinuxDrmSyncobjSurfaceV1, v));
                }
                Ok(MetaWpLinuxDrmSyncobjSurfaceV1::new(s, v))
            },
            "wp_linux_drm_syncobj_timeline_v1" => |s, v| {
                if v > MetaWpLinuxDrmSyncobjTimelineV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpLinuxDrmSyncobjTimelineV1, v));
                }
                Ok(MetaWpLinuxDrmSyncobjTimelineV1::new(s, v))
            },
            "zwp_linux_buffer_release_v1" => |s, v| {
                if v > MetaZwpLinuxBufferReleaseV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpLinuxBufferReleaseV1, v));
                }
                Ok(MetaZwpLinuxBufferReleaseV1::new(s, v))
            },
            "zwp_linux_explicit_synchronization_v1" => |s, v| {
                if v > MetaZwpLinuxExplicitSynchronizationV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpLinuxExplicitSynchronizationV1, v));
                }
                Ok(MetaZwpLinuxExplicitSynchronizationV1::new(s, v))
            },
            "zwp_linux_surface_synchronization_v1" => |s, v| {
                if v > MetaZwpLinuxSurfaceSynchronizationV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpLinuxSurfaceSynchronizationV1, v));
                }
                Ok(MetaZwpLinuxSurfaceSynchronizationV1::new(s, v))
            },
            "zwp_confined_pointer_v1" => |s, v| {
                if v > MetaZwpConfinedPointerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpConfinedPointerV1, v));
                }
                Ok(MetaZwpConfinedPointerV1::new(s, v))
            },
            "zwp_locked_pointer_v1" => |s, v| {
                if v > MetaZwpLockedPointerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpLockedPointerV1, v));
                }
                Ok(MetaZwpLockedPointerV1::new(s, v))
            },
            "zwp_pointer_constraints_v1" => |s, v| {
                if v > MetaZwpPointerConstraintsV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpPointerConstraintsV1, v));
                }
                Ok(MetaZwpPointerConstraintsV1::new(s, v))
            },
            "zwp_pointer_gesture_hold_v1" => |s, v| {
                if v > MetaZwpPointerGestureHoldV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpPointerGestureHoldV1, v));
                }
                Ok(MetaZwpPointerGestureHoldV1::new(s, v))
            },
            "zwp_pointer_gesture_pinch_v1" => |s, v| {
                if v > MetaZwpPointerGesturePinchV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpPointerGesturePinchV1, v));
                }
                Ok(MetaZwpPointerGesturePinchV1::new(s, v))
            },
            "zwp_pointer_gesture_swipe_v1" => |s, v| {
                if v > MetaZwpPointerGestureSwipeV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpPointerGestureSwipeV1, v));
                }
                Ok(MetaZwpPointerGestureSwipeV1::new(s, v))
            },
            "zwp_pointer_gestures_v1" => |s, v| {
                if v > MetaZwpPointerGesturesV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpPointerGesturesV1, v));
                }
                Ok(MetaZwpPointerGesturesV1::new(s, v))
            },
            "wp_pointer_warp_v1" => |s, v| {
                if v > MetaWpPointerWarpV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpPointerWarpV1, v));
                }
                Ok(MetaWpPointerWarpV1::new(s, v))
            },
            "wp_presentation" => |s, v| {
                if v > MetaWpPresentation::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpPresentation, v));
                }
                Ok(MetaWpPresentation::new(s, v))
            },
            "wp_presentation_feedback" => |s, v| {
                if v > MetaWpPresentationFeedback::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpPresentationFeedback, v));
                }
                Ok(MetaWpPresentationFeedback::new(s, v))
            },
            "zwp_primary_selection_device_manager_v1" => |s, v| {
                if v > MetaZwpPrimarySelectionDeviceManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpPrimarySelectionDeviceManagerV1, v));
                }
                Ok(MetaZwpPrimarySelectionDeviceManagerV1::new(s, v))
            },
            "zwp_primary_selection_device_v1" => |s, v| {
                if v > MetaZwpPrimarySelectionDeviceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpPrimarySelectionDeviceV1, v));
                }
                Ok(MetaZwpPrimarySelectionDeviceV1::new(s, v))
            },
            "zwp_primary_selection_offer_v1" => |s, v| {
                if v > MetaZwpPrimarySelectionOfferV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpPrimarySelectionOfferV1, v));
                }
                Ok(MetaZwpPrimarySelectionOfferV1::new(s, v))
            },
            "zwp_primary_selection_source_v1" => |s, v| {
                if v > MetaZwpPrimarySelectionSourceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpPrimarySelectionSourceV1, v));
                }
                Ok(MetaZwpPrimarySelectionSourceV1::new(s, v))
            },
            "zwp_relative_pointer_manager_v1" => |s, v| {
                if v > MetaZwpRelativePointerManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpRelativePointerManagerV1, v));
                }
                Ok(MetaZwpRelativePointerManagerV1::new(s, v))
            },
            "zwp_relative_pointer_v1" => |s, v| {
                if v > MetaZwpRelativePointerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpRelativePointerV1, v));
                }
                Ok(MetaZwpRelativePointerV1::new(s, v))
            },
            "wp_security_context_manager_v1" => |s, v| {
                if v > MetaWpSecurityContextManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpSecurityContextManagerV1, v));
                }
                Ok(MetaWpSecurityContextManagerV1::new(s, v))
            },
            "wp_security_context_v1" => |s, v| {
                if v > MetaWpSecurityContextV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpSecurityContextV1, v));
                }
                Ok(MetaWpSecurityContextV1::new(s, v))
            },
            "wp_single_pixel_buffer_manager_v1" => |s, v| {
                if v > MetaWpSinglePixelBufferManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpSinglePixelBufferManagerV1, v));
                }
                Ok(MetaWpSinglePixelBufferManagerV1::new(s, v))
            },
            "zwp_tablet_manager_v2" => |s, v| {
                if v > MetaZwpTabletManagerV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTabletManagerV2, v));
                }
                Ok(MetaZwpTabletManagerV2::new(s, v))
            },
            "zwp_tablet_pad_dial_v2" => |s, v| {
                if v > MetaZwpTabletPadDialV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTabletPadDialV2, v));
                }
                Ok(MetaZwpTabletPadDialV2::new(s, v))
            },
            "zwp_tablet_pad_group_v2" => |s, v| {
                if v > MetaZwpTabletPadGroupV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTabletPadGroupV2, v));
                }
                Ok(MetaZwpTabletPadGroupV2::new(s, v))
            },
            "zwp_tablet_pad_ring_v2" => |s, v| {
                if v > MetaZwpTabletPadRingV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTabletPadRingV2, v));
                }
                Ok(MetaZwpTabletPadRingV2::new(s, v))
            },
            "zwp_tablet_pad_strip_v2" => |s, v| {
                if v > MetaZwpTabletPadStripV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTabletPadStripV2, v));
                }
                Ok(MetaZwpTabletPadStripV2::new(s, v))
            },
            "zwp_tablet_pad_v2" => |s, v| {
                if v > MetaZwpTabletPadV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTabletPadV2, v));
                }
                Ok(MetaZwpTabletPadV2::new(s, v))
            },
            "zwp_tablet_seat_v2" => |s, v| {
                if v > MetaZwpTabletSeatV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTabletSeatV2, v));
                }
                Ok(MetaZwpTabletSeatV2::new(s, v))
            },
            "zwp_tablet_tool_v2" => |s, v| {
                if v > MetaZwpTabletToolV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTabletToolV2, v));
                }
                Ok(MetaZwpTabletToolV2::new(s, v))
            },
            "zwp_tablet_v2" => |s, v| {
                if v > MetaZwpTabletV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTabletV2, v));
                }
                Ok(MetaZwpTabletV2::new(s, v))
            },
            "wp_tearing_control_manager_v1" => |s, v| {
                if v > MetaWpTearingControlManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpTearingControlManagerV1, v));
                }
                Ok(MetaWpTearingControlManagerV1::new(s, v))
            },
            "wp_tearing_control_v1" => |s, v| {
                if v > MetaWpTearingControlV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpTearingControlV1, v));
                }
                Ok(MetaWpTearingControlV1::new(s, v))
            },
            "zwp_text_input_manager_v1" => |s, v| {
                if v > MetaZwpTextInputManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTextInputManagerV1, v));
                }
                Ok(MetaZwpTextInputManagerV1::new(s, v))
            },
            "zwp_text_input_v1" => |s, v| {
                if v > MetaZwpTextInputV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTextInputV1, v));
                }
                Ok(MetaZwpTextInputV1::new(s, v))
            },
            "zwp_text_input_manager_v3" => |s, v| {
                if v > MetaZwpTextInputManagerV3::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTextInputManagerV3, v));
                }
                Ok(MetaZwpTextInputManagerV3::new(s, v))
            },
            "zwp_text_input_v3" => |s, v| {
                if v > MetaZwpTextInputV3::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpTextInputV3, v));
                }
                Ok(MetaZwpTextInputV3::new(s, v))
            },
            "wp_viewport" => |s, v| {
                if v > MetaWpViewport::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpViewport, v));
                }
                Ok(MetaWpViewport::new(s, v))
            },
            "wp_viewporter" => |s, v| {
                if v > MetaWpViewporter::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WpViewporter, v));
                }
                Ok(MetaWpViewporter::new(s, v))
            },
            "wl_buffer" => |s, v| {
                if v > MetaWlBuffer::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlBuffer, v));
                }
                Ok(MetaWlBuffer::new(s, v))
            },
            "wl_callback" => |s, v| {
                if v > MetaWlCallback::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlCallback, v));
                }
                Ok(MetaWlCallback::new(s, v))
            },
            "wl_compositor" => |s, v| {
                if v > MetaWlCompositor::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlCompositor, v));
                }
                Ok(MetaWlCompositor::new(s, v))
            },
            "wl_data_device" => |s, v| {
                if v > MetaWlDataDevice::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlDataDevice, v));
                }
                Ok(MetaWlDataDevice::new(s, v))
            },
            "wl_data_device_manager" => |s, v| {
                if v > MetaWlDataDeviceManager::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlDataDeviceManager, v));
                }
                Ok(MetaWlDataDeviceManager::new(s, v))
            },
            "wl_data_offer" => |s, v| {
                if v > MetaWlDataOffer::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlDataOffer, v));
                }
                Ok(MetaWlDataOffer::new(s, v))
            },
            "wl_data_source" => |s, v| {
                if v > MetaWlDataSource::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlDataSource, v));
                }
                Ok(MetaWlDataSource::new(s, v))
            },
            "wl_display" => |s, v| {
                if v > MetaWlDisplay::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlDisplay, v));
                }
                Ok(MetaWlDisplay::new(s, v))
            },
            "wl_fixes" => |s, v| {
                if v > MetaWlFixes::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlFixes, v));
                }
                Ok(MetaWlFixes::new(s, v))
            },
            "wl_keyboard" => |s, v| {
                if v > MetaWlKeyboard::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlKeyboard, v));
                }
                Ok(MetaWlKeyboard::new(s, v))
            },
            "wl_output" => |s, v| {
                if v > MetaWlOutput::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlOutput, v));
                }
                Ok(MetaWlOutput::new(s, v))
            },
            "wl_pointer" => |s, v| {
                if v > MetaWlPointer::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlPointer, v));
                }
                Ok(MetaWlPointer::new(s, v))
            },
            "wl_region" => |s, v| {
                if v > MetaWlRegion::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlRegion, v));
                }
                Ok(MetaWlRegion::new(s, v))
            },
            "wl_registry" => |s, v| {
                if v > MetaWlRegistry::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlRegistry, v));
                }
                Ok(MetaWlRegistry::new(s, v))
            },
            "wl_seat" => |s, v| {
                if v > MetaWlSeat::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlSeat, v));
                }
                Ok(MetaWlSeat::new(s, v))
            },
            "wl_shell" => |s, v| {
                if v > MetaWlShell::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlShell, v));
                }
                Ok(MetaWlShell::new(s, v))
            },
            "wl_shell_surface" => |s, v| {
                if v > MetaWlShellSurface::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlShellSurface, v));
                }
                Ok(MetaWlShellSurface::new(s, v))
            },
            "wl_shm" => |s, v| {
                if v > MetaWlShm::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlShm, v));
                }
                Ok(MetaWlShm::new(s, v))
            },
            "wl_shm_pool" => |s, v| {
                if v > MetaWlShmPool::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlShmPool, v));
                }
                Ok(MetaWlShmPool::new(s, v))
            },
            "wl_subcompositor" => |s, v| {
                if v > MetaWlSubcompositor::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlSubcompositor, v));
                }
                Ok(MetaWlSubcompositor::new(s, v))
            },
            "wl_subsurface" => |s, v| {
                if v > MetaWlSubsurface::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlSubsurface, v));
                }
                Ok(MetaWlSubsurface::new(s, v))
            },
            "wl_surface" => |s, v| {
                if v > MetaWlSurface::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlSurface, v));
                }
                Ok(MetaWlSurface::new(s, v))
            },
            "wl_touch" => |s, v| {
                if v > MetaWlTouch::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::WlTouch, v));
                }
                Ok(MetaWlTouch::new(s, v))
            },
            "zwlr_data_control_device_v1" => |s, v| {
                if v > MetaZwlrDataControlDeviceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrDataControlDeviceV1, v));
                }
                Ok(MetaZwlrDataControlDeviceV1::new(s, v))
            },
            "zwlr_data_control_manager_v1" => |s, v| {
                if v > MetaZwlrDataControlManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrDataControlManagerV1, v));
                }
                Ok(MetaZwlrDataControlManagerV1::new(s, v))
            },
            "zwlr_data_control_offer_v1" => |s, v| {
                if v > MetaZwlrDataControlOfferV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrDataControlOfferV1, v));
                }
                Ok(MetaZwlrDataControlOfferV1::new(s, v))
            },
            "zwlr_data_control_source_v1" => |s, v| {
                if v > MetaZwlrDataControlSourceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrDataControlSourceV1, v));
                }
                Ok(MetaZwlrDataControlSourceV1::new(s, v))
            },
            "zwlr_export_dmabuf_frame_v1" => |s, v| {
                if v > MetaZwlrExportDmabufFrameV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrExportDmabufFrameV1, v));
                }
                Ok(MetaZwlrExportDmabufFrameV1::new(s, v))
            },
            "zwlr_export_dmabuf_manager_v1" => |s, v| {
                if v > MetaZwlrExportDmabufManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrExportDmabufManagerV1, v));
                }
                Ok(MetaZwlrExportDmabufManagerV1::new(s, v))
            },
            "zwlr_foreign_toplevel_handle_v1" => |s, v| {
                if v > MetaZwlrForeignToplevelHandleV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrForeignToplevelHandleV1, v));
                }
                Ok(MetaZwlrForeignToplevelHandleV1::new(s, v))
            },
            "zwlr_foreign_toplevel_manager_v1" => |s, v| {
                if v > MetaZwlrForeignToplevelManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrForeignToplevelManagerV1, v));
                }
                Ok(MetaZwlrForeignToplevelManagerV1::new(s, v))
            },
            "zwlr_gamma_control_manager_v1" => |s, v| {
                if v > MetaZwlrGammaControlManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrGammaControlManagerV1, v));
                }
                Ok(MetaZwlrGammaControlManagerV1::new(s, v))
            },
            "zwlr_gamma_control_v1" => |s, v| {
                if v > MetaZwlrGammaControlV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrGammaControlV1, v));
                }
                Ok(MetaZwlrGammaControlV1::new(s, v))
            },
            "zwlr_input_inhibit_manager_v1" => |s, v| {
                if v > MetaZwlrInputInhibitManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrInputInhibitManagerV1, v));
                }
                Ok(MetaZwlrInputInhibitManagerV1::new(s, v))
            },
            "zwlr_input_inhibitor_v1" => |s, v| {
                if v > MetaZwlrInputInhibitorV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrInputInhibitorV1, v));
                }
                Ok(MetaZwlrInputInhibitorV1::new(s, v))
            },
            "zwlr_layer_shell_v1" => |s, v| {
                if v > MetaZwlrLayerShellV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrLayerShellV1, v));
                }
                Ok(MetaZwlrLayerShellV1::new(s, v))
            },
            "zwlr_layer_surface_v1" => |s, v| {
                if v > MetaZwlrLayerSurfaceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrLayerSurfaceV1, v));
                }
                Ok(MetaZwlrLayerSurfaceV1::new(s, v))
            },
            "zwlr_output_configuration_head_v1" => |s, v| {
                if v > MetaZwlrOutputConfigurationHeadV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrOutputConfigurationHeadV1, v));
                }
                Ok(MetaZwlrOutputConfigurationHeadV1::new(s, v))
            },
            "zwlr_output_configuration_v1" => |s, v| {
                if v > MetaZwlrOutputConfigurationV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrOutputConfigurationV1, v));
                }
                Ok(MetaZwlrOutputConfigurationV1::new(s, v))
            },
            "zwlr_output_head_v1" => |s, v| {
                if v > MetaZwlrOutputHeadV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrOutputHeadV1, v));
                }
                Ok(MetaZwlrOutputHeadV1::new(s, v))
            },
            "zwlr_output_manager_v1" => |s, v| {
                if v > MetaZwlrOutputManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrOutputManagerV1, v));
                }
                Ok(MetaZwlrOutputManagerV1::new(s, v))
            },
            "zwlr_output_mode_v1" => |s, v| {
                if v > MetaZwlrOutputModeV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrOutputModeV1, v));
                }
                Ok(MetaZwlrOutputModeV1::new(s, v))
            },
            "zwlr_output_power_manager_v1" => |s, v| {
                if v > MetaZwlrOutputPowerManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrOutputPowerManagerV1, v));
                }
                Ok(MetaZwlrOutputPowerManagerV1::new(s, v))
            },
            "zwlr_output_power_v1" => |s, v| {
                if v > MetaZwlrOutputPowerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrOutputPowerV1, v));
                }
                Ok(MetaZwlrOutputPowerV1::new(s, v))
            },
            "zwlr_screencopy_frame_v1" => |s, v| {
                if v > MetaZwlrScreencopyFrameV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrScreencopyFrameV1, v));
                }
                Ok(MetaZwlrScreencopyFrameV1::new(s, v))
            },
            "zwlr_screencopy_manager_v1" => |s, v| {
                if v > MetaZwlrScreencopyManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrScreencopyManagerV1, v));
                }
                Ok(MetaZwlrScreencopyManagerV1::new(s, v))
            },
            "zwlr_virtual_pointer_manager_v1" => |s, v| {
                if v > MetaZwlrVirtualPointerManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrVirtualPointerManagerV1, v));
                }
                Ok(MetaZwlrVirtualPointerManagerV1::new(s, v))
            },
            "zwlr_virtual_pointer_v1" => |s, v| {
                if v > MetaZwlrVirtualPointerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwlrVirtualPointerV1, v));
                }
                Ok(MetaZwlrVirtualPointerV1::new(s, v))
            },
            "xdg_activation_token_v1" => |s, v| {
                if v > MetaXdgActivationTokenV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgActivationTokenV1, v));
                }
                Ok(MetaXdgActivationTokenV1::new(s, v))
            },
            "xdg_activation_v1" => |s, v| {
                if v > MetaXdgActivationV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgActivationV1, v));
                }
                Ok(MetaXdgActivationV1::new(s, v))
            },
            "zxdg_decoration_manager_v1" => |s, v| {
                if v > MetaZxdgDecorationManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZxdgDecorationManagerV1, v));
                }
                Ok(MetaZxdgDecorationManagerV1::new(s, v))
            },
            "zxdg_toplevel_decoration_v1" => |s, v| {
                if v > MetaZxdgToplevelDecorationV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZxdgToplevelDecorationV1, v));
                }
                Ok(MetaZxdgToplevelDecorationV1::new(s, v))
            },
            "xdg_dialog_v1" => |s, v| {
                if v > MetaXdgDialogV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgDialogV1, v));
                }
                Ok(MetaXdgDialogV1::new(s, v))
            },
            "xdg_wm_dialog_v1" => |s, v| {
                if v > MetaXdgWmDialogV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgWmDialogV1, v));
                }
                Ok(MetaXdgWmDialogV1::new(s, v))
            },
            "zxdg_exported_v2" => |s, v| {
                if v > MetaZxdgExportedV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZxdgExportedV2, v));
                }
                Ok(MetaZxdgExportedV2::new(s, v))
            },
            "zxdg_exporter_v2" => |s, v| {
                if v > MetaZxdgExporterV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZxdgExporterV2, v));
                }
                Ok(MetaZxdgExporterV2::new(s, v))
            },
            "zxdg_imported_v2" => |s, v| {
                if v > MetaZxdgImportedV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZxdgImportedV2, v));
                }
                Ok(MetaZxdgImportedV2::new(s, v))
            },
            "zxdg_importer_v2" => |s, v| {
                if v > MetaZxdgImporterV2::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZxdgImporterV2, v));
                }
                Ok(MetaZxdgImporterV2::new(s, v))
            },
            "zxdg_output_manager_v1" => |s, v| {
                if v > MetaZxdgOutputManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZxdgOutputManagerV1, v));
                }
                Ok(MetaZxdgOutputManagerV1::new(s, v))
            },
            "zxdg_output_v1" => |s, v| {
                if v > MetaZxdgOutputV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZxdgOutputV1, v));
                }
                Ok(MetaZxdgOutputV1::new(s, v))
            },
            "xdg_popup" => |s, v| {
                if v > MetaXdgPopup::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgPopup, v));
                }
                Ok(MetaXdgPopup::new(s, v))
            },
            "xdg_positioner" => |s, v| {
                if v > MetaXdgPositioner::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgPositioner, v));
                }
                Ok(MetaXdgPositioner::new(s, v))
            },
            "xdg_surface" => |s, v| {
                if v > MetaXdgSurface::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgSurface, v));
                }
                Ok(MetaXdgSurface::new(s, v))
            },
            "xdg_toplevel" => |s, v| {
                if v > MetaXdgToplevel::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgToplevel, v));
                }
                Ok(MetaXdgToplevel::new(s, v))
            },
            "xdg_wm_base" => |s, v| {
                if v > MetaXdgWmBase::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgWmBase, v));
                }
                Ok(MetaXdgWmBase::new(s, v))
            },
            "xdg_system_bell_v1" => |s, v| {
                if v > MetaXdgSystemBellV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgSystemBellV1, v));
                }
                Ok(MetaXdgSystemBellV1::new(s, v))
            },
            "xdg_toplevel_drag_manager_v1" => |s, v| {
                if v > MetaXdgToplevelDragManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgToplevelDragManagerV1, v));
                }
                Ok(MetaXdgToplevelDragManagerV1::new(s, v))
            },
            "xdg_toplevel_drag_v1" => |s, v| {
                if v > MetaXdgToplevelDragV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgToplevelDragV1, v));
                }
                Ok(MetaXdgToplevelDragV1::new(s, v))
            },
            "xdg_toplevel_icon_manager_v1" => |s, v| {
                if v > MetaXdgToplevelIconManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgToplevelIconManagerV1, v));
                }
                Ok(MetaXdgToplevelIconManagerV1::new(s, v))
            },
            "xdg_toplevel_icon_v1" => |s, v| {
                if v > MetaXdgToplevelIconV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgToplevelIconV1, v));
                }
                Ok(MetaXdgToplevelIconV1::new(s, v))
            },
            "xdg_toplevel_tag_manager_v1" => |s, v| {
                if v > MetaXdgToplevelTagManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XdgToplevelTagManagerV1, v));
                }
                Ok(MetaXdgToplevelTagManagerV1::new(s, v))
            },
            "zwp_xwayland_keyboard_grab_manager_v1" => |s, v| {
                if v > MetaZwpXwaylandKeyboardGrabManagerV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpXwaylandKeyboardGrabManagerV1, v));
                }
                Ok(MetaZwpXwaylandKeyboardGrabManagerV1::new(s, v))
            },
            "zwp_xwayland_keyboard_grab_v1" => |s, v| {
                if v > MetaZwpXwaylandKeyboardGrabV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::ZwpXwaylandKeyboardGrabV1, v));
                }
                Ok(MetaZwpXwaylandKeyboardGrabV1::new(s, v))
            },
            "xwayland_shell_v1" => |s, v| {
                if v > MetaXwaylandShellV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XwaylandShellV1, v));
                }
                Ok(MetaXwaylandShellV1::new(s, v))
            },
            "xwayland_surface_v1" => |s, v| {
                if v > MetaXwaylandSurfaceV1::XML_VERSION {
                    return Err(ObjectError::MaxVersion(ProxyInterface::XwaylandSurfaceV1, v));
                }
                Ok(MetaXwaylandSurfaceV1::new(s, v))
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
