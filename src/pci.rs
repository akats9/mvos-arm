use crate::{mmio::{self, mmio_read, mmio_write}, serial_print, serial_println};

const PCI_ECAM_BASE: u64 = 0x4010000000;
const PCI_BUS_MAX: u64 = 256;
const PCI_SLOT_MAX: u64 = 32;
const PCI_FUNC_MAX: u64 = 8;

const PCI_CMD_REG: u64 = 0x04;

pub fn pci_make_addr(bus: u32, slot: u32, func: u32, offset: u32) -> u64 {
    PCI_ECAM_BASE | ((bus as u64) << 20) | ((slot as u64) << 15) | ((func as u64) << 12) | (offset & 0xFFF) as u64
}

pub fn pci_get_bar(base: u64, index: u8, offset: u8) -> u64 {
    base + offset as u64 + (index as u64 * 4)
}

pub fn debug_read_bar(base: u64, index: u8, offset: u8) {
    serial_print!("Reading BAR @ ");
    let addr: u64 = pci_get_bar(base, index, offset);
    serial_print!("{:x}", addr);
    let val: u64 = mmio_read(addr);
    serial_println!(" ({:x}) content: {:x}", index, val);
}

pub fn inspect_bar(base: u64, offset: u8) {
    serial_println!("Inspecting GPU Bars...\n");

    // for bar_offset in (0x10..=0x28).step_by(4) {
    //     let mut bar: u64 = mmio_read(base + bar_offset) as u64;
    //     serial_println!("BAR @ offset {:x}: {:x}", bar_offset, bar);

    //     //Check for 64bit BAR
    //     if (bar & 0x4) != 0 {
    //         serial_println!("  - 64-bit Prefetchable Memory");
    //         let high: u64 = mmio_read(base + bar_offset + 4) as u64;
    //         serial_println!("  - Upper 32 bits: {:x}", high);
    //         bar |= (high as u64) << 32;
    //     }

    //     //Check if MMIO or IO
    //     if bar & 1 != 0 {
    //         serial_println!("  - I/O Space");
    //     } else {
    //         let mmio_base: u64 = bar & !0xF;
    //         serial_println!("  - MMIO Space");
    //         serial_println!("  - MMIO Base: {:x}", mmio_base);
    //     }
    // }

    serial_println!("Inspecting GPU BARs...");
    for bar_offset in (0x0..=0x18).step_by(4) {
        debug_read_bar(base, bar_offset, offset);
    }
}

pub fn find_pci_device(vendor_id: u32, device_id: u32, out_mmio_base: *mut u64) -> u64 {
    for bus in 0..PCI_BUS_MAX as u32 {
        for slot in 0..PCI_SLOT_MAX as u32 {
            for func in 0..PCI_FUNC_MAX as u32 {
                let device_address = pci_make_addr(bus, slot, func, 0x00);
                let vendor_device = mmio_read(device_address);

                if (vendor_device & 0xFFFF) == vendor_id as u64 && (vendor_device >> 16) & 0xFFFF == device_id as u64 {
                    serial_println!("Found device at bus {:x}, slot {:x}, func {:x}", bus, slot, func);

                    unsafe {
                        out_mmio_base.write_volatile(device_address);
                    }

                    return device_address;
                }
            }
        }
    }

    serial_println!("Device not found.");
    0_u64
}

pub fn dump_pci_config(base: u64) {
    serial_println!("Dumping PCI Configuration Space:");

    for offset in (0..0x40).step_by(4) {
        let val = mmio_read(base + offset);
        serial_println!("Offset {:x}: {:x}", offset, val);
    }
}

pub fn pci_enable_device(base: u64) {
    let cmd_before = mmio_read(base + 0x04);
    serial_println!("PCI Command Register before: {:x}", cmd_before);

    //Set the Memory Space Enable (MSE) and Bus Master Enable (BME) bits
    let cmd = cmd_before | 0x7;

    serial_println!("Setting CMD: {:x}", cmd);

    mmio_write(base + 0x4, cmd as u32);

    let cmd_after = mmio_read(base + 0x04);
    serial_println!("PCI Command Register after: {:x}", cmd_after);

    if (cmd_after & 0x7) == 0x7 {
        serial_println!("PCI device succesfully enabled.");
    } else {
        serial_println!("Failed to enable PCI device (MSE/BME not set).");
    }
}