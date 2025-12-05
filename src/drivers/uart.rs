use core::{ffi::{c_char, CStr}, fmt::Write};

use crate::{GPU_DEVICE, SCALE, SCREENHEIGHT, SCREENWIDTH, console_print, console_println, dbg, memory::mmio::mmio_write32, mvulkan::{color::GENERIC_WHITE, console::{self, newline}}, trinkets::templeos_color_palette::WHITE};

// UART base address for QEMU virt machine
const UART_BASE: *mut u8 = 0x09000000 as *mut u8;

// UART register offsets
const UART_DR: isize = 0x00;    // Data Register
const UART_FR: isize = 0x18;    // Flag Register
const UART_IMSC: isize = 0x38;
const UART_ICR: isize = 0x44;

// Flag register bits
const UART_FR_TXFF: u8 = 1 << 5; // Transmit FIFO full
const UART_RXIM: u8 = 1 << 4;
const UART_RTIM: u8 = 1 << 6;

// Input buffer
const BUF_SIZE: usize = 256;
static mut RX_BUFFER: [char; BUF_SIZE] = [0 as char; BUF_SIZE];
static mut RX_HEAD: usize = 0; // where irq writes
static mut RX_TAIL: usize = 0; // read

pub unsafe fn uart_enable_rxim() {
    (*((UART_BASE as isize+UART_IMSC) as *mut usize)) |= UART_RXIM as usize | UART_RTIM as usize;
}

pub fn uart_irq_handler() {
    let mut flags: *mut u32 = (UART_BASE as isize+UART_FR) as *mut u32;
    let mut data: *mut u32 = (UART_BASE as isize+UART_DR) as *mut u32;
    let mut icr: *mut u32 = (UART_BASE as isize+UART_ICR) as *mut u32;

    unsafe {
        while ((*flags)&(1<<4)) == 0 {
            let c: char = ((*data) & 0xff) as u8 as char;
            // store in buffer
            RX_BUFFER[RX_HEAD] = c;
            RX_HEAD = (RX_HEAD + 1) % BUF_SIZE;
            // print on screen
            dbg!("UART: got {:?}", c.as_ascii());
            if c == '\r' { console::newline();}
            else if c == '\x7f' { console::backspace(); }
            else { console_print!("{}", c ; color: GENERIC_WHITE); }
        }
        mmio_write32(icr as u64, UART_RXIM as u32 | UART_RTIM as u32);
    }
}

/// Write a single byte to the UART
fn uart_write_byte(byte: u8) {
    unsafe {
        // Wait until transmit FIFO is not full
        while (UART_BASE.offset(UART_FR).read_volatile() & UART_FR_TXFF) != 0 {
            core::hint::spin_loop();
        }
        // Write byte to data register
        UART_BASE.offset(UART_DR).write_volatile(byte);
    }
}

/// UART writer struct that implements Write trait
pub struct UartWriter;

impl Write for UartWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            uart_write_byte(byte);
        }
        Ok(())
    }
}

/// Print formatted string to UART without newline
#[macro_export]
#[macro_use]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut writer = $crate::UartWriter;
            write!(writer, $($arg)*).ok();
        }
    };
}

/// Print formatted string to UART with newline
#[macro_export]
#[macro_use]
macro_rules! serial_println {
    () => {
        $crate::serial_print!("\n")
    };
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut writer = $crate::UartWriter;
            writeln!(writer, $($arg)*).ok();
        }
    };
}

/// Print formatted debug string to UART with colored output and DEBUG prefix
#[macro_export]
#[macro_use]
macro_rules! dbg {
    () => {
        {
            use core::fmt::Write;
            let mut writer = $crate::UartWriter;
            write!(writer, "[   DEBUG   ] \x1b[0;33m\x1b[0m\n").ok();
        }
    };
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut writer = $crate::UartWriter;
            write!(writer, "[   DEBUG   ] \x1b[0;33m").ok();
            write!(writer, $($arg)*).ok();
            write!(writer, "\x1b[0m\n").ok();
        }
    };
}

// Example usage:
// serial_print!("Hello, ");
// serial_println!("world! Counter: {}", 42);
// serial_println!(); // Just a newline

/// serial_println FFI binding for C (no varargs)
#[unsafe(no_mangle)]
pub extern "C" fn c_serial_println(message: *const c_char) {
    unsafe {
        let c_str = CStr::from_ptr(message);
        match c_str.to_str() {
            Ok(str_slice) => serial_println!("{}", str_slice),
            Err(_) => serial_println!("[  SERIAL   ] \x1b[0;31mError: Invalid UTF-8 string passed from C.\x1b[0m"),
        }
    }
}

/// dbg FFI binding for C (no varargs)
#[unsafe(no_mangle)]
pub extern "C" fn c_dbg(message: *const c_char) {
    unsafe {
        let c_str = CStr::from_ptr(message);
        match c_str.to_str() {
            Ok(str_slice) => dbg!("{}", str_slice),
            Err(_) => serial_println!("[  SERIAL   ] \x1b[0;31mError: Invalid UTF-8 string passed from C.\x1b[0m"),
        }
    }
}

/// dgb for integers (hex format, u64)
#[unsafe(no_mangle)]
pub extern "C" fn c_dgb_hex(hex: u64) {
    dbg!("{:x}", hex);
}

/// dgb for integers (bin format, u64)
#[unsafe(no_mangle)]
pub extern "C" fn c_dbg_bin(bin: u64) {
    dbg!("{:b}", bin);
}