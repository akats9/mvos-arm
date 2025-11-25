//! Pseudo-random number generator for MVOS
//! 
//! Uses the Linear Congruential Generator algorithm

use alloc::vec::Vec;
use alloc::string::String;

use crate::{BIBLE, TIMER, console_println, SCALE, SCREENHEIGHT, SCREENWIDTH, GPU_DEVICE};

pub fn random(seed: usize) -> usize {
    let m = 2_usize.pow(32);
    let a = 1664525;
    let c = 1013904223;
    (a * seed + c) % m
}

/// Print a random line from the Bible to bless the system
pub fn random_bible_line(seed: usize) -> Option<&'static str> {
    let lines: Vec<&str> = BIBLE.lines().collect();

    if lines.is_empty() { return None; }

    let index = random(seed) % lines.len();
    Some(lines[index])
}

pub fn random_x_lines(x: usize) {
    for i in 0..x {
        let l = match random_bible_line(46748 * i * unsafe {TIMER + 35} + 482943 * unsafe { TIMER + 3 }) {
            Some(l) => l, 
            None => "--- !!! YOU HAVE REACHED HELL !!! ---",
        };
        let color = match l {
            "--- !!! YOU HAVE REACHED HELL !!! ---" => 0xff0000,
            _ => 0xffbb44,
        };
        console_println!("{}", l ; color: color);
    }
}