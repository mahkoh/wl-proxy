#![allow(non_upper_case_globals, unused)]

use linearize::{StaticCopyMap, Linearize};
use crate::protocols::ObjectInterface;

const wl_buffer: u32 = 1;
const wl_callback: u32 = 1;
const wl_compositor: u32 = 6;
const wl_data_device: u32 = 3;
const wl_data_device_manager: u32 = 3;
const wl_data_offer: u32 = 3;
const wl_data_source: u32 = 3;
const wl_display: u32 = 1;
const wl_fixes: u32 = 1;
const wl_keyboard: u32 = 10;
const wl_output: u32 = 4;
const wl_pointer: u32 = 10;
const wl_region: u32 = 1;
const wl_registry: u32 = 1;
const wl_seat: u32 = 10;
const wl_shell: u32 = 1;
const wl_shell_surface: u32 = 1;
const wl_shm: u32 = 2;
const wl_shm_pool: u32 = 2;
const wl_subcompositor: u32 = 1;
const wl_subsurface: u32 = 1;
const wl_surface: u32 = 6;
const wl_touch: u32 = 10;

#[rustfmt::skip]
pub(in super::super) const BASELINE: &StaticCopyMap<ObjectInterface, u32> = {
    static BASELINE: [u32; ObjectInterface::LENGTH] = {
        let mut baseline = [0; ObjectInterface::LENGTH];
        { baseline[ObjectInterface::WlBuffer.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = wl_buffer; }
        { baseline[ObjectInterface::WlCallback.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = wl_callback; }
        { baseline[ObjectInterface::WlCompositor.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = wl_compositor; }
        { baseline[ObjectInterface::WlDataDevice.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = wl_data_device; }
        { baseline[ObjectInterface::WlDataDeviceManager.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = wl_data_device_manager; }
        { baseline[ObjectInterface::WlDataOffer.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = wl_data_offer; }
        { baseline[ObjectInterface::WlDataSource.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = wl_data_source; }
        { baseline[ObjectInterface::WlDisplay.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = wl_display; }
        { baseline[ObjectInterface::WlFixes.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = wl_fixes; }
        { baseline[ObjectInterface::WlKeyboard.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = wl_keyboard; }
        { baseline[ObjectInterface::WlOutput.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = wl_output; }
        { baseline[ObjectInterface::WlPointer.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = wl_pointer; }
        { baseline[ObjectInterface::WlRegion.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = wl_region; }
        { baseline[ObjectInterface::WlRegistry.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = wl_registry; }
        { baseline[ObjectInterface::WlSeat.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = wl_seat; }
        { baseline[ObjectInterface::WlShell.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = wl_shell; }
        { baseline[ObjectInterface::WlShellSurface.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = wl_shell_surface; }
        { baseline[ObjectInterface::WlShm.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = wl_shm; }
        { baseline[ObjectInterface::WlShmPool.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = wl_shm_pool; }
        { baseline[ObjectInterface::WlSubcompositor.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = wl_subcompositor; }
        { baseline[ObjectInterface::WlSubsurface.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = wl_subsurface; }
        { baseline[ObjectInterface::WlSurface.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = wl_surface; }
        { baseline[ObjectInterface::WlTouch.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = wl_touch; }
        baseline
    };
    StaticCopyMap::from_ref(&BASELINE)
};
