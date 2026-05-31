use alloc::string::String;

use crate::{GPU_DEVICE, SCALE, SCREENHEIGHT, SCREENWIDTH, TEXT_DEFAULT, THEME, console_print, console_println, dbg, drivers::uart::{self, RX_BUFFER, get_key_latest}, games::breakout, mvulkan::console, shell_print};

/// Prepare the shell environment
pub fn start_shell() {
    unsafe {
        (*GPU_DEVICE.unwrap()).clear(0);
        let theme = THEME;
        console::origin();
        TEXT_DEFAULT = theme.yellow();
        RX_BUFFER = ['\0'; 256];
        shell_print!("" ; color: theme.yellow());
        run_shell();
    }
}

fn run_shell() {
    let mut hostname = "mvos";
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
                        shell_print!("" ; color: theme.yellow());
                    },
                    "version" => {
                        let v = env!("CARGO_PKG_VERSION");
                        console_println!("MVOS version {}", v; color: theme.white());
                        shell_print!("" ; color: theme.yellow());
                    },
                    "breakout" => {
                        breakout::breakout_main();
                    },
                    _ => {shell_print!(""; color: theme.yellow());}
                }
                RX_BUFFER = ['\0'; 256];
            }
        }
    }
}   