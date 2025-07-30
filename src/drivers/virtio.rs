// Docs: https://docs.oasis-open.org/virtio/virtio/v1.2/csd01/virtio-v1.2-csd01.html#x1-1420009

use crate::{drivers::pci::pci_get_bar, memory::mmio::mmio_read32, serial_println};

const ACKNOWLEDGE: usize = 1;
const DRIVER: usize = 2;
const FAILED: usize = 128;
const FEATURES_OK: usize = 8;
const DRIVER_OK: usize = 4;
const DEVICE_NEEDS_RESET: usize = 64;
const RESET: usize = 0;

#[repr(C, packed)]
struct VirtioPciCap {
    cap_vndr: u8, // 0x09; identifies a vendor specific capability
    cap_next: u8,
    cap_len: u8,
    cfg_type: u8, // identifies the structure, see consts below
    bar: u8,
    id: u8,
    padding: [u8; 2],
    offset: u32, // LITTLE ENDIAN
    length: u32, // LITTLE ENDIAN
}

// cfg_type

/// Common configuration
const VIRTIO_PCI_CAP_COMMON_CFG: usize = 1;
/// Notifications
const VIRTIO_PCI_CAP_NOTIFY_CFG: usize = 2;
/// ISR status
const VIRTIO_PCI_CAP_ISR_CFG: usize = 3;
/// Device specific configuration
const VIRTIO_PCI_CAP_DEVICE_CFG: usize = 4;
/// PCI configuration access
const VIRTIO_PCI_CAP_PCI_CFG: usize = 5;
/// Shared memory region
const VIRTIO_PCI_CAP_SHARED_MEMORY_CFG: usize = 8;
/// Vendor specific data
const VIRTIO_PCI_CAP_VENDOR_CFG: usize = 9;

/// The common configuration structure is found at the bar and offset within 
/// the VIRTIO_PCI_CAP_COMMON_CFG capability;
#[repr(C, packed)]
struct VirtioPciCommonCfg {
    // About the whole device
    device_feature_select: u32, // r/w; le
    device_feature: u32, // ro for driver; le
    driver_feature_select: u32, // r/w; le
    driver_feature: u32, // r/w; le
    config_msix_vector: u16, // r/w; le
    num_queues: u16, // ro for driver; le
    device_status: u8, // r/w
    config_generation: u8, // ro for driver

    // About a specific virtqueue
    queue_select: u16, // r/w; le
    queue_size: u16, // r/w; le
    queue_msix_vector: u16, // r/w; le
    queue_enable: u16, // r/w; le
    queue_notify_off: u16, // ro for driver; le
    queue_desc: u64, // r/w; le
    queue_driver: u64, // r/w; le
    queue_device: u64, // r/w; le
    queue_notify_data: u16, // ro for driver; le
    queue_reset: u16, // r/w; le
}

#[repr(C, packed)]
struct VirtioDevice {
    common_cfg: *mut VirtioPciCommonCfg,
    notify_cfg: *mut u8,
    device_cfg: *mut u8,
    isr_cfg: *mut u8,
    notify_off_multiplier: u32,
}