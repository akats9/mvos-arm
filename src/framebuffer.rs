use crate::{mmio::{mmio_read, mmio_write}, pci::{find_pci_device, pci_enable_device}, serial_println};

const VIRTIO_MMIO_STATUS: u64 = 0x70;

const VIRTIO_STATUS_RESET: u64 = 0x0;
const VIRTIO_STATUS_ACKNOWLEDGE: u64 = 0x01;
const VIRTIO_STATUS_DRIVER: u64 = 0x02;
const VIRTIO_DRIVER_OK: u64 = 0x04;
const VIRTIO_STATUS_FEATURES_OK: u64 = 0x08;
const VIRTIO_STATUS_FAILED: u64 = 0x80;

const VIRTIO_GPU_CMD_GET_DISPLAY_INFO: u64 = 0x100;

const PCI_BAR4: u64 = 0x20;
const PCI_BAR5: u64 = 0x24;

pub fn setup_gpu_bars(base: u64) -> u64 {
    serial_println!("Setting up GPU BAR4 and BAR5...");
    serial_println!("Writing BAR4 and BAR5...");

    mmio_write(base + 0x20, 0xFFFFFFFF);

    let mut bar4 = mmio_read(base + &PCI_BAR4);
    let mut bar5 = mmio_read(base + &PCI_BAR5);

    if bar4 == 0 || bar4 == 0xFFFFFFFF {
        serial_println!("BAR4 size probing failed");
        return 0x0;
    }

    let size = (!((bar5 << 32) | (bar4 & !0xF)) + 1) as u64;
    serial_println!("Calculated BAR size: {:x}", size);

    let mmio_base: u64 = 0x1001_0000;
    mmio_write(base + 0x20, (mmio_base & 0xFFFF_FFFF) as u32);
    mmio_write(base + 0x24, ((mmio_base >> 32) & 0xFFFF_FFFF) as u32);

    //Confirm the setup
    bar4 = mmio_read(base + &PCI_BAR4);
    bar5 = mmio_read(base + &PCI_BAR5);
    ((bar5 as u64) << 32) | (bar4 & !0xF)
}

pub fn virtio_gpu_display_on(base_addr: u64) {
    //Setting up the virtqueue (assuming queue 0 for simplicity)
    mmio_write(base_addr + 0x30, 0); //queue_select
    mmio_write(base_addr + 0x38, 128); //queue_size
    mmio_write(base_addr + 0x3C, 1); //queue_enable

    //Send display on command
    mmio_write(base_addr + 0x20, VIRTIO_GPU_CMD_GET_DISPLAY_INFO as u32);

    mmio_write(base_addr + 0x14, VIRTIO_DRIVER_OK as u32); //Set the DRIVER_OK status bit

    let status: u64 = mmio_read(base_addr + 0x14);
    if status & 4 != 0 {
        serial_println!("Display activated.");
    } else {
        serial_println!("Display activation failed.");
    }
}

pub fn virtio_gpu_init(base_addr: u64) {
    mmio_write(base_addr + 0x14, 0); //Reset the device
    while mmio_read(base_addr + 0x14) != 0 {
        core::hint::spin_loop();
    }

    mmio_write(base_addr + 0x14, VIRTIO_STATUS_ACKNOWLEDGE as u32); //Acknowledge the device
    mmio_write(base_addr, VIRTIO_STATUS_DRIVER as u32); //Driver

    mmio_write(base_addr + 0x0, 0); //Select feature bits 0-31
    let features = mmio_read(base_addr + 0x4); //Read features
    mmio_write(base_addr + 0x08, 0); //Select driver features 0-31
    mmio_write(base_addr + 0x0C, features as u32); //Write features

    mmio_write(base_addr + 0x14, VIRTIO_STATUS_FEATURES_OK as u32); //Features OK
    if !mmio_read(base_addr + 0x14) & 8 != 0 {
        serial_println!("Features OK not set, device unusable.");
        return;
    } 

    mmio_write(base_addr + 0x14, 4);
    serial_println!("GPU Initialization complete.");
}

pub fn gpu_init() {
    let mut mmio_base: u64 = 0x1001_0000;
    let address = find_pci_device(0x1AF4, 0x1050, mmio_base as *mut u64);

    if address > 0 {
        serial_println!("Virtio GPU detected at {:x}.", address);

        serial_println!("Initializing GPU...");

        mmio_base = setup_gpu_bars(address);
        pci_enable_device(address);

        if mmio_base == 0 {
            serial_println!("Failed to read GPU MMIO base.");
            return;
        }

        serial_println!("MMIO base: {:x}.", mmio_base);

        virtio_gpu_init(mmio_base);

        virtio_gpu_display_on(mmio_base);
    }
}