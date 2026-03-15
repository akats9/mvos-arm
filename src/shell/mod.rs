use alloc::string::String;

use crate::{GPU_DEVICE, SCALE, SCREENHEIGHT, SCREENWIDTH, TEXT_DEFAULT, THEME, console_print, console_println, dbg, drivers::uart::{self, RX_BUFFER, get_key_latest}, mvulkan::console};

/// Prepare the shell environment
pub fn start_shell() {
    unsafe {
        (*GPU_DEVICE.unwrap()).clear(0);
        let theme = THEME;
        console::origin();
        TEXT_DEFAULT = theme.yellow();
        RX_BUFFER = ['\0'; 256];
        console_print!("root@mvos % " ; color: theme.yellow());
        run_shell();
    }
}

fn run_shell() {
    unsafe {
        let theme = THEME;
        loop {
            if get_key_latest() == '\r' {
                let buf = String::from_iter(uart::RX_BUFFER);
                dbg!("shell buffer: {:?}", buf);
                dbg!("iter: {:#?}", buf.split_terminator('\0'));
                let mut cmd: &str = "line";
                for sl in buf.split_terminator('\0') {
                    if sl == "" { continue; }
                    else { cmd = sl; break; }
                }
                dbg!("{:#?}", cmd);      
                cmd = match cmd.strip_suffix('\r') {
                    Some(c) => c,
                    None => "line",
                };
                match cmd {
                    "line" => {
                        //console::newline();
                        console_print!("root@mvos % " ; color: theme.yellow());
                    },
                    "version" => {
                        let v = env!("CARGO_PKG_VERSION");
                        console_println!("MVOS version {}", v; color: theme.yellow());
                        console_print!("root@mvos % "; color: theme.yellow());
                    },
                    _ => {console_print!("root@mvos % "; color: theme.yellow());}
                }
                RX_BUFFER = ['\0'; 256];
            }
        }
    }
}   