//! ASCII art of Jesus to bless the system upon boot

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