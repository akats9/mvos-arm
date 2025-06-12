use core::fmt::Write;

// UART base address for QEMU virt machine
const UART_BASE: *mut u8 = 0x09000000 as *mut u8;

// UART register offsets
const UART_DR: isize = 0x00;    // Data Register
const UART_FR: isize = 0x18;    // Flag Register

// Flag register bits
const UART_FR_TXFF: u8 = 1 << 5; // Transmit FIFO full

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

// Example usage:
// serial_print!("Hello, ");
// serial_println!("world! Counter: {}", 42);
// serial_println!(); // Just a newline