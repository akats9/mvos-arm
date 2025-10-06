//! ASCII art of Jesus to bless the system upon boot

use core::ffi::c_char;

use crate::serial_println;

pub fn print_bootscreen() {
    serial_println!("⠀⠀⠀⠀⢀⡠⣾⣳⡀⠀⠀⠀⠀⠀
⠀⠀⡀⠀⠚⢿⣿⣿⡿⠙⠀⠀⠀⠀
⠀⣘⣿⣇⡀⢘⣿⣿⠀⢀⣠⣶⡀⠀
⠺⣿⣷⣝⣾⣿⣿⣿⣿⣿⣹⣷⣿⠆
⠀⠘⠟⠁⠀⠀⣿⣟⠀⠀⠙⠿⠁⠀
⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⢠⣿⣿⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⢸⣿⡿⡄⠀⠀⠀⠀⠀
⠀⠀⠀⠠⣖⣿⣿⣻⡷⡄⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠈⢻⡟⠁⠀⠀⠀⠀⠀")
}

unsafe extern "C" {
    fn display_bootscreen(fb_addr: *mut c_char);
}

pub fn bootscreen_visual(fb_addr: *mut c_char) {
    unsafe { display_bootscreen(fb_addr); }
}