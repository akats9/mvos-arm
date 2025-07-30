//! Helper functions to discover VirtIO devices over MMIO.

use crate::{serial_print, serial_println};

/// Find the MMIO base address of the virtio-gpu-device.
pub fn find_gpu() -> u64 {
    for addr in (0x000000000a003e00..0x000000000b003e00).step_by(0x200) {
        unsafe {
            if (addr as *const u64).read_volatile() == 0x74726976 {
                serial_println!("[   DEBUG   ] addr: {:x}", addr);
                return addr;
            } else {
                serial_println!("[   DEBUG   ] addr: {:x}", addr);
                continue;
            }
        }
    }
    0x0
}