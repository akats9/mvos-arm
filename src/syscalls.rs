use crate::GPU_DEVICE;

pub const SYS_DRAW_PIXEL: u64 = 10;

pub fn sys_draw_pixel(x: u32, y: u32, r: u8, g: u8, b: u8) {
    unsafe { (*GPU_DEVICE.unwrap()).set_pixel(x, y, r, g, b); }
}