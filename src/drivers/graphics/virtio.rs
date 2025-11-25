use crate::{drivers, mvulkan::MVulkanGPUDriver, serial_println};

unsafe extern "C" {
    fn virtio_generic_setup_c(virtio_base: u64, device_id: u16) -> i32;
    fn pci_enable_device_c(base: u64) -> bool;
}

pub struct VirtioDriver {
    base: u64,
}

impl VirtioDriver {
    pub fn new() -> Result<Self, u8> {
        let virtio_gpu_base = drivers::pci::find_pci_device(0x1af4, 0x1050);

        if virtio_gpu_base == 0x0 {
            serial_println!("[  DRIVERS  ] Finding VirtIO GPU device... \x1b[0;31mFAILED\x1b[0m");
            return Err(1);
        } else {
            serial_println!("[  DRIVERS  ] Finding VirtIO GPU device... \x1b[0;32mSUCCESS\x1b[0m");
            Ok(Self{base: virtio_gpu_base})
        }
    }
}

impl MVulkanGPUDriver for VirtioDriver {
    fn setup(&mut self) -> Result<(), &'static str> {
        unsafe { 
            let virtio_gpu_enabled =  pci_enable_device_c(self.base); 
            serial_println!("[  DRIVERS  ] Enabling VirtIO GPU device... {}\x1b[0m", {if virtio_gpu_enabled {"\x1b[0;32mSUCCESS"} else {"\x1b[0;31mFAILED"}});
        }
        unsafe {
            match virtio_generic_setup_c(self.base, 0x10) {
                0 => Ok(()),
                -1 => Err("Device status register does not contain a capabilities list."),
                -2 => Err("Common_cfg capability not found"),
                -3 => Err("Device did not accept negotiated features"),
                _ => panic!("VIRTIO_GENERIC_SETUP_C FUNCTION RETURNED AN UNKNOWN RETURN VALUE"),
            }
        }
    }

    fn draw_char(&mut self, utf8: usize, r: u8, g: u8, b: u8, x: u32, y: u32, scale: u8) {
        
    }

    fn clear(&mut self, color: u8) {
        
    }

    fn draw_rect(&mut self, minx: u32, maxx: u32, miny: u32, maxy: u32, r: u8, g: u8, b: u8) {
        
    }
    
    fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) {
        
    }
}